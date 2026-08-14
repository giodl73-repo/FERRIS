use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "ferris.process-exit-diagnostic-replacement/v1";
const PULSE_22_CONTRACT_DIGEST: &str =
    "sha256:6f4447444c49814b61a3c6234af6f6690e40d618aa3f6acfa78e00fd8c0a2ec8";
const PULSE_22_RESULT_DIGEST: &str =
    "sha256:3dcd2def79cced56fb266e16e5d6c4bc12e9f7db688bf6f34cb0eed47743d2e7";
const COLLECTOR_REPORT_DIGEST: &str =
    "sha256:1365dfd33f834b24f9e20f73afce313978b26d27e3c076ae90a8671eb841e723";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/simulations/profile-diff-held-out")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}

fn read_lf_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = fs::read(path).expect("read LF JSON");
    assert!(!bytes.contains(&b'\r'), "JSON must use LF framing");
    assert!(bytes.ends_with(b"\n"), "JSON must end with LF");
    let value = serde_json::from_slice(&bytes).expect("parse LF JSON");
    (bytes, value)
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn declaration_identity(value: &Value) -> String {
    let mut payload = value.clone();
    payload["declaration_identity"] = Value::String(String::new());
    let mut bytes = format!("{DOMAIN}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).expect("serialize declaration"));
    sha256(&bytes)
}

fn exact_keys(value: &Value, expected: &[&str]) -> bool {
    value.as_object().is_some_and(|object| {
        object.keys().map(String::as_str).collect::<BTreeSet<_>>()
            == expected.iter().copied().collect::<BTreeSet<_>>()
    })
}

fn digest(value: &Value) -> bool {
    value.as_str().is_some_and(|text| {
        text.len() == 71
            && text.starts_with("sha256:")
            && text[7..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    })
}

fn schema_const_matches(value: &Value, schema: &Value, field: &str) -> bool {
    value[field] == schema["properties"][field]["const"]
}

fn validate(value: &Value, schema: &Value, pulse_22: &Value) -> bool {
    const ROOT: [&str; 23] = [
        "schema",
        "declaration_identity",
        "program_id",
        "recorded_on",
        "status",
        "predecessor",
        "authority",
        "disclosure",
        "immutable_ferris",
        "collector_qualification",
        "preflight",
        "freshness",
        "platforms",
        "search_bounds",
        "seed_control",
        "coverage",
        "oracle",
        "collection",
        "minimization",
        "publication",
        "result",
        "custody_handoff",
        "limitations",
    ];
    if !exact_keys(value, &ROOT)
        || value["schema"] != DOMAIN
        || value["program_id"] != "FERRIS-P24-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-REPLACEMENT"
        || value["recorded_on"] != "2026-08-14"
        || value["status"] != "authorized-unexecuted"
        || !digest(&value["declaration_identity"])
        || declaration_identity(value) != value["declaration_identity"]
    {
        return false;
    }

    for field in [
        "predecessor",
        "authority",
        "disclosure",
        "platforms",
        "search_bounds",
        "coverage",
        "oracle",
        "collection",
        "minimization",
        "publication",
    ] {
        if !schema_const_matches(value, schema, field) {
            return false;
        }
    }

    if value["immutable_ferris"]
        != json!({
            "cutoff": "cef0daabc349ac2333869959f21b9a3106e10484",
            "direct_launch_required": true,
            "independent_commit_verification_required": true,
            "executable_digest_required_before_preflight": true,
            "executable_digest": null
        })
        || value["collector_qualification"]
            != json!({
                "reuse_policy": "exact-pinned-source-digest-only",
                "public_report_digest": COLLECTOR_REPORT_DIGEST,
                "source_digest": "sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558",
                "test_digest": "sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62",
                "windows_environment_digest": "sha256:92597acdd22522ff1c3e06d508bd0797910f98282a76dfead1f640eb5e4e097a",
                "ubuntu_environment_digest": "sha256:344fbddf4b2efc1a7563057edf0903a605aa613eb46a12ac2db722e601838b72",
                "independent_verification_required": true,
                "verified": false,
                "repair_in_this_pulse": false,
                "repair_requires_later_authorized_pulse": true
            })
        || value["preflight"]
            != json!({
                "required_before_candidate_launch": true,
                "harmless_synthetic_only": true,
                "atomic_cross_platform_pairs": true,
                "pair_minimum": 2,
                "counts_as_candidate": false,
                "counts_toward_search_process_bound": false,
                "durable_process_records_required": true,
                "pair_seal_after_both_records": true,
                "fresh_process_reload_required": true,
                "zero_residue_required": true,
                "failure_disposition": "invalid-before-candidates",
                "failure_prohibits_candidate_launch": true,
                "failure_repair_requires_later_authorized_pulse": true,
                "started": false,
                "completed_pairs": 0,
                "processes": 0,
                "failures": 0
            })
        || value["freshness"]
            != json!({
                "independent_custodian_required": true,
                "new_custody_identity_required": true,
                "new_custody_workspace_required": true,
                "new_private_seed_required": true,
                "new_seed_commitment_required": true,
                "new_classifier_required": true,
                "new_generator_required": true,
                "new_case_manifest_required": true,
                "new_coverage_manifest_required": true,
                "new_fresh_corpus_required": true,
                "implementation_author_constructed_cases": false,
                "implementation_author_selected_cases": false,
                "cases_constructed": false,
                "cases_selected": false,
                "case_bytes_present": false
            })
        || value["seed_control"]
            != json!({
                "visibility": "private-custody-only",
                "derivation": "sha256-seed-domain-counter-v1",
                "commitment_algorithm": "sha256",
                "committed_before_generation": true,
                "seed_present": false,
                "commitment_digest": null,
                "generator_digest": null,
                "classifier_digest": null,
                "case_manifest_digest": null,
                "coverage_manifest_digest": null
            })
    {
        return false;
    }

    if value["coverage"] != pulse_22["coverage"]
        || value["oracle"] != pulse_22["oracle"]
        || value["minimization"] != pulse_22["minimization"]
    {
        return false;
    }
    let Some(pulse_22_publication) = pulse_22["publication"].as_object() else {
        return false;
    };
    if pulse_22_publication
        .iter()
        .any(|(field, expected)| value["publication"][field] != *expected)
    {
        return false;
    }

    if value["result"]
        != json!({
            "disposition": "authorized-unexecuted",
            "collector_digests_verified": false,
            "preflight_started": false,
            "preflight_completed_pairs": 0,
            "preflight_processes": 0,
            "preflight_failures": 0,
            "search_started": false,
            "search_complete": false,
            "cases_generated": 0,
            "cases_executed_windows": 0,
            "cases_executed_ubuntu": 0,
            "completed_cross_platform_pairs": 0,
            "search_processes": 0,
            "retries": 0,
            "target_category_reproduced": null,
            "category_conclusion": null,
            "first_mismatch_case_id": null,
            "blocker_code": null,
            "blocker_stage": null,
            "further_launches_prohibited": false,
            "coverage_report_digest": null,
            "process_aggregate_digest": null,
            "release_receipt_digest": null
        })
        || value["custody_handoff"]
            != json!({
                "ready": true,
                "custodian_selected": false,
                "next_owner": "new-independent-validation-custodian",
                "implementation_author_case_access": false,
                "required_freezes": [
                    "new-custody-identity-and-workspace",
                    "immutable-ferris-cutoff-and-executable-digest",
                    "collector-report-source-test-and-environment-digest-verification",
                    "two-or-more-harmless-synthetic-atomic-cross-platform-preflight-pairs",
                    "new-independent-classifier-source-and-digest",
                    "new-deterministic-generator-source-and-digest",
                    "new-private-seed-commitment",
                    "new-case-and-coverage-manifests",
                    "one-execution-candidate-launch-authorization"
                ]
            })
    {
        return false;
    }

    value["limitations"].as_array().is_some_and(|items| {
        items.len() == 6
            && items
                .iter()
                .all(|item| item.as_str().is_some_and(|text| !text.is_empty()))
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("not a Pulse 22 retry"))
            })
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("No actual custody identity"))
            })
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("Pulse 22 remains permanently invalid"))
            })
    })
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            let (parent, key) = pointer.rsplit_once('/').expect("mutation pointer parent");
            let parent = if parent.is_empty() {
                value
            } else {
                value.pointer_mut(parent).expect("mutation parent")
            };
            if let Some(object) = parent.as_object_mut() {
                object.insert(key.to_owned(), mutation["value"].clone());
            } else {
                parent.as_array_mut().expect("mutation array")
                    [key.parse::<usize>().expect("array index")] = mutation["value"].clone();
            }
        }
        "remove" => {
            let (parent, key) = pointer.rsplit_once('/').expect("remove pointer parent");
            let parent = value.pointer_mut(parent).expect("remove parent");
            if let Some(object) = parent.as_object_mut() {
                object.remove(key);
            } else {
                parent
                    .as_array_mut()
                    .expect("remove array")
                    .remove(key.parse::<usize>().expect("array index"));
            }
        }
        operation => panic!("unsupported mutation {operation}"),
    }
}

