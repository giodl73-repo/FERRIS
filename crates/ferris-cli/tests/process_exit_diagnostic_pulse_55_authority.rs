use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "47113e444ef3309afec9a844f0cba62775f19f6f";
const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-55-authority/v1";
const DECLARATION_IDENTITY: &str =
    "sha256:45ac35775c34e8a86fdc90ad1554104f2728a676d51ab46125bfcf126db21655";
const MUTATION_COUNT: usize = 19_261;
const PRIOR_REGISTRY_TOTAL: usize = 81_321;
const REGISTRY_TOTAL: usize = 100_582;
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
    let p53 = &api["pulse_53"];
    assert_eq!(p53["role"], "sole-p55-production-runtime-callable");
    assert_eq!(p53["return_annotation"], "WitnessPreservingOrderedResult");
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
fn pulse_55_authority_is_closed_self_excluding_and_unexecuted() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-55-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-55-authority.v1.schema.json"),
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
    for field in [
        "authority_present_at_cutoff",
        "current_cutoff_identities_derived_only_from_immutable_git_blobs",
        "current_cutoff_identity_is_self_excluding",
        "checkout_materialization_independent_validator_required",
        "working_tree_materialization_is_not_identity_source",
    ] {
        assert_eq!(
            declaration["immutable_ferris"][field],
            field != "authority_present_at_cutoff"
        );
    }
    for path in [
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-55-authority.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-55-authority-mutations.json",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-55-authority.v1.schema.json",
        "crates/ferris-cli/tests/process_exit_diagnostic_pulse_55_authority.rs",
    ] {
        assert_cutoff_absent(path);
    }
    assert_zero_execution_state(&declaration["execution_state"]);
}

