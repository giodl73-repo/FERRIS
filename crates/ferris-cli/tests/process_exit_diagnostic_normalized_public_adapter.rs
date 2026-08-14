use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const DOMAIN: &str = "ferris.process-exit-diagnostic-normalized-public-adapter/v1";
const PROGRAM_ID: &str = "FERRIS-P30-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-NORMALIZED-PUBLIC-ADAPTER";
const CUTOFF: &str = "cf6b3309c31e5da37d4a8e6655a781f4e92ef603";
const DECLARATION_IDENTITY: &str =
    "sha256:3dd7ccb7071973dd93361cf29a1049bfdab57a483fd9eb86a2ee367b23952f9a";
const PULSE_28_CONTRACT: &str =
    "sha256:4253641e7039046715266eaccca13b10a6d85c44c565736f2c13297077f4ee60";
const PULSE_28_RESULT: &str =
    "sha256:955bb0e2f0ca614a988fbd72ae8abca43b411e46bf2416885d4238ab447309a2";
const NORMALIZATION_RECEIPT: &str =
    "sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225";
const NORMALIZATION_PAYLOAD: &str =
    "sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40";
const PULSE_30_RESULT: &str =
    "sha256:f75d33f054002cdd1b066678163ef926f62ec95ba826fef7273bc614c348f090";
const PULSE_30_RECEIPT: &str =
    "sha256:8f08b0cf27f1b1bb97bcea0591b92c2143cf324736e2112744122838ca58dc30";

const PULSE_25_MANIFEST: &str =
    "sha256:621ed59a5b2124204180be109f69010ac18337a09816c8d28e67713f63efb419";
const PULSE_25_SOURCE: &str =
    "sha256:71b41689202e0ee3c956c9e5408284deac63e53004530b717a403266237d73a7";
const PULSE_25_TEST: &str =
    "sha256:5de010365b3c1297144de030c1738e998e9f55994dee1497d0600b178b2d3de9";
const PULSE_25_BUNDLE: &str =
    "sha256:e296329ff56fad14eba2274d928f45c0fdf6a281db3d2d554c1cee3814d4b406";
const PULSE_25_QUALIFICATION: &str =
    "sha256:04491bea4828fd7329d622c84f9b186d7315dbb31d491176598ffee09be4499e";
const PULSE_25_RECEIPT: &str =
    "sha256:4ec9d50c4ff0f4ba8b65d57751fad28f2a1fcd610e67e664f1727baeb78aaf69";
const PULSE_25_SEAL: &str =
    "sha256:f1d10da9395f2b9f3834da260b6f11e365153ed5b33a75b937d7c410d9c08e1e";

const PULSE_27_MANIFEST: &str =
    "sha256:7a6e61dacb3d58ab6d8c75cf1267a70f7919219baadd34329b835640931e8d5e";
const PULSE_27_ADAPTER_SOURCE: &str =
    "sha256:cdca8d4a0206c9553c637b9228511cfa07e401b9082d96c439d112e2b25c6071";
const PULSE_27_ADAPTER_TEST: &str =
    "sha256:426bd87a7695bb2d5cefdb4c98fc4bef1524616100365656c2e3bc2c19747dff";
const PULSE_27_COLLECTOR: &str =
    "sha256:7a4645f3d3f5e7dcee709351d802e76d1ae6333a7a3b92412fe41d8ae656fc5b";
const PULSE_27_RELEASE: &str =
    "sha256:531113c7c8a50f1c71c446bc708e44549702623114625ea46f5aa874b6aea721";
const PULSE_27_ROOT_CAUSE: &str =
    "sha256:5f1760b7f7cf318029ea24407ef20a087340af16eb2991d7d0b7b0495efded1c";
const PULSE_27_QUALIFICATION: &str =
    "sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886";
