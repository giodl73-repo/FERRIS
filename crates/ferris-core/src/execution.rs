use super::{CoreError, ResultClass, StrictJsonValue, digest_bytes, git_stdout};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU8, Ordering},
};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const ACTION_PLAN_SCHEMA: &str = "ferris.action-plan/v1";
pub const OWNER_ENTRYPOINTS_SCHEMA: &str = "ferris.owner-entrypoints/v1";
pub const EXECUTION_APPROVAL_SCHEMA: &str = "ferris.execution-approval/v1";
pub const EXECUTION_RECEIPT_SCHEMA: &str = "ferris.execution-receipt/v1";
pub const EXECUTION_VERIFICATION_SCHEMA: &str = "ferris.execution-receipt-verification/v1";

const MAX_EXECUTION_FILE_BYTES: u64 = 1024 * 1024;
const MAX_RECEIPT_BYTES: u64 = 32 * 1024 * 1024;
const MAX_LANES: usize = 256;
const MAX_ARGUMENTS: usize = 256;
const MAX_ENVIRONMENT_NAMES: usize = 128;
const MAX_FILES_PER_COMMAND: usize = 256;
const MAX_METADATA_BYTES: usize = 1024;
const MAX_TIMEOUT_MS: u64 = 3_600_000;
const MAX_STREAM_BYTES: u64 = 16 * 1024 * 1024;
const MAX_DIAGNOSTIC_TAIL_BYTES: usize = 8 * 1024;
const MIN_REDACTION_TOKEN_BYTES: usize = 8;
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(5);
const PROCESS_CLEANUP_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundFile {
    pub path: String,
    pub identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EntrypointCommand {
    pub owner: String,
    pub executable: String,
    pub argv: Vec<String>,
    pub working_directory: String,
    pub inherited_environment: Vec<String>,
    pub credential_class: String,
    pub files: Vec<BoundFile>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerEntrypoint {
    pub entrypoint_id: String,
    pub entrypoint_identity: String,
    pub command: EntrypointCommand,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerEntrypointDeclaration {
    pub schema: String,
    pub declaration_id: String,
    pub source_revision: String,
    pub entrypoints: Vec<OwnerEntrypoint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionLane {
    pub lane_id: String,
    pub owner_gate_id: String,
    pub required: bool,
    pub depends_on: Vec<String>,
    pub entrypoint_id: String,
    pub entrypoint_identity: String,
    pub command: EntrypointCommand,
    pub timeout_ms: u64,
    pub stdout_limit_bytes: u64,
    pub stderr_limit_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionPlan {
    pub schema: String,
    pub action_plan_id: String,
    pub repository_id: String,
    pub source_revision: String,
    pub topology_id: String,
    pub declaration_id: String,
    pub approval_id: String,
    pub lanes: Vec<ActionLane>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionApproval {
    pub schema: String,
    pub approval_id: String,
    pub action_plan_id: String,
    pub principal: String,
    pub allowed_environment: Vec<String>,
    pub expires_at: String,
    pub revoked: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneTerminalStatus {
    Succeeded,
    Failed,
    TimedOut,
    Cancelled,
    BlockedByDependency,
    OutputLimitExceeded,
    LeakedSecret,
    InternalError,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupState {
    Complete,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionOutput {
    pub digest: String,
    pub diagnostic_tail: String,
    pub retained_bytes: u64,
    pub observed_bytes: u64,
    pub truncated: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LaneExecutionResult {
    pub lane_id: String,
    pub owner_gate_id: String,
    pub required: bool,
    pub depends_on: Vec<String>,
    pub entrypoint_id: String,
    pub entrypoint_identity: String,
    pub environment_identity: String,
    pub status: LaneTerminalStatus,
    pub exit_code: Option<i32>,
    pub stdout: ExecutionOutput,
    pub stderr: ExecutionOutput,
    pub elapsed_ms: u64,
    pub cleanup: CleanupState,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionAggregateStatus {
    Succeeded,
    Cancelled,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionReceipt {
    pub schema: String,
    pub receipt_id: String,
    pub action_plan_id: String,
    pub repository_id: String,
    pub approval_id: String,
    pub declaration_id: String,
    pub source_revision: String,
    pub topology_id: String,
    pub platform: ExecutionPlatform,
    pub selected_lane_count: usize,
    pub lanes: Vec<LaneExecutionResult>,
    pub aggregate_status: ExecutionAggregateStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionPlatform {
    pub os: String,
    pub architecture: String,
}

impl ExecutionReceipt {
    pub fn result_class(&self) -> ResultClass {
        match self.aggregate_status {
            ExecutionAggregateStatus::Succeeded => ResultClass::Success,
            ExecutionAggregateStatus::Cancelled => ResultClass::Cancelled,
            ExecutionAggregateStatus::Failed => ResultClass::Failed,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionOutcome {
    pub receipt: ExecutionReceipt,
    pub receipt_path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionVerification {
    pub schema: String,
    pub receipt_id: String,
    pub valid: bool,
}

#[derive(Serialize)]
struct EntrypointIdentityProjection<'a> {
    entrypoint_id: &'a str,
    command: &'a EntrypointCommand,
}

#[derive(Serialize)]
struct DeclarationIdentityProjection<'a> {
    schema: &'a str,
    source_revision: &'a str,
    entrypoints: &'a [OwnerEntrypoint],
}

#[derive(Serialize)]
struct ActionPlanIdentityProjection<'a> {
    schema: &'a str,
    repository_id: &'a str,
    source_revision: &'a str,
    topology_id: &'a str,
    declaration_id: &'a str,
    lanes: &'a [ActionLane],
}

#[derive(Serialize)]
struct ApprovalIdentityProjection<'a> {
    schema: &'a str,
    action_plan_id: &'a str,
    principal: &'a str,
    allowed_environment: &'a [String],
    expires_at: &'a str,
    revoked: bool,
}

pub fn owner_entrypoint_identity(entrypoint: &OwnerEntrypoint) -> String {
    sha256_json(&EntrypointIdentityProjection {
        entrypoint_id: &entrypoint.entrypoint_id,
        command: &entrypoint.command,
    })
}

pub fn owner_entrypoint_declaration_identity(declaration: &OwnerEntrypointDeclaration) -> String {
    sha256_json(&DeclarationIdentityProjection {
        schema: &declaration.schema,
        source_revision: &declaration.source_revision,
        entrypoints: &declaration.entrypoints,
    })
}

pub fn action_plan_identity(plan: &ActionPlan) -> String {
    sha256_json(&ActionPlanIdentityProjection {
        schema: &plan.schema,
        repository_id: &plan.repository_id,
        source_revision: &plan.source_revision,
        topology_id: &plan.topology_id,
        declaration_id: &plan.declaration_id,
        lanes: &plan.lanes,
    })
}

pub fn execution_approval_identity(approval: &ExecutionApproval) -> String {
    sha256_json(&ApprovalIdentityProjection {
        schema: &approval.schema,
        action_plan_id: &approval.action_plan_id,
        principal: &approval.principal,
        allowed_environment: &approval.allowed_environment,
        expires_at: &approval.expires_at,
        revoked: approval.revoked,
    })
}

pub fn file_content_identity(path: &Path) -> Result<String, CoreError> {
    let mut file = fs::File::open(path).map_err(|_| {
        execution_error(
            ResultClass::Stale,
            "FERRIS-EXECUTION-FILE-UNAVAILABLE",
            "A bound execution file is unavailable.",
        )
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|_| {
            execution_error(
                ResultClass::Stale,
                "FERRIS-EXECUTION-FILE-READ-FAILED",
                "Ferris could not hash a bound execution file.",
            )
        })?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!(
        "sha256:{}",
        hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    ))
}

pub fn execute_action_plan(
    repository_root: &Path,
    requested_plan_id: &str,
) -> Result<ExecutionOutcome, CoreError> {
    let cancellation = AtomicBool::new(false);
    execute_action_plan_with_cancellation(repository_root, requested_plan_id, &cancellation)
}

pub fn execute_action_plan_with_cancellation(
    repository_root: &Path,
    requested_plan_id: &str,
    cancellation: &AtomicBool,
) -> Result<ExecutionOutcome, CoreError> {
    validate_sha256_identity(requested_plan_id, "Action Plan")?;
    let root = repository_root.canonicalize().map_err(|_| {
        execution_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTION-ROOT-INVALID",
            "The current repository root could not be canonicalized.",
        )
    })?;
    if !root.is_dir() {
        return Err(execution_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTION-ROOT-INVALID",
            "The current repository root is not a directory.",
        ));
    }

    let files = load_execution_files(&root, requested_plan_id)?;
    let prepared = validate_execution_files(&root, &files)?;
    let mut results = Vec::with_capacity(files.plan.lanes.len());

    let mut stop_launching = cancellation.load(Ordering::Acquire);
    for (lane, prepared_lane) in files.plan.lanes.iter().zip(&prepared) {
        let blocked = lane.depends_on.iter().any(|dependency| {
            results.iter().any(|result: &LaneExecutionResult| {
                result.lane_id == *dependency && result.status != LaneTerminalStatus::Succeeded
            })
        });
        if blocked {
            results.push(blocked_lane_result(lane));
            continue;
        }

        if stop_launching || cancellation.load(Ordering::Acquire) {
            stop_launching = true;
            results.push(cancelled_lane_result(lane));
        } else if revalidate_before_launch(&root, &files, lane).is_err() {
            results.push(prelaunch_error_result(lane));
        } else {
            let result = run_lane(lane, prepared_lane, cancellation);
            stop_launching = result.status == LaneTerminalStatus::Cancelled;
            results.push(result);
        }
    }

    let aggregate_status = aggregate_status(&results);
    let mut receipt = ExecutionReceipt {
        schema: EXECUTION_RECEIPT_SCHEMA.to_owned(),
        receipt_id: String::new(),
        action_plan_id: files.plan.action_plan_id.clone(),
        repository_id: files.plan.repository_id.clone(),
        approval_id: files.approval.approval_id.clone(),
        declaration_id: files.declaration.declaration_id.clone(),
        source_revision: files.plan.source_revision.clone(),
        topology_id: files.plan.topology_id.clone(),
        platform: current_execution_platform(),
        selected_lane_count: results.len(),
        lanes: results,
        aggregate_status,
    };
    receipt.receipt_id = execution_receipt_identity(&receipt);
    let receipt_path = write_receipt(&root, &receipt)?;
    Ok(ExecutionOutcome {
        receipt,
        receipt_path,
    })
}

pub fn verify_execution_receipt(path: &Path) -> Result<ExecutionVerification, CoreError> {
    let receipt = load_verified_execution_receipt(path)?;
    Ok(ExecutionVerification {
        schema: EXECUTION_VERIFICATION_SCHEMA.to_owned(),
        receipt_id: receipt.receipt_id,
        valid: true,
    })
}

pub fn load_verified_execution_receipt(path: &Path) -> Result<ExecutionReceipt, CoreError> {
    let receipt: ExecutionReceipt = read_strict_json(path, MAX_RECEIPT_BYTES, "receipt")?;
    validate_receipt(&receipt)?;
    Ok(receipt)
}

pub fn execution_receipt_identity(receipt: &ExecutionReceipt) -> String {
    let mut semantic = receipt.clone();
    semantic.receipt_id.clear();
    for lane in &mut semantic.lanes {
        lane.elapsed_ms = 0;
    }
    sha256_json(&semantic)
}

struct ExecutionFiles {
    plan_path: PathBuf,
    declaration_path: PathBuf,
    approval_path: PathBuf,
    plan: ActionPlan,
    declaration: OwnerEntrypointDeclaration,
    approval: ExecutionApproval,
}

struct PreparedLane {
    executable: PathBuf,
    working_directory: PathBuf,
    environment: Vec<(String, OsString)>,
    environment_identity: String,
    redaction_tokens: Vec<Vec<u8>>,
}

fn load_execution_files(root: &Path, requested_plan_id: &str) -> Result<ExecutionFiles, CoreError> {
    let plan_path = execution_file_path(root, "action-plans", requested_plan_id)?;
    let plan: ActionPlan = read_strict_json(&plan_path, MAX_EXECUTION_FILE_BYTES, "Action Plan")?;
    validate_execution_file_identity(
        &plan.schema,
        ACTION_PLAN_SCHEMA,
        &plan.action_plan_id,
        requested_plan_id,
        &action_plan_identity(&plan),
        "Action Plan",
    )?;

    validate_sha256_identity(&plan.declaration_id, "entrypoint declaration")?;
    validate_sha256_identity(&plan.approval_id, "execution approval")?;
    let declaration_path = execution_file_path(root, "entrypoints", &plan.declaration_id)?;
    let approval_path = execution_file_path(root, "approvals", &plan.approval_id)?;
    let declaration: OwnerEntrypointDeclaration = read_strict_json(
        &declaration_path,
        MAX_EXECUTION_FILE_BYTES,
        "entrypoint declaration",
    )?;
    let approval: ExecutionApproval = read_strict_json(
        &approval_path,
        MAX_EXECUTION_FILE_BYTES,
        "execution approval",
    )?;
    validate_execution_file_identity(
        &declaration.schema,
        OWNER_ENTRYPOINTS_SCHEMA,
        &declaration.declaration_id,
        &plan.declaration_id,
        &owner_entrypoint_declaration_identity(&declaration),
        "entrypoint declaration",
    )?;
    validate_execution_file_identity(
        &approval.schema,
        EXECUTION_APPROVAL_SCHEMA,
        &approval.approval_id,
        &plan.approval_id,
        &execution_approval_identity(&approval),
        "execution approval",
    )?;
    Ok(ExecutionFiles {
        plan_path,
        declaration_path,
        approval_path,
        plan,
        declaration,
        approval,
    })
}

fn validate_execution_files(
    root: &Path,
    files: &ExecutionFiles,
) -> Result<Vec<PreparedLane>, CoreError> {
    let plan = &files.plan;
    let declaration = &files.declaration;
    let approval = &files.approval;
    validate_source_revision(&plan.source_revision)?;
    validate_metadata(&plan.repository_id, "repository identity")?;
    if declaration.source_revision != plan.source_revision {
        return Err(stale(
            "FERRIS-EXECUTION-DECLARATION-REVISION-MISMATCH",
            "The entrypoint declaration source revision does not match the Action Plan.",
        ));
    }
    if approval.action_plan_id != plan.action_plan_id {
        return Err(stale(
            "FERRIS-EXECUTION-APPROVAL-PLAN-MISMATCH",
            "The approval does not bind the requested Action Plan.",
        ));
    }
    validate_metadata(&plan.topology_id, "topology identity")?;
    validate_metadata(&approval.principal, "approval principal")?;
    if approval.revoked {
        return Err(execution_error(
            ResultClass::Denied,
            "FERRIS-EXECUTION-APPROVAL-REVOKED",
            "The execution approval is revoked.",
        ));
    }
    if approval_expired(&approval.expires_at)? {
        return Err(execution_error(
            ResultClass::Denied,
            "FERRIS-EXECUTION-APPROVAL-EXPIRED",
            "The execution approval has expired.",
        ));
    }
    let current_revision = git_stdout(root, &["rev-parse", "HEAD"]).ok_or_else(|| {
        execution_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTION-REVISION-UNAVAILABLE",
            "Ferris could not read the repository source revision.",
        )
    })?;
    if current_revision != plan.source_revision {
        return Err(stale(
            "FERRIS-EXECUTION-SOURCE-REVISION-MISMATCH",
            "The repository source revision does not match the Action Plan.",
        ));
    }
    if plan.lanes.is_empty() || plan.lanes.len() > MAX_LANES {
        return Err(invalid(
            "FERRIS-EXECUTION-LANES-INVALID",
            "The Action Plan must contain a bounded non-empty lane list.",
        ));
    }

    let mut entrypoints = BTreeMap::new();
    for entrypoint in &declaration.entrypoints {
        validate_metadata(&entrypoint.entrypoint_id, "entrypoint ID")?;
        validate_sha256_identity(&entrypoint.entrypoint_identity, "entrypoint")?;
        if owner_entrypoint_identity(entrypoint) != entrypoint.entrypoint_identity {
            return Err(stale(
                "FERRIS-EXECUTION-ENTRYPOINT-IDENTITY-MISMATCH",
                "An owner entrypoint identity does not match its declaration.",
            ));
        }
        validate_command(&entrypoint.command)?;
        if entrypoints
            .insert(entrypoint.entrypoint_id.as_str(), entrypoint)
            .is_some()
        {
            return Err(invalid(
                "FERRIS-EXECUTION-ENTRYPOINT-DUPLICATE",
                "The entrypoint declaration contains a duplicate entrypoint ID.",
            ));
        }
    }

    let mut lane_ids = BTreeSet::new();
    let mut environment_union = BTreeSet::new();
    let mut prepared = Vec::with_capacity(plan.lanes.len());
    for lane in &plan.lanes {
        validate_metadata(&lane.lane_id, "lane ID")?;
        validate_metadata(&lane.owner_gate_id, "owner gate ID")?;
        if !lane_ids.insert(lane.lane_id.clone()) {
            return Err(invalid(
                "FERRIS-EXECUTION-LANE-DUPLICATE",
                "The Action Plan contains a duplicate lane ID.",
            ));
        }
        if lane
            .depends_on
            .iter()
            .any(|dependency| !lane_ids.contains(dependency))
        {
            return Err(invalid(
                "FERRIS-EXECUTION-DEPENDENCY-INVALID",
                "Lane dependencies must name only earlier selected lanes.",
            ));
        }
        if has_duplicates(&lane.depends_on) {
            return Err(invalid(
                "FERRIS-EXECUTION-DEPENDENCY-DUPLICATE",
                "A lane contains a duplicate dependency.",
            ));
        }
        let entrypoint = entrypoints
            .get(lane.entrypoint_id.as_str())
            .ok_or_else(|| {
                invalid(
                    "FERRIS-EXECUTION-ENTRYPOINT-UNKNOWN",
                    "An Action Plan lane references an unknown owner entrypoint.",
                )
            })?;
        if lane.entrypoint_identity != entrypoint.entrypoint_identity
            || lane.command != entrypoint.command
        {
            return Err(stale(
                "FERRIS-EXECUTION-COMMAND-DRIFT",
                "An Action Plan lane no longer matches its owner entrypoint.",
            ));
        }
        if lane.timeout_ms == 0 || lane.timeout_ms > MAX_TIMEOUT_MS {
            return Err(invalid(
                "FERRIS-EXECUTION-TIMEOUT-INVALID",
                "A lane timeout is outside the supported bound.",
            ));
        }
        if !(1..=MAX_STREAM_BYTES).contains(&lane.stdout_limit_bytes)
            || !(1..=MAX_STREAM_BYTES).contains(&lane.stderr_limit_bytes)
        {
            return Err(invalid(
                "FERRIS-EXECUTION-OUTPUT-BOUND-INVALID",
                "A lane output bound is outside the supported range.",
            ));
        }
        validate_command(&lane.command)?;
        environment_union.extend(lane.command.inherited_environment.iter().cloned());
        prepared.push(prepare_lane(root, lane)?);
    }

    validate_sorted_environment(&approval.allowed_environment)?;
    if environment_union
        != approval
            .allowed_environment
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
    {
        return Err(stale(
            "FERRIS-EXECUTION-APPROVAL-ENVIRONMENT-MISMATCH",
            "The approval environment allowlist does not exactly bind the Action Plan.",
        ));
    }
    Ok(prepared)
}

fn validate_command(command: &EntrypointCommand) -> Result<(), CoreError> {
    validate_metadata(&command.owner, "owner reference")?;
    validate_relative_path(&command.executable, false, "executable")?;
    validate_relative_path(&command.working_directory, true, "working directory")?;
    if command.argv.len() > MAX_ARGUMENTS
        || command
            .argv
            .iter()
            .any(|argument| argument.as_bytes().len() > MAX_METADATA_BYTES)
    {
        return Err(invalid(
            "FERRIS-EXECUTION-ARGV-INVALID",
            "The owner argv exceeds the supported bounds.",
        ));
    }
    if command.credential_class != "none" {
        return Err(execution_error(
            ResultClass::Unsupported,
            "FERRIS-EXECUTION-CREDENTIAL-CLASS-UNSUPPORTED",
            "GO-WP-003 supports only credential class none.",
        ));
    }
    validate_sorted_environment(&command.inherited_environment)?;
    if command.files.is_empty() || command.files.len() > MAX_FILES_PER_COMMAND {
        return Err(invalid(
            "FERRIS-EXECUTION-FILES-INVALID",
            "Every owner command must bind a bounded non-empty file list.",
        ));
    }
    let mut paths = BTreeSet::new();
    for file in &command.files {
        validate_relative_path(&file.path, false, "bound file")?;
        validate_sha256_identity(&file.identity, "bound file")?;
        if !paths.insert(file.path.as_str()) {
            return Err(invalid(
                "FERRIS-EXECUTION-FILE-DUPLICATE",
                "An owner command binds the same file more than once.",
            ));
        }
    }
    if !paths.contains(command.executable.as_str()) {
        return Err(invalid(
            "FERRIS-EXECUTION-EXECUTABLE-UNBOUND",
            "The owner executable must be present in the command file bindings.",
        ));
    }
    Ok(())
}

fn validate_sorted_environment(names: &[String]) -> Result<(), CoreError> {
    if names.len() > MAX_ENVIRONMENT_NAMES {
        return Err(invalid(
            "FERRIS-EXECUTION-ENVIRONMENT-INVALID",
            "The environment-name allowlist exceeds the supported bound.",
        ));
    }
    let mut previous: Option<&str> = None;
    for name in names {
        if !valid_environment_name(name)
            || *name != name.to_ascii_uppercase()
            || credential_environment_name(name)
        {
            return Err(execution_error(
                ResultClass::Unsupported,
                "FERRIS-EXECUTION-ENVIRONMENT-UNSUPPORTED",
                "The environment allowlist contains an invalid or credential-associated name.",
            ));
        }
        if previous.is_some_and(|value| value >= name.as_str()) {
            return Err(invalid(
                "FERRIS-EXECUTION-ENVIRONMENT-ORDER-INVALID",
                "Environment allowlists must be unique and sorted.",
            ));
        }
        previous = Some(name);
    }
    Ok(())
}

fn prepare_lane(root: &Path, lane: &ActionLane) -> Result<PreparedLane, CoreError> {
    validate_bound_files(root, &lane.command)?;
    let executable = canonical_repository_path(root, &lane.command.executable, false)?;
    let working_directory = canonical_repository_path(root, &lane.command.working_directory, true)?;
    let mut environment = Vec::new();
    let mut redaction_tokens = Vec::new();
    for name in &lane.command.inherited_environment {
        if let Some(value) = std::env::var_os(name) {
            let bytes = environment_value_bytes(&value)?;
            if bytes.len() >= MIN_REDACTION_TOKEN_BYTES {
                redaction_tokens.push(bytes);
            }
            environment.push((name.clone(), value));
        }
    }
    redaction_tokens.sort();
    redaction_tokens.dedup();
    let environment_identity = environment_identity(&environment)?;
    Ok(PreparedLane {
        executable,
        working_directory,
        environment,
        environment_identity,
        redaction_tokens,
    })
}

fn validate_bound_files(root: &Path, command: &EntrypointCommand) -> Result<(), CoreError> {
    for file in &command.files {
        let path = canonical_repository_path(root, &file.path, false)?;
        if file_content_identity(&path)? != file.identity {
            return Err(stale(
                "FERRIS-EXECUTION-FILE-IDENTITY-MISMATCH",
                "A bound execution file identity has changed.",
            ));
        }
    }
    Ok(())
}

fn revalidate_before_launch(
    root: &Path,
    files: &ExecutionFiles,
    lane: &ActionLane,
) -> Result<(), CoreError> {
    let plan: ActionPlan =
        read_strict_json(&files.plan_path, MAX_EXECUTION_FILE_BYTES, "Action Plan")?;
    let declaration: OwnerEntrypointDeclaration = read_strict_json(
        &files.declaration_path,
        MAX_EXECUTION_FILE_BYTES,
        "entrypoint declaration",
    )?;
    let approval: ExecutionApproval = read_strict_json(
        &files.approval_path,
        MAX_EXECUTION_FILE_BYTES,
        "execution approval",
    )?;
    if plan != files.plan
        || declaration != files.declaration
        || approval != files.approval
        || action_plan_identity(&plan) != plan.action_plan_id
        || owner_entrypoint_declaration_identity(&declaration) != declaration.declaration_id
        || execution_approval_identity(&approval) != approval.approval_id
    {
        return Err(stale(
            "FERRIS-EXECUTION-FILE-CHANGED",
            "The plan, approval, or entrypoint declaration changed before lane launch.",
        ));
    }
    if approval.revoked || approval_expired(&approval.expires_at)? {
        return Err(execution_error(
            ResultClass::Denied,
            "FERRIS-EXECUTION-APPROVAL-NOT-ACTIVE",
            "The execution approval is no longer active.",
        ));
    }
    if git_stdout(root, &["rev-parse", "HEAD"]).as_deref()
        != Some(files.plan.source_revision.as_str())
    {
        return Err(stale(
            "FERRIS-EXECUTION-SOURCE-REVISION-CHANGED",
            "The repository source revision changed before lane launch.",
        ));
    }
    validate_bound_files(root, &lane.command)
}

fn run_lane(
    lane: &ActionLane,
    prepared: &PreparedLane,
    cancellation: &AtomicBool,
) -> LaneExecutionResult {
    let started = Instant::now();
    let mut command = Command::new(&prepared.executable);
    command
        .args(&lane.command.argv)
        .current_dir(&prepared.working_directory)
        .env_clear();
    for (name, value) in &prepared.environment {
        command.env(name, value);
    }

    let process = run_owned_process(
        command,
        Duration::from_millis(lane.timeout_ms),
        lane.stdout_limit_bytes as usize,
        lane.stderr_limit_bytes as usize,
        &prepared.redaction_tokens,
        cancellation,
    );
    let elapsed_ms = started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
    match process {
        Ok(process) => LaneExecutionResult {
            lane_id: lane.lane_id.clone(),
            owner_gate_id: lane.owner_gate_id.clone(),
            required: lane.required,
            depends_on: lane.depends_on.clone(),
            entrypoint_id: lane.entrypoint_id.clone(),
            entrypoint_identity: lane.entrypoint_identity.clone(),
            environment_identity: prepared.environment_identity.clone(),
            status: process.status,
            exit_code: process.exit_code,
            stdout: process.stdout,
            stderr: process.stderr,
            elapsed_ms,
            cleanup: process.cleanup,
        },
        Err(_) => LaneExecutionResult {
            lane_id: lane.lane_id.clone(),
            owner_gate_id: lane.owner_gate_id.clone(),
            required: lane.required,
            depends_on: lane.depends_on.clone(),
            entrypoint_id: lane.entrypoint_id.clone(),
            entrypoint_identity: lane.entrypoint_identity.clone(),
            environment_identity: prepared.environment_identity.clone(),
            status: LaneTerminalStatus::InternalError,
            exit_code: None,
            stdout: empty_output(),
            stderr: empty_output(),
            elapsed_ms,
            cleanup: CleanupState::Complete,
        },
    }
}

fn blocked_lane_result(lane: &ActionLane) -> LaneExecutionResult {
    LaneExecutionResult {
        lane_id: lane.lane_id.clone(),
        owner_gate_id: lane.owner_gate_id.clone(),
        required: lane.required,
        depends_on: lane.depends_on.clone(),
        entrypoint_id: lane.entrypoint_id.clone(),
        entrypoint_identity: lane.entrypoint_identity.clone(),
        environment_identity: empty_environment_identity(),
        status: LaneTerminalStatus::BlockedByDependency,
        exit_code: None,
        stdout: empty_output(),
        stderr: empty_output(),
        elapsed_ms: 0,
        cleanup: CleanupState::Complete,
    }
}

fn cancelled_lane_result(lane: &ActionLane) -> LaneExecutionResult {
    LaneExecutionResult {
        lane_id: lane.lane_id.clone(),
        owner_gate_id: lane.owner_gate_id.clone(),
        required: lane.required,
        depends_on: lane.depends_on.clone(),
        entrypoint_id: lane.entrypoint_id.clone(),
        entrypoint_identity: lane.entrypoint_identity.clone(),
        environment_identity: empty_environment_identity(),
        status: LaneTerminalStatus::Cancelled,
        exit_code: None,
        stdout: empty_output(),
        stderr: empty_output(),
        elapsed_ms: 0,
        cleanup: CleanupState::Complete,
    }
}

fn prelaunch_error_result(lane: &ActionLane) -> LaneExecutionResult {
    LaneExecutionResult {
        lane_id: lane.lane_id.clone(),
        owner_gate_id: lane.owner_gate_id.clone(),
        required: lane.required,
        depends_on: lane.depends_on.clone(),
        entrypoint_id: lane.entrypoint_id.clone(),
        entrypoint_identity: lane.entrypoint_identity.clone(),
        environment_identity: empty_environment_identity(),
        status: LaneTerminalStatus::InternalError,
        exit_code: None,
        stdout: empty_output(),
        stderr: empty_output(),
        elapsed_ms: 0,
        cleanup: CleanupState::Complete,
    }
}

fn aggregate_status(results: &[LaneExecutionResult]) -> ExecutionAggregateStatus {
    if results
        .iter()
        .any(|result| result.status == LaneTerminalStatus::LeakedSecret)
    {
        ExecutionAggregateStatus::Failed
    } else if results.iter().all(|result| {
        (!result.required || result.status == LaneTerminalStatus::Succeeded)
            && result.cleanup == CleanupState::Complete
    }) {
        ExecutionAggregateStatus::Succeeded
    } else if results
        .iter()
        .any(|result| result.status == LaneTerminalStatus::Cancelled)
    {
        ExecutionAggregateStatus::Cancelled
    } else {
        ExecutionAggregateStatus::Failed
    }
}

fn write_receipt(root: &Path, receipt: &ExecutionReceipt) -> Result<PathBuf, CoreError> {
    let directory = root.join(".ferris").join("receipts");
    fs::create_dir_all(&directory).map_err(|_| {
        execution_error(
            ResultClass::Internal,
            "FERRIS-EXECUTION-RECEIPT-DIRECTORY-FAILED",
            "Ferris could not create the receipt output directory.",
        )
    })?;
    let path = directory.join(format!("{}.json", identity_hex(&receipt.receipt_id)));
    let mut bytes = serde_json::to_vec_pretty(receipt).map_err(|_| {
        execution_error(
            ResultClass::Internal,
            "FERRIS-EXECUTION-RECEIPT-SERIALIZE-FAILED",
            "Ferris could not serialize the execution receipt.",
        )
    })?;
    bytes.push(b'\n');
    fs::write(&path, bytes).map_err(|_| {
        execution_error(
            ResultClass::Internal,
            "FERRIS-EXECUTION-RECEIPT-WRITE-FAILED",
            "Ferris could not write the execution receipt.",
        )
    })?;
    Ok(path)
}

fn validate_receipt(receipt: &ExecutionReceipt) -> Result<(), CoreError> {
    if receipt.schema != EXECUTION_RECEIPT_SCHEMA {
        return Err(invalid(
            "FERRIS-VERIFY-SCHEMA-INVALID",
            "The receipt schema is unsupported.",
        ));
    }
    for (identity, label) in [
        (&receipt.receipt_id, "receipt"),
        (&receipt.action_plan_id, "Action Plan"),
        (&receipt.approval_id, "approval"),
        (&receipt.declaration_id, "declaration"),
    ] {
        validate_sha256_identity(identity, label)?;
    }
    validate_source_revision(&receipt.source_revision)?;
    validate_metadata(&receipt.repository_id, "repository identity")?;
    validate_metadata(&receipt.topology_id, "topology identity")?;
    validate_metadata(&receipt.platform.os, "platform operating system")?;
    validate_metadata(&receipt.platform.architecture, "platform architecture")?;
    if receipt.selected_lane_count != receipt.lanes.len()
        || receipt.lanes.is_empty()
        || receipt.lanes.len() > MAX_LANES
    {
        return Err(invalid(
            "FERRIS-VERIFY-LANE-COMPLETENESS-INVALID",
            "The receipt lane count is incomplete or invalid.",
        ));
    }
    let mut statuses = BTreeMap::new();
    for lane in &receipt.lanes {
        validate_metadata(&lane.lane_id, "lane ID")?;
        validate_metadata(&lane.owner_gate_id, "owner gate ID")?;
        validate_metadata(&lane.entrypoint_id, "entrypoint ID")?;
        validate_sha256_identity(&lane.entrypoint_identity, "entrypoint")?;
        validate_sha256_identity(&lane.environment_identity, "environment")?;
        validate_sha256_identity(&lane.stdout.digest, "stdout")?;
        validate_sha256_identity(&lane.stderr.digest, "stderr")?;
        if lane.stdout.diagnostic_tail.as_bytes().len() > MAX_DIAGNOSTIC_TAIL_BYTES * 3
            || lane.stderr.diagnostic_tail.as_bytes().len() > MAX_DIAGNOSTIC_TAIL_BYTES * 3
        {
            return Err(invalid(
                "FERRIS-VERIFY-OUTPUT-INVALID",
                "A receipt output is inconsistent.",
            ));
        }
        if has_duplicates(&lane.depends_on)
            || lane
                .depends_on
                .iter()
                .any(|dependency| !statuses.contains_key(dependency))
        {
            return Err(invalid(
                "FERRIS-VERIFY-DEPENDENCY-INVALID",
                "Receipt dependencies must name only earlier lanes.",
            ));
        }
        let dependency_failed = lane
            .depends_on
            .iter()
            .any(|dependency| statuses.get(dependency) != Some(&LaneTerminalStatus::Succeeded));
        if dependency_failed != (lane.status == LaneTerminalStatus::BlockedByDependency) {
            return Err(invalid(
                "FERRIS-VERIFY-DEPENDENCY-TERMINAL-INVALID",
                "A receipt dependency terminal state is inconsistent.",
            ));
        }
        if lane.status == LaneTerminalStatus::Succeeded && lane.exit_code != Some(0) {
            return Err(invalid(
                "FERRIS-VERIFY-EXIT-CODE-INVALID",
                "A succeeded lane does not preserve a zero owner exit code.",
            ));
        }
        if lane.status == LaneTerminalStatus::Failed && lane.exit_code.is_some_and(|code| code == 0)
        {
            return Err(invalid(
                "FERRIS-VERIFY-EXIT-CODE-INVALID",
                "A failed lane contains a zero owner exit code.",
            ));
        }
        if !matches!(
            lane.status,
            LaneTerminalStatus::Succeeded | LaneTerminalStatus::Failed
        ) && lane.exit_code.is_some()
        {
            return Err(invalid(
                "FERRIS-VERIFY-EXIT-CODE-INVALID",
                "A lane that did not reach an owner exit has an exit code.",
            ));
        }
        if statuses.insert(lane.lane_id.clone(), lane.status).is_some() {
            return Err(invalid(
                "FERRIS-VERIFY-LANE-DUPLICATE",
                "The receipt contains a duplicate lane result.",
            ));
        }
    }
    if aggregate_status(&receipt.lanes) != receipt.aggregate_status {
        return Err(invalid(
            "FERRIS-VERIFY-AGGREGATE-INVALID",
            "The receipt aggregate classification is inconsistent.",
        ));
    }
    if execution_receipt_identity(receipt) != receipt.receipt_id {
        return Err(stale(
            "FERRIS-VERIFY-RECEIPT-IDENTITY-MISMATCH",
            "The receipt semantic identity does not match its content.",
        ));
    }
    Ok(())
}

fn validate_execution_file_identity(
    schema: &str,
    expected_schema: &str,
    embedded: &str,
    named: &str,
    computed: &str,
    label: &str,
) -> Result<(), CoreError> {
    if schema != expected_schema {
        return Err(invalid(
            "FERRIS-EXECUTION-FILE-SCHEMA-INVALID",
            format!("The {label} schema is unsupported."),
        ));
    }
    validate_sha256_identity(embedded, label)?;
    if embedded != named || embedded != computed {
        return Err(stale(
            "FERRIS-EXECUTION-FILE-IDENTITY-MISMATCH",
            format!("The {label} filename, embedded identity, and content identity differ."),
        ));
    }
    Ok(())
}

fn execution_file_path(root: &Path, kind: &str, identity: &str) -> Result<PathBuf, CoreError> {
    validate_sha256_identity(identity, kind)?;
    let directory = root.join(".ferris").join(kind);
    let portable = directory.join(format!("{}.json", identity_hex(identity)));
    let literal = directory.join(format!("{identity}.json"));
    let canonical = [literal, portable]
        .into_iter()
        .find_map(|path| path.canonicalize().ok())
        .ok_or_else(|| {
            execution_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTION-FILE-UNAVAILABLE",
                format!("The required {kind} file is unavailable."),
            )
        })?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(invalid(
            "FERRIS-EXECUTION-FILE-PATH-INVALID",
            format!("The required {kind} file is outside the current repository."),
        ));
    }
    Ok(canonical)
}

fn read_strict_json<T: DeserializeOwned>(
    path: &Path,
    maximum_bytes: u64,
    label: &str,
) -> Result<T, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        execution_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTION-INPUT-UNAVAILABLE",
            format!("The {label} input is unavailable."),
        )
    })?;
    if !metadata.is_file() || metadata.len() > maximum_bytes {
        return Err(invalid(
            "FERRIS-EXECUTION-INPUT-BOUND-INVALID",
            format!("The {label} input is not a bounded regular file."),
        ));
    }
    let bytes = fs::read(path).map_err(|_| {
        execution_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTION-INPUT-READ-FAILED",
            format!("Ferris could not read the {label} input."),
        )
    })?;
    let value = serde_json::from_slice::<StrictJsonValue>(&bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| {
            invalid(
                "FERRIS-EXECUTION-INPUT-JSON-INVALID",
                format!("The {label} input is not strict JSON."),
            )
        })?;
    serde_json::from_value(value).map_err(|_| {
        invalid(
            "FERRIS-EXECUTION-INPUT-SHAPE-INVALID",
            format!("The {label} input does not match its strict schema."),
        )
    })
}

fn canonical_repository_path(
    root: &Path,
    relative: &str,
    directory: bool,
) -> Result<PathBuf, CoreError> {
    validate_relative_path(relative, directory, "repository path")?;
    let path = root.join(Path::new(relative));
    let canonical = path.canonicalize().map_err(|_| {
        stale(
            "FERRIS-EXECUTION-PATH-UNAVAILABLE",
            "A declared repository-relative path is unavailable.",
        )
    })?;
    if !canonical.starts_with(root)
        || (directory && !canonical.is_dir())
        || (!directory && !canonical.is_file())
    {
        return Err(invalid(
            "FERRIS-EXECUTION-PATH-ESCAPE",
            "A declared path escapes the canonical repository root or has the wrong kind.",
        ));
    }
    Ok(canonical)
}

fn validate_relative_path(value: &str, allow_dot: bool, label: &str) -> Result<(), CoreError> {
    if value.is_empty()
        || value.contains('\\')
        || (!allow_dot && value == ".")
        || Path::new(value).is_absolute()
        || Path::new(value).components().any(|component| {
            !matches!(component, Component::Normal(_))
                && !(allow_dot && matches!(component, Component::CurDir))
        })
    {
        return Err(invalid(
            "FERRIS-EXECUTION-RELATIVE-PATH-INVALID",
            format!("The {label} must be a portable repository-relative path."),
        ));
    }
    Ok(())
}

fn validate_source_revision(value: &str) -> Result<(), CoreError> {
    if !(40..=64).contains(&value.len())
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(
            "FERRIS-EXECUTION-SOURCE-REVISION-INVALID",
            "The source revision must be a lowercase Git object identity.",
        ));
    }
    Ok(())
}

fn validate_sha256_identity(value: &str, label: &str) -> Result<(), CoreError> {
    if value.len() != 71
        || !value.starts_with("sha256:")
        || !value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(
            "FERRIS-EXECUTION-IDENTITY-INVALID",
            format!("The {label} identity must be lowercase sha256:<64-hex>."),
        ));
    }
    Ok(())
}

