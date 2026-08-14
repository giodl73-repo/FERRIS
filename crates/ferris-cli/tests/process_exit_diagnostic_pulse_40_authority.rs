use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-40-authority/v1";
const CUTOFF: &str = "65d1eec688f53bf7263ecfc8094ac849f9d3be4c";
const DECLARATION_IDENTITY: &str =
    "sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52";
const MUTATION_COUNT: usize = 9076;
const P39_RELEASE_TREE_FILE_COUNT: usize = 8;
const P39_MANIFEST_PAYLOAD_FILE_COUNT: usize = 5;
const P39_RELEASE_TREE_PATHS: [&str; P39_RELEASE_TREE_FILE_COUNT] = [
    "README.md",
    "checkout_verifier.py",
    "public-manifest.json",
    "qualification-receipt.json",
    "release-seal.json",
    "root-cause-report.json",
    "root-cause-report.md",
    "tests/test_checkout_verifier.py",
];
const PULSE_38_ID: &str = "sha256:a3317422e8c34d4e08d7c5e577e3539820f1376d7fba2ef38d262d1f967031b4";
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
const P29_RECEIPT_RAW: &str =
    "sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225";
const P29_RECEIPT_PAYLOAD: &str =
    "sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40";
const P35_MANIFEST_RAW: &str =
    "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1";
const P35_AGGREGATE: &str =
    "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69";
const P37_RECEIPT_RAW: &str =
    "sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6";
const P37_RECEIPT_ID: &str =
    "sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae";

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

fn read_git_clean(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read artifact");
    let mut clean = Vec::with_capacity(bytes.len());
    let mut source = bytes.into_iter();
    while let Some(byte) = source.next() {
        if byte == b'\r' {
            assert_eq!(source.next(), Some(b'\n'), "non-checkout CR in {path:?}");
            clean.push(b'\n');
        } else {
            clean.push(byte);
        }
    }
    assert!(!clean.contains(&b'\r'), "Git-clean {path:?} contains CR");
    clean
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

fn pulse_35_manifest_aggregate(entries: &[Value]) -> String {
    let mut digest = Sha256::new();
    for entry in entries {
        digest.update(entry["size"].as_u64().expect("size").to_string());
        digest.update(b"\0");
        digest.update(entry["path"].as_str().expect("path"));
        digest.update(b"\0");
        digest.update(
            entry["sha256"]
                .as_str()
                .expect("digest")
                .strip_prefix("sha256:")
                .expect("sha256 prefix"),
        );
        digest.update(b"\n");
    }
    format!("sha256:{:x}", digest.finalize())
}

fn pulse_39_manifest_aggregate(entries: &[Value]) -> String {
    let mut ordered = entries.to_vec();
    ordered.sort_by_key(|entry| entry["path"].as_str().expect("path").to_owned());
    let mut digest = Sha256::new();
    for entry in ordered {
        let path = entry["path"].as_str().expect("path");
        digest.update((path.len() as u64).to_be_bytes());
        digest.update(path.as_bytes());
        let hex = entry["sha256"]
            .as_str()
            .expect("digest")
            .strip_prefix("sha256:")
            .expect("sha256 prefix");
        for index in 0..32 {
            digest
                .update([u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16)
                    .expect("hexadecimal digest")]);
        }
    }
    format!("sha256:{:x}", digest.finalize())
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
            for (key, child) in object {
                assert_closed_schema(&schema["properties"][key], child);
            }
        }
        Value::Array(items) => {
            assert_eq!(schema["type"], "array");
            assert_eq!(schema["minItems"], items.len());
            assert_eq!(schema["maxItems"], items.len());
            assert_eq!(schema["items"], false);
            for (index, child) in items.iter().enumerate() {
                assert_closed_schema(&schema["prefixItems"][index], child);
            }
        }
        Value::Null => assert_eq!(schema["const"], Value::Null),
        _ => assert_eq!(schema["const"], *declaration),
    }
}

