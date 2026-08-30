use super::{CoreError, ResultClass, StrictJsonValue, digest_bytes, valid_portable_id};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const SCHEDULE_REPLAY_REQUEST_SCHEMA: &str = "ferris.schedule-replay-request/v1";
pub const SCHEDULE_REPLAY_REPORT_SCHEMA: &str = "ferris.schedule-replay-report/v1";
pub const SCHEDULE_SUCCESS_PREDICATE_ID: &str = "ferris.all-required-nodes-succeeded/v1";

const MAX_SCHEDULE_REQUEST_BYTES: u64 = 1024 * 1024;
const MAX_SCHEDULE_NODES: usize = 1024;
const MAX_SCHEDULE_DEPENDENCIES: usize = 50_000;
const MAX_OBSERVED_TIME_MS: u64 = 7 * 24 * 60 * 60 * 1000;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservedTerminalOutcome {
    Succeeded,
    Failed,
    Cancelled,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticValue {
    High,
    Standard,
    Low,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchedulingNode {
    pub node_id: String,
    pub required: bool,
    pub dependencies: Vec<String>,
    pub observed_start_ms: u64,
    pub observed_finish_ms: u64,
    pub observed_outcome: ObservedTerminalOutcome,
    pub owner_decisive_failure: bool,
    pub cancellable: Option<bool>,
    pub diagnostic_value: Option<DiagnosticValue>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleReplayRequest {
    pub schema: String,
    pub repository_id: String,
    pub pull_request_id: String,
    pub source_revision: String,
    pub topology_id: String,
    pub nodes: Vec<SchedulingNode>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchedulingProfile {
    ConservativeOwnerOrder,
    FailFast,
    FlushOut,
    Balanced,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchedulingDecision {
    Continued,
    BlockedByDependency,
    NotStartedAfterFailure,
    CancelledAfterFailure,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchedulingDecisionReason {
    UnsatisfiedPrerequisite,
    OwnerAuthorizedFailFast,
    OwnerAuthorizedLowDiagnosticValue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchedulingNodeDecision {
    pub node_id: String,
    pub required: bool,
    pub observed_outcome: ObservedTerminalOutcome,
    pub decision: SchedulingDecision,
    pub reason: Option<SchedulingDecisionReason>,
    pub trigger_node_id: Option<String>,
    pub projected_saved_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchedulingProfileResult {
    pub profile: SchedulingProfile,
    pub repository_id: String,
    pub pull_request_id: String,
    pub source_revision: String,
    pub topology_id: String,
    pub graph_id: String,
    pub required_node_ids: Vec<String>,
    pub success_predicate_id: String,
    pub projected_success: bool,
    pub node_decisions: Vec<SchedulingNodeDecision>,
    pub total_projected_saved_ms: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchedulingProductTargetStatus {
    InsufficientEvidence,
    ObservationOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleReplayReport {
    pub schema: String,
    pub report_id: String,
    pub repository_id: String,
    pub pull_request_id: String,
    pub source_revision: String,
    pub topology_id: String,
    pub graph_id: String,
    pub required_node_ids: Vec<String>,
    pub success_predicate_id: String,
    pub observed_success: bool,
    pub owner_policy_complete: bool,
    pub product_target_status: SchedulingProductTargetStatus,
    pub decisive_failure_node_id: Option<String>,
    pub profiles: Vec<SchedulingProfileResult>,
}

pub fn create_schedule_replay_report(
    request_path: &Path,
) -> Result<ScheduleReplayReport, CoreError> {
    let canonical_request = request_path.canonicalize().map_err(|_| {
        scheduling_error(
            ResultClass::Blocked,
            "FERRIS-SCHEDULE-REQUEST-UNAVAILABLE",
            "The scheduling replay request is unavailable.",
        )
    })?;
    let request = read_request(&canonical_request)?;
    let (identity, nodes) = validate_and_order_request(request)?;
    let graph_id = schedule_graph_identity(&nodes);
    let required_node_ids = nodes
        .iter()
        .filter(|node| node.required)
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    let observed_success = nodes
        .iter()
        .filter(|node| node.required)
        .all(|node| node.observed_outcome == ObservedTerminalOutcome::Succeeded);
    let owner_policy_complete = nodes
        .iter()
        .all(|node| node.cancellable.is_some() && node.diagnostic_value.is_some());
    let decisive_failure = nodes
        .iter()
        .filter(|node| {
            node.owner_decisive_failure && node.observed_outcome == ObservedTerminalOutcome::Failed
        })
        .min_by(|left, right| {
            (left.observed_finish_ms, &left.node_id)
                .cmp(&(right.observed_finish_ms, &right.node_id))
        });
    let product_target_status = if owner_policy_complete {
        SchedulingProductTargetStatus::ObservationOnly
    } else {
        SchedulingProductTargetStatus::InsufficientEvidence
    };
    let profiles = [
        SchedulingProfile::ConservativeOwnerOrder,
        SchedulingProfile::FailFast,
        SchedulingProfile::FlushOut,
        SchedulingProfile::Balanced,
    ]
    .into_iter()
    .map(|profile| {
        project_profile(
            profile,
            &identity,
            &graph_id,
            &required_node_ids,
            &nodes,
            decisive_failure,
        )
    })
    .collect::<Result<Vec<_>, CoreError>>()?;

    let mut report = ScheduleReplayReport {
        schema: SCHEDULE_REPLAY_REPORT_SCHEMA.to_owned(),
        report_id: String::new(),
        repository_id: identity.repository_id,
        pull_request_id: identity.pull_request_id,
        source_revision: identity.source_revision,
        topology_id: identity.topology_id,
        graph_id,
        required_node_ids,
        success_predicate_id: SCHEDULE_SUCCESS_PREDICATE_ID.to_owned(),
        observed_success,
        owner_policy_complete,
        product_target_status,
        decisive_failure_node_id: decisive_failure.map(|node| node.node_id.clone()),
        profiles,
    };
    report.report_id = schedule_replay_report_identity(&report);
    Ok(report)
}

pub fn schedule_replay_report_identity(report: &ScheduleReplayReport) -> String {
    let mut semantic = report.clone();
    semantic.report_id.clear();
    digest_bytes(
        &serde_json::to_vec(&semantic).expect("typed scheduling replay reports must serialize"),
    )
}

#[derive(Clone)]
struct RequestIdentity {
    repository_id: String,
    pull_request_id: String,
    source_revision: String,
    topology_id: String,
}

fn read_request(path: &Path) -> Result<ScheduleReplayRequest, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        scheduling_error(
            ResultClass::Blocked,
            "FERRIS-SCHEDULE-REQUEST-UNAVAILABLE",
            "The scheduling replay request is unavailable.",
        )
    })?;
    if !metadata.is_file() || metadata.len() > MAX_SCHEDULE_REQUEST_BYTES {
        return Err(scheduling_invalid(
            "FERRIS-SCHEDULE-REQUEST-BOUND-INVALID",
            "The scheduling replay request is not a bounded regular file.",
        ));
    }
    let bytes = fs::read(path).map_err(|_| {
        scheduling_error(
            ResultClass::Blocked,
            "FERRIS-SCHEDULE-REQUEST-READ-FAILED",
            "Ferris could not read the scheduling replay request.",
        )
    })?;
    let value = serde_json::from_slice::<StrictJsonValue>(&bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| {
            scheduling_invalid(
                "FERRIS-SCHEDULE-REQUEST-JSON-INVALID",
                "The scheduling replay request is not strict JSON.",
            )
        })?;
    serde_json::from_value(value).map_err(|_| {
        scheduling_invalid(
            "FERRIS-SCHEDULE-REQUEST-SHAPE-INVALID",
            "The scheduling replay request does not match its strict schema.",
        )
    })
}

fn validate_and_order_request(
    request: ScheduleReplayRequest,
) -> Result<(RequestIdentity, Vec<SchedulingNode>), CoreError> {
    if request.schema != SCHEDULE_REPLAY_REQUEST_SCHEMA {
        return Err(scheduling_invalid(
            "FERRIS-SCHEDULE-REQUEST-SCHEMA-INVALID",
            "The scheduling replay request schema is unsupported.",
        ));
    }
    for (value, label) in [
        (&request.repository_id, "repository"),
        (&request.pull_request_id, "pull request"),
        (&request.topology_id, "topology"),
    ] {
        validate_identity(value, label)?;
    }
    validate_source_revision(&request.source_revision)?;
    if request.nodes.is_empty() || request.nodes.len() > MAX_SCHEDULE_NODES {
        return Err(scheduling_invalid(
            "FERRIS-SCHEDULE-NODE-BOUND-INVALID",
            "The scheduling replay request must contain a bounded non-empty node list.",
        ));
    }

    let identity = RequestIdentity {
        repository_id: request.repository_id,
        pull_request_id: request.pull_request_id,
        source_revision: request.source_revision,
        topology_id: request.topology_id,
    };
    let mut nodes_by_id = BTreeMap::new();
    for node in request.nodes {
        validate_identity(&node.node_id, "node")?;
        if node.observed_start_ms > MAX_OBSERVED_TIME_MS
            || node.observed_finish_ms > MAX_OBSERVED_TIME_MS
            || node.observed_finish_ms < node.observed_start_ms
        {
            return Err(scheduling_invalid(
                "FERRIS-SCHEDULE-NODE-TIMING-INVALID",
                "A scheduling node has impossible or out-of-bounds observed timing.",
            ));
        }
        if node.owner_decisive_failure
            && (!node.required || node.observed_outcome != ObservedTerminalOutcome::Failed)
        {
            return Err(scheduling_invalid(
                "FERRIS-SCHEDULE-DECISIVE-FAILURE-INVALID",
                "Only an observed failed required node may be marked as an owner-decisive failure.",
            ));
        }
        let node_id = node.node_id.clone();
        if nodes_by_id.insert(node_id, node).is_some() {
            return Err(scheduling_invalid(
                "FERRIS-SCHEDULE-NODE-DUPLICATE",
                "The scheduling replay request contains a duplicate node identity.",
            ));
        }
    }
    if !nodes_by_id.values().any(|node| node.required) {
        return Err(scheduling_invalid(
            "FERRIS-SCHEDULE-REQUIRED-NODE-MISSING",
            "The scheduling replay request must contain at least one required node.",
        ));
    }

    let mut dependency_count = 0usize;
    for node in nodes_by_id.values_mut() {
        for dependency in &node.dependencies {
            validate_identity(dependency, "dependency")?;
        }
        node.dependencies.sort();
        if node.dependencies.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(scheduling_invalid(
                "FERRIS-SCHEDULE-DEPENDENCY-DUPLICATE",
                "A scheduling node contains a duplicate dependency.",
            ));
        }
        dependency_count = dependency_count
            .checked_add(node.dependencies.len())
            .ok_or_else(|| {
                scheduling_invalid(
                    "FERRIS-SCHEDULE-DEPENDENCY-BOUND-INVALID",
                    "The scheduling dependency count exceeds the supported bound.",
                )
            })?;
        if dependency_count > MAX_SCHEDULE_DEPENDENCIES {
            return Err(scheduling_invalid(
                "FERRIS-SCHEDULE-DEPENDENCY-BOUND-INVALID",
                "The scheduling dependency count exceeds the supported bound.",
            ));
        }
    }
    for node in nodes_by_id.values() {
        for dependency_id in &node.dependencies {
            nodes_by_id.get(dependency_id).ok_or_else(|| {
                scheduling_invalid(
                    "FERRIS-SCHEDULE-DEPENDENCY-UNKNOWN",
                    "A scheduling node references an unknown dependency.",
                )
            })?;
        }
    }

    let ordered = topological_order(nodes_by_id)?;
    let ordered_by_id = ordered
        .iter()
        .map(|node| (node.node_id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    for node in &ordered {
        for dependency_id in &node.dependencies {
            let dependency = ordered_by_id
                .get(dependency_id.as_str())
                .expect("validated dependency identity must remain present");
            if dependency.observed_outcome == ObservedTerminalOutcome::Succeeded
                && node.observed_start_ms < dependency.observed_finish_ms
            {
                return Err(scheduling_invalid(
                    "FERRIS-SCHEDULE-DEPENDENCY-TIMING-INVALID",
                    "A scheduling node starts before a successful dependency finishes.",
                ));
            }
        }
    }

    Ok((identity, ordered))
}

fn topological_order(
    nodes_by_id: BTreeMap<String, SchedulingNode>,
) -> Result<Vec<SchedulingNode>, CoreError> {
    let mut indegrees = nodes_by_id
        .iter()
        .map(|(node_id, node)| (node_id.clone(), node.dependencies.len()))
        .collect::<BTreeMap<_, _>>();
    let mut dependents = BTreeMap::<String, Vec<String>>::new();
    for node in nodes_by_id.values() {
        for dependency in &node.dependencies {
            dependents
                .entry(dependency.clone())
                .or_default()
                .push(node.node_id.clone());
        }
    }
    for values in dependents.values_mut() {
        values.sort();
    }
    let mut ready = indegrees
        .iter()
        .filter(|(_, indegree)| **indegree == 0)
        .map(|(node_id, _)| node_id.clone())
        .collect::<BTreeSet<_>>();
    let mut ordered_ids = Vec::with_capacity(nodes_by_id.len());
    while let Some(node_id) = ready.iter().next().cloned() {
        ready.remove(&node_id);
        ordered_ids.push(node_id.clone());
        if let Some(children) = dependents.get(&node_id) {
            for child in children {
                let indegree = indegrees
                    .get_mut(child)
                    .expect("validated dependency child must have an indegree");
                *indegree -= 1;
                if *indegree == 0 {
                    ready.insert(child.clone());
                }
            }
        }
    }
    if ordered_ids.len() != nodes_by_id.len() {
        return Err(scheduling_invalid(
            "FERRIS-SCHEDULE-GRAPH-CYCLE",
            "The scheduling replay graph contains a dependency cycle.",
        ));
    }
    Ok(ordered_ids
        .into_iter()
        .map(|node_id| {
            nodes_by_id
                .get(&node_id)
                .expect("topological node identity must remain present")
                .clone()
        })
        .collect())
}

fn schedule_graph_identity(nodes: &[SchedulingNode]) -> String {
    digest_bytes(
        &serde_json::to_vec(&(SCHEDULE_REPLAY_REQUEST_SCHEMA, nodes))
            .expect("typed scheduling graphs must serialize"),
    )
}

fn project_profile(
    profile: SchedulingProfile,
    identity: &RequestIdentity,
    graph_id: &str,
    required_node_ids: &[String],
    nodes: &[SchedulingNode],
    decisive_failure: Option<&SchedulingNode>,
) -> Result<SchedulingProfileResult, CoreError> {
    let nodes_by_id = nodes
        .iter()
        .map(|node| (node.node_id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let decisions = nodes
        .iter()
        .map(|node| project_node(profile, node, &nodes_by_id, decisive_failure))
        .collect::<Vec<_>>();
    let total_projected_saved_ms = decisions.iter().try_fold(0u64, |total, decision| {
        total
            .checked_add(decision.projected_saved_ms)
            .ok_or_else(|| {
                scheduling_error(
                    ResultClass::Internal,
                    "FERRIS-SCHEDULE-SAVINGS-OVERFLOW",
                    "Ferris could not total the bounded scheduling projection.",
                )
            })
    })?;
    let projected_success = nodes.iter().zip(&decisions).all(|(node, decision)| {
        !node.required
            || (node.observed_outcome == ObservedTerminalOutcome::Succeeded
                && decision.decision == SchedulingDecision::Continued)
    });
    Ok(SchedulingProfileResult {
        profile,
        repository_id: identity.repository_id.clone(),
        pull_request_id: identity.pull_request_id.clone(),
        source_revision: identity.source_revision.clone(),
        topology_id: identity.topology_id.clone(),
        graph_id: graph_id.to_owned(),
        required_node_ids: required_node_ids.to_vec(),
        success_predicate_id: SCHEDULE_SUCCESS_PREDICATE_ID.to_owned(),
        projected_success,
        node_decisions: decisions,
        total_projected_saved_ms,
    })
}

fn project_node(
    profile: SchedulingProfile,
    node: &SchedulingNode,
    nodes_by_id: &BTreeMap<&str, &SchedulingNode>,
    decisive_failure: Option<&SchedulingNode>,
) -> SchedulingNodeDecision {
    if let Some(trigger) = unsatisfied_ancestor_trigger(node, nodes_by_id) {
        return SchedulingNodeDecision {
            node_id: node.node_id.clone(),
            required: node.required,
            observed_outcome: node.observed_outcome,
            decision: SchedulingDecision::BlockedByDependency,
            reason: Some(SchedulingDecisionReason::UnsatisfiedPrerequisite),
            trigger_node_id: Some(trigger.node_id.clone()),
            projected_saved_ms: node.observed_finish_ms - node.observed_start_ms,
        };
    }

    let cancellation_reason = match profile {
        SchedulingProfile::FailFast if node.cancellable == Some(true) => {
            Some(SchedulingDecisionReason::OwnerAuthorizedFailFast)
        }
        SchedulingProfile::Balanced
            if node.cancellable == Some(true)
                && node.diagnostic_value == Some(DiagnosticValue::Low) =>
        {
            Some(SchedulingDecisionReason::OwnerAuthorizedLowDiagnosticValue)
        }
        SchedulingProfile::ConservativeOwnerOrder
        | SchedulingProfile::FlushOut
        | SchedulingProfile::FailFast
        | SchedulingProfile::Balanced => None,
    };
    if let (Some(trigger), Some(reason)) = (decisive_failure, cancellation_reason)
        && node.node_id != trigger.node_id
        && node.observed_finish_ms > trigger.observed_finish_ms
    {
        let (decision, projected_saved_ms) = if node.observed_start_ms >= trigger.observed_finish_ms
        {
            (
                SchedulingDecision::NotStartedAfterFailure,
                node.observed_finish_ms - node.observed_start_ms,
            )
        } else {
            (
                SchedulingDecision::CancelledAfterFailure,
                node.observed_finish_ms - trigger.observed_finish_ms,
            )
        };
        return SchedulingNodeDecision {
            node_id: node.node_id.clone(),
            required: node.required,
            observed_outcome: node.observed_outcome,
            decision,
            reason: Some(reason),
            trigger_node_id: Some(trigger.node_id.clone()),
            projected_saved_ms,
        };
    }

    SchedulingNodeDecision {
        node_id: node.node_id.clone(),
        required: node.required,
        observed_outcome: node.observed_outcome,
        decision: SchedulingDecision::Continued,
        reason: None,
        trigger_node_id: None,
        projected_saved_ms: 0,
    }
}

fn unsatisfied_ancestor_trigger<'a>(
    node: &SchedulingNode,
    nodes_by_id: &'a BTreeMap<&str, &SchedulingNode>,
) -> Option<&'a SchedulingNode> {
    let mut pending = node
        .dependencies
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let mut visited = BTreeSet::new();
    let mut candidates = Vec::new();
    while let Some(node_id) = pending.pop() {
        if !visited.insert(node_id) {
            continue;
        }
        let ancestor = nodes_by_id
            .get(node_id)
            .expect("validated dependency identity must remain present");
        if ancestor.observed_outcome != ObservedTerminalOutcome::Succeeded
            && node.observed_start_ms >= ancestor.observed_finish_ms
        {
            candidates.push(*ancestor);
        }
        pending.extend(ancestor.dependencies.iter().map(String::as_str));
    }
    candidates.into_iter().min_by(|left, right| {
        (left.observed_finish_ms, &left.node_id).cmp(&(right.observed_finish_ms, &right.node_id))
    })
}

fn validate_identity(value: &str, label: &str) -> Result<(), CoreError> {
    if valid_portable_id(value) {
        return Ok(());
    }
    Err(scheduling_invalid(
        "FERRIS-SCHEDULE-IDENTITY-INVALID",
        format!("The {label} identity is not a bounded stable portable identifier."),
    ))
}

fn validate_source_revision(value: &str) -> Result<(), CoreError> {
    if (40..=64).contains(&value.len())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Ok(());
    }
    Err(scheduling_invalid(
        "FERRIS-SCHEDULE-SOURCE-REVISION-INVALID",
        "The source revision must be a lowercase Git object identity.",
    ))
}

fn scheduling_invalid(code: &str, message: impl Into<String>) -> CoreError {
    scheduling_error(ResultClass::Invalid, code, message)
}

fn scheduling_error(class: ResultClass, code: &str, message: impl Into<String>) -> CoreError {
    CoreError::new(
        class,
        code,
        message,
        vec!["Repair the scheduling replay request and retry.".to_owned()],
    )
}
