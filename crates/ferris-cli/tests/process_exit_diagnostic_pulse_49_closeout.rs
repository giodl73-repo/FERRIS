use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AUTHORITY_COMMIT: &str = "80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5";
const AUTHORITY_DECLARATION_IDENTITY: &str =
    "sha256:01101bb7d2a63b657940f82f80eb3edcd3ab7bba05cb8cd54e4dd0c87ce8a3ee";
const AUTHORITY_DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-49-authority.json";
const AUTHORITY_SCHEMA: &str = "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-49-authority.v1.schema.json";
const AUTHORITY_MUTATIONS: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-49-authority-mutations.json";
const PULSE_35_MATERIALIZER: &str = "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/corpus_materializer.py";
const PULSE_35_RELEASE_ROOT: &str =
    "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read public artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_git_clean(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read Git-clean artifact");
    let mut clean = Vec::with_capacity(bytes.len());
    let mut source = bytes.into_iter();
    while let Some(byte) = source.next() {
        if byte == b'\r' {
            assert_eq!(
                source.next(),
                Some(b'\n'),
                "{path:?} contains a non-checkout CR byte"
            );
            clean.push(b'\n');
        } else {
            clean.push(byte);
        }
    }
    assert!(
        !clean.contains(&b'\r'),
        "{path:?} clean-filter output must contain no CR byte"
    );
    clean
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&read_lf(path)).expect("parse JSON")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn git_blob(spec: &str) -> Vec<u8> {
    let output = Command::new("git")
        .current_dir(repo_root())
        .args(["show", spec])
        .output()
        .expect("read Git blob");
    assert!(output.status.success(), "missing Git blob {spec}");
    output.stdout
}

fn git_text(args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run Git command");
    assert!(output.status.success(), "Git command failed: {args:?}");
    String::from_utf8(output.stdout)
        .expect("Git output is UTF-8")
        .trim()
        .to_owned()
}

fn historical_blob(relative_path: &str) -> Vec<u8> {
    git_blob(&format!("{AUTHORITY_COMMIT}:{relative_path}"))
}

fn assert_zero_execution_state(state: &Value) {
    for (field, value) in state.as_object().expect("execution-state object") {
        match value {
            Value::Number(number) => assert_eq!(
                number.as_i64(),
                Some(0),
                "execution state {field} must remain zero"
            ),
            Value::Bool(value) => assert!(!value, "execution state {field} must remain false"),
            Value::Null => {}
            value => panic!("execution state {field} has non-prelaunch value {value}"),
        }
    }
}

struct Sandbox {
    path: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let path = repo_root().join("target").join(format!(
            "pulse-49-prelaunch-closeout-{}",
            std::process::id()
        ));
        fs::remove_dir_all(&path).ok();
        fs::create_dir_all(&path).expect("create closeout sandbox");
        Self { path }
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).ok();
    }
}

