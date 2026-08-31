use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_ROOT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/validation-plan"
);
const RECORD_SCHEMA_ID: &str = "urn:ferris:schema:validation-plan:v0";
const COMMAND_SCHEMA_ID: &str = "urn:ferris:schema:command-result:v2:validation-plan";
const OWNER_DOMAINS_SCHEMA_ID: &str = "urn:ferris:schema:owner-validation-domains:v1";
const REVISION_BINDING_SCHEMA_ID: &str = "urn:ferris:schema:validation-revision-binding:v1";

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(path)
}

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-validation-schema-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create schema test directory");
        Self(path)
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

fn run_git(repository: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repository)
        .args(arguments)
        .output()
        .expect("run Git fixture command");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("Git fixture output")
        .trim()
        .to_owned()
}

fn revision_bound_output() -> (TestDirectory, Value) {
    let directory = TestDirectory::new("derived");
    copy_tree(&fixture("simple-workspace"), &directory.0);
    run_git(&directory.0, &["init", "--initial-branch", "main"]);
    run_git(
        &directory.0,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&directory.0, &["config", "user.name", "Ferris Test"]);
    run_git(&directory.0, &["add", "."]);
    run_git(&directory.0, &["commit", "-m", "base"]);
    let base = run_git(&directory.0, &["rev-parse", "HEAD"]);
    fs::write(
        directory.0.join("web/docs/package.json"),
        "{\"name\":\"web-docs\",\"revision\":2}\n",
    )
    .expect("change web-only fixture");
    run_git(&directory.0, &["add", "web/docs/package.json"]);
    run_git(&directory.0, &["commit", "-m", "change web input"]);
    let head = run_git(&directory.0, &["rev-parse", "HEAD"]);
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            directory
                .0
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--base-revision",
            &base,
            "--head-revision",
            &head,
            "--tested-revision",
            &head,
            "--owner-domains",
            directory
                .0
                .join("owner-domains.json")
                .to_str()
                .expect("owner domains path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run revision-bound validation plan");
    (directory, parse_machine_output(&output, 0))
}

fn schema(name: &str) -> Value {
    serde_json::from_slice(
        &fs::read(Path::new(SCHEMA_ROOT).join(name)).expect("read validation-plan schema"),
    )
    .expect("parse validation-plan schema JSON")
}

#[derive(Debug)]
enum SchemaError {
    Mismatch(String),
    Unsupported(String),
}

impl SchemaError {
    fn mismatch(message: impl Into<String>) -> Self {
        Self::Mismatch(message.into())
    }

    fn unsupported(message: impl Into<String>) -> Self {
        Self::Unsupported(message.into())
    }
}

impl fmt::Display for SchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mismatch(message) => write!(formatter, "schema mismatch: {message}"),
            Self::Unsupported(message) => write!(formatter, "unsupported schema: {message}"),
        }
    }
}

struct SchemaCatalog {
    roots: BTreeMap<String, Value>,
}

impl SchemaCatalog {
    fn load() -> Self {
        let mut roots = BTreeMap::new();
        for name in [
            "ferris.validation-plan.v0.schema.json",
            "ferris.command-result.v2.schema.json",
            "ferris.owner-validation-domains.v1.schema.json",
            "ferris.validation-revision-binding.v1.schema.json",
        ] {
            let root = schema(name);
            audit_schema(&root, name)
                .unwrap_or_else(|error| panic!("{name} uses an unsupported schema form: {error}"));
            let id = root["$id"].as_str().expect("schema root $id").to_owned();
            assert!(
                roots.insert(id.clone(), root).is_none(),
                "duplicate $id {id}"
            );
        }
        Self { roots }
    }

    fn root(&self, id: &str) -> &Value {
        self.roots
            .get(id)
            .unwrap_or_else(|| panic!("missing schema root {id}"))
    }

    fn validate(&self, id: &str, instance: &Value) -> Result<(), SchemaError> {
        let root = self
            .roots
            .get(id)
            .ok_or_else(|| SchemaError::unsupported(format!("unknown root $id {id}")))?;
        self.validate_schema(id, root, instance, "$")
    }

