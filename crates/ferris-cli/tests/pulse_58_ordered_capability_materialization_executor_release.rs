use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release";
const MANIFEST_RAW: &str =
    "sha256:8e321041d69a5953aa73a2c67c344a55c25c2fec25008828151d5cb5e16f968f";
const MANIFEST_AGGREGATE: &str =
    "sha256:7f09008023e9bcf4d111e7b8b82320fe3b99467480a81d8ae452c9f04025a47c";
const RECEIPT_RAW: &str = "sha256:9ad46590136de604d6dc1bc9929ac60c3daf56af147905d33d811d4b42037289";
const RECEIPT_PAYLOAD: &str =
    "sha256:49fb8397ffbd344552b6a4ff1880e3816af35ace55cff3ba6793c3dfef91e7e6";
const SEAL_RAW: &str = "sha256:5dadcb7a6c7b926d2d1166865df8cec075a1cfa4da957637350e5851dade4e93";
const SEAL_PAYLOAD: &str =
    "sha256:9bd64239dca64d8facbe493d2c243d91b4e6d53014efa17fcb9eb4ae2eaffdd5";

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

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read release file");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
    (bytes, value)
}

fn decode_sha256(value: &str) -> [u8; 32] {
    let hex = value.strip_prefix("sha256:").expect("SHA-256 prefix");
    assert_eq!(hex.len(), 64, "SHA-256 hex length");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex");
    }
    bytes
}