fn validate_metadata(value: &str, label: &str) -> Result<(), CoreError> {
    if value.is_empty()
        || value.as_bytes().len() > MAX_METADATA_BYTES
        || value
            .chars()
            .any(|character| character.is_control() || character == '\u{7f}')
    {
        return Err(invalid(
            "FERRIS-EXECUTION-METADATA-INVALID",
            format!("The {label} is not bounded printable metadata."),
        ));
    }
    Ok(())
}

fn valid_environment_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte == b'_' || byte.is_ascii_alphabetic())
        && bytes.all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

fn credential_environment_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    [
        "SECRET",
        "TOKEN",
        "PASSWORD",
        "CREDENTIAL",
        "API_KEY",
        "PRIVATE_KEY",
        "AUTHORIZATION",
        "COOKIE",
    ]
    .iter()
    .any(|marker| upper.contains(marker))
}

fn approval_expired(expires_at: &str) -> Result<bool, CoreError> {
    let expiry = parse_rfc3339_utc(expires_at).ok_or_else(|| {
        invalid(
            "FERRIS-EXECUTION-APPROVAL-EXPIRY-INVALID",
            "Approval expiry must be a valid UTC RFC 3339 timestamp.",
        )
    })?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            execution_error(
                ResultClass::Internal,
                "FERRIS-EXECUTION-CLOCK-INVALID",
                "The system clock is before the Unix epoch.",
            )
        })?
        .as_secs();
    Ok(now >= expiry)
}

