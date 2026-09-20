use super::{
    APPLICATION_SCHEMA, ApplicationDefinition, CommandEnvelope, CoreError, Diagnostic, ResultClass,
    StrictJsonValue, command_envelope, digest_bytes, hex_digest, invocation_identity,
    validate_application_id, validate_application_relationships,
};
use crate::readiness::{
    EnvironmentReadinessReport, ReadinessAggregateStatus, create_environment_readiness_from_bytes,
    mark_environment_readiness_stale,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

pub const APPLICATION_READINESS_REQUEST_SCHEMA: &str = "ferris.application-readiness-request/v1";
pub const APPLICATION_READINESS_REPORT_SCHEMA: &str = "ferris.application-readiness-report/v1";
pub const MAX_APPLICATION_READINESS_INPUT_BYTES: u64 = 1024 * 1024;
const APPLICATION_READINESS_LIMITATION: &str = "FERRIS-APPLICATION-READINESS-WORKSPACE-ONLY";
const MIN_APPLICATION_READINESS_WORKSPACES: usize = 2;
const MAX_APPLICATION_READINESS_WORKSPACES: usize = 16;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ApplicationReadinessRequest {
    schema: String,
    application_id: String,
    application_definition: BoundApplicationDefinition,
    workspaces: Vec<ApplicationReadinessWorkspace>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BoundApplicationDefinition {
    path: String,
    digest: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ApplicationReadinessWorkspace {
    workspace_id: String,
    manifest_path: String,
    requirements_path: String,
    requirements_digest: String,
}

struct LoadedApplicationReadinessRequest {
    request: ApplicationReadinessRequest,
    bytes: Vec<u8>,
    digest: String,
    path: PathBuf,
    root: PathBuf,
    identity: String,
}

struct LoadedPassiveApplication {
    definition: ApplicationDefinition,
    bytes: Vec<u8>,
    path: PathBuf,
    identity: String,
}

struct BoundWorkspace {
    workspace_id: String,
    manifest_path: PathBuf,
    requirements_path: PathBuf,
    requirements_digest: String,
    requirements_bytes: Vec<u8>,
    requirements_identity: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationReadinessCompositionStatus {
    Current,
    Stale,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationReadinessWorkspaceResult {
    pub workspace_id: String,
    pub requirements_digest: String,
    pub report_id: String,
    pub aggregate_status: ReadinessAggregateStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationReadinessReport {
    pub schema: String,
    pub report_id: String,
    pub request_digest: String,
    pub application_id: String,
    pub composition_status: ApplicationReadinessCompositionStatus,
    pub diagnostic_code: String,
    pub aggregate_status: ReadinessAggregateStatus,
    pub workspace_results: Vec<ApplicationReadinessWorkspaceResult>,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

pub fn create_application_readiness(
    request_path: &Path,
) -> Result<CommandEnvelope<ApplicationReadinessReport>, CoreError> {
    create_application_readiness_with_hook(request_path, || {})
}

fn create_application_readiness_with_hook(
    request_path: &Path,
    before_revalidation: impl FnOnce(),
) -> Result<CommandEnvelope<ApplicationReadinessReport>, CoreError> {
    let loaded = load_application_readiness_request(request_path)?;
    let application = load_passive_application(&loaded)
        .map_err(|error| error.with_invocation_selection(loaded.digest.clone()))?;
    let workspaces = bind_workspaces(&loaded, &application)
        .map_err(|error| error.with_invocation_selection(loaded.digest.clone()))?;

    let mut workspace_results = Vec::with_capacity(workspaces.len());
    for workspace in &workspaces {
        let envelope = create_environment_readiness_from_bytes(
            &workspace.manifest_path,
            &workspace.workspace_id,
            &workspace.requirements_path,
            &workspace.requirements_bytes,
        )
        .map_err(|error| error.with_invocation_selection(loaded.digest.clone()))?;
        let mut report = envelope.record.ok_or_else(application_readiness_internal)?;
        if !unchanged(
            &workspace.requirements_path,
            &workspace.requirements_bytes,
            &workspace.requirements_identity,
        ) {
            mark_environment_readiness_stale(&mut report)
                .map_err(|error| error.with_invocation_selection(loaded.digest.clone()))?;
        }
        if report.workspace_id != workspace.workspace_id
            || (report.requirements_digest != workspace.requirements_digest
                && report.aggregate_status != ReadinessAggregateStatus::Stale)
        {
            return Err(
                application_readiness_internal().with_invocation_selection(loaded.digest.clone())
            );
        }
        workspace_results.push(workspace_result(&report));
    }

    before_revalidation();
    let composition_status = if unchanged(&loaded.path, &loaded.bytes, &loaded.identity)
        && unchanged(&application.path, &application.bytes, &application.identity)
    {
        ApplicationReadinessCompositionStatus::Current
    } else {
        ApplicationReadinessCompositionStatus::Stale
    };
    let aggregate_status = application_aggregate_status(composition_status, &workspace_results);
    let diagnostic_code = match composition_status {
        ApplicationReadinessCompositionStatus::Stale => "FERRIS-APPLICATION-READINESS-INPUT-STALE",
        ApplicationReadinessCompositionStatus::Current
            if aggregate_status == ReadinessAggregateStatus::Stale =>
        {
            "FERRIS-APPLICATION-READINESS-WORKSPACE-STALE"
        }
        ApplicationReadinessCompositionStatus::Current => "FERRIS-APPLICATION-READINESS-CURRENT",
    };
    let mut report = ApplicationReadinessReport {
        schema: APPLICATION_READINESS_REPORT_SCHEMA.to_owned(),
        report_id: String::new(),
        request_digest: loaded.digest.clone(),
        application_id: loaded.request.application_id.clone(),
        composition_status,
        diagnostic_code: diagnostic_code.to_owned(),
        aggregate_status,
        workspace_results,
        unknowns: Vec::new(),
        limitations: vec![APPLICATION_READINESS_LIMITATION.to_owned()],
    };
    report.report_id = application_readiness_report_identity(&report)?;

    let selection_identity =
        application_readiness_selection_identity(&report.application_id, &loaded.digest);
    let invocation_identity =
        application_readiness_invocation_identity(&selection_identity, request_path);
    let result_class = result_class_for_aggregate(aggregate_status);
    let diagnostics = application_readiness_result_diagnostic(result_class)
        .into_iter()
        .collect();
    Ok(command_envelope(
        "doctor",
        selection_identity,
        invocation_identity,
        result_class,
        diagnostics,
        Some(report),
    ))
}

pub fn application_readiness_error_envelope<T>(
    request_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let selection_material = error
        .invocation_selection()
        .map(str::to_owned)
        .unwrap_or_else(|| explicit_path_identity(request_path));
    let selection_identity =
        application_readiness_selection_identity("unavailable", &selection_material);
    command_envelope(
        "doctor",
        selection_identity.clone(),
        application_readiness_invocation_identity(&selection_identity, request_path),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn render_application_readiness_human(
    envelope: &CommandEnvelope<ApplicationReadinessReport>,
) -> String {
    let report = envelope
        .record
        .as_ref()
        .expect("application readiness envelope has a report");
    let mut output = format!(
        "Ferris application readiness {}\nApplication ID: {}\nRequest digest: {}\nComposition: {}\nAggregate: {}\nWorkspaces:\n",
        report.report_id,
        report.application_id,
        report.request_digest,
        composition_status_name(report.composition_status),
        aggregate_status_name(report.aggregate_status),
    );
    for workspace in &report.workspace_results {
        output.push_str(&format!(
            "  - {}: {} ({})\n",
            workspace.workspace_id,
            aggregate_status_name(workspace.aggregate_status),
            workspace.report_id
        ));
    }
    output.push_str("Unknowns:\n  - none\nLimitations:\n");
    for limitation in &report.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

fn load_application_readiness_request(
    path: &Path,
) -> Result<LoadedApplicationReadinessRequest, CoreError> {
    reject_symlink_or_reparse(path).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The application readiness request path is not an ordinary file.",
        )
    })?;
    let bytes = read_bounded(path, "request")?;
    let digest = digest_bytes(&bytes);
    let value = parse_strict_json(&bytes)
        .map_err(|error| error.with_invocation_selection(digest.clone()))?;
    let request: ApplicationReadinessRequest = serde_json::from_value(value).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The application readiness request does not match its strict schema.",
        )
        .with_invocation_selection(digest.clone())
    })?;
    validate_request(&request).map_err(|error| error.with_invocation_selection(digest.clone()))?;
    let canonical_path = path.canonicalize().map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The application readiness request could not be resolved.",
        )
        .with_invocation_selection(digest.clone())
    })?;
    if !canonical_path.is_file() {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The application readiness request is not a file.",
        )
        .with_invocation_selection(digest));
    }
    let root = canonical_path
        .parent()
        .expect("a canonical request file has a parent")
        .to_path_buf();
    let identity = filesystem_identity(&canonical_path)?;
    Ok(LoadedApplicationReadinessRequest {
        request,
        bytes,
        digest,
        path: canonical_path,
        root,
        identity,
    })
}

