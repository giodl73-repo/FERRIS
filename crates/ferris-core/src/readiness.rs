use super::{
    CommandEnvelope, CoreError, Diagnostic, ResultClass, command_envelope, digest_bytes,
    hex_digest, invocation_identity,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub const ENVIRONMENT_REQUIREMENTS_SCHEMA: &str = "ferris.environment-requirements/v1";
pub const ENVIRONMENT_READINESS_REPORT_SCHEMA: &str = "ferris.environment-readiness-report/v1";
pub const MAX_ENVIRONMENT_REQUIREMENTS_BYTES: u64 = 1024 * 1024;
const READINESS_LIMITATION: &str = "FERRIS-READINESS-PRESENCE-DOES-NOT-PROVE-OWNER-SUCCESS";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessRequirementKind {
    Platform,
    Executable,
    Environment,
    Path,
}

impl ReadinessRequirementKind {
    fn diagnostic_name(self) -> &'static str {
        match self {
            Self::Platform => "PLATFORM",
            Self::Executable => "EXECUTABLE",
            Self::Environment => "ENVIRONMENT",
            Self::Path => "PATH",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessCriticality {
    Required,
    Advisory,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessObservationStatus {
    Satisfied,
    Missing,
    Mismatched,
    NotApplicable,
    Unsupported,
    Unavailable,
    NotObserved,
    Stale,
    Unknown,
}

impl ReadinessObservationStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Missing => "missing",
            Self::Mismatched => "mismatched",
            Self::NotApplicable => "not_applicable",
            Self::Unsupported => "unsupported",
            Self::Unavailable => "unavailable",
            Self::NotObserved => "not_observed",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    fn diagnostic_name(self) -> &'static str {
        match self {
            Self::Satisfied => "SATISFIED",
            Self::Missing => "MISSING",
            Self::Mismatched => "MISMATCHED",
            Self::NotApplicable => "NOT-APPLICABLE",
            Self::Unsupported => "UNSUPPORTED",
            Self::Unavailable => "UNAVAILABLE",
            Self::NotObserved => "NOT-OBSERVED",
            Self::Stale => "STALE",
            Self::Unknown => "UNKNOWN",
        }
    }

    fn observation_method(self, kind: ReadinessRequirementKind) -> ReadinessObservationMethod {
        if matches!(self, Self::Satisfied | Self::Missing | Self::Mismatched) {
            match kind {
                ReadinessRequirementKind::Platform => ReadinessObservationMethod::CurrentPlatform,
                ReadinessRequirementKind::Executable => {
                    ReadinessObservationMethod::ProcessPathResolution
                }
                ReadinessRequirementKind::Environment => {
                    ReadinessObservationMethod::EnvironmentNamePresence
                }
                ReadinessRequirementKind::Path => {
                    ReadinessObservationMethod::RepositoryPathMetadata
                }
            }
        } else {
            ReadinessObservationMethod::None
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessObservationMethod {
    CurrentPlatform,
    ProcessPathResolution,
    EnvironmentNamePresence,
    RepositoryPathMetadata,
    None,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessAggregateStatus {
    Ready,
    Unsupported,
    Incomplete,
    Stale,
    Blocked,
}

impl ReadinessAggregateStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Unsupported => "unsupported",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadinessObservedPlatform {
    pub operating_system: String,
    pub architecture: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadinessEvidence {
    pub observation_method: ReadinessObservationMethod,
    pub value_retained: bool,
    pub resolved_path_retained: bool,
    pub content_retained: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadinessObservation {
    pub requirement_id: String,
    pub kind: ReadinessRequirementKind,
    pub criticality: ReadinessCriticality,
    pub status: ReadinessObservationStatus,
    pub diagnostic_code: String,
    pub evidence: ReadinessEvidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadinessSourceDisposition {
    pub source_id: String,
    pub disposition: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnvironmentReadinessReport {
    pub schema: String,
    pub report_id: String,
    pub requirements_digest: String,
    pub workspace_id: String,
    pub observed_platform: ReadinessObservedPlatform,
    pub aggregate_status: ReadinessAggregateStatus,
    pub observations: Vec<ReadinessObservation>,
    pub source_dispositions: Vec<ReadinessSourceDisposition>,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentRequirements {
    schema: String,
    workspace_id: String,
    declaration_id: String,
    owner: String,
    sources: Vec<ReadinessSource>,
    requirements: Vec<RawRequirement>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadinessSource {
    source_id: String,
    kind: ReadinessSourceKind,
    claimed_authority: String,
    workspace_relative_path: Option<String>,
    content_digest: Option<String>,
    interpretation: ReadinessSourceInterpretation,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReadinessSourceKind {
    OwnerDeclaration,
    CargoMetadata,
    RustToolchain,
    AsdfToolVersions,
    Mise,
    Devcontainer,
    Devfile,
    Documentation,
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReadinessSourceInterpretation {
    DeclaredOnly,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRequirement {
    requirement_id: String,
    kind: ReadinessRequirementKind,
    criticality: ReadinessCriticality,
    applies_to: Option<ReadinessApplicability>,
    source_ids: Vec<String>,
    expectation: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadinessApplicability {
    operating_systems: Vec<DeclaredOperatingSystem>,
    architectures: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
enum DeclaredOperatingSystem {
    Windows,
    Linux,
    Macos,
}

impl DeclaredOperatingSystem {
    fn as_str(self) -> &'static str {
        match self {
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Macos => "macos",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlatformExpectation {
    operating_systems: Vec<DeclaredOperatingSystem>,
    architectures: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExecutableExpectation {
    name: String,
    resolution: ExecutableResolution,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ExecutableResolution {
    ProcessPath,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentExpectation {
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PathExpectation {
    workspace_relative_path: String,
    path_kind: ReadinessPathKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ReadinessPathKind {
    File,
    Directory,
}

#[derive(Clone, Debug)]
enum RequirementExpectation {
    Platform(PlatformExpectation),
    Executable(ExecutableExpectation),
    Environment(EnvironmentExpectation),
    Path(PathExpectation),
}

#[derive(Clone, Debug)]
struct ValidatedRequirement {
    requirement_id: String,
    kind: ReadinessRequirementKind,
    criticality: ReadinessCriticality,
    applies_to: Option<ReadinessApplicability>,
    expectation: RequirementExpectation,
}

#[derive(Debug)]
struct LoadedRequirements {
    declaration: EnvironmentRequirements,
    requirements: Vec<ValidatedRequirement>,
    digest: String,
}

struct ReadinessWorkspace {
    root: PathBuf,
    manifest_digest: String,
}

#[derive(Clone, Copy)]
struct ObservationPlatform<'a> {
    operating_system: &'a str,
    architecture: &'a str,
}

pub fn create_environment_readiness(
    manifest_path: &Path,
    workspace_id: &str,
    requirements_path: &Path,
) -> Result<CommandEnvelope<EnvironmentReadinessReport>, CoreError> {
    create_environment_readiness_with_hook(manifest_path, workspace_id, requirements_path, || {})
}

fn create_environment_readiness_with_hook(
    manifest_path: &Path,
    workspace_id: &str,
    requirements_path: &Path,
    before_revalidation: impl FnOnce(),
) -> Result<CommandEnvelope<EnvironmentReadinessReport>, CoreError> {
    let loaded = load_requirements(requirements_path)?;
    if loaded.declaration.workspace_id != workspace_id {
        return Err(readiness_invalid().with_invocation_selection(loaded.digest));
    }

    let workspace = load_readiness_workspace(manifest_path)
        .map_err(|error| error.with_invocation_selection(loaded.digest.clone()))?;
    let platform = current_platform();
    let mut observations = loaded
        .requirements
        .iter()
        .map(|requirement| observe_requirement(requirement, &workspace.root, platform))
        .collect::<Vec<_>>();
    before_revalidation();
    if read_requirements(requirements_path)
        .map(|bytes| digest_bytes(&bytes) != loaded.digest)
        .unwrap_or(true)
    {
        for observation in &mut observations {
            observation.status = ReadinessObservationStatus::Stale;
            observation.diagnostic_code = format!(
                "FERRIS-READINESS-{}-STALE",
                observation.kind.diagnostic_name()
            );
            observation.evidence.observation_method = ReadinessObservationMethod::None;
        }
    }
    let aggregate_status = aggregate_status(&observations);
    let source_dispositions = loaded
        .declaration
        .sources
        .iter()
        .map(|source| ReadinessSourceDisposition {
            source_id: source.source_id.clone(),
            disposition: "declared_only".to_owned(),
        })
        .collect();
    let mut report = EnvironmentReadinessReport {
        schema: ENVIRONMENT_READINESS_REPORT_SCHEMA.to_owned(),
        report_id: String::new(),
        requirements_digest: loaded.digest.clone(),
        workspace_id: workspace_id.to_owned(),
        observed_platform: ReadinessObservedPlatform {
            operating_system: platform.operating_system.to_owned(),
            architecture: platform.architecture.to_owned(),
        },
        aggregate_status,
        observations,
        source_dispositions,
        unknowns: Vec::new(),
        limitations: vec![READINESS_LIMITATION.to_owned()],
    };
    report.report_id = readiness_report_identity(&report)?;

    let selection_identity =
        readiness_selection_identity(workspace_id, &workspace.manifest_digest, &loaded.digest);
    let invocation_identity = readiness_invocation_identity(
        workspace_id,
        &selection_identity,
        manifest_path,
        requirements_path,
    );
    let result_class = result_class_for_aggregate(aggregate_status);
    let diagnostics = readiness_result_diagnostic(result_class)
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

pub fn environment_readiness_error_envelope<T>(
    workspace_id: &str,
    manifest_path: &Path,
    requirements_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let requirements_material = error
        .invocation_selection()
        .map(str::to_owned)
        .unwrap_or_else(|| explicit_path_identity(requirements_path));
    let manifest_material = explicit_path_identity(manifest_path);
    let selection_identity =
        readiness_selection_identity(workspace_id, &manifest_material, &requirements_material);
    command_envelope(
        "doctor",
        selection_identity.clone(),
        readiness_invocation_identity(
            workspace_id,
            &selection_identity,
            manifest_path,
            requirements_path,
        ),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn render_environment_readiness_human(
    envelope: &CommandEnvelope<EnvironmentReadinessReport>,
) -> String {
    let report = envelope
        .record
        .as_ref()
        .expect("readiness envelope has a report");
    let mut output = format!(
        "Ferris environment readiness {}\nWorkspace ID: {}\nRequirements digest: {}\nPlatform: {}/{}\nAggregate: {}\nObservations:\n",
        report.report_id,
        report.workspace_id,
        report.requirements_digest,
        report.observed_platform.operating_system,
        report.observed_platform.architecture,
        report.aggregate_status.as_str()
    );
    for observation in &report.observations {
        output.push_str(&format!(
            "  - {}: {} ({})\n",
            observation.requirement_id,
            observation.status.as_str(),
            observation.diagnostic_code
        ));
    }
    output.push_str("Source dispositions:\n");
    for source in &report.source_dispositions {
        output.push_str(&format!(
            "  - {}: {}\n",
            source.source_id, source.disposition
        ));
    }
    output.push_str("Unknowns:\n");
    if report.unknowns.is_empty() {
        output.push_str("  - none\n");
    } else {
        for unknown in &report.unknowns {
            output.push_str(&format!("  - {unknown}\n"));
        }
    }
    output.push_str("Limitations:\n");
    for limitation in &report.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

fn load_requirements(path: &Path) -> Result<LoadedRequirements, CoreError> {
    let bytes = read_requirements(path)?;
    let digest = digest_bytes(&bytes);
    let value = serde_json::from_slice::<super::StrictJsonValue>(&bytes)
        .map(super::StrictJsonValue::into_inner)
        .map_err(|_| readiness_invalid().with_invocation_selection(digest.clone()))?;
    match value.get("schema").and_then(serde_json::Value::as_str) {
        Some(ENVIRONMENT_REQUIREMENTS_SCHEMA) => {}
        Some(schema) if schema.starts_with("ferris.environment-requirements/") => {
            return Err(readiness_unsupported().with_invocation_selection(digest));
        }
        _ => return Err(readiness_invalid().with_invocation_selection(digest)),
    }
    if contains_json_null(&value) {
        return Err(readiness_invalid().with_invocation_selection(digest));
    }
    let declaration: EnvironmentRequirements = serde_json::from_value(value)
        .map_err(|_| readiness_invalid().with_invocation_selection(digest.clone()))?;
    let requirements = validate_requirements(&declaration)
        .map_err(|error| error.with_invocation_selection(digest.clone()))?;
    Ok(LoadedRequirements {
        declaration,
        requirements,
        digest,
    })
}

fn contains_json_null(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
        serde_json::Value::Array(values) => values.iter().any(contains_json_null),
        serde_json::Value::Object(values) => values.values().any(contains_json_null),
        _ => false,
    }
}

fn read_requirements(path: &Path) -> Result<Vec<u8>, CoreError> {
    let file = File::open(path).map_err(|_| readiness_unavailable())?;
    let mut bytes = Vec::new();
    file.take(MAX_ENVIRONMENT_REQUIREMENTS_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| readiness_unavailable())?;
    if bytes.len() as u64 > MAX_ENVIRONMENT_REQUIREMENTS_BYTES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-READINESS-REQUIREMENTS-BOUND-EXCEEDED",
            "The requirements declaration exceeds the supported size bound.",
            vec!["Reduce the requirements declaration below the documented bound.".to_owned()],
        ));
    }
    Ok(bytes)
}

fn validate_requirements(
    declaration: &EnvironmentRequirements,
) -> Result<Vec<ValidatedRequirement>, CoreError> {
    if declaration.schema != ENVIRONMENT_REQUIREMENTS_SCHEMA
        || !valid_workspace_id(&declaration.workspace_id)
        || !valid_identifier(&declaration.declaration_id)
        || !valid_identifier(&declaration.owner)
        || !(1..=64).contains(&declaration.sources.len())
        || !(1..=256).contains(&declaration.requirements.len())
        || !strictly_sorted_by(&declaration.sources, |source| source.source_id.as_str())
        || !strictly_sorted_by(&declaration.requirements, |requirement| {
            requirement.requirement_id.as_str()
        })
    {
        return Err(readiness_invalid());
    }

    let mut source_ids = BTreeSet::new();
    for source in &declaration.sources {
        let _ = source.kind;
        let _ = source.interpretation;
        if !valid_identifier(&source.source_id)
            || source.claimed_authority != declaration.owner
            || !source_ids.insert(source.source_id.as_str())
            || source
                .workspace_relative_path
                .as_deref()
                .is_some_and(|path| !valid_workspace_relative_path(path, false))
            || source
                .content_digest
                .as_deref()
                .is_some_and(|digest| !valid_digest(digest))
        {
            return Err(readiness_invalid());
        }
    }

    let mut validated = Vec::with_capacity(declaration.requirements.len());
    let mut path_expectations = BTreeMap::new();
    for raw in &declaration.requirements {
        if !valid_identifier(&raw.requirement_id)
            || !(1..=64).contains(&raw.source_ids.len())
            || !strictly_sorted_strings(&raw.source_ids)
            || raw
                .source_ids
                .iter()
                .any(|source_id| !source_ids.contains(source_id.as_str()))
            || raw
                .applies_to
                .as_ref()
                .is_some_and(|applicability| !valid_applicability(applicability))
        {
            return Err(readiness_invalid());
        }

        let expectation = match raw.kind {
            ReadinessRequirementKind::Platform => {
                let expectation: PlatformExpectation =
                    serde_json::from_value(raw.expectation.clone())
                        .map_err(|_| readiness_invalid())?;
                if !valid_operating_systems(&expectation.operating_systems)
                    || expectation
                        .architectures
                        .as_ref()
                        .is_some_and(|values| !valid_architectures(values))
                {
                    return Err(readiness_invalid());
                }
                RequirementExpectation::Platform(expectation)
            }
            ReadinessRequirementKind::Executable => {
                let expectation: ExecutableExpectation =
                    serde_json::from_value(raw.expectation.clone())
                        .map_err(|_| readiness_invalid())?;
                let _ = expectation.resolution;
                if !valid_identifier(&expectation.name) {
                    return Err(readiness_invalid());
                }
                RequirementExpectation::Executable(expectation)
            }
            ReadinessRequirementKind::Environment => {
                let expectation: EnvironmentExpectation =
                    serde_json::from_value(raw.expectation.clone())
                        .map_err(|_| readiness_invalid())?;
                if !valid_environment_name(&expectation.name) {
                    return Err(readiness_invalid());
                }
                RequirementExpectation::Environment(expectation)
            }
            ReadinessRequirementKind::Path => {
                let expectation: PathExpectation = serde_json::from_value(raw.expectation.clone())
                    .map_err(|_| readiness_invalid())?;
                if !valid_workspace_relative_path(&expectation.workspace_relative_path, true)
                    || (expectation.workspace_relative_path == "."
                        && expectation.path_kind != ReadinessPathKind::Directory)
                {
                    return Err(readiness_invalid());
                }
                if let Some(existing) = path_expectations.insert(
                    expectation.workspace_relative_path.clone(),
                    expectation.path_kind,
                ) {
                    if existing != expectation.path_kind {
                        return Err(readiness_invalid());
                    }
                }
                RequirementExpectation::Path(expectation)
            }
        };
        validated.push(ValidatedRequirement {
            requirement_id: raw.requirement_id.clone(),
            kind: raw.kind,
            criticality: raw.criticality,
            applies_to: raw.applies_to.clone(),
            expectation,
        });
    }
    Ok(validated)
}

fn valid_workspace_id(value: &str) -> bool {
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

fn valid_identifier(value: &str) -> bool {
    (1..=128).contains(&value.len())
        && value
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
}

fn valid_digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn valid_environment_name(value: &str) -> bool {
    (1..=128).contains(&value.len())
        && value
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_uppercase() || byte == b'_')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_workspace_relative_path(value: &str, allow_root: bool) -> bool {
    if value == "." {
        return allow_root;
    }
    (1..=1024).contains(&value.len())
        && value.split('/').all(|component| {
            !component.is_empty()
                && component != "."
                && component != ".."
                && component.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-')
                })
        })
}

fn valid_applicability(value: &ReadinessApplicability) -> bool {
    valid_operating_systems(&value.operating_systems)
        && value
            .architectures
            .as_ref()
            .is_none_or(|values| valid_architectures(values))
}

fn valid_operating_systems(values: &[DeclaredOperatingSystem]) -> bool {
    (1..=3).contains(&values.len())
        && values.iter().copied().collect::<BTreeSet<_>>().len() == values.len()
}

fn valid_architectures(values: &[String]) -> bool {
    (1..=16).contains(&values.len())
        && values.iter().collect::<BTreeSet<_>>().len() == values.len()
        && values.iter().all(|value| {
            (1..=64).contains(&value.len())
                && value
                    .bytes()
                    .next()
                    .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
                && value.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'_' | b'+' | b'-')
                })
        })
}

fn strictly_sorted_strings(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn strictly_sorted_by<T>(values: &[T], key: impl Fn(&T) -> &str) -> bool {
    values.windows(2).all(|pair| key(&pair[0]) < key(&pair[1]))
}

fn load_readiness_workspace(manifest_path: &Path) -> Result<ReadinessWorkspace, CoreError> {
    if manifest_path.file_name().and_then(OsStr::to_str) != Some("Cargo.toml") {
        return Err(readiness_invalid());
    }
    let manifest = manifest_path
        .canonicalize()
        .map_err(|_| readiness_invalid())?;
    if !manifest.is_file() {
        return Err(readiness_invalid());
    }
    let manifest_bytes = super::read_bounded_doctor_manifest(&manifest)?;
    let root = manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(readiness_invalid)?;
    Ok(ReadinessWorkspace {
        root,
        manifest_digest: digest_bytes(&manifest_bytes),
    })
}

fn current_platform() -> ObservationPlatform<'static> {
    let operating_system = if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "other"
    };
    ObservationPlatform {
        operating_system,
        architecture: std::env::consts::ARCH,
    }
}

fn observe_requirement(
    requirement: &ValidatedRequirement,
    workspace_root: &Path,
    platform: ObservationPlatform<'_>,
) -> ReadinessObservation {
    let status = if !is_applicable(requirement.applies_to.as_ref(), platform) {
        ReadinessObservationStatus::NotApplicable
    } else {
        match &requirement.expectation {
            RequirementExpectation::Platform(expectation) => {
                observe_platform(expectation, platform)
            }
            RequirementExpectation::Executable(expectation) => {
                observe_executable(&expectation.name)
            }
            RequirementExpectation::Environment(expectation) => {
                observe_environment(&expectation.name)
            }
            RequirementExpectation::Path(expectation) => observe_repository_path(
                workspace_root,
                &expectation.workspace_relative_path,
                expectation.path_kind,
            ),
        }
    };
    ReadinessObservation {
        requirement_id: requirement.requirement_id.clone(),
        kind: requirement.kind,
        criticality: requirement.criticality,
        status,
        diagnostic_code: format!(
            "FERRIS-READINESS-{}-{}",
            requirement.kind.diagnostic_name(),
            status.diagnostic_name()
        ),
        evidence: ReadinessEvidence {
            observation_method: status.observation_method(requirement.kind),
            value_retained: false,
            resolved_path_retained: false,
            content_retained: false,
        },
    }
}

fn is_applicable(
    applicability: Option<&ReadinessApplicability>,
    platform: ObservationPlatform<'_>,
) -> bool {
    applicability.is_none_or(|applicability| {
        applicability
            .operating_systems
            .iter()
            .any(|operating_system| operating_system.as_str() == platform.operating_system)
            && applicability
                .architectures
                .as_ref()
                .is_none_or(|architectures| {
                    architectures
                        .iter()
                        .any(|architecture| architecture == platform.architecture)
                })
    })
}

fn observe_platform(
    expectation: &PlatformExpectation,
    platform: ObservationPlatform<'_>,
) -> ReadinessObservationStatus {
    if platform.operating_system == "other" {
        return ReadinessObservationStatus::Unsupported;
    }
    if !expectation
        .operating_systems
        .iter()
        .any(|operating_system| operating_system.as_str() == platform.operating_system)
        || expectation
            .architectures
            .as_ref()
            .is_some_and(|architectures| {
                !architectures
                    .iter()
                    .any(|architecture| architecture == platform.architecture)
            })
    {
        ReadinessObservationStatus::Mismatched
    } else {
        ReadinessObservationStatus::Satisfied
    }
}

fn observe_executable(name: &str) -> ReadinessObservationStatus {
    let Some(path) = std::env::var_os("PATH") else {
        return ReadinessObservationStatus::Unavailable;
    };
    #[cfg(windows)]
    let path_extensions = std::env::var_os("PATHEXT");
    #[cfg(not(windows))]
    let path_extensions = None;
    observe_executable_in(name, &path, path_extensions.as_deref())
}

fn observe_executable_in(
    name: &str,
    search_path: &OsStr,
    path_extensions: Option<&OsStr>,
) -> ReadinessObservationStatus {
    let directories = std::env::split_paths(search_path)
        .filter(|path| !path.as_os_str().is_empty())
        .collect::<Vec<_>>();
    if directories.iter().any(|path| !path.is_absolute()) {
        return ReadinessObservationStatus::Unsupported;
    }

    #[cfg(unix)]
    {
        let _ = path_extensions;
        for directory in directories {
            match executable_candidate_status(&directory.join(name), true) {
                CandidateStatus::Found => return ReadinessObservationStatus::Satisfied,
                CandidateStatus::Absent => {}
                CandidateStatus::Unavailable => return ReadinessObservationStatus::Unavailable,
            }
        }
        ReadinessObservationStatus::Missing
    }

    #[cfg(windows)]
    {
        let has_extension = Path::new(name).extension().is_some();
        let (extensions, extension_failure) = if has_extension {
            (Vec::new(), None)
        } else {
            match path_extensions {
                Some(path_extensions) => match path_extensions.to_str() {
                    Some(path_extensions) => (
                        path_extensions
                            .split(';')
                            .filter(|extension| !extension.is_empty())
                            .map(|extension| extension.to_ascii_uppercase())
                            .collect::<Vec<_>>(),
                        None,
                    ),
                    None => (Vec::new(), Some(ReadinessObservationStatus::Unsupported)),
                },
                None => (Vec::new(), Some(ReadinessObservationStatus::Unavailable)),
            }
        };
        for directory in directories {
            match executable_candidate_status(&directory.join(name), false) {
                CandidateStatus::Found => return ReadinessObservationStatus::Satisfied,
                CandidateStatus::Absent => {}
                CandidateStatus::Unavailable => return ReadinessObservationStatus::Unavailable,
            }
            if !has_extension && extension_failure.is_none() {
                for extension in &extensions {
                    match executable_candidate_status(
                        &directory.join(format!("{name}{extension}")),
                        false,
                    ) {
                        CandidateStatus::Found => return ReadinessObservationStatus::Satisfied,
                        CandidateStatus::Absent => {}
                        CandidateStatus::Unavailable => {
                            return ReadinessObservationStatus::Unavailable;
                        }
                    }
                }
            }
        }
        extension_failure.unwrap_or(ReadinessObservationStatus::Missing)
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (name, directories, path_extensions);
        ReadinessObservationStatus::Unsupported
    }
}

enum CandidateStatus {
    Found,
    Absent,
    Unavailable,
}

fn executable_candidate_status(path: &Path, require_execute_bit: bool) -> CandidateStatus {
    match fs::metadata(path) {
        Ok(metadata) if !metadata.is_file() => CandidateStatus::Absent,
        Ok(metadata) => {
            let _ = &metadata;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if require_execute_bit && metadata.permissions().mode() & 0o111 == 0 {
                    return CandidateStatus::Absent;
                }
            }
            #[cfg(not(unix))]
            let _ = require_execute_bit;
            CandidateStatus::Found
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => CandidateStatus::Absent,
        Err(_) => CandidateStatus::Unavailable,
    }
}

fn observe_environment(name: &str) -> ReadinessObservationStatus {
    #[cfg(windows)]
    let present = std::env::vars_os().any(|(candidate, _)| {
        candidate
            .to_str()
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
    });
    #[cfg(not(windows))]
    let present = std::env::var_os(name).is_some();
    if present {
        ReadinessObservationStatus::Satisfied
    } else {
        ReadinessObservationStatus::Missing
    }
}

fn observe_repository_path(
    workspace_root: &Path,
    relative_path: &str,
    expected_kind: ReadinessPathKind,
) -> ReadinessObservationStatus {
    if relative_path == "." {
        return match fs::symlink_metadata(workspace_root) {
            Ok(metadata) if metadata.is_dir() => ReadinessObservationStatus::Satisfied,
            Ok(_) => ReadinessObservationStatus::Mismatched,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                ReadinessObservationStatus::Missing
            }
            Err(_) => ReadinessObservationStatus::Unavailable,
        };
    }

    let components = relative_path.split('/').collect::<Vec<_>>();
    let mut current = workspace_root.to_path_buf();
    for (index, component) in components.iter().enumerate() {
        current.push(component);
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return ReadinessObservationStatus::Missing;
            }
            Err(_) => return ReadinessObservationStatus::Unavailable,
        };
        if is_symlink_or_reparse(&metadata) {
            return ReadinessObservationStatus::Unsupported;
        }
        if index + 1 != components.len() && !metadata.is_dir() {
            return ReadinessObservationStatus::Missing;
        }
        if index + 1 == components.len() {
            let matches = match expected_kind {
                ReadinessPathKind::File => metadata.is_file(),
                ReadinessPathKind::Directory => metadata.is_dir(),
            };
            return if matches {
                ReadinessObservationStatus::Satisfied
            } else {
                ReadinessObservationStatus::Mismatched
            };
        }
    }
    ReadinessObservationStatus::Unavailable
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

fn aggregate_status(observations: &[ReadinessObservation]) -> ReadinessAggregateStatus {
    let required = observations
        .iter()
        .filter(|observation| observation.criticality == ReadinessCriticality::Required);
    let statuses = required
        .map(|observation| observation.status)
        .collect::<Vec<_>>();
    if statuses.contains(&ReadinessObservationStatus::Stale) {
        ReadinessAggregateStatus::Stale
    } else if statuses.iter().any(|status| {
        matches!(
            status,
            ReadinessObservationStatus::Missing
                | ReadinessObservationStatus::Mismatched
                | ReadinessObservationStatus::Unavailable
        )
    }) {
        ReadinessAggregateStatus::Blocked
    } else if statuses.iter().any(|status| {
        matches!(
            status,
            ReadinessObservationStatus::NotObserved | ReadinessObservationStatus::Unknown
        )
    }) {
        ReadinessAggregateStatus::Incomplete
    } else if statuses.contains(&ReadinessObservationStatus::Unsupported) {
        ReadinessAggregateStatus::Unsupported
    } else {
        ReadinessAggregateStatus::Ready
    }
}

fn result_class_for_aggregate(aggregate: ReadinessAggregateStatus) -> ResultClass {
    match aggregate {
        ReadinessAggregateStatus::Ready => ResultClass::Success,
        ReadinessAggregateStatus::Unsupported => ResultClass::Unsupported,
        ReadinessAggregateStatus::Incomplete => ResultClass::Incomplete,
        ReadinessAggregateStatus::Stale => ResultClass::Stale,
        ReadinessAggregateStatus::Blocked => ResultClass::Blocked,
    }
}

fn readiness_result_diagnostic(result_class: ResultClass) -> Option<Diagnostic> {
    let (code, message) = match result_class {
        ResultClass::Success => return None,
        ResultClass::Unsupported => (
            "FERRIS-READINESS-UNSUPPORTED",
            "A required readiness observation is unsupported.",
        ),
        ResultClass::Incomplete => (
            "FERRIS-READINESS-INCOMPLETE",
            "A required readiness observation is incomplete.",
        ),
        ResultClass::Stale => (
            "FERRIS-READINESS-STALE",
            "A required readiness observation is stale.",
        ),
        ResultClass::Blocked => (
            "FERRIS-READINESS-BLOCKED",
            "A required readiness observation is blocking.",
        ),
        _ => unreachable!("readiness reports use the frozen aggregate classes"),
    };
    Some(Diagnostic {
        code: code.to_owned(),
        severity: "error".to_owned(),
        result_class,
        message: message.to_owned(),
        source_digest: None,
        bounded_output: None,
        next_actions: vec!["Review the typed readiness report.".to_owned()],
    })
}

fn readiness_report_identity(report: &EnvironmentReadinessReport) -> Result<String, CoreError> {
    let mut value = serde_json::to_value(report).map_err(|_| readiness_internal())?;
    value
        .as_object_mut()
        .expect("typed readiness report serializes as an object")
        .remove("report_id");
    let bytes = serde_json::to_vec(&value).map_err(|_| readiness_internal())?;
    Ok(format!("report:{}", hex_digest(&Sha256::digest(bytes))))
}

fn readiness_selection_identity(
    workspace_id: &str,
    manifest_identity: &str,
    requirements_digest: &str,
) -> String {
    invocation_identity(&[
        "selection",
        "doctor",
        workspace_id,
        manifest_identity,
        requirements_digest,
    ])
    .replacen("invocation:", "selection:", 1)
}

fn readiness_invocation_identity(
    workspace_id: &str,
    selection_identity: &str,
    manifest_path: &Path,
    requirements_path: &Path,
) -> String {
    let manifest_request_identity = explicit_path_identity(manifest_path);
    let requirements_request_identity = explicit_path_identity(requirements_path);
    invocation_identity(&[
        "doctor",
        env!("CARGO_PKG_VERSION"),
        workspace_id,
        selection_identity,
        &manifest_request_identity,
        &requirements_request_identity,
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

fn readiness_invalid() -> CoreError {
    CoreError::new(
        ResultClass::Invalid,
        "FERRIS-READINESS-REQUIREMENTS-INVALID",
        "The requirements declaration is invalid.",
        vec!["Correct the requirements declaration and retry.".to_owned()],
    )
}

fn readiness_unsupported() -> CoreError {
    CoreError::new(
        ResultClass::Unsupported,
        "FERRIS-READINESS-REQUIREMENTS-UNSUPPORTED",
        "The requirements declaration version is unsupported.",
        vec!["Use a supported environment requirements schema version.".to_owned()],
    )
}

fn readiness_unavailable() -> CoreError {
    CoreError::new(
        ResultClass::Blocked,
        "FERRIS-READINESS-REQUIREMENTS-UNAVAILABLE",
        "The requirements declaration is unavailable.",
        vec!["Make the requirements declaration readable and retry.".to_owned()],
    )
}

fn readiness_internal() -> CoreError {
    CoreError::new(
        ResultClass::Internal,
        "FERRIS-READINESS-INTERNAL",
        "Ferris could not construct the readiness report.",
        vec!["Report this Ferris invariant failure.".to_owned()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/environment-readiness")
            .join(name)
    }

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(label: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time after epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "ferris-readiness-{label}-{}-{nonce}",
                std::process::id()
            ));
            fs::create_dir(&path).expect("create test directory");
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn frozen_requirements_parse_strictly_and_retain_exact_digest() {
        let bytes = fs::read(fixture("requirements-valid.json")).expect("read requirements");
        let loaded = load_requirements(&fixture("requirements-valid.json")).expect("requirements");
        assert_eq!(loaded.digest, format!("sha256:{:x}", Sha256::digest(bytes)));
        assert_eq!(loaded.requirements.len(), 6);
    }

    #[test]
    fn frozen_report_identity_matches_contract_vector() {
        let bytes = fs::read(fixture("report-ready.json")).expect("read report");
        let report: EnvironmentReadinessReport =
            serde_json::from_slice(&bytes).expect("parse report");
        assert_eq!(
            readiness_report_identity(&report).expect("report identity"),
            report.report_id
        );
    }

    #[test]
    fn frozen_report_state_vectors_match_identity_aggregate_and_process_mappings() {
        let base: serde_json::Value =
            serde_json::from_slice(&fs::read(fixture("report-ready.json")).expect("read report"))
                .expect("parse report");
        let vectors: serde_json::Value = serde_json::from_slice(
            &fs::read(fixture("report-state-vectors.json")).expect("read vectors"),
        )
        .expect("parse vectors");
        for vector in vectors["vectors"].as_array().expect("vectors") {
            let mut value = base.clone();
            for mutation in vector["mutations"].as_array().expect("mutations") {
                let pointer = mutation["pointer"].as_str().expect("mutation pointer");
                *value.pointer_mut(pointer).expect("mutation target") = mutation["value"].clone();
            }
            let report: EnvironmentReadinessReport =
                serde_json::from_value(value).expect("parse mutated report");
            assert_eq!(
                readiness_report_identity(&report).expect("report identity"),
                vector["expected_report_id"].as_str().expect("report ID"),
                "{}",
                vector["id"]
            );
            assert_eq!(
                aggregate_status(&report.observations),
                report.aggregate_status,
                "{}",
                vector["id"]
            );
            let expected_class: ResultClass =
                serde_json::from_value(vector["expected_result_class"].clone())
                    .expect("result class");
            let result_class = result_class_for_aggregate(report.aggregate_status);
            assert_eq!(result_class, expected_class, "{}", vector["id"]);
            assert_eq!(
                result_class.exit_code(),
                vector["expected_process_exit_code"]
                    .as_u64()
                    .expect("exit code") as u8,
                "{}",
                vector["id"]
            );
        }
    }

    #[test]
    fn strict_input_rejects_duplicate_unsupported_invalid_and_oversized() {
        let base = fs::read(fixture("requirements-valid.json")).expect("read requirements");
        let duplicate = String::from_utf8(base.clone())
            .expect("UTF-8 fixture")
            .replacen(
                "  \"owner\": \"repository-owner\",",
                "  \"owner\": \"repository-owner\",\n  \"owner\": \"duplicate-owner\",",
                1,
            );
        let directory = TestDirectory::new("strict");
        let duplicate_path = directory.0.join("duplicate.json");
        fs::write(&duplicate_path, duplicate).expect("write duplicate");
        let error = load_requirements(&duplicate_path).expect_err("duplicate rejected");
        assert_eq!(error.result_class(), ResultClass::Invalid);

        let unsupported_path = directory.0.join("unsupported.json");
        fs::write(
            &unsupported_path,
            String::from_utf8(base.clone())
                .expect("UTF-8 fixture")
                .replacen(
                    "ferris.environment-requirements/v1",
                    "ferris.environment-requirements/v2",
                    1,
                ),
        )
        .expect("write unsupported");
        let error = load_requirements(&unsupported_path).expect_err("unsupported rejected");
        assert_eq!(error.result_class(), ResultClass::Unsupported);

        let invalid_path = directory.0.join("invalid.json");
        fs::write(&invalid_path, b"{").expect("write invalid");
        assert_eq!(
            load_requirements(&invalid_path)
                .expect_err("invalid rejected")
                .result_class(),
            ResultClass::Invalid
        );

        let oversized_path = directory.0.join("oversized.json");
        fs::write(
            &oversized_path,
            vec![b' '; MAX_ENVIRONMENT_REQUIREMENTS_BYTES as usize + 1],
        )
        .expect("write oversized");
        let error = load_requirements(&oversized_path).expect_err("oversized rejected");
        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-READINESS-REQUIREMENTS-BOUND-EXCEEDED"
        );
    }

    #[test]
    fn frozen_requirements_controls_have_exact_result_classes() {
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(fixture("controls.json")).expect("read controls"))
                .expect("parse controls");
        let base = fs::read(fixture(
            manifest["requirements_base"]
                .as_str()
                .expect("requirements base"),
        ))
        .expect("read requirements base");
        let directory = TestDirectory::new("controls");

        for control in manifest["controls"].as_array().expect("controls") {
            if control["target"] != "requirements" {
                continue;
            }
            let id = control["id"].as_str().expect("control ID");
            let operation = control["operation"].as_str().expect("operation");
            let candidate = match operation {
                "none" => base.clone(),
                "add" | "replace" => {
                    let mut value: serde_json::Value =
                        serde_json::from_slice(&base).expect("parse base");
                    let pointer = control["pointer"].as_str().expect("pointer");
                    let replacement = control.get("value").cloned().expect("control value");
                    if operation == "replace" {
                        *value
                            .pointer_mut(pointer)
                            .unwrap_or_else(|| panic!("{id}: replacement target {pointer}")) =
                            replacement;
                    } else {
                        let (parent_pointer, member) =
                            pointer.rsplit_once('/').expect("pointer member");
                        value
                            .pointer_mut(parent_pointer)
                            .and_then(serde_json::Value::as_object_mut)
                            .unwrap_or_else(|| panic!("{id}: object parent {parent_pointer}"))
                            .insert(member.to_owned(), replacement);
                    }
                    serde_json::to_vec_pretty(&value).expect("serialize mutation")
                }
                "raw-duplicate" => String::from_utf8(base.clone())
                    .expect("UTF-8 base")
                    .replacen(
                        "  \"owner\": \"repository-owner\",",
                        "  \"owner\": \"repository-owner\",\n  \"owner\": \"duplicate-owner\",",
                        1,
                    )
                    .into_bytes(),
                "truncate" => {
                    let byte_count = control["byte_count"].as_u64().expect("byte count") as usize;
                    base[..base.len() - byte_count].to_vec()
                }
                "pad-string" => {
                    let mut value: serde_json::Value =
                        serde_json::from_slice(&base).expect("parse base");
                    let minimum_bytes =
                        control["minimum_bytes"].as_u64().expect("minimum bytes") as usize;
                    let pointer = control["pointer"].as_str().expect("pointer");
                    *value.pointer_mut(pointer).expect("padding target") =
                        serde_json::Value::String("x".repeat(minimum_bytes));
                    serde_json::to_vec(&value).expect("serialize padded mutation")
                }
                _ => panic!("unsupported control operation {operation}"),
            };
            let candidate_path = directory.0.join(format!("{id}.json"));
            fs::write(&candidate_path, candidate).expect("write candidate");
            let actual = match load_requirements(&candidate_path) {
                Ok(_) => "valid",
                Err(error) => match error.result_class() {
                    ResultClass::Unsupported => "unsupported",
                    ResultClass::Blocked => "blocked",
                    ResultClass::Invalid => {
                        if control["expected"] == "invalid-semantic" {
                            "invalid-semantic"
                        } else {
                            "invalid"
                        }
                    }
                    result_class => panic!("unexpected result class {result_class:?}"),
                },
            };
            assert_eq!(
                actual,
                control["expected"].as_str().expect("expected class"),
                "{id}"
            );
        }
    }

    #[test]
    fn semantic_validation_enforces_ordering_authority_privacy_and_root_rules() {
        let base = fs::read(fixture("requirements-valid.json")).expect("read requirements");
        let base = serde_json::from_slice::<super::super::StrictJsonValue>(&base)
            .expect("strict fixture")
            .into_inner();
        let directory = TestDirectory::new("semantics");
        let mut cases = Vec::new();

        let mut unknown = base.clone();
        unknown["install"] = serde_json::json!(true);
        cases.push(("unknown", unknown));

        let mut retained_value = base.clone();
        retained_value["requirements"][1]["expectation"]["value"] =
            serde_json::json!("not-retained");
        cases.push(("retained-value", retained_value));

        let mut explicit_null = base.clone();
        explicit_null["sources"][0]["workspace_relative_path"] = serde_json::Value::Null;
        cases.push(("explicit-null", explicit_null));

        let mut authority = base.clone();
        authority["sources"][0]["claimed_authority"] = serde_json::json!("other-owner");
        cases.push(("authority", authority));

        let mut source_order = base.clone();
        source_order["sources"]
            .as_array_mut()
            .expect("sources")
            .swap(0, 1);
        cases.push(("source-order", source_order));

        let mut requirement_order = base.clone();
        requirement_order["requirements"]
            .as_array_mut()
            .expect("requirements")
            .swap(0, 1);
        cases.push(("requirement-order", requirement_order));

        let mut source_references = base.clone();
        source_references["requirements"][0]["source_ids"] =
            serde_json::json!(["doc-source", "doc-source"]);
        cases.push(("source-references", source_references));

        let mut root_file = base;
        root_file["requirements"][4]["expectation"]["workspace_relative_path"] =
            serde_json::json!(".");
        cases.push(("root-file", root_file));

        for (name, value) in cases {
            let path = directory.0.join(format!("{name}.json"));
            fs::write(
                &path,
                serde_json::to_vec(&value).expect("serialize mutation"),
            )
            .expect("write mutation");
            let error = load_requirements(&path).expect_err("mutation must be invalid");
            assert_eq!(error.result_class(), ResultClass::Invalid, "{name}");
            assert_eq!(
                error.diagnostic().code,
                "FERRIS-READINESS-REQUIREMENTS-INVALID",
                "{name}"
            );
        }
    }

    #[test]
    fn declaration_workspace_must_match_the_invocation() {
        let directory = TestDirectory::new("workspace-match");
        let manifest = directory.0.join("Cargo.toml");
        fs::write(&manifest, b"[workspace]\n").expect("write manifest");
        let error = create_environment_readiness(
            &manifest,
            "ferris.fixture/different",
            &fixture("requirements-valid.json"),
        )
        .expect_err("workspace mismatch");
        assert_eq!(error.result_class(), ResultClass::Invalid);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-READINESS-REQUIREMENTS-INVALID"
        );
    }

    #[test]
    fn path_observation_distinguishes_missing_and_kind_mismatch() {
        let directory = TestDirectory::new("paths");
        fs::write(directory.0.join("present"), b"content").expect("write file");
        assert_eq!(
            observe_repository_path(&directory.0, "absent", ReadinessPathKind::File),
            ReadinessObservationStatus::Missing
        );
        assert_eq!(
            observe_repository_path(&directory.0, "present", ReadinessPathKind::Directory),
            ReadinessObservationStatus::Mismatched
        );
    }

    #[cfg(unix)]
    #[test]
    fn unix_executable_resolution_requires_an_execute_bit_and_never_launches() {
        use std::os::unix::fs::PermissionsExt;
        let directory = TestDirectory::new("unix-executable");
        let executable = directory.0.join("probe");
        let marker = directory.0.join("launched");
        fs::write(
            &executable,
            format!("#!/bin/sh\nprintf launched > '{}'\n", marker.display()),
        )
        .expect("write executable");
        let mut permissions = fs::metadata(&executable).expect("metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&executable, permissions).expect("permissions");
        assert_eq!(
            observe_executable_in("probe", directory.0.as_os_str(), None),
            ReadinessObservationStatus::Satisfied
        );
        assert!(!marker.exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_executable_resolution_uses_pathext_and_never_launches() {
        let directory = TestDirectory::new("windows-executable");
        let executable = directory.0.join("probe.CMD");
        let marker = directory.0.join("launched");
        fs::write(
            &executable,
            format!("@echo launched>\"{}\"\r\n", marker.display()),
        )
        .expect("write executable");
        assert_eq!(
            observe_executable_in(
                "probe",
                directory.0.as_os_str(),
                Some(OsStr::new(".CMD;.EXE"))
            ),
            ReadinessObservationStatus::Satisfied
        );
        assert!(!marker.exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_executable_resolution_checks_exact_leaf_without_pathext() {
        let directory = TestDirectory::new("windows-executable-exact");
        fs::write(directory.0.join("probe"), b"not launched").expect("write exact candidate");
        assert_eq!(
            observe_executable_in("probe", directory.0.as_os_str(), None),
            ReadinessObservationStatus::Satisfied
        );
    }

    #[test]
    fn relative_process_path_is_unsupported() {
        assert_eq!(
            observe_executable_in("cargo", OsStr::new("relative"), None),
            ReadinessObservationStatus::Unsupported
        );
    }

    #[cfg(unix)]
    #[test]
    fn repository_symlink_components_are_unsupported() {
        use std::os::unix::fs::symlink;
        let directory = TestDirectory::new("symlink");
        let target = directory.0.join("target");
        fs::create_dir(&target).expect("create target");
        symlink(&target, directory.0.join("link")).expect("create symlink");
        assert_eq!(
            observe_repository_path(&directory.0, "link", ReadinessPathKind::Directory),
            ReadinessObservationStatus::Unsupported
        );
    }

    #[cfg(windows)]
    #[test]
    fn repository_symlink_components_are_unsupported_when_creation_is_available() {
        use std::os::windows::fs::symlink_dir;
        let directory = TestDirectory::new("symlink");
        let target = directory.0.join("target");
        fs::create_dir(&target).expect("create target");
        if symlink_dir(&target, directory.0.join("link")).is_ok() {
            assert_eq!(
                observe_repository_path(&directory.0, "link", ReadinessPathKind::Directory),
                ReadinessObservationStatus::Unsupported
            );
        }
    }

    #[test]
    fn command_identities_bind_manifest_content_and_explicit_input_paths() {
        let directory = TestDirectory::new("identity-inputs");
        let first_workspace = directory.0.join("first");
        let second_workspace = directory.0.join("second");
        fs::create_dir(&first_workspace).expect("create first workspace");
        fs::create_dir(&second_workspace).expect("create second workspace");
        let first_manifest = first_workspace.join("Cargo.toml");
        let second_manifest = second_workspace.join("Cargo.toml");
        fs::write(&first_manifest, b"[workspace]\n").expect("write first manifest");
        fs::write(&second_manifest, b"[workspace]\nresolver = \"2\"\n")
            .expect("write second manifest");

        let first_requirements = directory.0.join("first-requirements.json");
        let second_requirements = directory.0.join("second-requirements.json");
        let requirements = fs::read(fixture("requirements-valid.json")).expect("read fixture");
        fs::write(&first_requirements, &requirements).expect("write first requirements");
        fs::write(&second_requirements, &requirements).expect("write second requirements");

        let first = create_environment_readiness(
            &first_manifest,
            "ferris.fixture/readiness",
            &first_requirements,
        )
        .expect("first readiness");
        let changed_manifest = create_environment_readiness(
            &second_manifest,
            "ferris.fixture/readiness",
            &first_requirements,
        )
        .expect("changed manifest readiness");
        let changed_request_path = create_environment_readiness(
            &first_manifest,
            "ferris.fixture/readiness",
            &second_requirements,
        )
        .expect("changed request path readiness");

        assert_ne!(
            first.selection_identity,
            changed_manifest.selection_identity
        );
        assert_ne!(
            first.invocation_identity,
            changed_manifest.invocation_identity
        );
        assert_eq!(
            first.selection_identity,
            changed_request_path.selection_identity
        );
        assert_ne!(
            first.invocation_identity,
            changed_request_path.invocation_identity
        );
        assert_ne!(
            explicit_path_identity(Path::new("Cargo.toml")),
            explicit_path_identity(&Path::new(".").join("Cargo.toml"))
        );
    }

    #[test]
    fn declaration_change_during_observation_emits_a_stale_report() {
        let directory = TestDirectory::new("stale-input");
        let manifest = directory.0.join("Cargo.toml");
        fs::write(&manifest, b"[workspace]\n").expect("write manifest");
        let requirements_path = directory.0.join("requirements.json");
        let requirements = fs::read(fixture("requirements-valid.json")).expect("read fixture");
        fs::write(&requirements_path, &requirements).expect("write requirements");

        let envelope = create_environment_readiness_with_hook(
            &manifest,
            "ferris.fixture/readiness",
            &requirements_path,
            || {
                let mut changed = requirements.clone();
                changed.push(b'\n');
                fs::write(&requirements_path, changed).expect("replace requirements");
            },
        )
        .expect("stale readiness report");

        assert_eq!(envelope.result_class, ResultClass::Stale);
        assert_eq!(envelope.process_exit_code, 6);
        let report = envelope.record.expect("stale report");
        assert_eq!(report.aggregate_status, ReadinessAggregateStatus::Stale);
        assert!(report.observations.iter().all(|observation| {
            observation.status == ReadinessObservationStatus::Stale
                && observation.evidence.observation_method == ReadinessObservationMethod::None
                && observation.diagnostic_code.ends_with("-STALE")
        }));
    }

    #[test]
    fn aggregate_precedence_matches_view_contract() {
        let observation = |status| ReadinessObservation {
            requirement_id: format!("{status:?}"),
            kind: ReadinessRequirementKind::Path,
            criticality: ReadinessCriticality::Required,
            status,
            diagnostic_code: String::new(),
            evidence: ReadinessEvidence {
                observation_method: ReadinessObservationMethod::None,
                value_retained: false,
                resolved_path_retained: false,
                content_retained: false,
            },
        };
        assert_eq!(
            aggregate_status(&[
                observation(ReadinessObservationStatus::Unsupported),
                observation(ReadinessObservationStatus::Unknown),
                observation(ReadinessObservationStatus::Missing),
                observation(ReadinessObservationStatus::Stale),
            ]),
            ReadinessAggregateStatus::Stale
        );
    }
}
