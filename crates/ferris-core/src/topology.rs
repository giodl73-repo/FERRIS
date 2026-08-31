use super::*;
use std::io::Read;

pub const VALIDATION_DECLARATION_SCHEMA: &str = "ferris.validation-declaration/v1";
pub const VALIDATION_OBSERVATION_SCHEMA: &str = "ferris.validation-observation/v1";
pub const VALIDATION_TOPOLOGY_PLAN_SCHEMA: &str = "ferris.validation-topology-plan/v1";

const MAX_TOPOLOGY_INPUT_BYTES: u64 = 1024 * 1024;
const MAX_TOPOLOGY_ITEMS: usize = 10_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationDeclaration {
    pub schema: String,
    pub application_id: String,
    pub declaration_revision: String,
    pub gate_sets: Vec<TopologyGateSet>,
    pub gates: Vec<TopologyGate>,
    pub nodes: Vec<TopologyNode>,
    #[serde(default)]
    pub credential_classes: Vec<TopologyCredentialClass>,
    #[serde(default)]
    pub coverage_gaps: Vec<TopologyCoverageGap>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyGateSet {
    pub gate_set_id: String,
    pub required_gates: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyGate {
    pub gate_id: String,
    pub kind: TopologyGateKind,
    pub node_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyGateKind {
    Technical,
    External,
    HumanPresence,
    Publication,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyNode {
    pub node_id: String,
    pub kind: TopologyNodeKind,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<TopologyPlatform>,
    pub condition: TopologyCondition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_class: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyPlatform {
    Linux,
    Windows,
    Macos,
    ProviderManaged,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyNodeKind {
    Run,
    Stage,
    Job,
    Shard,
    Barrier,
    Artifact,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyCondition {
    pub kind: TopologyConditionKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyConditionKind {
    Always,
    Path,
    Manual,
    Credential,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyCoverageGap {
    pub gap_id: String,
    pub path: String,
    pub disposition: TopologyGapDisposition,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyGapDisposition {
    AcceptedOwnerGap,
    CoverageDefect,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyCredentialClass {
    pub credential_class_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationObservation {
    pub schema: String,
    pub application_id: String,
    pub source_identity: String,
    pub freshness: ObservationFreshness,
    pub availability: ObservationAvailability,
    pub gates: Vec<ObservedGate>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationFreshness {
    Current,
    Stale,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationAvailability {
    Available,
    Unavailable,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedGate {
    pub gate_id: String,
    pub status: ObservedGateStatus,
    pub observed_node_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservedGateStatus {
    Active,
    Inactive,
    ExcludedHuman,
    ExcludedPublication,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyReconciliationStatus {
    Complete,
    Unreconciled,
    Stale,
    Unavailable,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyExclusionStatus {
    ExcludedHuman,
    ExcludedPublication,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExcludedTopologyGate {
    pub gate_id: String,
    pub kind: TopologyGateKind,
    pub node_ids: Vec<String>,
    pub exclusion_status: TopologyExclusionStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TopologyIssue {
    pub code: String,
    pub subject_id: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationTopologyPlan {
    pub schema: String,
    pub topology_plan_id: String,
    pub declaration_id: String,
    pub declaration_revision: String,
    pub observation_id: String,
    pub observation_source_identity: String,
    pub observation_freshness: ObservationFreshness,
    pub observation_availability: ObservationAvailability,
    pub application_id: String,
    pub gate_set_id: String,
    pub executable: bool,
    pub reconciliation_status: TopologyReconciliationStatus,
    pub required_gates: Vec<TopologyGate>,
    pub excluded_gates: Vec<ExcludedTopologyGate>,
    pub stage_count: usize,
    pub nodes: Vec<TopologyNode>,
    pub excluded_nodes: Vec<TopologyNode>,
    pub issues: Vec<TopologyIssue>,
    pub limitations: Vec<String>,
}

pub fn create_validation_topology_plan(
    declaration_path: &Path,
    observation_path: &Path,
    gate_set_id: &str,
) -> Result<CommandEnvelope<ValidationTopologyPlan>, CoreError> {
    let declaration_input =
        read_topology_input(declaration_path, "declaration").map_err(|error| {
            contextualize_topology_error(
                error,
                "source-unavailable",
                "source-unavailable",
                gate_set_id,
            )
        })?;
    let observation_input =
        read_topology_input(observation_path, "observation").map_err(|error| {
            contextualize_topology_error(
                error,
                &declaration_input.digest,
                "source-unavailable",
                gate_set_id,
            )
        })?;
    let declaration_digest = declaration_input.digest.clone();
    let observation_digest = observation_input.digest.clone();
    let result = (|| {
        let declaration: ValidationDeclaration =
            parse_topology_json(&declaration_input, "declaration")?;
        let observation: ValidationObservation =
            parse_topology_json(&observation_input, "observation")?;
        create_validation_topology_plan_inner(declaration, observation, gate_set_id)
    })();
    result.map_err(|error| {
        contextualize_topology_error(error, &declaration_digest, &observation_digest, gate_set_id)
    })
}

fn contextualize_topology_error(
    error: CoreError,
    declaration_digest: &str,
    observation_digest: &str,
    gate_set_id: &str,
) -> CoreError {
    let source_digest = digest_bytes(
        format!("{declaration_digest}\n{observation_digest}\n{gate_set_id}").as_bytes(),
    );
    let selection_identity = invocation_identity(&[
        "validation-topology-error-selection",
        declaration_digest,
        observation_digest,
        gate_set_id,
    ])
    .replacen("invocation:", "selection:", 1);
    error
        .with_source_digest(source_digest)
        .with_invocation_selection(selection_identity)
}

fn create_validation_topology_plan_inner(
    mut declaration: ValidationDeclaration,
    mut observation: ValidationObservation,
    gate_set_id: &str,
) -> Result<CommandEnvelope<ValidationTopologyPlan>, CoreError> {
    validate_topology_id(gate_set_id, "gate-set")?;
    if declaration.schema != VALIDATION_DECLARATION_SCHEMA {
        return Err(topology_error(
            ResultClass::Unsupported,
            "FERRIS-TOPOLOGY-DECLARATION-SCHEMA-UNSUPPORTED",
            format!("Use schema {VALIDATION_DECLARATION_SCHEMA}."),
        ));
    }

    if observation.schema != VALIDATION_OBSERVATION_SCHEMA {
        return Err(topology_error(
            ResultClass::Unsupported,
            "FERRIS-TOPOLOGY-OBSERVATION-SCHEMA-UNSUPPORTED",
            format!("Use schema {VALIDATION_OBSERVATION_SCHEMA}."),
        ));
    }
    validate_topology_id(&declaration.application_id, "application")?;
    validate_topology_id(&declaration.declaration_revision, "declaration-revision")?;
    validate_topology_id(&observation.application_id, "application")?;
    validate_topology_id(&observation.source_identity, "observation-source")?;
    if declaration.application_id != observation.application_id {
        return Err(topology_error(
            ResultClass::Invalid,
            "FERRIS-TOPOLOGY-APPLICATION-MISMATCH",
            "Declaration and observation application identities differ.",
        ));
    }
    enforce_topology_bounds(&declaration, &observation)?;
    normalize_and_validate_declaration(&mut declaration)?;
    normalize_and_validate_observation(&mut observation, &declaration)?;

    let gate_set = declaration
        .gate_sets
        .iter()
        .find(|gate_set| gate_set.gate_set_id == gate_set_id)
        .ok_or_else(|| {
            topology_error(
                ResultClass::Invalid,
                "FERRIS-TOPOLOGY-GATE-SET-NOT-FOUND",
                "The requested gate set is not declared.",
            )
        })?;
    let declaration_id = record_id("validation-declaration", &declaration)?;
    let observation_id = record_id("validation-observation", &observation)?;
    let gates_by_id = declaration
        .gates
        .iter()
        .map(|gate| (gate.gate_id.as_str(), gate))
        .collect::<BTreeMap<_, _>>();
    let observations_by_gate = observation
        .gates
        .iter()
        .map(|gate| (gate.gate_id.as_str(), gate))
        .collect::<BTreeMap<_, _>>();
    let mut required_gates = Vec::new();
    let mut excluded_gates = declaration
        .gates
        .iter()
        .filter(|gate| {
            matches!(
                gate.kind,
                TopologyGateKind::HumanPresence | TopologyGateKind::Publication
            )
        })
        .map(|gate| ExcludedTopologyGate {
            gate_id: gate.gate_id.clone(),
            kind: gate.kind,
            node_ids: gate.node_ids.clone(),
            exclusion_status: match gate.kind {
                TopologyGateKind::HumanPresence => TopologyExclusionStatus::ExcludedHuman,
                TopologyGateKind::Publication => TopologyExclusionStatus::ExcludedPublication,
                TopologyGateKind::Technical | TopologyGateKind::External => {
                    unreachable!("only human and publication gates are excluded")
                }
            },
        })
        .collect::<Vec<_>>();
    let mut required_node_ids = BTreeSet::new();
    let mut issues = Vec::new();
    for gate_id in &gate_set.required_gates {
        let gate = gates_by_id[gate_id.as_str()];
        if matches!(
            gate.kind,
            TopologyGateKind::HumanPresence | TopologyGateKind::Publication
        ) {
            continue;
        }
        required_gates.push(gate.clone());
        required_node_ids.extend(gate.node_ids.iter().cloned());
        match observations_by_gate.get(gate_id.as_str()) {
            None => issues.push(TopologyIssue {
                code: "FERRIS-TOPOLOGY-REQUIRED-GATE-MISSING".to_owned(),
                subject_id: gate_id.clone(),
                message: "A required technical gate is absent from provider observation."
                    .to_owned(),
            }),
            Some(observed) if observed.status != ObservedGateStatus::Active => {
                issues.push(TopologyIssue {
                    code: "FERRIS-TOPOLOGY-REQUIRED-GATE-NOT-ACTIVE".to_owned(),
                    subject_id: gate_id.clone(),
                    message: "A required technical gate is not observed active.".to_owned(),
                });
            }
            Some(observed) => {
                let observed_nodes = observed
                    .observed_node_ids
                    .iter()
                    .map(String::as_str)
                    .collect::<BTreeSet<_>>();
                for node_id in &gate.node_ids {
                    if !observed_nodes.contains(node_id.as_str()) {
                        issues.push(TopologyIssue {
                            code: "FERRIS-TOPOLOGY-REQUIRED-NODE-MISSING".to_owned(),
                            subject_id: node_id.clone(),
                            message: format!(
                                "Required node is absent from provider observation for gate {gate_id}."
                            ),
                        });
                    }
                }
            }
        }
    }
    for gate in &excluded_gates {
        let expected = match gate.exclusion_status {
            TopologyExclusionStatus::ExcludedHuman => ObservedGateStatus::ExcludedHuman,
            TopologyExclusionStatus::ExcludedPublication => ObservedGateStatus::ExcludedPublication,
        };
        if !matches!(
            observations_by_gate.get(gate.gate_id.as_str()),
            Some(observed) if observed.status == expected
        ) {
            issues.push(TopologyIssue {
                code: match gate.exclusion_status {
                    TopologyExclusionStatus::ExcludedHuman => {
                        "FERRIS-TOPOLOGY-EXCLUDED-HUMAN-UNPROVEN"
                    }
                    TopologyExclusionStatus::ExcludedPublication => {
                        "FERRIS-TOPOLOGY-EXCLUDED-PUBLICATION-UNPROVEN"
                    }
                }
                .to_owned(),
                subject_id: gate.gate_id.clone(),
                message: format!(
                    "Declared {:?} gate is not observed with {:?} status.",
                    gate.kind, expected
                ),
            });
        }
    }
    for gap in &declaration.coverage_gaps {
        if gap.disposition == TopologyGapDisposition::CoverageDefect {
            issues.push(TopologyIssue {
                code: "FERRIS-TOPOLOGY-COVERAGE-DEFECT".to_owned(),
                subject_id: gap.gap_id.clone(),
                message: format!("Required coverage is missing for {}.", gap.path),
            });
        }
    }
    required_gates.sort_by(|left, right| left.gate_id.cmp(&right.gate_id));
    excluded_gates.sort_by(|left, right| left.gate_id.cmp(&right.gate_id));
    issues.sort_by(|left, right| {
        (&left.code, &left.subject_id).cmp(&(&right.code, &right.subject_id))
    });
    let nodes = declaration
        .nodes
        .iter()
        .filter(|node| required_node_ids.contains(&node.node_id))
        .cloned()
        .collect::<Vec<_>>();
    let excluded_node_ids = excluded_gates
        .iter()
        .flat_map(|gate| gate.node_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let excluded_nodes = declaration
        .nodes
        .iter()
        .filter(|node| excluded_node_ids.contains(&node.node_id))
        .cloned()
        .collect::<Vec<_>>();
    let stage_count = nodes
        .iter()
        .filter(|node| node.kind == TopologyNodeKind::Stage)
        .count();
    let reconciliation_status = match (
        observation.availability,
        observation.freshness,
        issues.is_empty(),
    ) {
        (ObservationAvailability::Unsupported, _, _) => TopologyReconciliationStatus::Unsupported,
        (ObservationAvailability::Unavailable, _, _) => TopologyReconciliationStatus::Unavailable,
        (_, ObservationFreshness::Stale, _) => TopologyReconciliationStatus::Stale,
        (_, _, true) => TopologyReconciliationStatus::Complete,
        _ => TopologyReconciliationStatus::Unreconciled,
    };
    let topology_plan_id = record_id(
        "validation-topology-plan",
        &(
            VALIDATION_TOPOLOGY_PLAN_SCHEMA,
            &declaration_id,
            &observation_id,
            gate_set_id,
            reconciliation_status,
            &required_gates,
            &excluded_gates,
            &nodes,
            &excluded_nodes,
            &issues,
        ),
    )?;
    let plan = ValidationTopologyPlan {
        schema: VALIDATION_TOPOLOGY_PLAN_SCHEMA.to_owned(),
        topology_plan_id,
        declaration_id,
        declaration_revision: declaration.declaration_revision,
        observation_id,
        observation_source_identity: observation.source_identity,
        observation_freshness: observation.freshness,
        observation_availability: observation.availability,
        application_id: declaration.application_id,
        gate_set_id: gate_set_id.to_owned(),
        executable: false,
        reconciliation_status,
        required_gates,
        excluded_gates,
        stage_count,
        nodes,
        excluded_nodes,
        issues,
        limitations: vec![
            "This is a read-only projection of owner declaration and provider observation."
                .to_owned(),
            "It does not execute owner entrypoints or establish merge readiness.".to_owned(),
        ],
    };
    let result_class = match reconciliation_status {
        TopologyReconciliationStatus::Complete => ResultClass::Success,
        TopologyReconciliationStatus::Unreconciled => ResultClass::Incomplete,
        TopologyReconciliationStatus::Stale => ResultClass::Stale,
        TopologyReconciliationStatus::Unavailable => ResultClass::Incomplete,
        TopologyReconciliationStatus::Unsupported => ResultClass::Unsupported,
    };
    let selection_identity = invocation_identity(&[
        "validation-topology-selection",
        &plan.declaration_id,
        &plan.observation_id,
        gate_set_id,
    ])
    .replacen("invocation:", "selection:", 1);
    let diagnostics = reconciliation_diagnostics(reconciliation_status, &plan.issues);
    Ok(command_envelope(
        "plan",
        selection_identity.clone(),
        invocation_identity(&["validation-topology-plan", &selection_identity]),
        result_class,
        diagnostics,
        Some(plan),
    ))
}

pub fn validation_topology_error_envelope<T: Serialize>(
    declaration_path: &Path,
    observation_path: &Path,
    gate_set_id: &str,
    error: &CoreError,
) -> CommandEnvelope<T> {
    let selection_identity = error
        .invocation_selection()
        .filter(|identity| identity.starts_with("selection:"))
        .map(str::to_owned)
        .unwrap_or_else(|| {
            invocation_identity(&[
                "validation-topology-selection",
                &request_path_selection_digest(declaration_path.to_string_lossy().as_ref()),
                &request_path_selection_digest(observation_path.to_string_lossy().as_ref()),
                gate_set_id,
                error
                    .diagnostic()
                    .source_digest
                    .as_deref()
                    .unwrap_or("source-unavailable"),
            ])
            .replacen("invocation:", "selection:", 1)
        });
    command_envelope(
        "plan",
        selection_identity.clone(),
        invocation_identity(&["validation-topology-plan", &selection_identity]),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    )
}

pub fn render_validation_topology_plan_human(
    envelope: &CommandEnvelope<ValidationTopologyPlan>,
) -> String {
    let plan = envelope
        .record
        .as_ref()
        .expect("topology projection has a record");
    let mut output = format!(
        "Ferris validation topology {}\nApplication ID: {}\nGate set: {}\nDeclaration revision: {}\nObservation source: {}\nObservation freshness: {:?}\nReconciliation: {}\nStages: {}\nExecutable: no\nRequired gates:\n",
        plan.topology_plan_id,
        plan.application_id,
        plan.gate_set_id,
        plan.declaration_revision,
        plan.observation_source_identity,
        plan.observation_freshness,
        serde_json::to_value(plan.reconciliation_status)
            .expect("status serializes")
            .as_str()
            .expect("status is a string"),
        plan.stage_count,
    );
    for gate in &plan.required_gates {
        output.push_str(&format!("  - {} ({:?})\n", gate.gate_id, gate.kind));
    }
    if !plan.excluded_gates.is_empty() {
        output.push_str("Excluded owner gates:\n");
        for gate in &plan.excluded_gates {
            output.push_str(&format!(
                "  - {} ({:?}, {:?})\n",
                gate.gate_id, gate.kind, gate.exclusion_status
            ));
        }
    }
    if !plan.excluded_nodes.is_empty() {
        output.push_str("Excluded owner nodes:\n");
        for node in &plan.excluded_nodes {
            output.push_str(&format!("  - {} ({:?})\n", node.node_id, node.kind));
        }
    }
    if !plan.issues.is_empty() {
        output.push_str("Reconciliation issues:\n");
        for issue in &plan.issues {
            output.push_str(&format!(
                "  - {} [{}]: {}\n",
                issue.subject_id, issue.code, issue.message
            ));
        }
    }
    output
}

struct TopologyInput {
    bytes: Vec<u8>,
    digest: String,
}

fn read_topology_input(path: &Path, kind: &str) -> Result<TopologyInput, CoreError> {
    let file = fs::File::open(path).map_err(|_| {
        topology_error(
            ResultClass::Incomplete,
            "FERRIS-TOPOLOGY-INPUT-UNAVAILABLE",
            format!("The topology {kind} is missing or unreadable."),
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_TOPOLOGY_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            topology_error(
                ResultClass::Incomplete,
                "FERRIS-TOPOLOGY-INPUT-UNAVAILABLE",
                format!("The topology {kind} could not be read."),
            )
        })?;
    if bytes.len() as u64 > MAX_TOPOLOGY_INPUT_BYTES {
        return Err(topology_error(
            ResultClass::Blocked,
            "FERRIS-TOPOLOGY-INPUT-OVERSIZED",
            format!("The topology {kind} exceeds the {MAX_TOPOLOGY_INPUT_BYTES}-byte bound."),
        )
        .with_source_digest(digest_bytes(&bytes)));
    }
    let digest = digest_bytes(&bytes);
    Ok(TopologyInput { bytes, digest })
}

fn parse_topology_json<T: for<'de> Deserialize<'de>>(
    input: &TopologyInput,
    kind: &str,
) -> Result<T, CoreError> {
    serde_json::from_slice(&input.bytes).map_err(|_| {
        topology_error(
            ResultClass::Invalid,
            "FERRIS-TOPOLOGY-INPUT-INVALID",
            format!("The topology {kind} is not valid strict JSON."),
        )
        .with_source_digest(input.digest.clone())
    })
}

fn normalize_and_validate_declaration(
    declaration: &mut ValidationDeclaration,
) -> Result<(), CoreError> {
    let mut gate_set_ids = BTreeSet::new();
    for gate_set in &mut declaration.gate_sets {
        validate_topology_id(&gate_set.gate_set_id, "gate-set")?;
        ensure_unique(&mut gate_set_ids, &gate_set.gate_set_id, "GATE-SET")?;
        gate_set.required_gates.sort();
        reject_duplicates(&gate_set.required_gates, "REQUIRED-GATE")?;
        if gate_set.required_gates.is_empty() {
            return Err(topology_error(
                ResultClass::Invalid,
                "FERRIS-TOPOLOGY-GATE-SET-EMPTY",
                format!(
                    "Gate set {} has no required technical or external gates.",
                    gate_set.gate_set_id
                ),
            ));
        }
    }
    let mut gate_ids = BTreeSet::new();
    for gate in &mut declaration.gates {
        validate_topology_id(&gate.gate_id, "gate")?;
        ensure_unique(&mut gate_ids, &gate.gate_id, "GATE")?;
        gate.node_ids.sort();
        reject_duplicates(&gate.node_ids, "GATE-NODE")?;
    }
    let mut node_ids = BTreeSet::new();
    let mut credential_class_ids = BTreeSet::new();
    for credential_class in &declaration.credential_classes {
        validate_topology_id(&credential_class.credential_class_id, "credential-class")?;
        ensure_unique(
            &mut credential_class_ids,
            &credential_class.credential_class_id,
            "CREDENTIAL-CLASS",
        )?;
    }
    for node in &mut declaration.nodes {
        validate_topology_id(&node.node_id, "node")?;
        ensure_unique(&mut node_ids, &node.node_id, "NODE")?;
        node.depends_on.sort();
        reject_duplicates(&node.depends_on, "NODE-DEPENDENCY")?;
        validate_condition(&node.condition)?;
        if matches!(
            node.kind,
            TopologyNodeKind::Run | TopologyNodeKind::Job | TopologyNodeKind::Shard
        ) && node.owner_entrypoint.is_none()
        {
            return Err(topology_error(
                ResultClass::Invalid,
                "FERRIS-TOPOLOGY-OWNER-ENTRYPOINT-MISSING",
                format!(
                    "Executable node {} has no owner entrypoint reference.",
                    node.node_id
                ),
            ));
        }
        if let Some(owner_entrypoint) = &node.owner_entrypoint {
            validate_topology_id(owner_entrypoint, "owner-entrypoint")?;
        }
        if matches!(
            node.kind,
            TopologyNodeKind::Run | TopologyNodeKind::Job | TopologyNodeKind::Shard
        ) && node.platform.is_none()
        {
            return Err(topology_error(
                ResultClass::Invalid,
                "FERRIS-TOPOLOGY-PLATFORM-MISSING",
                format!("Executable node {} has no typed platform.", node.node_id),
            ));
        }
        if let Some(credential_class) = &node.credential_class
            && !credential_class_ids.contains(credential_class)
        {
            return Err(unknown_reference("CREDENTIAL-CLASS", credential_class));
        }
        if node.condition.kind == TopologyConditionKind::Credential {
            let condition_credential = node
                .condition
                .reference
                .as_deref()
                .expect("credential conditions require a reference");
            if !credential_class_ids.contains(condition_credential) {
                return Err(unknown_reference(
                    "CONDITION-CREDENTIAL-CLASS",
                    condition_credential,
                ));
            }
            if node.credential_class.as_deref() != Some(condition_credential) {
                return Err(topology_error(
                    ResultClass::Invalid,
                    "FERRIS-TOPOLOGY-CREDENTIAL-CONDITION-MISMATCH",
                    format!(
                        "Node {} credential condition does not match its credential class.",
                        node.node_id
                    ),
                ));
            }
        }
    }
    for gate_set in &declaration.gate_sets {
        for gate_id in &gate_set.required_gates {
            if !gate_ids.contains(gate_id) {
                return Err(unknown_reference("GATE-SET", gate_id));
            }
            let gate = declaration
                .gates
                .iter()
                .find(|gate| &gate.gate_id == gate_id)
                .expect("gate identity validated");
            if matches!(
                gate.kind,
                TopologyGateKind::HumanPresence | TopologyGateKind::Publication
            ) {
                return Err(topology_error(
                    ResultClass::Invalid,
                    "FERRIS-TOPOLOGY-GATE-SET-KIND-INVALID",
                    format!(
                        "Gate set {} lists excluded gate {} as required.",
                        gate_set.gate_set_id, gate_id
                    ),
                ));
            }
        }
    }
    let mut node_membership = declaration
        .nodes
        .iter()
        .map(|node| (node.node_id.as_str(), 0_usize))
        .collect::<BTreeMap<_, _>>();
    for gate in &declaration.gates {
        let members = gate
            .node_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for node_id in &gate.node_ids {
            if !node_ids.contains(node_id) {
                return Err(unknown_reference("GATE", node_id));
            }
            *node_membership
                .get_mut(node_id.as_str())
                .expect("node identity validated") += 1;
            let node = declaration
                .nodes
                .iter()
                .find(|node| &node.node_id == node_id)
                .expect("node identity validated");
            for dependency in &node.depends_on {
                if !node_ids.contains(dependency) {
                    return Err(unknown_reference("NODE", dependency));
                }
                if !members.contains(dependency.as_str()) {
                    return Err(topology_error(
                        ResultClass::Invalid,
                        "FERRIS-TOPOLOGY-CROSS-GATE-DEPENDENCY",
                        format!(
                            "Node {} depends on {} outside gate {}.",
                            node.node_id, dependency, gate.gate_id
                        ),
                    ));
                }
            }
        }
    }
    if let Some((node_id, membership)) = node_membership
        .iter()
        .find(|(_, membership)| **membership != 1)
    {
        return Err(topology_error(
            ResultClass::Invalid,
            "FERRIS-TOPOLOGY-NODE-MEMBERSHIP-INVALID",
            format!("Node {node_id} belongs to {membership} gates; exactly one is required."),
        ));
    }
    reject_cycles(&declaration.nodes)?;
    let mut gap_ids = BTreeSet::new();
    for gap in &declaration.coverage_gaps {
        validate_topology_id(&gap.gap_id, "coverage-gap")?;
        ensure_unique(&mut gap_ids, &gap.gap_id, "COVERAGE-GAP")?;
        if validate_path_authority_relative_path(&gap.path, true).is_err()
            || gap.reason.trim().is_empty()
        {
            return Err(topology_error(
                ResultClass::Invalid,
                "FERRIS-TOPOLOGY-COVERAGE-GAP-INVALID",
                "Coverage gaps require a portable repository-relative path and a reason.",
            ));
        }
    }
    declaration
        .gate_sets
        .sort_by(|left, right| left.gate_set_id.cmp(&right.gate_set_id));
    declaration
        .gates
        .sort_by(|left, right| left.gate_id.cmp(&right.gate_id));
    declaration
        .nodes
        .sort_by(|left, right| left.node_id.cmp(&right.node_id));
    declaration
        .credential_classes
        .sort_by(|left, right| left.credential_class_id.cmp(&right.credential_class_id));
    declaration
        .coverage_gaps
        .sort_by(|left, right| left.gap_id.cmp(&right.gap_id));
    Ok(())
}

fn normalize_and_validate_observation(
    observation: &mut ValidationObservation,
    declaration: &ValidationDeclaration,
) -> Result<(), CoreError> {
    let gate_ids = declaration
        .gates
        .iter()
        .map(|gate| gate.gate_id.as_str())
        .collect::<BTreeSet<_>>();
    let node_ids = declaration
        .nodes
        .iter()
        .map(|node| node.node_id.as_str())
        .collect::<BTreeSet<_>>();
    let gate_nodes = declaration
        .gates
        .iter()
        .map(|gate| {
            (
                gate.gate_id.as_str(),
                gate.node_ids
                    .iter()
                    .map(String::as_str)
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut observed_gate_ids = BTreeSet::new();
    for gate in &mut observation.gates {
        validate_topology_id(&gate.gate_id, "observed-gate")?;
        ensure_unique(&mut observed_gate_ids, &gate.gate_id, "OBSERVED-GATE")?;
        if !gate_ids.contains(gate.gate_id.as_str()) {
            return Err(unknown_reference("OBSERVATION-GATE", &gate.gate_id));
        }
        gate.observed_node_ids.sort();
        reject_duplicates(&gate.observed_node_ids, "OBSERVED-NODE")?;
        for node_id in &gate.observed_node_ids {
            if !node_ids.contains(node_id.as_str()) {
                return Err(unknown_reference("OBSERVATION-NODE", node_id));
            }
            if !gate_nodes[gate.gate_id.as_str()].contains(node_id.as_str()) {
                return Err(topology_error(
                    ResultClass::Invalid,
                    "FERRIS-TOPOLOGY-OBSERVATION-NODE-MISMATCH",
                    format!(
                        "Observed node {node_id} does not belong to gate {}.",
                        gate.gate_id
                    ),
                ));
            }
        }
    }
    observation
        .gates
        .sort_by(|left, right| left.gate_id.cmp(&right.gate_id));
    Ok(())
}

fn validate_condition(condition: &TopologyCondition) -> Result<(), CoreError> {
    let reference_required = condition.kind != TopologyConditionKind::Always;
    if reference_required != condition.reference.is_some() {
        return Err(topology_error(
            ResultClass::Invalid,
            "FERRIS-TOPOLOGY-CONDITION-INVALID",
            "Always conditions omit reference; path, manual, and credential conditions require it.",
        ));
    }
    if let Some(reference) = &condition.reference {
        validate_topology_id(reference, "condition-reference")?;
    }
    Ok(())
}

fn reconciliation_diagnostics(
    status: TopologyReconciliationStatus,
    issues: &[TopologyIssue],
) -> Vec<Diagnostic> {
    if status == TopologyReconciliationStatus::Complete {
        return Vec::new();
    }
    let result_class = match status {
        TopologyReconciliationStatus::Unreconciled | TopologyReconciliationStatus::Unavailable => {
            ResultClass::Incomplete
        }
        TopologyReconciliationStatus::Stale => ResultClass::Stale,
        TopologyReconciliationStatus::Unsupported => ResultClass::Unsupported,
        TopologyReconciliationStatus::Complete => unreachable!("complete status handled above"),
    };
    let status_name = serde_json::to_value(status)
        .expect("topology status serializes")
        .as_str()
        .expect("topology status is a string")
        .to_owned();
    vec![Diagnostic {
        code: match status {
            TopologyReconciliationStatus::Unreconciled => "FERRIS-TOPOLOGY-UNRECONCILED",
            TopologyReconciliationStatus::Stale => "FERRIS-TOPOLOGY-STALE",
            TopologyReconciliationStatus::Unavailable => "FERRIS-TOPOLOGY-UNAVAILABLE",
            TopologyReconciliationStatus::Unsupported => "FERRIS-TOPOLOGY-UNSUPPORTED",
            TopologyReconciliationStatus::Complete => unreachable!("complete status handled above"),
        }
        .to_owned(),
        severity: "error".to_owned(),
        result_class,
        message: format!(
            "Owner validation topology projection is {status_name} with {} reconciliation issue(s).",
            issues.len()
        ),
        source_digest: None,
        bounded_output: None,
        next_actions: vec![
            "Inspect record.issues and reconcile the owner declaration with provider observation."
                .to_owned(),
            "Do not execute or omit validation nodes while topology is not complete.".to_owned(),
        ],
    }]
}

fn reject_cycles(nodes: &[TopologyNode]) -> Result<(), CoreError> {
    let mut remaining_dependencies = nodes
        .iter()
        .map(|node| (node.node_id.as_str(), node.depends_on.len()))
        .collect::<BTreeMap<_, _>>();
    let mut dependents = BTreeMap::<&str, Vec<&str>>::new();
    for node in nodes {
        for dependency in &node.depends_on {
            dependents
                .entry(dependency.as_str())
                .or_default()
                .push(node.node_id.as_str());
        }
    }
    let mut ready = remaining_dependencies
        .iter()
        .filter_map(|(node_id, count)| (*count == 0).then_some(*node_id))
        .collect::<Vec<_>>();
    let mut visited = 0_usize;
    while let Some(node_id) = ready.pop() {
        visited += 1;
        for dependent in dependents.get(node_id).into_iter().flatten() {
            let count = remaining_dependencies
                .get_mut(dependent)
                .expect("dependency identities validated");
            *count -= 1;
            if *count == 0 {
                ready.push(dependent);
            }
        }
    }
    if visited != nodes.len() {
        return Err(topology_error(
            ResultClass::Invalid,
            "FERRIS-TOPOLOGY-CYCLE",
            "The declared owner topology contains a dependency cycle.",
        ));
    }
    Ok(())
}

fn enforce_topology_bounds(
    declaration: &ValidationDeclaration,
    observation: &ValidationObservation,
) -> Result<(), CoreError> {
    let count = declaration
        .gate_sets
        .len()
        .saturating_add(declaration.gates.len())
        .saturating_add(declaration.nodes.len())
        .saturating_add(declaration.credential_classes.len())
        .saturating_add(declaration.coverage_gaps.len())
        .saturating_add(observation.gates.len());
    if count > MAX_TOPOLOGY_ITEMS {
        return Err(topology_error(
            ResultClass::Blocked,
            "FERRIS-TOPOLOGY-BOUND-EXCEEDED",
            format!("Topology inputs contain {count} items; the limit is {MAX_TOPOLOGY_ITEMS}."),
        ));
    }
    Ok(())
}

fn validate_topology_id(value: &str, kind: &str) -> Result<(), CoreError> {
    if valid_portable_id(value) {
        return Ok(());
    }
    Err(topology_error(
        ResultClass::Invalid,
        "FERRIS-TOPOLOGY-ID-INVALID",
        format!("The {kind} identity is not a stable portable identifier."),
    ))
}

fn ensure_unique(values: &mut BTreeSet<String>, value: &str, kind: &str) -> Result<(), CoreError> {
    if values.insert(value.to_owned()) {
        return Ok(());
    }
    Err(topology_error(
        ResultClass::Invalid,
        "FERRIS-TOPOLOGY-ID-DUPLICATE",
        format!("{kind} identity {value} is declared more than once."),
    ))
}

fn reject_duplicates(values: &[String], kind: &str) -> Result<(), CoreError> {
    if values.windows(2).all(|pair| pair[0] != pair[1]) {
        return Ok(());
    }
    Err(topology_error(
        ResultClass::Invalid,
        "FERRIS-TOPOLOGY-REFERENCE-DUPLICATE",
        format!("{kind} references contain a duplicate."),
    ))
}

fn unknown_reference(kind: &str, value: &str) -> CoreError {
    topology_error(
        ResultClass::Invalid,
        "FERRIS-TOPOLOGY-REFERENCE-UNKNOWN",
        format!("{kind} references unknown identity {value}."),
    )
}

fn topology_error(class: ResultClass, code: &str, message: impl Into<String>) -> CoreError {
    CoreError::new(
        class,
        code,
        message,
        vec!["Correct the owner declaration or provider observation and retry.".to_owned()],
    )
}