const PULSE_27_SEAL: &str =
    "sha256:8abcc449d4b4aff30ed3ade168fa59c7f159e68d3172180703971bb79f096a6e";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn pulse_25_root() -> PathBuf {
    held_out_root().join("pulse-25-collector-source-release")
}

fn pulse_27_root() -> PathBuf {
    held_out_root().join("pulse-27-preflight-adapter-release")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF file");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR bytes");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end with LF");
    bytes
}

fn read_lf_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse LF JSON");
    (bytes, value)
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

fn exact_keys(value: &Value, expected: &[&str]) -> bool {
    value.as_object().is_some_and(|object| {
        object.keys().map(String::as_str).collect::<BTreeSet<_>>()
            == expected.iter().copied().collect::<BTreeSet<_>>()
    })
}

fn assert_closed_object_schemas(value: &Value) {
    match value {
        Value::Object(object) => {
            if object.get("type") == Some(&Value::String("object".to_owned())) {
                assert_eq!(
                    object.get("additionalProperties"),
                    Some(&Value::Bool(false)),
                    "typed object schema must be closed"
                );
            }
            object.values().for_each(assert_closed_object_schemas);
        }
        Value::Array(items) => items.iter().for_each(assert_closed_object_schemas),
        _ => {}
    }
}

fn validate(value: &Value, canonical: &Value) -> bool {
    const ROOT_KEYS: [&str; 25] = [
        "schema",
        "declaration_identity",
        "program_id",
        "recorded_on",
        "status",
        "closed_predecessors",
        "authority",
        "disclosure",
        "immutable_ferris",
        "pulse_25_collector_binding",
        "public_adapter_release",
        "preflight",
        "freshness",
        "platforms",
        "search_bounds",
        "seed_control",
        "coverage",
        "oracle",
        "collection",
        "minimization",
        "publication",
        "result",
        "custody_handoff",
        "limitations",
        "checkout_normalization",
    ];
    if !exact_keys(value, &ROOT_KEYS)
        || value["schema"] != DOMAIN
        || value["program_id"] != PROGRAM_ID
        || value["recorded_on"] != "2026-08-14"
        || value["status"] != "authorized-unexecuted"
        || declaration_identity(value) != value["declaration_identity"]
    {
        return false;
    }

    let mut actual = value.clone();
    let mut expected = canonical.clone();
    actual["declaration_identity"] = Value::String(String::new());
    expected["declaration_identity"] = Value::String(String::new());
    actual == expected
}

fn apply_mutation(value: &mut Value, mutation: &Value) {
    let pointer = mutation["pointer"].as_str().expect("mutation pointer");
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            let (parent, key) = pointer.rsplit_once('/').expect("mutation pointer parent");
            let parent = if parent.is_empty() {
                value
            } else {
                value.pointer_mut(parent).expect("mutation parent")
            };
            if let Some(object) = parent.as_object_mut() {
                object.insert(key.to_owned(), mutation["value"].clone());
            } else {
                parent.as_array_mut().expect("mutation array")
                    [key.parse::<usize>().expect("array index")] = mutation["value"].clone();
            }
        }
        "remove" => {
            let (parent, key) = pointer.rsplit_once('/').expect("remove pointer parent");
            let parent = value.pointer_mut(parent).expect("remove parent");
            if let Some(object) = parent.as_object_mut() {
                object.remove(key);
            } else {
                parent
                    .as_array_mut()
                    .expect("remove array")
                    .remove(key.parse::<usize>().expect("array index"));
            }
        }
        operation => panic!("unsupported mutation {operation}"),
    }
}

