use super::{
    CoreError, ExecutionPlatform, ExecutionReceipt, LaneExecutionResult, LaneTerminalStatus,
    ResultClass, StrictJsonValue, digest_bytes, load_verified_execution_receipt,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const ITERATION_REPLAY_REQUEST_SCHEMA: &str = "ferris.iteration-replay-request/v1";
pub const REMOTE_ITERATION_EVIDENCE_SCHEMA: &str = "ferris.remote-iteration-evidence/v1";
pub const ITERATION_REPLAY_REPORT_SCHEMA: &str = "ferris.iteration-replay-report/v1";

const MAX_REPLAY_REQUEST_BYTES: u64 = 1024 * 1024;
const MAX_REPLAY_CASES: usize = 1024;
const MAX_REPLAY_METADATA_BYTES: usize = 1024;
const MAX_RECEIPT_PATH_BYTES: usize = 4096;
const MAX_REMOTE_DURATION_MS: u64 = 7 * 24 * 60 * 60 * 1000;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayProvenance {
    Fixture,
    HistoricalReplay,
    ShadowObservation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteTerminalStatus {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerFailureCategory {
    OwnerActionable,
    InfrastructureOnly,
    SecretOnly,
    Unavailable,
    Cancelled,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteIterationIdentity {
    pub repository_id: String,
    pub pull_request_id: String,
    pub provider: String,
    pub pipeline: String,
    pub run_id: String,
    pub attempt: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteOutputEvidence {
    pub digest: String,
    pub complete: bool,
    pub truncated: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteIterationEvidence {
    pub schema: String,
    pub iteration: RemoteIterationIdentity,
    pub source_revision: String,
    pub owner_gate_id: String,
    pub lane_id: String,
    pub entrypoint_id: String,
    pub entrypoint_identity: String,
    pub platform: ExecutionPlatform,
    pub environment_identity: String,
    pub terminal_status: RemoteTerminalStatus,
    pub owner_failure_category: OwnerFailureCategory,
    pub failure_fingerprint: String,
    pub stdout: RemoteOutputEvidence,
    pub stderr: RemoteOutputEvidence,
    pub duration_ms: u64,
    pub evidence_complete: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IterationReplayCase {
    pub case_id: String,
    pub receipt_path: String,
    pub local_lane_id: String,
    pub local_failure_fingerprint: String,
    pub remote: RemoteIterationEvidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IterationReplayRequest {
    pub schema: String,
    pub provenance: ReplayProvenance,
    pub cases: Vec<IterationReplayCase>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IterationReplayClassification {
    PreventedIterationSupported,
    LocalDidNotFail,
    RemoteDidNotFail,
    RepositoryMismatch,
    SourceMismatch,
    PlatformMismatch,
    GateMismatch,
    LaneMismatch,
    EntrypointMismatch,
    EnvironmentMismatch,
    FailureEvidenceMismatch,
    InfrastructureOnly,
    SecretOnly,
    Unavailable,
    Cancelled,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FailedPullRequestIdentity {
    pub repository_id: String,
    pub pull_request_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IterationReplayCaseResult {
    pub case_id: String,
    pub repository_id: String,
    pub pull_request_id: String,
    pub remote_iteration: RemoteIterationIdentity,
    pub source_revision: String,
    pub receipt_id: String,
    pub classification: IterationReplayClassification,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRatio {
    pub numerator: usize,
    pub denominator: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayTargetStatus {
    FixtureNotEvaluated,
    Satisfied,
    Rejected,
    InsufficientEvidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayTarget {
    pub minimum_supported_iterations_per_eligible_failed_pr: ExactRatio,
    pub status: ReplayTargetStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IterationReplayCohort {
    pub failed_prs_observed: Vec<FailedPullRequestIdentity>,
    pub eligible_failed_prs: Vec<FailedPullRequestIdentity>,
    pub supported_prevented_iterations: usize,
    pub supported_prevented_iterations_per_eligible_failed_pr: ExactRatio,
    pub eligible_failed_prs_with_support: Vec<FailedPullRequestIdentity>,
    pub eligible_failed_pr_coverage: ExactRatio,
    pub target: ReplayTarget,
    pub avoided_source_revisions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IterationReplayReport {
    pub schema: String,
    pub report_id: String,
    pub provenance: ReplayProvenance,
    pub cases: Vec<IterationReplayCaseResult>,
    pub cohort: IterationReplayCohort,
}

pub fn create_iteration_replay_report(
    request_path: &Path,
) -> Result<IterationReplayReport, CoreError> {
    let canonical_request = request_path.canonicalize().map_err(|_| {
        replay_error(
            ResultClass::Blocked,
            "FERRIS-REPLAY-REQUEST-UNAVAILABLE",
            "The iteration replay request is unavailable.",
        )
    })?;
    let request_directory = canonical_request.parent().ok_or_else(|| {
        replay_error(
            ResultClass::Invalid,
            "FERRIS-REPLAY-REQUEST-PATH-INVALID",
            "The iteration replay request has no containing directory.",
        )
    })?;
    let request = read_request(&canonical_request)?;
    validate_request(&request)?;

    let mut case_ids = BTreeSet::new();
    let mut remote_iterations = BTreeSet::new();
    let mut failed_prs = BTreeSet::new();
    let mut eligible_failed_prs = BTreeSet::new();
    let mut supported_failed_prs = BTreeSet::new();
    let mut avoided_revisions = BTreeSet::new();
    let mut results = Vec::with_capacity(request.cases.len());

    for case in &request.cases {
        if !case_ids.insert(case.case_id.as_str()) {
            return Err(replay_invalid(
                "FERRIS-REPLAY-CASE-DUPLICATE",
                "The replay request contains a duplicate case ID.",
            ));
        }
        if !remote_iterations.insert(case.remote.iteration.clone()) {
            return Err(replay_invalid(
                "FERRIS-REPLAY-REMOTE-ITERATION-DUPLICATE",
                "The replay request contains a duplicate remote iteration identity.",
            ));
        }

        let receipt_path = resolve_receipt_path(request_directory, &case.receipt_path)?;
        let receipt = load_verified_execution_receipt(&receipt_path)?;
        let lane = receipt
            .lanes
            .iter()
            .find(|lane| lane.lane_id == case.local_lane_id)
            .ok_or_else(|| {
                replay_invalid(
                    "FERRIS-REPLAY-RECEIPT-LANE-UNKNOWN",
                    "A replay case names a lane that is absent from its verified receipt.",
                )
            })?;
        let pull_request = FailedPullRequestIdentity {
            repository_id: case.remote.iteration.repository_id.clone(),
            pull_request_id: case.remote.iteration.pull_request_id.clone(),
        };
        if case.remote.terminal_status == RemoteTerminalStatus::Failed {
            failed_prs.insert(pull_request.clone());
            if eligible_remote_failure(&case.remote) {
                eligible_failed_prs.insert(pull_request.clone());
            }
        }

        let classification = classify_case(case, &receipt, lane);
        if classification == IterationReplayClassification::PreventedIterationSupported {
            avoided_revisions.insert(case.remote.source_revision.clone());
            supported_failed_prs.insert(pull_request);
        }
        results.push(IterationReplayCaseResult {
            case_id: case.case_id.clone(),
            repository_id: case.remote.iteration.repository_id.clone(),
            pull_request_id: case.remote.iteration.pull_request_id.clone(),
            remote_iteration: case.remote.iteration.clone(),
            source_revision: case.remote.source_revision.clone(),
            receipt_id: receipt.receipt_id,
            classification,
        });
    }

    results.sort_by(|left, right| left.case_id.cmp(&right.case_id));
    let supported = results
        .iter()
        .filter(|case| {
            case.classification == IterationReplayClassification::PreventedIterationSupported
        })
        .count();
    let eligible_count = eligible_failed_prs.len();
    let target_status = match request.provenance {
        ReplayProvenance::Fixture => ReplayTargetStatus::FixtureNotEvaluated,
        _ if eligible_count == 0 => ReplayTargetStatus::InsufficientEvidence,
        _ if supported >= eligible_count => ReplayTargetStatus::Satisfied,
        _ => ReplayTargetStatus::Rejected,
    };
    let mut report = IterationReplayReport {
        schema: ITERATION_REPLAY_REPORT_SCHEMA.to_owned(),
        report_id: String::new(),
        provenance: request.provenance,
        cases: results,
        cohort: IterationReplayCohort {
            failed_prs_observed: failed_prs.into_iter().collect(),
            eligible_failed_prs: eligible_failed_prs.into_iter().collect(),
            supported_prevented_iterations: supported,
            supported_prevented_iterations_per_eligible_failed_pr: ExactRatio {
                numerator: supported,
                denominator: eligible_count,
            },
            eligible_failed_prs_with_support: supported_failed_prs.iter().cloned().collect(),
            eligible_failed_pr_coverage: ExactRatio {
                numerator: supported_failed_prs.len(),
                denominator: eligible_count,
            },
            target: ReplayTarget {
                minimum_supported_iterations_per_eligible_failed_pr: ExactRatio {
                    numerator: 1,
                    denominator: 1,
                },
                status: target_status,
            },
            avoided_source_revisions: avoided_revisions.into_iter().collect(),
        },
    };
    report.report_id = iteration_replay_report_identity(&report);
    Ok(report)
}

pub fn iteration_replay_report_identity(report: &IterationReplayReport) -> String {
    let mut semantic = report.clone();
    semantic.report_id.clear();
    digest_bytes(&serde_json::to_vec(&semantic).expect("typed replay reports must serialize"))
}

fn read_request(path: &Path) -> Result<IterationReplayRequest, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        replay_error(
            ResultClass::Blocked,
            "FERRIS-REPLAY-REQUEST-UNAVAILABLE",
            "The iteration replay request is unavailable.",
        )
    })?;
    if !metadata.is_file() || metadata.len() > MAX_REPLAY_REQUEST_BYTES {
        return Err(replay_invalid(
            "FERRIS-REPLAY-REQUEST-BOUND-INVALID",
            "The iteration replay request is not a bounded regular file.",
        ));
    }
    let bytes = fs::read(path).map_err(|_| {
        replay_error(
            ResultClass::Blocked,
            "FERRIS-REPLAY-REQUEST-READ-FAILED",
            "Ferris could not read the iteration replay request.",
        )
    })?;
    let value = serde_json::from_slice::<StrictJsonValue>(&bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| {
            replay_invalid(
                "FERRIS-REPLAY-REQUEST-JSON-INVALID",
                "The iteration replay request is not strict JSON.",
            )
        })?;
    serde_json::from_value(value).map_err(|_| {
        replay_invalid(
            "FERRIS-REPLAY-REQUEST-SHAPE-INVALID",
            "The iteration replay request does not match its strict schema.",
        )
    })
}

fn validate_request(request: &IterationReplayRequest) -> Result<(), CoreError> {
    if request.schema != ITERATION_REPLAY_REQUEST_SCHEMA {
        return Err(replay_invalid(
            "FERRIS-REPLAY-REQUEST-SCHEMA-INVALID",
            "The iteration replay request schema is unsupported.",
        ));
    }
    if request.cases.is_empty() || request.cases.len() > MAX_REPLAY_CASES {
        return Err(replay_invalid(
            "FERRIS-REPLAY-CASE-BOUND-INVALID",
            "The replay request must contain a bounded non-empty case list.",
        ));
    }
    for case in &request.cases {
        for (value, label) in [
            (&case.case_id, "case ID"),
            (&case.local_lane_id, "local lane ID"),
            (&case.remote.iteration.repository_id, "remote repository ID"),
            (
                &case.remote.iteration.pull_request_id,
                "remote pull request ID",
            ),
            (&case.remote.iteration.provider, "remote provider"),
            (&case.remote.iteration.pipeline, "remote pipeline"),
            (&case.remote.iteration.run_id, "remote run ID"),
            (&case.remote.owner_gate_id, "remote owner gate ID"),
            (&case.remote.lane_id, "remote lane ID"),
            (&case.remote.entrypoint_id, "remote entrypoint ID"),
            (&case.remote.platform.os, "remote platform operating system"),
            (
                &case.remote.platform.architecture,
                "remote platform architecture",
            ),
        ] {
            validate_metadata(value, label)?;
        }
        validate_receipt_relative_path(&case.receipt_path)?;
        if case.remote.schema != REMOTE_ITERATION_EVIDENCE_SCHEMA {
            return Err(replay_invalid(
                "FERRIS-REPLAY-REMOTE-SCHEMA-INVALID",
                "A remote iteration evidence schema is unsupported.",
            ));
        }
        if case.remote.iteration.attempt == 0 {
            return Err(replay_invalid(
                "FERRIS-REPLAY-REMOTE-ITERATION-INVALID",
                "A remote iteration attempt must be greater than zero.",
            ));
        }
        validate_source_revision(&case.remote.source_revision)?;
        validate_sha256(&case.remote.entrypoint_identity, "remote entrypoint")?;
        validate_sha256(&case.remote.environment_identity, "remote environment")?;
        validate_sha256(&case.local_failure_fingerprint, "local failure fingerprint")?;
        validate_sha256(
            &case.remote.failure_fingerprint,
            "remote failure fingerprint",
        )?;
        validate_output_evidence(&case.remote.stdout, "remote stdout")?;
        validate_output_evidence(&case.remote.stderr, "remote stderr")?;
        if case.remote.duration_ms > MAX_REMOTE_DURATION_MS {
            return Err(replay_invalid(
                "FERRIS-REPLAY-REMOTE-DURATION-INVALID",
                "A remote iteration duration exceeds the supported bound.",
            ));
        }
        if !case.remote.evidence_complete {
            return Err(replay_error(
                ResultClass::Incomplete,
                "FERRIS-REPLAY-REMOTE-EVIDENCE-INCOMPLETE",
                "A remote iteration evidence record is explicitly incomplete.",
            ));
        }
        let category_matches_status = match case.remote.terminal_status {
            RemoteTerminalStatus::Succeeded => {
                case.remote.owner_failure_category == OwnerFailureCategory::Unknown
            }
            RemoteTerminalStatus::Failed => {
                case.remote.owner_failure_category != OwnerFailureCategory::Cancelled
            }
            RemoteTerminalStatus::Cancelled => {
                case.remote.owner_failure_category == OwnerFailureCategory::Cancelled
            }
        };
        if !category_matches_status {
            return Err(replay_invalid(
                "FERRIS-REPLAY-REMOTE-CLASSIFICATION-INVALID",
                "A remote terminal status and owner failure category are inconsistent.",
            ));
        }
    }
    Ok(())
}

fn validate_output_evidence(evidence: &RemoteOutputEvidence, label: &str) -> Result<(), CoreError> {
    validate_sha256(&evidence.digest, label)?;
    if evidence.complete && evidence.truncated {
        return Err(replay_invalid(
            "FERRIS-REPLAY-REMOTE-OUTPUT-INVALID",
            "Remote output cannot be both complete and truncated.",
        ));
    }
    Ok(())
}

fn resolve_receipt_path(root: &Path, value: &str) -> Result<PathBuf, CoreError> {
    validate_receipt_relative_path(value)?;
    let canonical = root.join(Path::new(value)).canonicalize().map_err(|_| {
        replay_error(
            ResultClass::Blocked,
            "FERRIS-REPLAY-RECEIPT-UNAVAILABLE",
            "A referenced execution receipt is unavailable.",
        )
    })?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(replay_invalid(
            "FERRIS-REPLAY-RECEIPT-PATH-ESCAPE",
            "A receipt path escapes the canonical replay request directory or is not a file.",
        ));
    }
    Ok(canonical)
}

fn validate_receipt_relative_path(value: &str) -> Result<(), CoreError> {
    if value.is_empty()
        || value.len() > MAX_RECEIPT_PATH_BYTES
        || value.contains('\\')
        || Path::new(value).is_absolute()
        || Path::new(value)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(replay_invalid(
            "FERRIS-REPLAY-RECEIPT-PATH-INVALID",
            "Receipt paths must be portable paths relative to the replay request directory.",
        ));
    }
    Ok(())
}

fn eligible_remote_failure(remote: &RemoteIterationEvidence) -> bool {
    remote.terminal_status == RemoteTerminalStatus::Failed
        && remote.owner_failure_category == OwnerFailureCategory::OwnerActionable
        && remote.stdout.complete
        && remote.stderr.complete
        && !remote.stdout.truncated
        && !remote.stderr.truncated
}

fn classify_case(
    case: &IterationReplayCase,
    receipt: &ExecutionReceipt,
    lane: &LaneExecutionResult,
) -> IterationReplayClassification {
    match case.remote.terminal_status {
        RemoteTerminalStatus::Succeeded => {
            return IterationReplayClassification::RemoteDidNotFail;
        }
        RemoteTerminalStatus::Cancelled => return IterationReplayClassification::Cancelled,
        RemoteTerminalStatus::Failed => {}
    }
    match case.remote.owner_failure_category {
        OwnerFailureCategory::InfrastructureOnly => {
            return IterationReplayClassification::InfrastructureOnly;
        }
        OwnerFailureCategory::SecretOnly => return IterationReplayClassification::SecretOnly,
        OwnerFailureCategory::Unavailable => return IterationReplayClassification::Unavailable,
        OwnerFailureCategory::Cancelled => return IterationReplayClassification::Cancelled,
        OwnerFailureCategory::Unknown => return IterationReplayClassification::Unknown,
        OwnerFailureCategory::OwnerActionable => {}
    }
    if lane.status != LaneTerminalStatus::Failed {
        return IterationReplayClassification::LocalDidNotFail;
    }
    if receipt.repository_id != case.remote.iteration.repository_id {
        return IterationReplayClassification::RepositoryMismatch;
    }
    if receipt.source_revision != case.remote.source_revision {
        return IterationReplayClassification::SourceMismatch;
    }
    if receipt.platform != case.remote.platform {
        return IterationReplayClassification::PlatformMismatch;
    }
    if lane.owner_gate_id != case.remote.owner_gate_id {
        return IterationReplayClassification::GateMismatch;
    }
    if case.local_lane_id != case.remote.lane_id {
        return IterationReplayClassification::LaneMismatch;
    }
    if lane.entrypoint_id != case.remote.entrypoint_id
        || lane.entrypoint_identity != case.remote.entrypoint_identity
    {
        return IterationReplayClassification::EntrypointMismatch;
    }
    if lane.environment_identity != case.remote.environment_identity {
        return IterationReplayClassification::EnvironmentMismatch;
    }
    if case.local_failure_fingerprint != case.remote.failure_fingerprint
        || lane.stdout.truncated
        || lane.stderr.truncated
        || !case.remote.stdout.complete
        || !case.remote.stderr.complete
        || case.remote.stdout.truncated
        || case.remote.stderr.truncated
    {
        return IterationReplayClassification::FailureEvidenceMismatch;
    }
    IterationReplayClassification::PreventedIterationSupported
}

fn validate_metadata(value: &str, label: &str) -> Result<(), CoreError> {
    if value.is_empty()
        || value.len() > MAX_REPLAY_METADATA_BYTES
        || value
            .chars()
            .any(|character| character.is_control() || character == '\u{7f}')
    {
        return Err(replay_invalid(
            "FERRIS-REPLAY-METADATA-INVALID",
            format!("The {label} is not bounded printable metadata."),
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
        return Err(replay_invalid(
            "FERRIS-REPLAY-SOURCE-REVISION-INVALID",
            "A remote source revision must be a lowercase Git object identity.",
        ));
    }
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> Result<(), CoreError> {
    if value.len() != 71
        || !value.starts_with("sha256:")
        || !value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(replay_invalid(
            "FERRIS-REPLAY-IDENTITY-INVALID",
            format!("The {label} identity must be lowercase sha256:<64-hex>."),
        ));
    }
    Ok(())
}

fn replay_invalid(code: &str, message: impl Into<String>) -> CoreError {
    replay_error(ResultClass::Invalid, code, message)
}

fn replay_error(class: ResultClass, code: &str, message: impl Into<String>) -> CoreError {
    CoreError::new(
        class,
        code,
        message,
        vec!["Repair the replay request or its referenced receipt evidence and retry.".to_owned()],
    )
}
