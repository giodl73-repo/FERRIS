use clap::{Parser, Subcommand, ValueEnum, error::ErrorKind};
use ferris_core::{
    CommandEnvelope, Diagnostic, ResultClass, command_envelope, command_line_invocation_identity,
    command_line_selection_identity, create_doctor, create_explanation, create_graph, create_plan,
    create_profile_diff, doctor_error_envelope, error_envelope, profile_diff_error_envelope,
    render_doctor_human, render_explanation_human, render_graph_human, render_plan_human,
    render_profile_diff_human,
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
    ProfileDiff(ProfileDiffArgs),
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

#[derive(clap::Args)]
struct ProfileDiffArgs {
    #[arg(long, value_name = "PROFILE_JSON")]
    before: PathBuf,

    #[arg(long, value_name = "PROFILE_JSON")]
    after: PathBuf,

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
        FerrisCommand::ProfileDiff(args) => run_profile_diff(args),
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
        .filter(|command| {
            matches!(
                *command,
                "plan" | "explain" | "graph" | "doctor" | "profile-diff"
            )
        })
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

        let outcome = dispatch(&args);

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

        let outcome = dispatch(&args);
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

        let outcome = dispatch(&args);

        assert_eq!(outcome.process_exit_code, 5);
        assert!(outcome.stdout.is_empty());
        let value: serde_json::Value =
            serde_json::from_slice(&outcome.stderr).expect("typed incomplete result");
        assert_eq!(value["semantic_command_id"], "profile-diff");
        assert_eq!(value["result_class"], "incomplete");
        assert_eq!(value["record"], serde_json::Value::Null);
    }
}
