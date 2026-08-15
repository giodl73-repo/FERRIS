use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-51-diagnostic-executor-release";
const MANIFEST_RAW: &str =
    "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc";
const MANIFEST_AGGREGATE: &str =
    "sha256:18d61962245d75e42fed30f581555a5b436e0a83d89e3383d059dca035e978e6";
const RECEIPT_RAW: &str = "sha256:ef2b423520e1f2680c0cadd246a51c0af1a4502f45d757f018982f42c326f1c9";
const RECEIPT_PAYLOAD: &str =
    "sha256:77408aabd377801c3c578a889523c18ee95eb286ac55b04df6c30f74d45ef452";
const SEAL_RAW: &str = "sha256:968f495555b4617329318686b5adb460faf3fe95a07c8da160e163c9395eb767";
const SEAL_PAYLOAD: &str =
    "sha256:1d22ad1248a2f47c78984d8020c3c6507253c468b53f30073efcfb5ab880c0d4";

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
            collect_files(root, &path, output);
        } else {
            assert!(metadata.is_file(), "{path:?} must be regular");
            output.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn assert_no_release_scratch(directory: &Path) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        let name = entry.file_name();
        let name = name.to_string_lossy();
        assert!(
            name != "__pycache__" && name != ".run" && name != ".qualification-work",
            "{path:?} must not retain Python or qualification scratch"
        );
        if metadata.is_dir() {
            assert_no_release_scratch(&path);
        } else {
            assert!(
                !name.ends_with(".pyc") && !name.ends_with(".pyo"),
                "{path:?} must not retain Python bytecode"
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
        .expect("run Pulse 51 Python validation")
}

#[test]
fn pulse_51_diagnostic_executor_release_is_sealed_and_synthetic_only() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "diagnostic_executor.py".to_owned(),
        "fixtures/fake_ferris.py".to_owned(),
        "fixtures/p51-public-gate-catalog.json".to_owned(),
        "frozen_profile_diff.py".to_owned(),
        "p31_contract_verifier.py".to_owned(),
        "p35-p37-custody-binding.json".to_owned(),
        "p35_p37_custody.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-51-diagnostic-executor.v1.schema.json".to_owned(),
        "sealed_dependencies.py".to_owned(),
        "synthetic_fixture.py".to_owned(),
        "tests/test_diagnostic_executor.py".to_owned(),
    ]);
    assert_no_release_scratch(&release);
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
        "ferris.pulse-51-diagnostic-executor-public-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(
        files.len(),
        manifest["file_count"].as_u64().expect("file count") as usize
    );
    assert_eq!(aggregate(files), MANIFEST_AGGREGATE);
    let mut total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        assert!(!Path::new(path).is_absolute());
        assert!(!path.contains(".."));
        let bytes = read_lf(release.join(path));
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("file size")
        );
        assert_eq!(sha256(&bytes), file["sha256"], "{path} hash");
        total += bytes.len() as u64;
    }
    assert_eq!(
        total,
        manifest["total_bytes"].as_u64().expect("total bytes")
    );
    assert_eq!(
        manifest["release_tree_file_count"],
        actual_paths.len() as u64,
        "manifest binds the complete release tree"
    );

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-51-diagnostic-executor-qualification-envelope/v2"
    );
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    assert_eq!(receipt["payload"]["cycles_required"], 20);
    assert_eq!(receipt["payload"]["cycles_passed"], 20);
    assert_eq!(
        receipt["payload"]["schema"],
        "ferris.pulse-51-diagnostic-executor-qualification/v2"
    );
    assert_eq!(
        receipt["payload"]["fake_launches_per_platform_per_cycle"],
        69
    );
    assert_eq!(
        receipt["payload"]["p43_terminal_publication_invocations"],
        0
    );
    assert_eq!(
        receipt["payload"]["p47_terminal_publication_invocations"],
        0
    );
    assert_eq!(
        receipt["payload"]["p44_p45_bridges_per_platform_per_cycle"],
        1
    );
    assert_eq!(
        receipt["payload"]["windows_native_dispatches_per_cycle"],
        69
    );
    assert_eq!(receipt["payload"]["ubuntu_wsl_dispatches_per_cycle"], 69);
    assert_eq!(receipt["payload"]["wsl_distribution"], "Ubuntu-24.04");
    assert_eq!(
        receipt["payload"]["p27_successful_cycle_retention"],
        "private-until-cycle-cleanup"
    );
    let cleanup = &receipt["payload"]["synthetic_scratch_cleanup"];
    assert_eq!(
        cleanup["retry_delays_seconds"],
        serde_json::json!([0.02, 0.05, 0.10, 0.20])
    );
    assert_eq!(
        cleanup["retryable_errors"],
        serde_json::json!(["PermissionError", "WinError32"])
    );
    assert_eq!(cleanup["root_absence_verified"], true);
    assert_eq!(receipt["payload"]["private_seed_created"], false);
    assert_eq!(receipt["payload"]["ferris_executed"], false);

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(seal["payload"]["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(
        seal["payload"]["qualification_receipt"]["raw_sha256"],
        RECEIPT_RAW
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["payload_sha256"],
        RECEIPT_PAYLOAD
    );
    assert_eq!(
        seal["payload"]["verification"]["synthetic_scratch_cleanup"],
        receipt["payload"]["synthetic_scratch_cleanup"]
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
    let qualification = python_output(&release, &["-B", "qualify.py", "--cycles", "20"]);
    assert!(
        qualification.status.success(),
        "Python qualification failed: stdout={} stderr={}",
        String::from_utf8_lossy(&qualification.stdout),
        String::from_utf8_lossy(&qualification.stderr)
    );
    let payload: Value =
        serde_json::from_slice(&qualification.stdout).expect("qualification payload");
    assert_eq!(payload["cycles_run"], 20);
    assert_eq!(
        payload["schema"],
        "ferris.pulse-51-diagnostic-executor-qualification/v2"
    );
    assert_eq!(payload["fake_launches_total"], 2_760);
    assert_eq!(payload["windows_native_dispatches_per_cycle"], 69);
    assert_eq!(payload["ubuntu_wsl_dispatches_per_cycle"], 69);
    assert_eq!(payload["wsl_distribution"], "Ubuntu-24.04");
    assert_eq!(payload["p47_terminal_publication_invocations"], 0);
    assert_no_release_scratch(&release);
    let mut final_paths = BTreeSet::new();
    collect_files(&release, &release, &mut final_paths);
    assert_eq!(final_paths, expected_paths);
}