fn aggregate(files: &BTreeMap<String, (String, Vec<u8>)>, kinds: Option<&[&str]>) -> String {
    let mut hasher = Sha256::new();
    for (path, (kind, bytes)) in files {
        if kinds.is_some_and(|accepted| !accepted.contains(&kind.as_str())) {
            continue;
        }
        let path_bytes = path.as_bytes();
        hasher.update((path_bytes.len() as u64).to_be_bytes());
        hasher.update(path_bytes);
        hasher.update(Sha256::digest(bytes));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn manifest_files(
    root: &Path,
    manifest: &Value,
    nested_root: Option<&str>,
) -> BTreeMap<String, (String, Vec<u8>)> {
    let mut files = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for file in manifest["files"].as_array().expect("manifest files") {
        let path = file["path"].as_str().expect("manifest path");
        let relative = Path::new(path);
        assert!(!relative.is_absolute(), "manifest path must be relative");
        assert!(
            !relative.components().any(|component| matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )),
            "manifest path must remain inside the release"
        );
        assert!(paths.insert(path.to_owned()), "duplicate manifest path");
        let kind = file["kind"].as_str().expect("manifest kind");
        let disk_path = nested_root.map_or_else(
            || root.join(relative),
            |nested| root.join(nested).join(relative),
        );
        let bytes = read_lf(disk_path);
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("manifest size")
        );
        assert_eq!(sha256(&bytes), file["sha256"]);
        files.insert(path.to_owned(), (kind.to_owned(), bytes));
    }
    files
}

fn recursive_files(root: &Path) -> Vec<PathBuf> {
    fn visit(root: &Path, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(root).expect("read release directory") {
            let path = entry.expect("release entry").path();
            if path.is_dir() {
                visit(&path, files);
            } else {
                files.push(path);
            }
        }
    }

    let mut files = Vec::new();
    visit(root, &mut files);
    files.sort();
    files
}

fn git_output(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run git")
}