    fn validate_schema(
        &self,
        current_root_id: &str,
        schema: &Value,
        instance: &Value,
        path: &str,
    ) -> Result<(), SchemaError> {
        if let Some(allowed) = schema.as_bool() {
            return if allowed {
                Ok(())
            } else {
                Err(SchemaError::mismatch(format!(
                    "{path} is rejected by a false schema"
                )))
            };
        }
        let object = schema.as_object().ok_or_else(|| {
            SchemaError::unsupported(format!("{path} schema is neither an object nor a boolean"))
        })?;

        if let Some(reference) = object.get("$ref") {
            self.validate_reference(
                current_root_id,
                reference.as_str().expect("audited $ref"),
                instance,
                path,
            )?;
        }

        if let Some(expected_type) = object.get("type") {
            let expected_type = expected_type.as_str().expect("audited type");
            let matches = match expected_type {
                "object" => instance.is_object(),
                "array" => instance.is_array(),
                "string" => instance.is_string(),
                "boolean" => instance.is_boolean(),
                "integer" => instance.as_i64().is_some() || instance.as_u64().is_some(),
                _ => {
                    return Err(SchemaError::unsupported(format!(
                        "{path} type {expected_type}"
                    )));
                }
            };
            if !matches {
                return Err(SchemaError::mismatch(format!(
                    "{path} is not type {expected_type}"
                )));
            }
        }

        if let Some(expected) = object.get("const")
            && instance != expected
        {
            return Err(SchemaError::mismatch(format!(
                "{path} does not equal const {expected}"
            )));
        }

        if let Some(values) = object.get("enum") {
            let values = values.as_array().expect("audited enum");
            if !values.contains(instance) {
                return Err(SchemaError::mismatch(format!(
                    "{path} is not one of the published enum values"
                )));
            }
        }

        if let Some(string) = instance.as_str() {
            let length = string.chars().count();
            if let Some(minimum) = object.get("minLength").map(schema_usize)
                && length < minimum
            {
                return Err(SchemaError::mismatch(format!(
                    "{path} has length {length}, below {minimum}"
                )));
            }

            if let Some(maximum) = object.get("maxLength").map(schema_usize)
                && length > maximum
            {
                return Err(SchemaError::mismatch(format!(
                    "{path} has length {length}, above {maximum}"
                )));
            }
            if let Some(pattern) = object.get("pattern") {
                let pattern = pattern.as_str().expect("audited pattern");
                if !pattern_matches(string, pattern)? {
                    return Err(SchemaError::mismatch(format!(
                        "{path} does not match {pattern}"
                    )));
                }
            }
        }

        if let Some(integer) = instance.as_u64() {
            if let Some(minimum) = object.get("minimum").map(schema_u64)
                && integer < minimum
            {
                return Err(SchemaError::mismatch(format!(
                    "{path} is {integer}, below {minimum}"
                )));
            }
            if let Some(maximum) = object.get("maximum").map(schema_u64)
                && integer > maximum
            {
                return Err(SchemaError::mismatch(format!(
                    "{path} is {integer}, above {maximum}"
                )));
            }
        }

        if let Some(instance_object) = instance.as_object() {
            if let Some(required) = object.get("required") {
                for key in required.as_array().expect("audited required") {
                    let key = key.as_str().expect("audited required member");
                    if !instance_object.contains_key(key) {
                        return Err(SchemaError::mismatch(format!(
                            "{path} is missing required property {key}"
                        )));
                    }
                }
            }

            let properties = object.get("properties").and_then(Value::as_object);
            if let Some(properties) = properties {
                for (key, property_schema) in properties {
                    if let Some(value) = instance_object.get(key) {
                        self.validate_schema(
                            current_root_id,
                            property_schema,
                            value,
                            &child_path(path, key),
                        )?;
                    }
                }
            }

            if object.get("additionalProperties") == Some(&Value::Bool(false)) {
                for key in instance_object.keys() {
                    if !properties.is_some_and(|properties| properties.contains_key(key)) {
                        return Err(SchemaError::mismatch(format!(
                            "{path} contains unpublished property {key}"
                        )));
                    }
                }
            }
        }

        if let Some(array) = instance.as_array() {
            if let Some(minimum) = object.get("minItems").map(schema_usize)
                && array.len() < minimum
            {
                return Err(SchemaError::mismatch(format!(
                    "{path} has {} items, below {minimum}",
                    array.len()
                )));
            }
            if let Some(maximum) = object.get("maxItems").map(schema_usize)
                && array.len() > maximum
            {
                return Err(SchemaError::mismatch(format!(
                    "{path} has {} items, above {maximum}",
                    array.len()
                )));
            }
            if object.get("uniqueItems") == Some(&Value::Bool(true)) {
                for left in 0..array.len() {
                    for right in left + 1..array.len() {
                        if array[left] == array[right] {
                            return Err(SchemaError::mismatch(format!(
                                "{path} items {left} and {right} are not unique"
                            )));
                        }
                    }
                }
            }

            let prefix_items = object
                .get("prefixItems")
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or_default();
            for (index, item_schema) in prefix_items.iter().enumerate() {
                if let Some(item) = array.get(index) {
                    self.validate_schema(
                        current_root_id,
                        item_schema,
                        item,
                        &format!("{path}/{index}"),
                    )?;
                }
            }

            if let Some(items) = object.get("items") {
                for (index, item) in array.iter().enumerate().skip(prefix_items.len()) {
                    self.validate_schema(current_root_id, items, item, &format!("{path}/{index}"))?;
                }
            }
        }

        if let Some(all_of) = object.get("allOf") {
            for member in all_of.as_array().expect("audited allOf") {
                self.validate_schema(current_root_id, member, instance, path)?;
            }
        }

        if let Some(one_of) = object.get("oneOf") {
            let mut matches = 0;
            for member in one_of.as_array().expect("audited oneOf") {
                match self.validate_schema(current_root_id, member, instance, path) {
                    Ok(()) => matches += 1,
                    Err(SchemaError::Mismatch(_)) => {}
                    Err(error @ SchemaError::Unsupported(_)) => return Err(error),
                }
            }
            if matches != 1 {
                return Err(SchemaError::mismatch(format!(
                    "{path} matched {matches} oneOf branches"
                )));
            }
        }

        if let Some(condition) = object.get("if") {
            let condition_matches =
                match self.validate_schema(current_root_id, condition, instance, path) {
                    Ok(()) => true,
                    Err(SchemaError::Mismatch(_)) => false,
                    Err(error @ SchemaError::Unsupported(_)) => return Err(error),
                };
            let branch = if condition_matches {
                object.get("then")
            } else {
                object.get("else")
            };
            if let Some(branch) = branch {
                self.validate_schema(current_root_id, branch, instance, path)?;
            }
        }

        Ok(())
    }

    fn validate_reference(
        &self,
        current_root_id: &str,
        reference: &str,
        instance: &Value,
        path: &str,
    ) -> Result<(), SchemaError> {
        let (root_id, fragment) = if let Some(fragment) = reference.strip_prefix('#') {
            (current_root_id, fragment)
        } else if let Some((root_id, fragment)) = reference.split_once('#') {
            (root_id, fragment)
        } else {
            (reference, "")
        };
        let root = self
            .roots
            .get(root_id)
            .ok_or_else(|| SchemaError::unsupported(format!("unresolved $ref {reference}")))?;
        let referenced = if fragment.is_empty() {
            root
        } else {
            root.pointer(fragment)
                .ok_or_else(|| SchemaError::unsupported(format!("unresolved $ref {reference}")))?
        };
        self.validate_schema(root_id, referenced, instance, path)
    }
}

