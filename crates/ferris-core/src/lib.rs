use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread;
use std::time::{Duration, Instant};

pub const COMMAND_RESULT_SCHEMA: &str = "ferris.command-result/v0";
pub const PLAN_SCHEMA: &str = "ferris.blueprint-plan/v0";
pub const EXPLANATION_SCHEMA: &str = "ferris.explanation/v0";
pub const GRAPH_SCHEMA: &str = "ferris.workspace-graph/v0";
pub const DOCTOR_SCHEMA: &str = "ferris.doctor-report/v0";

const MAX_GRAPH_NODES: usize = 10_000;
const MAX_GRAPH_EDGES: usize = 50_000;
const MAX_DOCTOR_MANIFEST_BYTES: u64 = 1024 * 1024;
const MAX_DOCTOR_OUTPUT_BYTES: usize = 64 * 1024;
const DOCTOR_TIMEOUT: Duration = Duration::from_secs(5);

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
    pub source_digest: Option<String>,
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
    pub command_representation: String,
    pub working_directory: String,
    pub workspace_id: String,
    pub owner_output_digest: String,
    pub metadata_format_version: u64,
    pub offline: bool,
    pub rustup_auto_install: bool,
    pub toolchain_selection: String,
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
    pub workspace_id: String,
    pub executable: bool,
    pub selected_manifest: String,
    pub workspace_root: String,
    pub packages: Vec<PackageRecord>,
    pub evidence: EvidenceSource,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GraphNode {
    pub identity: String,
    pub name: String,
    pub version: String,
    pub manifest_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GraphEdge {
    pub identity: String,
    pub from: String,
    pub target: Option<String>,
    pub dependency_name: String,
    pub dependency_alias: Option<String>,
    pub kind: String,
    pub optional: bool,
    pub target_condition: Option<String>,
    pub resolution: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GraphRecord {
    pub schema: String,
    pub graph_id: String,
    pub workspace_id: String,
    pub executable: bool,
    pub selected_manifest: String,
    pub workspace_root: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub evidence: EvidenceSource,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExplanationRecord {
    pub schema: String,
    pub plan_id: String,
    pub workspace_id: String,
    pub selected: Vec<String>,
    pub reasons: Vec<String>,
    pub omitted: Vec<String>,
    pub unknowns: Vec<String>,
    pub evidence_owner: String,
    pub fallback: String,
    pub change_evidence: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DoctorCheck {
    pub check_id: String,
    pub status: String,
    pub summary: String,
    pub evidence_digest: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DoctorEvidence {
    pub owner: String,
    pub command: Vec<String>,
    pub command_representation: String,
    pub working_directory: String,
    pub owner_output_digest: String,
    pub network_requested: bool,
    pub owner_work_requested: bool,
    pub cargo_network_offline: bool,
    pub rustup_auto_install: bool,
    pub toolchain_selection: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DoctorBounds {
    pub manifest_max_bytes: u64,
    pub probe_timeout_millis: u64,
    pub stdout_max_bytes: usize,
    pub stderr_max_bytes: usize,
    pub owner_output_framing: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DoctorReport {
    pub schema: String,
    pub report_id: String,
    pub workspace_id: String,
    pub manifest_digest: String,
    pub cargo_version: String,
    pub cargo_commit: Option<String>,
    pub cargo_release_date: Option<String>,
    pub checks: Vec<DoctorCheck>,
    pub evidence: DoctorEvidence,
    pub bounds: DoctorBounds,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
    pub fallback: String,
}

#[derive(Debug)]
pub struct CoreError {
    class: ResultClass,
    diagnostic: Box<Diagnostic>,
    invocation_selection: Option<String>,
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
            diagnostic: Box::new(Diagnostic {
                code: code.to_owned(),
                severity: "error".to_owned(),
                result_class: class,
                message: message.into(),
                source_digest: None,
                next_actions,
            }),
            invocation_selection: None,
        }
    }

    fn with_source_digest(mut self, source_digest: String) -> Self {
        self.diagnostic.source_digest = Some(source_digest);
        self
    }

    fn with_invocation_selection(mut self, invocation_selection: String) -> Self {
        self.invocation_selection = Some(invocation_selection);
        self
    }

    pub const fn result_class(&self) -> ResultClass {
        self.class
    }

    pub fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    fn invocation_selection(&self) -> Option<&str> {
        self.invocation_selection.as_deref()
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
    #[serde(default)]
    dependencies: Vec<CargoDependency>,
}

#[derive(Deserialize)]
struct CargoDependency {
    name: String,
    rename: Option<String>,
    kind: Option<String>,
    optional: bool,
    target: Option<String>,
    path: Option<String>,
}

struct MetadataInvocation {
    manifest_path: PathBuf,
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct BoundedOutput {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

#[derive(Debug)]
enum BoundedCommandError {
    Start(io::Error),
    Wait(io::Error),
    Read,
    Timeout,
    OutputLimit,
}

#[derive(Debug)]
struct CargoVersionEvidence {
    version: String,
    commit: Option<String>,
    release_date: Option<String>,
}

#[derive(Serialize)]
struct DoctorIdentityInput<'a> {
    schema: &'a str,
    workspace_id: &'a str,
    manifest_digest: &'a str,
    cargo_version: &'a str,
    cargo_commit: &'a str,
    cargo_release_date: &'a str,
    owner_output_digest: &'a str,
    command: &'a str,
    working_directory: &'a str,
    cargo_version_probe: bool,
    network_requested: bool,
    owner_work_requested: bool,
    cargo_network_offline: bool,
    rustup_auto_install: bool,
    toolchain_selection: &'a str,
    manifest_max_bytes: u64,
    probe_timeout_millis: u64,
    stdout_max_bytes: usize,
    stderr_max_bytes: usize,
    owner_output_framing: &'a str,
}

pub fn create_plan(
    manifest_path: &Path,
    workspace_id: &str,
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    create_plan_with_cargo(manifest_path, workspace_id, Path::new("cargo"))
}

fn create_plan_with_cargo(
    manifest_path: &Path,
    workspace_id: &str,
    cargo_program: &Path,
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    validate_workspace_id(workspace_id)?;
    let invocation = load_cargo_metadata(manifest_path, cargo_program)?;
    plan_from_metadata(&invocation.manifest_path, workspace_id, &invocation.bytes)
}

pub fn create_graph(
    manifest_path: &Path,
    workspace_id: &str,
) -> Result<CommandEnvelope<GraphRecord>, CoreError> {
    validate_workspace_id(workspace_id)?;
    let invocation = load_cargo_metadata(manifest_path, Path::new("cargo"))?;
    graph_from_metadata(&invocation.manifest_path, workspace_id, &invocation.bytes)
}

pub fn create_doctor(
    manifest_path: &Path,
    workspace_id: &str,
) -> Result<CommandEnvelope<DoctorReport>, CoreError> {
    create_doctor_with_cargo(manifest_path, workspace_id, Path::new("cargo"))
}

fn create_doctor_with_cargo(
    manifest_path: &Path,
    workspace_id: &str,
    cargo_program: &Path,
) -> Result<CommandEnvelope<DoctorReport>, CoreError> {
    validate_workspace_id(workspace_id)?;
    let manifest_path = canonical_manifest_path(manifest_path)?;
    if manifest_path.file_name().and_then(|name| name.to_str()) != Some("Cargo.toml") {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-DOCTOR-MANIFEST-NAME-INVALID",
            "The passive doctor requires an explicit Cargo.toml manifest.",
            vec!["Pass an existing Cargo.toml with --manifest-path.".to_owned()],
        )
        .with_source_digest(digest_text(&normalize_request_path(&manifest_path))));
    }
    let manifest_bytes = read_bounded_doctor_manifest(&manifest_path)?;
    let manifest_digest = digest_bytes(&manifest_bytes);
    let working_directory = manifest_path.parent().ok_or_else(|| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-DOCTOR-MANIFEST-PARENT-INVALID",
            "The explicit Cargo.toml has no selectable parent directory.",
            vec!["Pass a Cargo.toml within a local directory.".to_owned()],
        )
        .with_invocation_selection(manifest_digest.clone())
    })?;

    let mut command = Command::new(cargo_program);
    configure_passive_cargo_probe(&mut command, working_directory);
    let output = run_bounded_command(&mut command, DOCTOR_TIMEOUT, MAX_DOCTOR_OUTPUT_BYTES)
        .map_err(|error| doctor_command_error(error, &manifest_digest))?;

    if !output.status.success() {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-BLOCKED",
            "The passive Cargo version probe did not succeed.",
            vec!["Run cargo --version directly and repair the local toolchain.".to_owned()],
        )
        .with_source_digest(digest_command_output(&output.stdout, &output.stderr))
        .with_invocation_selection(manifest_digest.clone()));
    }

    let cargo_evidence = decode_cargo_probe(&output.stdout, &output.stderr)
        .map_err(|error| error.with_invocation_selection(manifest_digest.clone()))?;
    let owner_output_digest = digest_command_output(&output.stdout, &output.stderr);
    let report_id = doctor_report_id(
        workspace_id,
        &manifest_digest,
        &cargo_evidence,
        &owner_output_digest,
    )?;
    let invocation_identity = doctor_invocation_identity(
        workspace_id,
        &manifest_digest,
        &cargo_evidence,
        &owner_output_digest,
    );
    let record = DoctorReport {
        schema: DOCTOR_SCHEMA.to_owned(),
        report_id,
        workspace_id: workspace_id.to_owned(),
        manifest_digest: manifest_digest.clone(),
        cargo_version: cargo_evidence.version.clone(),
        cargo_commit: cargo_evidence.commit.clone(),
        cargo_release_date: cargo_evidence.release_date.clone(),
        checks: vec![
            DoctorCheck {
                check_id: "workspace-identity".to_owned(),
                status: "pass".to_owned(),
                summary: "The explicit portable workspace identity is valid.".to_owned(),
                evidence_digest: None,
            },
            DoctorCheck {
                check_id: "manifest-readable".to_owned(),
                status: "pass".to_owned(),
                summary: "The explicit Cargo manifest is present and readable.".to_owned(),
                evidence_digest: Some(manifest_digest.clone()),
            },
            DoctorCheck {
                check_id: "cargo-cli-available".to_owned(),
                status: "pass".to_owned(),
                summary: "The Cargo CLI completed the passive version probe.".to_owned(),
                evidence_digest: Some(owner_output_digest.clone()),
            },
            DoctorCheck {
                check_id: "cargo-version-parse".to_owned(),
                status: "pass".to_owned(),
                summary: format!(
                    "Cargo reported canonical semantic version evidence for {}.",
                    cargo_evidence.version
                ),
                evidence_digest: Some(owner_output_digest.clone()),
            },
        ],
        evidence: DoctorEvidence {
            owner: "Cargo".to_owned(),
            command: vec!["cargo".to_owned(), "--version".to_owned()],
            command_representation: "portable-equivalent".to_owned(),
            working_directory: "selected-manifest-directory-path-not-retained".to_owned(),
            owner_output_digest,
            network_requested: false,
            owner_work_requested: false,
            cargo_network_offline: true,
            rustup_auto_install: false,
            toolchain_selection:
                "owner-resolution-from-selected-manifest-directory-and-environment".to_owned(),
        },
        bounds: DoctorBounds {
            manifest_max_bytes: MAX_DOCTOR_MANIFEST_BYTES,
            probe_timeout_millis: DOCTOR_TIMEOUT.as_millis() as u64,
            stdout_max_bytes: MAX_DOCTOR_OUTPUT_BYTES,
            stderr_max_bytes: MAX_DOCTOR_OUTPUT_BYTES,
            owner_output_framing: "stdout-nul-stderr".to_owned(),
        },
        unknowns: vec![
            "Cargo metadata, dependency availability, lock state, targets, and build readiness were not observed."
                .to_owned(),
            "Compiler, linker, native SDK, test, deployment, credential, connector, and remote evidence readiness were not observed."
                .to_owned(),
        ],
        limitations: vec![
            "This passive report is not a support, compatibility, correctness, security, or performance claim."
                .to_owned(),
            "Ferris requested no owner workspace work, network access, active probe, or mutation; this report is not an operating-system sandbox audit."
                .to_owned(),
        ],
        fallback: "Run cargo --version and ordinary Cargo commands directly; Ferris changed no owner state."
            .to_owned(),
    };

    Ok(success_envelope("doctor", invocation_identity, record))
}

fn load_cargo_metadata(
    manifest_path: &Path,
    cargo_program: &Path,
) -> Result<MetadataInvocation, CoreError> {
    let manifest_path = canonical_manifest_path(manifest_path)?;
    let working_directory = manifest_path.parent().ok_or_else(|| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-PARENT-INVALID",
            "The explicit manifest has no selectable parent directory.",
            vec!["Pass a Cargo.toml within a local directory.".to_owned()],
        )
    })?;
    let mut command = Command::new(cargo_program);
    configure_owner_toolchain_guards(&mut command, working_directory);
    let output = command
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
                "Cargo metadata could not start.",
                vec![
                    "Install Cargo 1.95.0 or make it available on PATH.".to_owned(),
                    "Run the recorded cargo metadata command directly.".to_owned(),
                ],
            )
            .with_source_digest(digest_text(&error.to_string()))
        })?;

    if !output.status.success() {
        let stderr_digest = digest_bytes(&output.stderr);
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let (class, code) = classify_cargo_failure(&stderr);
        let safe_message = if class == ResultClass::Invalid {
            "Cargo rejected the selected manifest or workspace metadata request."
        } else {
            "Cargo metadata was blocked by offline, locked, or source availability requirements."
        };
        return Err(CoreError::new(
            class,
            code,
            if stderr.is_empty() {
                "Cargo metadata failed without a diagnostic.".to_owned()
            } else {
                safe_message.to_owned()
            },
            vec![
                "Run cargo metadata with the same manifest and offline flags.".to_owned(),
                "Repair the owner manifest or make required offline sources available.".to_owned(),
            ],
        )
        .with_source_digest(stderr_digest));
    }

    Ok(MetadataInvocation {
        manifest_path,
        bytes: output.stdout,
    })
}

