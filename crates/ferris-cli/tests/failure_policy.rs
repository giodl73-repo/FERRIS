use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const POLICY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/failure-policy/cargo-owner-policy.json"
);
const DEPENDENCY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/cargo-failure/dependency.stderr"
);
const LOCKFILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/cargo-failure/lockfile.stderr"
);
const OFFLINE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/cargo-failure/offline.stderr"
);
const POLICY_SCHEMA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/failure-policy/ferris.failure-policy.v1.schema.json"
);
const DECISION_SCHEMA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/failure-policy/ferris.failure-policy-decision.v1.schema.json"
);

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn cargo_ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cargo-ferris"))
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
}

fn diagnose(path: &str) -> Vec<u8> {
    let output = ferris()
        .args(["diagnose-cargo", "--stderr", path, "--format", "json"])
        .output()
        .expect("run Cargo failure diagnosis");
    assert_success(&output);
    output.stdout
}

fn evaluate(command: &mut Command, policy: &Path, diagnosis: &Path) -> Output {
    command
        .arg("failure-policy")
        .arg("--policy")
        .arg(policy)
        .arg("--diagnosis")
        .arg(diagnosis)
        .args(["--format", "json"])
        .output()
        .expect("run failure policy")
}

fn evaluate_stdin(command: &mut Command, policy: &Path, diagnosis: &[u8]) -> Output {
    let mut child = command
        .arg("failure-policy")
        .arg("--policy")
        .arg(policy)
        .args(["--diagnosis", "-", "--format", "json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start failure policy");
    child
        .stdin
        .take()
        .expect("piped standard input")
        .write_all(diagnosis)
        .expect("write diagnosis");
    child.wait_with_output().expect("finish failure policy")
}

#[test]
fn matched_policy_is_deterministic_private_and_passive_across_entrypoints() {
    let directory = TestDirectory::new();
    let diagnosis = diagnose(DEPENDENCY);
    let diagnosis_path = directory.path.join("diagnosis.json");
    fs::write(&diagnosis_path, &diagnosis).expect("write diagnosis");
    let arguments = [
        "failure-policy",
        "--policy",
        POLICY,
        "--diagnosis",
        diagnosis_path.to_str().expect("UTF-8 temporary path"),
        "--format",
        "json",
    ];
    let direct = ferris().args(arguments).output().expect("run ferris");
    let cargo_direct = cargo_ferris()
        .args(arguments)
        .output()
        .expect("run cargo-ferris");
    let cargo_style = cargo_ferris()
        .args(std::iter::once("ferris").chain(arguments))
        .output()
        .expect("run cargo ferris");
    assert_success(&direct);
    assert_success(&cargo_direct);
    assert_success(&cargo_style);
    assert_eq!(direct.stdout, cargo_direct.stdout);
    assert_eq!(direct.stdout, cargo_style.stdout);

    let stdin = evaluate_stdin(&mut ferris(), Path::new(POLICY), &diagnosis);
    assert_success(&stdin);
    assert_eq!(direct.stdout, stdin.stdout);

    let envelope: Value = serde_json::from_slice(&direct.stdout).expect("decision JSON");
    let record = &envelope["record"];
    assert_eq!(envelope["semantic_command_id"], "failure-policy");
    assert_eq!(record["schema"], "ferris.failure-policy-decision/v1");
    assert_eq!(record["classification"], "dependency");
    assert_eq!(record["matched"], true);
    assert_eq!(record["matched_rule_id"], "dependency-owner");
    assert_eq!(record["disposition"], "route");
    assert_eq!(record["owner_action_id"], "owner.dependencies.review");
    assert_eq!(record["executable"], false);
    assert_eq!(record["action_plan_created"], false);
    assert_eq!(record["approval_granted"], false);
    let serialized = String::from_utf8(direct.stdout).expect("UTF-8 decision");
    assert!(!serialized.contains("private-missing-dependency"));
    assert!(!serialized.contains("private-checkout"));
    assert!(!serialized.contains(diagnosis_path.to_string_lossy().as_ref()));
}

#[test]
fn policy_selects_prepare_action_and_fail_closed_fallback_without_execution() {
    let directory = TestDirectory::new();
    for (stderr, expected) in [
        (
            OFFLINE,
            json!({
                "matched": true,
                "rule": "offline-cache",
                "disposition": "prepare_action",
                "action": "owner.cargo-cache.populate"
            }),
        ),
        (
            LOCKFILE,
            json!({
                "matched": false,
                "rule": null,
                "disposition": "halt",
                "action": "owner.failure.manual-review"
            }),
        ),
    ] {
        let diagnosis_path = directory.path.join(format!(
            "{}.json",
            expected["disposition"].as_str().expect("disposition")
        ));
        fs::write(&diagnosis_path, diagnose(stderr)).expect("write diagnosis");
        let output = evaluate(&mut ferris(), Path::new(POLICY), &diagnosis_path);
        assert_success(&output);
        let envelope: Value = serde_json::from_slice(&output.stdout).expect("decision JSON");
        let record = &envelope["record"];
        assert_eq!(record["matched"], expected["matched"]);
        assert_eq!(record["matched_rule_id"], expected["rule"]);
        assert_eq!(record["disposition"], expected["disposition"]);
        assert_eq!(record["owner_action_id"], expected["action"]);
        assert_eq!(record["executable"], false);
    }
}

#[test]
fn malformed_policy_and_tampered_diagnosis_fail_closed_without_paths() {
    let directory = TestDirectory::new();
    let diagnosis_path = directory.path.join("diagnosis.json");
    let duplicate_policy = directory.path.join("duplicate-policy.json");
    let mut policy: Value =
        serde_json::from_slice(&fs::read(POLICY).expect("read policy")).expect("policy JSON");
    let duplicate = policy["rules"][0].clone();
    policy["rules"]
        .as_array_mut()
        .expect("rules")
        .push(duplicate);
    fs::write(
        &duplicate_policy,
        serde_json::to_vec(&policy).expect("serialize policy"),
    )
    .expect("write policy");
    fs::write(&diagnosis_path, diagnose(DEPENDENCY)).expect("write diagnosis");
    let duplicate_output = evaluate(&mut ferris(), &duplicate_policy, &diagnosis_path);
    assert_invalid_private(
        &duplicate_output,
        "FERRIS-FAILURE-POLICY-INVALID",
        &[&duplicate_policy],
    );

    let mut diagnosis: Value =
        serde_json::from_slice(&fs::read(&diagnosis_path).expect("read diagnosis"))
            .expect("diagnosis JSON");
    diagnosis["record"]["classification"] = json!("lockfile");
    fs::write(
        &diagnosis_path,
        serde_json::to_vec(&diagnosis).expect("serialize diagnosis"),
    )
    .expect("write tampered diagnosis");
    let tampered_output = evaluate(&mut ferris(), Path::new(POLICY), &diagnosis_path);
    assert_invalid_private(
        &tampered_output,
        "FERRIS-FAILURE-DIAGNOSIS-INVALID",
        &[&diagnosis_path],
    );

    let mut diagnosis: Value =
        serde_json::from_slice(&diagnose(DEPENDENCY)).expect("diagnosis JSON");
    diagnosis["selection_identity"] =
        json!("selection:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    diagnosis["result_identity"] = json!(result_identity(&diagnosis));
    fs::write(
        &diagnosis_path,
        serde_json::to_vec(&diagnosis).expect("serialize diagnosis"),
    )
    .expect("write diagnosis with self-consistent forged selection");
    let forged_output = evaluate(&mut ferris(), Path::new(POLICY), &diagnosis_path);
    assert_invalid_private(
        &forged_output,
        "FERRIS-FAILURE-DIAGNOSIS-INVALID",
        &[&diagnosis_path],
    );
}

#[test]
fn unavailable_empty_and_oversized_inputs_fail_closed() {
    let directory = TestDirectory::new();
    let diagnosis_path = directory.path.join("diagnosis.json");
    let missing_policy = directory.path.join("missing-policy.json");
    let empty_policy = directory.path.join("empty-policy.json");
    let oversized_policy = directory.path.join("oversized-policy.json");
    fs::write(&diagnosis_path, diagnose(DEPENDENCY)).expect("write diagnosis");
    fs::write(&empty_policy, []).expect("write empty policy");
    fs::write(&oversized_policy, vec![b'x'; 64 * 1024 + 1]).expect("write oversized policy");

    for (policy, code) in [
        (&missing_policy, "FERRIS-FAILURE-POLICY-UNAVAILABLE"),
        (&empty_policy, "FERRIS-FAILURE-POLICY-INVALID"),
        (&oversized_policy, "FERRIS-FAILURE-POLICY-INVALID"),
    ] {
        let output = evaluate(&mut ferris(), policy, &diagnosis_path);
        assert_invalid_private(&output, code, &[policy]);
    }

    for diagnosis in [Vec::new(), vec![b'x'; 128 * 1024 + 1]] {
        let output = evaluate_stdin(&mut ferris(), Path::new(POLICY), &diagnosis);
        assert_invalid_private(&output, "FERRIS-FAILURE-DIAGNOSIS-INVALID", &[]);
    }
}

#[test]
fn checked_in_schemas_are_closed_and_cataloged() {
    for (path, expected) in [
        (POLICY_SCHEMA, "ferris.failure-policy/v1"),
        (DECISION_SCHEMA, "ferris.failure-policy-decision/v1"),
    ] {
        let schema: Value =
            serde_json::from_slice(&fs::read(path).expect("read schema")).expect("parse schema");
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(schema["properties"]["schema"]["const"], expected);
    }

    let contracts = ferris()
        .args(["contracts", "--format", "json"])
        .output()
        .expect("run contracts");
    assert_success(&contracts);
    let envelope: Value = serde_json::from_slice(&contracts.stdout).expect("catalog JSON");
    let entries = envelope["record"]["contracts"]
        .as_array()
        .expect("contracts array");
    for (schema, accepted, emitted) in [
        ("ferris.failure-policy/v1", true, false),
        ("ferris.failure-policy-decision/v1", true, true),
    ] {
        let entry = entries
            .iter()
            .find(|entry| entry["schema"] == schema)
            .expect("catalog entry");
        assert_eq!(entry["accepted"], accepted);
        assert_eq!(entry["emitted"], emitted);
    }
}

fn assert_invalid_private(output: &Output, code: &str, paths: &[&PathBuf]) {
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr.clone()).expect("UTF-8 error");
    for path in paths {
        assert!(!stderr.contains(path.to_string_lossy().as_ref()));
    }
    let envelope: Value = serde_json::from_str(&stderr).expect("error JSON");
    assert_eq!(envelope["diagnostics"][0]["code"], code);
}

fn result_identity(envelope: &Value) -> String {
    #[derive(Serialize)]
    struct Identity<'a> {
        schema: &'a Value,
        command_version: &'a Value,
        semantic_command_id: &'a Value,
        selection_identity: &'a Value,
        invocation_identity: &'a Value,
        result_class: &'a Value,
        process_exit_code: &'a Value,
        diagnostics: &'a Value,
        record: &'a Value,
    }

    let identity = Identity {
        schema: &envelope["schema"],
        command_version: &envelope["command_version"],
        semantic_command_id: &envelope["semantic_command_id"],
        selection_identity: &envelope["selection_identity"],
        invocation_identity: &envelope["invocation_identity"],
        result_class: &envelope["result_class"],
        process_exit_code: &envelope["process_exit_code"],
        diagnostics: &envelope["diagnostics"],
        record: &envelope["record"],
    };
    let bytes = serde_json::to_vec(&identity).expect("serialize result identity");
    format!("result:{:x}", Sha256::digest(bytes))
}

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-failure-policy-test-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create test directory");
        Self { path }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
