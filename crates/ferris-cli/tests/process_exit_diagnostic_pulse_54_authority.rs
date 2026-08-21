use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "42a16e298c5af55b05df5ceb8e3477d0dd45c814";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-54-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:44420f3496067b0422c4146bd4b51354c72c45f7a2758677cf501a683d702d49";
const MUTATION_COUNT: usize = 13_485;
const PRIOR_REGISTRY_TOTAL: usize = 67_836;
const REGISTRY_TOTAL: usize = 81_321;
const HELD_OUT: &str = "docs/simulations/profile-diff-held-out";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn historical_crlf_sha256(bytes: &[u8]) -> String {
    assert!(!bytes.contains(&b'\r'), "current artifact must be LF-only");
    let historical = bytes
        .iter()
        .flat_map(|byte| {
            if *byte == b'\n' {
                [Some(b'\r'), Some(b'\n')]
            } else {
                [Some(*byte), None]
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    sha256(&historical)
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

fn collect_files(root: &Path, directory: &Path, output: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        assert!(
            !metadata.file_type().is_symlink(),
            "{path:?} must not be a symlink"
        );
        if metadata.is_dir() {
            collect_files(root, &path, output);
        } else {
            assert!(metadata.is_file(), "{path:?} must be a regular file");
            output.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
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
    let exists = git_output(&[
        "cat-file".to_owned(),
        "-e".to_owned(),
        format!("{commit}^{{commit}}"),
    ]);
    assert!(exists.status.success(), "missing release commit {commit}");
    let ancestor = git_output(&[
        "merge-base".to_owned(),
        "--is-ancestor".to_owned(),
        commit.to_owned(),
        CUTOFF.to_owned(),
    ]);
    assert!(
        ancestor.status.success(),
        "{commit} must precede cutoff {CUTOFF}"
    );
}

fn expected_digest(value: &Value) -> &str {
    value
        .as_str()
        .or_else(|| value["sha256"].as_str())
        .expect("bound SHA-256")
}

fn assert_json_envelope(root: &Path, binding: &Value) {
    let path = binding["path"].as_str().expect("envelope path");
    let bytes = fs::read(root.join(path)).expect("read envelope");
    assert_eq!(sha256(&bytes), binding["raw_sha256"], "{path} raw identity");
    let parsed: Value = serde_json::from_slice(&bytes).expect("parse envelope");
    for (field, expected) in binding["identity_fields"]
        .as_object()
        .expect("envelope identity fields")
    {
        assert_eq!(parsed[field], *expected, "{path} {field}");
    }
}

fn assert_release_binding(name: &str, binding: &Value) {
    let root = repo_root();
    match binding["binding_type"].as_str().expect("binding type") {
        "sealed-release" => {
            assert_eq!(
                binding["complete_exact_current_cutoff_tree_required"], true,
                "{name} exact tree required"
            );
            assert_eq!(
                binding["cutoff_tree_raw_file_bindings_required"], true,
                "{name} raw bindings required"
            );
            assert_eq!(
                binding["cutoff_tree_release_path_set_required"], true,
                "{name} path set required"
            );
            assert_eq!(
                binding["sealed_identities_required"], true,
                "{name} sealed identities required"
            );
            assert_commit_precedes_cutoff(
                binding["released_at_commit"]
                    .as_str()
                    .expect("release commit"),
            );
            let release_root = binding["release_root"].as_str().expect("release root");
            let directory = root.join(release_root);
            let expected_paths = binding["exact_release_tree_paths"]
                .as_array()
                .expect("release path set")
                .iter()
                .map(|value| value.as_str().expect("release path").to_owned())
                .collect::<BTreeSet<_>>();
            let mut actual_paths = BTreeSet::new();
            collect_files(&directory, &directory, &mut actual_paths);
            assert_eq!(actual_paths, expected_paths, "{name} exact path set");
            assert_eq!(
                actual_paths.len(),
                binding["release_tree_file_count"]
                    .as_u64()
                    .expect("release tree count") as usize,
                "{name} tree count"
            );
            let hashes = binding["raw_sha256_by_path"]
                .as_object()
                .expect("raw hash map");
            assert_eq!(hashes.len(), actual_paths.len(), "{name} hash count");
            let cutoff_hashes = binding["cutoff_raw_sha256_by_path"]
                .as_object()
                .expect("cutoff hash map");
            assert_eq!(
                cutoff_hashes.len(),
                actual_paths.len(),
                "{name} cutoff hash count"
            );
            for path in actual_paths {
                assert!(!Path::new(&path).is_absolute(), "{name} relative path");
                assert!(
                    !path.split('/').any(|part| part == ".."),
                    "{name} safe path"
                );
                let bytes = fs::read(directory.join(&path)).expect("read release file");
                let current_hash =
                    if name == "pulse_35_public_corpus_materializer" && !path.ends_with(".json") {
                        historical_crlf_sha256(&bytes)
                    } else {
                        sha256(&bytes)
                    };
                assert_eq!(
                    current_hash,
                    expected_digest(hashes.get(&path).expect("path binding")),
                    "{name} current hash {path}"
                );
                assert_eq!(
                    sha256(&cutoff_blob(&format!("{release_root}/{path}"))),
                    expected_digest(cutoff_hashes.get(&path).expect("cutoff path binding")),
                    "{name} cutoff blob hash {path}"
                );
            }
            for envelope in binding["manifest_receipt_seal"]
                .as_object()
                .expect("release envelopes")
                .values()
            {
                assert_json_envelope(&directory, envelope);
            }
            for (path, identity) in binding["supplemental_artifacts"]
                .as_object()
                .expect("supplemental artifacts")
            {
                let bytes = fs::read(root.join(path)).expect("read supplemental artifact");
                assert_eq!(bytes.len() as u64, identity["size"], "{name} {path} size");
                assert_eq!(sha256(&bytes), identity["sha256"], "{name} {path} hash");
                assert_eq!(
                    sha256(&cutoff_blob(path)),
                    identity["cutoff_sha256"],
                    "{name} {path} cutoff"
                );
            }
            if let Some(receipt) = binding.get("normalization_receipt") {
                assert_json_envelope(&directory, receipt);
            }
        }
        "artifact-set" => {
            assert_eq!(
                binding["complete_exact_current_cutoff_artifacts_required"], true,
                "{name} exact artifacts required"
            );
            assert_eq!(
                binding["sealed_identities_required"], true,
                "{name} sealed identities required"
            );
            assert_eq!(
                binding["manifest_receipt_seal"]["status"],
                "not-applicable-public-contract-artifact-set"
            );
            assert_commit_precedes_cutoff(
                binding["released_at_commit"]
                    .as_str()
                    .expect("artifact commit"),
            );
            let artifacts = binding["raw_sha256_by_path"]
                .as_object()
                .expect("artifact hash map");
            let cutoff_hashes = binding["cutoff_raw_sha256_by_path"]
                .as_object()
                .expect("artifact cutoff hash map");
            let expected_paths = binding["exact_artifact_paths"]
                .as_array()
                .expect("artifact paths")
                .iter()
                .map(|value| value.as_str().expect("artifact path").to_owned())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                artifacts.keys().cloned().collect::<BTreeSet<_>>(),
                expected_paths,
                "{name} artifact path set"
            );
            assert_eq!(
                cutoff_hashes.keys().cloned().collect::<BTreeSet<_>>(),
                expected_paths,
                "{name} cutoff artifact path set"
            );
            for (path, identity) in artifacts {
                let bytes = fs::read(root.join(path)).expect("read public contract artifact");
                assert_eq!(bytes.len() as u64, identity["size"], "{name} {path} size");
                assert_eq!(sha256(&bytes), identity["sha256"], "{name} {path} hash");
                assert_eq!(
                    sha256(&cutoff_blob(path)),
                    cutoff_hashes[path],
                    "{name} {path} cutoff"
                );
            }
        }
        other => panic!("unknown binding type {other}"),
    }
}

fn normalized_signature(source: &str, callable: &str) -> (String, String) {
    let marker = format!("def {callable}(");
    let start = source.find(&marker).expect("callable definition");
    let open = start + marker.len() - 1;
    let bytes = source.as_bytes();
    let mut depth = 0_i32;
    let mut close = None;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close.expect("callable header close");
    let header = source[start..=close]
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    let after = &source[close + 1..];
    let arrow = after.find("->").expect("return annotation");
    let terminator = after[arrow + 2..]
        .find(':')
        .expect("return annotation terminator");
    let annotation = after[arrow + 2..arrow + 2 + terminator]
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    (header, annotation)
}

fn assert_api_bindings(declaration: &Value) {
    let api = declaration["api_bindings"]
        .as_object()
        .expect("API bindings");
    assert_eq!(api.len(), 13);
    for (name, binding) in api {
        let source_path = binding["module_path"].as_str().expect("module path");
        let source = fs::read(repo_root().join(source_path)).expect("read API source");
        assert_eq!(
            sha256(&source),
            binding["source_sha256"],
            "{name} source hash"
        );
        let source = String::from_utf8(source).expect("UTF-8 API source");
        let (header, return_annotation) =
            normalized_signature(&source, binding["callable"].as_str().expect("callable"));
        assert_eq!(
            header, binding["normalized_signature"],
            "{name} callable signature"
        );
        assert_eq!(
            return_annotation, binding["return_annotation"],
            "{name} return annotation"
        );
    }

    let p53 = &api["pulse_53"];
    assert_eq!(
        p53["normalized_signature"],
        "defrun_witness_preserving_ordered_executor(repo_root:Path,private_runtime_root:Path,p27_cycle_root:Path,p39_checkout_root:Path,p41_final_root:Path,retained_custodies:Mapping[str,object],)"
    );
    assert_eq!(p53["return_annotation"], "WitnessPreservingOrderedResult");
    assert_eq!(p53["role"], "sole-p54-production-runtime-callable");
}

fn assert_public_identifier(value: &str, forbidden: &BTreeSet<&str>) {
    let bytes = value.as_bytes();
    assert!((1..=48).contains(&bytes.len()), "identifier length");
    assert!(bytes[0].is_ascii_lowercase(), "identifier first character");
    assert!(
        bytes[1..]
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-'),
        "identifier characters"
    );
    assert!(
        !value.split('-').any(|part| forbidden.contains(part)),
        "identifier forbidden token"
    );
}

fn assert_zero_execution_state(value: &Value) {
    for (field, state) in value.as_object().expect("execution state") {
        match state {
            Value::Number(number) => {
                assert_eq!(number.as_i64(), Some(0), "state {field} must be zero")
            }
            Value::Bool(boolean) => assert!(!boolean, "state {field} must be false"),
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
                parent.as_array_mut().expect("mutation array target")
                    [token.parse::<usize>().expect("mutation array index")] =
                    mutation["value"].clone();
            }
        }
        "remove" => {
            if let Some(object) = parent.as_object_mut() {
                assert!(object.remove(&token).is_some(), "remove object member");
            } else {
                parent
                    .as_array_mut()
                    .expect("mutation array target")
                    .remove(token.parse::<usize>().expect("mutation array index"));
            }
        }
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

#[test]
fn pulse_54_authority_is_closed_self_excluding_and_unexecuted() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-54-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-54-authority.v1.schema.json"),
    );

    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&declaration), DECLARATION_IDENTITY);
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["$ref"], "#/$defs/exactAuthority");
    assert_closed_schema(&schema["$defs"]["exactAuthority"], &declaration);

    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["current_cutoff_identity_is_self_excluding"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["authority_present_at_cutoff"],
        false
    );
    for path in [
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-54-authority.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-54-authority-mutations.json",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-54-authority.v1.schema.json",
    ] {
        assert_cutoff_absent(path);
    }
    assert_zero_execution_state(&declaration["execution_state"]);

    for (pulse, expected_disposition, withdrawn) in [
        ("pulse_48", "invalid-publication-integrity", false),
        ("pulse_49", "invalid-prelaunch-authority-integrity", true),
        (
            "pulse_50",
            "invalid-prelaunch-infrastructure-integrity",
            true,
        ),
    ] {
        let predecessor = &declaration["closed_predecessors"][pulse];
        assert_eq!(predecessor["disposition"], expected_disposition);
        assert_eq!(predecessor["permanently_closed"], true);
        assert_eq!(predecessor["non_retryable"], true);
        assert_eq!(predecessor["further_launches_prohibited"], true);
        assert_eq!(predecessor["withdrawn_as_recorded"], withdrawn);
        assert_eq!(predecessor["category_conclusion"], Value::Null);
        assert_eq!(predecessor["diagnostic_conclusion"], Value::Null);
        assert_eq!(predecessor["product_conclusion"], Value::Null);
        for field in [
            "retry",
            "resume",
            "reconstruction",
            "reseed",
            "reuse",
            "inference",
        ] {
            assert_eq!(predecessor[field], false, "{pulse} {field}");
        }
    }
    for field in [
        "retry",
        "resume",
        "reconstruction",
        "reseed",
        "reuse",
        "correlation",
        "inference",
        "same_authority",
        "same_cutoff",
        "same_execution",
    ] {
        assert_eq!(
            declaration["closed_predecessors"]["relationship_to_pulses_48_49_50"][field], false,
            "Pulse 54 relationship {field}"
        );
    }
}

