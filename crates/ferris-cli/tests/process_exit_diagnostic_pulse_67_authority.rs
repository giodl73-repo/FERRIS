use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "3ec6a36009fd34765508f729e795042fd610e5d4";
const PULSE_59_HEAD: &str = "6945f5fc96868c97267a1635fbb5219cc398eeb4";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-67-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05";
const MUTATION_COUNT: usize = 28_196;
const PRIOR_REGISTRY_TOTAL: usize = 262_306;
const REGISTRY_TOTAL: usize = 290_502;
const HELD_OUT: &str = "docs/simulations/profile-diff-held-out";
const P67_PROBE_WORKER: &str =
    "docs/simulations/profile-diff-held-out/fixtures/p67_wsl_probe_worker.py";
const P67_PROBE_WORKER_SHA256: &str =
    "sha256:b2082b56a5eb3fca7e0c23c5e63099837d3fe64d261cead63410c4f5f5ae91e5";
const P67_PROBE_WORKER_SIZE: u64 = 12_853;
const P67_PROBE_DEPENDENCY: &str =
    "docs/simulations/profile-diff-held-out/fixtures/p67_wsl_probe_sealed_dependencies.py";
const P67_PROBE_DEPENDENCY_SHA256: &str =
    "sha256:317d11a7468647dfafef26db5079a0422ca39cc8228cf389ebc6949be4a4236f";
const P67_PROBE_DEPENDENCY_SIZE: u64 = 5_842;
const P67_PROBE_PROTOCOL_SCHEMA: &str =
    "docs/simulations/profile-diff-held-out/schemas/ferris.pulse-67-wsl-probe-session.v1.schema.json";
const P67_PROBE_PROTOCOL_SCHEMA_SHA256: &str =
    "sha256:76761ec589149369b0dba590382d3d826dedec15da3f3132cb83fd51b512c78d";
const P67_PROBE_PROTOCOL_SCHEMA_SIZE: u64 = 3_863;
const P57_WORKER_SOURCE: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py";
const P57_WORKER_SHA256: &str =
    "sha256:9b0d91f7c4e2aed57d7dc40b95f5860f017138717364d3399d132884047904cb";
const P57_SEALED_DEPENDENCIES_SOURCE: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py";
const P57_SEALED_DEPENDENCIES_SHA256: &str =
    "sha256:fe36a56a10d5d3659fae9cfacc3cd48075aaf0e3327ae029a2470d1107da6c8d";

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

