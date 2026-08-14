use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-38-authority/v1";
const CUTOFF: &str = "6807bd68aa01cbf0c819198765b7d6b5aa443328";
const DECLARATION_IDENTITY: &str =
    "sha256:a3317422e8c34d4e08d7c5e577e3539820f1376d7fba2ef38d262d1f967031b4";
const MUTATION_COUNT: usize = 7288;
const NORMALIZED_MANIFEST_RAW: &str =
    "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1";
const NORMALIZED_AGGREGATE: &str =
    "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69";
const NORMALIZED_SEAL_RAW: &str =
    "sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23";
const NORMALIZED_SEAL_PAYLOAD: &str =
    "sha256:834781867ea008dc14a54d7b811002ee1b8fa759c0b1d7f32432ea6c0d5c5375";
const PULSE_37_RECEIPT_RAW: &str =
    "sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6";
const PULSE_37_RECEIPT_ID: &str =
    "sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae";
const PULSE_35_QUALIFICATION_RAW: &str =
    "sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037";
const PULSE_35_QUALIFICATION_PAYLOAD: &str =
    "sha256:7f1154ca94009cef966ab2f43ba74a9f017989ed5dbbdbfd8c3ce8fe64fe5cee";
const PULSE_35_MACHINE_SCHEMA_RELEASE_RAW: &str =
    "sha256:d85cea956a2cf82d0bf360cbccda2d19c25705c3c17f8d2a255a8dc11852825b";
const PULSE_35_MACHINE_SCHEMA_GIT_BLOB_RAW: &str =
    "sha256:3543c1d83815e0d6b2fcaee3ee14bca4ec13f1a9ef02102993ffa9edbb7c08f9";
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

fn manifest_aggregate(entries: &[Value]) -> String {
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
                    .expect("remove target array")
                    .remove(key.parse::<usize>().expect("array index"));
            }
        }
        operation => panic!("unsupported mutation {operation}"),
    }
}

