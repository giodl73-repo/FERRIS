use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MANIFEST_RAW: &str =
    "sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b";
const MANIFEST_AGGREGATE: &str =
    "sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7";
const QUALIFICATION_RAW: &str =
    "sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037";
const QUALIFICATION_PAYLOAD: &str =
    "sha256:7f1154ca94009cef966ab2f43ba74a9f017989ed5dbbdbfd8c3ce8fe64fe5cee";
const ROOT_CAUSE_RAW: &str =
    "sha256:02f3a34195858b1f82acd4b9c2ea9abc42413306e40caea3b9594ed0492b6ffe";
const ROOT_CAUSE_PAYLOAD: &str =
    "sha256:26d1a9a9051f5c4656da62f3743df19c371297634dbfdaf898ae76ed37b623ce";
const SEAL_RAW: &str = "sha256:51edf2f2df9210291705332fa8a4c3b55cb2a19a1aff22ecd882434a5ebefef2";
const SEAL_PAYLOAD: &str =
    "sha256:5b5e4383ffe5274f36f355069a5339c1684674aea342229f54f63ef247d21e52";
const SCHEMA_RAW: &str = "sha256:d85cea956a2cf82d0bf360cbccda2d19c25705c3c17f8d2a255a8dc11852825b";
const PULSE_34_RECEIPT: &str =
    "sha256:dca0ad1579257a6f265ada501533a4034070963267ef7c25478bf38267ee1588";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn release_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn read_json(path: &Path) -> (Vec<u8>, Value) {
    let bytes = fs::read(path).expect("read JSON");
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
    (bytes, value)
}

fn envelope(relative: &str, raw: &str, payload: &str) -> Value {
    let (bytes, value) = read_json(&release_root().join(relative));
    assert_eq!(sha256(&bytes), raw, "{relative} raw digest");
    assert_eq!(value["payload_sha256"], payload, "{relative} payload field");
    assert_eq!(
        canonical_payload_sha256(&value["payload"]),
        payload,
        "{relative} canonical payload"
    );
    assert_eq!(value["receipt_id"], payload, "{relative} receipt identity");
    value
}