fn load_passive_application(
    loaded: &LoadedApplicationReadinessRequest,
) -> Result<LoadedPassiveApplication, CoreError> {
    let path = resolve_contained_file(
        &loaded.root,
        &loaded.request.application_definition.path,
        RelativeFileKind::ApplicationDefinition,
    )?;
    let bytes = read_bounded(&path, "application definition")?;
    if digest_bytes(&bytes) != loaded.request.application_definition.digest {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-DEFINITION-DIGEST-MISMATCH",
            "The Application Definition bytes do not match the request.",
        ));
    }
    let value = parse_strict_json(&bytes)?;
    let definition: ApplicationDefinition = serde_json::from_value(value).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The Application Definition does not match strict ferris.application/v0.",
        )
    })?;
    if definition.schema != APPLICATION_SCHEMA {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The Application Definition schema is unsupported.",
        ));
    }
    validate_application_id(&definition.application_id).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The Application Definition application_id is invalid.",
        )
    })?;
    if !(MIN_APPLICATION_READINESS_WORKSPACES..=MAX_APPLICATION_READINESS_WORKSPACES)
        .contains(&definition.workspaces.len())
    {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-WORKSPACE-COVERAGE-MISMATCH",
            "The Application Definition workspace count is outside the supported bound.",
        ));
    }
    validate_application_relationships(&definition).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The Application Definition relationships are invalid.",
        )
    })?;
    if definition.application_id != loaded.request.application_id {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-APPLICATION-ID-MISMATCH",
            "The request and Application Definition application IDs differ.",
        ));
    }
    let identity = filesystem_identity(&path)?;
    Ok(LoadedPassiveApplication {
        definition,
        bytes,
        path,
        identity,
    })
}

