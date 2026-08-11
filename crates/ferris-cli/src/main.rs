use clap::{Parser, Subcommand, ValueEnum, error::ErrorKind};
use ferris_core::{
    CommandEnvelope, Diagnostic, ResultClass, command_line_invocation_identity, create_explanation,
    create_graph, create_plan, error_envelope, render_error_human, render_explanation_human,
    render_graph_human, render_plan_human,
};
use serde::Serialize;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "ferris", version, about = "Read-only Ferris planning")]
struct Cli {
    #[command(subcommand)]
    command: FerrisCommand,
}

#[derive(Subcommand)]
enum FerrisCommand {
    Plan(CommandArgs),
    Explain(CommandArgs),
    Graph(CommandArgs),
}

#[derive(clap::Args)]
struct CommandArgs {
    #[arg(long, value_name = "PORTABLE_ID")]
    workspace_id: String,

    #[arg(long, value_name = "CARGO_TOML")]
    manifest_path: PathBuf,

    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
}

fn main() -> ExitCode {
    let raw_args = std::env::args_os().collect::<Vec<_>>();
    let cli = match Cli::try_parse_from(&raw_args) {
        Ok(cli) => cli,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            let _ = error.print();
            return ExitCode::SUCCESS;
        }
        Err(error) => {
            if requests_json(&raw_args) {
                let args = raw_args
                    .iter()
                    .map(|argument| argument.to_string_lossy().into_owned())
                    .collect::<Vec<_>>();
                let command = semantic_command_from_args(&args);
                let envelope = invalid_cli_envelope(command, &args);
                eprintln!(
                    "{}",
                    serde_json::to_string_pretty(&envelope)
                        .expect("typed Ferris diagnostics must serialize")
                );
            } else {
                let _ = error.print();
            }
            return ExitCode::from(ResultClass::Invalid.exit_code());
        }
    };
    match cli.command {
        FerrisCommand::Plan(args) => run_plan(args),
        FerrisCommand::Explain(args) => run_explain(args),
        FerrisCommand::Graph(args) => run_graph(args),
    }
}

fn run_plan(args: CommandArgs) -> ExitCode {
    match create_plan(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => {
            print_success(args.format, &envelope, || render_plan_human(&envelope));
            ExitCode::SUCCESS
        }
        Err(error) => print_error("plan", &args, error),
    }
}

fn run_explain(args: CommandArgs) -> ExitCode {
    match create_explanation(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => {
            print_success(args.format, &envelope, || {
                render_explanation_human(&envelope)
            });
            ExitCode::SUCCESS
        }
        Err(error) => print_error("explain", &args, error),
    }
}

fn run_graph(args: CommandArgs) -> ExitCode {
    match create_graph(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => {
            print_success(args.format, &envelope, || render_graph_human(&envelope));
            ExitCode::SUCCESS
        }
        Err(error) => print_error("graph", &args, error),
    }
}

fn print_success<T: Serialize>(
    format: OutputFormat,
    envelope: &CommandEnvelope<T>,
    human: impl FnOnce() -> String,
) {
    match format {
        OutputFormat::Human => print!("{}", human()),
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(envelope)
                    .expect("typed Ferris records must serialize")
            );
        }
    }
}

fn print_error(command: &str, args: &CommandArgs, error: ferris_core::CoreError) -> ExitCode {
    match args.format {
        OutputFormat::Human => eprint!("{}", render_error_human(&error)),
        OutputFormat::Json => {
            let envelope: CommandEnvelope<serde_json::Value> =
                error_envelope(command, &args.workspace_id, &args.manifest_path, &error);
            eprintln!(
                "{}",
                serde_json::to_string_pretty(&envelope)
                    .expect("typed Ferris diagnostics must serialize")
            );
        }
    }
    ExitCode::from(error.result_class().exit_code())
}

fn requests_json(args: &[OsString]) -> bool {
    args.windows(2)
        .any(|pair| pair[0] == "--format" && pair[1].to_string_lossy().eq_ignore_ascii_case("json"))
        || args.iter().any(|argument| {
            argument
                .to_string_lossy()
                .eq_ignore_ascii_case("--format=json")
        })
}

fn semantic_command_from_args(args: &[String]) -> &str {
    args.get(1)
        .map(String::as_str)
        .filter(|command| matches!(*command, "plan" | "explain" | "graph"))
        .unwrap_or("cli")
}

fn invalid_cli_envelope(
    semantic_command_id: &str,
    args: &[String],
) -> CommandEnvelope<serde_json::Value> {
    CommandEnvelope {
        schema: ferris_core::COMMAND_RESULT_SCHEMA.to_owned(),
        command_version: env!("CARGO_PKG_VERSION").to_owned(),
        semantic_command_id: semantic_command_id.to_owned(),
        invocation_identity: command_line_invocation_identity(semantic_command_id, args),
        result_class: ResultClass::Invalid,
        diagnostics: vec![Diagnostic {
            code: "FERRIS-CLI-INVALID".to_owned(),
            severity: "error".to_owned(),
            result_class: ResultClass::Invalid,
            message: "Command-line arguments are invalid.".to_owned(),
            source_digest: None,
            next_actions: vec!["Run ferris --help or ferris <command> --help.".to_owned()],
        }],
        record: None,
    }
}
