use super::{
    ACTION_PLAN_LANES_SCHEMA, ACTION_PLAN_SCHEMA, APPLICATION_READINESS_REPORT_SCHEMA,
    APPLICATION_READINESS_REQUEST_SCHEMA, APPLICATION_SCHEMA, ARTIFACT_QUALIFICATION_REPORT_SCHEMA,
    ARTIFACT_REUSE_REPORT_SCHEMA, ARTIFACT_REUSE_REQUEST_SCHEMA, BOUNDED_OUTPUT_EVIDENCE_SCHEMA,
    COMMAND_RESULT_SCHEMA, CommandEnvelope, DOCTOR_SCHEMA, ENVIRONMENT_READINESS_REPORT_SCHEMA,
    ENVIRONMENT_REQUIREMENTS_SCHEMA, EXECUTION_APPROVAL_SCHEMA, EXECUTION_RECEIPT_SCHEMA,
    EXECUTION_VERIFICATION_SCHEMA, EXPLANATION_SCHEMA, FEDERATED_PLAN_REQUEST_SCHEMA,
    FEDERATED_PLAN_SCHEMA, FEDERATED_VALIDATION_PLAN_SCHEMA, GRAPH_SCHEMA,
    ITERATION_REPLAY_REPORT_SCHEMA, ITERATION_REPLAY_REQUEST_SCHEMA,
    LEGACY_EXECUTION_RECEIPT_SCHEMA, OWNER_ENTRYPOINTS_SCHEMA, OWNER_VALIDATION_DOMAINS_SCHEMA,
    OWNER_VALIDATION_DOMAINS_V2_SCHEMA, PATH_AUTHORITY_SCHEMA, PLAN_SCHEMA, PROFILE_DIFF_SCHEMA,
    PROFILE_EVIDENCE_SCHEMA, REMOTE_ITERATION_EVIDENCE_SCHEMA, REVISION_SKEW_REPORT_SCHEMA,
    REVISION_SKEW_REQUEST_SCHEMA, ROOT_QUALIFIED_PLAN_SCHEMA, SCHEDULE_REPLAY_REPORT_SCHEMA,
    SCHEDULE_REPLAY_REQUEST_SCHEMA, VALIDATION_DECLARATION_SCHEMA, VALIDATION_OBSERVATION_SCHEMA,
    VALIDATION_PLAN_SCHEMA, VALIDATION_REVISION_BINDING_SCHEMA, VALIDATION_TOPOLOGY_PLAN_SCHEMA,
    digest_bytes, invocation_identity, selection_identity, success_envelope,
};
use serde::Serialize;

pub const CONTRACT_CATALOG_SCHEMA: &str = "ferris.contract-catalog/v1";

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
        support(COMMAND_RESULT_SCHEMA, false, true),
        support(CONTRACT_CATALOG_SCHEMA, false, true),
        support(DOCTOR_SCHEMA, false, true),
        support(ENVIRONMENT_READINESS_REPORT_SCHEMA, true, true),
        support(ENVIRONMENT_REQUIREMENTS_SCHEMA, true, false),
        support(EXECUTION_APPROVAL_SCHEMA, true, false),
        support(EXECUTION_RECEIPT_SCHEMA, true, true),
        support(EXECUTION_VERIFICATION_SCHEMA, false, true),
        support(EXPLANATION_SCHEMA, false, true),
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
}
