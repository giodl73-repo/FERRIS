use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-federated-scaling-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write_application(parent: &Path, workspace_count: usize) -> PathBuf {
    let root = parent.join(format!("application-{workspace_count}"));
    fs::create_dir_all(&root).expect("create application root");
    let mut workspaces = Vec::new();
    for index in 0..workspace_count {
        let workspace_name = format!("ws-{index:02}");
        let package_name = format!("fvs-{index:02}");
        let workspace_root = root.join(&workspace_name);
        fs::create_dir_all(workspace_root.join("src")).expect("create workspace source");
        fs::write(
            workspace_root.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n"
            ),
        )
        .expect("write manifest");
        fs::write(
            workspace_root.join("src/lib.rs"),
            format!("pub fn workspace_index() -> usize {{\n    {index}\n}}\n"),
        )
        .expect("write source");
        let mut workspace = json!({
            "workspace_id": format!("ferris.scaling/{workspace_name}"),
            "manifest_path": format!("{workspace_name}/Cargo.toml")
        });
        if index > 0 {
            workspace["depends_on"] = json!([format!("ferris.scaling/ws-{:02}", index - 1)]);
        }
        workspaces.push(workspace);
    }
    let application = root.join("application.json");
    fs::write(
        &application,
        serde_json::to_vec_pretty(&json!({
            "schema": "ferris.application/v0",
            "application_id": format!("ferris.scaling/application-{workspace_count}"),
            "workspaces": workspaces
        }))
        .expect("serialize application"),
    )
    .expect("write application");
    application
}

fn write_multi_package_application(
    parent: &Path,
    workspace_count: usize,
    packages_per_workspace: usize,
) -> (PathBuf, Vec<PathBuf>, Vec<String>) {
    let root = parent.join("application-mixed-inputs");
    fs::create_dir_all(&root).expect("create mixed-input application root");
    let mut workspaces = Vec::new();
    let mut changed_paths = Vec::new();
    let mut changed_packages = Vec::new();
    for workspace_index in 0..workspace_count {
        let workspace_name = format!("ws-{workspace_index:02}");
        let workspace_id = format!("ferris.scaling/{workspace_name}");
        let workspace_root = root.join(&workspace_name);
        let members = (0..packages_per_workspace)
            .map(|package_index| {
                format!("\"crates/fmx-w{workspace_index:02}-c{package_index:02}\"")
            })
            .collect::<Vec<_>>()
            .join(",\n  ");
        fs::create_dir_all(&workspace_root).expect("create mixed-input workspace");
        fs::write(
            workspace_root.join("Cargo.toml"),
            format!("[workspace]\nresolver = \"2\"\nmembers = [\n  {members}\n]\n"),
        )
        .expect("write mixed-input workspace manifest");
        for package_index in 0..packages_per_workspace {
            let package_name = format!("fmx-w{workspace_index:02}-c{package_index:02}");
            let package_root = workspace_root.join("crates").join(&package_name);
            fs::create_dir_all(package_root.join("src")).expect("create mixed-input package");
            fs::write(
                package_root.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"
                ),
            )
            .expect("write mixed-input package manifest");
            let source = package_root.join("src/lib.rs");
            fs::write(
                &source,
                format!("pub fn package_index() -> usize {{\n    {package_index}\n}}\n"),
            )
            .expect("write mixed-input package source");
            changed_paths.push(source);
            changed_packages.push(format!("{workspace_id}:{package_name}"));
        }
        workspaces.push(json!({
            "workspace_id": workspace_id,
            "manifest_path": format!("{workspace_name}/Cargo.toml")
        }));
    }
    let application = root.join("application.json");
    fs::write(
        &application,
        serde_json::to_vec_pretty(&json!({
            "schema": "ferris.application/v0",
            "application_id": "ferris.scaling/mixed-inputs",
            "workspaces": workspaces
        }))
        .expect("serialize mixed-input application"),
    )
    .expect("write mixed-input application");
    (application, changed_paths, changed_packages)
}

