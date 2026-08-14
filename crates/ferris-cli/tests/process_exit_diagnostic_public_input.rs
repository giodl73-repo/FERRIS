use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-public-input/v1";
const PROGRAM_ID: &str = "FERRIS-P32-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-INPUT";
const CUTOFF: &str = "29517d732db13cc2ffa304684b344f3538ab587d";
const DECLARATION_IDENTITY: &str =
    "sha256:88bdbd263fed865e94d16cbd0e6f78a2f330cdae5788f7d7bf93c51afd758812";
const PULSE_30_CONTRACT: &str =
    "sha256:a7c3c0ed5fff01dee8fccf81b5248d7183afef524e3bc68d355022aa714094b1";
const PULSE_30_RESULT: &str =
    "sha256:f75d33f054002cdd1b066678163ef926f62ec95ba826fef7273bc614c348f090";
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

fn validate(value: &Value, canonical: &Value) -> bool {
    const ROOT_KEYS: [&str; 27] = [
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
        "public_profile_evidence_input",
        "public_input_preflight",
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
        "checkout_normalization",
    ];
    if !exact_keys(value, &ROOT_KEYS)
        || value["schema"] != DOMAIN
        || value["program_id"] != PROGRAM_ID
        || value["recorded_on"] != "2026-08-14"
        || value["status"] != "authorized-unexecuted"
        || declaration_identity(value) != value["declaration_identity"]
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

#[test]
fn pulse_32_authority_is_closed_unexecuted_inherited_and_strongly_mutated() {
    let (_, schema) = read_lf_json(
        held_out_root()
            .join("schemas")
            .join("ferris.process-exit-diagnostic-public-input.v1.schema.json"),
    );
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_closed_object_schemas(&schema);

    let (_, declaration) = read_lf_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-input.json"),
    );
    assert!(validate(&declaration, &declaration));
    assert!(exact_schema_accepts(&schema, &declaration));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["authority_present_at_cutoff"],
        false
    );

    for pulse in ["pulse_22", "pulse_24", "pulse_26", "pulse_28", "pulse_30"] {
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
    assert_eq!(
        declaration["closed_predecessors"]["pulse_30"]["public_contract_digest"],
        PULSE_30_CONTRACT
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_30"]["public_result_digest"],
        PULSE_30_RESULT
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_30"]["further_launches_prohibited"],
        true
    );

    let pulse_30 = read_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-normalized-public-adapter.json"),
    );
    for inherited in [
        "disclosure",
        "public_adapter_release",
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
            declaration[inherited], pulse_30[inherited],
            "Pulse 32 must inherit Pulse 30 {inherited}"
        );
    }

    let mut expected_normalization = pulse_30["checkout_normalization"].clone();
    expected_normalization["materialization"]["source_commit"] = Value::String(CUTOFF.to_owned());
    assert_eq!(
        declaration["checkout_normalization"], expected_normalization,
        "only the immutable materialization commit advances"
    );

    let mut expected_collector = pulse_30["pulse_25_collector_binding"].clone();
    expected_collector["p28_custody_copy_source"] = Value::String(
        "Pulse 32 copies the collector only as the byte-identical collector subset of the normalized Pulse 27 release."
            .to_owned(),
    );
    assert_eq!(
        declaration["pulse_25_collector_binding"],
        expected_collector
    );

    let mut expected_freshness = pulse_30["freshness"].clone();
    for field in [
        "pulse_30_custody_material_access",
        "pulse_30_custody_material_reuse",
        "pulse_30_custody_material_correlation",
        "pulse_30_custody_material_inference",
    ] {
        expected_freshness[field] = Value::Bool(false);
    }
    assert_eq!(declaration["freshness"], expected_freshness);

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

    let (_, mutations) = read_lf_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-input-mutations.json"),
    );
    assert_eq!(
        mutations["schema"],
        "ferris.process-exit-diagnostic-public-input-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 538);
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
            !validate(&candidate, &declaration),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
        assert!(
            !exact_schema_accepts(&schema, &candidate),
            "schema accepted mutation: {}",
            mutation["id"]
        );
    }
}