#[test]
fn pulse_55_binds_cutoff_blobs_and_exact_working_materializations() {
    let held_out = repo_root().join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-55-authority.json"));
    let bindings = declaration["public_release_bindings"]
        .as_object()
        .expect("bindings");
    assert_eq!(bindings.len(), 14);
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
        "42a16e298c5af55b05df5ceb8e3477d0dd45c814"
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
    let p51_custody: Value = serde_json::from_slice(&cutoff_blob("docs/simulations/profile-diff-held-out/pulse-51-diagnostic-executor-release/p35-p37-custody-binding.json")).expect("P51 custody");
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
fn pulse_55_preserves_closeouts_and_exact_one_call_p53_contract() {
    let root = repo_root();
    let held_out = root.join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-55-authority.json"));
    let predecessors = declaration["closed_predecessors"]
        .as_object()
        .expect("predecessors");
    for (pulse, disposition) in [
        ("pulse_46", "invalid-publication-integrity"),
        ("pulse_48", "invalid-publication-integrity"),
        ("pulse_49", "invalid-prelaunch-authority-integrity"),
        ("pulse_50", "invalid-prelaunch-infrastructure-integrity"),
        ("pulse_54", "invalid-prelaunch-checkout-variant-integrity"),
    ] {
        let closed = &predecessors[pulse];
        assert_eq!(closed["disposition"], disposition, "{pulse} disposition");
        assert_eq!(closed["permanently_closed"], true, "{pulse} closed");
        assert_eq!(closed["non_retryable"], true, "{pulse} non-retryable");
        assert_eq!(
            closed["further_launches_prohibited"], true,
            "{pulse} prohibited"
        );
        assert_eq!(
            closed["category_conclusion"],
            Value::Null,
            "{pulse} category"
        );
        assert_eq!(
            closed["diagnostic_conclusion"],
            Value::Null,
            "{pulse} diagnostic"
        );
        assert_eq!(closed["product_conclusion"], Value::Null, "{pulse} product");
    }
    let p54 = &predecessors["pulse_54"];
    assert_eq!(
        p54["integrity_blocker"],
        "P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY"
    );
    for field in [
        "authority_callable_invocations",
        "p53_callable_invocations",
        "private_runtime_artifacts",
        "publication_artifacts",
        "result_artifacts",
        "runtime_artifacts",
        "seed_artifacts",
        "witness_artifacts",
    ] {
        assert_eq!(p54[field], 0, "P54 {field}");
    }
    for group in ["calls", "artifacts"] {
        for (field, value) in p54[group].as_object().expect("P54 zero group") {
            assert_eq!(value, 0, "P54 {group} {field}");
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
        "amendment",
        "same_authority",
        "same_cutoff",
        "same_execution",
    ] {
        assert_eq!(
            predecessors["relationship_to_prior_permanent_dispositions"][field], false,
            "prior disposition {field}"
        );
    }
    assert_eq!(
        predecessors["relationship_to_prior_permanent_dispositions"]["fresh_authority"],
        true
    );

    let catalog = &declaration["p43_public_catalog"];
    assert_eq!(
        catalog["gate_ids"],
        serde_json::json!([
            "pulse-41-pulse-39-public-custody",
            "windows-retained-binary-custody",
            "ubuntu-retained-binary-custody",
            "exact-adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "bounded-materialization",
            "bounded-process-exit-search"
        ])
    );
    assert_eq!(
        catalog["precall_validation_ids"],
        serde_json::json!(["public-catalog-prevalidation", "public-input-contract"])
    );
    let pre_call = &declaration["pre_call_public_prerequisites"];
    assert_eq!(pre_call["cutoff_checkouts"]["revision"], CUTOFF);
    assert_eq!(pre_call["cutoff_checkouts"]["core_autocrlf"], false);
    assert_eq!(pre_call["cutoff_checkouts"]["anonymous"], true);
    assert_eq!(pre_call["cutoff_checkouts"]["fresh"], true);
    assert_eq!(pre_call["p44_custody"]["operation_per_platform_exact"], 1);
    assert_eq!(
        pre_call["p33_build_freeze"]["windows"]["artifact_size"],
        1_436_672
    );
    assert_eq!(
        pre_call["p33_build_freeze"]["ubuntu"]["artifact_size"],
        1_945_448
    );
    let runtime = &declaration["runtime_binding"];
    assert_eq!(
        runtime["sole_callable"],
        "run_witness_preserving_ordered_executor"
    );
    assert_eq!(runtime["sole_callable_invocations_exact"], 1);
    assert_eq!(
        runtime["prohibited_direct_callables"],
        serde_json::json!([
            "pulse_43.publish_result",
            "pulse_47.witness_pulse_43",
            "pulse_51.run_diagnostic_executor",
            "pulse_52.run_ordered_materialization_executor"
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
    assert_eq!(declaration["topology"]["case_dispositions_total"], 140);
    assert_eq!(declaration["topology"]["os_processes_total"], 138);
    assert_eq!(declaration["topology"]["no_launch_dispositions_total"], 2);
    assert_eq!(declaration["execution_order"]["p27_invocations_exact"], 1);
    assert_eq!(
        declaration["execution_order"]["p35_materializer_invocations_exact"],
        1
    );
    assert_eq!(
        declaration["execution_order"]["p35_verifier_invocations_exact"],
        1
    );
    assert_eq!(
        declaration["execution_order"]["p47_to_p43_invocations_exact"],
        1
    );

    let transfer = &declaration["terminal_transfer_contract"];
    let published = &transfer["published_result"];
    let failure = &transfer["published_failure_witness"];
    assert_eq!(published["p43_result_exact_file_count"], 2);
    assert_eq!(published["p47_witness_exact_file_count"], 2);
    assert_eq!(failure["p43_result_path_must_remain_absent"], true);
    assert_eq!(failure["p47_witness_exact_file_count"], 2);
    for destination in [
        published["p43_destination"].as_str().unwrap(),
        published["p47_destination"].as_str().unwrap(),
    ] {
        assert!(destination.contains("pulse-55-"), "Pulse 55 destination");
        assert!(
            !root.join(destination.trim_end_matches('/')).exists(),
            "no runtime artifact"
        );
    }
}

#[test]
fn pulse_55_prohibits_p54_local_hash_comparison_and_rejects_mutations() {
    let root = repo_root();
    let p54 = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_54_authority.rs"),
    )
    .expect("P54 validator source");
    let p55 = fs::read_to_string(
        root.join("crates/ferris-cli/tests/process_exit_diagnostic_pulse_55_authority.rs"),
    )
    .expect("P55 validator source");
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
        p55.contains("cutoff_blob"),
        "P55 reads immutable cutoff blobs"
    );
    assert!(
        p55.contains("canonical_identity_by_path"),
        "P55 validates canonical identities"
    );
    assert!(
        !p55.contains(&legacy_raw_map),
        "P55 cannot compare P54 local raw identities"
    );

    let held_out = root.join(HELD_OUT);
    let declaration =
        read_json(held_out.join("fixtures/process-exit-diagnostic-pulse-55-authority.json"));
    let schema = read_json(
        held_out.join("schemas/ferris.process-exit-diagnostic-pulse-55-authority.v1.schema.json"),
    );
    let mutations = read_json(
        held_out.join("fixtures/process-exit-diagnostic-pulse-55-authority-mutations.json"),
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
        assert_eq!(id, format!("P55-M{:05}", index + 1));
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
