use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "48697c8da0e93b92fa633e353925ca05707bf9ed";
const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release";
const OLD_MANIFEST_RAW: &str =
    "sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b";
const OLD_MANIFEST_AGGREGATE: &str =
    "sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7";
const OLD_SEAL_RAW: &str =
    "sha256:51edf2f2df9210291705332fa8a4c3b55cb2a19a1aff22ecd882434a5ebefef2";
const OLD_SEAL_PAYLOAD: &str =
    "sha256:5b5e4383ffe5274f36f355069a5339c1684674aea342229f54f63ef247d21e52";
const NEW_MANIFEST_RAW: &str =
    "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1";
const NEW_MANIFEST_AGGREGATE: &str =
    "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69";
const NEW_SEAL_RAW: &str =
    "sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23";
const NEW_SEAL_PAYLOAD: &str =
    "sha256:834781867ea008dc14a54d7b811002ee1b8fa759c0b1d7f32432ea6c0d5c5375";
const RECEIPT_RAW: &str = "sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6";
const RECEIPT_ID: &str = "sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae";
const PULSE_36_RESULT_RAW: &str =
    "sha256:735353e311dc63cd0cdef85c112bd60fd2c50c18f29858929a58f886b34009cc";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn release_root() -> PathBuf {
    repo_root().join(RELEASE)
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn cutoff_blob(path: &str) -> Vec<u8> {
    let output = Command::new("git")
        .current_dir(repo_root())
        .args(["show", &format!("{CUTOFF}:{path}")])
        .output()
        .expect("read cutoff blob");
    assert!(output.status.success(), "missing cutoff blob {path}");
    output.stdout
}

fn clean_filter_lf(bytes: Vec<u8>, path: &Path) -> Vec<u8> {
    let mut clean = Vec::with_capacity(bytes.len());
    let mut source = bytes.into_iter();
    while let Some(byte) = source.next() {
        if byte == b'\r' {
            assert_eq!(
                source.next(),
                Some(b'\n'),
                "{path:?} contains a non-checkout CR byte"
            );
            clean.push(b'\n');
        } else {
            clean.push(byte);
        }
    }
    assert!(
        !clean.contains(&b'\r'),
        "{path:?} Git-clean materialization must contain no CR bytes"
    );
    clean
}

fn read_git_clean(path: &Path) -> Vec<u8> {
    clean_filter_lf(fs::read(path).expect("read current artifact"), path)
}

fn manifest_file<'a>(manifest: &'a Value, path: &str) -> &'a Value {
    manifest["files"]
        .as_array()
        .expect("manifest files")
        .iter()
        .find(|entry| entry["path"] == path)
        .unwrap_or_else(|| panic!("missing manifest entry {path}"))
}

fn manifest_aggregate(files: &[Value]) -> String {
    let mut digest = Sha256::new();
    for file in files {
        digest.update(file["size"].as_u64().expect("size").to_string());
        digest.update(b"\0");
        digest.update(file["path"].as_str().expect("path"));
        digest.update(b"\0");
        digest.update(
            file["sha256"]
                .as_str()
                .expect("digest")
                .strip_prefix("sha256:")
                .expect("sha256 prefix"),
        );
        digest.update(b"\n");
    }
    format!("sha256:{:x}", digest.finalize())
}

fn crlf_from_lf(bytes: &[u8]) -> Vec<u8> {
    assert!(!bytes.contains(&b'\r'), "Git blob must be LF-only");
    let mut crlf = Vec::with_capacity(bytes.len() + bytes.iter().filter(|&&b| b == b'\n').count());
    for byte in bytes {
        if *byte == b'\n' {
            crlf.extend_from_slice(b"\r\n");
        } else {
            crlf.push(*byte);
        }
    }
    crlf
}

