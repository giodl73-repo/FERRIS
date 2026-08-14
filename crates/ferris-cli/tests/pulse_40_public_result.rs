use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const AUTHORITY: &str = "26c555ca58c1fed2e1c6222f85216c3b6e779dae";
const CUTOFF: &str = "65d1eec688f53bf7263ecfc8094ac849f9d3be4c";
const DECLARATION: &str = "sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52";
const RESULT_RAW: &str = "sha256:b91ca8ed81a17ddcdb819044e2fa42be53a319a0dec71aaef2ca59b22f9352ca";
const RESULT_PAYLOAD: &str =
    "sha256:53a9260f9cd52bb04d65e6bdf1f996a8fa0562c530fdbede09f6a37307a5699e";
const RECEIPT_RAW: &str = "sha256:5175c3b659968f3a8d442bf48450f912772e692229bf58316a7c5108fc279312";
const RECEIPT: &str = "sha256:6e78c4e808c24c42f6dbe1df1565768b53a3f71549b82e65621c2e72f4e62237";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn result_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out/pulse-40-public-result")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf_json(name: &str) -> (Vec<u8>, Value) {
    let bytes = fs::read(result_root().join(name)).expect("read Pulse 40 result artifact");
    assert!(!bytes.contains(&b'\r'), "{name} must contain no CR");
    assert!(bytes.ends_with(b"\n"), "{name} must end in LF");
    let value = serde_json::from_slice(&bytes).expect("parse Pulse 40 JSON");
    (bytes, value)
}

#[test]
fn pulse_40_public_result_seals_the_post_copy_invalid_stop() {
    let (result_bytes, result) = read_lf_json("public-result.json");
    assert_eq!(sha256(&result_bytes), RESULT_RAW);
    assert_eq!(result["payload_sha256"], RESULT_PAYLOAD);
    assert_eq!(
        sha256(&serde_json::to_vec(&result["payload"]).expect("serialize result payload")),
        RESULT_PAYLOAD
    );

    let payload = &result["payload"];
    assert_eq!(payload["authority_commit"], AUTHORITY);
    assert_eq!(payload["cutoff"], CUTOFF);
    assert_eq!(payload["declaration_identity"], DECLARATION);
    assert_eq!(payload["disposition"], "invalid");
    assert_eq!(payload["category_conclusion"], Value::Null);
    assert_eq!(payload["blocker_gate"], "pulse-39-release-custody");
    assert_eq!(
        payload["blocker_code"],
        "P40-P39-CUSTODY-VALIDATION-FAILURE"
    );
    assert_eq!(payload["sanitized_reproducer"], Value::Null);
    assert_eq!(payload["no_reproduction_aggregate"], Value::Null);

    let counts = &payload["counts"];
    assert_eq!(counts["authority_mutation_controls_verified"], "9076/9076");
    assert_eq!(counts["cutoff_p39_release_tree_verified"], "8/8");
    assert_eq!(counts["cutoff_p39_raw_bindings_verified"], "8/8");
    assert_eq!(
        counts["cutoff_p39_manifest_payload_bindings_verified"],
        "5/5"
    );
    assert_eq!(counts["cutoff_p39_manifest_payload_bytes"], 26_455);
    assert_eq!(counts["p39_release_files_copied"], "8/8");
    assert_eq!(
        counts["p39_post_copy_raw_binding_transaction_completed"],
        "0/8"
    );
    assert_eq!(counts["cutoff_checkouts_created"], 0);
    assert_eq!(counts["verifier_git_processes"], 0);
    assert_eq!(counts["normalized_bindings_verified"], "0/76");
    assert_eq!(counts["private_seeds_created"], 0);
    assert_eq!(counts["descriptors_materialized"], "0/70");
    assert_eq!(counts["search_executions"], 0);
    assert_eq!(counts["candidate_processes_total"], 0);
    assert_eq!(counts["retries"], 0);

    let privacy = payload["privacy"].as_object().expect("privacy object");
    for (field, value) in privacy {
        assert_eq!(value, &Value::Bool(false), "privacy.{field}");
    }

    let (receipt_bytes, receipt) = read_lf_json("release-receipt.json");
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT);
    assert_eq!(receipt["receipt_id"], RECEIPT);
    assert_eq!(
        sha256(&serde_json::to_vec(&receipt["payload"]).expect("serialize receipt payload")),
        RECEIPT
    );
    assert_eq!(receipt["payload"]["result_raw_sha256"], RESULT_RAW);
    assert_eq!(receipt["payload"]["result_payload_sha256"], RESULT_PAYLOAD);
    assert_eq!(receipt["payload"]["category_conclusion"], Value::Null);
}