#[test]
fn pulse_54_binds_complete_release_chain_and_apis_without_execution() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-54-authority.json"));
    let bindings = declaration["public_release_bindings"]
        .as_object()
        .expect("release bindings");
    let expected = BTreeSet::from([
        "pulse_27_exact_two_pair_adapter",
        "pulse_31_public_input_contract",
        "pulse_33_public_build_freeze",
        "pulse_35_public_corpus_materializer",
        "pulse_37_checkout_normalization",
        "pulse_39_checkout_verifier",
        "pulse_41_transactional_copy",
        "pulse_43_ordered_result_publisher",
        "pulse_44_retained_binary_custody",
        "pulse_45_binary_custody_event_bridge",
        "pulse_47_publication_outcome_witness",
        "pulse_51_diagnostic_executor",
        "pulse_52_ordered_materialization_executor",
        "pulse_53_witness_preserving_ordered_executor",
    ]);
    assert_eq!(
        bindings.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        expected
    );
    for (name, binding) in bindings {
        assert_release_binding(name, binding);
    }
    assert_eq!(
        bindings["pulse_51_diagnostic_executor"]["released_at_commit"],
        "d09c923c1e2cd2be003026597f4ad2a0e2d3764f"
    );
    assert_eq!(
        bindings["pulse_52_ordered_materialization_executor"]["released_at_commit"],
        "e4ef9617f227670f3911be42ca63df4b2e66d24f"
    );
    assert_eq!(
        bindings["pulse_53_witness_preserving_ordered_executor"]["released_at_commit"],
        CUTOFF
    );
    assert_eq!(
        bindings["pulse_37_checkout_normalization"]["normalization_receipt"]["raw_sha256"],
        "sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6"
    );
    assert_eq!(
        bindings["pulse_35_public_corpus_materializer"]["checkout_variant_policy"]
            ["fresh_core_autocrlf_false_canonical_cutoff_required"],
        true
    );
    assert_eq!(
        bindings["pulse_35_public_corpus_materializer"]["checkout_variant_policy"]
            ["pulse_37_normalization_binding_required"],
        true
    );
    assert_api_bindings(&declaration);
}