fn parse_cargo_version(bytes: &[u8]) -> Option<CargoVersionEvidence> {
    let value = std::str::from_utf8(bytes).ok()?;
    let value = value
        .strip_suffix("\r\n")
        .or_else(|| value.strip_suffix('\n'))
        .unwrap_or(value);
    if value.is_empty()
        || value
            .chars()
            .any(|character| matches!(character, '\r' | '\n' | '\t' | '\0'))
        || value.starts_with(' ')
        || value.ends_with(' ')
    {
        return None;
    }
    let parts = value.split(' ').collect::<Vec<_>>();
    if !matches!(parts.len(), 2 | 4) || parts[0] != "cargo" {
        return None;
    }
    let version = parts[1];
    let components = version.split('.').collect::<Vec<_>>();
    if components.len() != 3
        || components
            .iter()
            .any(|component| !valid_semver_number(component))
    {
        return None;
    }
    let (commit, release_date) = if parts.len() == 4 {
        let commit = parts[2].strip_prefix('(')?;
        let release_date = parts[3].strip_suffix(')')?;
        if !(7..=40).contains(&commit.len())
            || !commit
                .chars()
                .all(|character| character.is_ascii_hexdigit())
            || !valid_release_date(release_date)
        {
            return None;
        }
        (Some(commit.to_owned()), Some(release_date.to_owned()))
    } else {
        (None, None)
    };
    Some(CargoVersionEvidence {
        version: version.to_owned(),
        commit,
        release_date,
    })
}

