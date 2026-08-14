use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

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

fn aggregate(files: &BTreeMap<String, (String, Vec<u8>)>, kinds: Option<&[&str]>) -> String {
    let mut hasher = Sha256::new();
    for (path, (kind, bytes)) in files {
        if kinds.is_some_and(|accepted| !accepted.contains(&kind.as_str())) {
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
fn pulse_28_pins_and_recomputes_every_pulse_25_collector_binding() {
    let release_root = root().join("pulse-25-collector-source-release");
    let manifest_bytes = fs::read(release_root.join("public-manifest.json")).expect("manifest");
    let receipt_bytes = fs::read(release_root.join("release-receipt.json")).expect("receipt");
    let seal_bytes = fs::read(release_root.join("release-seal.json")).expect("seal");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse manifest");
    let receipt: Value = serde_json::from_slice(&receipt_bytes).expect("parse receipt");
    let qualification_bytes = fs::read(
        root()
            .join("pulse-23-collector-qualification")
            .join("collector-qualification-report.json"),
    )
    .expect("qualification report");

    assert_eq!(sha256(&manifest_bytes), PULSE_25_MANIFEST_DIGEST);
    assert_eq!(sha256(&qualification_bytes), PULSE_25_QUALIFICATION_DIGEST);
    assert_eq!(sha256(&receipt_bytes), PULSE_25_RECEIPT_DIGEST);
    assert_eq!(sha256(&seal_bytes), PULSE_25_SEAL_DIGEST);

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
    assert_eq!(binding["files"], manifest["files"]);

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
    assert_eq!(
        aggregate(&files, Some(&["source"])),
        PULSE_25_SOURCE_AGGREGATE
    );
    assert_eq!(aggregate(&files, Some(&["test"])), PULSE_25_TEST_AGGREGATE);
    assert_eq!(aggregate(&files, None), PULSE_25_BUNDLE_AGGREGATE);
    assert_eq!(binding["source_aggregate"], PULSE_25_SOURCE_AGGREGATE);
    assert_eq!(binding["test_aggregate"], PULSE_25_TEST_AGGREGATE);
    assert_eq!(binding["bundle_aggregate"], PULSE_25_BUNDLE_AGGREGATE);
    assert_eq!(receipt["disposition"], "pass");
    assert_eq!(receipt["prohibitions_observed"]["ferris_executed"], false);
}

#[test]
fn pulse_28_pins_every_adapter_file_digest_aggregate_and_exact_preflight() {
    let release_root = root().join("pulse-27-preflight-adapter-release");
    let (manifest_bytes, manifest) = read_lf_json(release_root.join("public-manifest.json"));
    let (root_cause_bytes, root_cause) = read_lf_json(release_root.join("root-cause-report.json"));
    let (qualification_bytes, qualification) =
        read_lf_json(release_root.join("qualification-receipt.json"));
    let (seal_bytes, seal) = read_lf_json(release_root.join("release-seal.json"));

    assert_eq!(sha256(&manifest_bytes), ADAPTER_MANIFEST_DIGEST);
    assert_eq!(sha256(&root_cause_bytes), ROOT_CAUSE_DIGEST);
    assert_eq!(sha256(&qualification_bytes), QUALIFICATION_DIGEST);
    assert_eq!(sha256(&seal_bytes), RELEASE_SEAL_DIGEST);

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
    assert_eq!(binding["files"], manifest["files"]);
    assert_eq!(binding["digests"], manifest["digests"]);
    assert_eq!(binding["counts"], manifest["counts"]);
    assert_eq!(binding["file_count"], 20);
    assert_eq!(binding["private_workspace_access"], false);
    assert_eq!(binding["non_manifest_files_in_copied_workspace"], false);

    let mut files = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for file in manifest["files"].as_array().expect("manifest files") {
        let path = file["path"].as_str().expect("file path");
        let relative = Path::new(path);
        assert!(!relative.is_absolute(), "manifest path must be relative");
        assert!(
            !relative.components().any(|component| matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )),
            "manifest path must remain inside the release"
        );
        assert!(paths.insert(path.to_owned()), "duplicate manifest path");
        let kind = file["kind"].as_str().expect("file kind");
        let bytes = fs::read(release_root.join(relative)).expect("release file");
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("file size")
        );
        assert_eq!(sha256(&bytes), file["sha256"]);
        files.insert(path.to_owned(), (kind.to_owned(), bytes));
    }
    assert_eq!(files.len(), 20);
    assert_eq!(aggregate(&files, None), ADAPTER_RELEASE_AGGREGATE);
    assert_eq!(
        aggregate(&files, Some(&["adapter-source"])),
        ADAPTER_SOURCE_AGGREGATE
    );
    assert_eq!(
        aggregate(&files, Some(&["adapter-test"])),
        ADAPTER_TEST_AGGREGATE
    );
    assert_eq!(
        aggregate(
            &files,
            Some(&["immutable-collector-source", "immutable-collector-test"])
        ),
        ADAPTER_COLLECTOR_AGGREGATE
    );
    assert_eq!(
        manifest["digests"]["release_aggregate"],
        ADAPTER_RELEASE_AGGREGATE
    );

    let pulse_25_bundle = root().join("pulse-25-collector-source-release/bundle");
    for (path, (kind, bytes)) in &files {
        if kind.starts_with("immutable-collector-") {
            let source_path = path
                .strip_prefix("collector/")
                .expect("collector release path");
            assert_eq!(
                bytes,
                &fs::read(pulse_25_bundle.join(source_path)).expect("Pulse 25 collector file"),
                "adapter release changed immutable collector file {path}"
            );
        }
    }

    assert_eq!(
        root_cause["blocker"],
        "preflight-cardinality-reload-failure"
    );
    assert_eq!(
        root_cause["collector_conclusion"]["modification_needed"],
        false
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["cycles_passed"],
        50
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["process_rows"],
        200
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["pair_seals"],
        100
    );
    assert_eq!(
        root_cause["public_evidence"]["qualification"]["fresh_process_reloads"],
        100
    );
    assert_eq!(qualification["payload"]["outcome"], "pass");
    assert_eq!(qualification["payload"]["cycles_passed"], 50);
    assert_eq!(qualification["payload"]["process_row_count"], 200);
    assert_eq!(qualification["payload"]["pair_seal_count"], 100);
    assert_eq!(qualification["payload"]["fresh_process_reload_count"], 100);
    assert_eq!(qualification["payload"]["retries_per_cycle"], 0);
    assert_eq!(qualification["payload"]["residue_count"], 0);
    assert_eq!(seal["payload"]["collector_modified"], false);
    assert_eq!(
        seal["payload"]["release"]["aggregate"],
        ADAPTER_RELEASE_AGGREGATE
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