#[test]
fn pulse_54_authorizes_one_injection_free_p53_route_after_exact_public_gates() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-54-authority.json"));
    let catalog = &declaration["p43_public_catalog"];
    let expected_gates = serde_json::json!([
        "pulse-41-pulse-39-public-custody",
        "windows-retained-binary-custody",
        "ubuntu-retained-binary-custody",
        "exact-adapter-preflight",
        "pulse-31-public-input",
        "pulse-35-pulse-37-normalization",
        "bounded-materialization",
        "bounded-process-exit-search",
    ]);
    assert_eq!(catalog["schema"], "ferris.pulse-43-ordered-gate-catalog/v1");
    assert_eq!(catalog["gate_ids"], expected_gates);
    assert_eq!(
        catalog["precall_validation_ids"],
        serde_json::json!(["public-catalog-prevalidation", "public-input-contract"])
    );
    assert_eq!(
        catalog["forbidden_standalone_tokens"],
        serde_json::json!([
            "candidate",
            "corpus",
            "credential",
            "home",
            "password",
            "private",
            "seed",
            "secret",
            "token",
            "user",
            "workspace",
        ])
    );
    let forbidden = catalog["forbidden_standalone_tokens"]
        .as_array()
        .expect("forbidden tokens")
        .iter()
        .map(|value| value.as_str().expect("forbidden token"))
        .collect::<BTreeSet<_>>();
    for identifier in catalog["gate_ids"]
        .as_array()
        .expect("gate identifiers")
        .iter()
        .chain(
            catalog["precall_validation_ids"]
                .as_array()
                .expect("validation identifiers"),
        )
    {
        assert_public_identifier(identifier.as_str().expect("public identifier"), &forbidden);
    }
    assert_eq!(
        catalog["prevalidate_all_gate_and_validation_ids_before_callable"],
        true
    );

    let pre_call = &declaration["pre_call_public_prerequisites"];
    assert_eq!(pre_call["cutoff_checkouts"]["revision"], CUTOFF);
    assert_eq!(pre_call["cutoff_checkouts"]["anonymous"], true);
    assert_eq!(pre_call["cutoff_checkouts"]["fresh"], true);
    assert_eq!(pre_call["cutoff_checkouts"]["core_autocrlf"], false);
    assert_eq!(
        pre_call["cutoff_checkouts"]["platforms"],
        serde_json::json!(["windows-x86_64", "ubuntu-24.04-x86_64"])
    );
    assert_eq!(
        pre_call["artifact_construction_prohibited_before_callable"],
        serde_json::json!({
            "candidate_processes": false,
            "descriptor_root": false,
            "private_seed": false
        })
    );
    assert_eq!(
        pre_call["p44_custody"]["roots_and_complete_summaries_supplied_to_p53"],
        true
    );
    assert_eq!(pre_call["p44_custody"]["operation_per_platform_exact"], 1);
    assert_eq!(
        pre_call["fresh_roots"],
        serde_json::json!({
            "p27_cycle_root": "fresh",
            "p39_checkout_root": "fresh",
            "p41_final_copy_root": "fresh-absent",
            "runtime_root": "fresh"
        })
    );
    let windows = &pre_call["p33_build_freeze"]["windows"];
    assert_eq!(
        windows["cargo_version"],
        "cargo 1.95.0 (f2d3ce0bd 2026-03-21)"
    );
    assert_eq!(
        windows["rustc_version"],
        "rustc 1.95.0 (59807616e 2026-04-14)"
    );
    assert_eq!(
        windows["reproducibility_control"],
        "RUSTFLAGS=-C link-arg=/Brepro"
    );
    assert_eq!(windows["artifact_size"], 1_436_672);
    assert_eq!(
        windows["artifact_sha256"],
        "sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8"
    );
    let ubuntu = &pre_call["p33_build_freeze"]["ubuntu"];
    assert_eq!(ubuntu["wsl_distribution"], "Ubuntu-24.04");
    assert_eq!(
        ubuntu["cargo_version"],
        "cargo 1.97.1 (c980f4866 2026-06-30)"
    );
    assert_eq!(
        ubuntu["rustc_version"],
        "rustc 1.97.1 (8bab26f4f 2026-07-14)"
    );
    assert_eq!(ubuntu["artifact_size"], 1_945_448);
    assert_eq!(
        ubuntu["artifact_sha256"],
        "sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4"
    );

    let runtime = &declaration["runtime_binding"];
    assert_eq!(
        runtime["sole_callable"],
        "run_witness_preserving_ordered_executor"
    );
    assert_eq!(runtime["module"], "witness_preserving_ordered_executor");
    assert_eq!(
        runtime["accepted_input_names"],
        serde_json::json!([
            "repo_root",
            "private_runtime_root",
            "p27_cycle_root",
            "p39_checkout_root",
            "p41_final_root",
            "retained_custodies",
        ])
    );
    assert_eq!(runtime["sole_callable_invocations_exact"], 1);
    assert_eq!(runtime["no_runtime_injection"], true);
    assert_eq!(
        runtime["prohibited_direct_callables"],
        serde_json::json!([
            "pulse_43.publish_result",
            "pulse_47.witness_pulse_43",
            "pulse_51.run_diagnostic_executor",
            "pulse_52.run_ordered_materialization_executor",
        ])
    );
    assert_eq!(
        declaration["authority"]["authority_consumed_by_pre_call_public_preparation"],
        false
    );
    assert_eq!(
        declaration["authority"]["authority_consumed_on_sole_callable_invocation_attempt"],
        true
    );

    let topology = &declaration["topology"];
    assert_eq!(topology["case_dispositions_per_platform"], 70);
    assert_eq!(topology["launch_ready_os_processes_per_platform"], 69);
    assert_eq!(topology["no_launch_dispositions_per_platform"], 1);
    assert_eq!(topology["case_dispositions_total"], 140);
    assert_eq!(topology["os_processes_total"], 138);
    assert_eq!(topology["no_launch_dispositions_total"], 2);
    assert_eq!(topology["first_semantic_projection_mismatch_stops"], true);
    assert_eq!(
        topology["ordinal_70_no_launch_must_not_start_process"],
        true
    );

    assert_eq!(
        declaration["execution_order"]["ordered_steps"],
        serde_json::json!([
            "p39-p41-public-custody",
            "windows-p44-p45",
            "ubuntu-p44-p45",
            "p27-exact-callable",
            "p31-exact-contract",
            "p35-p37-custody",
            "csprng-32-byte-seed",
            "p35-materializer-once",
            "p35-verifier-once",
            "windows-70-dispositions-69-os-processes-1-no-launch",
            "ubuntu-70-dispositions-69-os-processes-1-no-launch",
            "first-semantic-projection-mismatch-stop",
            "one-terminal-p47-to-p43",
        ])
    );
}

