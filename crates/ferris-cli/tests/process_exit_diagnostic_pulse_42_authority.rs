use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-42-authority/v1";
const CUTOFF: &str = "2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8";
const DECLARATION_IDENTITY: &str =
    "sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc";
const MUTATION_COUNT: usize = 9_046;
const TOTAL_DECLARED_MUTATIONS: usize = 29_611;

const P41_RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release";
const P39_RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release";
const P41_MANIFEST_RAW: &str =
    "sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8";
const P41_MANIFEST_AGGREGATE: &str =
    "sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755";
const P41_REPORT_RAW: &str =
    "sha256:e16b84700318b3bedb82b283d9af2df8ae963fe63465fd2056a43839dfcfd8ee";
const P41_REPORT_PAYLOAD: &str =
    "sha256:fbcfd656024e3fd6b3e24d18cf8991532fd77284611ab81f578e1985f6a5b4cc";
const P41_RECEIPT_RAW: &str =
    "sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c";
const P41_RECEIPT_PAYLOAD: &str =
    "sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f";
const P41_SEAL_RAW: &str =
    "sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a";
const P41_SEAL_PAYLOAD: &str =
    "sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf";

const P39_MANIFEST_RAW: &str =
    "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c";
const P39_MANIFEST_AGGREGATE: &str =
    "sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c";
const P39_REPORT_RAW: &str =
    "sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd";
const P39_REPORT_PAYLOAD: &str =
    "sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab";
const P39_RECEIPT_RAW: &str =
    "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8";
const P39_RECEIPT_PAYLOAD: &str =
    "sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546";
const P39_SEAL_RAW: &str =
    "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c";
const P39_SEAL_PAYLOAD: &str =
    "sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b";

const P41_PATHS: [&str; 8] = [
    "README.md",
    "public-manifest.json",
    "qualification-receipt.json",
    "release-seal.json",
    "root-cause-report.json",
    "root-cause-report.md",
    "tests/test_transactional_copy.py",
    "transactional_copy.py",
];
const P39_PATHS: [&str; 8] = [
    "README.md",
    "checkout_verifier.py",
    "public-manifest.json",
    "qualification-receipt.json",
    "release-seal.json",
    "root-cause-report.json",
    "root-cause-report.md",
    "tests/test_checkout_verifier.py",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
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

fn git_output(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run git")
}

fn cutoff_blob(path: &str) -> Vec<u8> {
    let output = git_output(&["show", &format!("{CUTOFF}:{path}")]);
    assert!(output.status.success(), "missing cutoff blob {path}");
    output.stdout
}

fn cutoff_tree_paths(release_root: &str) -> BTreeSet<String> {
    let output = git_output(&["ls-tree", "-r", "--name-only", CUTOFF, "--", release_root]);
    assert!(
        output.status.success(),
        "list cutoff release tree {release_root}"
    );
    let prefix = format!("{release_root}/");
    String::from_utf8(output.stdout)
        .expect("UTF-8 cutoff release tree")
        .lines()
        .map(|path| {
            path.strip_prefix(&prefix)
                .expect("release tree path under root")
                .to_owned()
        })
        .collect()
}

fn decode_sha256(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64, "SHA-256 hex length");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).expect("SHA-256 hex");
    }
    bytes
}

