use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const REPORT_SHA256: &str =
    "sha256:1365dfd33f834b24f9e20f73afce313978b26d27e3c076ae90a8671eb841e723";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/simulations/profile-diff-held-out/pulse-23-collector-qualification")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

#[test]
fn collector_durability_repair_is_synthetically_qualified_and_public_safe() {
    let report_bytes =
        fs::read(root().join("collector-qualification-report.json")).expect("read report");
    let report: Value = serde_json::from_slice(&report_bytes).expect("parse report");
    let payload = &report["payload"];

    assert_eq!(sha256(&report_bytes), REPORT_SHA256);
    assert_eq!(report["schema"], "collector-sealed-json-v1");
    assert_eq!(payload["schema"], "collector-durability-public-report-v1");
    assert_eq!(payload["outcome"], "pass");
    assert_eq!(
        payload["scope"],
        "Synthetic collector infrastructure qualification only; no diagnostic workload was launched or replayed."
    );
    assert_eq!(payload["unit_tests"]["total"]["passed"], 20);
    assert_eq!(payload["unit_tests"]["total"]["failed"], 0);
    assert_eq!(payload["synthetic_qualification"]["pair_count"], 20);
    assert_eq!(payload["synthetic_qualification"]["pair_pass_count"], 20);
    assert_eq!(payload["synthetic_qualification"]["pair_fail_count"], 0);
    assert_eq!(
        payload["synthetic_qualification"]["command_execution_count"],
        40
    );
    assert_eq!(payload["synthetic_qualification"]["command_fail_count"], 0);
    assert_eq!(
        payload["synthetic_qualification"]["fresh_process_reload_pass_count"],
        2
    );
    assert_eq!(
        payload["synthetic_qualification"]["fresh_process_reload_fail_count"],
        0
    );
    assert_eq!(payload["synthetic_qualification"]["residue_count"], 0);
    assert_eq!(
        payload["closed_workspace_preservation"]["byte_for_byte"],
        true
    );
    assert_eq!(
        payload["closed_workspace_preservation"]["aggregate_unchanged"],
        true
    );

    let public_text = String::from_utf8(report_bytes).expect("report UTF-8");
    for forbidden in [
        "C:\\",
        ".p22-custody-",
        "\"seed\":\"",
        "\"candidate_bytes\"",
        "\"stdout\":\"",
        "\"stderr\":\"",
    ] {
        assert!(
            !public_text.contains(forbidden),
            "qualification disclosed forbidden material: {forbidden}"
        );
    }
}
