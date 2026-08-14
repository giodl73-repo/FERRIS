use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn aggregate(files: &BTreeMap<String, (String, Vec<u8>)>, kind: Option<&str>) -> String {
    let mut hasher = Sha256::new();
    for (path, (file_kind, bytes)) in files {
        if kind.is_some_and(|expected| file_kind != expected) {
            continue;
        }
        let path_bytes = path.as_bytes();
        hasher.update((path_bytes.len() as u64).to_be_bytes());
        hasher.update(path_bytes);
        hasher.update(Sha256::digest(bytes));
    }
    format!("sha256:{:x}", hasher.finalize())
}

#[test]
fn qualified_collector_source_bundle_is_exact_public_and_reproducible() {
    let manifest_bytes = fs::read(root().join("public-manifest.json")).expect("read manifest");
    let receipt_bytes = fs::read(root().join("release-receipt.json")).expect("read receipt");
    let seal_bytes = fs::read(root().join("release-seal.json")).expect("read seal");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse manifest");
    let receipt: Value = serde_json::from_slice(&receipt_bytes).expect("parse receipt");

    assert_eq!(
        sha256(&manifest_bytes),
        "sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75"
    );
    assert_eq!(
        sha256(&receipt_bytes),
        "sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780"
    );
    assert_eq!(
        sha256(&seal_bytes),
        "sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0"
    );
    assert_eq!(manifest["file_count"], 9);
    assert_eq!(
        manifest["forbidden_content_attestation"]["zero_forbidden_content"],
        true
    );
    assert_eq!(receipt["disposition"], "pass");
    assert_eq!(receipt["copy_verification"]["byte_for_byte_passed"], 9);
    assert_eq!(receipt["unit_tests"]["total"]["passed"], 20);
    assert_eq!(receipt["unit_tests"]["total"]["failed"], 0);
    assert_eq!(receipt["synthetic_qualification"]["pair_count"], 20);
    assert_eq!(receipt["synthetic_qualification"]["command_failed"], 0);
    assert_eq!(
        receipt["synthetic_qualification"]["residue"]["generated_paths_remaining_in_bundle"],
        0
    );
    assert_eq!(receipt["prohibitions_observed"]["ferris_executed"], false);

    let mut files = BTreeMap::new();
    for file in manifest["files"].as_array().expect("manifest files") {
        let path = file["path"].as_str().expect("file path");
        let kind = file["kind"].as_str().expect("file kind");
        let bytes = fs::read(root().join("bundle").join(path)).expect("read bundle file");
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("file size")
        );
        assert_eq!(sha256(&bytes), file["sha256"]);
        let text = String::from_utf8(bytes.clone()).expect("bundle source UTF-8");
        for forbidden in [
            "C:\\",
            ".p22-custody-",
            "PULSE-17",
            "\"seed\"",
            "candidate_bytes",
        ] {
            assert!(
                !text.contains(forbidden),
                "bundle file {path} disclosed forbidden material: {forbidden}"
            );
        }
        files.insert(path.to_owned(), (kind.to_owned(), bytes));
    }

    assert_eq!(
        aggregate(&files, Some("source")),
        manifest["digests"]["source_aggregate"]
    );
    assert_eq!(
        aggregate(&files, Some("test")),
        manifest["digests"]["test_aggregate"]
    );
    assert_eq!(
        aggregate(&files, None),
        manifest["digests"]["bundle_aggregate"]
    );
    assert_eq!(
        manifest["digests"]["source_aggregate"],
        "sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558"
    );
    assert_eq!(
        manifest["digests"]["test_aggregate"],
        "sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62"
    );
    assert_eq!(
        manifest["digests"]["bundle_aggregate"],
        "sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc"
    );
}