fn parse_rfc3339_utc(value: &str) -> Option<u64> {
    if !value.is_ascii() {
        return None;
    }
    let core = value.strip_suffix('Z')?;
    let core = core.split_once('.').map_or(core, |(seconds, fraction)| {
        if fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
            ""
        } else {
            seconds
        }
    });
    if core.len() != 19
        || &core[4..5] != "-"
        || &core[7..8] != "-"
        || &core[10..11] != "T"
        || &core[13..14] != ":"
        || &core[16..17] != ":"
    {
        return None;
    }
    let year = core[0..4].parse::<i32>().ok()?;
    let month = core[5..7].parse::<u32>().ok()?;
    let day = core[8..10].parse::<u32>().ok()?;
    let hour = core[11..13].parse::<u32>().ok()?;
    let minute = core[14..16].parse::<u32>().ok()?;
    let second = core[17..19].parse::<u32>().ok()?;
    if !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }
    let days = days_from_civil(year, month, day);
    if days < 0 {
        return None;
    }
    Some(
        (days as u64)
            .saturating_mul(86_400)
            .saturating_add(u64::from(hour) * 3600)
            .saturating_add(u64::from(minute) * 60)
            .saturating_add(u64::from(second)),
    )
}

#[cfg(unix)]
fn environment_value_bytes(value: &OsStr) -> Result<Vec<u8>, CoreError> {
    use std::os::unix::ffi::OsStrExt;
    Ok(value.as_bytes().to_vec())
}