#[test]
fn pulse_40_authority_is_closed_unexecuted_and_inherits_pulse_38() {
    let schema: Value = serde_json::from_slice(&read_lf(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-40-authority.v1.schema.json"),
    ))
    .expect("parse schema");
    let declaration: Value = serde_json::from_slice(&read_lf(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-40-authority.json"),
    ))
    .expect("parse declaration");
    let pulse_38: Value = serde_json::from_slice(&read_lf(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-38-authority.json"),
    ))
    .expect("parse Pulse 38 declaration");

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
        declaration["immutable_ferris"]["cutoff_contains_pulse_39_checkout_verifier_release"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_pulse_40_authority"],
        false
    );
    assert_eq!(
        declaration["inherited_public_gates"],
        pulse_38["inherited_public_gates"]
    );
    assert_eq!(
        declaration["pulse_38_gate_baseline"]["authority_declaration_identity"],
        PULSE_38_ID
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_38"]["disposition"],
        "invalid-before-normalized-checkout-verification"
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_38"]["further_launches_prohibited"],
        true
    );
    for field in [
        "retry",
        "resume",
        "reseed",
        "reuse",
        "correlation",
        "inference",
    ] {
        assert_eq!(
            declaration["closed_predecessors"]["pulse_38"][field], false,
            "Pulse 38 {field}"
        );
        assert_eq!(declaration["authority"][format!("pulse_38_{field}")], false);
    }

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
    assert_eq!(
        declaration["execution_order"]["order"][0],
        "exact-pulse-39-public-verifier-release-custody-and-root-anchored-cutoff-checkout-verification"
    );
    assert_eq!(
        declaration["execution_order"]["order"][1],
        "pulse-25-pulse-27-public-package-custody"
    );
    assert_eq!(
        declaration["execution_order"]["order"][8],
        "one-pulse-40-bounded-transactional-cross-platform-search"
    );
    let search = &declaration["pulse_40_bounded_transactional_search"];
    assert_eq!(search["after_pulse_39_verifier_custody"], true);
    assert_eq!(search["cases_per_platform_max"], 70);
    assert_eq!(search["processes_per_platform_max"], 70);
    assert_eq!(search["total_processes_max"], 140);
    assert_eq!(search["candidate_retries"], 0);
    for field in [
        "pulse_39_copied_release_files",
        "pulse_39_manifest_payload_bindings_completed",
        "pulse_39_manifest_raw_bindings_completed",
        "pulse_39_qualification_receipt_bindings_completed",
        "pulse_39_release_seal_bindings_completed",
        "pulse_39_release_tree_raw_bindings_completed",
    ] {
        assert_eq!(state[field], 0, "Pulse 39 {field}");
    }
}