fn assert_closed_object_schemas(value: &Value) {
    match value {
        Value::Object(object) => {
            if object.get("type") == Some(&Value::String("object".to_owned())) {
                assert_eq!(
                    object.get("additionalProperties"),
                    Some(&Value::Bool(false)),
                    "typed object schema must be closed"
                );
            }
            object.values().for_each(assert_closed_object_schemas);
        }
        Value::Array(items) => items.iter().for_each(assert_closed_object_schemas),
        _ => {}
    }
}

#[test]
fn pulse_24_replacement_authority_is_closed_unexecuted_and_mutation_resistant() {
    let schema_path = root()
        .join("schemas")
        .join("ferris.process-exit-diagnostic-replacement.v1.schema.json");
    let (_, schema) = read_lf_json(schema_path);
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["properties"]["schema"]["const"], DOMAIN);
    assert_closed_object_schemas(&schema);

    let fixture_path = root()
        .join("fixtures")
        .join("process-exit-diagnostic-replacement.json");
    let (_, declaration) = read_lf_json(fixture_path);
    let pulse_22 = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-replication.json"),
    );
    assert!(validate(&declaration, &schema, &pulse_22));
    assert_eq!(
        declaration["coverage"], pulse_22["coverage"],
        "mandatory public coverage must be unchanged"
    );
    assert_eq!(
        declaration["oracle"], pulse_22["oracle"],
        "public oracle and all target predicates must be unchanged"
    );

    let pulse_22_contract =
        fs::read(root().join("PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md")).expect("read Pulse 22");
    assert_eq!(sha256(&pulse_22_contract), PULSE_22_CONTRACT_DIGEST);
    let pulse_22_result = fs::read(
        root()
            .join("pulse-22-public-result")
            .join("PULSE-22-PUBLIC-RESULT.json"),
    )
    .expect("read Pulse 22 result");
    assert_eq!(sha256(&pulse_22_result), PULSE_22_RESULT_DIGEST);
    let pulse_22_result: Value =
        serde_json::from_slice(&pulse_22_result).expect("parse Pulse 22 result");
    assert_eq!(pulse_22_result["result"]["disposition"], "invalid");
    assert_eq!(pulse_22_result["result"]["cases_executed_windows"], 1);
    assert_eq!(pulse_22_result["result"]["cases_executed_ubuntu"], 0);
    assert_eq!(pulse_22_result["result"]["retries"], 0);
    assert_eq!(
        pulse_22_result["result"]["target_category_reproduced"],
        Value::Null
    );

    let collector_report_bytes = fs::read(
        root()
            .join("pulse-23-collector-qualification")
            .join("collector-qualification-report.json"),
    )
    .expect("read collector report");
    assert_eq!(sha256(&collector_report_bytes), COLLECTOR_REPORT_DIGEST);
    let collector_report: Value =
        serde_json::from_slice(&collector_report_bytes).expect("parse collector report");
    assert_eq!(
        collector_report["payload"]["source_digest"],
        declaration["collector_qualification"]["source_digest"]
    );
    assert_eq!(
        collector_report["payload"]["test_digest"],
        declaration["collector_qualification"]["test_digest"]
    );
    assert_eq!(
        collector_report["payload"]["platform_environment_digests"]["windows"],
        declaration["collector_qualification"]["windows_environment_digest"]
    );
    assert_eq!(
        collector_report["payload"]["platform_environment_digests"]["ubuntu"],
        declaration["collector_qualification"]["ubuntu_environment_digest"]
    );

    let contract = fs::read_to_string(root().join("PROCESS_EXIT_DIAGNOSTIC_REPLACEMENT.md"))
        .expect("read replacement contract");
    for required in [
        "not a retry, resume, reseed",
        "cef0daabc349ac2333869959f21b9a3106e10484",
        COLLECTOR_REPORT_DIGEST,
        "sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558",
        "sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62",
        "sha256:92597acdd22522ff1c3e06d508bd0797910f98282a76dfead1f640eb5e4e097a",
        "sha256:344fbddf4b2efc1a7563057edf0903a605aa613eb46a12ac2db722e601838b72",
        "at least two harmless synthetic atomic cross-platform pairs",
        "512",
        "1,024",
        "128",
        "256",
        "fresh read-only process",
        "bounded no-reproduction; no fix authority",
        "ferris.post-score-diagnostic-release/v1",
    ] {
        assert!(
            contract.contains(required),
            "missing replacement contract term {required}"
        );
    }

    let mutations_path = root()
        .join("fixtures")
        .join("process-exit-diagnostic-replacement-mutations.json");
    let (_, mutations) = read_lf_json(mutations_path);
    assert_eq!(
        mutations["schema"],
        "ferris.process-exit-diagnostic-replacement-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 82);
    let mut ids = BTreeSet::new();
    for mutation in mutations {
        assert!(exact_keys(
            mutation,
            &["id", "operation", "pointer", "value", "recompute_identity"]
        ));
        assert!(ids.insert(mutation["id"].as_str().expect("mutation id")));
        let mut candidate = declaration.clone();
        apply_mutation(&mut candidate, mutation);
        if mutation["recompute_identity"] == true {
            candidate["declaration_identity"] = Value::String(declaration_identity(&candidate));
        }
        assert!(
            !validate(&candidate, &schema, &pulse_22),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
    }
}

#[test]
fn pulse_24_invalid_result_stopped_before_candidates_and_is_sealed() {
    let result_root = root().join("pulse-24-public-result");
    let result_bytes =
        fs::read(result_root.join("PULSE-24-PUBLIC-RESULT.json")).expect("read result");
    let evidence_bytes =
        fs::read(result_root.join("PULSE-24-PUBLIC-EVIDENCE.json")).expect("read evidence");
    let result: Value = serde_json::from_slice(&result_bytes).expect("parse result");
    let evidence: Value = serde_json::from_slice(&evidence_bytes).expect("parse evidence");

    assert_eq!(result["schema"], DOMAIN);
    assert_eq!(result["status"], "executed");
    assert_eq!(result["result"]["disposition"], "invalid");
    assert_eq!(
        result["result"]["blocker_code"],
        "collector-source-copy-unavailable-without-prohibited-access"
    );
    assert_eq!(
        result["result"]["blocker_stage"],
        "qualification-before-preflight"
    );
    assert_eq!(result["result"]["preflight_processes"], 0);
    assert_eq!(result["result"]["cases_generated"], 0);
    assert_eq!(result["result"]["search_processes"], 0);
    assert_eq!(result["result"]["completed_cross_platform_pairs"], 0);
    assert_eq!(result["result"]["retries"], 0);
    assert_eq!(result["result"]["category_conclusion"], Value::Null);
    assert_eq!(result["result"]["target_category_reproduced"], Value::Null);

    assert_eq!(evidence["preflight"]["processes"], 0);
    assert_eq!(evidence["generated_and_executed"]["cases_generated"], 0);
    assert_eq!(evidence["generated_and_executed"]["search_processes"], 0);
    assert_eq!(
        evidence["disposition"]["blocker_code"],
        "collector-source-copy-unavailable-without-prohibited-access"
    );
    assert_eq!(evidence["disposition"]["category_conclusion"], Value::Null);
    assert_eq!(
        sha256(&result_bytes),
        "sha256:b845858d0ca8c7011443140d8cfdbebbbe925f1d17e26c414cad09e48df19db4"
    );
    assert_eq!(
        sha256(&evidence_bytes),
        "sha256:5535e0069f5de20026aaf57130e4293a2a928af576958bfd89b3bd4ab029bbea"
    );

    let public_text = format!(
        "{}\n{}",
        String::from_utf8(result_bytes).expect("result UTF-8"),
        String::from_utf8(evidence_bytes).expect("evidence UTF-8")
    );
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
            "public result disclosed forbidden material: {forbidden}"
        );
    }
}
