use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryTree {
    root: PathBuf,
}

impl TemporaryTree {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "ferris-platform-lifecycle-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("temporary root");
        Self { root }
    }
}

impl Drop for TemporaryTree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination");
    for entry in fs::read_dir(source).expect("source directory") {
        let entry = entry.expect("source entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy file");
        }
    }
}

fn replace_tree(source: &Path, destination: &Path) {
    if destination.exists() {
        fs::remove_dir_all(destination).expect("remove isolated tree");
    }
    copy_tree(source, destination);
}

fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn collect(root: &Path, directory: &Path, values: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(directory).expect("snapshot directory") {
            let entry = entry.expect("snapshot entry");
            if entry.file_type().expect("snapshot type").is_dir() {
                collect(root, &entry.path(), values);
            } else {
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .expect("relative path")
                    .to_string_lossy()
                    .replace('\\', "/");
                values.insert(relative, fs::read(entry.path()).expect("snapshot file"));
            }
        }
    }
    let mut values = BTreeMap::new();
    collect(root, root, &mut values);
    values
}

fn owner_test(consumer: &Path, target: &Path) {
    let output = Command::new(option_env!("CARGO").unwrap_or("cargo"))
        .current_dir(consumer)
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .env("CARGO_TARGET_DIR", target)
        .args(["test", "--lib", "--locked", "--offline"])
        .output()
        .expect("owner test");
    assert!(
        output.status.success(),
        "owner test failed\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn pure_data_renewal_and_rollback_restore_exact_tree() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/pure-data");
    let r1 = fixtures.join("r1/consumer");
    let r2 = fixtures.join("r2/consumer");
    let r1_snapshot = snapshot(&r1);
    let r2_snapshot = snapshot(&r2);
    assert_ne!(r1_snapshot, r2_snapshot);

    let temporary = TemporaryTree::new("renewal");
    let consumer = temporary.root.join("consumer");
    replace_tree(&r1, &consumer);
    assert_eq!(snapshot(&consumer), r1_snapshot);
    owner_test(&consumer, &temporary.root.join("target-r1-before"));

    replace_tree(&r2, &consumer);
    assert_eq!(snapshot(&consumer), r2_snapshot);
    owner_test(&consumer, &temporary.root.join("target-r2-renewed"));

    replace_tree(&r1, &consumer);
    assert_eq!(snapshot(&consumer), r1_snapshot);
    owner_test(&consumer, &temporary.root.join("target-r1-restored"));

    assert_eq!(snapshot(&r1), r1_snapshot);
    assert_eq!(snapshot(&r2), r2_snapshot);
}