#[test]
fn pulse_40_recomputes_cutoff_git_blobs_and_all_pulse_39_identities() {
    let declaration: Value = serde_json::from_slice(&read_lf(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-40-authority.json"),
    ))
    .expect("parse declaration");
    let release = &declaration["pulse_39_public_checkout_verifier_release"];
    let manifest_path = release["manifest"]["path"].as_str().expect("manifest path");
    let manifest_bytes = cutoff_blob(manifest_path);
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse manifest");
    assert_eq!(sha256(&manifest_bytes), P39_MANIFEST_RAW);
    assert_eq!(release["manifest"]["raw_sha256"], P39_MANIFEST_RAW);
    assert_eq!(manifest["aggregate"], P39_MANIFEST_AGGREGATE);
    assert_eq!(
        release["release_tree_file_count"],
        P39_RELEASE_TREE_FILE_COUNT
    );
    assert_eq!(
        release["manifest_payload_file_count"],
        P39_MANIFEST_PAYLOAD_FILE_COUNT
    );
    assert_eq!(manifest["file_count"], P39_MANIFEST_PAYLOAD_FILE_COUNT);
    assert_eq!(manifest["total_bytes"], 26_455);
    assert_eq!(manifest["files"], release["manifest"]["files"]);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), P39_MANIFEST_PAYLOAD_FILE_COUNT);
    assert_eq!(pulse_39_manifest_aggregate(files), P39_MANIFEST_AGGREGATE);

    let release_root = manifest["release_root"].as_str().expect("release root");
    assert_eq!(release["manifest"]["release_root"], release_root);
    let expected_tree_paths = P39_RELEASE_TREE_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    let declared_tree_paths = release["exact_release_tree_paths"]
        .as_array()
        .expect("exact release tree paths")
        .iter()
        .map(|path| path.as_str().expect("release tree path").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(declared_tree_paths.len(), P39_RELEASE_TREE_FILE_COUNT);
    assert_eq!(declared_tree_paths, expected_tree_paths);
    assert_eq!(cutoff_tree_paths(release_root), expected_tree_paths);
    assert_eq!(
        release["copy_and_recomputation"]["before_package_copy"],
        true
    );
    assert_eq!(
        release["copy_and_recomputation"]["exact_release_file_bindings_required"],
        true
    );
    assert_eq!(
        release["copy_and_recomputation"]["independent_recomputation_required"],
        true
    );
    assert_eq!(
        release["copy_and_recomputation"]["lf_git_clean_bytes_required"],
        true
    );
    assert_eq!(
        release["copy_and_recomputation"]["all_release_tree_raw_bindings_recomputed_required"],
        true
    );
    assert_eq!(
        release["copy_and_recomputation"]["missing_release_tree_files_rejected"],
        true
    );
    assert_eq!(
        release["copy_and_recomputation"]["extra_release_tree_files_rejected"],
        true
    );

    let mut paths = BTreeSet::new();
    let mut raw_bindings = BTreeMap::new();
    raw_bindings.insert(
        "public-manifest.json".to_owned(),
        P39_MANIFEST_RAW.to_owned(),
    );
    let mut total = 0_u64;
    for entry in files {
        let path = entry["path"].as_str().expect("release path");
        assert!(!Path::new(path).is_absolute());
        assert!(!path.contains(".."));
        assert!(!path.contains('\\'));
        assert!(paths.insert(path), "duplicate release path");
        let bytes = cutoff_blob(&format!("{release_root}/{path}"));
        assert_eq!(bytes, read_lf(repo_root().join(release_root).join(path)));
        assert_eq!(sha256(&bytes), entry["sha256"], "{path} digest");
        assert_eq!(bytes.len() as u64, entry["size"].as_u64().expect("size"));
        raw_bindings.insert(
            path.to_owned(),
            entry["sha256"]
                .as_str()
                .expect("manifest raw binding")
                .to_owned(),
        );
        total += bytes.len() as u64;
    }
    assert_eq!(paths.len(), P39_MANIFEST_PAYLOAD_FILE_COUNT);
    assert_eq!(total, 26_455);

    for (record, raw, payload) in [
        ("root_cause_report", P39_REPORT_RAW, P39_REPORT_PAYLOAD),
        (
            "qualification_receipt",
            P39_RECEIPT_RAW,
            P39_RECEIPT_PAYLOAD,
        ),
        ("release_seal", P39_SEAL_RAW, P39_SEAL_PAYLOAD),
    ] {
        let record = &release[record];
        let path = record["path"].as_str().expect("record path");
        let relative_path = path
            .strip_prefix(&format!("{release_root}/"))
            .expect("record under release root");
        assert!(expected_tree_paths.contains(relative_path));
        let bytes = cutoff_blob(path);
        let envelope: Value = serde_json::from_slice(&bytes).expect("parse envelope");
        assert_eq!(sha256(&bytes), raw);
        assert_eq!(record["raw_sha256"], raw);
        assert_eq!(record["payload_sha256"], payload);
        assert_eq!(envelope["payload_sha256"], payload);
        assert_eq!(canonical_payload_sha256(&envelope["payload"]), payload);
        if let Some(previous) = raw_bindings.insert(relative_path.to_owned(), raw.to_owned()) {
            assert_eq!(previous, raw, "{relative_path} raw binding");
        }
    }
    assert_eq!(raw_bindings.len(), P39_RELEASE_TREE_FILE_COUNT);
    assert_eq!(
        raw_bindings.keys().cloned().collect::<BTreeSet<_>>(),
        expected_tree_paths
    );
    for path in P39_RELEASE_TREE_PATHS {
        let bytes = cutoff_blob(&format!("{release_root}/{path}"));
        assert_eq!(bytes, read_lf(repo_root().join(release_root).join(path)));
        let expected_raw = raw_bindings
            .get(path)
            .expect("release tree raw binding")
            .as_str();
        assert_eq!(sha256(&bytes).as_str(), expected_raw, "{path} raw binding");
    }
    let report = serde_json::from_slice::<Value>(&cutoff_blob(
        release["root_cause_report"]["path"]
            .as_str()
            .expect("report path"),
    ))
    .expect("parse report");
    assert_eq!(
        report["payload"]["cutoff"],
        "6807bd68aa01cbf0c819198765b7d6b5aa443328"
    );
    assert_eq!(report["payload"]["pulse_38"]["disposition"], "invalid");
    let receipt = serde_json::from_slice::<Value>(&cutoff_blob(
        release["qualification_receipt"]["path"]
            .as_str()
            .expect("receipt path"),
    ))
    .expect("parse receipt");
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
    let seal = serde_json::from_slice::<Value>(&cutoff_blob(
        release["release_seal"]["path"].as_str().expect("seal path"),
    ))
    .expect("parse seal");
    assert_eq!(
        seal["payload"]["verification"]["normalized_bindings"],
        "76/76"
    );
    assert_eq!(seal["payload"]["release_limits"]["ferris_execution"], false);

    let p29 = cutoff_blob(
        "docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json",
    );
    let p29: Value = serde_json::from_slice(&p29).expect("parse Pulse 29 receipt");
    assert_eq!(
        sha256(&cutoff_blob(
            "docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json"
        )),
        P29_RECEIPT_RAW
    );
    assert_eq!(p29["payload_sha256"], P29_RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&p29["payload"]),
        P29_RECEIPT_PAYLOAD
    );
    assert_eq!(p29["payload"]["binding_checks"]["passed"], 76);
    assert_eq!(p29["payload"]["binding_checks"]["failed"], 0);

    let pulse_35 = &declaration["pulse_35_public_corpus_materializer_release"];
    let p35_manifest_path = pulse_35["manifest"]["path"]
        .as_str()
        .expect("Pulse 35 manifest");
    let p35_manifest_bytes = cutoff_blob(p35_manifest_path);
    let p35_manifest: Value =
        serde_json::from_slice(&p35_manifest_bytes).expect("parse Pulse 35 manifest");
    assert_eq!(sha256(&p35_manifest_bytes), P35_MANIFEST_RAW);
    assert_eq!(p35_manifest["aggregate"], P35_AGGREGATE);
    let p35_files = p35_manifest["files"].as_array().expect("Pulse 35 files");
    assert_eq!(p35_files.len(), 8);
    assert_eq!(p35_manifest["total_bytes"], 403_316);
    assert_eq!(pulse_35_manifest_aggregate(p35_files), P35_AGGREGATE);
    let p35_root = p35_manifest_path.rsplit_once('/').expect("Pulse 35 root").0;
    for file in p35_files {
        let path = file["path"].as_str().expect("Pulse 35 path");
        let bytes = cutoff_blob(&format!("{p35_root}/{path}"));
        assert_eq!(sha256(&bytes), file["sha256"], "{path} digest");
        assert_eq!(bytes, read_git_clean(repo_root().join(p35_root).join(path)));
    }
    let p37_path = pulse_35["normalized_pulse_37_proof"]["receipt_path"]
        .as_str()
        .expect("Pulse 37 receipt");
    let p37 = cutoff_blob(p37_path);
    let p37: Value = serde_json::from_slice(&p37).expect("parse Pulse 37 receipt");
    assert_eq!(sha256(&cutoff_blob(p37_path)), P37_RECEIPT_RAW);
    assert_eq!(p37["receipt_id"], P37_RECEIPT_ID);
    assert_eq!(canonical_payload_sha256(&p37["payload"]), P37_RECEIPT_ID);
    assert_eq!(p37["payload"]["binding_checks"]["files_passed"], 8);

    let absence = git_output(&[
        "cat-file",
        "-e",
        &format!(
            "{CUTOFF}:docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-40-authority.json"
        ),
    ]);
    assert!(
        !absence.status.success(),
        "authority must be absent from cutoff"
    );
}

