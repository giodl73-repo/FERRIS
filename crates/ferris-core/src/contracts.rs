use super::{
    ACTION_PLAN_LANES_SCHEMA, ACTION_PLAN_SCHEMA, APPLICATION_READINESS_REPORT_SCHEMA,
    APPLICATION_READINESS_REQUEST_SCHEMA, APPLICATION_SCHEMA, ARTIFACT_QUALIFICATION_REPORT_SCHEMA,
    ARTIFACT_REUSE_REPORT_SCHEMA, ARTIFACT_REUSE_REQUEST_SCHEMA, BOUNDED_OUTPUT_EVIDENCE_SCHEMA,
    CARGO_FAILURE_REPORT_SCHEMA, COMMAND_RESULT_SCHEMA, CommandEnvelope, DOCTOR_SCHEMA, Diagnostic,
    ENVIRONMENT_READINESS_REPORT_SCHEMA, ENVIRONMENT_REQUIREMENTS_SCHEMA,
    EXECUTION_APPROVAL_SCHEMA, EXECUTION_RECEIPT_SCHEMA, EXECUTION_VERIFICATION_SCHEMA,
    EXPLANATION_SCHEMA, FAILURE_POLICY_DECISION_SCHEMA, FAILURE_POLICY_SCHEMA,
    FEDERATED_PLAN_REQUEST_SCHEMA, FEDERATED_PLAN_SCHEMA, FEDERATED_VALIDATION_PLAN_SCHEMA,
    GRAPH_SCHEMA, ITERATION_REPLAY_REPORT_SCHEMA, ITERATION_REPLAY_REQUEST_SCHEMA,
    LEGACY_EXECUTION_RECEIPT_SCHEMA, OWNER_ENTRYPOINTS_SCHEMA, OWNER_VALIDATION_DOMAINS_SCHEMA,
    OWNER_VALIDATION_DOMAINS_V2_SCHEMA, PATH_AUTHORITY_SCHEMA, PLAN_SCHEMA, PROFILE_DIFF_SCHEMA,
    PROFILE_EVIDENCE_SCHEMA, REMOTE_ITERATION_EVIDENCE_SCHEMA, REVISION_SKEW_REPORT_SCHEMA,
    REVISION_SKEW_REQUEST_SCHEMA, ROOT_QUALIFIED_PLAN_SCHEMA, ResultClass,
    SCHEDULE_REPLAY_REPORT_SCHEMA, SCHEDULE_REPLAY_REQUEST_SCHEMA, StrictJsonValue,
    VALIDATION_DECLARATION_SCHEMA, VALIDATION_OBSERVATION_SCHEMA, VALIDATION_PLAN_SCHEMA,
    VALIDATION_REVISION_BINDING_SCHEMA, VALIDATION_TOPOLOGY_PLAN_SCHEMA, command_envelope,
    digest_bytes, invocation_identity, selection_identity, success_envelope,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const CONTRACT_CATALOG_SCHEMA: &str = "ferris.contract-catalog/v1";
pub const CONTRACT_REQUIREMENTS_SCHEMA: &str = "ferris.contract-requirements/v1";
pub const CONTRACT_COMPATIBILITY_REPORT_SCHEMA: &str = "ferris.contract-compatibility-report/v1";

const MAX_CONTRACT_REQUIREMENTS_BYTES: u64 = 64 * 1024;
const MAX_CONTRACT_REQUIREMENTS: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractLifecycle {
    Experimental,
    Incubating,
    LegacyReadOnly,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractHandling {
    pub schema: String,
    pub accepted: bool,
    pub emitted: bool,
    pub lifecycle: ContractLifecycle,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractCatalog {
    pub schema: String,
    pub catalog_id: String,
    pub command_version: String,
    pub product_status: String,
    pub compatibility_policy: String,
    pub unknown_schema_policy: String,
    pub contracts: Vec<ContractHandling>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredContractHandling {
    Accepted,
    Emitted,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractRequirement {
    pub schema: String,
    pub handling: RequiredContractHandling,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractRequirements {
    pub schema: String,
    pub consumer_id: String,
    pub requirements: Vec<ContractRequirement>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractRequirementResult {
    pub schema: String,
    pub handling: RequiredContractHandling,
    pub satisfied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<ContractLifecycle>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractCompatibilityReport {
    pub schema: String,
    pub report_id: String,
    pub requirement_set_id: String,
    pub catalog_id: String,
    pub consumer_id: String,
    pub compatible: bool,
    pub requirements: Vec<ContractRequirementResult>,
    pub unsatisfied_count: usize,
    pub limitations: Vec<String>,
}

#[derive(Serialize)]
struct ContractRequirementsIdentity<'a> {
    schema: &'a str,
    consumer_id: &'a str,
    requirements: &'a [ContractRequirement],
}

#[derive(Serialize)]
struct ContractCompatibilityIdentity<'a> {
    schema: &'a str,
    requirement_set_id: &'a str,
    catalog_id: &'a str,
    consumer_id: &'a str,
    compatible: bool,
    requirements: &'a [ContractRequirementResult],
    unsatisfied_count: usize,
    limitations: &'a [String],
}

#[derive(Serialize)]
struct ContractCatalogIdentity<'a> {
    schema: &'a str,
    command_version: &'a str,
    product_status: &'a str,
    compatibility_policy: &'a str,
    unknown_schema_policy: &'a str,
    contracts: &'a [ContractHandling],
    limitations: &'a [String],
}

pub fn create_contract_catalog() -> CommandEnvelope<ContractCatalog> {
    let mut contracts = vec![
        support(ACTION_PLAN_LANES_SCHEMA, true, false),
        support(ACTION_PLAN_SCHEMA, true, true),
        support(APPLICATION_READINESS_REPORT_SCHEMA, false, true),
        support(APPLICATION_READINESS_REQUEST_SCHEMA, true, true),
        support(APPLICATION_SCHEMA, true, false),
        support(ARTIFACT_QUALIFICATION_REPORT_SCHEMA, false, true),
        support(ARTIFACT_REUSE_REPORT_SCHEMA, false, true),
        support(ARTIFACT_REUSE_REQUEST_SCHEMA, true, false),
        support(BOUNDED_OUTPUT_EVIDENCE_SCHEMA, false, true),
        support(CARGO_FAILURE_REPORT_SCHEMA, false, true),
        support(COMMAND_RESULT_SCHEMA, false, true),
        support(CONTRACT_CATALOG_SCHEMA, false, true),
        support(CONTRACT_COMPATIBILITY_REPORT_SCHEMA, false, true),
        support(CONTRACT_REQUIREMENTS_SCHEMA, true, false),
        support(DOCTOR_SCHEMA, false, true),
        support(ENVIRONMENT_READINESS_REPORT_SCHEMA, true, true),
        support(ENVIRONMENT_REQUIREMENTS_SCHEMA, true, false),
        support(EXECUTION_APPROVAL_SCHEMA, true, false),
        support(EXECUTION_RECEIPT_SCHEMA, true, true),
        support(EXECUTION_VERIFICATION_SCHEMA, false, true),
        support(EXPLANATION_SCHEMA, false, true),
        support(FAILURE_POLICY_DECISION_SCHEMA, false, true),
        support(FAILURE_POLICY_SCHEMA, true, false),
        support(FEDERATED_PLAN_REQUEST_SCHEMA, true, false),
        support(FEDERATED_PLAN_SCHEMA, false, true),
        support(FEDERATED_VALIDATION_PLAN_SCHEMA, false, true),
        support(GRAPH_SCHEMA, false, true),
        support(ITERATION_REPLAY_REPORT_SCHEMA, false, true),
        support(ITERATION_REPLAY_REQUEST_SCHEMA, true, false),
        legacy_support(LEGACY_EXECUTION_RECEIPT_SCHEMA),
        support(OWNER_ENTRYPOINTS_SCHEMA, true, false),
        support(OWNER_VALIDATION_DOMAINS_SCHEMA, true, false),
        support(OWNER_VALIDATION_DOMAINS_V2_SCHEMA, true, false),
        support(PATH_AUTHORITY_SCHEMA, true, false),
        support(PLAN_SCHEMA, false, true),
        support(PROFILE_DIFF_SCHEMA, false, true),
        support(PROFILE_EVIDENCE_SCHEMA, true, false),
        support(REMOTE_ITERATION_EVIDENCE_SCHEMA, true, false),
        support(REVISION_SKEW_REPORT_SCHEMA, false, true),
        support(REVISION_SKEW_REQUEST_SCHEMA, true, false),
        support(ROOT_QUALIFIED_PLAN_SCHEMA, false, true),
        support(SCHEDULE_REPLAY_REPORT_SCHEMA, false, true),
        support(SCHEDULE_REPLAY_REQUEST_SCHEMA, true, false),
        support(VALIDATION_DECLARATION_SCHEMA, true, false),
        support(VALIDATION_OBSERVATION_SCHEMA, true, false),
        support(VALIDATION_PLAN_SCHEMA, false, true),
        support(VALIDATION_REVISION_BINDING_SCHEMA, false, true),
        support(VALIDATION_TOPOLOGY_PLAN_SCHEMA, false, true),
    ];
    contracts.sort_by(|left, right| left.schema.cmp(&right.schema));

    let command_version = env!("CARGO_PKG_VERSION").to_owned();
    let product_status = "incubation".to_owned();
    let compatibility_policy = "exact-schema-identifier".to_owned();
    let unknown_schema_policy = "reject".to_owned();
    let limitations = vec![
        "This catalog reports implemented local schema handling, not production support."
            .to_owned(),
        "Schema recognition does not establish semantic compatibility between different versions."
            .to_owned(),
        "Ferris does not negotiate, migrate, fetch, approve, or execute work from this catalog."
            .to_owned(),
    ];
    let identity = ContractCatalogIdentity {
        schema: CONTRACT_CATALOG_SCHEMA,
        command_version: &command_version,
        product_status: &product_status,
        compatibility_policy: &compatibility_policy,
        unknown_schema_policy: &unknown_schema_policy,
        contracts: &contracts,
        limitations: &limitations,
    };
    let catalog_id = format!(
        "contract-catalog:{}",
        digest_bytes(
            &serde_json::to_vec(&identity)
                .expect("typed Ferris contract catalog identity must serialize")
        )
        .trim_start_matches("sha256:")
    );
    let catalog = ContractCatalog {
        schema: CONTRACT_CATALOG_SCHEMA.to_owned(),
        catalog_id: catalog_id.clone(),
        command_version,
        product_status,
        compatibility_policy,
        unknown_schema_policy,
        contracts,
        limitations,
    };
    success_envelope(
        "contracts",
        selection_identity("contracts", &catalog_id),
        invocation_identity(&["contracts", &catalog_id]),
        catalog,
    )
}

pub fn create_contract_compatibility_report(
    requirements_path: &Path,
) -> Result<CommandEnvelope<ContractCompatibilityReport>, super::CoreError> {
    let bytes = read_contract_requirements(requirements_path)?;
    let value = serde_json::from_slice::<StrictJsonValue>(&bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| {
            contract_requirements_error(
                "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
                "The contract requirements input is not strict JSON.",
            )
        })?;
    let mut request = serde_json::from_value::<ContractRequirements>(value).map_err(|_| {
        contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
            "The contract requirements input does not match the required shape.",
        )
    })?;
    validate_contract_requirements(&request)?;
    request.requirements.sort();

    let requirement_set_id = format!(
        "contract-requirements:{}",
        digest_serializable(&ContractRequirementsIdentity {
            schema: CONTRACT_REQUIREMENTS_SCHEMA,
            consumer_id: &request.consumer_id,
            requirements: &request.requirements,
        })
    );
    let catalog = create_contract_catalog()
        .record
        .expect("successful contract catalog has a record");
    let requirements = request
        .requirements
        .iter()
        .map(|required| {
            let observed = catalog
                .contracts
                .iter()
                .find(|contract| contract.schema == required.schema);
            let satisfied = observed.is_some_and(|contract| match required.handling {
                RequiredContractHandling::Accepted => contract.accepted,
                RequiredContractHandling::Emitted => contract.emitted,
            });
            ContractRequirementResult {
                schema: required.schema.clone(),
                handling: required.handling,
                satisfied,
                lifecycle: observed.map(|contract| contract.lifecycle),
            }
        })
        .collect::<Vec<_>>();
    let unsatisfied_count = requirements
        .iter()
        .filter(|requirement| !requirement.satisfied)
        .count();
    let compatible = unsatisfied_count == 0;
    let limitations = vec![
        "This report checks exact installed-binary schema handling only; it does not compare record semantics."
            .to_owned(),
        "A compatible result is not a support-duration, migration, correctness, or production-readiness claim."
            .to_owned(),
    ];
    let identity = ContractCompatibilityIdentity {
        schema: CONTRACT_COMPATIBILITY_REPORT_SCHEMA,
        requirement_set_id: &requirement_set_id,
        catalog_id: &catalog.catalog_id,
        consumer_id: &request.consumer_id,
        compatible,
        requirements: &requirements,
        unsatisfied_count,
        limitations: &limitations,
    };
    let report_id = format!("contract-compatibility:{}", digest_serializable(&identity));
    let report = ContractCompatibilityReport {
        schema: CONTRACT_COMPATIBILITY_REPORT_SCHEMA.to_owned(),
        report_id,
        requirement_set_id: requirement_set_id.clone(),
        catalog_id: catalog.catalog_id.clone(),
        consumer_id: request.consumer_id,
        compatible,
        requirements,
        unsatisfied_count,
        limitations,
    };
    let result_class = if compatible {
        ResultClass::Success
    } else {
        ResultClass::Difference
    };
    let diagnostics = if compatible {
        Vec::new()
    } else {
        vec![Diagnostic {
            code: "FERRIS-CONTRACT-REQUIREMENTS-UNSATISFIED".to_owned(),
            severity: "error".to_owned(),
            result_class,
            message: format!(
                "The installed Ferris binary does not satisfy {unsatisfied_count} explicit contract requirement(s)."
            ),
            source_digest: None,
            bounded_output: None,
            next_actions: vec![
                "Inspect the report requirements with satisfied=false.".to_owned(),
                "Use a Ferris binary whose exact catalog satisfies the adopter requirement set."
                    .to_owned(),
            ],
        }]
    };
    Ok(command_envelope(
        "contracts",
        selection_identity("contracts", &requirement_set_id),
        invocation_identity(&["contracts", &requirement_set_id, &catalog.catalog_id]),
        result_class,
        diagnostics,
        Some(report),
    ))
}

pub fn render_contract_catalog_human(envelope: &CommandEnvelope<ContractCatalog>) -> String {
    let catalog = envelope
        .record
        .as_ref()
        .expect("successful contract catalog envelope has a record");
    let mut output = format!(
        "Ferris contract catalog {}\nCommand version: {}\nProduct status: {}\nCompatibility: {}\nUnknown schemas: {}\nContracts:\n",
        catalog.catalog_id,
        catalog.command_version,
        catalog.product_status,
        catalog.compatibility_policy,
        catalog.unknown_schema_policy,
    );
    for contract in &catalog.contracts {
        let direction = match (contract.accepted, contract.emitted) {
            (true, true) => "accepted, emitted",
            (true, false) => "accepted",
            (false, true) => "emitted",
            (false, false) => "unavailable",
        };
        output.push_str(&format!(
            "  - {} ({direction}; {})\n",
            contract.schema,
            lifecycle_name(contract.lifecycle)
        ));
    }
    output.push_str("Limitations:\n");
    for limitation in &catalog.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

pub fn render_contract_compatibility_human(
    envelope: &CommandEnvelope<ContractCompatibilityReport>,
) -> String {
    let report = envelope
        .record
        .as_ref()
        .expect("contract compatibility result has a report");
    let mut output = format!(
        "Ferris contract compatibility {}\nConsumer: {}\nCatalog: {}\nCompatible: {}\nRequirements:\n",
        report.report_id, report.consumer_id, report.catalog_id, report.compatible,
    );
    for requirement in &report.requirements {
        let lifecycle = requirement
            .lifecycle
            .map(lifecycle_name)
            .unwrap_or("unknown");
        output.push_str(&format!(
            "  - {} ({}, satisfied={}, lifecycle={})\n",
            requirement.schema,
            required_handling_name(requirement.handling),
            requirement.satisfied,
            lifecycle,
        ));
    }
    output.push_str("Limitations:\n");
    for limitation in &report.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

fn validate_contract_requirements(request: &ContractRequirements) -> Result<(), super::CoreError> {
    if request.schema != CONTRACT_REQUIREMENTS_SCHEMA {
        return Err(contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-SCHEMA-UNSUPPORTED",
            format!("Use schema {CONTRACT_REQUIREMENTS_SCHEMA}."),
        ));
    }
    if !valid_consumer_id(&request.consumer_id) {
        return Err(contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
            "The contract requirements consumer_id is invalid.",
        ));
    }
    if request.requirements.is_empty() || request.requirements.len() > MAX_CONTRACT_REQUIREMENTS {
        return Err(contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
            "The contract requirements count is outside the supported bound of 1 through 128.",
        ));
    }
    if request
        .requirements
        .iter()
        .any(|requirement| !valid_schema_identifier(&requirement.schema))
    {
        return Err(contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
            "A required contract schema identifier is invalid.",
        ));
    }
    let mut normalized = request.requirements.clone();
    normalized.sort();
    if normalized.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
            "The contract requirements contain a duplicate schema and handling pair.",
        ));
    }
    Ok(())
}

