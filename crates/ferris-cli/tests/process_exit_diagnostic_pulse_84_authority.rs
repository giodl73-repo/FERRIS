use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "f874ebfe29e58460fc0a553418d11d6785e84df9";
const PULSE_82_COMMIT: &str = "4549aef5748345bb3e17e2234c51f7ec460061d3";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-84-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:f5a76eebaa70d5c07de53e25fa34287083e166e29e0ede5f682732fd6dd1da5f";
const MANIFEST_RAW: &str =
    "sha256:7b08a16a3c6b07bf3759a54ea98d4cb887c3f2789d8fc25569356836f05266fd";
const MANIFEST_AGGREGATE: &str =
    "sha256:a6a529e5ca960a519852e048f21320ce45d9a9da7be73074498f578d9f7ae0c2";
const SEAL_RAW: &str = "sha256:0f57a5601dd24ae51cee2e54eca584c34cdac17fecb72499b6dcfe483bb71efd";
const RECEIPT_RAW: &str = "sha256:de87d8c5c1f2ca7b9e69f43ad149e022a1a7b33df359f661e6b17957897a6183";
const SOURCE_RAW: &str = "sha256:20a85b3009d2a75eba8684a4d17a3be24f16d832b34928cb21d59ebd1a0f8543";
const MUTATION_COUNT: usize = 389;
const PRIOR_REGISTRY_TOTAL: usize = 319_332;
const REGISTRY_TOTAL: usize = 319_721;
const RELEASE_ROOT: &str = "docs/simulations/profile-diff-held-out/pulse-82-witness-preserving-capability-materialization-executor-pulse35-release-tree-successor-release";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&read_lf(path)).expect("parse JSON")
}

fn git_output(arguments: &[String]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(arguments)
        .output()
        .expect("run Git")
}

fn cutoff_blob(relative: &str) -> Vec<u8> {
    let output = git_output(&["show".to_owned(), format!("{CUTOFF}:{relative}")]);
    assert!(output.status.success(), "missing cutoff blob {relative}");
    output.stdout
}

fn assert_cutoff_absent(relative: &str) {
    let output = git_output(&[
        "cat-file".to_owned(),
        "-e".to_owned(),
        format!("{CUTOFF}:{relative}"),
    ]);
    assert!(
        !output.status.success(),
        "self-excluding cutoff must not contain {relative}"
    );
}

fn assert_commit_precedes_cutoff(commit: &str) {
    let output = git_output(&[
        "merge-base".to_owned(),
        "--is-ancestor".to_owned(),
        commit.to_owned(),
        CUTOFF.to_owned(),
    ]);
    assert!(
        output.status.success(),
        "{commit} must precede cutoff {CUTOFF}"
    );
}

fn cutoff_tree_paths() -> BTreeSet<String> {
    let output = git_output(&[
        "ls-tree".to_owned(),
        "-r".to_owned(),
        "--name-only".to_owned(),
        CUTOFF.to_owned(),
        "--".to_owned(),
        RELEASE_ROOT.to_owned(),
    ]);
    assert!(output.status.success(), "list cutoff release tree");
    let prefix = format!("{RELEASE_ROOT}/");
    String::from_utf8(output.stdout)
        .expect("UTF-8 Git paths")
        .lines()
        .map(|path| {
            path.strip_prefix(&prefix)
                .expect("release-relative path")
                .to_owned()
        })
        .collect()
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
            let required = schema["required"]
                .as_array()
                .expect("required array")
                .iter()
                .map(|entry| entry.as_str().expect("required key").to_owned())
                .collect::<BTreeSet<_>>();
            assert_eq!(required, object.keys().cloned().collect());
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

fn pointer_tokens(pointer: &str) -> Vec<String> {
    if pointer.is_empty() {
        return Vec::new();
    }
    pointer
        .strip_prefix('/')
        .expect("JSON pointer")
        .split('/')
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect()
}

fn pointer_mut<'a>(root: &'a mut Value, pointer: &str) -> &'a mut Value {
    let mut current = root;
    for token in pointer_tokens(pointer) {
        current = match current {
            Value::Object(object) => object.get_mut(&token).expect("object pointer"),
            Value::Array(array) => {
                let index = token.parse::<usize>().expect("array index");
                array.get_mut(index).expect("array pointer")
            }
            _ => panic!("pointer traverses scalar"),
        };
    }
    current
}

fn apply_mutation(value: &mut Value, control: &Value) {
    let pointer = control["pointer"].as_str().expect("mutation pointer");
    match control["operation"].as_str().expect("mutation operation") {
        "replace-scalar" => {
            *pointer_mut(value, pointer) = control["value"].clone();
        }
        "remove-object-member" => {
            let object = pointer_mut(value, pointer)
                .as_object_mut()
                .expect("object removal target");
            let key = control["key"].as_str().expect("removed key");
            assert!(object.remove(key).is_some(), "removed member must exist");
        }
        "add-object-member" => {
            let object = pointer_mut(value, pointer)
                .as_object_mut()
                .expect("object addition target");
            let key = control["key"].as_str().expect("added key");
            assert!(object
                .insert(key.to_owned(), control["value"].clone())
                .is_none());
        }
        "remove-array-item" => {
            let array = pointer_mut(value, pointer)
                .as_array_mut()
                .expect("array removal target");
            let index = control["index"].as_u64().expect("removed index") as usize;
            array.remove(index);
        }
        operation => panic!("unknown mutation operation {operation}"),
    }
}