#[cfg(windows)]
fn environment_value_bytes(value: &OsStr) -> Result<Vec<u8>, CoreError> {
    value
        .to_str()
        .map(|value| value.as_bytes().to_vec())
        .ok_or_else(|| {
            execution_error(
                ResultClass::Unsupported,
                "FERRIS-EXECUTION-ENVIRONMENT-VALUE-UNSUPPORTED",
                "An allowlisted environment value cannot be represented for redaction.",
            )
        })
}

#[cfg(not(any(unix, windows)))]
fn environment_value_bytes(value: &OsStr) -> Result<Vec<u8>, CoreError> {
    Ok(value.to_string_lossy().as_bytes().to_vec())
}

pub fn current_execution_platform() -> ExecutionPlatform {
    ExecutionPlatform {
        os: std::env::consts::OS.to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
    }
}

fn environment_identity(environment: &[(String, OsString)]) -> Result<String, CoreError> {
    let values = environment
        .iter()
        .map(|(name, value)| Ok((name, digest_bytes(&environment_value_bytes(value)?))))
        .collect::<Result<Vec<_>, CoreError>>()?;
    Ok(sha256_json(&values))
}

fn empty_environment_identity() -> String {
    sha256_json(&Vec::<(String, String)>::new())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 31,
    }
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let shifted_month = month as i32 + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * shifted_month + 2) / 5 + day as i32 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    i64::from(era * 146_097 + day_of_era - 719_468)
}