#[test]
fn pulse_40_mutations_and_indexes_cover_the_complete_authority() {
    let schema: Value = serde_json::from_slice(&read_lf(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-40-authority.v1.schema.json"),
    ))
    .expect("parse schema");
    let declaration: Value = serde_json::from_slice(&read_lf(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-40-authority.json"),
    ))
    .expect("parse declaration");
    let controls: Value = serde_json::from_slice(&read_lf(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-40-authority-mutations.json"),
    ))
    .expect("parse mutation controls");
    let mutations = controls["mutations"].as_array().expect("mutation list");
    assert_eq!(mutations.len(), MUTATION_COUNT);
    let mut ids = BTreeSet::new();
    let pointers = mutations
        .iter()
        .map(|mutation| {
            assert!(ids.insert(mutation["id"].as_str().expect("mutation id")));
            let mut candidate = declaration.clone();
            apply_mutation(&mut candidate, mutation);
            if mutation["recompute_identity"] == true {
                candidate["declaration_identity"] = Value::String(declaration_identity(&candidate));
            }
            assert!(
                candidate != declaration,
                "no-op mutation {}",
                mutation["id"]
            );
            assert!(
                schema["$defs"]["exactAuthority"]["const"] != candidate,
                "schema accepted {}",
                mutation["id"]
            );
            mutation["pointer"].as_str().expect("mutation pointer")
        })
        .collect::<Vec<_>>();
    for required in [
        "/immutable_ferris/cutoff",
        "/immutable_ferris/cutoff_contains_pulse_39_checkout_verifier_release",
        "/closed_predecessors/pulse_38/disposition",
        "/pulse_39_public_checkout_verifier_release/release_tree_file_count",
        "/pulse_39_public_checkout_verifier_release/manifest_payload_file_count",
        "/pulse_39_public_checkout_verifier_release/exact_release_tree_paths/0",
        "/pulse_39_public_checkout_verifier_release/exact_release_tree_paths/7",
        "/pulse_39_public_checkout_verifier_release/manifest/raw_sha256",
        "/pulse_39_public_checkout_verifier_release/manifest/files/0/sha256",
        "/pulse_39_public_checkout_verifier_release/root_cause_report/payload_sha256",
        "/pulse_39_public_checkout_verifier_release/qualification_receipt/payload_sha256",
        "/pulse_39_public_checkout_verifier_release/release_seal/payload_sha256",
        "/pulse_39_public_checkout_verifier_release/copy_and_recomputation/all_release_tree_raw_bindings_recomputed_required",
        "/pulse_39_public_checkout_verifier_release/copy_and_recomputation/missing_release_tree_files_rejected",
        "/pulse_39_public_checkout_verifier_release/copy_and_recomputation/extra_release_tree_files_rejected",
        "/pulse_39_public_checkout_verifier_release/verifier_control/total_git_processes",
        "/pulse_39_public_checkout_verifier_release/normalized_binding_verification/binding_checks_total",
        "/pulse_35_public_corpus_materializer_release/manifest/raw_sha256",
        "/pulse_35_public_corpus_materializer_release/private_seed/exact_byte_count",
        "/pulse_40_bounded_transactional_search/cases_per_platform_max",
        "/pulse_40_bounded_transactional_search/total_processes_max",
        "/execution_state/candidate_processes",
        "/execution_state/pulse_39_copied_release_files",
        "/execution_state/pulse_39_release_tree_raw_bindings_completed",
    ] {
        assert!(
            pointers.contains(&required),
            "missing mutation control {required}"
        );
    }

    let attributes = fs::read_to_string(repo_root().join(".gitattributes")).expect("attributes");
    for rule in [
        "/docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_40_AUTHORITY.md text eol=lf",
        "/context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-40.md text eol=lf",
        "/docs/plans/reviews/PULSE-40-VERIFIER-CUSTODY-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md text eol=lf",
        "/crates/ferris-cli/tests/process_exit_diagnostic_pulse_40_authority.rs text eol=lf",
    ] {
        assert!(
            attributes.lines().any(|line| line == rule),
            "missing {rule}"
        );
    }
    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(
        schema_index.contains("ferris.process-exit-diagnostic-pulse-40-authority.v1.schema.json")
    );
    assert!(schema_index.contains("All 24 schemas"));
    assert!(schema_index.contains("9076 mutations"));
    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    assert!(fixture_index.contains("process-exit-diagnostic-pulse-40-authority.json"));
    assert!(fixture_index.contains("9076 rejection controls"));
    assert!(fixture_index.contains("20565 total declared mutations"));
    for relative in [
        "CONTEXT.md",
        "README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-40.md",
        "docs/simulations/profile-diff-held-out/README.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_40_AUTHORITY.md",
        "docs/plans/reviews/PULSE-40-VERIFIER-CUSTODY-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 40 index");
        assert!(text.contains(CUTOFF), "{relative} cutoff");
        assert!(text.contains(DECLARATION_IDENTITY), "{relative} identity");
        assert!(
            text.contains(P39_MANIFEST_RAW),
            "{relative} Pulse 39 manifest"
        );
        assert!(text.contains("Pulse 38"), "{relative} preserves Pulse 38");
    }
}
