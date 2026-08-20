use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

pub const COMMAND_RESULT_SCHEMA: &str = "ferris.command-result/v2";
pub const PLAN_SCHEMA: &str = "ferris.blueprint-plan/v0";
pub const EXPLANATION_SCHEMA: &str = "ferris.explanation/v0";
pub const GRAPH_SCHEMA: &str = "ferris.workspace-graph/v0";
pub const DOCTOR_SCHEMA: &str = "ferris.doctor-report/v0";
pub const PROFILE_EVIDENCE_SCHEMA: &str = "ferris.profile-evidence/v0";
pub const PROFILE_DIFF_SCHEMA: &str = "ferris.profile-diff/v0";
pub const VALIDATION_PLAN_SCHEMA: &str = "ferris.validation-plan/v0";
pub const FEDERATED_PLAN_REQUEST_SCHEMA: &str = "ferris.federated-plan-request/v0";
pub const FEDERATED_PLAN_SCHEMA: &str = "ferris.federated-plan/v0";
pub const APPLICATION_SCHEMA: &str = "ferris.application/v0";
pub const FEDERATED_VALIDATION_PLAN_SCHEMA: &str = "ferris.federated-validation-plan/v0";
pub const REVISION_SKEW_REQUEST_SCHEMA: &str = "ferris.revision-skew-request/v0";
pub const REVISION_SKEW_REPORT_SCHEMA: &str = "ferris.revision-skew-report/v0";

const MAX_GRAPH_NODES: usize = 10_000;
const MAX_GRAPH_EDGES: usize = 50_000;
const MAX_DOCTOR_MANIFEST_BYTES: u64 = 1024 * 1024;
const MAX_DOCTOR_OUTPUT_BYTES: usize = 64 * 1024;
const MAX_PROFILE_INPUT_BYTES: u64 = 1024 * 1024;
const MAX_PROFILE_CHANGES: usize = 10_000;
const MAX_PROFILE_IDENTITY_BYTES: usize = 256;
const MAX_PROFILE_OBJECT_KEY_BYTES: usize = 256;
const MAX_VALIDATION_INPUTS: usize = 256;
const MAX_FEDERATED_PLAN_REQUEST_BYTES: u64 = 1024 * 1024;
const MIN_FEDERATED_PLAN_WORKSPACES: usize = 2;
const MAX_FEDERATED_PLAN_WORKSPACES: usize = 16;
const MAX_APPLICATION_INPUT_BYTES: u64 = MAX_FEDERATED_PLAN_REQUEST_BYTES;
const MAX_REVISION_SKEW_REQUEST_BYTES: u64 = MAX_FEDERATED_PLAN_REQUEST_BYTES;
const MAX_REVISION_SKEW_PRODUCERS: usize = 16;
const MAX_REVISION_SKEW_CONSUMERS: usize = 16;
const MAX_REVISION_SKEW_DEPENDENCIES: usize = 64;
const MAX_REVISION_SKEW_LOCK_BYTES: u64 = 16 * 1024 * 1024;
const MAX_FEDERATED_PLAN_METADATA_BYTES: usize = 256;
const FEDERATED_PLAN_METADATA_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const REVISION_SKEW_GIT_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_REVISION_SKEW_GIT_OUTPUT_BYTES: usize = 64 * 1024;
const DOCTOR_TIMEOUT: Duration = Duration::from_secs(5);
const WORKSPACE_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_WORKSPACE_DISCOVERY_OUTPUT_BYTES: usize = 64 * 1024;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounded_output: Option<BoundedOutputEvidence>,
    pub next_actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundedOutputEvidence {
    pub schema: String,
    pub owner_output_framing: String,
    pub stdout_retained_bytes: u64,
    pub stdout_observed_bytes: u64,
    pub stdout_omitted_observed_bytes: u64,
    pub stdout_unobserved_bytes_unknown: bool,
    pub stdout_complete: bool,
    pub stdout_truncated: bool,
    pub stdout_read_failed: bool,
    pub stderr_retained_bytes: u64,
    pub stderr_observed_bytes: u64,
    pub stderr_omitted_observed_bytes: u64,
    pub stderr_unobserved_bytes_unknown: bool,
    pub stderr_complete: bool,
    pub stderr_truncated: bool,
    pub stderr_read_failed: bool,
    pub output_digest: String,
    pub termination: String,
    pub termination_scope: String,
    pub termination_cleanup_complete: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandEnvelope<T> {
    pub schema: String,
    pub command_version: String,
    pub semantic_command_id: String,
    pub selection_identity: String,
    pub invocation_identity: String,
    pub result_identity: String,
    pub result_class: ResultClass,
    pub process_exit_code: u8,
    pub diagnostics: Vec<Diagnostic>,
    pub record: Option<T>,
}

#[derive(Serialize)]
struct CommandResultIdentityInput<'a, T> {
    schema: &'a str,
    command_version: &'a str,
    semantic_command_id: &'a str,
    selection_identity: &'a str,
    invocation_identity: &'a str,
    result_class: ResultClass,
    process_exit_code: u8,
    diagnostics: &'a [Diagnostic],
    record: &'a Option<T>,
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
pub struct FederatedWorkspacePlan {
    pub workspace_id: String,
    pub plan: PlanRecord,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FederatedPlanRecord {
    pub schema: String,
    pub federated_plan_id: String,
    pub application_id: String,
    pub revision: String,
    pub owner: String,
    pub executable: bool,
    pub workspaces: Vec<FederatedWorkspacePlan>,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FederatedPlanRequest {
    schema: String,
    application_id: String,
    revision: String,
    owner: String,
    workspaces: Vec<FederatedWorkspaceRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FederatedWorkspaceRequest {
    workspace_id: String,
    manifest_path: String,
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationInputKind {
    Path,
    Package,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationInputDisposition {
    ExplicitPackage,
    OwnedRustPath,
    FullWorkspaceFallback,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationInputRecord {
    pub kind: ValidationInputKind,
    pub value: String,
    pub disposition: ValidationInputDisposition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_identity: Option<String>,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationPackageDisposition {
    Anchor,
    ReverseDependency,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationPackageSelection {
    pub package: PackageRecord,
    pub disposition: ValidationPackageDisposition,
    pub reasons: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationActivityFamily {
    Check,
    Test,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationActivityScope {
    SelectedPackageClosure,
    FullWorkspaceFallback,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationActivityPlan {
    pub family: ValidationActivityFamily,
    pub owner: String,
    pub package_scope: ValidationActivityScope,
    pub package_identities: Vec<String>,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationFallbackPlan {
    pub boundary: String,
    pub required_by_inputs: bool,
    pub reasons: Vec<String>,
    pub packages: Vec<PackageRecord>,
    pub activities: Vec<ValidationActivityPlan>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationPlanRecord {
    pub schema: String,
    pub validation_plan_id: String,
    pub workspace_id: String,
    pub executable: bool,
    pub selected_manifest: String,
    pub workspace_root: String,
    pub inputs: Vec<ValidationInputRecord>,
    pub selected_packages: Vec<ValidationPackageSelection>,
    pub selected_activities: Vec<ValidationActivityPlan>,
    pub fallback: ValidationFallbackPlan,
    pub evidence: EvidenceSource,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationDefinition {
    pub schema: String,
    pub application_id: String,
    pub workspaces: Vec<ApplicationWorkspaceDefinition>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationWorkspaceDefinition {
    pub workspace_id: String,
    pub manifest_path: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FederatedValidationWorkspaceDisposition {
    DirectPlan,
    RelationshipFallback,
    ApplicationFallback,
    NotSelected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FederatedValidationWorkspacePlan {
    pub workspace_id: String,
    pub manifest_path: String,
    pub disposition: FederatedValidationWorkspaceDisposition,
    pub reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_plan: Option<ValidationPlanRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FederatedValidationFallback {
    pub boundary: String,
    pub required_by_inputs: bool,
    pub workspace_ids: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FederatedValidationPlanRecord {
    pub schema: String,
    pub federated_validation_plan_id: String,
    pub application_id: String,
    pub application_definition: String,
    pub executable: bool,
    pub workspaces: Vec<FederatedValidationWorkspacePlan>,
    pub fallback: FederatedValidationFallback,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RevisionDeclarationKind {
    Branch,
    Revision,
    Tag,
    DefaultBranch,
    Ambiguous,
    Missing,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RevisionSkewStatus {
    Equal,
    Behind,
    Ahead,
    Divergent,
    Unavailable,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RevisionDeclarationEvidence {
    pub kind: RevisionDeclarationKind,
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RevisionSkewDependencyRecord {
    pub consumer_id: String,
    pub producer_id: String,
    pub package_name: String,
    pub declaration: RevisionDeclarationEvidence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_revision: Option<String>,
    pub observed_revision: String,
    pub status: RevisionSkewStatus,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RevisionSkewReportRecord {
    pub schema: String,
    pub report_id: String,
    pub analysis_id: String,
    pub executable: bool,
    pub dependencies: Vec<RevisionSkewDependencyRecord>,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RevisionSkewRequest {
    schema: String,
    analysis_id: String,
    producers: Vec<RevisionSkewProducerRequest>,
    consumers: Vec<RevisionSkewConsumerRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RevisionSkewProducerRequest {
    producer_id: String,
    repository_url: String,
    checkout_path: String,
    observed_revision: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RevisionSkewConsumerRequest {
    consumer_id: String,
    manifest_path: String,
    dependencies: Vec<RevisionSkewDependencyRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RevisionSkewDependencyRequest {
    producer_id: String,
    package_name: String,
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
    pub stdout_retained_bytes: u64,
    pub stdout_observed_bytes: u64,
    pub stdout_omitted_observed_bytes: u64,
    pub stdout_unobserved_bytes_unknown: bool,
    pub stdout_complete: bool,
    pub stdout_truncated: bool,
    pub stderr_retained_bytes: u64,
    pub stderr_observed_bytes: u64,
    pub stderr_omitted_observed_bytes: u64,
    pub stderr_unobserved_bytes_unknown: bool,
    pub stderr_complete: bool,
    pub stderr_truncated: bool,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProfileReference {
    pub profile_id: String,
    pub revision: String,
    pub consumer: String,
    pub content_digest: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileChangeKind {
    Added,
    Removed,
    Changed,
}

impl fmt::Display for ProfileChangeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = serde_json::to_value(self).map_err(|_| fmt::Error)?;
        formatter.write_str(value.as_str().ok_or(fmt::Error)?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProfileChange {
    pub path: String,
    pub change_kind: ProfileChangeKind,
    pub before_value_digest: Option<String>,
    pub after_value_digest: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProfileDiffRecord {
    pub schema: String,
    pub diff_id: String,
    pub before: ProfileReference,
    pub after: ProfileReference,
    pub changed_sections: Vec<String>,
    pub changes: Vec<ProfileChange>,
    pub unchanged_sections: Vec<String>,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
    pub executable: bool,
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
                bounded_output: None,
                next_actions,
            }),
            invocation_selection: None,
        }
    }

    fn with_source_digest(mut self, source_digest: String) -> Self {
        self.diagnostic.source_digest = Some(source_digest);
        self
    }

    fn with_bounded_output(mut self, bounded_output: BoundedOutputEvidence) -> Self {
        self.diagnostic.source_digest = Some(bounded_output.output_digest.clone());
        self.diagnostic.bounded_output = Some(bounded_output);
        self
    }

    fn with_invocation_selection(mut self, invocation_selection: String) -> Self {
        self.invocation_selection = Some(invocation_selection);
        self
    }

    fn with_federated_workspace(mut self, workspace_id: &str) -> Self {
        self.diagnostic.message = format!(
            "Federated workspace '{workspace_id}': {}",
            self.diagnostic.message
        );
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

const PROFILE_SECTION_NAMES: [&str; 12] = [
    "identity",
    "closure",
    "features",
    "toolchain",
    "targets",
    "providers",
    "native",
    "stages",
    "assurance",
    "stewardship",
    "support",
    "lifecycle",
];

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ProfileEvidence {
    schema: String,
    profile_id: String,
    revision: String,
    consumer: String,
    sections: ProfileSections,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ProfileSections {
    identity: serde_json::Value,
    closure: serde_json::Value,
    features: serde_json::Value,
    toolchain: serde_json::Value,
    targets: serde_json::Value,
    providers: serde_json::Value,
    native: serde_json::Value,
    stages: serde_json::Value,
    assurance: serde_json::Value,
    stewardship: serde_json::Value,
    support: serde_json::Value,
    lifecycle: serde_json::Value,
}

impl ProfileSections {
    fn get(&self, name: &str) -> &serde_json::Value {
        match name {
            "identity" => &self.identity,
            "closure" => &self.closure,
            "features" => &self.features,
            "toolchain" => &self.toolchain,
            "targets" => &self.targets,
            "providers" => &self.providers,
            "native" => &self.native,
            "stages" => &self.stages,
            "assurance" => &self.assurance,
            "stewardship" => &self.stewardship,
            "support" => &self.support,
            "lifecycle" => &self.lifecycle,
            _ => unreachable!("profile section names are fixed"),
        }
    }
}

struct LoadedProfile {
    evidence: ProfileEvidence,
    content_digest: String,
}

struct StrictJsonValue(serde_json::Value);

impl StrictJsonValue {
    fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonValueVisitor)
    }
}

struct StrictJsonValueVisitor;

impl<'de> Visitor<'de> for StrictJsonValueVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value with unique, safe object member names")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::String(value.to_owned())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictJsonValue>()? {
            values.push(value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if !valid_profile_object_key(&key) {
                return Err(de::Error::custom(
                    "invalid output-visible JSON object member",
                ));
            }
            if values.contains_key(&key) {
                return Err(de::Error::custom("duplicate JSON object member"));
            }
            let value = map.next_value::<StrictJsonValue>()?;
            values.insert(key, value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Object(values)))
    }
}

fn valid_profile_identity(value: &str) -> bool {
    valid_output_visible_metadata(value, MAX_PROFILE_IDENTITY_BYTES)
}

fn valid_profile_object_key(value: &str) -> bool {
    valid_output_visible_metadata(value, MAX_PROFILE_OBJECT_KEY_BYTES)
}

fn valid_output_visible_metadata(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_bytes
        && value.bytes().all(|byte| matches!(byte, b'!'..=b'~'))
}

fn valid_federated_metadata(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FEDERATED_PLAN_METADATA_BYTES
        && !value.starts_with(' ')
        && !value.ends_with(' ')
        && value.bytes().all(|byte| matches!(byte, b' '..=b'~'))
}

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
    source: Option<String>,
}

struct MetadataInvocation {
    manifest_path: PathBuf,
    bytes: Vec<u8>,
}

struct WorkspacePackage {
    package: PackageRecord,
    package_root: String,
    package_root_absolute: PathBuf,
    dependencies: Vec<CargoDependency>,
}

struct ValidationPackageBuilder {
    package: PackageRecord,
    disposition: ValidationPackageDisposition,
    reasons: Vec<String>,
}

const VALIDATION_INPUT_CODE_EXPLICIT_PACKAGE: &str = "explicit_package";
const VALIDATION_INPUT_CODE_OWNED_RUST_PATH: &str = "owned_rust_path";
const VALIDATION_INPUT_CODE_PACKAGE_PATH_REQUIRES_FULL_WORKSPACE_FALLBACK: &str =
    "package_path_requires_full_workspace_fallback";
const VALIDATION_INPUT_CODE_WORKSPACE_PATH_OUTSIDE_PACKAGE_ANCHOR: &str =
    "workspace_path_outside_package_anchor";
const VALIDATION_INPUT_CODE_AMBIGUOUS_PACKAGE_ROOT_MATCH: &str = "ambiguous_package_root_match";
const VALIDATION_FALLBACK_CODE_SELECTED_PACKAGE_EVIDENCE_NOT_FULL_REFERENCE: &str =
    "selected_package_evidence_not_full_reference";
const VALIDATION_FALLBACK_CODE_OWNER_DEFINED_VALIDATION_OUTSIDE_PULSE: &str =
    "owner_defined_validation_outside_pulse";

#[derive(Clone, Debug)]
struct ValidationInputBuilder {
    record: ValidationInputRecord,
    semantic_code: &'static str,
}

#[derive(Debug, Serialize)]
struct ValidationPlanIdentityProjection {
    schema: String,
    workspace_id: String,
    selected_manifest: String,
    inputs: Vec<ValidationPlanIdentityInput>,
    selected_packages: Vec<ValidationPlanIdentityPackage>,
    selected_activities: Vec<ValidationPlanIdentityActivity>,
    fallback: ValidationPlanIdentityFallback,
}

#[derive(Debug, Serialize)]
struct ValidationPlanIdentityInput {
    kind: ValidationInputKind,
    value: String,
    disposition: ValidationInputDisposition,
    package_identity: Option<String>,
    semantic_code: &'static str,
}

#[derive(Debug, Serialize)]
struct ValidationPlanIdentityPackage {
    identity: String,
    disposition: ValidationPackageDisposition,
}

#[derive(Debug, Serialize)]
struct ValidationPlanIdentityActivity {
    family: ValidationActivityFamily,
    owner: String,
    package_scope: ValidationActivityScope,
    package_identities: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ValidationPlanIdentityFallback {
    boundary: String,
    required_by_inputs: bool,
    package_identities: Vec<String>,
    activities: Vec<ValidationPlanIdentityActivity>,
    stable_codes: Vec<&'static str>,
}

struct LoadedApplicationDefinition {
    definition: ApplicationDefinition,
    definition_identity: String,
    application_name: String,
    application_root: PathBuf,
    workspaces: Vec<LoadedApplicationWorkspace>,
}

struct LoadedApplicationWorkspace {
    definition: ApplicationWorkspaceDefinition,
    manifest_path: PathBuf,
    workspace_root: PathBuf,
    metadata: CargoMetadata,
    metadata_bytes: Vec<u8>,
}

#[derive(Serialize)]
struct ApplicationDefinitionIdentityWorkspace<'a> {
    workspace_id: &'a str,
    manifest_path: &'a str,
    depends_on: Vec<&'a str>,
}

#[derive(Serialize)]
struct ApplicationDefinitionIdentity<'a> {
    schema: &'a str,
    application_id: &'a str,
    workspaces: Vec<ApplicationDefinitionIdentityWorkspace<'a>>,
}

#[derive(Serialize)]
struct FederatedValidationIdentityWorkspace<'a> {
    workspace_id: &'a str,
    manifest_path: &'a str,
    disposition: FederatedValidationWorkspaceDisposition,
    reasons: &'a [String],
    validation_plan_id: Option<&'a str>,
}

#[derive(Serialize)]
struct FederatedValidationIdentityProjection<'a> {
    schema: &'a str,
    application_id: &'a str,
    application_definition_identity: &'a str,
    workspaces: Vec<FederatedValidationIdentityWorkspace<'a>>,
    fallback: &'a FederatedValidationFallback,
}

#[derive(Debug)]
struct BoundedOutput {
    status: ExitStatus,
    capture: BoundedCapture,
}

#[derive(Clone, Debug)]
struct CapturedStream {
    retained: Vec<u8>,
    observed_bytes: u64,
    complete: bool,
    truncated: bool,
    failed: bool,
}

#[derive(Clone, Debug)]
struct BoundedCapture {
    stdout: CapturedStream,
    stderr: CapturedStream,
    termination_cleanup_complete: bool,
}

#[derive(Debug)]
enum BoundedCommandError {
    Start(io::Error),
    Wait(io::Error),
    Read,
    ReadCapture(BoundedCapture),
    Timeout(BoundedCapture),
    OutputLimit(BoundedCapture),
}

#[derive(Debug)]
struct CargoVersionEvidence {
    version: String,
    commit: Option<String>,
    release_date: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CargoLocateProject {
    root: PathBuf,
}

pub fn create_profile_diff(
    before_path: &Path,
    after_path: &Path,
) -> Result<CommandEnvelope<ProfileDiffRecord>, CoreError> {
    let request_material = profile_diff_request_material(before_path, after_path);
    let before = load_profile(before_path)
        .map_err(|error| error.with_invocation_selection(request_material.clone()))?;
    let after = load_profile(after_path).map_err(|error| {
        error.with_invocation_selection(format!(
            "before={};after-request={}",
            before.content_digest,
            profile_request_digest(after_path)
        ))
    })?;
    let selection_identity =
        profile_diff_selection_identity(&before.content_digest, &after.content_digest);
    let invocation_identity = profile_diff_invocation_identity(&selection_identity);
    let content_selection = selection_identity.clone();

    if before.evidence.profile_id != after.evidence.profile_id {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-PROFILE-DIFF-PROFILE-ID-MISMATCH",
            "The two profile evidence files declare different profile identities.",
            vec!["Compare revisions of the same explicit profile identity.".to_owned()],
        )
        .with_invocation_selection(content_selection.clone()));
    }
    if before.evidence.consumer != after.evidence.consumer {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-PROFILE-DIFF-CONSUMER-MISMATCH",
            "The two profile evidence files declare different consumers.",
            vec!["Compare evidence for the same explicit consumer.".to_owned()],
        )
        .with_invocation_selection(content_selection.clone()));
    }

    let mut changes = Vec::new();
    if before.evidence.revision != after.evidence.revision {
        push_profile_change(
            &mut changes,
            "/revision".to_owned(),
            ProfileChangeKind::Changed,
            Some(&serde_json::Value::String(before.evidence.revision.clone())),
            Some(&serde_json::Value::String(after.evidence.revision.clone())),
        )
        .map_err(|error| error.with_invocation_selection(content_selection.clone()))?;
    }

    let mut changed_sections = Vec::new();
    let mut unchanged_sections = Vec::new();
    for section in PROFILE_SECTION_NAMES {
        let before_value = before.evidence.sections.get(section);
        let after_value = after.evidence.sections.get(section);
        if before_value == after_value {
            unchanged_sections.push(section.to_owned());
        } else {
            changed_sections.push(section.to_owned());
            diff_profile_value(
                before_value,
                after_value,
                &format!("/sections/{}", escape_json_pointer_token(section)),
                &mut changes,
            )
            .map_err(|error| error.with_invocation_selection(content_selection.clone()))?;
        }
    }
    changed_sections.sort();
    unchanged_sections.sort();
    changes.sort_by(|left, right| left.path.cmp(&right.path));

    let before_reference = profile_reference(&before);
    let after_reference = profile_reference(&after);
    let mut record = ProfileDiffRecord {
        schema: PROFILE_DIFF_SCHEMA.to_owned(),
        diff_id: String::new(),
        before: before_reference,
        after: after_reference,
        changed_sections,
        changes,
        unchanged_sections,
        unknowns: vec![
            "Semantic equivalence and compatibility are not assessed.".to_owned(),
            "Support, freshness, approval, and decision authority are not assessed.".to_owned(),
        ],
        limitations: vec![
            "This record compares only explicit caller-provided evidence and does not interpret support, compatibility, approval, correctness, freshness, or readiness."
                .to_owned(),
            "Ferris did not generate either profile, invoke an owner tool, discover files, contact a network, select packages, or mutate input, repository, or environment state."
                .to_owned(),
            "Value digests identify compared JSON values; raw section values are intentionally omitted."
                .to_owned(),
            "Profile identifiers, revisions, consumers, and JSON object keys are output-visible metadata; callers must not place secrets in those fields."
                .to_owned(),
        ],
        executable: false,
    };
    record.diff_id = profile_diff_record_id(&record)
        .map_err(|error| error.with_invocation_selection(content_selection))?;
    let result_class = if record.changes.is_empty() {
        ResultClass::Success
    } else {
        ResultClass::Difference
    };
    Ok(command_envelope(
        "profile-diff",
        selection_identity,
        invocation_identity,
        result_class,
        Vec::new(),
        Some(record),
    ))
}

fn load_profile(path: &Path) -> Result<LoadedProfile, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-PROFILE-INPUT-UNAVAILABLE",
            "An explicit profile evidence input is missing or unreadable.",
            vec!["Pass two readable local files with --before and --after.".to_owned()],
        )
    })?;
    if !metadata.is_file() {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-PROFILE-INPUT-NOT-FILE",
            "An explicit profile evidence input is not a regular file.",
            vec!["Pass two readable local files with --before and --after.".to_owned()],
        ));
    }
    if metadata.len() > MAX_PROFILE_INPUT_BYTES {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-PROFILE-INPUT-OVERSIZED",
            format!(
                "An explicit profile evidence input exceeds the {MAX_PROFILE_INPUT_BYTES}-byte bound."
            ),
            vec!["Reduce the explicit input below the documented bound.".to_owned()],
        ));
    }

    let file = fs::File::open(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-PROFILE-INPUT-UNAVAILABLE",
            "An explicit profile evidence input is missing or unreadable.",
            vec!["Pass two readable local files with --before and --after.".to_owned()],
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_PROFILE_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-PROFILE-INPUT-UNAVAILABLE",
                "An explicit profile evidence input could not be read completely.",
                vec!["Pass two readable local files with --before and --after.".to_owned()],
            )
        })?;
    if bytes.len() as u64 > MAX_PROFILE_INPUT_BYTES {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-PROFILE-INPUT-OVERSIZED",
            format!(
                "An explicit profile evidence input exceeds the {MAX_PROFILE_INPUT_BYTES}-byte bound."
            ),
            vec!["Reduce the explicit input below the documented bound.".to_owned()],
        ));
    }

    let value = serde_json::from_slice::<StrictJsonValue>(&bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|error| {
            let (code, message, next_action) =
                if error.to_string().contains("duplicate JSON object member") {
                    (
                        "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER",
                        "An explicit profile evidence input contains a duplicate JSON object member.",
                        "Remove duplicate object members so every JSON Pointer has one unambiguous value.",
                    )
                } else if error
                    .to_string()
                    .contains("invalid output-visible JSON object member")
                {
                    (
                        "FERRIS-PROFILE-METADATA-INVALID",
                        "An explicit profile evidence input contains an invalid output-visible JSON object member.",
                        "Use 1 to 256 visible ASCII characters for every JSON object member name.",
                    )
                } else {
                    (
                        "FERRIS-PROFILE-JSON-INVALID",
                        "An explicit profile evidence input is not valid JSON.",
                        "Provide a complete JSON profile evidence fixture.",
                    )
                };
            CoreError::new(
                ResultClass::Invalid,
                code,
                message,
                vec![next_action.to_owned()],
            )
            .with_source_digest(digest_bytes(&bytes))
        })?;
    let schema = value.get("schema").and_then(serde_json::Value::as_str);
    if let Some(schema) = schema
        && schema != PROFILE_EVIDENCE_SCHEMA
    {
        return Err(CoreError::new(
            ResultClass::Unsupported,
            "FERRIS-PROFILE-SCHEMA-UNSUPPORTED",
            "An explicit profile evidence input uses an unsupported schema.",
            vec![format!("Use schema {PROFILE_EVIDENCE_SCHEMA}.")],
        )
        .with_source_digest(canonical_value_digest(&value)?));
    }
    let evidence: ProfileEvidence = serde_json::from_value(value).map_err(|_| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-PROFILE-SHAPE-INVALID",
            "An explicit profile evidence input does not match the experimental fixture contract.",
            vec![
                "Provide only schema, profile_id, revision, consumer, and all twelve required sections."
                    .to_owned(),
            ],
        )
        .with_source_digest(digest_bytes(&bytes))
    })?;
    if evidence.schema != PROFILE_EVIDENCE_SCHEMA {
        return Err(CoreError::new(
            ResultClass::Unsupported,
            "FERRIS-PROFILE-SCHEMA-UNSUPPORTED",
            "An explicit profile evidence input uses an unsupported schema.",
            vec![format!("Use schema {PROFILE_EVIDENCE_SCHEMA}.")],
        ));
    }
    if [
        evidence.profile_id.as_str(),
        evidence.revision.as_str(),
        evidence.consumer.as_str(),
    ]
    .iter()
    .any(|value| !valid_profile_identity(value))
    {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-PROFILE-IDENTITY-INVALID",
            "Profile identity, revision, and consumer must use bounded single-line output-visible metadata.",
            vec![
                "Use 1 to 256 visible ASCII characters for profile_id, revision, and consumer, and do not place secrets in them."
                    .to_owned(),
            ],
        ));
    }
    let content_digest = canonical_value_digest(&evidence)?;
    Ok(LoadedProfile {
        evidence,
        content_digest,
    })
}

fn profile_reference(profile: &LoadedProfile) -> ProfileReference {
    ProfileReference {
        profile_id: profile.evidence.profile_id.clone(),
        revision: profile.evidence.revision.clone(),
        consumer: profile.evidence.consumer.clone(),
        content_digest: profile.content_digest.clone(),
    }
}

fn diff_profile_value(
    before: &serde_json::Value,
    after: &serde_json::Value,
    path: &str,
    changes: &mut Vec<ProfileChange>,
) -> Result<(), CoreError> {
    if before == after {
        return Ok(());
    }
    match (before, after) {
        (serde_json::Value::Object(before), serde_json::Value::Object(after)) => {
            let keys = before
                .keys()
                .chain(after.keys())
                .map(String::as_str)
                .collect::<BTreeSet<_>>();
            if keys.is_empty() {
                return push_profile_change(
                    changes,
                    path.to_owned(),
                    ProfileChangeKind::Changed,
                    Some(&serde_json::Value::Object(before.clone())),
                    Some(&serde_json::Value::Object(after.clone())),
                );
            }
            for key in keys {
                let child_path = format!("{path}/{}", escape_json_pointer_token(key));
                match (before.get(key), after.get(key)) {
                    (Some(before), Some(after)) => {
                        diff_profile_value(before, after, &child_path, changes)?
                    }
                    (Some(before), None) => add_profile_subtree(
                        before,
                        &child_path,
                        ProfileChangeKind::Removed,
                        changes,
                    )?,
                    (None, Some(after)) => {
                        add_profile_subtree(after, &child_path, ProfileChangeKind::Added, changes)?
                    }
                    (None, None) => unreachable!("combined object key exists"),
                }
            }
            Ok(())
        }
        (serde_json::Value::Array(before), serde_json::Value::Array(after)) => {
            let maximum = before.len().max(after.len());
            if maximum == 0 {
                return push_profile_change(
                    changes,
                    path.to_owned(),
                    ProfileChangeKind::Changed,
                    Some(&serde_json::Value::Array(before.clone())),
                    Some(&serde_json::Value::Array(after.clone())),
                );
            }
            for index in 0..maximum {
                let child_path = format!("{path}/{index}");
                match (before.get(index), after.get(index)) {
                    (Some(before), Some(after)) => {
                        diff_profile_value(before, after, &child_path, changes)?
                    }
                    (Some(before), None) => add_profile_subtree(
                        before,
                        &child_path,
                        ProfileChangeKind::Removed,
                        changes,
                    )?,
                    (None, Some(after)) => {
                        add_profile_subtree(after, &child_path, ProfileChangeKind::Added, changes)?
                    }
                    (None, None) => unreachable!("array index is within maximum"),
                }
            }
            Ok(())
        }
        _ => push_profile_change(
            changes,
            path.to_owned(),
            ProfileChangeKind::Changed,
            Some(before),
            Some(after),
        ),
    }
}

fn add_profile_subtree(
    value: &serde_json::Value,
    path: &str,
    kind: ProfileChangeKind,
    changes: &mut Vec<ProfileChange>,
) -> Result<(), CoreError> {
    match value {
        serde_json::Value::Object(object) if !object.is_empty() => {
            for (key, child) in object {
                add_profile_subtree(
                    child,
                    &format!("{path}/{}", escape_json_pointer_token(key)),
                    kind,
                    changes,
                )?;
            }
            Ok(())
        }
        serde_json::Value::Array(array) if !array.is_empty() => {
            for (index, child) in array.iter().enumerate() {
                add_profile_subtree(child, &format!("{path}/{index}"), kind, changes)?;
            }
            Ok(())
        }
        _ => match kind {
            ProfileChangeKind::Added => {
                push_profile_change(changes, path.to_owned(), kind, None, Some(value))
            }
            ProfileChangeKind::Removed => {
                push_profile_change(changes, path.to_owned(), kind, Some(value), None)
            }
            ProfileChangeKind::Changed => unreachable!("subtree changes are added or removed"),
        },
    }
}

fn push_profile_change(
    changes: &mut Vec<ProfileChange>,
    path: String,
    change_kind: ProfileChangeKind,
    before: Option<&serde_json::Value>,
    after: Option<&serde_json::Value>,
) -> Result<(), CoreError> {
    if changes.len() >= MAX_PROFILE_CHANGES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-PROFILE-DIFF-BOUND-EXCEEDED",
            format!("The profile diff exceeds the {MAX_PROFILE_CHANGES}-change bound."),
            vec![
                "Use a more narrowly scoped explicit evidence fixture or owner-native comparison tools."
                    .to_owned(),
            ],
        ));
    }
    changes.push(ProfileChange {
        path,
        change_kind,
        before_value_digest: before.map(canonical_value_digest).transpose()?,
        after_value_digest: after.map(canonical_value_digest).transpose()?,
    });
    Ok(())
}

fn escape_json_pointer_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn canonical_value_digest<T: Serialize>(value: &T) -> Result<String, CoreError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        CoreError::new(
            ResultClass::Internal,
            "FERRIS-PROFILE-CANONICALIZATION-FAILED",
            "Ferris could not canonicalize profile evidence safely.",
            vec!["Report this Ferris invariant failure.".to_owned()],
        )
    })?;
    Ok(digest_bytes(&bytes))
}

