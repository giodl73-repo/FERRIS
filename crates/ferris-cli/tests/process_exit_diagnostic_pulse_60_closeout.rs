use serde_json::Value;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "a06fc0bc189eb41f57309b5a4656033ea28f7198";
const AUTHORITY_CUTOFF: &str = "6945f5fc96868c97267a1635fbb5219cc398eeb4";
const DECLARATION_IDENTITY: &str =
    "sha256:13ba3aaa5d61c536a9dd22b3a57816b1b7d93c2e11592c87117190709cbfb40c";
const DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-60-authority.json";
const SCHEMA: &str = "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-60-authority.v1.schema.json";
const MUTATIONS: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-60-authority-mutations.json";
const PULSE_41: &str = "docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/transactional_copy.py";
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

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
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
fn pulse_60_authority_artifacts_remain_exact_historical_prelaunch_record() {
    let authority_revision = format!("{AUTHORITY_COMMIT}^{{commit}}");
    let cutoff_revision = format!("{AUTHORITY_CUTOFF}^{{commit}}");
    assert_eq!(
        git_text(&["rev-parse", &authority_revision]),
        AUTHORITY_COMMIT
    );
    assert_eq!(git_text(&["rev-parse", &cutoff_revision]), AUTHORITY_CUTOFF);
    assert_eq!(
        git_text(&["merge-base", AUTHORITY_CUTOFF, AUTHORITY_COMMIT]),
        AUTHORITY_CUTOFF,
        "the historical cutoff predates its authority"
    );

    let root = repo_root();
    for path in [DECLARATION, SCHEMA, MUTATIONS] {
        assert_eq!(
            read_lf(root.join(path)),
            git_blob(AUTHORITY_COMMIT, path),
            "{path} must remain the exact historical authority artifact"
        );
    }

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["authority_present_at_cutoff"],
        false
    );
    assert_eq!(
        declaration["pre_call_public_prerequisites"]["fresh_roots"]["private_runtime_root"],
        "fresh-absent"
    );
    assert_eq!(
        declaration["pre_call_public_prerequisites"]["fresh_roots"]["p27_cycle_root"],
        "fresh"
    );
    assert_zero_execution_state(&declaration["execution_state"]);
}

#[test]
fn pulse_60_closeout_records_exact_root_contract_contradiction_without_execution() {
    let root = repo_root();
    let pulse = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-60.md"),
    )
    .expect("Pulse 60 wave record");
    let record = fs::read_to_string(root.join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_60_AUTHORITY.md",
    ))
    .expect("Pulse 60 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-runtime-root-contract",
            "P60-RUNTIME-ROOT-CALLABLE-CONTRACT",
            "Every call, seed, descriptor, process,",
            "publication, transfer, result, witness, and conclusion count remains zero or",
        ],
        "Pulse 60 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 60 is now permanently withdrawn before launch under",
            "P60-RUNTIME-ROOT-CALLABLE-CONTRACT",
            "Pulse 60 declared `private_runtime_root` as `fresh-absent`",
            "Pulse 60 declared `p27_cycle_root` only as `fresh`",
            "No Pulse 59 callable was invoked.",
            "No direct Pulse 58, Pulse 57, Pulse 56,",
        ],
        "Pulse 60 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    let p41 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_41)).expect("UTF-8 P41");
    let p52 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_52)).expect("UTF-8 P52");
    let p56 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_56)).expect("UTF-8 P56");
    let p57 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57)).expect("UTF-8 P57");
    let p58 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_58)).expect("UTF-8 P58");
    let p59 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_59)).expect("UTF-8 P59");

    assert_contains_all(
        &p58,
        &[
            "def _assert_runtime_fresh",
            "runtime_root = p51._safe_runtime_root(value)",
            "if next(os.scandir(runtime_root), None) is not None",
            "cycle.parent != runtime_root",
            "os.path.lexists(cycle)",
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
            "if _is_within(final_root, source_root) or _is_within(source_root, final_root)",
        ],
        "Pulse 41 helper source",
    );
    assert_contains_all(
        &p59,
        &[
            "candidate = parent / f\"{runtime_root.name}{TERMINAL_ROOT_SUFFIX}\"",
            "if candidate == runtime_root or os.path.lexists(candidate)",
        ],
        "Pulse 59 helper source",
    );
    assert_contains_all(
        &p57,
        &["def _native_wsl_parent", "or value.startswith(\"/mnt/\")"],
        "Pulse 57 helper source",
    );
    assert_contains_all(
        &p56,
        &[
            "parent = _safe_existing_directory(Path(os.fspath(runtime_parent)), \"P56-RUNTIME-PARENT\")",
            "run = _fresh_child(parent, f\".p56-",
        ],
        "Pulse 56 helper source",
    );

    assert_eq!(
        declaration["pre_call_public_prerequisites"]["fresh_roots"]["private_runtime_root"],
        "fresh-absent"
    );
    assert_eq!(
        declaration["pre_call_public_prerequisites"]["fresh_roots"]["p27_cycle_root"],
        "fresh"
    );
    assert_eq!(
        declaration["pre_call_public_prerequisites"]["fresh_roots"]["p41_final_copy_root"],
        "fresh-absent"
    );
}

#[test]
fn pulse_60_withdrawal_remains_zero_call_zero_artifact_null_conclusion() {
    let root = repo_root();
    let declaration = read_json(root.join(DECLARATION));
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(
        declaration["execution_state"]["category_conclusion"],
        Value::Null
    );
    assert_eq!(
        declaration["execution_state"]["product_conclusion"],
        Value::Null
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
        declaration["execution_state"]["terminal_invalid_witness_publications"],
        0
    );
    assert_eq!(
        declaration["execution_state"]["terminal_cleanup_indeterminate"],
        false
    );

    for destination in [
        declaration["terminal_transfer_contract"]["published_result"]["p43_destination"]
            .as_str()
            .expect("P43 destination"),
        declaration["terminal_transfer_contract"]["published_result"]["p47_destination"]
            .as_str()
            .expect("P47 destination"),
        declaration["terminal_transfer_contract"]["published_failure_witness"]["destination"]
            .as_str()
            .expect("failure destination"),
    ] {
        assert!(
            !root.join(destination.trim_end_matches('/')).exists(),
            "withdrawn Pulse 60 retains no runtime transfer root"
        );
    }

    let pulse = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-60.md"),
    )
    .expect("Pulse 60 wave record");
    let record = fs::read_to_string(root.join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_60_AUTHORITY.md",
    ))
    .expect("Pulse 60 authority record");
    assert_contains_all(
        &pulse,
        &[
            "non-retryable",
            "non-resumable",
            "Every call, seed, descriptor",
        ],
        "Pulse 60 wave record closeout",
    );
    assert_contains_all(
        &record,
        &[
            "runtime root, seed, descriptor, candidate process, publication root, result",
            "Every call, seed, descriptor, process,",
            "publication, transfer, result, and witness count remains zero. Category,",
        ],
        "Pulse 60 authority record closeout",
    );

    let bytes = read_lf(root.join(DECLARATION));
    assert_eq!(
        sha256(&bytes),
        sha256(&git_blob(AUTHORITY_COMMIT, DECLARATION)),
        "withdrawal does not amend the historical declaration artifact"
    );
}
