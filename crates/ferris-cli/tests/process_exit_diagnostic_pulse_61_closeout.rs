use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "53e3c86653a71171a9301dd5cff185a522af1231";
const AUTHORITY_CUTOFF: &str = "70ed752359c04e4aac77a49280c37f2cf6b8d012";
const DECLARATION_IDENTITY: &str =
    "sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d";
const DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-61-authority.json";
const PULSE_41: &str = "docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/transactional_copy.py";
const PULSE_51: &str = "docs/simulations/profile-diff-held-out/pulse-51-diagnostic-executor-release/diagnostic_executor.py";
const PULSE_52: &str = "docs/simulations/profile-diff-held-out/pulse-52-ordered-materialization-executor-release/ordered_materialization_executor.py";
const PULSE_56: &str = "docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/retained_build_custody.py";
const PULSE_57: &str = "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py";
const PULSE_58: &str = "docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release/ordered_capability_materialization_executor.py";
const PULSE_59: &str = "docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/witness_preserving_capability_materialization_executor.py";

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
fn pulse_61_closeout_records_exact_root_creatability_contradiction_without_execution() {
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
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-61.md"),
    )
    .expect("Pulse 61 wave record");
    let record = fs::read_to_string(
        root.join("docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_61_AUTHORITY.md"),
    )
    .expect("Pulse 61 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-root-creatability-contract",
            "P61-ROOT-CREATABILITY-CALLABLE-CONTRACT",
            "exact child-creation and",
            "Every call, seed, descriptor,",
            "process, publication, transfer, result, witness, and conclusion count",
        ],
        "Pulse 61 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 61 is now permanently withdrawn before launch under",
            "P61-ROOT-CREATABILITY-CALLABLE-CONTRACT",
            "`.pulse58-private-launch` namespace child",
            "Pulse 56's exact Windows",
            "same-filesystem stage→final rename",
            "path-length headroom",
            "Pulse 57 `.p57-*` bundle",
            "executable/noexec prerequisites",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 61 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_eq!(
        declaration["pre_call_public_prerequisites"].get("reversible_creatability_probes"),
        None
    );
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(declaration["execution_state"]["category_conclusion"], Value::Null);
    assert_eq!(declaration["execution_state"]["product_conclusion"], Value::Null);
    assert_eq!(declaration["execution_state"]["publication_result_transfers"], 0);
    assert_eq!(declaration["execution_state"]["publication_failure_witness_transfers"], 0);

    let p41 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_41)).expect("UTF-8 P41");
    let p51 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_51)).expect("UTF-8 P51");
    let p52 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_52)).expect("UTF-8 P52");
    let p56 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_56)).expect("UTF-8 P56");
    let p57 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57)).expect("UTF-8 P57");
    let p58 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_58)).expect("UTF-8 P58");
    let p59 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_59)).expect("UTF-8 P59");

    assert_contains_all(
        &p51,
        &[
            "def _safe_runtime_root(path: Path) -> Path:",
            "if stat.S_ISLNK(metadata.st_mode):",
        ],
        "Pulse 51 helper source",
    );
    assert_contains_all(
        &p58,
        &[
            "PRIVATE_NAMESPACE = \".pulse58-private-launch\"",
            "namespace = _runtime_path(p51, runtime_root, runtime_root / PRIVATE_NAMESPACE, \"P58-PRIVATE-ROOT\", absent=True)",
            "if namespace.parent != runtime_root or cycle.parent != runtime_root",
            "os.mkdir(namespace, 0o700)",
            "windows_handle = p56.publish_retained_build_and_custody(\"windows-x86_64\", runtime_root)",
        ],
        "Pulse 58 helper source",
    );
    assert_contains_all(
        &p52,
        &[
            "stage_root = final_root.parent / f\".{final_root.name}.pulse-41-stage\"",
            "for candidate in (final_root, stage_root)",
        ],
        "Pulse 52 helper source",
    );
    assert_contains_all(
        &p41,
        &[
            "staging_root = final_parent / f\".{final_before_publication.name}.pulse-41-stage\"",
            "renamer(staging_root, final_before_publication)",
            "state.rollback_attempted = True",
            "remover(final_root)",
        ],
        "Pulse 41 helper source",
    );
    assert_contains_all(
        &p59,
        &[
            "candidate = parent / f\"{runtime_root.name}{TERMINAL_ROOT_SUFFIX}\"",
            "if candidate == runtime_root or os.path.lexists(candidate)",
            "os.mkdir(terminal_parent, 0o700)",
        ],
        "Pulse 59 helper source",
    );
    assert_contains_all(
        &p57,
        &[
            "def _native_wsl_parent(value: str) -> None:",
            "or value.startswith(\"/mnt/\")",
            "name = \".p57-\" + secrets.token_hex(16)",
            "expected_root = runtime_parent.rstrip(\"/\") + \"/\" + name",
        ],
        "Pulse 57 helper source",
    );
    assert_contains_all(
        &p56,
        &[
            "parent = _safe_existing_directory(Path(os.fspath(runtime_parent)), \"P56-RUNTIME-PARENT\")",
            "run = _fresh_child(parent, f\".p56-{secrets.token_hex(12)}\", \"P56-RUNTIME-ROOT\")",
            "run_id = _mkdir_exclusive(run, \"P56-RUNTIME-ROOT\")",
        ],
        "Pulse 56 helper source",
    );
}

#[test]
fn pulse_61_closeout_remains_zero_call_zero_artifact_null_conclusion() {
    let declaration = read_json(repo_root().join(DECLARATION));
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(declaration["execution_state"]["authority_callable_invocation_attempts"], 0);
    assert_eq!(declaration["execution_state"]["authority_callable_invocations"], 0);
    assert_eq!(declaration["execution_state"]["authority_consumptions"], 0);
    assert_eq!(declaration["execution_state"]["p59_invocations"], 0);
    assert_eq!(declaration["execution_state"]["private_seed_created"], false);
    assert_eq!(declaration["execution_state"]["private_descriptor_root_created"], false);
    assert_eq!(declaration["execution_state"]["runtime_root_created"], false);
    assert_eq!(declaration["execution_state"]["terminal_root_created"], false);
    assert_eq!(declaration["execution_state"]["publication_result_transfers"], 0);
    assert_eq!(declaration["execution_state"]["publication_failure_witness_transfers"], 0);
    assert_eq!(declaration["execution_state"]["publication_success_claims"], 0);
    assert_eq!(declaration["execution_state"]["category_conclusion"], Value::Null);
    assert_eq!(declaration["execution_state"]["product_conclusion"], Value::Null);
}