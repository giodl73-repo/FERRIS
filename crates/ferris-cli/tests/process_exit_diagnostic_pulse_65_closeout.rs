use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "5277cbe5188ef043673aa068a5e8d0adbc7e08a0";
const AUTHORITY_CUTOFF: &str = "e3b0b62f6dd62b5071886d32a9eedca85c76b4ae";
const DECLARATION_IDENTITY: &str =
    "sha256:5bd7c876180a3bfb9f0bcb1518ef68921d1b28210d1f717c904753508e28abb0";
const DECLARATION: &str =
    "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-65-authority.json";
const PULSE_57: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&read_lf(path)).expect("parse JSON")
}

fn git_output(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run Git")
}

fn git_text(args: &[&str]) -> String {
    let output = git_output(args);
    assert!(
        output.status.success(),
        "Git command failed {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("Git output is UTF-8")
        .trim()
        .to_owned()
}

fn git_blob(revision: &str, relative_path: &str) -> Vec<u8> {
    let spec = format!("{revision}:{relative_path}");
    let output = git_output(&["show", &spec]);
    assert!(
        output.status.success(),
        "missing Git blob {spec}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn assert_zero_execution_state(value: &Value) {
    for (field, state) in value.as_object().expect("execution state") {
        match state {
            Value::Number(number) => assert_eq!(number.as_i64(), Some(0), "{field} zero"),
            Value::Bool(boolean) => assert!(!boolean, "{field} false"),
            Value::Null => {}
            other => panic!("state {field} must be zero, false, or null: {other}"),
        }
    }
}

fn assert_contains_all(text: &str, snippets: &[&str], label: &str) {
    for snippet in snippets {
        assert!(text.contains(snippet), "{label} missing {snippet}");
    }
}

#[test]
fn pulse_65_closeout_records_exact_two_spawn_requirement_without_execution() {
    let authority_revision = format!("{AUTHORITY_COMMIT}^{{commit}}");
    let cutoff_revision = format!("{AUTHORITY_CUTOFF}^{{commit}}");
    assert_eq!(git_text(&["rev-parse", &authority_revision]), AUTHORITY_COMMIT);
    assert_eq!(git_text(&["rev-parse", &cutoff_revision]), AUTHORITY_CUTOFF);
    assert_eq!(
        git_text(&["merge-base", AUTHORITY_CUTOFF, AUTHORITY_COMMIT]),
        AUTHORITY_CUTOFF
    );

    let root = repo_root();
    let pulse = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-65.md"),
    )
    .expect("Pulse 65 wave record");
    let record = fs::read_to_string(
        root.join(
            "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_65_AUTHORITY.md",
        ),
    )
    .expect("Pulse 65 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-wsl-spawn-cardinality-contract",
            "P65-P57-WSL-TWO-SPAWN-CONTRACT",
            "`single_wsl_process_spawn` at true",
            "`subprocess.run(...)`",
            "`subprocess.Popen(...)`",
            "No Pulse 59 callable was invoked.",
            "Every call, seed, descriptor,",
        ],
        "Pulse 65 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 65 is now permanently withdrawn before launch",
            "P65-P57-WSL-TWO-SPAWN-CONTRACT",
            "two distinct WSL process spawns",
            "`subprocess.run(...)`",
            "`subprocess.Popen(...)`",
            "did not bind a distinct second harmless worker-bootstrap proof",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 65 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);

    let preflight = &declaration["pre_call_public_prerequisites"]["exact_wsl_route_preflight"];
    assert_eq!(preflight["invocations_exact"], 1);
    assert_eq!(preflight["single_wsl_process_spawn"], true);
    assert_eq!(
        preflight["exact_stage_bundle_and_worker_bootstrap_route_only"],
        true
    );
    assert_eq!(
        preflight["stage_bundle_route"]["inline_bootstrap_symbol"],
        "_WSL_BUNDLE_BOOTSTRAP"
    );
    assert_eq!(
        preflight["worker_bootstrap_route"]["inline_bootstrap_symbol"],
        "_WSL_WORKER_BOOTSTRAP"
    );

    let p57 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57)).expect("UTF-8 Pulse 57");
    assert_contains_all(
        &p57,
        &[
            "staged = _stage_wsl_bundle(repo_root, ubuntu_runtime_parent)",
            "completed = subprocess.run(",
            "self._process = subprocess.Popen(",
            "ready = self._read()",
            "self._process.terminate",
            "self._process.kill",
            "self._write(_canonical_line({\"schema\": WSL_SCHEMA, \"type\": \"close\"}))",
        ],
        "Pulse 57 source",
    );
    assert_eq!(p57.matches("subprocess.run(").count(), 1);
    assert_eq!(p57.matches("subprocess.Popen(").count(), 1);
}

#[test]
fn pulse_65_closeout_remains_zero_call_zero_artifact_null_conclusion() {
    let declaration = read_json(repo_root().join(DECLARATION));
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(
        declaration["execution_state"]["authority_callable_invocation_attempts"],
        0
    );
    assert_eq!(declaration["execution_state"]["authority_callable_invocations"], 0);
    assert_eq!(declaration["execution_state"]["authority_consumptions"], 0);
    assert_eq!(declaration["execution_state"]["p59_invocations"], 0);
    assert_eq!(
        declaration["execution_state"]["exact_wsl_route_preflight_invocations"],
        0
    );
    assert_eq!(
        declaration["execution_state"]["exact_wsl_route_preflight_wsl_processes"],
        0
    );
    assert_eq!(
        declaration["execution_state"]["publication_result_transfers"],
        0
    );
    assert_eq!(
        declaration["execution_state"]["publication_failure_witness_transfers"],
        0
    );
    assert_eq!(declaration["execution_state"]["publication_success_claims"], 0);
    assert_eq!(declaration["execution_state"]["category_conclusion"], Value::Null);
    assert_eq!(declaration["execution_state"]["diagnostic_conclusion"], Value::Null);
    assert_eq!(declaration["execution_state"]["product_conclusion"], Value::Null);
}
