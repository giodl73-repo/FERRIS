use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "ferris.process-exit-diagnostic-replication/v1";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/simulations/profile-diff-held-out")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
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

fn strings(value: &Value) -> Option<Vec<&str>> {
    value
        .as_array()?
        .iter()
        .map(Value::as_str)
        .collect::<Option<Vec<_>>>()
}

fn integers(value: &Value) -> Option<Vec<u64>> {
    value
        .as_array()?
        .iter()
        .map(Value::as_u64)
        .collect::<Option<Vec<_>>>()
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

fn validate(value: &Value) -> bool {
    const ROOT: [&str; 18] = [
        "schema",
        "declaration_identity",
        "program_id",
        "recorded_on",
        "status",
        "authority",
        "disclosure",
        "freshness",
        "platforms",
        "search_bounds",
        "seed_control",
        "coverage",
        "oracle",
        "minimization",
        "publication",
        "result",
        "custody_handoff",
        "limitations",
    ];
    if !exact_keys(value, &ROOT)
        || value["schema"] != DOMAIN
        || value["program_id"] != "FERRIS-P22-PROCESS-EXIT-DIAGNOSTIC-REPLICATION"
        || value["recorded_on"] != "2026-08-13"
        || value["status"] != "authorized-unexecuted"
        || !digest(&value["declaration_identity"])
        || declaration_identity(value) != value["declaration_identity"]
    {
        return false;
    }

    if value["authority"]
        != json!({
            "diagnostic_only": true,
            "score": false,
            "certification": false,
            "product_fix": false,
            "production_code_change": false,
            "hidden_material_access": false,
            "old_fixture_access": false,
            "pulse_17_result_immutable": true,
            "pulse_17_retry": false,
            "pulse_17_rescore": false,
            "pulse_17_reuse": false,
            "pulse_17_inference": false,
            "platform_001_status_change": false
        })
        || value["disclosure"]
            != json!({
                "precommitted_tier": "sanitized-reproducer",
                "precommitted_before_generation": true,
                "tier_change_after_generation": false,
                "release_receipt_schema": "ferris.post-score-diagnostic-release/v1"
            })
        || value["freshness"]
            != json!({
                "independent_custodian_required": true,
                "implementation_author_constructed_cases": false,
                "implementation_author_selected_cases": false,
                "pulse_19_case_bytes_reused": false,
                "cases_constructed": false,
                "cases_selected": false,
                "case_bytes_present": false
            })
    {
        return false;
    }

    if value["platforms"]
        != json!([
            {
                "id": "windows-x86_64",
                "environment": "Windows x86-64",
                "launch": "direct-immutable-ferris-binary"
            },
            {
                "id": "ubuntu-24.04-wsl2-x86_64",
                "environment": "Ubuntu 24.04 WSL2 x86-64",
                "launch": "direct-immutable-ferris-binary"
            }
        ])
        || value["search_bounds"]
            != json!({
                "logical_case_max": 512,
                "cases_per_platform_max": 512,
                "platform_count": 2,
                "search_process_max": 1024,
                "search_executions": 1,
                "candidate_attempts_per_platform": 1,
                "candidate_retries": 0,
                "cross_platform_pair_atomic": true,
                "stop_after_first_target_mismatch_pair": true,
                "process_timeout_millis": 60000,
                "stdout_max_bytes": 8388608,
                "stderr_max_bytes": 8388608,
                "network_allowed": false
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
                "case_manifest_digest": null
            })
    {
        return false;
    }

    let coverage = &value["coverage"];
    if !exact_keys(
        coverage,
        &[
            "result_classes",
            "human_parity_classes",
            "metadata_sites",
            "metadata_byte_boundaries",
            "metadata_character_kinds",
            "json_value_kinds",
            "number_representations",
            "pointer_key_kinds",
            "duplicate_depths",
            "member_orderings",
            "input_role_orderings",
            "failure_positions",
            "path_states",
            "path_forms",
            "lexical_normalization",
            "input_byte_boundaries",
            "change_count_boundaries",
            "interaction_requirements",
        ],
    ) || coverage["result_classes"]
        != json!([
            {
                "class": "success",
                "exit": 0,
                "json_route": "stdout-only",
                "record": "non-null",
                "diagnostics": "empty"
            },
            {
                "class": "difference",
                "exit": 1,
                "json_route": "stdout-only",
                "record": "non-null",
                "diagnostics": "empty"
            },
            {
                "class": "invalid",
                "exit": 2,
                "json_route": "stderr-only",
                "record": "null",
                "diagnostics": "exactly-one-matching-class"
            },
            {
                "class": "unsupported",
                "exit": 4,
                "json_route": "stderr-only",
                "record": "null",
                "diagnostics": "exactly-one-matching-class"
            },
            {
                "class": "incomplete",
                "exit": 5,
                "json_route": "stderr-only",
                "record": "null",
                "diagnostics": "exactly-one-matching-class"
            },
            {
                "class": "blocked",
                "exit": 7,
                "json_route": "stderr-only",
                "record": "null",
                "diagnostics": "exactly-one-matching-class"
            }
        ])
        || strings(&coverage["human_parity_classes"]) != Some(vec!["success", "difference"])
        || strings(&coverage["metadata_sites"])
            != Some(vec!["profile_id", "revision", "consumer", "object-key"])
        || integers(&coverage["metadata_byte_boundaries"]) != Some(vec![0, 1, 255, 256, 257])
        || strings(&coverage["metadata_character_kinds"])
            != Some(vec!["visible-ascii", "ascii-control", "non-ascii"])
        || strings(&coverage["json_value_kinds"])
            != Some(vec![
                "null",
                "false",
                "true",
                "string",
                "number",
                "array-empty",
                "array-nonempty",
                "object-empty",
                "object-nonempty",
                "nested-array",
                "nested-object",
            ])
        || strings(&coverage["number_representations"])
            != Some(vec![
                "0",
                "-0",
                "1",
                "-1",
                "1.0",
                "1e0",
                "1E+0",
                "1e-0",
                "9007199254740991",
                "-9007199254740991",
            ])
        || strings(&coverage["pointer_key_kinds"])
            != Some(vec!["slash", "tilde", "slash-and-tilde", "nested"])
        || integers(&coverage["duplicate_depths"]) != Some(vec![0, 1, 2, 8, 32])
        || strings(&coverage["member_orderings"])
            != Some(vec![
                "before-reordered",
                "after-reordered",
                "both-reordered-equivalent",
            ])
        || strings(&coverage["input_role_orderings"])
            != Some(vec![
                "before-then-after-read",
                "difference-pair-original-roles",
                "difference-pair-swapped-roles",
            ])
        || strings(&coverage["failure_positions"])
            != Some(vec![
                "before-only",
                "after-only-after-valid-before",
                "both-before-precedence",
                "relocated-valid-before-same-after-failure",
            ])
        || strings(&coverage["path_states"]) != Some(vec!["missing", "non-file", "regular-file"])
        || strings(&coverage["path_forms"])
            != Some(vec![
                "relative-simple",
                "relative-dot",
                "relative-reducible-dotdot",
                "relative-unreducible-dotdot",
                "windows-drive-absolute",
                "windows-extended-absolute",
                "windows-unc",
                "unix-absolute",
                "mixed-separators",
            ])
        || strings(&coverage["lexical_normalization"])
            != Some(vec![
                "extended-prefix-strip",
                "backslash-to-slash",
                "empty-component-removal",
                "dot-component-removal",
                "reducible-dotdot-pop",
                "rooted-dotdot-discard",
                "relative-dotdot-preserve",
                "repeated-separator-collapse",
                "drive-case-preserve",
                "drive-rooted-versus-relative",
            ])
        || integers(&coverage["input_byte_boundaries"])
            != Some(vec![1_048_575, 1_048_576, 1_048_577])
        || integers(&coverage["change_count_boundaries"]) != Some(vec![9_999, 10_000, 10_001])
        || strings(&coverage["interaction_requirements"])
            != Some(vec![
                "metadata-site-by-visible-ascii-byte-boundary",
                "metadata-site-by-character-kind-for-nonempty-values",
                "input-position-by-path-state-by-path-form",
                "input-position-by-input-byte-boundary",
                "json-value-kind-by-member-ordering",
                "duplicate-depth-by-failure-position",
                "expected-result-class-by-json-route",
                "success-difference-by-json-human-pair",
            ])
    {
        return false;
    }

    if value["oracle"]
        != json!({
            "public_only": true,
            "frozen_before_generation": true,
            "independent_implementation": true,
            "target_category": "process-exit-agreement",
            "compared_fields": [
                "expected-result-class",
                "expected-exit",
                "emitted-result-class",
                "emitted-process-exit-code",
                "actual-os-exit",
                "diagnostic-result-class",
                "record-nullability",
                "stream-route"
            ],
            "target_predicates": [
                "expected_exit == public_map(expected_class)",
                "emitted_class == expected_class",
                "emitted_exit == public_map(emitted_class)",
                "emitted_exit == expected_exit",
                "actual_os_exit == emitted_exit",
                "actual_os_exit == expected_exit"
            ],
            "adjacent_mismatch_is_target": false,
            "target_output_used_to_infer_expected": false
        })
        || value["minimization"]
            != json!({
                "authorized_only_after_reproduction": true,
                "separate_phase": true,
                "first_reproducer_immutable": true,
                "transformation_max": 128,
                "launches_per_transformation_max": 2,
                "process_max": 256,
                "candidate_retries": 0,
                "lineage_required": true,
                "derived_public_candidates_may_run": true,
                "pulse_17_access": false,
                "size_order": [
                    "regular-file-count",
                    "total-input-bytes",
                    "total-json-nodes",
                    "total-argv-bytes",
                    "lexical-description-digest"
                ]
            })
        || value["publication"]
            != json!({
                "reproduced_requires_public_directory": true,
                "reproduced_requires_release_receipt": true,
                "zero_overlap_counts_required": true,
                "public_regular_file_max": 16,
                "public_total_bytes_max": 1048576,
                "commands_per_platform_max": 4,
                "command_timeout_millis_max": 60000,
                "stream_bytes_max": 1048576,
                "private_search_future_certification_eligible": false,
                "public_reproducer_future_certification_eligible": false,
                "no_reproduction_statement": "bounded no-reproduction; no fix authority",
                "fix_authority": false
            })
    {
        return false;
    }

    if value["result"]
        != json!({
            "disposition": "authorized-unexecuted",
            "search_started": false,
            "search_complete": false,
            "cases_generated": 0,
            "cases_executed_windows": 0,
            "cases_executed_ubuntu": 0,
            "search_processes": 0,
            "retries": 0,
            "target_category_reproduced": null,
            "first_mismatch_case_id": null,
            "coverage_report_digest": null,
            "process_aggregate_digest": null,
            "release_receipt_digest": null
        })
        || value["custody_handoff"]
            != json!({
                "ready": true,
                "custodian_selected": false,
                "next_owner": "independent-validation-custodian",
                "implementation_author_case_access": false,
                "required_freezes": [
                    "custody-identity-and-workspace",
                    "immutable-ferris-cutoff-and-executable-digest",
                    "independent-classifier-source-and-digest",
                    "deterministic-generator-source-and-digest",
                    "private-seed-commitment",
                    "case-and-coverage-manifests",
                    "one-execution-launch-authorization"
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
                    .is_some_and(|text| text.contains("not a score or certification"))
            })
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("No actual seed, case"))
            })
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("Pulse 17 remains immutable"))
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
fn process_exit_diagnostic_replication_is_frozen_unexecuted_and_mutation_resistant() {
    let schema = read_json(
        root()
            .join("schemas")
            .join("ferris.process-exit-diagnostic-replication.v1.schema.json"),
    );
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["properties"]["schema"]["const"], DOMAIN);
    assert_closed_object_schemas(&schema);

    let declaration = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-replication.json"),
    );
    assert!(validate(&declaration));

    let contract = fs::read_to_string(root().join("PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md"))
        .expect("read replication contract");
    for required in [
        "`sanitized-reproducer`",
        "512 unique candidates",
        "1,024",
        "1048575",
        "1048576",
        "1048577",
        "9999",
        "10000",
        "10001",
        "128",
        "bounded no-reproduction; no fix authority",
    ] {
        assert!(
            contract.contains(required),
            "missing contract term {required}"
        );
    }

    let mutations = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-replication-mutations.json"),
    );
    assert_eq!(
        mutations["schema"],
        "ferris.process-exit-diagnostic-replication-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 35);
    for mutation in mutations {
        assert!(exact_keys(
            mutation,
            &["id", "operation", "pointer", "value", "recompute_identity",],
        ));
        let mut candidate = declaration.clone();
        apply_mutation(&mut candidate, mutation);
        if mutation["recompute_identity"] == true {
            candidate["declaration_identity"] = Value::String(declaration_identity(&candidate));
        }
        assert!(
            !validate(&candidate),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
    }
}