#[test]
fn pulse_54_terminal_transfer_is_path_safe_and_result_artifacts_are_absent() {
    let root = repo_root();
    let held_out = root.join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-54-authority.json"));
    let transfer = &declaration["terminal_transfer_contract"];
    let published = &transfer["published_result"];
    assert_eq!(published["p43_result_exact_file_count"], 2);
    assert_eq!(published["p47_witness_exact_file_count"], 2);
    assert_eq!(
        published["transfer"],
        "copy-exact-verified-p43-result-and-p47-witness-trees"
    );
    assert_eq!(published["verify_both_copied_trees"], true);
    let failure = &transfer["published_failure_witness"];
    assert_eq!(failure["p43_result_path_must_remain_absent"], true);
    assert_eq!(failure["p47_witness_exact_file_count"], 2);
    assert_eq!(
        failure["transfer"],
        "copy-exact-verified-p47-two-file-witness-tree-only"
    );
    assert_eq!(failure["permanent_publication_integrity_closeout"], true);
    let invalid = &transfer["cleanup_indeterminate_or_invalid_witness_publication"];
    assert_eq!(invalid["public_success_claim"], false);
    assert_eq!(invalid["transfer_permitted"], false);
    assert_eq!(
        invalid["permanent_disposition"],
        "invalid-publication-integrity"
    );
    assert_eq!(transfer["private_record_or_root_path_disclosure"], false);
    assert_eq!(transfer["source_paths_never_publicly_serialized"], true);
    for conclusion in [
        &published["conclusions"],
        &failure["conclusions"],
        &invalid["conclusions"],
    ] {
        assert_eq!(conclusion["category"], Value::Null);
        assert_eq!(conclusion["diagnostic"], Value::Null);
        assert_eq!(conclusion["product"], Value::Null);
    }

    let result_path = published["p43_destination"]
        .as_str()
        .expect("public result destination")
        .trim_end_matches('/');
    let witness_path = published["p47_destination"]
        .as_str()
        .expect("publication witness destination")
        .trim_end_matches('/');
    assert_eq!(
        witness_path,
        failure["destination"]
            .as_str()
            .expect("failure destination")
            .trim_end_matches('/')
    );
    assert!(
        !root.join(result_path).exists(),
        "no Pulse 54 result artifact"
    );
    assert!(
        !root.join(witness_path).exists(),
        "no Pulse 54 witness artifact"
    );
}