fn valid_semver_number(component: &str) -> bool {
    !component.is_empty()
        && component
            .chars()
            .all(|character| character.is_ascii_digit())
        && (component == "0" || !component.starts_with('0'))
}

fn valid_release_date(value: &str) -> bool {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return false;
    }
    let year = parts[0].parse::<u16>().ok();
    let month = parts[1].parse::<u8>().ok();
    let day = parts[2].parse::<u8>().ok();
    year.is_some_and(|year| year >= 2010)
        && month.is_some_and(|month| (1..=12).contains(&month))
        && day.is_some_and(|day| (1..=31).contains(&day))
}

fn decode_cargo_probe(stdout: &[u8], stderr: &[u8]) -> Result<CargoVersionEvidence, CoreError> {
    if !stderr.is_empty() {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-DIAGNOSTIC",
            "The passive Cargo version probe emitted an additional diagnostic.",
            vec!["Run cargo --version directly and inspect its diagnostic.".to_owned()],
        )
        .with_source_digest(digest_command_output(stdout, stderr)));
    }
    parse_cargo_version(stdout).ok_or_else(|| {
        CoreError::new(
            ResultClass::Unsupported,
            "FERRIS-DOCTOR-CARGO-VERSION-UNSUPPORTED",
            "Cargo returned a successful version response that Ferris could not safely parse.",
            vec![
                "Run cargo --version directly and retain the exact output.".to_owned(),
                "Use a Cargo release with a conventional semantic version response.".to_owned(),
            ],
        )
        .with_source_digest(digest_bytes(stdout))
    })
}

fn read_bounded_doctor_manifest(manifest_path: &Path) -> Result<Vec<u8>, CoreError> {
    let file = fs::File::open(manifest_path).map_err(|error| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-UNREADABLE",
            "The explicit manifest could not be read.",
            vec!["Pass a readable Cargo.toml with --manifest-path.".to_owned()],
        )
        .with_source_digest(digest_text(&error.to_string()))
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_DOCTOR_MANIFEST_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| {
            CoreError::new(
                ResultClass::Invalid,
                "FERRIS-MANIFEST-UNREADABLE",
                "The explicit manifest could not be read.",
                vec!["Pass a readable Cargo.toml with --manifest-path.".to_owned()],
            )
            .with_source_digest(digest_text(&error.to_string()))
        })?;
    if bytes.len() as u64 > MAX_DOCTOR_MANIFEST_BYTES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-MANIFEST-BOUND-EXCEEDED",
            "The explicit Cargo.toml exceeds the passive doctor manifest bound.",
            vec![format!(
                "Reduce the manifest below {MAX_DOCTOR_MANIFEST_BYTES} bytes or inspect it with owner tools."
            )],
        ));
    }
    Ok(bytes)
}

fn doctor_command_error(error: BoundedCommandError, manifest_digest: &str) -> CoreError {
    let error = match error {
        BoundedCommandError::Start(source) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-UNAVAILABLE",
            "The passive Cargo version probe could not start.",
            vec![
                "Install Cargo or make it available on PATH.".to_owned(),
                "Run cargo --version directly.".to_owned(),
            ],
        )
        .with_source_digest(digest_text(&source.to_string())),
        BoundedCommandError::Wait(source) => CoreError::new(
            ResultClass::Internal,
            "FERRIS-DOCTOR-CARGO-WAIT-FAILED",
            "Ferris could not observe completion of the passive Cargo version probe.",
            vec!["Report this Ferris process-control failure.".to_owned()],
        )
        .with_source_digest(digest_text(&source.to_string())),
        BoundedCommandError::Read => CoreError::new(
            ResultClass::Internal,
            "FERRIS-DOCTOR-CARGO-OUTPUT-FAILED",
            "Ferris could not retain bounded Cargo version output.",
            vec!["Report this Ferris process-output failure.".to_owned()],
        ),
        BoundedCommandError::Timeout => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-TIMEOUT",
            "The passive Cargo version probe exceeded its time bound.",
            vec![format!(
                "Run cargo --version directly; Ferris stopped waiting after {} seconds.",
                DOCTOR_TIMEOUT.as_secs()
            )],
        ),
        BoundedCommandError::OutputLimit => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-OUTPUT-BOUND-EXCEEDED",
            "The passive Cargo version probe exceeded its output bound.",
            vec![format!(
                "Run cargo --version directly; Ferris retains at most {MAX_DOCTOR_OUTPUT_BYTES} bytes per stream."
            )],
        ),
    };
    error.with_invocation_selection(manifest_digest.to_owned())
}

fn run_bounded_command(
    command: &mut Command,
    timeout: Duration,
    output_limit: usize,
) -> Result<BoundedOutput, BoundedCommandError> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(BoundedCommandError::Start)?;
    let stdout = child.stdout.take().ok_or(BoundedCommandError::Read)?;
    let stderr = child.stderr.take().ok_or(BoundedCommandError::Read)?;
    let exceeded = Arc::new(AtomicBool::new(false));
    let stdout_receiver = spawn_bounded_reader(stdout, output_limit, Arc::clone(&exceeded));
    let stderr_receiver = spawn_bounded_reader(stderr, output_limit, Arc::clone(&exceeded));
    let started = Instant::now();

    let status = loop {
        if exceeded.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(BoundedCommandError::OutputLimit);
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(BoundedCommandError::Timeout);
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(BoundedCommandError::Wait(error));
            }
        }
    };

    let remaining = timeout.saturating_sub(started.elapsed());
    let stdout = stdout_receiver
        .recv_timeout(remaining)
        .map_err(|error| match error {
            mpsc::RecvTimeoutError::Timeout => BoundedCommandError::Timeout,
            mpsc::RecvTimeoutError::Disconnected => BoundedCommandError::Read,
        })?
        .map_err(|_| BoundedCommandError::Read)?;
    let remaining = timeout.saturating_sub(started.elapsed());
    let stderr = stderr_receiver
        .recv_timeout(remaining)
        .map_err(|error| match error {
            mpsc::RecvTimeoutError::Timeout => BoundedCommandError::Timeout,
            mpsc::RecvTimeoutError::Disconnected => BoundedCommandError::Read,
        })?
        .map_err(|_| BoundedCommandError::Read)?;
    if exceeded.load(Ordering::Relaxed) {
        return Err(BoundedCommandError::OutputLimit);
    }

    Ok(BoundedOutput {
        status,
        stdout,
        stderr,
    })
}

fn spawn_bounded_reader(
    mut reader: impl Read + Send + 'static,
    output_limit: usize,
    exceeded: Arc<AtomicBool>,
) -> mpsc::Receiver<io::Result<Vec<u8>>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut retained = Vec::new();
        let mut buffer = [0_u8; 4096];
        let result = loop {
            match reader.read(&mut buffer) {
                Ok(0) => break Ok(retained),
                Ok(count) => {
                    let remaining = output_limit.saturating_sub(retained.len());
                    retained.extend_from_slice(&buffer[..count.min(remaining)]);
                    if count > remaining {
                        exceeded.store(true, Ordering::Relaxed);
                    }
                }
                Err(error) => break Err(error),
            }
        };
        let _ = sender.send(result);
    });
    receiver
}

fn configure_owner_toolchain_guards(command: &mut Command, working_directory: &Path) {
    command
        .current_dir(working_directory)
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .env("RUSTUP_NO_UPDATE_CHECK", "1");
}

