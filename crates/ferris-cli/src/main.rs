use clap::{Parser, Subcommand, ValueEnum, error::ErrorKind};
use ferris_core::{
    CommandEnvelope, Diagnostic, ResultClass, command_envelope, command_line_invocation_identity,
    command_line_selection_identity, create_doctor, create_explanation, create_graph, create_plan,
    doctor_error_envelope, error_envelope, render_doctor_human, render_explanation_human,
    render_graph_human, render_plan_human,
};
use serde::Serialize;
use std::ffi::OsString;
use std::io::{self, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
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
    Doctor(CommandArgs),
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
    let outcome = guard_cli_execution(&raw_args, || dispatch(&raw_args));
    ExitCode::from(emit_to(
        outcome,
        &raw_args,
        &mut io::stdout().lock(),
        &mut io::stderr().lock(),
    ))
}

struct CliOutcome {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    process_exit_code: u8,
}

fn dispatch(raw_args: &[OsString]) -> CliOutcome {
    let cli = match Cli::try_parse_from(raw_args) {
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
            let args = raw_args
                .iter()
                .map(|argument| argument.to_string_lossy().into_owned())
                .collect::<Vec<_>>();
            let command = semantic_command_from_args(&args);
            let envelope = invalid_cli_envelope(command, &args);
            return error_outcome(&envelope);
        }
    };
    match cli.command {
        FerrisCommand::Plan(args) => run_plan(args),
        FerrisCommand::Explain(args) => run_explain(args),
        FerrisCommand::Graph(args) => run_graph(args),
        FerrisCommand::Doctor(args) => run_doctor(args),
    }
}

fn run_plan(args: CommandArgs) -> CliOutcome {
    match create_plan(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || render_plan_human(&envelope)),
        Err(error) => command_error_outcome("plan", &args, error),
    }
}

fn run_explain(args: CommandArgs) -> CliOutcome {
    match create_explanation(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || {
            render_explanation_human(&envelope)
        }),
        Err(error) => command_error_outcome("explain", &args, error),
    }
}

fn run_graph(args: CommandArgs) -> CliOutcome {
    match create_graph(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || render_graph_human(&envelope)),
        Err(error) => command_error_outcome("graph", &args, error),
    }
}

fn run_doctor(args: CommandArgs) -> CliOutcome {
    match create_doctor(&args.manifest_path, &args.workspace_id) {
        Ok(envelope) => success_outcome(args.format, &envelope, || render_doctor_human(&envelope)),
        Err(error) => doctor_error_outcome(&args, error),
    }
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
    args: &CommandArgs,
    error: ferris_core::CoreError,
) -> CliOutcome {
    let envelope: CommandEnvelope<serde_json::Value> =
        error_envelope(command, &args.workspace_id, &args.manifest_path, &error);
    error_outcome(&envelope)
}

fn doctor_error_outcome(args: &CommandArgs, error: ferris_core::CoreError) -> CliOutcome {
    let envelope: CommandEnvelope<serde_json::Value> =
        doctor_error_envelope(&args.workspace_id, &args.manifest_path, &error);
    error_outcome(&envelope)
}

fn error_outcome<T: Serialize>(envelope: &CommandEnvelope<T>) -> CliOutcome {
    CliOutcome {
        stdout: Vec::new(),
        stderr: serialize_line(envelope),
        process_exit_code: envelope.process_exit_code,
    }
}

fn serialize_line<T: Serialize>(value: &T) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("typed Ferris records must serialize");
    bytes.push(b'\n');
    bytes
}

fn guard_cli_execution(raw_args: &[OsString], execute: impl FnOnce() -> CliOutcome) -> CliOutcome {
    // Product execution is single-threaded; future in-process worker threads must
    // replace this process-global hook boundary with thread-owned error capture.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = catch_unwind(AssertUnwindSafe(execute));
    std::panic::set_hook(previous_hook);
    result.unwrap_or_else(|_| {
        let args = raw_args
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        let command = semantic_command_from_args(&args);
        let envelope = internal_cli_envelope(
            command,
            &args,
            "FERRIS-CLI-INTERNAL",
            "Ferris could not complete the command safely.",
            "Retry with the same arguments and report the Ferris version if the failure repeats.",
        );
        error_outcome(&envelope)
    })
}

fn emit_to(
    outcome: CliOutcome,
    raw_args: &[OsString],
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> u8 {
    if !outcome.stdout.is_empty() && stdout.write_all(&outcome.stdout).is_err() {
        let args = raw_args
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        let command = semantic_command_from_args(&args);
        let envelope = internal_cli_envelope(
            command,
            &args,
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

fn semantic_command_from_args(args: &[String]) -> &str {
    args.get(1)
        .map(String::as_str)
        .filter(|command| matches!(*command, "plan" | "explain" | "graph" | "doctor"))
        .unwrap_or("cli")
}

fn invalid_cli_envelope(
    semantic_command_id: &str,
    args: &[String],
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
            next_actions: vec!["Run ferris --help or ferris <command> --help.".to_owned()],
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

    #[test]
    fn panic_is_converted_to_typed_internal_outcome() {
        let args = vec![
            OsString::from("ferris"),
            OsString::from("doctor"),
            OsString::from("--workspace-id"),
            OsString::from("ferris.test/example"),
        ];

        let outcome = guard_cli_execution(&args, || panic!("injected test panic"));

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
        let outcome = CliOutcome {
            stdout: b"help".to_vec(),
            stderr: Vec::new(),
            process_exit_code: 0,
        };
        let mut stdout = FailingWriter;
        let mut stderr = Vec::new();

        let exit = emit_to(outcome, &args, &mut stdout, &mut stderr);

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
}