#[test]
fn pulse_37_rebinds_the_current_release_to_cutoff_git_clean_bytes() {
    let old_manifest_bytes = cutoff_blob(&format!("{RELEASE}/public-manifest.json"));
    assert_eq!(sha256(&old_manifest_bytes), OLD_MANIFEST_RAW);
    let old_manifest: Value = serde_json::from_slice(&old_manifest_bytes).expect("old manifest");
    assert_eq!(old_manifest["aggregate"], OLD_MANIFEST_AGGREGATE);
    assert_eq!(old_manifest["total_bytes"], 405_414);
    assert_eq!(
        manifest_aggregate(old_manifest["files"].as_array().expect("old files")),
        OLD_MANIFEST_AGGREGATE
    );

    let current_manifest_bytes = read_git_clean(&release_root().join("public-manifest.json"));
    assert_eq!(sha256(&current_manifest_bytes), NEW_MANIFEST_RAW);
    let current_manifest: Value =
        serde_json::from_slice(&current_manifest_bytes).expect("current manifest");
    let files = current_manifest["files"].as_array().expect("current files");
    assert_eq!(files.len(), 8);
    assert_eq!(current_manifest["aggregate"], NEW_MANIFEST_AGGREGATE);
    assert_eq!(current_manifest["total_bytes"], 403_316);
    assert_eq!(manifest_aggregate(files), NEW_MANIFEST_AGGREGATE);

    let text_deltas = [
        ("README.md", -91_i64),
        ("corpus_materializer.py", -970),
        ("qualify.py", -188),
        ("root-cause-report.md", -10),
        ("tests/test_materializer.py", -203),
        ("verify_materialization.py", -636),
    ];
    let mut total = 0_u64;
    for entry in files {
        let path = entry["path"].as_str().expect("current path");
        let current = read_git_clean(&release_root().join(path));
        let cutoff = cutoff_blob(&format!("{RELEASE}/{path}"));
        assert_eq!(
            current, cutoff,
            "{path} must bind its exact cutoff Git-clean bytes"
        );
        assert_eq!(current.len() as u64, entry["size"], "{path} size");
        assert_eq!(sha256(&current), entry["sha256"], "{path} digest");
        total += current.len() as u64;
    }
    assert_eq!(total, 403_316);

    for (path, delta) in text_deltas {
        let old = manifest_file(&old_manifest, path);
        let current = manifest_file(&current_manifest, path);
        let cutoff = cutoff_blob(&format!("{RELEASE}/{path}"));
        assert_eq!(
            current["size"].as_i64().expect("current size")
                - old["size"].as_i64().expect("old size"),
            delta,
            "{path} CRLF-to-LF delta"
        );
        assert_eq!(
            cutoff.len() as i64 - old["size"].as_i64().expect("old size"),
            delta
        );
        assert_eq!(
            sha256(&crlf_from_lf(&cutoff)),
            old["sha256"],
            "{path} old CRLF seal"
        );
        assert_eq!(sha256(&cutoff), current["sha256"], "{path} new LF binding");
    }

    for path in ["qualification-receipt.json", "root-cause-report.json"] {
        let old = manifest_file(&old_manifest, path);
        let current = manifest_file(&current_manifest, path);
        let cutoff = cutoff_blob(&format!("{RELEASE}/{path}"));
        assert_eq!(
            current["size"], old["size"],
            "{path} size must remain unchanged"
        );
        assert_eq!(
            current["sha256"], old["sha256"],
            "{path} digest must remain unchanged"
        );
        assert_eq!(sha256(&cutoff), old["sha256"], "{path} cutoff digest");
    }

    let old_seal_bytes = cutoff_blob(&format!("{RELEASE}/release-seal.json"));
    assert_eq!(sha256(&old_seal_bytes), OLD_SEAL_RAW);
    let old_seal: Value = serde_json::from_slice(&old_seal_bytes).expect("old seal");
    assert_eq!(old_seal["payload_sha256"], OLD_SEAL_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&old_seal["payload"]),
        OLD_SEAL_PAYLOAD
    );

    let current_seal_bytes = read_git_clean(&release_root().join("release-seal.json"));
    assert_eq!(sha256(&current_seal_bytes), NEW_SEAL_RAW);
    let current_seal: Value = serde_json::from_slice(&current_seal_bytes).expect("current seal");
    assert_eq!(current_seal["payload_sha256"], NEW_SEAL_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&current_seal["payload"]),
        NEW_SEAL_PAYLOAD
    );
    assert_eq!(
        current_seal["payload"]["manifest"]["sha256"],
        NEW_MANIFEST_RAW
    );
    assert_eq!(
        current_seal["payload"]["manifest"]["aggregate"],
        NEW_MANIFEST_AGGREGATE
    );
    assert_eq!(current_seal["payload"]["manifest"]["total_bytes"], 403_316);
    for field in [
        "qualification_receipt",
        "root_cause_report",
        "public_contracts",
        "release_limits",
    ] {
        assert_eq!(
            current_seal["payload"][field], old_seal["payload"][field],
            "Pulse 37 must preserve {field}"
        );
    }
    assert_eq!(
        current_seal["payload"]["release_limits"]["diagnostic_execution"],
        false
    );
    assert_eq!(
        current_seal["payload"]["release_limits"]["product_files_modified"],
        false
    );
}

