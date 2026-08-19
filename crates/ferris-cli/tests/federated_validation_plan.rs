use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn cargo_ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cargo-ferris"))
}

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(path)
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-federated-validation-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create test directory");
        Self(path)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create fixture destination");
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("fixture entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().expect("fixture file type").is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(source_path, destination_path).expect("copy fixture file");
        }
    }
}

fn run_plan(
    command: &mut Command,
    application: &Path,
    changed_paths: &[&Path],
    changed_packages: &[&str],
    format: &str,
) -> Output {
    command
        .arg("federated-validation-plan")
        .arg("--application")
        .arg(application);
    for changed_path in changed_paths {
        command.arg("--changed-path").arg(changed_path);
    }
    for changed_package in changed_packages {
        command.arg("--changed-package").arg(changed_package);
    }
    command
        .args(["--format", format])
        .output()
        .expect("run federated-validation-plan")
}

fn assert_typed_error(output: Output, code: &str, result_class: &str, exit_code: i32) -> Value {
    assert_eq!(output.status.code(), Some(exit_code), "{code} exit");
    assert!(output.stdout.is_empty(), "{code} stdout");
    let value: Value = serde_json::from_slice(&output.stderr).expect("typed error JSON");
    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["semantic_command_id"], "federated-validation-plan");
    assert_eq!(value["result_class"], result_class);
    assert_eq!(value["diagnostics"][0]["code"], code);
    value
}

fn assert_path_free(value: &Value, paths: &[&Path]) {
    let serialized = serde_json::to_string(value).expect("serialize error");
    let repository_root = repository_root();
    assert!(
        !serialized.contains(&repository_root.to_string_lossy().into_owned()),
        "error leaked repository root"
    );
    for path in paths {
        assert!(
            !serialized.contains(&path.to_string_lossy().into_owned()),
            "error leaked path"
        );
    }
}

fn application_value() -> Value {
    serde_json::from_slice(
        &fs::read(fixture("sibling-workspaces/application.json")).expect("read application"),
    )
    .expect("application JSON")
}

fn write_application(path: &Path, value: &Value) {
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("serialize application"),
    )
    .expect("write application");
}

#[test]
fn direct_selection_uses_current_validation_plan_and_transitive_fallback() {
    let application = fixture("sibling-workspaces/application.json");
    let output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let value: Value = serde_json::from_slice(&output.stdout).expect("federated validation JSON");

    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["semantic_command_id"], "federated-validation-plan");
    assert_eq!(
        value["record"]["schema"],
        "ferris.federated-validation-plan/v0"
    );
    assert_eq!(value["record"]["executable"], false);
    assert_eq!(
        value["record"]["workspaces"][0]["workspace_id"],
        "ferris.test/gateway"
    );
    assert_eq!(
        value["record"]["workspaces"][0]["disposition"],
        "relationship_fallback"
    );
    assert_eq!(
        value["record"]["workspaces"][1]["workspace_id"],
        "ferris.test/selected"
    );
    assert_eq!(
        value["record"]["workspaces"][1]["disposition"],
        "direct_plan"
    );
    assert_eq!(
        value["record"]["workspaces"][1]["validation_plan"]["schema"],
        "ferris.validation-plan/v0"
    );
    assert_eq!(
        value["record"]["workspaces"][2]["workspace_id"],
        "ferris.test/sibling"
    );
    assert_eq!(
        value["record"]["workspaces"][2]["disposition"],
        "relationship_fallback"
    );

    let standalone = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/selected",
            "--manifest-path",
        ])
        .arg(fixture("sibling-workspaces/selected/Cargo.toml"))
        .args(["--changed-package", "selected-member", "--format", "json"])
        .output()
        .expect("run standalone validation-plan");
    assert!(standalone.status.success());
    let standalone_value: Value =
        serde_json::from_slice(&standalone.stdout).expect("standalone validation JSON");
    assert_eq!(
        value["record"]["workspaces"][1]["validation_plan"],
        standalone_value["record"]
    );

    let human = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "human",
    );
    assert!(human.status.success());
    assert!(human.stderr.is_empty());
    let human = String::from_utf8(human.stdout).expect("human output");
    assert!(human.contains("Ferris federated validation plan"));
    assert!(human.contains("direct plan"));
    assert!(human.contains("relationship fallback"));
    assert!(human.contains("Executable: no"));
}

