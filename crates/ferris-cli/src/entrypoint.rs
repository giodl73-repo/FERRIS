use clap::{CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum, error::ErrorKind};
use ferris_core::{
    ArtifactQualificationStatus, CommandEnvelope, Diagnostic, ResultClass, ValidationPlanRequest,
    command_envelope, command_line_invocation_identity, command_line_selection_identity,
    create_artifact_qualification_report, create_artifact_reuse_report, create_doctor,
    create_explanation, create_federated_plan, create_federated_validation_plan, create_graph,
    create_iteration_replay_report, create_plan, create_profile_diff, create_revision_skew_report,
    create_root_qualified_plan, create_schedule_replay_report,
    create_validation_plan_with_owner_domains, create_validation_topology_plan,
    doctor_error_envelope, error_envelope, execute_action_plan_with_cancellation,
    federated_plan_error_envelope, federated_validation_plan_error_envelope,
    locate_workspace_manifest, profile_diff_error_envelope, render_doctor_human,
    render_explanation_human, render_federated_plan_human, render_federated_validation_plan_human,
    render_graph_human, render_plan_human, render_profile_diff_human, render_revision_skew_human,
    render_root_qualified_plan_human, render_validation_plan_human,
    render_validation_topology_plan_human, revision_skew_error_envelope,
    root_qualified_plan_error_envelope, validation_plan_error_envelope_for_request,
    validation_topology_error_envelope, verify_execution_receipt,
};
use serde::Serialize;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const CARGO_FERRIS_EXECUTABLE: &str = "cargo-ferris";
const CARGO_FERRIS_SUBCOMMAND: &str = "ferris";

#[derive(Parser)]
#[command(version, about = "Ferris planning and approved local execution")]
struct Cli {
    #[command(subcommand)]
    command: FerrisCommand,
}

#[derive(Subcommand)]
enum FerrisCommand {
    Plan(PlanArgs),
    ValidationPlan(ValidationPlanArgs),
    Explain(CommandArgs),
    Graph(CommandArgs),
    Doctor(CommandArgs),
    ProfileDiff(ProfileDiffArgs),
    FederatedPlan(FederatedPlanArgs),
    FederatedValidationPlan(FederatedValidationPlanArgs),
    RevisionSkew(RevisionSkewArgs),
    Replay(ReplayArgs),
    Schedule(ScheduleArgs),
    Artifacts(ArtifactsArgs),
    Go(GoArgs),
    Verify(VerifyArgs),
}

#[derive(clap::Args)]
struct PlanArgs {
    #[arg(
        long,
        value_name = "PORTABLE_ID",
        required_unless_present = "topology_declaration"
    )]
    workspace_id: Option<String>,

    #[arg(
        long,
        value_name = "CARGO_TOML",
        help = "Cargo.toml selection; cargo-ferris defaults to Cargo's current workspace"
    )]
    manifest_path: Option<PathBuf>,

    #[arg(
        long,
        value_name = "PATH_AUTHORITY_JSON",
        help = "Strict ferris.path-authority/v0 input for repository-root-qualified identity"
    )]
    path_authority: Option<PathBuf>,

    #[arg(long, value_name = "DECLARATION_JSON")]
    topology_declaration: Option<PathBuf>,

    #[arg(long, value_name = "OBSERVATION_JSON")]
    topology_observation: Option<PathBuf>,

    #[arg(long)]
    full: bool,

    #[arg(long, value_name = "GATE_SET_ID")]
    gate_set: Option<String>,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(clap::Args)]
struct CommandArgs {
    #[arg(long, value_name = "PORTABLE_ID")]
    workspace_id: String,

    #[arg(
        long,
        value_name = "CARGO_TOML",
        help = "Cargo.toml selection; cargo-ferris defaults to Cargo's current workspace"
    )]
    manifest_path: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(clap::Args)]
struct ProfileDiffArgs {
    #[arg(long, value_name = "PROFILE_JSON")]
    before: PathBuf,

    #[arg(long, value_name = "PROFILE_JSON")]
    after: PathBuf,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(clap::Args)]
struct FederatedPlanArgs {
    #[arg(long, value_name = "REQUEST_JSON")]
    request: PathBuf,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(clap::Args)]
struct FederatedValidationPlanArgs {
    #[arg(long, value_name = "APPLICATION_JSON")]
    application: PathBuf,

    #[arg(long, value_name = "PATH")]
    changed_path: Vec<PathBuf>,

    #[arg(long, value_name = "WORKSPACE_ID:PACKAGE")]
    changed_package: Vec<String>,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(clap::Args)]
struct RevisionSkewArgs {
    #[arg(long, value_name = "REQUEST_JSON")]
    request: PathBuf,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(clap::Args)]
struct ReplayArgs {
    #[arg(long, value_name = "REQUEST_JSON")]
    request: PathBuf,
}

#[derive(clap::Args)]
struct ScheduleArgs {
    #[arg(long, value_name = "REQUEST_JSON")]
    request: PathBuf,
}

#[derive(clap::Args)]
struct ArtifactsArgs {
    #[arg(long, value_name = "REQUEST_JSON")]
    request: PathBuf,

    #[arg(long, value_name = "ARTIFACT", requires = "manifest_path")]
    artifact_path: Option<PathBuf>,

    #[arg(long, value_name = "MANIFEST", requires = "artifact_path")]
    manifest_path: Option<PathBuf>,

