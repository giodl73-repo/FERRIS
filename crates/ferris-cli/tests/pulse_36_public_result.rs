use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY: &str = "2bf480459614dc56ee2bd744302e79f20a571092";
const CUTOFF: &str = "48697c8da0e93b92fa633e353925ca05707bf9ed";
const RESULT_RAW: &str = "sha256:735353e311dc63cd0cdef85c112bd60fd2c50c18f29858929a58f886b34009cc";
const RECEIPT: &str = "sha256:d1f6f648ae8bb9a1fc44def2d392b72b76446b49439ff8f31e4124ad1fafc628";
const PULSE_35_MANIFEST: &str =
    "sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn result_path() -> PathBuf {
    repo_root().join(
        "docs/simulations/profile-diff-held-out/pulse-36-public-result/PULSE-36-PUBLIC-RESULT.json",
    )
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn git_blob(path: &str) -> Vec<u8> {
    let output = Command::new("git")
        .current_dir(repo_root())
        .args(["show", &format!("{CUTOFF}:{path}")])
        .output()
        .expect("read cutoff blob");
    assert!(output.status.success(), "cutoff artifact missing: {path}");
    output.stdout
}

fn manifest_file<'a>(manifest: &'a Value, path: &str) -> &'a Value {
    manifest["files"]
        .as_array()
        .expect("manifest files")
        .iter()
        .find(|file| file["path"] == path)
        .unwrap_or_else(|| panic!("missing manifest file: {path}"))
}

fn crlf_from_lf(bytes: &[u8]) -> Vec<u8> {
    assert!(!bytes.contains(&b'\r'), "cutoff text blob must be LF-only");
    let mut normalized =
        Vec::with_capacity(bytes.len() + bytes.iter().filter(|&&b| b == b'\n').count());
    for byte in bytes {
        if *byte == b'\n' {
            normalized.extend_from_slice(b"\r\n");
        } else {
            normalized.push(*byte);
        }
    }
    normalized
}

#[test]
fn pulse_36_public_result_envelope_preserves_gates_counts_and_privacy() {
    let bytes = fs::read(result_path()).expect("read Pulse 36 result");
    assert_eq!(sha256(&bytes), RESULT_RAW);
    assert!(!bytes.contains(&b'\r'), "result bytes must remain LF");
    assert!(bytes.ends_with(b"\n"), "result must end in LF");

    let envelope: Value = serde_json::from_slice(&bytes).expect("parse result envelope");
    assert_eq!(
        envelope["schema"],
        "ferris.pulse-36-public-result-envelope/v1"
    );
    assert_eq!(envelope["payload_sha256"], RECEIPT);
    assert_eq!(envelope["receipt_id"], RECEIPT);
    assert_eq!(canonical_payload_sha256(&envelope["payload"]), RECEIPT);

    let payload = &envelope["payload"];
    assert_eq!(payload["schema"], "ferris.pulse-36-public-result/v1");
    assert_eq!(
        payload["program"],
        "FERRIS-P36-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-MATERIALIZED-PUBLIC-AUTHORITY"
    );
    assert_eq!(payload["authority_commit"], AUTHORITY);
    assert_eq!(payload["cutoff"], CUTOFF);
    assert_eq!(
        payload["disposition"],
        "invalid-before-pulse35-materialization"
    );
    assert_eq!(payload["stop_stage"], "pulse35-release-copy-verification");
    assert_eq!(payload["further_launches_prohibited"], true);
    assert_eq!(payload["target_result"]["category_conclusion"], Value::Null);
    assert_eq!(payload["target_result"]["fix_authority"], false);

    let gates = &payload["gates"];
    assert_eq!(gates["cutoff_authority_binding"]["passed"], true);
    assert_eq!(
        gates["cutoff_authority_binding"]["cutoff_predates_authority"],
        true
    );
    assert_eq!(gates["normalization"]["files_checked"], 36);
    assert_eq!(gates["normalization"]["lf_passed"], 36);
    assert_eq!(gates["normalization"]["text_eol_attributes"], 36);
    assert_eq!(gates["normalization"]["cr_bytes_observed"], 0);
    assert_eq!(gates["pulse25_27_bindings"]["total_passed"], 76);
    assert_eq!(gates["pulse25_27_bindings"]["pulse25_passed"], 22);
    assert_eq!(gates["pulse25_27_bindings"]["pulse27_passed"], 45);
    assert_eq!(gates["pulse25_27_bindings"]["collector_identity_passed"], 9);
    assert_eq!(gates["pulse33_release"]["manifest_files_verified"], 37);
    assert_eq!(
        gates["pulse33_release"]["receipt_seal_payloads_verified"],
        3
    );
    assert_eq!(gates["cutoff_freeze"]["platforms"], 2);
    assert_eq!(gates["cutoff_freeze"]["binaries"], 2);
    assert_eq!(
        gates["cutoff_freeze"]["cargo_compiler_artifact_discovery"],
        2
    );
    assert_eq!(gates["cutoff_freeze"]["receipts"], 2);

    let preflight = &gates["pulse27_preflight"];
    assert_eq!(preflight["adapter_invocations"], 1);
    assert_eq!(preflight["pairs"], 2);
    assert_eq!(preflight["process_rows"], 4);
    assert_eq!(preflight["pair_seals"], 2);
    assert_eq!(preflight["fresh_verifiers"], 2);
    assert_eq!(preflight["whole_store_cardinality"], "2/2/2");
    assert_eq!(preflight["retries"], 0);
    assert_eq!(preflight["residue"], 0);
    assert_eq!(gates["pulse31"]["artifacts_verified"], 9);
    assert_eq!(gates["pulse31"]["validation_passed"], 39);
    assert_eq!(gates["pulse31"]["validation_total"], 39);
    assert_eq!(gates["pulse31"]["positive_accepts"], 6);
    assert_eq!(gates["pulse31"]["negative_classifications"], 33);

    let blocker = &payload["public_safe_blocker"];
    assert_eq!(blocker["code"], "P36-P35-MANIFEST-BINDING-MISMATCH");
    assert_eq!(blocker["copy_attempts"], 1);
    assert_eq!(blocker["copy_retries"], 0);
    assert_eq!(blocker["expected_file_count"], 8);
    assert_eq!(blocker["copied_file_count"], 8);
    assert_eq!(blocker["manifest_file_bindings_matched"], 2);
    assert_eq!(blocker["manifest_file_bindings_mismatched"], 6);
    assert_eq!(blocker["expected_total_bytes"], 405_414);
    assert_eq!(blocker["observed_total_bytes"], 403_316);
    assert_eq!(gates["pulse35_release"]["copy_verified"], false);
    assert_eq!(
        gates["pulse35_release"]["manifest_raw_sha256"],
        PULSE_35_MANIFEST
    );

    let materialization = &payload["materialization"];
    assert_eq!(materialization["seed_generated"], false);
    assert_eq!(materialization["materializer_processes"], 0);
    assert_eq!(materialization["fresh_seed_verifier_processes"], 0);
    assert_eq!(materialization["descriptors"], 0);
    assert_eq!(materialization["domains_derived"], "0/18");
    assert_eq!(materialization["interactions_derived"], "0/8");
    assert_eq!(materialization["tuple_counts"], Value::Null);
    assert_eq!(materialization["rename_attempts"], 0);
    assert_eq!(materialization["logical_retries"], 0);
    assert_eq!(materialization["residue"], 0);

    let search = &payload["search"];
    for field in [
        "candidate_cases_per_platform",
        "candidate_pair_seals",
        "candidate_processes",
        "candidate_retries",
        "collection_residue",
        "fresh_candidate_reload_processes",
        "minimization_processes",
        "minimization_transformations",
        "paired_records",
    ] {
        assert_eq!(search[field], 0, "{field}");
    }
    assert_eq!(search["coverage"], Value::Null);
    assert_eq!(search["oracle_classifier_frozen"], false);

    let privacy = payload["privacy"].as_object().expect("privacy object");
    assert!(!privacy.is_empty());
    for (field, disclosed) in privacy {
        assert_eq!(disclosed, &Value::Bool(false), "{field}");
    }
}

