use clap::{Parser, Subcommand, ValueEnum};
use ferris_core::{
    CommandEnvelope, create_explanation, create_graph, create_plan, error_envelope,
    render_error_human, render_explanation_human, render_graph_human, render_plan_human,
};
use serde::Serialize;
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
    let cli = Cli::parse();
    match cli.command {
        FerrisCommand::Plan(args) => run_plan(args),
        FerrisCommand::Explain(args) => run_explain(args),
        FerrisCommand::Graph(args) => run_graph(args),
    }
}

fn run_plan(args: CommandArgs) -> ExitCode {
    match create_plan(&args.manifest_path) {
        Ok(envelope) => {
            print_success(args.format, &envelope, || render_plan_human(&envelope));
            ExitCode::SUCCESS
        }
        Err(error) => print_error("plan", &args, error),
    }
}

fn run_explain(args: CommandArgs) -> ExitCode {
    match create_explanation(&args.manifest_path) {
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
    match create_graph(&args.manifest_path) {
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
                error_envelope(command, &args.manifest_path, &error);
            eprintln!(
                "{}",
                serde_json::to_string_pretty(&envelope)
                    .expect("typed Ferris diagnostics must serialize")
            );
        }
    }
    ExitCode::from(error.result_class().exit_code())
}
