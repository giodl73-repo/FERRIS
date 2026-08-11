use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const COMMAND_RESULT_SCHEMA: &str = "ferris.command-result/v0";
pub const PLAN_SCHEMA: &str = "ferris.blueprint-plan/v0";
pub const EXPLANATION_SCHEMA: &str = "ferris.explanation/v0";
pub const GRAPH_SCHEMA: &str = "ferris.workspace-graph/v0";

const MAX_GRAPH_NODES: usize = 10_000;
const MAX_GRAPH_EDGES: usize = 50_000;

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

#[derive(Debug)]
pub struct CoreError {
    class: ResultClass,
    diagnostic: Box<Diagnostic>,
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
        }
    }

    fn with_source_digest(mut self, source_digest: String) -> Self {
        self.diagnostic.source_digest = Some(source_digest);
        self
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

fn load_cargo_metadata(
    manifest_path: &Path,
    cargo_program: &Path,
) -> Result<MetadataInvocation, CoreError> {
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
        "Evidence: owner={}, representation={}, working-directory={}, workspace-id={}, metadata-format={}, offline={}, output-digest={}\nCommand: {}\n",
        graph.evidence.owner,
        graph.evidence.command_representation,
        graph.evidence.working_directory,
        graph.evidence.workspace_id,
        graph.evidence.metadata_format_version,
        graph.evidence.offline,
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
        working_directory: "selected-workspace-root".to_owned(),
        workspace_id: workspace_id.to_owned(),
        owner_output_digest: digest_bytes(bytes),
        metadata_format_version: 1,
        offline: true,
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
    ])
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
}