fn profile_diff_record_id(record: &ProfileDiffRecord) -> Result<String, CoreError> {
    debug_assert!(record.diff_id.is_empty());
    record_id("profile-diff", record)
}

fn profile_diff_selection_identity(before_digest: &str, after_digest: &str) -> String {
    invocation_identity(&["profile-diff-selection", before_digest, after_digest]).replacen(
        "invocation:",
        "selection:",
        1,
    )
}

fn profile_diff_invocation_identity(selection_identity: &str) -> String {
    invocation_identity(&[
        "profile-diff",
        selection_identity,
        "profile-schema=ferris.profile-evidence/v0",
        "input-max-bytes=1048576",
        "change-max=10000",
        "owner-tools=false",
        "network=false",
        "mutation=false",
    ])
}

fn profile_request_digest(path: &Path) -> String {
    digest_text(&lexically_normalize_path_text(&path.to_string_lossy()))
}

fn profile_diff_request_material(before_path: &Path, after_path: &Path) -> String {
    format!(
        "before-request={};after-request={}",
        profile_request_digest(before_path),
        profile_request_digest(after_path)
    )
}

pub fn create_plan(
    manifest_path: &Path,
    workspace_id: &str,
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    create_plan_with_cargo(manifest_path, workspace_id, Path::new("cargo"))
}

pub fn create_federated_plan(
    request_path: &Path,
) -> Result<CommandEnvelope<FederatedPlanRecord>, CoreError> {
    let bytes = read_federated_plan_request(request_path)?;
    let input_selection = selection_identity("federated-plan-request", &digest_bytes(&bytes));
    let request: FederatedPlanRequest = serde_json::from_slice(&bytes).map_err(|_| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-REQUEST-SHAPE-INVALID",
            "The federated plan request is not valid JSON with exactly the supported fields.",
            vec![
                "Provide only schema, application_id, revision, owner, and workspaces; each workspace may contain only workspace_id and manifest_path."
                    .to_owned(),
            ],
        )
        .with_source_digest(digest_bytes(&bytes))
        .with_invocation_selection(input_selection.clone())
    })?;
    let request_selection = federated_plan_request_selection_identity(&request);

    if request.schema != FEDERATED_PLAN_REQUEST_SCHEMA {
        return Err(CoreError::new(
            ResultClass::Unsupported,
            "FERRIS-FEDERATED-PLAN-REQUEST-SCHEMA-UNSUPPORTED",
            "The federated plan request uses an unsupported schema.",
            vec![format!("Use schema {FEDERATED_PLAN_REQUEST_SCHEMA}.")],
        )
        .with_invocation_selection(request_selection));
    }
    validate_application_id(&request.application_id)
        .map_err(|error| error.with_invocation_selection(request_selection.clone()))?;
    if !valid_federated_metadata(&request.revision) || !valid_federated_metadata(&request.owner) {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-METADATA-INVALID",
            "Federated plan revision and owner must use 1 to 256 ASCII bytes, may contain interior spaces, and must not contain leading or trailing spaces or control characters.",
            vec![
                "Use bounded non-control ASCII revision and owner values, and do not place secrets in them."
                    .to_owned(),
            ],
        )
        .with_invocation_selection(request_selection));
    }
    if !(MIN_FEDERATED_PLAN_WORKSPACES..=MAX_FEDERATED_PLAN_WORKSPACES)
        .contains(&request.workspaces.len())
    {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-WORKSPACE-BOUND-INVALID",
            format!(
                "A federated plan request must contain {MIN_FEDERATED_PLAN_WORKSPACES} to {MAX_FEDERATED_PLAN_WORKSPACES} workspaces."
            ),
            vec![
                "Provide an explicit bounded federation of independent Cargo workspaces."
                    .to_owned(),
            ],
        )
        .with_invocation_selection(request_selection));
    }

    let request_parent = canonical_federated_request_parent(request_path)
        .map_err(|error| error.with_invocation_selection(request_selection.clone()))?;
    let mut workspace_ids = BTreeSet::new();
    let mut manifests = BTreeSet::new();
    let mut resolved = Vec::with_capacity(request.workspaces.len());
    for workspace in request.workspaces {
        validate_workspace_id(&workspace.workspace_id)
            .map_err(|error| error.with_invocation_selection(request_selection.clone()))?;
        if !workspace_ids.insert(workspace.workspace_id.clone()) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-PLAN-WORKSPACE-ID-DUPLICATE",
                "The federated plan request contains a duplicate workspace identity.",
                vec!["Use one unique portable workspace_id per workspace.".to_owned()],
            )
            .with_federated_workspace(&workspace.workspace_id)
            .with_invocation_selection(request_selection));
        }

        validate_federated_manifest_request(&workspace.manifest_path).map_err(|error| {
            error
                .with_federated_workspace(&workspace.workspace_id)
                .with_invocation_selection(request_selection.clone())
        })?;
        let relative_manifest = Path::new(&workspace.manifest_path);
        let manifest =
            canonical_manifest_path(&request_parent.join(relative_manifest)).map_err(|error| {
                error
                    .with_federated_workspace(&workspace.workspace_id)
                    .with_invocation_selection(request_selection.clone())
            })?;
        if !manifest.starts_with(&request_parent) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-PLAN-MANIFEST-OUTSIDE-REQUEST",
                "The canonical manifest is outside the canonical request parent.",
                vec![
                    "Place the request at a common ancestor of every selected manifest.".to_owned(),
                ],
            )
            .with_federated_workspace(&workspace.workspace_id)
            .with_invocation_selection(request_selection));
        }
        if !manifests.insert(manifest.clone()) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-PLAN-MANIFEST-DUPLICATE",
                "The federated plan request selects the same resolved manifest more than once.",
                vec!["Select each independent Cargo workspace manifest once.".to_owned()],
            )
            .with_federated_workspace(&workspace.workspace_id)
            .with_invocation_selection(request_selection));
        }
        resolved.push((workspace.workspace_id, manifest));
    }
    resolved.sort_by(|left, right| left.0.cmp(&right.0));

    let mut workspaces = Vec::with_capacity(resolved.len());
    let mut workspace_roots = BTreeSet::new();
    for (workspace_id, manifest) in resolved {
        let invocation = load_bounded_federated_cargo_metadata(&manifest, Path::new("cargo"))
            .map_err(|error| {
                error
                    .with_federated_workspace(&workspace_id)
                    .with_invocation_selection(request_selection.clone())
            })?;
        let metadata = decode_metadata(&invocation.bytes).map_err(|error| {
            error
                .with_federated_workspace(&workspace_id)
                .with_invocation_selection(request_selection.clone())
        })?;
        let workspace_root = canonical_federated_workspace_root(&metadata).map_err(|error| {
            error
                .with_federated_workspace(&workspace_id)
                .with_invocation_selection(request_selection.clone())
        })?;
        if !workspace_root.starts_with(&request_parent) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-PLAN-WORKSPACE-ROOT-OUTSIDE-REQUEST",
                "Cargo reported a workspace root outside the canonical request parent.",
                vec![
                    "Place the request at a common ancestor of every complete Cargo workspace."
                        .to_owned(),
                ],
            )
            .with_federated_workspace(&workspace_id)
            .with_invocation_selection(request_selection));
        }
        if !workspace_roots.insert(workspace_root.clone()) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-PLAN-WORKSPACE-ROOT-DUPLICATE",
                "The selected manifest belongs to a Cargo workspace already present in this request.",
                vec![
                    "Select exactly one manifest for each independent Cargo workspace.".to_owned(),
                ],
            )
            .with_federated_workspace(&workspace_id)
            .with_invocation_selection(request_selection));
        }
        let envelope = plan_from_decoded_metadata(
            &invocation.manifest_path,
            &workspace_id,
            metadata,
            &workspace_root,
            &invocation.bytes,
        )
        .map_err(|error| {
            error
                .with_federated_workspace(&workspace_id)
                .with_invocation_selection(request_selection.clone())
        })?;
        let plan = envelope.record.ok_or_else(|| {
            CoreError::new(
                ResultClass::Internal,
                "FERRIS-FEDERATED-PLAN-WORKSPACE-PLAN-MISSING",
                "A successful workspace plan did not contain its typed record.",
                vec!["Report this Ferris invariant failure.".to_owned()],
            )
            .with_invocation_selection(request_selection.clone())
        })?;
        workspaces.push(FederatedWorkspacePlan { workspace_id, plan });
    }

    let federated_plan_id = federated_plan_record_id(
        &request.application_id,
        &request.revision,
        &request.owner,
        &workspaces,
    )
    .map_err(|error| error.with_invocation_selection(request_selection.clone()))?;
    let record = FederatedPlanRecord {
        schema: FEDERATED_PLAN_SCHEMA.to_owned(),
        federated_plan_id,
        application_id: request.application_id,
        revision: request.revision,
        owner: request.owner,
        executable: false,
        workspaces,
        unknowns: vec![
            "No cross-workspace dependency, lock, affected, validation, native, service, or contract relationship is inferred."
                .to_owned(),
            "Repository policy, lifecycle, support, deployment, and execution requirements remain owner-defined and unobserved at the federation level."
                .to_owned(),
        ],
        limitations: vec![
            "Independent Cargo metadata invocations and workspace boundaries are not combined; the retained Blueprint Plan does not carry a Cargo lock digest or lock identity."
                .to_owned(),
            format!(
                "Cargo metadata runs sequentially once per requested workspace, with a {}-second timeout and {}-byte limit per stream for each workspace.",
                FEDERATED_PLAN_METADATA_TIMEOUT.as_secs(),
                MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES
            ),
            format!(
                "At the {MAX_FEDERATED_PLAN_WORKSPACES}-workspace maximum, sequential per-workspace time bounds permit up to {} seconds before owner-process startup and cleanup overhead.",
                MAX_FEDERATED_PLAN_WORKSPACES as u64
                    * FEDERATED_PLAN_METADATA_TIMEOUT.as_secs()
            ),
            "Timeout and output-bound termination covers the direct Cargo child; process-tree control for custom Cargo wrappers is outside this V0."
                .to_owned(),
            "Every complete Cargo workspace must share one request-parent ancestor; workspaces on different Windows drives cannot be grouped by this V0 request syntax."
                .to_owned(),
            "This federated plan is non-executable and no Cargo build, validation, native, service, contract, or other owner work is executed."
                .to_owned(),
            "The request records a bounded caller-authored grouping and is not a canonical Application Definition."
                .to_owned(),
        ],
    };

    Ok(success_envelope(
        "federated-plan",
        request_selection.clone(),
        federated_plan_invocation_identity(&request_selection),
        record,
    ))
}

fn read_federated_plan_request(path: &Path) -> Result<Vec<u8>, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-FEDERATED-PLAN-REQUEST-UNAVAILABLE",
            "The explicit federated plan request is missing or unreadable.",
            vec!["Pass a readable local JSON file with --request.".to_owned()],
        )
    })?;
    if !metadata.is_file() {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-FEDERATED-PLAN-REQUEST-NOT-FILE",
            "The explicit federated plan request is not a regular file.",
            vec!["Pass a readable local JSON file with --request.".to_owned()],
        ));
    }
    if metadata.len() > MAX_FEDERATED_PLAN_REQUEST_BYTES {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-FEDERATED-PLAN-REQUEST-OVERSIZED",
            format!(
                "The explicit federated plan request exceeds the {MAX_FEDERATED_PLAN_REQUEST_BYTES}-byte bound."
            ),
            vec!["Reduce the request below the documented bound.".to_owned()],
        ));
    }

    let file = fs::File::open(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-FEDERATED-PLAN-REQUEST-UNAVAILABLE",
            "The explicit federated plan request is missing or unreadable.",
            vec!["Pass a readable local JSON file with --request.".to_owned()],
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_FEDERATED_PLAN_REQUEST_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-FEDERATED-PLAN-REQUEST-UNAVAILABLE",
                "The explicit federated plan request could not be read completely.",
                vec!["Pass a readable local JSON file with --request.".to_owned()],
            )
        })?;
    if bytes.len() as u64 > MAX_FEDERATED_PLAN_REQUEST_BYTES {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-FEDERATED-PLAN-REQUEST-OVERSIZED",
            format!(
                "The explicit federated plan request exceeds the {MAX_FEDERATED_PLAN_REQUEST_BYTES}-byte bound."
            ),
            vec!["Reduce the request below the documented bound.".to_owned()],
        ));
    }
    Ok(bytes)
}

fn is_absolute_manifest_request(value: &str) -> bool {
    let path = Path::new(value);
    path.is_absolute()
        || path.has_root()
        || value.starts_with('/')
        || (value.as_bytes().get(1) == Some(&b':'))
}

fn validate_federated_manifest_request(value: &str) -> Result<(), CoreError> {
    if value.is_empty() || value.contains('\0') {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-MANIFEST-SYNTAX-INVALID",
            "Federated manifest paths must use non-empty portable request-relative syntax with forward slashes.",
            vec![
                "Use a forward-slash request-relative path such as workspace/Cargo.toml."
                    .to_owned(),
            ],
        ));
    }
    if is_absolute_manifest_request(value) {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-MANIFEST-ABSOLUTE",
            "Federated workspace manifest paths must not use absolute, rooted, or drive syntax.",
            vec!["Use a request-relative Cargo.toml path.".to_owned()],
        ));
    }
    if value.contains('\\') {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-MANIFEST-SYNTAX-INVALID",
            "Federated manifest paths must use non-empty portable request-relative syntax with forward slashes.",
            vec![
                "Use a forward-slash request-relative path such as workspace/Cargo.toml."
                    .to_owned(),
            ],
        ));
    }
    if value.split('/').any(|component| component == "..") {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-PLAN-MANIFEST-TRAVERSAL",
            "Federated workspace manifest paths must not contain a parent-directory component.",
            vec![
                "Place the request at a common ancestor and use a descendant manifest path."
                    .to_owned(),
            ],
        ));
    }
    Ok(())
}

fn canonical_federated_request_parent(request_path: &Path) -> Result<PathBuf, CoreError> {
    let parent = request_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    parent.canonicalize().map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-FEDERATED-PLAN-REQUEST-PARENT-UNAVAILABLE",
            "The federated plan request parent could not be resolved.",
            vec!["Pass a request from a readable local directory.".to_owned()],
        )
    })
}

pub fn create_validation_plan(
    manifest_path: &Path,
    workspace_id: &str,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> Result<CommandEnvelope<ValidationPlanRecord>, CoreError> {
    let request_selection_identity = validation_plan_request_selection_identity(
        workspace_id,
        manifest_path,
        changed_paths,
        changed_packages,
    );
    validate_workspace_id(workspace_id)
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    if changed_paths.is_empty() && changed_packages.is_empty() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-VALIDATION-PLAN-INPUT-MISSING",
            "The validation plan requires at least one explicit changed path or changed package.",
            vec!["Pass one or more --changed-path or --changed-package values.".to_owned()],
        )
        .with_invocation_selection(request_selection_identity));
    }
    if changed_paths.len().saturating_add(changed_packages.len()) > MAX_VALIDATION_INPUTS {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-VALIDATION-PLAN-INPUT-BOUND-EXCEEDED",
            format!(
                "The validation plan accepts at most {MAX_VALIDATION_INPUTS} explicit changed paths and packages in one request."
            ),
            vec![format!(
                "Split the request into batches below the {MAX_VALIDATION_INPUTS}-input bound."
            )],
        )
        .with_invocation_selection(request_selection_identity));
    }

    let invocation = load_cargo_metadata(manifest_path, Path::new("cargo"))
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    let metadata = decode_metadata(&invocation.bytes)
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    validation_plan_from_decoded_metadata(
        &invocation.manifest_path,
        &invocation.bytes,
        metadata,
        workspace_id,
        changed_paths,
        changed_packages,
        request_selection_identity,
    )
}

fn validation_plan_from_decoded_metadata(
    manifest_path: &Path,
    metadata_bytes: &[u8],
    metadata: CargoMetadata,
    workspace_id: &str,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
    request_selection_identity: String,
) -> Result<CommandEnvelope<ValidationPlanRecord>, CoreError> {
    let workspace_root = PathBuf::from(&metadata.workspace_root)
        .canonicalize()
        .map_err(|error| {
            CoreError::new(
                ResultClass::Internal,
                "FERRIS-CARGO-WORKSPACE-ROOT-INVALID",
                "Cargo reported a workspace root Ferris could not resolve safely.",
                vec![
                    "Run cargo metadata directly and retain its workspace_root field.".to_owned(),
                    "Report the Cargo and Ferris versions.".to_owned(),
                ],
            )
            .with_source_digest(digest_text(&format!(
                "{}\0{error}",
                normalize_path_text(&metadata.workspace_root)
            )))
        })
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    let selected_manifest = workspace_relative_path(manifest_path, &workspace_root)
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    let workspace_packages = workspace_packages_from_metadata(metadata, &workspace_root)
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    let all_packages = workspace_packages
        .iter()
        .map(|package| package.package.clone())
        .collect::<Vec<_>>();
    let mut directory_to_identities: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut packages_by_identity = BTreeMap::new();
    for package in &workspace_packages {
        directory_to_identities
            .entry(package.package_root.clone())
            .or_default()
            .push(package.package.identity.clone());
        packages_by_identity.insert(package.package.identity.clone(), package);
    }
    for identities in directory_to_identities.values_mut() {
        identities.sort();
    }

    let normalized_package_requests = normalize_validation_package_requests(changed_packages)
        .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    let mut selected_packages = BTreeMap::new();
    let mut input_builders = Vec::new();
    let mut fallback_reasons = Vec::new();

    for package_name in normalized_package_requests {
        let matches = workspace_packages
            .iter()
            .filter(|package| package.package.name == package_name)
            .collect::<Vec<_>>();
        let package = match matches.as_slice() {
            [package] => *package,
            [] => {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-VALIDATION-PACKAGE-NOT-FOUND",
                    "The validation plan changed-package input does not match a workspace package.",
                    vec![
                        "Pass an existing workspace package name or use --changed-path.".to_owned(),
                    ],
                )
                .with_invocation_selection(request_selection_identity.clone()));
            }
            _ => {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-VALIDATION-PACKAGE-AMBIGUOUS",
                    "The validation plan changed-package input matched more than one workspace package.",
                    vec![
                        "Use --changed-path with an exact workspace path until package names are unique."
                            .to_owned(),
                    ],
                )
                .with_invocation_selection(request_selection_identity.clone()));
            }
        };
        let reason = format!(
            "The caller explicitly named workspace package {}.",
            package.package.name
        );
        record_validation_package(
            &mut selected_packages,
            &package.package,
            ValidationPackageDisposition::Anchor,
            reason.clone(),
        );
        input_builders.push(ValidationInputBuilder {
            record: ValidationInputRecord {
                kind: ValidationInputKind::Package,
                value: package.package.name.clone(),
                disposition: ValidationInputDisposition::ExplicitPackage,
                package_identity: Some(package.package.identity.clone()),
                reason,
            },
            semantic_code: VALIDATION_INPUT_CODE_EXPLICIT_PACKAGE,
        });
    }

    let mut seen_paths = BTreeSet::new();
    for changed_path in changed_paths {
        let path_digest = digest_text(&lexically_normalize_path_text(
            &changed_path.to_string_lossy(),
        ));
        let metadata = fs::metadata(changed_path).map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-VALIDATION-CHANGE-PATH-UNAVAILABLE",
                "An explicit changed path is missing or unreadable.",
                vec![
                    "Pass an existing local path inside the selected workspace or use --changed-package."
                        .to_owned(),
                ],
            )
            .with_source_digest(path_digest.clone())
            .with_invocation_selection(request_selection_identity.clone())
        })?;
        if !metadata.is_file() && !metadata.is_dir() {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-VALIDATION-CHANGE-PATH-TYPE-INVALID",
                "An explicit changed path is not a regular file or directory.",
                vec![
                    "Pass an existing local file or directory inside the selected workspace."
                        .to_owned(),
                ],
            )
            .with_source_digest(path_digest)
            .with_invocation_selection(request_selection_identity.clone()));
        }
        let canonical_path = changed_path.canonicalize().map_err(|error| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-VALIDATION-CHANGE-PATH-UNAVAILABLE",
                "An explicit changed path could not be resolved completely.",
                vec![
                    "Pass an existing local path inside the selected workspace or use --changed-package."
                        .to_owned(),
                ],
            )
            .with_source_digest(digest_text(&format!(
                "{}\0{error}",
                lexically_normalize_path_text(&changed_path.to_string_lossy())
            )))
            .with_invocation_selection(request_selection_identity.clone())
        })?;
        let relative_path = explicit_workspace_relative_path(&canonical_path, &workspace_root)
            .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
        if !seen_paths.insert(relative_path.clone()) {
            continue;
        }
        let matching_packages = workspace_packages
            .iter()
            .filter(|package| {
                canonical_path == package.package_root_absolute
                    || canonical_path.starts_with(&package.package_root_absolute)
            })
            .collect::<Vec<_>>();
        if let [package] = matching_packages.as_slice() {
            if supports_validation_path_anchor(&canonical_path, &package.package_root_absolute) {
                let reason = format!(
                    "The explicit changed path {relative_path} is a supported package-owned Rust anchor for workspace package {}.",
                    package.package.name
                );
                record_validation_package(
                    &mut selected_packages,
                    &package.package,
                    ValidationPackageDisposition::Anchor,
                    reason.clone(),
                );
                input_builders.push(ValidationInputBuilder {
                    record: ValidationInputRecord {
                        kind: ValidationInputKind::Path,
                        value: relative_path,
                        disposition: ValidationInputDisposition::OwnedRustPath,
                        package_identity: Some(package.package.identity.clone()),
                        reason,
                    },
                    semantic_code: VALIDATION_INPUT_CODE_OWNED_RUST_PATH,
                });
            } else {
                let reason = format!(
                    "The explicit changed path {relative_path} is inside workspace package {} but is not an exact package root or a non-build Rust path, so this pulse widens to the full workspace fallback.",
                    package.package.name
                );
                fallback_reasons.push(reason.clone());
                input_builders.push(ValidationInputBuilder {
                    record: ValidationInputRecord {
                        kind: ValidationInputKind::Path,
                        value: relative_path,
                        disposition: ValidationInputDisposition::FullWorkspaceFallback,
                        package_identity: Some(package.package.identity.clone()),
                        reason,
                    },
                    semantic_code:
                        VALIDATION_INPUT_CODE_PACKAGE_PATH_REQUIRES_FULL_WORKSPACE_FALLBACK,
                });
            }
        } else {
            let (reason, semantic_code) = if matching_packages.is_empty() {
                (
                    format!(
                        "The explicit changed path {relative_path} is inside the selected workspace but outside any package-owned Rust anchor, so this pulse widens to the full workspace fallback."
                    ),
                    VALIDATION_INPUT_CODE_WORKSPACE_PATH_OUTSIDE_PACKAGE_ANCHOR,
                )
            } else {
                (
                    format!(
                        "The explicit changed path {relative_path} falls under more than one workspace package root, so this pulse widens to the full workspace fallback."
                    ),
                    VALIDATION_INPUT_CODE_AMBIGUOUS_PACKAGE_ROOT_MATCH,
                )
            };
            fallback_reasons.push(reason.clone());
            input_builders.push(ValidationInputBuilder {
                record: ValidationInputRecord {
                    kind: ValidationInputKind::Path,
                    value: relative_path,
                    disposition: ValidationInputDisposition::FullWorkspaceFallback,
                    package_identity: None,
                    reason,
                },
                semantic_code,
            });
        }
    }

    let mut reverse_dependencies: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for package in &workspace_packages {
        for dependency in &package.dependencies {
            let (target, resolution) =
                dependency_target(dependency, &workspace_root, &directory_to_identities);
            if resolution == "workspace-member"
                && let Some(target) = target
            {
                reverse_dependencies
                    .entry(target)
                    .or_default()
                    .push(package.package.identity.clone());
            }
        }
    }
    for dependents in reverse_dependencies.values_mut() {
        dependents.sort();
        dependents.dedup();
    }

    let mut closure_queue = selected_packages.keys().cloned().collect::<Vec<_>>();
    let mut closure_seen = selected_packages.keys().cloned().collect::<BTreeSet<_>>();
    while let Some(identity) = closure_queue.pop() {
        let Some(source_package) = packages_by_identity.get(&identity) else {
            continue;
        };
        let Some(dependents) = reverse_dependencies.get(&identity) else {
            continue;
        };
        for dependent in dependents {
            if !closure_seen.insert(dependent.clone()) {
                continue;
            }
            let dependent_package = packages_by_identity.get(dependent).ok_or_else(|| {
                CoreError::new(
                    ResultClass::Internal,
                    "FERRIS-VALIDATION-CLOSURE-INCONSISTENT",
                    "A Cargo workspace dependency closure was missing from Ferris normalization.",
                    vec!["Report this Ferris invariant failure.".to_owned()],
                )
            })?;
            let reason = format!(
                "Workspace package {} depends on selected package {} through the Cargo-declared workspace closure.",
                dependent_package.package.name, source_package.package.name
            );
            record_validation_package(
                &mut selected_packages,
                &dependent_package.package,
                ValidationPackageDisposition::ReverseDependency,
                reason,
            );
            closure_queue.push(dependent.clone());
        }
    }

    input_builders.sort_by(|left, right| {
        validation_input_kind_name(left.record.kind)
            .cmp(validation_input_kind_name(right.record.kind))
            .then_with(|| left.record.value.cmp(&right.record.value))
    });
    let inputs = input_builders
        .iter()
        .map(|builder| builder.record.clone())
        .collect::<Vec<_>>();
    let selection_identity =
        validation_plan_selection_identity(workspace_id, &selected_manifest, &inputs);
    let invocation_identity = validation_plan_invocation_identity(&selection_identity);

    let mut selected_package_values = selected_packages
        .into_values()
        .map(|mut package| {
            package.reasons.sort();
            package.reasons.dedup();
            ValidationPackageSelection {
                package: package.package,
                disposition: package.disposition,
                reasons: package.reasons,
            }
        })
        .collect::<Vec<_>>();
    selected_package_values
        .sort_by(|left, right| left.package.identity.cmp(&right.package.identity));
    let selected_package_records = selected_package_values
        .iter()
        .map(|package| package.package.clone())
        .collect::<Vec<_>>();
    let selected_activities = validation_activity_plans(
        &selected_package_records,
        ValidationActivityScope::SelectedPackageClosure,
        "Ferris retained a package-specific Cargo activity plan from the caller's supported explicit package and Rust-path anchors.",
    );

    fallback_reasons.push(
        "Selected-package evidence is not full-reference evidence; use the full workspace fallback before making repository-wide, release, or CI claims."
            .to_owned(),
    );
    fallback_reasons.push(
        "Repository policy, lint, feature, target, profile, release, native, environment, generated-input, build-script, macro, and other non-Cargo validation requirements remain owner-defined outside this pulse."
            .to_owned(),
    );
    fallback_reasons.sort();
    fallback_reasons.dedup();
    let fallback = ValidationFallbackPlan {
        boundary: "full-workspace-plus-owner-reference".to_owned(),
        required_by_inputs: inputs
            .iter()
            .any(|input| input.disposition == ValidationInputDisposition::FullWorkspaceFallback),
        reasons: fallback_reasons,
        packages: all_packages.clone(),
        activities: validation_activity_plans(
            &all_packages,
            ValidationActivityScope::FullWorkspaceFallback,
            "Ferris retained the full workspace fallback so owner-native full-reference validation remains available.",
        ),
    };

    let validation_plan_id = validation_plan_record_id(
        workspace_id,
        &selected_manifest,
        &input_builders,
        &selected_package_values,
        &selected_activities,
        &fallback,
    )
    .map_err(|error| error.with_invocation_selection(selection_identity.clone()))?;
    let record = ValidationPlanRecord {
        schema: VALIDATION_PLAN_SCHEMA.to_owned(),
        validation_plan_id,
        workspace_id: workspace_id.to_owned(),
        executable: false,
        selected_manifest: selected_manifest.clone(),
        workspace_root: ".".to_owned(),
        inputs,
        selected_packages: selected_package_values,
        selected_activities,
        fallback,
        evidence: metadata_evidence(&selected_manifest, workspace_id, metadata_bytes),
        unknowns: vec![
            "Cargo metadata does not declare repository-required format, Clippy, release, native, environment, runtime-data, or policy gates in this pulse."
                .to_owned(),
            "Feature, target, profile, doctest, and execution requirements remain owner-defined; this record keeps only Cargo check/test activity families explicit."
                .to_owned(),
        ],
        limitations: vec![
            "This record does not execute Cargo validation commands or observe their results."
                .to_owned(),
            "Selected packages are a conservative Cargo package closure, not a full-suite, release, platform, support, or CI equivalence claim."
                .to_owned(),
            "Only explicit package names, exact package roots, and existing non-build Rust paths inside one workspace package narrow package scope in this pulse."
                .to_owned(),
        ],
    };

    Ok(success_envelope(
        "validation-plan",
        selection_identity,
        invocation_identity,
        record,
    ))
}