fn aggregate(files: &[Value]) -> String {
    let mut ordered = files.to_vec();
    ordered.sort_by_key(|value| value["path"].as_str().expect("manifest path").to_owned());
    let mut hasher = Sha256::new();
    for file in ordered {
        let path = file["path"].as_str().expect("manifest path");
        hasher.update((path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(decode_sha256(
            file["sha256"].as_str().expect("manifest digest"),
        ));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn collect_files(root: &Path, directory: &Path, output: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        assert!(
            !metadata.file_type().is_symlink(),
            "{path:?} must not be symlink"
        );
        if metadata.is_dir() {
            assert_ne!(entry.file_name(), "__pycache__", "no Python residue");
            collect_files(root, &path, output);
        } else {
            assert!(metadata.is_file(), "{path:?} must be regular");
            output.insert(
                path.strip_prefix(root)
                    .expect("relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("canonical JSON payload"))
}

#[test]
fn pulse_58_ordered_capability_materialization_executor_is_sealed_and_fake_only() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/__init__.py".to_owned(),
        "fixtures/fake_p56.py".to_owned(),
        "fixtures/p52_synthetic_fixture.py".to_owned(),
        "fixtures/p58-public-gate-catalog.json".to_owned(),
        "generate_release.py".to_owned(),
        "ordered_capability_materialization_executor.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-58-ordered-capability-materialization-executor.v1.schema.json"
            .to_owned(),
        "sealed_dependencies.py".to_owned(),
        "tests/test_ordered_capability_materialization_executor.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let source = String::from_utf8(read_lf(
        release.join("ordered_capability_materialization_executor.py"),
    ))
    .expect("UTF-8 executor");
    assert!(source.contains("def run_ordered_capability_materialization_executor("));
    assert!(
        source.contains("secrets.token_bytes(32)") && source.contains("P58-INDETERMINATE-CLEANUP")
    );
    assert!(
        source.contains("_verify_public_prelaunch_custody")
            && source.contains("_execute_p57_semantics")
    );
    assert!(!source.contains("run_capability_bound_diagnostic_executor("));
    for forbidden in [
        "descriptor_root",
        "process_runner",
        "retained_custodies",
        "callback",
        "environment",
    ] {
        assert!(!source.contains(&format!(
            "def run_ordered_capability_materialization_executor(\n    {forbidden}"
        )));
    }

    let (_, catalog) = read_json(release.join("fixtures/p58-public-gate-catalog.json"));
    assert_eq!(
        catalog["gate_ids"],
        serde_json::json!([
            "pulse-41-pulse-39-public-custody",
            "sealed-predecessor-binding",
            "windows-capability-build-custody",
            "ubuntu-capability-build-custody",
            "exact-adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "bounded-materialization",
            "descriptor-validation",
            "bounded-process-exit-search"
        ])
    );

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-58-ordered-capability-materialization-executor-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len() as u64, manifest["file_count"].as_u64().unwrap());
    assert_eq!(aggregate(files), MANIFEST_AGGREGATE);
    assert_eq!(
        manifest["release_tree_file_count"],
        actual_paths.len() as u64
    );
    let mut total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        let bytes = read_lf(release.join(path));
        assert_eq!(bytes.len() as u64, file["size"].as_u64().unwrap());
        assert_eq!(sha256(&bytes), file["sha256"], "{path} digest");
        total += bytes.len() as u64;
    }
    assert_eq!(total, manifest["total_bytes"].as_u64().unwrap());

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    let payload = &receipt["payload"];
    assert_eq!(payload["cycles_required"], 20);
    assert_eq!(payload["cycles_passed"], 20);
    assert_eq!(payload["fake_launches_total"], 2_760);
    assert_eq!(payload["ferris_executed"], false);
    assert_eq!(payload["negative_control_tests_run"], 20);
    assert_eq!(payload["negative_control_tests_passed"], 20);
    assert_eq!(
        payload["p39_caller_authority_precondition"],
        "future-authority-supplied-fresh-anonymous-exact-cutoff-root"
    );
    assert_eq!(payload["p39_execution_scope"], "exact-p39-semantics-only");
    let control_ids = payload["negative_control_test_ids"]
        .as_array()
        .expect("receipt control IDs");
    assert_eq!(control_ids.len(), 20);
    let actual_control_ids = control_ids
        .iter()
        .map(|value| value.as_str().expect("receipt control ID").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_control_ids,
        BTreeSet::from([
            "capability-built-before-seed".to_owned(),
            "directory-substitution-rejected".to_owned(),
            "directory-symlink-and-wsl-no-follow".to_owned(),
            "final-cleanup".to_owned(),
            "first-semantic-mismatch-stop".to_owned(),
            "no-launch-topology".to_owned(),
            "ordinal-69-semantic-mismatch-cleanup".to_owned(),
            "ordinal-69-ubuntu-failure-cleanup".to_owned(),
            "p27-bounded-failure-cleanup".to_owned(),
            "p39-failure-terminal-cleanup".to_owned(),
            "p41-failure-terminal-cleanup".to_owned(),
            "production-surface-rejects-injection".to_owned(),
            "release-generator-rejects-cache-residue".to_owned(),
            "seed-zero-public-failure".to_owned(),
            "single-materialization-per-cycle".to_owned(),
            "single-seed-per-cycle".to_owned(),
            "synthetic-p39-root-only".to_owned(),
            "unknown-fault-cleanup-indeterminate".to_owned(),
            "unknown-fault-cleanup-reraise".to_owned(),
            "worker-protocol-replay-rejected".to_owned(),
        ])
    );
    assert_eq!(payload["p44_p45_execution_invocations"], 0);
    assert_eq!(payload["publication_invocations"], 0);
    assert_eq!(payload["cycles"].as_array().unwrap().len(), 20);

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["scope"]["diagnostic_authority"], false);
    assert_eq!(seal["payload"]["scope"]["public_publication"], false);
    assert_eq!(
        seal["payload"]["predecessors"]["pulse_56"]["manifest"],
        "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a"
    );
    assert_eq!(
        seal["payload"]["predecessors"]["pulse_57"]["manifest"],
        "sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291"
    );

    let unit = Command::new(python())
        .current_dir(&release)
        .args(["-B", "-m", "unittest", "discover", "-s", "tests", "-v"])
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 58 Python tests");
    assert!(
        unit.status.success(),
        "Python tests failed: stdout={} stderr={}",
        String::from_utf8_lossy(&unit.stdout),
        String::from_utf8_lossy(&unit.stderr)
    );
}
