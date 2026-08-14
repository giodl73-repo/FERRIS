use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-43-ordered-result-publisher-release";
const MANIFEST_RAW: &str =
    "sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4";
const MANIFEST_AGGREGATE: &str =
    "sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346";
const RECEIPT_RAW: &str = "sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c";
const RECEIPT_PAYLOAD: &str =
    "sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2";
const SEAL_RAW: &str = "sha256:4445a0b181419b303c28f1d91e1700a594d2a040fbae1cd0dc526fa7fc8e8f05";
const SEAL_PAYLOAD: &str =
    "sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn python() -> PathBuf {
    env::var_os("PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("python"))
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read release file");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse release JSON");
    (bytes, value)
}

fn decode_sha256(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64, "SHA-256 hexadecimal length");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).expect("hex digest");
    }
    bytes
}

fn manifest_aggregate(files: &[Value]) -> String {
    let mut ordered = files.to_vec();
    ordered.sort_by_key(|file| file["path"].as_str().expect("manifest path").to_owned());
    let mut hasher = Sha256::new();
    for file in ordered {
        let path = file["path"].as_str().expect("manifest path");
        hasher.update((path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(decode_sha256(
            file["sha256"]
                .as_str()
                .expect("manifest digest")
                .strip_prefix("sha256:")
                .expect("SHA-256 prefix"),
        ));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        assert!(
            !metadata.file_type().is_symlink(),
            "{path:?} must not be a symlink"
        );
        if metadata.is_dir() {
            collect_files(root, &path, files);
        } else {
            assert!(metadata.is_file(), "{path:?} must be a regular file");
            files.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn run_publisher(
    release: &Path,
    catalog: &Path,
    events: &Path,
    final_root: &Path,
) -> (bool, Value, String) {
    let output = Command::new(python())
        .arg(release.join("ordered_result_publisher.py"))
        .arg("--catalog")
        .arg(catalog)
        .arg("--events")
        .arg(events)
        .arg("--final-root")
        .arg(final_root)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 43 publisher");
    let stdout = String::from_utf8(output.stdout).expect("publisher stdout UTF-8");
    (
        output.status.success(),
        serde_json::from_str(&stdout).expect("publisher JSON summary"),
        String::from_utf8(output.stderr).expect("publisher stderr UTF-8"),
    )
}

#[test]
fn pulse_43_ordered_result_publisher_release_is_sealed_and_mutation_controlled() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/complete-events.json".to_owned(),
        "fixtures/public-gate-catalog.json".to_owned(),
        "ordered_result_publisher.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "release-seal.json".to_owned(),
        "schemas/ferris.pulse-43-ordered-result.v1.schema.json".to_owned(),
        "tests/test_ordered_result_publisher.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-43-ordered-result-public-manifest/v1"
    );
    assert_eq!(manifest["file_count"], 6);
    assert_eq!(manifest["release_tree_file_count"], 9);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 6);
    assert_eq!(manifest_aggregate(files), MANIFEST_AGGREGATE);
    let mut manifest_total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        assert!(!Path::new(path).is_absolute());
        assert!(!path.contains(".."));
        let bytes = read_lf(release.join(path));
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("manifest size")
        );
        assert_eq!(sha256(&bytes), file["sha256"], "{path} binding");
        manifest_total += bytes.len() as u64;
    }
    assert_eq!(manifest_total, 47_973);
    assert_eq!(manifest["total_bytes"], 47_973);

    let (_, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    assert_eq!(
        sha256(&read_lf(release.join("qualification-receipt.json"))),
        RECEIPT_RAW
    );
    assert_eq!(
        receipt["payload"]["qualification"]["python_test_methods"],
        18
    );
    assert_eq!(
        receipt["payload"]["qualification"]["success_cycles"],
        "20/20"
    );
    assert_eq!(receipt["payload"]["qualification"]["retries"], 0);
    assert_eq!(receipt["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(
        receipt["payload"]["manifest"]["aggregate"],
        MANIFEST_AGGREGATE
    );
    assert_eq!(receipt["payload"]["manifest"]["total_bytes"], 47_973);

    let (_, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(
        sha256(&read_lf(release.join("release-seal.json"))),
        SEAL_RAW
    );
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(seal["payload"]["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(seal["payload"]["manifest"]["total_bytes"], 47_973);
    assert_eq!(
        seal["payload"]["qualification_receipt"]["payload_sha256"],
        RECEIPT_PAYLOAD
    );

    let test_output = Command::new(python())
        .arg(release.join("tests/test_ordered_result_publisher.py"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 43 Python qualification");
    assert!(
        test_output.status.success(),
        "Python qualification failed: stdout={} stderr={}",
        String::from_utf8_lossy(&test_output.stdout),
        String::from_utf8_lossy(&test_output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&test_output.stderr).contains("Ran 18 tests"),
        "qualification count: {}",
        String::from_utf8_lossy(&test_output.stderr)
    );

    let work = root
        .join("target")
        .join(format!("pulse-43-rust-{}", std::process::id()));
    if work.exists() {
        fs::remove_dir_all(&work).expect("clean Rust integration directory");
    }
    fs::create_dir_all(&work).expect("create Rust integration directory");
    let catalog = release.join("fixtures/public-gate-catalog.json");
    let complete_events = release.join("fixtures/complete-events.json");

    let complete_final = work.join("complete");
    let (complete_ok, complete, complete_stderr) =
        run_publisher(&release, &catalog, &complete_events, &complete_final);
    assert!(complete_ok, "{complete_stderr}");
    assert_eq!(complete["publication"]["state"], "published");
    assert_eq!(complete["publication"]["files"], "2/2");
    assert_eq!(complete["publication"]["rename_attempts"], 1);
    assert_eq!(complete["publication"]["retries"], 0);
    assert_eq!(complete["ordered_execution"]["completed_gate_count"], 3);
    let (result_bytes, result) = read_json(complete_final.join("public-result.json"));
    let (_, output_receipt) = read_json(complete_final.join("release-receipt.json"));
    assert_eq!(
        canonical_payload_sha256(&result["payload"]),
        result["payload_sha256"]
    );
    assert_eq!(
        output_receipt["payload"]["result_raw_sha256"],
        sha256(&result_bytes)
    );
    assert_eq!(
        output_receipt["payload"]["result_payload_sha256"],
        result["payload_sha256"]
    );

    let stopped_events = work.join("stopped-events.json");
    fs::write(
        &stopped_events,
        serde_json::to_vec(&json!([
            {
                "classification": "public-artifact-self-validation",
                "completed_checks": 39,
                "event_kind": "validation-complete",
                "expected_checks": 39,
                "schema": "ferris.pulse-43-ordered-result-event/v1",
                "validation_id": "public-input-contract"
            },
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-33-freeze",
                "outcome": "stopped",
                "schema": "ferris.pulse-43-ordered-result-event/v1"
            }
        ]))
        .expect("serialize stopped mutation"),
    )
    .expect("write stopped mutation");
    let (stopped_ok, stopped, stopped_stderr) =
        run_publisher(&release, &catalog, &stopped_events, &work.join("stopped"));
    assert!(stopped_ok, "{stopped_stderr}");
    assert_eq!(stopped["publication"]["state"], "published");
    assert_eq!(stopped["public_self_validation"]["completed_checks"], 39);
    assert_eq!(stopped["ordered_execution"]["attempted_gate_count"], 1);
    assert_eq!(stopped["ordered_execution"]["completed_gate_count"], 0);

    let late_events = work.join("late-events.json");
    fs::write(
        &late_events,
        serde_json::to_vec(&json!([
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-33-freeze",
                "outcome": "stopped",
                "schema": "ferris.pulse-43-ordered-result-event/v1"
            },
            {
                "classification": "ordered-execution",
                "event_kind": "gate-complete",
                "gate_id": "pulse-31-public-input",
                "outcome": "passed",
                "schema": "ferris.pulse-43-ordered-result-event/v1"
            },
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-35-normalization",
                "outcome": "completed",
                "schema": "ferris.pulse-43-ordered-result-event/v1"
            }
        ]))
        .expect("serialize late mutation"),
    )
    .expect("write late mutation");
    let late_final = work.join("late");
    let (late_ok, late, _) = run_publisher(&release, &catalog, &late_events, &late_final);
    assert!(!late_ok);
    assert_eq!(late["failure_code"], "P43-ORDERED-AFTER-TERMINAL");
    assert_eq!(late["publication"]["state"], "absent");
    assert!(!late_final.exists());

    fs::remove_dir_all(&work).expect("remove Rust integration directory");
}