fn assert_contains_all(text: &str, snippets: &[&str], label: &str) {
    for snippet in snippets {
        assert!(
            text.contains(snippet),
            "{label} missing expected snippet: {snippet}"
        );
    }
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
            let actual_required = schema["required"]
                .as_array()
                .expect("required array")
                .iter()
                .map(|entry| entry.as_str().expect("required key").to_owned())
                .collect::<BTreeSet<_>>();
            let expected_required = object.keys().cloned().collect::<BTreeSet<_>>();
            assert_eq!(
                actual_required, expected_required,
                "closed schema required keys"
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

fn cutoff_tree_paths(release_root: &str) -> BTreeSet<String> {
    let output = git_output(&[
        "ls-tree".to_owned(),
        "-r".to_owned(),
        "--name-only".to_owned(),
        CUTOFF.to_owned(),
        "--".to_owned(),
        release_root.to_owned(),
    ]);
    assert!(output.status.success(), "list cutoff release tree");
    let prefix = format!("{release_root}/");
    String::from_utf8(output.stdout)
        .expect("UTF-8 Git paths")
        .lines()
        .map(|path| {
            path.strip_prefix(&prefix)
                .expect("release-relative cutoff path")
                .to_owned()
        })
        .collect()
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

fn assert_canonical_identity(identity: &Value, bytes: &[u8], label: &str) {
    assert_eq!(
        identity["source"], "immutable-git-blob-at-cutoff",
        "{label} source"
    );
    assert_eq!(identity["size"], bytes.len() as u64, "{label} size");
    assert_eq!(identity["sha256"], sha256(bytes), "{label} hash");
}

fn materialization(bytes: &[u8]) -> Value {
    let cr = bytes.iter().filter(|byte| **byte == b'\r').count();
    let lf = bytes.iter().filter(|byte| **byte == b'\n').count();
    let framing = if cr == 0 && bytes.ends_with(b"\n") {
        "lf-only-terminated"
    } else if cr == lf
        && cr > 0
        && bytes.ends_with(b"\r\n")
        && !bytes
            .windows(2)
            .enumerate()
            .any(|(index, window)| window == b"\n" && (index == 0 || bytes[index - 1] != b'\r'))
    {
        "crlf-only-terminated"
    } else {
        "exact-binary-or-nontext-framing"
    };
    serde_json::json!({
        "cr_bytes": cr,
        "lf_bytes": lf,
        "newline_framing": framing,
        "sha256": sha256(bytes),
        "size": bytes.len(),
    })
}

fn assert_working_materialization(
    path: &Path,
    canonical: &Value,
    authorized: Option<&Value>,
    label: &str,
) {
    let metadata = fs::symlink_metadata(path).expect("working-tree metadata");
    assert!(
        metadata.file_type().is_file(),
        "{label} must be a regular file"
    );
    assert!(
        !metadata.file_type().is_symlink(),
        "{label} must not be symlinked"
    );
    let actual = materialization(&fs::read(path).expect("working-tree bytes"));
    if let Some(authorized) = authorized {
        let variants = authorized.as_array().expect("authorized variants");
        assert!(!variants.is_empty(), "{label} explicit variants");
        let canonical_seen = variants.iter().any(|variant| {
            variant["sha256"] == canonical["sha256"] && variant["size"] == canonical["size"]
        });
        assert!(canonical_seen, "{label} canonical materialization listed");
        let exact = variants.iter().any(|variant| {
            variant["sha256"] == actual["sha256"]
                && variant["size"] == actual["size"]
                && variant["cr_bytes"] == actual["cr_bytes"]
                && variant["lf_bytes"] == actual["lf_bytes"]
                && variant["newline_framing"] == actual["newline_framing"]
        });
        assert!(
            exact,
            "{label} must use an explicitly sealed materialization"
        );
    } else {
        assert_eq!(
            actual["sha256"], canonical["sha256"],
            "{label} canonical hash"
        );
        assert_eq!(actual["size"], canonical["size"], "{label} canonical size");
    }
}

fn assert_cutoff_json_envelope(relative: &str, binding: &Value) {
    let bytes = cutoff_blob(relative);
    assert_eq!(
        sha256(&bytes),
        binding["canonical_sha256"],
        "{relative} identity"
    );
    let parsed: Value = serde_json::from_slice(&bytes).expect("parse cutoff envelope");
    for (field, expected) in binding["identity_fields"]
        .as_object()
        .expect("envelope identity fields")
    {
        assert_eq!(parsed[field], *expected, "{relative} {field}");
    }
}

fn assert_release_binding(name: &str, binding: &Value) {
    assert_eq!(
        binding["cutoff_blob_identities_required"], true,
        "{name} blob only"
    );
    assert_eq!(
        binding["working_tree_materialization_validation_required"], true,
        "{name} materialization validation"
    );
    assert_eq!(binding["sealed_identities_required"], true, "{name} sealed");
    assert_commit_precedes_cutoff(
        binding["released_at_commit"]
            .as_str()
            .expect("release commit"),
    );
    let root = repo_root();
    match binding["binding_type"].as_str().expect("binding type") {
        "sealed-release" => {
            assert_eq!(binding["complete_exact_cutoff_tree_required"], true);
            assert_eq!(binding["cutoff_tree_release_path_set_required"], true);
            let release_root = binding["release_root"].as_str().expect("release root");
            let expected_paths = binding["exact_release_tree_paths"]
                .as_array()
                .expect("release paths")
                .iter()
                .map(|value| value.as_str().expect("release path").to_owned())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                cutoff_tree_paths(release_root),
                expected_paths,
                "{name} cutoff paths"
            );
            let mut working_paths = BTreeSet::new();
            collect_files(
                &root.join(release_root),
                &root.join(release_root),
                &mut working_paths,
            );
            assert_eq!(working_paths, expected_paths, "{name} working paths");
            assert_eq!(
                binding["release_tree_file_count"],
                expected_paths.len() as u64,
                "{name} release count"
            );
            let identities = binding["canonical_identity_by_path"]
                .as_object()
                .expect("canonical identities");
            assert_eq!(
                identities.keys().cloned().collect::<BTreeSet<_>>(),
                expected_paths
            );
            let variants = binding.get("authorized_working_tree_identities_by_path");
            for path in &working_paths {
                let cutoff_path = format!("{release_root}/{path}");
                let canonical = identities.get(path).expect("canonical release identity");
                assert_canonical_identity(canonical, &cutoff_blob(&cutoff_path), &cutoff_path);
                assert_working_materialization(
                    &root.join(release_root).join(path),
                    canonical,
                    variants.and_then(|values| values.get(path)),
                    &cutoff_path,
                );
            }
            for envelope in binding["manifest_receipt_seal"]
                .as_object()
                .expect("release envelopes")
                .values()
            {
                assert_cutoff_json_envelope(
                    &format!(
                        "{release_root}/{}",
                        envelope["path"].as_str().expect("envelope path")
                    ),
                    envelope,
                );
            }
            if let Some(receipt) = binding.get("normalization_receipt") {
                assert_cutoff_json_envelope(
                    &format!(
                        "{release_root}/{}",
                        receipt["path"].as_str().expect("receipt path")
                    ),
                    receipt,
                );
            }
            let supplemental_variants =
                binding.get("authorized_supplemental_working_tree_identities_by_path");
            for (path, identity) in binding["supplemental_artifacts"]
                .as_object()
                .expect("supplemental artifacts")
            {
                assert_canonical_identity(identity, &cutoff_blob(path), path);
                assert_working_materialization(
                    &root.join(path),
                    identity,
                    supplemental_variants.and_then(|values| values.get(path)),
                    path,
                );
            }
        }
        "artifact-set" => {
            assert_eq!(binding["complete_exact_cutoff_artifacts_required"], true);
            let expected_paths = binding["exact_artifact_paths"]
                .as_array()
                .expect("artifact paths")
                .iter()
                .map(|value| value.as_str().expect("artifact path").to_owned())
                .collect::<BTreeSet<_>>();
            let identities = binding["canonical_identity_by_path"]
                .as_object()
                .expect("artifact identities");
            assert_eq!(
                identities.keys().cloned().collect::<BTreeSet<_>>(),
                expected_paths
            );
            for path in expected_paths {
                let identity = identities.get(&path).expect("artifact identity");
                assert_canonical_identity(identity, &cutoff_blob(&path), &path);
                assert_working_materialization(&root.join(&path), identity, None, &path);
            }
            assert_eq!(
                binding["manifest_receipt_seal"]["status"],
                "not-applicable-public-contract-artifact-set"
            );
        }
        other => panic!("unknown binding type {other}"),
    }
    let legacy_raw_map = ["raw", "sha256", "by", "path"].join("_");
    let legacy_cutoff_map = ["cutoff", "raw", "sha256", "by", "path"].join("_");
    assert!(
        binding.get(&legacy_raw_map).is_none(),
        "{name} rejects P54 local raw map"
    );
    assert!(
        binding.get(&legacy_cutoff_map).is_none(),
        "{name} uses canonical blob map"
    );
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
    let defined_callables = api
        .values()
        .map(|binding| binding["callable"].as_str().expect("callable").to_owned())
        .collect::<BTreeSet<_>>();
    for (name, binding) in api {
        let path = binding["module_path"].as_str().expect("module path");
        let cutoff = cutoff_blob(path);
        assert_canonical_identity(&binding["canonical_source_identity"], &cutoff, path);
        assert_working_materialization(
            &repo_root().join(path),
            &binding["canonical_source_identity"],
            binding.get("authorized_working_tree_identities"),
            path,
        );
        let source = String::from_utf8(cutoff).expect("UTF-8 cutoff API source");
        let (header, return_annotation) =
            normalized_signature(&source, binding["callable"].as_str().expect("callable"));
        assert_eq!(header, binding["normalized_signature"], "{name} signature");
        assert_eq!(
            return_annotation, binding["return_annotation"],
            "{name} return"
        );
        assert!(
            binding.get("source_sha256").is_none(),
            "{name} has no working-tree source hash"
        );
    }
    assert!(
        defined_callables.contains(
            declaration["runtime_binding"]["sole_callable"]
                .as_str()
                .expect("sole callable")
        )
    );
    assert!(!defined_callables.contains("qualify_exact_p57_wsl_bootstrap_contract"));
    let p56 = &api["pulse_56"];
    assert_eq!(
        p56["role"],
        "inside-p59-exact-stack-capability-build-custody-per-platform"
    );
    assert_eq!(p56["return_annotation"], "CustodyHandle");
    assert_eq!(
        p56["normalized_signature"],
        "defpublish_retained_build_and_custody(platform:str,runtime_parent:str|os.PathLike[str])"
    );
    let p57 = &api["pulse_57"];
    assert_eq!(
        p57["role"],
        "inside-p58-exact-stack-after-public-custody-before-terminalization"
    );
    assert_eq!(p57["return_annotation"], "ExecutorResult");
    let p58 = &api["pulse_58"];
    assert_eq!(p58["role"], "inside-p59-exact-stack-before-terminalization");
    assert_eq!(
        p58["return_annotation"],
        "OrderedCapabilityMaterializationResult"
    );
    assert_eq!(
        p58["normalized_signature"],
        "defrun_ordered_capability_materialization_executor(repo_root:Path,private_runtime_root:Path,p27_cycle_root:Path,p39_checkout_root:Path,p41_final_root:Path,ubuntu_runtime_parent:str,)"
    );
    let p59 = &api["pulse_59"];
    assert_eq!(p59["role"], "sole-p67-production-runtime-callable");
    assert_eq!(
        p59["return_annotation"],
        "WitnessPreservingCapabilityMaterializationResult"
    );
    assert_eq!(
        p59["normalized_signature"],
        "defrun_witness_preserving_capability_materialization_executor(repo_root:Path,private_runtime_root:Path,p27_cycle_root:Path,p39_checkout_root:Path,p41_final_root:Path,ubuntu_runtime_parent:str,)"
    );
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

fn apply_object_mutation(
    object: &mut serde_json::Map<String, Value>,
    key: String,
    mutation: &Value,
) {
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            object.insert(key, mutation["value"].clone());
        }
        "remove" => {
            assert!(object.remove(&key).is_some(), "remove object member");
        }
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

fn apply_array_mutation(array: &mut Vec<Value>, index: usize, mutation: &Value) {
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            array[index] = mutation["value"].clone();
        }
        "remove" => {
            array.remove(index);
        }
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

fn try_apply_mutation(value: &mut Value, segments: &[String], mutation: &Value) -> bool {
    if segments.is_empty() {
        return false;
    }
    if let Some(object) = value.as_object_mut() {
        for split in 1..=segments.len() {
            let key = segments[..split].join("/");
            if split < segments.len() {
                if let Some(child) = object.get_mut(&key) {
                    if try_apply_mutation(child, &segments[split..], mutation) {
                        return true;
                    }
                }
                continue;
            }
            apply_object_mutation(object, key, mutation);
            return true;
        }
        return false;
    }
    if let Some(array) = value.as_array_mut() {
        let index = segments[0].parse::<usize>().expect("mutation array index");
        if segments.len() == 1 {
            apply_array_mutation(array, index, mutation);
            return true;
        }
        return try_apply_mutation(
            array.get_mut(index).expect("mutation array target"),
            &segments[1..],
            mutation,
        );
    }
    false
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    let segments = pointer
        .split('/')
        .skip(1)
        .map(unescape_token)
        .collect::<Vec<_>>();
    assert!(
        try_apply_mutation(value, &segments, mutation),
        "mutation target parent"
    );
}


#[test]
fn pulse_67_binds_historical_cutoff_probe_artifacts_and_zero_execution() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-67-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json"),
    );
    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(
        declaration["program_id"],
        "FERRIS-P67-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-WITNESS-PRESERVING-CAPABILITY-MATERIALIZATION-AUTHORITY"
    );
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
        declaration["immutable_ferris"]["pulse_59_head"],
        PULSE_59_HEAD
    );
    for field in [
        "checkout_materialization_independent_validator_required",
        "complete_exact_current_cutoff_release_trees_required",
        "core_autocrlf_false_fixed_before_checkout",
        "current_cutoff_identities_derived_only_from_immutable_git_blobs",
        "current_cutoff_identity_is_self_excluding",
        "fresh_anonymous_exact_cutoff_checkouts_required",
        "independent_commit_verification_required",
        "public_artifacts_read_only_cutoff_checkout_required",
        "pulse_61_withdrawal_present_at_cutoff",
        "pulse_62_withdrawal_present_at_cutoff",
        "pulse_63_withdrawal_present_at_cutoff",
        "pulse_64_withdrawal_present_at_cutoff",
        "pulse_65_withdrawal_present_at_cutoff",
        "pulse_66_withdrawal_present_at_cutoff",
        "pulse_67_probe_worker_present_at_cutoff",
        "pulse_67_probe_dependency_present_at_cutoff",
        "pulse_67_probe_protocol_schema_present_at_cutoff",
        "supported_complete_file_crlf_lf_variants_only",
        "working_tree_materialization_is_not_identity_source",
    ] {
        assert_eq!(declaration["immutable_ferris"][field], true, "{field}");
    }
    assert_eq!(declaration["immutable_ferris"]["authority_present_at_cutoff"], false);
    for path in [
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority-mutations.json",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_67_AUTHORITY.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-67.md",
        "crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_authority.rs",
    ] {
        assert_cutoff_absent(path);
    }
    let p66_wave = String::from_utf8(cutoff_blob(
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-66.md",
    ))
    .expect("UTF-8 Pulse 66 wave");
    assert_contains_all(
        &p66_wave,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-wsl-probe-bundle-contract",
            "P66-WORKER-HASH-BUNDLE-LIFETIME",
        ],
        "Pulse 66 wave at cutoff",
    );
    let p66_record = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_66_AUTHORITY.md",
    ))
    .expect("UTF-8 Pulse 66 closeout");
    assert_contains_all(
        &p66_record,
        &[
            "Pulse 66 is now permanently withdrawn before launch",
            "P66-WORKER-HASH-BUNDLE-LIFETIME",
        ],
        "Pulse 66 closeout at cutoff",
    );

    let probe_bundle = &declaration["pre_call_public_prerequisites"]["exact_wsl_route_preflight"]
        ["worker_bootstrap_route"]["probe_bundle"];
    for (path, size, digest, field) in [
        (P67_PROBE_WORKER, P67_PROBE_WORKER_SIZE, P67_PROBE_WORKER_SHA256, "probe_worker_source_cutoff_identity"),
        (P67_PROBE_DEPENDENCY, P67_PROBE_DEPENDENCY_SIZE, P67_PROBE_DEPENDENCY_SHA256, "probe_dependency_source_cutoff_identity"),
        (P67_PROBE_PROTOCOL_SCHEMA, P67_PROBE_PROTOCOL_SCHEMA_SIZE, P67_PROBE_PROTOCOL_SCHEMA_SHA256, "protocol_schema_cutoff_identity"),
    ] {
        let bytes = cutoff_blob(path);
        assert_eq!(bytes.len() as u64, size, "{path} size");
        assert_eq!(sha256(&bytes), digest, "{path} digest");
        assert_eq!(probe_bundle[field]["sha256"], digest, "{path} binding digest");
        assert_eq!(probe_bundle[field]["size"], size, "{path} binding size");
    }

    let root = repo_root();
    let pulse = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-67.md"),
    )
    .expect("Pulse 67 wave record");
    let record = fs::read_to_string(
        root.join(
            "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_67_AUTHORITY.md",
        ),
    )
    .expect("Pulse 67 authority record");
    assert_contains_all(
        &pulse,
        &[
            "Status: Permanently withdrawn invalid-prelaunch-cutoff-probe-claim-contract",
            "P67-ROOT-CUTOFF-P56-LOADER-CONTRACT",
            CUTOFF,
            "authority_checkout_root.revision",
            "load_exact_p56(repo_root)",
            "28196",
        ],
        "Pulse 67 wave record",
    );
    assert_contains_all(
        &record,
        &[
            "Pulse 67 is now permanently withdrawn before launch",
            "P67-ROOT-CUTOFF-P56-LOADER-CONTRACT",
            CUTOFF,
            "repo_root = p56_root.parents[3]",
            "Path(p56.__file__).parent == p56_root",
            "28196",
        ],
        "Pulse 67 authority record",
    );
    assert_zero_execution_state(&declaration["execution_state"]);
}