#[test]
fn direct_and_cargo_adapters_return_identical_json_and_help() {
    let application = fixture("sibling-workspaces/application.json");
    let ferris_output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    let direct_output = run_plan(
        &mut cargo_ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    let cargo_style_output = cargo_ferris()
        .arg("ferris")
        .arg("federated-validation-plan")
        .arg("--application")
        .arg(&application)
        .args([
            "--changed-package",
            "ferris.test/selected:selected-member",
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style adapter");
    for output in [&ferris_output, &direct_output, &cargo_style_output] {
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
    }
    let ferris_value: Value = serde_json::from_slice(&ferris_output.stdout).expect("ferris JSON");
    let direct_value: Value =
        serde_json::from_slice(&direct_output.stdout).expect("direct adapter JSON");
    let cargo_style_value: Value =
        serde_json::from_slice(&cargo_style_output.stdout).expect("cargo-style JSON");
    assert_eq!(direct_value, ferris_value);
    assert_eq!(cargo_style_value, ferris_value);

    for mut command in [ferris(), cargo_ferris()] {
        let help = command.arg("--help").output().expect("run help");
        assert!(help.status.success());
        assert!(
            String::from_utf8(help.stdout)
                .expect("help output")
                .contains("federated-validation-plan")
        );
    }
}

#[test]
fn application_level_path_widens_every_workspace() {
    let application = fixture("sibling-workspaces/application.json");
    let policy = fixture("sibling-workspaces/application-policy.txt");
    let output = run_plan(&mut ferris(), &application, &[&policy], &[], "json");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("application fallback JSON");
    assert_eq!(value["record"]["fallback"]["required_by_inputs"], true);
    for workspace in value["record"]["workspaces"]
        .as_array()
        .expect("workspaces")
    {
        assert_eq!(workspace["disposition"], "application_fallback");
        assert!(workspace.get("validation_plan").is_none());
    }
    let serialized = String::from_utf8(output.stdout).expect("JSON output");
    assert!(serialized.contains("application-policy.txt"));
    assert!(!serialized.contains(&repository_root().to_string_lossy().into_owned()));
}

#[test]
fn unknown_workspace_package_and_invalid_qualifier_are_typed_and_path_free() {
    let application = fixture("sibling-workspaces/application.json");
    let unknown_workspace = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/missing:selected-member"],
        "json",
    );
    let unknown_workspace = assert_typed_error(
        unknown_workspace,
        "FERRIS-FEDERATED-VALIDATION-PACKAGE-WORKSPACE-NOT-FOUND",
        "invalid",
        2,
    );
    assert_path_free(&unknown_workspace, &[&application]);

    let unknown_package = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:missing-package"],
        "json",
    );
    let unknown_package = assert_typed_error(
        unknown_package,
        "FERRIS-VALIDATION-PACKAGE-NOT-FOUND",
        "invalid",
        2,
    );
    assert_path_free(&unknown_package, &[&application]);

    let invalid_qualifier = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["selected-member"],
        "json",
    );
    let invalid_qualifier = assert_typed_error(
        invalid_qualifier,
        "FERRIS-FEDERATED-VALIDATION-PACKAGE-QUALIFIER-INVALID",
        "invalid",
        2,
    );
    assert_path_free(&invalid_qualifier, &[&application]);
}

#[test]
fn definitions_reject_cycles_duplicates_traversal_and_unknown_fields() {
    let directory = TestDirectory::new("invalid-definitions");
    copy_tree(
        &fixture("sibling-workspaces"),
        &directory.path("application"),
    );
    let application = directory.path("application/application.json");

    let mut cycle = application_value();
    cycle["workspaces"][0]["depends_on"] = json!(["ferris.test/sibling"]);
    write_application(&application, &cycle);
    let cycle_output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    assert_typed_error(
        cycle_output,
        "FERRIS-APPLICATION-DEPENDENCY-CYCLE",
        "invalid",
        2,
    );

    let mut duplicate_id = application_value();
    duplicate_id["workspaces"][2]["workspace_id"] = json!("ferris.test/selected");
    write_application(&application, &duplicate_id);
    let duplicate_id_output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    assert_typed_error(
        duplicate_id_output,
        "FERRIS-APPLICATION-WORKSPACE-ID-DUPLICATE",
        "invalid",
        2,
    );

    let mut duplicate_root = application_value();
    duplicate_root["workspaces"][1]["manifest_path"] = json!("selected/selected-member/Cargo.toml");
    duplicate_root["workspaces"][1]["depends_on"] = json!([]);
    write_application(&application, &duplicate_root);
    let duplicate_root_output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    assert_typed_error(
        duplicate_root_output,
        "FERRIS-APPLICATION-WORKSPACE-ROOT-DUPLICATE",
        "invalid",
        2,
    );

    let mut traversal = application_value();
    traversal["workspaces"][0]["manifest_path"] = json!("../outside/Cargo.toml");
    write_application(&application, &traversal);
    let traversal_output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    assert_typed_error(
        traversal_output,
        "FERRIS-APPLICATION-MANIFEST-PATH-INVALID",
        "invalid",
        2,
    );

    let mut unknown_field = application_value();
    unknown_field["owner"] = json!("not-supported");
    write_application(&application, &unknown_field);
    let unknown_field_output = run_plan(
        &mut ferris(),
        &application,
        &[],
        &["ferris.test/selected:selected-member"],
        "json",
    );
    assert_typed_error(
        unknown_field_output,
        "FERRIS-APPLICATION-INPUT-INVALID",
        "invalid",
        2,
    );
}