#[test]
fn pulse_32_pins_all_public_input_blobs_controls_and_self_validation_gates() {
    let declaration = read_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-input.json"),
    );
    let public = &declaration["public_profile_evidence_input"];
    assert_eq!(public["source_commit"], CUTOFF);
    assert_eq!(public["byte_source"], "immutable-git-blob-by-path");
    assert_eq!(public["artifact_count"], 9);
    assert_eq!(public["all_artifacts_required_before_generation"], true);

    let contract = &public["contract"];
    let contract_path = contract["path"].as_str().expect("contract path");
    let contract_bytes = git_blob(contract_path);
    assert_eq!(contract_bytes.len() as u64, contract["size"]);
    assert_eq!(sha256(&contract_bytes), INPUT_CONTRACT);
    assert_eq!(contract["raw_sha256"], INPUT_CONTRACT);
    assert!(!contract_bytes.contains(&b'\r'));
    assert!(contract_bytes.ends_with(b"\n"));

    let schema = &public["schema"];
    let schema_path = schema["path"].as_str().expect("schema path");
    let schema_bytes = git_blob(schema_path);
    assert_eq!(schema_bytes.len() as u64, schema["size"]);
    assert_eq!(sha256(&schema_bytes), INPUT_SCHEMA);
    assert_eq!(schema["raw_sha256"], INPUT_SCHEMA);
    assert_eq!(schema["profile_schema"], "ferris.profile-evidence/v0");
    assert_eq!(
        schema["dialect"],
        "https://json-schema.org/draft/2020-12/schema"
    );

    let expected_fixture_names = [
        "profile-evidence-v0-positive-scalars.json",
        "profile-evidence-v0-positive-arrays.json",
        "profile-evidence-v0-positive-objects.json",
        "profile-evidence-v0-positive-nested-mixed.json",
        "profile-evidence-v0-positive-boundary-minimum.json",
        "profile-evidence-v0-positive-boundary-maximum.json",
    ];
    let fixtures = public["positive_fixtures"]
        .as_array()
        .expect("positive fixtures");
    assert_eq!(public["positive_fixture_count"], 6);
    assert_eq!(fixtures.len(), 6);
    for (fixture, expected_name) in fixtures.iter().zip(expected_fixture_names) {
        let path = fixture["path"].as_str().expect("fixture path");
        assert!(path.ends_with(expected_name));
        let bytes = git_blob(path);
        assert_eq!(bytes.len() as u64, fixture["size"]);
        assert_eq!(sha256(&bytes), fixture["raw_sha256"]);
        assert!(!bytes.contains(&b'\r'));
        assert!(bytes.ends_with(b"\n"));
    }

    let mutation_binding = &public["mutation_controls"];
    let mutation_path = mutation_binding["path"].as_str().expect("mutation path");
    let mutation_bytes = git_blob(mutation_path);
    assert_eq!(mutation_bytes.len() as u64, mutation_binding["size"]);
    assert_eq!(sha256(&mutation_bytes), INPUT_MUTATIONS);
    assert_eq!(mutation_binding["raw_sha256"], INPUT_MUTATIONS);
    assert_eq!(
        mutation_binding["schema"],
        "ferris.profile-evidence-v0-mutations/v1"
    );
    assert_eq!(
        mutation_binding["base_fixture"],
        "profile-evidence-v0-positive-nested-mixed.json"
    );
    assert_eq!(
        mutation_binding["control_digest_algorithm"],
        "sha256-canonical-json-sort-keys-v1"
    );
    assert_eq!(mutation_binding["control_count"], 33);

    let mutation_document: Value =
        serde_json::from_slice(&mutation_bytes).expect("public mutations");
    let controls = mutation_document["mutations"]
        .as_array()
        .expect("public controls");
    assert_eq!(controls.len(), 33);
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
    assert_eq!(declared.len(), 33);
    for control in controls {
        let id = control["id"].as_str().expect("control id");
        let canonical = serde_json::to_vec(control).expect("canonical control");
        let digest = sha256(&canonical);
        assert_eq!(declared.get(id).copied(), Some(digest.as_str()), "{id}");
    }

    let rules = &public["rules"];
    assert_eq!(rules["max_complete_file_bytes"], 1_048_576);
    assert_eq!(rules["strict_duplicate_rejection"], true);
    assert_eq!(rules["recursive_visible_ascii_member_names"], true);
    assert_eq!(rules["closed_root_and_sections"], true);
    assert_eq!(rules["classification_precedence_required"], true);
    assert_eq!(rules["generator_uses_only_public_rules"], true);
    assert_eq!(rules["classifier_uses_only_public_rules"], true);
    assert_eq!(rules["ferris_source_allowed"], false);
    assert_eq!(rules["ferris_tests_allowed"], false);

    let preflight = &declaration["public_input_preflight"];
    assert_eq!(preflight["required_before_generation"], true);
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
    assert_eq!(preflight["generator_freeze_allowed_only_after_pass"], true);
    assert_eq!(preflight["classifier_freeze_allowed_only_after_pass"], true);
    assert_eq!(
        preflight["failure_disposition"],
        "invalid-before-generation"
    );
    assert_eq!(preflight["started"], false);
    assert_eq!(preflight["passed"], false);

    let allowed = preflight["allowed_read_scope"]
        .as_array()
        .expect("allowed read scope")
        .iter()
        .map(|item| item.as_str().expect("allowed path"))
        .collect::<BTreeSet<_>>();
    let expected = std::iter::once(contract_path)
        .chain(std::iter::once(schema_path))
        .chain(
            fixtures
                .iter()
                .map(|fixture| fixture["path"].as_str().expect("fixture path")),
        )
        .chain(std::iter::once(mutation_path))
        .collect::<BTreeSet<_>>();
    assert_eq!(allowed, expected);
    assert!(
        allowed
            .iter()
            .all(|path| path.starts_with("docs/simulations/profile-diff-held-out/"))
    );
}