#[test]
fn pulse_84_binds_self_excluding_cutoff_and_exact_pulse_82() {
    assert_commit_precedes_cutoff(PULSE_82_COMMIT);
    assert_cutoff_absent(
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-84.md",
    );
    assert_cutoff_absent(
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-84-authority.json",
    );
    let pulse_83 =
        cutoff_blob("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-83.md");
    assert!(String::from_utf8(pulse_83)
        .expect("UTF-8 Pulse 83")
        .contains("ready-for-separate-authority-drafting"));

    let manifest_bytes = cutoff_blob(&format!("{RELEASE_ROOT}/public-manifest.json"));
    let seal_bytes = cutoff_blob(&format!("{RELEASE_ROOT}/release-seal.json"));
    let receipt_bytes = cutoff_blob(&format!("{RELEASE_ROOT}/qualification-receipt.json"));
    let source = cutoff_blob(&format!(
        "{RELEASE_ROOT}/witness_preserving_capability_materialization_executor.py"
    ));
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
    let seal: Value = serde_json::from_slice(&seal_bytes).expect("seal JSON");

    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(manifest["file_count"], 13);
    assert_eq!(manifest["release_tree_file_count"], 15);
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(seal["payload"]["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(sha256(&source), SOURCE_RAW);
    assert_eq!(source.len(), 29_487);
    assert_eq!(cutoff_tree_paths().len(), 15);
}

#[test]
fn pulse_84_authority_is_closed_comprehensive_and_unexecuted() {
    let root = repo_root();
    let declaration = read_json(root.join(
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-84-authority.json",
    ));
    let schema = read_json(root.join(
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-84-authority.v1.schema.json",
    ));
    let mutations = read_json(root.join(
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-84-authority-mutations.json",
    ));

    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&declaration), DECLARATION_IDENTITY);
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(declaration["pulse_82_release"]["commit"], PULSE_82_COMMIT);
    assert_eq!(
        declaration["runtime_binding"]["sole_callable"],
        "run_witness_preserving_capability_materialization_executor"
    );
    assert_eq!(
        declaration["runtime_binding"]["sole_callable_invocations_exact"],
        1
    );
    assert_eq!(
        declaration["authority"]["authority_consumed_on_sole_callable_invocation_attempt"],
        true
    );
    assert_eq!(declaration["authority"]["retry"], false);
    assert_eq!(declaration["authority"]["resume"], false);
    for value in declaration["execution_state"]
        .as_object()
        .expect("execution state")
        .values()
    {
        assert!(
            value == &Value::from(0) || value == &Value::Bool(false) || value.is_null(),
            "initial execution state must remain zero, false, or null"
        );
    }

    assert_eq!(schema["$id"], DOMAIN);
    assert_closed_schema(&schema["$defs"]["exactAuthority"], &declaration);
    let controls = mutations["controls"].as_array().expect("controls");
    assert_eq!(mutations["mutation_count"], MUTATION_COUNT);
    assert_eq!(controls.len(), MUTATION_COUNT);
    assert_eq!(mutations["prior_registry_total"], PRIOR_REGISTRY_TOTAL);
    assert_eq!(mutations["registry_total"], REGISTRY_TOTAL);
    assert_eq!(PRIOR_REGISTRY_TOTAL + MUTATION_COUNT, REGISTRY_TOTAL);
    assert_eq!(mutations["base_declaration_identity"], DECLARATION_IDENTITY);
    for covered in mutations["control_coverage"]
        .as_object()
        .expect("coverage")
        .values()
    {
        assert_eq!(*covered, Value::Bool(true));
    }
    for (index, control) in controls.iter().enumerate() {
        assert_eq!(control["id"], format!("P84-M{:05}", index + 1));
        let mut mutated = declaration.clone();
        apply_mutation(&mut mutated, control);
        assert_ne!(mutated, declaration);
        assert_ne!(schema["$defs"]["exactAuthority"]["const"], mutated);
    }
}

#[test]
fn pulse_84_governance_grants_one_later_call_but_executes_nothing() {
    let root = repo_root();
    let pulse = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-84.md"),
    )
    .expect("Pulse 84 record");
    let authority = fs::read_to_string(root.join(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_84_AUTHORITY.md",
    ))
    .expect("authority record");
    let review = fs::read_to_string(root.join(
        "docs/plans/reviews/PULSE-84-WITNESSED-CAPABILITY-MATERIALIZATION-AUTHORITY-ROLE-REVIEW.md",
    ))
    .expect("role review");
    let validator = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_84_authority.rs"),
    )
    .expect("validator source");

    for required in [
        "authorized and unexecuted",
        "exactly one later independent Pulse 82 callable attempt",
        "non-retryable",
        "performs no custody or execution",
    ] {
        assert!(
            format!("{pulse}\n{authority}\n{review}").contains(required),
            "missing authority boundary: {required}"
        );
    }
    for required in [
        "Completed revisions",
        "Remaining gates",
        "Implementation authority",
        "fake-qualified",
    ] {
        assert!(
            review.contains(required),
            "missing review field: {required}"
        );
    }
    let python_module = ["python", " -B"].concat();
    let python_process = ["Command::new(", "\"python"].concat();
    assert!(!validator.contains(&python_module));
    assert!(!validator.contains(&python_process));
}
