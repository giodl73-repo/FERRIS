use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

const PULSE_25_MANIFEST: &str =
    "sha256:621ed59a5b2124204180be109f69010ac18337a09816c8d28e67713f63efb419";
const PULSE_25_SOURCE: &str =
    "sha256:71b41689202e0ee3c956c9e5408284deac63e53004530b717a403266237d73a7";
const PULSE_25_TEST: &str =
    "sha256:5de010365b3c1297144de030c1738e998e9f55994dee1497d0600b178b2d3de9";
const PULSE_25_BUNDLE: &str =
    "sha256:e296329ff56fad14eba2274d928f45c0fdf6a281db3d2d554c1cee3814d4b406";
const PULSE_25_QUALIFICATION: &str =
    "sha256:04491bea4828fd7329d622c84f9b186d7315dbb31d491176598ffee09be4499e";
const PULSE_25_RECEIPT: &str =
    "sha256:4ec9d50c4ff0f4ba8b65d57751fad28f2a1fcd610e67e664f1727baeb78aaf69";
const PULSE_25_SEAL: &str =
    "sha256:f1d10da9395f2b9f3834da260b6f11e365153ed5b33a75b937d7c410d9c08e1e";

const PULSE_27_MANIFEST: &str =
    "sha256:7a6e61dacb3d58ab6d8c75cf1267a70f7919219baadd34329b835640931e8d5e";
const PULSE_27_ADAPTER_SOURCE: &str =
    "sha256:cdca8d4a0206c9553c637b9228511cfa07e401b9082d96c439d112e2b25c6071";
const PULSE_27_ADAPTER_TEST: &str =
    "sha256:426bd87a7695bb2d5cefdb4c98fc4bef1524616100365656c2e3bc2c19747dff";
const PULSE_27_COLLECTOR: &str =
    "sha256:7a4645f3d3f5e7dcee709351d802e76d1ae6333a7a3b92412fe41d8ae656fc5b";
const PULSE_27_RELEASE: &str =
    "sha256:531113c7c8a50f1c71c446bc708e44549702623114625ea46f5aa874b6aea721";
const PULSE_27_ROOT_CAUSE: &str =
    "sha256:5f1760b7f7cf318029ea24407ef20a087340af16eb2991d7d0b7b0495efded1c";
const PULSE_27_QUALIFICATION: &str =
    "sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886";
const PULSE_27_SEAL: &str =
    "sha256:8abcc449d4b4aff30ed3ade168fa59c7f159e68d3172180703971bb79f096a6e";

const NORMALIZATION_RECEIPT: &str =
    "sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn pulse_25_root() -> PathBuf {
    held_out_root().join("pulse-25-collector-source-release")
}

fn pulse_27_root() -> PathBuf {
    held_out_root().join("pulse-27-preflight-adapter-release")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF file");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR bytes");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end with LF");
    bytes
}

fn read_lf_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse LF JSON");
    (bytes, value)
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn aggregate(files: &BTreeMap<String, (String, Vec<u8>)>, kinds: Option<&[&str]>) -> String {
    let mut hasher = Sha256::new();
    for (path, (kind, bytes)) in files {
        if kinds.is_some_and(|accepted| !accepted.contains(&kind.as_str())) {
            continue;
        }
        let path_bytes = path.as_bytes();
        hasher.update((path_bytes.len() as u64).to_be_bytes());
        hasher.update(path_bytes);
        hasher.update(Sha256::digest(bytes));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn manifest_files(
    root: &Path,
    manifest: &Value,
    nested_root: Option<&str>,
) -> BTreeMap<String, (String, Vec<u8>)> {
    let mut files = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for file in manifest["files"].as_array().expect("manifest files") {
        let path = file["path"].as_str().expect("manifest path");
        let relative = Path::new(path);
        assert!(!relative.is_absolute(), "manifest path must be relative");
        assert!(
            !relative.components().any(|component| matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )),
            "manifest path must remain inside the release"
        );
        assert!(paths.insert(path.to_owned()), "duplicate manifest path");
        let kind = file["kind"].as_str().expect("manifest kind");
        let disk_path = nested_root.map_or_else(
            || root.join(relative),
            |nested| root.join(nested).join(relative),
        );
        let bytes = read_lf(disk_path);
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("manifest size")
        );
        assert_eq!(sha256(&bytes), file["sha256"]);
        files.insert(path.to_owned(), (kind.to_owned(), bytes));
    }
    files
}