#[test]
fn pulse_67_binds_cutoff_blobs_supported_variants_and_exact_p59_surface() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration_text = String::from_utf8(read_lf(
        held_out.join("fixtures/process-exit-diagnostic-pulse-67-authority.json"),
    ))
    .expect("UTF-8 declaration");
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-67-authority.json"));
    assert!(!declaration_text.contains("%SystemRoot%"));
    assert!(!declaration_text.contains("qualify_exact_p57_wsl_bootstrap_contract"));
    assert!(!declaration_text.contains("fake_dependency_scope"));
    assert!(!declaration_text.contains("host_side_prestaging_only"));
    assert!(!declaration_text.contains("cleanup_same_invocation_required"));
    let bindings = declaration["public_release_bindings"]
        .as_object()
        .expect("bindings");
    assert_eq!(bindings.len(), 14);
    for (name, binding) in bindings {
        assert_release_binding(name, binding);
    }
    assert_eq!(
        bindings["pulse_56_retained_build_custody"]["released_at_commit"],
        "f2f32f70adfae0d6041381b6a4b66afa3fea3060"
    );
    assert_eq!(
        bindings["pulse_57_capability_bound_diagnostic_executor"]["released_at_commit"],
        "8091eaa1e6cdbfd9eb44a344d0b746ace7f0099b"
    );
    assert_eq!(
        bindings["pulse_58_ordered_capability_materialization_executor"]["released_at_commit"],
        "7c66d70800edd06642274ed4f2e4aee224b7583e"
    );
    assert_eq!(
        bindings["pulse_59_witness_preserving_capability_materialization_executor"]["released_at_commit"],
        PULSE_59_HEAD
    );
    assert_api_bindings(&declaration);
}