fn bind_workspaces(
    loaded: &LoadedApplicationReadinessRequest,
    application: &LoadedPassiveApplication,
) -> Result<Vec<BoundWorkspace>, CoreError> {
    let definitions = application
        .definition
        .workspaces
        .iter()
        .map(|workspace| {
            (
                workspace.workspace_id.as_str(),
                workspace.manifest_path.as_str(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    if definitions.len() != loaded.request.workspaces.len()
        || loaded
            .request
            .workspaces
            .iter()
            .any(|workspace| !definitions.contains_key(workspace.workspace_id.as_str()))
    {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-WORKSPACE-COVERAGE-MISMATCH",
            "The request does not cover every Application Definition workspace exactly once.",
        ));
    }

    let mut manifests = BTreeSet::new();
    let mut roots = Vec::with_capacity(loaded.request.workspaces.len());
    let mut bound = Vec::with_capacity(loaded.request.workspaces.len());
    for workspace in &loaded.request.workspaces {
        if definitions.get(workspace.workspace_id.as_str()).copied()
            != Some(workspace.manifest_path.as_str())
        {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-MANIFEST-MISMATCH",
                "A request manifest path differs from the Application Definition.",
            ));
        }
        let manifest_path = resolve_contained_file(
            &loaded.root,
            &workspace.manifest_path,
            RelativeFileKind::CargoManifest,
        )?;
        if !manifests.insert(filesystem_identity(&manifest_path)?) {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-WORKSPACE-ROOT-DUPLICATE",
                "The request resolves more than one workspace to the same manifest.",
            ));
        }
        let workspace_root = manifest_path
            .parent()
            .expect("a canonical Cargo.toml has a parent")
            .to_path_buf();
        if roots.iter().any(|existing: &PathBuf| {
            workspace_root.starts_with(existing) || existing.starts_with(&workspace_root)
        }) {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-WORKSPACE-ROOT-NESTED",
                "Application readiness workspace roots must be distinct and non-nested.",
            ));
        }
        roots.push(workspace_root);

        let requirements_path = resolve_contained_file(
            &loaded.root,
            &workspace.requirements_path,
            RelativeFileKind::Requirements,
        )?;
        let requirements_bytes = read_bounded(&requirements_path, "requirements declaration")?;
        if digest_bytes(&requirements_bytes) != workspace.requirements_digest {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUIREMENTS-DIGEST-MISMATCH",
                "A requirements declaration does not match the request digest.",
            ));
        }
        let requirements_value = parse_strict_json(&requirements_bytes)?;
        if requirements_value
            .get("schema")
            .and_then(serde_json::Value::as_str)
            != Some(super::ENVIRONMENT_REQUIREMENTS_SCHEMA)
        {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "A requirements declaration is not ferris.environment-requirements/v1.",
            ));
        }
        if requirements_value
            .get("workspace_id")
            .and_then(serde_json::Value::as_str)
            != Some(workspace.workspace_id.as_str())
        {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-WORKSPACE-ID-MISMATCH",
                "A requirements declaration workspace ID differs from the request.",
            ));
        }
        let requirements_identity = filesystem_identity(&requirements_path)?;
        bound.push(BoundWorkspace {
            workspace_id: workspace.workspace_id.clone(),
            manifest_path,
            requirements_path,
            requirements_digest: workspace.requirements_digest.clone(),
            requirements_identity,
            requirements_bytes,
        });
    }
    Ok(bound)
}

fn validate_request(request: &ApplicationReadinessRequest) -> Result<(), CoreError> {
    if request.schema != APPLICATION_READINESS_REQUEST_SCHEMA
        || !(MIN_APPLICATION_READINESS_WORKSPACES..=MAX_APPLICATION_READINESS_WORKSPACES)
            .contains(&request.workspaces.len())
        || request.application_definition.path.contains('/')
        || !valid_relative_component(&request.application_definition.path)
        || !valid_digest(&request.application_definition.digest)
    {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The application readiness request is invalid.",
        ));
    }
    validate_application_id(&request.application_id).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "The application readiness request application_id is invalid.",
        )
    })?;

    let mut previous = None;
    for workspace in &request.workspaces {
        if previous.is_some_and(|value: &str| value >= workspace.workspace_id.as_str())
            || !valid_portable_id(&workspace.workspace_id)
            || !valid_relative_file(&workspace.manifest_path)
            || Path::new(&workspace.manifest_path)
                .file_name()
                .and_then(OsStr::to_str)
                != Some("Cargo.toml")
            || !valid_relative_file(&workspace.requirements_path)
            || !valid_digest(&workspace.requirements_digest)
        {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "The application readiness workspace entries are invalid or not sorted.",
            ));
        }
        previous = Some(workspace.workspace_id.as_str());
    }
    Ok(())
}