pub fn create_federated_validation_plan(
    application_path: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> Result<CommandEnvelope<FederatedValidationPlanRecord>, CoreError> {
    let provisional_selection_identity =
        federated_validation_plan_provisional_request_selection_identity(
            changed_paths,
            changed_packages,
        );
    if changed_paths.is_empty() && changed_packages.is_empty() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-FEDERATED-VALIDATION-INPUT-MISSING",
            "The federated validation plan requires at least one explicit changed path or workspace-qualified changed package.",
            vec![
                "Pass one or more --changed-path values or --changed-package WORKSPACE_ID:PACKAGE values."
                    .to_owned(),
            ],
        )
        .with_invocation_selection(provisional_selection_identity));
    }
    if changed_paths.len().saturating_add(changed_packages.len()) > MAX_VALIDATION_INPUTS {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-FEDERATED-VALIDATION-INPUT-BOUND-EXCEEDED",
            format!(
                "The federated validation plan accepts at most {MAX_VALIDATION_INPUTS} explicit changed paths and packages in one request."
            ),
            vec![format!(
                "Split the request into batches below the {MAX_VALIDATION_INPUTS}-input bound."
            )],
        )
        .with_invocation_selection(provisional_selection_identity));
    }

    let loaded = load_application_definition(application_path)
        .map_err(|error| error.with_invocation_selection(provisional_selection_identity))?;
    let request_selection_identity = federated_validation_plan_semantic_request_selection_identity(
        &loaded.definition_identity,
        &loaded.application_root,
        changed_paths,
        changed_packages,
    );
    let declared_workspace_ids = loaded
        .definition
        .workspaces
        .iter()
        .map(|workspace| workspace.workspace_id.clone())
        .collect::<BTreeSet<_>>();
    let changed_packages_by_workspace =
        qualify_federated_validation_packages(changed_packages, &declared_workspace_ids)
            .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;
    let mut changed_paths_by_workspace: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
    let mut application_fallback_paths = Vec::new();

    for changed_path in changed_paths {
        let path_digest = digest_text(&lexically_normalize_path_text(
            &changed_path.to_string_lossy(),
        ));
        let metadata = fs::metadata(changed_path).map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-FEDERATED-VALIDATION-CHANGE-PATH-UNAVAILABLE",
                "An explicit federated changed path is missing or unreadable.",
                vec![
                    "Pass an existing local file or directory beneath the application definition directory."
                        .to_owned(),
                ],
            )
            .with_source_digest(path_digest.clone())
            .with_invocation_selection(request_selection_identity.clone())
        })?;
        if !metadata.is_file() && !metadata.is_dir() {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-VALIDATION-CHANGE-PATH-TYPE-INVALID",
                "An explicit federated changed path is not a regular file or directory.",
                vec![
                    "Pass an existing regular file or directory beneath the application definition directory."
                        .to_owned(),
                ],
            )
            .with_source_digest(path_digest)
            .with_invocation_selection(request_selection_identity));
        }
        let canonical_path = changed_path.canonicalize().map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-FEDERATED-VALIDATION-CHANGE-PATH-UNAVAILABLE",
                "An explicit federated changed path could not be resolved completely.",
                vec![
                    "Pass an existing local file or directory beneath the application definition directory."
                        .to_owned(),
                ],
            )
            .with_source_digest(path_digest)
            .with_invocation_selection(request_selection_identity.clone())
        })?;
        let application_relative =
            canonical_path
                .strip_prefix(&loaded.application_root)
                .map_err(|_| {
                    CoreError::new(
                        ResultClass::Invalid,
                        "FERRIS-FEDERATED-VALIDATION-CHANGE-PATH-OUTSIDE-APPLICATION",
                        "An explicit federated changed path is outside the application definition directory.",
                        vec![
                            "Pass an existing path beneath the application definition directory."
                                .to_owned(),
                        ],
                    )
                    .with_source_digest(digest_text(&lexically_normalize_path_text(
                        &changed_path.to_string_lossy(),
                    )))
                    .with_invocation_selection(request_selection_identity.clone())
                })?;
        let owners = loaded
            .workspaces
            .iter()
            .filter(|workspace| canonical_path.starts_with(&workspace.workspace_root))
            .collect::<Vec<_>>();
        match owners.as_slice() {
            [workspace] => changed_paths_by_workspace
                .entry(workspace.definition.workspace_id.clone())
                .or_default()
                .push(canonical_path),
            [] => application_fallback_paths.push(portable_relative_path(application_relative)),
            _ => {
                return Err(CoreError::new(
                    ResultClass::Internal,
                    "FERRIS-FEDERATED-VALIDATION-WORKSPACE-OWNERSHIP-AMBIGUOUS",
                    "A changed path matched more than one declared Cargo workspace root.",
                    vec!["Report this Ferris application validation invariant failure.".to_owned()],
                )
                .with_invocation_selection(request_selection_identity));
            }
        }
    }

    let mut direct_plans = BTreeMap::new();
    let mut direct_workspace_ids = BTreeSet::new();
    for workspace in loaded.workspaces {
        let workspace_id = workspace.definition.workspace_id.clone();
        let paths = changed_paths_by_workspace
            .get(&workspace_id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let packages = changed_packages_by_workspace
            .get(&workspace_id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        if paths.is_empty() && packages.is_empty() {
            continue;
        }
        let envelope = validation_plan_from_decoded_metadata(
            &workspace.manifest_path,
            &workspace.metadata_bytes,
            workspace.metadata,
            &workspace_id,
            paths,
            packages,
            request_selection_identity.clone(),
        )
        .map_err(|error| {
            error
                .with_federated_workspace(&workspace_id)
                .with_invocation_selection(request_selection_identity.clone())
        })?;
        direct_workspace_ids.insert(workspace_id.clone());
        direct_plans.insert(
            workspace_id,
            envelope
                .record
                .expect("successful workspace validation plan has a record"),
        );
    }

    let mut dependents_by_workspace: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for workspace in &loaded.definition.workspaces {
        for dependency in &workspace.depends_on {
            dependents_by_workspace
                .entry(dependency.clone())
                .or_default()
                .push(workspace.workspace_id.clone());
        }
    }
    for dependents in dependents_by_workspace.values_mut() {
        dependents.sort();
        dependents.dedup();
    }

    let mut relationship_workspace_ids = BTreeSet::new();
    let mut frontier = direct_workspace_ids.iter().cloned().collect::<Vec<_>>();
    while let Some(affected_workspace) = frontier.pop() {
        let Some(dependents) = dependents_by_workspace.get(&affected_workspace) else {
            continue;
        };
        for dependent in dependents {
            if direct_workspace_ids.contains(dependent)
                || !relationship_workspace_ids.insert(dependent.clone())
            {
                continue;
            }
            frontier.push(dependent.clone());
        }
    }

    application_fallback_paths.sort();
    application_fallback_paths.dedup();
    let application_fallback_required = !application_fallback_paths.is_empty();
    let mut workspaces = Vec::with_capacity(loaded.definition.workspaces.len());
    for workspace in &loaded.definition.workspaces {
        let workspace_id = &workspace.workspace_id;
        let direct_plan = direct_plans.remove(workspace_id);
        let (disposition, mut reasons) = if application_fallback_required {
            (
                FederatedValidationWorkspaceDisposition::ApplicationFallback,
                application_fallback_paths
                    .iter()
                    .map(|path| {
                        format!(
                            "Application-relative changed path {path} is outside every declared workspace root, so all workspaces require the full-application fallback."
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        } else if direct_workspace_ids.contains(workspace_id) {
            (
                FederatedValidationWorkspaceDisposition::DirectPlan,
                vec![
                    "This workspace directly owns at least one explicit changed path or workspace-qualified package input."
                        .to_owned(),
                ],
            )
        } else if relationship_workspace_ids.contains(workspace_id) {
            let mut affected_dependencies = workspace
                .depends_on
                .iter()
                .filter(|dependency| {
                    direct_workspace_ids.contains(*dependency)
                        || relationship_workspace_ids.contains(*dependency)
                })
                .cloned()
                .collect::<Vec<_>>();
            affected_dependencies.sort();
            (
                FederatedValidationWorkspaceDisposition::RelationshipFallback,
                vec![format!(
                    "The application declares this workspace depends on affected workspace(s) {}; V0 requires full-workspace owner validation without fabricating a changed input.",
                    affected_dependencies.join(", ")
                )],
            )
        } else {
            (
                FederatedValidationWorkspaceDisposition::NotSelected,
                vec![
                    "No explicit input or reverse application relationship selected this workspace."
                        .to_owned(),
                ],
            )
        };
        if direct_plan.is_some() && application_fallback_required {
            reasons.push(
                "A direct workspace plan is retained as evidence, but it does not narrow the required full-application fallback."
                    .to_owned(),
            );
        }
        reasons.sort();
        reasons.dedup();
        workspaces.push(FederatedValidationWorkspacePlan {
            workspace_id: workspace_id.clone(),
            manifest_path: workspace.manifest_path.clone(),
            disposition,
            reasons,
            validation_plan: direct_plan,
        });
    }
    workspaces.sort_by(|left, right| left.workspace_id.cmp(&right.workspace_id));

    let mut workspace_ids = declared_workspace_ids.into_iter().collect::<Vec<_>>();
    workspace_ids.sort();
    let mut fallback_reasons = vec![
        "Cargo metadata remains authoritative inside each independent workspace; application relationships do not merge Cargo resolution or lockfiles."
            .to_owned(),
        "Repository and application owners retain validation commands, policy, release, native, environment, and support decisions."
            .to_owned(),
    ];
    for path in &application_fallback_paths {
        fallback_reasons.push(format!(
            "Application-relative changed path {path} has no declared workspace owner."
        ));
    }
    fallback_reasons.sort();
    fallback_reasons.dedup();

    let fallback = FederatedValidationFallback {
        boundary: "full-application-plus-owner-reference".to_owned(),
        required_by_inputs: application_fallback_required,
        workspace_ids,
        reasons: fallback_reasons,
    };
    let mut record = FederatedValidationPlanRecord {
        schema: FEDERATED_VALIDATION_PLAN_SCHEMA.to_owned(),
        federated_validation_plan_id: String::new(),
        application_id: loaded.definition.application_id,
        application_definition: loaded.application_name,
        executable: false,
        workspaces,
        fallback,
        unknowns: vec![
            "Application relationships declare validation propagation only; they do not establish package, artifact, ABI, runtime, deployment, or support compatibility."
                .to_owned(),
            "Validation requirements outside Cargo metadata remain application- and workspace-owner defined."
                .to_owned(),
        ],
        limitations: vec![
            "This record does not execute validation commands, discover Git changes, resolve across workspaces, mutate inputs, or collect remote evidence."
                .to_owned(),
            "Relationship fallback is intentionally full-workspace and contains no fabricated changed input or per-workspace plan."
                .to_owned(),
            "This bounded consumer-owned application record is not the full APPLICATION-001 model."
                .to_owned(),
        ],
    };
    record.federated_validation_plan_id = federated_validation_plan_identity(
        "federated-validation-plan",
        &loaded.definition_identity,
        &record,
    )
    .map_err(|error| error.with_invocation_selection(request_selection_identity.clone()))?;

    Ok(success_envelope(
        "federated-validation-plan",
        request_selection_identity.clone(),
        federated_validation_plan_invocation_identity(&request_selection_identity),
        record,
    ))
}

pub fn create_revision_skew_report(
    request_path: &Path,
) -> Result<CommandEnvelope<RevisionSkewReportRecord>, CoreError> {
    let request_bytes = read_revision_skew_request(request_path)?;
    let mut request: RevisionSkewRequest =
        serde_json::from_slice(&request_bytes).map_err(|_| {
            CoreError::new(
                ResultClass::Invalid,
                "FERRIS-REVISION-SKEW-REQUEST-INVALID",
                "The revision-skew request is not valid strict ferris.revision-skew-request/v0 JSON.",
                vec![
                    "Use only the documented analysis, producer, consumer, and dependency fields."
                        .to_owned(),
                ],
            )
            .with_source_digest(digest_bytes(&request_bytes))
        })?;
    let provisional_selection =
        selection_identity("revision-skew-request", &digest_bytes(&request_bytes));
    if request.schema != REVISION_SKEW_REQUEST_SCHEMA {
        return Err(CoreError::new(
            ResultClass::Unsupported,
            "FERRIS-REVISION-SKEW-SCHEMA-UNSUPPORTED",
            "The revision-skew request schema is unsupported.",
            vec![format!("Use schema {REVISION_SKEW_REQUEST_SCHEMA}.")],
        )
        .with_invocation_selection(provisional_selection));
    }
    if !valid_portable_id(&request.analysis_id) {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-REVISION-SKEW-ANALYSIS-ID-INVALID",
            "The revision-skew analysis identity is invalid.",
            vec!["Use a non-empty printable portable identity.".to_owned()],
        )
        .with_invocation_selection(provisional_selection));
    }
    if request.producers.is_empty()
        || request.producers.len() > MAX_REVISION_SKEW_PRODUCERS
        || request.consumers.is_empty()
        || request.consumers.len() > MAX_REVISION_SKEW_CONSUMERS
    {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-REVISION-SKEW-REQUEST-BOUND-INVALID",
            format!(
                "The revision-skew request requires 1-{MAX_REVISION_SKEW_PRODUCERS} producers and 1-{MAX_REVISION_SKEW_CONSUMERS} consumers."
            ),
            vec!["Split larger comparisons into explicit bounded requests.".to_owned()],
        )
        .with_invocation_selection(provisional_selection));
    }

    request
        .producers
        .sort_by(|left, right| left.producer_id.cmp(&right.producer_id));
    request
        .consumers
        .sort_by(|left, right| left.consumer_id.cmp(&right.consumer_id));
    for consumer in &mut request.consumers {
        consumer.dependencies.sort_by(|left, right| {
            left.producer_id
                .cmp(&right.producer_id)
                .then_with(|| left.package_name.cmp(&right.package_name))
        });
    }
    let selection_identity = canonical_value_digest(&request)
        .map(|digest| selection_identity("revision-skew", &digest))
        .map_err(|error| error.with_invocation_selection(provisional_selection.clone()))?;
    validate_revision_skew_request(&request)
        .map_err(|error| error.with_invocation_selection(selection_identity.clone()))?;

    let request_path = request_path.canonicalize().map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-REVISION-SKEW-REQUEST-UNAVAILABLE",
            "The explicit revision-skew request path could not be resolved.",
            vec!["Pass an existing local request file.".to_owned()],
        )
        .with_invocation_selection(selection_identity.clone())
    })?;
    let request_root = request_path.parent().ok_or_else(|| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-REVISION-SKEW-REQUEST-PARENT-INVALID",
            "The revision-skew request has no selectable parent directory.",
            vec!["Place the request beneath a local comparison root.".to_owned()],
        )
        .with_invocation_selection(selection_identity.clone())
    })?;

    let mut producers = BTreeMap::new();
    for producer in &request.producers {
        let checkout_path =
            canonical_request_child(request_root, &producer.checkout_path, "producer checkout")
                .map_err(|error| error.with_invocation_selection(selection_identity.clone()))?;
        producers.insert(
            producer.producer_id.clone(),
            (
                normalize_git_repository(&producer.repository_url),
                checkout_path,
                producer.observed_revision.clone(),
            ),
        );
    }

    let cargo_program = Path::new("cargo");
    let mut dependencies = Vec::new();
    for consumer in &request.consumers {
        let manifest_path =
            canonical_request_child(request_root, &consumer.manifest_path, "consumer manifest")
                .map_err(|error| error.with_invocation_selection(selection_identity.clone()))?;
        if manifest_path.file_name() != Some(std::ffi::OsStr::new("Cargo.toml")) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-REVISION-SKEW-MANIFEST-PATH-INVALID",
                "A consumer manifest path does not select Cargo.toml.",
                vec!["Select an explicit Cargo.toml beneath the request directory.".to_owned()],
            )
            .with_invocation_selection(selection_identity));
        }
        let metadata_invocation =
            load_bounded_revision_skew_cargo_metadata(&manifest_path, cargo_program).map_err(
                |error| {
                    error
                        .with_federated_workspace(&consumer.consumer_id)
                        .with_invocation_selection(selection_identity.clone())
                },
            )?;
        let metadata: CargoMetadata =
            serde_json::from_slice(&metadata_invocation.bytes).map_err(|_| {
                CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-REVISION-SKEW-METADATA-INVALID",
                    "Cargo returned revision-skew metadata that Ferris could not decode.",
                    vec![
                        "Run the recorded locked and offline Cargo metadata command directly."
                            .to_owned(),
                    ],
                )
                .with_source_digest(digest_bytes(&metadata_invocation.bytes))
                .with_invocation_selection(selection_identity.clone())
            })?;
        let workspace_root = Path::new(&metadata.workspace_root)
            .canonicalize()
            .map_err(|_| {
                CoreError::new(
                    ResultClass::Incomplete,
                    "FERRIS-REVISION-SKEW-WORKSPACE-ROOT-UNAVAILABLE",
                    "Cargo reported a workspace root that could not be resolved.",
                    vec![
                        "Run Cargo metadata directly and verify the workspace_root field."
                            .to_owned(),
                    ],
                )
                .with_invocation_selection(selection_identity.clone())
            })?;
        if !workspace_root.starts_with(request_root) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-REVISION-SKEW-WORKSPACE-ROOT-OUTSIDE-REQUEST",
                "A consumer manifest belongs to a Cargo workspace outside the request directory.",
                vec![
                    "Place the request at a common ancestor of every compared Cargo workspace root."
                        .to_owned(),
                ],
            )
            .with_invocation_selection(selection_identity));
        }
        for dependency in &consumer.dependencies {
            let (repository, checkout_path, observed_revision) = producers
                .get(&dependency.producer_id)
                .expect("validated producer reference exists");
            let declaration =
                revision_declaration_evidence(&metadata, &dependency.package_name, repository);
            let resolved_revisions = resolved_git_revisions_from_lock(
                &workspace_root,
                &dependency.package_name,
                repository,
            );
            let (resolved_revision, mut reasons) = match resolved_revisions {
                Ok(revisions) => match revisions.as_slice() {
                    [revision] => (Some(revision.clone()), Vec::new()),
                    [] => (
                        None,
                        vec![
                            "The Cargo lockfile exposed no git package matching the explicit package and producer repository."
                                .to_owned(),
                        ],
                    ),
                    _ => (
                        None,
                        vec![
                            "The Cargo lockfile exposed multiple revisions for the explicit package and producer repository."
                                .to_owned(),
                        ],
                    ),
                },
                Err(()) => (
                    None,
                    vec![
                        "The Cargo lockfile was missing, oversized, unreadable, or structurally unusable for bounded revision evidence."
                            .to_owned(),
                    ],
                ),
            };
            if declaration.kind == RevisionDeclarationKind::Missing {
                reasons.push(
                    "No workspace-member package declared the explicit git dependency.".to_owned(),
                );
            } else if declaration.kind == RevisionDeclarationKind::Ambiguous {
                reasons.push(
                    "Workspace-member packages declared more than one source mode for the explicit dependency."
                        .to_owned(),
                );
            }
            let status = classify_revision_skew(
                checkout_path,
                observed_revision,
                resolved_revision.as_deref(),
                &mut reasons,
            );
            reasons.sort();
            reasons.dedup();
            dependencies.push(RevisionSkewDependencyRecord {
                consumer_id: consumer.consumer_id.clone(),
                producer_id: dependency.producer_id.clone(),
                package_name: dependency.package_name.clone(),
                declaration,
                resolved_revision,
                observed_revision: observed_revision.clone(),
                status,
                reasons,
            });
        }
    }
    dependencies.sort_by(|left, right| {
        left.consumer_id
            .cmp(&right.consumer_id)
            .then_with(|| left.producer_id.cmp(&right.producer_id))
            .then_with(|| left.package_name.cmp(&right.package_name))
    });

    let mut record = RevisionSkewReportRecord {
        schema: REVISION_SKEW_REPORT_SCHEMA.to_owned(),
        report_id: String::new(),
        analysis_id: request.analysis_id,
        executable: false,
        dependencies,
        unknowns: vec![
            "Revision ancestry does not establish source, API, ABI, behavioral, data, deployment, or support compatibility."
                .to_owned(),
            "Dependencies not named explicitly by the request were not inspected.".to_owned(),
        ],
        limitations: vec![
            "Ferris used locked and offline Cargo metadata plus local read-only Git ancestry checks; it requested no network access, build, test, validation, checkout, fetch, or mutation."
                .to_owned(),
            "Declaration evidence is limited to workspace-member package dependency records exposed by Cargo metadata."
                .to_owned(),
            "Observed producer revisions are accepted only when they equal the explicit local checkout HEAD."
                .to_owned(),
        ],
    };
    record.report_id = record_id("revision-skew-report", &record)
        .map_err(|error| error.with_invocation_selection(selection_identity.clone()))?;
    Ok(success_envelope(
        "revision-skew",
        selection_identity.clone(),
        revision_skew_invocation_identity(&selection_identity),
        record,
    ))
}

fn validate_revision_skew_request(request: &RevisionSkewRequest) -> Result<(), CoreError> {
    let mut producer_ids = BTreeSet::new();
    for producer in &request.producers {
        if !valid_portable_id(&producer.producer_id)
            || !valid_git_repository(&producer.repository_url)
            || !valid_revision(&producer.observed_revision)
            || !valid_relative_request_path(&producer.checkout_path)
            || !producer_ids.insert(producer.producer_id.clone())
        {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-REVISION-SKEW-PRODUCER-INVALID",
                "A producer declaration is invalid or duplicated.",
                vec![
                    "Use unique portable producer IDs, explicit git repository URLs, relative checkout paths, and lowercase 40-character revisions."
                        .to_owned(),
                ],
            ));
        }
    }
    let mut consumer_ids = BTreeSet::new();
    let mut dependency_count = 0usize;
    for consumer in &request.consumers {
        if !valid_portable_id(&consumer.consumer_id)
            || !valid_relative_request_path(&consumer.manifest_path)
            || consumer.dependencies.is_empty()
            || !consumer_ids.insert(consumer.consumer_id.clone())
        {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-REVISION-SKEW-CONSUMER-INVALID",
                "A consumer declaration is invalid, duplicated, or has no explicit dependencies.",
                vec![
                    "Use unique portable consumer IDs, relative Cargo.toml paths, and at least one explicit dependency."
                        .to_owned(),
                ],
            ));
        }
        let mut dependencies = BTreeSet::new();
        for dependency in &consumer.dependencies {
            dependency_count = dependency_count.saturating_add(1);
            if !producer_ids.contains(&dependency.producer_id)
                || !valid_package_name(&dependency.package_name)
                || !dependencies.insert((
                    dependency.producer_id.clone(),
                    dependency.package_name.clone(),
                ))
            {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-REVISION-SKEW-DEPENDENCY-INVALID",
                    "A consumer dependency is invalid, duplicated, or names no declared producer.",
                    vec![
                        "Use one unique producer_id and Cargo package name pair per consumer dependency."
                            .to_owned(),
                    ],
                ));
            }
        }
    }
    if dependency_count > MAX_REVISION_SKEW_DEPENDENCIES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-REVISION-SKEW-DEPENDENCY-BOUND-EXCEEDED",
            format!(
                "The revision-skew request accepts at most {MAX_REVISION_SKEW_DEPENDENCIES} explicit dependency comparisons."
            ),
            vec!["Split the comparison into smaller explicit requests.".to_owned()],
        ));
    }
    Ok(())
}

fn valid_git_repository(value: &str) -> bool {
    valid_federated_metadata(value)
        && (value.starts_with("https://")
            || value.starts_with("http://")
            || value.starts_with("ssh://")
            || value.starts_with("git://")
            || value.starts_with("file://"))
}

