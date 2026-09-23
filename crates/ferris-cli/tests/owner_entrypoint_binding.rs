use ferris_core::{
    ACTION_PLAN_SCHEMA, ActionPlan, OWNER_ENTRYPOINTS_SCHEMA, OwnerEntrypointDeclaration,
    file_content_identity, owner_entrypoint_declaration_identity, owner_entrypoint_identity,
};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

struct TestRepository {
    root: PathBuf,
    executable: String,
}

impl TestRepository {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "ferris-entrypoint-binding-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("bin")).expect("create bin");
        fs::create_dir_all(root.join("work")).expect("create work");
        fs::write(root.join("work").join(".keep"), b"").expect("retain work directory");
        fs::write(root.join(".gitattributes"), b"* -text\n")
            .expect("disable checkout byte normalization");
        let executable_name = if cfg!(windows) {
            "owner-command.exe"
        } else {
            "owner-command"
        };
        let executable = format!("bin/{executable_name}");
        fs::copy(
            std::env::current_exe().expect("test executable"),
            root.join(&executable),
        )
        .expect("stage owner executable");
        fs::write(root.join("config.txt"), "owner configuration\n").expect("write config");
        fs::write(
            root.join("intents.json"),
            serde_json::to_vec_pretty(&base_intents(&executable)).expect("serialize intents"),
        )
        .expect("write intents");
        run_git(&root, &["init", "--quiet"]);
        run_git(&root, &["config", "user.email", "ferris@example.invalid"]);
        run_git(&root, &["config", "user.name", "Ferris Test"]);
        run_git(&root, &["add", "."]);
        run_git(
            &root,
            &[
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "-m",
                "fixture",
            ],
        );
        Self { root, executable }
    }

    fn write_intents(&self, name: &str, value: &Value) {
        fs::write(
            self.root.join(name),
            serde_json::to_vec_pretty(value).expect("serialize intents"),
        )
        .expect("write intents");
    }
}

impl Drop for TestRepository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn base_intents(executable: &str) -> Value {
    json!({
        "schema": "ferris.owner-entrypoint-intents/v1",
        "entrypoints": [{
            "entrypoint_id": "owner/check",
            "owner": "owner/build-platform",
            "executable": executable,
            "argv": ["--exact", "owner check", "--flag=value", ""],
            "working_directory": "work",
            "inherited_environment": ["FERRIS_MODE", "RUST_BACKTRACE"],
            "credential_class": "none",
            "bound_files": [executable, "config.txt"]
        }]
    })
}

fn run_git(root: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{output:?}");
}

fn git_output(root: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout)
        .expect("Git output UTF-8")
        .trim()
        .to_owned()
}

fn bind(root: &Path, intents: &str, output: &str) -> Output {
    ferris()
        .current_dir(root)
        .args([
            "bind-owner-entrypoints",
            "--intents",
            intents,
            "--output",
            output,
            "--format",
            "json",
        ])
        .output()
        .expect("bind owner entrypoints")
}