fn audit_schema(schema: &Value, path: &str) -> Result<(), SchemaError> {
    if schema.is_boolean() {
        return Ok(());
    }
    let object = schema.as_object().ok_or_else(|| {
        SchemaError::unsupported(format!("{path} schema is neither an object nor a boolean"))
    })?;
    for (keyword, value) in object {
        match keyword.as_str() {
            "$schema" | "$id" | "title" | "$ref" => {
                if !value.is_string() {
                    return Err(SchemaError::unsupported(format!(
                        "{path}/{keyword} must be a string"
                    )));
                }
            }
            "$defs" | "properties" => {
                let children = value.as_object().ok_or_else(|| {
                    SchemaError::unsupported(format!("{path}/{keyword} must be an object"))
                })?;
                for (name, child) in children {
                    audit_schema(child, &format!("{path}/{keyword}/{name}"))?;
                }
            }
            "type" => {
                let value = value.as_str().ok_or_else(|| {
                    SchemaError::unsupported(format!("{path}/type must be a string"))
                })?;
                if !matches!(value, "object" | "array" | "string" | "boolean" | "integer") {
                    return Err(SchemaError::unsupported(format!("{path}/type {value}")));
                }
            }
            "const" => {}
            "enum" => {
                if !value.is_array() {
                    return Err(SchemaError::unsupported(format!(
                        "{path}/enum must be an array"
                    )));
                }
            }
            "pattern" => {
                let pattern = value.as_str().ok_or_else(|| {
                    SchemaError::unsupported(format!("{path}/pattern must be a string"))
                })?;
                ensure_supported_pattern(pattern)?;
            }
            "minLength" | "maxLength" | "minItems" | "maxItems" | "minimum" | "maximum" => {
                if value.as_u64().is_none() {
                    return Err(SchemaError::unsupported(format!(
                        "{path}/{keyword} must be a non-negative integer"
                    )));
                }
            }
            "uniqueItems" | "additionalProperties" => {
                if !value.is_boolean() {
                    return Err(SchemaError::unsupported(format!(
                        "{path}/{keyword} must be boolean"
                    )));
                }
            }
            "required" => {
                let required = value.as_array().ok_or_else(|| {
                    SchemaError::unsupported(format!("{path}/required must be an array"))
                })?;
                if !required.iter().all(Value::is_string) {
                    return Err(SchemaError::unsupported(format!(
                        "{path}/required members must be strings"
                    )));
                }
            }
            "items" | "if" | "then" | "else" => {
                audit_schema(value, &format!("{path}/{keyword}"))?;
            }
            "prefixItems" | "allOf" | "oneOf" => {
                let members = value.as_array().ok_or_else(|| {
                    SchemaError::unsupported(format!("{path}/{keyword} must be an array"))
                })?;
                for (index, member) in members.iter().enumerate() {
                    audit_schema(member, &format!("{path}/{keyword}/{index}"))?;
                }
            }
            _ => {
                return Err(SchemaError::unsupported(format!(
                    "{path} keyword {keyword}"
                )));
            }
        }
    }
    Ok(())
}

fn schema_usize(value: &Value) -> usize {
    usize::try_from(value.as_u64().expect("audited non-negative integer"))
        .expect("schema bound fits usize")
}

fn schema_u64(value: &Value) -> u64 {
    value.as_u64().expect("audited non-negative integer")
}

fn ensure_supported_pattern(pattern: &str) -> Result<(), SchemaError> {
    if pattern == "^[A-Za-z0-9._:/-]+$"
        || pattern == "^[A-Za-z0-9._:-]+$"
        || pattern == "^cargo-workspace-package:.+@.+:.+$"
        || pattern == "^[0-9a-f]{40}$"
        || pattern == "^[0-9a-f]{64}$"
        || fixed_hex_prefix(pattern).is_some()
    {
        Ok(())
    } else {
        Err(SchemaError::unsupported(format!(
            "pattern {pattern} is outside the test validator subset"
        )))
    }
}

fn fixed_hex_prefix(pattern: &str) -> Option<&str> {
    pattern.strip_prefix('^')?.strip_suffix("[0-9a-f]{64}$")
}

