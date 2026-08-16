use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "3f7619244420b2ba7762bfc2b1b119d7b1a294a2";
const CUTOFF: &str = "e38dd20f37923e84ac3a3377892c1a5d0954266a";
const PULSE_59_HEAD: &str = "6945f5fc96868c97267a1635fbb5219cc398eeb4";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-62-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:f0db3ddf18a796d0ec107d6d73e9a08cf5e59d47cdad880d584ee8c7e8f61c5a";
const MUTATION_COUNT: usize = 21_644;
const PRIOR_REGISTRY_TOTAL: usize = 139_725;
const REGISTRY_TOTAL: usize = 161_369;
const HELD_OUT: &str = "docs/simulations/profile-diff-held-out";

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

fn git_text(arguments: &[String]) -> String {
    let output = git_output(arguments);
    assert!(
        output.status.success(),
        "Git command failed {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("Git output is UTF-8")
        .trim()
        .to_owned()
}

fn git_blob(revision: &str, relative: &str) -> Vec<u8> {
    let output = git_output(&["show".to_owned(), format!("{revision}:{relative}")]);
    assert!(output.status.success(), "missing Git blob {revision}:{relative}");
    output.stdout
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
    assert_eq!(p59["role"], "sole-p62-production-runtime-callable");
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
fn pulse_62_authority_artifacts_remain_exact_historical_prelaunch_record() {
    let authority_revision = format!("{AUTHORITY_COMMIT}^{{commit}}");
    let cutoff_revision = format!("{CUTOFF}^{{commit}}");
    assert_eq!(git_text(&["rev-parse".to_owned(), authority_revision.clone()]), AUTHORITY_COMMIT);
    assert_eq!(git_text(&["rev-parse".to_owned(), cutoff_revision.clone()]), CUTOFF);
    assert_eq!(
        git_text(&["merge-base".to_owned(), CUTOFF.to_owned(), AUTHORITY_COMMIT.to_owned()]),
        CUTOFF,
        "the historical cutoff predates its authority"
    );

    let held_out = repo_root().join(HELD_OUT);
    for path in [
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority-mutations.json",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-62-authority.v1.schema.json",
    ] {
        assert_eq!(
            read_lf(repo_root().join(path)),
            git_blob(AUTHORITY_COMMIT, path),
            "{path} must remain the exact historical authority artifact"
        );
    }
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-62-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-62-authority.v1.schema.json"),
    );
    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(
        declaration["program_id"],
        "FERRIS-P62-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-WITNESS-PRESERVING-CAPABILITY-MATERIALIZATION-AUTHORITY"
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
        "authority_present_at_cutoff",
        "checkout_materialization_independent_validator_required",
        "complete_exact_current_cutoff_release_trees_required",
        "core_autocrlf_false_fixed_before_checkout",
        "current_cutoff_identities_derived_only_from_immutable_git_blobs",
        "current_cutoff_identity_is_self_excluding",
        "fresh_anonymous_exact_cutoff_checkouts_required",
        "independent_commit_verification_required",
        "pulse_61_withdrawal_present_at_cutoff",
        "public_artifacts_read_only_cutoff_checkout_required",
        "supported_complete_file_crlf_lf_variants_only",
        "working_tree_materialization_is_not_identity_source",
    ] {
        assert_eq!(
            declaration["immutable_ferris"][field],
            field != "authority_present_at_cutoff"
        );
    }
    for path in [
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority-mutations.json",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-62-authority.v1.schema.json",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_62_AUTHORITY.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-62.md",
        "crates/ferris-cli/tests/process_exit_diagnostic_pulse_62_authority.rs",
    ] {
        assert_cutoff_absent(path);
    }
    assert_zero_execution_state(&declaration["execution_state"]);
}

#[test]
fn pulse_62_binds_cutoff_blobs_supported_variants_and_exact_p59_surface() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-62-authority.json"));
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
    assert_eq!(
        bindings["pulse_56_retained_build_custody"]["manifest_receipt_seal"]["public_manifest"]["canonical_sha256"],
        "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a"
    );
    assert_eq!(
        bindings["pulse_56_retained_build_custody"]["manifest_receipt_seal"]["qualification_receipt"]
            ["identity_fields"]["payload_sha256"],
        "sha256:6006f98a103cd822dc51fb2e8297e3755848fea72e4ec50e15ca6cb04a83f8d5"
    );
    assert_eq!(
        bindings["pulse_56_retained_build_custody"]["manifest_receipt_seal"]["release_seal"]["identity_fields"]
            ["payload_sha256"],
        "sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747"
    );
    assert_eq!(
        bindings["pulse_57_capability_bound_diagnostic_executor"]["manifest_receipt_seal"]["public_manifest"]
            ["canonical_sha256"],
        "sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291"
    );
    assert_eq!(
        bindings["pulse_57_capability_bound_diagnostic_executor"]["manifest_receipt_seal"]["qualification_receipt"]
            ["identity_fields"]["payload_sha256"],
        "sha256:5cedec87b57e350d3ab11245c09b9cd7be1f485682d88cb9c1190a939f6bd134"
    );
    assert_eq!(
        bindings["pulse_57_capability_bound_diagnostic_executor"]["manifest_receipt_seal"]["release_seal"]
            ["identity_fields"]["payload_sha256"],
        "sha256:727e1806ede5ca9b5438b5f3b00bec3a59b4e36404ccc1b72e6adb661ebee144"
    );
    assert_eq!(
        bindings["pulse_58_ordered_capability_materialization_executor"]["manifest_receipt_seal"]["public_manifest"]
            ["canonical_sha256"],
        "sha256:8e321041d69a5953aa73a2c67c344a55c25c2fec25008828151d5cb5e16f968f"
    );
    assert_eq!(
        bindings["pulse_58_ordered_capability_materialization_executor"]["manifest_receipt_seal"]["qualification_receipt"]
            ["identity_fields"]["payload_sha256"],
        "sha256:49fb8397ffbd344552b6a4ff1880e3816af35ace55cff3ba6793c3dfef91e7e6"
    );
    assert_eq!(
        bindings["pulse_58_ordered_capability_materialization_executor"]["manifest_receipt_seal"]["release_seal"]
            ["identity_fields"]["payload_sha256"],
        "sha256:9bd64239dca64d8facbe493d2c243d91b4e6d53014efa17fcb9eb4ae2eaffdd5"
    );
    assert_eq!(
        bindings["pulse_59_witness_preserving_capability_materialization_executor"]["manifest_receipt_seal"]
            ["public_manifest"]["canonical_sha256"],
        "sha256:02d887e9fc46ab8729329fdece9b1f150eab17d255e4a2b5cc2e8d3fd46b8242"
    );
    assert_eq!(
        bindings["pulse_59_witness_preserving_capability_materialization_executor"]["manifest_receipt_seal"]
            ["qualification_receipt"]["identity_fields"]["payload_sha256"],
        "sha256:c659f2cb780bbbd838ae3a89edfbbfbada6cc624f8bcb0899e4b37447970d093"
    );
    assert_eq!(
        bindings["pulse_59_witness_preserving_capability_materialization_executor"]["manifest_receipt_seal"]
            ["release_seal"]["identity_fields"]["payload_sha256"],
        "sha256:82d6f6b21444fbc17c035568a9fc69595a41ace68bd866a00c9203fce9085870"
    );

    let p35 = &bindings["pulse_35_public_corpus_materializer"];
    assert_eq!(
        p35["checkout_variant_policy"]["fresh_core_autocrlf_false_canonical_cutoff_required"],
        true
    );
    assert_eq!(
        p35["checkout_variant_policy"]["sealed_pulse_51_p35_p37_variant_binding_required"],
        true
    );
    let p51_custody: Value = serde_json::from_slice(&cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-51-diagnostic-executor-release/p35-p37-custody-binding.json",
    ))
    .expect("P51 custody");
    for file in p51_custody["files"].as_array().expect("P51 files") {
        let full_path = file["path"].as_str().expect("P51 path");
        let relative = full_path.strip_prefix(
            "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/",
        );
        let declared = match relative {
            Some(relative) => &p35["authorized_working_tree_identities_by_path"][relative],
            None => &p35["authorized_supplemental_working_tree_identities_by_path"][full_path],
        };
        assert_eq!(
            declared.as_array().expect("declared variants").len(),
            file["raw_checkout_variants"]
                .as_array()
                .expect("P51 variants")
                .len()
        );
        for (declared_variant, sealed_variant) in declared
            .as_array()
            .unwrap()
            .iter()
            .zip(file["raw_checkout_variants"].as_array().unwrap())
        {
            for field in ["cr_bytes", "lf_bytes", "sha256", "size"] {
                assert_eq!(
                    declared_variant[field], sealed_variant[field],
                    "{full_path} {field}"
                );
            }
        }
    }
    let p37 = &bindings["pulse_37_checkout_normalization"];
    let p37_variants = p37["authorized_working_tree_identities_by_path"]["README.md"]
        .as_array()
        .expect("P37 variants");
    assert_eq!(p37_variants.len(), 2);
    let canonical = cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-37-checkout-normalization/README.md",
    );
    let crlf = String::from_utf8(canonical.clone())
        .expect("UTF-8 P37 README")
        .replace('\n', "\r\n")
        .into_bytes();
    assert_eq!(p37_variants[0]["sha256"], sha256(&canonical));
    assert_eq!(p37_variants[1]["sha256"], sha256(&crlf));
    assert_api_bindings(&declaration);
}