fn parse_strict_json(bytes: &[u8]) -> Result<serde_json::Value, CoreError> {
    let value = serde_json::from_slice::<StrictJsonValue>(bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| {
            application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "An application readiness input is not strict JSON.",
            )
        })?;
    if contains_json_null(&value) {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "Application readiness inputs must not contain explicit null values.",
        ));
    }
    Ok(value)
}

fn read_bounded(path: &Path, label: &str) -> Result<Vec<u8>, CoreError> {
    let file = File::open(path).map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            format!("The {label} is unavailable."),
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_APPLICATION_READINESS_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                format!("The {label} could not be read."),
            )
        })?;
    if bytes.len() as u64 > MAX_APPLICATION_READINESS_INPUT_BYTES {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            format!("The {label} exceeds the supported size bound."),
        ));
    }
    Ok(bytes)
}

#[derive(Clone, Copy)]
enum RelativeFileKind {
    ApplicationDefinition,
    CargoManifest,
    Requirements,
}

fn resolve_contained_file(
    root: &Path,
    relative: &str,
    kind: RelativeFileKind,
) -> Result<PathBuf, CoreError> {
    if !valid_relative_file(relative)
        || matches!(kind, RelativeFileKind::ApplicationDefinition) && relative.contains('/')
        || matches!(kind, RelativeFileKind::CargoManifest)
            && Path::new(relative).file_name().and_then(OsStr::to_str) != Some("Cargo.toml")
    {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "An application readiness path is invalid.",
        ));
    }
    let mut current = root.to_path_buf();
    for component in relative.split('/') {
        current.push(component);
        let metadata = fs::symlink_metadata(&current).map_err(|_| {
            application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "An application readiness path is unavailable.",
            )
        })?;
        if is_symlink_or_reparse(&metadata) {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "Application readiness paths must not contain symlinks or reparse points.",
            ));
        }
    }
    let canonical = current.canonicalize().map_err(|_| {
        application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "An application readiness path could not be resolved.",
        )
    })?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(application_readiness_invalid(
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            "An application readiness path is outside the application root or is not a file.",
        ));
    }
    Ok(canonical)
}

fn valid_relative_file(value: &str) -> bool {
    (1..=1024).contains(&value.len())
        && !Path::new(value).is_absolute()
        && Path::new(value)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        && value
            .split('/')
            .all(|component| valid_relative_component(component))
}

fn valid_relative_component(value: &str) -> bool {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.ends_with('.')
        || value.ends_with(' ')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
    {
        return false;
    }
    let base = value
        .split('.')
        .next()
        .unwrap_or(value)
        .to_ascii_uppercase();
    !matches!(
        base.as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    )
}

fn valid_portable_id(value: &str) -> bool {
    if value.len() > 128 || value.matches('/').count() != 1 {
        return false;
    }
    value.split('/').all(|segment| {
        !segment.is_empty()
            && segment.len() <= 64
            && segment
                .bytes()
                .next()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            && segment.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
    })
}

fn valid_digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn contains_json_null(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
        serde_json::Value::Array(values) => values.iter().any(contains_json_null),
        serde_json::Value::Object(values) => values.values().any(contains_json_null),
        _ => false,
    }
}

fn reject_symlink_or_reparse(path: &Path) -> std::io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if is_symlink_or_reparse(&metadata) {
        return Err(std::io::Error::other("unsupported link"));
    }
    Ok(())
}

fn is_symlink_or_reparse(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn unchanged(path: &Path, original: &[u8], original_identity: &str) -> bool {
    reject_symlink_or_reparse(path).is_ok()
        && filesystem_identity(path).is_ok_and(|identity| identity == original_identity)
        && read_bounded(path, "composition input").is_ok_and(|bytes| bytes == original)
}

fn filesystem_identity(path: &Path) -> Result<String, CoreError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::metadata(path).map_err(|_| {
            application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "An application readiness filesystem identity is unavailable.",
            )
        })?;
        Ok(format!("unix:{}:{}", metadata.dev(), metadata.ino()))
    }
    #[cfg(windows)]
    {
        use std::ffi::c_void;
        use std::fs::File;
        use std::mem::MaybeUninit;
        use std::os::windows::io::AsRawHandle;

        #[repr(C)]
        struct FileTime {
            low: u32,
            high: u32,
        }

        #[repr(C)]
        struct ByHandleFileInformation {
            attributes: u32,
            creation_time: FileTime,
            last_access_time: FileTime,
            last_write_time: FileTime,
            volume_serial_number: u32,
            file_size_high: u32,
            file_size_low: u32,
            number_of_links: u32,
            file_index_high: u32,
            file_index_low: u32,
        }

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetFileInformationByHandle(
                file: *mut c_void,
                information: *mut ByHandleFileInformation,
            ) -> i32;
        }

        let file = File::open(path).map_err(|_| {
            application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "An application readiness file identity is unavailable.",
            )
        })?;
        let mut information = MaybeUninit::<ByHandleFileInformation>::uninit();
        // SAFETY: the handle remains open for the call and the OS initializes the output on success.
        let result = unsafe {
            GetFileInformationByHandle(file.as_raw_handle().cast(), information.as_mut_ptr())
        };
        if result == 0 {
            return Err(application_readiness_invalid(
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
                "An application readiness file identity is unavailable.",
            ));
        }
        // SAFETY: a successful GetFileInformationByHandle call initialized the structure.
        let information = unsafe { information.assume_init() };
        Ok(format!(
            "windows:{}:{}:{}",
            information.volume_serial_number,
            information.file_index_high,
            information.file_index_low
        ))
    }
    #[cfg(not(any(unix, windows)))]
    {
        Ok(format!(
            "other:{}",
            digest_bytes(path.as_os_str().to_string_lossy().as_bytes())
        ))
    }
}