fn manifest_aggregate(files: &[Value]) -> String {
    let mut ordered = files.to_vec();
    ordered.sort_by_key(|file| file["path"].as_str().expect("manifest path").to_owned());
    let mut hasher = Sha256::new();
    for file in ordered {
        let path = file["path"].as_str().expect("manifest path");
        hasher.update((path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(decode_sha256(
            file["sha256"]
                .as_str()
                .expect("manifest digest")
                .strip_prefix("sha256:")
                .expect("sha256 prefix"),
        ));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn unescape_token(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    let (parent, key) = pointer.rsplit_once('/').expect("mutation parent");
    let key = unescape_token(key);
    let parent = if parent.is_empty() {
        value
    } else {
        value.pointer_mut(parent).expect("mutation target parent")
    };
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            if let Some(object) = parent.as_object_mut() {
                object.insert(key, mutation["value"].clone());
            } else {
                parent.as_array_mut().expect("mutation target array")
                    [key.parse::<usize>().expect("array index")] = mutation["value"].clone();
            }
        }
        "remove" => {
            if let Some(object) = parent.as_object_mut() {
                object.remove(&key);
            } else {
                parent
                    .as_array_mut()
                    .expect("mutation target array")
                    .remove(key.parse::<usize>().expect("array index"));
            }
        }
        operation => panic!("unsupported mutation {operation}"),
    }
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

fn declared_paths(value: &Value, field: &str) -> BTreeSet<String> {
    value[field]
        .as_array()
        .expect("declared paths")
        .iter()
        .map(|path| path.as_str().expect("path").to_owned())
        .collect()
}

#[test]
fn pulse_42_authority_is_closed_unexecuted_and_ordered() {
    let schema = read_json(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-42-authority.v1.schema.json"),
    );
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-42-authority.json"));
    let pulse_40 =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-40-authority.json"));

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
        declaration["immutable_ferris"]["cutoff_contains_pulse_41_transactional_copy_release"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_pulse_42_authority"],
        false
    );
    assert_eq!(
        declaration["inherited_public_gates"],
        pulse_40["inherited_public_gates"]
    );
    for predecessor in ["pulse_38", "pulse_40"] {
        let record = &declaration["closed_predecessors"][predecessor];
        assert!(record["permanently_closed"].as_bool().expect("closed"));
        assert!(
            record["further_launches_prohibited"]
                .as_bool()
                .expect("prohibited")
        );
        assert_eq!(record["category_conclusion"], Value::Null);
        for relationship in [
            "retry",
            "resume",
            "reseed",
            "reuse",
            "correlation",
            "inference",
        ] {
            assert_eq!(record[relationship], false, "{predecessor} {relationship}");
            assert_eq!(
                declaration["authority"][format!("{predecessor}_{relationship}")],
                false,
                "{predecessor} authority {relationship}"
            );
        }
    }

    let p41 = &declaration["pulse_41_transactional_copy_release"];
    assert_eq!(p41["release_tree_file_count"], 8);
    assert_eq!(p41["manifest_payload_file_count"], 5);
    assert_eq!(p41["manifest_payload_total_bytes"], 49_120);
    assert_eq!(
        declared_paths(p41, "exact_release_tree_paths"),
        BTreeSet::from_iter(P41_PATHS.map(str::to_owned))
    );
    assert_eq!(p41["manifest"]["raw_sha256"], P41_MANIFEST_RAW);
    assert_eq!(p41["manifest"]["aggregate"], P41_MANIFEST_AGGREGATE);
    assert_eq!(p41["root_cause_report"]["raw_sha256"], P41_REPORT_RAW);
    assert_eq!(
        p41["root_cause_report"]["payload_sha256"],
        P41_REPORT_PAYLOAD
    );
    assert_eq!(p41["qualification_receipt"]["raw_sha256"], P41_RECEIPT_RAW);
    assert_eq!(
        p41["qualification_receipt"]["payload_sha256"],
        P41_RECEIPT_PAYLOAD
    );
    assert_eq!(p41["release_seal"]["raw_sha256"], P41_SEAL_RAW);
    assert_eq!(p41["release_seal"]["payload_sha256"], P41_SEAL_PAYLOAD);
    let adapter = &p41["direct_adapter_execution"];
    assert_eq!(
        adapter["adapter_path"],
        format!("{P41_RELEASE}/transactional_copy.py")
    );
    assert_eq!(
        adapter["adapter_must_be_executed_directly_from_immutable_cutoff_checkout"],
        true
    );
    assert_eq!(
        adapter["alternative_copier_before_adapter_prohibited"],
        true
    );
    assert_eq!(
        adapter["python_dont_write_bytecode_environment"],
        "PYTHONDONTWRITEBYTECODE=1"
    );
    assert_eq!(
        adapter["source_root"],
        "absolute-cutoff-pulse-39-source-root"
    );
    assert_eq!(
        adapter["final_root"],
        "fresh-absent-absolute-custody-final-root"
    );
    for (field, value) in [
        ("invocations_exact", 1),
        ("retries", 0),
        ("source_files_expected", 8),
        ("stage_files_expected", 8),
        ("final_files_expected", 8),
        ("destination_file_fsyncs_expected", 8),
        ("staging_directory_sync_attempts_expected", 2),
        ("rename_attempts_expected", 1),
        ("rollback_attempts_expected", 0),
        ("final_recomputation_files_expected", 8),
    ] {
        assert_eq!(adapter[field], value, "Pulse 41 {field}");
    }
    assert_eq!(
        adapter["staging_directory_sync_posture"],
        serde_json::json!(["synced", "unsupported"])
    );
    assert_eq!(
        adapter["final_parent_sync_posture"],
        serde_json::json!(["synced", "unsupported"])
    );
    assert_eq!(adapter["indeterminate_publication_expected"], false);
    assert_eq!(adapter["stage_residue_expected"], false);
    assert_eq!(adapter["final_residue_expected"], false);

    let p39 = &declaration["pulse_39_public_checkout_verifier_release"];
    assert_eq!(p39["release_tree_file_count"], 8);
    assert_eq!(p39["manifest_payload_file_count"], 5);
    assert_eq!(p39["custodied_final_root_from_pulse_41_required"], true);
    assert_eq!(
        p39["verifier_path_below_custodied_final_root"],
        "checkout_verifier.py"
    );
    assert_eq!(p39["verifier_control"]["total_git_processes"], 2);
    assert_eq!(p39["verifier_control"]["check_attr_invocations"], 1);
    assert_eq!(p39["verifier_control"]["git_version_probes"], 1);
    assert_eq!(p39["verifier_control"]["retries"], 0);
    assert_eq!(p39["verifier_control"]["zero_fallbacks"], true);
    assert_eq!(
        p39["normalized_binding_verification"]["binding_checks_total"],
        76
    );

    let order = declaration["execution_order"]["order"]
        .as_array()
        .expect("order");
    assert_eq!(order.len(), 11);
    assert_eq!(
        order[0],
        "fresh-immutable-read-only-public-artifacts-cutoff-checkout-and-exact-pulse-41-release-verification"
    );
    assert_eq!(
        order[1],
        "direct-cutoff-pulse-41-transactional-copy-from-exact-absolute-pulse-39-source-root-to-fresh-absent-absolute-custody-final-root"
    );
    assert_eq!(
        order[3],
        "separate-fresh-core-autocrlf-true-cutoff-checkout-and-copied-pulse-39-checkout-verifier"
    );
    assert_eq!(
        order[10],
        "one-pulse-42-bounded-transactional-cross-platform-search"
    );
    let search = &declaration["pulse_42_bounded_transactional_search"];
    assert_eq!(search["one_search_execution_max"], 1);
    assert_eq!(search["candidate_retries"], 0);
    assert_eq!(search["cases_per_platform_max"], 70);
    assert_eq!(search["processes_per_platform_max"], 70);
    assert_eq!(search["total_processes_max"], 140);

    let state = declaration["execution_state"]
        .as_object()
        .expect("execution state");
    assert!(!state.is_empty());
    for (field, value) in state {
        assert!(
            value == &Value::Bool(false) || value.as_u64() == Some(0) || value.is_null(),
            "execution field must be zero, false, or null: {field}={value}"
        );
    }
    for field in [
        "pulse_41_adapter_invocations",
        "pulse_41_source_files_copied",
        "pulse_41_stage_files_verified",
        "pulse_41_final_files_verified",
        "pulse_41_destination_file_fsyncs",
        "pulse_41_staging_directory_sync_attempts",
        "pulse_41_rename_attempts",
        "pulse_41_retries",
        "pulse_41_rollback_attempts",
        "pulse_39_verifier_invocations",
        "pulse_39_check_attr_invocations",
        "pulse_39_git_version_probes",
        "pulse_39_git_processes",
        "pulse_39_normalized_bindings_verified",
        "pulse_25_27_package_files_copied",
        "cutoff_binaries_frozen",
        "adapter_invocations",
        "public_input_classifications",
        "normalized_pulse_35_release_files_copied",
        "materialized_descriptors",
        "candidate_processes",
        "search_executions",
    ] {
        assert_eq!(state[field], 0, "zero execution state {field}");
    }
    assert_eq!(state["private_seed_created"], false);
}

#[test]
fn pulse_42_recomputes_cutoff_git_blobs_and_pulse_41_pulse_39_bindings() {
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-42-authority.json"));
    let p41 = &declaration["pulse_41_transactional_copy_release"];
    let p41_paths = BTreeSet::from_iter(P41_PATHS.map(str::to_owned));
    assert_eq!(cutoff_tree_paths(P41_RELEASE), p41_paths);
    assert_eq!(declared_paths(p41, "exact_release_tree_paths"), p41_paths);
    let bindings = p41["release_tree_raw_bindings"]
        .as_array()
        .expect("Pulse 41 raw bindings");
    assert_eq!(bindings.len(), 8);
    let mut bound_paths = BTreeSet::new();
    for binding in bindings {
        let path = binding["path"].as_str().expect("Pulse 41 path");
        assert!(bound_paths.insert(path.to_owned()), "unique Pulse 41 path");
        let cutoff_bytes = cutoff_blob(&format!("{P41_RELEASE}/{path}"));
        let checkout_bytes = read_lf(repo_root().join(P41_RELEASE).join(path));
        assert_eq!(cutoff_bytes, checkout_bytes, "Git-clean Pulse 41 {path}");
        assert_eq!(
            cutoff_bytes.len() as u64,
            binding["size"].as_u64().expect("size")
        );
        assert_eq!(sha256(&cutoff_bytes), binding["sha256"], "Pulse 41 {path}");
    }
    assert_eq!(bound_paths, p41_paths);

    let manifest_bytes = cutoff_blob(&format!("{P41_RELEASE}/public-manifest.json"));
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("Pulse 41 manifest");
    assert_eq!(sha256(&manifest_bytes), P41_MANIFEST_RAW);
    assert_eq!(p41["manifest"]["raw_sha256"], P41_MANIFEST_RAW);
    assert_eq!(manifest["aggregate"], P41_MANIFEST_AGGREGATE);
    assert_eq!(manifest["file_count"], 5);
    assert_eq!(manifest["manifest_payload_file_count"], 5);
    assert_eq!(manifest["release_tree_file_count"], 8);
    assert_eq!(manifest["total_bytes"], 49_120);
    assert_eq!(manifest["files"], p41["manifest"]["files"]);
    let manifest_files = manifest["files"].as_array().expect("Pulse 41 files");
    assert_eq!(manifest_aggregate(manifest_files), P41_MANIFEST_AGGREGATE);
    assert_eq!(manifest_files.len(), 5);
    assert_eq!(
        manifest_files
            .iter()
            .map(|file| file["size"].as_u64().expect("size"))
            .sum::<u64>(),
        49_120
    );
    for file in manifest_files {
        let path = file["path"].as_str().expect("Pulse 41 manifest path");
        let bytes = cutoff_blob(&format!("{P41_RELEASE}/{path}"));
        assert_eq!(bytes.len() as u64, file["size"].as_u64().expect("size"));
        assert_eq!(sha256(&bytes), file["sha256"], "Pulse 41 manifest {path}");
    }

    for (name, raw, payload) in [
        ("root_cause_report", P41_REPORT_RAW, P41_REPORT_PAYLOAD),
        (
            "qualification_receipt",
            P41_RECEIPT_RAW,
            P41_RECEIPT_PAYLOAD,
        ),
        ("release_seal", P41_SEAL_RAW, P41_SEAL_PAYLOAD),
    ] {
        let record = &p41[name];
        let path = record["path"].as_str().expect("Pulse 41 record path");
        let bytes = cutoff_blob(path);
        let envelope: Value = serde_json::from_slice(&bytes).expect("Pulse 41 envelope");
        assert_eq!(sha256(&bytes), raw, "Pulse 41 {name} raw");
        assert_eq!(record["raw_sha256"], raw);
        assert_eq!(record["payload_sha256"], payload);
        assert_eq!(envelope["payload_sha256"], payload);
        assert_eq!(canonical_payload_sha256(&envelope["payload"]), payload);
    }

    let p39 = &declaration["pulse_39_public_checkout_verifier_release"];
    let p39_manifest_path = p39["manifest"]["path"]
        .as_str()
        .expect("Pulse 39 manifest path");
    let p39_manifest_bytes = cutoff_blob(p39_manifest_path);
    let p39_manifest: Value =
        serde_json::from_slice(&p39_manifest_bytes).expect("Pulse 39 manifest");
    assert_eq!(sha256(&p39_manifest_bytes), P39_MANIFEST_RAW);
    assert_eq!(p39["manifest"]["raw_sha256"], P39_MANIFEST_RAW);
    assert_eq!(p39_manifest["aggregate"], P39_MANIFEST_AGGREGATE);
    assert_eq!(p39_manifest["file_count"], 5);
    assert_eq!(p39_manifest["total_bytes"], 26_455);
    let p39_files = p39_manifest["files"].as_array().expect("Pulse 39 files");
    assert_eq!(p39_files.len(), 5);
    assert_eq!(manifest_aggregate(p39_files), P39_MANIFEST_AGGREGATE);
    assert_eq!(p39_manifest["files"], p39["manifest"]["files"]);
    let p39_root = p39_manifest["release_root"]
        .as_str()
        .expect("Pulse 39 root");
    assert_eq!(p39_root, P39_RELEASE);
    assert_eq!(
        cutoff_tree_paths(p39_root),
        BTreeSet::from_iter(P39_PATHS.map(str::to_owned))
    );
    assert_eq!(
        declared_paths(p39, "exact_release_tree_paths"),
        BTreeSet::from_iter(P39_PATHS.map(str::to_owned))
    );
    for path in P39_PATHS {
        let bytes = cutoff_blob(&format!("{p39_root}/{path}"));
        assert_eq!(bytes, read_lf(repo_root().join(p39_root).join(path)));
    }
    for file in p39_files {
        let path = file["path"].as_str().expect("Pulse 39 manifest path");
        let bytes = cutoff_blob(&format!("{p39_root}/{path}"));
        assert_eq!(sha256(&bytes), file["sha256"], "Pulse 39 manifest {path}");
        assert_eq!(bytes.len() as u64, file["size"].as_u64().expect("size"));
    }
    for (name, raw, payload) in [
        ("root_cause_report", P39_REPORT_RAW, P39_REPORT_PAYLOAD),
        (
            "qualification_receipt",
            P39_RECEIPT_RAW,
            P39_RECEIPT_PAYLOAD,
        ),
        ("release_seal", P39_SEAL_RAW, P39_SEAL_PAYLOAD),
    ] {
        let record = &p39[name];
        let path = record["path"].as_str().expect("Pulse 39 record path");
        let bytes = cutoff_blob(path);
        let envelope: Value = serde_json::from_slice(&bytes).expect("Pulse 39 envelope");
        assert_eq!(sha256(&bytes), raw, "Pulse 39 {name} raw");
        assert_eq!(record["raw_sha256"], raw);
        assert_eq!(record["payload_sha256"], payload);
        assert_eq!(envelope["payload_sha256"], payload);
        assert_eq!(canonical_payload_sha256(&envelope["payload"]), payload);
    }
    let receipt: Value = serde_json::from_slice(&cutoff_blob(
        p39["qualification_receipt"]["path"]
            .as_str()
            .expect("Pulse 39 receipt path"),
    ))
    .expect("Pulse 39 receipt");
    assert_eq!(receipt["payload"]["status_counts"]["expected_files"], 36);
    assert_eq!(
        receipt["payload"]["status_counts"]["attribute_files_passed"],
        36
    );
    assert_eq!(receipt["payload"]["line_endings"]["lf_files_passed"], 36);
    assert_eq!(receipt["payload"]["line_endings"]["cr_bytes_observed"], 0);
    assert_eq!(
        receipt["payload"]["git_process_accounting"]["total_git_processes"],
        2
    );
    assert_eq!(
        receipt["payload"]["git_process_accounting"]["check_attr_invocations"],
        1
    );
    assert_eq!(
        receipt["payload"]["git_process_accounting"]["git_version_probes"],
        1
    );
    assert_eq!(receipt["payload"]["git_process_accounting"]["retries"], 0);
    let seal: Value = serde_json::from_slice(&cutoff_blob(
        p39["release_seal"]["path"]
            .as_str()
            .expect("Pulse 39 seal path"),
    ))
    .expect("Pulse 39 seal");
    assert_eq!(
        seal["payload"]["verification"]["normalized_bindings"],
        "76/76"
    );

    let absence = git_output(&[
        "cat-file",
        "-e",
        &format!(
            "{CUTOFF}:docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-42-authority.json"
        ),
    ]);
    assert!(
        !absence.status.success(),
        "Pulse 42 must be absent from its cutoff"
    );
}

#[test]
fn pulse_42_mutations_indexes_and_lf_rules_are_complete() {
    let schema = read_json(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-42-authority.v1.schema.json"),
    );
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-42-authority.json"));
    let controls = read_json(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-42-authority-mutations.json"),
    );
    assert_eq!(controls["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(controls["mutation_count"], MUTATION_COUNT);
    let mutations = controls["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), MUTATION_COUNT);
    let mut ids = BTreeSet::new();
    let mut pointers = BTreeSet::new();
    for mutation in mutations {
        assert!(ids.insert(mutation["id"].as_str().expect("mutation ID")));
        pointers.insert(mutation["pointer"].as_str().expect("mutation pointer"));
        let mut candidate = declaration.clone();
        apply_mutation(&mut candidate, mutation);
        if mutation["recompute_identity"] == true {
            candidate["declaration_identity"] = Value::String(declaration_identity(&candidate));
        }
        assert_ne!(candidate, declaration, "no-op {}", mutation["id"]);
        assert_ne!(
            schema["$defs"]["exactAuthority"]["const"], candidate,
            "schema accepted {}",
            mutation["id"]
        );
    }
    for pointer in [
        "/immutable_ferris/cutoff",
        "/closed_predecessors/pulse_38/disposition",
        "/closed_predecessors/pulse_40/disposition",
        "/pulse_41_transactional_copy_release/manifest/raw_sha256",
        "/pulse_41_transactional_copy_release/manifest/aggregate",
        "/pulse_41_transactional_copy_release/release_tree_raw_bindings/0/sha256",
        "/pulse_41_transactional_copy_release/direct_adapter_execution/adapter_must_be_executed_directly_from_immutable_cutoff_checkout",
        "/pulse_41_transactional_copy_release/direct_adapter_execution/destination_file_fsyncs_expected",
        "/pulse_41_transactional_copy_release/direct_adapter_execution/staging_directory_sync_attempts_expected",
        "/pulse_41_transactional_copy_release/direct_adapter_execution/rename_attempts_expected",
        "/pulse_39_public_checkout_verifier_release/verifier_control/total_git_processes",
        "/pulse_39_public_checkout_verifier_release/normalized_binding_verification/binding_checks_total",
        "/pulse_42_bounded_transactional_search/total_processes_max",
        "/execution_state/pulse_41_adapter_invocations",
        "/execution_state/pulse_41_rollback_attempts",
        "/execution_state/pulse_39_verifier_invocations",
        "/execution_state/candidate_processes",
    ] {
        assert!(pointers.contains(pointer), "missing mutation {pointer}");
    }

    let attributes = fs::read_to_string(repo_root().join(".gitattributes")).expect("attributes");
    for rule in [
        "/docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_42_AUTHORITY.md text eol=lf",
        "/context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-42.md text eol=lf",
        "/docs/plans/reviews/PULSE-42-TRANSACTIONAL-COPY-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md text eol=lf",
        "/crates/ferris-cli/tests/process_exit_diagnostic_pulse_42_authority.rs text eol=lf",
    ] {
        assert!(
            attributes.lines().any(|line| line == rule),
            "missing {rule}"
        );
    }
    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(
        schema_index.contains("ferris.process-exit-diagnostic-pulse-42-authority.v1.schema.json")
    );
    assert!(schema_index.contains("All 25 schemas"));
    assert!(schema_index.contains("9046 mutations"));
    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    assert!(fixture_index.contains("process-exit-diagnostic-pulse-42-authority.json"));
    assert!(fixture_index.contains("9046 comprehensive rejection controls"));
    assert!(fixture_index.contains("29611 total declared mutations"));
    for relative in [
        "CONTEXT.md",
        "README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-42.md",
        "docs/simulations/profile-diff-held-out/README.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_42_AUTHORITY.md",
        "docs/plans/reviews/PULSE-42-TRANSACTIONAL-COPY-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 42 index");
        assert!(text.contains(CUTOFF), "{relative} cutoff");
        assert!(text.contains(DECLARATION_IDENTITY), "{relative} identity");
        assert!(
            text.contains(P41_MANIFEST_RAW),
            "{relative} Pulse 41 binding"
        );
        assert!(
            text.contains(P39_MANIFEST_RAW),
            "{relative} Pulse 39 binding"
        );
        assert!(text.contains("Pulse 38"), "{relative} Pulse 38 closure");
        assert!(text.contains("Pulse 40"), "{relative} Pulse 40 closure");
    }
    assert_eq!(20_565 + MUTATION_COUNT, TOTAL_DECLARED_MUTATIONS);
}
