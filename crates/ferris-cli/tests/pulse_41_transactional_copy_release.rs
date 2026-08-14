use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release";
const P39_RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release";
const MANIFEST_RAW: &str =
    "sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8";
const MANIFEST_AGGREGATE: &str =
    "sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755";
const REPORT_RAW: &str = "sha256:e16b84700318b3bedb82b283d9af2df8ae963fe63465fd2056a43839dfcfd8ee";
const REPORT_PAYLOAD: &str =
    "sha256:fbcfd656024e3fd6b3e24d18cf8991532fd77284611ab81f578e1985f6a5b4cc";
const RECEIPT_RAW: &str = "sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c";
const RECEIPT_PAYLOAD: &str =
    "sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f";
const SEAL_RAW: &str = "sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a";
const SEAL_PAYLOAD: &str =
    "sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf";

const P39_BINDINGS: [(&str, u64, &str); 8] = [
    (
        "README.md",
        1786,
        "sha256:9e19afae44aa5c112ddcde67fbdaf501903b5cb39ce3757e5bc6fea8554c7989",
    ),
    (
        "checkout_verifier.py",
        9685,
        "sha256:783283fd127170460ce52106a7a1158054cdc2608475e53899ff45a7a6a31d12",
    ),
    (
        "public-manifest.json",
        1387,
        "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c",
    ),
    (
        "qualification-receipt.json",
        2057,
        "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8",
    ),
    (
        "release-seal.json",
        1901,
        "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c",
    ),
    (
        "root-cause-report.json",
        1266,
        "sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd",
    ),
    (
        "root-cause-report.md",
        1727,
        "sha256:9cfedd9a239bc869c35b728564267c206db981126c502121bce43a68b533b92e",
    ),
    (
        "tests/test_checkout_verifier.py",
        11991,
        "sha256:02a57858dbb65cb678b614e0a906a8bab6f9437d69efd2cbc60fac0d4b689440",
    ),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
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
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
    (bytes, value)
}