#[test]
fn pulse_30_authority_is_closed_unexecuted_strongly_mutated_and_inherited() {
    let schema_path = held_out_root()
        .join("schemas")
        .join("ferris.process-exit-diagnostic-normalized-public-adapter.v1.schema.json");
    let (_, schema) = read_lf_json(schema_path);
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_closed_object_schemas(&schema);

    let fixture_path = held_out_root()
        .join("fixtures")
        .join("process-exit-diagnostic-normalized-public-adapter.json");
    let (_, declaration) = read_lf_json(fixture_path);
    assert!(validate(&declaration, &declaration));
    assert_eq!(declaration["declaration_identity"], DECLARATION_IDENTITY);
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["authority_present_at_cutoff"],
        false
    );

    for pulse in ["pulse_22", "pulse_24", "pulse_26", "pulse_28"] {
        let predecessor = &declaration["closed_predecessors"][pulse];
        assert_eq!(predecessor["disposition"], "invalid");
        assert_eq!(predecessor["candidate_retries"], 0);
        assert_eq!(predecessor["category_conclusion"], Value::Null);
        assert_eq!(predecessor["permanently_closed"], true);
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
    assert_eq!(
        declaration["closed_predecessors"]["pulse_28"]["public_contract_digest"],
        PULSE_28_CONTRACT
    );
    assert_eq!(
        declaration["closed_predecessors"]["pulse_28"]["public_result_digest"],
        PULSE_28_RESULT
    );

    let pulse_26 = read_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-public-bundle.json"),
    );
    for inherited in [
        "coverage",
        "oracle",
        "search_bounds",
        "collection",
        "minimization",
        "publication",
    ] {
        assert_eq!(
            declaration[inherited], pulse_26[inherited],
            "Pulse 30 must inherit Pulse 26 {inherited} exactly"
        );
    }
    assert_eq!(
        declaration["coverage"]["interaction_requirements"]
            .as_array()
            .expect("interactions")
            .len(),
        8
    );
    assert_eq!(
        declaration["oracle"]["compared_fields"]
            .as_array()
            .expect("oracle fields")
            .len(),
        8
    );
    assert_eq!(
        declaration["oracle"]["target_predicates"]
            .as_array()
            .expect("target predicates")
            .len(),
        6
    );
    assert_eq!(declaration["search_bounds"]["logical_case_max"], 512);
    assert_eq!(declaration["search_bounds"]["search_process_max"], 1024);
    assert_eq!(declaration["search_bounds"]["candidate_retries"], 0);
    assert_eq!(declaration["minimization"]["transformation_max"], 128);
    assert_eq!(declaration["minimization"]["process_max"], 256);
    assert_eq!(
        declaration["publication"]["no_reproduction_statement"],
        "bounded no-reproduction; no fix authority"
    );

    let contract = fs::read_to_string(
        held_out_root().join("PROCESS_EXIT_DIAGNOSTIC_NORMALIZED_PUBLIC_ADAPTER.md"),
    )
    .expect("Pulse 30 contract");
    for required in [
        CUTOFF,
        NORMALIZATION_RECEIPT,
        PULSE_25_MANIFEST,
        PULSE_25_BUNDLE,
        PULSE_27_MANIFEST,
        PULSE_27_RELEASE,
        "`core.autocrlf=true`",
        "`text: set` and `eol: lf`",
        "36 LF-framed files",
        "76 passes",
        "exactly one adapter invocation",
        "exactly two fresh",
        "cardinality `2/2/2`",
        "512",
        "1,024",
        "128",
        "256",
        "bounded no-reproduction; no fix authority",
    ] {
        assert!(
            contract.contains(required),
            "missing contract term {required}"
        );
    }

    let (_, mutations) = read_lf_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-normalized-public-adapter-mutations.json"),
    );
    assert_eq!(
        mutations["schema"],
        "ferris.process-exit-diagnostic-normalized-public-adapter-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 322);
    let mut ids = BTreeSet::new();
    for mutation in mutations {
        assert!(exact_keys(
            mutation,
            &["id", "operation", "pointer", "value", "recompute_identity"]
        ));
        assert!(ids.insert(mutation["id"].as_str().expect("mutation id")));
        let mut candidate = declaration.clone();
        apply_mutation(&mut candidate, mutation);
        if mutation["recompute_identity"] == true {
            candidate["declaration_identity"] = Value::String(declaration_identity(&candidate));
        }
        assert!(
            !validate(&candidate, &declaration),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
    }
}

#[test]
fn pulse_30_cutoff_receipt_attributes_and_all_76_bindings_are_exact() {
    let attributes = git_output(&["show", &format!("{CUTOFF}:.gitattributes")]);
    assert!(
        attributes.status.success(),
        "cutoff must contain .gitattributes"
    );
    let attributes = String::from_utf8(attributes.stdout).expect("UTF-8 attributes");
    for rule in [
        "/docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/** text eol=lf",
        "/docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/** text eol=lf",
    ] {
        assert!(
            attributes.lines().any(|line| line == rule),
            "missing {rule}"
        );
    }

    let receipt_repo_path = "docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json";
    let cutoff_receipt = git_output(&["show", &format!("{CUTOFF}:{receipt_repo_path}")]);
    assert!(
        cutoff_receipt.status.success(),
        "cutoff must contain receipt"
    );
    assert_eq!(sha256(&cutoff_receipt.stdout), NORMALIZATION_RECEIPT);

    for path in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-30.md",
        "docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_NORMALIZED_PUBLIC_ADAPTER.md",
        "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-normalized-public-adapter.v1.schema.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-normalized-public-adapter.json",
        "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-normalized-public-adapter-mutations.json",
        "docs/plans/reviews/PULSE-30-NORMALIZED-PUBLIC-ADAPTER-AUTHORITY-ROLE-REVIEW.md",
        "crates/ferris-cli/tests/process_exit_diagnostic_normalized_public_adapter.rs",
    ] {
        let absent = git_output(&["cat-file", "-e", &format!("{CUTOFF}:{path}")]);
        assert!(
            !absent.status.success(),
            "Pulse 30 authority must be later than cutoff: {path}"
        );
    }

    let receipt_path = repo_root().join(receipt_repo_path);
    let (receipt_bytes, receipt) = read_lf_json(receipt_path);
    assert_eq!(sha256(&receipt_bytes), NORMALIZATION_RECEIPT);
    assert_eq!(receipt["receipt_id"], NORMALIZATION_PAYLOAD);
    assert_eq!(
        receipt["payload_sha256"],
        canonical_payload_sha256(&receipt["payload"])
    );
    assert_eq!(receipt["payload"]["materialization"]["core_autocrlf"], true);
    assert_eq!(receipt["payload"]["line_endings"]["files_checked"], 36);
    assert_eq!(receipt["payload"]["line_endings"]["lf_passed"], 36);
    assert_eq!(receipt["payload"]["binding_checks"]["passed"], 76);
    assert_eq!(receipt["payload"]["binding_checks"]["failed"], 0);

    let pulse_25_files = recursive_files(&pulse_25_root());
    let pulse_27_files = recursive_files(&pulse_27_root());
    assert_eq!(pulse_25_files.len(), 14);
    assert_eq!(pulse_27_files.len(), 22);
    let all_files = pulse_25_files
        .iter()
        .chain(&pulse_27_files)
        .collect::<Vec<_>>();
    assert_eq!(all_files.len(), 36);

    let mut attr_args = vec!["check-attr", "text", "eol", "--"];
    let relative_paths = all_files
        .iter()
        .map(|path| {
            path.strip_prefix(repo_root())
                .expect("repository file")
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect::<Vec<_>>();
    attr_args.extend(relative_paths.iter().map(String::as_str));
    let attributes = git_output(&attr_args);
    assert!(attributes.status.success(), "git check-attr must succeed");
    let attributes = String::from_utf8(attributes.stdout).expect("UTF-8 check-attr");
    assert_eq!(attributes.lines().count(), 72);
    for path in &relative_paths {
        assert!(
            attributes
                .lines()
                .any(|line| line == format!("{path}: text: set")),
            "{path} text attribute"
        );
        assert!(
            attributes
                .lines()
                .any(|line| line == format!("{path}: eol: lf")),
            "{path} eol attribute"
        );
    }
    for path in all_files {
        read_lf(path);
    }

    let mut checks = 0_u64;
    let (manifest_bytes, pulse_25_manifest) =
        read_lf_json(pulse_25_root().join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), PULSE_25_MANIFEST);
    checks += 1;
    let pulse_25_files = manifest_files(&pulse_25_root(), &pulse_25_manifest, Some("bundle"));
    checks += 18;
    assert_eq!(
        aggregate(&pulse_25_files, Some(&["source"])),
        PULSE_25_SOURCE
    );
    assert_eq!(aggregate(&pulse_25_files, Some(&["test"])), PULSE_25_TEST);
    assert_eq!(aggregate(&pulse_25_files, None), PULSE_25_BUNDLE);
    checks += 3;
    assert_eq!(
        sha256(&read_lf(pulse_25_root().join("qualification-report.json"))),
        PULSE_25_QUALIFICATION
    );
    assert_eq!(
        sha256(&read_lf(pulse_25_root().join("release-receipt.json"))),
        PULSE_25_RECEIPT
    );
    assert_eq!(
        sha256(&read_lf(pulse_25_root().join("release-seal.json"))),
        PULSE_25_SEAL
    );
    assert_eq!(checks, 22);

    let (manifest_bytes, pulse_27_manifest) =
        read_lf_json(pulse_27_root().join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), PULSE_27_MANIFEST);
    checks += 1;
    let pulse_27_files = manifest_files(&pulse_27_root(), &pulse_27_manifest, None);
    checks += 40;
    assert_eq!(
        aggregate(&pulse_27_files, Some(&["adapter-source"])),
        PULSE_27_ADAPTER_SOURCE
    );
    assert_eq!(
        aggregate(&pulse_27_files, Some(&["adapter-test"])),
        PULSE_27_ADAPTER_TEST
    );
    assert_eq!(
        aggregate(
            &pulse_27_files,
            Some(&["immutable-collector-source", "immutable-collector-test"])
        ),
        PULSE_27_COLLECTOR
    );
    assert_eq!(aggregate(&pulse_27_files, None), PULSE_27_RELEASE);
    checks += 4;
    assert_eq!(
        sha256(&read_lf(pulse_27_root().join("root-cause-report.json"))),
        PULSE_27_ROOT_CAUSE
    );
    assert_eq!(
        sha256(&read_lf(pulse_27_root().join("qualification-receipt.json"))),
        PULSE_27_QUALIFICATION
    );
    assert_eq!(
        sha256(&read_lf(pulse_27_root().join("release-seal.json"))),
        PULSE_27_SEAL
    );
    assert_eq!(checks, 67);

    for file in pulse_27_manifest["files"]
        .as_array()
        .expect("Pulse 27 files")
    {
        let kind = file["kind"].as_str().expect("Pulse 27 kind");
        if kind.starts_with("immutable-collector-") {
            let path = file["path"]
                .as_str()
                .expect("collector path")
                .strip_prefix("collector/")
                .expect("collector prefix");
            assert_eq!(
                read_lf(pulse_27_root().join("collector").join(path)),
                read_lf(pulse_25_root().join("bundle").join(path))
            );
            checks += 1;
        }
    }
    assert_eq!(checks, 76);
}

