use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const AUTHORITY_COMMIT: &str = "4825c25de764055c6d5ded84da43e0b0916fcc30";
const CUTOFF: &str = "2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8";
const DECLARATION: &str = "sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc";
const RESULT_RAW: &str = "sha256:ff0ba22671e9e08f1234db1b6a4949bf0d0f7345b975028ef19d9c3f0741e433";
const RESULT_PAYLOAD: &str =
    "sha256:5fb7cda080f5f4ec5287da1902f937325bdbeacf0f3020d1d8d4923f23e6a46b";
const RECEIPT_RAW: &str = "sha256:3313775ddbc126133b414daf279f7ab4ebf1882363b9a0c252ba23f39a05eb65";
const RECEIPT: &str = "sha256:44b87f0643dc082a9ad9166873aa12e4cc7d062d6cf9bbfaa995d83122ef11b2";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn result_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out/pulse-42-public-result")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf_json(name: &str) -> (Vec<u8>, Value) {
    let bytes = fs::read(result_root().join(name)).expect("read Pulse 42 result artifact");
    assert!(!bytes.contains(&b'\r'), "{name} must contain no CR");
    assert!(bytes.ends_with(b"\n"), "{name} must end in LF");
    let value = serde_json::from_slice(&bytes).expect("parse Pulse 42 JSON");
    (bytes, value)
}

fn authority_position(text: &str, gate: &str) -> usize {
    text.find(gate)
        .unwrap_or_else(|| panic!("Pulse 42 authority must name {gate}"))
}