#[test]
fn pulse_67_records_withdrawn_pulse_66_and_historical_probe_worker_preflight_contract() {
    let declaration = read_json(
        repo_root().join("docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json"),
    );
    let predecessors = declaration["closed_predecessors"]
        .as_object()
        .expect("predecessors");
    let p66 = &predecessors["pulse_66"];
    assert_eq!(
        p66["declaration_identity"],
        "sha256:2cf44e16b0c61d79ed5ac889ab6fbfe46ee693ce6d9ccf2b4528bb877db45034"
    );
    assert_eq!(
        p66["authority_commit"],
        "1ac79ec4ebec0f2870d9b917403532569c15267a"
    );
    assert_eq!(p66["closeout_commit"], CUTOFF);
    assert_eq!(p66["immutable_cutoff"], "3a99e9e0f383a9821297ef47778fd586b447b7ba");
    assert_eq!(p66["disposition"], "invalid-prelaunch-wsl-probe-bundle-contract");
    assert_eq!(p66["integrity_blocker"], "P66-WORKER-HASH-BUNDLE-LIFETIME");
    assert_eq!(p66["withdrawn_as_recorded"], true);
    assert_eq!(p66["publication"], "not-attempted");
    assert_eq!(p66["authority_callable_invocations"], 0);
    assert_eq!(p66["p59_callable_invocations"], 0);
    assert_eq!(p66["non_retryable"], true);
    assert_eq!(p66["non_resumable"], true);
    assert_eq!(
        p66["contract_review"]["probe_worker_required"],
        "separate-sealed-harmless-probe-worker-with-matching-sealed-probe-dependency"
    );
    assert_eq!(
        predecessors["relationship_to_prior_permanent_dispositions"]["new_cutoff_containing_pulse_66_withdrawal"],
        true
    );

    let authority = &declaration["authority"];
    assert_eq!(authority["pre_call_probe_worker_route_equivalence_only"], true);
    assert_eq!(
        authority["pre_call_probe_worker_single_cleanup_after_spawn2_required"],
        true
    );
    assert_eq!(
        authority["pre_call_static_exact_p57_worker_dependency_binding_required"],
        true
    );
    assert_eq!(
        authority["pre_call_production_worker_execution_claims_forbidden"],
        true
    );

    let runtime = &declaration["runtime_binding"];
    assert_eq!(runtime["pre_call_probe_worker_protocol_schema"], "ferris.pulse-67-wsl-probe-session/v1");
    assert_eq!(runtime["pre_call_exact_wsl_route_preflight_probe_launches_exact"], 1);
    assert_eq!(runtime["pre_call_probe_worker_route_equivalence_only"], true);
    assert_eq!(runtime["pre_call_probe_worker_single_cleanup_after_spawn2_required"], true);
    assert_eq!(runtime["pre_call_static_exact_p57_worker_dependency_binding_required"], true);
    assert_eq!(runtime["pre_call_production_worker_execution_claims_forbidden"], true);

    let pre = &declaration["pre_call_public_prerequisites"];
    let audit = &pre["reversible_creatability_probes"]["helper_reaudit_safe_prerequisites"];
    assert_eq!(audit["p57_probe_worker_route_equivalence_scope_reaudited"], true);
    assert_eq!(audit["p57_probe_worker_ready_one_probe_close_lifecycle_reaudited"], true);
    let platform_stops = &pre["reversible_creatability_probes"]["platform_specific_failure_stops"];
    assert_eq!(platform_stops["stop_before_seed_and_sole_p59_call"], true);
    assert_eq!(
        platform_stops["windows"],
        serde_json::json!([
            "P67-PRIVATE-RUNTIME-TOPOLOGY-PROBE",
            "P67-P41-ACTUAL-BASENAME-RENAME-PROBE",
            "P67-P59-TERMINAL-SIBLING-PROBE"
        ])
    );
    assert_eq!(
        platform_stops["native_linux"],
        serde_json::json!([
            "P67-EXACT-P57-WSL-STAGE-BUNDLE-SUBPROCESS-RUN-PREFLIGHT",
            "P67-SYSTEMROOT-SOURCE-PRECEDENCE-DERIVATION",
            "P67-WINDOWS-WSL-CMD-NONREPARSE-IDENTITY",
            "P67-EXACT-P57-WORKER-PROBE-BUNDLE",
            "P67-EXACT-P57-WORKER-BOOTSTRAP-POPEN-PREFLIGHT",
            "P67-P57-WSL-TWO-SPAWN-CARDINALITY",
            "P67-UBUNTU-NOEXEC-PREREQUISITE"
        ])
    );

    let wsl = &pre["exact_wsl_route_preflight"];
    assert_eq!(wsl["dynamic_route_equivalence_only"], true);
    assert_eq!(wsl["production_worker_execution_claims_forbidden"], true);
    assert_eq!(wsl["static_exact_p57_worker_dependency_binding_required"], true);
    assert_eq!(wsl["probe_worker_single_harmless_launch_required"], true);
    assert_eq!(wsl["worker_bootstrap_ready_one_probe_close_lifecycle"], true);
    assert_eq!(wsl["exact_stage_bundle_and_probe_worker_bootstrap_route_only"], true);
    assert_eq!(wsl["no_ferris_or_production_p56_callable_execution"], true);
    assert!(wsl.get("fake_dependency_only_publish_and_close_during_worker_preflight").is_none());
    assert!(wsl.get("worker_bootstrap_launch_request_forbidden_during_preflight").is_none());
    assert!(wsl.get("worker_bootstrap_startup_ready_plus_close_handshake_only").is_none());

    let cleanup = &wsl["preflight_cleanup"];
    assert_eq!(cleanup["bundle_root_from_spawn1_retained_for_spawn2"], true);
    assert_eq!(cleanup["cleanup_before_spawn2_forbidden"], true);
    assert_eq!(cleanup["single_cleanup_after_spawn2_required"], true);
    assert_eq!(cleanup["bundle_root_absence_verified_after_final_cleanup"], true);
    assert_eq!(cleanup["probe_worker_exit_precedes_cleanup"], true);

    let stage = &wsl["stage_bundle_route"];
    assert_eq!(stage["bundle_root_directory_mode_octal"], "0o700");
    assert_eq!(stage["intermediate_directory_mode_octal"], "0o700");
    assert_eq!(stage["staged_file_mode_octal"], "0o500");
    assert_eq!(stage["bundle_root_handoff_to_spawn2_required"], true);
    assert_eq!(stage["cleanup_before_spawn2_forbidden"], true);
    assert!(stage.get("cleanup_same_invocation_required").is_none());
    assert_eq!(stage["staged_file_count"], 12);

    let worker_route = &wsl["worker_bootstrap_route"];
    assert_eq!(worker_route["worker_source_path"], "<bundle_root>/worker/wsl_session_worker.py");
    assert_eq!(worker_route["worker_source_sha256"], P67_PROBE_WORKER_SHA256);
    assert_eq!(worker_route["sealed_dependencies_source_path"], "<bundle_root>/worker/sealed_dependencies.py");
    assert_eq!(worker_route["sealed_dependencies_source_sha256"], P67_PROBE_DEPENDENCY_SHA256);
    assert_eq!(worker_route["ready_message_count"], 1);
    assert_eq!(worker_route["ready_message_schema"], "ferris.pulse-67-wsl-probe-session/v1");
    assert_eq!(worker_route["probe_launches_exact"], 1);
    assert_eq!(worker_route["probe_launch_argument_count_exact"], 7);
    assert_eq!(worker_route["close_requests_exact"], 1);
    assert_eq!(worker_route["probe_result_returncode_exact"], 0);
    assert_eq!(worker_route["probe_result_stdout_schema"], "ferris.pulse-67-wsl-probe-result/v1");
    assert_eq!(worker_route["bundle_root_public_token"], "<bundle_root>");
    let probe_bundle = &worker_route["probe_bundle"];
    assert_eq!(probe_bundle["probe_worker_source_cutoff_path"], P67_PROBE_WORKER);
    assert_eq!(probe_bundle["probe_dependency_source_cutoff_path"], P67_PROBE_DEPENDENCY);
    assert_eq!(probe_bundle["protocol_schema_cutoff_path"], P67_PROBE_PROTOCOL_SCHEMA);
    assert_eq!(probe_bundle["spawn1_staged_bundle_reused_by_spawn2"], true);
    assert_eq!(probe_bundle["wsl_spawn_required_for_prestaging"], true);
    assert_eq!(probe_bundle["staged_file_count"], 12);
    assert_eq!(probe_bundle["probe_worker_source_cutoff_identity"]["sha256"], P67_PROBE_WORKER_SHA256);
    assert_eq!(probe_bundle["probe_dependency_source_cutoff_identity"]["sha256"], P67_PROBE_DEPENDENCY_SHA256);
    assert_eq!(probe_bundle["protocol_schema_cutoff_identity"]["sha256"], P67_PROBE_PROTOCOL_SCHEMA_SHA256);
    let static_binding = &worker_route["production_worker_static_binding"];
    assert_eq!(static_binding["byte_binding_only"], true);
    assert_eq!(static_binding["dynamic_execution_forbidden"], true);
    assert_eq!(static_binding["production_callable_name"], "run_capability_bound_diagnostic_executor");
    assert_eq!(static_binding["production_worker_source_cutoff_path"], P57_WORKER_SOURCE);
    assert_eq!(static_binding["production_worker_source_cutoff_identity"]["sha256"], P57_WORKER_SHA256);
    assert_eq!(static_binding["production_dependency_source_cutoff_path"], P57_SEALED_DEPENDENCIES_SOURCE);
    assert_eq!(static_binding["production_dependency_source_cutoff_identity"]["sha256"], P57_SEALED_DEPENDENCIES_SHA256);
    assert_eq!(
        static_binding["internal_route_symbols"],
        serde_json::json!([
            "_stage_wsl_bundle",
            "_NativeWslSession",
            "_WSL_BUNDLE_BOOTSTRAP",
            "_WSL_WORKER_BOOTSTRAP"
        ])
    );
    let probe_scope = &worker_route["probe_worker_scope"];
    assert_eq!(
        probe_scope["probe_only_semantics"],
        serde_json::json!([
            "worker-uses-p67-probe-sealed-dependencies-instead-of-production-sealed_dependencies",
            "one-harmless-probe-launch-produces-no-real-p56-capability",
            "production-worker-byte-execution-is-not-claimed"
        ])
    );
    assert_eq!(
        worker_route["source_loader_contract"],
        serde_json::json!({
            "bundled_probe_sealed_dependencies_only": true,
            "descriptor_only_open_hash_compile": true,
            "no_ambient_pythonpath": true,
            "no_mounted_import_route": true,
            "no_sitecustomize": true,
            "no_worker_path_reopen": true,
            "no_wslenv": true,
            "probe_dependency_validates_bundle_root_parent": true,
            "probe_dependency_validates_exact_p56_release_file_set": true,
            "probe_dependency_validates_exact_p56_root": true
        })
    );
    let terminal = &declaration["terminal_transfer_contract"];
    assert_eq!(
        terminal["published_result"]["p43_destination"],
        "docs/simulations/profile-diff-held-out/pulse-67-public-result/"
    );
    assert_eq!(
        terminal["published_result"]["p47_destination"],
        "docs/simulations/profile-diff-held-out/pulse-67-publication-witness/"
    );
    assert_eq!(
        terminal["published_failure_witness"]["destination"],
        "docs/simulations/profile-diff-held-out/pulse-67-publication-witness/"
    );

    let p57 = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py",
    ))
    .expect("UTF-8 Pulse 57 source");
    let worker_source = String::from_utf8(cutoff_blob(P67_PROBE_WORKER)).expect("UTF-8 P67 worker");
    let dep_source = String::from_utf8(cutoff_blob(P67_PROBE_DEPENDENCY)).expect("UTF-8 P67 dependency");
    assert!(p57.contains("completed = subprocess.run("));
    assert!(p57.contains("self._process = subprocess.Popen("));
    assert!(p57.contains("_WSL_BUNDLE_BOOTSTRAP = r\"\"\""));
    assert!(p57.contains("_WSL_WORKER_BOOTSTRAP = r\"\"\""));
    assert!(worker_source.contains("SCHEMA = \"ferris.pulse-67-wsl-probe-session/v1\""));
    assert!(worker_source.contains("REQUEST_COUNT = 1"));
    assert!(worker_source.contains("class ProbeProtocol:"));
    assert!(worker_source.contains("launch_harmless_probe"));
    assert!(dep_source.contains("PROBE_SCHEMA = \"ferris.pulse-67-wsl-probe-session/v1\""));
    assert!(dep_source.contains("PROBE_RESULT_SCHEMA = \"ferris.pulse-67-wsl-probe-result/v1\""));
    assert!(dep_source.contains("PRODUCTION_P57_WORKER_SHA256"));
    assert!(dep_source.contains("EXACT_P56_RELEASE_FILES"));
    assert!(dep_source.contains("def launch_harmless_probe("));
}