    #[arg(long, requires_all = ["artifact_path", "manifest_path"])]
    require_compatible: bool,
}

#[derive(clap::Args)]
struct GoArgs {
    #[arg(long, value_name = "SHA256_ID")]
    action_plan: String,
}

#[derive(clap::Args)]
struct VerifyArgs {
    #[arg(value_name = "RECEIPT")]
    receipt: PathBuf,
}

#[derive(clap::Args)]
struct ValidationPlanArgs {
    #[arg(long, value_name = "PORTABLE_ID")]
    workspace_id: String,

    #[arg(
        long,
        value_name = "CARGO_TOML",
        help = "Cargo.toml selection; cargo-ferris defaults to Cargo's current workspace"
    )]
    manifest_path: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    changed_path: Vec<PathBuf>,

    #[arg(
        long,
        value_name = "WORKSPACE_RELATIVE_PATH",
        help = "Missing path resolved against the Cargo workspace root, not the current directory"
    )]
    deleted_path: Vec<PathBuf>,

    #[arg(long, value_name = "PACKAGE")]
    changed_package: Vec<String>,

    #[arg(long, value_name = "REVISION")]
    base_revision: Option<String>,

    #[arg(long, value_name = "REVISION")]
    head_revision: Option<String>,

    #[arg(long, value_name = "REVISION")]
    tested_revision: Option<String>,

    #[arg(long, value_name = "OWNER_DOMAINS_JSON")]
    owner_domains: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
}

pub(crate) fn main_exit_code() -> ExitCode {
    let invocation = InvocationContext::capture();
    let outcome = guard_cli_execution(&invocation, || dispatch(&invocation));
    ExitCode::from(emit_to(
        outcome,
        &invocation,
        &mut io::stdout().lock(),
        &mut io::stderr().lock(),
    ))
}

struct CliOutcome {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    process_exit_code: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InvocationContext {
    normalized_args_os: Vec<OsString>,
    normalized_args: Vec<String>,
    command_name: &'static str,
    kind: InvocationKind,
}

impl InvocationContext {
    fn capture() -> Self {
        Self::from_raw_args(std::env::args_os().collect())
    }

    fn from_raw_args(raw_args: Vec<OsString>) -> Self {
        Self::from_raw_args_for_platform(raw_args, InvocationPlatform::current())
    }

    fn from_raw_args_for_platform(raw_args: Vec<OsString>, platform: InvocationPlatform) -> Self {
        let kind = InvocationKind::detect(&raw_args, platform);
        let command_name = kind.command_name();
        let normalized_args_os = kind.normalize_args(raw_args);
        let normalized_args = normalized_args_os
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect();
        Self {
            normalized_args_os,
            normalized_args,
            command_name,
            kind,
        }
    }

    fn help_guidance(&self) -> String {
        format!(
            "Run {} --help or {} <command> --help.",
            self.command_name, self.command_name
        )
    }
}

fn dispatch(invocation: &InvocationContext) -> CliOutcome {
    let cli = match parse_cli(invocation) {
        Ok(cli) => cli,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            return CliOutcome {
                stdout: error.to_string().into_bytes(),
                stderr: Vec::new(),
                process_exit_code: 0,
            };
        }
        Err(_) => {
            let command = semantic_command_from_args(&invocation.normalized_args);
            let help_guidance = invocation.help_guidance();
            let envelope =
                invalid_cli_envelope(command, &invocation.normalized_args, &help_guidance);
            return error_outcome(&envelope);
        }
    };
    match cli.command {
        FerrisCommand::Plan(args) => run_plan(invocation, args),
        FerrisCommand::ValidationPlan(args) => run_validation_plan(invocation, args),
        FerrisCommand::Explain(args) => run_explain(invocation, args),
        FerrisCommand::Graph(args) => run_graph(invocation, args),
        FerrisCommand::Doctor(args) => run_doctor(invocation, args),
        FerrisCommand::ProfileDiff(args) => run_profile_diff(args),
        FerrisCommand::FederatedPlan(args) => run_federated_plan(args),
        FerrisCommand::FederatedValidationPlan(args) => run_federated_validation_plan(args),
        FerrisCommand::RevisionSkew(args) => run_revision_skew(args),
        FerrisCommand::Replay(args) => run_replay(invocation, args),
        FerrisCommand::Schedule(args) => run_schedule(invocation, args),
        FerrisCommand::Artifacts(args) => run_artifacts(invocation, args),
        FerrisCommand::Go(args) => run_go(invocation, args),
        FerrisCommand::Verify(args) => run_verify(invocation, args),
    }
}

fn parse_cli(invocation: &InvocationContext) -> Result<Cli, clap::Error> {
    let mut command = Cli::command();
    command = command.name(invocation.command_name);
    command = command.bin_name(invocation.command_name);
    if !invocation.kind.allows_current_workspace_default() {
        for subcommand in ["validation-plan", "explain", "graph", "doctor"] {
            command = command.mut_subcommand(subcommand, |command| {
                command.mut_arg("manifest_path", |argument| argument.required(true))
            });
        }
    }
    let mut matches = command.try_get_matches_from_mut(invocation.normalized_args_os.clone())?;
    Cli::from_arg_matches_mut(&mut matches)
}

