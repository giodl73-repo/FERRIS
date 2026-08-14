use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-public-authority/v1";
const PROGRAM_ID: &str = "FERRIS-P34-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-AUTHORITY";
const CUTOFF: &str = "5df7492fa759c415f6ce540a33a4e89c46714348";
const DECLARATION_IDENTITY: &str =
    "sha256:8975e07b9dd417604d06be12a24a448e8ae1834991aca9db086ae7c11b0b1e34";
const PULSE_32_RESULT: &str =
    "sha256:27ff0f0c2a4768628fdcdfa7916efa7fe12217faa7bec20f65dbde8e526f88fd";
const PULSE_33_MANIFEST: &str =
    "sha256:9082bb18ab72e5e5ced2ec43811ecc5ce14ac43c9cd2878e4db0f10bf7a741fd";
const PULSE_33_AGGREGATE: &str =
    "sha256:07df7bc02cab288adccbdc0f87e45f3fd52939ebc9d22c8d064f773843e861a4";
const PULSE_33_SEAL: &str =
    "sha256:057f6dea59665401331b29ad984e203cca474143d7576a6617588922bf678cbd";
const PULSE_33_SEAL_PAYLOAD: &str =
    "sha256:7ebb70ddc2a610b8c7638f30d03d0707b7d00c3eabe56ab679f085d7035f109a";
const BUILD_ADAPTER: &str =
    "sha256:43bb31210175ceacba2431a238608d9973672a08de57572543ad0f9dae41cbe6";
const INPUT_CONTRACT: &str =
    "sha256:26fdb4b9eed558f1f03a66eaec13749bfbad7ea4612c6f7e58bb8e7b79e69295";
const INPUT_SCHEMA: &str =
    "sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b";
const INPUT_MUTATIONS: &str =
    "sha256:b33985e51f54c2ed0121b94571b622ee47bbd00450c8ab1c3d65d0f463276158";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR bytes");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end with LF");
    bytes
}

fn read_lf_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse LF JSON");
    (bytes, value)
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
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

fn exact_schema_accepts(schema: &Value, value: &Value) -> bool {
    if let Some(expected) = schema.get("const") {
        return expected == value;
    }
    if schema.get("type") != Some(&Value::String("object".to_owned()))
        || schema.get("additionalProperties") != Some(&Value::Bool(false))
    {
        return false;
    }
    let Some(object) = value.as_object() else {
        return false;
    };
    let required = schema["required"]
        .as_array()
        .expect("required")
        .iter()
        .map(|item| item.as_str().expect("required string"))
        .collect::<BTreeSet<_>>();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != required {
        return false;
    }
    let properties = schema["properties"].as_object().expect("properties");
    object.iter().all(|(key, item)| {
        properties
            .get(key)
            .is_some_and(|property| exact_schema_accepts(property, item))
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

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            let (parent, key) = pointer.rsplit_once('/').expect("mutation parent");
            let parent = if parent.is_empty() {
                value
            } else {
                value.pointer_mut(parent).expect("mutation target parent")
            };
            if let Some(object) = parent.as_object_mut() {
                object.insert(key.to_owned(), mutation["value"].clone());
            } else {
                parent.as_array_mut().expect("mutation target array")
                    [key.parse::<usize>().expect("array index")] = mutation["value"].clone();
            }
        }
        "remove" => {
            let (parent, key) = pointer.rsplit_once('/').expect("remove parent");
            let parent = if parent.is_empty() {
                value
            } else {
                value.pointer_mut(parent).expect("remove target parent")
            };
            if let Some(object) = parent.as_object_mut() {
                object.remove(key);
            } else {
                parent
                    .as_array_mut()
                    .expect("remove target array")
                    .remove(key.parse::<usize>().expect("array index"));
            }
        }
        operation => panic!("unsupported mutation {operation}"),
    }
}

fn git_output(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run git")
}

fn git_blob(path: &str) -> Vec<u8> {
    let output = git_output(&["show", &format!("{CUTOFF}:{path}")]);
    assert!(output.status.success(), "cutoff artifact missing: {path}");
    output.stdout
}

