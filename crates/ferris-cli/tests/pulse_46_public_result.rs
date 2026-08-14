use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const AUTHORITY_COMMIT: &str = "a80111845f942b75e985c412389bfe6a89ccdc99";
const CUTOFF: &str = "22ea38e274b882d6e607810382f842b76e483f10";
const DECLARATION: &str = "sha256:92847e645338fd142710c1afcff5d6ad5540c35e6322ccf59b574f2fd3d61534";
const RESULT_RAW: &str = "sha256:0bdda2927857ca90c1a64bed49d4236ee6d7545199ee8c40765a832862862862";
const RESULT_PAYLOAD: &str =
    "sha256:e526f44d93148018ecae62e6f539d3436cd87a7c9969533ad753a371829942df";
const RECEIPT_RAW: &str = "sha256:0a12dc7abcf096751cac36a1ef8a961876ad6cd9e2936d008b7379481b728015";
const RECEIPT: &str = "sha256:ffa1c9598b3689bb8e82cb6bac23a70d2279c0c7c75cace1646d9e46c33a5a5a";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn result_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out/pulse-46-public-result")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf_json(name: &str) -> (Vec<u8>, Value) {
    let bytes = fs::read(result_root().join(name)).expect("read Pulse 46 result artifact");
    assert!(!bytes.contains(&b'\r'), "{name} must contain no CR");
    assert!(bytes.ends_with(b"\n"), "{name} must end in LF");
    let value = serde_json::from_slice(&bytes).expect("parse Pulse 46 JSON");
    (bytes, value)
}

fn assert_all_privacy_disclosures_false(privacy: &Value, name: &str) {
    let privacy = privacy.as_object().expect("privacy object");
    assert!(!privacy.is_empty(), "{name} privacy disclosures");
    for (field, value) in privacy {
        assert_eq!(value, &Value::Bool(false), "{name}.{field}");
    }
}

#[test]
fn pulse_46_closeout_is_public_safe_indeterminate_and_non_retryable() {
    let (result_bytes, result) = read_lf_json("public-result.json");
    assert_eq!(sha256(&result_bytes), RESULT_RAW);
    assert_eq!(
        result["schema"],
        "ferris.pulse-46-public-result-envelope/v1"
    );
    assert_eq!(result["payload_sha256"], RESULT_PAYLOAD);
    assert_eq!(
        sha256(&serde_json::to_vec(&result["payload"]).expect("serialize result payload")),
        RESULT_PAYLOAD
    );

    let payload = &result["payload"];
    assert_eq!(payload["schema"], "ferris.pulse-46-public-result/v1");
    assert_eq!(payload["disposition"], "invalid-publication-integrity");
    assert_eq!(payload["blocker"]["code"], "P46-PUBLICATION-INDETERMINATE");
    assert_eq!(payload["blocker"]["stage"], "public-result-publication");
    assert_eq!(payload["launches"], 1);
    assert_eq!(payload["retries"], 0);
    assert_eq!(
        payload["public_custodian_statement"],
        "Publication posture: indeterminate. The required final public-result directory is absent."
    );

    assert_eq!(payload["authority"]["authority_commit"], AUTHORITY_COMMIT);
    assert_eq!(payload["authority"]["cutoff"], CUTOFF);
    assert_eq!(payload["authority"]["declaration_identity"], DECLARATION);

    let publication = &payload["publication_integrity"];
    assert_eq!(
        publication
            .as_object()
            .expect("publication integrity object")
            .len(),
        3
    );
    assert_eq!(publication["expected_transactional_result_files_absent"], 1);
    assert_eq!(publication["claimed_or_observed_result_paths"], 0);
    assert_eq!(publication["publication_state"], "indeterminate");

    assert_eq!(
        payload["conclusions"]
            .as_object()
            .expect("conclusions object")
            .len(),
        4
    );
    assert_eq!(payload["conclusions"]["category"], Value::Null);
    assert_eq!(payload["conclusions"]["diagnostic"], Value::Null);
    assert_eq!(payload["conclusions"]["product"], Value::Null);
    assert_eq!(payload["conclusions"]["fix_authority"], false);
    let ordered_execution = payload["ordered_execution"]
        .as_object()
        .expect("ordered execution object");
    assert_eq!(ordered_execution.len(), 4);
    for field in [
        "ordered_gate_attempts",
        "ordered_gate_completions",
        "terminal_gate",
        "search_count",
    ] {
        assert_eq!(
            payload["ordered_execution"][field],
            Value::Null,
            "{field} is indeterminate"
        );
    }
    assert!(
        payload.get("ordered_gate_counts").is_none()
            && payload.get("reported_unvalidated").is_none()
            && payload.get("claimed_private_blocker").is_none(),
        "the closeout cannot publish gate counts or a private blocker"
    );

    assert_eq!(payload["retry_policy"]["permanently_non_retryable"], true);
    assert_eq!(payload["retry_policy"]["retries_permitted"], 0);
    assert_eq!(payload["identity"]["pulse"], 46);
    assert_eq!(payload["identity"]["sole_independent_launch"], true);
    assert_eq!(
        payload["identity"]["not_pulse_43_transactional_result"],
        true
    );
    assert_all_privacy_disclosures_false(&payload["privacy"], "result privacy");

    let (receipt_bytes, receipt) = read_lf_json("release-receipt.json");
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-46-public-result-receipt-envelope/v1"
    );
    assert_eq!(receipt["payload_sha256"], RECEIPT);
    assert_eq!(receipt["receipt_id"], RECEIPT);
    assert_eq!(
        sha256(&serde_json::to_vec(&receipt["payload"]).expect("serialize receipt payload")),
        RECEIPT
    );

    let receipt_payload = &receipt["payload"];
    assert_eq!(
        receipt_payload["schema"],
        "ferris.pulse-46-public-result-receipt/v1"
    );
    assert_eq!(receipt_payload["authority"], payload["authority"]);
    assert_eq!(receipt_payload["result_raw_sha256"], RESULT_RAW);
    assert_eq!(receipt_payload["result_payload_sha256"], RESULT_PAYLOAD);
    assert_eq!(receipt_payload["disposition"], payload["disposition"]);
    assert_eq!(receipt_payload["blocker"], payload["blocker"]);
    assert_eq!(receipt_payload["launches"], 1);
    assert_eq!(receipt_payload["retries"], 0);
    assert_eq!(receipt_payload["publication_state"], "indeterminate");
    assert_eq!(receipt_payload["conclusions"], payload["conclusions"]);
    assert_eq!(
        receipt_payload["ordered_execution"],
        payload["ordered_execution"]
    );
    assert_eq!(receipt_payload["retry_policy"], payload["retry_policy"]);
    assert_eq!(receipt_payload["not_pulse_43_transactional_result"], true);
    assert_all_privacy_disclosures_false(&receipt_payload["privacy"], "receipt privacy");

    let readme = fs::read_to_string(result_root().join("README.md")).expect("read closeout README");
    assert!(readme.contains("not** the failed Pulse 43 transactional result"));
    assert!(readme.contains("P46-PUBLICATION-INDETERMINATE"));

    let attributes =
        fs::read_to_string(repo_root().join(".gitattributes")).expect("read attributes");
    assert!(
        attributes.contains(
            "/docs/simulations/profile-diff-held-out/pulse-46-public-result/** text eol=lf"
        )
    );
}