fn run_plan(invocation: &InvocationContext, args: PlanArgs) -> CliOutcome {
    if let Some(declaration) = &args.topology_declaration {
        let Some(observation) = &args.topology_observation else {
            return plan_cli_error(
                invocation,
                "--topology-declaration requires --topology-observation.",
            );
        };
        let Some(gate_set) = &args.gate_set else {
            return plan_cli_error(invocation, "Topology planning requires --gate-set.");
        };
        if !args.full {
            return plan_cli_error(
                invocation,
                "Topology planning requires --full because it projects the complete declared gate set.",
            );
        }
        if args.workspace_id.is_some()
            || args.manifest_path.is_some()
            || args.path_authority.is_some()
        {
            return plan_cli_error(
                invocation,
                "Topology planning cannot be combined with workspace or path-authority planning arguments.",
            );
        }
        return match create_validation_topology_plan(declaration, observation, gate_set) {
            Ok(envelope) => success_outcome(args.format, &envelope, || {
                render_validation_topology_plan_human(&envelope)
            }),
            Err(error) => {
                let envelope: CommandEnvelope<serde_json::Value> =
                    validation_topology_error_envelope(declaration, observation, gate_set, &error);
                error_outcome(&envelope)
            }
        };
    }
    if args.topology_observation.is_some() || args.full || args.gate_set.is_some() {
        return plan_cli_error(
            invocation,
            "--full, --gate-set, and --topology-observation require --topology-declaration.",
        );
    }
    let Some(workspace_id) = args.workspace_id else {
        return plan_cli_error(invocation, "Workspace planning requires --workspace-id.");
    };
    let resolved = match resolve_command_args(
        invocation,
        "plan",
        CommandArgs {
            workspace_id,
            manifest_path: args.manifest_path,
            format: args.format,
        },
    ) {
        Ok(args) => args,
        Err(outcome) => return outcome,
    };
    if let Some(path_authority) = &args.path_authority {
        return match create_root_qualified_plan(
            &resolved.manifest_path,
            &resolved.workspace_id,
            path_authority,
        ) {
            Ok(envelope) => success_outcome(resolved.format, &envelope, || {
                render_root_qualified_plan_human(&envelope)
            }),
            Err(error) => {
                let envelope: CommandEnvelope<serde_json::Value> =
                    root_qualified_plan_error_envelope(
                        &resolved.workspace_id,
                        &resolved.manifest_path,
                        path_authority,
                        &error,
                    );
                error_outcome(&envelope)
            }
        };
    }
    match create_plan(&resolved.manifest_path, &resolved.workspace_id) {
        Ok(envelope) => {
            success_outcome(resolved.format, &envelope, || render_plan_human(&envelope))
        }
        Err(error) => command_error_outcome("plan", &resolved, error),
    }
}

fn run_validation_plan(invocation: &InvocationContext, args: ValidationPlanArgs) -> CliOutcome {
    let args = match resolve_validation_plan_args(invocation, args) {
        Ok(args) => args,
        Err(outcome) => return outcome,
    };
    match create_validation_plan_with_owner_domains(
        &args.manifest_path,
        &args.workspace_id,
        ValidationPlanRequest::new(&args.changed_path, &args.changed_package)
            .with_deleted_paths(&args.deleted_path)
            .with_owner_domains(args.owner_domains.as_deref())
            .with_revision_options(
                args.base_revision.as_deref(),
                args.head_revision.as_deref(),
                args.tested_revision.as_deref(),
            ),
    ) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_validation_plan_human(&envelope)
        }),
        Err(error) => validation_plan_error_outcome(&args, error),
    }
}

fn run_explain(invocation: &InvocationContext, args: CommandArgs) -> CliOutcome {
    let args = match resolve_command_args(invocation, "explain", args) {
        Ok(args) => args,
        Err(outcome) => return outcome,
    };
    match create_explanation(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_explanation_human(&envelope)
        }),
        Err(error) => command_error_outcome("explain", &args, error),
    }
}

fn run_graph(invocation: &InvocationContext, args: CommandArgs) -> CliOutcome {
    let args = match resolve_command_args(invocation, "graph", args) {
        Ok(args) => args,
        Err(outcome) => return outcome,
    };
    match create_graph(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || render_graph_human(&envelope)),
        Err(error) => command_error_outcome("graph", &args, error),
    }
}

fn run_doctor(invocation: &InvocationContext, args: CommandArgs) -> CliOutcome {
    let args = match resolve_command_args(invocation, "doctor", args) {
        Ok(args) => args,
        Err(outcome) => return outcome,
    };
    match create_doctor(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || render_doctor_human(&envelope)),
        Err(error) => doctor_error_outcome(&args, error),
    }
}

fn run_profile_diff(args: ProfileDiffArgs) -> CliOutcome {
    match create_profile_diff(&args.before, &args.after) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_profile_diff_human(&envelope)
        }),
        Err(error) => {
            let envelope: CommandEnvelope<serde_json::Value> =
                profile_diff_error_envelope(&args.before, &args.after, &error);
            error_outcome(&envelope)
        }
    }
}

fn run_federated_plan(args: FederatedPlanArgs) -> CliOutcome {
    match create_federated_plan(&args.request) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_federated_plan_human(&envelope)
        }),
        Err(error) => {
            let envelope: CommandEnvelope<serde_json::Value> =
                federated_plan_error_envelope(&args.request, &error);
            error_outcome(&envelope)
        }
    }
}