fn recursive_files(root: &Path) -> Vec<PathBuf> {
    fn visit(root: &Path, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(root).expect("read release directory") {
            let path = entry.expect("release entry").path();
            if path.is_dir() {
                visit(&path, files);
            } else {
                files.push(path);
            }
        }
    }

    let mut files = Vec::new();
    visit(root, &mut files);
    files.sort();
    files
}

#[test]
fn pulse_29_attributes_and_current_release_bindings_are_exact_lf() {
    let attributes =
        fs::read_to_string(repo_root().join(".gitattributes")).expect("read .gitattributes");
    for rule in [
        "/docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/** text eol=lf",
        "/docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/** text eol=lf",
        "/docs/simulations/profile-diff-held-out/pulse-28-public-result/** text eol=lf",
        "/docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/** text eol=lf",
    ] {
        assert!(
            attributes.lines().any(|line| line == rule),
            "missing exact checkout rule {rule}"
        );
    }

    let pulse_25_files = recursive_files(&pulse_25_root());
    let pulse_27_files = recursive_files(&pulse_27_root());
    assert_eq!(pulse_25_files.len(), 14);
    assert_eq!(pulse_27_files.len(), 22);
    for path in pulse_25_files.iter().chain(&pulse_27_files) {
        read_lf(path);
    }

    let (manifest_bytes, manifest) = read_lf_json(pulse_25_root().join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), PULSE_25_MANIFEST);
    let files = manifest_files(&pulse_25_root(), &manifest, Some("bundle"));
    assert_eq!(files.len(), 9);
    assert_eq!(aggregate(&files, Some(&["source"])), PULSE_25_SOURCE);
    assert_eq!(aggregate(&files, Some(&["test"])), PULSE_25_TEST);
    assert_eq!(aggregate(&files, None), PULSE_25_BUNDLE);
    assert_eq!(manifest["digests"]["source_aggregate"], PULSE_25_SOURCE);
    assert_eq!(manifest["digests"]["test_aggregate"], PULSE_25_TEST);
    assert_eq!(manifest["digests"]["bundle_aggregate"], PULSE_25_BUNDLE);
    assert_eq!(
        sha256(&read_lf(pulse_25_root().join("qualification-report.json"))),
        PULSE_25_QUALIFICATION
    );
    assert_eq!(
        sha256(&read_lf(pulse_25_root().join("release-receipt.json"))),
        PULSE_25_RECEIPT
    );
    assert_eq!(
        sha256(&read_lf(pulse_25_root().join("release-seal.json"))),
        PULSE_25_SEAL
    );

    let (manifest_bytes, manifest) = read_lf_json(pulse_27_root().join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), PULSE_27_MANIFEST);
    let files = manifest_files(&pulse_27_root(), &manifest, None);
    assert_eq!(files.len(), 20);
    assert_eq!(
        aggregate(&files, Some(&["adapter-source"])),
        PULSE_27_ADAPTER_SOURCE
    );
    assert_eq!(
        aggregate(&files, Some(&["adapter-test"])),
        PULSE_27_ADAPTER_TEST
    );
    assert_eq!(
        aggregate(
            &files,
            Some(&["immutable-collector-source", "immutable-collector-test"])
        ),
        PULSE_27_COLLECTOR
    );
    assert_eq!(aggregate(&files, None), PULSE_27_RELEASE);
    assert_eq!(
        sha256(&read_lf(pulse_27_root().join("root-cause-report.json"))),
        PULSE_27_ROOT_CAUSE
    );
    assert_eq!(
        sha256(&read_lf(pulse_27_root().join("qualification-receipt.json"))),
        PULSE_27_QUALIFICATION
    );
    assert_eq!(
        sha256(&read_lf(pulse_27_root().join("release-seal.json"))),
        PULSE_27_SEAL
    );

    for file in manifest["files"].as_array().expect("Pulse 27 files") {
        let kind = file["kind"].as_str().expect("Pulse 27 kind");
        if kind.starts_with("immutable-collector-") {
            let path = file["path"]
                .as_str()
                .expect("collector path")
                .strip_prefix("collector/")
                .expect("collector prefix");
            assert_eq!(
                read_lf(pulse_27_root().join("collector").join(path)),
                read_lf(pulse_25_root().join("bundle").join(path))
            );
        }
    }
}