#[test]
fn pulse_42_public_result_seals_publication_integrity_without_gate_conclusions() {
    let (result_bytes, result) = read_lf_json("public-result.json");
    assert_eq!(sha256(&result_bytes), RESULT_RAW);
    assert_eq!(
        result["schema"],
        "ferris.pulse-42-public-result-envelope/v1"
    );
    assert_eq!(result["payload_sha256"], RESULT_PAYLOAD);
    assert_eq!(
        sha256(&serde_json::to_vec(&result["payload"]).expect("serialize result payload")),
        RESULT_PAYLOAD
    );

    let payload = &result["payload"];
    assert_eq!(payload["schema"], "ferris.pulse-42-public-result/v1");
    assert_eq!(payload["disposition"], "invalid-publication-integrity");
    assert_eq!(payload["category_conclusion"], Value::Null);
    assert_eq!(payload["diagnostic_conclusion"], Value::Null);
    assert_eq!(payload["product_conclusion"], Value::Null);
    assert_eq!(payload["fix_authority"], false);
    assert_eq!(
        payload["stop"]["code"],
        "P42-PUBLIC-RESULT-UNAVAILABLE-ORDER-INCONSISTENT"
    );
    assert_eq!(payload["stop"]["stage"], "public-result-publication");
    assert_eq!(payload["publication_integrity"]["result_files_absent"], 1);
    assert_eq!(
        payload["publication_integrity"]["claimed_result_paths_observed"],
        0
    );
    assert_eq!(
        payload["publication_integrity"]["ordering_consistency"],
        false
    );
    assert_eq!(payload["retry_policy"]["permanently_non_retryable"], true);
    assert_eq!(payload["retry_policy"]["retries_permitted"], 0);

    assert_eq!(payload["authority"]["authority_commit"], AUTHORITY_COMMIT);
    assert_eq!(payload["authority"]["cutoff"], CUTOFF);
    assert_eq!(payload["authority"]["declaration_identity"], DECLARATION);
    assert_eq!(payload["authority"]["stop_on_failure"], true);
    assert_eq!(
        payload["authority"]["ordered_gates"],
        json!([
            "pulse-41-transactional-copy",
            "pulse-39-checkout-verifier",
            "pulse-25-27-package",
            "pulse-33-freeze",
            "adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "private-materialization",
            "search",
        ])
    );

    let gate_counts = &payload["ordered_gate_counts"];
    for gate in [
        "pulse_31_public_input_validation",
        "pulse_35_pulse_37_normalized_release",
        "private_materialization",
        "search",
    ] {
        assert_eq!(gate_counts[gate], Value::Null, "{gate} is indeterminate");
    }

    let reported = &payload["reported_unvalidated"];
    assert_eq!(reported["claimed_disposition"], "invalid/null");
    assert_eq!(reported["launches"], 1);
    assert_eq!(reported["retries"], 0);
    assert_eq!(reported["privacy_disclosure"], false);
    assert_eq!(
        reported["claimed_stop"]["stage"],
        "pulse-33-private-frozen-binary-custody"
    );
    assert_eq!(
        reported["claimed_stop"]["code"],
        "P42-FROZEN-BINARY-UNAVAILABLE"
    );
    assert_eq!(reported["counts"]["authority_controls"], "9046/9046");
    assert_eq!(reported["counts"]["pulse_41"], "8/8");
    assert_eq!(reported["counts"]["pulse_39"], "36/36");
    assert_eq!(reported["counts"]["pulse_39_bindings"], "76/76");
    assert_eq!(reported["counts"]["pulse_31"], "39/39");
    assert_eq!(reported["counts"]["pulse_35_descriptors"], 70);
    assert_eq!(reported["counts"]["pulse_35_domains"], "18/18");
    assert_eq!(reported["counts"]["pulse_35_tuple_catalogs"], "8/8");
    assert_eq!(reported["counts"]["pulse_33_builds"], "2/2");
    assert_eq!(reported["counts"]["pulse_33_retained_binaries"], "0/2");
    assert_eq!(reported["search"]["count"], "0/140");
    assert_eq!(reported["search"]["certified"], false);

    for (field, value) in payload["privacy"].as_object().expect("privacy object") {
        assert_eq!(value, &Value::Bool(false), "privacy.{field}");
    }

    let authority = fs::read_to_string(repo_root().join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_42_AUTHORITY.md",
    ))
    .expect("read public Pulse 42 authority");
    let positions = [
        authority_position(&authority, "Pulse 41 release tree"),
        authority_position(&authority, "Pulse 39 verifier"),
        authority_position(&authority, "25/27 package gate"),
        authority_position(&authority, "Pulse 33 freezes"),
        authority_position(&authority, "adapter preflight"),
        authority_position(&authority, "Pulse 31"),
        authority_position(&authority, "normalized Pulse 35 eight-file copy"),
        authority_position(&authority, "fresh regular private 32-byte seed"),
        authority_position(&authority, "one search"),
    ];
    assert!(
        positions.windows(2).all(|pair| pair[0] < pair[1]),
        "the authority places the reported Pulse 33 stop before reported P31/P35 quantities"
    );

    let (receipt_bytes, receipt) = read_lf_json("release-receipt.json");
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-42-public-result-receipt-envelope/v1"
    );
    assert_eq!(receipt["payload_sha256"], RECEIPT);
    assert_eq!(receipt["receipt_id"], RECEIPT);
    assert_eq!(
        sha256(&serde_json::to_vec(&receipt["payload"]).expect("serialize receipt payload")),
        RECEIPT
    );
    assert_eq!(receipt["payload"]["result_raw_sha256"], RESULT_RAW);
    assert_eq!(receipt["payload"]["result_payload_sha256"], RESULT_PAYLOAD);
    assert_eq!(receipt["payload"]["category_conclusion"], Value::Null);
    assert_eq!(receipt["payload"]["fix_authority"], false);
    for (field, value) in receipt["payload"]["privacy"]
        .as_object()
        .expect("receipt privacy object")
    {
        assert_eq!(value, &Value::Bool(false), "receipt privacy.{field}");
    }

    let attributes =
        fs::read_to_string(repo_root().join(".gitattributes")).expect("read attributes");
    assert!(
        attributes.contains(
            "/docs/simulations/profile-diff-held-out/pulse-42-public-result/** text eol=lf"
        )
    );
}