#[test]
fn pulse_38_authority_is_closed_unexecuted_and_preserves_all_invalid_predecessors() {
    let (_, schema): (Vec<u8>, Value) = {
        let bytes = read_git_clean(
            held_out_root()
                .join("schemas/ferris.process-exit-diagnostic-pulse-38-authority.v1.schema.json"),
        );
        let value = serde_json::from_slice(&bytes).expect("parse schema");
        (bytes, value)
    };
    let declaration_path =
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-38-authority.json");
    let declaration_bytes = read_git_clean(&declaration_path);
    let declaration: Value = serde_json::from_slice(&declaration_bytes).expect("parse declaration");

    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["$ref"], "#/$defs/exactAuthority");
    assert_eq!(schema["$defs"]["exactAuthority"]["const"], declaration);
    assert_eq!(declaration["schema"], DOMAIN);
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration_identity(&declaration), DECLARATION_IDENTITY);
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_pulse_37_normalization"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_pulse_38_authority"],
        false
    );

    for pulse in [
        "pulse_22", "pulse_24", "pulse_26", "pulse_28", "pulse_30", "pulse_32", "pulse_34",
    ] {
        let predecessor = &declaration["closed_predecessors"][pulse];
        assert_eq!(predecessor["disposition"], "invalid", "{pulse}");
        assert_eq!(predecessor["candidate_retries"], 0, "{pulse}");
        assert_eq!(predecessor["category_conclusion"], Value::Null, "{pulse}");
        assert_eq!(predecessor["permanently_closed"], true, "{pulse}");
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
    let pulse_36 = &declaration["closed_predecessors"]["pulse_36"];
    assert_eq!(pulse_36["public_label"], "invalid");
    assert_eq!(
        pulse_36["disposition"],
        "invalid-before-pulse35-materialization"
    );
    assert_eq!(pulse_36["stop_stage"], "pulse35-release-copy-verification");
    assert_eq!(pulse_36["further_launches_prohibited"], true);
    for field in [
        "retry",
        "resume",
        "reseed",
        "reuse",
        "correlation",
        "inference",
    ] {
        assert_eq!(declaration["authority"][field], false, "authority {field}");
        let pulse_36_field = format!("pulse_36_{field}");
        assert_eq!(declaration["authority"][pulse_36_field.as_str()], false);
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
}

#[test]
fn pulse_38_inherits_pulse_36_and_pulse_34_gates_and_binds_normalized_release() {
    let declaration: Value = serde_json::from_slice(&read_git_clean(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-38-authority.json"),
    ))
    .expect("parse declaration");
    let p36: Value = serde_json::from_slice(&read_git_clean(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-36-authority.json"),
    ))
    .expect("parse Pulse 36 authority");

    assert_eq!(
        declaration["inherited_public_gates"],
        p36["inherited_public_gates"]
    );
    let baseline = &declaration["pulse_36_gate_baseline"];
    assert_eq!(
        baseline["declaration_identity"],
        p36["declaration_identity"]
    );
    assert_eq!(baseline["gates"], p36["inherited_public_gates"]);
    assert_eq!(
        baseline["pulse_34_gate_baseline"],
        p36["pulse_34_gate_baseline"]
    );
    assert_eq!(baseline["execution_order"], p36["execution_order"]);

    let gates = &declaration["inherited_public_gates"];
    assert_eq!(
        gates["checkout_normalization"]["git_attributes"]["file_count"],
        36
    );
    assert_eq!(
        gates["checkout_normalization"]["binding_checks"]["passed_required"],
        76
    );
    assert_eq!(gates["public_adapter_release"]["file_count"], 20);
    assert_eq!(
        gates["public_build_freeze_release"]["manifest"]["file_count"],
        37
    );
    assert_eq!(gates["cutoff_build_freeze"]["exact_platform_count"], 2);
    assert_eq!(gates["preflight"]["exact_adapter_invocations"], 1);
    assert_eq!(gates["preflight"]["exact_pair_count"], 2);
    assert_eq!(gates["preflight"]["exact_process_rows"], 4);
    assert_eq!(gates["preflight"]["exact_pair_seals"], 2);
    assert_eq!(gates["preflight"]["exact_fresh_verifier_processes"], 2);
    assert_eq!(
        gates["preflight"]["whole_store_cardinality"]["windows_rows"],
        2
    );
    assert_eq!(
        gates["public_input_preflight"]["self_validation_total_classifications_required"],
        39
    );
    assert_eq!(gates["search_bounds"]["logical_case_max"], 512);
    assert_eq!(gates["search_bounds"]["search_process_max"], 1024);
    assert_eq!(gates["minimization"]["process_max"], 256);
    assert_eq!(gates["publication"]["fix_authority"], false);

    let release = &declaration["pulse_35_public_corpus_materializer_release"];
    assert_eq!(release["source_commit"], CUTOFF);
    assert_eq!(release["manifest"]["raw_sha256"], NORMALIZED_MANIFEST_RAW);
    assert_eq!(release["manifest"]["aggregate"], NORMALIZED_AGGREGATE);
    assert_eq!(release["manifest"]["file_count"], 8);
    assert_eq!(release["manifest"]["total_bytes"], 403_316);
    assert_eq!(release["release_seal"]["raw_sha256"], NORMALIZED_SEAL_RAW);
    assert_eq!(
        release["release_seal"]["payload_sha256"],
        NORMALIZED_SEAL_PAYLOAD
    );
    assert_eq!(
        release["qualification_receipt"]["raw_sha256"],
        PULSE_35_QUALIFICATION_RAW
    );
    assert_eq!(
        release["qualification_receipt"]["payload_sha256"],
        PULSE_35_QUALIFICATION_PAYLOAD
    );
    assert_eq!(
        release["machine_schema"]["raw_sha256"],
        PULSE_35_MACHINE_SCHEMA_RELEASE_RAW
    );
    assert_eq!(
        release["machine_schema"]["cutoff_git_blob_raw_sha256"],
        PULSE_35_MACHINE_SCHEMA_GIT_BLOB_RAW
    );
    assert_eq!(release["qualification_receipt"]["cycles_required"], 20);
    assert_eq!(release["private_seed"]["exact_byte_count"], 32);
    assert_eq!(release["private_seed"]["source"], "csprng");
    assert_eq!(release["private_seed"]["seed_value"], Value::Null);
    assert_eq!(
        release["private_seed"]["private_seed_required_for_verification"],
        true
    );
    assert_eq!(
        release["private_seed"]["derivation"],
        "hmac-sha256-seed-key-domain-purpose-counter-v1"
    );

    let materialization = &release["materialization_requirements"];
    assert_eq!(materialization["descriptor_count_required"], 70);
    assert_eq!(materialization["coverage_domains_required"], "18/18");
    assert_eq!(materialization["coverage_interactions_required"], "8/8");
    assert_eq!(
        materialization["exact_tuple_counts"],
        serde_json::json!([20, 12, 54, 6, 33, 20, 6, 4])
    );
    assert_eq!(materialization["publication_replacements"], 1);
    assert_eq!(materialization["publication_logical_retries"], 0);
    assert_eq!(
        materialization["directory_sync_statuses"],
        serde_json::json!(["synced", "unsupported"])
    );

    let search = &declaration["pulse_38_bounded_transactional_search"];
    assert_eq!(search["after_normalized_copy_and_pulse_37_proof"], true);
    assert_eq!(
        search["after_new_private_seed_materialization_and_fresh_verification"],
        true
    );
    assert_eq!(search["one_search_execution_max"], 1);
    assert_eq!(search["cases_per_platform_max"], 70);
    assert_eq!(search["processes_per_platform_max"], 70);
    assert_eq!(search["total_processes_max"], 140);
    assert_eq!(search["candidate_retries"], 0);
    assert_eq!(search["stop_after_first_target_mismatch_pair"], true);
    assert_eq!(search["inherited_minimization_required"], true);
    assert_eq!(search["inherited_publication_required"], true);

    assert_eq!(
        declaration["execution_order"]["order"],
        serde_json::json!([
            "normalized-checkout-and-pulse-25-pulse-27-public-package-custody",
            "pulse-33-build-freeze-and-cutoff-binary-freeze",
            "exact-adapter-preflight",
            "pulse-31-public-input-validation",
            "exact-normalized-eight-file-pulse-35-copy-and-recomputation",
            "pulse-37-clean-filter-proof-and-exact-git-blob-binding",
            "new-private-32-byte-csprng-seed-and-materialization",
            "fresh-process-private-seed-materialization-verification",
            "one-pulse-38-bounded-transactional-cross-platform-search",
            "inherited-minimization-and-result-publication-or-stop",
        ])
    );
}

#[test]
fn pulse_38_recomputes_normalized_cutoff_git_blobs_and_pulse_37_proof() {
    let declaration: Value = serde_json::from_slice(&read_git_clean(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-38-authority.json"),
    ))
    .expect("parse declaration");
    let release = &declaration["pulse_35_public_corpus_materializer_release"];
    let manifest_path = release["manifest"]["path"].as_str().expect("manifest path");
    let manifest_bytes = cutoff_blob(manifest_path);
    assert_eq!(sha256(&manifest_bytes), NORMALIZED_MANIFEST_RAW);
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("parse manifest");
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 8);
    assert_eq!(manifest["aggregate"], NORMALIZED_AGGREGATE);
    assert_eq!(manifest["total_bytes"], 403_316);
    assert_eq!(manifest_aggregate(files), NORMALIZED_AGGREGATE);

    let proof = &release["normalized_pulse_37_proof"];
    assert_eq!(proof["raw_sha256"], PULSE_37_RECEIPT_RAW);
    assert_eq!(proof["receipt_identity"], PULSE_37_RECEIPT_ID);
    assert_eq!(proof["files_expected"], 8);
    assert_eq!(proof["files_passed"], 8);
    assert_eq!(proof["size_hash_bindings_passed"], 8);
    assert_eq!(proof["text_files_lf_passed"], 6);
    assert_eq!(proof["text_cr_bytes_observed"], 0);
    assert_eq!(proof["all_release_file_cr_bytes_observed"], 0);
    assert_eq!(
        proof["unchanged_json_files"],
        serde_json::json!(["qualification-receipt.json", "root-cause-report.json"])
    );

    let release_root = manifest_path.rsplit_once('/').expect("release parent").0;
    let mut total = 0_u64;
    let mut paths = BTreeSet::new();
    for entry in files {
        let path = entry["path"].as_str().expect("file path");
        assert!(paths.insert(path), "duplicate manifest path");
        let cutoff = cutoff_blob(&format!("{release_root}/{path}"));
        let current = read_git_clean(repo_root().join(release_root).join(path));
        assert_eq!(current, cutoff, "{path} must equal exact cutoff Git blob");
        assert_eq!(cutoff.len() as u64, entry["size"].as_u64().expect("size"));
        assert_eq!(sha256(&cutoff), entry["sha256"], "{path} digest");
        total += cutoff.len() as u64;
    }
    assert_eq!(total, 403_316);

    for path in proof["text_files"].as_array().expect("text files") {
        let path = path.as_str().expect("text path");
        let bytes = cutoff_blob(&format!("{release_root}/{path}"));
        assert!(!bytes.contains(&b'\r'), "{path} must be LF-only");
    }
    for path in ["qualification-receipt.json", "root-cause-report.json"] {
        let current = read_git_clean(repo_root().join(release_root).join(path));
        assert_eq!(
            current,
            cutoff_blob(&format!("{release_root}/{path}")),
            "unchanged JSON {path}"
        );
    }

    let seal_path = release["release_seal"]["path"].as_str().expect("seal path");
    let seal_bytes = cutoff_blob(seal_path);
    assert_eq!(sha256(&seal_bytes), NORMALIZED_SEAL_RAW);
    let seal: Value = serde_json::from_slice(&seal_bytes).expect("parse seal");
    assert_eq!(seal["payload_sha256"], NORMALIZED_SEAL_PAYLOAD);
    assert_eq!(seal["receipt_id"], NORMALIZED_SEAL_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&seal["payload"]),
        NORMALIZED_SEAL_PAYLOAD
    );

    let qualification = cutoff_blob(&format!("{release_root}/qualification-receipt.json"));
    assert_eq!(sha256(&qualification), PULSE_35_QUALIFICATION_RAW);
    let qualification: Value = serde_json::from_slice(&qualification).expect("parse qualification");
    assert_eq!(
        qualification["payload_sha256"],
        PULSE_35_QUALIFICATION_PAYLOAD
    );
    assert_eq!(
        canonical_payload_sha256(&qualification["payload"]),
        PULSE_35_QUALIFICATION_PAYLOAD
    );
    assert_eq!(qualification["payload"]["case_count_per_cycle"], 70);

    let schema = cutoff_blob(
        "docs/simulations/profile-diff-held-out/schemas/ferris.pulse-35-corpus-materializer.v1.schema.json",
    );
    assert_eq!(sha256(&schema), PULSE_35_MACHINE_SCHEMA_GIT_BLOB_RAW);
    let schema: Value = serde_json::from_slice(&schema).expect("parse machine schema");
    assert_eq!(
        schema["$defs"]["caseManifest"]["properties"]["case_count"]["const"],
        70
    );

    let receipt_path = proof["receipt_path"].as_str().expect("receipt path");
    let receipt_bytes = cutoff_blob(receipt_path);
    assert_eq!(sha256(&receipt_bytes), PULSE_37_RECEIPT_RAW);
    let receipt: Value = serde_json::from_slice(&receipt_bytes).expect("parse Pulse 37 receipt");
    assert_eq!(receipt["receipt_id"], PULSE_37_RECEIPT_ID);
    assert_eq!(receipt["payload_sha256"], PULSE_37_RECEIPT_ID);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        PULSE_37_RECEIPT_ID
    );
    assert_eq!(receipt["payload"]["binding_checks"]["files_passed"], 8);
    assert_eq!(
        receipt["payload"]["line_endings"]["text_cr_bytes_observed"],
        0
    );

    let absence = git_output(&[
        "cat-file",
        "-e",
        &format!(
            "{CUTOFF}:docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-38-authority.json"
        ),
    ]);
    assert!(
        !absence.status.success(),
        "authority must be absent from cutoff"
    );
}