fn run_federated_validation_plan(args: FederatedValidationPlanArgs) -> CliOutcome {
    match create_federated_validation_plan(
        &args.application,
        &args.changed_path,
        &args.changed_package,
    ) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_federated_validation_plan_human(&envelope)
        }),
        Err(error) => {
            let envelope: CommandEnvelope<serde_json::Value> =
                federated_validation_plan_error_envelope(
                    &args.application,
                    &args.changed_path,
                    &args.changed_package,
                    &error,
                );
            error_outcome(&envelope)
        }
    }
}

fn run_revision_skew(args: RevisionSkewArgs) -> CliOutcome {
    match create_revision_skew_report(&args.request) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_revision_skew_human(&envelope)
        }),
        Err(error) => {
            let envelope: CommandEnvelope<serde_json::Value> =
                revision_skew_error_envelope(&args.request, &error);
            error_outcome(&envelope)
        }
    }
}

fn run_replay(invocation: &InvocationContext, args: ReplayArgs) -> CliOutcome {
    match create_iteration_replay_report(&args.request) {
        Ok(report) => CliOutcome {
            stdout: serialize_line(&report),
            stderr: Vec::new(),
            process_exit_code: ResultClass::Success.exit_code(),
        },
        Err(error) => execution_error_outcome(invocation, "replay", error),
    }
}

fn run_schedule(invocation: &InvocationContext, args: ScheduleArgs) -> CliOutcome {
    match create_schedule_replay_report(&args.request) {
        Ok(report) => CliOutcome {
            stdout: serialize_line(&report),
            stderr: Vec::new(),
            process_exit_code: ResultClass::Success.exit_code(),
        },
        Err(error) => execution_error_outcome(invocation, "schedule", error),
    }
}

fn run_artifacts(invocation: &InvocationContext, args: ArtifactsArgs) -> CliOutcome {
    if let (Some(artifact_path), Some(manifest_path)) =
        (args.artifact_path.as_deref(), args.manifest_path.as_deref())
    {
        return match create_artifact_qualification_report(
            &args.request,
            artifact_path,
            manifest_path,
        ) {
            Ok(report) => {
                let process_exit_code = if args.require_compatible
                    && report.status != ArtifactQualificationStatus::Qualified
                {
                    ResultClass::Difference.exit_code()
                } else {
                    ResultClass::Success.exit_code()
                };
                CliOutcome {
                    stdout: serialize_line(&report),
                    stderr: Vec::new(),
                    process_exit_code,
                }
            }
            Err(error) => execution_error_outcome(invocation, "artifacts", error),
        };
    }

    match create_artifact_reuse_report(&args.request) {
        Ok(report) => CliOutcome {
            stdout: serialize_line(&report),
            stderr: Vec::new(),
            process_exit_code: ResultClass::Success.exit_code(),
        },
        Err(error) => execution_error_outcome(invocation, "artifacts", error),
    }
}

fn run_go(invocation: &InvocationContext, args: GoArgs) -> CliOutcome {
    let repository_root = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            let envelope = internal_cli_envelope(
                "go",
                &invocation.normalized_args,
                "FERRIS-EXECUTION-CURRENT-DIRECTORY-FAILED",
                "Ferris could not identify the current repository directory.",
                "Run from the repository root and retry.",
            );
            return error_outcome(&envelope);
        }
    };
    let cancellation = Arc::new(AtomicBool::new(false));
    let handler_cancellation = Arc::clone(&cancellation);
    if ctrlc::set_handler(move || handler_cancellation.store(true, Ordering::Release)).is_err() {
        let envelope = internal_cli_envelope(
            "go",
            &invocation.normalized_args,
            "FERRIS-EXECUTION-CANCELLATION-HANDLER-FAILED",
            "Ferris could not install cancellation handling.",
            "Retry the command in a process that permits console signal handling.",
        );
        return error_outcome(&envelope);
    }
    match execute_action_plan_with_cancellation(
        &repository_root,
        &args.action_plan,
        cancellation.as_ref(),
    ) {
        Ok(outcome) => CliOutcome {
            stdout: serialize_line(&outcome.receipt),
            stderr: Vec::new(),
            process_exit_code: outcome.receipt.result_class().exit_code(),
        },
        Err(error) => execution_error_outcome(invocation, "go", error),
    }
}

fn run_verify(invocation: &InvocationContext, args: VerifyArgs) -> CliOutcome {
    match verify_execution_receipt(&args.receipt) {
        Ok(verification) => CliOutcome {
            stdout: serialize_line(&verification),
            stderr: Vec::new(),
            process_exit_code: ResultClass::Success.exit_code(),
        },
        Err(error) => execution_error_outcome(invocation, "verify", error),
    }
}

struct ResolvedCommandArgs {
    workspace_id: String,
    manifest_path: PathBuf,
    format: OutputFormat,
}

struct ResolvedValidationPlanArgs {
    workspace_id: String,
    manifest_path: PathBuf,
    changed_path: Vec<PathBuf>,
    deleted_path: Vec<PathBuf>,
    changed_package: Vec<String>,
    base_revision: Option<String>,
    head_revision: Option<String>,
    tested_revision: Option<String>,
    owner_domains: Option<PathBuf>,
    format: OutputFormat,
}