fn has_duplicates(values: &[String]) -> bool {
    let mut unique = BTreeSet::new();
    values.iter().any(|value| !unique.insert(value))
}

fn identity_hex(identity: &str) -> &str {
    identity.strip_prefix("sha256:").unwrap_or(identity)
}

fn sha256_json(value: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(value).expect("typed execution identity must serialize");
    let digest = Sha256::digest(bytes);
    format!(
        "sha256:{}",
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    )
}

fn invalid(code: &str, message: impl Into<String>) -> CoreError {
    execution_error(ResultClass::Invalid, code, message)
}

fn stale(code: &str, message: impl Into<String>) -> CoreError {
    execution_error(ResultClass::Stale, code, message)
}

fn execution_error(class: ResultClass, code: &str, message: impl Into<String>) -> CoreError {
    CoreError::new(
        class,
        code,
        message,
        vec!["Repair the plan or approval files and request a new approval.".to_owned()],
    )
}

struct ProcessOutcome {
    status: LaneTerminalStatus,
    exit_code: Option<i32>,
    stdout: ExecutionOutput,
    stderr: ExecutionOutput,
    cleanup: CleanupState,
}

#[derive(Clone)]
struct StreamState {
    retained: Vec<u8>,
    observed_bytes: u64,
    complete: bool,
    truncated: bool,
    failed: bool,
}