fn configure_passive_cargo_probe(command: &mut Command, working_directory: &Path) {
    configure_owner_toolchain_guards(command, working_directory);
    command.arg("--version").env("CARGO_NET_OFFLINE", "true");
}

pub fn create_explanation(
    manifest_path: &Path,
    workspace_id: &str,
) -> Result<CommandEnvelope<ExplanationRecord>, CoreError> {
    let plan = create_plan(manifest_path, workspace_id)?;
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
        workspace_id: workspace_id.to_owned(),
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
        invocation_identity_for_selection("explain", workspace_id, &plan_record.selected_manifest),
        explanation,
    ))
}

pub fn error_envelope<T>(
    semantic_command_id: &str,
    workspace_id: &str,
    manifest_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T> {
    CommandEnvelope {
        schema: COMMAND_RESULT_SCHEMA.to_owned(),
        command_version: env!("CARGO_PKG_VERSION").to_owned(),
        semantic_command_id: semantic_command_id.to_owned(),
        invocation_identity: invocation_identity_for_request(
            semantic_command_id,
            workspace_id,
            manifest_path,
        ),
        result_class: error.result_class(),
        diagnostics: vec![error.diagnostic().clone()],
        record: None,
    }
}

pub fn doctor_error_envelope<T>(
    workspace_id: &str,
    manifest_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T> {
    let selection = error
        .invocation_selection()
        .map(str::to_owned)
        .unwrap_or_else(|| normalize_request_path(manifest_path));
    CommandEnvelope {
        schema: COMMAND_RESULT_SCHEMA.to_owned(),
        command_version: env!("CARGO_PKG_VERSION").to_owned(),
        semantic_command_id: "doctor".to_owned(),
        invocation_identity: invocation_identity(&[
            "doctor",
            workspace_id,
            &selection,
            "command=cargo --version",
            "working-directory=selected-manifest-directory",
            "cargo-version-probe=true",
            "network-requested=false",
            "owner-work-requested=false",
            "cargo-network-offline=true",
            "rustup-auto-install=false",
            "toolchain=owner-resolution-from-selected-manifest-directory-and-environment",
            "manifest-max-bytes=1048576",
            "probe-timeout-millis=5000",
            "stdout-max-bytes=65536",
            "stderr-max-bytes=65536",
            "owner-output-framing=stdout-nul-stderr",
        ]),
        result_class: error.result_class(),
        diagnostics: vec![error.diagnostic().clone()],
        record: None,
    }
}

pub fn render_plan_human(envelope: &CommandEnvelope<PlanRecord>) -> String {
    let plan = envelope.record.as_ref().expect("success plan has a record");
    let mut output = format!(
        "Ferris plan {}\nWorkspace ID: {}\nWorkspace: {}\nExecutable: no\nPackages:\n",
        plan.plan_id, plan.workspace_id, plan.workspace_root
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

pub fn render_graph_human(envelope: &CommandEnvelope<GraphRecord>) -> String {
    let graph = envelope
        .record
        .as_ref()
        .expect("success graph has a record");
    let mut output = format!(
        "Ferris declared workspace graph {}\nWorkspace ID: {}\nWorkspace: {}\nSelected manifest: {}\nExecutable: no\nNodes:\n",
        graph.graph_id, graph.workspace_id, graph.workspace_root, graph.selected_manifest
    );
    for node in &graph.nodes {
        output.push_str(&format!(
            "  - {} {} (identity={}, manifest={})\n",
            node.name, node.version, node.identity, node.manifest_path
        ));
    }
    output.push_str("Dependency declarations:\n");
    for edge in &graph.edges {
        let target = edge.target.as_deref().unwrap_or("unresolved");
        let alias = edge.dependency_alias.as_deref().unwrap_or("none");
        let condition = edge.target_condition.as_deref().unwrap_or("all targets");
        output.push_str(&format!(
            "  - {} -> {} (dependency={}, alias={}, kind={}, optional={}, condition={}, resolution={})\n",
            edge.from,
            target,
            edge.dependency_name,
            alias,
            edge.kind,
            edge.optional,
            condition,
            edge.resolution
        ));
    }
    output.push_str("Unknowns:\n");
    if graph.unknowns.is_empty() {
        output.push_str("  - none in the bounded declaration projection\n");
    } else {
        for unknown in &graph.unknowns {
            output.push_str(&format!("  - {unknown}\n"));
        }
    }
    output.push_str("Limitations:\n");
    for limitation in &graph.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output.push_str(&format!(
        "Evidence: owner={}, representation={}, working-directory={}, workspace-id={}, metadata-format={}, offline={}, rustup-auto-install={}, toolchain={}, output-digest={}\nCommand: {}\n",
        graph.evidence.owner,
        graph.evidence.command_representation,
        graph.evidence.working_directory,
        graph.evidence.workspace_id,
        graph.evidence.metadata_format_version,
        graph.evidence.offline,
        graph.evidence.rustup_auto_install,
        graph.evidence.toolchain_selection,
        graph.evidence.owner_output_digest,
        graph.evidence.command.join(" ")
    ));
    output
}

pub fn render_explanation_human(envelope: &CommandEnvelope<ExplanationRecord>) -> String {
    let explanation = envelope
        .record
        .as_ref()
        .expect("success explanation has a record");
    let mut output = format!(
        "Ferris explanation for {}\nWorkspace ID: {}\nSelected:\n",
        explanation.plan_id, explanation.workspace_id
    );
    for selected in &explanation.selected {
        output.push_str(&format!("  - {selected}\n"));
    }
    output.push_str("Why:\n");
    for reason in &explanation.reasons {
        output.push_str(&format!("  - {reason}\n"));
    }
    output.push_str("Omitted:\n");
    for omitted in &explanation.omitted {
        output.push_str(&format!("  - {omitted}\n"));
    }
    output.push_str("Unknowns:\n");
    for unknown in &explanation.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str(&format!(
        "Evidence owner: {}\nEvidence that would change the result: {}\n",
        explanation.evidence_owner, explanation.change_evidence
    ));
    output.push_str(&format!("Fallback: {}\n", explanation.fallback));
    output
}

pub fn render_doctor_human(envelope: &CommandEnvelope<DoctorReport>) -> String {
    let report = envelope
        .record
        .as_ref()
        .expect("success doctor envelope has a record");
    let mut output = format!(
        "Ferris passive doctor {}\nWorkspace ID: {}\nManifest digest: {}\nCargo version: {}\nCargo commit: {}\nCargo release date: {}\nChecks:\n",
        report.report_id,
        report.workspace_id,
        report.manifest_digest,
        report.cargo_version,
        report.cargo_commit.as_deref().unwrap_or("not reported"),
        report
            .cargo_release_date
            .as_deref()
            .unwrap_or("not reported")
    );
    for check in &report.checks {
        output.push_str(&format!(
            "  - {}: {} - {}\n",
            check.check_id, check.status, check.summary
        ));
        if let Some(digest) = &check.evidence_digest {
            output.push_str(&format!("    evidence: {digest}\n"));
        }
    }
    output.push_str("Unknowns:\n");
    for unknown in &report.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &report.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output.push_str(&format!(
        "Evidence: owner={}, representation={}, working-directory={}, network-requested={}, owner-work-requested={}, cargo-network-offline={}, rustup-auto-install={}, toolchain={}, output-digest={}\nBounds: manifest-bytes={}, timeout-ms={}, stdout-bytes={}, stderr-bytes={}, framing={}\nCommand: {}\nFallback: {}\n",
        report.evidence.owner,
        report.evidence.command_representation,
        report.evidence.working_directory,
        report.evidence.network_requested,
        report.evidence.owner_work_requested,
        report.evidence.cargo_network_offline,
        report.evidence.rustup_auto_install,
        report.evidence.toolchain_selection,
        report.evidence.owner_output_digest,
        report.bounds.manifest_max_bytes,
        report.bounds.probe_timeout_millis,
        report.bounds.stdout_max_bytes,
        report.bounds.stderr_max_bytes,
        report.bounds.owner_output_framing,
        report.evidence.command.join(" "),
        report.fallback
    ));
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
            "The explicit manifest path is not a file.",
            vec!["Pass an existing Cargo.toml with --manifest-path.".to_owned()],
        )
        .with_source_digest(digest_text(&normalize_path_text(
            &manifest_path.to_string_lossy(),
        ))));
    }

    manifest_path.canonicalize().map_err(|error| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-UNREADABLE",
            "The explicit manifest path could not be resolved.",
            vec!["Pass a readable Cargo.toml with --manifest-path.".to_owned()],
        )
        .with_source_digest(digest_text(&format!(
            "{}\0{error}",
            normalize_path_text(&manifest_path.to_string_lossy())
        )))
    })
}

