use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "3f7619244420b2ba7762bfc2b1b119d7b1a294a2";
const AUTHORITY_CUTOFF: &str = "e38dd20f37923e84ac3a3377892c1a5d0954266a";
const DECLARATION_IDENTITY: &str =
    "sha256:f0db3ddf18a796d0ec107d6d73e9a08cf5e59d47cdad880d584ee8c7e8f61c5a";
const DECLARATION: &str =
    "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority.json";
const PULSE_41: &str =
    "docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/transactional_copy.py";
const PULSE_56: &str =
    "docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/retained_build_custody.py";
const PULSE_57: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py";
const PULSE_59: &str =
    "docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/witness_preserving_capability_materialization_executor.py";

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
fn pulse_62_closeout_records_exact_path_and_wsl_route_contradiction_without_execution() {
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
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-62.md"),
    )
    .expect("Pulse 62 wave record");
    let record = fs::read_to_string(
        root.join("docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_62_AUTHORITY.md"),
    )
    .expect("Pulse 62 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-path-route-contract",
            "P62-REAL-PATH-WSL-ROUTE-CONTRACT",
            "actual caller-supplied root basenames",
            "deepest real Pulse 39 relative path",
            "exact harmless WSL route preflight",
            "No Pulse 59 callable was invoked.",
            "Every call, seed, descriptor, process,",
        ],
        "Pulse 62 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 62 is now permanently withdrawn before launch",
            "P62-REAL-PATH-WSL-ROUTE-CONTRACT",
            "actual caller-supplied basenames",
            "`<private_runtime_root.name>.pulse59-terminal-publication`",
            "Windows `wsl.exe`",
            "`Ubuntu-24.04`",
            "`/usr/bin/python3`",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 62 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);

    let pre = &declaration["pre_call_public_prerequisites"];
    let probes = &pre["reversible_creatability_probes"];
    assert_eq!(
        probes["p41_final_parent"]["probe_payload_relative_path"],
        "payload/probe.marker"
    );
    assert_eq!(
        probes["p59_terminal_parent"]["terminal_probe_child_name"],
        "pulse62-probe-e38dd20f3792-6945f5fc9686.pulse59-terminal-publication.probe"
    );
    assert_eq!(
        probes["ubuntu_runtime_parent"]["p57_bundle_probe_child_name"],
        ".p57-probe-e38dd20f3792-6945f5fc9686"
    );
    assert_eq!(
        probes["ubuntu_runtime_parent"]["p56_ubuntu_probe_child_name"],
        ".p56-probe-e38dd20f3792-6945f5fc9686-ubuntu"
    );
    assert_eq!(pre.get("exact_wsl_route_preflight"), None);

    let p41 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_41)).expect("UTF-8 P41");
    let p56 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_56)).expect("UTF-8 P56");
    let p57 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57)).expect("UTF-8 P57");
    let p59 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_59)).expect("UTF-8 P59");

    assert_contains_all(
        &p41,
        &[
            r#"staging_root = final_parent / f".{final_before_publication.name}.pulse-41-stage""#,
            "renamer(staging_root, final_before_publication)",
            "remover(final_root)",
        ],
        "Pulse 41 helper source",
    );
    assert_contains_all(
        &p59,
        &[
            r#"candidate = parent / f"{runtime_root.name}{TERMINAL_ROOT_SUFFIX}""#,
            "if candidate == runtime_root or os.path.lexists(candidate)",
        ],
        "Pulse 59 helper source",
    );
    assert_contains_all(
        &p57,
        &[
            r#"return os.fspath(Path(system_root) / "System32" / "wsl.exe")"#,
            r#""Ubuntu-24.04""#,
            r#""/usr/bin/python3""#,
            "if completed.returncode != 0 or completed.stderr:",
        ],
        "Pulse 57 helper source",
    );
    assert_contains_all(
        &p56,
        &[
            r#"run = _fresh_child(parent, f".p56-{secrets.token_hex(12)}", "P56-RUNTIME-ROOT")"#,
            r#"checkout = _fresh_child(work, f"checkout-{label}", "P56-WORK-ROOT")"#,
            r#"target = _fresh_child(work, f"target-{label}", "P56-WORK-ROOT")"#,
        ],
        "Pulse 56 helper source",
    );
}

#[test]
fn pulse_62_closeout_remains_zero_call_zero_artifact_null_conclusion() {
    let declaration = read_json(repo_root().join(DECLARATION));
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(
        declaration["execution_state"]["authority_callable_invocation_attempts"],
        0
    );
    assert_eq!(declaration["execution_state"]["authority_callable_invocations"], 0);
    assert_eq!(declaration["execution_state"]["authority_consumptions"], 0);
    assert_eq!(declaration["execution_state"]["p59_invocations"], 0);
    assert_eq!(declaration["execution_state"]["private_seed_created"], false);
    assert_eq!(
        declaration["execution_state"]["private_descriptor_root_created"],
        false
    );
    assert_eq!(declaration["execution_state"]["runtime_root_created"], false);
    assert_eq!(declaration["execution_state"]["terminal_root_created"], false);
    assert_eq!(declaration["execution_state"]["publication_result_transfers"], 0);
    assert_eq!(
        declaration["execution_state"]["publication_failure_witness_transfers"],
        0
    );
    assert_eq!(declaration["execution_state"]["publication_success_claims"], 0);
    assert_eq!(declaration["execution_state"]["category_conclusion"], Value::Null);
    assert_eq!(declaration["execution_state"]["product_conclusion"], Value::Null);
}
