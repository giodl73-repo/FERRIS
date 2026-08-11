use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const COMMAND_RESULT_SCHEMA: &str = "ferris.command-result/v0";
pub const PLAN_SCHEMA: &str = "ferris.blueprint-plan/v0";
pub const EXPLANATION_SCHEMA: &str = "ferris.explanation/v0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultClass {
    Success,
    Difference,
    Invalid,
    Denied,
    Unsupported,
    Incomplete,
    Stale,
    Blocked,
    Cancelled,
    Partial,
    Failed,
    Internal,
}

impl ResultClass {
    pub const fn exit_code(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Difference => 1,
            Self::Invalid => 2,
            Self::Denied => 3,
            Self::Unsupported => 4,
            Self::Incomplete => 5,
            Self::Stale => 6,
            Self::Blocked => 7,
            Self::Cancelled => 8,
            Self::Partial => 9,
            Self::Failed => 10,
            Self::Internal => 11,
        }
    }
}

impl fmt::Display for ResultClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = serde_json::to_value(self).map_err(|_| fmt::Error)?;
        formatter.write_str(value.as_str().ok_or(fmt::Error)?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub severity: String,
    pub result_class: ResultClass,
    pub message: String,
    pub next_actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandEnvelope<T> {
    pub schema: String,
    pub command_version: String,
    pub semantic_command_id: String,
    pub invocation_identity: String,
    pub result_class: ResultClass,
    pub diagnostics: Vec<Diagnostic>,
    pub record: Option<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceSource {
    pub owner: String,
    pub command: Vec<String>,
    pub metadata_format_version: u64,
    pub offline: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PackageRecord {
    pub identity: String,
    pub name: String,
    pub version: String,
    pub manifest_path: String,
    pub workspace_member: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanRecord {
    pub schema: String,
    pub plan_id: String,
    pub executable: bool,
    pub selected_manifest: String,
    pub workspace_root: String,
    pub packages: Vec<PackageRecord>,
    pub evidence: EvidenceSource,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExplanationRecord {
    pub schema: String,
    pub plan_id: String,
    pub selected: Vec<String>,
    pub reasons: Vec<String>,
    pub omitted: Vec<String>,
    pub unknowns: Vec<String>,
    pub evidence_owner: String,
    pub fallback: String,
    pub change_evidence: String,
}

#[derive(Debug)]
pub struct CoreError {
    class: ResultClass,
    diagnostic: Diagnostic,
}

impl CoreError {
    fn new(
        class: ResultClass,
        code: &str,
        message: impl Into<String>,
        next_actions: Vec<String>,
    ) -> Self {
        Self {
            class,
            diagnostic: Diagnostic {
                code: code.to_owned(),
                severity: "error".to_owned(),
                result_class: class,
                message: message.into(),
                next_actions,
            },
        }
    }

    pub const fn result_class(&self) -> ResultClass {
        self.class
    }

    pub fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.diagnostic.code, self.diagnostic.message
        )
    }
}

impl std::error::Error for CoreError {}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
    workspace_members: Vec<String>,
    workspace_root: String,
    version: Option<u64>,
}

#[derive(Deserialize)]
struct CargoPackage {
    id: String,
    name: String,
    version: String,
    manifest_path: String,
}

pub fn create_plan(manifest_path: &Path) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    create_plan_with_cargo(manifest_path, Path::new("cargo"))
}

fn create_plan_with_cargo(
    manifest_path: &Path,
    cargo_program: &Path,
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    let invocation_identity = invocation_identity_for_path(manifest_path);
    let manifest_path = canonical_manifest_path(manifest_path)?;
    let output = Command::new(cargo_program)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
            "--locked",
            "--manifest-path",
        ])
        .arg(&manifest_path)
        .output()
        .map_err(|error| {
            CoreError::new(
                ResultClass::Blocked,
                "FERRIS-CARGO-UNAVAILABLE",
                format!("Cargo metadata could not start: {error}"),
                vec![
                    "Install Cargo 1.95.0 or make it available on PATH.".to_owned(),
                    "Run the recorded cargo metadata command directly.".to_owned(),
                ],
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let (class, code) = classify_cargo_failure(&stderr);
        return Err(CoreError::new(
            class,
            code,
            if stderr.is_empty() {
                "Cargo metadata failed without a diagnostic.".to_owned()
            } else {
                stderr
            },
            vec![
                "Run cargo metadata with the same manifest and offline flags.".to_owned(),
                "Repair the owner manifest or make required offline sources available.".to_owned(),
            ],
        ));
    }

    plan_from_metadata(&manifest_path, invocation_identity, &output.stdout)
}

pub fn create_explanation(
    manifest_path: &Path,
) -> Result<CommandEnvelope<ExplanationRecord>, CoreError> {
    let plan = create_plan(manifest_path)?;
    let invocation_identity = plan.invocation_identity.clone();
    let plan_record = plan.record.ok_or_else(|| {
        CoreError::new(
            ResultClass::Internal,
            "FERRIS-PLAN-MISSING",
            "A successful plan envelope did not contain a plan.",
            vec!["Report this Ferris invariant failure.".to_owned()],
        )
    })?;

    let selected = plan_record
        .packages
        .iter()
        .filter(|package| package.workspace_member)
        .map(|package| format!("{} {}", package.name, package.version))
        .collect::<Vec<_>>();

    let explanation = ExplanationRecord {
        schema: EXPLANATION_SCHEMA.to_owned(),
        plan_id: plan_record.plan_id,
        selected,
        reasons: vec![
            "Cargo reported the package as a member of the explicitly selected workspace."
                .to_owned(),
            "The pulse permits planning only; no owner action was requested.".to_owned(),
        ],
        omitted: vec![
            "Dependency resolution and non-workspace dependency package detail were not requested because this command uses --no-deps."
                .to_owned(),
        ],
        unknowns: plan_record.unknowns,
        evidence_owner: "Cargo".to_owned(),
        fallback: "Run ordinary Cargo commands directly; Ferris has made no source or manifest change."
            .to_owned(),
        change_evidence:
            "A new explicit manifest selection or changed Cargo metadata would change this explanation."
                .to_owned(),
    };

    Ok(success_envelope(
        "explain",
        invocation_identity,
        explanation,
    ))
}

pub fn error_envelope<T>(
    semantic_command_id: &str,
    manifest_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T> {
    CommandEnvelope {
        schema: COMMAND_RESULT_SCHEMA.to_owned(),
        command_version: env!("CARGO_PKG_VERSION").to_owned(),
        semantic_command_id: semantic_command_id.to_owned(),
        invocation_identity: invocation_identity_for_path(manifest_path),
        result_class: error.result_class(),
        diagnostics: vec![error.diagnostic().clone()],
        record: None,
    }
}

pub fn render_plan_human(envelope: &CommandEnvelope<PlanRecord>) -> String {
    let plan = envelope.record.as_ref().expect("success plan has a record");
    let mut output = format!(
        "Ferris plan {}\nWorkspace: {}\nExecutable: no\nPackages:\n",
        plan.plan_id, plan.workspace_root
    );
    for package in plan
        .packages
        .iter()
        .filter(|package| package.workspace_member)
    {
        output.push_str(&format!("  - {} {}\n", package.name, package.version));
    }
    output.push_str("Evidence: Cargo metadata v1, offline, locked, no dependencies\n");
    output.push_str("Unknowns:\n");
    for unknown in &plan.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &plan.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output.push_str(
        "Next: use explain for selection reasons; run ordinary Cargo commands directly.\n",
    );
    output
}

pub fn render_explanation_human(envelope: &CommandEnvelope<ExplanationRecord>) -> String {
    let explanation = envelope
        .record
        .as_ref()
        .expect("success explanation has a record");
    let mut output = format!(
        "Ferris explanation for {}\nSelected:\n",
        explanation.plan_id
    );
    for selected in &explanation.selected {
        output.push_str(&format!("  - {selected}\n"));
    }
    output.push_str("Why:\n");
    for reason in &explanation.reasons {
        output.push_str(&format!("  - {reason}\n"));
    }
    output.push_str(&format!("Fallback: {}\n", explanation.fallback));
    output
}

pub fn render_error_human(error: &CoreError) -> String {
    let mut output = format!(
        "{} [{}]: {}\n",
        error.diagnostic.code, error.class, error.diagnostic.message
    );
    for action in &error.diagnostic.next_actions {
        output.push_str(&format!("  - {action}\n"));
    }
    output
}

fn canonical_manifest_path(manifest_path: &Path) -> Result<PathBuf, CoreError> {
    if !manifest_path.is_file() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-NOT-FOUND",
            format!(
                "The explicit manifest path is not a file: {}",
                manifest_path.display()
            ),
            vec!["Pass an existing Cargo.toml with --manifest-path.".to_owned()],
        ));
    }

    manifest_path.canonicalize().map_err(|error| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-UNREADABLE",
            format!(
                "The explicit manifest path could not be resolved ({}): {error}",
                manifest_path.display()
            ),
            vec!["Pass a readable Cargo.toml with --manifest-path.".to_owned()],
        )
    })
}