fn valid_revision(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_package_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FEDERATED_PLAN_METADATA_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

fn valid_relative_request_path(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 1024
        && !Path::new(value).is_absolute()
        && Path::new(value).components().all(|component| {
            matches!(
                component,
                std::path::Component::Normal(_) | std::path::Component::CurDir
            )
        })
}

fn canonical_request_child(
    request_root: &Path,
    relative_path: &str,
    label: &str,
) -> Result<PathBuf, CoreError> {
    if !valid_relative_request_path(relative_path) {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-REVISION-SKEW-PATH-INVALID",
            format!("An explicit {label} path is not a safe relative path."),
            vec![
                "Use a relative path beneath the request directory without parent traversal."
                    .to_owned(),
            ],
        ));
    }
    let canonical = request_root
        .join(relative_path)
        .canonicalize()
        .map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-REVISION-SKEW-PATH-UNAVAILABLE",
                format!("An explicit {label} path is missing or unreadable."),
                vec!["Make the explicit local evidence path available and retry.".to_owned()],
            )
        })?;
    if !canonical.starts_with(request_root) {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-REVISION-SKEW-PATH-OUTSIDE-REQUEST",
            format!("An explicit {label} path resolves outside the request directory."),
            vec!["Keep all compared local evidence beneath the request directory.".to_owned()],
        ));
    }
    Ok(canonical)
}

fn read_revision_skew_request(path: &Path) -> Result<Vec<u8>, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-REVISION-SKEW-REQUEST-UNAVAILABLE",
            "The explicit revision-skew request is missing or unreadable.",
            vec!["Pass an existing local JSON request file.".to_owned()],
        )
    })?;
    if !metadata.is_file() || metadata.len() > MAX_REVISION_SKEW_REQUEST_BYTES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-REVISION-SKEW-REQUEST-BOUND-EXCEEDED",
            format!(
                "The revision-skew request must be a regular file no larger than {MAX_REVISION_SKEW_REQUEST_BYTES} bytes."
            ),
            vec!["Use a smaller explicit request file.".to_owned()],
        ));
    }
    fs::read(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-REVISION-SKEW-REQUEST-UNAVAILABLE",
            "The explicit revision-skew request could not be read completely.",
            vec!["Repair local file access and retry.".to_owned()],
        )
    })
}

fn normalize_git_repository(value: &str) -> String {
    let repository = value
        .strip_prefix("git+")
        .unwrap_or(value)
        .split(['?', '#'])
        .next()
        .unwrap_or(value)
        .trim_end_matches('/');
    repository
        .strip_suffix(".git")
        .unwrap_or(repository)
        .to_owned()
}

fn revision_declaration_evidence(
    metadata: &CargoMetadata,
    package_name: &str,
    repository: &str,
) -> RevisionDeclarationEvidence {
    let workspace_members = metadata.workspace_members.iter().collect::<BTreeSet<_>>();
    let mut sources = metadata
        .packages
        .iter()
        .filter(|package| workspace_members.contains(&package.id))
        .flat_map(|package| package.dependencies.iter())
        .filter(|dependency| dependency.name == package_name)
        .filter_map(|dependency| dependency.source.as_deref())
        .filter(|source| normalize_git_repository(source) == repository)
        .map(revision_declaration_source)
        .collect::<Vec<_>>();
    sources.sort();
    sources.dedup();
    let kind = if sources.is_empty() {
        RevisionDeclarationKind::Missing
    } else {
        let kinds = sources
            .iter()
            .map(|source| {
                source
                    .split_once(':')
                    .map_or(source.as_str(), |pair| pair.0)
            })
            .collect::<BTreeSet<_>>();
        if kinds.len() != 1 {
            RevisionDeclarationKind::Ambiguous
        } else {
            match *kinds.iter().next().expect("one declaration kind") {
                "branch" => RevisionDeclarationKind::Branch,
                "revision" => RevisionDeclarationKind::Revision,
                "tag" => RevisionDeclarationKind::Tag,
                "default_branch" => RevisionDeclarationKind::DefaultBranch,
                _ => RevisionDeclarationKind::Ambiguous,
            }
        }
    };
    RevisionDeclarationEvidence { kind, sources }
}

fn revision_declaration_source(source: &str) -> String {
    let query = source.split_once('?').map(|(_, value)| value).unwrap_or("");
    let query = query.split('#').next().unwrap_or(query);
    for field in query.split('&') {
        if let Some(value) = field.strip_prefix("branch=") {
            return format!("branch:{value}");
        }
        if let Some(value) = field.strip_prefix("rev=") {
            return format!("revision:{value}");
        }
        if let Some(value) = field.strip_prefix("tag=") {
            return format!("tag:{value}");
        }
    }
    "default_branch".to_owned()
}

fn resolved_git_revisions_from_lock(
    workspace_root: &Path,
    package_name: &str,
    repository: &str,
) -> Result<Vec<String>, ()> {
    let lock_path = workspace_root.join("Cargo.lock");
    let metadata = fs::metadata(&lock_path).map_err(|_| ())?;
    if !metadata.is_file() || metadata.len() > MAX_REVISION_SKEW_LOCK_BYTES {
        return Err(());
    }
    let lock = fs::read_to_string(lock_path).map_err(|_| ())?;
    let mut revisions = Vec::new();
    let mut current_name = None;
    let mut current_source = None;
    for line in lock.lines().chain(std::iter::once("[[package]]")) {
        let line = line.trim();
        if line == "[[package]]" {
            if current_name.as_deref() == Some(package_name)
                && let Some(source) = current_source.as_deref()
                && normalize_git_repository(source) == repository
                && let Some((_, revision)) = source.rsplit_once('#')
                && valid_revision(revision)
            {
                revisions.push(revision.to_owned());
            }
            current_name = None;
            current_source = None;
        } else if let Some(value) = parse_lock_basic_string(line, "name") {
            current_name = Some(value);
        } else if let Some(value) = parse_lock_basic_string(line, "source") {
            current_source = Some(value);
        }
    }
    revisions.sort();
    revisions.dedup();
    Ok(revisions)
}

fn parse_lock_basic_string(line: &str, key: &str) -> Option<String> {
    let value = line.strip_prefix(key)?.trim_start();
    let value = value.strip_prefix('=')?.trim_start();
    let value = value.strip_prefix('"')?.strip_suffix('"')?;
    if value.contains(['\\', '"', '\r', '\n']) {
        return None;
    }
    Some(value.to_owned())
}

fn classify_revision_skew(
    checkout_path: &Path,
    observed_revision: &str,
    resolved_revision: Option<&str>,
    reasons: &mut Vec<String>,
) -> RevisionSkewStatus {
    let Some(head) = git_stdout(checkout_path, &["rev-parse", "HEAD"]) else {
        reasons.push("The explicit producer checkout HEAD could not be observed.".to_owned());
        return RevisionSkewStatus::Unavailable;
    };
    if head != observed_revision {
        reasons.push(
            "The explicit observed producer revision does not equal the local checkout HEAD."
                .to_owned(),
        );
        return RevisionSkewStatus::Unavailable;
    }
    match git_checkout_is_clean(checkout_path) {
        Some(true) => {}
        Some(false) => {
            reasons.push(
                "The explicit producer checkout is dirty, so HEAD does not fully identify the observed source state."
                    .to_owned(),
            );
            return RevisionSkewStatus::Unavailable;
        }
        None => {
            reasons.push(
                "The explicit producer checkout cleanliness could not be observed.".to_owned(),
            );
            return RevisionSkewStatus::Unavailable;
        }
    }
    let Some(resolved_revision) = resolved_revision else {
        return RevisionSkewStatus::Unknown;
    };
    if resolved_revision == observed_revision {
        reasons.push("The locked and observed producer revisions are equal.".to_owned());
        return RevisionSkewStatus::Equal;
    }
    if !git_commit_exists(checkout_path, resolved_revision)
        || !git_commit_exists(checkout_path, observed_revision)
    {
        reasons.push(
            "At least one compared revision is unavailable in the explicit local producer checkout."
                .to_owned(),
        );
        return RevisionSkewStatus::Unavailable;
    }
    match (
        git_is_ancestor(checkout_path, resolved_revision, observed_revision),
        git_is_ancestor(checkout_path, observed_revision, resolved_revision),
    ) {
        (Some(true), Some(false)) => {
            reasons.push(
                "The locked consumer revision is an ancestor of the observed producer revision."
                    .to_owned(),
            );
            RevisionSkewStatus::Behind
        }
        (Some(false), Some(true)) => {
            reasons.push(
                "The observed producer revision is an ancestor of the locked consumer revision."
                    .to_owned(),
            );
            RevisionSkewStatus::Ahead
        }
        (Some(false), Some(false)) => {
            reasons.push("The locked and observed revisions have divergent ancestry.".to_owned());
            RevisionSkewStatus::Divergent
        }
        _ => {
            reasons.push("Git ancestry could not be classified completely.".to_owned());
            RevisionSkewStatus::Unavailable
        }
    }
}

fn git_stdout(checkout_path: &Path, arguments: &[&str]) -> Option<String> {
    let output = run_revision_skew_git(checkout_path, arguments)?;
    if !output.status.success() || !output.capture.stdout.complete {
        return None;
    }
    let value = std::str::from_utf8(&output.capture.stdout.retained)
        .ok()?
        .trim();
    if value.is_empty() || value.contains(char::is_whitespace) {
        return None;
    }
    Some(value.to_owned())
}

fn git_commit_exists(checkout_path: &Path, revision: &str) -> bool {
    let commit = format!("{revision}^{{commit}}");
    run_revision_skew_git(checkout_path, &["cat-file", "-e", &commit])
        .is_some_and(|output| output.status.success())
}

fn git_checkout_is_clean(checkout_path: &Path) -> Option<bool> {
    let output = run_revision_skew_git(
        checkout_path,
        &["status", "--porcelain=v1", "--untracked-files=normal"],
    )?;
    if !output.status.success() || !output.capture.stdout.complete {
        return None;
    }
    Some(output.capture.stdout.retained.is_empty())
}

fn git_is_ancestor(checkout_path: &Path, ancestor: &str, descendant: &str) -> Option<bool> {
    let output = run_revision_skew_git(
        checkout_path,
        &["merge-base", "--is-ancestor", ancestor, descendant],
    )?;
    match output.status.code() {
        Some(0) => Some(true),
        Some(1) => Some(false),
        _ => None,
    }
}

fn run_revision_skew_git(checkout_path: &Path, arguments: &[&str]) -> Option<BoundedOutput> {
    let mut command = Command::new("git");
    command
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("-C")
        .arg(checkout_path)
        .args(arguments);
    run_bounded_command(
        &mut command,
        REVISION_SKEW_GIT_TIMEOUT,
        MAX_REVISION_SKEW_GIT_OUTPUT_BYTES,
    )
    .ok()
}

fn revision_skew_invocation_identity(selection_identity: &str) -> String {
    invocation_identity(&[
        "revision-skew",
        selection_identity,
        "request-schema=ferris.revision-skew-request/v0",
        "result-schema=ferris.revision-skew-report/v0",
        "cargo-metadata-format=1",
        "no-deps=true",
        "offline=true",
        "locked=true",
        "lockfile=bounded-direct-read",
        "git-ancestry=local-read-only",
        "observed-revision=checkout-head",
        "producer-checkout=clean",
        "execution=false",
    ])
}

fn qualify_federated_validation_packages(
    changed_packages: &[String],
    declared_workspace_ids: &BTreeSet<String>,
) -> Result<BTreeMap<String, Vec<String>>, CoreError> {
    let mut packages_by_workspace: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for qualified_package in changed_packages {
        let (workspace_id, package_name) =
            qualified_package.rsplit_once(':').ok_or_else(|| {
                CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-FEDERATED-VALIDATION-PACKAGE-QUALIFIER-INVALID",
                    "A federated changed-package input is not workspace-qualified.",
                    vec![
                        "Pass packages as WORKSPACE_ID:PACKAGE without changing single-workspace package syntax."
                            .to_owned(),
                    ],
                )
            })?;
        if !valid_portable_id(workspace_id)
            || package_name.is_empty()
            || package_name != package_name.trim()
        {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-VALIDATION-PACKAGE-QUALIFIER-INVALID",
                "A federated changed-package input has an invalid workspace or package component.",
                vec!["Pass packages as WORKSPACE_ID:PACKAGE.".to_owned()],
            ));
        }
        let package_name = normalize_validation_package_requests(&[package_name.to_owned()])?
            .pop()
            .expect("one valid package request remains");
        if !declared_workspace_ids.contains(workspace_id) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-FEDERATED-VALIDATION-PACKAGE-WORKSPACE-NOT-FOUND",
                "A federated changed-package qualifier names no declared workspace.",
                vec![
                    "Use a workspace_id declared by the explicit application definition."
                        .to_owned(),
                ],
            ));
        }
        packages_by_workspace
            .entry(workspace_id.to_owned())
            .or_default()
            .push(package_name);
    }
    for packages in packages_by_workspace.values_mut() {
        packages.sort();
        packages.dedup();
    }
    Ok(packages_by_workspace)
}

fn load_application_definition(path: &Path) -> Result<LoadedApplicationDefinition, CoreError> {
    let bytes = read_application_definition(path)?;
    let mut definition: ApplicationDefinition = serde_json::from_slice(&bytes).map_err(|_| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-APPLICATION-INPUT-INVALID",
            "The explicit application definition is not valid strict ferris.application/v0 JSON.",
            vec![
                "Use only schema, application_id, workspaces, workspace_id, manifest_path, and optional depends_on fields."
                    .to_owned(),
            ],
        )
        .with_source_digest(digest_bytes(&bytes))
    })?;
    if definition.schema != APPLICATION_SCHEMA {
        return Err(CoreError::new(
            ResultClass::Unsupported,
            "FERRIS-APPLICATION-SCHEMA-UNSUPPORTED",
            "The explicit application definition schema is unsupported.",
            vec![format!("Use schema {APPLICATION_SCHEMA}.")],
        ));
    }
    validate_application_id(&definition.application_id)?;
    if !(MIN_FEDERATED_PLAN_WORKSPACES..=MAX_FEDERATED_PLAN_WORKSPACES)
        .contains(&definition.workspaces.len())
    {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-APPLICATION-WORKSPACE-COUNT-INVALID",
            format!(
                "A bounded application must declare {MIN_FEDERATED_PLAN_WORKSPACES} to {MAX_FEDERATED_PLAN_WORKSPACES} workspaces."
            ),
            vec!["Declare an explicit bounded set of independent Cargo workspaces.".to_owned()],
        ));
    }
    validate_application_relationships(&definition)?;
    for workspace in &mut definition.workspaces {
        workspace.depends_on.sort();
    }
    definition
        .workspaces
        .sort_by(|left, right| left.workspace_id.cmp(&right.workspace_id));
    let definition_identity = application_definition_identity(&definition)?;

    let application_path = path.canonicalize().map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-APPLICATION-INPUT-UNAVAILABLE",
            "The explicit application definition could not be resolved completely.",
            vec!["Pass a readable local JSON file with --application.".to_owned()],
        )
    })?;
    let application_name = application_path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| {
            CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-INPUT-NAME-INVALID",
                "The explicit application definition file name is not portable UTF-8.",
                vec!["Use a portable UTF-8 JSON file name.".to_owned()],
            )
        })?
        .to_owned();
    let application_root = application_path
        .parent()
        .expect("a canonical file has a parent")
        .to_path_buf();
    let mut manifests = BTreeSet::new();
    let mut workspace_roots = BTreeSet::new();
    let mut loaded_workspaces = Vec::with_capacity(definition.workspaces.len());
    for workspace in &definition.workspaces {
        validate_application_manifest_request(&workspace.manifest_path)
            .map_err(|error| error.with_federated_workspace(&workspace.workspace_id))?;
        let manifest_path =
            canonical_manifest_path(&application_root.join(&workspace.manifest_path))
                .map_err(|error| error.with_federated_workspace(&workspace.workspace_id))?;
        if !manifest_path.starts_with(&application_root) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-MANIFEST-OUTSIDE-ROOT",
                "A declared workspace manifest resolves outside the application definition directory.",
                vec!["Use a contained application-relative Cargo.toml path.".to_owned()],
            )
            .with_federated_workspace(&workspace.workspace_id));
        }
        if !manifests.insert(manifest_path.clone()) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-MANIFEST-DUPLICATE",
                "The application definition selects the same resolved manifest more than once.",
                vec!["Select each independent Cargo workspace manifest once.".to_owned()],
            )
            .with_federated_workspace(&workspace.workspace_id));
        }

        let invocation = load_bounded_federated_cargo_metadata(&manifest_path, Path::new("cargo"))
            .map_err(|error| error.with_federated_workspace(&workspace.workspace_id))?;
        let metadata = decode_metadata(&invocation.bytes)
            .map_err(|error| error.with_federated_workspace(&workspace.workspace_id))?;
        let workspace_root = canonical_federated_workspace_root(&metadata)
            .map_err(|error| error.with_federated_workspace(&workspace.workspace_id))?;
        if !workspace_root.starts_with(&application_root) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-WORKSPACE-ROOT-OUTSIDE-APPLICATION",
                "Cargo reported a workspace root outside the application definition directory.",
                vec![
                    "Place the application definition at a common ancestor of every complete Cargo workspace."
                        .to_owned(),
                ],
            )
            .with_federated_workspace(&workspace.workspace_id));
        }
        if !manifest_path.starts_with(&workspace_root) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-MANIFEST-OUTSIDE-WORKSPACE",
                "The selected manifest is outside Cargo's reported workspace root.",
                vec!["Select a manifest owned by the declared Cargo workspace.".to_owned()],
            )
            .with_federated_workspace(&workspace.workspace_id));
        }
        if !workspace_roots.insert(workspace_root.clone()) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-WORKSPACE-ROOT-DUPLICATE",
                "The application definition selects the same Cargo workspace more than once.",
                vec![
                    "Select exactly one manifest for each independent Cargo workspace.".to_owned(),
                ],
            )
            .with_federated_workspace(&workspace.workspace_id));
        }
        loaded_workspaces.push(LoadedApplicationWorkspace {
            definition: workspace.clone(),
            manifest_path: invocation.manifest_path,
            workspace_root,
            metadata,
            metadata_bytes: invocation.bytes,
        });
    }

    for left in 0..loaded_workspaces.len() {
        for right in (left + 1)..loaded_workspaces.len() {
            let left_root = &loaded_workspaces[left].workspace_root;
            let right_root = &loaded_workspaces[right].workspace_root;
            if left_root.starts_with(right_root) || right_root.starts_with(left_root) {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-APPLICATION-WORKSPACE-ROOT-NESTED",
                    "Application Cargo workspace roots must not be nested.",
                    vec![
                        "Declare independent, non-nested Cargo workspace roots so path ownership is unambiguous."
                            .to_owned(),
                    ],
                ));
            }
        }
    }

    Ok(LoadedApplicationDefinition {
        definition,
        definition_identity,
        application_name,
        application_root,
        workspaces: loaded_workspaces,
    })
}

fn read_application_definition(path: &Path) -> Result<Vec<u8>, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-APPLICATION-INPUT-UNAVAILABLE",
            "The explicit application definition is missing or unreadable.",
            vec!["Pass a readable local JSON file with --application.".to_owned()],
        )
    })?;
    if !metadata.is_file() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-APPLICATION-INPUT-NOT-FILE",
            "The explicit application definition is not a regular file.",
            vec!["Pass a readable local JSON file with --application.".to_owned()],
        ));
    }
    if metadata.len() > MAX_APPLICATION_INPUT_BYTES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-APPLICATION-INPUT-OVERSIZED",
            format!(
                "The explicit application definition exceeds the {MAX_APPLICATION_INPUT_BYTES}-byte bound."
            ),
            vec!["Reduce the application definition below the documented bound.".to_owned()],
        ));
    }
    let file = fs::File::open(path).map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-APPLICATION-INPUT-UNAVAILABLE",
            "The explicit application definition is missing or unreadable.",
            vec!["Pass a readable local JSON file with --application.".to_owned()],
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_APPLICATION_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            CoreError::new(
                ResultClass::Incomplete,
                "FERRIS-APPLICATION-INPUT-UNAVAILABLE",
                "The explicit application definition could not be read completely.",
                vec!["Pass a readable local JSON file with --application.".to_owned()],
            )
        })?;
    if bytes.len() as u64 > MAX_APPLICATION_INPUT_BYTES {
        return Err(CoreError::new(
            ResultClass::Blocked,
            "FERRIS-APPLICATION-INPUT-OVERSIZED",
            format!(
                "The explicit application definition exceeds the {MAX_APPLICATION_INPUT_BYTES}-byte bound."
            ),
            vec!["Reduce the application definition below the documented bound.".to_owned()],
        ));
    }
    Ok(bytes)
}

fn validate_application_manifest_request(value: &str) -> Result<(), CoreError> {
    let components = value.split('/').collect::<Vec<_>>();
    if value.is_empty()
        || value.len() > 1024
        || value.contains(['\0', '\\'])
        || is_absolute_manifest_request(value)
        || components
            .iter()
            .any(|component| component.is_empty() || matches!(*component, "." | ".."))
        || components.last().copied() != Some("Cargo.toml")
    {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-APPLICATION-MANIFEST-PATH-INVALID",
            "A workspace manifest_path must be a bounded portable application-relative path ending in Cargo.toml without empty, current, or parent components.",
            vec!["Use a contained forward-slash path such as workspace/Cargo.toml.".to_owned()],
        ));
    }
    Ok(())
}

fn validate_application_relationships(definition: &ApplicationDefinition) -> Result<(), CoreError> {
    let mut workspace_ids = BTreeSet::new();
    for workspace in &definition.workspaces {
        validate_workspace_id(&workspace.workspace_id)?;
        if !workspace_ids.insert(workspace.workspace_id.clone()) {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-APPLICATION-WORKSPACE-ID-DUPLICATE",
                "The application definition declares a duplicate workspace_id.",
                vec!["Give every declared workspace a unique portable workspace_id.".to_owned()],
            ));
        }
    }

    let mut dependency_counts = BTreeMap::new();
    let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for workspace in &definition.workspaces {
        let mut dependencies = BTreeSet::new();
        for dependency in &workspace.depends_on {
            if dependency == &workspace.workspace_id {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-APPLICATION-DEPENDENCY-SELF",
                    "A workspace depends_on list contains its own workspace_id.",
                    vec!["Declare only other application workspaces as dependencies.".to_owned()],
                )
                .with_federated_workspace(&workspace.workspace_id));
            }
            if !dependencies.insert(dependency.clone()) {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-APPLICATION-DEPENDENCY-DUPLICATE",
                    "A workspace depends_on list contains a duplicate reference.",
                    vec!["Declare each dependency workspace_id at most once.".to_owned()],
                )
                .with_federated_workspace(&workspace.workspace_id));
            }
            if !workspace_ids.contains(dependency) {
                return Err(CoreError::new(
                    ResultClass::Invalid,
                    "FERRIS-APPLICATION-DEPENDENCY-NOT-FOUND",
                    "A workspace depends_on reference names no declared workspace.",
                    vec!["Reference only workspace_id values in the same application.".to_owned()],
                )
                .with_federated_workspace(&workspace.workspace_id));
            }
            dependents
                .entry(dependency.clone())
                .or_default()
                .push(workspace.workspace_id.clone());
        }
        dependency_counts.insert(workspace.workspace_id.clone(), dependencies.len());
    }

    let mut ready = dependency_counts
        .iter()
        .filter_map(|(workspace_id, count)| (*count == 0).then_some(workspace_id.clone()))
        .collect::<BTreeSet<_>>();
    let mut visited = 0;
    while let Some(workspace_id) = ready.pop_first() {
        visited += 1;
        if let Some(workspace_dependents) = dependents.get(&workspace_id) {
            for dependent in workspace_dependents {
                let count = dependency_counts
                    .get_mut(dependent)
                    .expect("declared dependent has a dependency count");
                *count -= 1;
                if *count == 0 {
                    ready.insert(dependent.clone());
                }
            }
        }
    }
    if visited != definition.workspaces.len() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-APPLICATION-DEPENDENCY-CYCLE",
            "The application depends_on relationships contain a cycle.",
            vec!["Use an acyclic explicit application relationship graph.".to_owned()],
        ));
    }
    Ok(())
}

fn portable_relative_path(path: &Path) -> String {
    let normalized = normalize_path_text(&path.to_string_lossy());
    if normalized.is_empty() {
        ".".to_owned()
    } else {
        normalized
    }
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
        .with_bounded_output(bounded_output_evidence(&output.capture, "completed"))
        .with_invocation_selection(manifest_digest.clone()));
    }

    let cargo_evidence = decode_cargo_probe(
        &output.capture.stdout.retained,
        &output.capture.stderr.retained,
    )
    .map_err(|error| {
        error
            .with_bounded_output(bounded_output_evidence(&output.capture, "completed"))
            .with_invocation_selection(manifest_digest.clone())
    })?;
    let owner_output_digest = digest_command_output(
        &output.capture.stdout.retained,
        &output.capture.stderr.retained,
    );
    let mut record = DoctorReport {
        schema: DOCTOR_SCHEMA.to_owned(),
        report_id: String::new(),
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
            stdout_retained_bytes: output.capture.stdout.retained.len() as u64,
            stdout_observed_bytes: output.capture.stdout.observed_bytes,
            stdout_omitted_observed_bytes: output
                .capture
                .stdout
                .observed_bytes
                .saturating_sub(output.capture.stdout.retained.len() as u64),
            stdout_unobserved_bytes_unknown: !output.capture.stdout.complete,
            stdout_complete: output.capture.stdout.complete,
            stdout_truncated: output.capture.stdout.truncated,
            stderr_retained_bytes: output.capture.stderr.retained.len() as u64,
            stderr_observed_bytes: output.capture.stderr.observed_bytes,
            stderr_omitted_observed_bytes: output
                .capture
                .stderr
                .observed_bytes
                .saturating_sub(output.capture.stderr.retained.len() as u64),
            stderr_unobserved_bytes_unknown: !output.capture.stderr.complete,
            stderr_complete: output.capture.stderr.complete,
            stderr_truncated: output.capture.stderr.truncated,
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
            owner_output_framing: "length-prefixed-stdout-stderr/v1".to_owned(),
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
    record.report_id = doctor_record_id(&record)?;
    let selection_identity = doctor_selection_identity(workspace_id, &manifest_digest);
    let invocation_identity = doctor_invocation_identity(workspace_id, &selection_identity);

    Ok(success_envelope(
        "doctor",
        selection_identity,
        invocation_identity,
        record,
    ))
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

fn load_bounded_federated_cargo_metadata(
    manifest_path: &Path,
    cargo_program: &Path,
) -> Result<MetadataInvocation, CoreError> {
    let manifest_path = canonical_manifest_path(manifest_path)?;
    let working_directory = manifest_path.parent().ok_or_else(|| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-PARENT-INVALID",
            "The explicit manifest has no selectable parent directory.",
            vec!["Select a Cargo.toml within a local directory.".to_owned()],
        )
    })?;
    let mut command = Command::new(cargo_program);
    configure_owner_toolchain_guards(&mut command, working_directory);
    command
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
            "--locked",
            "--manifest-path",
        ])
        .arg(&manifest_path);
    let output = run_bounded_command(
        &mut command,
        FEDERATED_PLAN_METADATA_TIMEOUT,
        MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES,
    )
    .map_err(federated_metadata_command_error)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.capture.stderr.retained)
            .trim()
            .to_owned();
        let (class, code) = classify_cargo_failure(&stderr);
        let message = if stderr.is_empty() {
            "Cargo metadata failed without a diagnostic."
        } else if class == ResultClass::Invalid {
            "Cargo rejected the selected manifest or workspace metadata request."
        } else {
            "Cargo metadata was blocked by offline, locked, or source availability requirements."
        };
        return Err(CoreError::new(
            class,
            code,
            message,
            vec![
                "Run cargo metadata with the same manifest and offline flags.".to_owned(),
                "Repair the owner manifest or make required offline sources available.".to_owned(),
            ],
        )
        .with_bounded_output(bounded_output_evidence(&output.capture, "completed")));
    }

    Ok(MetadataInvocation {
        manifest_path,
        bytes: output.capture.stdout.retained,
    })
}

fn load_bounded_revision_skew_cargo_metadata(
    manifest_path: &Path,
    cargo_program: &Path,
) -> Result<MetadataInvocation, CoreError> {
    let manifest_path = canonical_manifest_path(manifest_path)?;
    let working_directory = manifest_path.parent().ok_or_else(|| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-MANIFEST-PARENT-INVALID",
            "The explicit manifest has no selectable parent directory.",
            vec!["Select a Cargo.toml within a local directory.".to_owned()],
        )
    })?;
    let mut command = Command::new(cargo_program);
    configure_owner_toolchain_guards(&mut command, working_directory);
    command
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
            "--locked",
            "--manifest-path",
        ])
        .arg(&manifest_path);
    let output = run_bounded_command(
        &mut command,
        FEDERATED_PLAN_METADATA_TIMEOUT,
        MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES,
    )
    .map_err(federated_metadata_command_error)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.capture.stderr.retained)
            .trim()
            .to_owned();
        let (class, code) = classify_cargo_failure(&stderr);
        return Err(CoreError::new(
            class,
            code,
            if stderr.is_empty() {
                "Cargo metadata failed without a diagnostic."
            } else if class == ResultClass::Invalid {
                "Cargo rejected the selected revision-skew manifest or lock request."
            } else {
                "Cargo revision-skew metadata was blocked by offline, locked, or source availability requirements."
            },
            vec![
                "Run cargo metadata --format-version 1 --offline --locked with the same manifest."
                    .to_owned(),
                "Repair the owner manifest, lockfile, or offline source availability.".to_owned(),
            ],
        )
        .with_bounded_output(bounded_output_evidence(&output.capture, "completed")));
    }

    Ok(MetadataInvocation {
        manifest_path,
        bytes: output.capture.stdout.retained,
    })
}

