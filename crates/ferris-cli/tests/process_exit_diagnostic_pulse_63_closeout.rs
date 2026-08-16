use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "81a50dcd6fd114a3968bc6e6f34b604bcb404780";
const AUTHORITY_CUTOFF: &str = "5ad78a0623611ad57797ec4e9da34345b40a6e38";
const DECLARATION_IDENTITY: &str =
    "sha256:b8cfea5cc8cb6dc52a7974f4fee35f6351557158943cc92af388c534421915d5";
const DECLARATION: &str =
    "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-63-authority.json";
const PULSE_56_MANIFEST: &str =
    "docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/public-manifest.json";
const PULSE_57: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py";
const PULSE_57_WORKER: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py";
const MISLEADING_DECLARED_P59_SCHEMA: &str =
    "repository/docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/schemas/ferris.pulse-59-witness-preserving-capability-materialization-executor.v1.schema.json";
const ACTUAL_P56_STAGE_SCHEMA: &str =
    "repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/schemas/ferris.pulse-56-retained-build-receipt.v1.schema.json";

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

fn staged_p56_paths() -> Vec<String> {
    let manifest: Value = serde_json::from_slice(&git_blob(AUTHORITY_CUTOFF, PULSE_56_MANIFEST))
        .expect("parse Pulse 56 manifest");
    let prefix =
        "repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/";
    let mut paths = vec![
        format!("{prefix}public-manifest.json"),
        format!("{prefix}qualification-receipt.json"),
        format!("{prefix}release-seal.json"),
    ];
    for entry in manifest["files"].as_array().expect("manifest files") {
        paths.push(format!(
            "{prefix}{}",
            entry["path"].as_str().expect("manifest path")
        ));
    }
    paths.sort();
    paths.dedup();
    paths
}

#[test]
fn pulse_63_closeout_records_exact_p57_bootstrap_contradiction_without_execution() {
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
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-63.md"),
    )
    .expect("Pulse 63 wave record");
    let record = fs::read_to_string(
        root.join("docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_63_AUTHORITY.md"),
    )
    .expect("Pulse 63 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-wsl-bootstrap-contract",
            "P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT",
            "Pulse 57 WSL bootstrap contract",
            "actual staged Pulse 56 release tree",
            "bundle_root == ubuntu_runtime_parent.rstrip(\"/\") + \"/\" + name",
            "No Pulse 59 callable was invoked.",
            "Every call, seed, descriptor,",
        ],
        "Pulse 63 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 63 is now permanently withdrawn before launch",
            "P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT",
            "Exact Pulse 57 bundle-bootstrap route remained underbound",
            "`worker/wsl_session_worker.py`",
            "`worker/sealed_dependencies.py`",
            "`repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/`",
            "`bundle_root`, `python`, and `schema`",
            "pulse-59-witness-preserving-capability-materialization-executor-release",
            "`_WSL_WORKER_BOOTSTRAP`",
            "`repo_root =",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 63 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);

    let pre = &declaration["pre_call_public_prerequisites"];
    let preflight = &pre["exact_wsl_route_preflight"];
    assert_eq!(preflight["input_json_max_bytes"], 16384);
    assert_eq!(preflight["result_json_max_bytes"], 65536);
    assert_eq!(preflight["process_timeout_seconds_max"], 60);
    assert_eq!(
        preflight["result_json_fields"],
        serde_json::json!([
            "bundle_probe_root",
            "p56_probe_root",
            "platform",
            "python",
            "schema"
        ])
    );
    assert_eq!(
        preflight["wsl_command_prefix"],
        serde_json::json!([
            "%SystemRoot%\\System32\\wsl.exe",
            "--distribution",
            "Ubuntu-24.04",
            "--exec",
            "/usr/bin/python3"
        ])
    );
    assert_eq!(preflight.get("bundle_bootstrap_argv_shape"), None);
    assert_eq!(
        pre["reversible_creatability_probes"]["ubuntu_runtime_parent"]["p57_bundle_exact_topology"]
            ["repository_deepest_exact_relative_path"],
        MISLEADING_DECLARED_P59_SCHEMA
    );

    let staged = staged_p56_paths();
    assert!(staged.iter().any(|path| path == ACTUAL_P56_STAGE_SCHEMA));
    assert_eq!(staged.len(), 10);
    let deepest = staged
        .iter()
        .max_by_key(|path| path.len())
        .expect("deepest staged path");
    assert_eq!(deepest, ACTUAL_P56_STAGE_SCHEMA);
    assert_ne!(deepest, &MISLEADING_DECLARED_P59_SCHEMA.to_owned());

    let p57 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57)).expect("UTF-8 Pulse 57");
    let worker =
        String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57_WORKER)).expect("UTF-8 worker");
    assert_contains_all(
        &p57,
        &[
            "MAX_BUNDLE_BYTES = 1_048_576",
            "MAX_PROTOCOL_BYTES = 2_800_000",
            "PROTOCOL_TIMEOUT_SECONDS = 15",
            "_WSL_BUNDLE_BOOTSTRAP = r\"\"\"",
            "payload = canonical_bytes({\"files\": files, \"schema\": BUNDLE_SCHEMA})",
            "if len(payload) > MAX_BUNDLE_BYTES:",
            "name = \".p57-\" + secrets.token_hex(16)",
            "expected_root = runtime_parent.rstrip(\"/\") + \"/\" + name",
            "\"bundle_root\":root,\"python\":{\"executable\":sys.executable,\"version\":list(sys.version_info[:3])},\"schema\":\"ferris.pulse-57-wsl-bundle-staged/v1\"",
            "_WSL_WORKER_BOOTSTRAP = r\"\"\"",
            "sys.argv=[worker,*sys.argv[3:]]",
            "staged.root + \"/worker/wsl_session_worker.py\"",
            "WSL_WORKER_SHA256",
            "\"--runtime-parent\"",
            "\"--bundle-root\"",
            "\"--p56-root\"",
            "pulse-56-retained-build-custody-release",
            "env=_wsl_environment()",
        ],
        "Pulse 57 source",
    );
    assert_contains_all(
        &worker,
        &[
            "source = bundle_root / \"worker\" / \"sealed_dependencies.py\"",
            "expected_p56 = (",
            "pulse-56-retained-build-custody-release",
            "dependencies = _load_sealed_dependencies(bundle_root)",
            "if p56_root != expected_p56:",
            "repo_root = p56_root.parents[3]",
        ],
        "Pulse 57 worker source",
    );
}

#[test]
fn pulse_63_closeout_remains_zero_call_zero_artifact_null_conclusion() {
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
    assert_eq!(declaration["execution_state"]["product_conclusion"], Value::Null);
}
