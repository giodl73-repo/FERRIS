use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "afb0d367cc14a750b0afd25a643432c3e91031c1";
const AUTHORITY_CUTOFF: &str = "48c26aff381eb66459bf099559f0d44971d46f97";
const DECLARATION_IDENTITY: &str =
    "sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818";
const DECLARATION: &str =
    "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-68-authority.json";
const P57_EXECUTOR: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py";
const P58_README: &str =
    "docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release/README.md";
const P59_README: &str =
    "docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/README.md";

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
fn pulse_68_closeout_records_predecessor_cleanup_blocker_without_execution() {
    let authority_revision = format!("{AUTHORITY_COMMIT}^{{commit}}");
    let cutoff_revision = format!("{AUTHORITY_CUTOFF}^{{commit}}");
    assert_eq!(
        git_text(&["rev-parse", &authority_revision]),
        AUTHORITY_COMMIT
    );
    assert_eq!(git_text(&["rev-parse", &cutoff_revision]), AUTHORITY_CUTOFF);
    assert_eq!(
        git_text(&["merge-base", AUTHORITY_CUTOFF, AUTHORITY_COMMIT]),
        AUTHORITY_CUTOFF
    );

    let root = repo_root();
    let pulse = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-68.md"),
    )
    .expect("Pulse 68 wave record");
    let record = fs::read_to_string(root.join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_68_AUTHORITY.md",
    ))
    .expect("Pulse 68 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-predecessor-cleanup-contract",
            "P68-P57-STAGED-BUNDLE-CLEANUP",
            "_NativeWslSession.close()",
            "Pulse 58 and Pulse 59 overclaimed stack cleanup and zero residue",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 68 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Status: historical authority permanently withdrawn before launch",
            "P68-P57-STAGED-BUNDLE-CLEANUP",
            "stages a native `.p57-*` bundle",
            "never removes `staged.root`",
            "Pulse 58 claims",
            "Pulse 59 claims",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 68 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);

    let executor = String::from_utf8(git_blob(AUTHORITY_CUTOFF, P57_EXECUTOR))
        .expect("UTF-8 Pulse 57 executor");
    assert_contains_all(
        &executor,
        &[
            "staged = _stage_wsl_bundle(repo_root, ubuntu_runtime_parent)",
            "self._write(_canonical_line({\"schema\": WSL_SCHEMA, \"type\": \"close\"}))",
            "status = self._end_process()",
            "stdout, stderr = self._drain()",
        ],
        "Pulse 57 executor",
    );
    assert!(!executor.contains("self._staged"));
    assert!(!executor.contains("_cleanup_staged_bundle"));
    assert!(!executor.contains("bundle_root_absence_verified"));

    let p58 =
        String::from_utf8(git_blob(AUTHORITY_CUTOFF, P58_README)).expect("UTF-8 Pulse 58 README");
    assert_contains_all(
        &p58,
        &[
            "terminal path P58 closes both live capabilities/worker",
            "verifies absence",
        ],
        "Pulse 58 README",
    );
    let p59 =
        String::from_utf8(git_blob(AUTHORITY_CUTOFF, P59_README)).expect("UTF-8 Pulse 59 README");
    assert_contains_all(
        &p59,
        &[
            "Pulse 58 removes its private runtime root on every terminal path",
            "event list is therefore exact Pulse 58 output",
        ],
        "Pulse 59 README",
    );
}

#[test]
fn pulse_68_closeout_remains_zero_call_zero_artifact_null_conclusion() {
    let declaration = read_json(repo_root().join(DECLARATION));
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(
        declaration["execution_state"]["authority_callable_invocation_attempts"],
        0
    );
    assert_eq!(
        declaration["execution_state"]["authority_callable_invocations"],
        0
    );
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
    assert_eq!(
        declaration["execution_state"]["publication_success_claims"],
        0
    );
    assert_eq!(
        declaration["execution_state"]["category_conclusion"],
        Value::Null
    );
    assert_eq!(
        declaration["execution_state"]["diagnostic_conclusion"],
        Value::Null
    );
    assert_eq!(
        declaration["execution_state"]["product_conclusion"],
        Value::Null
    );
}