#[test]
fn pulse_67_validator_is_checkout_only_and_rejects_mutations() {
    let root = repo_root();
    let p54 = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_54_authority.rs"),
    )
    .expect("P54 validator source");
    let p67 = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_authority.rs"),
    )
    .expect("P67 validator source");
    let legacy_raw_map = ["raw", "sha256", "by", "path"].join("_");
    assert!(p54.contains(&legacy_raw_map), "P54 records the prohibited raw map");
    assert!(
        p67.contains("cutoff_blob"),
        "P67 reads immutable cutoff blobs"
    );
    assert!(
        p67.contains("canonical_identity_by_path"),
        "P67 validates canonical identities"
    );
    assert!(
        !p67.contains(&legacy_raw_map),
        "P67 cannot compare working-tree-only raw identities"
    );
    let python_helper = ["fn", "python()"].join(" ");
    let pulse_59_python_tests = ["run", "Pulse", "59", "Python", "tests"].join(" ");
    let pulse_58_python_tests = ["run", "Pulse", "58", "Python", "tests"].join(" ");
    assert!(
        !p67.contains(&python_helper)
            && !p67.contains(&pulse_59_python_tests)
            && !p67.contains(&pulse_58_python_tests),
        "P67 test must stay declaration/mutation/checkout only"
    );

    let declaration = read_json(
        root.join("docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json"),
    );
    let schema = read_json(
        root.join("docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json"),
    );
    let mutations = read_json(
        root.join("docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority-mutations.json"),
    );
    let controls = mutations["controls"].as_array().expect("controls");
    assert_eq!(mutations["mutation_count"], MUTATION_COUNT);
    assert_eq!(controls.len(), MUTATION_COUNT);
    assert_eq!(mutations["prior_registry_total"], PRIOR_REGISTRY_TOTAL);
    assert_eq!(mutations["registry_total"], REGISTRY_TOTAL);
    assert_eq!(PRIOR_REGISTRY_TOTAL + MUTATION_COUNT, REGISTRY_TOTAL);
    for covered in mutations["control_coverage"]
        .as_object()
        .expect("coverage")
        .values()
    {
        assert_eq!(*covered, Value::Bool(true));
    }
    for (index, control) in controls.iter().enumerate() {
        assert_eq!(control["id"], format!("P67-M{:05}", index + 1));
        let mut mutated = declaration.clone();
        apply_mutation(&mut mutated, control);
        assert_ne!(mutated, declaration, "control {} mutates declaration", index + 1);
        if control["pointer"] != "/declaration_identity" {
            assert_ne!(declaration_identity(&mutated), DECLARATION_IDENTITY);
        }
        assert_ne!(schema["$defs"]["exactAuthority"]["const"], mutated);
    }
}