fn federated_metadata_command_error(error: BoundedCommandError) -> CoreError {
    match error {
        BoundedCommandError::Start(source) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-CARGO-METADATA-UNAVAILABLE",
            "The bounded Cargo metadata command could not start.",
            vec!["Install Cargo or make it available on PATH.".to_owned()],
        )
        .with_source_digest(digest_text(&source.to_string())),
        BoundedCommandError::Wait(source) => CoreError::new(
            ResultClass::Internal,
            "FERRIS-CARGO-METADATA-WAIT-FAILED",
            "Ferris could not observe completion of the bounded Cargo metadata command.",
            vec!["Report this Ferris process-control failure.".to_owned()],
        )
        .with_source_digest(digest_text(&source.to_string())),
        BoundedCommandError::Read => CoreError::new(
            ResultClass::Internal,
            "FERRIS-CARGO-METADATA-OUTPUT-FAILED",
            "Ferris could not retain bounded Cargo metadata output.",
            vec!["Report this Ferris process-output failure.".to_owned()],
        ),
        BoundedCommandError::ReadCapture(capture) => CoreError::new(
            ResultClass::Internal,
            "FERRIS-CARGO-METADATA-OUTPUT-FAILED",
            "Ferris could not retain bounded Cargo metadata output.",
            vec!["Report this Ferris process-output failure.".to_owned()],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "read-failed")),
        BoundedCommandError::Timeout(capture) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-CARGO-METADATA-TIMEOUT",
            "Cargo metadata exceeded the federated per-workspace time bound.",
            vec![format!(
                "Run cargo metadata directly; Ferris stopped waiting after {} seconds.",
                FEDERATED_PLAN_METADATA_TIMEOUT.as_secs()
            )],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "timeout")),
        BoundedCommandError::OutputLimit(capture) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-CARGO-METADATA-OUTPUT-BOUND-EXCEEDED",
            "Cargo metadata exceeded the federated per-workspace output bound.",
            vec![format!(
                "Run cargo metadata directly; Ferris retains at most {MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES} bytes per stream."
            )],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "output-bound")),
    }
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
        if !matches!(commit.len(), 9 | 40)
            || !commit
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
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
    let (Some(year), Some(month), Some(day)) = (year, month, day) else {
        return false;
    };
    if year < 2010 || !(1..=12).contains(&month) {
        return false;
    }
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let maximum_day = match month {
        2 if leap_year => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    (1..=maximum_day).contains(&day)
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
        )
        .with_invocation_selection(format!(
            "oversized-manifest-prefix:{}:retained-bytes={}",
            digest_bytes(&bytes),
            bytes.len()
        )));
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
        BoundedCommandError::ReadCapture(capture) => CoreError::new(
            ResultClass::Internal,
            "FERRIS-DOCTOR-CARGO-OUTPUT-FAILED",
            "Ferris could not retain bounded Cargo version output.",
            vec!["Report this Ferris process-output failure.".to_owned()],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "read-failed")),
        BoundedCommandError::Timeout(capture) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-TIMEOUT",
            "The passive Cargo version probe exceeded its time bound.",
            vec![format!(
                "Run cargo --version directly; Ferris stopped waiting after {} seconds.",
                DOCTOR_TIMEOUT.as_secs()
            )],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "timeout")),
        BoundedCommandError::OutputLimit(capture) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-DOCTOR-CARGO-OUTPUT-BOUND-EXCEEDED",
            "The passive Cargo version probe exceeded its output bound.",
            vec![format!(
                "Run cargo --version directly; Ferris retains at most {MAX_DOCTOR_OUTPUT_BYTES} bytes per stream."
            )],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "output-bound")),
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
    let stdout_state = spawn_bounded_reader(stdout, output_limit, Arc::clone(&exceeded));
    let stderr_state = spawn_bounded_reader(stderr, output_limit, Arc::clone(&exceeded));
    let started = Instant::now();

    let status = loop {
        if exceeded.load(Ordering::Relaxed) {
            return Err(BoundedCommandError::OutputLimit(terminate_and_capture(
                &mut child,
                &stdout_state,
                &stderr_state,
            )?));
        }
        if started.elapsed() >= timeout {
            return Err(BoundedCommandError::Timeout(terminate_and_capture(
                &mut child,
                &stdout_state,
                &stderr_state,
            )?));
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

    loop {
        let capture = bounded_capture(&stdout_state, &stderr_state)?;
        if capture.stdout.failed || capture.stderr.failed {
            return Err(BoundedCommandError::ReadCapture(capture));
        }
        if exceeded.load(Ordering::Relaxed) {
            return Err(BoundedCommandError::OutputLimit(capture));
        }
        if stream_settled(&capture.stdout) && stream_settled(&capture.stderr) {
            return Ok(BoundedOutput {
                status,
                capture: BoundedCapture {
                    termination_cleanup_complete: true,
                    ..capture
                },
            });
        }
        if started.elapsed() >= timeout {
            return Err(BoundedCommandError::Timeout(terminate_and_capture(
                &mut child,
                &stdout_state,
                &stderr_state,
            )?));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn spawn_bounded_reader(
    mut reader: impl Read + Send + 'static,
    output_limit: usize,
    exceeded: Arc<AtomicBool>,
) -> Arc<Mutex<CapturedStream>> {
    let state = Arc::new(Mutex::new(CapturedStream {
        retained: Vec::new(),
        observed_bytes: 0,
        complete: false,
        truncated: false,
        failed: false,
    }));
    let reader_state = Arc::clone(&state);
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        loop {
            let remaining = reader_state
                .lock()
                .map(|state| output_limit.saturating_sub(state.retained.len()))
                .unwrap_or(0);
            let read_limit = buffer.len().min(remaining.saturating_add(1));
            match reader.read(&mut buffer[..read_limit]) {
                Ok(0) => {
                    if let Ok(mut state) = reader_state.lock() {
                        state.complete = true;
                    }
                    break;
                }
                Ok(count) => {
                    let Ok(mut state) = reader_state.lock() else {
                        break;
                    };
                    state.observed_bytes = state.observed_bytes.saturating_add(count as u64);
                    let remaining = output_limit.saturating_sub(state.retained.len());
                    state
                        .retained
                        .extend_from_slice(&buffer[..count.min(remaining)]);
                    if count > remaining {
                        state.truncated = true;
                        exceeded.store(true, Ordering::Relaxed);
                        break;
                    }
                }
                Err(_) => {
                    if let Ok(mut state) = reader_state.lock() {
                        state.failed = true;
                    }
                    break;
                }
            }
        }
    });
    state
}

fn bounded_capture(
    stdout: &Arc<Mutex<CapturedStream>>,
    stderr: &Arc<Mutex<CapturedStream>>,
) -> Result<BoundedCapture, BoundedCommandError> {
    let stdout = stdout
        .lock()
        .map_err(|_| BoundedCommandError::Read)?
        .clone();
    let stderr = stderr
        .lock()
        .map_err(|_| BoundedCommandError::Read)?
        .clone();
    Ok(BoundedCapture {
        stdout,
        stderr,
        termination_cleanup_complete: false,
    })
}

fn terminate_and_capture(
    child: &mut std::process::Child,
    stdout: &Arc<Mutex<CapturedStream>>,
    stderr: &Arc<Mutex<CapturedStream>>,
) -> Result<BoundedCapture, BoundedCommandError> {
    let _ = child.kill();
    let deadline = Instant::now() + Duration::from_secs(1);
    let mut child_exited = false;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => child_exited = true,
            Ok(None) => {}
            Err(_) => {}
        }
        let mut capture = bounded_capture(stdout, stderr)?;
        let streams_settled = stream_settled(&capture.stdout) && stream_settled(&capture.stderr);
        if child_exited && streams_settled {
            capture.termination_cleanup_complete = true;
            return Ok(capture);
        }
        if Instant::now() >= deadline {
            capture.termination_cleanup_complete = false;
            return Ok(capture);
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn stream_settled(stream: &CapturedStream) -> bool {
    stream.complete || stream.truncated
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

pub fn locate_workspace_manifest(
    current_directory: &Path,
    workspace_id: &str,
) -> Result<PathBuf, CoreError> {
    validate_workspace_id(workspace_id)?;
    locate_workspace_manifest_with_cargo(current_directory, Path::new("cargo"))
}

fn locate_workspace_manifest_with_cargo(
    current_directory: &Path,
    cargo_program: &Path,
) -> Result<PathBuf, CoreError> {
    if !current_directory.is_dir() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-WORKSPACE-DISCOVERY-DIRECTORY-INVALID",
            "The current directory is not available for Cargo workspace discovery.",
            vec!["Run cargo ferris from a readable Cargo workspace directory.".to_owned()],
        ));
    }

    let mut command = Command::new(cargo_program);
    configure_owner_toolchain_guards(&mut command, current_directory);
    command.args(["locate-project", "--workspace", "--message-format", "json"]);
    let output = run_bounded_command(
        &mut command,
        WORKSPACE_DISCOVERY_TIMEOUT,
        MAX_WORKSPACE_DISCOVERY_OUTPUT_BYTES,
    )
    .map_err(workspace_discovery_command_error)?;

    if !output.status.success() {
        return Err(CoreError::new(
            ResultClass::Invalid,
            "FERRIS-WORKSPACE-DISCOVERY-NOT-FOUND",
            "Cargo could not identify a current workspace manifest.",
            vec![
                "Run cargo ferris from a Cargo workspace or pass --manifest-path explicitly."
                    .to_owned(),
                "Run cargo locate-project --workspace --message-format json directly to inspect Cargo's result."
                    .to_owned(),
            ],
        )
        .with_bounded_output(bounded_output_evidence(&output.capture, "completed")));
    }

    let located: CargoLocateProject =
        serde_json::from_slice(&output.capture.stdout.retained).map_err(|_| {
            CoreError::new(
                ResultClass::Unsupported,
                "FERRIS-WORKSPACE-DISCOVERY-OUTPUT-UNSUPPORTED",
                "Cargo returned a successful workspace discovery response that Ferris could not safely parse.",
                vec![
                    "Pass --manifest-path explicitly.".to_owned(),
                    "Use a Cargo release that supports JSON locate-project output.".to_owned(),
                ],
            )
            .with_source_digest(digest_command_output(
                &output.capture.stdout.retained,
                &output.capture.stderr.retained,
            ))
        })?;
    canonical_manifest_path(&located.root)
}

fn workspace_discovery_command_error(error: BoundedCommandError) -> CoreError {
    match error {
        BoundedCommandError::Start(source) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-WORKSPACE-DISCOVERY-CARGO-UNAVAILABLE",
            "Cargo workspace discovery could not start.",
            vec![
                "Install Cargo or make it available on PATH.".to_owned(),
                "Pass --manifest-path explicitly to bypass discovery.".to_owned(),
            ],
        )
        .with_source_digest(digest_text(&source.to_string())),
        BoundedCommandError::Wait(source) => CoreError::new(
            ResultClass::Internal,
            "FERRIS-WORKSPACE-DISCOVERY-WAIT-FAILED",
            "Ferris could not observe completion of Cargo workspace discovery.",
            vec!["Report this Ferris process-control failure.".to_owned()],
        )
        .with_source_digest(digest_text(&source.to_string())),
        BoundedCommandError::Read => CoreError::new(
            ResultClass::Internal,
            "FERRIS-WORKSPACE-DISCOVERY-OUTPUT-FAILED",
            "Ferris could not retain bounded Cargo workspace discovery output.",
            vec!["Report this Ferris process-output failure.".to_owned()],
        ),
        BoundedCommandError::ReadCapture(capture) => CoreError::new(
            ResultClass::Internal,
            "FERRIS-WORKSPACE-DISCOVERY-OUTPUT-FAILED",
            "Ferris could not retain bounded Cargo workspace discovery output.",
            vec!["Report this Ferris process-output failure.".to_owned()],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "read-failed")),
        BoundedCommandError::Timeout(capture) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-WORKSPACE-DISCOVERY-TIMEOUT",
            "Cargo workspace discovery exceeded its time bound.",
            vec![
                "Pass --manifest-path explicitly or run cargo locate-project directly.".to_owned(),
            ],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "timeout")),
        BoundedCommandError::OutputLimit(capture) => CoreError::new(
            ResultClass::Blocked,
            "FERRIS-WORKSPACE-DISCOVERY-OUTPUT-BOUND-EXCEEDED",
            "Cargo workspace discovery exceeded its output bound.",
            vec![
                "Pass --manifest-path explicitly or run cargo locate-project directly.".to_owned(),
            ],
        )
        .with_bounded_output(bounded_output_evidence(&capture, "output-bound")),
    }
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
        selection_identity(workspace_id, &plan_record.selected_manifest),
        invocation_identity_for_selection("explain", workspace_id, &plan_record.selected_manifest),
        explanation,
    ))
}