#[test]
fn pulse_38_mutation_controls_reject_weakening_and_indexes_are_consistent() {
    let schema: Value = serde_json::from_slice(&read_git_clean(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-38-authority.v1.schema.json"),
    ))
    .expect("parse schema");
    let declaration: Value = serde_json::from_slice(&read_git_clean(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-38-authority.json"),
    ))
    .expect("parse declaration");
    let mutations: Value = serde_json::from_slice(&read_git_clean(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-38-authority-mutations.json"),
    ))
    .expect("parse mutations");
    let mutations = mutations["mutations"].as_array().expect("mutation list");
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
            assert_ne!(candidate, declaration, "no-op mutation {}", mutation["id"]);
            assert_ne!(
                schema["$defs"]["exactAuthority"]["const"], candidate,
                "schema accepted {}",
                mutation["id"]
            );
            mutation["pointer"].as_str().expect("mutation pointer")
        })
        .collect::<Vec<_>>();
    for required in [
        "/immutable_ferris/cutoff",
        "/closed_predecessors/pulse_36/disposition",
        "/pulse_35_public_corpus_materializer_release/manifest/raw_sha256",
        "/pulse_35_public_corpus_materializer_release/normalized_pulse_37_proof/receipt_identity",
        "/pulse_35_public_corpus_materializer_release/materialization_requirements/exact_tuple_counts/0",
        "/pulse_35_public_corpus_materializer_release/private_seed/exact_byte_count",
        "/pulse_38_bounded_transactional_search/cases_per_platform_max",
        "/pulse_38_bounded_transactional_search/total_processes_max",
        "/execution_state/candidate_processes",
    ] {
        assert!(
            pointers.contains(&required),
            "missing mutation control {required}"
        );
    }

    let attributes = fs::read_to_string(repo_root().join(".gitattributes")).expect("attributes");
    for rule in [
        "/docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_38_AUTHORITY.md text eol=lf",
        "/context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-38.md text eol=lf",
        "/docs/plans/reviews/PULSE-38-NORMALIZED-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md text eol=lf",
        "/crates/ferris-cli/tests/process_exit_diagnostic_pulse_38_authority.rs text eol=lf",
    ] {
        assert!(
            attributes.lines().any(|line| line == rule),
            "missing {rule}"
        );
    }
    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(
        schema_index.contains("ferris.process-exit-diagnostic-pulse-38-authority.v1.schema.json")
    );
    assert!(schema_index.contains("All 23 schemas"));
    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    assert!(fixture_index.contains("process-exit-diagnostic-pulse-38-authority.json"));
    assert!(fixture_index.contains("7288 rejection controls"));
    assert!(fixture_index.contains("11489 total declared mutations"));
    for relative in [
        "CONTEXT.md",
        "README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-38.md",
        "docs/simulations/profile-diff-held-out/README.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_38_AUTHORITY.md",
        "docs/plans/reviews/PULSE-38-NORMALIZED-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 38 document");
        assert!(text.contains(CUTOFF), "{relative} cutoff");
        assert!(text.contains(DECLARATION_IDENTITY), "{relative} identity");
        assert!(text.contains("Pulse 36"), "{relative} preserves Pulse 36");
        assert!(text.contains("Pulse 37"), "{relative} binds Pulse 37");
    }
}
