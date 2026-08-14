use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-pulse-36-authority/v1";
const CUTOFF: &str = "48697c8da0e93b92fa633e353925ca05707bf9ed";
const DECLARATION_IDENTITY: &str =
    "sha256:f4d83498f780e6d35bd0073f8d8ddeaa67d99fb2426978190f7af25fff746952";
const MUTATION_COUNT: usize = 1998;
const PULSE_35_MANIFEST: &str =
    "sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b";
const PULSE_35_AGGREGATE: &str =
    "sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7";
const PULSE_35_QUALIFICATION: &str =
    "sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037";
const PULSE_35_QUALIFICATION_PAYLOAD: &str =
    "sha256:7f1154ca94009cef966ab2f43ba74a9f017989ed5dbbdbfd8c3ce8fe64fe5cee";
const PULSE_35_SEAL: &str =
    "sha256:51edf2f2df9210291705332fa8a4c3b55cb2a19a1aff22ecd882434a5ebefef2";
const PULSE_35_SEAL_PAYLOAD: &str =
    "sha256:5b5e4383ffe5274f36f355069a5339c1684674aea342229f54f63ef247d21e52";
const PULSE_35_SCHEMA: &str =
    "sha256:d85cea956a2cf82d0bf360cbccda2d19c25705c3c17f8d2a255a8dc11852825b";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR bytes");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end with LF");
    bytes
}

fn read_lf_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
    (bytes, value)
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
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

fn git_output(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run git")
}

fn git_blob(path: &str) -> Vec<u8> {
    let output = git_output(&["show", &format!("{CUTOFF}:{path}")]);
    assert!(output.status.success(), "cutoff artifact missing: {path}");
    output.stdout
}