fn collect_release_files(directory: &Path, root: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let path = entry.expect("release entry").path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if path.is_dir() {
            if name != "__pycache__" && name != ".test-runs" && name != ".qualification-work" {
                collect_release_files(&path, root, files);
            }
        } else if name != "public-manifest.json" && name != "release-seal.json" {
            files.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn assert_bound_artifact(binding: &Value) {
    let relative = binding["path"].as_str().expect("binding path");
    let bytes = fs::read(repo_root().join(relative)).expect("read public binding");
    assert_eq!(
        bytes.len() as u64,
        binding["size"].as_u64().expect("binding size")
    );
    assert_eq!(
        sha256(&bytes),
        binding["sha256"],
        "{relative} binding digest"
    );
}

#[test]
fn pulse_35_release_has_exact_semantic_contract_and_seal() {
    let root = release_root();
    let (manifest_bytes, manifest) = read_json(&root.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-35-public-corpus-materializer-manifest/v1"
    );
    assert_eq!(
        manifest["aggregate_algorithm"],
        "sha256-length-path-filedigest-v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(manifest["file_count"], 8);
    assert_eq!(manifest["total_bytes"], 405_414);

    let mut aggregate = Sha256::new();
    let mut listed = BTreeSet::new();
    let mut total_bytes = 0_u64;
    for entry in manifest["files"].as_array().expect("manifest files") {
        let relative = entry["path"].as_str().expect("manifest path");
        assert!(
            listed.insert(relative.to_owned()),
            "duplicate manifest path"
        );
        let bytes = fs::read(root.join(relative)).expect("read manifest file");
        assert_eq!(
            bytes.len() as u64,
            entry["size"].as_u64().expect("manifest size")
        );
        assert_eq!(sha256(&bytes), entry["sha256"], "{relative} digest");
        total_bytes += bytes.len() as u64;
        aggregate.update(bytes.len().to_string().as_bytes());
        aggregate.update(b"\0");
        aggregate.update(relative.as_bytes());
        aggregate.update(b"\0");
        aggregate.update(
            entry["sha256"]
                .as_str()
                .expect("digest")
                .trim_start_matches("sha256:")
                .as_bytes(),
        );
        aggregate.update(b"\n");
    }
    assert_eq!(total_bytes, 405_414);
    assert_eq!(
        format!("sha256:{:x}", aggregate.finalize()),
        MANIFEST_AGGREGATE
    );
    let mut actual = BTreeSet::new();
    collect_release_files(&root, &root, &mut actual);
    assert_eq!(actual, listed);

    let bindings = &manifest["public_contract_bindings"];
    let pulse_31 = &bindings["pulse_31_profile_evidence"];
    assert_eq!(pulse_31["artifact_count"], 9);
    assert_eq!(
        pulse_31["accepted_profile_schema"],
        "ferris.profile-evidence/v0"
    );
    assert_eq!(pulse_31["max_complete_file_bytes"], 1_048_576);
    assert_eq!(pulse_31["strict_duplicate_rejection"], true);
    assert_eq!(pulse_31["recursive_visible_ascii_member_names"], true);
    assert_eq!(pulse_31["closed_root_and_sections"], true);
    assert_bound_artifact(&pulse_31["contract"]);
    assert_bound_artifact(&pulse_31["schema"]);
    for fixture in pulse_31["positive_fixtures"]
        .as_array()
        .expect("positive fixtures")
    {
        assert_bound_artifact(fixture);
    }
    assert_bound_artifact(&pulse_31["mutation_controls"]);

    let pulse_34 = &bindings["pulse_34_public_authority"];
    assert_bound_artifact(&pulse_34["declaration"]);
    assert_eq!(pulse_34["coverage_domain_count"], 18);
    assert_eq!(pulse_34["coverage_interaction_count"], 8);
    assert_eq!(pulse_34["logical_case_max"], 512);
    assert_bound_artifact(&pulse_34["result"]);
    assert_eq!(pulse_34["result"]["receipt_id"], PULSE_34_RECEIPT);
    assert_eq!(pulse_34["result"]["disposition"], "invalid");
    assert_eq!(pulse_34["result"]["stage"], "generation-materialization");
    assert_eq!(pulse_34["result"]["further_launches_prohibited"], true);
    assert_bound_artifact(&bindings["pulse_35_machine_schema"]);
    assert_eq!(bindings["pulse_35_machine_schema"]["sha256"], SCHEMA_RAW);

    let (_, schema) = read_json(&repo_root().join(
        "docs/simulations/profile-diff-held-out/schemas/ferris.pulse-35-corpus-materializer.v1.schema.json",
    ));
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        schema["$defs"]["caseManifest"]["additionalProperties"],
        false
    );
    assert_eq!(
        schema["$defs"]["coverageManifest"]["additionalProperties"],
        false
    );
    assert_eq!(
        schema["$defs"]["caseManifest"]["properties"]["case_count"]["const"],
        70
    );
    assert_eq!(
        schema["$defs"]["coverageManifest"]["properties"]["case_count"]["const"],
        70
    );
    assert_eq!(
        schema["$defs"]["case"]["properties"]["ordinal"]["maximum"],
        70
    );
    assert_eq!(
        schema["$defs"]["interactionName"]["enum"]
            .as_array()
            .expect("interaction names")
            .len(),
        8
    );

    let qualification = envelope(
        "qualification-receipt.json",
        QUALIFICATION_RAW,
        QUALIFICATION_PAYLOAD,
    );
    let qualification = &qualification["payload"];
    assert_eq!(qualification["outcome"], "pass");
    assert_eq!(qualification["cycles_required"], 20);
    assert_eq!(qualification["cycles_run"], 20);
    assert_eq!(qualification["cycles_passed"], 20);
    assert_eq!(qualification["case_count_per_cycle"], 70);
    assert_eq!(qualification["coverage_domains_closed_per_cycle"], "18/18");
    assert_eq!(
        qualification["coverage_interactions_closed_per_cycle"],
        "8/8"
    );
    assert_eq!(qualification["fresh_process_reloads"], 20);
    assert_eq!(qualification["deterministic_same_seed_checks"], 20);
    assert_eq!(qualification["different_seed_divergence_checks"], 20);
    assert_eq!(qualification["seed_length_rejections"], 3);
    assert_eq!(qualification["semantic_fake_coverage_rejections"], 20);
    assert_eq!(qualification["logical_retries"], 0);
    assert_eq!(
        qualification["seed_material_requirement"],
        "exactly-32-byte-csprng"
    );

    let root_cause = envelope("root-cause-report.json", ROOT_CAUSE_RAW, ROOT_CAUSE_PAYLOAD);
    assert_eq!(root_cause["payload"]["pulse_34_immutable"], true);
    assert_eq!(root_cause["payload"]["pulse_34_disposition"], "invalid");

    let seal = envelope("release-seal.json", SEAL_RAW, SEAL_PAYLOAD);
    let seal = &seal["payload"];
    assert_eq!(seal["manifest"]["sha256"], MANIFEST_RAW);
    assert_eq!(seal["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(seal["manifest"]["file_count"], 8);
    assert_eq!(seal["manifest"]["total_bytes"], 405_414);
    assert_eq!(seal["qualification_receipt"]["sha256"], QUALIFICATION_RAW);
    assert_eq!(
        seal["qualification_receipt"]["payload_sha256"],
        QUALIFICATION_PAYLOAD
    );
    assert_eq!(seal["root_cause_report"]["sha256"], ROOT_CAUSE_RAW);
    assert_eq!(
        seal["root_cause_report"]["payload_sha256"],
        ROOT_CAUSE_PAYLOAD
    );
    assert_eq!(seal["public_contracts"]["complete_case_count"], 70);
    assert_eq!(
        seal["public_contracts"]["exact_interaction_tuple_counts"],
        serde_json::json!([20, 12, 54, 6, 33, 20, 6, 4])
    );
    assert_eq!(seal["release_limits"]["logical_retries"], 0);
    assert_eq!(
        seal["release_limits"]["seed_material_requirement"],
        "exactly-32-byte-csprng"
    );
    assert_eq!(seal["release_limits"]["diagnostic_execution"], false);
    assert_eq!(seal["release_limits"]["product_files_modified"], false);
    assert_eq!(seal["release_limits"]["pulse_34_reopened"], false);
    assert_eq!(
        seal["release_limits"]["verification_requires_private_seed"],
        true
    );
}

#[test]
fn pulse_35_runs_only_independent_public_rule_validation() {
    let root = release_root();
    let materializer =
        fs::read_to_string(root.join("corpus_materializer.py")).expect("materializer source");
    let verifier =
        fs::read_to_string(root.join("verify_materialization.py")).expect("verifier source");
    for required in [
        "REQUIRED_CASE_COUNT = 70",
        "exactly 32 bytes of CSPRNG material",
        "hmac-sha256-seed-key-domain-purpose-counter-v1",
        "ferris-p35-seed-commitment-v1",
        "PublicationIndeterminateError",
        "published output was rolled back",
        "derive_coverage_catalog",
        "input-position-by-path-state-by-path-form",
        "unc-authority-preserve",
        "pair_change_count",
    ] {
        assert!(
            materializer.contains(required),
            "missing materializer control {required}"
        );
    }
    assert!(
        !materializer.contains("subprocess"),
        "materializer must not execute a process"
    );
    assert!(
        !materializer.contains("time.sleep"),
        "materializer must not retry publication"
    );
    assert!(
        !verifier.contains("from corpus_materializer"),
        "verifier must implement public rules independently"
    );
    assert!(verifier.contains("coverage catalog is not independently derived"));
    let attributes = fs::read_to_string(repo_root().join(".gitattributes")).expect("attributes");
    assert!(attributes.contains("/docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/** text eol=lf"));
    let status = Command::new("python")
        .args(["-m", "unittest", "discover", "-s", "tests", "-v"])
        .current_dir(&root)
        .status()
        .expect("run public Python materializer tests");
    fs::remove_dir_all(root.join("__pycache__")).ok();
    fs::remove_dir_all(root.join("tests").join("__pycache__")).ok();
    assert!(status.success(), "public materializer tests failed");
}