fn workspace_result(report: &EnvironmentReadinessReport) -> ApplicationReadinessWorkspaceResult {
    ApplicationReadinessWorkspaceResult {
        workspace_id: report.workspace_id.clone(),
        requirements_digest: report.requirements_digest.clone(),
        report_id: report.report_id.clone(),
        aggregate_status: report.aggregate_status,
    }
}

fn application_aggregate_status(
    composition: ApplicationReadinessCompositionStatus,
    workspaces: &[ApplicationReadinessWorkspaceResult],
) -> ReadinessAggregateStatus {
    if composition == ApplicationReadinessCompositionStatus::Stale
        || workspaces
            .iter()
            .any(|workspace| workspace.aggregate_status == ReadinessAggregateStatus::Stale)
    {
        ReadinessAggregateStatus::Stale
    } else if workspaces
        .iter()
        .any(|workspace| workspace.aggregate_status == ReadinessAggregateStatus::Blocked)
    {
        ReadinessAggregateStatus::Blocked
    } else if workspaces
        .iter()
        .any(|workspace| workspace.aggregate_status == ReadinessAggregateStatus::Incomplete)
    {
        ReadinessAggregateStatus::Incomplete
    } else if workspaces
        .iter()
        .any(|workspace| workspace.aggregate_status == ReadinessAggregateStatus::Unsupported)
    {
        ReadinessAggregateStatus::Unsupported
    } else {
        ReadinessAggregateStatus::Ready
    }
}

fn result_class_for_aggregate(status: ReadinessAggregateStatus) -> ResultClass {
    match status {
        ReadinessAggregateStatus::Ready => ResultClass::Success,
        ReadinessAggregateStatus::Unsupported => ResultClass::Unsupported,
        ReadinessAggregateStatus::Incomplete => ResultClass::Incomplete,
        ReadinessAggregateStatus::Stale => ResultClass::Stale,
        ReadinessAggregateStatus::Blocked => ResultClass::Blocked,
    }
}

fn application_readiness_result_diagnostic(result_class: ResultClass) -> Option<Diagnostic> {
    let (code, message) = match result_class {
        ResultClass::Success => return None,
        ResultClass::Unsupported => (
            "FERRIS-APPLICATION-READINESS-UNSUPPORTED",
            "A workspace readiness result is unsupported.",
        ),
        ResultClass::Incomplete => (
            "FERRIS-APPLICATION-READINESS-INCOMPLETE",
            "A workspace readiness result is incomplete.",
        ),
        ResultClass::Stale => (
            "FERRIS-APPLICATION-READINESS-STALE",
            "Application readiness input or workspace evidence is stale.",
        ),
        ResultClass::Blocked => (
            "FERRIS-APPLICATION-READINESS-BLOCKED",
            "A workspace readiness result is blocking.",
        ),
        _ => unreachable!("application readiness reports use frozen aggregate classes"),
    };
    Some(Diagnostic {
        code: code.to_owned(),
        severity: "error".to_owned(),
        result_class,
        message: message.to_owned(),
        source_digest: None,
        bounded_output: None,
        next_actions: vec!["Review the typed application readiness report.".to_owned()],
    })
}

fn application_readiness_report_identity(
    report: &ApplicationReadinessReport,
) -> Result<String, CoreError> {
    let mut value = serde_json::to_value(report).map_err(|_| application_readiness_internal())?;
    value
        .as_object_mut()
        .expect("typed application readiness report serializes as an object")
        .remove("report_id");
    let bytes = serde_json::to_vec(&value).map_err(|_| application_readiness_internal())?;
    Ok(format!(
        "application-readiness:{}",
        hex_digest(&Sha256::digest(bytes))
    ))
}

fn application_readiness_selection_identity(application_id: &str, request_digest: &str) -> String {
    invocation_identity(&[
        "selection",
        "doctor",
        "application-readiness",
        application_id,
        request_digest,
    ])
    .replacen("invocation:", "selection:", 1)
}

fn application_readiness_invocation_identity(
    selection_identity: &str,
    request_path: &Path,
) -> String {
    invocation_identity(&[
        "doctor",
        env!("CARGO_PKG_VERSION"),
        "application-readiness",
        selection_identity,
        &explicit_path_identity(request_path),
    ])
}

#[cfg(unix)]
fn explicit_path_identity(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    let mut material = b"ferris.explicit-path/unix/v1\0".to_vec();
    material.extend_from_slice(path.as_os_str().as_bytes());
    digest_bytes(&material)
}

#[cfg(windows)]
fn explicit_path_identity(path: &Path) -> String {
    use std::os::windows::ffi::OsStrExt;
    let mut material = b"ferris.explicit-path/windows/v1\0".to_vec();
    for unit in path.as_os_str().encode_wide() {
        material.extend_from_slice(&unit.to_le_bytes());
    }
    digest_bytes(&material)
}