#[test]
fn pulse_37_receipt_seals_windows_clean_filter_materialization() {
    let receipt_path = repo_root().join(
        "docs/simulations/profile-diff-held-out/pulse-37-checkout-normalization/PULSE-37-CHECKOUT-NORMALIZATION-RECEIPT.json",
    );
    let receipt_bytes = read_git_clean(&receipt_path);
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    let receipt: Value = serde_json::from_slice(&receipt_bytes).expect("Pulse 37 receipt");
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-37-public-artifact-checkout-normalization-receipt/v1"
    );
    assert_eq!(receipt["payload_sha256"], RECEIPT_ID);
    assert_eq!(receipt["receipt_id"], RECEIPT_ID);
    assert_eq!(canonical_payload_sha256(&receipt["payload"]), RECEIPT_ID);

    let payload = &receipt["payload"];
    assert_eq!(payload["pulse"], 37);
    assert_eq!(payload["disposition"], "pass");
    assert_eq!(payload["artifact"]["source_cutoff"], CUTOFF);
    assert_eq!(
        payload["gitattributes"]["exact_rule"],
        format!("/{RELEASE}/** text eol=lf")
    );
    assert_eq!(payload["materialization"]["platform"], "Windows_NT");
    assert_eq!(payload["materialization"]["core_autocrlf"], true);
    assert_eq!(
        payload["materialization"]["method"],
        "temporary-index-git-add-clean-filter-checkout-index"
    );
    assert_eq!(
        payload["materialization"]["source"],
        "resulting-uncommitted-index"
    );
    assert_eq!(payload["materialization"]["working_tree_copy_used"], false);
    assert_eq!(
        payload["materialization"]["release_tree_oid"],
        "fcc9e21f1adc5cb42c97d47cba8058ad09c77679"
    );
    assert_eq!(payload["binding_checks"]["files_expected"], 8);
    assert_eq!(payload["binding_checks"]["files_passed"], 8);
    assert_eq!(payload["binding_checks"]["size_hash_bindings_passed"], 8);
    assert_eq!(payload["binding_checks"]["individual_size_checks"], 8);
    assert_eq!(payload["binding_checks"]["individual_sha256_checks"], 8);
    assert_eq!(payload["binding_checks"]["failed"], 0);
    assert_eq!(payload["line_endings"]["text_files_checked"], 6);
    assert_eq!(payload["line_endings"]["text_files_lf_passed"], 6);
    assert_eq!(payload["line_endings"]["text_cr_bytes_observed"], 0);
    assert_eq!(
        payload["line_endings"]["all_release_file_cr_bytes_observed"],
        0
    );
    assert_eq!(
        payload["unchanged_files"],
        serde_json::json!(["qualification-receipt.json", "root-cause-report.json"])
    );

    assert_eq!(
        payload["old_cutoff_release"]["manifest"]["raw_sha256"],
        OLD_MANIFEST_RAW
    );
    assert_eq!(
        payload["old_cutoff_release"]["manifest"]["aggregate"],
        OLD_MANIFEST_AGGREGATE
    );
    assert_eq!(
        payload["old_cutoff_release"]["manifest"]["total_bytes"],
        405_414
    );
    assert_eq!(
        payload["old_cutoff_release"]["release_seal"]["raw_sha256"],
        OLD_SEAL_RAW
    );
    assert_eq!(
        payload["old_cutoff_release"]["release_seal"]["payload_sha256"],
        OLD_SEAL_PAYLOAD
    );
    assert_eq!(
        payload["new_normalized_successor"]["manifest"]["raw_sha256"],
        NEW_MANIFEST_RAW
    );
    assert_eq!(
        payload["new_normalized_successor"]["manifest"]["aggregate"],
        NEW_MANIFEST_AGGREGATE
    );
    assert_eq!(
        payload["new_normalized_successor"]["manifest"]["total_bytes"],
        403_316
    );
    assert_eq!(
        payload["new_normalized_successor"]["release_seal"]["raw_sha256"],
        NEW_SEAL_RAW
    );
    assert_eq!(
        payload["new_normalized_successor"]["release_seal"]["payload_sha256"],
        NEW_SEAL_PAYLOAD
    );
    assert_eq!(
        payload["new_normalized_successor"]["release_limits_preserved"],
        true
    );

    let manifest: Value = serde_json::from_slice(&read_git_clean(
        &release_root().join("public-manifest.json"),
    ))
    .expect("current manifest");
    let receipt_files = payload["files"].as_array().expect("receipt files");
    assert_eq!(receipt_files.len(), 8);
    for entry in receipt_files {
        let path = entry["path"].as_str().expect("receipt path");
        let manifest_entry = manifest_file(&manifest, path);
        assert_eq!(entry["new"]["size"], manifest_entry["size"], "{path} size");
        assert_eq!(
            entry["new"]["sha256"], manifest_entry["sha256"],
            "{path} digest"
        );
    }

    assert_eq!(
        payload["pulse_36_historical_result"]["disposition"],
        "invalid-before-pulse35-materialization"
    );
    assert_eq!(payload["pulse_36_historical_result"]["immutable"], true);
    for field in [
        "diagnostic_execution",
        "ferris_executed",
        "new_diagnostic_authority",
        "product_files_modified",
    ] {
        assert_eq!(payload["prohibitions_observed"][field], false, "{field}");
    }
}