fn manifest_aggregate(entries: &[Value]) -> String {
    let mut digest = Sha256::new();
    for entry in entries {
        digest.update(entry["size"].as_u64().expect("manifest size").to_string());
        digest.update(b"\0");
        digest.update(entry["path"].as_str().expect("manifest path"));
        digest.update(b"\0");
        digest.update(
            entry["sha256"]
                .as_str()
                .expect("manifest digest")
                .strip_prefix("sha256:")
                .expect("digest prefix"),
        );
        digest.update(b"\n");
    }
    format!("sha256:{:x}", digest.finalize())
}

#[test]
fn pulse_34_authority_inherits_all_public_gates_and_rejects_704_mutations() {
    let (_, schema) = read_lf_json(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-public-authority.v1.schema.json"),
    );
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_closed_object_schemas(&schema);

    let (_, declaration) = read_lf_json(
        held_out_root().join("fixtures/process-exit-diagnostic-public-authority.json"),
    );
    assert!(exact_schema_accepts(&schema, &declaration));
    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["program_id"], PROGRAM_ID);
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&declaration), DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_complete_pulse_33_release"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_pulse_34_authority"],
        false
    );

    for pulse in [
        "pulse_22", "pulse_24", "pulse_26", "pulse_28", "pulse_30", "pulse_32",
    ] {
        let predecessor = &declaration["closed_predecessors"][pulse];
        assert_eq!(predecessor["disposition"], "invalid", "{pulse}");
        assert_eq!(predecessor["candidate_retries"], 0, "{pulse}");
        assert_eq!(predecessor["category_conclusion"], Value::Null, "{pulse}");
        assert_eq!(predecessor["permanently_closed"], true, "{pulse}");
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
    assert_eq!(
        declaration["closed_predecessors"]["pulse_32"]["public_result_digest"],
        PULSE_32_RESULT
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_32"]["blocker_stage"],
        "cutoff-build-freeze"
    );

    let pulse_32 =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-public-input.json"));
    for inherited in [
        "disclosure",
        "public_adapter_release",
        "public_input_preflight",
        "preflight",
        "platforms",
        "search_bounds",
        "seed_control",
        "coverage",
        "oracle",
        "collection",
        "minimization",
        "publication",
    ] {
        assert_eq!(
            declaration[inherited], pulse_32[inherited],
            "Pulse 34 must inherit Pulse 32 {inherited}"
        );
    }

    let mut expected_input = pulse_32["public_profile_evidence_input"].clone();
    expected_input["source_commit"] = Value::String(CUTOFF.to_owned());
    assert_eq!(declaration["public_profile_evidence_input"], expected_input);

    let mut expected_normalization = pulse_32["checkout_normalization"].clone();
    expected_normalization["materialization"]["source_commit"] = Value::String(CUTOFF.to_owned());
    assert_eq!(
        declaration["checkout_normalization"],
        expected_normalization
    );

    assert_eq!(
        declaration["checkout_normalization"]["git_attributes"]["file_count"],
        36
    );
    assert_eq!(
        declaration["checkout_normalization"]["binding_checks"]["passed_required"],
        76
    );
    assert_eq!(declaration["preflight"]["exact_adapter_invocations"], 1);
    assert_eq!(declaration["preflight"]["exact_pair_count"], 2);
    assert_eq!(declaration["preflight"]["exact_process_rows"], 4);
    assert_eq!(declaration["preflight"]["exact_pair_seals"], 2);
    assert_eq!(
        declaration["preflight"]["whole_store_cardinality"]["windows_rows"],
        2
    );
    assert_eq!(
        declaration["coverage"]["interaction_requirements"]
            .as_array()
            .expect("coverage interactions")
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
    assert_eq!(declaration["search_bounds"]["logical_case_max"], 512);
    assert_eq!(declaration["search_bounds"]["search_process_max"], 1024);
    assert_eq!(declaration["search_bounds"]["search_executions"], 1);
    assert_eq!(declaration["search_bounds"]["candidate_retries"], 0);
    assert_eq!(declaration["minimization"]["transformation_max"], 128);
    assert_eq!(declaration["minimization"]["process_max"], 256);
    assert_eq!(
        declaration["publication"]["no_reproduction_statement"],
        "bounded no-reproduction; no fix authority"
    );

    let (_, mutation_document) = read_lf_json(
        held_out_root().join("fixtures/process-exit-diagnostic-public-authority-mutations.json"),
    );
    assert_eq!(
        mutation_document["schema"],
        "ferris.process-exit-diagnostic-public-authority-mutations/v1"
    );
    let mutations = mutation_document["mutations"]
        .as_array()
        .expect("mutations");
    assert_eq!(mutations.len(), 704);
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
        assert_ne!(candidate, declaration, "no-op mutation {}", mutation["id"]);
        assert!(
            !exact_schema_accepts(&schema, &candidate),
            "schema accepted mutation: {}",
            mutation["id"]
        );
    }
}

