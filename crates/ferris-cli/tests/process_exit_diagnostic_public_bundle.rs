use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "ferris.process-exit-diagnostic-public-bundle/v1";
const PROGRAM_ID: &str = "FERRIS-P26-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-BUNDLE";
const CUTOFF: &str = "e01130a5c1fc5b8e58e13bbde03dfc39b8f1bf60";
const MANIFEST_DIGEST: &str =
    "sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75";
const SOURCE_AGGREGATE: &str =
    "sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558";
const TEST_AGGREGATE: &str =
    "sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62";
const BUNDLE_AGGREGATE: &str =
    "sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc";
const RELEASE_RECEIPT_DIGEST: &str =
    "sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780";
const RELEASE_SEAL_DIGEST: &str =
    "sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0";
const PULSE_22_CONTRACT_DIGEST: &str =
    "sha256:6f4447444c49814b61a3c6234af6f6690e40d618aa3f6acfa78e00fd8c0a2ec8";
const PULSE_22_RESULT_DIGEST: &str =
    "sha256:3dcd2def79cced56fb266e16e5d6c4bc12e9f7db688bf6f34cb0eed47743d2e7";
const PULSE_24_CONTRACT_DIGEST: &str =
    "sha256:1fc3904bd55587e36d22ac1d7532b365e87793dbe493f0073bd392fb546e02e8";
const PULSE_24_RESULT_DIGEST: &str =
    "sha256:b845858d0ca8c7011443140d8cfdbebbbe925f1d17e26c414cad09e48df19db4";

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

