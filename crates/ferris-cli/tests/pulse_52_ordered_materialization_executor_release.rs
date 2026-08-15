use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-52-ordered-materialization-executor-release";
const MANIFEST_RAW: &str =
    "sha256:e585d6baaf83783ff1a1c65e1d3f281ce1d3afd9806f9cb9811b328eff9811da";
const MANIFEST_AGGREGATE: &str =
    "sha256:3da8401a52d020ead7b9c6854461da5f28dfb9d1117385cd6943592f74e8aaec";
const RECEIPT_RAW: &str = "sha256:1eaf50c293e4c44f9312b28efa581912ed4165e8f77014c703cfc54496b37192";
const RECEIPT_PAYLOAD: &str =
    "sha256:183a7c6f0ebbab38bbe5b29efc4c1ebd3c5e1e8ca8ca84a5cc5d29107798a7ac";
const SEAL_RAW: &str = "sha256:febee1ea581a3564da89714aaeae1c909b0a9345676958bbb6e2fe4ec2d72ca6";
const SEAL_PAYLOAD: &str =
    "sha256:46d9e8bb1aa75780fb7397fd4833e13c5e28c0ec79254185ef6da793e4ed7f84";

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
        let encoded = path.as_bytes();
        hasher.update((encoded.len() as u64).to_be_bytes());
        hasher.update(encoded);
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
            "{path:?} must not be a symlink"
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
            "{path:?} must not retain Python scratch"
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
        .expect("run Pulse 52 Python validation")
}

#[test]
fn pulse_52_ordered_materialization_executor_is_sealed_and_fake_only() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/p52-public-gate-catalog.json".to_owned(),
        "ordered_materialization_executor.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-52-ordered-materialization-executor.v1.schema.json".to_owned(),
        "sealed_dependencies.py".to_owned(),
        "synthetic_fixture.py".to_owned(),
        "tests/test_ordered_materialization_executor.py".to_owned(),
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
        "ferris.pulse-52-ordered-materialization-executor-public-manifest/v1"
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
    assert_eq!(
        manifest["predecessors"]["pulse_51"]["source_sha256"],
        "sha256:97c404dbf29d387561878772403c7fbd2672e97283b0620e838e7126ecbdd637"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_39"]["source_sha256"],
        "sha256:783283fd127170460ce52106a7a1158054cdc2608475e53899ff45a7a6a31d12"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_39"]["receipt_raw_sha256"],
        "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_39"]["seal_raw_sha256"],
        "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_39"]["checkout_file_count"],
        36
    );
    assert_eq!(
        manifest["predecessors"]["pulse_41"]["source_sha256"],
        "sha256:900a89de3401f78558970d896214568f851ca644def28639476e66154235c8cf"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_41"]["receipt_raw_sha256"],
        "sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_41"]["seal_raw_sha256"],
        "sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a"
    );
    assert_eq!(manifest["predecessors"]["pulse_41"]["copied_file_count"], 8);

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-52-ordered-materialization-executor-qualification-envelope/v1"
    );
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    let payload = &receipt["payload"];
    assert_eq!(payload["cycles_required"], 20);
    assert_eq!(payload["cycles_passed"], 20);
    assert_eq!(payload["fake_dispatches_per_cycle"], 138);
    assert_eq!(payload["fake_dispatches_total"], 2_760);
    assert_eq!(payload["topology_per_cycle"], "70/69/1");
    assert_eq!(payload["materializer_invocations_per_cycle"], 1);
    assert_eq!(payload["verifier_invocations_per_cycle"], 1);
    assert_eq!(payload["private_seed_bytes_per_cycle"], 32);
    assert_eq!(payload["seed_values_disclosed"], false);
    assert_eq!(payload["descriptor_paths_disclosed"], false);
    assert_eq!(payload["ferris_executed"], false);
    assert_eq!(payload["p43_terminal_publication_invocations"], 20);
    assert_eq!(payload["p47_terminal_publication_invocations"], 20);
    assert_eq!(payload["terminal_publication_successes"], 20);
    assert_eq!(payload["terminal_publication_failures"], 0);
    assert_eq!(
        payload["failure_boundary_hardening"],
        serde_json::json!({
            "exact_predecessor_public_failures": "bounded-prelaunch",
            "exact_terminal_publication_failures": "invalid-publication-integrity",
            "programmer_faults": "propagate",
        })
    );
    assert_eq!(payload["p39_checkout_verifications_per_cycle"], 1);
    assert_eq!(payload["p41_transactional_copy_invocations_per_cycle"], 1);
    assert_eq!(payload["p41_post_copy_binding_per_cycle"], "8/8");
    for cycle in payload["cycles"].as_array().expect("qualification cycles") {
        assert_eq!(cycle["publication_disposition"], "published");
    }

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["receipt_id"], SEAL_PAYLOAD);
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
    assert_eq!(seal["payload"]["limits"]["diagnostic_execution"], false);
    assert_eq!(seal["payload"]["limits"]["prelaunch_event_accepted"], false);
    assert_eq!(
        seal["payload"]["limits"]["terminal_failure_disposition"],
        "invalid-publication-integrity"
    );
    assert_eq!(
        seal["payload"]["limits"]["terminal_publication_success_shape_required"],
        true
    );
    assert_eq!(
        seal["payload"]["limits"]["terminal_cleanup_indeterminate_state"],
        "terminal-publication-cleanup-indeterminate"
    );
    assert_eq!(
        seal["payload"]["limits"]["programmer_faults_propagate"],
        true
    );
    assert_eq!(
        seal["payload"]["verification"]["p39_checkout_verifications_per_cycle"],
        1
    );
    assert_eq!(
        seal["payload"]["verification"]["p41_transactional_copy_invocations_per_cycle"],
        1
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
    let qualified: Value =
        serde_json::from_slice(&qualification.stdout).expect("qualification payload");
    assert_eq!(qualified["cycles_run"], 20);
    assert_eq!(qualified["fake_dispatches_total"], 2_760);
    assert_eq!(qualified["topology_per_cycle"], "70/69/1");
    assert_eq!(qualified["materializer_invocations_per_cycle"], 1);
    assert_eq!(qualified["verifier_invocations_per_cycle"], 1);
    assert_eq!(qualified["ferris_executed"], false);
    assert_no_release_scratch(&release);
    let mut final_paths = BTreeSet::new();
    collect_files(&release, &release, &mut final_paths);
    assert_eq!(final_paths, expected_paths);
}