struct StreamReader {
    state: Arc<Mutex<StreamState>>,
}

const STOP_NONE: u8 = 0;
const STOP_OUTPUT_LIMIT: u8 = 1;
const STOP_LEAKED_SECRET: u8 = 2;
const STOP_READ_FAILURE: u8 = 3;
const STOP_TIMEOUT: u8 = 4;
const STOP_CANCELLED: u8 = 5;

fn run_owned_process(
    mut command: Command,
    timeout: Duration,
    stdout_limit: usize,
    stderr_limit: usize,
    redaction_tokens: &[Vec<u8>],
    cancellation: &AtomicBool,
) -> io::Result<ProcessOutcome> {
    configure_process_tree(&mut command);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut owner = ProcessOwner::spawn(command)?;
    let stdout = owner
        .child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("stdout pipe unavailable"))?;
    let stderr = owner
        .child
        .stderr
        .take()
        .ok_or_else(|| io::Error::other("stderr pipe unavailable"))?;
    let stop = Arc::new(AtomicU8::new(STOP_NONE));
    let stdout_reader = spawn_stream_reader(
        stdout,
        stdout_limit,
        redaction_tokens.to_vec(),
        Arc::clone(&stop),
    );
    let stderr_reader = spawn_stream_reader(
        stderr,
        stderr_limit,
        redaction_tokens.to_vec(),
        Arc::clone(&stop),
    );
    let started = Instant::now();
    let mut status = None;
    let mut requested_stop = STOP_NONE;

    loop {
        if cancellation.load(Ordering::Acquire) {
            requested_stop = STOP_CANCELLED;
            break;
        }
        let observed_stop = stop.load(Ordering::Acquire);
        if observed_stop != STOP_NONE {
            requested_stop = observed_stop;
            break;
        }
        if started.elapsed() >= timeout {
            requested_stop = STOP_TIMEOUT;
            break;
        }
        match owner.child.try_wait()? {
            Some(exit) => {
                status = Some(exit);
                break;
            }
            None => thread::sleep(PROCESS_POLL_INTERVAL),
        }
    }

    let tree_terminated = owner.terminate_tree();
    let direct_exited = wait_for_child(&mut owner.child, PROCESS_CLEANUP_TIMEOUT);
    let streams_settled = wait_for_streams(&stdout_reader, &stderr_reader, PROCESS_CLEANUP_TIMEOUT);
    if status.is_none() {
        status = owner.child.try_wait().ok().flatten();
    }
    let cleanup = if tree_terminated && direct_exited && streams_settled {
        CleanupState::Complete
    } else {
        CleanupState::Failed
    };
    let final_stop = match stop.load(Ordering::Acquire) {
        STOP_NONE => requested_stop,
        observed => observed,
    };
    let stdout_state = snapshot_stream(&stdout_reader);
    let stderr_state = snapshot_stream(&stderr_reader);
    let terminal = match final_stop {
        STOP_OUTPUT_LIMIT => LaneTerminalStatus::OutputLimitExceeded,
        STOP_LEAKED_SECRET => LaneTerminalStatus::LeakedSecret,
        STOP_READ_FAILURE => LaneTerminalStatus::InternalError,
        STOP_TIMEOUT => LaneTerminalStatus::TimedOut,
        STOP_CANCELLED => LaneTerminalStatus::Cancelled,
        _ if status.as_ref().is_some_and(ExitStatus::success) => LaneTerminalStatus::Succeeded,
        _ => LaneTerminalStatus::Failed,
    };
    let exit_code = match terminal {
        LaneTerminalStatus::Succeeded | LaneTerminalStatus::Failed => {
            status.and_then(|value| value.code())
        }
        _ => None,
    };
    Ok(ProcessOutcome {
        status: terminal,
        exit_code,
        stdout: execution_output(stdout_state, redaction_tokens),
        stderr: execution_output(stderr_state, redaction_tokens),
        cleanup,
    })
}