fn validate(value: &Value, canonical: &Value, schema: &Value) -> bool {
    const ROOT_KEYS: [&str; 23] = [
        "schema",
        "declaration_identity",
        "program_id",
        "recorded_on",
        "status",
        "closed_predecessors",
        "authority",
        "disclosure",
        "immutable_ferris",
        "public_collector_bundle",
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
    if !exact_keys(value, &ROOT_KEYS)
        || value["schema"] != DOMAIN
        || value["program_id"] != PROGRAM_ID
        || value["recorded_on"] != "2026-08-14"
        || value["status"] != "authorized-unexecuted"
        || !digest(&value["declaration_identity"])
        || declaration_identity(value) != value["declaration_identity"]
        || value["closed_predecessors"] != schema["properties"]["closed_predecessors"]["const"]
        || value["authority"] != schema["properties"]["authority"]["const"]
    {
        return false;
    }

    let mut actual = value.clone();
    let mut expected = canonical.clone();
    actual["declaration_identity"] = Value::String(String::new());
    expected["declaration_identity"] = Value::String(String::new());
    actual == expected
}

fn aggregate(files: &BTreeMap<String, (String, Vec<u8>)>, kind: Option<&str>) -> String {
    let mut hasher = Sha256::new();
    for (path, (file_kind, bytes)) in files {
        if kind.is_some_and(|expected| file_kind != expected) {
            continue;
        }
        let path_bytes = path.as_bytes();
        hasher.update((path_bytes.len() as u64).to_be_bytes());
        hasher.update(path_bytes);
        hasher.update(Sha256::digest(bytes));
    }
    format!("sha256:{:x}", hasher.finalize())
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

#[test]
fn pulse_26_authority_is_closed_unexecuted_and_preserves_public_contracts() {
    let schema_path = root()
        .join("schemas")
        .join("ferris.process-exit-diagnostic-public-bundle.v1.schema.json");
    let (_, schema) = read_lf_json(schema_path);
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["properties"]["schema"]["const"], DOMAIN);
    assert_closed_object_schemas(&schema);

    let fixture_path = root()
        .join("fixtures")
        .join("process-exit-diagnostic-public-bundle.json");
    let (_, declaration) = read_lf_json(fixture_path);
    assert!(validate(&declaration, &declaration, &schema));
    assert_eq!(
        declaration["declaration_identity"],
        "sha256:2c40011557fdc25a2865993e3f7e61420b7ce18c98f660e532b450337d35cb75"
    );
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);

    let pulse_22 = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-replication.json"),
    );
    let pulse_24 = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-replacement.json"),
    );
    for inherited in ["coverage", "oracle", "minimization"] {
        assert_eq!(
            declaration[inherited], pulse_22[inherited],
            "Pulse 22 {inherited} must be inherited exactly"
        );
        assert_eq!(
            declaration[inherited], pulse_24[inherited],
            "Pulse 24 {inherited} must be inherited exactly"
        );
    }
    assert_eq!(declaration["publication"], pulse_24["publication"]);
    for (field, expected) in pulse_22["publication"]
        .as_object()
        .expect("Pulse 22 publication")
    {
        assert_eq!(
            declaration["publication"][field], *expected,
            "Pulse 22 publication field {field} must be inherited exactly"
        );
    }
    assert_eq!(
        declaration["coverage"]["interaction_requirements"]
            .as_array()
            .expect("interactions")
            .len(),
        8
    );
    assert_eq!(
        declaration["oracle"]["compared_fields"]
            .as_array()
            .expect("oracle fields")
            .len(),
        8
    );
    assert_eq!(
        declaration["oracle"]["target_predicates"]
            .as_array()
            .expect("target predicates")
            .len(),
        6
    );
    assert_eq!(declaration["search_bounds"], pulse_24["search_bounds"]);
    assert_eq!(declaration["collection"], pulse_24["collection"]);
    assert_eq!(declaration["preflight"]["exact_pair_count"], 2);
    assert_eq!(declaration["preflight"]["pair_retries"], 0);
    assert_eq!(declaration["preflight"]["started"], false);
    assert_eq!(declaration["result"]["search_started"], false);
    assert_eq!(declaration["result"]["cases_generated"], 0);
    assert_eq!(declaration["result"]["search_processes"], 0);
    assert_eq!(declaration["result"]["category_conclusion"], Value::Null);

    let pulse_22_contract =
        fs::read(root().join("PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md")).expect("Pulse 22 contract");
    assert_eq!(sha256(&pulse_22_contract), PULSE_22_CONTRACT_DIGEST);
    let pulse_24_contract =
        fs::read(root().join("PROCESS_EXIT_DIAGNOSTIC_REPLACEMENT.md")).expect("Pulse 24 contract");
    assert_eq!(sha256(&pulse_24_contract), PULSE_24_CONTRACT_DIGEST);

    let pulse_22_result_bytes = fs::read(
        root()
            .join("pulse-22-public-result")
            .join("PULSE-22-PUBLIC-RESULT.json"),
    )
    .expect("Pulse 22 result");
    assert_eq!(sha256(&pulse_22_result_bytes), PULSE_22_RESULT_DIGEST);
    let pulse_22_result: Value =
        serde_json::from_slice(&pulse_22_result_bytes).expect("parse Pulse 22 result");
    assert_eq!(pulse_22_result["result"]["disposition"], "invalid");
    assert_eq!(pulse_22_result["result"]["retries"], 0);
    assert_eq!(
        pulse_22_result["result"]["category_conclusion"],
        Value::Null
    );
    assert_eq!(
        pulse_22_result["result"]["target_category_reproduced"],
        Value::Null
    );

    let pulse_24_result_bytes = fs::read(
        root()
            .join("pulse-24-public-result")
            .join("PULSE-24-PUBLIC-RESULT.json"),
    )
    .expect("Pulse 24 result");
    assert_eq!(sha256(&pulse_24_result_bytes), PULSE_24_RESULT_DIGEST);
    let pulse_24_result: Value =
        serde_json::from_slice(&pulse_24_result_bytes).expect("parse Pulse 24 result");
    assert_eq!(pulse_24_result["result"]["disposition"], "invalid");
    assert_eq!(pulse_24_result["result"]["preflight_processes"], 0);
    assert_eq!(pulse_24_result["result"]["search_processes"], 0);
    assert_eq!(pulse_24_result["result"]["retries"], 0);
    assert_eq!(
        pulse_24_result["result"]["category_conclusion"],
        Value::Null
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_24"]["public_label"],
        "invalid-before-candidates"
    );

    let contract = fs::read_to_string(root().join("PROCESS_EXIT_DIAGNOSTIC_PUBLIC_BUNDLE.md"))
        .expect("Pulse 26 contract");
    for required in [
        "not a retry, resume, reseed, rescore, reuse, or continuation",
        CUTOFF,
        MANIFEST_DIGEST,
        SOURCE_AGGREGATE,
        TEST_AGGREGATE,
        BUNDLE_AGGREGATE,
        RELEASE_RECEIPT_DIGEST,
        RELEASE_SEAL_DIGEST,
        "all nine per-file SHA-256 digests",
        "exactly two harmless synthetic atomic",
        "Windows/Ubuntu pairs",
        "512",
        "1,024",
        "128",
        "256",
        "bounded no-reproduction; no fix authority",
        "ferris.post-score-diagnostic-release/v1",
    ] {
        assert!(
            contract.contains(required),
            "missing Pulse 26 contract term {required}"
        );
    }

    let mutations_path = root()
        .join("fixtures")
        .join("process-exit-diagnostic-public-bundle-mutations.json");
    let (_, mutations) = read_lf_json(mutations_path);
    assert_eq!(
        mutations["schema"],
        "ferris.process-exit-diagnostic-public-bundle-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 176);
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
            !validate(&candidate, &declaration, &schema),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
    }
}

