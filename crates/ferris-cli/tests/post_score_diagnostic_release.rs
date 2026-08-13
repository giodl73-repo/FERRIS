use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "ferris.post-score-diagnostic-release/v1";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/simulations/profile-diff-held-out")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn receipt_identity(value: &Value) -> String {
    let mut payload = value.clone();
    payload["receipt_identity"] = Value::String(String::new());
    let mut bytes = format!("{DOMAIN}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).expect("serialize receipt"));
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

fn validate(value: &Value) -> bool {
    const ROOT: [&str; 14] = [
        "schema",
        "receipt_identity",
        "release_id",
        "program_id",
        "precommitted_tier",
        "failure_category",
        "original_result",
        "custody",
        "reproducer",
        "environments",
        "equivalence",
        "bounds",
        "lifecycle",
        "limitations",
    ];
    if !exact_keys(value, &ROOT)
        || value["schema"] != DOMAIN
        || value["precommitted_tier"] != "sanitized-reproducer"
        || !digest(&value["receipt_identity"])
        || receipt_identity(value) != value["receipt_identity"]
    {
        return false;
    }

    let original = &value["original_result"];
    if !exact_keys(
        original,
        &[
            "uri",
            "digest",
            "immutable",
            "score_attempt",
            "scorer_attempt",
        ],
    ) || original["immutable"] != true
        || original["score_attempt"] != 1
        || original["scorer_attempt"] != 1
        || !digest(&original["digest"])
    {
        return false;
    }

    let custody = &value["custody"];
    if !exact_keys(
        custody,
        &[
            "independent",
            "original_fixture_retired",
            "original_fixture_reused",
            "hidden_canary_hits",
            "private_identifier_hits",
            "private_digest_hits",
            "byte_identical_hidden_inputs",
            "sealed_changed_path_hits",
            "oracle_detail_hits",
        ],
    ) || custody["independent"] != true
        || custody["original_fixture_retired"] != true
        || custody["original_fixture_reused"] != false
        || [
            "hidden_canary_hits",
            "private_identifier_hits",
            "private_digest_hits",
            "byte_identical_hidden_inputs",
            "sealed_changed_path_hits",
            "oracle_detail_hits",
        ]
        .iter()
        .any(|field| custody[*field] != 0)
    {
        return false;
    }

    let reproducer = &value["reproducer"];
    let files = reproducer["files"].as_array();
    let commands = reproducer["commands"].as_array();
    if !exact_keys(reproducer, &["public_root", "files", "commands"])
        || files.is_none_or(|items| items.is_empty() || items.len() > 16)
        || commands.is_none_or(|items| items.is_empty() || items.len() > 4)
    {
        return false;
    }

    let environments = value["environments"].as_array();
    if environments.is_none_or(|items| items.len() != 2) {
        return false;
    }
    let environments = environments.unwrap();
    let platforms = environments
        .iter()
        .filter_map(|environment| environment["platform"].as_str())
        .collect::<BTreeSet<_>>();
    if platforms != BTreeSet::from(["windows-x86_64", "ubuntu-24.04-wsl2-x86_64"])
        || environments.iter().any(|environment| {
            !exact_keys(
                environment,
                &[
                    "platform",
                    "command_digest",
                    "actual_exit",
                    "category_reproduced",
                ],
            ) || environment["category_reproduced"] != true
                || !digest(&environment["command_digest"])
        })
        || environments[0]["actual_exit"] != environments[1]["actual_exit"]
    {
        return false;
    }

    let equivalence = &value["equivalence"];
    if !exact_keys(
        equivalence,
        &["category_matches", "exit_matches", "platforms_complete"],
    ) || equivalence["category_matches"] != true
        || equivalence["exit_matches"] != true
        || equivalence["platforms_complete"] != true
    {
        return false;
    }

    let bounds = &value["bounds"];
    if !exact_keys(
        bounds,
        &[
            "regular_files",
            "total_bytes",
            "commands_per_platform",
            "timeout_millis",
            "stdout_max_bytes",
            "stderr_max_bytes",
        ],
    ) || bounds["regular_files"]
        .as_u64()
        .is_none_or(|value| !(1..=16).contains(&value))
        || bounds["total_bytes"]
            .as_u64()
            .is_none_or(|value| !(1..=1_048_576).contains(&value))
        || bounds["commands_per_platform"]
            .as_u64()
            .is_none_or(|value| !(1..=4).contains(&value))
        || bounds["timeout_millis"]
            .as_u64()
            .is_none_or(|value| !(1..=60_000).contains(&value))
    {
        return false;
    }

    let lifecycle = &value["lifecycle"];
    exact_keys(
        lifecycle,
        &[
            "published",
            "cleanup_complete",
            "original_future_certification_eligible",
            "reproducer_future_certification_eligible",
            "replacement_package_required",
        ],
    ) && lifecycle["published"] == true
        && lifecycle["cleanup_complete"] == true
        && lifecycle["original_future_certification_eligible"] == false
        && lifecycle["reproducer_future_certification_eligible"] == false
        && lifecycle["replacement_package_required"] == true
        && value["limitations"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            if let Some((parent, key)) = pointer.rsplit_once('/') {
                let parent = if parent.is_empty() {
                    value
                } else {
                    value.pointer_mut(parent).expect("mutation parent")
                };
                if let Some(object) = parent.as_object_mut() {
                    object.insert(key.to_owned(), mutation["value"].clone());
                } else if let Some(array) = parent.as_array_mut() {
                    array[key.parse::<usize>().expect("array index")] = mutation["value"].clone();
                }
            }
        }
        "remove" => {
            let (parent, key) = pointer.rsplit_once('/').expect("remove pointer");
            let array = value
                .pointer_mut(parent)
                .expect("remove parent")
                .as_array_mut()
                .expect("remove array");
            array.remove(key.parse::<usize>().expect("remove index"));
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
fn prospective_post_score_release_contract_is_closed() {
    let schema = read_json(
        root()
            .join("schemas")
            .join("ferris.post-score-diagnostic-release.v1.schema.json"),
    );
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_closed_object_schemas(&schema);

    let fixture_path = root()
        .join("fixtures")
        .join("post-score-diagnostic-release.json");
    let fixture = read_json(&fixture_path);
    assert!(validate(&fixture));
    let mutations = read_json(
        root()
            .join("fixtures")
            .join("post-score-diagnostic-release-mutations.json"),
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 12);
    for mutation in mutations {
        let mut candidate = fixture.clone();
        apply_mutation(&mut candidate, mutation);
        if mutation["recompute_identity"] == true {
            candidate["receipt_identity"] = Value::String(receipt_identity(&candidate));
        }
        assert!(
            !validate(&candidate),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
    }
}