#[test]
fn pulse_49_is_permanently_invalid_before_launch_for_pulse_35_case_process_conflict() {
    let resolved_authority = git_text(&["rev-parse", &format!("{AUTHORITY_COMMIT}^{{commit}}")]);
    assert_eq!(resolved_authority, AUTHORITY_COMMIT);

    let root = repo_root();
    for relative_path in [AUTHORITY_DECLARATION, AUTHORITY_SCHEMA, AUTHORITY_MUTATIONS] {
        assert_eq!(
            read_lf(root.join(relative_path)),
            historical_blob(relative_path),
            "{relative_path} must remain the exact prelaunch authority artifact"
        );
    }

    let declaration = read_json(root.join(AUTHORITY_DECLARATION));
    assert_eq!(
        declaration["declaration_identity"],
        AUTHORITY_DECLARATION_IDENTITY
    );
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(
        declaration["immutable_ferris"]["authority_present_at_cutoff"],
        false
    );
    assert_eq!(
        declaration["pulse_35_public_corpus_materializer_release"]["materializer"]["path"],
        "corpus_materializer.py"
    );
    assert_eq!(
        declaration["pulse_35_public_corpus_materializer_release"]["materialization_requirements"]
            ["descriptor_count_required"],
        70
    );

    let materializer = root.join(PULSE_35_MATERIALIZER);
    let materializer_bytes = read_git_clean(&materializer);
    assert_eq!(
        sha256(&materializer_bytes),
        declaration["pulse_35_public_corpus_materializer_release"]["materializer"]["raw_sha256"]
    );
    let cutoff = declaration["immutable_ferris"]["cutoff"]
        .as_str()
        .expect("immutable cutoff");
    assert_eq!(
        materializer_bytes,
        git_blob(&format!("{cutoff}:{PULSE_35_MATERIALIZER}")),
        "materializer must be the exact authority-bound public cutoff blob"
    );

    let sandbox = Sandbox::new();
    let seed = Sha256::digest(b"ferris-p49-prelaunch-closeout-public-test-seed-v1").to_vec();
    let seed_path = sandbox.path.join("public-synthetic-seed.bin");
    let output_path = sandbox.path.join("pulse-35-descriptors");
    fs::write(&seed_path, seed).expect("write fixed synthetic public test seed");
    let materialize = Command::new("python")
        .current_dir(root.join(PULSE_35_RELEASE_ROOT))
        .arg(&materializer)
        .arg("--seed-file")
        .arg(&seed_path)
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("execute exact public Pulse 35 materializer");
    assert!(
        materialize.status.success(),
        "Pulse 35 materializer failed: {}",
        String::from_utf8_lossy(&materialize.stderr)
    );

    let manifest = read_json(output_path.join("case-manifest.json"));
    let cases = manifest["cases"].as_array().expect("materialized cases");
    assert_eq!(manifest["case_count"], 70);
    assert_eq!(cases.len(), 70);
    let ordinals = cases
        .iter()
        .map(|case| case["ordinal"].as_u64().expect("case ordinal"))
        .collect::<BTreeSet<_>>();
    assert_eq!(ordinals, (1..=70).collect());

    let launch_ready = cases
        .iter()
        .filter(|case| case["execution"]["mode"] == "launch-ready")
        .collect::<Vec<_>>();
    let no_launch_cases = cases
        .iter()
        .filter(|case| case["execution"]["mode"] == "no-launch")
        .collect::<Vec<_>>();
    assert_eq!(launch_ready.len(), 69);
    assert_eq!(no_launch_cases.len(), 1);
    let no_launch = no_launch_cases[0];
    assert_eq!(no_launch["ordinal"], 70);
    assert_eq!(no_launch["execution"]["format"], "no-launch");
    assert_eq!(no_launch["before"]["state"], "not-materialized");
    assert_eq!(no_launch["after"]["state"], "not-materialized");
    assert_eq!(no_launch["before"]["target"], Value::Null);
    assert_eq!(no_launch["after"]["target"], Value::Null);
    assert_eq!(
        no_launch["external_prerequisite"],
        "external-immutable-binary-freeze"
    );

    let search = &declaration["pulse_49_bounded_process_exit_search"];
    assert_eq!(search["cases_per_platform"], 70);
    assert_eq!(search["processes_per_platform"], 70);
    assert_eq!(search["total_processes"], 140);
    assert_eq!(search["one_launch"], 1);
    assert_eq!(search["retries"], 0);
    assert_eq!(search["fallbacks"], 0);
    assert_ne!(
        search["processes_per_platform"].as_u64(),
        Some(launch_ready.len() as u64),
        "the authority's 70 processes per platform cannot honor its one no-launch descriptor"
    );
    assert_eq!(
        search["processes_per_platform"]
            .as_u64()
            .expect("declared processes")
            - launch_ready.len() as u64,
        no_launch_cases.len() as u64
    );
    assert_eq!(
        cases.len() as u64 * 2,
        140,
        "two-platform case dispositions"
    );
    assert_eq!(
        launch_ready.len() as u64 * 2,
        138,
        "two-platform launch-ready processes"
    );
    assert_eq!(
        no_launch_cases.len() as u64 * 2,
        2,
        "two-platform no-launch dispositions"
    );
    assert_ne!(
        search["total_processes"].as_u64(),
        Some(launch_ready.len() as u64 * 2),
        "the authority's total processes conflict with the exact descriptor split"
    );

    let execution = &declaration["execution_state"];
    assert_zero_execution_state(execution);
    for field in [
        "candidate_processes",
        "pulse_49_search_processes",
        "pulse_47_witness_invocations",
        "pulse_43_invocations_via_pulse_47",
        "pulse_43_direct_terminal_publisher_invocations",
        "publication_attempts",
    ] {
        assert_eq!(execution[field], 0, "{field} remains unconsumed");
    }
    for field in [
        "result_receipt",
        "terminal_disposition",
        "external_public_summary",
        "category_conclusion",
        "reproducer",
    ] {
        assert_eq!(execution[field], Value::Null, "{field} remains null");
    }
    for field in [
        "p43_result_root_path_sets_observed_in_actual_main_workspace",
        "pulse_47_witness_root_path_sets_observed_in_actual_main_workspace",
        "pulse_47_witness_transaction_published",
    ] {
        assert_eq!(execution[field], false, "{field} remains absent");
    }
    assert_eq!(
        declaration["authority"]["pulse_49_private_runtime_roots_committed"],
        false
    );
    assert_eq!(declaration["authority"]["pulse_49_inference"], false);

    let tracked_paths = git_text(&["ls-tree", "-r", "--name-only", AUTHORITY_COMMIT]);
    let pulse_49_result_or_witness_paths = tracked_paths
        .lines()
        .filter(|path| {
            let path = path.to_ascii_lowercase();
            (path.contains("pulse-49") || path.contains("pulse_49"))
                && (path.contains("result") || path.contains("witness"))
        })
        .collect::<Vec<_>>();
    assert!(
        pulse_49_result_or_witness_paths.is_empty(),
        "prelaunch authority must not retain result/witness artifacts: {pulse_49_result_or_witness_paths:?}"
    );
}