fn spawn_stream_reader(
    mut reader: impl Read + Send + 'static,
    limit: usize,
    redaction_tokens: Vec<Vec<u8>>,
    stop: Arc<AtomicU8>,
) -> StreamReader {
    let state = Arc::new(Mutex::new(StreamState {
        retained: Vec::new(),
        observed_bytes: 0,
        complete: false,
        truncated: false,
        failed: false,
    }));
    let thread_state = Arc::clone(&state);
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    if let Ok(mut state) = thread_state.lock() {
                        state.complete = true;
                    }
                    break;
                }
                Ok(count) => {
                    let Ok(mut state) = thread_state.lock() else {
                        let _ = stop.compare_exchange(
                            STOP_NONE,
                            STOP_READ_FAILURE,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        );
                        break;
                    };
                    state.observed_bytes = state.observed_bytes.saturating_add(count as u64);
                    let remaining = limit.saturating_sub(state.retained.len());
                    state
                        .retained
                        .extend_from_slice(&buffer[..count.min(remaining)]);
                    if contains_any(&state.retained, &redaction_tokens) {
                        state.truncated = true;
                        stop.store(STOP_LEAKED_SECRET, Ordering::Release);
                        break;
                    }
                    if count > remaining {
                        state.truncated = true;
                        let _ = stop.compare_exchange(
                            STOP_NONE,
                            STOP_OUTPUT_LIMIT,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        );
                        break;
                    }
                }
                Err(_) => {
                    if let Ok(mut state) = thread_state.lock() {
                        state.failed = true;
                    }
                    let _ = stop.compare_exchange(
                        STOP_NONE,
                        STOP_READ_FAILURE,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );
                    break;
                }
            }
        }
    });
    StreamReader { state }
}

fn wait_for_child(child: &mut Child, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return true,
            Ok(None) if Instant::now() < deadline => thread::sleep(PROCESS_POLL_INTERVAL),
            _ => return false,
        }
    }
}

fn wait_for_streams(stdout: &StreamReader, stderr: &StreamReader, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        let stdout = snapshot_stream(stdout);
        let stderr = snapshot_stream(stderr);
        if stream_settled(&stdout) && stream_settled(&stderr) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(PROCESS_POLL_INTERVAL);
    }
}

fn snapshot_stream(reader: &StreamReader) -> StreamState {
    reader
        .state
        .lock()
        .map(|state| state.clone())
        .unwrap_or(StreamState {
            retained: Vec::new(),
            observed_bytes: 0,
            complete: false,
            truncated: false,
            failed: true,
        })
}

fn stream_settled(stream: &StreamState) -> bool {
    stream.complete || stream.truncated || stream.failed
}

fn execution_output(mut stream: StreamState, redaction_tokens: &[Vec<u8>]) -> ExecutionOutput {
    stream.retained = redact_bytes(&stream.retained, redaction_tokens);
    let tail_start = stream
        .retained
        .len()
        .saturating_sub(MAX_DIAGNOSTIC_TAIL_BYTES);
    let tail = String::from_utf8_lossy(&stream.retained[tail_start..]).into_owned();
    ExecutionOutput {
        digest: digest_bytes(&stream.retained),
        diagnostic_tail: tail,
        retained_bytes: stream.retained.len() as u64,
        observed_bytes: stream.observed_bytes,
        truncated: stream.truncated || !stream.complete,
    }
}