#[test]
fn pulse_29_checkout_normalization_receipt_is_exact_and_sealed() {
    let receipt_path = held_out_root()
        .join("pulse-29-checkout-normalization")
        .join("PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json");
    let (bytes, receipt) = read_lf_json(receipt_path);
    assert_eq!(sha256(&bytes), NORMALIZATION_RECEIPT);
    assert_eq!(
        receipt["schema"],
        "ferris.public-artifact-checkout-normalization-receipt/v1"
    );
    assert_eq!(receipt["receipt_id"], receipt["payload_sha256"]);
    assert_eq!(
        receipt["payload_sha256"],
        canonical_payload_sha256(&receipt["payload"])
    );

    let payload = &receipt["payload"];
    assert_eq!(payload["disposition"], "pass");
    assert_eq!(
        payload["materialization"]["method"],
        "temporary-index-checkout-index"
    );
    assert_eq!(payload["materialization"]["core_autocrlf"], true);
    assert_eq!(
        payload["materialization"]["source"],
        "resulting-uncommitted-index"
    );
    assert_eq!(payload["line_endings"]["files_checked"], 36);
    assert_eq!(payload["line_endings"]["lf_passed"], 36);
    assert_eq!(payload["line_endings"]["cr_bytes_observed"], 0);
    assert_eq!(payload["binding_checks"]["passed"], 76);
    assert_eq!(payload["binding_checks"]["failed"], 0);
    assert_eq!(payload["binding_checks"]["pulse_25"], 22);
    assert_eq!(payload["binding_checks"]["pulse_27"], 45);
    assert_eq!(payload["binding_checks"]["collector_identity"], 9);
    assert_eq!(payload["pulse_25"]["manifest_sha256"], PULSE_25_MANIFEST);
    assert_eq!(payload["pulse_25"]["source_aggregate"], PULSE_25_SOURCE);
    assert_eq!(payload["pulse_25"]["test_aggregate"], PULSE_25_TEST);
    assert_eq!(payload["pulse_25"]["bundle_aggregate"], PULSE_25_BUNDLE);
    assert_eq!(payload["pulse_27"]["manifest_sha256"], PULSE_27_MANIFEST);
    assert_eq!(
        payload["pulse_27"]["adapter_source_aggregate"],
        PULSE_27_ADAPTER_SOURCE
    );
    assert_eq!(
        payload["pulse_27"]["adapter_test_aggregate"],
        PULSE_27_ADAPTER_TEST
    );
    assert_eq!(
        payload["pulse_27"]["collector_aggregate"],
        PULSE_27_COLLECTOR
    );
    assert_eq!(payload["pulse_27"]["release_aggregate"], PULSE_27_RELEASE);
    assert_eq!(
        payload["prohibitions_observed"]["diagnostic_candidates_executed"],
        0
    );
    assert_eq!(
        payload["prohibitions_observed"]["private_data_accessed"],
        false
    );
    assert_eq!(payload["pulse_28"]["public_result_modified"], false);
    assert_eq!(
        payload["pulse_28"]["public_result_sha256"],
        "sha256:955bb0e2f0ca614a988fbd72ae8abca43b411e46bf2416885d4238ab447309a2"
    );
}