fn diagnostic_code(output: &Output) -> String {
    assert!(!output.status.success(), "unexpected success: {output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let value: Value = serde_json::from_slice(&output.stderr).expect("error envelope");
    assert_eq!(value["semantic_command_id"], "bind-owner-entrypoints");
    value["diagnostics"][0]["code"]
        .as_str()
        .expect("diagnostic code")
        .to_owned()
}

#[test]
fn binds_deterministic_revision_files_and_exact_argv() {
    let repository = TestRepository::new();
    let first = bind(&repository.root, "intents.json", "entrypoints-a.json");
    assert!(first.status.success(), "{first:?}");
    assert!(first.stderr.is_empty(), "{first:?}");
    let declaration: OwnerEntrypointDeclaration =
        serde_json::from_slice(&first.stdout).expect("entrypoint declaration");
    assert_eq!(declaration.schema, OWNER_ENTRYPOINTS_SCHEMA);
    assert_eq!(
        declaration.declaration_id,
        owner_entrypoint_declaration_identity(&declaration)
    );
    assert_eq!(
        declaration.source_revision,
        git_output(&repository.root, &["rev-parse", "HEAD"])
    );
    assert_eq!(declaration.entrypoints.len(), 1);
    let entrypoint = &declaration.entrypoints[0];
    assert_eq!(
        entrypoint.entrypoint_identity,
        owner_entrypoint_identity(entrypoint)
    );
    assert_eq!(
        entrypoint.command.argv,
        ["--exact", "owner check", "--flag=value", ""]
    );
    assert_eq!(
        entrypoint.command.files[0].identity,
        file_content_identity(&repository.root.join(&repository.executable))
            .expect("executable identity")
    );
    assert_eq!(
        entrypoint.command.files[1].identity,
        file_content_identity(&repository.root.join("config.txt")).expect("config identity")
    );
    assert_eq!(
        fs::read(repository.root.join("entrypoints-a.json")).expect("bound output"),
        first.stdout
    );

    let second = bind(&repository.root, "intents.json", "entrypoints-b.json");
    assert!(second.status.success(), "{second:?}");
    assert_eq!(first.stdout, second.stdout);

    let clone = repository.root.with_extension("clone");
    let cloned = Command::new("git")
        .args(["clone", "--quiet"])
        .arg(&repository.root)
        .arg(&clone)
        .output()
        .expect("clone equivalent repository");
    assert!(cloned.status.success(), "{cloned:?}");
    let equivalent = bind(&clone, "intents.json", "entrypoints.json");
    assert!(equivalent.status.success(), "{equivalent:?}");
    assert_eq!(first.stdout, equivalent.stdout);
    fs::remove_dir_all(clone).expect("remove equivalent repository");
}

#[test]
fn output_is_consumed_directly_by_action_plan_preparation() {
    let repository = TestRepository::new();
    let binding = bind(&repository.root, "intents.json", "entrypoints.json");
    assert!(binding.status.success(), "{binding:?}");
    let prepared = ferris()
        .current_dir(&repository.root)
        .args([
            "prepare-action-plan",
            "--entrypoints",
            "entrypoints.json",
            "--entrypoint",
            "owner/check",
            "--lane-id",
            "check",
            "--owner-gate-id",
            "owner/check",
            "--repository-id",
            "test/repository",
            "--topology-id",
            "test/topology",
            "--required",
            "true",
            "--timeout-ms",
            "2000",
            "--stdout-limit-bytes",
            "65536",
            "--stderr-limit-bytes",
            "65536",
            "--output",
            "action-plan.json",
            "--format",
            "json",
        ])
        .output()
        .expect("prepare Action Plan");
    assert!(prepared.status.success(), "{prepared:?}");
    let plan: ActionPlan = serde_json::from_slice(&prepared.stdout).expect("Action Plan");
    assert_eq!(plan.schema, ACTION_PLAN_SCHEMA);
    assert_eq!(plan.lanes[0].command.argv[1], "owner check");
}

#[test]
fn rejects_shape_duplicate_and_command_contract_violations() {
    let repository = TestRepository::new();
    let baseline = base_intents(&repository.executable);
    let mut cases = Vec::new();

    let mut unknown = baseline.clone();
    unknown["entrypoints"][0]["unexpected"] = json!(true);
    cases.push((
        "unknown.json",
        unknown,
        "FERRIS-EXECUTION-INPUT-SHAPE-INVALID",
    ));

    let mut duplicate_entrypoint = baseline.clone();
    duplicate_entrypoint["entrypoints"] = json!([
        baseline["entrypoints"][0].clone(),
        baseline["entrypoints"][0].clone()
    ]);
    cases.push((
        "duplicate-entrypoint.json",
        duplicate_entrypoint,
        "FERRIS-ENTRYPOINT-BINDING-ENTRYPOINT-DUPLICATE",
    ));

    let mut duplicate_file = baseline.clone();
    duplicate_file["entrypoints"][0]["bound_files"] =
        json!([repository.executable, repository.executable]);
    cases.push((
        "duplicate-file.json",
        duplicate_file,
        "FERRIS-EXECUTION-FILE-DUPLICATE",
    ));

    let mut executable_unbound = baseline.clone();
    executable_unbound["entrypoints"][0]["bound_files"] = json!(["config.txt"]);
    cases.push((
        "executable-unbound.json",
        executable_unbound,
        "FERRIS-EXECUTION-EXECUTABLE-UNBOUND",
    ));

    let mut credentials = baseline.clone();
    credentials["entrypoints"][0]["credential_class"] = json!("ambient");
    cases.push((
        "credentials.json",
        credentials,
        "FERRIS-EXECUTION-CREDENTIAL-CLASS-UNSUPPORTED",
    ));

    let mut environment = baseline.clone();
    environment["entrypoints"][0]["inherited_environment"] =
        json!(["RUST_BACKTRACE", "FERRIS_MODE"]);
    cases.push((
        "environment.json",
        environment,
        "FERRIS-EXECUTION-ENVIRONMENT-ORDER-INVALID",
    ));

    for (index, (name, value, expected)) in cases.into_iter().enumerate() {
        repository.write_intents(name, &value);
        let output = bind(&repository.root, name, &format!("rejected-{index}.json"));
        assert_eq!(diagnostic_code(&output), expected, "case {name}");
        assert!(
            !repository
                .root
                .join(format!("rejected-{index}.json"))
                .exists()
        );
    }
}

#[test]
fn rejects_malformed_and_duplicate_key_json() {
    let repository = TestRepository::new();
    fs::write(repository.root.join("malformed.json"), b"{not json\n")
        .expect("write malformed JSON");
    assert_eq!(
        diagnostic_code(&bind(
            &repository.root,
            "malformed.json",
            "malformed-output.json"
        )),
        "FERRIS-EXECUTION-INPUT-JSON-INVALID"
    );

    fs::write(
        repository.root.join("duplicate-key.json"),
        br#"{"schema":"ferris.owner-entrypoint-intents/v1","schema":"ferris.owner-entrypoint-intents/v1","entrypoints":[]}"#,
    )
    .expect("write duplicate-key JSON");
    assert_eq!(
        diagnostic_code(&bind(
            &repository.root,
            "duplicate-key.json",
            "duplicate-key-output.json"
        )),
        "FERRIS-EXECUTION-INPUT-JSON-INVALID"
    );
}

#[test]
fn rejects_intents_larger_than_one_mebibyte() {
    let repository = TestRepository::new();
    fs::write(
        repository.root.join("oversized.json"),
        vec![b' '; 1024 * 1024 + 1],
    )
    .expect("write oversized intents");
    assert_eq!(
        diagnostic_code(&bind(
            &repository.root,
            "oversized.json",
            "oversized-output.json"
        )),
        "FERRIS-ENTRYPOINT-BINDING-INTENTS-BOUND-INVALID"
    );
}

#[test]
fn rejects_out_of_root_and_unavailable_paths() {
    let repository = TestRepository::new();
    let mut escaped_executable = base_intents(&repository.executable);
    escaped_executable["entrypoints"][0]["executable"] = json!("../outside.exe");
    escaped_executable["entrypoints"][0]["bound_files"] = json!(["../outside.exe"]);
    repository.write_intents("escaped.json", &escaped_executable);
    assert_eq!(
        diagnostic_code(&bind(
            &repository.root,
            "escaped.json",
            "escaped-output.json"
        )),
        "FERRIS-EXECUTION-RELATIVE-PATH-INVALID"
    );

    let mut missing = base_intents(&repository.executable);
    missing["entrypoints"][0]["bound_files"] = json!([repository.executable, "missing.txt"]);
    repository.write_intents("missing.json", &missing);
    assert_eq!(
        diagnostic_code(&bind(
            &repository.root,
            "missing.json",
            "missing-output.json"
        )),
        "FERRIS-EXECUTION-PATH-UNAVAILABLE"
    );

    let outside = repository
        .root
        .parent()
        .expect("repository parent")
        .join(format!("{}-outside.json", std::process::id()));
    fs::write(
        &outside,
        serde_json::to_vec(&base_intents(&repository.executable)).unwrap(),
    )
    .expect("write outside intents");
    let outside_input = ferris()
        .current_dir(&repository.root)
        .arg("bind-owner-entrypoints")
        .arg("--intents")
        .arg(&outside)
        .args(["--output", "outside-input-output.json", "--format", "json"])
        .output()
        .expect("bind outside input");
    assert_eq!(
        diagnostic_code(&outside_input),
        "FERRIS-ENTRYPOINT-BINDING-INTENTS-PATH-INVALID"
    );
    let _ = fs::remove_file(&outside);

    let outside_output = repository
        .root
        .parent()
        .expect("repository parent")
        .join(format!("{}-bound.json", std::process::id()));
    let output = ferris()
        .current_dir(&repository.root)
        .arg("bind-owner-entrypoints")
        .args(["--intents", "intents.json", "--output"])
        .arg(&outside_output)
        .args(["--format", "json"])
        .output()
        .expect("bind outside output");
    assert_eq!(
        diagnostic_code(&output),
        "FERRIS-ENTRYPOINT-BINDING-OUTPUT-INVALID"
    );
    assert!(!outside_output.exists());
}

#[test]
fn never_overwrites_existing_output() {
    let repository = TestRepository::new();
    fs::write(repository.root.join("entrypoints.json"), "owner data\n")
        .expect("write existing output");
    let output = bind(&repository.root, "intents.json", "entrypoints.json");
    assert_eq!(
        diagnostic_code(&output),
        "FERRIS-ENTRYPOINT-BINDING-OUTPUT-EXISTS"
    );
    assert_eq!(
        fs::read_to_string(repository.root.join("entrypoints.json")).expect("existing output"),
        "owner data\n"
    );
}

#[test]
fn human_output_reports_bound_identity_revision_count_and_path() {
    let repository = TestRepository::new();
    let output = ferris()
        .current_dir(&repository.root)
        .args([
            "bind-owner-entrypoints",
            "--intents",
            "intents.json",
            "--output",
            "entrypoints.json",
        ])
        .output()
        .expect("bind owner entrypoints");
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).expect("human output UTF-8");
    assert!(stdout.starts_with("Ferris owner entrypoints bound\n"));
    assert!(stdout.contains("Declaration: sha256:"));
    assert!(stdout.contains("Entrypoints: 1"));
    assert!(stdout.contains(&format!(
        "Source revision: {}",
        git_output(&repository.root, &["rev-parse", "HEAD"])
    )));
    assert!(stdout.contains("entrypoints.json"));
}
