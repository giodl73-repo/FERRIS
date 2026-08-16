use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "d220d42be02a3ed5beb69df1fa9c132bab0a9680";
const AUTHORITY_CUTOFF: &str = "2388b7d9a5fda7f9cbf772e12d1b4c07d22f2161";
const DECLARATION_IDENTITY: &str =
    "sha256:634e7b3197f5d550c6f3816dbf13770d44738c4f05de6956aa07966548a0be23";
const DECLARATION: &str =
    "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-64-authority.json";
const PULSE_57: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py";
const QUALIFICATION_CALLABLE: &str = "qualify_exact_p57_wsl_bootstrap_contract";

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
fn pulse_64_closeout_records_unbound_optional_callable_and_placeholder_derivation_without_execution(
) {
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
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-64.md"),
    )
    .expect("Pulse 64 wave record");
    let record = fs::read_to_string(
        root.join(
            "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_64_AUTHORITY.md",
        ),
    )
    .expect("Pulse 64 authority record");

    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-unbound-wsl-qualification-contract",
            "P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION",
            "qualify_exact_p57_wsl_bootstrap_contract",
            "literal `%SystemRoot%`",
            "No Pulse 59 callable was invoked.",
            "Every call, seed, descriptor,",
        ],
        "Pulse 64 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 64 is now permanently withdrawn before launch",
            "P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION",
            "nonexistent and unbound",
            "`_stage_wsl_bundle` plus `_NativeWslSession`",
            "`SystemRoot`/`SYSTEMROOT` derivation and identity proof remained underbound",
            "`%SystemRoot%\\\\System32\\\\wsl.exe`",
            "`%SystemRoot%\\\\System32\\\\cmd.exe`",
            "`System32\\\\wsl.exe` and `System32\\\\cmd.exe`",
            "No Pulse 59 callable was invoked.",
        ],
        "Pulse 64 authority record",
    );

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_zero_execution_state(&declaration["execution_state"]);
    assert_eq!(
        declaration["authority"]["pre_call_public_qualification_callable_may_prove_exact_p57_route_invariants"],
        true
    );
    assert_eq!(
        declaration["runtime_binding"]
            ["pre_call_public_qualification_callable_may_prove_exact_p57_route_invariants"],
        true
    );

    let preflight = &declaration["pre_call_public_prerequisites"]["exact_wsl_route_preflight"];
    assert_eq!(preflight["public_qualification_callable"]["allowed"], true);
    assert_eq!(
        preflight["public_qualification_callable"]["callable_name"],
        QUALIFICATION_CALLABLE
    );
    assert_eq!(
        preflight["resolved_windows_wsl_executable"],
        "%SystemRoot%\\System32\\wsl.exe"
    );
    assert_eq!(
        preflight["environment_exact_values"],
        serde_json::json!({
            "ComSpec": "%SystemRoot%\\System32\\cmd.exe",
            "PATH": "%SystemRoot%\\System32",
            "SystemRoot": "%SystemRoot%"
        })
    );

    let missing_marker = format!("def {QUALIFICATION_CALLABLE}(");
    for binding in declaration["api_bindings"].as_object().expect("API bindings").values() {
        let module_path = binding["module_path"].as_str().expect("module path");
        let source = String::from_utf8(git_blob(AUTHORITY_CUTOFF, module_path))
            .expect("UTF-8 cutoff API source");
        assert!(
            !source.contains(&missing_marker),
            "{module_path} must not define the nonexistent optional qualification callable"
        );
    }

    let p57 = String::from_utf8(git_blob(AUTHORITY_CUTOFF, PULSE_57)).expect("UTF-8 Pulse 57");
    assert_contains_all(
        &p57,
        &[
            "system_root = os.environ.get(\"SystemRoot\") or os.environ.get(\"SYSTEMROOT\")",
            "system32 = os.fspath(Path(system_root) / \"System32\")",
            "\"ComSpec\": os.fspath(Path(system32) / \"cmd.exe\")",
            "\"PATH\": system32",
            "\"SystemRoot\": system_root",
            "return os.fspath(Path(system_root) / \"System32\" / \"wsl.exe\")",
            "def _stage_wsl_bundle(repo_root: Path, runtime_parent: str) -> _StagedBundle:",
            "class _NativeWslSession:",
        ],
        "Pulse 57 source",
    );
    assert!(
        !p57.contains("%SystemRoot%"),
        "Pulse 57 source derives concrete paths rather than literal placeholders"
    );
}

#[test]
fn pulse_64_closeout_remains_zero_call_zero_artifact_null_conclusion() {
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