fn manifest_aggregate(entries: &[Value]) -> String {
    let mut digest = Sha256::new();
    for entry in entries {
        digest.update(entry["size"].as_u64().expect("manifest size").to_string());
        digest.update(b"\0");
        digest.update(entry["path"].as_str().expect("manifest path"));
        digest.update(b"\0");
        digest.update(
            entry["sha256"]
                .as_str()
                .expect("manifest digest")
                .strip_prefix("sha256:")
                .expect("digest prefix"),
        );
        digest.update(b"\n");
    }
    format!("sha256:{:x}", digest.finalize())
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            let (parent, key) = pointer.rsplit_once('/').expect("mutation parent");
            let parent = if parent.is_empty() {
                value
            } else {
                value.pointer_mut(parent).expect("mutation target parent")
            };
            if let Some(object) = parent.as_object_mut() {
                object.insert(key.to_owned(), mutation["value"].clone());
            } else {
                parent.as_array_mut().expect("mutation target array")
                    [key.parse::<usize>().expect("array index")] = mutation["value"].clone();
            }
        }
        "remove" => {
            let (parent, key) = pointer.rsplit_once('/').expect("remove parent");
            let parent = if parent.is_empty() {
                value
            } else {
                value.pointer_mut(parent).expect("remove target parent")
            };
            if let Some(object) = parent.as_object_mut() {
                object.remove(key);
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
fn pulse_36_authority_is_closed_unexecuted_and_preserves_invalid_predecessors() {
    let (_, schema) = read_lf_json(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-36-authority.v1.schema.json"),
    );
    let (_, declaration) = read_lf_json(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-36-authority.json"),
    );

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
        declaration["immutable_ferris"]["cutoff_contains_complete_pulse_35_release"],
        true
    );
    assert_eq!(
        declaration["immutable_ferris"]["cutoff_contains_pulse_36_authority"],
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
    assert_eq!(state["materialized_descriptors"], 0);
    assert_eq!(state["candidate_processes"], 0);
    assert_eq!(state["category_conclusion"], Value::Null);
}

#[test]
fn pulse_36_inherits_pulse_34_and_binds_the_exact_pulse_35_release() {
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-36-authority.json"));
    let pulse_34 =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-public-authority.json"));
    let baseline = &declaration["pulse_34_gate_baseline"];
    assert_eq!(
        baseline["declaration_identity"],
        pulse_34["declaration_identity"]
    );
    assert_eq!(baseline["cutoff"], pulse_34["immutable_ferris"]["cutoff"]);
    for field in [
        "checkout_normalization",
        "public_adapter_release",
        "public_build_freeze_release",
        "cutoff_build_freeze",
        "preflight",
        "public_profile_evidence_input",
        "public_input_preflight",
        "seed_control",
        "coverage",
        "oracle",
        "collection",
        "search_bounds",
        "minimization",
        "publication",
        "platforms",
        "disclosure",
    ] {
        assert_eq!(
            baseline["gates"][field], pulse_34[field],
            "Pulse 34 {field}"
        );
    }

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
    assert_eq!(
        gates["search_bounds"]["stop_after_first_target_mismatch_pair"],
        true
    );

    let release = &declaration["pulse_35_public_corpus_materializer_release"];
    assert_eq!(release["source_commit"], CUTOFF);
    assert_eq!(release["manifest"]["raw_sha256"], PULSE_35_MANIFEST);
    assert_eq!(release["manifest"]["aggregate"], PULSE_35_AGGREGATE);
    assert_eq!(release["manifest"]["file_count"], 8);
    assert_eq!(release["manifest"]["total_bytes"], 405_414);
    assert_eq!(release["release_seal"]["raw_sha256"], PULSE_35_SEAL);
    assert_eq!(
        release["release_seal"]["payload_sha256"],
        PULSE_35_SEAL_PAYLOAD
    );
    assert_eq!(
        release["qualification_receipt"]["raw_sha256"],
        PULSE_35_QUALIFICATION
    );
    assert_eq!(
        release["qualification_receipt"]["payload_sha256"],
        PULSE_35_QUALIFICATION_PAYLOAD
    );
    assert_eq!(release["machine_schema"]["raw_sha256"], PULSE_35_SCHEMA);
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
    assert_eq!(
        declaration["execution_order"]["search_before_materialization_verification"],
        false
    );
    assert_eq!(
        declaration["execution_order"]["candidate_launch_before_materialization_verification"],
        false
    );
}

#[test]
fn pulse_36_recomputes_every_pulse_35_cutoff_file_and_envelope() {
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-36-authority.json"));
    let release = &declaration["pulse_35_public_corpus_materializer_release"];
    let manifest_path = release["manifest"]["path"].as_str().expect("manifest path");
    let manifest_bytes = git_blob(manifest_path);
    assert_eq!(sha256(&manifest_bytes), PULSE_35_MANIFEST);
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("Pulse 35 manifest");
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 8);
    assert_eq!(manifest["total_bytes"], 405_414);
    assert_eq!(manifest_aggregate(files), PULSE_35_AGGREGATE);

    let release_root = manifest_path
        .rsplit_once('/')
        .expect("Pulse 35 manifest parent")
        .0;
    let mut listed = BTreeSet::new();
    let mut actual = BTreeSet::new();
    let mut observed_total = 0_u64;
    let mut matched = 0_u64;
    let mut mismatched = 0_u64;
    for entry in files {
        let relative = entry["path"].as_str().expect("release file path");
        assert!(
            listed.insert(relative.to_owned()),
            "duplicate manifest path"
        );
        let bytes = git_blob(&format!("{release_root}/{relative}"));
        observed_total += bytes.len() as u64;
        if bytes.len() as u64 == entry["size"] && sha256(&bytes) == entry["sha256"] {
            matched += 1;
        } else {
            mismatched += 1;
        }
        actual.insert(relative.to_owned());
    }
    assert_eq!(actual, listed);
    assert_eq!(observed_total, 403_316);
    assert_eq!(matched, 2);
    assert_eq!(mismatched, 6);

    for (path, raw, payload) in [
        (
            "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/qualification-receipt.json",
            PULSE_35_QUALIFICATION,
            PULSE_35_QUALIFICATION_PAYLOAD,
        ),
        (
            "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/release-seal.json",
            PULSE_35_SEAL,
            PULSE_35_SEAL_PAYLOAD,
        ),
    ] {
        let bytes = git_blob(path);
        assert_eq!(sha256(&bytes), raw, "{path}");
        let envelope: Value = serde_json::from_slice(&bytes).expect("release envelope");
        assert_eq!(envelope["payload_sha256"], payload);
        assert_eq!(canonical_payload_sha256(&envelope["payload"]), payload);
    }
    let qualification: Value = serde_json::from_slice(&git_blob(
        "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/qualification-receipt.json",
    ))
    .expect("qualification receipt");
    assert_eq!(qualification["payload"]["cycles_required"], 20);
    assert_eq!(qualification["payload"]["cycles_passed"], 20);
    assert_eq!(qualification["payload"]["case_count_per_cycle"], 70);
    assert_eq!(
        qualification["payload"]["coverage_domains_closed_per_cycle"],
        "18/18"
    );
    assert_eq!(
        qualification["payload"]["coverage_interactions_closed_per_cycle"],
        "8/8"
    );

    let schema_path = release["machine_schema"]["path"]
        .as_str()
        .expect("schema path");
    let schema_bytes = fs::read(repo_root().join(schema_path)).expect("read machine schema");
    assert_eq!(sha256(&schema_bytes), PULSE_35_SCHEMA);
    let machine_schema: Value = serde_json::from_slice(&schema_bytes).expect("machine schema");
    assert_eq!(
        machine_schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        machine_schema["$defs"]["caseManifest"]["properties"]["case_count"]["const"],
        70
    );
}

#[test]
fn pulse_36_mutations_reject_weakening_and_indexes_are_consistent() {
    let schema = read_json(
        held_out_root()
            .join("schemas/ferris.process-exit-diagnostic-pulse-36-authority.v1.schema.json"),
    );
    let declaration =
        read_json(held_out_root().join("fixtures/process-exit-diagnostic-pulse-36-authority.json"));
    let mutation_document = read_json(
        held_out_root().join("fixtures/process-exit-diagnostic-pulse-36-authority-mutations.json"),
    );
    let mutations = mutation_document["mutations"]
        .as_array()
        .expect("mutations");
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
                "schema accepted mutation {}",
                mutation["id"]
            );
            mutation["pointer"].as_str().expect("pointer")
        })
        .collect::<Vec<_>>();
    for required in [
        "/immutable_ferris/cutoff",
        "/pulse_35_public_corpus_materializer_release/manifest/raw_sha256",
        "/inherited_public_gates/preflight/exact_pair_count",
        "/pulse_35_public_corpus_materializer_release/materialization_requirements/descriptor_count_required",
        "/pulse_35_public_corpus_materializer_release/materialization_requirements/exact_tuple_counts/0",
        "/pulse_35_public_corpus_materializer_release/private_seed/exact_byte_count",
        "/pulse_35_public_corpus_materializer_release/private_seed/seed_bytes_disclosed",
        "/execution_state/candidate_processes",
    ] {
        assert!(
            pointers.contains(&required),
            "missing required control {required}"
        );
    }

    let absence = git_output(&[
        "cat-file",
        "-e",
        &format!(
            "{CUTOFF}:docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-36-authority.json"
        ),
    ]);
    assert!(
        !absence.status.success(),
        "authority must be absent from cutoff"
    );
    assert!(
        git_output(&[
            "cat-file",
            "-e",
            &format!(
                "{CUTOFF}:docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/public-manifest.json"
            ),
        ])
        .status
        .success(),
        "Pulse 35 release must be at cutoff"
    );

    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(
        schema_index.contains("ferris.process-exit-diagnostic-pulse-36-authority.v1.schema.json")
    );
    assert!(schema_index.contains("All 22 schemas"));
    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    assert!(fixture_index.contains("process-exit-diagnostic-pulse-36-authority.json"));
    assert!(fixture_index.contains("1998 rejection controls"));
    assert!(fixture_index.contains("4201 total declared mutations"));
    for relative in [
        "CONTEXT.md",
        "README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-36.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_36_AUTHORITY.md",
        "docs/plans/reviews/PULSE-36-MATERIALIZED-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 36 document");
        assert!(text.contains(CUTOFF), "{relative} cutoff");
        assert!(text.contains(DECLARATION_IDENTITY), "{relative} identity");
        assert!(text.contains("Pulse 35"), "{relative} release binding");
    }
}