fn read_contract_requirements(path: &Path) -> Result<Vec<u8>, super::CoreError> {
    let file = File::open(path).map_err(|_| {
        contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-UNAVAILABLE",
            "The contract requirements input is unavailable.",
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_CONTRACT_REQUIREMENTS_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            contract_requirements_error(
                "FERRIS-CONTRACT-REQUIREMENTS-UNAVAILABLE",
                "The contract requirements input could not be read.",
            )
        })?;
    if bytes.len() as u64 > MAX_CONTRACT_REQUIREMENTS_BYTES {
        return Err(contract_requirements_error(
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID",
            "The contract requirements input exceeds the 64 KiB bound.",
        ));
    }
    Ok(bytes)
}

fn contract_requirements_error(code: &str, message: impl Into<String>) -> super::CoreError {
    super::CoreError::new(
        ResultClass::Invalid,
        code,
        message,
        vec!["Provide a strict ferris.contract-requirements/v1 JSON record and retry.".to_owned()],
    )
}

fn valid_consumer_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
}

fn valid_schema_identifier(value: &str) -> bool {
    let Some((name, version)) = value.rsplit_once("/v") else {
        return false;
    };
    name.strip_prefix("ferris.").is_some_and(|name| {
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    }) && !version.is_empty()
        && version.bytes().all(|byte| byte.is_ascii_digit())
        && value.len() <= 128
}