pub fn error_envelope<T>(
    semantic_command_id: &str,
    workspace_id: &str,
    manifest_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    command_envelope(
        semantic_command_id,
        request_selection_identity(workspace_id, manifest_path),
        invocation_identity_for_request(semantic_command_id, workspace_id, manifest_path),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn federated_plan_error_envelope<T>(
    request_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let selection_identity = error
        .invocation_selection()
        .filter(|selection| selection.starts_with("selection:"))
        .map(str::to_owned)
        .unwrap_or_else(|| {
            selection_identity(
                "federated-plan-request",
                &digest_text(&request_path_identity_material(request_path)),
            )
        });
    command_envelope(
        "federated-plan",
        selection_identity.clone(),
        federated_plan_invocation_identity(&selection_identity),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn federated_validation_plan_error_envelope<T>(
    application_path: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let provisional_selection = federated_validation_plan_provisional_request_selection_identity(
        changed_paths,
        changed_packages,
    );
    let selection_identity = error
        .invocation_selection()
        .filter(|selection| selection.starts_with("selection:"))
        .filter(|selection| *selection != provisional_selection)
        .map(str::to_owned)
        .or_else(|| {
            try_federated_validation_plan_semantic_request_selection_identity(
                application_path,
                changed_paths,
                changed_packages,
            )
        })
        .unwrap_or(provisional_selection);
    command_envelope(
        "federated-validation-plan",
        selection_identity.clone(),
        federated_validation_plan_invocation_identity(&selection_identity),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn revision_skew_error_envelope<T>(request_path: &Path, error: &CoreError) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let selection_identity = error
        .invocation_selection()
        .filter(|selection| selection.starts_with("selection:"))
        .map(str::to_owned)
        .unwrap_or_else(|| {
            selection_identity(
                "revision-skew-request",
                &digest_text(&request_path_identity_material(request_path)),
            )
        });
    command_envelope(
        "revision-skew",
        selection_identity.clone(),
        revision_skew_invocation_identity(&selection_identity),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn doctor_error_envelope<T>(
    workspace_id: &str,
    manifest_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let selection_identity = error
        .invocation_selection()
        .map(|selection| doctor_selection_identity(workspace_id, selection))
        .unwrap_or_else(|| request_selection_identity(workspace_id, manifest_path));
    command_envelope(
        "doctor",
        selection_identity.clone(),
        doctor_invocation_identity(workspace_id, &selection_identity),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn validation_plan_error_envelope<T>(
    workspace_id: &str,
    manifest_path: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let selection_identity = error
        .invocation_selection()
        .filter(|selection| selection.starts_with("selection:"))
        .map(str::to_owned)
        .unwrap_or_else(|| {
            validation_plan_request_selection_identity(
                workspace_id,
                manifest_path,
                changed_paths,
                changed_packages,
            )
        });
    command_envelope(
        "validation-plan",
        selection_identity.clone(),
        validation_plan_invocation_identity(&selection_identity),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn profile_diff_error_envelope<T>(
    before_path: &Path,
    after_path: &Path,
    error: &CoreError,
) -> CommandEnvelope<T>
where
    T: Serialize,
{
    let selection_identity = error
        .invocation_selection()
        .map(|selection| {
            if selection.starts_with("selection:") {
                selection.to_owned()
            } else {
                invocation_identity(&["profile-diff-selection", selection]).replacen(
                    "invocation:",
                    "selection:",
                    1,
                )
            }
        })
        .unwrap_or_else(|| {
            invocation_identity(&[
                "profile-diff-selection",
                &profile_diff_request_material(before_path, after_path),
            ])
            .replacen("invocation:", "selection:", 1)
        });
    command_envelope(
        "profile-diff",
        selection_identity.clone(),
        profile_diff_invocation_identity(&selection_identity),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
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

pub fn render_federated_plan_human(envelope: &CommandEnvelope<FederatedPlanRecord>) -> String {
    let record = envelope
        .record
        .as_ref()
        .expect("success federated plan has a record");
    let mut output = format!(
        "Ferris federated plan {}\nApplication ID: {}\nRevision: {}\nOwner: {}\nExecutable: no\nWorkspaces:\n",
        record.federated_plan_id, record.application_id, record.revision, record.owner
    );
    for workspace in &record.workspaces {
        let package_count = workspace
            .plan
            .packages
            .iter()
            .filter(|package| package.workspace_member)
            .count();
        output.push_str(&format!("  - {}\n", workspace.workspace_id));
        output.push_str(&format!("    Plan ID: {}\n", workspace.plan.plan_id));
        output.push_str(&format!("    Package count: {package_count}\n"));
    }
    output.push_str("Unknowns:\n");
    for unknown in &record.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &record.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

pub fn render_federated_validation_plan_human(
    envelope: &CommandEnvelope<FederatedValidationPlanRecord>,
) -> String {
    let record = envelope
        .record
        .as_ref()
        .expect("success federated validation plan has a record");
    let mut output = format!(
        "Ferris federated validation plan {}\nApplication ID: {}\nApplication definition: {}\nExecutable: no\nWorkspaces:\n",
        record.federated_validation_plan_id, record.application_id, record.application_definition
    );
    for workspace in &record.workspaces {
        output.push_str(&format!(
            "  - {} ({}) -> {}\n",
            workspace.workspace_id,
            workspace.manifest_path,
            federated_validation_workspace_disposition_name(workspace.disposition)
        ));
        for reason in &workspace.reasons {
            output.push_str(&format!("    reason: {reason}\n"));
        }
        if let Some(plan) = &workspace.validation_plan {
            output.push_str(&format!(
                "    direct plan: {} (selected packages: {}, input fallback: {})\n",
                plan.validation_plan_id,
                plan.selected_packages.len(),
                plan.fallback.required_by_inputs
            ));
        }
    }
    output.push_str(&format!(
        "Application fallback: {} (required by inputs: {})\n",
        record.fallback.boundary, record.fallback.required_by_inputs
    ));
    output.push_str(&format!(
        "Fallback workspaces: {}\n",
        record.fallback.workspace_ids.join(", ")
    ));
    for reason in &record.fallback.reasons {
        output.push_str(&format!("  - {reason}\n"));
    }
    output.push_str("Unknowns:\n");
    for unknown in &record.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &record.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output.push_str(
        "Next: run each workspace owner's ordinary validation directly; Ferris does not execute this record.\n",
    );
    output
}

pub fn render_revision_skew_human(envelope: &CommandEnvelope<RevisionSkewReportRecord>) -> String {
    let record = envelope
        .record
        .as_ref()
        .expect("successful revision-skew report has a record");
    let mut output = format!(
        "Ferris revision-skew report {}\nAnalysis ID: {}\nExecutable: no\nDependencies:\n",
        record.report_id, record.analysis_id
    );
    for dependency in &record.dependencies {
        output.push_str(&format!(
            "  - {} -> {}:{} = {}\n",
            dependency.consumer_id,
            dependency.producer_id,
            dependency.package_name,
            revision_skew_status_name(dependency.status)
        ));
        output.push_str(&format!(
            "    declaration: {}",
            revision_declaration_kind_name(dependency.declaration.kind)
        ));
        if !dependency.declaration.sources.is_empty() {
            output.push_str(&format!(" ({})", dependency.declaration.sources.join(", ")));
        }
        output.push('\n');
        output.push_str(&format!(
            "    resolved: {}\n",
            dependency.resolved_revision.as_deref().unwrap_or("unknown")
        ));
        output.push_str(&format!("    observed: {}\n", dependency.observed_revision));
        for reason in &dependency.reasons {
            output.push_str(&format!("    reason: {reason}\n"));
        }
    }
    output.push_str("Unknowns:\n");
    for unknown in &record.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &record.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output.push_str(
        "Next: let repository owners decide compatibility, migration, validation, and revision updates.\n",
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

pub fn render_validation_plan_human(envelope: &CommandEnvelope<ValidationPlanRecord>) -> String {
    let record = envelope
        .record
        .as_ref()
        .expect("success validation plan has a record");
    let mut package_names = BTreeMap::new();
    for package in record
        .selected_packages
        .iter()
        .map(|selection| &selection.package)
        .chain(record.fallback.packages.iter())
    {
        package_names.insert(
            package.identity.clone(),
            format!("{} {}", package.name, package.version),
        );
    }

    let mut output = format!(
        "Ferris validation plan {}\nWorkspace ID: {}\nWorkspace: {}\nSelected manifest: {}\nExecutable: no\nInputs:\n",
        record.validation_plan_id,
        record.workspace_id,
        record.workspace_root,
        record.selected_manifest
    );
    for input in &record.inputs {
        let package = input
            .package_identity
            .as_ref()
            .and_then(|identity| package_names.get(identity))
            .map(|display| format!(" ({display})"))
            .unwrap_or_default();
        output.push_str(&format!(
            "  - {} {} -> {}{}: {}\n",
            validation_input_kind_name(input.kind),
            input.value,
            validation_input_disposition_name(input.disposition),
            package,
            input.reason
        ));
    }

    output.push_str("Selected packages:\n");
    if record.selected_packages.is_empty() {
        output.push_str(
            "  - none; every explicit input widened immediately to the full workspace fallback\n",
        );
    } else {
        for selection in &record.selected_packages {
            output.push_str(&format!(
                "  - {} {} ({})\n",
                selection.package.name,
                selection.package.version,
                validation_package_disposition_name(selection.disposition)
            ));
            for reason in &selection.reasons {
                output.push_str(&format!("    reason: {reason}\n"));
            }
        }
    }

    output.push_str("Selected validation:\n");
    if record.selected_activities.is_empty() {
        output.push_str(
            "  - no package-specific Cargo activity plan was retained because the request widens directly to the full workspace fallback\n",
        );
    } else {
        for activity in &record.selected_activities {
            output.push_str(&format!(
                "  - cargo {} over {} [{}]\n",
                validation_activity_family_name(activity.family),
                validation_activity_packages(activity, &package_names),
                validation_activity_scope_name(activity.package_scope)
            ));
            output.push_str(&format!("    reason: {}\n", activity.reason));
        }
    }

    output.push_str(&format!(
        "Fallback validation: {} (required by inputs: {})\n",
        record.fallback.boundary, record.fallback.required_by_inputs
    ));
    for reason in &record.fallback.reasons {
        output.push_str(&format!("  - {reason}\n"));
    }
    output.push_str("Fallback packages:\n");
    for package in &record.fallback.packages {
        output.push_str(&format!("  - {} {}\n", package.name, package.version));
    }
    output.push_str("Fallback activities:\n");
    for activity in &record.fallback.activities {
        output.push_str(&format!(
            "  - cargo {} over {} [{}]\n",
            validation_activity_family_name(activity.family),
            validation_activity_packages(activity, &package_names),
            validation_activity_scope_name(activity.package_scope)
        ));
        output.push_str(&format!("    reason: {}\n", activity.reason));
    }
    output.push_str("Unknowns:\n");
    for unknown in &record.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &record.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output.push_str(&format!(
        "Evidence: owner={}, representation={}, working-directory={}, workspace-id={}, metadata-format={}, offline={}, rustup-auto-install={}, toolchain={}, output-digest={}\nCommand: {}\nNext: choose between the selected package closure and the full workspace fallback, then run ordinary Cargo validation directly.\n",
        record.evidence.owner,
        record.evidence.command_representation,
        record.evidence.working_directory,
        record.evidence.workspace_id,
        record.evidence.metadata_format_version,
        record.evidence.offline,
        record.evidence.rustup_auto_install,
        record.evidence.toolchain_selection,
        record.evidence.owner_output_digest,
        record.evidence.command.join(" "),
    ));
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
        "Evidence: owner={}, representation={}, working-directory={}, network-requested={}, owner-work-requested={}, cargo-network-offline={}, rustup-auto-install={}, toolchain={}, output-digest={}\nCaptured output: stdout-retained={}, stdout-observed={}, stdout-omitted-observed={}, stdout-unobserved-unknown={}, stdout-complete={}, stdout-truncated={}, stderr-retained={}, stderr-observed={}, stderr-omitted-observed={}, stderr-unobserved-unknown={}, stderr-complete={}, stderr-truncated={}\nBounds: manifest-bytes={}, timeout-ms={}, stdout-bytes={}, stderr-bytes={}, framing={}\nCommand: {}\nFallback: {}\n",
        report.evidence.owner,
        report.evidence.command_representation,
        report.evidence.working_directory,
        report.evidence.network_requested,
        report.evidence.owner_work_requested,
        report.evidence.cargo_network_offline,
        report.evidence.rustup_auto_install,
        report.evidence.toolchain_selection,
        report.evidence.owner_output_digest,
        report.evidence.stdout_retained_bytes,
        report.evidence.stdout_observed_bytes,
        report.evidence.stdout_omitted_observed_bytes,
        report.evidence.stdout_unobserved_bytes_unknown,
        report.evidence.stdout_complete,
        report.evidence.stdout_truncated,
        report.evidence.stderr_retained_bytes,
        report.evidence.stderr_observed_bytes,
        report.evidence.stderr_omitted_observed_bytes,
        report.evidence.stderr_unobserved_bytes_unknown,
        report.evidence.stderr_complete,
        report.evidence.stderr_truncated,
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

pub fn render_profile_diff_human(envelope: &CommandEnvelope<ProfileDiffRecord>) -> String {
    let record = envelope
        .record
        .as_ref()
        .expect("profile diff result has a record");
    let mut output = format!(
        "Ferris profile diff {}\nSchema: {}\nResult: {}\nExecutable: {}\nBefore: profile_id={}, revision={}, consumer={}, content_digest={}\nAfter: profile_id={}, revision={}, consumer={}, content_digest={}\nChanged sections:\n",
        record.diff_id,
        record.schema,
        envelope.result_class,
        record.executable,
        record.before.profile_id,
        record.before.revision,
        record.before.consumer,
        record.before.content_digest,
        record.after.profile_id,
        record.after.revision,
        record.after.consumer,
        record.after.content_digest,
    );
    if record.changed_sections.is_empty() {
        output.push_str("  - none\n");
    } else {
        for section in &record.changed_sections {
            output.push_str(&format!("  - {section}\n"));
        }
    }
    output.push_str("Changes:\n");
    if record.changes.is_empty() {
        output.push_str("  - none\n");
    } else {
        for change in &record.changes {
            output.push_str(&format!(
                "  - {}: {} (before_digest={}, after_digest={})\n",
                change.path,
                change.change_kind,
                change.before_value_digest.as_deref().unwrap_or("none"),
                change.after_value_digest.as_deref().unwrap_or("none"),
            ));
        }
    }
    output.push_str("Unchanged sections:\n");
    if record.unchanged_sections.is_empty() {
        output.push_str("  - none\n");
    } else {
        for section in &record.unchanged_sections {
            output.push_str(&format!("  - {section}\n"));
        }
    }
    output.push_str("Unknowns:\n");
    if record.unknowns.is_empty() {
        output.push_str("  - none\n");
    } else {
        for unknown in &record.unknowns {
            output.push_str(&format!("  - {unknown}\n"));
        }
    }
    output.push_str("Limitations:\n");
    for limitation in &record.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
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
    plan_from_decoded_metadata(
        manifest_path,
        workspace_id,
        metadata,
        &workspace_root,
        bytes,
    )
}

fn plan_from_decoded_metadata(
    manifest_path: &Path,
    workspace_id: &str,
    metadata: CargoMetadata,
    workspace_root: &Path,
    bytes: &[u8],
) -> Result<CommandEnvelope<PlanRecord>, CoreError> {
    let selected_manifest = workspace_relative_path(manifest_path, workspace_root)?;
    let mut packages = metadata
        .packages
        .into_iter()
        .map(|package| -> Result<PackageRecord, CoreError> {
            let workspace_member = metadata.workspace_members.contains(&package.id);
            let manifest_path =
                workspace_relative_path(Path::new(&package.manifest_path), workspace_root)?;
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
        selection_identity(workspace_id, &selected_manifest),
        invocation_identity_for_selection("plan", workspace_id, &selected_manifest),
        record,
    ))
}

fn canonical_federated_workspace_root(metadata: &CargoMetadata) -> Result<PathBuf, CoreError> {
    let workspace_root = Path::new(&metadata.workspace_root);
    if !workspace_root.is_dir() {
        return Err(CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-CARGO-WORKSPACE-ROOT-INVALID",
            "Cargo metadata did not report an available workspace root directory.",
            vec!["Run cargo metadata directly and inspect its workspace_root field.".to_owned()],
        ));
    }
    workspace_root.canonicalize().map_err(|_| {
        CoreError::new(
            ResultClass::Incomplete,
            "FERRIS-CARGO-WORKSPACE-ROOT-UNREADABLE",
            "Cargo metadata reported a workspace root that could not be resolved.",
            vec!["Run cargo metadata directly and inspect its workspace_root field.".to_owned()],
        )
    })
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
        selection_identity(workspace_id, &selected_manifest),
        invocation_identity_for_selection("graph", workspace_id, &selected_manifest),
        record,
    ))
}

fn workspace_packages_from_metadata(
    metadata: CargoMetadata,
    workspace_root: &Path,
) -> Result<Vec<WorkspacePackage>, CoreError> {
    let workspace_members = metadata
        .workspace_members
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut packages = metadata
        .packages
        .into_iter()
        .filter(|package| workspace_members.contains(&package.id))
        .map(|package| {
            let manifest_path_absolute = PathBuf::from(&package.manifest_path)
                .canonicalize()
                .map_err(|error| {
                    CoreError::new(
                        ResultClass::Internal,
                        "FERRIS-CARGO-PACKAGE-PATH-INVALID",
                        "Cargo reported a package manifest Ferris could not resolve safely.",
                        vec![
                            "Run cargo metadata directly and retain its package manifest_path fields."
                                .to_owned(),
                            "Report the Cargo and Ferris versions.".to_owned(),
                        ],
                    )
                    .with_source_digest(digest_text(&format!(
                        "{}\0{error}",
                        normalize_path_text(&package.manifest_path)
                    )))
                })?;
            let manifest_path =
                workspace_relative_path(&manifest_path_absolute, workspace_root)?;
            let package_root_absolute = manifest_path_absolute.parent().ok_or_else(|| {
                CoreError::new(
                    ResultClass::Internal,
                    "FERRIS-CARGO-PACKAGE-PARENT-MISSING",
                    "Cargo reported a package manifest without a selectable parent directory.",
                    vec!["Report this Ferris invariant failure.".to_owned()],
                )
            })?;
            let package_root = parent_path_text(&manifest_path);
            Ok(WorkspacePackage {
                package: PackageRecord {
                    identity: package_identity(&package.name, &package.version, &manifest_path),
                    name: package.name,
                    version: package.version,
                    manifest_path,
                    workspace_member: true,
                },
                package_root,
                package_root_absolute: package_root_absolute.to_path_buf(),
                dependencies: package.dependencies,
            })
        })
        .collect::<Result<Vec<_>, CoreError>>()?;
    packages.sort_by(|left, right| left.package.identity.cmp(&right.package.identity));
    Ok(packages)
}

fn normalize_validation_package_requests(values: &[String]) -> Result<Vec<String>, CoreError> {
    let mut packages = Vec::new();
    for value in values {
        let normalized = value.trim();
        if normalized.is_empty()
            || normalized
                .chars()
                .any(|character| matches!(character, '\r' | '\n' | '\t' | '\0' | ' '))
        {
            return Err(CoreError::new(
                ResultClass::Invalid,
                "FERRIS-VALIDATION-PACKAGE-INVALID",
                "Changed-package inputs must be non-empty Cargo package names without whitespace.",
                vec![
                    "Pass workspace package names with --changed-package or use --changed-path."
                        .to_owned(),
                ],
            ));
        }
        packages.push(normalized.to_owned());
    }
    packages.sort();
    packages.dedup();
    Ok(packages)
}

fn explicit_workspace_relative_path(
    path: &Path,
    workspace_root: &Path,
) -> Result<String, CoreError> {
    let relative = path.strip_prefix(workspace_root).map_err(|_| {
        CoreError::new(
            ResultClass::Invalid,
            "FERRIS-VALIDATION-CHANGE-PATH-OUTSIDE-WORKSPACE",
            "The explicit changed path is outside the selected workspace.",
            vec![
                "Pass a local path inside the selected workspace or use --changed-package."
                    .to_owned(),
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

fn supports_validation_path_anchor(path: &Path, package_root: &Path) -> bool {
    if path == package_root {
        return path.is_dir();
    }
    if !path.is_file() {
        return false;
    }
    path.extension().and_then(|extension| extension.to_str()) == Some("rs")
        && path.file_name().and_then(|name| name.to_str()) != Some("build.rs")
}

fn record_validation_package(
    packages: &mut BTreeMap<String, ValidationPackageBuilder>,
    package: &PackageRecord,
    disposition: ValidationPackageDisposition,
    reason: String,
) {
    let entry =
        packages
            .entry(package.identity.clone())
            .or_insert_with(|| ValidationPackageBuilder {
                package: package.clone(),
                disposition,
                reasons: Vec::new(),
            });
    if entry.disposition != ValidationPackageDisposition::Anchor
        && disposition == ValidationPackageDisposition::Anchor
    {
        entry.disposition = disposition;
    }
    entry.reasons.push(reason);
}

fn validation_activity_plans(
    packages: &[PackageRecord],
    package_scope: ValidationActivityScope,
    reason_prefix: &str,
) -> Vec<ValidationActivityPlan> {
    if packages.is_empty() {
        return Vec::new();
    }
    let package_identities = packages
        .iter()
        .map(|package| package.identity.clone())
        .collect::<Vec<_>>();
    vec![
        ValidationActivityPlan {
            family: ValidationActivityFamily::Check,
            owner: "Cargo".to_owned(),
            package_scope,
            package_identities: package_identities.clone(),
            reason: format!(
                "{reason_prefix} Cargo check remains separate from tests and does not imply repository-wide validation completeness."
            ),
        },
        ValidationActivityPlan {
            family: ValidationActivityFamily::Test,
            owner: "Cargo".to_owned(),
            package_scope,
            package_identities,
            reason: format!(
                "{reason_prefix} Cargo test remains separate from non-Cargo policy, release, native, and other owner-defined validation gates."
            ),
        },
    ]
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

fn success_envelope<T: Serialize>(
    semantic_command_id: &str,
    selection_identity: String,
    invocation_identity: String,
    record: T,
) -> CommandEnvelope<T> {
    command_envelope(
        semantic_command_id,
        selection_identity,
        invocation_identity,
        ResultClass::Success,
        Vec::new(),
        Some(record),
    )
}

pub fn command_envelope<T: Serialize>(
    semantic_command_id: &str,
    selection_identity: String,
    invocation_identity: String,
    result_class: ResultClass,
    diagnostics: Vec<Diagnostic>,
    record: Option<T>,
) -> CommandEnvelope<T> {
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| diagnostic.result_class == result_class),
        "typed Ferris diagnostics must match the command result class"
    );
    let process_exit_code = result_class.exit_code();
    let command_version = env!("CARGO_PKG_VERSION");
    let identity_input = CommandResultIdentityInput {
        schema: COMMAND_RESULT_SCHEMA,
        command_version,
        semantic_command_id,
        selection_identity: &selection_identity,
        invocation_identity: &invocation_identity,
        result_class,
        process_exit_code,
        diagnostics: &diagnostics,
        record: &record,
    };
    let result_identity = record_id("result", &identity_input)
        .expect("typed Ferris command results must serialize for identity");
    CommandEnvelope {
        schema: COMMAND_RESULT_SCHEMA.to_owned(),
        command_version: command_version.to_owned(),
        semantic_command_id: semantic_command_id.to_owned(),
        selection_identity,
        invocation_identity,
        result_identity,
        result_class,
        process_exit_code,
        diagnostics,
        record,
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

fn selection_identity(workspace_id: &str, selected_manifest: &str) -> String {
    invocation_identity(&["selection", workspace_id, selected_manifest]).replacen(
        "invocation:",
        "selection:",
        1,
    )
}

fn request_selection_identity(workspace_id: &str, manifest_path: &Path) -> String {
    let normalized = normalize_path_text(&manifest_path.to_string_lossy());
    let suffix = normalized
        .split('/')
        .filter(|component| !component.is_empty())
        .rev()
        .take(2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/");
    selection_identity(workspace_id, &suffix)
}

fn doctor_selection_identity(workspace_id: &str, selection: &str) -> String {
    selection_identity(workspace_id, selection)
}

fn doctor_invocation_identity(workspace_id: &str, selection_identity: &str) -> String {
    invocation_identity(&["doctor", workspace_id, selection_identity])
}

fn doctor_record_id(record: &DoctorReport) -> Result<String, CoreError> {
    debug_assert!(record.report_id.is_empty());
    record_id("doctor", record)
}

fn validation_plan_record_id(
    workspace_id: &str,
    selected_manifest: &str,
    inputs: &[ValidationInputBuilder],
    selected_packages: &[ValidationPackageSelection],
    selected_activities: &[ValidationActivityPlan],
    fallback: &ValidationFallbackPlan,
) -> Result<String, CoreError> {
    let mut fallback_codes = BTreeSet::from([
        VALIDATION_FALLBACK_CODE_OWNER_DEFINED_VALIDATION_OUTSIDE_PULSE,
        VALIDATION_FALLBACK_CODE_SELECTED_PACKAGE_EVIDENCE_NOT_FULL_REFERENCE,
    ]);
    for input in inputs {
        match input.semantic_code {
            VALIDATION_INPUT_CODE_PACKAGE_PATH_REQUIRES_FULL_WORKSPACE_FALLBACK
            | VALIDATION_INPUT_CODE_WORKSPACE_PATH_OUTSIDE_PACKAGE_ANCHOR
            | VALIDATION_INPUT_CODE_AMBIGUOUS_PACKAGE_ROOT_MATCH => {
                fallback_codes.insert(input.semantic_code);
            }
            _ => {}
        }
    }

    let projection = ValidationPlanIdentityProjection {
        schema: VALIDATION_PLAN_SCHEMA.to_owned(),
        workspace_id: workspace_id.to_owned(),
        selected_manifest: selected_manifest.to_owned(),
        inputs: validation_plan_identity_inputs(inputs),
        selected_packages: validation_plan_identity_packages(selected_packages),
        selected_activities: validation_plan_identity_activities(selected_activities),
        fallback: ValidationPlanIdentityFallback {
            boundary: fallback.boundary.clone(),
            required_by_inputs: fallback.required_by_inputs,
            package_identities: validation_plan_identity_package_ids(&fallback.packages),
            activities: validation_plan_identity_activities(&fallback.activities),
            stable_codes: fallback_codes.into_iter().collect(),
        },
    };
    record_id("validation-plan", &projection)
}

fn validation_plan_identity_inputs(
    inputs: &[ValidationInputBuilder],
) -> Vec<ValidationPlanIdentityInput> {
    let mut projection = inputs
        .iter()
        .map(|input| ValidationPlanIdentityInput {
            kind: input.record.kind,
            value: input.record.value.clone(),
            disposition: input.record.disposition,
            package_identity: input.record.package_identity.clone(),
            semantic_code: input.semantic_code,
        })
        .collect::<Vec<_>>();
    projection.sort_by(|left, right| {
        validation_input_kind_name(left.kind)
            .cmp(validation_input_kind_name(right.kind))
            .then_with(|| left.value.cmp(&right.value))
    });
    projection
}

fn validation_plan_identity_packages(
    packages: &[ValidationPackageSelection],
) -> Vec<ValidationPlanIdentityPackage> {
    let mut projection = packages
        .iter()
        .map(|package| ValidationPlanIdentityPackage {
            identity: package.package.identity.clone(),
            disposition: package.disposition,
        })
        .collect::<Vec<_>>();
    projection.sort_by(|left, right| left.identity.cmp(&right.identity));
    projection
}

fn validation_plan_identity_package_ids(packages: &[PackageRecord]) -> Vec<String> {
    let mut identities = packages
        .iter()
        .map(|package| package.identity.clone())
        .collect::<Vec<_>>();
    identities.sort();
    identities
}

fn validation_plan_identity_activities(
    activities: &[ValidationActivityPlan],
) -> Vec<ValidationPlanIdentityActivity> {
    let mut projection = activities
        .iter()
        .map(|activity| {
            let mut package_identities = activity.package_identities.clone();
            package_identities.sort();
            ValidationPlanIdentityActivity {
                family: activity.family,
                owner: activity.owner.clone(),
                package_scope: activity.package_scope,
                package_identities,
            }
        })
        .collect::<Vec<_>>();
    projection.sort_by(|left, right| {
        validation_activity_family_name(left.family)
            .cmp(validation_activity_family_name(right.family))
            .then_with(|| left.owner.cmp(&right.owner))
            .then_with(|| {
                validation_activity_scope_name(left.package_scope)
                    .cmp(validation_activity_scope_name(right.package_scope))
            })
    });
    projection
}

fn validation_plan_request_selection_identity(
    workspace_id: &str,
    manifest_path: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> String {
    let normalized = normalize_path_text(&manifest_path.to_string_lossy());
    let manifest_suffix = normalized
        .split('/')
        .filter(|component| !component.is_empty())
        .rev()
        .take(2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/");
    let request_material = validation_plan_request_material(changed_paths, changed_packages);
    invocation_identity(&[
        "validation-plan-selection-request",
        workspace_id,
        &manifest_suffix,
        &request_material,
    ])
    .replacen("invocation:", "selection:", 1)
}

fn validation_plan_request_material(
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> String {
    let mut items = changed_packages
        .iter()
        .map(|package| format!("package:{}", package.trim()))
        .chain(changed_paths.iter().map(|path| {
            format!(
                "path:{}",
                lexically_normalize_path_text(&path.to_string_lossy())
            )
        }))
        .collect::<Vec<_>>();
    items.sort();
    items.dedup();
    items.join("\0")
}

fn validation_plan_selection_identity(
    workspace_id: &str,
    selected_manifest: &str,
    inputs: &[ValidationInputRecord],
) -> String {
    invocation_identity(&[
        "validation-plan-selection",
        workspace_id,
        selected_manifest,
        &validation_input_material(inputs),
    ])
    .replacen("invocation:", "selection:", 1)
}

fn validation_input_material(inputs: &[ValidationInputRecord]) -> String {
    let mut items = inputs
        .iter()
        .map(|input| {
            format!(
                "{}:{}:{}:{}",
                validation_input_kind_name(input.kind),
                input.value,
                validation_input_disposition_name(input.disposition),
                input.package_identity.as_deref().unwrap_or("-")
            )
        })
        .collect::<Vec<_>>();
    items.sort();
    items.join("\0")
}

fn validation_plan_invocation_identity(selection_identity: &str) -> String {
    invocation_identity(&[
        "validation-plan",
        selection_identity,
        "activity-families=check,test",
        "path-anchors=explicit-package-root-or-non-build-rs",
        "fallback=full-workspace-plus-owner-reference",
        "input-max=256",
        "cargo-metadata-format=1",
        "no-deps=true",
        "offline=true",
        "locked=true",
        "rustup-auto-install=false",
        "toolchain=owner-resolution-from-selected-manifest-directory-and-environment",
    ])
}

fn application_definition_identity(
    definition: &ApplicationDefinition,
) -> Result<String, CoreError> {
    let workspaces = definition
        .workspaces
        .iter()
        .map(|workspace| {
            let mut depends_on = workspace
                .depends_on
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            depends_on.sort();
            ApplicationDefinitionIdentityWorkspace {
                workspace_id: &workspace.workspace_id,
                manifest_path: &workspace.manifest_path,
                depends_on,
            }
        })
        .collect::<Vec<_>>();
    record_id(
        "application-definition",
        &ApplicationDefinitionIdentity {
            schema: &definition.schema,
            application_id: &definition.application_id,
            workspaces,
        },
    )
}

fn federated_validation_plan_identity(
    prefix: &str,
    application_definition_identity: &str,
    record: &FederatedValidationPlanRecord,
) -> Result<String, CoreError> {
    let workspaces = record
        .workspaces
        .iter()
        .map(|workspace| FederatedValidationIdentityWorkspace {
            workspace_id: &workspace.workspace_id,
            manifest_path: &workspace.manifest_path,
            disposition: workspace.disposition,
            reasons: &workspace.reasons,
            validation_plan_id: workspace
                .validation_plan
                .as_ref()
                .map(|plan| plan.validation_plan_id.as_str()),
        })
        .collect::<Vec<_>>();
    record_id(
        prefix,
        &FederatedValidationIdentityProjection {
            schema: &record.schema,
            application_id: &record.application_id,
            application_definition_identity,
            workspaces,
            fallback: &record.fallback,
        },
    )
}

fn federated_validation_plan_provisional_request_selection_identity(
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> String {
    let changed_path_count = format!("changed-path-count={}", changed_paths.len());
    let changed_package_count = format!("changed-package-count={}", changed_packages.len());
    invocation_identity(&[
        "federated-validation-plan-selection-request",
        "application-definition=unavailable",
        &changed_path_count,
        &changed_package_count,
    ])
    .replacen("invocation:", "selection:", 1)
}

fn federated_validation_plan_semantic_request_selection_identity(
    application_definition_identity: &str,
    application_root: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> String {
    let mut inputs = changed_packages
        .iter()
        .map(|package| format!("package:{}", package.trim()))
        .chain(changed_paths.iter().map(|path| {
            let value = path
                .canonicalize()
                .ok()
                .and_then(|canonical| {
                    canonical
                        .strip_prefix(application_root)
                        .ok()
                        .map(portable_relative_path)
                })
                .unwrap_or_else(|| "unavailable-or-outside-application".to_owned());
            format!("path:{value}")
        }))
        .collect::<Vec<_>>();
    inputs.sort();
    inputs.dedup();
    invocation_identity(&[
        "federated-validation-plan-selection",
        application_definition_identity,
        &inputs.join("\0"),
    ])
    .replacen("invocation:", "selection:", 1)
}

fn try_federated_validation_plan_semantic_request_selection_identity(
    application_path: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
) -> Option<String> {
    let bytes = read_application_definition(application_path).ok()?;
    let mut definition: ApplicationDefinition = serde_json::from_slice(&bytes).ok()?;
    if definition.schema != APPLICATION_SCHEMA {
        return None;
    }
    for workspace in &mut definition.workspaces {
        workspace.depends_on.sort();
    }
    definition
        .workspaces
        .sort_by(|left, right| left.workspace_id.cmp(&right.workspace_id));
    let definition_identity = application_definition_identity(&definition).ok()?;
    let application_path = application_path.canonicalize().ok()?;
    let application_root = application_path.parent()?;
    Some(
        federated_validation_plan_semantic_request_selection_identity(
            &definition_identity,
            application_root,
            changed_paths,
            changed_packages,
        ),
    )
}

fn federated_validation_plan_invocation_identity(selection_identity: &str) -> String {
    let application_max_bytes = format!("application-max-bytes={MAX_APPLICATION_INPUT_BYTES}");
    let workspace_min = format!("workspace-min={MIN_FEDERATED_PLAN_WORKSPACES}");
    let workspace_max = format!("workspace-max={MAX_FEDERATED_PLAN_WORKSPACES}");
    let metadata_timeout = format!(
        "cargo-metadata-timeout-seconds={}",
        FEDERATED_PLAN_METADATA_TIMEOUT.as_secs()
    );
    let metadata_output =
        format!("cargo-metadata-output-max-bytes={MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES}");
    invocation_identity(&[
        "federated-validation-plan",
        selection_identity,
        "application-schema=ferris.application/v0",
        "result-schema=ferris.federated-validation-plan/v0",
        &application_max_bytes,
        &workspace_min,
        &workspace_max,
        "input-max=256",
        "path-ownership=canonical-cargo-workspace-root",
        "relationship-propagation=transitive-reverse-full-workspace-fallback",
        "fallback=full-application-plus-owner-reference",
        "cargo-resolution=independent-workspaces",
        "cargo-metadata-sequential=true",
        "cargo-metadata-once-per-declared-workspace=true",
        &metadata_timeout,
        &metadata_output,
        "cargo-metadata-format=1",
        "no-deps=true",
        "offline=true",
        "locked=true",
        "rustup-auto-install=false",
        "execution=false",
    ])
}

fn federated_plan_request_selection_identity(request: &FederatedPlanRequest) -> String {
    let mut workspaces = request
        .workspaces
        .iter()
        .map(|workspace| format!("{}={}", workspace.workspace_id, workspace.manifest_path))
        .collect::<Vec<_>>();
    workspaces.sort();
    invocation_identity(&[
        "federated-plan-selection",
        &request.schema,
        &request.application_id,
        &request.revision,
        &request.owner,
        &workspaces.join("\0"),
    ])
    .replacen("invocation:", "selection:", 1)
}

fn federated_plan_invocation_identity(selection_identity: &str) -> String {
    let request_max_bytes = format!("request-max-bytes={MAX_FEDERATED_PLAN_REQUEST_BYTES}");
    let workspace_min = format!("workspace-min={MIN_FEDERATED_PLAN_WORKSPACES}");
    let workspace_max = format!("workspace-max={MAX_FEDERATED_PLAN_WORKSPACES}");
    let metadata_timeout = format!(
        "cargo-metadata-timeout-seconds={}",
        FEDERATED_PLAN_METADATA_TIMEOUT.as_secs()
    );
    let metadata_stdout_max =
        format!("cargo-metadata-stdout-max-bytes={MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES}");
    let metadata_stderr_max =
        format!("cargo-metadata-stderr-max-bytes={MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES}");
    invocation_identity(&[
        "federated-plan",
        selection_identity,
        "request-schema=ferris.federated-plan-request/v0",
        &request_max_bytes,
        &workspace_min,
        &workspace_max,
        "cargo-metadata-sequential=true",
        "cargo-metadata-once-per-requested-workspace=true",
        &metadata_timeout,
        &metadata_stdout_max,
        &metadata_stderr_max,
        "cargo-metadata-format=1",
        "no-deps=true",
        "offline=true",
        "locked=true",
        "rustup-auto-install=false",
        "toolchain=owner-resolution-from-selected-manifest-directory-and-environment",
        "manifest-path-syntax=request-relative-forward-slash-no-parent",
        "workspace-root-deduplication=canonical",
        "executable=false",
    ])
}

fn federated_plan_record_id(
    application_id: &str,
    revision: &str,
    owner: &str,
    workspaces: &[FederatedWorkspacePlan],
) -> Result<String, CoreError> {
    let workspace_plans = workspaces
        .iter()
        .map(|workspace| {
            (
                workspace.workspace_id.as_str(),
                workspace.plan.plan_id.as_str(),
            )
        })
        .collect::<Vec<_>>();
    record_id(
        "federated-plan",
        &(
            FEDERATED_PLAN_SCHEMA,
            application_id,
            revision,
            owner,
            workspace_plans,
        ),
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
    let normalized = normalize_command_line_arguments(args);
    let mut parts = Vec::with_capacity(normalized.len() + 1);
    parts.push(semantic_command_id);
    parts.extend(normalized.iter().map(String::as_str));
    invocation_identity(&parts)
}

pub fn command_line_selection_identity(semantic_command_id: &str, args: &[String]) -> String {
    let normalized = normalize_command_line_arguments(args);
    selection_identity(semantic_command_id, &digest_text(&normalized.join("\0")))
}

fn normalize_command_line_arguments(args: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    let mut arguments = args.iter().skip(1);
    if arguments.clone().next().is_some_and(|argument| {
        matches!(
            argument.as_str(),
            "plan"
                | "validation-plan"
                | "explain"
                | "graph"
                | "doctor"
                | "profile-diff"
                | "federated-plan"
                | "federated-validation-plan"
                | "revision-skew"
        )
    }) {
        arguments.next();
    }
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--workspace-id" => {
                normalized.push("option:workspace-id".to_owned());
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", digest_text(value)))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--manifest-path" => {
                normalized.push("option:manifest-path".to_owned());
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", manifest_selection_digest(value)))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--before" | "--after" => {
                normalized.push(format!("option:{}", argument.trim_start_matches('-')));
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", profile_path_selection_digest(value)))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--request" | "--application" => {
                normalized.push(format!("option:{}", argument.trim_start_matches('-')));
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", request_path_selection_digest(value)))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--changed-path" => {
                normalized.push("option:changed-path".to_owned());
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", profile_path_selection_digest(value)))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--changed-package" => {
                normalized.push("option:changed-package".to_owned());
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", digest_text(value.trim())))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--format" => {
                normalized.push("option:format".to_owned());
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", value.to_ascii_lowercase()))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            value if value.starts_with("--workspace-id=") => {
                normalized.push("option:workspace-id".to_owned());
                normalized.push(format!(
                    "value:{}",
                    digest_text(value.trim_start_matches("--workspace-id="))
                ));
            }
            value if value.starts_with("--manifest-path=") => {
                normalized.push("option:manifest-path".to_owned());
                normalized.push(format!(
                    "value:{}",
                    manifest_selection_digest(value.trim_start_matches("--manifest-path="))
                ));
            }
            value if value.starts_with("--before=") || value.starts_with("--after=") => {
                let (option, path) = value.split_once('=').expect("matched option assignment");
                normalized.push(format!("option:{option}"));
                normalized.push(format!("value:{}", profile_path_selection_digest(path)));
            }
            value if value.starts_with("--request=") => {
                normalized.push("option:request".to_owned());
                normalized.push(format!(
                    "value:{}",
                    request_path_selection_digest(value.trim_start_matches("--request="))
                ));
            }
            value if value.starts_with("--application=") => {
                normalized.push("option:application".to_owned());
                normalized.push(format!(
                    "value:{}",
                    request_path_selection_digest(value.trim_start_matches("--application="))
                ));
            }
            value if value.starts_with("--changed-path=") => {
                normalized.push("option:changed-path".to_owned());
                normalized.push(format!(
                    "value:{}",
                    profile_path_selection_digest(value.trim_start_matches("--changed-path="))
                ));
            }
            value if value.starts_with("--changed-package=") => {
                normalized.push("option:changed-package".to_owned());
                normalized.push(format!(
                    "value:{}",
                    digest_text(value.trim_start_matches("--changed-package=").trim())
                ));
            }
            value if value.starts_with("--format=") => {
                normalized.push("option:format".to_owned());
                normalized.push(format!(
                    "value:{}",
                    value.trim_start_matches("--format=").to_ascii_lowercase()
                ));
            }
            value if value.starts_with('-') => {
                if let Some((option, option_value)) = value.split_once('=') {
                    normalized.push(format!("option:{option}"));
                    normalized.push(format!("value:{}", digest_text(option_value)));
                } else {
                    normalized.push(format!("option:{value}"));
                }
            }
            value => normalized.push(format!("argument:{}", digest_text(value))),
        }
    }
    normalized
}

fn manifest_selection_digest(value: &str) -> String {
    digest_text(&portable_path_suffix(Path::new(value), 2))
}

fn portable_path_suffix(path: &Path, component_count: usize) -> String {
    let normalized = normalize_path_text(&path.to_string_lossy());
    normalized
        .split('/')
        .filter(|component| !component.is_empty())
        .rev()
        .take(component_count)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/")
}

fn profile_path_selection_digest(value: &str) -> String {
    digest_text(&lexically_normalize_path_text(value))
}

fn request_path_selection_digest(value: &str) -> String {
    digest_text(&request_path_identity_material(Path::new(value)))
}

fn request_path_identity_material(path: &Path) -> String {
    let absolute = if path.is_absolute() || path.has_root() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|directory| directory.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };
    lexically_normalize_path_text(&absolute.to_string_lossy())
}

fn validate_workspace_id(workspace_id: &str) -> Result<(), CoreError> {
    let valid = valid_portable_id(workspace_id);
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

fn validate_application_id(application_id: &str) -> Result<(), CoreError> {
    if valid_portable_id(application_id) {
        return Ok(());
    }
    Err(CoreError::new(
        ResultClass::Invalid,
        "FERRIS-APPLICATION-ID-INVALID",
        "Application identity must contain 1 to 128 ASCII letters, digits, '.', '-', '_', ':', or '/'.",
        vec![
            "Pass a stable portable application identity such as org.example/application."
                .to_owned(),
        ],
    ))
}

fn valid_portable_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_' | ':' | '/')
        })
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

fn lexically_normalize_path_text(value: &str) -> String {
    let normalized = normalize_path_text(value);
    let (prefix, remainder, rooted) = if let Some(remainder) = normalized.strip_prefix("//") {
        ("//", remainder, true)
    } else if let Some(remainder) = normalized.strip_prefix('/') {
        ("/", remainder, true)
    } else if normalized.len() >= 3
        && normalized.as_bytes()[1] == b':'
        && normalized.as_bytes()[2] == b'/'
    {
        (&normalized[..3], &normalized[3..], true)
    } else {
        ("", normalized.as_str(), false)
    };

    let mut components = Vec::new();
    for component in remainder.split('/') {
        match component {
            "" | "." => {}
            ".." if components.last().is_some_and(|last| *last != "..") => {
                components.pop();
            }
            ".." if !rooted => components.push(component),
            ".." => {}
            _ => components.push(component),
        }
    }

    let joined = components.join("/");
    if prefix.is_empty() {
        if joined.is_empty() {
            ".".to_owned()
        } else {
            joined
        }
    } else if joined.is_empty() {
        prefix.to_owned()
    } else if prefix.ends_with('/') {
        format!("{prefix}{joined}")
    } else {
        format!("{prefix}/{joined}")
    }
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

fn validation_input_kind_name(kind: ValidationInputKind) -> &'static str {
    match kind {
        ValidationInputKind::Path => "path",
        ValidationInputKind::Package => "package",
    }
}

fn validation_input_disposition_name(disposition: ValidationInputDisposition) -> &'static str {
    match disposition {
        ValidationInputDisposition::ExplicitPackage => "explicit_package",
        ValidationInputDisposition::OwnedRustPath => "owned_rust_path",
        ValidationInputDisposition::FullWorkspaceFallback => "full_workspace_fallback",
    }
}

fn validation_package_disposition_name(disposition: ValidationPackageDisposition) -> &'static str {
    match disposition {
        ValidationPackageDisposition::Anchor => "anchor",
        ValidationPackageDisposition::ReverseDependency => "reverse_dependency",
    }
}

fn validation_activity_family_name(family: ValidationActivityFamily) -> &'static str {
    match family {
        ValidationActivityFamily::Check => "check",
        ValidationActivityFamily::Test => "test",
    }
}

fn validation_activity_scope_name(scope: ValidationActivityScope) -> &'static str {
    match scope {
        ValidationActivityScope::SelectedPackageClosure => "selected_package_closure",
        ValidationActivityScope::FullWorkspaceFallback => "full_workspace_fallback",
    }
}

fn federated_validation_workspace_disposition_name(
    disposition: FederatedValidationWorkspaceDisposition,
) -> &'static str {
    match disposition {
        FederatedValidationWorkspaceDisposition::DirectPlan => "direct plan",
        FederatedValidationWorkspaceDisposition::RelationshipFallback => "relationship fallback",
        FederatedValidationWorkspaceDisposition::ApplicationFallback => "application fallback",
        FederatedValidationWorkspaceDisposition::NotSelected => "not selected",
    }
}

fn revision_declaration_kind_name(kind: RevisionDeclarationKind) -> &'static str {
    match kind {
        RevisionDeclarationKind::Branch => "branch",
        RevisionDeclarationKind::Revision => "revision",
        RevisionDeclarationKind::Tag => "tag",
        RevisionDeclarationKind::DefaultBranch => "default branch",
        RevisionDeclarationKind::Ambiguous => "ambiguous",
        RevisionDeclarationKind::Missing => "missing",
    }
}

fn revision_skew_status_name(status: RevisionSkewStatus) -> &'static str {
    match status {
        RevisionSkewStatus::Equal => "equal",
        RevisionSkewStatus::Behind => "behind",
        RevisionSkewStatus::Ahead => "ahead",
        RevisionSkewStatus::Divergent => "divergent",
        RevisionSkewStatus::Unavailable => "unavailable",
        RevisionSkewStatus::Unknown => "unknown",
    }
}

fn validation_activity_packages(
    activity: &ValidationActivityPlan,
    package_names: &BTreeMap<String, String>,
) -> String {
    activity
        .package_identities
        .iter()
        .map(|identity| {
            package_names
                .get(identity)
                .cloned()
                .unwrap_or_else(|| identity.clone())
        })
        .collect::<Vec<_>>()
        .join(", ")
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
    hasher.update(b"ferris.command-output/v1");
    hasher.update([0]);
    hasher.update((stdout.len() as u64).to_le_bytes());
    hasher.update(stdout);
    hasher.update((stderr.len() as u64).to_le_bytes());
    hasher.update(stderr);
    format!("sha256:{}", hex_digest(&hasher.finalize()))
}

fn bounded_output_evidence(capture: &BoundedCapture, termination: &str) -> BoundedOutputEvidence {
    let stdout = canonical_failure_stream(&capture.stdout, termination);
    let stderr = canonical_failure_stream(&capture.stderr, termination);
    BoundedOutputEvidence {
        schema: "ferris.bounded-output-evidence/v0".to_owned(),
        owner_output_framing: "length-prefixed-stdout-stderr/v1".to_owned(),
        stdout_retained_bytes: stdout.retained.len() as u64,
        stdout_observed_bytes: stdout.observed_bytes,
        stdout_omitted_observed_bytes: stdout
            .observed_bytes
            .saturating_sub(stdout.retained.len() as u64),
        stdout_unobserved_bytes_unknown: !stdout.complete || stdout.truncated,
        stdout_complete: stdout.complete,
        stdout_truncated: stdout.truncated,
        stdout_read_failed: stdout.failed,
        stderr_retained_bytes: stderr.retained.len() as u64,
        stderr_observed_bytes: stderr.observed_bytes,
        stderr_omitted_observed_bytes: stderr
            .observed_bytes
            .saturating_sub(stderr.retained.len() as u64),
        stderr_unobserved_bytes_unknown: !stderr.complete || stderr.truncated,
        stderr_complete: stderr.complete,
        stderr_truncated: stderr.truncated,
        stderr_read_failed: stderr.failed,
        output_digest: digest_command_output(&stdout.retained, &stderr.retained),
        termination: termination.to_owned(),
        termination_scope: "direct-child".to_owned(),
        termination_cleanup_complete: capture.termination_cleanup_complete,
    }
}

fn canonical_failure_stream(stream: &CapturedStream, termination: &str) -> CapturedStream {
    if termination == "output-bound" && stream.truncated {
        let mut stream = stream.clone();
        stream.observed_bytes = stream.retained.len() as u64 + 1;
        return stream;
    }
    let discard_unsettled = match termination {
        "output-bound" => !stream.truncated,
        "timeout" => !stream.complete,
        _ => false,
    };
    if discard_unsettled {
        return CapturedStream {
            retained: Vec::new(),
            observed_bytes: 0,
            complete: false,
            truncated: false,
            failed: false,
        };
    }
    stream.clone()
}

fn hex_digest(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture(path: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(path)
    }

    fn manifest() -> PathBuf {
        fixture("simple-workspace/Cargo.toml")
    }

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "ferris-profile-diff-{label}-{}-{nonce}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, AtomicOrdering::Relaxed)
            ));
            fs::create_dir_all(&path).expect("create profile test directory");
            Self(path)
        }

        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn copy_tree(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("create fixture destination");
        for entry in fs::read_dir(source).expect("read fixture directory") {
            let entry = entry.expect("fixture entry");
            let source_path = entry.path();
            let destination_path = destination.join(entry.file_name());
            if entry.file_type().expect("fixture file type").is_dir() {
                copy_tree(&source_path, &destination_path);
            } else {
                fs::copy(source_path, destination_path).expect("copy fixture file");
            }
        }
    }

    fn profile_value(
        profile_id: &str,
        revision: &str,
        consumer: &str,
        identity: serde_json::Value,
    ) -> serde_json::Value {
        serde_json::json!({
            "schema": PROFILE_EVIDENCE_SCHEMA,
            "profile_id": profile_id,
            "revision": revision,
            "consumer": consumer,
            "sections": {
                "identity": identity,
                "closure": {},
                "features": {},
                "toolchain": {},
                "targets": {},
                "providers": {},
                "native": {},
                "stages": {},
                "assurance": {},
                "stewardship": {},
                "support": {},
                "lifecycle": {}
            }
        })
    }

    fn write_profile(path: &Path, value: &serde_json::Value) {
        fs::write(
            path,
            serde_json::to_vec_pretty(value).expect("serialize profile fixture"),
        )
        .expect("write profile fixture");
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
    fn profile_diff_identical_and_revision_only_results_are_typed() {
        let directory = TestDirectory::new("identical-revision");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        let before = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({"state": "not_observed"}),
        );
        write_profile(&before_path, &before);
        write_profile(&after_path, &before);

        let identical = create_profile_diff(&before_path, &after_path).expect("identical diff");
        assert_eq!(identical.result_class, ResultClass::Success);
        assert_eq!(identical.process_exit_code, 0);
        assert!(identical.record.expect("record").changes.is_empty());

        let after = profile_value(
            "profile.example",
            "r2",
            "consumer.example",
            serde_json::json!({"state": "not_observed"}),
        );
        write_profile(&after_path, &after);
        let revision = create_profile_diff(&before_path, &after_path).expect("revision diff");
        assert_eq!(revision.result_class, ResultClass::Difference);
        assert_eq!(revision.process_exit_code, 1);
        let record = revision.record.expect("record");
        assert!(record.changed_sections.is_empty());
        assert_eq!(record.changes.len(), 1);
        assert_eq!(record.changes[0].path, "/revision");
    }

    #[test]
    fn profile_diff_is_deterministic_across_key_order_and_path_relocation() {
        let first_directory = TestDirectory::new("deterministic-a");
        let second_directory = TestDirectory::new("deterministic-b");
        let first_before = first_directory.path("before.json");
        let first_after = first_directory.path("after.json");
        let second_before = second_directory.path("relocated-before.json");
        let second_after = second_directory.path("relocated-after.json");
        let before = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({"alpha": 1, "beta": 2}),
        );
        let after = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({"alpha": 1, "beta": 3}),
        );
        write_profile(&first_before, &before);
        write_profile(&first_after, &after);
        let reordered_before = serde_json::to_string(&before)
            .expect("serialize reordered fixture")
            .replace(r#""alpha":1,"beta":2"#, r#""beta":2,"alpha":1"#);
        let reordered_after = serde_json::to_string(&after)
            .expect("serialize reordered fixture")
            .replace(r#""alpha":1,"beta":3"#, r#""beta":3,"alpha":1"#);
        fs::write(&second_before, reordered_before).expect("write relocated before");
        fs::write(&second_after, reordered_after).expect("write relocated after");

        let first = create_profile_diff(&first_before, &first_after).expect("first diff");
        let second = create_profile_diff(&second_before, &second_after).expect("second diff");
        assert_eq!(first.selection_identity, second.selection_identity);
        assert_eq!(first.invocation_identity, second.invocation_identity);
        assert_eq!(first.result_identity, second.result_identity);
        assert_eq!(first.record, second.record);
    }

    #[test]
    fn profile_diff_pre_read_failures_bind_complete_normalized_paths() {
        let directory = TestDirectory::new("pre-read-identities");
        let valid_path = directory.path("valid.json");
        write_profile(
            &valid_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({}),
            ),
        );
        let first_missing = directory.path("first").join("same").join("missing.json");
        let second_missing = directory.path("second").join("same").join("missing.json");

        let first_error =
            create_profile_diff(&first_missing, &valid_path).expect_err("first missing input");
        let second_error =
            create_profile_diff(&second_missing, &valid_path).expect_err("second missing input");
        let first_envelope: CommandEnvelope<serde_json::Value> =
            profile_diff_error_envelope(&first_missing, &valid_path, &first_error);
        let second_envelope: CommandEnvelope<serde_json::Value> =
            profile_diff_error_envelope(&second_missing, &valid_path, &second_error);

        assert_ne!(
            first_envelope.selection_identity,
            second_envelope.selection_identity
        );
        assert_ne!(
            first_envelope.invocation_identity,
            second_envelope.invocation_identity
        );
        assert_ne!(
            first_envelope.result_identity,
            second_envelope.result_identity
        );
    }

    #[test]
    fn profile_diff_second_input_failure_binds_first_content_and_full_request() {
        let directory = TestDirectory::new("second-input-identity");
        let before_path = directory.path("before.json");
        let after_path = directory.path("nested").join("other").join("missing.json");
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({"value": "before"}),
            ),
        );
        let before_digest = load_profile(&before_path)
            .expect("load first profile")
            .content_digest;

        let error =
            create_profile_diff(&before_path, &after_path).expect_err("second input missing");
        assert_eq!(error.result_class(), ResultClass::Incomplete);
        let envelope: CommandEnvelope<serde_json::Value> =
            profile_diff_error_envelope(&before_path, &after_path, &error);
        let expected_material = format!(
            "before={before_digest};after-request={}",
            profile_request_digest(&after_path)
        );
        let expected_selection =
            invocation_identity(&["profile-diff-selection", &expected_material]).replacen(
                "invocation:",
                "selection:",
                1,
            );
        assert_eq!(envelope.selection_identity, expected_selection);
    }

    #[test]
    fn profile_request_paths_are_lexically_normalized_cross_platform() {
        assert_eq!(
            lexically_normalize_path_text(r"C:\repo\profiles\..\evidence\before.json"),
            "C:/repo/evidence/before.json"
        );
        assert_eq!(
            lexically_normalize_path_text("/repo/profiles/../evidence/before.json"),
            "/repo/evidence/before.json"
        );
        assert_eq!(
            profile_request_digest(Path::new(r"C:\repo\profiles\..\evidence\before.json")),
            digest_text("C:/repo/evidence/before.json")
        );
        assert_eq!(
            profile_request_digest(Path::new("/repo/profiles/../evidence/before.json")),
            digest_text("/repo/evidence/before.json")
        );
        assert_ne!(
            profile_request_digest(Path::new(r"C:\first\same\missing.json")),
            profile_request_digest(Path::new(r"C:\second\same\missing.json"))
        );
    }

    #[test]
    fn profile_diff_reports_sorted_pointer_changes_and_escaping() {
        let directory = TestDirectory::new("pointers");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({
                    "changed": "old",
                    "removed": "gone",
                    "nested": {"a/b~c": "old"}
                }),
            ),
        );
        write_profile(
            &after_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({
                    "added": "new",
                    "changed": "new",
                    "nested": {"a/b~c": "new"}
                }),
            ),
        );

        let record = create_profile_diff(&before_path, &after_path)
            .expect("profile diff")
            .record
            .expect("record");
        let observed = record
            .changes
            .iter()
            .map(|change| (change.path.as_str(), change.change_kind))
            .collect::<Vec<_>>();
        assert_eq!(
            observed,
            vec![
                ("/sections/identity/added", ProfileChangeKind::Added),
                ("/sections/identity/changed", ProfileChangeKind::Changed),
                (
                    "/sections/identity/nested/a~1b~0c",
                    ProfileChangeKind::Changed
                ),
                ("/sections/identity/removed", ProfileChangeKind::Removed),
            ]
        );
        assert_eq!(record.changed_sections, vec!["identity"]);
        assert!(record.changes[0].before_value_digest.is_none());
        assert!(record.changes[0].after_value_digest.is_some());
        assert!(record.changes[3].before_value_digest.is_some());
        assert!(record.changes[3].after_value_digest.is_none());
    }

    #[test]
    fn profile_diff_rejects_mismatched_identity_and_consumer() {
        let directory = TestDirectory::new("mismatch");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        write_profile(
            &before_path,
            &profile_value("profile.one", "r1", "consumer.one", serde_json::json!({})),
        );
        write_profile(
            &after_path,
            &profile_value("profile.two", "r2", "consumer.one", serde_json::json!({})),
        );
        let profile_error =
            create_profile_diff(&before_path, &after_path).expect_err("profile mismatch");
        assert_eq!(profile_error.result_class(), ResultClass::Invalid);

        write_profile(
            &after_path,
            &profile_value("profile.one", "r2", "consumer.two", serde_json::json!({})),
        );
        let consumer_error =
            create_profile_diff(&before_path, &after_path).expect_err("consumer mismatch");
        assert_eq!(consumer_error.result_class(), ResultClass::Invalid);
    }

    #[test]
    fn profile_diff_classifies_schema_json_file_and_size_failures() {
        let directory = TestDirectory::new("failures");
        let valid_path = directory.path("valid.json");
        let input_path = directory.path("input.json");
        let missing_path = directory.path("missing.json");
        write_profile(
            &valid_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({}),
            ),
        );

        let mut unsupported = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({}),
        );
        unsupported["schema"] = serde_json::json!("ferris.profile-evidence/v99");
        write_profile(&input_path, &unsupported);
        assert_eq!(
            create_profile_diff(&input_path, &valid_path)
                .expect_err("unsupported schema")
                .result_class(),
            ResultClass::Unsupported
        );

        fs::write(&input_path, b"{").expect("write malformed JSON");
        assert_eq!(
            create_profile_diff(&input_path, &valid_path)
                .expect_err("malformed JSON")
                .result_class(),
            ResultClass::Invalid
        );

        let mut unknown_field = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({}),
        );
        unknown_field["unexpected"] = serde_json::json!(true);
        write_profile(&input_path, &unknown_field);
        assert_eq!(
            create_profile_diff(&input_path, &valid_path)
                .expect_err("unknown top-level field")
                .result_class(),
            ResultClass::Invalid
        );

        let empty_identity = profile_value(" ", "r1", "consumer.example", serde_json::json!({}));
        write_profile(&input_path, &empty_identity);
        assert_eq!(
            create_profile_diff(&input_path, &valid_path)
                .expect_err("empty identity")
                .result_class(),
            ResultClass::Invalid
        );

        assert_eq!(
            create_profile_diff(&missing_path, &valid_path)
                .expect_err("missing input")
                .result_class(),
            ResultClass::Incomplete
        );

        fs::write(
            &input_path,
            vec![b'x'; MAX_PROFILE_INPUT_BYTES as usize + 1],
        )
        .expect("write oversized input");
        assert_eq!(
            create_profile_diff(&input_path, &valid_path)
                .expect_err("oversized input")
                .result_class(),
            ResultClass::Incomplete
        );
    }

    #[test]
    fn profile_diff_rejects_duplicate_members_recursively() {
        let directory = TestDirectory::new("duplicate-members");
        let valid_path = directory.path("valid.json");
        let input_path = directory.path("input.json");
        let profile = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({"value": "one"}),
        );
        write_profile(&valid_path, &profile);
        let compact = serde_json::to_string(&profile).expect("serialize compact profile");

        let cases = [
            (
                "top-level",
                compact.replacen(
                    r#""profile_id":"profile.example""#,
                    r#""profile_id":"profile.example","profile_id":"profile.other""#,
                    1,
                ),
            ),
            (
                "sections",
                compact.replacen(r#""closure":{}"#, r#""closure":{},"closure":{}"#, 1),
            ),
            (
                "nested",
                compact.replacen(r#""value":"one""#, r#""value":"one","value":"two""#, 1),
            ),
        ];

        for (label, contents) in cases {
            fs::write(&input_path, contents).expect("write duplicate profile");
            let error =
                create_profile_diff(&input_path, &valid_path).expect_err("duplicate rejected");
            assert_eq!(error.result_class(), ResultClass::Invalid, "{label}");
            assert_eq!(
                error.diagnostic().code,
                "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER",
                "{label}"
            );
        }
    }

    #[test]
    fn profile_diff_rejects_unsafe_output_visible_metadata() {
        let directory = TestDirectory::new("unsafe-metadata");
        let valid_path = directory.path("valid.json");
        let input_path = directory.path("input.json");
        let valid = profile_value(
            "profile.example",
            "r1",
            "consumer.example",
            serde_json::json!({}),
        );
        write_profile(&valid_path, &valid);

        for invalid_identity in [
            "profile\nforged".to_owned(),
            "profile\u{1b}[31m".to_owned(),
            "x".repeat(MAX_PROFILE_IDENTITY_BYTES + 1),
        ] {
            let invalid = profile_value(
                &invalid_identity,
                "r1",
                "consumer.example",
                serde_json::json!({}),
            );
            write_profile(&input_path, &invalid);
            let error =
                create_profile_diff(&input_path, &valid_path).expect_err("identity rejected");
            assert_eq!(error.result_class(), ResultClass::Invalid);
            assert_eq!(error.diagnostic().code, "FERRIS-PROFILE-IDENTITY-INVALID");
        }

        for invalid_key in [
            "forged\nkey".to_owned(),
            "escape\u{1b}key".to_owned(),
            "k".repeat(MAX_PROFILE_OBJECT_KEY_BYTES + 1),
        ] {
            let mut identity = serde_json::Map::new();
            identity.insert(invalid_key, serde_json::json!("value"));
            let invalid = profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::Value::Object(identity),
            );
            write_profile(&input_path, &invalid);
            let error =
                create_profile_diff(&input_path, &valid_path).expect_err("object member rejected");
            assert_eq!(error.result_class(), ResultClass::Invalid);
            assert_eq!(error.diagnostic().code, "FERRIS-PROFILE-METADATA-INVALID");
        }
    }

    #[test]
    fn profile_diff_arrays_use_positional_json_pointers() {
        let directory = TestDirectory::new("arrays");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({"items": ["a", "b"]}),
            ),
        );
        write_profile(
            &after_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({"items": ["x", "a", "b"]}),
            ),
        );

        let record = create_profile_diff(&before_path, &after_path)
            .expect("array diff")
            .record
            .expect("record");
        assert_eq!(
            record
                .changes
                .iter()
                .map(|change| (change.path.as_str(), change.change_kind))
                .collect::<Vec<_>>(),
            vec![
                ("/sections/identity/items/0", ProfileChangeKind::Changed),
                ("/sections/identity/items/1", ProfileChangeKind::Changed),
                ("/sections/identity/items/2", ProfileChangeKind::Added),
            ]
        );
    }

    #[test]
    fn profile_diff_preserves_added_and_removed_empty_containers() {
        let directory = TestDirectory::new("empty-containers");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({"removed_array": [], "removed_object": {}}),
            ),
        );
        write_profile(
            &after_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({"added_array": [], "added_object": {}}),
            ),
        );

        let record = create_profile_diff(&before_path, &after_path)
            .expect("empty-container diff")
            .record
            .expect("record");
        assert_eq!(
            record
                .changes
                .iter()
                .map(|change| (change.path.as_str(), change.change_kind))
                .collect::<Vec<_>>(),
            vec![
                ("/sections/identity/added_array", ProfileChangeKind::Added),
                ("/sections/identity/added_object", ProfileChangeKind::Added),
                (
                    "/sections/identity/removed_array",
                    ProfileChangeKind::Removed
                ),
                (
                    "/sections/identity/removed_object",
                    ProfileChangeKind::Removed
                ),
            ]
        );
    }

    #[test]
    fn profile_diff_allows_exactly_the_change_bound() {
        let directory = TestDirectory::new("exact-change-bound");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({}),
            ),
        );
        let mut values = serde_json::Map::new();
        for index in 0..MAX_PROFILE_CHANGES {
            values.insert(format!("key-{index:05}"), serde_json::json!(index));
        }
        write_profile(
            &after_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::Value::Object(values),
            ),
        );

        let record = create_profile_diff(&before_path, &after_path)
            .expect("exact change bound succeeds")
            .record
            .expect("record");
        assert_eq!(record.changes.len(), MAX_PROFILE_CHANGES);
    }

    #[test]
    fn profile_diff_blocks_more_than_the_change_bound() {
        let directory = TestDirectory::new("change-bound");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({}),
            ),
        );
        let mut values = serde_json::Map::new();
        for index in 0..=MAX_PROFILE_CHANGES {
            values.insert(format!("key-{index:05}"), serde_json::json!(index));
        }
        write_profile(
            &after_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::Value::Object(values),
            ),
        );

        let error =
            create_profile_diff(&before_path, &after_path).expect_err("change bound exceeded");
        assert_eq!(error.result_class(), ResultClass::Blocked);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-PROFILE-DIFF-BOUND-EXCEEDED"
        );
    }

    #[test]
    fn profile_diff_never_renders_raw_section_values() {
        let directory = TestDirectory::new("redaction");
        let before_path = directory.path("before.json");
        let after_path = directory.path("after.json");
        let before_secret = "SECRET-BEFORE-7d53f6";
        let after_secret = "SECRET-AFTER-a924be";
        let visible_key = "public/key~name";
        write_profile(
            &before_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({(visible_key): before_secret}),
            ),
        );
        write_profile(
            &after_path,
            &profile_value(
                "profile.example",
                "r1",
                "consumer.example",
                serde_json::json!({(visible_key): after_secret}),
            ),
        );

        let envelope = create_profile_diff(&before_path, &after_path).expect("profile diff");
        let json = serde_json::to_string_pretty(&envelope).expect("JSON output");
        let human = render_profile_diff_human(&envelope);
        for output in [&json, &human] {
            assert!(!output.contains(before_secret));
            assert!(!output.contains(after_secret));
            assert!(output.contains("public~1key~0name"));
        }
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
    fn federated_plan_preserves_sorted_independent_workspace_plans() {
        let request = fixture("federated-plan/request.json");
        let envelope = create_federated_plan(&request).expect("federated plan");
        let record = envelope.record.expect("federated plan record");

        assert_eq!(record.schema, FEDERATED_PLAN_SCHEMA);
        assert!(!record.executable);
        assert_eq!(record.application_id, "ferris.test/federated");
        assert_eq!(
            record
                .workspaces
                .iter()
                .map(|workspace| workspace.workspace_id.as_str())
                .collect::<Vec<_>>(),
            vec!["ferris.test/alpha", "ferris.test/beta"]
        );
        for workspace in &record.workspaces {
            let direct = create_plan(
                &fixture(&format!(
                    "federated-plan/workspace-{}/Cargo.toml",
                    workspace
                        .workspace_id
                        .strip_prefix("ferris.test/")
                        .expect("fixture workspace ID")
                )),
                &workspace.workspace_id,
            )
            .expect("direct workspace plan")
            .record
            .expect("direct workspace plan record");
            assert_eq!(workspace.plan, direct);
        }
        assert!(record.unknowns[0].contains(
            "dependency, lock, affected, validation, native, service, or contract relationship"
        ));
    }

    #[test]
    fn federated_plan_request_rejects_unknown_fields_and_workspace_bounds() {
        let directory = TestDirectory::new("federated-request-validation");
        let request_path = directory.path("request.json");
        let unknown = serde_json::json!({
            "schema": FEDERATED_PLAN_REQUEST_SCHEMA,
            "application_id": "ferris.test/federated",
            "revision": "r1",
            "owner": "ferris.test/owner",
            "workspaces": [],
            "unexpected": true
        });
        fs::write(
            &request_path,
            serde_json::to_vec(&unknown).expect("serialize unknown-field request"),
        )
        .expect("write unknown-field request");
        let unknown_error =
            create_federated_plan(&request_path).expect_err("unknown field should fail");
        assert_eq!(unknown_error.result_class(), ResultClass::Invalid);
        assert_eq!(
            unknown_error.diagnostic().code,
            "FERRIS-FEDERATED-PLAN-REQUEST-SHAPE-INVALID"
        );

        let one_workspace = serde_json::json!({
            "schema": FEDERATED_PLAN_REQUEST_SCHEMA,
            "application_id": "ferris.test/federated",
            "revision": "r1",
            "owner": "ferris.test/owner",
            "workspaces": [{
                "workspace_id": "ferris.test/only",
                "manifest_path": "Cargo.toml"
            }]
        });
        fs::write(
            &request_path,
            serde_json::to_vec(&one_workspace).expect("serialize bounded request"),
        )
        .expect("write bounded request");
        let bound_error =
            create_federated_plan(&request_path).expect_err("one workspace should fail");
        assert_eq!(bound_error.result_class(), ResultClass::Invalid);
        assert_eq!(
            bound_error.diagnostic().code,
            "FERRIS-FEDERATED-PLAN-WORKSPACE-BOUND-INVALID"
        );
    }

    #[test]
    fn federated_plan_rejects_non_portable_manifest_syntax() {
        assert!(is_absolute_manifest_request("/workspace/Cargo.toml"));
        assert!(is_absolute_manifest_request(r"C:\workspace\Cargo.toml"));
        assert!(!is_absolute_manifest_request("workspace-alpha/Cargo.toml"));

        for value in ["", r"workspace\Cargo.toml"] {
            let error =
                validate_federated_manifest_request(value).expect_err("syntax should be rejected");
            assert_eq!(
                error.diagnostic().code,
                "FERRIS-FEDERATED-PLAN-MANIFEST-SYNTAX-INVALID"
            );
        }
        for value in ["/workspace/Cargo.toml", "C:/workspace/Cargo.toml"] {
            let error = validate_federated_manifest_request(value)
                .expect_err("absolute syntax should be rejected");
            assert_eq!(
                error.diagnostic().code,
                "FERRIS-FEDERATED-PLAN-MANIFEST-ABSOLUTE"
            );
        }
        let traversal = validate_federated_manifest_request("workspace/../Cargo.toml")
            .expect_err("traversal should be rejected");
        assert_eq!(
            traversal.diagnostic().code,
            "FERRIS-FEDERATED-PLAN-MANIFEST-TRAVERSAL"
        );

        let slash_request = FederatedPlanRequest {
            schema: FEDERATED_PLAN_REQUEST_SCHEMA.to_owned(),
            application_id: "ferris.test/federated".to_owned(),
            revision: "r1".to_owned(),
            owner: "ferris.test/owner".to_owned(),
            workspaces: vec![FederatedWorkspaceRequest {
                workspace_id: "ferris.test/workspace".to_owned(),
                manifest_path: "workspace/Cargo.toml".to_owned(),
            }],
        };
        let backslash_request = FederatedPlanRequest {
            workspaces: vec![FederatedWorkspaceRequest {
                workspace_id: "ferris.test/workspace".to_owned(),
                manifest_path: r"workspace\Cargo.toml".to_owned(),
            }],
            ..slash_request.clone()
        };
        assert_ne!(
            federated_plan_request_selection_identity(&slash_request),
            federated_plan_request_selection_identity(&backslash_request)
        );
    }

    #[test]
    fn federated_plan_rejects_root_and_member_of_one_cargo_workspace() {
        let directory = TestDirectory::new("duplicate-workspace-root");
        copy_tree(
            &fixture("simple-workspace"),
            &directory.path("simple-workspace"),
        );
        let request_path = directory.path("request.json");
        let request = serde_json::json!({
            "schema": FEDERATED_PLAN_REQUEST_SCHEMA,
            "application_id": "ferris.test/federated",
            "revision": "r1",
            "owner": "ferris.test/owner",
            "workspaces": [
                {
                    "workspace_id": "ferris.test/root",
                    "manifest_path": "simple-workspace/Cargo.toml"
                },
                {
                    "workspace_id": "ferris.test/member",
                    "manifest_path": "simple-workspace/alpha/Cargo.toml"
                }
            ]
        });
        fs::write(
            &request_path,
            serde_json::to_vec_pretty(&request).expect("serialize request"),
        )
        .expect("write request");

        let error = create_federated_plan(&request_path)
            .expect_err("one Cargo workspace must not be federated twice");
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-FEDERATED-PLAN-WORKSPACE-ROOT-DUPLICATE"
        );
        assert!(error.diagnostic().message.contains("ferris.test/root"));
        let diagnostic = serde_json::to_string(error.diagnostic()).expect("diagnostic JSON");
        assert!(!diagnostic.contains(&directory.0.to_string_lossy().into_owned()));
        assert!(
            !diagnostic.contains(
                &Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .canonicalize()
                    .expect("repository root")
                    .to_string_lossy()
                    .into_owned()
            )
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
    fn federated_validation_selects_direct_and_transitive_relationship_fallbacks() {
        let application = fixture("sibling-workspaces/application.json");
        let changed_packages = ["ferris.test/selected:selected-member".to_owned()];
        let envelope = create_federated_validation_plan(&application, &[], &changed_packages)
            .expect("federated validation plan");
        let record = envelope.record.expect("federated validation record");

        assert_eq!(record.schema, FEDERATED_VALIDATION_PLAN_SCHEMA);
        assert!(!record.executable);
        assert!(!record.fallback.required_by_inputs);
        assert_eq!(record.workspaces.len(), 3);
        let selected = record
            .workspaces
            .iter()
            .find(|workspace| workspace.workspace_id == "ferris.test/selected")
            .expect("selected workspace");
        assert_eq!(
            selected.disposition,
            FederatedValidationWorkspaceDisposition::DirectPlan
        );
        let direct_plan = selected.validation_plan.as_ref().expect("direct plan");
        assert_eq!(direct_plan.schema, VALIDATION_PLAN_SCHEMA);
        assert_eq!(
            direct_plan.selected_packages[0].package.name,
            "selected-member"
        );
        for workspace_id in ["ferris.test/sibling", "ferris.test/gateway"] {
            let workspace = record
                .workspaces
                .iter()
                .find(|workspace| workspace.workspace_id == workspace_id)
                .expect("relationship workspace");
            assert_eq!(
                workspace.disposition,
                FederatedValidationWorkspaceDisposition::RelationshipFallback
            );
            assert!(workspace.validation_plan.is_none());
        }
    }

    #[test]
    fn federated_validation_retains_the_current_direct_workspace_plan() {
        let application = fixture("sibling-workspaces/application.json");
        let manifest = fixture("sibling-workspaces/selected/Cargo.toml");
        let changed_path = fixture("sibling-workspaces/selected/selected-member/src/lib.rs");
        let standalone = create_validation_plan(
            &manifest,
            "ferris.test/selected",
            std::slice::from_ref(&changed_path),
            &[],
        )
        .expect("standalone validation plan")
        .record
        .expect("standalone record");
        let federated = create_federated_validation_plan(
            &application,
            std::slice::from_ref(&changed_path),
            &[],
        )
        .expect("federated validation plan")
        .record
        .expect("federated record");
        let direct = federated
            .workspaces
            .iter()
            .find(|workspace| workspace.workspace_id == "ferris.test/selected")
            .and_then(|workspace| workspace.validation_plan.as_ref())
            .expect("embedded direct plan");

        assert_eq!(direct, &standalone);
    }

    #[test]
    fn federated_validation_application_path_requires_all_workspace_fallbacks() {
        let application = fixture("sibling-workspaces/application.json");
        let policy = fixture("sibling-workspaces/application-policy.txt");
        let record =
            create_federated_validation_plan(&application, std::slice::from_ref(&policy), &[])
                .expect("application fallback plan")
                .record
                .expect("application fallback record");

        assert!(record.fallback.required_by_inputs);
        assert!(record.workspaces.iter().all(|workspace| {
            workspace.disposition == FederatedValidationWorkspaceDisposition::ApplicationFallback
                && workspace.validation_plan.is_none()
        }));
    }

    #[test]
    fn application_definition_rejects_cycles_duplicates_and_traversal() {
        let cycle: ApplicationDefinition = serde_json::from_value(serde_json::json!({
            "schema": APPLICATION_SCHEMA,
            "application_id": "ferris.test/cycle",
            "workspaces": [
                {
                    "workspace_id": "ferris.test/a",
                    "manifest_path": "a/Cargo.toml",
                    "depends_on": ["ferris.test/b"]
                },
                {
                    "workspace_id": "ferris.test/b",
                    "manifest_path": "b/Cargo.toml",
                    "depends_on": ["ferris.test/a"]
                }
            ]
        }))
        .expect("cycle definition");
        let cycle_error = validate_application_relationships(&cycle).expect_err("cycle must fail");
        assert_eq!(
            cycle_error.diagnostic().code,
            "FERRIS-APPLICATION-DEPENDENCY-CYCLE"
        );

        let duplicate: ApplicationDefinition = serde_json::from_value(serde_json::json!({
            "schema": APPLICATION_SCHEMA,
            "application_id": "ferris.test/duplicate",
            "workspaces": [
                {
                    "workspace_id": "ferris.test/a",
                    "manifest_path": "a/Cargo.toml"
                },
                {
                    "workspace_id": "ferris.test/a",
                    "manifest_path": "b/Cargo.toml"
                }
            ]
        }))
        .expect("duplicate definition");
        let duplicate_error =
            validate_application_relationships(&duplicate).expect_err("duplicate must fail");
        assert_eq!(
            duplicate_error.diagnostic().code,
            "FERRIS-APPLICATION-WORKSPACE-ID-DUPLICATE"
        );

        let traversal = validate_application_manifest_request("../outside/Cargo.toml")
            .expect_err("traversal must fail");
        assert_eq!(
            traversal.diagnostic().code,
            "FERRIS-APPLICATION-MANIFEST-PATH-INVALID"
        );
    }

    #[test]
    fn federated_validation_identity_is_stable_after_relocation() {
        let first = TestDirectory::new("federated-validation-relocation-a");
        let second = TestDirectory::new("federated-validation-relocation-b");
        copy_tree(
            &fixture("sibling-workspaces"),
            &first.path("sibling-workspaces"),
        );
        copy_tree(
            &fixture("sibling-workspaces"),
            &second.path("sibling-workspaces"),
        );
        let first_application = first.path("sibling-workspaces/definition-a.json");
        let second_application = second.path("sibling-workspaces/renamed-definition.json");
        fs::rename(
            first.path("sibling-workspaces/application.json"),
            &first_application,
        )
        .expect("rename first application definition");
        fs::rename(
            second.path("sibling-workspaces/application.json"),
            &second_application,
        )
        .expect("rename second application definition");
        let first_changed = first.path("sibling-workspaces/selected/selected-member/src/lib.rs");
        let second_changed = second.path("sibling-workspaces/selected/selected-member/src/lib.rs");
        let first_plan =
            create_federated_validation_plan(&first_application, &[first_changed], &[])
                .expect("first relocated plan");
        let second_plan =
            create_federated_validation_plan(&second_application, &[second_changed], &[])
                .expect("second relocated plan");
        let first_record = first_plan.record.expect("first relocated record");
        let second_record = second_plan.record.expect("second relocated record");
        let first_direct = first_record
            .workspaces
            .iter()
            .find(|workspace| workspace.workspace_id == "ferris.test/selected")
            .and_then(|workspace| workspace.validation_plan.as_ref())
            .expect("first direct plan");
        let second_direct = second_record
            .workspaces
            .iter()
            .find(|workspace| workspace.workspace_id == "ferris.test/selected")
            .and_then(|workspace| workspace.validation_plan.as_ref())
            .expect("second direct plan");

        assert_eq!(
            first_plan.selection_identity,
            second_plan.selection_identity
        );
        assert_eq!(
            first_plan.invocation_identity,
            second_plan.invocation_identity
        );
        assert_eq!(
            first_record.federated_validation_plan_id,
            second_record.federated_validation_plan_id
        );
        assert_ne!(
            first_record.application_definition,
            second_record.application_definition
        );
        assert_eq!(
            first_direct.validation_plan_id,
            second_direct.validation_plan_id
        );
    }

    #[test]
    fn validation_plan_selects_package_closure_for_supported_inputs() {
        let changed_path = manifest()
            .parent()
            .expect("workspace root")
            .join("alpha/src/lib.rs");
        let envelope = create_validation_plan(
            &manifest(),
            "ferris.test/simple",
            &[changed_path],
            &["fixture-alpha".to_owned()],
        )
        .expect("validation plan");
        let record = envelope.record.expect("validation plan record");

        assert_eq!(record.schema, VALIDATION_PLAN_SCHEMA);
        assert!(!record.executable);
        assert_eq!(record.inputs.len(), 2);
        assert_eq!(record.selected_packages.len(), 2);
        assert_eq!(
            record
                .selected_packages
                .iter()
                .map(|selection| selection.package.name.as_str())
                .collect::<Vec<_>>(),
            vec!["fixture-alpha", "fixture-beta"]
        );
        assert_eq!(record.selected_activities.len(), 2);
        assert!(!record.fallback.required_by_inputs);
    }

    #[test]
    fn validation_plan_marks_unknown_workspace_paths_for_full_fallback() {
        let changed_path = manifest()
            .parent()
            .expect("workspace root")
            .join("workspace-policy.txt");
        let envelope =
            create_validation_plan(&manifest(), "ferris.test/simple", &[changed_path], &[])
                .expect("validation plan");
        let record = envelope.record.expect("validation plan record");

        assert_eq!(record.inputs.len(), 1);
        assert!(record.selected_packages.is_empty());
        assert!(record.selected_activities.is_empty());
        assert!(record.fallback.required_by_inputs);
        assert_eq!(record.fallback.packages.len(), 2);
        assert!(
            record
                .fallback
                .reasons
                .iter()
                .any(|reason| { reason.contains("workspace-policy.txt") })
        );
    }

    #[test]
    fn validation_plan_rejects_paths_outside_the_selected_workspace() {
        let outside = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
        let error = create_validation_plan(&manifest(), "ferris.test/simple", &[outside], &[])
            .expect_err("outside path should fail");

        assert_eq!(error.result_class(), ResultClass::Invalid);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-VALIDATION-CHANGE-PATH-OUTSIDE-WORKSPACE"
        );
    }

    #[test]
    fn validation_plan_ambiguous_package_roots_widen_without_owner_identity() {
        let manifest_path = fixture("ambiguous-package-roots/Cargo.toml");
        let changed_path = fixture("ambiguous-package-roots/outer/member/src/lib.rs");
        let envelope = create_validation_plan(
            &manifest_path,
            "ferris.test/ambiguous",
            &[changed_path],
            &[],
        )
        .expect("validation plan");
        let record = envelope.record.expect("validation plan record");
        let input = record.inputs.first().expect("input");

        assert_eq!(record.inputs.len(), 1);
        assert_eq!(
            input.disposition,
            ValidationInputDisposition::FullWorkspaceFallback
        );
        assert_eq!(input.package_identity, None);
        assert!(
            input
                .reason
                .contains("more than one workspace package root")
        );
        assert!(record.selected_packages.is_empty());
        assert!(record.selected_activities.is_empty());
        assert!(record.fallback.required_by_inputs);
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
            source: None,
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
    fn validation_plan_identity_is_independent_of_checkout_path_and_evidence_digest() {
        let first_manifest = fixture("validation-plan-checkout/checkout-a/Cargo.toml");
        let second_manifest = fixture("validation-plan-checkout/checkout-b/Cargo.toml");
        let first_changed_path = fixture("validation-plan-checkout/checkout-a/alpha/src/lib.rs");
        let second_changed_path = fixture("validation-plan-checkout/checkout-b/alpha/src/lib.rs");
        let changed_packages = ["fixture-alpha".to_owned()];
        let first = create_validation_plan(
            &first_manifest,
            "ferris.test/portable",
            &[first_changed_path],
            &changed_packages,
        )
        .expect("first validation plan");
        let second = create_validation_plan(
            &second_manifest,
            "ferris.test/portable",
            &[second_changed_path],
            &changed_packages,
        )
        .expect("second validation plan");
        let first_record = first.record.expect("first validation plan record");
        let second_record = second.record.expect("second validation plan record");

        assert_eq!(
            first_record.validation_plan_id,
            second_record.validation_plan_id
        );
        assert_ne!(
            first_record.evidence.owner_output_digest,
            second_record.evidence.owner_output_digest
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
    fn invalid_cli_identity_is_independent_of_executable_and_checkout_paths() {
        let first = vec![
            r"C:\one\ferris.exe".to_owned(),
            "doctor".to_owned(),
            "--workspace-id".to_owned(),
            "ferris.test/simple".to_owned(),
            "--manifest-path".to_owned(),
            r"C:\one\fixture\Cargo.toml".to_owned(),
            "--format".to_owned(),
            "JSON".to_owned(),
            "--unknown=value-a".to_owned(),
        ];
        let second = vec![
            r"D:\two\ferris.exe".to_owned(),
            "doctor".to_owned(),
            "--workspace-id=ferris.test/simple".to_owned(),
            r"--manifest-path=D:\two\fixture\Cargo.toml".to_owned(),
            "--format=json".to_owned(),
            "--unknown=value-a".to_owned(),
        ];

        assert_eq!(
            command_line_invocation_identity("doctor", &first),
            command_line_invocation_identity("doctor", &second)
        );

        let mut changed = second;
        *changed.last_mut().expect("unknown option") = "--unknown=value-b".to_owned();
        assert_ne!(
            command_line_invocation_identity("doctor", &first),
            command_line_invocation_identity("doctor", &changed)
        );
    }

    #[test]
    fn command_result_identity_binds_complete_outcome_and_exit_code() {
        let diagnostic = Diagnostic {
            code: "FERRIS-TEST-INVALID".to_owned(),
            severity: "error".to_owned(),
            result_class: ResultClass::Invalid,
            message: "invalid".to_owned(),
            source_digest: None,
            bounded_output: None,
            next_actions: Vec::new(),
        };
        let first: CommandEnvelope<serde_json::Value> = command_envelope(
            "doctor",
            "selection:test".to_owned(),
            "invocation:test".to_owned(),
            ResultClass::Invalid,
            vec![diagnostic.clone()],
            None,
        );
        let same: CommandEnvelope<serde_json::Value> = command_envelope(
            "doctor",
            "selection:test".to_owned(),
            "invocation:test".to_owned(),
            ResultClass::Invalid,
            vec![diagnostic.clone()],
            None,
        );
        let changed: CommandEnvelope<serde_json::Value> = command_envelope(
            "doctor",
            "selection:test".to_owned(),
            "invocation:test".to_owned(),
            ResultClass::Invalid,
            vec![Diagnostic {
                message: "changed".to_owned(),
                ..diagnostic
            }],
            None,
        );

        assert_eq!(first.result_identity, same.result_identity);
        assert_ne!(first.result_identity, changed.result_identity);
        assert_eq!(first.process_exit_code, first.result_class.exit_code());
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
        assert!(parse_cargo_version(b"cargo 1.2.3 (ABCDEF012 2026-03-21)\n").is_none());
        assert!(parse_cargo_version(b"cargo 1.2.3 (abcdef01 2026-03-21)\n").is_none());
        assert!(parse_cargo_version(b"cargo 1.2.3 (abcdef012 2026-02-30)\n").is_none());

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
    fn owner_output_digest_uses_unambiguous_length_framing() {
        let mut expected_frame = b"ferris.command-output/v1\0".to_vec();
        expected_frame.extend_from_slice(&3_u64.to_le_bytes());
        expected_frame.extend_from_slice(b"out");
        expected_frame.extend_from_slice(&3_u64.to_le_bytes());
        expected_frame.extend_from_slice(b"err");
        assert_eq!(
            digest_command_output(b"out", b"err"),
            digest_bytes(&expected_frame)
        );
        assert_ne!(
            digest_command_output(b"a\0", b"b"),
            digest_command_output(b"a", b"\0b")
        );
        assert_ne!(
            digest_command_output(b"stdout", b""),
            digest_command_output(b"", b"stdout")
        );
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
    fn doctor_identity_hashes_the_complete_typed_record() {
        let mut baseline_record = create_doctor(&manifest(), "ferris.test/one")
            .expect("doctor")
            .record
            .expect("doctor record");
        baseline_record.report_id.clear();
        let baseline = doctor_record_id(&baseline_record).expect("report ID");
        let same = doctor_record_id(&baseline_record).expect("report ID");

        let mut workspace_change = baseline_record.clone();
        workspace_change.workspace_id = "ferris.test/two".to_owned();
        let workspace_change = doctor_record_id(&workspace_change).expect("report ID");

        let mut owner_output_change = baseline_record.clone();
        owner_output_change.evidence.owner_output_digest = "sha256:changed".to_owned();
        let owner_output_change_id = doctor_record_id(&owner_output_change).expect("report ID");

        let mut bound_change = baseline_record.clone();
        bound_change.bounds.stdout_max_bytes += 1;
        let bound_change = doctor_record_id(&bound_change).expect("report ID");

        let mut limitation_change = baseline_record.clone();
        limitation_change.limitations.push("changed".to_owned());
        let limitation_change = doctor_record_id(&limitation_change).expect("report ID");

        assert_eq!(baseline, same);
        assert_ne!(baseline, workspace_change);
        assert_ne!(baseline, owner_output_change_id);
        assert_ne!(baseline, bound_change);
        assert_ne!(baseline, limitation_change);
        let selection =
            doctor_selection_identity("ferris.test/one", &baseline_record.manifest_digest);
        let invocation = doctor_invocation_identity("ferris.test/one", &selection);
        let baseline_envelope = success_envelope(
            "doctor",
            selection.clone(),
            invocation.clone(),
            baseline_record.clone(),
        );
        let changed_envelope =
            success_envelope("doctor", selection, invocation, owner_output_change);
        assert_eq!(
            baseline_envelope.invocation_identity,
            changed_envelope.invocation_identity
        );
        assert_ne!(
            baseline_envelope.result_identity,
            changed_envelope.result_identity
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
        assert_eq!(first.selection_identity, second.selection_identity);

        let changed_error = CoreError::new(
            ResultClass::Blocked,
            "FERRIS-TEST-BLOCKED",
            "blocked",
            Vec::new(),
        )
        .with_source_digest("sha256:changed".to_owned())
        .with_invocation_selection("sha256:manifest".to_owned());
        let changed: CommandEnvelope<serde_json::Value> = doctor_error_envelope(
            "ferris.test/simple",
            Path::new("checkout-a/Cargo.toml"),
            &changed_error,
        );
        assert_eq!(first.invocation_identity, changed.invocation_identity);
        assert_eq!(first.selection_identity, changed.selection_identity);
        assert_ne!(first.result_identity, changed.result_identity);

        let preselection_error = CoreError::new(
            ResultClass::Invalid,
            "FERRIS-TEST-INVALID",
            "invalid",
            Vec::new(),
        );
        let preselection_a: CommandEnvelope<serde_json::Value> = doctor_error_envelope(
            "ferris.test/simple",
            Path::new(r"C:\checkout-a\fixture\Cargo.toml"),
            &preselection_error,
        );
        let preselection_b: CommandEnvelope<serde_json::Value> = doctor_error_envelope(
            "ferris.test/simple",
            Path::new(r"D:\checkout-b\fixture\Cargo.toml"),
            &preselection_error,
        );
        assert_eq!(
            preselection_a.selection_identity,
            preselection_b.selection_identity
        );
        assert_eq!(
            preselection_a.invocation_identity,
            preselection_b.invocation_identity
        );
    }

    #[test]
    fn doctor_success_and_post_read_failure_share_request_identity() {
        let success = create_doctor(&manifest(), "ferris.test/simple").expect("doctor");
        let report = success.record.as_ref().expect("doctor record");
        let error = CoreError::new(
            ResultClass::Blocked,
            "FERRIS-TEST-BLOCKED",
            "blocked",
            Vec::new(),
        )
        .with_invocation_selection(report.manifest_digest.clone());
        let failure: CommandEnvelope<serde_json::Value> =
            doctor_error_envelope("ferris.test/simple", &manifest(), &error);

        assert_eq!(success.selection_identity, failure.selection_identity);
        assert_eq!(success.invocation_identity, failure.invocation_identity);
        assert_ne!(success.result_identity, failure.result_identity);
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
        assert!(
            error
                .invocation_selection()
                .expect("portable oversized selection")
                .starts_with("oversized-manifest-prefix:sha256:")
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

        let BoundedCommandError::Timeout(capture) = error else {
            panic!("expected timeout");
        };
        assert_eq!(
            capture.stdout.retained.len() as u64,
            capture.stdout.observed_bytes
        );
        assert_eq!(
            capture.stderr.retained.len() as u64,
            capture.stderr.observed_bytes
        );
        assert!(!capture.stdout.truncated);
        assert!(!capture.stderr.truncated);
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

        let BoundedCommandError::OutputLimit(capture) = error else {
            panic!("expected output limit");
        };
        assert_eq!(capture.stdout.retained.len(), 1024);
        assert_eq!(capture.stdout.observed_bytes, 1025);
    }

    #[test]
    fn bounded_doctor_failure_retains_typed_output_evidence() {
        let capture = BoundedCapture {
            stdout: CapturedStream {
                retained: b"bounded".to_vec(),
                observed_bytes: 4096,
                complete: false,
                truncated: true,
                failed: false,
            },
            stderr: CapturedStream {
                retained: Vec::new(),
                observed_bytes: 0,
                complete: true,
                truncated: false,
                failed: false,
            },
            termination_cleanup_complete: true,
        };
        let error =
            doctor_command_error(BoundedCommandError::OutputLimit(capture), "sha256:manifest");
        let evidence = error
            .diagnostic()
            .bounded_output
            .as_ref()
            .expect("bounded evidence");

        assert_eq!(evidence.stdout_retained_bytes, 7);
        assert_eq!(evidence.stdout_observed_bytes, 8);
        assert_eq!(evidence.stdout_omitted_observed_bytes, 1);
        assert!(evidence.stdout_unobserved_bytes_unknown);
        assert!(evidence.stdout_truncated);
        assert_eq!(evidence.termination, "output-bound");
        assert_eq!(evidence.termination_scope, "direct-child");
        assert!(evidence.termination_cleanup_complete);
        assert_eq!(
            error.diagnostic().source_digest.as_deref(),
            Some(evidence.output_digest.as_str())
        );
    }

    #[test]
    fn federated_metadata_bounds_and_error_mapping_are_explicit() {
        assert_eq!(FEDERATED_PLAN_METADATA_TIMEOUT, Duration::from_secs(30));
        assert_eq!(MAX_FEDERATED_PLAN_METADATA_OUTPUT_BYTES, 4 * 1024 * 1024);

        let capture = BoundedCapture {
            stdout: CapturedStream {
                retained: b"bounded".to_vec(),
                observed_bytes: 4096,
                complete: false,
                truncated: true,
                failed: false,
            },
            stderr: CapturedStream {
                retained: Vec::new(),
                observed_bytes: 0,
                complete: true,
                truncated: false,
                failed: false,
            },
            termination_cleanup_complete: true,
        };
        let timeout =
            federated_metadata_command_error(BoundedCommandError::Timeout(capture.clone()));
        assert_eq!(timeout.diagnostic().code, "FERRIS-CARGO-METADATA-TIMEOUT");
        assert_eq!(
            timeout
                .diagnostic()
                .bounded_output
                .as_ref()
                .expect("timeout evidence")
                .termination,
            "timeout"
        );

        let output_limit =
            federated_metadata_command_error(BoundedCommandError::OutputLimit(capture));
        assert_eq!(
            output_limit.diagnostic().code,
            "FERRIS-CARGO-METADATA-OUTPUT-BOUND-EXCEEDED"
        );
        assert_eq!(
            output_limit
                .diagnostic()
                .bounded_output
                .as_ref()
                .expect("output-bound evidence")
                .termination,
            "output-bound"
        );

        let start = federated_metadata_command_error(BoundedCommandError::Start(io::Error::new(
            io::ErrorKind::NotFound,
            "cargo unavailable",
        )));
        assert_eq!(start.diagnostic().code, "FERRIS-CARGO-METADATA-UNAVAILABLE");
        assert!(start.diagnostic().bounded_output.is_none());
    }

    #[test]
    fn federated_metadata_allows_only_bounded_trimmed_ascii() {
        assert!(valid_federated_metadata("release owner"));
        assert!(!valid_federated_metadata(" release owner"));
        assert!(!valid_federated_metadata("release owner "));
        assert!(!valid_federated_metadata("release\towner"));
        assert!(!valid_federated_metadata(&"x".repeat(257)));
    }

    #[test]
    #[ignore]
    fn bounded_command_sleep_helper() {
        thread::sleep(Duration::from_secs(30));
    }

    #[test]
    fn revision_skew_classifies_equal_ahead_behind_and_divergent() {
        fn run_git(repository: &Path, arguments: &[&str]) -> String {
            let output = Command::new("git")
                .arg("-C")
                .arg(repository)
                .args(arguments)
                .output()
                .expect("run git");
            assert!(
                output.status.success(),
                "git failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout)
                .expect("git stdout")
                .trim()
                .to_owned()
        }

        let directory = TestDirectory::new("revision-skew-ancestry");
        let repository = directory.path("producer");
        fs::create_dir_all(&repository).expect("create producer");
        run_git(&repository, &["init", "--initial-branch", "main"]);
        run_git(
            &repository,
            &["config", "user.email", "ferris@example.invalid"],
        );
        run_git(&repository, &["config", "user.name", "Ferris Test"]);
        fs::write(repository.join("value.txt"), "a\n").expect("write A");
        run_git(&repository, &["add", "."]);
        run_git(&repository, &["commit", "-m", "A"]);
        let revision_a = run_git(&repository, &["rev-parse", "HEAD"]);

        fs::write(repository.join("value.txt"), "b\n").expect("write B");
        run_git(&repository, &["add", "."]);
        run_git(&repository, &["commit", "-m", "B"]);
        let revision_b = run_git(&repository, &["rev-parse", "HEAD"]);

        run_git(&repository, &["checkout", "-b", "divergent", &revision_a]);
        fs::write(repository.join("value.txt"), "c\n").expect("write C");
        run_git(&repository, &["add", "."]);
        run_git(&repository, &["commit", "-m", "C"]);
        let revision_c = run_git(&repository, &["rev-parse", "HEAD"]);

        let mut reasons = Vec::new();
        assert_eq!(
            classify_revision_skew(&repository, &revision_c, Some(&revision_b), &mut reasons),
            RevisionSkewStatus::Divergent
        );

        run_git(&repository, &["checkout", "--detach", &revision_b]);
        reasons.clear();
        assert_eq!(
            classify_revision_skew(&repository, &revision_b, Some(&revision_a), &mut reasons),
            RevisionSkewStatus::Behind
        );

        run_git(&repository, &["checkout", "--detach", &revision_a]);
        reasons.clear();
        assert_eq!(
            classify_revision_skew(&repository, &revision_a, Some(&revision_b), &mut reasons),
            RevisionSkewStatus::Ahead
        );
        reasons.clear();
        assert_eq!(
            classify_revision_skew(&repository, &revision_a, Some(&revision_a), &mut reasons),
            RevisionSkewStatus::Equal
        );
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