fn plan_from_metadata(
    manifest_path: &Path,
    workspace_id: &str,
    bytes: &[u8],
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    let metadata = decode_metadata(bytes)?;
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
                identity: package_identity(&package.name, &package.version, &manifest_path),
                name: package.name,
                version: package.version,
                manifest_path,
                workspace_member,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    packages.sort_by(|left, right| left.identity.cmp(&right.identity));

    let plan_id = record_id(
        "plan",
        &(PLAN_SCHEMA, workspace_id, &selected_manifest, &packages),
    )?;
    let record = PlanRecord {
        schema: PLAN_SCHEMA.to_owned(),
        plan_id,
        workspace_id: workspace_id.to_owned(),
        executable: false,
        selected_manifest: selected_manifest.clone(),
        workspace_root: ".".to_owned(),
        packages,
        evidence: metadata_evidence(&selected_manifest, workspace_id, bytes),
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

    Ok(success_envelope(
        "plan",
        invocation_identity_for_selection("plan", workspace_id, &selected_manifest),
        record,
    ))
}

fn decode_metadata(bytes: &[u8]) -> Result<CargoMetadata, CoreError> {
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

    Ok(metadata)
}

fn graph_from_metadata(
    manifest_path: &Path,
    workspace_id: &str,
    bytes: &[u8],
) -> Result<CommandEnvelope<GraphRecord>, CoreError> {
    let metadata = decode_metadata(bytes)?;
    let workspace_root = PathBuf::from(&metadata.workspace_root);
    let selected_manifest = workspace_relative_path(manifest_path, &workspace_root)?;
    let workspace_packages = metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
        .collect::<Vec<_>>();
    let edge_count = workspace_packages
        .iter()
        .map(|package| package.dependencies.len())
        .sum();
    enforce_graph_bounds(workspace_packages.len(), edge_count)?;

    let mut nodes = Vec::with_capacity(workspace_packages.len());
    let mut cargo_id_to_identity = BTreeMap::new();
    let mut directory_to_identities: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for package in &workspace_packages {
        let manifest_path =
            workspace_relative_path(Path::new(&package.manifest_path), &workspace_root)?;
        let identity = package_identity(&package.name, &package.version, &manifest_path);
        let directory = parent_path_text(&manifest_path);
        cargo_id_to_identity.insert(package.id.clone(), identity.clone());
        directory_to_identities
            .entry(directory)
            .or_default()
            .push(identity.clone());
        nodes.push(GraphNode {
            identity,
            name: package.name.clone(),
            version: package.version.clone(),
            manifest_path,
        });
    }
    nodes.sort_by(|left, right| left.identity.cmp(&right.identity));
    for identities in directory_to_identities.values_mut() {
        identities.sort();
    }

    let mut edges = Vec::with_capacity(edge_count);
    let mut unresolved_count = 0_usize;
    for package in workspace_packages {
        let from = cargo_id_to_identity
            .get(&package.id)
            .cloned()
            .ok_or_else(|| {
                CoreError::new(
                    ResultClass::Internal,
                    "FERRIS-GRAPH-SOURCE-MISSING",
                    "A Cargo workspace member was missing from the normalized node map.",
                    vec!["Report this Ferris invariant failure.".to_owned()],
                )
            })?;
        for dependency in &package.dependencies {
            let (target, resolution) =
                dependency_target(dependency, &workspace_root, &directory_to_identities);
            if target.is_none() {
                unresolved_count += 1;
            }
            let kind = dependency
                .kind
                .clone()
                .unwrap_or_else(|| "normal".to_owned());
            let edge_identity = record_id(
                "edge",
                &(
                    GRAPH_SCHEMA,
                    &from,
                    &target,
                    &dependency.name,
                    &dependency.rename,
                    &kind,
                    dependency.optional,
                    &dependency.target,
                    resolution,
                ),
            )?;
            edges.push(GraphEdge {
                identity: edge_identity,
                from: from.clone(),
                target,
                dependency_name: dependency.name.clone(),
                dependency_alias: dependency.rename.clone(),
                kind,
                optional: dependency.optional,
                target_condition: dependency.target.clone(),
                resolution: resolution.to_owned(),
            });
        }
    }
    edges.sort_by(|left, right| left.identity.cmp(&right.identity));

    let graph_id = record_id(
        "graph",
        &(
            GRAPH_SCHEMA,
            workspace_id,
            &selected_manifest,
            &nodes,
            &edges,
        ),
    )?;
    let mut unknowns = Vec::new();
    if unresolved_count > 0 {
        unknowns.push(format!(
            "{unresolved_count} dependency declarations do not resolve to a unique workspace member in this declaration-only projection."
        ));
    }
    let record = GraphRecord {
        schema: GRAPH_SCHEMA.to_owned(),
        graph_id,
        workspace_id: workspace_id.to_owned(),
        executable: false,
        selected_manifest: selected_manifest.clone(),
        workspace_root: ".".to_owned(),
        nodes,
        edges,
        evidence: metadata_evidence(&selected_manifest, workspace_id, bytes),
        unknowns,
        limitations: vec![
            "This graph contains Cargo-declared relationships, not resolved build units.".to_owned(),
            "External, registry, git, ambiguous, and non-member targets remain unresolved."
                .to_owned(),
            "This graph does not establish affected scope, invalidation, scheduling, freshness, validation, native, ABI, or runtime behavior."
                .to_owned(),
        ],
    };

    Ok(success_envelope(
        "graph",
        invocation_identity_for_selection("graph", workspace_id, &selected_manifest),
        record,
    ))
}

fn dependency_target(
    dependency: &CargoDependency,
    workspace_root: &Path,
    directory_to_identities: &BTreeMap<String, Vec<String>>,
) -> (Option<String>, &'static str) {
    let Some(path) = dependency.path.as_deref() else {
        return (None, "external-unresolved");
    };
    let Some(directory) = relative_path_text_if_inside(path, workspace_root) else {
        return (None, "outside-workspace");
    };
    match directory_to_identities.get(&directory).map(Vec::as_slice) {
        Some([identity]) => (Some(identity.clone()), "workspace-member"),
        Some(_) => (None, "ambiguous-workspace-target"),
        None => (None, "unresolved-workspace-path"),
    }
}

fn relative_path_text_if_inside(path: &str, root: &Path) -> Option<String> {
    let path = normalize_path_text(path);
    let root = normalize_path_text(&root.to_string_lossy());
    if path == root {
        return Some(".".to_owned());
    }
    path.strip_prefix(&format!("{}/", root.trim_end_matches('/')))
        .map(str::to_owned)
}

fn enforce_graph_bounds(node_count: usize, edge_count: usize) -> Result<(), CoreError> {
    if node_count > MAX_GRAPH_NODES || edge_count > MAX_GRAPH_EDGES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-GRAPH-BOUND-EXCEEDED",
            format!(
                "The declared workspace graph has {node_count} nodes and {edge_count} edges; Pulse 02 permits at most {MAX_GRAPH_NODES} nodes and {MAX_GRAPH_EDGES} edges."
            ),
            vec![
                "Use Cargo owner tools directly for this workspace.".to_owned(),
                "Do not treat a truncated graph as a successful Ferris projection.".to_owned(),
            ],
        ));
    }
    Ok(())
}