#[test]
fn pulse_32_authority_is_later_than_cutoff_and_indexes_publish_exact_counts() {
    for path in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-32.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_INPUT.md",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-public-input.v1.schema.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-input.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-input-mutations.json",
        "docs/plans/reviews/PULSE-32-PUBLIC-INPUT-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
        "crates/ferris-cli/tests/process_exit_diagnostic_public_input.rs",
    ] {
        let absent = git_output(&["cat-file", "-e", &format!("{CUTOFF}:{path}")]);
        assert!(
            !absent.status.success(),
            "Pulse 32 authority must be later than cutoff: {path}"
        );
    }

    let contract =
        fs::read_to_string(held_out_root().join("PROCESS_EXIT_DIAGNOSTIC_PUBLIC_INPUT.md"))
            .expect("Pulse 32 contract");
    for required in [
        CUTOFF,
        DECLARATION_IDENTITY,
        INPUT_CONTRACT,
        INPUT_SCHEMA,
        INPUT_MUTATIONS,
        "six positive",
        "33 per-control",
        "39/39",
        "36/36",
        "76/76",
        "`2/2/2`",
        "512",
        "1,024",
        "128",
        "256",
        "bounded no-reproduction; no fix authority",
        "Ferris production source",
        "Ferris tests",
    ] {
        assert!(
            contract.contains(required),
            "missing contract term {required}"
        );
    }

    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(schema_index.contains("ferris.process-exit-diagnostic-public-input.v1.schema.json"));
    assert!(schema_index.contains("All 19 schemas"));

    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    assert!(fixture_index.contains("process-exit-diagnostic-public-input.json"));
    assert!(fixture_index.contains("538 rejection controls"));
    assert!(fixture_index.contains("1499 total declared mutations"));

    for relative in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-32.md",
        "docs/plans/reviews/PULSE-32-PUBLIC-INPUT-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 32 document");
        assert!(text.contains(CUTOFF), "{relative} cutoff");
        assert!(text.contains(DECLARATION_IDENTITY), "{relative} identity");
        assert!(text.contains(INPUT_SCHEMA), "{relative} schema digest");
        assert!(text.contains("538"), "{relative} mutation count");
        assert!(text.contains("33"), "{relative} control count");
        assert!(text.contains("no execution") || text.contains("unexecuted"));
    }
}