fn resolve_command_args(
    invocation: &InvocationContext,
    command: &str,
    args: CommandArgs,
) -> Result<ResolvedCommandArgs, CliOutcome> {
    let manifest_path =
        resolve_manifest_path(invocation, command, &args.workspace_id, args.manifest_path)?;
    Ok(ResolvedCommandArgs {
        workspace_id: args.workspace_id,
        manifest_path,
        format: args.format,
    })
}

fn resolve_validation_plan_args(
    invocation: &InvocationContext,
    args: ValidationPlanArgs,
) -> Result<ResolvedValidationPlanArgs, CliOutcome> {
    let manifest_path = resolve_manifest_path(
        invocation,
        "validation-plan",
        &args.workspace_id,
        args.manifest_path,
    )?;
    Ok(ResolvedValidationPlanArgs {
        workspace_id: args.workspace_id,
        manifest_path,
        changed_path: args.changed_path,
        deleted_path: args.deleted_path,
        changed_package: args.changed_package,
        base_revision: args.base_revision,
        head_revision: args.head_revision,
        tested_revision: args.tested_revision,
        owner_domains: args.owner_domains,
        format: args.format,
    })
}

fn resolve_manifest_path(
    invocation: &InvocationContext,
    command: &str,
    workspace_id: &str,
    manifest_path: Option<PathBuf>,
) -> Result<PathBuf, CliOutcome> {
    if let Some(manifest_path) = manifest_path {
        return Ok(manifest_path);
    }
    if !invocation.kind.allows_current_workspace_default() {
        let envelope = invalid_cli_envelope(
            command,
            &invocation.normalized_args,
            &format!(
                "Pass --manifest-path explicitly. {}",
                invocation.help_guidance()
            ),
        );
        return Err(error_outcome(&envelope));
    }

    let current_directory = std::env::current_dir().map_err(|_| {
        let envelope = internal_cli_envelope(
            command,
            &invocation.normalized_args,
            "FERRIS-WORKSPACE-DISCOVERY-CURRENT-DIRECTORY-FAILED",
            "Ferris could not identify the current directory for Cargo workspace discovery.",
            "Run from a readable directory or pass --manifest-path explicitly.",
        );
        error_outcome(&envelope)
    })?;
    locate_workspace_manifest(&current_directory, workspace_id).map_err(|error| {
        let envelope: CommandEnvelope<serde_json::Value> =
            error_envelope(command, workspace_id, Path::new("Cargo.toml"), &error);
        error_outcome(&envelope)
    })
}

fn success_outcome<T: Serialize>(
    format: OutputFormat,
    envelope: &CommandEnvelope<T>,
    human: impl FnOnce() -> String,
) -> CliOutcome {
    let stdout = match format {
        OutputFormat::Human => human().into_bytes(),
        OutputFormat::Json => serialize_line(envelope),
    };
    CliOutcome {
        stdout,
        stderr: Vec::new(),
        process_exit_code: envelope.process_exit_code,
    }
}

fn command_error_outcome(
    command: &str,
    args: &ResolvedCommandArgs,
    error: ferris_core::CoreError,
) -> CliOutcome {
    let envelope: CommandEnvelope<serde_json::Value> =
        error_envelope(command, &args.workspace_id, &args.manifest_path, &error);
    error_outcome(&envelope)
}

fn doctor_error_outcome(args: &ResolvedCommandArgs, error: ferris_core::CoreError) -> CliOutcome {
    let envelope: CommandEnvelope<serde_json::Value> =
        doctor_error_envelope(&args.workspace_id, &args.manifest_path, &error);
    error_outcome(&envelope)
}

fn validation_plan_error_outcome(
    args: &ResolvedValidationPlanArgs,
    error: ferris_core::CoreError,
) -> CliOutcome {
    let envelope: CommandEnvelope<serde_json::Value> = validation_plan_error_envelope_for_request(
        &args.workspace_id,
        &args.manifest_path,
        ValidationPlanRequest::new(&args.changed_path, &args.changed_package)
            .with_deleted_paths(&args.deleted_path)
            .with_owner_domains(args.owner_domains.as_deref())
            .with_revision_options(
                args.base_revision.as_deref(),
                args.head_revision.as_deref(),
                args.tested_revision.as_deref(),
            ),
        &error,
    );
    error_outcome(&envelope)
}

fn error_outcome<T: Serialize>(envelope: &CommandEnvelope<T>) -> CliOutcome {
    CliOutcome {
        stdout: Vec::new(),
        stderr: serialize_line(envelope),
        process_exit_code: envelope.process_exit_code,
    }
}

fn execution_error_outcome(
    invocation: &InvocationContext,
    command: &str,
    error: ferris_core::CoreError,
) -> CliOutcome {
    let selection_identity = command_line_selection_identity(command, &invocation.normalized_args);
    let envelope: CommandEnvelope<serde_json::Value> = command_envelope(
        command,
        selection_identity,
        command_line_invocation_identity(command, &invocation.normalized_args),
        error.result_class(),
        vec![error.diagnostic().clone()],
        None,
    );
    error_outcome(&envelope)
}

fn plan_cli_error(invocation: &InvocationContext, guidance: &str) -> CliOutcome {
    let envelope = invalid_cli_envelope("plan", &invocation.normalized_args, guidance);
    error_outcome(&envelope)
}