#[test]
fn pulse_36_reproduces_six_crlf_seals_against_lf_cutoff_blobs() {
    let manifest_bytes = git_blob(
        "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/public-manifest.json",
    );
    assert_eq!(sha256(&manifest_bytes), PULSE_35_MANIFEST);
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse Pulse 35 manifest");
    assert_eq!(manifest["file_count"], 8);
    assert_eq!(manifest["total_bytes"], 405_414);

    let attributes = String::from_utf8(git_blob(".gitattributes")).expect("UTF-8 attributes");
    assert!(attributes.contains(
        "/docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/** text eol=lf"
    ));

    let release = "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release";
    let expected_mismatches = [
        ("README.md", -91_i64),
        ("corpus_materializer.py", -970),
        ("qualify.py", -188),
        ("root-cause-report.md", -10),
        ("tests/test_materializer.py", -203),
        ("verify_materialization.py", -636),
    ];
    let mut cutoff_total = 0_u64;
    let mut matches = 0_u64;
    let mut mismatches = 0_u64;

    for file in manifest["files"].as_array().expect("manifest files") {
        let relative = file["path"].as_str().expect("manifest path");
        let blob = git_blob(&format!("{release}/{relative}"));
        cutoff_total += blob.len() as u64;
        if sha256(&blob) == file["sha256"] {
            matches += 1;
        } else {
            mismatches += 1;
        }
    }
    assert_eq!(cutoff_total, 403_316);
    assert_eq!(matches, 2);
    assert_eq!(mismatches, 6);

    for (path, cutoff_minus_sealed) in expected_mismatches {
        let file = manifest_file(&manifest, path);
        let cutoff = git_blob(&format!("{release}/{path}"));
        let sealed_size = file["size"].as_u64().expect("sealed size");
        assert_eq!(
            cutoff.len() as i64 - sealed_size as i64,
            cutoff_minus_sealed,
            "{path} cutoff delta"
        );
        assert_eq!(
            cutoff.iter().filter(|&&byte| byte == b'\n').count() as i64,
            -cutoff_minus_sealed,
            "{path} LF count"
        );
        assert_ne!(sha256(&cutoff), file["sha256"], "{path} must mismatch");

        let reconstructed_sealed = crlf_from_lf(&cutoff);
        assert_eq!(
            reconstructed_sealed.len() as u64,
            sealed_size,
            "{path} size"
        );
        assert_eq!(
            sha256(&reconstructed_sealed),
            file["sha256"],
            "{path} CRLF sealed digest"
        );
    }

    for path in ["qualification-receipt.json", "root-cause-report.json"] {
        let file = manifest_file(&manifest, path);
        let cutoff = git_blob(&format!("{release}/{path}"));
        assert_eq!(cutoff.len() as u64, file["size"], "{path} size");
        assert_eq!(sha256(&cutoff), file["sha256"], "{path} digest");
        assert!(!cutoff.contains(&b'\r'), "{path} must remain LF-only");
    }
}