fn pattern_matches(value: &str, pattern: &str) -> Result<bool, SchemaError> {
    ensure_supported_pattern(pattern)?;
    if pattern == "^[A-Za-z0-9._:/-]+$" {
        return Ok(!value.is_empty()
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'/' | b'-')
            }));
    }
    if pattern == "^[A-Za-z0-9._:-]+$" {
        return Ok(!value.is_empty()
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')
            }));
    }
    if pattern == "^cargo-workspace-package:.+@.+:.+$" {
        let Some(value) = value.strip_prefix("cargo-workspace-package:") else {
            return Ok(false);
        };
        let Some((name, version_and_path)) = value.split_once('@') else {
            return Ok(false);
        };
        let Some((version, manifest_path)) = version_and_path.split_once(':') else {
            return Ok(false);
        };
        return Ok(!name.is_empty() && !version.is_empty() && !manifest_path.is_empty());
    }
    if matches!(pattern, "^[0-9a-f]{40}$" | "^[0-9a-f]{64}$") {
        let expected = if pattern.contains("{40}") { 40 } else { 64 };
        return Ok(value.len() == expected
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    }
    let prefix = fixed_hex_prefix(pattern).expect("supported fixed-hex pattern");
    let Some(hex) = value.strip_prefix(prefix) else {
        return Ok(false);
    };
    Ok(hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
}

fn child_path(parent: &str, key: &str) -> String {
    format!("{parent}/{}", key.replace('~', "~0").replace('/', "~1"))
}

fn strict_object_schemas(value: &Value) -> bool {
    match value {
        Value::Object(object) => {
            if object.get("type") == Some(&Value::String("object".to_owned()))
                && object.get("additionalProperties") != Some(&Value::Bool(false))
            {
                return false;
            }
            object.values().all(strict_object_schemas)
        }
        Value::Array(array) => array.iter().all(strict_object_schemas),
        _ => true,
    }
}

fn pointer_tokens(pointer: &str) -> Vec<String> {
    pointer
        .split('/')
        .skip(1)
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect()
}

fn mutate(value: &mut Value, operation: &str, pointer: &str, replacement: Option<Value>) {
    let tokens = pointer_tokens(pointer);
    let (last, parents) = tokens.split_last().expect("non-root pointer");
    let mut current = value;
    for token in parents {
        current = if let Ok(index) = token.parse::<usize>() {
            &mut current.as_array_mut().expect("array parent")[index]
        } else {
            current
                .as_object_mut()
                .expect("object parent")
                .get_mut(token)
                .expect("existing object member")
        };
    }
    if let Ok(index) = last.parse::<usize>() {
        let array = current.as_array_mut().expect("array target");
        match operation {
            "remove" => {
                array.remove(index);
            }
            "add" => array.insert(index, replacement.expect("replacement value")),
            "replace" => array[index] = replacement.expect("replacement value"),
            _ => panic!("unknown mutation operation"),
        }
    } else {
        let object = current.as_object_mut().expect("object target");
        match operation {
            "remove" => {
                object.remove(last);
            }
            "add" | "replace" => {
                object.insert(last.clone(), replacement.expect("replacement value"));
            }
            _ => panic!("unknown mutation operation"),
        }
    }
}

fn parse_machine_output(output: &Output, expected_exit_code: i32) -> Value {
    assert_eq!(output.status.code(), Some(expected_exit_code));
    if expected_exit_code == 0 {
        assert!(output.stderr.is_empty(), "success stderr should be empty");
        serde_json::from_slice(&output.stdout).expect("parse success JSON")
    } else {
        assert!(output.stdout.is_empty(), "error stdout should be empty");
        serde_json::from_slice(&output.stderr).expect("parse error JSON")
    }
}

fn success_output() -> Value {
    parse_machine_output(
        &ferris()
            .args([
                "validation-plan",
                "--workspace-id",
                "ferris.test/simple",
                "--manifest-path",
                fixture("simple-workspace/Cargo.toml")
                    .to_str()
                    .expect("fixture path"),
                "--changed-path",
                fixture("simple-workspace/alpha/src/lib.rs")
                    .to_str()
                    .expect("fixture path"),
                "--changed-package",
                "fixture-alpha",
                "--format",
                "json",
            ])
            .output()
            .expect("run validation-plan success"),
        0,
    )
}

fn fallback_output() -> Value {
    parse_machine_output(
        &ferris()
            .args([
                "validation-plan",
                "--workspace-id",
                "ferris.test/simple",
                "--manifest-path",
                fixture("simple-workspace/Cargo.toml")
                    .to_str()
                    .expect("fixture path"),
                "--changed-path",
                fixture("simple-workspace/workspace-policy.txt")
                    .to_str()
                    .expect("fixture path"),
                "--format",
                "json",
            ])
            .output()
            .expect("run validation-plan fallback"),
        0,
    )
}

fn owner_domain_output() -> Value {
    parse_machine_output(
        &ferris()
            .args([
                "validation-plan",
                "--workspace-id",
                "ferris.test/simple",
                "--manifest-path",
                fixture("simple-workspace/Cargo.toml")
                    .to_str()
                    .expect("fixture path"),
                "--changed-path",
                fixture("simple-workspace/web/docs/package.json")
                    .to_str()
                    .expect("fixture path"),
                "--owner-domains",
                fixture("simple-workspace/owner-domains.json")
                    .to_str()
                    .expect("fixture path"),
                "--format",
                "json",
            ])
            .output()
            .expect("run validation-plan owner domain"),
        0,
    )
}

fn deleted_path_output() -> Value {
    parse_machine_output(
        &ferris()
            .args([
                "validation-plan",
                "--workspace-id",
                "ferris.test/simple",
                "--manifest-path",
                fixture("simple-workspace/Cargo.toml")
                    .to_str()
                    .expect("fixture path"),
                "--deleted-path",
                "web/docs/deleted.ts",
                "--owner-domains",
                fixture("simple-workspace/owner-domains.json")
                    .to_str()
                    .expect("fixture path"),
                "--format",
                "json",
            ])
            .output()
            .expect("run validation-plan deleted path"),
        0,
    )
}

fn package_identity_order(packages: &Value, nested: bool) -> Result<Vec<String>, String> {
    packages
        .as_array()
        .ok_or_else(|| "package collection is not an array".to_owned())?
        .iter()
        .map(|package| {
            let package = if nested { &package["package"] } else { package };
            package["identity"]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "package identity is not a string".to_owned())
        })
        .collect()
}

fn require_unique_identity_keys(identities: &[String], label: &str) -> Result<(), String> {
    let unique = identities.iter().collect::<BTreeSet<_>>();
    if unique.len() != identities.len() {
        return Err(format!("{label} identity keys are not unique"));
    }
    Ok(())
}

fn require_sorted_identity_keys(identities: &[String], label: &str) -> Result<(), String> {
    if !identities.windows(2).all(|window| window[0] < window[1]) {
        return Err(format!("{label} identity keys are not in serializer order"));
    }
    Ok(())
}

