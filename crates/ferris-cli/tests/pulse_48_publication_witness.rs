use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "5a8d92d211806d0f2940016af6c317878c5fdfc1";
const CUTOFF: &str = "70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d";
const AUTHORITY_DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-48-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e";
const WITNESS_RAW: &str = "sha256:65183b80fba13f27a6680e2d0f99f0410e40c659446e39716856cb8aed63c6f1";
const WITNESS_PAYLOAD: &str =
    "sha256:5c547fb2c482f1879bd18bc17d8e574dd7e2cc676f3e51e9d5d7ea8f1dfca35c";
const RECEIPT_RAW: &str = "sha256:e2f7e44e89731e4ac2bccae1c2f9312832cee33368aa368696ff218a0e6e9c01";
const RECEIPT_PAYLOAD: &str =
    "sha256:07607f76c9cc548655ba298c3d9f9f2e62efa643f009de48f3391816029fe265";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read public artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse public JSON");
    (bytes, value)
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("JSON object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect::<BTreeSet<_>>());
}

fn declaration_identity(value: &Value) -> String {
    let mut payload = value.clone();
    payload["declaration_identity"] = Value::String(String::new());
    let mut bytes = format!("{AUTHORITY_DOMAIN}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).expect("serialize declaration"));
    sha256(&bytes)
}

fn authority_blob(path: &str) -> Vec<u8> {
    let output = Command::new("git")
        .current_dir(repo_root())
        .args(["show", &format!("{AUTHORITY_COMMIT}:{path}")])
        .output()
        .expect("read authority commit blob");
    assert!(
        output.status.success(),
        "missing authority blob {path}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn p43_public_catalog_rejection(gate_ids: &[Value]) -> Result<(), &'static str> {
    const PRIVACY_BEARING_PARTS: &[&str] = &[
        "candidate",
        "corpus",
        "credential",
        "home",
        "password",
        "private",
        "seed",
        "secret",
        "token",
        "user",
        "workspace",
    ];

    if !(1..=24).contains(&gate_ids.len()) {
        return Err("P43-CATALOG-CARDINALITY");
    }
    let mut seen = BTreeSet::new();
    for gate_id in gate_ids {
        let gate_id = gate_id.as_str().ok_or("P43-CATALOG-GATE-ID")?;
        let valid_identifier = gate_id.len() <= 48
            && gate_id
                .chars()
                .enumerate()
                .all(|(index, character)| match index {
                    0 => character.is_ascii_lowercase(),
                    _ => {
                        character.is_ascii_lowercase()
                            || character.is_ascii_digit()
                            || character == '-'
                    }
                });
        if !valid_identifier {
            return Err("P43-CATALOG-GATE-ID");
        }
        if gate_id
            .split('-')
            .any(|part| PRIVACY_BEARING_PARTS.contains(&part))
        {
            return Err("P43-PRIVACY-BEARING-IDENTIFIER");
        }
        if !seen.insert(gate_id) {
            return Err("P43-DUPLICATE-CATALOG-GATE");
        }
    }
    Ok(())
}

#[test]
fn pulse_48_witness_is_exact_public_failure_and_catalog_rejection() {
    let held_out = held_out_root();
    let witness_root = held_out.join("pulse-48-publication-witness");
    let authority_path = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-48-authority.json";
    let authority =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-48-authority.json")).1;

    assert_eq!(
        authority_blob(authority_path),
        read_lf(held_out.join("fixtures/process-exit-diagnostic-pulse-48-authority.json")),
        "the authority fixture remains the committed public declaration"
    );
    assert_eq!(authority["schema"], AUTHORITY_DOMAIN);
    assert_eq!(authority["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&authority), DECLARATION_IDENTITY);
    assert_eq!(authority["status"], "authorized-unexecuted");
    assert_eq!(authority["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        authority["publication_integrity"]["witness_callable"],
        "witness_pulse_43"
    );

    let gate_ids = authority["closed_execution_catalog"]["ordered_gate_ids"]
        .as_array()
        .expect("authority ordered catalog");
    assert_eq!(gate_ids.len(), 8);
    assert_eq!(gate_ids[6], "private-materialization");
    assert_eq!(
        p43_public_catalog_rejection(gate_ids),
        Err("P43-PRIVACY-BEARING-IDENTIFIER"),
        "the public Pulse 43 identifier policy rejects the committed catalog"
    );

    let names = fs::read_dir(&witness_root)
        .expect("read witness root")
        .map(|entry| {
            let entry = entry.expect("witness entry");
            let metadata = fs::symlink_metadata(entry.path()).expect("witness metadata");
            assert!(
                metadata.is_file() && !metadata.file_type().is_symlink(),
                "witness tree entries must be regular files"
            );
            entry.file_name().to_string_lossy().into_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        names,
        BTreeSet::from([
            "publication-witness.json".to_owned(),
            "release-receipt.json".to_owned(),
        ]),
        "the witness root must contain exactly two files"
    );

    let (witness_bytes, witness) = read_json(witness_root.join("publication-witness.json"));
    assert_eq!(sha256(&witness_bytes), WITNESS_RAW);
    assert_eq!(
        witness["schema"],
        "ferris.pulse-47-publication-outcome-witness-envelope/v1"
    );
    assert_eq!(witness["payload_sha256"], WITNESS_PAYLOAD);
    assert_eq!(
        sha256(&serde_json::to_vec(&witness["payload"]).expect("serialize witness payload")),
        WITNESS_PAYLOAD
    );

    let payload = &witness["payload"];
    assert_exact_keys(
        payload,
        &[
            "publication_outcome",
            "pulse_43",
            "release_limits",
            "schema",
        ],
    );
    assert_eq!(
        payload["schema"],
        "ferris.pulse-47-publication-outcome-witness/v1"
    );
    assert_eq!(
        payload["release_limits"],
        serde_json::json!({
            "diagnostic_authority": false,
            "private_data_access": false,
            "product_conclusion": null,
        })
    );

    let outcome = &payload["publication_outcome"];
    assert_exact_keys(outcome, &["failure_code", "kind", "publication"]);
    assert_eq!(outcome["kind"], "failed");
    assert_eq!(outcome["failure_code"], "P43-PRIVACY-BEARING-IDENTIFIER");
    assert_eq!(
        outcome["publication"],
        serde_json::json!({
            "final_files_present": false,
            "rename_attempts": 0,
            "retries": 0,
            "state": "absent",
            "sync": {
                "final_parent": {
                    "attempted": false,
                    "error_category": "not-attempted",
                    "mechanism": "not-attempted",
                    "status": "not-attempted",
                },
                "rollback_parent": {
                    "attempted": false,
                    "error_category": "not-attempted",
                    "mechanism": "not-attempted",
                    "status": "not-attempted",
                },
                "stage": {
                    "attempted": false,
                    "error_category": "not-attempted",
                    "mechanism": "not-attempted",
                    "status": "not-attempted",
                },
            },
        })
    );
    assert!(
        outcome.get("catalog").is_none()
            && outcome.get("events").is_none()
            && outcome.get("ordered_execution").is_none()
            && outcome.get("search").is_none(),
        "the witness must omit private gate and search detail"
    );

    let p43_release = held_out.join("pulse-43-ordered-result-publisher-release");
    let (manifest_bytes, manifest) = read_json(p43_release.join("public-manifest.json"));
    let (qualification_bytes, qualification) =
        read_json(p43_release.join("qualification-receipt.json"));
    let (seal_bytes, seal) = read_json(p43_release.join("release-seal.json"));
    let source_bytes = read_lf(p43_release.join("ordered_result_publisher.py"));
    let expected_identities = serde_json::json!({
        "manifest_aggregate": manifest["aggregate"],
        "manifest_raw_sha256": sha256(&manifest_bytes),
        "qualification_receipt_payload_sha256": qualification["payload_sha256"],
        "qualification_receipt_raw_sha256": sha256(&qualification_bytes),
        "release_seal_payload_sha256": seal["payload_sha256"],
        "release_seal_raw_sha256": sha256(&seal_bytes),
        "source_sha256": sha256(&source_bytes),
    });
    let p43_custody = &authority["public_release_custody"]["pulse_43_ordered_result_publisher"];
    assert_eq!(
        p43_custody["manifest"]["aggregate"],
        expected_identities["manifest_aggregate"]
    );
    assert_eq!(
        p43_custody["manifest"]["raw_sha256"],
        expected_identities["manifest_raw_sha256"]
    );
    assert_eq!(
        p43_custody["qualification_receipt"]["payload_sha256"],
        expected_identities["qualification_receipt_payload_sha256"]
    );
    assert_eq!(
        p43_custody["qualification_receipt"]["raw_sha256"],
        expected_identities["qualification_receipt_raw_sha256"]
    );
    assert_eq!(
        p43_custody["release_seal"]["payload_sha256"],
        expected_identities["release_seal_payload_sha256"]
    );
    assert_eq!(
        p43_custody["release_seal"]["raw_sha256"],
        expected_identities["release_seal_raw_sha256"]
    );
    assert_eq!(
        p43_custody["raw_sha256_by_path"]["ordered_result_publisher.py"],
        expected_identities["source_sha256"]
    );
    assert_exact_keys(
        &payload["pulse_43"],
        &["identities", "invocation_count", "retries"],
    );
    assert_eq!(
        payload["pulse_43"]["identities"], expected_identities,
        "the witness must bind exact public Pulse 43 identities"
    );
    assert_eq!(payload["pulse_43"]["invocation_count"], 1);
    assert_eq!(payload["pulse_43"]["retries"], 0);

    let (receipt_bytes, receipt) = read_json(witness_root.join("release-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-47-publication-outcome-witness-receipt-envelope/v1"
    );
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(
        sha256(&serde_json::to_vec(&receipt["payload"]).expect("serialize receipt payload")),
        RECEIPT_PAYLOAD
    );
    assert_exact_keys(
        &receipt["payload"],
        &[
            "publication",
            "schema",
            "witness_payload_sha256",
            "witness_raw_sha256",
        ],
    );
    assert_eq!(
        receipt["payload"]["schema"],
        "ferris.pulse-47-publication-outcome-witness-receipt/v1"
    );
    assert_eq!(receipt["payload"]["witness_raw_sha256"], WITNESS_RAW);
    assert_eq!(
        receipt["payload"]["witness_payload_sha256"],
        WITNESS_PAYLOAD
    );
    assert_eq!(
        receipt["payload"]["publication"],
        serde_json::json!({
            "file_count": 2,
            "file_fsync": "2/2-os.fsync-before-close",
            "rename_attempts": 1,
            "retries": 0,
            "transaction": "absent-staged-verified-one-rename-final-verified",
        })
    );
}