fn serialize_line<T: Serialize>(value: &T) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("typed Ferris records must serialize");
    bytes.push(b'\n');
    bytes
}

fn guard_cli_execution(
    invocation: &InvocationContext,
    execute: impl FnOnce() -> CliOutcome,
) -> CliOutcome {
    // Product execution is single-threaded; future in-process worker threads must
    // replace this process-global hook boundary with thread-owned error capture.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = catch_unwind(AssertUnwindSafe(execute));
    std::panic::set_hook(previous_hook);
    result.unwrap_or_else(|_| {
        let command = semantic_command_from_args(&invocation.normalized_args);
        let envelope = internal_cli_envelope(
            command,
            &invocation.normalized_args,
            "FERRIS-CLI-INTERNAL",
            "Ferris could not complete the command safely.",
            "Retry with the same arguments and report the Ferris version if the failure repeats.",
        );
        error_outcome(&envelope)
    })
}

fn emit_to(
    outcome: CliOutcome,
    invocation: &InvocationContext,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> u8 {
    if !outcome.stdout.is_empty() && stdout.write_all(&outcome.stdout).is_err() {
        let command = semantic_command_from_args(&invocation.normalized_args);
        let envelope = internal_cli_envelope(
            command,
            &invocation.normalized_args,
            "FERRIS-CLI-OUTPUT-FAILED",
            "Ferris could not emit the command result safely.",
            "Retry with a writable output stream and report the Ferris version if the failure repeats.",
        );
        let _ = stderr.write_all(&serialize_line(&envelope));
        return envelope.process_exit_code;
    }
    if !outcome.stderr.is_empty() && stderr.write_all(&outcome.stderr).is_err() {
        return ResultClass::Internal.exit_code();
    }
    outcome.process_exit_code
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InvocationPlatform {
    Windows,
    Other,
}

impl InvocationPlatform {
    fn current() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else {
            Self::Other
        }
    }

    fn matches_executable(self, candidate: &str) -> bool {
        match self {
            Self::Windows => candidate.eq_ignore_ascii_case(CARGO_FERRIS_EXECUTABLE),
            Self::Other => candidate == CARGO_FERRIS_EXECUTABLE,
        }
    }

    fn matches_subcommand(self, candidate: &str) -> bool {
        match self {
            Self::Windows => candidate.eq_ignore_ascii_case(CARGO_FERRIS_SUBCOMMAND),
            Self::Other => candidate == CARGO_FERRIS_SUBCOMMAND,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InvocationKind {
    Ferris,
    CargoFerrisDirect,
    CargoFerrisSubcommand,
}

impl InvocationKind {
    fn detect(raw_args: &[OsString], platform: InvocationPlatform) -> Self {
        if !is_cargo_ferris_executable(raw_args.first().map(OsString::as_os_str), platform) {
            return Self::Ferris;
        }

        if raw_args.get(1).is_some_and(|argument| {
            platform.matches_subcommand(&argument.as_os_str().to_string_lossy())
        }) {
            Self::CargoFerrisSubcommand
        } else {
            Self::CargoFerrisDirect
        }
    }

    fn command_name(self) -> &'static str {
        match self {
            Self::Ferris => "ferris",
            Self::CargoFerrisDirect => CARGO_FERRIS_EXECUTABLE,
            Self::CargoFerrisSubcommand => "cargo ferris",
        }
    }

    fn allows_current_workspace_default(self) -> bool {
        !matches!(self, Self::Ferris)
    }

    fn normalize_args(self, mut raw_args: Vec<OsString>) -> Vec<OsString> {
        if matches!(self, Self::CargoFerrisSubcommand) {
            raw_args.remove(1);
        }
        raw_args
    }
}

fn is_cargo_ferris_executable(program: Option<&OsStr>, platform: InvocationPlatform) -> bool {
    program
        .and_then(|value| Path::new(value).file_stem())
        .is_some_and(|stem| platform.matches_executable(&stem.to_string_lossy()))
}

fn semantic_command_from_args(args: &[String]) -> &str {
    args.get(1)
        .map(String::as_str)
        .filter(|command| {
            matches!(
                *command,
                "plan"
                    | "validation-plan"
                    | "explain"
                    | "graph"
                    | "doctor"
                    | "profile-diff"
                    | "federated-plan"
                    | "federated-validation-plan"
                    | "revision-skew"
                    | "replay"
                    | "schedule"
                    | "artifacts"
                    | "go"
                    | "verify"
            )
        })
        .unwrap_or("cli")
}

fn invalid_cli_envelope(
    semantic_command_id: &str,
    args: &[String],
    help_guidance: &str,
) -> CommandEnvelope<serde_json::Value> {
    command_envelope(
        semantic_command_id,
        command_line_selection_identity(semantic_command_id, args),
        command_line_invocation_identity(semantic_command_id, args),
        ResultClass::Invalid,
        vec![Diagnostic {
            code: "FERRIS-CLI-INVALID".to_owned(),
            severity: "error".to_owned(),
            result_class: ResultClass::Invalid,
            message: "Command-line arguments are invalid.".to_owned(),
            source_digest: None,
            bounded_output: None,
            next_actions: vec![help_guidance.to_owned()],
        }],
        None,
    )
}

fn internal_cli_envelope(
    semantic_command_id: &str,
    args: &[String],
    code: &str,
    message: &str,
    next_action: &str,
) -> CommandEnvelope<serde_json::Value> {
    command_envelope(
        semantic_command_id,
        command_line_selection_identity(semantic_command_id, args),
        command_line_invocation_identity(semantic_command_id, args),
        ResultClass::Internal,
        vec![Diagnostic {
            code: code.to_owned(),
            severity: "error".to_owned(),
            result_class: ResultClass::Internal,
            message: message.to_owned(),
            source_digest: None,
            bounded_output: None,
            next_actions: vec![next_action.to_owned()],
        }],
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "ferris-cli-profile-diff-{label}-{}-{nonce}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, AtomicOrdering::Relaxed)
            ));
            fs::create_dir_all(&path).expect("create CLI test directory");
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

    fn write_profile(path: &Path, revision: &str, value: &str) {
        let profile = serde_json::json!({
            "schema": "ferris.profile-evidence/v0",
            "profile_id": "profile.example",
            "revision": revision,
            "consumer": "consumer.example",
            "sections": {
                "identity": {"value": value},
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
        });
        fs::write(
            path,
            serde_json::to_vec_pretty(&profile).expect("serialize CLI profile"),
        )
        .expect("write CLI profile");
    }

    #[test]
    fn panic_is_converted_to_typed_internal_outcome() {
        let invocation = InvocationContext::from_raw_args(vec![
            OsString::from("ferris"),
            OsString::from("doctor"),
            OsString::from("--workspace-id"),
            OsString::from("ferris.test/example"),
        ]);

        let outcome = guard_cli_execution(&invocation, || panic!("injected test panic"));

        assert_eq!(outcome.process_exit_code, 11);
        assert!(outcome.stdout.is_empty());
        let value: serde_json::Value =
            serde_json::from_slice(&outcome.stderr).expect("typed internal outcome");
        assert_eq!(value["schema"], "ferris.command-result/v2");
        assert_eq!(value["semantic_command_id"], "doctor");
        assert_eq!(value["result_class"], "internal");
        assert_eq!(value["process_exit_code"], 11);
        assert_eq!(value["diagnostics"][0]["code"], "FERRIS-CLI-INTERNAL");
        assert!(value["record"].is_null());
    }

    #[test]
    fn stdout_failure_returns_typed_internal_outcome_on_stderr() {
        struct FailingWriter;

        impl Write for FailingWriter {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "injected"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let args = vec![OsString::from("ferris"), OsString::from("--help")];
        let invocation = InvocationContext::from_raw_args(args);
        let outcome = CliOutcome {
            stdout: b"help".to_vec(),
            stderr: Vec::new(),
            process_exit_code: 0,
        };
        let mut stdout = FailingWriter;
        let mut stderr = Vec::new();

        let exit = emit_to(outcome, &invocation, &mut stdout, &mut stderr);

        assert_eq!(exit, 11);
        let value: serde_json::Value =
            serde_json::from_slice(&stderr).expect("typed output failure");
        assert_eq!(value["schema"], "ferris.command-result/v2");
        assert_eq!(value["semantic_command_id"], "cli");
        assert_eq!(value["result_class"], "internal");
        assert_eq!(value["process_exit_code"], 11);
        assert_eq!(value["diagnostics"][0]["code"], "FERRIS-CLI-OUTPUT-FAILED");
        assert!(value["record"].is_null());
    }

    #[test]
    fn profile_diff_json_difference_uses_stdout_and_exit_one() {
        let directory = TestDirectory::new("difference");
        let before = directory.path("before.json");
        let after = directory.path("after.json");
        write_profile(&before, "r1", "before");
        write_profile(&after, "r2", "after");
        let args = vec![
            OsString::from("ferris"),
            OsString::from("profile-diff"),
            OsString::from("--before"),
            before.into_os_string(),
            OsString::from("--after"),
            after.into_os_string(),
            OsString::from("--format"),
            OsString::from("json"),
        ];

        let outcome = dispatch(&InvocationContext::from_raw_args(args));

        assert_eq!(outcome.process_exit_code, 1);
        assert!(outcome.stderr.is_empty());
        let value: serde_json::Value =
            serde_json::from_slice(&outcome.stdout).expect("profile diff JSON");
        assert_eq!(value["semantic_command_id"], "profile-diff");
        assert_eq!(value["result_class"], "difference");
        assert_eq!(value["record"]["schema"], "ferris.profile-diff/v0");
        assert_eq!(value["record"]["executable"], false);
    }

    #[test]
    fn profile_diff_human_output_is_complete_and_redacted() {
        let directory = TestDirectory::new("human");
        let before = directory.path("before.json");
        let after = directory.path("after.json");
        let secret = "SECRET-CLI-RAW-91c0";
        write_profile(&before, "r1", secret);
        write_profile(&after, "r1", "replacement");
        let args = vec![
            OsString::from("ferris"),
            OsString::from("profile-diff"),
            OsString::from("--before"),
            before.into_os_string(),
            OsString::from("--after"),
            after.into_os_string(),
        ];

        let outcome = dispatch(&InvocationContext::from_raw_args(args));
        let output = String::from_utf8(outcome.stdout).expect("human output");

        assert_eq!(outcome.process_exit_code, 1);
        assert!(outcome.stderr.is_empty());
        assert!(output.contains("Changed sections:"));
        assert!(output.contains("Changes:"));
        assert!(output.contains("Unchanged sections:"));
        assert!(output.contains("Unknowns:"));
        assert!(output.contains("Limitations:"));
        assert!(!output.contains(secret));
    }

    #[test]
    fn profile_diff_missing_file_is_typed_incomplete_stderr() {
        let directory = TestDirectory::new("missing");
        let before = directory.path("missing.json");
        let after = directory.path("after.json");
        write_profile(&after, "r1", "value");
        let args = vec![
            OsString::from("ferris"),
            OsString::from("profile-diff"),
            OsString::from("--before"),
            before.into_os_string(),
            OsString::from("--after"),
            after.into_os_string(),
            OsString::from("--format"),
            OsString::from("json"),
        ];

        let outcome = dispatch(&InvocationContext::from_raw_args(args));

        assert_eq!(outcome.process_exit_code, 5);
        assert!(outcome.stdout.is_empty());
        let value: serde_json::Value =
            serde_json::from_slice(&outcome.stderr).expect("typed incomplete result");
        assert_eq!(value["semantic_command_id"], "profile-diff");
        assert_eq!(value["result_class"], "incomplete");
        assert_eq!(value["record"], serde_json::Value::Null);
    }

    #[test]
    fn cargo_ferris_direct_help_uses_direct_binary_name() {
        let invocation = InvocationContext::from_raw_args(vec![
            OsString::from("cargo-ferris"),
            OsString::from("--help"),
        ]);

        let outcome = dispatch(&invocation);

        assert_eq!(outcome.process_exit_code, 0);
        assert!(outcome.stderr.is_empty());
        let output = String::from_utf8(outcome.stdout).expect("help output");
        assert!(output.contains("Usage: cargo-ferris"));
        assert!(output.contains("validation-plan"));
    }

    #[test]
    fn cargo_external_subcommand_help_uses_cargo_name() {
        let invocation = InvocationContext::from_raw_args(vec![
            OsString::from("cargo-ferris"),
            OsString::from("ferris"),
            OsString::from("--help"),
        ]);

        assert_eq!(
            invocation.normalized_args,
            vec!["cargo-ferris".to_owned(), "--help".to_owned()]
        );

        let outcome = dispatch(&invocation);

        assert_eq!(outcome.process_exit_code, 0);
        assert!(outcome.stderr.is_empty());
        let output = String::from_utf8(outcome.stdout).expect("help output");
        assert!(output.contains("Usage: cargo ferris"));
        assert!(output.contains("validation-plan"));
    }

    #[test]
    fn version_banners_match_invocation_names() {
        for (args, expected_banner) in [
            (
                vec![OsString::from("ferris"), OsString::from("--version")],
                format!("ferris {}\n", env!("CARGO_PKG_VERSION")),
            ),
            (
                vec![OsString::from("cargo-ferris"), OsString::from("--version")],
                format!("cargo-ferris {}\n", env!("CARGO_PKG_VERSION")),
            ),
            (
                vec![
                    OsString::from("cargo-ferris"),
                    OsString::from("ferris"),
                    OsString::from("--version"),
                ],
                format!("cargo ferris {}\n", env!("CARGO_PKG_VERSION")),
            ),
        ] {
            let outcome = dispatch(&InvocationContext::from_raw_args(args));

            assert_eq!(outcome.process_exit_code, 0);
            assert!(outcome.stderr.is_empty());
            assert_eq!(
                String::from_utf8(outcome.stdout).expect("version output"),
                expected_banner
            );
        }
    }

    #[test]
    fn windows_mixed_case_cargo_subcommand_is_normalized() {
        let invocation = InvocationContext::from_raw_args_for_platform(
            vec![
                OsString::from("Cargo-Ferris"),
                OsString::from("Ferris"),
                OsString::from("--version"),
            ],
            InvocationPlatform::Windows,
        );

        assert_eq!(
            invocation.normalized_args,
            vec!["Cargo-Ferris".to_owned(), "--version".to_owned()]
        );
        assert_eq!(invocation.command_name, "cargo ferris");

        let outcome = dispatch(&invocation);

        assert_eq!(outcome.process_exit_code, 0);
        assert!(outcome.stderr.is_empty());
        assert_eq!(
            String::from_utf8(outcome.stdout).expect("version output"),
            format!("cargo ferris {}\n", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn unix_mixed_case_cargo_subcommand_remains_literal() {
        let invocation = InvocationContext::from_raw_args_for_platform(
            vec![
                OsString::from("cargo-ferris"),
                OsString::from("Ferris"),
                OsString::from("--help"),
            ],
            InvocationPlatform::Other,
        );

        assert_eq!(
            invocation.normalized_args,
            vec![
                "cargo-ferris".to_owned(),
                "Ferris".to_owned(),
                "--help".to_owned()
            ]
        );
        assert_eq!(invocation.command_name, "cargo-ferris");

        let outcome = dispatch(&invocation);

        assert_eq!(outcome.process_exit_code, 2);
        assert!(outcome.stdout.is_empty());
        let value: serde_json::Value =
            serde_json::from_slice(&outcome.stderr).expect("typed invalid result");
        assert_eq!(value["semantic_command_id"], "cli");
        assert_eq!(value["result_class"], "invalid");
        assert_eq!(
            value["diagnostics"][0]["next_actions"][0],
            "Run cargo-ferris --help or cargo-ferris <command> --help."
        );
    }
}
