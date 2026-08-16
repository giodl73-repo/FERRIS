use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-69-capability-bound-diagnostic-executor-successor-release";
const MANIFEST_RAW: &str =
    "sha256:dd52c64100cdcd91ae1ea92f91af112f56296faf803511832ce3db489e071463";
const MANIFEST_AGGREGATE: &str =
    "sha256:070d56d191be0b743b34d4444350f6b0e78272088c9141a39ec2b60524216331";
const RECEIPT_RAW: &str = "sha256:5fe60dcabc27a3260439c302407079feb3f004e2463877c8ff133a055884cbd4";
const RECEIPT_PAYLOAD: &str =
    "sha256:244f02e9fb5a164a747d3407696f4ef4a7fa318fa7ee5fe33d8ce1baa533def9";
const SEAL_RAW: &str = "sha256:c25b7b9868825734c7d649c1a2f4d303cf0de8da2ae7acebde386031fd689363";
const SEAL_PAYLOAD: &str =
    "sha256:2ac37cac02569d89cc068778cd956c8b5273c086685a1296dc568d13d191885f";

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
    let value = serde_json::from_slice(&bytes).expect("parse release JSON");
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
    sha256(&serde_json::to_vec(value).expect("canonical payload"))
}

fn python_output(release: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(python())
        .current_dir(release)
        .args(arguments)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 69 Python validation")
}

#[test]
fn pulse_69_capability_bound_diagnostic_executor_successor_is_sealed_and_fake_only() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "capability_bound_diagnostic_executor_successor.py".to_owned(),
        "fixtures/fake_p56.py".to_owned(),
        "fixtures/p69-public-gate-catalog.json".to_owned(),
        "fixtures/p69_fake_native_wsl.py".to_owned(),
        "generate_release.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-69-capability-bound-diagnostic-executor-successor.v1.schema.json"
            .to_owned(),
        "sealed_dependencies.py".to_owned(),
        "tests/test_capability_bound_diagnostic_executor_successor.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let source = String::from_utf8(read_lf(
        release.join("capability_bound_diagnostic_executor_successor.py"),
    ))
    .expect("UTF-8 executor");
    assert!(source.contains("load_exact_p57_stack"));
    assert!(source.contains("class _OwnedBundle"));
    assert!(source.contains("_WSL_BUNDLE_IDENTITY_BOOTSTRAP"));
    assert!(source.contains("_WSL_BUNDLE_CLEANUP_BOOTSTRAP"));
    assert!(source.contains("def _stage_owned_bundle("));
    assert!(source.contains("def _cleanup_owned_bundle("));
    assert!(source.contains("def run_capability_bound_diagnostic_executor("));
    assert!(source.contains("_P57._execute("));
    assert!(source.contains("P57-INDETERMINATE-CLEANUP"));

    let sealed_source = String::from_utf8(read_lf(release.join("sealed_dependencies.py")))
        .expect("UTF-8 sealed dependencies");
    assert!(sealed_source.contains("P57 = ReleaseIdentity("));
    assert!(sealed_source.contains("load_exact_p57_stack"));
    assert!(sealed_source.contains("release_identities"));

    let (_, catalog) = read_json(release.join("fixtures/p69-public-gate-catalog.json"));
    assert_eq!(
        catalog["gate_ids"],
        serde_json::json!([
            "sealed-predecessor-binding",
            "windows-capability-build-custody",
            "ubuntu-capability-build-custody",
            "exact-adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "descriptor-validation",
            "bounded-process-exit-search"
        ])
    );

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-69-capability-bound-diagnostic-executor-successor-manifest/v1"
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
    assert_eq!(
        manifest["predecessors"]["pulse_57"]["source"],
        "sha256:bcb5eac2cd5aa0abd271dec2e93963ec855faa1c5ecbd628dfef61f52358c2c0"
    );

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
    assert_eq!(payload["negative_control_tests_run"], 8);
    assert_eq!(payload["negative_control_tests_passed"], 8);
    assert_eq!(payload["owned_bundle_cleanup_total"], 20);
    assert_eq!(
        payload["bundle_retained_during_worker_lifetime_verified"],
        true
    );
    assert_eq!(payload["zero_residue_after_close_verified"], true);
    let control_ids = payload["negative_control_test_ids"]
        .as_array()
        .expect("receipt control IDs");
    assert_eq!(control_ids.len(), 8);
    let actual_control_ids = control_ids
        .iter()
        .map(|value| value.as_str().expect("receipt control ID").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_control_ids,
        BTreeSet::from([
            "bundle-retained-and-zero-residue".to_owned(),
            "cleanup-precedence-over-protocol-failure".to_owned(),
            "concurrent-owned-bundles-isolated".to_owned(),
            "exact-p57-binding-and-signature".to_owned(),
            "root-substitution-rejected".to_owned(),
            "source-no-follow-symlink-rejection".to_owned(),
            "startup-failure-cleanup".to_owned(),
            "terminate-kill-then-bundle-cleanup".to_owned(),
        ])
    );

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(
        seal["payload"]["limits"]["cleanup_uncertainty_disposition"],
        "P57-INDETERMINATE-CLEANUP"
    );
    assert_eq!(
        seal["payload"]["limits"]["owned_bundle_sibling_deletion"],
        false
    );
    assert_eq!(
        seal["payload"]["scope"]["native_bundle_cleanup_owned_by_session"],
        true
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["owned_bundle_cleanup_total"],
        20
    );

    let unit = python_output(
        &release,
        &["-B", "-m", "unittest", "discover", "-s", "tests", "-v"],
    );
    assert!(
        unit.status.success(),
        "Python tests failed: stdout={} stderr={}",
        String::from_utf8_lossy(&unit.stdout),
        String::from_utf8_lossy(&unit.stderr)
    );
}