#[cfg(not(any(unix, windows)))]
fn explicit_path_identity(path: &Path) -> String {
    let mut material = b"ferris.explicit-path/other/v1\0".to_vec();
    material.extend_from_slice(path.as_os_str().to_string_lossy().as_bytes());
    digest_bytes(&material)
}

fn aggregate_status_name(status: ReadinessAggregateStatus) -> &'static str {
    match status {
        ReadinessAggregateStatus::Ready => "ready",
        ReadinessAggregateStatus::Unsupported => "unsupported",
        ReadinessAggregateStatus::Incomplete => "incomplete",
        ReadinessAggregateStatus::Stale => "stale",
        ReadinessAggregateStatus::Blocked => "blocked",
    }
}

fn composition_status_name(status: ApplicationReadinessCompositionStatus) -> &'static str {
    match status {
        ApplicationReadinessCompositionStatus::Current => "current",
        ApplicationReadinessCompositionStatus::Stale => "stale",
    }
}

fn application_readiness_invalid(code: &str, message: impl Into<String>) -> CoreError {
    CoreError::new(
        ResultClass::Invalid,
        code,
        message,
        vec!["Correct the explicit application readiness inputs and retry.".to_owned()],
    )
}

fn application_readiness_internal() -> CoreError {
    CoreError::new(
        ResultClass::Internal,
        "FERRIS-APPLICATION-READINESS-INTERNAL",
        "Ferris could not construct the application readiness report.",
        vec!["Report this Ferris invariant failure.".to_owned()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/application-readiness")
            .join(name)
    }

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn from_fixture(label: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time after epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "ferris-application-readiness-{label}-{}-{nonce}",
                std::process::id()
            ));
            copy_directory(
                fixture(".")
                    .canonicalize()
                    .expect("canonical application readiness fixture")
                    .as_path(),
                &path,
            );
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn copy_directory(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("create fixture copy");
        for entry in fs::read_dir(source).expect("read fixture directory") {
            let entry = entry.expect("fixture entry");
            let target = destination.join(entry.file_name());
            if entry.file_type().expect("fixture type").is_dir() {
                copy_directory(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).expect("copy fixture file");
            }
        }
    }

    fn rewrite_request_digest(root: &Path, field: &str, digest: String) {
        let path = root.join("request-valid.json");
        let mut request: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).expect("read request")).expect("parse request");
        match field {
            "application" => request["application_definition"]["digest"] = digest.into(),
            "alpha" => request["workspaces"][0]["requirements_digest"] = digest.into(),
            _ => unreachable!("known fixture binding"),
        }
        fs::write(
            path,
            serde_json::to_vec_pretty(&request).expect("serialize request"),
        )
        .expect("write request");
    }

    fn read_json(path: &Path) -> serde_json::Value {
        serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
    }

    fn write_json(path: &Path, value: &serde_json::Value) -> Vec<u8> {
        let bytes = serde_json::to_vec_pretty(value).expect("serialize JSON");
        fs::write(path, &bytes).expect("write JSON");
        bytes
    }

    #[test]
    fn frozen_request_loads_passively_and_matches_exact_bindings() {
        let loaded =
            load_application_readiness_request(&fixture("request-valid.json")).expect("request");
        let application = load_passive_application(&loaded).expect("application");
        let workspaces = bind_workspaces(&loaded, &application).expect("workspaces");
        assert_eq!(workspaces.len(), 3);
        assert_eq!(workspaces[0].workspace_id, "ferris.fixture/alpha");
        assert_eq!(
            loaded.request.application_definition.digest,
            digest_bytes(&application.bytes)
        );
    }

    #[test]
    fn frozen_request_produces_deterministic_ready_composition() {
        let first = create_application_readiness(&fixture("request-valid.json")).expect("first");
        let second = create_application_readiness(&fixture("request-valid.json")).expect("second");
        assert_eq!(first, second);
        let report = first.record.expect("report");
        assert_eq!(report.aggregate_status, ReadinessAggregateStatus::Ready);
        assert_eq!(report.workspace_results.len(), 3);
        assert_eq!(
            application_readiness_report_identity(&report).expect("identity"),
            report.report_id
        );
    }

    #[test]
    fn application_precedence_is_fail_closed() {
        let workspace = |status| ApplicationReadinessWorkspaceResult {
            workspace_id: "ferris.test/workspace".to_owned(),
            requirements_digest: format!("sha256:{}", "1".repeat(64)),
            report_id: format!("report:{}", "2".repeat(64)),
            aggregate_status: status,
        };
        for (statuses, expected) in [
            (
                vec![
                    ReadinessAggregateStatus::Ready,
                    ReadinessAggregateStatus::Unsupported,
                ],
                ReadinessAggregateStatus::Unsupported,
            ),
            (
                vec![
                    ReadinessAggregateStatus::Blocked,
                    ReadinessAggregateStatus::Incomplete,
                ],
                ReadinessAggregateStatus::Blocked,
            ),
            (
                vec![
                    ReadinessAggregateStatus::Blocked,
                    ReadinessAggregateStatus::Stale,
                ],
                ReadinessAggregateStatus::Stale,
            ),
        ] {
            let workspaces = statuses.into_iter().map(workspace).collect::<Vec<_>>();
            assert_eq!(
                application_aggregate_status(
                    ApplicationReadinessCompositionStatus::Current,
                    &workspaces
                ),
                expected
            );
        }
        assert_eq!(
            application_aggregate_status(
                ApplicationReadinessCompositionStatus::Stale,
                &[workspace(ReadinessAggregateStatus::Ready)]
            ),
            ReadinessAggregateStatus::Stale
        );
    }

    #[test]
    fn request_or_definition_change_becomes_stale() {
        let directory = TestDirectory::from_fixture("stale");
        let request = directory.0.join("request-valid.json");
        let definition = directory.0.join("application.json");
        let envelope = create_application_readiness_with_hook(&request, || {
            let mut bytes = fs::read(&definition).expect("read definition");
            bytes.push(b' ');
            fs::write(&definition, bytes).expect("change definition");
        })
        .expect("readiness");
        assert_eq!(
            envelope.record.expect("report").composition_status,
            ApplicationReadinessCompositionStatus::Stale
        );
        assert_eq!(envelope.result_class, ResultClass::Stale);

        let replacement = TestDirectory::from_fixture("replacement");
        let request = replacement.0.join("request-valid.json");
        let definition = replacement.0.join("application.json");
        let original = fs::read(&definition).expect("read definition");
        let envelope = create_application_readiness_with_hook(&request, || {
            fs::remove_file(&definition).expect("remove definition");
            fs::write(&definition, original).expect("replace definition");
        })
        .expect("readiness");
        assert_eq!(
            envelope.record.expect("report").composition_status,
            ApplicationReadinessCompositionStatus::Stale
        );
    }

    #[test]
    fn binding_mismatches_have_stable_application_diagnostics() {
        let application_mismatch = TestDirectory::from_fixture("application-id");
        let application_path = application_mismatch.0.join("application.json");
        let mut application: serde_json::Value =
            serde_json::from_slice(&fs::read(&application_path).expect("read application"))
                .expect("parse application");
        application["application_id"] = "ferris.fixture/other-application".into();
        let bytes = serde_json::to_vec_pretty(&application).expect("serialize application");
        fs::write(&application_path, &bytes).expect("write application");
        rewrite_request_digest(&application_mismatch.0, "application", digest_bytes(&bytes));
        let error =
            create_application_readiness(&application_mismatch.0.join("request-valid.json"))
                .expect_err("application mismatch");
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-APPLICATION-READINESS-APPLICATION-ID-MISMATCH"
        );

        let workspace_mismatch = TestDirectory::from_fixture("workspace-id");
        let requirements_path = workspace_mismatch.0.join("requirements-alpha.json");
        let mut requirements: serde_json::Value =
            serde_json::from_slice(&fs::read(&requirements_path).expect("read requirements"))
                .expect("parse requirements");
        requirements["workspace_id"] = "ferris.fixture/other-workspace".into();
        let bytes = serde_json::to_vec_pretty(&requirements).expect("serialize requirements");
        fs::write(&requirements_path, &bytes).expect("write requirements");
        rewrite_request_digest(&workspace_mismatch.0, "alpha", digest_bytes(&bytes));
        let error = create_application_readiness(&workspace_mismatch.0.join("request-valid.json"))
            .expect_err("workspace mismatch");
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-APPLICATION-READINESS-WORKSPACE-ID-MISMATCH"
        );

        let unsupported = TestDirectory::from_fixture("unsupported-schema");
        let requirements_path = unsupported.0.join("requirements-alpha.json");
        let mut requirements: serde_json::Value =
            serde_json::from_slice(&fs::read(&requirements_path).expect("read requirements"))
                .expect("parse requirements");
        requirements["schema"] = "ferris.environment-requirements/v2".into();
        let bytes = serde_json::to_vec_pretty(&requirements).expect("serialize requirements");
        fs::write(&requirements_path, &bytes).expect("write requirements");
        rewrite_request_digest(&unsupported.0, "alpha", digest_bytes(&bytes));
        let error = create_application_readiness(&unsupported.0.join("request-valid.json"))
            .expect_err("unsupported schema");
        assert_eq!(error.result_class(), ResultClass::Invalid);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-APPLICATION-READINESS-REQUEST-INVALID"
        );
    }

    #[test]
    fn strict_and_path_controls_fail_closed() {
        assert!(!valid_relative_file("../Cargo.toml"));
        assert!(!valid_relative_file("alpha/../Cargo.toml"));
        assert!(!valid_relative_file("alpha\\Cargo.toml"));
        assert!(!valid_relative_file("CON/Cargo.toml"));
        assert!(!valid_relative_file("alpha./Cargo.toml"));
        let duplicate =
            br#"{"schema":"ferris.application-readiness-request/v1","schema":"duplicate"}"#;
        assert!(parse_strict_json(duplicate).is_err());

        let nested = TestDirectory::from_fixture("nested");
        let request_path = nested.0.join("request-valid.json");
        let application_path = nested.0.join("application.json");
        let mut request: serde_json::Value =
            serde_json::from_slice(&fs::read(&request_path).expect("read request"))
                .expect("parse request");
        let mut application: serde_json::Value =
            serde_json::from_slice(&fs::read(&application_path).expect("read application"))
                .expect("parse application");
        request["workspaces"][1]["manifest_path"] = "alpha/nested/Cargo.toml".into();
        application["workspaces"][1]["manifest_path"] = "alpha/nested/Cargo.toml".into();
        let application_bytes =
            serde_json::to_vec_pretty(&application).expect("serialize application");
        fs::write(&application_path, &application_bytes).expect("write application");
        request["application_definition"]["digest"] = digest_bytes(&application_bytes).into();
        fs::write(
            &request_path,
            serde_json::to_vec_pretty(&request).expect("serialize request"),
        )
        .expect("write request");
        let error = create_application_readiness(&request_path).expect_err("nested root");
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-APPLICATION-READINESS-WORKSPACE-ROOT-NESTED"
        );

        let alias = TestDirectory::from_fixture("hard-link");
        let request_path = alias.0.join("request-valid.json");
        let application_path = alias.0.join("application.json");
        fs::create_dir(alias.0.join("alpha-alias")).expect("create alias root");
        fs::hard_link(
            alias.0.join("alpha").join("Cargo.toml"),
            alias.0.join("alpha-alias").join("Cargo.toml"),
        )
        .expect("create manifest hard link");
        let mut request: serde_json::Value =
            serde_json::from_slice(&fs::read(&request_path).expect("read request"))
                .expect("parse request");
        let mut application: serde_json::Value =
            serde_json::from_slice(&fs::read(&application_path).expect("read application"))
                .expect("parse application");
        request["workspaces"][1]["manifest_path"] = "alpha-alias/Cargo.toml".into();
        application["workspaces"][1]["manifest_path"] = "alpha-alias/Cargo.toml".into();
        let application_bytes =
            serde_json::to_vec_pretty(&application).expect("serialize application");
        fs::write(&application_path, &application_bytes).expect("write application");
        request["application_definition"]["digest"] = digest_bytes(&application_bytes).into();
        fs::write(
            &request_path,
            serde_json::to_vec_pretty(&request).expect("serialize request"),
        )
        .expect("write request");
        let error = create_application_readiness(&request_path).expect_err("duplicate root alias");
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-APPLICATION-READINESS-WORKSPACE-ROOT-DUPLICATE"
        );
    }

    #[test]
    fn frozen_request_controls_fail_at_the_intended_gate() {
        let cases = [
            (
                "duplicate-workspace",
                Box::new(|root: &Path| {
                    let request_path = root.join("request-valid.json");
                    let mut request = read_json(&request_path);
                    request["workspaces"][1]["workspace_id"] = "ferris.fixture/alpha".into();
                    write_json(&request_path, &request);
                }) as Box<dyn Fn(&Path)>,
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            ),
            (
                "traversal-manifest",
                Box::new(|root: &Path| {
                    let request_path = root.join("request-valid.json");
                    let mut request = read_json(&request_path);
                    request["workspaces"][0]["manifest_path"] = "../Cargo.toml".into();
                    write_json(&request_path, &request);
                }),
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            ),
            (
                "traversal-definition",
                Box::new(|root: &Path| {
                    let request_path = root.join("request-valid.json");
                    let mut request = read_json(&request_path);
                    request["application_definition"]["path"] = "..".into();
                    write_json(&request_path, &request);
                }),
                "FERRIS-APPLICATION-READINESS-REQUEST-INVALID",
            ),
            (
                "requirements-digest",
                Box::new(|root: &Path| {
                    let path = root.join("requirements-alpha.json");
                    let mut bytes = fs::read(&path).expect("read requirements");
                    bytes.push(b' ');
                    fs::write(path, bytes).expect("append requirements byte");
                }),
                "FERRIS-APPLICATION-READINESS-REQUIREMENTS-DIGEST-MISMATCH",
            ),
            (
                "missing-workspace",
                Box::new(|root: &Path| {
                    let request_path = root.join("request-valid.json");
                    let mut request = read_json(&request_path);
                    request["workspaces"]
                        .as_array_mut()
                        .expect("workspace array")
                        .pop();
                    write_json(&request_path, &request);
                }),
                "FERRIS-APPLICATION-READINESS-WORKSPACE-COVERAGE-MISMATCH",
            ),
            (
                "additional-workspace",
                Box::new(|root: &Path| {
                    let application_path = root.join("application.json");
                    let mut application = read_json(&application_path);
                    application["workspaces"]
                        .as_array_mut()
                        .expect("workspace array")
                        .push(serde_json::json!({
                            "workspace_id": "ferris.fixture/delta",
                            "manifest_path": "delta/Cargo.toml"
                        }));
                    let bytes = write_json(&application_path, &application);
                    rewrite_request_digest(root, "application", digest_bytes(&bytes));
                }),
                "FERRIS-APPLICATION-READINESS-WORKSPACE-COVERAGE-MISMATCH",
            ),
        ];

        for (label, mutate, expected) in cases {
            let directory = TestDirectory::from_fixture(label);
            mutate(&directory.0);
            let error = create_application_readiness(&directory.0.join("request-valid.json"))
                .expect_err(label);
            assert_eq!(error.diagnostic().code, expected, "{label}");
        }
    }
}
