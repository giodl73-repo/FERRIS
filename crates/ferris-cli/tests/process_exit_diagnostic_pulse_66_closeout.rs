use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "1ac79ec4ebec0f2870d9b917403532569c15267a";
const AUTHORITY_CUTOFF: &str = "3a99e9e0f383a9821297ef47778fd586b447b7ba";
const DECLARATION_IDENTITY: &str =
    "sha256:2cf44e16b0c61d79ed5ac889ab6fbfe46ee693ce6d9ccf2b4528bb877db45034";
const DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-66-authority.json";
const P57_WORKER: &str = "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py";
const P57_DEPENDENCY: &str = "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py";
const P66_PROBE_DEPENDENCY: &str =
    "docs/simulations/profile-diff-held-out/fixtures/p66_wsl_probe_sealed_dependencies.py";
const P57_DEPENDENCY_SHA256: &str =
    "sha256:fe36a56a10d5d3659fae9cfacc3cd48075aaf0e3327ae029a2470d1107da6c8d";
const P66_PROBE_DEPENDENCY_SHA256: &str =
    "sha256:a8f1c2e089cc5c1f32245e78ec6f574e0fc1ef4b25b3f7a341dcad058c614269";

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

fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("sha256:{:x}", Sha256::digest(bytes))
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
fn pulse_66_closeout_records_worker_hash_and_bundle_lifetime_blockers_without_execution() {
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
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-66.md"),
    )
    .expect("Pulse 66 wave record");
    let record = fs::read_to_string(root.join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_66_AUTHORITY.md",
    ))
    .expect("Pulse 66 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-wsl-probe-bundle-contract",
            "P66-WORKER-HASH-BUNDLE-LIFETIME",
            "Exact worker hash rejected the fake dependency bundle",
            "Spawn 1 cleanup contradicted the required bundle handoff",
            "No Pulse 59 callable was invoked.",
            "Every call, seed, descriptor, process,",
        ],
        "Pulse 66 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 66 is now permanently withdrawn before launch",
            "P66-WORKER-HASH-BUNDLE-LIFETIME",
            "fake dependency bytes do not match the",
            "exact worker's sealed hash",
            "`bundle_root` to remain available",
            "cleanup occurs",
            "once after both WSL spawns finish",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 66 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);

    let preflight = &declaration["pre_call_public_prerequisites"]["exact_wsl_route_preflight"];
    assert_eq!(preflight["invocations_exact"], 1);
    assert_eq!(preflight["wsl_process_spawns_exact"], 2);
    assert_eq!(
        preflight["stage_bundle_route"]["cleanup_same_invocation_required"],
        true
    );
    assert_eq!(
        preflight["worker_bootstrap_route"]["probe_bundle"]["host_side_prestaging_only"],
        true
    );
    assert_eq!(
        preflight["worker_bootstrap_route"]["probe_bundle"]["fake_dependency_source_cutoff_path"],
        P66_PROBE_DEPENDENCY
    );

    let worker = String::from_utf8(git_blob(AUTHORITY_CUTOFF, P57_WORKER)).expect("UTF-8 worker");
    let production_dependency = git_blob(AUTHORITY_CUTOFF, P57_DEPENDENCY);
    let probe_dependency = git_blob(AUTHORITY_CUTOFF, P66_PROBE_DEPENDENCY);
    assert_eq!(sha256(&production_dependency), P57_DEPENDENCY_SHA256);
    assert_eq!(sha256(&probe_dependency), P66_PROBE_DEPENDENCY_SHA256);
    assert_ne!(probe_dependency, production_dependency);
    assert_contains_all(
        &worker,
        &[
            r#"SEALED_DEPENDENCIES_SHA256 = "sha256:fe36a56a10d5d3659fae9cfacc3cd48075aaf0e3327ae029a2470d1107da6c8d""#,
            r#"content = _safe_regular(source, "P57-WSL-BUNDLE")"#,
            r#"if _sha256_bytes(content) != SEALED_DEPENDENCIES_SHA256:"#,
            r#"raise WorkerFailure("P57-WSL-BUNDLE")"#,
        ],
        "Pulse 57 worker source",
    );
}

#[test]
fn pulse_66_closeout_remains_zero_call_zero_artifact_null_conclusion() {
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
