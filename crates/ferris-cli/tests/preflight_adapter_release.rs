use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

const RELEASE_AGGREGATE: &str =
    "sha256:31f38a79629d6b5da1fab9cb335450a95a1763f1ac80b1d8d851b103a318e540";
const MANIFEST_DIGEST: &str =
    "sha256:449851e7b917f474fb1829b2d9f89a3f08a886733c476889dfad1ae27d097154";
const ROOT_CAUSE_DIGEST: &str =
    "sha256:9bd5ac7aa29b1f621df09eefc2ff33369c5ba5810e4e5f76f4e7e15aab57f478";
const QUALIFICATION_DIGEST: &str =
    "sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886";
const RELEASE_SEAL_DIGEST: &str =
    "sha256:6bff93434170ee79a4b5210ee26b647ab6dba351dccc4ccf3a5e224de56ced38";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release")
}

fn pulse_25_bundle() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/bundle",
    )
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = fs::read(path).expect("read LF JSON");
    assert!(!bytes.contains(&b'\r'), "JSON must use LF framing");
    assert!(bytes.ends_with(b"\n"), "JSON must end with LF");
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

#[test]
fn pulse_27_public_adapter_release_is_exact_qualified_and_collector_immutable() {
    let (manifest_bytes, manifest) = read_lf_json(root().join("public-manifest.json"));
    let (root_cause_bytes, root_cause) = read_lf_json(root().join("root-cause-report.json"));
    let (qualification_bytes, qualification) =
        read_lf_json(root().join("qualification-receipt.json"));
    let (seal_bytes, seal) = read_lf_json(root().join("release-seal.json"));
    let (_, audit) = read_lf_json(root().join("audit-report.json"));
    let (_, reproduction) = read_lf_json(root().join("reproduction-receipt.json"));

    assert_eq!(sha256(&manifest_bytes), MANIFEST_DIGEST);
    assert_eq!(sha256(&root_cause_bytes), ROOT_CAUSE_DIGEST);
    assert_eq!(sha256(&qualification_bytes), QUALIFICATION_DIGEST);
    assert_eq!(sha256(&seal_bytes), RELEASE_SEAL_DIGEST);
    assert_eq!(manifest["file_count"], 20);
    assert_eq!(manifest["collector_modified"], false);
    assert_eq!(
        manifest["scope"],
        "Public synthetic infrastructure adapter qualification only"
    );

    let mut files = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for file in manifest["files"].as_array().expect("manifest files") {
        let path = file["path"].as_str().expect("file path");
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

        let kind = file["kind"].as_str().expect("file kind");
        let bytes = fs::read(root().join(relative)).expect("read release file");
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("file size")
        );
        assert_eq!(sha256(&bytes), file["sha256"]);
        files.insert(path.to_owned(), (kind.to_owned(), bytes));
    }

    assert_eq!(files.len(), 20);
    assert_eq!(aggregate(&files, None), RELEASE_AGGREGATE);
    assert_eq!(
        aggregate(&files, Some(&["adapter-source"])),
        manifest["digests"]["adapter_source_aggregate"]
    );
    assert_eq!(
        aggregate(&files, Some(&["adapter-test"])),
        manifest["digests"]["adapter_test_aggregate"]
    );
    assert_eq!(
        aggregate(
            &files,
            Some(&["immutable-collector-source", "immutable-collector-test"])
        ),
        manifest["digests"]["collector_bundle_aggregate"]
    );
    assert_eq!(manifest["digests"]["release_aggregate"], RELEASE_AGGREGATE);

    let collector_files = files
        .iter()
        .filter(|(_, (kind, _))| kind.starts_with("immutable-collector-"))
        .collect::<Vec<_>>();
    assert_eq!(collector_files.len(), 9);
    for (path, (_, bytes)) in collector_files {
        let source_path = path
            .strip_prefix("collector/")
            .expect("collector release path");
        assert_eq!(
            bytes,
            &fs::read(pulse_25_bundle().join(source_path)).expect("read Pulse 25 collector file"),
            "Pulse 27 changed immutable collector file {path}"
        );
    }

    assert_eq!(
        root_cause["blocker"],
        "preflight-cardinality-reload-failure"
    );
    assert_eq!(root_cause["collector_conclusion"]["status"], "correct");
    assert_eq!(
        root_cause["collector_conclusion"]["modification_needed"],
        false
    );
    let cause = root_cause["public_safe_root_cause"]
        .as_str()
        .expect("public root cause");
    for required in [
        "whole-store cardinality",
        "pair-local orchestration",
        "count one",
        "extra row",
    ] {
        assert!(
            cause.contains(required),
            "missing root-cause term {required}"
        );
    }
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["cycles_passed"],
        50
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["process_rows"],
        200
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["pair_seals"],
        100
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["fresh_process_reloads"],
        100
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["retries_per_cycle"],
        0
    );
    assert_eq!(root_cause["public_evidence"]["qualification"]["residue"], 0);

    assert_eq!(qualification["schema"], "collector-sealed-json-v1");
    assert_eq!(
        qualification["payload_sha256"],
        canonical_payload_sha256(&qualification["payload"])
    );
    let receipt = &qualification["payload"];
    assert_eq!(receipt["outcome"], "pass");
    assert_eq!(receipt["cycles_required"], 50);
    assert_eq!(receipt["cycles_run"], 50);
    assert_eq!(receipt["cycles_passed"], 50);
    assert_eq!(receipt["cycles_failed"], 0);
    assert_eq!(receipt["process_row_count"], 200);
    assert_eq!(receipt["pair_seal_count"], 100);
    assert_eq!(receipt["fresh_process_reload_count"], 100);
    assert_eq!(receipt["retries_per_cycle"], 0);
    assert_eq!(receipt["residue_count"], 0);
    assert_eq!(receipt["disposable_stores_remaining"], 0);
    let cycles = receipt["cycle_receipts"]
        .as_array()
        .expect("cycle receipts");
    assert_eq!(cycles.len(), 50);
    for cycle in cycles {
        assert_eq!(cycle["pair_count"], 2);
        assert_eq!(cycle["process_record_count"], 4);
        assert_eq!(cycle["pair_seal_count"], 2);
        assert_eq!(cycle["fresh_process_reload_count"], 2);
        assert_eq!(cycle["retries"], 0);
        assert_eq!(cycle["residue_count"], 0);
    }

    assert_eq!(audit["outcome"], "pass");
    assert_eq!(audit["files_audited"], 19);
    assert_eq!(
        audit["findings"].as_array().expect("audit findings").len(),
        0
    );
    assert_eq!(audit["atomic_write_residue_files"], 0);
    assert_eq!(audit["generated_stores_remaining"], 0);

    assert_eq!(reproduction["schema"], "collector-sealed-json-v1");
    assert_eq!(
        reproduction["payload_sha256"],
        canonical_payload_sha256(&reproduction["payload"])
    );
    assert_eq!(
        reproduction["payload"]["generic_trigger"],
        "A growing two-row store was reloaded with a pair-local expected count of one. The collector correctly rejected the extra row."
    );
    assert_eq!(reproduction["payload"]["durable_process_records"], 4);
    assert_eq!(reproduction["payload"]["durable_pair_seals"], 1);
    assert_eq!(reproduction["payload"]["retries"], 0);
    assert_eq!(reproduction["payload"]["residue_count"], 0);

    assert_eq!(seal["schema"], "exact-two-preflight-release-seal-v1");
    assert_eq!(
        seal["payload_sha256"],
        canonical_payload_sha256(&seal["payload"])
    );
    assert_eq!(seal["payload"]["collector_modified"], false);
    assert_eq!(seal["payload"]["release"]["file_count"], 20);
    assert_eq!(seal["payload"]["release"]["aggregate"], RELEASE_AGGREGATE);
    assert_eq!(
        seal["payload"]["artifacts"]["root-cause-report.json"]["sha256"],
        ROOT_CAUSE_DIGEST
    );
    assert_eq!(
        seal["payload"]["artifacts"]["qualification-receipt.json"]["sha256"],
        QUALIFICATION_DIGEST
    );
    assert_eq!(seal["payload"]["qualification"]["cycles_passed"], 50);
    assert_eq!(seal["payload"]["qualification"]["process_rows"], 200);
    assert_eq!(seal["payload"]["qualification"]["pair_seals"], 100);
    assert_eq!(seal["payload"]["qualification"]["residue"], 0);
}