fn package_identity(name: &str, version: &str, manifest_path: &str) -> String {
    format!("cargo-workspace-package:{name}@{version}:{manifest_path}")
}

fn parent_path_text(path: &str) -> String {
    Path::new(path)
        .parent()
        .map(|parent| normalize_path_text(&parent.to_string_lossy()))
        .filter(|parent| !parent.is_empty())
        .unwrap_or_else(|| ".".to_owned())
}

fn metadata_evidence(selected_manifest: &str, workspace_id: &str, bytes: &[u8]) -> EvidenceSource {
    EvidenceSource {
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
            selected_manifest.to_owned(),
        ],
        command_representation: "portable-equivalent".to_owned(),
        working_directory: "selected-manifest-directory-path-not-retained".to_owned(),
        workspace_id: workspace_id.to_owned(),
        owner_output_digest: digest_bytes(bytes),
        metadata_format_version: 1,
        offline: true,
        rustup_auto_install: false,
        toolchain_selection: "owner-resolution-from-selected-manifest-directory-and-environment"
            .to_owned(),
    }
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

fn invocation_identity_for_request(
    semantic_command_id: &str,
    workspace_id: &str,
    path: &Path,
) -> String {
    invocation_identity_for_selection(
        semantic_command_id,
        workspace_id,
        &normalize_request_path(path),
    )
}

fn normalize_request_path(path: &Path) -> String {
    let text = normalize_path_text(&path.to_string_lossy());
    let rooted = text.starts_with('/');
    let mut components = Vec::new();
    for component in text.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if components
                    .last()
                    .is_some_and(|previous: &&str| *previous != ".." && !previous.ends_with(':'))
                {
                    components.pop();
                } else if !rooted {
                    components.push(component);
                }
            }
            _ => components.push(component),
        }
    }
    let normalized = components.join("/");
    if rooted {
        format!("/{normalized}")
    } else if normalized.is_empty() {
        ".".to_owned()
    } else {
        normalized
    }
}

fn invocation_identity_for_selection(
    semantic_command_id: &str,
    workspace_id: &str,
    selected_manifest: &str,
) -> String {
    invocation_identity(&[
        semantic_command_id,
        workspace_id,
        selected_manifest,
        "cargo-metadata-format=1",
        "no-deps=true",
        "offline=true",
        "locked=true",
        "rustup-auto-install=false",
        "toolchain=owner-resolution-from-selected-manifest-directory-and-environment",
    ])
}

fn doctor_invocation_identity(
    workspace_id: &str,
    manifest_digest: &str,
    cargo_evidence: &CargoVersionEvidence,
    owner_output_digest: &str,
) -> String {
    invocation_identity(&[
        "doctor",
        workspace_id,
        manifest_digest,
        &cargo_evidence.version,
        cargo_evidence.commit.as_deref().unwrap_or("none"),
        cargo_evidence.release_date.as_deref().unwrap_or("none"),
        owner_output_digest,
        "command=cargo --version",
        "working-directory=selected-manifest-directory",
        "cargo-version-probe=true",
        "network-requested=false",
        "owner-work-requested=false",
        "cargo-network-offline=true",
        "rustup-auto-install=false",
        "toolchain=owner-resolution-from-selected-manifest-directory-and-environment",
        "manifest-max-bytes=1048576",
        "probe-timeout-millis=5000",
        "stdout-max-bytes=65536",
        "stderr-max-bytes=65536",
        "owner-output-framing=stdout-nul-stderr",
    ])
}

fn doctor_report_id(
    workspace_id: &str,
    manifest_digest: &str,
    cargo_evidence: &CargoVersionEvidence,
    owner_output_digest: &str,
) -> Result<String, CoreError> {
    record_id(
        "doctor",
        &DoctorIdentityInput {
            schema: DOCTOR_SCHEMA,
            workspace_id,
            manifest_digest,
            cargo_version: &cargo_evidence.version,
            cargo_commit: cargo_evidence.commit.as_deref().unwrap_or("none"),
            cargo_release_date: cargo_evidence.release_date.as_deref().unwrap_or("none"),
            owner_output_digest,
            command: "cargo --version",
            working_directory: "selected-manifest-directory",
            cargo_version_probe: true,
            network_requested: false,
            owner_work_requested: false,
            cargo_network_offline: true,
            rustup_auto_install: false,
            toolchain_selection: "owner-resolution-from-selected-manifest-directory-and-environment",
            manifest_max_bytes: MAX_DOCTOR_MANIFEST_BYTES,
            probe_timeout_millis: DOCTOR_TIMEOUT.as_millis() as u64,
            stdout_max_bytes: MAX_DOCTOR_OUTPUT_BYTES,
            stderr_max_bytes: MAX_DOCTOR_OUTPUT_BYTES,
            owner_output_framing: "stdout-nul-stderr",
        },
    )
}

fn invocation_identity(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    let digest = hasher.finalize();
    format!("invocation:{}", hex_digest(&digest))
}

pub fn command_line_invocation_identity(semantic_command_id: &str, args: &[String]) -> String {
    let mut parts = Vec::with_capacity(args.len() + 1);
    parts.push(semantic_command_id);
    parts.extend(args.iter().map(String::as_str));
    invocation_identity(&parts)
}

fn validate_workspace_id(workspace_id: &str) -> Result<(), CoreError> {
    let valid = !workspace_id.is_empty()
        && workspace_id.len() <= 128
        && workspace_id.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_' | ':' | '/')
        });
    if valid {
        return Ok(());
    }
    Err(CoreError::new(
        ResultClass::Invalid,
        "FERRIS-WORKSPACE-ID-INVALID",
        "Workspace identity must contain 1 to 128 ASCII letters, digits, '.', '-', '_', ':', or '/'.",
        vec![
            "Pass a stable portable identity such as --workspace-id org.example/project."
                .to_owned(),
        ],
    ))
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

fn record_id<T: Serialize>(prefix: &str, value: &T) -> Result<String, CoreError> {
    let bytes = serde_json::to_vec(value).map_err(|error| {
        CoreError::new(
            ResultClass::Internal,
            "FERRIS-CANONICAL-SERIALIZATION-FAILED",
            format!("Ferris could not serialize a canonical planning input: {error}"),
            vec!["Report this Ferris invariant failure.".to_owned()],
        )
    })?;
    let digest = Sha256::digest(bytes);
    Ok(format!("{prefix}:{}", hex_digest(&digest)))
}

fn digest_text(value: &str) -> String {
    digest_bytes(value.as_bytes())
}

fn digest_bytes(value: &[u8]) -> String {
    let digest = Sha256::digest(value);
    format!("sha256:{}", hex_digest(&digest))
}

