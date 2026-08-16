use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "6b46ef8669f211a4daf5d79b7e40f33365d5d783";
const AUTHORITY_CUTOFF: &str = "3ec6a36009fd34765508f729e795042fd610e5d4";
const PRIOR_CUTOFF: &str = "3a99e9e0f383a9821297ef47778fd586b447b7ba";
const DECLARATION_IDENTITY: &str =
    "sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05";
const DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json";
const P57_WORKER: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py";
const P67_PROBE_WORKER: &str =
    "docs/simulations/profile-diff-held-out/fixtures/p67_wsl_probe_worker.py";
const P67_PROBE_DEPENDENCY: &str =
    "docs/simulations/profile-diff-held-out/fixtures/p67_wsl_probe_sealed_dependencies.py";

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
fn pulse_67_closeout_records_stale_cutoff_fields_and_missing_exact_p56_loader_leg() {
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
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-67.md"),
    )
    .expect("Pulse 67 wave record");
    let record = fs::read_to_string(root.join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_67_AUTHORITY.md",
    ))
    .expect("Pulse 67 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-cutoff-probe-claim-contract",
            "P67-ROOT-CUTOFF-P56-LOADER-CONTRACT",
            "authority_checkout_root.revision",
            "p39_checkout_root.head",
            "repo_root.revision",
            "load_exact_p56(repo_root)",
            "Path(p56.__file__).parent == p56_root",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 67 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 67 is now permanently withdrawn before launch",
            "P67-ROOT-CUTOFF-P56-LOADER-CONTRACT",
            "authority_checkout_root.revision",
            "p39_checkout_root.head",
            "repo_root = p56_root.parents[3]",
            "load_exact_p56(repo_root)",
            "Path(p56.__file__).parent == p56_root",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 67 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);

    let prerequisites = &declaration["pre_call_public_prerequisites"];
    assert_eq!(
        prerequisites["authority_checkout_root"]["revision"],
        PRIOR_CUTOFF
    );
    assert_eq!(prerequisites["p39_checkout_root"]["head"], PRIOR_CUTOFF);
    assert_eq!(prerequisites["p39_checkout_root"]["revision"], PRIOR_CUTOFF);
    assert_eq!(prerequisites["repo_root"]["revision"], PRIOR_CUTOFF);
    assert_ne!(prerequisites["authority_checkout_root"]["revision"], AUTHORITY_CUTOFF);
    assert_ne!(prerequisites["p39_checkout_root"]["head"], AUTHORITY_CUTOFF);
    assert_ne!(prerequisites["p39_checkout_root"]["revision"], AUTHORITY_CUTOFF);
    assert_ne!(prerequisites["repo_root"]["revision"], AUTHORITY_CUTOFF);

    let probe_scope =
        &prerequisites["exact_wsl_route_preflight"]["worker_bootstrap_route"]["probe_worker_scope"];
    assert!(
        probe_scope["exact_route_equivalence_semantics_proven"]
            .as_array()
            .expect("probe semantics")
            .iter()
            .any(|value| value == "exact-p56-root-equality-check-and-repo-root-parent-derivation")
    );

    let production_worker =
        String::from_utf8(git_blob(AUTHORITY_CUTOFF, P57_WORKER)).expect("UTF-8 P57 worker");
    let historical_worker =
        String::from_utf8(git_blob(AUTHORITY_CUTOFF, P67_PROBE_WORKER)).expect("UTF-8 P67 worker");
    let historical_dependency = String::from_utf8(git_blob(AUTHORITY_CUTOFF, P67_PROBE_DEPENDENCY))
        .expect("UTF-8 P67 dependency");

    assert_contains_all(
        &production_worker,
        &[
            "repo_root = p56_root.parents[3]",
            "p56 = dependencies.load_exact_p56(repo_root)",
            "if Path(p56.__file__).parent != p56_root:",
        ],
        "Pulse 57 worker source",
    );
    assert!(!historical_worker.contains("repo_root = p56_root.parents[3]"));
    assert!(!historical_worker.contains("dependencies.load_exact_p56(repo_root)"));
    assert!(!historical_worker.contains("Path(p56.__file__).parent != p56_root"));
    assert!(!historical_dependency.contains("def load_exact_p56(repo_root: Path) -> ModuleType:"));
    assert!(historical_dependency.contains("EXACT_P56_RELEASE_FILES"));
    assert!(historical_dependency.contains("def bind_probe_context("));
}

#[test]
fn pulse_67_closeout_remains_zero_call_zero_artifact_null_conclusion() {
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
