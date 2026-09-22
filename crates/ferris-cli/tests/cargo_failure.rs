use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

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
const TEST_ASSERTION: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/cargo-failure/test-assertion.stderr"
);
const REPORT_SCHEMA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/cargo-failure/ferris.cargo-failure-report.v1.schema.json"
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

fn diagnose(path: &str) -> Output {
    ferris()
        .args(["diagnose-cargo", "--stderr", path, "--format", "json"])
        .output()
        .expect("run Cargo failure diagnosis")
}

#[test]
fn dependency_report_is_deterministic_private_and_matches_all_entrypoints() {
    let arguments = ["diagnose-cargo", "--stderr", DEPENDENCY, "--format", "json"];
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
    let directory = TestDirectory::new();
    let relocated = directory.path.join("relocated.stderr");
    fs::copy(DEPENDENCY, &relocated).expect("copy stderr fixture");
    let relocated_output = diagnose(relocated.to_str().expect("UTF-8 temporary path"));
    assert_success(&relocated_output);
    assert_eq!(direct.stdout, relocated_output.stdout);

    let envelope: Value = serde_json::from_slice(&direct.stdout).expect("report JSON");
    assert_eq!(envelope["semantic_command_id"], "diagnose-cargo");
    assert_eq!(
        envelope["record"]["schema"],
        "ferris.cargo-failure-report/v1"
    );
    assert_eq!(envelope["record"]["classification"], "dependency");
    assert_eq!(envelope["record"]["classified"], true);
    assert_eq!(
        envelope["record"]["diagnostic_code"],
        "FERRIS-CARGO-DEPENDENCY-BLOCKED"
    );
    assert_eq!(envelope["record"]["raw_output_retained"], false);
    assert_eq!(envelope["record"]["executable"], false);
    let serialized = String::from_utf8(direct.stdout).expect("UTF-8 JSON");
    assert!(!serialized.contains("private-missing-dependency"));
    assert!(!serialized.contains("private-checkout"));
    assert!(!serialized.contains(DEPENDENCY));

    let human = ferris()
        .args(["diagnose-cargo", "--stderr", DEPENDENCY])
        .output()
        .expect("run human diagnosis");
    assert_success(&human);
    let human = String::from_utf8(human.stdout).expect("UTF-8 human output");
    assert!(human.contains("Classification: dependency"));
    assert!(human.contains("Raw output retained: false"));
    assert!(!human.contains("private-missing-dependency"));
    assert!(!human.contains("private-checkout"));
}

#[test]
fn supported_classes_and_test_assertion_negative_control_are_distinct() {
    for (path, classification, code, classified) in [
        (LOCKFILE, "lockfile", "FERRIS-CARGO-LOCK-BLOCKED", true),
        (
            OFFLINE,
            "offline_policy",
            "FERRIS-CARGO-OFFLINE-BLOCKED",
            true,
        ),
        (
            TEST_ASSERTION,
            "unclassified",
            "FERRIS-CARGO-FAILURE-UNCLASSIFIED",
            false,
        ),
    ] {
        let output = diagnose(path);
        assert_success(&output);
        let envelope: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
        assert_eq!(envelope["record"]["classification"], classification);
        assert_eq!(envelope["record"]["diagnostic_code"], code);
        assert_eq!(envelope["record"]["classified"], classified);
    }
}

#[test]
fn unavailable_empty_non_utf8_and_oversized_inputs_fail_closed_without_paths() {
    let directory = TestDirectory::new();
    let missing = directory.path.join("private-missing.stderr");
    let empty = directory.path.join("empty.stderr");
    let non_utf8 = directory.path.join("non-utf8.stderr");
    let oversized = directory.path.join("oversized.stderr");
    fs::write(&empty, []).expect("write empty input");
    fs::write(&non_utf8, [0xff]).expect("write non-UTF-8 input");
    fs::write(&oversized, vec![b'x'; 64 * 1024 + 1]).expect("write oversized input");

    for (path, code) in [
        (&missing, "FERRIS-CARGO-DIAGNOSTIC-UNAVAILABLE"),
        (&empty, "FERRIS-CARGO-DIAGNOSTIC-INVALID"),
        (&non_utf8, "FERRIS-CARGO-DIAGNOSTIC-INVALID"),
        (&oversized, "FERRIS-CARGO-DIAGNOSTIC-INVALID"),
    ] {
        let output = ferris()
            .args(["diagnose-cargo", "--stderr"])
            .arg(path)
            .args(["--format", "json"])
            .output()
            .expect("run invalid diagnosis");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).expect("error JSON");
        assert!(!stderr.contains(path.to_string_lossy().as_ref()));
        let envelope: Value = serde_json::from_str(&stderr).expect("error envelope");
        assert_eq!(envelope["diagnostics"][0]["code"], code);
    }
}

#[test]
fn checked_in_schema_is_closed_and_matches_report_fields() {
    let schema: Value = serde_json::from_slice(&fs::read(REPORT_SCHEMA).expect("read schema"))
        .expect("parse schema");
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        schema["properties"]["schema"]["const"],
        "ferris.cargo-failure-report/v1"
    );
    let output = diagnose(DEPENDENCY);
    assert_success(&output);
    let envelope: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let report = envelope["record"].as_object().expect("report object");
    for required in schema["required"].as_array().expect("required fields") {
        assert!(
            report.contains_key(required.as_str().expect("required field")),
            "missing required field {required}"
        );
    }
    assert!(report.keys().all(|field| {
        schema["properties"]
            .as_object()
            .expect("schema properties")
            .contains_key(field)
    }));
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
            "ferris-cargo-failure-test-{}-{nonce}",
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