fn digest_serializable(value: &impl Serialize) -> String {
    digest_bytes(&serde_json::to_vec(value).expect("typed Ferris contract record must serialize"))
        .trim_start_matches("sha256:")
        .to_owned()
}

fn required_handling_name(handling: RequiredContractHandling) -> &'static str {
    match handling {
        RequiredContractHandling::Accepted => "accepted",
        RequiredContractHandling::Emitted => "emitted",
    }
}

fn support(schema: &str, accepted: bool, emitted: bool) -> ContractHandling {
    ContractHandling {
        schema: schema.to_owned(),
        accepted,
        emitted,
        lifecycle: if schema.ends_with("/v0") {
            ContractLifecycle::Experimental
        } else {
            ContractLifecycle::Incubating
        },
    }
}

fn legacy_support(schema: &str) -> ContractHandling {
    ContractHandling {
        schema: schema.to_owned(),
        accepted: true,
        emitted: false,
        lifecycle: ContractLifecycle::LegacyReadOnly,
    }
}

fn lifecycle_name(lifecycle: ContractLifecycle) -> &'static str {
    match lifecycle {
        ContractLifecycle::Experimental => "experimental",
        ContractLifecycle::Incubating => "incubating",
        ContractLifecycle::LegacyReadOnly => "legacy read-only",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_FILE_ID: AtomicU64 = AtomicU64::new(0);

    fn requirements_bytes_file(bytes: &[u8]) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "ferris-contract-requirements-{}-{}.json",
            std::process::id(),
            TEMP_FILE_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::write(&path, bytes).expect("write requirements");
        path
    }

    fn requirements_file(value: serde_json::Value) -> std::path::PathBuf {
        requirements_bytes_file(&serde_json::to_vec_pretty(&value).expect("serialize requirements"))
    }

    #[test]
    fn catalog_is_sorted_unique_and_explicit_about_legacy_receipts() {
        let envelope = create_contract_catalog();
        let catalog = envelope.record.expect("catalog");
        let schemas = catalog
            .contracts
            .iter()
            .map(|contract| contract.schema.as_str())
            .collect::<Vec<_>>();
        assert!(schemas.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(
            catalog
                .contracts
                .iter()
                .all(|contract| contract.accepted || contract.emitted)
        );
        let legacy = catalog
            .contracts
            .iter()
            .find(|contract| contract.schema == LEGACY_EXECUTION_RECEIPT_SCHEMA)
            .expect("legacy receipt support");
        assert!(legacy.accepted);
        assert!(!legacy.emitted);
        assert_eq!(legacy.lifecycle, ContractLifecycle::LegacyReadOnly);
    }

    #[test]
    fn catalog_and_rendering_are_deterministic() {
        let first = create_contract_catalog();
        let second = create_contract_catalog();
        assert_eq!(first.result_identity, second.result_identity);
        assert_eq!(first.record, second.record);
        let human = render_contract_catalog_human(&first);
        assert!(human.contains(CONTRACT_CATALOG_SCHEMA));
        assert!(human.contains("ferris.execution-receipt/v1 (accepted; legacy read-only)"));
        assert!(human.contains("not production support"));
    }

    #[test]
    fn compatibility_report_normalizes_and_satisfies_explicit_requirements() {
        let path = requirements_file(serde_json::json!({
            "schema": CONTRACT_REQUIREMENTS_SCHEMA,
            "consumer_id": "synthetic/corpus",
            "requirements": [
                {"schema": VALIDATION_PLAN_SCHEMA, "handling": "emitted"},
                {"schema": OWNER_VALIDATION_DOMAINS_SCHEMA, "handling": "accepted"},
                {"schema": APPLICATION_SCHEMA, "handling": "accepted"}
            ]
        }));
        let envelope = create_contract_compatibility_report(&path).expect("compatibility report");
        fs::remove_file(path).expect("remove requirements");

        let reordered_path = requirements_file(serde_json::json!({
            "schema": CONTRACT_REQUIREMENTS_SCHEMA,
            "consumer_id": "synthetic/corpus",
            "requirements": [
                {"schema": APPLICATION_SCHEMA, "handling": "accepted"},
                {"schema": OWNER_VALIDATION_DOMAINS_SCHEMA, "handling": "accepted"},
                {"schema": VALIDATION_PLAN_SCHEMA, "handling": "emitted"}
            ]
        }));
        let reordered =
            create_contract_compatibility_report(&reordered_path).expect("reordered report");
        fs::remove_file(reordered_path).expect("remove reordered requirements");

        assert_eq!(envelope.result_class, ResultClass::Success);
        assert_eq!(envelope.result_identity, reordered.result_identity);
        let report = envelope.record.expect("report");
        assert!(report.compatible);
        assert_eq!(report.unsatisfied_count, 0);
        assert!(
            report
                .requirements
                .windows(2)
                .all(|pair| pair[0].schema <= pair[1].schema)
        );
        assert!(report.requirements.iter().all(|item| item.satisfied));
    }

    #[test]
    fn compatibility_report_retains_known_and_unknown_unsatisfied_requirements() {
        let path = requirements_file(serde_json::json!({
            "schema": CONTRACT_REQUIREMENTS_SCHEMA,
            "consumer_id": "upgrade-gate",
            "requirements": [
                {"schema": APPLICATION_READINESS_REPORT_SCHEMA, "handling": "accepted"},
                {"schema": "ferris.future-record/v1", "handling": "emitted"}
            ]
        }));
        let envelope = create_contract_compatibility_report(&path).expect("difference report");
        fs::remove_file(path).expect("remove requirements");

        assert_eq!(envelope.result_class, ResultClass::Difference);
        assert_eq!(envelope.process_exit_code, 1);
        assert_eq!(envelope.diagnostics.len(), 1);
        let human = render_contract_compatibility_human(&envelope);
        assert!(human.contains("Compatible: false"));
        assert!(human.contains("ferris.future-record/v1 (emitted, satisfied=false"));
        let report = envelope.record.expect("report");
        assert!(!report.compatible);
        assert_eq!(report.unsatisfied_count, 2);
        assert_eq!(
            report.requirements[0].lifecycle,
            Some(ContractLifecycle::Incubating)
        );
        assert_eq!(report.requirements[1].lifecycle, None);
    }

    #[test]
    fn compatibility_requirements_reject_duplicate_pairs() {
        let path = requirements_file(serde_json::json!({
            "schema": CONTRACT_REQUIREMENTS_SCHEMA,
            "consumer_id": "duplicate-gate",
            "requirements": [
                {"schema": VALIDATION_PLAN_SCHEMA, "handling": "emitted"},
                {"schema": VALIDATION_PLAN_SCHEMA, "handling": "emitted"}
            ]
        }));
        let error = create_contract_compatibility_report(&path).expect_err("duplicate rejected");
        fs::remove_file(path).expect("remove requirements");

        assert_eq!(error.result_class(), ResultClass::Invalid);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID"
        );
    }

    #[test]
    fn compatibility_requirements_reject_duplicate_json_keys() {
        let path = requirements_bytes_file(
            br#"{
                "schema":"ferris.contract-requirements/v1",
                "consumer_id":"first",
                "consumer_id":"second",
                "requirements":[{"schema":"ferris.validation-plan/v0","handling":"emitted"}]
            }"#,
        );
        let error =
            create_contract_compatibility_report(&path).expect_err("duplicate key rejected");
        fs::remove_file(path).expect("remove requirements");

        assert_eq!(error.result_class(), ResultClass::Invalid);
        assert_eq!(
            error.diagnostic().code,
            "FERRIS-CONTRACT-REQUIREMENTS-INVALID"
        );
    }

    #[test]
    fn compatibility_requirements_enforce_the_input_size_bound() {
        let path =
            requirements_bytes_file(&vec![b' '; MAX_CONTRACT_REQUIREMENTS_BYTES as usize + 1]);
        let error = create_contract_compatibility_report(&path).expect_err("oversize rejected");
        fs::remove_file(path).expect("remove requirements");

        assert_eq!(error.result_class(), ResultClass::Invalid);
        assert!(error.diagnostic().message.contains("64 KiB"));
    }
}
