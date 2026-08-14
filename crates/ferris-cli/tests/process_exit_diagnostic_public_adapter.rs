use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "ferris.process-exit-diagnostic-public-adapter/v1";
const PROGRAM_ID: &str = "FERRIS-P28-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-ADAPTER";
const CUTOFF: &str = "2935f44475b811e619f2ef62e0d408f39c7e8149";
const DECLARATION_IDENTITY: &str =
    "sha256:ee8964d34862f24fc1de3114a5e0dab97ece7ce784e041ea5ad412c1091a7f3a";
const PULSE_26_CONTRACT_DIGEST: &str =
    "sha256:e3546b7258706731141acd436ed83bb4fc05937d4af97ac87559b725e3daee86";
const PULSE_26_RESULT_DIGEST: &str =
    "sha256:00f19dda516fe4ec354b1b41ca0b9b78c32aba41a667ad077c505d60458d3842";
const PULSE_25_MANIFEST_DIGEST: &str =
    "sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75";
const PULSE_25_SOURCE_AGGREGATE: &str =
    "sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558";
const PULSE_25_TEST_AGGREGATE: &str =
    "sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62";
const PULSE_25_BUNDLE_AGGREGATE: &str =
    "sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc";
const PULSE_25_QUALIFICATION_DIGEST: &str =
    "sha256:1365dfd33f834b24f9e20f73afce313978b26d27e3c076ae90a8671eb841e723";
const PULSE_25_RECEIPT_DIGEST: &str =
    "sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780";
const PULSE_25_SEAL_DIGEST: &str =
    "sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0";
const ADAPTER_MANIFEST_DIGEST: &str =
    "sha256:449851e7b917f474fb1829b2d9f89a3f08a886733c476889dfad1ae27d097154";
const ADAPTER_RELEASE_AGGREGATE: &str =
    "sha256:31f38a79629d6b5da1fab9cb335450a95a1763f1ac80b1d8d851b103a318e540";
const ADAPTER_SOURCE_AGGREGATE: &str =
    "sha256:33106fb3ffc6c71148f954870dc5ae00ec607fc6c3b86412a81258c1fe7cfa63";
const ADAPTER_TEST_AGGREGATE: &str =
    "sha256:59b285cbc2eb6a285a88503c75c9c6b2d89f219851e7783c074b0a1f7a9a10ff";
const ADAPTER_COLLECTOR_AGGREGATE: &str =
    "sha256:c0421b4d44fecf132ea31939a044d7b8e1545dd2472da13ce5f0702defd85c0c";
const ROOT_CAUSE_DIGEST: &str =
    "sha256:9bd5ac7aa29b1f621df09eefc2ff33369c5ba5810e4e5f76f4e7e15aab57f478";
const QUALIFICATION_DIGEST: &str =
    "sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886";
const RELEASE_SEAL_DIGEST: &str =
    "sha256:6bff93434170ee79a4b5210ee26b647ab6dba351dccc4ccf3a5e224de56ced38";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
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

