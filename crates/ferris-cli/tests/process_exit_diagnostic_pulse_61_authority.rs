use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "53e3c86653a71171a9301dd5cff185a522af1231";
const AUTHORITY_CUTOFF: &str = "70ed752359c04e4aac77a49280c37f2cf6b8d012";
const DECLARATION_IDENTITY: &str =
    "sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d";
const DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-61-authority.json";
const SCHEMA: &str = "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-61-authority.v1.schema.json";
const MUTATIONS: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-61-authority-mutations.json";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-61-authority/v1";
const MUTATION_COUNT: usize = 20_058;
const PRIOR_REGISTRY_TOTAL: usize = 119_667;
const REGISTRY_TOTAL: usize = 139_725;

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
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

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn declaration_identity(value: &Value) -> String {
    let mut payload = value.clone();
    payload["declaration_identity"] = Value::String(String::new());
    let mut bytes = format!("{DOMAIN}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).expect("canonical declaration"));
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
            assert_eq!(schema["items"], false);
            assert_eq!(schema["minItems"], items.len());
            assert_eq!(schema["maxItems"], items.len());
            assert_eq!(schema["const"], *declaration);
            for (index, child) in items.iter().enumerate() {
                assert_closed_schema(&schema["prefixItems"][index], child);
            }
        }
        _ => assert_eq!(schema["const"], *declaration),
    }
}

fn git_output(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run Git")
}

