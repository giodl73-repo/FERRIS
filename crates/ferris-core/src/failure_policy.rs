use super::{
    CARGO_FAILURE_REPORT_SCHEMA, COMMAND_RESULT_SCHEMA, CargoFailureKind, CargoFailureReport,
    CommandEnvelope, CommandResultIdentityInput, CoreError, Diagnostic, ResultClass,
    StrictJsonValue, cargo_failure_report_is_valid, digest_bytes, invocation_identity, record_id,
    selection_identity, success_envelope, valid_portable_id,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const FAILURE_POLICY_SCHEMA: &str = "ferris.failure-policy/v1";
pub const FAILURE_POLICY_DECISION_SCHEMA: &str = "ferris.failure-policy-decision/v1";
pub const MAX_FAILURE_POLICY_INPUT_BYTES: u64 = 64 * 1024;
pub const MAX_FAILURE_DIAGNOSIS_INPUT_BYTES: u64 = 128 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureDisposition {
    Halt,
    Route,
    PrepareAction,
}

impl FailureDisposition {
    fn name(self) -> &'static str {
        match self {
            Self::Halt => "halt",
            Self::Route => "route",
            Self::PrepareAction => "prepare_action",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FailurePolicyResponse {
    pub disposition: FailureDisposition,
    pub owner_action_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FailurePolicyRule {
    pub rule_id: String,
    pub classification: CargoFailureKind,
    pub response: FailurePolicyResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FailurePolicy {
    pub schema: String,
    pub policy_id: String,
    pub rules: Vec<FailurePolicyRule>,
    pub fallback: FailurePolicyResponse,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FailurePolicyDecision {
    pub schema: String,
    pub decision_id: String,
    pub policy_id: String,
    pub policy_digest: String,
    pub failure_report_id: String,
    pub classification: CargoFailureKind,
    pub diagnostic_code: String,
    pub matched: bool,
    pub matched_rule_id: Option<String>,
    pub disposition: FailureDisposition,
    pub owner_action_id: String,
    pub executable: bool,
    pub action_plan_created: bool,
    pub approval_granted: bool,
    pub limitations: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CargoFailureCommandResult {
    schema: String,
    command_version: String,
    semantic_command_id: String,
    selection_identity: String,
    invocation_identity: String,
    result_identity: String,
    result_class: ResultClass,
    process_exit_code: u8,
    diagnostics: Vec<Diagnostic>,
    record: Option<CargoFailureReport>,
}

#[derive(Serialize)]
struct FailurePolicyDecisionIdentity<'a> {
    schema: &'a str,
    policy_id: &'a str,
    policy_digest: &'a str,
    failure_report_id: &'a str,
    classification: CargoFailureKind,
    diagnostic_code: &'a str,
    matched: bool,
    matched_rule_id: &'a Option<String>,
    disposition: FailureDisposition,
    owner_action_id: &'a str,
    executable: bool,
    action_plan_created: bool,
    approval_granted: bool,
    limitations: &'a [String],
}

pub fn evaluate_failure_policy(
    policy_path: &Path,
    diagnosis_path: &Path,
) -> Result<CommandEnvelope<FailurePolicyDecision>, CoreError> {
    let diagnosis = File::open(diagnosis_path).map_err(|_| {
        failure_policy_error(
            "FERRIS-FAILURE-DIAGNOSIS-UNAVAILABLE",
            "The explicit Cargo failure diagnosis input is unavailable.",
        )
    })?;
    evaluate_failure_policy_from_reader(policy_path, diagnosis)
}

pub fn evaluate_failure_policy_from_reader(
    policy_path: &Path,
    diagnosis: impl Read,
) -> Result<CommandEnvelope<FailurePolicyDecision>, CoreError> {
    let policy_file = File::open(policy_path).map_err(|_| {
        failure_policy_error(
            "FERRIS-FAILURE-POLICY-UNAVAILABLE",
            "The explicit failure policy input is unavailable.",
        )
    })?;
    let policy_bytes = read_bounded(
        policy_file,
        MAX_FAILURE_POLICY_INPUT_BYTES,
        "FERRIS-FAILURE-POLICY-INVALID",
        "The explicit failure policy input is empty or exceeds the 64 KiB bound.",
    )?;
    let diagnosis_bytes = read_bounded(
        diagnosis,
        MAX_FAILURE_DIAGNOSIS_INPUT_BYTES,
        "FERRIS-FAILURE-DIAGNOSIS-INVALID",
        "The explicit Cargo failure diagnosis is empty or exceeds the 128 KiB bound.",
    )?;
    let policy: FailurePolicy = parse_strict(
        &policy_bytes,
        "FERRIS-FAILURE-POLICY-INVALID",
        "The explicit failure policy does not match its strict schema.",
    )?;
    validate_policy(&policy)?;
    let diagnosis: CargoFailureCommandResult = parse_strict(
        &diagnosis_bytes,
        "FERRIS-FAILURE-DIAGNOSIS-INVALID",
        "The explicit Cargo failure diagnosis does not match its strict schema.",
    )?;
    let report = validate_diagnosis(&diagnosis)?;

    let matched_rule = policy
        .rules
        .iter()
        .find(|rule| rule.classification == report.classification);
    let (matched, matched_rule_id, response) = match matched_rule {
        Some(rule) => (true, Some(rule.rule_id.clone()), &rule.response),
        None => (false, None, &policy.fallback),
    };
    let policy_digest = digest_bytes(
        &serde_json::to_vec(&policy).expect("validated failure policy must serialize"),
    );
    let limitations = vec![
        "This decision matches only the supplied typed Cargo failure classification; it does not inspect raw stderr or infer root cause."
            .to_owned(),
        "The owner action identity is opaque and was not resolved to a command, entrypoint, or environment."
            .to_owned(),
        "No Action Plan, approval, retry, or owner command was created or executed.".to_owned(),
    ];
    let identity = FailurePolicyDecisionIdentity {
        schema: FAILURE_POLICY_DECISION_SCHEMA,
        policy_id: &policy.policy_id,
        policy_digest: &policy_digest,
        failure_report_id: &report.report_id,
        classification: report.classification,
        diagnostic_code: &report.diagnostic_code,
        matched,
        matched_rule_id: &matched_rule_id,
        disposition: response.disposition,
        owner_action_id: &response.owner_action_id,
        executable: false,
        action_plan_created: false,
        approval_granted: false,
        limitations: &limitations,
    };
    let decision_id = record_id("failure-policy-decision", &identity)?;
    let decision = FailurePolicyDecision {
        schema: FAILURE_POLICY_DECISION_SCHEMA.to_owned(),
        decision_id: decision_id.clone(),
        policy_id: policy.policy_id,
        policy_digest: policy_digest.clone(),
        failure_report_id: report.report_id.clone(),
        classification: report.classification,
        diagnostic_code: report.diagnostic_code.clone(),
        matched,
        matched_rule_id,
        disposition: response.disposition,
        owner_action_id: response.owner_action_id.clone(),
        executable: false,
        action_plan_created: false,
        approval_granted: false,
        limitations,
    };
    Ok(success_envelope(
        "failure-policy",
        selection_identity("failure-policy", &decision_id),
        invocation_identity(&[
            "failure-policy",
            &policy_digest,
            &report.report_id,
            "execution=false",
            "action-plan-created=false",
            "approval-granted=false",
        ]),
        decision,
    ))
}

pub fn render_failure_policy_human(envelope: &CommandEnvelope<FailurePolicyDecision>) -> String {
    let decision = envelope
        .record
        .as_ref()
        .expect("successful failure policy evaluation has a decision");
    let rule = decision.matched_rule_id.as_deref().unwrap_or("fallback");
    let mut output = format!(
        "Ferris failure policy decision {}\nPolicy: {}\nFailure report: {}\nClassification: {}\nDiagnostic code: {}\nMatched: {}\nRule: {}\nDisposition: {}\nOwner action: {}\nExecutable: no\nAction Plan created: false\nApproval granted: false\nLimitations:\n",
        decision.decision_id,
        decision.policy_id,
        decision.failure_report_id,
        decision.classification.name(),
        decision.diagnostic_code,
        decision.matched,
        rule,
        decision.disposition.name(),
        decision.owner_action_id,
    );
    for limitation in &decision.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

fn validate_policy(policy: &FailurePolicy) -> Result<(), CoreError> {
    if policy.schema != FAILURE_POLICY_SCHEMA {
        return Err(failure_policy_error(
            "FERRIS-FAILURE-POLICY-INVALID",
            "The explicit failure policy schema is unsupported.",
        ));
    }
    if !valid_portable_id(&policy.policy_id)
        || policy.rules.is_empty()
        || policy.rules.len() > 4
        || !valid_response(&policy.fallback)
    {
        return Err(failure_policy_error(
            "FERRIS-FAILURE-POLICY-INVALID",
            "The explicit failure policy contains invalid bounds or identifiers.",
        ));
    }
    let mut rule_ids = BTreeSet::new();
    let mut classifications = BTreeSet::new();
    for rule in &policy.rules {
        if !valid_portable_id(&rule.rule_id)
            || !valid_response(&rule.response)
            || !rule_ids.insert(&rule.rule_id)
            || !classifications.insert(rule.classification)
        {
            return Err(failure_policy_error(
                "FERRIS-FAILURE-POLICY-INVALID",
                "Failure policy rules require unique classifications and stable identifiers.",
            ));
        }
    }
    Ok(())
}

fn valid_response(response: &FailurePolicyResponse) -> bool {
    valid_portable_id(&response.owner_action_id)
}

fn validate_diagnosis(
    diagnosis: &CargoFailureCommandResult,
) -> Result<&CargoFailureReport, CoreError> {
    let Some(report) = diagnosis.record.as_ref() else {
        return Err(invalid_diagnosis());
    };
    let expected_selection = selection_identity("diagnose-cargo", &report.input_digest);
    let expected_invocation = invocation_identity(&[
        "diagnose-cargo",
        &report.input_digest,
        "input=caller-supplied-cargo-stderr",
        "input-max-bytes=65536",
        "execution=false",
        "raw-output-retained=false",
    ]);
    let valid_envelope = diagnosis.schema == COMMAND_RESULT_SCHEMA
        && diagnosis.command_version == env!("CARGO_PKG_VERSION")
        && diagnosis.semantic_command_id == "diagnose-cargo"
        && diagnosis.selection_identity == expected_selection
        && diagnosis.invocation_identity == expected_invocation
        && diagnosis.result_class == ResultClass::Success
        && diagnosis.process_exit_code == 0
        && diagnosis.diagnostics.is_empty();
    if !valid_envelope
        || report.schema != CARGO_FAILURE_REPORT_SCHEMA
        || !cargo_failure_report_is_valid(report)
    {
        return Err(invalid_diagnosis());
    }
    let expected_result_id = record_id(
        "result",
        &CommandResultIdentityInput {
            schema: &diagnosis.schema,
            command_version: &diagnosis.command_version,
            semantic_command_id: &diagnosis.semantic_command_id,
            selection_identity: &diagnosis.selection_identity,
            invocation_identity: &diagnosis.invocation_identity,
            result_class: diagnosis.result_class,
            process_exit_code: diagnosis.process_exit_code,
            diagnostics: &diagnosis.diagnostics,
            record: &diagnosis.record,
        },
    )?;
    if expected_result_id != diagnosis.result_identity {
        return Err(invalid_diagnosis());
    }
    Ok(report)
}

fn read_bounded(
    reader: impl Read,
    bound: u64,
    code: &str,
    message: &str,
) -> Result<Vec<u8>, CoreError> {
    let mut bytes = Vec::new();
    reader
        .take(bound + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| failure_policy_error(code, message))?;
    if bytes.is_empty() || bytes.len() as u64 > bound {
        return Err(failure_policy_error(code, message));
    }
    Ok(bytes)
}

fn parse_strict<T: DeserializeOwned>(
    bytes: &[u8],
    code: &str,
    message: &str,
) -> Result<T, CoreError> {
    let value = serde_json::from_slice::<StrictJsonValue>(bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| failure_policy_error(code, message))?;
    serde_json::from_value(value).map_err(|_| failure_policy_error(code, message))
}

fn invalid_diagnosis() -> CoreError {
    failure_policy_error(
        "FERRIS-FAILURE-DIAGNOSIS-INVALID",
        "The explicit Cargo failure diagnosis is not a valid successful Ferris diagnosis.",
    )
}

fn failure_policy_error(code: &str, message: &str) -> CoreError {
    CoreError::new(
        ResultClass::Invalid,
        code,
        message,
        vec![
            "Provide one bounded strict V1 failure policy and one complete diagnose-cargo JSON result."
                .to_owned(),
        ],
    )
}