fn run_plan_inputs(
    application: &Path,
    changed_paths: &[PathBuf],
    changed_packages: &[String],
    packages_first: bool,
) -> Output {
    let application_root = application.parent().expect("application parent");
    let mut command = Command::new(env!("CARGO_BIN_EXE_ferris"));
    command
        .current_dir(application_root)
        .arg("federated-validation-plan")
        .arg("--application")
        .arg(application.file_name().expect("application file name"));
    if packages_first {
        for package in changed_packages {
            command.arg("--changed-package").arg(package);
        }
        for path in changed_paths {
            command.arg("--changed-path").arg(
                path.strip_prefix(application_root)
                    .expect("relative changed path"),
            );
        }
    } else {
        for path in changed_paths {
            command.arg("--changed-path").arg(
                path.strip_prefix(application_root)
                    .expect("relative changed path"),
            );
        }
        for package in changed_packages {
            command.arg("--changed-package").arg(package);
        }
    }
    command
        .args(["--format", "json"])
        .output()
        .expect("run mixed-input federated validation plan")
}

fn run_plan(application: &Path, changed_package: &str) -> (Output, Duration) {
    let started = Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_ferris"))
        .arg("federated-validation-plan")
        .arg("--application")
        .arg(application)
        .arg("--changed-package")
        .arg(changed_package)
        .args(["--format", "json"])
        .output()
        .expect("run federated validation plan");
    (output, started.elapsed())
}

fn successful_value(output: Output) -> Value {
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).expect("federated validation JSON")
}

fn disposition_count(value: &Value, disposition: &str) -> usize {
    value["record"]["workspaces"]
        .as_array()
        .expect("workspaces")
        .iter()
        .filter(|workspace| workspace["disposition"] == disposition)
        .count()
}

#[test]
fn accepts_declared_sizes_through_sixteen_and_propagates_the_full_chain() {
    let directory = TestDirectory::new("accepted-sizes");
    for workspace_count in [2usize, 4, 8, 16] {
        let application = write_application(directory.path(), workspace_count);
        let leaf = format!(
            "ferris.scaling/ws-{:02}:fvs-{:02}",
            workspace_count - 1,
            workspace_count - 1
        );
        let (leaf_output, _) = run_plan(&application, &leaf);
        let leaf_value = successful_value(leaf_output);
        assert_eq!(
            leaf_value["record"]["workspaces"]
                .as_array()
                .expect("workspaces")
                .len(),
            workspace_count
        );
        assert_eq!(disposition_count(&leaf_value, "direct_plan"), 1);
        assert_eq!(disposition_count(&leaf_value, "relationship_fallback"), 0);
        assert_eq!(
            disposition_count(&leaf_value, "not_selected"),
            workspace_count - 1
        );

        let (root_output, _) = run_plan(&application, "ferris.scaling/ws-00:fvs-00");
        let root_value = successful_value(root_output);
        assert_eq!(disposition_count(&root_value, "direct_plan"), 1);
        assert_eq!(
            disposition_count(&root_value, "relationship_fallback"),
            workspace_count - 1
        );
        assert_eq!(disposition_count(&root_value, "not_selected"), 0);
    }
}

#[test]
fn rejects_seventeen_workspaces_before_owner_metadata_loading() {
    let directory = TestDirectory::new("rejected-size");
    let application = write_application(directory.path(), 17);
    let (output, _) = run_plan(&application, "ferris.scaling/ws-16:fvs-16");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let error: Value = serde_json::from_slice(&output.stderr).expect("typed error JSON");
    assert_eq!(error["result_class"], "invalid");
    assert_eq!(
        error["diagnostics"][0]["code"],
        "FERRIS-APPLICATION-WORKSPACE-COUNT-INVALID"
    );
}