fn git_text(args: &[&str]) -> String {
    let output = git_output(args);
    assert!(
        output.status.success(),
        "Git command failed {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("Git output is UTF-8")
        .trim()
        .to_owned()
}

fn git_blob(revision: &str, relative_path: &str) -> Vec<u8> {
    let spec = format!("{revision}:{relative_path}");
    let output = git_output(&["show", &spec]);
    assert!(
        output.status.success(),
        "missing Git blob {spec}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn assert_zero_execution_state(value: &Value) {
    for (field, state) in value.as_object().expect("execution state") {
        match state {
            Value::Number(number) => assert_eq!(number.as_i64(), Some(0), "{field} zero"),
            Value::Bool(boolean) => assert!(!boolean, "{field} false"),
            Value::Null => {}
            other => panic!("state {field} must be zero, false, or null: {other}"),
        }
    }
}

fn unescape_token(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    let tokens = if pointer.is_empty() {
        Vec::new()
    } else {
        pointer
            .strip_prefix('/')
            .expect("absolute pointer")
            .split('/')
            .map(unescape_token)
            .collect::<Vec<_>>()
    };
    let (last, parents) = tokens.split_last().expect("non-root mutation pointer");
    let mut parent = value;
    for token in parents {
        if parent.is_object() {
            parent = parent
                .as_object_mut()
                .expect("object mutation segment")
                .get_mut(token)
                .expect("object mutation segment");
        } else {
            let index = token.parse::<usize>().expect("array mutation index");
            parent = parent
                .as_array_mut()
                .expect("array mutation segment")
                .get_mut(index)
                .expect("array mutation segment");
        }
    }

    match mutation["operation"].as_str().expect("mutation operation") {
        "add" | "replace" => {
            let replacement = mutation["value"].clone();
            if let Some(object) = parent.as_object_mut() {
                object.insert(last.clone(), replacement);
            } else {
                let index = last.parse::<usize>().expect("array mutation index");
                parent.as_array_mut().expect("mutation array target")[index] = replacement;
            }
        }
        "remove" => {
            if let Some(object) = parent.as_object_mut() {
                assert!(object.remove(last).is_some(), "remove object member");
            } else {
                parent
                    .as_array_mut()
                    .expect("mutation array target")
                    .remove(last.parse::<usize>().expect("mutation array index"));
            }
        }
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

#[test]
fn pulse_61_authority_artifacts_remain_exact_historical_prelaunch_record() {
    let authority_revision = format!("{AUTHORITY_COMMIT}^{{commit}}");
    let cutoff_revision = format!("{AUTHORITY_CUTOFF}^{{commit}}");
    assert_eq!(git_text(&["rev-parse", &authority_revision]), AUTHORITY_COMMIT);
    assert_eq!(git_text(&["rev-parse", &cutoff_revision]), AUTHORITY_CUTOFF);
    assert_eq!(
        git_text(&["merge-base", AUTHORITY_CUTOFF, AUTHORITY_COMMIT]),
        AUTHORITY_CUTOFF,
        "the historical cutoff predates its authority"
    );

    let root = repo_root();
    for path in [DECLARATION, SCHEMA, MUTATIONS] {
        assert_eq!(
            read_lf(root.join(path)),
            git_blob(AUTHORITY_COMMIT, path),
            "{path} must remain the exact historical authority artifact"
        );
    }

    let declaration = read_json(root.join(DECLARATION));
    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&declaration), DECLARATION_IDENTITY);
    assert_eq!(declaration["immutable_ferris"]["cutoff"], AUTHORITY_CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["pulse_59_head"],
        "6945f5fc96868c97267a1635fbb5219cc398eeb4"
    );
    assert_eq!(declaration["immutable_ferris"]["authority_present_at_cutoff"], false);
    assert_eq!(
        declaration["immutable_ferris"]["pulse_60_withdrawal_present_at_cutoff"],
        true
    );
    let pre = &declaration["pre_call_public_prerequisites"];
    assert_eq!(pre.get("reversible_creatability_probes"), None);
    assert_eq!(pre["private_runtime_root"]["existing"], true);
    assert_eq!(pre["private_runtime_root"]["empty"], true);
    assert_eq!(pre["private_runtime_root"]["safe_existing_directory_via_p51"], true);
    assert_eq!(pre["p27_cycle_root"]["absent"], true);
    assert_eq!(pre["p27_cycle_root"]["direct_child_of_private_runtime_root"], true);
    assert_eq!(pre["p41_public_custody_roots"]["final_root_absent"], true);
    assert_eq!(pre["p41_public_custody_roots"]["stage_root_absent"], true);
    assert_eq!(
        pre["p41_public_custody_roots"]["rollback_path_absent_before_callable"],
        true
    );
    assert_eq!(pre["p59_terminal_root"]["absent"], true);
    assert_eq!(pre["p59_terminal_root"]["derived_from_private_runtime_root_parent"], true);
    assert_eq!(pre["ubuntu_runtime_parent"]["absolute_native_linux_path_string"], true);
    assert_eq!(pre["ubuntu_runtime_parent"]["not_mnt_mount"], true);
    assert_zero_execution_state(&declaration["execution_state"]);
}

#[test]
fn pulse_61_historical_authority_retains_exact_p59_surface_and_closed_schema() {
    let root = repo_root();
    let declaration = read_json(root.join(DECLARATION));
    let schema = read_json(root.join(SCHEMA));
    assert_eq!(schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
    assert_eq!(schema["$ref"], "#/$defs/exactAuthority");
    assert_closed_schema(&schema["$defs"]["exactAuthority"], &declaration);

    let runtime = &declaration["runtime_binding"];
    assert_eq!(
        runtime["sole_callable"],
        "run_witness_preserving_capability_materialization_executor"
    );
    assert_eq!(runtime["sole_callable_invocations_exact"], 1);
    assert_eq!(runtime["no_runtime_injection"], true);
    assert_eq!(
        runtime["accepted_input_names"],
        serde_json::json!([
            "repo_root",
            "private_runtime_root",
            "p27_cycle_root",
            "p39_checkout_root",
            "p41_final_root",
            "ubuntu_runtime_parent"
        ])
    );
    let order = &declaration["execution_order"];
    assert_eq!(order["p59_invocations_exact"], 1);
    assert_eq!(order["p58_invocations_exact"], 1);
    assert_eq!(order["p56_capability_publications_per_platform_exact"], 1);
    assert_eq!(order["terminalization_only_after_private_runtime_cleanup"], true);
    assert_eq!(declaration["public_release_bindings"].as_object().unwrap().len(), 14);
    let transfer = &declaration["terminal_transfer_contract"];
    assert_eq!(
        transfer["published_result"]["p43_destination"],
        "docs/simulations/profile-diff-held-out/pulse-61-public-result/"
    );
    assert_eq!(
        transfer["published_failure_witness"]["destination"],
        "docs/simulations/profile-diff-held-out/pulse-61-publication-witness/"
    );
    for destination in [
        transfer["published_result"]["p43_destination"].as_str().unwrap(),
        transfer["published_result"]["p47_destination"].as_str().unwrap(),
        transfer["published_failure_witness"]["destination"].as_str().unwrap(),
    ] {
        assert!(destination.contains("pulse-61-"));
        assert!(!root.join(destination.trim_end_matches('/')).exists());
    }
}

#[test]
fn pulse_61_mutation_registry_remains_closed_and_monotonic() {
    let root = repo_root();
    let declaration = read_json(root.join(DECLARATION));
    let schema = read_json(root.join(SCHEMA));
    let mutations = read_json(root.join(MUTATIONS));
    let controls = mutations["controls"].as_array().expect("mutation controls");
    assert_eq!(mutations["authority_schema"], DOMAIN);
    assert_eq!(mutations["base_declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(mutations["mutation_count"], MUTATION_COUNT);
    assert_eq!(controls.len(), MUTATION_COUNT);
    assert_eq!(mutations["prior_registry_total"], PRIOR_REGISTRY_TOTAL);
    assert_eq!(mutations["registry_total"], REGISTRY_TOTAL);
    assert_eq!(PRIOR_REGISTRY_TOTAL + MUTATION_COUNT, REGISTRY_TOTAL);
    assert_eq!(mutations["mutation_model"]["scalar_replacements_per_leaf"], 10);
    for covered in mutations["control_coverage"]
        .as_object()
        .expect("coverage")
        .values()
    {
        assert_eq!(covered, &Value::Bool(true));
    }
    assert_eq!(schema["$defs"]["exactAuthority"]["const"], declaration);
    let mut ids = BTreeSet::new();
    for (index, control) in controls.iter().enumerate() {
        let id = control["id"].as_str().expect("control ID");
        assert_eq!(id, format!("P61-M{:05}", index + 1));
        assert!(ids.insert(id.to_owned()), "unique mutation ID");
        let mut changed = declaration.clone();
        apply_mutation(&mut changed, control);
        assert_ne!(changed, declaration, "mutation {id} changes authority");
        assert_ne!(
            changed, schema["$defs"]["exactAuthority"]["const"],
            "schema rejects {id}"
        );
    }
}