fn plan_from_metadata(
    manifest_path: &Path,
    invocation_identity: String,
    bytes: &[u8],
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| {
        CoreError::new(
            ResultClass::Internal,
            "FERRIS-CARGO-METADATA-MALFORMED",
            format!("Cargo returned malformed metadata JSON: {error}"),
            vec![
                "Run cargo metadata directly and retain its output.".to_owned(),
                "Report the Cargo and Ferris versions.".to_owned(),
            ],
        )
    })?;

    let version = value.get("version").and_then(serde_json::Value::as_u64);
    match version {
        Some(1) => {}
        Some(other) => {
            return Err(CoreError::new(
                ResultClass::Unsupported,
                "FERRIS-CARGO-METADATA-UNSUPPORTED",
                format!("Cargo metadata format version {other} is unsupported."),
                vec!["Use Cargo metadata format version 1.".to_owned()],
            ));
        }
        None => {
            return Err(CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-CARGO-METADATA-INCOMPLETE",
                "Cargo metadata did not declare its format version.",
                vec!["Retain complete Cargo metadata format version 1 output.".to_owned()],
            ));
        }
    }

    let metadata: CargoMetadata = serde_json::from_value(value).map_err(|error| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-CARGO-METADATA-INCOMPLETE",
            format!("Cargo metadata is missing a required planning field: {error}"),
            vec!["Retain complete Cargo metadata format version 1 output.".to_owned()],
        )
    })?;

    if metadata.version != Some(1) {
        return Err(CoreError::new(
            ResultClass::Internal,
            "FERRIS-CARGO-METADATA-VERSION-DRIFT",
            "Cargo metadata changed while it was being normalized.",
            vec!["Report this Ferris invariant failure.".to_owned()],
        ));
    }

    if metadata.workspace_members.is_empty() {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-CARGO-WORKSPACE-EMPTY",
            "Cargo metadata did not report a workspace member.",
            vec![
                "Select a Cargo manifest whose workspace contains at least one package.".to_owned(),
            ],
        ));
    }

    let workspace_root = PathBuf::from(&metadata.workspace_root);
    let selected_manifest = workspace_relative_path(manifest_path, &workspace_root)?;
    let mut packages = metadata
        .packages
        .into_iter()
        .map(|package| -> Result<PackageRecord, CoreError> {
            let workspace_member = metadata.workspace_members.contains(&package.id);
            let manifest_path =
                workspace_relative_path(Path::new(&package.manifest_path), &workspace_root)?;
            Ok(PackageRecord {
                identity: format!(
                    "cargo-workspace-package:{}@{}:{}",
                    package.name, package.version, manifest_path
                ),
                name: package.name,
                version: package.version,
                manifest_path,
                workspace_member,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    packages.sort_by(|left, right| left.identity.cmp(&right.identity));

    let plan_id = content_id(&(PLAN_SCHEMA, &selected_manifest, &packages))?;
    let record = PlanRecord {
        schema: PLAN_SCHEMA.to_owned(),
        plan_id,
        executable: false,
        selected_manifest: selected_manifest.clone(),
        workspace_root: ".".to_owned(),
        packages,
        evidence: EvidenceSource {
            owner: "Cargo".to_owned(),
            command: vec![
                "cargo".to_owned(),
                "metadata".to_owned(),
                "--format-version".to_owned(),
                "1".to_owned(),
                "--no-deps".to_owned(),
                "--offline".to_owned(),
                "--locked".to_owned(),
                "--manifest-path".to_owned(),
                selected_manifest.clone(),
            ],
            metadata_format_version: 1,
            offline: true,
        },
        unknowns: vec![
            "Dependency packages and resolved dependency edges are not observed in this pulse."
                .to_owned(),
            "Build scripts, native inputs, compiler units, freshness, and validation coverage are not observed."
                .to_owned(),
        ],
        limitations: vec![
            "This plan is non-executable.".to_owned(),
            "This pulse does not calculate affected-only scope.".to_owned(),
            "No correctness, performance, support, or conformance claim is made.".to_owned(),
        ],
    };

    Ok(success_envelope("plan", invocation_identity, record))
}

fn success_envelope<T>(
    semantic_command_id: &str,
    invocation_identity: String,
    record: T,
) -> CommandEnvelope<T> {
    CommandEnvelope {
        schema: COMMAND_RESULT_SCHEMA.to_owned(),
        command_version: env!("CARGO_PKG_VERSION").to_owned(),
        semantic_command_id: semantic_command_id.to_owned(),
        invocation_identity,
        result_class: ResultClass::Success,
        diagnostics: Vec::new(),
        record: Some(record),
    }
}

fn invocation_identity_for_path(path: &Path) -> String {
    invocation_identity(&normalize_path_text(&path.to_string_lossy()))
}

fn invocation_identity(selection: &str) -> String {
    let digest = Sha256::digest(selection.as_bytes());
    format!("invocation:{}", hex_digest(&digest))
}

fn workspace_relative_path(path: &Path, workspace_root: &Path) -> Result<String, CoreError> {
    let relative = path.strip_prefix(workspace_root).map_err(|_| {
        CoreError::new(
            ResultClass::Internal,
            "FERRIS-CARGO-PATH-INCONSISTENT",
            "Cargo reported a package or selected manifest outside its workspace root.",
            vec![
                "Run cargo metadata directly and retain its path fields.".to_owned(),
                "Report the Cargo and Ferris versions.".to_owned(),
            ],
        )
    })?;
    let normalized = normalize_path_text(&relative.to_string_lossy());
    Ok(if normalized.is_empty() {
        ".".to_owned()
    } else {
        normalized
    })
}

fn normalize_path_text(value: &str) -> String {
    value
        .strip_prefix(r"\\?\")
        .unwrap_or(value)
        .replace('\\', "/")
}

fn classify_cargo_failure(stderr: &str) -> (ResultClass, &'static str) {
    let lower = stderr.to_ascii_lowercase();
    let blocked = [
        "--locked",
        "lock file needs to be updated",
        "offline",
        "failed to get",
        "failed to load source for dependency",
        "no matching package named",
        "attempting to make an http request",
    ]
    .iter()
    .any(|indicator| lower.contains(indicator));
    if blocked {
        return (ResultClass::Blocked, "FERRIS-CARGO-METADATA-BLOCKED");
    }

    let invalid = [
        "failed to parse manifest",
        "failed to read manifest",
        "unclosed table",
        "invalid table header",
        "duplicate key",
        "missing field",
        "manifest path",
        "cargo.toml",
    ]
    .iter()
    .any(|indicator| lower.contains(indicator));
    if invalid {
        (ResultClass::Invalid, "FERRIS-MANIFEST-INVALID")
    } else {
        (ResultClass::Blocked, "FERRIS-CARGO-METADATA-BLOCKED")
    }
}

fn content_id<T: Serialize>(value: &T) -> Result<String, CoreError> {
    let bytes = serde_json::to_vec(value).map_err(|error| {
        CoreError::new(
            ResultClass::Internal,
            "FERRIS-CANONICAL-SERIALIZATION-FAILED",
            format!("Ferris could not serialize a canonical planning input: {error}"),
            vec!["Report this Ferris invariant failure.".to_owned()],
        )
    })?;
    let digest = Sha256::digest(bytes);
    Ok(format!("plan:{}", hex_digest(&digest)))
}

fn hex_digest(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/simple-workspace/Cargo.toml")
    }

    #[test]
    fn fixed_exit_codes_match_view_contract() {
        assert_eq!(ResultClass::Success.exit_code(), 0);
        assert_eq!(ResultClass::Difference.exit_code(), 1);
        assert_eq!(ResultClass::Invalid.exit_code(), 2);
        assert_eq!(ResultClass::Denied.exit_code(), 3);
        assert_eq!(ResultClass::Unsupported.exit_code(), 4);
        assert_eq!(ResultClass::Incomplete.exit_code(), 5);
        assert_eq!(ResultClass::Stale.exit_code(), 6);
        assert_eq!(ResultClass::Blocked.exit_code(), 7);
        assert_eq!(ResultClass::Cancelled.exit_code(), 8);
        assert_eq!(ResultClass::Partial.exit_code(), 9);
        assert_eq!(ResultClass::Failed.exit_code(), 10);
        assert_eq!(ResultClass::Internal.exit_code(), 11);
    }

    #[test]
    fn plan_is_stable_and_non_executable() {
        let first = create_plan(&manifest()).expect("first plan");
        let second = create_plan(&manifest()).expect("second plan");
        assert_eq!(first, second);
        let plan = first.record.expect("plan record");
        assert!(!plan.executable);
        assert_eq!(
            plan.packages
                .iter()
                .filter(|package| package.workspace_member)
                .map(|package| package.name.as_str())
                .collect::<Vec<_>>(),
            vec!["fixture-alpha", "fixture-beta"]
        );
    }

    #[test]
    fn explanation_uses_the_same_plan_identity() {
        let plan = create_plan(&manifest()).expect("plan");
        let explanation = create_explanation(&manifest()).expect("explanation");
        assert_eq!(
            plan.record.expect("plan record").plan_id,
            explanation.record.expect("explanation record").plan_id
        );
    }

    #[test]
    fn unsupported_metadata_version_is_explicit() {
        let error = plan_from_metadata(
            &manifest(),
            "invocation:test".to_owned(),
            br#"{"version":2,"packages":[],"workspace_members":[],"workspace_root":"x"}"#,
        )
        .expect_err("unsupported version");
        assert_eq!(error.result_class(), ResultClass::Unsupported);
        assert_eq!(error.diagnostic().code, "FERRIS-CARGO-METADATA-UNSUPPORTED");
    }

    #[test]
    fn missing_metadata_version_is_incomplete() {
        let error = plan_from_metadata(
            &manifest(),
            "invocation:test".to_owned(),
            br#"{"packages":[],"workspace_members":[],"workspace_root":"x"}"#,
        )
        .expect_err("missing version");
        assert_eq!(error.result_class(), ResultClass::Incomplete);
    }

    #[test]
    fn malformed_metadata_is_internal() {
        let error = plan_from_metadata(&manifest(), "invocation:test".to_owned(), b"{")
            .expect_err("malformed metadata should fail");
        assert_eq!(error.result_class(), ResultClass::Internal);
    }

    #[test]
    fn unavailable_cargo_is_blocked() {
        let error = create_plan_with_cargo(
            &manifest(),
            Path::new("ferris-definitely-missing-cargo-executable"),
        )
        .expect_err("missing cargo should fail");
        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(error.diagnostic().code, "FERRIS-CARGO-UNAVAILABLE");
    }

    #[test]
    fn plan_identity_is_independent_of_checkout_path() {
        fn metadata(root: &str) -> Vec<u8> {
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "workspace_root": root,
                "workspace_members": [format!("path+file:///{root}/member#fixture@0.1.0")],
                "packages": [{
                    "id": format!("path+file:///{root}/member#fixture@0.1.0"),
                    "name": "fixture",
                    "version": "0.1.0",
                    "manifest_path": format!("{root}/member/Cargo.toml")
                }]
            }))
            .expect("metadata JSON")
        }

        let first = plan_from_metadata(
            Path::new("checkout-a/Cargo.toml"),
            "invocation:a".to_owned(),
            &metadata("checkout-a"),
        )
        .expect("first plan");
        let second = plan_from_metadata(
            Path::new("checkout-b/Cargo.toml"),
            "invocation:b".to_owned(),
            &metadata("checkout-b"),
        )
        .expect("second plan");

        assert_eq!(
            first.record.expect("first record").plan_id,
            second.record.expect("second record").plan_id
        );
    }
}