fn require_activity_identities(
    activities: &Value,
    expected: &[String],
    label: &str,
) -> Result<(), String> {
    for activity in activities
        .as_array()
        .ok_or_else(|| format!("{label} activities are not an array"))?
    {
        let actual = activity["package_identities"]
            .as_array()
            .ok_or_else(|| format!("{label} activity identities are not an array"))?
            .iter()
            .map(|identity| {
                identity
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| format!("{label} activity identity is not a string"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if actual != expected {
            return Err(format!(
                "{label} activity identities do not equal package order"
            ));
        }
    }
    Ok(())
}

fn semantic_conformance(value: &Value) -> Result<(), String> {
    let record = &value["record"];
    let selected = package_identity_order(&record["selected_packages"], true)?;
    require_unique_identity_keys(&selected, "selected package")?;
    require_sorted_identity_keys(&selected, "selected package")?;
    require_activity_identities(&record["selected_activities"], &selected, "selected")?;

    let fallback = package_identity_order(&record["fallback"]["packages"], false)?;
    require_unique_identity_keys(&fallback, "fallback package")?;
    require_sorted_identity_keys(&fallback, "fallback package")?;
    require_activity_identities(&record["fallback"]["activities"], &fallback, "fallback")?;

    let fallback_set = fallback.iter().map(String::as_str).collect::<BTreeSet<_>>();
    if !selected
        .iter()
        .all(|identity| fallback_set.contains(identity.as_str()))
    {
        return Err("selected package identities are not in fallback packages".to_owned());
    }
    for input in record["inputs"]
        .as_array()
        .ok_or_else(|| "inputs are not an array".to_owned())?
    {
        if let Some(identity) = input.get("package_identity").and_then(Value::as_str)
            && !fallback_set.contains(identity)
        {
            return Err("input package identity is not in fallback packages".to_owned());
        }
    }

    let selected_domains = record
        .get("selected_owner_domains")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let domain_ids = selected_domains
        .iter()
        .map(|domain| {
            domain["domain_id"]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "owner domain ID is not a string".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    require_unique_identity_keys(&domain_ids, "owner domain")?;
    require_sorted_identity_keys(&domain_ids, "owner domain")?;
    let selected_entrypoints = record
        .get("selected_owner_entrypoints")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(|entrypoint| {
            entrypoint
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "owner entrypoint ID is not a string".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    require_unique_identity_keys(&selected_entrypoints, "owner entrypoint")?;
    require_sorted_identity_keys(&selected_entrypoints, "owner entrypoint")?;
    let domain_set = domain_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let entrypoint_set = selected_entrypoints
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for input in record["inputs"]
        .as_array()
        .expect("schema-validated inputs")
    {
        if let Some(domain_id) = input.get("owner_domain_id").and_then(Value::as_str)
            && !domain_set.contains(domain_id)
        {
            return Err("input owner domain is not selected".to_owned());
        }
        if let Some(entrypoints) = input.get("owner_entrypoint_ids").and_then(Value::as_array) {
            for entrypoint in entrypoints {
                if !entrypoint
                    .as_str()
                    .is_some_and(|entrypoint| entrypoint_set.contains(entrypoint))
                {
                    return Err("input owner entrypoint is not selected".to_owned());
                }
            }
        }
    }
    if !selected_domains.is_empty() && record.get("owner_domain_contract").is_none() {
        return Err("selected owner domains require contract evidence".to_owned());
    }

    let required_by_inputs = record["inputs"]
        .as_array()
        .expect("schema-validated inputs")
        .iter()
        .any(|input| {
            matches!(
                input["disposition"].as_str(),
                Some("full_workspace_fallback" | "owner_domain_path_with_full_workspace_fallback")
            )
        });
    if record["fallback"]["required_by_inputs"] != required_by_inputs {
        return Err("fallback required_by_inputs does not derive from inputs".to_owned());
    }

    if record["evidence"]["workspace_id"] != record["workspace_id"] {
        return Err("evidence workspace_id does not equal record workspace_id".to_owned());
    }
    if record["evidence"]["command"][8] != record["selected_manifest"] {
        return Err("evidence manifest argument does not equal selected_manifest".to_owned());
    }
    if let Some(binding) = record.get("revision_binding") {
        let inputs = record["inputs"]
            .as_array()
            .ok_or_else(|| "revision-bound inputs are not an array".to_owned())?;
        if inputs.iter().any(|input| input["kind"] != "path") {
            return Err("revision-bound inputs must all be paths".to_owned());
        }
        let mut changes = inputs
            .iter()
            .map(|input| {
                let kind = if input.get("path_evidence").and_then(Value::as_str)
                    == Some("lexical_missing")
                {
                    "deleted"
                } else {
                    "changed"
                };
                let path = input["value"]
                    .as_str()
                    .ok_or_else(|| "revision-bound input path is not a string".to_owned())?;
                Ok((kind, path))
            })
            .collect::<Result<Vec<_>, String>>()?;
        changes.sort();
        let changed_count = changes
            .iter()
            .filter(|(kind, _)| *kind == "changed")
            .count() as u64;
        let deleted_count = changes
            .iter()
            .filter(|(kind, _)| *kind == "deleted")
            .count() as u64;
        if binding["changed_path_count"].as_u64() != Some(changed_count)
            || binding["deleted_path_count"].as_u64() != Some(deleted_count)
        {
            return Err("revision binding path counts do not derive from inputs".to_owned());
        }
        let expected_relationship = if binding["head_revision"] == binding["tested_revision"] {
            "tested_is_head"
        } else {
            "tested_contains_head"
        };
        if binding["relationship"] != expected_relationship {
            return Err("revision binding relationship contradicts resolved revisions".to_owned());
        }
        let mut hasher = Sha256::new();
        for (kind, path) in changes {
            hasher.update(kind.as_bytes());
            hasher.update([0]);
            hasher.update(path.as_bytes());
            hasher.update([0]);
        }
        let expected_change_set = format!(
            "change-set:{}",
            hasher
                .finalize()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        if binding["change_set_id"] != expected_change_set {
            return Err("revision binding change_set_id does not derive from inputs".to_owned());
        }
    }
    Ok(())
}

fn assert_schema_valid(catalog: &SchemaCatalog, value: &Value) {
    catalog
        .validate(COMMAND_SCHEMA_ID, value)
        .unwrap_or_else(|error| panic!("published schema rejected actual success output: {error}"));
}

#[test]
fn validation_plan_schema_documents_parse_and_use_supported_closed_subset() {
    let catalog = SchemaCatalog::load();
    let record_schema = catalog.root(RECORD_SCHEMA_ID);
    assert_eq!(
        record_schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        record_schema["properties"]["schema"]["const"],
        "ferris.validation-plan/v0"
    );
    assert_eq!(record_schema["properties"]["inputs"]["uniqueItems"], true);
    assert_eq!(
        record_schema["allOf"][1]["then"]["properties"]["inputs"]["maxItems"],
        4096
    );
    assert_eq!(
        record_schema["allOf"][1]["else"]["properties"]["inputs"]["maxItems"],
        256
    );
    assert_eq!(
        record_schema["properties"]["selected_packages"]["uniqueItems"],
        true
    );
    assert!(strict_object_schemas(record_schema));

    let command_schema = catalog.root(COMMAND_SCHEMA_ID);
    assert_eq!(
        command_schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        command_schema["properties"]["semantic_command_id"]["const"],
        "validation-plan"
    );
    assert_eq!(
        command_schema["properties"]["result_class"]["const"],
        "success"
    );
    assert_eq!(
        command_schema["properties"]["record"]["$ref"],
        RECORD_SCHEMA_ID
    );
    assert!(strict_object_schemas(command_schema));

    let owner_domains_schema = catalog.root(OWNER_DOMAINS_SCHEMA_ID);
    let owner_domains: Value = serde_json::from_slice(
        &fs::read(fixture("simple-workspace/owner-domains.json"))
            .expect("read owner domains fixture"),
    )
    .expect("parse owner domains fixture");
    catalog
        .validate(OWNER_DOMAINS_SCHEMA_ID, &owner_domains)
        .expect("owner domains fixture validates");
    assert!(strict_object_schemas(owner_domains_schema));

    let revision_binding_schema = catalog.root(REVISION_BINDING_SCHEMA_ID);
    assert_eq!(
        revision_binding_schema["properties"]["schema"]["const"],
        "ferris.validation-revision-binding/v1"
    );
    assert!(strict_object_schemas(revision_binding_schema));

    let unsupported = serde_json::json!({
        "type": "array",
        "contains": {"const": "unimplemented"}
    });
    assert!(matches!(
        audit_schema(&unsupported, "unsupported-control"),
        Err(SchemaError::Unsupported(message)) if message.contains("contains")
    ));
}

#[test]
fn real_cli_outputs_validate_published_schemas_and_semantic_invariants() {
    let catalog = SchemaCatalog::load();
    let success = success_output();
    assert_schema_valid(&catalog, &success);
    semantic_conformance(&success).expect("selected-package semantic conformance");
    assert_eq!(
        success["record"]["validation_plan_id"],
        "validation-plan:fb73ae3c12fcad53993e2b058038ed97519f00821764d195f80ab45976a3bc3e"
    );
    assert_eq!(
        success["record"]["selected_packages"]
            .as_array()
            .expect("selected packages")
            .len(),
        2
    );

    let fallback = fallback_output();
    assert_schema_valid(&catalog, &fallback);
    semantic_conformance(&fallback).expect("fallback semantic conformance");

    let owner_domain = owner_domain_output();
    assert_schema_valid(&catalog, &owner_domain);
    semantic_conformance(&owner_domain).expect("owner-domain semantic conformance");
    assert_eq!(
        owner_domain["record"]["selected_owner_entrypoints"][0],
        "web-docs-build"
    );

    let deleted_path = deleted_path_output();
    assert_schema_valid(&catalog, &deleted_path);
    semantic_conformance(&deleted_path).expect("deleted-path semantic conformance");
    assert_eq!(
        deleted_path["record"]["inputs"][0]["path_evidence"],
        "lexical_missing"
    );
    assert_eq!(
        fallback["record"]["selected_packages"]
            .as_array()
            .expect("selected packages")
            .len(),
        0
    );
    assert_eq!(fallback["record"]["fallback"]["required_by_inputs"], true);
    assert!(success["record"].get("revision_binding").is_none());

    let (_directory, revision_bound) = revision_bound_output();
    assert_schema_valid(&catalog, &revision_bound);
    semantic_conformance(&revision_bound).expect("revision-bound semantic conformance");
    assert_eq!(
        revision_bound["record"]["revision_binding"]["schema"],
        "ferris.validation-revision-binding/v1"
    );
    assert_eq!(
        revision_bound["record"]["selected_owner_entrypoints"][0],
        "web-docs-build"
    );
    assert_eq!(
        revision_bound["record"]["fallback"]["required_by_inputs"],
        false
    );
}

#[test]
fn published_schema_rejects_negative_structural_mutations() {
    let catalog = SchemaCatalog::load();
    let success = success_output();
    let selected_package = success["record"]["selected_packages"][0].clone();
    let selected_reason = success["record"]["selected_packages"][0]["reasons"][0].clone();
    let fallback_package = success["record"]["fallback"]["packages"][0].clone();
    let owner_domain = owner_domain_output();
    let selected_owner_domain = owner_domain["record"]["selected_owner_domains"][0].clone();
    let selected_owner_entrypoint = owner_domain["record"]["selected_owner_entrypoints"][0].clone();
    let (_revision_directory, revision_bound) = revision_bound_output();
    let revision_bound_for_input_limit = revision_bound.clone();

    let mutations = vec![
        (
            "extra-root-field",
            success.clone(),
            "add",
            "/unexpected",
            Some(Value::String("nope".to_owned())),
        ),
        (
            "missing-validation-plan-id",
            success.clone(),
            "remove",
            "/record/validation_plan_id",
            None,
        ),
        (
            "blocked-result-class-is-not-published",
            success.clone(),
            "replace",
            "/result_class",
            Some(Value::String("blocked".to_owned())),
        ),
        (
            "wrong-success-exit-code",
            success.clone(),
            "replace",
            "/process_exit_code",
            Some(Value::from(7)),
        ),
        (
            "diagnostics-must-stay-empty",
            success.clone(),
            "add",
            "/diagnostics/0",
            Some(serde_json::json!({"code": "not-published"})),
        ),
        (
            "record-must-stay-non-null",
            success.clone(),
            "replace",
            "/record",
            Some(Value::Null),
        ),
        (
            "extra-fallback-member",
            success.clone(),
            "add",
            "/record/fallback/extra",
            Some(Value::Bool(true)),
        ),
        (
            "invalid-input-disposition",
            success.clone(),
            "replace",
            "/record/inputs/0/disposition",
            Some(Value::String("package_scope".to_owned())),
        ),
        (
            "inputs-must-stay-non-empty",
            success.clone(),
            "replace",
            "/record/inputs",
            Some(Value::Array(Vec::new())),
        ),
        (
            "selected-package-records-must-be-unique",
            success.clone(),
            "add",
            "/record/selected_packages/1",
            Some(selected_package),
        ),
        (
            "selection-reasons-must-be-unique",
            success.clone(),
            "add",
            "/record/selected_packages/0/reasons/1",
            Some(selected_reason),
        ),
        (
            "fallback-package-records-must-be-unique",
            success.clone(),
            "add",
            "/record/fallback/packages/1",
            Some(fallback_package),
        ),
        (
            "selected-packages-require-two-activities",
            success.clone(),
            "replace",
            "/record/selected_activities",
            Some(Value::Array(Vec::new())),
        ),
        (
            "fallback-requires-two-activities",
            success.clone(),
            "replace",
            "/record/fallback/activities",
            Some(Value::Array(Vec::new())),
        ),
        (
            "unknowns-have-current-serializer-cardinality",
            success.clone(),
            "remove",
            "/record/unknowns/1",
            None,
        ),
        (
            "limitations-have-current-serializer-cardinality",
            success.clone(),
            "remove",
            "/record/limitations/2",
            None,
        ),
        (
            "owner-domain-input-requires-domain-id",
            owner_domain.clone(),
            "remove",
            "/record/inputs/0/owner_domain_id",
            None,
        ),
        (
            "owner-domain-selection-is-closed",
            owner_domain.clone(),
            "add",
            "/record/selected_owner_domains/0/extra",
            Some(Value::Bool(true)),
        ),
        (
            "owner-entrypoint-id-must-be-portable",
            owner_domain.clone(),
            "replace",
            "/record/selected_owner_entrypoints/0",
            Some(Value::String("web/docs-build".to_owned())),
        ),
        (
            "selected-owner-domains-must-be-structurally-unique",
            owner_domain.clone(),
            "add",
            "/record/selected_owner_domains/1",
            Some(selected_owner_domain),
        ),
        (
            "selected-owner-entrypoints-must-be-unique",
            owner_domain,
            "add",
            "/record/selected_owner_entrypoints/1",
            Some(selected_owner_entrypoint),
        ),
        (
            "revision-binding-is-closed",
            revision_bound.clone(),
            "add",
            "/record/revision_binding/extra",
            Some(Value::Bool(true)),
        ),
        (
            "revision-binding-requires-exact-commit",
            revision_bound.clone(),
            "replace",
            "/record/revision_binding/head_revision",
            Some(Value::String("HEAD".to_owned())),
        ),
        (
            "revision-binding-count-is-bounded",
            revision_bound,
            "replace",
            "/record/revision_binding/changed_path_count",
            Some(Value::from(4097)),
        ),
    ];

    for (label, mut value, operation, pointer, replacement) in mutations {
        mutate(&mut value, operation, pointer, replacement);
        assert!(
            catalog.validate(COMMAND_SCHEMA_ID, &value).is_err(),
            "structural mutation {label} was accepted by the published schema"
        );
    }

    let mut oversized_explicit = success;
    let explicit_template = oversized_explicit["record"]["inputs"][0].clone();
    let explicit_inputs = oversized_explicit["record"]["inputs"]
        .as_array_mut()
        .expect("explicit inputs");
    for index in explicit_inputs.len()..257 {
        let mut input = explicit_template.clone();
        input["value"] = Value::String(format!("package-{index}"));
        explicit_inputs.push(input);
    }
    assert!(
        catalog
            .validate(COMMAND_SCHEMA_ID, &oversized_explicit)
            .is_err(),
        "the published schema accepted more than 256 explicit inputs"
    );

    let mut derived_257 = revision_bound_for_input_limit;
    let derived_template = derived_257["record"]["inputs"][0].clone();
    let derived_inputs = derived_257["record"]["inputs"]
        .as_array_mut()
        .expect("derived inputs");
    for index in derived_inputs.len()..257 {
        let mut input = derived_template.clone();
        input["value"] = Value::String(format!("web/docs/generated-{index}.md"));
        derived_inputs.push(input);
    }
    assert!(
        catalog.validate(COMMAND_SCHEMA_ID, &derived_257).is_ok(),
        "the published schema rejected the revision-bound 257th input"
    );
}

#[test]
fn semantic_mutations_remain_outside_portable_schema() {
    let catalog = SchemaCatalog::load();

    let mut selected_activity_mismatch = success_output();
    selected_activity_mismatch["record"]["selected_activities"][0]["package_identities"]
        .as_array_mut()
        .expect("selected identities")
        .reverse();
    assert_schema_valid(&catalog, &selected_activity_mismatch);
    assert!(
        semantic_conformance(&selected_activity_mismatch)
            .expect_err("selected activity mismatch must fail semantics")
            .contains("selected activity identities")
    );

    let mut fallback_activity_mismatch = success_output();
    fallback_activity_mismatch["record"]["fallback"]["activities"][0]["package_identities"]
        .as_array_mut()
        .expect("fallback identities")
        .reverse();
    assert_schema_valid(&catalog, &fallback_activity_mismatch);
    assert!(
        semantic_conformance(&fallback_activity_mismatch)
            .expect_err("fallback activity mismatch must fail semantics")
            .contains("fallback activity identities")
    );

    let mut derived_flag_mismatch = fallback_output();
    derived_flag_mismatch["record"]["fallback"]["required_by_inputs"] = Value::Bool(false);
    assert_schema_valid(&catalog, &derived_flag_mismatch);
    assert!(
        semantic_conformance(&derived_flag_mismatch)
            .expect_err("derived fallback flag mismatch must fail semantics")
            .contains("required_by_inputs")
    );

    let mut duplicate_identity_key = success_output();
    let mut duplicate = duplicate_identity_key["record"]["selected_packages"][0].clone();
    duplicate["reasons"]
        .as_array_mut()
        .expect("selection reasons")
        .push(Value::String(
            "Structurally distinct duplicate identity control.".to_owned(),
        ));
    duplicate_identity_key["record"]["selected_packages"]
        .as_array_mut()
        .expect("selected packages")
        .push(duplicate);
    assert_schema_valid(&catalog, &duplicate_identity_key);
    assert!(
        semantic_conformance(&duplicate_identity_key)
            .expect_err("duplicate identity key must fail semantics")
            .contains("identity keys are not unique")
    );

    let mut missing_owner_contract = owner_domain_output();
    missing_owner_contract["record"]
        .as_object_mut()
        .expect("validation record")
        .remove("owner_domain_contract");
    assert_schema_valid(&catalog, &missing_owner_contract);
    assert!(
        semantic_conformance(&missing_owner_contract)
            .expect_err("selected owner domains without contract evidence must fail semantics")
            .contains("require contract evidence")
    );

    let mut unsorted_owner_entrypoints = owner_domain_output();
    unsorted_owner_entrypoints["record"]["selected_owner_entrypoints"] =
        serde_json::json!(["z-owner-entrypoint", "web-docs-build"]);
    assert_schema_valid(&catalog, &unsorted_owner_entrypoints);
    assert!(
        semantic_conformance(&unsorted_owner_entrypoints)
            .expect_err("unsorted owner entrypoints must fail semantics")
            .contains("owner entrypoint identity keys are not in serializer order")
    );

    let mut unselected_owner_reference = owner_domain_output();
    unselected_owner_reference["record"]["inputs"][0]["owner_domain_id"] =
        Value::String("unselected-domain".to_owned());
    assert_schema_valid(&catalog, &unselected_owner_reference);
    assert!(
        semantic_conformance(&unselected_owner_reference)
            .expect_err("unselected owner-domain input reference must fail semantics")
            .contains("input owner domain is not selected")
    );

    let mut duplicate_owner_domain_identity = owner_domain_output();
    let mut duplicate_domain =
        duplicate_owner_domain_identity["record"]["selected_owner_domains"][0].clone();
    duplicate_domain["reasons"]
        .as_array_mut()
        .expect("owner domain reasons")
        .push(Value::String(
            "Structurally distinct duplicate owner-domain identity control.".to_owned(),
        ));
    duplicate_owner_domain_identity["record"]["selected_owner_domains"]
        .as_array_mut()
        .expect("selected owner domains")
        .push(duplicate_domain);
    assert_schema_valid(&catalog, &duplicate_owner_domain_identity);
    assert!(
        semantic_conformance(&duplicate_owner_domain_identity)
            .expect_err("duplicate owner-domain identity must fail semantics")
            .contains("owner domain identity keys are not unique")
    );

    let (_revision_directory, revision_bound) = revision_bound_output();
    let mut relationship_mismatch = revision_bound.clone();
    relationship_mismatch["record"]["revision_binding"]["relationship"] =
        Value::String("tested_contains_head".to_owned());
    assert_schema_valid(&catalog, &relationship_mismatch);
    assert!(
        semantic_conformance(&relationship_mismatch)
            .expect_err("revision relationship mismatch must fail semantics")
            .contains("relationship contradicts")
    );

    let mut count_mismatch = revision_bound.clone();
    count_mismatch["record"]["revision_binding"]["changed_path_count"] = Value::from(2);
    assert_schema_valid(&catalog, &count_mismatch);
    assert!(
        semantic_conformance(&count_mismatch)
            .expect_err("revision count mismatch must fail semantics")
            .contains("path counts")
    );

    let mut change_set_mismatch = revision_bound;
    change_set_mismatch["record"]["revision_binding"]["change_set_id"] =
        Value::String(format!("change-set:{}", "0".repeat(64)));
    assert_schema_valid(&catalog, &change_set_mismatch);
    assert!(
        semantic_conformance(&change_set_mismatch)
            .expect_err("change-set mismatch must fail semantics")
            .contains("change_set_id")
    );
}