fn assert_lf_file(path: impl AsRef<Path>) {
    let bytes = fs::read(path).expect("read LF file");
    assert!(!bytes.contains(&b'\r'), "public file must use LF framing");
    assert!(bytes.ends_with(b"\n"), "public file must end with LF");
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
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
    const ROOT_KEYS: [&str; 24] = [
        "schema",
        "declaration_identity",
        "program_id",
        "recorded_on",
        "status",
        "closed_predecessors",
        "authority",
        "disclosure",
        "immutable_ferris",
        "pulse_25_collector_binding",
        "public_adapter_release",
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
fn pulse_28_authority_is_closed_unexecuted_and_preserves_pulse_26_bounds() {
    let schema_path = root()
        .join("schemas")
        .join("ferris.process-exit-diagnostic-public-adapter.v1.schema.json");
    let (_, schema) = read_lf_json(schema_path);
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["properties"]["schema"]["const"], DOMAIN);
    assert_closed_object_schemas(&schema);

    let fixture_path = root()
        .join("fixtures")
        .join("process-exit-diagnostic-public-adapter.json");
    let (_, declaration) = read_lf_json(fixture_path);
    assert!(validate(&declaration, &declaration, &schema));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);

    for pulse in ["pulse_22", "pulse_24", "pulse_26"] {
        let predecessor = &declaration["closed_predecessors"][pulse];
        assert_eq!(predecessor["disposition"], "invalid");
        assert_eq!(predecessor["candidate_retries"], 0);
        assert_eq!(predecessor["category_conclusion"], Value::Null);
        assert_eq!(predecessor["permanently_closed"], true);
        for field in [
            "retry",
            "resume",
            "reseed",
            "rescore",
            "reuse",
            "continuation",
            "correlation",
            "inference",
        ] {
            assert_eq!(predecessor[field], false, "{pulse} {field}");
        }
    }

    let pulse_26 = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-bundle.json"),
    );
    assert_eq!(
        declaration["pulse_25_collector_binding"]["pulse_26_public_collector_bundle"],
        pulse_26["public_collector_bundle"]
    );
    for inherited in [
        "coverage",
        "oracle",
        "search_bounds",
        "collection",
        "minimization",
        "publication",
    ] {
        assert_eq!(
            declaration[inherited], pulse_26[inherited],
            "Pulse 28 must inherit Pulse 26 {inherited} exactly"
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
    assert_eq!(declaration["preflight"]["exact_adapter_invocations"], 1);
    assert_eq!(declaration["preflight"]["exact_pair_count"], 2);
    assert_eq!(declaration["preflight"]["exact_process_rows"], 4);
    assert_eq!(declaration["preflight"]["exact_pair_seals"], 2);
    assert_eq!(
        declaration["preflight"]["exact_fresh_verifier_processes"],
        2
    );
    assert_eq!(
        declaration["preflight"]["whole_store_cardinality"],
        serde_json::json!({"windows_rows": 2, "ubuntu_rows": 2, "pair_seals": 2})
    );
    assert_eq!(declaration["result"]["search_started"], false);
    assert_eq!(declaration["result"]["cases_generated"], 0);
    assert_eq!(declaration["result"]["category_conclusion"], Value::Null);

    assert_eq!(
        sha256(
            &fs::read(root().join("PROCESS_EXIT_DIAGNOSTIC_PUBLIC_BUNDLE.md"))
                .expect("Pulse 26 contract")
        ),
        PULSE_26_CONTRACT_DIGEST
    );
    assert_eq!(
        sha256(
            &fs::read(
                root()
                    .join("pulse-26-public-result")
                    .join("PULSE-26-PUBLIC-RESULT.json")
            )
            .expect("Pulse 26 result")
        ),
        PULSE_26_RESULT_DIGEST
    );

    let contract = fs::read_to_string(root().join("PROCESS_EXIT_DIAGNOSTIC_PUBLIC_ADAPTER.md"))
        .expect("Pulse 28 contract");
    for required in [
        "not a retry, resume, reseed, rescore, reuse, continuation",
        CUTOFF,
        PULSE_25_MANIFEST_DIGEST,
        ADAPTER_MANIFEST_DIGEST,
        ADAPTER_RELEASE_AGGREGATE,
        ROOT_CAUSE_DIGEST,
        QUALIFICATION_DIGEST,
        RELEASE_SEAL_DIGEST,
        "exactly one adapter invocation",
        "exactly two fresh",
        "whole-store cardinality `2/2/2`",
        "512",
        "1,024",
        "128",
        "256",
        "bounded no-reproduction; no fix authority",
    ] {
        assert!(
            contract.contains(required),
            "missing Pulse 28 contract term {required}"
        );
    }

    for public_file in [
        root()
            .join("schemas")
            .join("ferris.process-exit-diagnostic-public-adapter.v1.schema.json"),
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-adapter.json"),
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-adapter-mutations.json"),
    ] {
        assert_lf_file(public_file);
    }
    for public_file in [
        root().join("PROCESS_EXIT_DIAGNOSTIC_PUBLIC_ADAPTER.md"),
        repo_root()
            .join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-28.md"),
        repo_root().join(
            "docs/plans/reviews/PULSE-28-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-ADAPTER-ROLE-REVIEW.md",
        ),
    ] {
        assert!(public_file.is_file(), "missing public file {public_file:?}");
    }

    let (_, mutations) = read_lf_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-adapter-mutations.json"),
    );
    assert_eq!(
        mutations["schema"],
        "ferris.process-exit-diagnostic-public-adapter-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 263);
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
fn pulse_28_pins_the_historical_pulse_25_collector_binding() {
    let declaration = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-adapter.json"),
    );
    let binding = &declaration["pulse_25_collector_binding"]["pulse_26_public_collector_bundle"];
    assert_eq!(binding["manifest_digest"], PULSE_25_MANIFEST_DIGEST);
    assert_eq!(
        binding["qualification_report_digest"],
        PULSE_25_QUALIFICATION_DIGEST
    );
    assert_eq!(binding["release_receipt_digest"], PULSE_25_RECEIPT_DIGEST);
    assert_eq!(binding["release_seal_digest"], PULSE_25_SEAL_DIGEST);
    assert_eq!(binding["source_aggregate"], PULSE_25_SOURCE_AGGREGATE);
    assert_eq!(binding["test_aggregate"], PULSE_25_TEST_AGGREGATE);
    assert_eq!(binding["bundle_aggregate"], PULSE_25_BUNDLE_AGGREGATE);
    let files = binding["files"].as_array().expect("historical files");
    assert_eq!(files.len(), 9);
    let mut paths = BTreeSet::new();
    for file in files {
        assert!(exact_keys(file, &["path", "kind", "size", "sha256"]));
        assert!(paths.insert(file["path"].as_str().expect("historical path")));
    }
    assert_eq!(files[0]["path"], "durability.py");
    assert_eq!(files[0]["size"], 8748);
    assert_eq!(
        files[0]["sha256"],
        "sha256:a53b44f9536f2728ee018a315659d8a460de3fa1f5cd24c17c7637e5cd58d8dc"
    );
}

#[test]
fn pulse_28_pins_the_historical_pulse_27_adapter_binding_and_preflight() {
    let declaration = read_json(
        root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-adapter.json"),
    );
    let binding = &declaration["public_adapter_release"];
    assert_eq!(binding["manifest_digest"], ADAPTER_MANIFEST_DIGEST);
    assert_eq!(binding["root_cause_report_digest"], ROOT_CAUSE_DIGEST);
    assert_eq!(
        binding["qualification_receipt_digest"],
        QUALIFICATION_DIGEST
    );
    assert_eq!(binding["release_seal_digest"], RELEASE_SEAL_DIGEST);
    assert_eq!(binding["file_count"], 20);
    assert_eq!(binding["private_workspace_access"], false);
    assert_eq!(binding["non_manifest_files_in_copied_workspace"], false);
    assert_eq!(
        binding["digests"]["release_aggregate"],
        ADAPTER_RELEASE_AGGREGATE
    );
    assert_eq!(
        binding["digests"]["adapter_source_aggregate"],
        ADAPTER_SOURCE_AGGREGATE
    );
    assert_eq!(
        binding["digests"]["adapter_test_aggregate"],
        ADAPTER_TEST_AGGREGATE
    );
    assert_eq!(
        binding["digests"]["collector_bundle_aggregate"],
        ADAPTER_COLLECTOR_AGGREGATE
    );
    assert_eq!(binding["counts"]["qualification_cycles"], 50);
    assert_eq!(binding["counts"]["qualification_process_rows"], 200);
    assert_eq!(binding["counts"]["qualification_pair_seals"], 100);
    assert_eq!(binding["collector_modified"], false);
    let files = binding["files"].as_array().expect("historical files");
    assert_eq!(files.len(), 20);
    let mut paths = BTreeSet::new();
    for file in files {
        assert!(exact_keys(file, &["kind", "path", "sha256", "size"]));
        assert!(paths.insert(file["path"].as_str().expect("historical path")));
    }
    assert_eq!(files[0]["path"], "README.md");
    assert_eq!(files[0]["size"], 1203);
    assert_eq!(
        files[0]["sha256"],
        "sha256:9cd2b14dacac1bd34d327ea4557b5facdd49437fc655a99defeb78e1bdc7f0f3"
    );

    let preflight = &declaration["preflight"];
    assert_eq!(preflight["exact_adapter_invocations"], 1);
    assert_eq!(preflight["exact_pair_count"], 2);
    assert_eq!(preflight["exact_windows_rows"], 2);
    assert_eq!(preflight["exact_ubuntu_rows"], 2);
    assert_eq!(preflight["exact_process_rows"], 4);
    assert_eq!(preflight["exact_pair_seals"], 2);
    assert_eq!(preflight["exact_fresh_verifier_processes"], 2);
    assert_eq!(preflight["exact_windows_verifier_processes"], 1);
    assert_eq!(preflight["exact_ubuntu_verifier_processes"], 1);
    assert_eq!(preflight["pair_retries"], 0);
    assert_eq!(preflight["adapter_invocation_retries"], 0);
    assert_eq!(preflight["verifier_retries"], 0);
    assert_eq!(preflight["zero_residue_required"], true);
    assert_eq!(
        preflight["failure_disposition"],
        "invalid-before-candidates"
    );
    assert_eq!(preflight["started"], false);
}

#[test]
fn pulse_28_public_result_is_exact_invalid_before_candidates_and_sealed() {
    let result_path = root()
        .join("pulse-28-public-result")
        .join("PULSE-28-PUBLIC-RESULT.json");
    let (bytes, result) = read_lf_json(result_path);
    assert_eq!(
        sha256(&bytes),
        "sha256:955bb0e2f0ca614a988fbd72ae8abca43b411e46bf2416885d4238ab447309a2"
    );
    assert!(exact_keys(
        &result,
        &["payload", "payload_sha256", "receipt_id", "schema"]
    ));
    assert_eq!(
        result["schema"],
        "ferris.pulse-28-public-custody-receipt/v1"
    );
    assert_eq!(
        result["payload_sha256"],
        "sha256:23b595e6bad0b41170ff8b48d55b4f2b6d3db605c6773e5550b24a61cc8767a2"
    );
    assert_eq!(result["receipt_id"], result["payload_sha256"]);
    assert_eq!(
        result["payload_sha256"],
        canonical_payload_sha256(&result["payload"])
    );

    let payload = &result["payload"];
    assert_eq!(payload["disposition"], "invalid");
    assert_eq!(payload["category_conclusion"], Value::Null);
    assert_eq!(
        payload["blocker"]["stage"],
        "public-package-binding-before-copy"
    );
    assert_eq!(
        payload["blocker"]["effect"],
        "invalid-before-candidates; repair, substitution, adapter preflight, and candidate launch are prohibited"
    );
    assert_eq!(
        payload["blocker"]["first_mismatch"]["binding"],
        "pulse-25-public-manifest-sha256"
    );
    assert_eq!(
        payload["blocker"]["first_mismatch"]["expected"],
        "sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75"
    );
    assert_eq!(
        payload["blocker"]["first_mismatch"]["observed"],
        "sha256:03322e9fe6a3df6c71161e5f3916c51cc66c9453e9f1f3141bcc703bd02d7a0d"
    );

    let audit = &payload["public_binding_audit"];
    assert_eq!(audit["checks"], 60);
    assert_eq!(audit["passed"], 10);
    assert_eq!(audit["failed"], 50);
    assert_eq!(audit["pulse_25"]["checks"], 18);
    assert_eq!(audit["pulse_25"]["passed"], 0);
    assert_eq!(audit["pulse_25"]["failed"], 18);
    assert_eq!(audit["pulse_27"]["checks"], 42);
    assert_eq!(audit["pulse_27"]["passed"], 10);
    assert_eq!(audit["pulse_27"]["failed"], 32);
    assert_eq!(audit["custody_package_file_count"], 0);

    assert_eq!(payload["cutoff_and_build"]["binaries_built"], 0);
    assert_eq!(payload["cutoff_and_build"]["binary_digests"], Value::Null);
    assert_eq!(payload["preflight"]["started"], false);
    assert_eq!(payload["preflight"]["adapter_invocations"], 0);
    assert_eq!(payload["preflight"]["process_rows"], 0);
    assert_eq!(payload["preflight"]["completed_pairs"], 0);
    assert_eq!(payload["preflight"]["pair_seals"], 0);
    assert_eq!(payload["preflight"]["retries"], 0);
    assert_eq!(payload["fresh_generation"]["started"], false);
    assert_eq!(payload["fresh_generation"]["cases_generated"], 0);
    assert_eq!(payload["search"]["started"], false);
    assert_eq!(payload["search"]["ferris_process_launches"], 0);
    assert_eq!(payload["search"]["completed_cross_platform_pairs"], 0);
    assert_eq!(payload["search"]["retries"], 0);
    assert_eq!(payload["search"]["further_launches_prohibited"], true);
    assert_eq!(
        payload["publication"]["authorized_reproducer_id"],
        Value::Null
    );
    assert_eq!(
        payload["publication"]["authorized_reproducer_path"],
        Value::Null
    );
    assert_eq!(
        payload["prohibitions_observed"]["candidate_launched"],
        false
    );
    assert_eq!(
        payload["prohibitions_observed"]["adapter_or_verifier_retried"],
        false
    );
}