#[test]
fn maximum_mixed_inputs_are_order_invariant_and_overflow_is_blocked() {
    let directory = TestDirectory::new("maximum-mixed-inputs");
    let (application, paths, packages) = write_multi_package_application(directory.path(), 16, 8);
    assert_eq!(paths.len() + packages.len(), 256);

    let forward = successful_value(run_plan_inputs(&application, &paths, &packages, false));
    let mut reversed_paths = paths.clone();
    reversed_paths.reverse();
    let mut reversed_packages = packages.clone();
    reversed_packages.reverse();
    let reverse = successful_value(run_plan_inputs(
        &application,
        &reversed_paths,
        &reversed_packages,
        true,
    ));
    let offset = paths.len() / 3;
    let rotated_paths = paths
        .iter()
        .cycle()
        .skip(offset)
        .take(paths.len())
        .cloned()
        .collect::<Vec<_>>();
    let rotated_packages = packages
        .iter()
        .cycle()
        .skip(offset)
        .take(packages.len())
        .cloned()
        .collect::<Vec<_>>();
    let rotated = successful_value(run_plan_inputs(
        &application,
        &rotated_paths,
        &rotated_packages,
        false,
    ));

    assert_eq!(reverse, forward);
    assert_eq!(rotated, forward);
    assert_eq!(disposition_count(&forward, "direct_plan"), 16);
    let selected_package_count = forward["record"]["workspaces"]
        .as_array()
        .expect("workspaces")
        .iter()
        .map(|workspace| {
            workspace["validation_plan"]["selected_packages"]
                .as_array()
                .expect("selected packages")
                .len()
        })
        .sum::<usize>();
    assert_eq!(selected_package_count, 128);

    let extra_path = application
        .parent()
        .expect("application parent")
        .join("policy.txt");
    fs::write(&extra_path, "application policy\n").expect("write extra input");
    let mut overflowing_paths = paths;
    overflowing_paths.push(extra_path);
    fs::remove_file(
        application
            .parent()
            .expect("application parent")
            .join("ws-00/Cargo.toml"),
    )
    .expect("remove owner manifest");
    let overflow = run_plan_inputs(&application, &overflowing_paths, &packages, false);
    assert_eq!(overflow.status.code(), Some(7));
    assert!(overflow.stdout.is_empty());
    let error: Value = serde_json::from_slice(&overflow.stderr).expect("typed error JSON");
    assert_eq!(error["result_class"], "blocked");
    assert_eq!(
        error["diagnostics"][0]["code"],
        "FERRIS-FEDERATED-VALIDATION-INPUT-BOUND-EXCEEDED"
    );
}

#[test]
#[ignore = "run explicitly to emit local sequential metadata scaling samples"]
fn report_local_workspace_scaling() {
    let directory = TestDirectory::new("measurement");
    for workspace_count in [2usize, 4, 8, 16] {
        let application = write_application(directory.path(), workspace_count);
        let changed_package = format!(
            "ferris.scaling/ws-{:02}:fvs-{:02}",
            workspace_count - 1,
            workspace_count - 1
        );
        let mut samples = Vec::new();
        for _ in 0..5 {
            let (output, elapsed) = run_plan(&application, &changed_package);
            let value = successful_value(output);
            assert_eq!(disposition_count(&value, "direct_plan"), 1);
            assert_eq!(
                disposition_count(&value, "not_selected"),
                workspace_count - 1
            );
            samples.push(elapsed);
        }
        samples.sort();
        let median = samples[samples.len() / 2];
        println!(
            "{}",
            json!({
                "schema": "ferris.federated-validation-scaling/v0",
                "workspace_count": workspace_count,
                "samples": samples.len(),
                "median_planning_ms": median.as_secs_f64() * 1000.0,
                "required_workspace_scopes": 1,
                "avoided_workspace_scopes": workspace_count - 1,
                "validation_executed": false
            })
        );
    }
}