#[test]
fn pulse_30_requires_normalization_then_exact_copy_preflight_and_fresh_material() {
    let declaration = read_json(
        held_out_root()
            .join("fixtures")
            .join("process-exit-diagnostic-normalized-public-adapter.json"),
    );
    let normalization = &declaration["checkout_normalization"];
    assert_eq!(normalization["receipt_raw_sha256"], NORMALIZATION_RECEIPT);
    assert_eq!(normalization["required_before_package_copy"], true);
    assert_eq!(normalization["materialization"]["source_commit"], CUTOFF);
    assert_eq!(normalization["materialization"]["core_autocrlf"], true);
    assert_eq!(
        normalization["materialization"]["working_tree_copy_allowed"],
        false
    );
    assert_eq!(normalization["git_attributes"]["file_count"], 36);
    assert_eq!(normalization["git_attributes"]["text"], "set");
    assert_eq!(normalization["git_attributes"]["eol"], "lf");
    assert_eq!(normalization["line_endings"]["files_required"], 36);
    assert_eq!(normalization["line_endings"]["lf_required"], 36);
    assert_eq!(normalization["line_endings"]["cr_bytes_allowed"], 0);
    assert_eq!(normalization["binding_checks"]["required"], 76);
    assert_eq!(normalization["binding_checks"]["passed_required"], 76);
    assert_eq!(normalization["binding_checks"]["failed_allowed"], 0);

    let pulse_25_manifest = read_json(pulse_25_root().join("public-manifest.json"));
    let pulse_25_binding =
        &declaration["pulse_25_collector_binding"]["pulse_26_public_collector_bundle"];
    assert_eq!(pulse_25_binding["manifest_digest"], PULSE_25_MANIFEST);
    assert_eq!(pulse_25_binding["files"], pulse_25_manifest["files"]);
    assert_eq!(pulse_25_binding["source_aggregate"], PULSE_25_SOURCE);
    assert_eq!(pulse_25_binding["test_aggregate"], PULSE_25_TEST);
    assert_eq!(pulse_25_binding["bundle_aggregate"], PULSE_25_BUNDLE);

    let pulse_27_manifest = read_json(pulse_27_root().join("public-manifest.json"));
    let package = &declaration["public_adapter_release"];
    assert_eq!(package["manifest_digest"], PULSE_27_MANIFEST);
    assert_eq!(package["files"], pulse_27_manifest["files"]);
    assert_eq!(package["file_count"], 20);
    assert_eq!(
        package["copy_policy"],
        "copy-exactly-twenty-manifest-listed-public-files-only"
    );
    assert_eq!(package["private_workspace_access"], false);
    assert_eq!(package["non_manifest_files_in_copied_workspace"], false);
    assert_eq!(package["normalization_verified_before_copy_required"], true);
    assert_eq!(package["copy_started_before_normalization"], false);
    assert_eq!(
        package["independent_per_file_hash_recomputation_required"],
        true
    );
    assert_eq!(
        package["independent_release_aggregate_recomputation_required"],
        true
    );
    assert_eq!(package["copied"], false);
    assert_eq!(package["verified"], false);
    assert_eq!(package["per_file_hashes_recomputed"], 0);

    let preflight = &declaration["preflight"];
    assert_eq!(preflight["exact_adapter_invocations"], 1);
    assert_eq!(preflight["exact_pair_count"], 2);
    assert_eq!(preflight["exact_windows_rows"], 2);
    assert_eq!(preflight["exact_ubuntu_rows"], 2);
    assert_eq!(preflight["exact_process_rows"], 4);
    assert_eq!(preflight["exact_pair_seals"], 2);
    assert_eq!(preflight["exact_fresh_verifier_processes"], 2);
    assert_eq!(preflight["exact_windows_verifier_processes"], 1);
    assert_eq!(preflight["exact_ubuntu_verifier_processes"], 1);
    assert_eq!(
        preflight["whole_store_cardinality"],
        serde_json::json!({"windows_rows": 2, "ubuntu_rows": 2, "pair_seals": 2})
    );
    assert_eq!(preflight["pair_retries"], 0);
    assert_eq!(preflight["adapter_invocation_retries"], 0);
    assert_eq!(preflight["verifier_retries"], 0);
    assert_eq!(preflight["zero_residue_required"], true);
    assert_eq!(preflight["started"], false);

    assert_eq!(
        declaration["freshness"]["generation_material_allowed_only_after_preflight_pass"],
        true
    );
    let new_material = &declaration["freshness"]["post_preflight_new_material"];
    for field in [
        "custody_identity",
        "seed",
        "classifier",
        "generator",
        "case_manifest",
        "coverage_manifest",
        "corpus",
    ] {
        assert_eq!(new_material[field], true, "fresh {field}");
    }

    let result = &declaration["result"];
    assert_eq!(result["checkout_materialization_started"], false);
    assert_eq!(result["checkout_normalization_verified"], false);
    assert_eq!(result["package_copy_started"], false);
    assert_eq!(result["preflight_started"], false);
    assert_eq!(result["search_started"], false);
    assert_eq!(result["cases_generated"], 0);
    assert_eq!(result["retries"], 0);
    assert_eq!(result["category_conclusion"], Value::Null);
}