fn digest_command_output(stdout: &[u8], stderr: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(stdout);
    hasher.update([0]);
    hasher.update(stderr);
    format!("sha256:{}", hex_digest(&hasher.finalize()))
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
        let first = create_plan(&manifest(), "ferris.test/simple").expect("first plan");
        let second = create_plan(&manifest(), "ferris.test/simple").expect("second plan");
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
        let plan = create_plan(&manifest(), "ferris.test/simple").expect("plan");
        let explanation =
            create_explanation(&manifest(), "ferris.test/simple").expect("explanation");
        assert_eq!(
            plan.record.expect("plan record").plan_id,
            explanation.record.expect("explanation record").plan_id
        );
    }

    #[test]
    fn unsupported_metadata_version_is_explicit() {
        let error = plan_from_metadata(
            &manifest(),
            "ferris.test/simple",
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
            "ferris.test/simple",
            br#"{"packages":[],"workspace_members":[],"workspace_root":"x"}"#,
        )
        .expect_err("missing version");
        assert_eq!(error.result_class(), ResultClass::Incomplete);
    }

    #[test]
    fn malformed_metadata_is_internal() {
        let error = plan_from_metadata(&manifest(), "ferris.test/simple", b"{")
            .expect_err("malformed metadata should fail");
        assert_eq!(error.result_class(), ResultClass::Internal);
    }

    #[test]
    fn unavailable_cargo_is_blocked() {
        let error = create_plan_with_cargo(
            &manifest(),
            "ferris.test/simple",
            Path::new("ferris-definitely-missing-cargo-executable"),
        )
        .expect_err("missing cargo should fail");
        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(error.diagnostic().code, "FERRIS-CARGO-UNAVAILABLE");
    }

    #[test]
    fn graph_bounds_block_without_partial_output() {
        let node_error =
            enforce_graph_bounds(MAX_GRAPH_NODES + 1, 0).expect_err("node bound should block");
        assert_eq!(node_error.result_class(), ResultClass::Blocked);
        assert_eq!(node_error.diagnostic().code, "FERRIS-GRAPH-BOUND-EXCEEDED");

        let edge_error =
            enforce_graph_bounds(1, MAX_GRAPH_EDGES + 1).expect_err("edge bound should block");
        assert_eq!(edge_error.result_class(), ResultClass::Blocked);
    }

    #[test]
    fn windows_dependency_paths_resolve_to_workspace_members() {
        let dependency = CargoDependency {
            name: "alpha".to_owned(),
            rename: None,
            kind: None,
            optional: false,
            target: None,
            path: Some(r"\\?\C:\checkout\alpha".to_owned()),
        };
        let directories = BTreeMap::from([(
            "alpha".to_owned(),
            vec!["cargo-workspace-package:alpha@0.1.0:alpha/Cargo.toml".to_owned()],
        )]);

        let (target, resolution) =
            dependency_target(&dependency, Path::new(r"\\?\C:\checkout"), &directories);
        assert_eq!(
            target.as_deref(),
            Some("cargo-workspace-package:alpha@0.1.0:alpha/Cargo.toml")
        );
        assert_eq!(resolution, "workspace-member");
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
            "ferris.test/portable",
            &metadata("checkout-a"),
        )
        .expect("first plan");
        let second = plan_from_metadata(
            Path::new("checkout-b/Cargo.toml"),
            "ferris.test/portable",
            &metadata("checkout-b"),
        )
        .expect("second plan");

        assert_eq!(
            first.record.expect("first record").plan_id,
            second.record.expect("second record").plan_id
        );
    }

    #[test]
    fn graph_identity_is_independent_of_checkout_path_and_json_order() {
        fn metadata(root: &str, reverse: bool) -> Vec<u8> {
            let alpha_id = format!("path+file:///{root}/alpha#alpha@0.1.0");
            let beta_id = format!("path+file:///{root}/beta#beta@0.1.0");
            let alpha = serde_json::json!({
                "id": alpha_id,
                "name": "alpha",
                "version": "0.1.0",
                "manifest_path": format!("{root}/alpha/Cargo.toml"),
                "dependencies": []
            });
            let beta = serde_json::json!({
                "id": beta_id,
                "name": "beta",
                "version": "0.1.0",
                "manifest_path": format!("{root}/beta/Cargo.toml"),
                "dependencies": [{
                    "name": "alpha",
                    "rename": null,
                    "kind": null,
                    "optional": false,
                    "target": null,
                    "path": format!("{root}/alpha")
                }]
            });
            let packages = if reverse {
                vec![beta, alpha]
            } else {
                vec![alpha, beta]
            };
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "workspace_root": root,
                "workspace_members": [alpha_id, beta_id],
                "packages": packages
            }))
            .expect("metadata JSON")
        }

        let first = graph_from_metadata(
            Path::new("checkout-a/Cargo.toml"),
            "ferris.test/portable",
            &metadata("checkout-a", false),
        )
        .expect("first graph");
        let second = graph_from_metadata(
            Path::new("checkout-b/Cargo.toml"),
            "ferris.test/portable",
            &metadata("checkout-b", true),
        )
        .expect("second graph");

        assert_eq!(
            first.record.expect("first record").graph_id,
            second.record.expect("second record").graph_id
        );
    }

    #[test]
    fn workspace_identity_separates_plan_and_graph_records() {
        let first_plan = create_plan(&manifest(), "ferris.test/one")
            .expect("first plan")
            .record
            .expect("first plan record");
        let second_plan = create_plan(&manifest(), "ferris.test/two")
            .expect("second plan")
            .record
            .expect("second plan record");
        let first_graph = create_graph(&manifest(), "ferris.test/one")
            .expect("first graph")
            .record
            .expect("first graph record");
        let second_graph = create_graph(&manifest(), "ferris.test/two")
            .expect("second graph")
            .record
            .expect("second graph record");

        assert_ne!(first_plan.plan_id, second_plan.plan_id);
        assert_ne!(first_graph.graph_id, second_graph.graph_id);
    }

    #[test]
    fn semantic_commands_have_distinct_invocation_identities() {
        let plan = create_plan(&manifest(), "ferris.test/simple").expect("plan");
        let explanation =
            create_explanation(&manifest(), "ferris.test/simple").expect("explanation");
        let graph = create_graph(&manifest(), "ferris.test/simple").expect("graph");

        assert_ne!(plan.invocation_identity, explanation.invocation_identity);
        assert_ne!(plan.invocation_identity, graph.invocation_identity);
        assert_ne!(explanation.invocation_identity, graph.invocation_identity);
    }

    #[test]
    fn request_identity_lexically_normalizes_manifest_paths() {
        let direct = invocation_identity_for_request(
            "plan",
            "ferris.test/simple",
            Path::new("fixtures/Cargo.toml"),
        );
        let equivalent = invocation_identity_for_request(
            "plan",
            "ferris.test/simple",
            Path::new("fixtures/./missing/../Cargo.toml"),
        );

        assert_eq!(direct, equivalent);
    }

    #[test]
    fn passive_doctor_reports_local_prerequisites() {
        let envelope = create_doctor(&manifest(), "ferris.test/simple").expect("doctor report");
        let report = envelope.record.expect("doctor record");

        assert_eq!(report.schema, DOCTOR_SCHEMA);
        assert_eq!(report.checks.len(), 4);
        assert!(report.checks.iter().all(|check| check.status == "pass"));
        assert!(!report.cargo_version.is_empty());
        assert!(!report.evidence.network_requested);
        assert!(!report.evidence.owner_work_requested);
        assert!(report.evidence.cargo_network_offline);
        assert!(!report.evidence.rustup_auto_install);
        assert_eq!(
            report.evidence.toolchain_selection,
            "owner-resolution-from-selected-manifest-directory-and-environment"
        );
    }

    #[test]
    fn passive_doctor_blocks_when_cargo_is_unavailable() {
        let error = create_doctor_with_cargo(
            &manifest(),
            "ferris.test/simple",
            Path::new("ferris-definitely-missing-cargo-executable"),
        )
        .expect_err("missing cargo should block doctor");

        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(error.diagnostic().code, "FERRIS-DOCTOR-CARGO-UNAVAILABLE");
        assert!(error.diagnostic().source_digest.is_some());
    }

    #[test]
    fn passive_doctor_rejects_malformed_version_output() {
        let error =
            decode_cargo_probe(b"cargo version unknown\n", b"").expect_err("version should fail");

        assert_eq!(error.result_class(), ResultClass::Unsupported);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-DOCTOR-CARGO-VERSION-UNSUPPORTED"
        );
        assert!(error.diagnostic().source_digest.is_some());

        assert!(parse_cargo_version(b"cargo 01.2.3\n").is_none());
        assert!(parse_cargo_version(b"cargo 1.2.3 unexpected\n").is_none());
        assert!(parse_cargo_version(b" cargo 1.2.3\n").is_none());

        let evidence =
            parse_cargo_version(b"cargo 1.95.0 (f2d3ce0bd 2026-03-21)\n").expect("Cargo evidence");
        assert_eq!(evidence.version, "1.95.0");
        assert_eq!(evidence.commit.as_deref(), Some("f2d3ce0bd"));
        assert_eq!(evidence.release_date.as_deref(), Some("2026-03-21"));
    }

    #[test]
    fn passive_doctor_blocks_additional_probe_diagnostics() {
        let error = decode_cargo_probe(b"cargo 1.95.0\n", b"warning")
            .expect_err("stderr should block a passive report");

        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(error.diagnostic().code, "FERRIS-DOCTOR-CARGO-DIAGNOSTIC");
    }

    #[test]
    fn passive_doctor_configures_rustup_and_cargo_guards() {
        let mut command = Command::new("cargo");
        let working_directory = manifest().parent().expect("manifest parent").to_path_buf();
        configure_passive_cargo_probe(&mut command, &working_directory);
        let environment = command
            .get_envs()
            .map(|(key, value)| {
                (
                    key.to_string_lossy().into_owned(),
                    value.map(|configured| configured.to_string_lossy().into_owned()),
                )
            })
            .collect::<BTreeMap<_, _>>();

        assert_eq!(
            command
                .get_args()
                .map(|arg| arg.to_string_lossy())
                .collect::<Vec<_>>(),
            vec!["--version"]
        );
        assert_eq!(command.get_current_dir(), Some(working_directory.as_path()));
        assert_eq!(environment["CARGO_NET_OFFLINE"].as_deref(), Some("true"));
        assert_eq!(environment["RUSTUP_AUTO_INSTALL"].as_deref(), Some("0"));
        assert_eq!(environment["RUSTUP_NO_UPDATE_CHECK"].as_deref(), Some("1"));
        assert!(!environment.contains_key("RUSTUP_TOOLCHAIN"));
    }

    #[test]
    fn doctor_identity_tracks_workspace_manifest_and_cargo_version() {
        let cargo = CargoVersionEvidence {
            version: "1.95.0".to_owned(),
            commit: Some("f2d3ce0bd".to_owned()),
            release_date: Some("2026-03-21".to_owned()),
        };
        let changed_cargo = CargoVersionEvidence {
            version: "1.96.0".to_owned(),
            commit: Some("abcdef012".to_owned()),
            release_date: Some("2026-04-01".to_owned()),
        };
        let baseline = doctor_report_id(
            "ferris.test/one",
            "sha256:manifest-a",
            &cargo,
            "sha256:output-a",
        )
        .expect("report ID");
        let same = doctor_report_id(
            "ferris.test/one",
            "sha256:manifest-a",
            &cargo,
            "sha256:output-a",
        )
        .expect("report ID");
        let workspace_change = doctor_report_id(
            "ferris.test/two",
            "sha256:manifest-a",
            &cargo,
            "sha256:output-a",
        )
        .expect("report ID");
        let manifest_change = doctor_report_id(
            "ferris.test/one",
            "sha256:manifest-b",
            &cargo,
            "sha256:output-a",
        )
        .expect("report ID");
        let cargo_change = doctor_report_id(
            "ferris.test/one",
            "sha256:manifest-a",
            &changed_cargo,
            "sha256:output-a",
        )
        .expect("report ID");
        let output_change = doctor_report_id(
            "ferris.test/one",
            "sha256:manifest-a",
            &cargo,
            "sha256:output-b",
        )
        .expect("report ID");

        assert_eq!(baseline, same);
        assert_ne!(baseline, workspace_change);
        assert_ne!(baseline, manifest_change);
        assert_ne!(baseline, cargo_change);
        assert_ne!(baseline, output_change);

        assert_ne!(
            doctor_invocation_identity(
                "ferris.test/one",
                "sha256:manifest-a",
                &cargo,
                "sha256:output-a",
            ),
            doctor_invocation_identity(
                "ferris.test/one",
                "sha256:manifest-a",
                &cargo,
                "sha256:output-b",
            )
        );
    }

    #[test]
    fn doctor_failure_identity_uses_manifest_digest_after_read() {
        let error = CoreError::new(
            ResultClass::Blocked,
            "FERRIS-TEST-BLOCKED",
            "blocked",
            Vec::new(),
        )
        .with_invocation_selection("sha256:manifest".to_owned());
        let first: CommandEnvelope<serde_json::Value> = doctor_error_envelope(
            "ferris.test/simple",
            Path::new("checkout-a/Cargo.toml"),
            &error,
        );
        let second: CommandEnvelope<serde_json::Value> = doctor_error_envelope(
            "ferris.test/simple",
            Path::new("checkout-b/Cargo.toml"),
            &error,
        );

        assert_eq!(first.invocation_identity, second.invocation_identity);
    }

    #[test]
    fn passive_doctor_blocks_oversized_manifests() {
        let directory = std::env::temp_dir().join(format!(
            "ferris-doctor-manifest-bound-{}",
            std::process::id()
        ));
        let manifest = directory.join("Cargo.toml");
        fs::create_dir_all(&directory).expect("create test directory");
        fs::write(
            &manifest,
            vec![b'x'; MAX_DOCTOR_MANIFEST_BYTES as usize + 1],
        )
        .expect("write oversized manifest");

        let error =
            read_bounded_doctor_manifest(&manifest).expect_err("oversized manifest should block");

        fs::remove_file(&manifest).expect("remove test manifest");
        fs::remove_dir(&directory).expect("remove test directory");
        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-DOCTOR-MANIFEST-BOUND-EXCEEDED"
        );
    }

    #[test]
    fn passive_doctor_bounds_probe_time() {
        let mut command = Command::new(std::env::current_exe().expect("test executable"));
        command.args([
            "--exact",
            "tests::bounded_command_sleep_helper",
            "--ignored",
            "--nocapture",
        ]);

        let error = run_bounded_command(&mut command, Duration::from_millis(200), 4096)
            .expect_err("sleeping helper should time out");

        assert!(matches!(error, BoundedCommandError::Timeout));
    }

    #[test]
    fn passive_doctor_bounds_probe_output() {
        let mut command = Command::new(std::env::current_exe().expect("test executable"));
        command.args([
            "--exact",
            "tests::bounded_command_output_helper",
            "--ignored",
            "--nocapture",
        ]);

        let error = run_bounded_command(&mut command, Duration::from_secs(5), 1024)
            .expect_err("verbose helper should exceed output bound");

        assert!(matches!(error, BoundedCommandError::OutputLimit));
    }

    #[test]
    #[ignore]
    fn bounded_command_sleep_helper() {
        thread::sleep(Duration::from_secs(30));
    }

    #[test]
    #[ignore]
    fn bounded_command_output_helper() {
        use std::io::Write as _;

        std::io::stdout()
            .write_all(&vec![b'x'; 128 * 1024])
            .expect("write helper output");
    }
}
