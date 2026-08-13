use ferris_core::{
    CommandEnvelope, ResultClass, create_profile_diff, profile_diff_error_envelope,
    render_profile_diff_human,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const BASELINE_GIT_CUTOFF: &str = "0c5db524b1c6f1c5505f1362bb46aac9dd2985aa";
const EXPECTED_PROCESS_COUNT: usize = 26;
const MAX_PROFILE_INPUT_BYTES: usize = 1024 * 1024;
const MAX_PROFILE_CHANGES: usize = 10_000;
const MANIFEST_NAME: &str = "process_exit_diagnostic_manifest.json";
const PRIVACY_CANARIES: [&str; 2] = [
    "P19-PUBLIC-RAW-BEFORE-CANARY-4f32",
    "P19-PUBLIC-RAW-AFTER-CANARY-b917",
];

#[derive(Clone, Debug, Deserialize)]
struct DiagnosticManifest {
    schema: String,
    cases: Vec<CaseSpec>,
}

#[derive(Clone, Debug, Deserialize)]
struct CaseSpec {
    id: String,
    input_branch: String,
    branch: String,
    format: CaseFormat,
    expected_result_class: ResultClass,
    expected_exit_code: u8,
    required_route: StreamRoute,
    #[serde(default)]
    expected_diagnostic_code: Option<String>,
    #[serde(default)]
    expected_change_count: Option<usize>,
    #[serde(default)]
    required_change_path: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum CaseFormat {
    Json,
    Human,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum StreamRoute {
    StdoutOnly,
    StderrOnly,
    Both,
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Localization {
    CoreClassification,
    EnvelopeConstruction,
    CliEmissionExitCode,
    FormatParity,
    NoReproduction,
}

#[derive(Clone)]
struct FixturePaths {
    before: PathBuf,
    after: PathBuf,
}

struct DiagnosticDirectory(PathBuf);

impl DiagnosticDirectory {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/pulse-19-process-exit")
            .join(format!(
                "{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create isolated Pulse 19 directory");
        Self(path)
    }
}

impl Drop for DiagnosticDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone)]
struct CoreObservation {
    envelope: Value,
    human_render: Option<Vec<u8>>,
    receipt: CoreReceipt,
    core_problems: Vec<String>,
    envelope_problems: Vec<String>,
}

struct RuntimeRow {
    receipt: ProcessRowReceipt,
    emitted_envelope: Option<Value>,
    stdout: Vec<u8>,
    problems: Vec<String>,
}

#[derive(Serialize)]
struct DiagnosticReceipt {
    schema: &'static str,
    evidence_class: &'static str,
    evidence_status: &'static str,
    run_id: String,
    recorded_at_utc: String,
    environment: EnvironmentReceipt,
    baseline_git_cutoff: &'static str,
    source_cutoff: String,
    executable_digest: String,
    fixture_manifest_schema: String,
    fixture_manifest_digest: String,
    process_cardinality: ProcessCardinalityReceipt,
    aggregate_outcome: Localization,
    branches: Vec<BranchReceipt>,
    rows: Vec<ProcessRowReceipt>,
}

#[derive(Serialize)]
struct EnvironmentReceipt {
    platform: String,
    operating_system: String,
    kernel: String,
    architecture: String,
    rustc: String,
    cargo: String,
}

#[derive(Serialize)]
struct ProcessCardinalityReceipt {
    expected: usize,
    declared: usize,
    attempted: usize,
    started: usize,
    retained: usize,
    retries: usize,
    missing_ids: Vec<String>,
    duplicate_ids: Vec<String>,
    extra_ids: Vec<String>,
}

#[derive(Serialize)]
struct BranchReceipt {
    branch_id: String,
    process_row_ids: Vec<String>,
    localization: Localization,
    mismatches: Vec<String>,
}

#[derive(Serialize)]
struct ProcessRowReceipt {
    id: String,
    input_branch: String,
    branch: String,
    format: CaseFormat,
    expected_result_class: String,
    expected_exit_code: u8,
    required_route: StreamRoute,
    argv: Vec<String>,
    working_directory: &'static str,
    before_input: String,
    after_input: String,
    core: CoreReceipt,
    process_started: bool,
    actual_exit_code: Option<i32>,
    actual_route: StreamRoute,
    stdout_bytes: usize,
    stderr_bytes: usize,
    complete_stdout_digest: String,
    complete_stderr_digest: String,
    parsed_envelope: Option<EnvelopeReceipt>,
    human_parity: Option<HumanParityReceipt>,
    privacy_canaries_absent: bool,
    localization: Localization,
    mismatches: Vec<String>,
}

#[derive(Clone, Serialize)]
struct CoreReceipt {
    outcome: &'static str,
    classification: String,
    diagnostic_class: Option<String>,
    envelope: EnvelopeReceipt,
}

#[derive(Clone, Serialize)]
struct EnvelopeReceipt {
    schema: Option<String>,
    semantic_command_id: Option<String>,
    result_class: Option<String>,
    process_exit_code: Option<u64>,
    diagnostic_count: Option<usize>,
    diagnostic_class: Option<String>,
    diagnostic_code: Option<String>,
    record_present: Option<bool>,
    record_schema: Option<String>,
    record_change_count: Option<usize>,
}

#[derive(Serialize)]
struct HumanParityReceipt {
    paired_json_row: String,
    byte_identical_inputs: bool,
    exact_core_render: bool,
    public_semantics_match: bool,
    privacy_canaries_absent: bool,
}

fn manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(MANIFEST_NAME)
}

fn load_manifest() -> (DiagnosticManifest, Vec<u8>) {
    let bytes = fs::read(manifest_path()).expect("read Pulse 19 public manifest");
    let manifest =
        serde_json::from_slice::<DiagnosticManifest>(&bytes).expect("parse Pulse 19 manifest");
    (manifest, bytes)
}

fn validate_manifest(manifest: &DiagnosticManifest) {
    assert_eq!(
        manifest.schema,
        "ferris.profile-diff-process-exit-diagnostic-manifest/v1"
    );
    let expected_ids = (1..=23)
        .map(|index| format!("J{index:02}"))
        .chain((1..=3).map(|index| format!("H{index:02}")))
        .collect::<Vec<_>>();
    let declared_ids = manifest
        .cases
        .iter()
        .map(|case| case.id.clone())
        .collect::<Vec<_>>();
    assert_eq!(declared_ids, expected_ids);
    assert_eq!(manifest.cases.len(), EXPECTED_PROCESS_COUNT);
    assert_eq!(
        declared_ids.iter().collect::<BTreeSet<_>>().len(),
        EXPECTED_PROCESS_COUNT
    );

    for case in &manifest.cases {
        assert_eq!(
            case.expected_exit_code,
            case.expected_result_class.exit_code(),
            "{}",
            case.id
        );
        let expected_route = if matches!(
            case.expected_result_class,
            ResultClass::Success | ResultClass::Difference
        ) {
            StreamRoute::StdoutOnly
        } else {
            StreamRoute::StderrOnly
        };
        assert_eq!(case.required_route, expected_route, "{}", case.id);
        if case.id.starts_with('J') {
            assert_eq!(case.format, CaseFormat::Json, "{}", case.id);
            assert_eq!(case.input_branch, case.id, "{}", case.id);
        } else {
            assert_eq!(case.format, CaseFormat::Human, "{}", case.id);
        }
    }

    for (human_id, json_id) in [("H01", "J01"), ("H02", "J02"), ("H03", "J23")] {
        let human = manifest
            .cases
            .iter()
            .find(|case| case.id == human_id)
            .expect("declared human row");
        let json = manifest
            .cases
            .iter()
            .find(|case| case.id == json_id)
            .expect("declared JSON row");
        assert_eq!(human.input_branch, json_id);
        assert_eq!(
            human.expected_result_class, json.expected_result_class,
            "{human_id}"
        );
        assert_eq!(human.expected_exit_code, json.expected_exit_code);
        assert_eq!(human.required_route, json.required_route);
    }
}

fn profile_value(profile_id: &str, revision: &str, consumer: &str, identity: Value) -> Value {
    json!({
        "schema": "ferris.profile-evidence/v0",
        "profile_id": profile_id,
        "revision": revision,
        "consumer": consumer,
        "sections": {
            "identity": identity,
            "closure": {},
            "features": {},
            "toolchain": {},
            "targets": {},
            "providers": {},
            "native": {},
            "stages": {},
            "assurance": {},
            "stewardship": {},
            "support": {},
            "lifecycle": {}
        }
    })
}

fn write_json(path: &Path, value: &Value) {
    let mut bytes = serde_json::to_vec(value).expect("serialize synthetic profile");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("write synthetic profile");
}

fn valid_profile(revision: &str) -> Value {
    profile_value(
        "public.profile",
        revision,
        "public.consumer",
        json!({"public": "value"}),
    )
}

fn write_pair(directory: &Path, before: &Value, after: &Value) -> FixturePaths {
    let before_path = directory.join("before.json");
    let after_path = directory.join("after.json");
    write_json(&before_path, before);
    write_json(&after_path, after);
    FixturePaths {
        before: before_path,
        after: after_path,
    }
}

fn generate_fixtures(root: &Path) -> BTreeMap<String, FixturePaths> {
    let mut fixtures = BTreeMap::new();
    for index in 1..=23 {
        let id = format!("J{index:02}");
        let directory = root.join(&id);
        fs::create_dir_all(&directory).expect("create branch directory");
        let before = valid_profile("public-r1");
        let after = valid_profile("public-r1");
        let paths = match id.as_str() {
            "J01" => write_pair(&directory, &before, &after),
            "J02" => write_pair(&directory, &before, &valid_profile("public-r2")),
            "J03" => {
                let paths = write_pair(&directory, &before, &after);
                let compact = serde_json::to_string(&before).expect("serialize duplicate source");
                let duplicate = compact.replacen(
                    r#""profile_id":"public.profile""#,
                    r#""profile_id":"public.profile","profile_id":"public.other""#,
                    1,
                );
                assert_ne!(duplicate, compact);
                fs::write(&paths.before, duplicate).expect("write duplicate top-level member");
                paths
            }
            "J04" => {
                let paths = write_pair(&directory, &before, &after);
                let compact = serde_json::to_string(&before).expect("serialize duplicate source");
                let duplicate = compact.replacen(
                    r#""public":"value""#,
                    r#""public":"value","public":"other""#,
                    1,
                );
                assert_ne!(duplicate, compact);
                fs::write(&paths.before, duplicate).expect("write duplicate nested member");
                paths
            }
            "J05" => {
                let paths = write_pair(&directory, &before, &after);
                fs::write(&paths.before, br#"{"schema":"ferris.profile-evidence/v0""#)
                    .expect("write malformed JSON");
                paths
            }
            "J06" => {
                let mut invalid = before.clone();
                invalid["unexpected"] = json!(true);
                write_pair(&directory, &invalid, &after)
            }
            "J07" => {
                let mut invalid = before.clone();
                invalid["sections"]["unexpected"] = json!(true);
                write_pair(&directory, &invalid, &after)
            }
            "J08" => write_pair(
                &directory,
                &profile_value("public\nprofile", "public-r1", "public.consumer", json!({})),
                &after,
            ),
            "J09" => write_pair(
                &directory,
                &profile_value("public.profile", "public\nr1", "public.consumer", json!({})),
                &after,
            ),
            "J10" => write_pair(
                &directory,
                &profile_value("public.profile", "public-r1", "public\nconsumer", json!({})),
                &after,
            ),
            "J11" => write_pair(
                &directory,
                &profile_value(
                    "public.profile",
                    "public-r1",
                    "public.consumer",
                    json!({"invalid\nkey": "value"}),
                ),
                &after,
            ),
            "J12" => write_pair(
                &directory,
                &profile_value(
                    "public.profile.before",
                    "public-r1",
                    "public.consumer",
                    json!({}),
                ),
                &profile_value(
                    "public.profile.after",
                    "public-r2",
                    "public.consumer",
                    json!({}),
                ),
            ),
            "J13" => write_pair(
                &directory,
                &profile_value(
                    "public.profile",
                    "public-r1",
                    "public.consumer.before",
                    json!({}),
                ),
                &profile_value(
                    "public.profile",
                    "public-r2",
                    "public.consumer.after",
                    json!({}),
                ),
            ),
            "J14" => {
                let mut unsupported = before.clone();
                unsupported["schema"] = json!("ferris.profile-evidence/v99");
                write_pair(&directory, &unsupported, &after)
            }
            "J15" => {
                let after_path = directory.join("after.json");
                write_json(&after_path, &after);
                FixturePaths {
                    before: directory.join("missing-before.json"),
                    after: after_path,
                }
            }
            "J16" => {
                let before_path = directory.join("before.json");
                write_json(&before_path, &before);
                FixturePaths {
                    before: before_path,
                    after: directory.join("missing-after.json"),
                }
            }
            "J17" => {
                let before_path = directory.join("before-directory");
                let after_path = directory.join("after.json");
                fs::create_dir(&before_path).expect("create non-file first input");
                write_json(&after_path, &after);
                FixturePaths {
                    before: before_path,
                    after: after_path,
                }
            }
            "J18" => {
                let before_path = directory.join("before.json");
                let after_path = directory.join("after-directory");
                write_json(&before_path, &before);
                fs::create_dir(&after_path).expect("create non-file second input");
                FixturePaths {
                    before: before_path,
                    after: after_path,
                }
            }
            "J19" => {
                let paths = write_pair(&directory, &before, &after);
                fs::write(&paths.before, vec![b'X'; MAX_PROFILE_INPUT_BYTES + 1])
                    .expect("write oversized first input");
                paths
            }
            "J20" => {
                let paths = write_pair(&directory, &before, &after);
                fs::write(&paths.after, vec![b'X'; MAX_PROFILE_INPUT_BYTES + 1])
                    .expect("write oversized second input");
                paths
            }
            "J21" | "J22" => {
                let count = if id == "J21" {
                    MAX_PROFILE_CHANGES
                } else {
                    MAX_PROFILE_CHANGES + 1
                };
                let mut values = serde_json::Map::new();
                for value in 0..count {
                    values.insert(format!("key-{value:05}"), json!(value));
                }
                write_pair(
                    &directory,
                    &profile_value("public.profile", "public-r1", "public.consumer", json!({})),
                    &profile_value(
                        "public.profile",
                        "public-r1",
                        "public.consumer",
                        Value::Object(values),
                    ),
                )
            }
            "J23" => write_pair(
                &directory,
                &profile_value(
                    "public.profile-23",
                    "public-r1",
                    "public.consumer-23",
                    json!({"public/key~name": PRIVACY_CANARIES[0]}),
                ),
                &profile_value(
                    "public.profile-23",
                    "public-r2",
                    "public.consumer-23",
                    json!({"public/key~name": PRIVACY_CANARIES[1]}),
                ),
            ),
            _ => unreachable!("frozen public branch"),
        };
        fixtures.insert(id, paths);
    }
    fixtures
}

fn observe_core(case: &CaseSpec, paths: &FixturePaths) -> CoreObservation {
    let (outcome, classification, diagnostic_class, envelope, human_render, mut core_problems) =
        match create_profile_diff(&paths.before, &paths.after) {
            Ok(envelope) => {
                let classification = envelope.result_class;
                let human_render = Some(render_profile_diff_human(&envelope).into_bytes());
                let value =
                    serde_json::to_value(&envelope).expect("serialize profile diff envelope");
                (
                    "success",
                    classification,
                    None,
                    value,
                    human_render,
                    Vec::new(),
                )
            }
            Err(error) => {
                let classification = error.result_class();
                let diagnostic_class = Some(error.diagnostic().result_class);
                let envelope: CommandEnvelope<Value> =
                    profile_diff_error_envelope(&paths.before, &paths.after, &error);
                let value =
                    serde_json::to_value(&envelope).expect("serialize profile error envelope");
                let mut problems = Vec::new();
                if diagnostic_class != Some(classification) {
                    problems.push("CoreError diagnostic class differs from CoreError class".into());
                }
                (
                    "error",
                    classification,
                    diagnostic_class,
                    value,
                    None,
                    problems,
                )
            }
        };

    if classification != case.expected_result_class {
        core_problems.push(format!(
            "core classification expected {} but observed {}",
            case.expected_result_class, classification
        ));
    }
    let envelope_receipt = envelope_receipt(&envelope);
    let envelope_problems = validate_envelope(case, &envelope);
    CoreObservation {
        envelope,
        human_render,
        receipt: CoreReceipt {
            outcome,
            classification: classification.to_string(),
            diagnostic_class: diagnostic_class.map(|class| class.to_string()),
            envelope: envelope_receipt,
        },
        core_problems,
        envelope_problems,
    }
}

fn validate_envelope(case: &CaseSpec, envelope: &Value) -> Vec<String> {
    let mut problems = Vec::new();
    if envelope["schema"] != "ferris.command-result/v2" {
        problems.push("envelope schema differs from ferris.command-result/v2".into());
    }
    if envelope["semantic_command_id"] != "profile-diff" {
        problems.push("envelope semantic_command_id differs from profile-diff".into());
    }
    if envelope["result_class"].as_str() != Some(&case.expected_result_class.to_string()) {
        problems.push("envelope result_class differs from manifest".into());
    }
    if envelope["process_exit_code"].as_u64() != Some(u64::from(case.expected_exit_code)) {
        problems.push("envelope process_exit_code differs from manifest".into());
    }

    let diagnostics = envelope["diagnostics"].as_array();
    let record_present = !envelope["record"].is_null();
    if matches!(
        case.expected_result_class,
        ResultClass::Success | ResultClass::Difference
    ) {
        if diagnostics.is_none_or(|values| !values.is_empty()) {
            problems.push("success/difference envelope diagnostics are not empty".into());
        }
        if !record_present {
            problems.push("success/difference envelope record is null".into());
        }
    } else {
        if diagnostics.is_none_or(|values| values.len() != 1) {
            problems.push("non-success envelope does not contain exactly one diagnostic".into());
        }
        if record_present {
            problems.push("non-success envelope record is present".into());
        }
        if diagnostics
            .and_then(|values| values.first())
            .and_then(|diagnostic| diagnostic["result_class"].as_str())
            != Some(&case.expected_result_class.to_string())
        {
            problems.push("envelope diagnostic class differs from manifest".into());
        }
    }

    if let Some(expected_code) = &case.expected_diagnostic_code
        && diagnostics
            .and_then(|values| values.first())
            .and_then(|diagnostic| diagnostic["code"].as_str())
            != Some(expected_code)
    {
        problems.push(format!(
            "envelope diagnostic code differs from declared {} branch",
            case.id
        ));
    }
    if let Some(expected_count) = case.expected_change_count
        && envelope["record"]["changes"].as_array().map(Vec::len) != Some(expected_count)
    {
        problems.push(format!(
            "envelope change count differs from declared {} branch",
            case.id
        ));
    }
    if let Some(required_path) = &case.required_change_path
        && !envelope["record"]["changes"]
            .as_array()
            .is_some_and(|changes| {
                changes
                    .iter()
                    .any(|change| change["path"] == *required_path)
            })
    {
        problems.push(format!(
            "envelope omits the declared {} change path",
            case.id
        ));
    }
    problems
}

fn run_process(
    case: &CaseSpec,
    root: &Path,
    paths: &FixturePaths,
    core: CoreObservation,
) -> RuntimeRow {
    let mut args = vec![
        OsString::from("profile-diff"),
        OsString::from("--before"),
        paths.before.as_os_str().to_owned(),
        OsString::from("--after"),
        paths.after.as_os_str().to_owned(),
        OsString::from("--format"),
    ];
    args.push(OsString::from(match case.format {
        CaseFormat::Json => "json",
        CaseFormat::Human => "human",
    }));
    let normalized_argv = vec![
        "ferris".to_owned(),
        "profile-diff".to_owned(),
        "--before".to_owned(),
        format!("<P19-RUN>/{}/before", case.input_branch),
        "--after".to_owned(),
        format!("<P19-RUN>/{}/after", case.input_branch),
        "--format".to_owned(),
        match case.format {
            CaseFormat::Json => "json".to_owned(),
            CaseFormat::Human => "human".to_owned(),
        },
    ];
    let process = Command::new(env!("CARGO_BIN_EXE_ferris"))
        .current_dir(root)
        .args(&args)
        .output();

    let mut problems = core.core_problems.clone();
    problems.extend(core.envelope_problems.clone());
    let localization = if !core.core_problems.is_empty() {
        Localization::CoreClassification
    } else if !core.envelope_problems.is_empty() {
        Localization::EnvelopeConstruction
    } else {
        Localization::NoReproduction
    };
    let process_problem_start = problems.len();

    let (started, actual_exit_code, stdout, stderr) = match process {
        Ok(output) => (true, output.status.code(), output.stdout, output.stderr),
        Err(_) => {
            problems.push("ferris process could not be started".into());
            (false, None, Vec::new(), Vec::new())
        }
    };
    let actual_route = stream_route(&stdout, &stderr);
    let privacy_canaries_absent = PRIVACY_CANARIES.iter().all(|canary| {
        !contains_bytes(&stdout, canary.as_bytes()) && !contains_bytes(&stderr, canary.as_bytes())
    });
    if !privacy_canaries_absent {
        problems.push("raw privacy canary appeared in process output".into());
    }

    let mut parsed_envelope = None;
    let mut emitted_envelope = None;
    if started {
        if actual_exit_code != Some(i32::from(case.expected_exit_code)) {
            problems.push("actual OS exit differs from manifest".into());
        }
        if actual_route != case.required_route {
            problems.push("stdout/stderr route differs from manifest".into());
        }
        match case.format {
            CaseFormat::Json => {
                let stream = single_nonempty_stream(&stdout, &stderr);
                match stream.and_then(parse_single_lf_envelope) {
                    Ok(value) => {
                        parsed_envelope = Some(envelope_receipt(&value));
                        if value != core.envelope {
                            problems.push(
                                "emitted JSON envelope differs from in-process envelope".into(),
                            );
                        }
                        emitted_envelope = Some(value);
                    }
                    Err(problem) => problems.push(problem),
                }
            }
            CaseFormat::Human => {
                if core
                    .human_render
                    .as_ref()
                    .is_none_or(|expected| stdout != *expected)
                {
                    problems.push("human output differs from public core rendering".into());
                }
            }
        }
    }

    if case.input_branch == "J23" {
        let public_output = match case.format {
            CaseFormat::Json => emitted_envelope.as_ref().map_or(Vec::new(), |value| {
                serde_json::to_vec(value).expect("serialize emitted envelope")
            }),
            CaseFormat::Human => stdout.clone(),
        };
        for required in [
            "public.profile-23",
            "public-r1",
            "public-r2",
            "public.consumer-23",
            "public~1key~0name",
        ] {
            if !contains_bytes(&public_output, required.as_bytes()) {
                problems.push(format!("public J23 output omits {required}"));
            }
        }
    }

    let process_has_problem = problems.len() > process_problem_start;
    let localization = if localization != Localization::NoReproduction {
        localization
    } else if process_has_problem || !started {
        match case.format {
            CaseFormat::Json => Localization::CliEmissionExitCode,
            CaseFormat::Human => Localization::FormatParity,
        }
    } else {
        Localization::NoReproduction
    };
    let before_input = input_evidence(&paths.before);
    let after_input = input_evidence(&paths.after);
    let receipt = ProcessRowReceipt {
        id: case.id.clone(),
        input_branch: case.input_branch.clone(),
        branch: case.branch.clone(),
        format: case.format,
        expected_result_class: case.expected_result_class.to_string(),
        expected_exit_code: case.expected_exit_code,
        required_route: case.required_route,
        argv: normalized_argv,
        working_directory: "<P19-RUN>",
        before_input,
        after_input,
        core: core.receipt,
        process_started: started,
        actual_exit_code,
        actual_route,
        stdout_bytes: stdout.len(),
        stderr_bytes: stderr.len(),
        complete_stdout_digest: sha256(&stdout),
        complete_stderr_digest: sha256(&stderr),
        parsed_envelope,
        human_parity: None,
        privacy_canaries_absent,
        localization,
        mismatches: problems.clone(),
    };
    RuntimeRow {
        receipt,
        emitted_envelope,
        stdout,
        problems,
    }
}

fn apply_format_parity(rows: &mut [RuntimeRow]) {
    for (human_id, json_id) in [("H01", "J01"), ("H02", "J02"), ("H03", "J23")] {
        let json_index = rows
            .iter()
            .position(|row| row.receipt.id == json_id)
            .expect("JSON pair row");
        let human_index = rows
            .iter()
            .position(|row| row.receipt.id == human_id)
            .expect("human pair row");
        let (problems, parity) = compare_format_pair(&rows[json_index], &rows[human_index]);
        rows[human_index].receipt.human_parity = Some(parity);
        if !problems.is_empty() {
            rows[human_index].problems.extend(problems.clone());
            rows[human_index].receipt.mismatches.extend(problems);
            if rows[human_index].receipt.localization == Localization::NoReproduction {
                rows[human_index].receipt.localization = Localization::FormatParity;
            }
        }
    }
}

fn compare_format_pair(
    json_row: &RuntimeRow,
    human_row: &RuntimeRow,
) -> (Vec<String>, HumanParityReceipt) {
    let byte_identical_inputs = json_row.receipt.before_input == human_row.receipt.before_input
        && json_row.receipt.after_input == human_row.receipt.after_input;
    let exact_core_render = !human_row
        .problems
        .iter()
        .any(|problem| problem == "human output differs from public core rendering");
    let privacy_canaries_absent =
        json_row.receipt.privacy_canaries_absent && human_row.receipt.privacy_canaries_absent;
    let public_semantics_match = json_row
        .emitted_envelope
        .as_ref()
        .is_some_and(|envelope| human_matches_envelope(&human_row.stdout, envelope));
    let mut problems = Vec::new();
    if !byte_identical_inputs {
        problems.push("human and JSON rows did not use byte-identical inputs".into());
    }
    if json_row.receipt.actual_exit_code != human_row.receipt.actual_exit_code {
        problems.push("human and JSON actual exits differ".into());
    }
    if json_row.receipt.actual_route != human_row.receipt.actual_route {
        problems.push("human and JSON stream routes differ".into());
    }
    if !exact_core_render {
        problems.push("human output differs from public core rendering".into());
    }
    if !public_semantics_match {
        problems.push("human output does not preserve JSON public semantics".into());
    }
    if !privacy_canaries_absent {
        problems.push("human/JSON privacy parity failed".into());
    }
    (
        problems,
        HumanParityReceipt {
            paired_json_row: json_row.receipt.id.clone(),
            byte_identical_inputs,
            exact_core_render,
            public_semantics_match,
            privacy_canaries_absent,
        },
    )
}

fn human_matches_envelope(human: &[u8], envelope: &Value) -> bool {
    let Ok(human) = std::str::from_utf8(human) else {
        return false;
    };
    let record = &envelope["record"];
    let Some(diff_id) = record["diff_id"].as_str() else {
        return false;
    };
    let Some(schema) = record["schema"].as_str() else {
        return false;
    };
    let Some(result_class) = envelope["result_class"].as_str() else {
        return false;
    };
    let required_headers = [
        format!("Ferris profile diff {diff_id}\n"),
        format!("Schema: {schema}\n"),
        format!("Result: {result_class}\n"),
        format!("Executable: {}\n", record["executable"]),
        format!(
            "Before: profile_id={}, revision={}, consumer={}, content_digest={}\n",
            text(&record["before"]["profile_id"]),
            text(&record["before"]["revision"]),
            text(&record["before"]["consumer"]),
            text(&record["before"]["content_digest"])
        ),
        format!(
            "After: profile_id={}, revision={}, consumer={}, content_digest={}\n",
            text(&record["after"]["profile_id"]),
            text(&record["after"]["revision"]),
            text(&record["after"]["consumer"]),
            text(&record["after"]["content_digest"])
        ),
    ];
    if required_headers.iter().any(|line| !human.contains(line)) {
        return false;
    }
    for field in ["changed_sections", "unchanged_sections"] {
        let Some(values) = record[field].as_array() else {
            return false;
        };
        if values.is_empty() {
            if !human.contains("  - none\n") {
                return false;
            }
        } else if values
            .iter()
            .filter_map(Value::as_str)
            .any(|value| !human.contains(&format!("  - {value}\n")))
        {
            return false;
        }
    }
    let Some(changes) = record["changes"].as_array() else {
        return false;
    };
    if changes.is_empty() {
        return human.contains("Changes:\n  - none\n");
    }
    changes.iter().all(|change| {
        let before = change["before_value_digest"].as_str().unwrap_or("none");
        let after = change["after_value_digest"].as_str().unwrap_or("none");
        human.contains(&format!(
            "  - {}: {} (before_digest={before}, after_digest={after})\n",
            text(&change["path"]),
            text(&change["change_kind"])
        ))
    })
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or("")
}

fn envelope_receipt(value: &Value) -> EnvelopeReceipt {
    let diagnostics = value["diagnostics"].as_array();
    let record = &value["record"];
    EnvelopeReceipt {
        schema: value["schema"].as_str().map(str::to_owned),
        semantic_command_id: value["semantic_command_id"].as_str().map(str::to_owned),
        result_class: value["result_class"].as_str().map(str::to_owned),
        process_exit_code: value["process_exit_code"].as_u64(),
        diagnostic_count: diagnostics.map(Vec::len),
        diagnostic_class: diagnostics
            .and_then(|values| values.first())
            .and_then(|diagnostic| diagnostic["result_class"].as_str())
            .map(str::to_owned),
        diagnostic_code: diagnostics
            .and_then(|values| values.first())
            .and_then(|diagnostic| diagnostic["code"].as_str())
            .map(str::to_owned),
        record_present: Some(!record.is_null()),
        record_schema: record["schema"].as_str().map(str::to_owned),
        record_change_count: record["changes"].as_array().map(Vec::len),
    }
}

fn stream_route(stdout: &[u8], stderr: &[u8]) -> StreamRoute {
    match (stdout.is_empty(), stderr.is_empty()) {
        (false, true) => StreamRoute::StdoutOnly,
        (true, false) => StreamRoute::StderrOnly,
        (false, false) => StreamRoute::Both,
        (true, true) => StreamRoute::Empty,
    }
}

fn single_nonempty_stream<'a>(stdout: &'a [u8], stderr: &'a [u8]) -> Result<&'a [u8], String> {
    match stream_route(stdout, stderr) {
        StreamRoute::StdoutOnly => Ok(stdout),
        StreamRoute::StderrOnly => Ok(stderr),
        StreamRoute::Both => Err("JSON process emitted on both streams".into()),
        StreamRoute::Empty => Err("JSON process emitted no envelope".into()),
    }
}

fn parse_single_lf_envelope(stream: &[u8]) -> Result<Value, String> {
    let Some((&last, body)) = stream.split_last() else {
        return Err("JSON stream is empty".into());
    };
    if last != b'\n' {
        return Err("JSON envelope is not followed by one LF".into());
    }
    if body.is_empty()
        || body.last().is_some_and(u8::is_ascii_whitespace)
        || stream.contains(&b'\r')
    {
        return Err("JSON stream has non-canonical outer whitespace".into());
    }
    serde_json::from_slice(body).map_err(|_| "JSON stream is not one complete envelope".into())
}

fn input_evidence(path: &Path) -> String {
    if path.is_file() {
        sha256(&fs::read(path).expect("read input evidence"))
    } else if path.is_dir() {
        "directory".to_owned()
    } else {
        "missing".to_owned()
    }
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn localization_priority(localization: Localization) -> u8 {
    match localization {
        Localization::CoreClassification => 0,
        Localization::EnvelopeConstruction => 1,
        Localization::CliEmissionExitCode => 2,
        Localization::FormatParity => 3,
        Localization::NoReproduction => 4,
    }
}

fn earlier(left: Localization, right: Localization) -> Localization {
    if localization_priority(left) <= localization_priority(right) {
        left
    } else {
        right
    }
}

fn build_branches(rows: &[RuntimeRow]) -> Vec<BranchReceipt> {
    (1..=23)
        .map(|index| {
            let branch_id = format!("J{index:02}");
            let branch_rows = rows
                .iter()
                .filter(|row| row.receipt.input_branch == branch_id)
                .collect::<Vec<_>>();
            let localization = branch_rows
                .iter()
                .fold(Localization::NoReproduction, |outcome, row| {
                    earlier(outcome, row.receipt.localization)
                });
            let mismatches = branch_rows
                .iter()
                .flat_map(|row| row.receipt.mismatches.iter().cloned())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            BranchReceipt {
                branch_id,
                process_row_ids: branch_rows
                    .iter()
                    .map(|row| row.receipt.id.clone())
                    .collect(),
                localization,
                mismatches,
            }
        })
        .collect()
}

fn environment_value(name: &str, fallback: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| fallback.to_owned())
}

fn write_receipt(receipt: &DiagnosticReceipt) {
    let Some(path) = std::env::var_os("FERRIS_P19_RECEIPT_OUT") else {
        return;
    };
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create receipt directory");
    }
    let mut bytes = serde_json::to_vec_pretty(receipt).expect("serialize diagnostic receipt");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("write diagnostic receipt");
}

#[test]
#[ignore = "authorized diagnostic: exactly 26 ferris processes and an explicit retained receipt"]
fn public_process_exit_agreement_diagnostic() {
    let (manifest, manifest_bytes) = load_manifest();
    validate_manifest(&manifest);
    let directory = DiagnosticDirectory::new();
    let fixtures = generate_fixtures(&directory.0);
    let mut rows = Vec::with_capacity(EXPECTED_PROCESS_COUNT);
    let mut attempted = 0;
    for case in &manifest.cases {
        attempted += 1;
        let paths = fixtures
            .get(&case.input_branch)
            .expect("declared public input branch");
        let core = observe_core(case, paths);
        rows.push(run_process(case, &directory.0, paths, core));
    }
    apply_format_parity(&mut rows);

    let branches = build_branches(&rows);
    let aggregate_outcome = branches
        .iter()
        .fold(Localization::NoReproduction, |outcome, branch| {
            earlier(outcome, branch.localization)
        });
    let started = rows
        .iter()
        .filter(|row| row.receipt.process_started)
        .count();
    let retained = rows.len();
    let privacy_failure = rows.iter().any(|row| !row.receipt.privacy_canaries_absent);
    let cardinality_complete = attempted == EXPECTED_PROCESS_COUNT
        && started == EXPECTED_PROCESS_COUNT
        && retained == EXPECTED_PROCESS_COUNT;
    let evidence_status = if cardinality_complete && !privacy_failure {
        "complete"
    } else {
        "invalid"
    };
    let receipt = DiagnosticReceipt {
        schema: "ferris.profile-diff-process-exit-diagnostic-receipt/v1",
        evidence_class: "public-development-diagnostic",
        evidence_status,
        run_id: environment_value("FERRIS_P19_RUN_ID", "unretained-development-run"),
        recorded_at_utc: environment_value("FERRIS_P19_RECORDED_AT_UTC", "not-recorded"),
        environment: EnvironmentReceipt {
            platform: environment_value("FERRIS_P19_PLATFORM", std::env::consts::OS),
            operating_system: environment_value(
                "FERRIS_P19_OPERATING_SYSTEM",
                std::env::consts::OS,
            ),
            kernel: environment_value("FERRIS_P19_KERNEL", "not-recorded"),
            architecture: environment_value("FERRIS_P19_ARCHITECTURE", std::env::consts::ARCH),
            rustc: environment_value("FERRIS_P19_RUSTC_VERSION", "not-recorded"),
            cargo: environment_value("FERRIS_P19_CARGO_VERSION", "not-recorded"),
        },
        baseline_git_cutoff: BASELINE_GIT_CUTOFF,
        source_cutoff: environment_value("FERRIS_P19_SOURCE_CUTOFF", "not-recorded"),
        executable_digest: sha256(
            &fs::read(env!("CARGO_BIN_EXE_ferris")).expect("read ferris executable"),
        ),
        fixture_manifest_schema: manifest.schema,
        fixture_manifest_digest: sha256(&manifest_bytes),
        process_cardinality: ProcessCardinalityReceipt {
            expected: EXPECTED_PROCESS_COUNT,
            declared: manifest.cases.len(),
            attempted,
            started,
            retained,
            retries: 0,
            missing_ids: Vec::new(),
            duplicate_ids: Vec::new(),
            extra_ids: Vec::new(),
        },
        aggregate_outcome,
        branches,
        rows: rows.into_iter().map(|row| row.receipt).collect(),
    };
    write_receipt(&receipt);

    let divergent = receipt
        .branches
        .iter()
        .filter(|branch| branch.localization != Localization::NoReproduction)
        .map(|branch| format!("{}:{:?}", branch.branch_id, branch.localization))
        .collect::<Vec<_>>();
    println!(
        "Pulse 19 diagnostic: processes={started}/{EXPECTED_PROCESS_COUNT}, aggregate={:?}",
        receipt.aggregate_outcome
    );
    assert!(
        evidence_status == "complete" && receipt.aggregate_outcome == Localization::NoReproduction,
        "Pulse 19 public diagnostic mismatch; earliest branches: {}",
        divergent.join(", ")
    );
}