#[test]
fn pulse_54_mutation_controls_reject_every_authority_change() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-54-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-54-authority.v1.schema.json"),
    );
    let mutations = read_json(
        held_out.join("fixtures/process-exit-diagnostic-pulse-54-authority-mutations.json"),
    );
    let controls = mutations["controls"].as_array().expect("mutation controls");

    assert_eq!(mutations["authority_schema"], DOMAIN);
    assert_eq!(mutations["base_declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(mutations["mutation_count"], MUTATION_COUNT);
    assert_eq!(controls.len(), MUTATION_COUNT);
    assert!(MUTATION_COUNT > 9_862);
    assert_eq!(mutations["prior_registry_total"], PRIOR_REGISTRY_TOTAL);
    assert_eq!(mutations["registry_total"], REGISTRY_TOTAL);
    assert_eq!(PRIOR_REGISTRY_TOTAL + MUTATION_COUNT, REGISTRY_TOTAL);
    assert_eq!(
        mutations["mutation_model"]["scalar_replacements_per_leaf"],
        10
    );
    for covered in mutations["control_coverage"]
        .as_object()
        .expect("control coverage")
        .values()
    {
        assert_eq!(covered, &Value::Bool(true));
    }
    assert_eq!(schema["$defs"]["exactAuthority"]["const"], declaration);

    let mut ids = BTreeSet::new();
    for (index, control) in controls.iter().enumerate() {
        let id = control["id"].as_str().expect("control ID");
        assert_eq!(id, format!("P54-M{:05}", index + 1));
        assert!(ids.insert(id.to_owned()), "unique mutation ID");
        assert_eq!(
            control["expected_rejection"],
            "closed-schema-and-declaration-identity"
        );
        let mut changed = declaration.clone();
        apply_mutation(&mut changed, control);
        assert_ne!(changed, declaration, "mutation {id} changes authority");
        assert_ne!(
            changed, schema["$defs"]["exactAuthority"]["const"],
            "closed schema rejects {id}"
        );
    }
}