#[test]
fn pulse_30_public_result_is_exact_invalid_after_preflight_and_sealed() {
    let result_root = held_out_root().join("pulse-30-public-result");
    let (bytes, result) = read_lf_json(result_root.join("PULSE-30-PUBLIC-RESULT.json"));
    assert_eq!(sha256(&bytes), PULSE_30_RESULT);
    assert!(exact_keys(&result, &["payload", "receipt_id"]));
    assert_eq!(result["receipt_id"], PULSE_30_RECEIPT);
    assert_eq!(
        canonical_payload_sha256(&result["payload"]),
        PULSE_30_RECEIPT
    );

    let payload = &result["payload"];
    assert!(exact_keys(
        payload,
        &[
            "authority_commit",
            "blocker",
            "category_conclusion",
            "cutoff",
            "cutoff_freeze",
            "disposition",
            "execution",
            "freshness",
            "generation",
            "normalization",
            "package",
            "preflight",
            "program_id",
            "publication",
            "schema",
        ]
    ));
    assert_eq!(payload["schema"], "ferris.pulse-30-public-result/v1");
    assert_eq!(payload["program_id"], PROGRAM_ID);
    assert_eq!(payload["cutoff"], CUTOFF);
    assert_eq!(payload["disposition"], "invalid");
    assert_eq!(payload["category_conclusion"], Value::Null);

    assert_eq!(payload["normalization"]["attribute_files_verified"], 36);
    assert_eq!(payload["normalization"]["lf_files_verified"], 36);
    assert_eq!(payload["normalization"]["binding_checks_passed"], 76);
    assert_eq!(payload["normalization"]["binding_checks_failed"], 0);
    assert_eq!(
        payload["normalization"]["receipt_raw_sha256"],
        NORMALIZATION_RECEIPT
    );

    assert_eq!(payload["package"]["manifest_listed_files_copied"], 20);
    assert_eq!(payload["package"]["per_file_hashes_recomputed"], 20);
    assert_eq!(payload["package"]["aggregate_bindings_recomputed"], 4);
    assert_eq!(
        payload["package"]["report_receipt_seal_bindings_verified"],
        6
    );
    assert_eq!(payload["package"]["extra_files"], 0);

    assert_eq!(payload["cutoff_freeze"]["windows_binary_frozen"], true);
    assert_eq!(payload["cutoff_freeze"]["ubuntu_binary_frozen"], true);
    assert_eq!(payload["cutoff_freeze"]["environments_frozen"], 2);
    assert_eq!(payload["cutoff_freeze"]["direct_launch_required"], true);

    let preflight = &payload["preflight"];
    assert_eq!(preflight["outcome"], "pass");
    assert_eq!(preflight["adapter_invocations"], 1);
    assert_eq!(preflight["pair_ids"], 2);
    assert_eq!(preflight["windows_rows"], 2);
    assert_eq!(preflight["ubuntu_rows"], 2);
    assert_eq!(preflight["process_rows"], 4);
    assert_eq!(preflight["pair_seals"], 2);
    assert_eq!(preflight["fresh_verifier_processes"], 2);
    assert_eq!(preflight["whole_store_cardinality"], "2/2/2");
    assert_eq!(preflight["retries"], 0);
    assert_eq!(preflight["residue_count"], 0);

    assert_eq!(
        payload["blocker"]["code"],
        "public-input-schema-unavailable-under-authorized-read-scope"
    );
    assert_eq!(
        payload["blocker"]["stage"],
        "generation-before-case-materialization"
    );
    assert_eq!(payload["blocker"]["public_safe"], true);
    assert_eq!(payload["generation"]["cases_generated"], 0);
    assert_eq!(payload["generation"]["fresh_corpus_created"], false);
    assert_eq!(payload["execution"]["candidate_pairs_completed"], 0);
    assert_eq!(payload["execution"]["candidate_processes"], 0);
    assert_eq!(payload["execution"]["windows_candidate_processes"], 0);
    assert_eq!(payload["execution"]["ubuntu_candidate_processes"], 0);
    assert_eq!(payload["execution"]["candidate_retries"], 0);
    assert_eq!(payload["execution"]["search_executions"], 0);
    assert_eq!(payload["execution"]["minimization_processes"], 0);
    assert_eq!(payload["publication"]["reproducer_created"], false);
    assert_eq!(payload["publication"]["fix_authority"], false);
    assert_eq!(payload["publication"]["further_launches_prohibited"], true);

    let readme = fs::read_to_string(result_root.join("README.md")).expect("Pulse 30 README");
    for required in [
        PULSE_30_RESULT,
        PULSE_30_RECEIPT,
        "generation-before-case-materialization",
        "36/36",
        "76/76",
        "20 files",
        "20 hashes",
        "four aggregates",
        "six report/receipt/seal bindings",
        "zero candidates",
        "null",
    ] {
        assert!(readme.contains(required), "missing result term {required}");
    }
}