#[test]
fn pulse_34_pins_the_complete_build_freeze_release_adapter_and_receipts() {
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-public-authority.json"));
    let release = &declaration["public_build_freeze_release"];
    assert_eq!(release["source_commit"], CUTOFF);
    assert_eq!(release["manifest"]["raw_sha256"], PULSE_33_MANIFEST);
    assert_eq!(release["manifest"]["aggregate"], PULSE_33_AGGREGATE);
    assert_eq!(release["manifest"]["file_count"], 37);
    assert_eq!(release["manifest"]["total_bytes"], 59_895);
    assert_eq!(release["release_seal"]["raw_sha256"], PULSE_33_SEAL);
    assert_eq!(
        release["release_seal"]["payload_sha256"],
        PULSE_33_SEAL_PAYLOAD
    );
    assert_eq!(release["build_adapter"]["raw_sha256"], BUILD_ADAPTER);

    let manifest_path = release["manifest"]["path"].as_str().expect("manifest path");
    let manifest_bytes = git_blob(manifest_path);
    assert_eq!(manifest_bytes.len() as u64, release["manifest"]["size"]);
    assert_eq!(sha256(&manifest_bytes), PULSE_33_MANIFEST);
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest");
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 37);
    assert_eq!(manifest_aggregate(files), PULSE_33_AGGREGATE);
    let release_root = "docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/";
    for entry in files {
        let relative = entry["path"].as_str().expect("release file path");
        let bytes = git_blob(&format!("{release_root}{relative}"));
        assert_eq!(bytes.len() as u64, entry["size"], "{relative}");
        assert_eq!(sha256(&bytes), entry["sha256"], "{relative}");
    }

    let seal_bytes = git_blob(release["release_seal"]["path"].as_str().expect("seal path"));
    assert_eq!(sha256(&seal_bytes), PULSE_33_SEAL);
    let seal: Value = serde_json::from_slice(&seal_bytes).expect("seal");
    assert_eq!(
        canonical_payload_sha256(&seal["payload"]),
        PULSE_33_SEAL_PAYLOAD
    );
    assert_eq!(seal["payload"]["manifest"]["sha256"], PULSE_33_MANIFEST);

    let adapter_bytes = git_blob(
        release["build_adapter"]["path"]
            .as_str()
            .expect("adapter path"),
    );
    assert_eq!(adapter_bytes.len(), 12_300);
    assert_eq!(sha256(&adapter_bytes), BUILD_ADAPTER);
    let adapter = String::from_utf8(adapter_bytes).expect("UTF-8 adapter");
    for required in [
        ".cargo",
        "compiler-artifact",
        "--message-format=json-render-diagnostics",
        "CARGO_INCREMENTAL",
        "/Brepro",
    ] {
        assert!(adapter.contains(required), "adapter missing {required}");
    }

    assert_eq!(release["public_build_receipt_count"], 2);
    let expected = BTreeMap::from([
        (
            "ubuntu-24.04-x86_64",
            (
                "sha256:23e4f56dc26be96adc140f5a1aa181389a8cdcd8497ca30fc47c15763dfc91c0",
                "sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae",
                1_945_448_u64,
                "sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4",
            ),
        ),
        (
            "windows-x86_64",
            (
                "sha256:3d1624d02fc5784a7b3daab9403123b377761bc8f63ec3d46aea7411ca460622",
                "sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a",
                1_436_672_u64,
                "sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8",
            ),
        ),
    ]);
    for receipt in release["public_build_receipts"]
        .as_array()
        .expect("public receipts")
    {
        let platform = receipt["platform"].as_str().expect("platform");
        let (raw_digest, payload_digest, size, artifact_digest) = expected[platform];
        let bytes = git_blob(receipt["path"].as_str().expect("receipt path"));
        assert_eq!(sha256(&bytes), raw_digest);
        let envelope: Value = serde_json::from_slice(&bytes).expect("receipt envelope");
        assert_eq!(
            canonical_payload_sha256(&envelope["payload"]),
            payload_digest
        );
        assert_eq!(receipt["payload_sha256"], payload_digest);
        assert_eq!(receipt["artifact_size"], size);
        assert_eq!(receipt["artifact_sha256"], artifact_digest);
        assert_eq!(
            receipt["artifact_discovery"],
            "cargo-compiler-artifact-json"
        );
        assert_eq!(receipt["diagnostic_execution"], false);
    }

    let freeze = &declaration["cutoff_build_freeze"];
    assert_eq!(freeze["exact_cutoff"], CUTOFF);
    assert_eq!(freeze["adapter_raw_sha256"], BUILD_ADAPTER);
    assert_eq!(freeze["exact_platform_count"], 2);
    assert_eq!(freeze["exact_binary_count_required"], 2);
    assert_eq!(freeze["exact_receipt_count_required"], 2);
    assert_eq!(
        freeze["cargo_resolution_order"],
        serde_json::json!(["PATH", "$HOME/.cargo/bin/cargo"])
    );
    assert_eq!(
        freeze["wsl_non_login_explicit_cargo_fallback_required"],
        true
    );
    assert_eq!(freeze["artifact_discovery"], "cargo-compiler-artifact-json");
    assert_eq!(freeze["target_directory_path_guessing_allowed"], false);
    assert_eq!(freeze["diagnostic_execution_allowed"], false);
    assert_eq!(freeze["started"], false);
    assert_eq!(freeze["passed"], false);
    let platforms = freeze["platforms"].as_array().expect("freeze platforms");
    assert_eq!(platforms.len(), 2);
    assert_eq!(platforms[0]["expected_filename"], "ferris.exe");
    assert_eq!(platforms[1]["expected_filename"], "ferris");
    for platform in platforms {
        assert_eq!(platform["exact_binary_size"], Value::Null);
        assert_eq!(platform["exact_binary_sha256"], Value::Null);
    }
}