fn decode_sha256(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64, "SHA-256 hex length");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte =
            u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).expect("SHA-256 hexadecimal");
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
                .expect("sha256 prefix"),
        ));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release directory entry");
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
                    .expect("release-relative file")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn python() -> PathBuf {
    env::var_os("PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("python"))
}

#[test]
fn pulse_41_transactional_copy_release_is_sealed_and_qualified() {
    let root = repo_root();
    let release = root.join(RELEASE);

    let expected_release_paths = BTreeSet::from([
        "README.md".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "tests/test_transactional_copy.py".to_owned(),
        "transactional_copy.py".to_owned(),
    ]);
    let mut actual_release_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_release_paths);
    assert_eq!(actual_release_paths, expected_release_paths);
    for path in &actual_release_paths {
        read_lf(release.join(path));
    }

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-41-transactional-copy-public-manifest/v1"
    );
    assert_eq!(manifest["file_count"], 5);
    assert_eq!(manifest["manifest_payload_file_count"], 5);
    assert_eq!(manifest["release_tree_file_count"], 8);
    assert_eq!(manifest["total_bytes"], 49_120);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let manifest_files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(manifest_files.len(), 5);
    assert_eq!(manifest_aggregate(manifest_files), MANIFEST_AGGREGATE);
    let mut manifest_total = 0_u64;
    let mut manifest_paths = BTreeSet::new();
    for file in manifest_files {
        let path = file["path"].as_str().expect("manifest path");
        assert!(
            !Path::new(path).is_absolute(),
            "manifest path stays relative"
        );
        assert!(!path.contains(".."), "manifest path stays contained");
        let bytes = read_lf(release.join(path));
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("manifest size")
        );
        assert_eq!(sha256(&bytes), file["sha256"], "{path} binding");
        manifest_total += bytes.len() as u64;
        manifest_paths.insert(path.to_owned());
    }
    assert_eq!(manifest_total, 49_120);
    assert_eq!(
        manifest_paths,
        BTreeSet::from([
            "README.md".to_owned(),
            "root-cause-report.json".to_owned(),
            "root-cause-report.md".to_owned(),
            "tests/test_transactional_copy.py".to_owned(),
            "transactional_copy.py".to_owned(),
        ])
    );

    let p39 = root.join(P39_RELEASE);
    let mut p39_total = 0_u64;
    for (path, size, digest) in P39_BINDINGS {
        let bytes = read_lf(p39.join(path));
        assert_eq!(bytes.len() as u64, size, "Pulse 39 {path} size");
        assert_eq!(sha256(&bytes), digest, "Pulse 39 {path} digest");
        p39_total += bytes.len() as u64;
    }
    assert_eq!(p39_total, 31_800);

    let (report_bytes, report) = read_json(release.join("root-cause-report.json"));
    assert_eq!(sha256(&report_bytes), REPORT_RAW);
    assert_eq!(report["payload_sha256"], REPORT_PAYLOAD);
    assert_eq!(report["report_id"], REPORT_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&report["payload"]), REPORT_PAYLOAD);
    assert_eq!(report["payload"]["exact_private_cause_provable"], false);
    assert_eq!(
        report["payload"]["bounded_public_classes"],
        serde_json::json!([
            "stale-staging-path-after-rename",
            "duplicated-or-omitted-release-root",
            "wrong-cwd-or-relative-root",
            "pre-final-sync-verification"
        ])
    );
    assert_eq!(report["payload"]["pulse_40"]["disposition"], "invalid");
    assert_eq!(
        report["payload"]["pulse_40"]["blocker_gate"],
        "pulse-39-release-custody"
    );
    assert_eq!(report["payload"]["pulse_40"]["retry_authorized"], false);
    assert_eq!(
        report["payload"]["durability_control"],
        "Every staged destination file is flushed and fsynced before close; staging directories sync bottom-up; post-rename rollback requires path absence plus a synced or explicit unsupported parent-sync posture."
    );

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    assert_eq!(
        receipt["payload"]["pulse_39_source"]["source_bindings"],
        "8/8"
    );
    assert_eq!(
        receipt["payload"]["pulse_39_source"]["release_tree_bytes"],
        31_800
    );
    assert_eq!(
        receipt["payload"]["pulse_39_source"]["manifest_payload_bytes"],
        26_455
    );
    assert_eq!(
        receipt["payload"]["qualification"]["success_cycles"],
        "20/20"
    );
    assert_eq!(
        receipt["payload"]["qualification"]["failure_control_test_methods"],
        "11/11"
    );
    assert_eq!(
        receipt["payload"]["qualification"]["python_test_methods"],
        17
    );
    assert_eq!(receipt["payload"]["qualification"]["retries"], 0);
    assert_eq!(
        receipt["payload"]["successful_invocation"],
        serde_json::json!({
            "final": "8/8",
            "rename_attempts": 1,
            "retries": 0,
            "source": "8/8",
            "stage": "8/8"
        })
    );
    let staging_posture = &receipt["payload"]["directory_sync_posture"]["staging"];
    assert_eq!(staging_posture["status"], "unsupported");
    assert_eq!(
        staging_posture["mechanism"],
        "os.open+os.fsync-directory-v1"
    );
    assert_eq!(staging_posture["directories"], 2);
    assert_eq!(staging_posture["attempts"], 2);
    assert_eq!(staging_posture["synced"], 0);
    assert_eq!(staging_posture["unsupported"], 2);
    assert_eq!(staging_posture["operational_failures"], 0);
    assert_eq!(
        staging_posture["unsupported_error_categories"],
        serde_json::json!(["unsupported-by-platform-or-filesystem"])
    );
    let final_parent_posture = &receipt["payload"]["directory_sync_posture"]["final_parent"];
    assert_eq!(final_parent_posture["status"], "unsupported");
    assert_eq!(
        final_parent_posture["mechanism"],
        "os.open+os.fsync-directory-v1"
    );
    assert_eq!(
        final_parent_posture["error_category"],
        "unsupported-by-platform-or-filesystem"
    );
    let rollback_parent_posture = &receipt["payload"]["directory_sync_posture"]["rollback_parent"];
    assert_eq!(rollback_parent_posture["attempted"], false);
    assert_eq!(rollback_parent_posture["status"], "unsupported");
    assert_eq!(rollback_parent_posture["mechanism"], "not-attempted");
    assert_eq!(
        receipt["payload"]["durability_controls"]["staged_destinations"],
        "8/8-flush-and-os.fsync-before-close"
    );
    assert_eq!(
        receipt["payload"]["durability_controls"]["staging_directories"],
        "2/2-bottom-up-tests-then-staging-root"
    );
    assert_eq!(
        receipt["payload"]["durability_controls"]["rollback_proof"],
        "final-path-absent-plus-rollback-parent-synced-or-explicit-unsupported"
    );

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(seal["payload"]["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(seal["payload"]["manifest"]["total_bytes"], 49_120);
    assert_eq!(
        seal["payload"]["qualification_receipt"]["raw_sha256"],
        RECEIPT_RAW
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["payload_sha256"],
        RECEIPT_PAYLOAD
    );
    assert_eq!(
        seal["payload"]["root_cause_report"]["raw_sha256"],
        REPORT_RAW
    );
    assert_eq!(
        seal["payload"]["root_cause_report"]["payload_sha256"],
        REPORT_PAYLOAD
    );
    assert_eq!(seal["payload"]["pulse_40"]["disposition"], "invalid");
    assert_eq!(
        seal["payload"]["release_limits"]["diagnostic_authority"],
        false
    );
    assert_eq!(seal["payload"]["release_limits"]["ferris_execution"], false);
    assert_eq!(
        seal["payload"]["durability_controls"]["rollback_proof"],
        "final-path-absent-plus-rollback-parent-synced-or-explicit-unsupported"
    );

    let output = Command::new(python())
        .arg(release.join("tests/test_transactional_copy.py"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 41 Python qualification");
    assert!(
        output.status.success(),
        "Pulse 41 Python qualification failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Ran 17 tests"),
        "Python qualification count: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
