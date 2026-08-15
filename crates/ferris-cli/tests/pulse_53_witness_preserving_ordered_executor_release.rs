use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-53-witness-preserving-ordered-executor-release";
const MANIFEST_RAW: &str =
    "sha256:6825130932c02e9cbd822896aba5ac7780f6f9231bd0c2de1b9cfba57b525ddc";
const MANIFEST_AGGREGATE: &str =
    "sha256:6ad0ecdacebd8229f7cb5f653b44d030951ccf5990f4c2f915ad1d1c7872a896";
const RECEIPT_RAW: &str = "sha256:83950c762647d53ac60e3b19e71baa113c826d8e8f981400f4a5b2b54da32899";
const RECEIPT_PAYLOAD: &str =
    "sha256:b2ee7ca22f35e867c9c27b1ed351d910db784d1c9124318d73f27858abebd0a3";
const SEAL_RAW: &str = "sha256:769ab51cc827149545ad12139ffda05a5764f76a05c3a6e3c3f34eb29e23e4e0";
const SEAL_PAYLOAD: &str =
    "sha256:bb46d2ca5a838bfe15466c27b1fc6a608a7e3c5d1f504c2ef60aec44a08f0716";

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
        .expect("run Pulse 53 Python validation")
}

#[test]
fn pulse_53_witness_preserving_ordered_executor_is_sealed_and_fake_only() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/p53-public-gate-catalog.json".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-53-witness-preserving-ordered-executor.v1.schema.json".to_owned(),
        "sealed_dependencies.py".to_owned(),
        "synthetic_fixture.py".to_owned(),
        "tests/test_witness_preserving_ordered_executor.py".to_owned(),
        "witness_preserving_ordered_executor.py".to_owned(),
    ]);
    assert_no_release_scratch(&release);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let source = String::from_utf8(read_lf(
        release.join("witness_preserving_ordered_executor.py"),
    ))
    .expect("UTF-8 source");
    assert!(source.contains("run_witness_preserving_ordered_executor"));
    assert!(source.contains("published-failure-witness"));
    assert!(source.contains("invalid-witness-publication"));
    assert!(source.contains("load_pulse52"));
    assert!(!source.contains("run_ordered_materialization_executor("));

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-53-witness-preserving-ordered-executor-public-manifest/v1"
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
        actual_paths.len() as u64
    );
    assert_eq!(
        manifest["predecessors"]["pulse_51"]["commit"],
        "d09c923c1e2cd2be003026597f4ad2a0e2d3764f"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_52"]["commit"],
        "e4ef9617f227670f3911be42ca63df4b2e66d24f"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_52"]["source_sha256"],
        "sha256:768f4dc3af1009515e2e28ebc211af76215f434cee209b547d7be923a1bcec73"
    );

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-53-witness-preserving-ordered-executor-qualification-envelope/v1"
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
    assert_eq!(payload["published_results"], 10);
    assert_eq!(payload["published_failure_witnesses"], 10);
    assert_eq!(payload["invalid_witness_publications"], 0);
    assert_eq!(payload["failure_witness_postures"]["absent"], 4);
    assert_eq!(payload["failure_witness_postures"]["rolled-back"], 3);
    assert_eq!(payload["failure_witness_postures"]["indeterminate"], 3);
    assert_eq!(payload["ferris_executed"], false);
    assert_eq!(payload["seed_values_disclosed"], false);
    assert_eq!(payload["descriptor_paths_disclosed"], false);
    for cycle in payload["cycles"].as_array().expect("qualification cycles") {
        assert!(
            cycle["publication_disposition"] == "published-result"
                || cycle["publication_disposition"] == "published-failure-witness"
        );
    }

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(
        seal["schema"],
        "ferris.pulse-53-witness-preserving-ordered-executor-release-seal/v1"
    );
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
    assert_eq!(
        seal["payload"]["limits"]["terminal_failure_witness_disposition"],
        "published-failure-witness"
    );
    assert_eq!(
        seal["payload"]["limits"]["terminal_failure_disposition"],
        "invalid-witness-publication"
    );
    assert_eq!(
        seal["payload"]["limits"]["production_injection_accepted"],
        false
    );
    assert_eq!(
        seal["payload"]["limits"]["transfer_descriptor_paths_or_ids"],
        false
    );

    let schema: Value = serde_json::from_slice(&read_lf(
        release.join("schemas/ferris.pulse-53-witness-preserving-ordered-executor.v1.schema.json"),
    ))
    .expect("parse public return schema");
    assert_eq!(
        schema["$id"],
        "urn:ferris:schema:pulse-53-witness-preserving-ordered-executor:v1"
    );
    assert_eq!(
        schema["$defs"]["terminalPublication"]["properties"]["disposition"]["enum"],
        serde_json::json!([
            "not-attempted",
            "published-result",
            "published-failure-witness",
            "invalid-witness-publication"
        ])
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
    assert_eq!(qualified["published_results"], 10);
    assert_eq!(qualified["published_failure_witnesses"], 10);
    assert_eq!(qualified["ferris_executed"], false);
    assert_no_release_scratch(&release);
    let mut final_paths = BTreeSet::new();
    collect_files(&release, &release, &mut final_paths);
    assert_eq!(final_paths, expected_paths);
}