#[test]
fn pulse_34_pins_public_input_blobs_and_self_validation_before_generation() {
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-public-authority.json"));
    let public = &declaration["public_profile_evidence_input"];
    assert_eq!(public["source_commit"], CUTOFF);
    assert_eq!(public["artifact_count"], 9);

    let contract = &public["contract"];
    let contract_bytes = git_blob(contract["path"].as_str().expect("contract path"));
    assert_eq!(sha256(&contract_bytes), INPUT_CONTRACT);
    assert_eq!(contract["raw_sha256"], INPUT_CONTRACT);

    let schema = &public["schema"];
    let schema_bytes = git_blob(schema["path"].as_str().expect("schema path"));
    assert_eq!(sha256(&schema_bytes), INPUT_SCHEMA);
    assert_eq!(schema["raw_sha256"], INPUT_SCHEMA);

    let fixtures = public["positive_fixtures"]
        .as_array()
        .expect("positive fixtures");
    assert_eq!(fixtures.len(), 6);
    for fixture in fixtures {
        let bytes = git_blob(fixture["path"].as_str().expect("fixture path"));
        assert_eq!(bytes.len() as u64, fixture["size"]);
        assert_eq!(sha256(&bytes), fixture["raw_sha256"]);
    }

    let mutation_binding = &public["mutation_controls"];
    let mutation_bytes = git_blob(
        mutation_binding["path"]
            .as_str()
            .expect("input mutation path"),
    );
    assert_eq!(sha256(&mutation_bytes), INPUT_MUTATIONS);
    assert_eq!(mutation_binding["control_count"], 33);
    let mutation_document: Value =
        serde_json::from_slice(&mutation_bytes).expect("input mutations");
    let declared = mutation_binding["controls"]
        .as_array()
        .expect("declared controls")
        .iter()
        .map(|item| {
            (
                item["id"].as_str().expect("declared id"),
                item["sha256"].as_str().expect("declared digest"),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for control in mutation_document["mutations"]
        .as_array()
        .expect("input controls")
    {
        let id = control["id"].as_str().expect("control id");
        let digest = sha256(&serde_json::to_vec(control).expect("canonical control"));
        assert_eq!(declared.get(id).copied(), Some(digest.as_str()), "{id}");
    }

    let preflight = &declaration["public_input_preflight"];
    assert_eq!(preflight["runs_after_adapter_preflight_pass"], true);
    assert_eq!(preflight["allowed_artifact_count"], 9);
    assert_eq!(preflight["ferris_source_allowed"], false);
    assert_eq!(preflight["ferris_tests_allowed"], false);
    assert_eq!(preflight["self_validation_positive_accepts_required"], 6);
    assert_eq!(preflight["self_validation_negative_matches_required"], 33);
    assert_eq!(
        preflight["self_validation_total_classifications_required"],
        39
    );
    assert_eq!(preflight["started"], false);
    assert_eq!(preflight["passed"], false);
}

#[test]
fn pulse_34_authority_is_later_than_cutoff_and_indexes_publish_exact_counts() {
    for path in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-34.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_AUTHORITY.md",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-public-authority.v1.schema.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-authority.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-authority-mutations.json",
        "docs/plans/reviews/PULSE-34-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
        "crates/ferris-cli/tests/process_exit_diagnostic_public_authority.rs",
    ] {
        let absent = git_output(&["cat-file", "-e", &format!("{CUTOFF}:{path}")]);
        assert!(
            !absent.status.success(),
            "Pulse 34 authority must be later than cutoff: {path}"
        );
    }

    let contract =
        fs::read_to_string(held_out_root().join("PROCESS_EXIT_DIAGNOSTIC_PUBLIC_AUTHORITY.md"))
            .expect("Pulse 34 contract");
    for required in [
        CUTOFF,
        DECLARATION_IDENTITY,
        PULSE_33_MANIFEST,
        PULSE_33_AGGREGATE,
        PULSE_33_SEAL,
        BUILD_ADAPTER,
        "36/36",
        "76/76",
        "39/39",
        "`2/2/2`",
        "$HOME/.cargo/bin/cargo",
        "compiler-artifact",
        "two platforms, two binaries, and two receipts",
        "512",
        "1,024",
        "128",
        "256",
        "704 rejection controls",
    ] {
        assert!(
            contract.contains(required),
            "missing contract term {required}"
        );
    }

    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(
        schema_index.contains("ferris.process-exit-diagnostic-public-authority.v1.schema.json")
    );
    assert!(schema_index.contains("All 20 schemas"));

    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    assert!(fixture_index.contains("process-exit-diagnostic-public-authority.json"));
    assert!(fixture_index.contains("704 rejection controls"));
    assert!(fixture_index.contains("2203 total declared mutations"));

    for relative in [
        "CONTEXT.md",
        "README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-34.md",
        "docs/plans/reviews/PULSE-34-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 34 document");
        assert!(text.contains(CUTOFF), "{relative} cutoff");
        assert!(text.contains(DECLARATION_IDENTITY), "{relative} identity");
        assert!(text.contains("704"), "{relative} mutation count");
        assert!(
            text.contains("unexecuted") || text.contains("executes nothing"),
            "{relative} execution boundary"
        );
    }
}