#[test]
fn pulse_62_records_withdrawn_pulse_61_and_reversible_creatability_contract() {
    let root = repo_root();
    let held_out = root.join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-62-authority.json"));
    let predecessors = declaration["closed_predecessors"]
        .as_object()
        .expect("predecessors");
    for (pulse, disposition) in [
        ("pulse_46", "invalid-publication-integrity"),
        ("pulse_48", "invalid-publication-integrity"),
        ("pulse_49", "invalid-prelaunch-authority-integrity"),
        ("pulse_50", "invalid-prelaunch-infrastructure-integrity"),
        ("pulse_54", "invalid-prelaunch-checkout-variant-integrity"),
        ("pulse_55", "terminal-prerequisite-identity-failure"),
        ("pulse_60", "invalid-prelaunch-runtime-root-contract"),
    ] {
        let closed = &predecessors[pulse];
        assert_eq!(closed["disposition"], disposition, "{pulse} disposition");
        assert_eq!(closed["permanently_closed"], true, "{pulse} closed");
        assert_eq!(closed["non_retryable"], true, "{pulse} non-retryable");
        assert_eq!(
            closed["further_launches_prohibited"],
            true,
            "{pulse} prohibited"
        );
    }
    let p61 = &predecessors["pulse_61"];
    assert_eq!(p61["declaration_identity"], "sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d");
    assert_eq!(p61["authority_commit"], "53e3c86653a71171a9301dd5cff185a522af1231");
    assert_eq!(p61["closeout_commit"], CUTOFF);
    assert_eq!(p61["immutable_cutoff"], "70ed752359c04e4aac77a49280c37f2cf6b8d012");
    assert_eq!(p61["disposition"], "invalid-prelaunch-root-creatability-contract");
    assert_eq!(
        p61["integrity_blocker"],
        "P61-ROOT-CREATABILITY-CALLABLE-CONTRACT"
    );
    assert_eq!(p61["withdrawn_as_recorded"], true);
    assert_eq!(p61["publication"], "not-attempted");
    assert_eq!(p61["authority_callable_invocations"], 0);
    assert_eq!(p61["p59_callable_invocations"], 0);
    assert_eq!(p61["non_retryable"], true);
    assert_eq!(p61["non_resumable"], true);
    for field in ["retry", "resume", "reconstruction", "reseed", "reuse"] {
        assert_eq!(p61[field], false, "Pulse 61 closeout {field}");
    }
    for field in [
        "descriptors",
        "private_runtime_roots",
        "processes",
        "publications",
        "seeds",
        "transfers",
    ] {
        assert_eq!(p61["artifacts"][field], 0, "Pulse 61 artifact {field}");
    }
    assert_eq!(
        p61["contract_review"]["private_runtime_root_declared"],
        "absolute-existing-empty-safe-directory"
    );
    assert_eq!(
        p61["contract_review"]["private_runtime_root_required"],
        "reversible-child-creatability-proof-for-pulse58-namespace-and-p56-windows-child"
    );
    assert_eq!(
        p61["contract_review"]["p41_final_parent_declared"],
        "safe-existing-parent-with-absent-final-stage-rollback-paths"
    );
    assert_eq!(
        p61["contract_review"]["p41_final_parent_required"],
        "reversible-stage-final-rename-cleanup-probe-with-same-filesystem-and-path-length-proof"
    );
    assert_eq!(
        p61["contract_review"]["p59_terminal_parent_required"],
        "reversible-terminal-sibling-creatability-proof"
    );
    assert_eq!(
        p61["contract_review"]["ubuntu_runtime_parent_required"],
        "reversible-p57-bundle-and-p56-ubuntu-child-creatability-proof-plus-executable-noexec-audit"
    );
    assert_eq!(
        predecessors["relationship_to_prior_permanent_dispositions"]["new_cutoff_containing_pulse_61_withdrawal"],
        true
    );

    let pre = &declaration["pre_call_public_prerequisites"];
    let probes = &pre["reversible_creatability_probes"];
    assert_eq!(probes["performed_before_seed"], true);
    assert_eq!(probes["performed_before_sole_p59_invocation"], true);
    assert_eq!(probes["public_constant_derivation_only"], true);
    assert_eq!(probes["private_seed_derivation_forbidden"], true);
    let stem = probes["probe_name_stem"].as_str().expect("probe stem");
    assert!(stem.contains(&CUTOFF[..12]));
    assert!(stem.contains(&PULSE_59_HEAD[..12]));
    assert_eq!(
        probes["collision_with_real_stack_names_forbidden"],
        serde_json::json!([
            ".pulse58-private-launch",
            ".p56-",
            ".p57-",
            ".pulse59-terminal-publication",
            ".pulse-41-stage"
        ])
    );
    let private_runtime = &probes["private_runtime_root_parent"];
    assert_eq!(private_runtime["parent"], "private_runtime_root");
    assert!(
        private_runtime["p58_namespace_probe_child_name"]
            .as_str()
            .unwrap()
            .starts_with(".pulse62-probe-")
    );
    assert!(
        private_runtime["p56_windows_probe_child_name"]
            .as_str()
            .unwrap()
            .starts_with(".p56-probe-")
    );
    assert_eq!(private_runtime["probe_file_name"], "probe.marker");
    let p41_probes = &probes["p41_final_parent"];
    assert_eq!(p41_probes["same_filesystem_stage_final_rename_required"], true);
    assert_eq!(p41_probes["path_length_headroom_required"], true);
    assert_eq!(
        p41_probes["probe_stage_root_name"],
        Value::String(format!(
            ".{}.pulse-41-stage",
            p41_probes["probe_final_root_name"].as_str().unwrap()
        ))
    );
    let terminal_probe = &probes["p59_terminal_parent"];
    assert_eq!(terminal_probe["real_suffix_reserved"], ".pulse59-terminal-publication");
    assert!(
        terminal_probe["terminal_probe_child_name"]
            .as_str()
            .unwrap()
            .contains(stem)
    );
    let ubuntu_probe = &probes["ubuntu_runtime_parent"];
    assert!(
        ubuntu_probe["p57_bundle_probe_child_name"]
            .as_str()
            .unwrap()
            .starts_with(".p57-probe-")
    );
    assert!(
        ubuntu_probe["p56_ubuntu_probe_child_name"]
            .as_str()
            .unwrap()
            .starts_with(".p56-probe-")
    );
    assert_eq!(ubuntu_probe["native_linux_executable_filesystem_required"], true);
    assert_eq!(ubuntu_probe["native_linux_noexec_forbidden"], true);
    let platform_stops = &probes["platform_specific_failure_stops"];
    assert_eq!(platform_stops["stop_before_seed_and_sole_p59_call"], true);
    assert_eq!(
        platform_stops["windows"],
        serde_json::json!([
            "P62-PRIVATE-RUNTIME-PROBE",
            "P62-P41-RENAME-TOPOLOGY-PROBE",
            "P62-P59-TERMINAL-PROBE"
        ])
    );
    assert_eq!(
        platform_stops["native_linux"],
        serde_json::json!([
            "P62-UBUNTU-RUNTIME-PROBE",
            "P62-UBUNTU-NOEXEC-PREREQUISITE"
        ])
    );
    let audit = &probes["helper_reaudit_safe_prerequisites"];
    assert_eq!(audit["p56_restrictive_child_modes_and_directory_sync_reaudited"], true);
    assert_eq!(audit["p57_native_linux_parent_and_bundle_path_rules_reaudited"], true);
    assert_eq!(audit["p58_namespace_absence_and_child_creation_order_reaudited"], true);
    assert_eq!(audit["p59_terminal_parent_child_creation_reaudited"], true);
    assert_eq!(audit["p41_stage_final_rollback_rename_topology_reaudited"], true);
    assert_eq!(audit["no_real_ferris_execution_or_p59_consumption_during_audit"], true);
    assert_eq!(pre["private_runtime_root"]["p58_namespace_child_proven_pre_call"], true);
    assert_eq!(pre["private_runtime_root"]["p56_windows_custody_child_proven_pre_call"], true);
    assert_eq!(pre["p41_public_custody_roots"]["same_filesystem_stage_final_rename_required"], true);
    assert_eq!(pre["p41_public_custody_roots"]["path_length_headroom_validation_required"], true);
    assert_eq!(
        pre["p59_terminal_root"]["reversible_creatability_probe_required_before_callable"],
        true
    );
    assert_eq!(pre["ubuntu_runtime_parent"]["native_linux_noexec_forbidden"], true);
    assert_eq!(
        declaration["execution_order"]["pre_call_reversible_probe_groups_exact"],
        4
    );
    assert_eq!(
        declaration["execution_order"]["pre_call_reversible_probe_failures_stop_before_p59"],
        true
    );

    let p41 = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/transactional_copy.py",
    ))
    .expect("UTF-8 Pulse 41 source");
    let p56 = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/retained_build_custody.py",
    ))
    .expect("UTF-8 Pulse 56 source");
    let p57 = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/capability_bound_executor.py",
    ))
    .expect("UTF-8 Pulse 57 source");
    let p58 = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release/ordered_capability_materialization_executor.py",
    ))
    .expect("UTF-8 Pulse 58 source");
    let p59 = String::from_utf8(cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/witness_preserving_capability_materialization_executor.py",
    ))
    .expect("UTF-8 Pulse 59 source");
    assert!(p58.contains(r#"PRIVATE_NAMESPACE = ".pulse58-private-launch""#));
    assert!(p58.contains("os.mkdir(namespace, 0o700)"));
    assert!(p58.contains(
        r#"windows_handle = p56.publish_retained_build_and_custody("windows-x86_64", runtime_root)"#
    ));
    assert!(p41.contains(
        r#"staging_root = final_parent / f".{final_before_publication.name}.pulse-41-stage""#
    ));
    assert!(p41.contains("renamer(staging_root, final_before_publication)"));
    assert!(p41.contains("_rollback_final("));
    assert!(p59.contains(
        r#"candidate = parent / f"{runtime_root.name}{TERMINAL_ROOT_SUFFIX}""#
    ));
    assert!(p59.contains("os.mkdir(terminal_parent, 0o700)"));
    assert!(p57.contains(r#"name = ".p57-" + secrets.token_hex(16)"#));
    assert!(p57.contains(r#"expected_root = runtime_parent.rstrip("/") + "/" + name"#));
    assert!(p56.contains(
        r#"run = _fresh_child(parent, f".p56-{secrets.token_hex(12)}", "P56-RUNTIME-ROOT")"#
    ));
    assert!(p56.contains("def _sync_directory(path: Path) -> str:"));
}

#[test]
fn pulse_62_validator_is_checkout_only_and_rejects_mutations() {

    let root = repo_root();
    let p54 = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_54_authority.rs"),
    )
    .expect("P54 validator source");
    let p62 = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_62_authority.rs"),
    )
    .expect("P62 validator source");
    let legacy_raw_map = ["raw", "sha256", "by", "path"].join("_");
    assert!(
        p54.contains(&legacy_raw_map),
        "P54 records the prohibited raw map"
    );
    assert!(
        p54.contains("fs::read(directory.join(&path))"),
        "P54 compares local release bytes"
    );
    assert!(
        p62.contains("cutoff_blob"),
        "P62 reads immutable cutoff blobs"
    );
    assert!(
        p62.contains("canonical_identity_by_path"),
        "P62 validates canonical identities"
    );
    assert!(
        !p62.contains(&legacy_raw_map),
        "P62 cannot compare working-tree-only raw identities"
    );
    let python_helper = ["fn", "python()"].join(" ");
    let pulse_59_python_tests = ["run", "Pulse", "59", "Python", "tests"].join(" ");
    let pulse_58_python_tests = ["run", "Pulse", "58", "Python", "tests"].join(" ");
    assert!(
        !p62.contains(&python_helper)
            && !p62.contains(&pulse_59_python_tests)
            && !p62.contains(&pulse_58_python_tests),
        "P62 test must stay declaration/mutation/checkout only"
    );

    let held_out = root.join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-62-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-62-authority.v1.schema.json"),
    );
    let mutations = read_json(
        held_out.join("fixtures/process-exit-diagnostic-pulse-62-authority-mutations.json"),
    );
    let controls = mutations["controls"].as_array().expect("mutation controls");
    assert_eq!(mutations["authority_schema"], DOMAIN);
    assert_eq!(mutations["base_declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(mutations["mutation_count"], MUTATION_COUNT);
    assert_eq!(controls.len(), MUTATION_COUNT);
    assert_eq!(mutations["prior_registry_total"], PRIOR_REGISTRY_TOTAL);
    assert_eq!(mutations["registry_total"], REGISTRY_TOTAL);
    assert_eq!(PRIOR_REGISTRY_TOTAL + MUTATION_COUNT, REGISTRY_TOTAL);
    assert_eq!(
        mutations["mutation_model"]["scalar_replacements_per_leaf"],
        10
    );
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
        assert_eq!(id, format!("P62-M{:05}", index + 1));
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
