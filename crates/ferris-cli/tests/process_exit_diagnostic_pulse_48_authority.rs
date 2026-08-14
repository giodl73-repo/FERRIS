use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-48-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e";
const MUTATION_COUNT: usize = 9_498;
const PRIOR_TOTAL_MUTATION_COUNT: usize = 38_819;

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&read_lf(path)).expect("parse JSON")
}

fn declaration_identity(value: &Value) -> String {
    let mut payload = value.clone();
    payload["declaration_identity"] = Value::String(String::new());
    let mut bytes = format!("{DOMAIN}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).expect("serialize declaration"));
    sha256(&bytes)
}

fn assert_closed_schema(schema: &Value, declaration: &Value) {
    match declaration {
        Value::Object(object) => {
            assert_eq!(schema["type"], "object");
            assert_eq!(schema["additionalProperties"], false);
            assert_eq!(
                schema["required"],
                Value::Array(object.keys().cloned().map(Value::String).collect())
            );
            assert_eq!(schema["const"], *declaration);
            for (key, child) in object {
                assert_closed_schema(&schema["properties"][key], child);
            }
        }
        Value::Array(items) => {
            assert_eq!(schema["type"], "array");
            assert_eq!(schema["minItems"], items.len());
            assert_eq!(schema["maxItems"], items.len());
            assert_eq!(schema["items"], false);
            assert_eq!(schema["const"], *declaration);
            for (index, child) in items.iter().enumerate() {
                assert_closed_schema(&schema["prefixItems"][index], child);
            }
        }
        _ => assert_eq!(schema["const"], *declaration),
    }
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release directory entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        assert!(
            !metadata.file_type().is_symlink(),
            "{path:?} must not be a symlink"
        );
        if metadata.is_dir() {
            collect_files(root, &path, files);
        } else {
            assert!(metadata.is_file(), "{path:?} must be a regular file");
            files.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn cutoff_blob(path: &str) -> Vec<u8> {
    let output = Command::new("git")
        .current_dir(repo_root())
        .args(["show", &format!("{CUTOFF}:{path}")])
        .output()
        .expect("read immutable cutoff blob");
    assert!(output.status.success(), "missing cutoff blob {path}");
    output.stdout
}

fn assert_release_is_complete_current_cutoff_copy(name: &str, release: &Value) {
    assert_eq!(
        release["complete_exact_current_cutoff_tree_required"], true,
        "{name} complete cutoff tree"
    );
    assert_eq!(
        release["cutoff_tree_raw_file_bindings_required"], true,
        "{name} cutoff raw bindings"
    );
    assert_eq!(
        release["cutoff_tree_release_path_set_required"], true,
        "{name} cutoff path set"
    );
    assert_eq!(
        release["sealed_identities_required"], true,
        "{name} sealed identities"
    );

    let root = repo_root();
    let release_root = release["release_root"].as_str().expect("release root");
    let directory = root.join(release_root);
    let expected_paths = release["exact_release_tree_paths"]
        .as_array()
        .expect("release paths")
        .iter()
        .map(|value| value.as_str().expect("release path").to_owned())
        .collect::<BTreeSet<_>>();
    let mut actual_paths = BTreeSet::new();
    collect_files(&directory, &directory, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths, "{name} exact release tree");
    assert_eq!(
        actual_paths.len(),
        release["release_tree_file_count"]
            .as_u64()
            .expect("release tree count") as usize,
        "{name} release tree count"
    );

    let bindings = release["raw_sha256_by_path"]
        .as_object()
        .expect("raw release bindings");
    assert_eq!(
        bindings.len(),
        actual_paths.len(),
        "{name} raw binding count"
    );
    for path in actual_paths {
        assert!(!Path::new(&path).is_absolute());
        assert!(!path.split('/').any(|component| component == ".."));
        let bytes = read_lf(directory.join(&path));
        assert_eq!(
            sha256(&bytes),
            bindings
                .get(&path)
                .and_then(Value::as_str)
                .expect("raw path binding"),
            "{name} current raw binding for {path}"
        );
        assert_eq!(
            cutoff_blob(&format!("{release_root}/{path}")),
            bytes,
            "{name} must use the immutable cutoff copy for {path}"
        );
    }

    let manifest = read_json(directory.join("public-manifest.json"));
    let receipt = read_json(directory.join("qualification-receipt.json"));
    let seal = read_json(directory.join("release-seal.json"));
    assert_eq!(
        sha256(&read_lf(directory.join("public-manifest.json"))),
        release["manifest"]["raw_sha256"],
        "{name} manifest raw identity"
    );
    assert_eq!(
        manifest["aggregate"], release["manifest"]["aggregate"],
        "{name} manifest aggregate"
    );
    assert_eq!(
        manifest["file_count"], release["manifest"]["payload_file_count"],
        "{name} manifest payload count"
    );
    assert_eq!(
        sha256(&read_lf(directory.join("qualification-receipt.json"))),
        release["qualification_receipt"]["raw_sha256"],
        "{name} receipt raw identity"
    );
    assert_eq!(
        receipt["payload_sha256"], release["qualification_receipt"]["payload_sha256"],
        "{name} receipt payload identity"
    );
    assert_eq!(
        sha256(&read_lf(directory.join("release-seal.json"))),
        release["release_seal"]["raw_sha256"],
        "{name} seal raw identity"
    );
    assert_eq!(
        seal["payload_sha256"], release["release_seal"]["payload_sha256"],
        "{name} seal payload identity"
    );
}

fn unescape_token(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    let (parent_pointer, token) = pointer.rsplit_once('/').expect("mutation parent");
    let token = unescape_token(token);
    let parent = if parent_pointer.is_empty() {
        value
    } else {
        value
            .pointer_mut(parent_pointer)
            .expect("mutation target parent")
    };
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            if let Some(object) = parent.as_object_mut() {
                object.insert(token, mutation["value"].clone());
            } else {
                parent.as_array_mut().expect("mutation target array")
                    [token.parse::<usize>().expect("mutation array index")] =
                    mutation["value"].clone();
            }
        }
        "remove" => {
            if let Some(object) = parent.as_object_mut() {
                object.remove(&token);
            } else {
                parent
                    .as_array_mut()
                    .expect("mutation target array")
                    .remove(token.parse::<usize>().expect("mutation array index"));
            }
        }
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

fn assert_zero_execution_state(state: &Value) {
    for (field, value) in state.as_object().expect("execution state") {
        match value {
            Value::Number(number) => assert_eq!(
                number.as_i64(),
                Some(0),
                "execution state {field} must be zero"
            ),
            Value::Bool(boolean) => assert!(!boolean, "execution state {field} must be false"),
            Value::Null => {}
            other => panic!("execution state {field} has nonzero-state value {other}"),
        }
    }
}

#[test]
fn pulse_48_authority_is_closed_unexecuted_and_witness_publication_ordered() {
    let held_out = held_out_root();
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-48-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-48-authority.v1.schema.json"),
    );
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["$ref"], "#/$defs/exactAuthority");
    assert_eq!(schema["$defs"]["exactAuthority"]["const"], declaration);
    assert_closed_schema(&schema["$defs"]["exactAuthority"], &declaration);
    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&declaration), DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["current_cutoff_identity_is_self_excluding"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["core_autocrlf_false_fixed_before_checkout"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["fresh_platform_cutoff_checkouts"],
        serde_json::json!(["windows-x86_64", "ubuntu-24.04-x86_64"])
    );
    assert_zero_execution_state(&declaration["execution_state"]);

    for predecessor in ["pulse_42", "pulse_46"] {
        let closed = &declaration["closed_predecessors"][predecessor];
        assert_eq!(closed["disposition"], "invalid-publication-integrity");
        assert_eq!(closed["category_conclusion"], Value::Null);
        assert_eq!(closed["permanently_closed"], true);
        assert_eq!(closed["non_retryable"], true);
        assert_eq!(closed["further_launches_prohibited"], true);
        for relationship in [
            "retry",
            "resume",
            "reconstruction",
            "reseed",
            "reuse",
            "correlation",
            "inference",
        ] {
            assert_eq!(closed[relationship], false, "{predecessor} {relationship}");
        }
    }

    let expected_gates = serde_json::json!([
        "pulse-41-pulse-39-public-custody",
        "windows-retained-binary-custody",
        "ubuntu-retained-binary-custody",
        "exact-adapter-preflight",
        "pulse-31-public-input",
        "pulse-35-pulse-37-normalization",
        "private-materialization",
        "bounded-process-exit-search",
    ]);
    assert_eq!(
        declaration["closed_execution_catalog"]["ordered_gate_ids"],
        expected_gates
    );
    assert_eq!(
        declaration["closed_execution_catalog"]["terminal_gate_id"],
        "bounded-process-exit-search"
    );
    assert_eq!(
        declaration["closed_execution_catalog"]["public_self_validation_classification"],
        "public-artifact-self-validation"
    );
    assert_eq!(
        declaration["closed_execution_catalog"]["public_self_validation_cannot_advance_execution"],
        true
    );

    let order = &declaration["execution_order"];
    assert_eq!(order["failure_stops_later_gates"], true);
    assert_eq!(
        order["later_gate_counts_after_stop_are_indeterminate_not_execution"],
        true
    );
    assert_eq!(order["pulse_41_pulse_39_custody_invocations_exact"], 1);
    assert_eq!(order["pulse_45_invocations_per_platform_exact"], 1);
    assert_eq!(
        order["pulse_44_invocations_per_platform_via_pulse_45_exact"],
        1
    );
    assert_eq!(
        order["pulse_47_witness_invocations_at_terminal_disposition_exact"],
        1
    );
    assert_eq!(order["pulse_47_invokes_pulse_43_exactly_once"], true);
    assert_eq!(
        order["pulse_43_terminal_invocations_only_through_pulse_47"],
        true
    );
    assert_eq!(order["direct_pulse_43_terminal_invocations"], 0);
    assert_eq!(
        order["p43_result_and_pulse_47_witness_roots_must_not_overlap"],
        true
    );

    let search = &declaration["pulse_48_bounded_process_exit_search"];
    assert_eq!(search["one_launch"], 1);
    assert_eq!(search["retries"], 0);
    assert_eq!(search["fallbacks"], 0);
    assert_eq!(search["cases_per_platform"], 70);
    assert_eq!(search["processes_per_platform"], 70);
    assert_eq!(search["total_processes"], 140);
    assert_eq!(search["fresh_private_seed_bytes"], 32);
    assert_eq!(search["first_target_mismatch_stop"], true);
    assert_eq!(
        search["category_conclusion_before_complete_search"],
        Value::Null
    );

    let releases = &declaration["public_release_custody"];
    for release in [
        "pulse_41_transactional_copy",
        "pulse_39_checkout_verifier",
        "pulse_43_ordered_result_publisher",
        "pulse_44_retained_binary_custody",
        "pulse_45_binary_custody_event_bridge",
        "pulse_47_publication_outcome_witness",
    ] {
        assert_release_is_complete_current_cutoff_copy(release, &releases[release]);
    }
    let pulse_47 = &releases["pulse_47_publication_outcome_witness"];
    assert_eq!(
        pulse_47["manifest"]["raw_sha256"],
        "sha256:44d5c72b9eb09dc7e24b476a4535fed662eadde3edee6ecbfe1fdfa644082f8b"
    );
    assert_eq!(
        pulse_47["manifest"]["aggregate"],
        "sha256:5cb97276ee2752888c40d44a50e45079c9e550f7e26398e5aa4841d98083143d"
    );
    assert_eq!(
        pulse_47["qualification_receipt"]["payload_sha256"],
        "sha256:dbe44afbb9f0ad43549113028da8dc5d2d0ca5fe9faa15824d7cd80e3edea355"
    );
    assert_eq!(
        pulse_47["release_seal"]["payload_sha256"],
        "sha256:a00478e73897781ddd88e8e0fcbca2d1453a72758cbbd8ec06ccd9d0c228f681"
    );

    let publication = &declaration["publication_integrity"];
    assert_eq!(
        publication["witness_publication_release"],
        "pulse_47_publication_outcome_witness"
    );
    assert_eq!(publication["witness_callable"], "witness_pulse_43");
    assert_eq!(
        publication["witness_invocations_at_terminal_disposition_exact"],
        1
    );
    assert_eq!(
        publication["pulse_43_invocations_per_pulse_47_witness_exact"],
        1
    );
    assert_eq!(
        publication["p43_terminal_invocations_only_through_pulse_47"],
        true
    );
    assert_eq!(publication["direct_pulse_43_publisher_invocations"], 0);
    assert_eq!(
        publication["witness_callable_argument_order"],
        serde_json::json!([
            "public-catalog",
            "public-events",
            "fresh-absent-absolute-p43-result-final-root",
            "fresh-absent-absolute-pulse-47-witness-final-root",
        ])
    );
    assert_ne!(
        publication["p43_result_final_root"],
        publication["pulse_47_witness_final_root"]
    );
    for control in [
        "p43_result_final_root_fresh_absent_absolute",
        "pulse_47_witness_final_root_fresh_absent_absolute",
        "roots_distinct",
        "root_path_sets_non_overlapping",
        "actual_main_workspace_observation_required_before_claiming_publication",
        "p43_result_root_path_set_observed_in_actual_main_workspace",
        "pulse_47_witness_root_path_set_observed_in_actual_main_workspace",
        "no_external_terminal_summary_before_witness_published",
        "witness_published_required_before_external_terminal_summary",
        "raw_hashes_recomputed_after_witness_publication",
        "payload_hashes_recomputed_after_witness_publication",
    ] {
        assert_eq!(publication[control], true, "{control}");
    }
    assert_eq!(publication["witness_final_files"], "2/2");
    assert_eq!(publication["rename_attempts"], 1);
    assert_eq!(publication["retries"], 0);
    assert_eq!(publication["fallbacks"], 0);
    assert_eq!(
        publication["external_summary_if_witnessed_p43_published"],
        serde_json::json!([
            "witnessed-public-aggregates",
            "witnessed-public-hashes",
            "public-safe-conclusion",
        ])
    );
    assert_eq!(
        publication["external_summary_if_witnessed_p43_failure"],
        serde_json::json!([
            "witnessed-p43-failure-code",
            "witnessed-p43-publication-sync-posture",
            "witnessed-p43-publication-rename-posture",
        ])
    );
    assert_eq!(
        publication["external_summary_if_witness_publication_fails"],
        serde_json::json!([
            "witness-publication-posture",
            "witness-publication-failure-code",
        ])
    );
    assert_eq!(
        publication["private_ordered_gate_details_in_external_summary"],
        false
    );
}

#[test]
fn pulse_48_mutation_controls_reject_each_authority_change() {
    let held_out = held_out_root();
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-48-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-48-authority.v1.schema.json"),
    );
    let mutations = read_json(
        held_out.join("fixtures/process-exit-diagnostic-pulse-48-authority-mutations.json"),
    );
    let controls = mutations["controls"].as_array().expect("mutation controls");

    assert_eq!(mutations["authority_schema"], DOMAIN);
    assert_eq!(mutations["base_declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(mutations["mutation_count"], MUTATION_COUNT);
    assert_eq!(controls.len(), MUTATION_COUNT);
    assert_eq!(
        PRIOR_TOTAL_MUTATION_COUNT + MUTATION_COUNT,
        48_317,
        "updated declared mutation registry"
    );
    for (_, covered) in mutations["control_coverage"]
        .as_object()
        .expect("control coverage")
    {
        assert_eq!(covered, true);
    }

    let mut ids = BTreeSet::new();
    for control in controls {
        assert!(
            ids.insert(control["id"].as_str().expect("control ID").to_owned()),
            "unique mutation ID"
        );
        let mut changed = declaration.clone();
        apply_mutation(&mut changed, control);
        assert_ne!(changed, declaration, "control {}", control["id"]);
        assert_ne!(
            changed, schema["$defs"]["exactAuthority"]["const"],
            "closed schema rejects control {}",
            control["id"]
        );
        assert!(
            changed.get("declaration_identity")
                != Some(&Value::String(DECLARATION_IDENTITY.to_owned()))
                || declaration_identity(&changed) != DECLARATION_IDENTITY,
            "identity rejects control {}",
            control["id"]
        );
    }
}