fn empty_output() -> ExecutionOutput {
    ExecutionOutput {
        digest: digest_bytes(&[]),
        diagnostic_tail: String::new(),
        retained_bytes: 0,
        observed_bytes: 0,
        truncated: false,
    }
}

fn contains_any(haystack: &[u8], needles: &[Vec<u8>]) -> bool {
    needles
        .iter()
        .any(|needle| find_bytes(haystack, needle).is_some())
}

fn replace_bytes(haystack: &[u8], needle: &[u8], replacement: &[u8]) -> Vec<u8> {
    if needle.is_empty() {
        return haystack.to_vec();
    }
    let mut output = Vec::with_capacity(haystack.len());
    let mut offset = 0;
    while let Some(relative) = find_bytes(&haystack[offset..], needle) {
        let start = offset + relative;
        output.extend_from_slice(&haystack[offset..start]);
        output.extend_from_slice(replacement);
        offset = start + needle.len();
    }
    output.extend_from_slice(&haystack[offset..]);
    output
}

fn redact_bytes(value: &[u8], tokens: &[Vec<u8>]) -> Vec<u8> {
    let mut redacted = value.to_vec();
    for token in tokens {
        redacted = replace_bytes(&redacted, token, b"[REDACTED]");
        if let Some(prefix_length) = (MIN_REDACTION_TOKEN_BYTES..token.len())
            .rev()
            .find(|length| redacted.ends_with(&token[..*length]))
        {
            redacted.truncate(redacted.len() - prefix_length);
            redacted.extend_from_slice(b"[REDACTED]");
        }
    }
    redacted
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|candidate| candidate == needle)
}

struct ProcessOwner {
    child: Child,
    tree_terminated: bool,
    #[cfg(unix)]
    process_group: i32,
    #[cfg(windows)]
    job: WindowsJob,
}

impl ProcessOwner {
    fn spawn(mut command: Command) -> io::Result<Self> {
        #[cfg(windows)]
        let job = WindowsJob::new()?;
        let child = command.spawn()?;
        #[cfg(unix)]
        let process_group = child.id() as i32;
        #[cfg(windows)]
        if let Err(error) = job
            .assign(&child)
            .and_then(|()| resume_suspended_process_threads(child.id()))
        {
            let mut child = child;
            let _ = job.terminate();
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }
        Ok(Self {
            child,
            tree_terminated: false,
            #[cfg(unix)]
            process_group,
            #[cfg(windows)]
            job,
        })
    }

    fn terminate_tree(&mut self) -> bool {
        if self.tree_terminated {
            return true;
        }
        #[cfg(unix)]
        {
            // SAFETY: the child is created as the leader of this process group.
            let result = unsafe { libc::kill(-self.process_group, libc::SIGKILL) };
            self.tree_terminated =
                result == 0 || io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH);
        }
        #[cfg(windows)]
        {
            self.tree_terminated = self.job.terminate();
        }
        #[cfg(not(any(unix, windows)))]
        {
            self.tree_terminated = self.child.kill().is_ok();
        }
        self.tree_terminated
    }
}

impl Drop for ProcessOwner {
    fn drop(&mut self) {
        let _ = self.terminate_tree();
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
        }
        let _ = self.child.wait();
    }
}

#[cfg(unix)]
fn configure_process_tree(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
}

#[cfg(windows)]
fn configure_process_tree(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    use windows_sys::Win32::System::Threading::CREATE_SUSPENDED;
    command.creation_flags(CREATE_SUSPENDED);
}

#[cfg(not(any(unix, windows)))]
fn configure_process_tree(_command: &mut Command) {}

#[cfg(windows)]
struct WindowsJob {
    handle: windows_sys::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
struct WindowsHandle(windows_sys::Win32::Foundation::HANDLE);

#[cfg(windows)]
impl Drop for WindowsHandle {
    fn drop(&mut self) {
        // SAFETY: the handle is owned by this wrapper and is closed exactly once.
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

#[cfg(windows)]
fn resume_suspended_process_threads(process_id: u32) -> io::Result<()> {
    use std::mem::{size_of, zeroed};
    use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
    };
    use windows_sys::Win32::System::Threading::{OpenThread, ResumeThread, THREAD_SUSPEND_RESUME};

    // SAFETY: the snapshot call has no borrowed inputs and returns an owned handle.
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    if snapshot == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }
    let snapshot = WindowsHandle(snapshot);
    // SAFETY: zero initialization followed by the required dwSize is valid.
    let mut entry: THREADENTRY32 = unsafe { zeroed() };
    entry.dwSize = size_of::<THREADENTRY32>() as u32;
    // SAFETY: snapshot and entry remain valid for the duration of the call.
    if unsafe { Thread32First(snapshot.0, &mut entry) } == 0 {
        return Err(io::Error::last_os_error());
    }

    let mut resumed = 0_u32;
    loop {
        if entry.th32OwnerProcessID == process_id {
            // SAFETY: the enumerated thread ID is used only to obtain a scoped handle.
            let thread = unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, entry.th32ThreadID) };
            if thread.is_null() {
                return Err(io::Error::last_os_error());
            }
            let thread = WindowsHandle(thread);
            // CREATE_SUSPENDED must leave each initial thread with a positive count.
            let previous = unsafe { ResumeThread(thread.0) };
            if previous == u32::MAX {
                return Err(io::Error::last_os_error());
            }
            if previous == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "suspended process thread was already running",
                ));
            }
            let mut remaining = previous - 1;
            while remaining > 0 {
                // SAFETY: the thread handle remains live and owned by this scope.
                let previous = unsafe { ResumeThread(thread.0) };
                if previous == u32::MAX {
                    return Err(io::Error::last_os_error());
                }
                remaining = previous.saturating_sub(1);
            }
            resumed = resumed.saturating_add(1);
        }
        // SAFETY: snapshot and entry remain valid for the duration of the call.
        if unsafe { Thread32Next(snapshot.0, &mut entry) } == 0 {
            break;
        }
    }
    if resumed == 0 {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "suspended process thread was not found",
        ));
    }
    Ok(())
}

#[cfg(windows)]
impl WindowsJob {
    fn new() -> io::Result<Self> {
        use std::mem::{size_of, zeroed};
        use windows_sys::Win32::System::JobObjects::{
            CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        };

        // SAFETY: null security/name pointers request an unnamed job with defaults.
        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: zero is a valid baseline for this Windows information structure.
        let mut information: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { zeroed() };
        information.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        // SAFETY: the pointer and byte count describe `information` for this call.
        let configured = unsafe {
            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &information as *const _ as *const _,
                size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        };
        if configured == 0 {
            // SAFETY: handle was returned by CreateJobObjectW and is owned here.
            unsafe {
                windows_sys::Win32::Foundation::CloseHandle(handle);
            }
            return Err(io::Error::last_os_error());
        }
        Ok(Self { handle })
    }

    fn assign(&self, child: &Child) -> io::Result<()> {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;
        // SAFETY: both handles are live for the duration of the call.
        if unsafe {
            AssignProcessToJobObject(self.handle, child.as_raw_handle() as *mut std::ffi::c_void)
        } == 0
        {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn terminate(&self) -> bool {
        use windows_sys::Win32::System::JobObjects::TerminateJobObject;
        // SAFETY: the job handle remains owned by self.
        unsafe { TerminateJobObject(self.handle, 1) != 0 }
    }
}

#[cfg(windows)]
impl Drop for WindowsJob {
    fn drop(&mut self) {
        // SAFETY: handle was returned by CreateJobObjectW and is closed once here.
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.handle);
        }
    }
}
