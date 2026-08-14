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

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
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
    let qualification_bytes =
        fs::read(root().join("qualification-report.json")).expect("read qualification");
    let receipt_bytes = fs::read(root().join("release-receipt.json")).expect("read receipt");
    let seal_bytes = fs::read(root().join("release-seal.json")).expect("read seal");
    for bytes in [
        &manifest_bytes,
        &qualification_bytes,
        &receipt_bytes,
        &seal_bytes,
    ] {
        assert!(!bytes.contains(&b'\r'), "release JSON must use LF");
        assert!(bytes.ends_with(b"\n"), "release JSON must end with LF");
    }
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse manifest");
    let qualification: Value =
        serde_json::from_slice(&qualification_bytes).expect("parse qualification");
    let receipt: Value = serde_json::from_slice(&receipt_bytes).expect("parse receipt");
    let seal: Value = serde_json::from_slice(&seal_bytes).expect("parse seal");

    assert_eq!(
        sha256(&manifest_bytes),
        "sha256:621ed59a5b2124204180be109f69010ac18337a09816c8d28e67713f63efb419"
    );
    assert_eq!(
        sha256(&qualification_bytes),
        "sha256:04491bea4828fd7329d622c84f9b186d7315dbb31d491176598ffee09be4499e"
    );
    assert_eq!(
        sha256(&receipt_bytes),
        "sha256:4ec9d50c4ff0f4ba8b65d57751fad28f2a1fcd610e67e664f1727baeb78aaf69"
    );
    assert_eq!(
        sha256(&seal_bytes),
        "sha256:f1d10da9395f2b9f3834da260b6f11e365153ed5b33a75b937d7c410d9c08e1e"
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
        assert!(!bytes.contains(&b'\r'), "bundle file {path} must use LF");
        assert!(
            bytes.ends_with(b"\n"),
            "bundle file {path} must end with LF"
        );
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
        "sha256:71b41689202e0ee3c956c9e5408284deac63e53004530b717a403266237d73a7"
    );
    assert_eq!(
        manifest["digests"]["test_aggregate"],
        "sha256:5de010365b3c1297144de030c1738e998e9f55994dee1497d0600b178b2d3de9"
    );
    assert_eq!(
        manifest["digests"]["bundle_aggregate"],
        "sha256:e296329ff56fad14eba2274d928f45c0fdf6a281db3d2d554c1cee3814d4b406"
    );
    assert_eq!(
        qualification["payload"]["source_digest"],
        manifest["digests"]["source_aggregate"]
    );
    assert_eq!(
        qualification["payload"]["test_digest"],
        manifest["digests"]["test_aggregate"]
    );
    assert_eq!(
        qualification["payload_sha256"],
        canonical_payload_sha256(&qualification["payload"])
    );
    assert_eq!(
        seal["payload_sha256"],
        canonical_payload_sha256(&seal["payload"])
    );
    assert_eq!(
        seal["payload"]["bundle"]["sha256"],
        manifest["digests"]["bundle_aggregate"]
    );
    for (name, bytes) in [
        (
            "README.md",
            fs::read(root().join("README.md")).expect("read README"),
        ),
        ("public-manifest.json", manifest_bytes),
        ("qualification-report.json", qualification_bytes),
        ("release-receipt.json", receipt_bytes),
    ] {
        assert_eq!(
            seal["payload"]["artifacts"][name]["size"],
            bytes.len() as u64
        );
        assert_eq!(seal["payload"]["artifacts"][name]["sha256"], sha256(&bytes));
    }
}