#[test]
fn pulse_37_indexes_and_historical_pulse_36_result_remain_consistent() {
    let attributes = fs::read_to_string(repo_root().join(".gitattributes")).expect("attributes");
    for rule in [
        format!("/{RELEASE}/** text eol=lf"),
        "/context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-37.md text eol=lf"
            .to_owned(),
        "/docs/plans/reviews/PULSE-37-PUBLIC-ARTIFACT-CHECKOUT-NORMALIZATION-ROLE-REVIEW.md text eol=lf"
            .to_owned(),
        "/docs/simulations/profile-diff-held-out/pulse-37-checkout-normalization/** text eol=lf"
            .to_owned(),
        "/crates/ferris-cli/tests/pulse_37_public_artifact_checkout_normalization.rs text eol=lf"
            .to_owned(),
    ] {
        assert!(attributes.lines().any(|line| line == rule), "missing {rule}");
    }

    let pulse_37 = fs::read_to_string(
        repo_root()
            .join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-37.md"),
    )
    .expect("Pulse 37 document");
    for identity in [
        OLD_MANIFEST_RAW,
        OLD_MANIFEST_AGGREGATE,
        OLD_SEAL_RAW,
        OLD_SEAL_PAYLOAD,
        NEW_MANIFEST_RAW,
        NEW_MANIFEST_AGGREGATE,
        NEW_SEAL_RAW,
        NEW_SEAL_PAYLOAD,
        RECEIPT_ID,
    ] {
        assert!(pulse_37.contains(identity), "Pulse 37 identity {identity}");
    }
    assert!(pulse_37.contains("no diagnostic authority or execution"));

    for relative in [
        "CONTEXT.md",
        "README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "docs/simulations/profile-diff-held-out/README.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("current index");
        assert!(text.contains("Pulse 37"), "{relative} must index Pulse 37");
        assert!(
            text.contains("Pulse 36"),
            "{relative} must preserve Pulse 36"
        );
        assert!(
            text.contains("invalid"),
            "{relative} must preserve its invalid result"
        );
    }

    let result_path = repo_root().join(
        "docs/simulations/profile-diff-held-out/pulse-36-public-result/PULSE-36-PUBLIC-RESULT.json",
    );
    let result_bytes = read_git_clean(&result_path);
    assert_eq!(sha256(&result_bytes), PULSE_36_RESULT_RAW);
    let result: Value = serde_json::from_slice(&result_bytes).expect("Pulse 36 result");
    assert_eq!(
        result["payload"]["disposition"],
        "invalid-before-pulse35-materialization"
    );
    assert_eq!(
        result["payload"]["stop_stage"],
        "pulse35-release-copy-verification"
    );
    assert_eq!(result["payload"]["further_launches_prohibited"], true);
}