#[test]
fn changed_paths_outside_the_application_are_rejected_without_path_disclosure() {
    let application = fixture("sibling-workspaces/application.json");
    let outside = fixture("simple-workspace/Cargo.toml");
    let output = run_plan(&mut ferris(), &application, &[&outside], &[], "json");
    let value = assert_typed_error(
        output,
        "FERRIS-FEDERATED-VALIDATION-CHANGE-PATH-OUTSIDE-APPLICATION",
        "invalid",
        2,
    );
    assert_path_free(&value, &[&application, &outside]);
}

#[test]
fn identities_are_stable_after_relocating_equivalent_fixtures() {
    let first = TestDirectory::new("relocation-a");
    let second = TestDirectory::new("relocation-b");
    copy_tree(
        &fixture("sibling-workspaces"),
        &first.path("sibling-workspaces"),
    );
    copy_tree(
        &fixture("sibling-workspaces"),
        &second.path("sibling-workspaces"),
    );
    let first_application = first.path("sibling-workspaces/definition-a.json");
    let second_application = second.path("sibling-workspaces/renamed-definition.json");
    fs::rename(
        first.path("sibling-workspaces/application.json"),
        &first_application,
    )
    .expect("rename first application definition");
    fs::rename(
        second.path("sibling-workspaces/application.json"),
        &second_application,
    )
    .expect("rename second application definition");
    let first_changed = first.path("sibling-workspaces/selected/selected-member/src/lib.rs");
    let second_changed = second.path("sibling-workspaces/selected/selected-member/src/lib.rs");
    let first_output = run_plan(
        &mut ferris(),
        &first_application,
        &[&first_changed],
        &[],
        "json",
    );
    let second_output = run_plan(
        &mut ferris(),
        &second_application,
        &[&second_changed],
        &[],
        "json",
    );
    assert!(first_output.status.success());
    assert!(second_output.status.success());
    let first_value: Value = serde_json::from_slice(&first_output.stdout).expect("first JSON");
    let second_value: Value = serde_json::from_slice(&second_output.stdout).expect("second JSON");
    assert_eq!(
        first_value["selection_identity"],
        second_value["selection_identity"]
    );
    assert_eq!(
        first_value["invocation_identity"],
        second_value["invocation_identity"]
    );
    assert_eq!(
        first_value["record"]["federated_validation_plan_id"],
        second_value["record"]["federated_validation_plan_id"]
    );
    assert_ne!(
        first_value["record"]["application_definition"],
        second_value["record"]["application_definition"]
    );
    assert_eq!(
        first_value["record"]["workspaces"][1]["validation_plan"]["validation_plan_id"],
        second_value["record"]["workspaces"][1]["validation_plan"]["validation_plan_id"]
    );
    let first_text = String::from_utf8(first_output.stdout).expect("first output");
    let second_text = String::from_utf8(second_output.stdout).expect("second output");
    assert!(!first_text.contains(&first.0.to_string_lossy().into_owned()));
    assert!(!second_text.contains(&second.0.to_string_lossy().into_owned()));

    let first_error = run_plan(
        &mut ferris(),
        &first_application,
        &[],
        &["ferris.test/selected:missing-package"],
        "json",
    );
    let second_error = run_plan(
        &mut ferris(),
        &second_application,
        &[],
        &["ferris.test/selected:missing-package"],
        "json",
    );
    let first_error = assert_typed_error(
        first_error,
        "FERRIS-VALIDATION-PACKAGE-NOT-FOUND",
        "invalid",
        2,
    );
    let second_error = assert_typed_error(
        second_error,
        "FERRIS-VALIDATION-PACKAGE-NOT-FOUND",
        "invalid",
        2,
    );
    assert_eq!(
        first_error["selection_identity"],
        second_error["selection_identity"]
    );
    assert_eq!(
        first_error["invocation_identity"],
        second_error["invocation_identity"]
    );
}

#[test]
fn existing_validation_and_federated_plan_commands_remain_unchanged() {
    let validation = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
        ])
        .arg(fixture("simple-workspace/Cargo.toml"))
        .args(["--changed-package", "fixture-alpha", "--format", "json"])
        .output()
        .expect("run validation-plan");
    assert!(validation.status.success());
    let validation: Value =
        serde_json::from_slice(&validation.stdout).expect("validation-plan JSON");
    assert_eq!(validation["semantic_command_id"], "validation-plan");
    assert_eq!(validation["record"]["schema"], "ferris.validation-plan/v0");

    let federated = ferris()
        .args(["federated-plan", "--request"])
        .arg(fixture("federated-plan/request.json"))
        .args(["--format", "json"])
        .output()
        .expect("run federated-plan");
    assert!(federated.status.success());
    let federated: Value = serde_json::from_slice(&federated.stdout).expect("federated-plan JSON");
    assert_eq!(federated["semantic_command_id"], "federated-plan");
    assert_eq!(federated["record"]["schema"], "ferris.federated-plan/v0");
}