#[test]
fn pulse_26_public_collector_bundle_is_available_and_exact() {
    let release_root = root().join("pulse-25-collector-source-release");
    let manifest_bytes = fs::read(release_root.join("public-manifest.json")).expect("manifest");
    let receipt_bytes = fs::read(release_root.join("release-receipt.json")).expect("receipt");
    let seal_bytes = fs::read(release_root.join("release-seal.json")).expect("seal");
    assert_eq!(sha256(&manifest_bytes), MANIFEST_DIGEST);
    assert_eq!(sha256(&receipt_bytes), RELEASE_RECEIPT_DIGEST);
    assert_eq!(sha256(&seal_bytes), RELEASE_SEAL_DIGEST);

    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse manifest");
    let receipt: Value = serde_json::from_slice(&receipt_bytes).expect("parse receipt");
    let declaration = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-bundle.json"),
    );
    let binding = &declaration["public_collector_bundle"];

    assert!(release_root.join("bundle").is_dir());
    assert_eq!(manifest["file_count"], 9);
    assert_eq!(binding["file_count"], 9);
    assert_eq!(binding["files"], manifest["files"]);
    assert_eq!(binding["manifest_digest"], MANIFEST_DIGEST);
    assert_eq!(binding["source_aggregate"], SOURCE_AGGREGATE);
    assert_eq!(binding["test_aggregate"], TEST_AGGREGATE);
    assert_eq!(binding["bundle_aggregate"], BUNDLE_AGGREGATE);
    assert_eq!(binding["release_receipt_digest"], RELEASE_RECEIPT_DIGEST);
    assert_eq!(binding["release_seal_digest"], RELEASE_SEAL_DIGEST);
    assert_eq!(receipt["disposition"], "pass");
    assert_eq!(receipt["copy_verification"]["byte_for_byte_passed"], 9);
    assert_eq!(receipt["prohibitions_observed"]["ferris_executed"], false);

    let mut files = BTreeMap::new();
    for file in manifest["files"].as_array().expect("manifest files") {
        let path = file["path"].as_str().expect("file path");
        let kind = file["kind"].as_str().expect("file kind");
        let bytes = fs::read(release_root.join("bundle").join(path)).expect("bundle file");
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("file size")
        );
        assert_eq!(sha256(&bytes), file["sha256"]);
        files.insert(path.to_owned(), (kind.to_owned(), bytes));
    }
    assert_eq!(files.len(), 9);
    assert_eq!(aggregate(&files, Some("source")), SOURCE_AGGREGATE);
    assert_eq!(aggregate(&files, Some("test")), TEST_AGGREGATE);
    assert_eq!(aggregate(&files, None), BUNDLE_AGGREGATE);

    assert_eq!(binding["copy_policy"], "copy-nine-public-bundle-files-only");
    assert_eq!(binding["non_bundle_files_in_copied_workspace"], false);
    assert_eq!(
        binding["independent_per_file_hash_recomputation_required"],
        true
    );
    assert_eq!(
        binding["independent_source_aggregate_recomputation_required"],
        true
    );
    assert_eq!(
        binding["independent_test_aggregate_recomputation_required"],
        true
    );
    assert_eq!(
        binding["independent_bundle_aggregate_recomputation_required"],
        true
    );
    assert_eq!(
        declaration["result"],
        json!({
            "disposition": "authorized-unexecuted",
            "public_bundle_copied": false,
            "public_bundle_verified": false,
            "copied_file_count": 0,
            "per_file_hashes_recomputed": 0,
            "source_aggregate_recomputed": false,
            "test_aggregate_recomputed": false,
            "bundle_aggregate_recomputed": false,
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
    );
}
