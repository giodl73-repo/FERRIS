use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const AUTHORITY: &str = "1e060cbd105df9f8a765cd94745c40126c8a9cd9";
const CUTOFF: &str = "6807bd68aa01cbf0c819198765b7d6b5aa443328";
const RESULT_RAW: &str = "sha256:d3e74d220a9de9da4f2fff72812443de42272c9a8f78b0efad37573ab33b1c9c";
const RECEIPT: &str = "sha256:56ddacc0e3043b327b8ce2d6ce869e9662a564faee9ce4f9a2c3d783a390bdad";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn result_path() -> PathBuf {
    repo_root().join(
        "docs/simulations/profile-diff-held-out/pulse-38-public-result/PULSE-38-PUBLIC-RESULT.json",
    )
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

#[test]
fn pulse_38_public_result_preserves_the_invalid_stop_and_privacy_boundary() {
    let bytes = fs::read(result_path()).expect("read Pulse 38 result");
    let text = String::from_utf8(bytes).expect("result must be UTF-8");
    let normalized = text.replace("\r\n", "\n");
    assert!(
        !normalized.contains('\r'),
        "result must not contain bare CR"
    );
    assert_eq!(sha256(normalized.as_bytes()), RESULT_RAW);
    assert!(normalized.ends_with('\n'), "result must end in LF");

    let envelope: Value = serde_json::from_str(&normalized).expect("parse result envelope");
    assert_eq!(
        envelope["schema"],
        "ferris.pulse-38-public-result-envelope/v1"
    );
    assert_eq!(envelope["payload_sha256"], RECEIPT);
    assert_eq!(envelope["receipt_id"], RECEIPT);
    assert_eq!(
        sha256(&serde_json::to_vec(&envelope["payload"]).expect("serialize payload")),
        RECEIPT
    );

    let payload = &envelope["payload"];
    assert_eq!(payload["schema"], "ferris.pulse-38-public-result/v1");
    assert_eq!(
        payload["program"],
        "FERRIS-P38-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-NORMALIZED-PUBLIC-AUTHORITY"
    );
    assert_eq!(payload["authority_commit"], AUTHORITY);
    assert_eq!(payload["cutoff"], CUTOFF);
    assert_eq!(
        payload["disposition"],
        "invalid-before-normalized-checkout-verification"
    );
    assert_eq!(payload["stop_stage"], "normalized-checkout-verification");
    assert_eq!(payload["further_launches_prohibited"], true);
    assert_eq!(payload["target_result"]["category_conclusion"], Value::Null);
    assert_eq!(payload["target_result"]["fix_authority"], false);

    let normalization = &payload["gates"]["normalization"];
    assert_eq!(normalization["cutoff_materializations"], 1);
    assert_eq!(
        normalization["release_tree_cardinality_checks_completed"],
        1
    );
    assert_eq!(normalization["attribute_checks_attempted"], 1);
    assert_eq!(normalization["attribute_checks_completed"], 0);
    assert_eq!(normalization["lf_checks_completed"], 0);
    assert_eq!(normalization["binding_checks_completed"], 0);
    assert_eq!(normalization["retries"], 0);

    let blocker = &payload["public_safe_blocker"];
    assert_eq!(blocker["code"], "P38-NORMALIZATION-GATE-INCOMPLETE");
    assert_eq!(blocker["normalization_attempts"], 1);
    assert_eq!(blocker["normalization_retries"], 0);

    for gate in ["pulse25_27_packages", "pulse27_preflight", "pulse31"] {
        let object = payload["gates"][gate].as_object().expect("gate object");
        for (field, value) in object {
            if field != "validation_total" && field != "whole_store_cardinality" {
                assert_eq!(value, 0, "{gate}.{field}");
            }
        }
    }
    assert_eq!(
        payload["gates"]["pulse27_preflight"]["whole_store_cardinality"],
        Value::Null
    );
    assert_eq!(payload["gates"]["pulse31"]["validation_total"], 39);

    assert_eq!(payload["materialization"]["seed_generated"], false);
    assert_eq!(payload["materialization"]["descriptors"], 0);
    assert_eq!(payload["materialization"]["domains_derived"], "0/18");
    assert_eq!(payload["materialization"]["interactions_derived"], "0/8");
    assert_eq!(payload["materialization"]["tuple_counts"], Value::Null);

    let search = payload["search"].as_object().expect("search object");
    for (field, value) in search {
        if field == "coverage" {
            assert_eq!(value, &Value::Null);
        } else if field == "oracle_classifier_frozen" {
            assert_eq!(value, &Value::Bool(false));
        } else {
            assert_eq!(value, 0, "search.{field}");
        }
    }

    let privacy = payload["privacy"].as_object().expect("privacy object");
    for (field, value) in privacy {
        assert_eq!(value, &Value::Bool(false), "privacy.{field}");
    }
}
