use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::{BTreeSet, HashSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const PLATFORM_PROFILE_SCHEMA: &str = "ferris.platform-profile/v1";
const CONTROL_SCHEMA: &str = "ferris.platform-profile-schema-controls/v1";
const MAX_PLATFORM_PROFILE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum ExpectedClass {
    Valid,
    Unsupported,
    Invalid,
    Blocked,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum MutationOperation {
    None,
    Replace,
    Add,
    RawDuplicate,
    Truncate,
    PadString,
}

#[derive(Debug, Deserialize)]
struct ControlManifest {
    schema: String,
    base: String,
    controls: Vec<Control>,
}

#[derive(Debug, Deserialize)]
struct Control {
    id: String,
    operation: MutationOperation,
    pointer: Option<String>,
    value: Option<Value>,
    byte_count: Option<usize>,
    minimum_canonical_bytes: Option<usize>,
    expected: ExpectedClass,
}

struct StrictJsonValue(Value);

impl StrictJsonValue {
    fn into_inner(self) -> Value {
        self.0
    }
}

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonValueVisitor)
    }
}

struct StrictJsonValueVisitor;

impl<'de> Visitor<'de> for StrictJsonValueVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value without duplicate object members")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::String(value.to_owned())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue(Value::Null))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictJsonValue>()? {
            values.push(value.0);
        }
        Ok(StrictJsonValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom("duplicate JSON object member"));
            }
            let value = map.next_value::<StrictJsonValue>()?;
            values.insert(key, value.0);
        }
        Ok(StrictJsonValue(Value::Object(values)))
    }
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/platform-profiles/schema")
}

fn parse_strict(bytes: &[u8]) -> Result<Value, serde_json::Error> {
    serde_json::from_slice::<StrictJsonValue>(bytes).map(StrictJsonValue::into_inner)
}

fn valid_identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    value.len() <= 256
        && first.is_ascii_alphanumeric()
        && bytes.all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b':' | b'/' | b'@' | b'+' | b'-')
        })
}

fn validate_source_references(value: &Value) -> bool {
    match value {
        Value::Array(values) => values.iter().all(validate_source_references),
        Value::Object(values) => {
            let is_source = ["kind", "identity", "owner", "revision", "observed_at"]
                .iter()
                .all(|field| values.contains_key(*field))
                && (values.contains_key("path") || values.contains_key("uri"));
            if is_source && (values.contains_key("path") == values.contains_key("uri")) {
                return false;
            }
            values.values().all(validate_source_references)
        }
        _ => true,
    }
}

fn validate_stage_states(value: &Value) -> bool {
    let allowed = [
        "pass",
        "fail",
        "expected-rejection",
        "unsupported",
        "unavailable",
        "not-observed",
        "stale",
        "conflicting",
        "revoked",
        "blocked",
        "unknown",
    ];
    value
        .get("stages")
        .and_then(Value::as_array)
        .is_some_and(|stages| {
            stages.iter().all(|stage| {
                stage
                    .get("state")
                    .and_then(Value::as_str)
                    .is_some_and(|state| allowed.contains(&state))
            })
        })
}

fn validate_profile(bytes: &[u8]) -> ExpectedClass {
    if bytes.len() > MAX_PLATFORM_PROFILE_BYTES {
        return ExpectedClass::Blocked;
    }

    let Ok(value) = parse_strict(bytes) else {
        return ExpectedClass::Invalid;
    };
    let Some(profile) = value.as_object() else {
        return ExpectedClass::Invalid;
    };

    match profile.get("schema").and_then(Value::as_str) {
        Some(PLATFORM_PROFILE_SCHEMA) => {}
        Some(schema) if schema.starts_with("ferris.platform-profile/") => {
            return ExpectedClass::Unsupported;
        }
        _ => return ExpectedClass::Invalid,
    }

    let required = [
        "schema",
        "profile_id",
        "revision",
        "family",
        "consumer",
        "operation",
        "owner",
        "status",
        "created_at",
        "expires_at",
        "requirements",
        "selection",
        "closures",
        "features",
        "contracts",
        "environment",
        "stages",
        "assurance",
        "stewardship",
        "support",
        "lifecycle",
        "limitations",
    ];
    if !required.iter().all(|field| profile.contains_key(*field)) {
        return ExpectedClass::Invalid;
    }

    let allowed = required
        .into_iter()
        .chain(["predecessor", "supersedes", "extensions"])
        .collect::<BTreeSet<_>>();
    if profile
        .keys()
        .any(|field| !allowed.contains(field.as_str()))
    {
        return ExpectedClass::Invalid;
    }

    if !["profile_id", "revision"].iter().all(|field| {
        profile
            .get(*field)
            .and_then(Value::as_str)
            .is_some_and(valid_identifier)
    }) {
        return ExpectedClass::Invalid;
    }

    if !validate_source_references(&value) || !validate_stage_states(&value) {
        return ExpectedClass::Invalid;
    }

    ExpectedClass::Valid
}

fn pointer_tokens(pointer: &str) -> Vec<String> {
    assert!(pointer.starts_with('/'), "JSON Pointer must be absolute");
    pointer[1..]
        .split('/')
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect()
}

fn set_pointer(root: &mut Value, pointer: &str, value: Value, replace: bool) {
    let tokens = pointer_tokens(pointer);
    let (last, parents) = tokens.split_last().expect("non-root JSON Pointer");
    let mut current = root;
    for token in parents {
        current = match current {
            Value::Object(values) => values.get_mut(token).expect("object pointer segment"),
            Value::Array(values) => {
                let index = token.parse::<usize>().expect("array pointer index");
                values.get_mut(index).expect("array pointer segment")
            }
            _ => panic!("pointer parent is not a container"),
        };
    }

    match current {
        Value::Object(values) => {
            if replace {
                assert!(values.contains_key(last), "replace target exists");
            } else {
                assert!(!values.contains_key(last), "add target is absent");
            }
            values.insert(last.clone(), value);
        }
        Value::Array(values) => {
            let index = last.parse::<usize>().expect("array pointer index");
            if replace {
                *values.get_mut(index).expect("replace array target") = value;
            } else {
                values.insert(index, value);
            }
        }
        _ => panic!("pointer target parent is not a container"),
    }
}

fn apply_control(base: &[u8], control: &Control) -> Vec<u8> {
    match control.operation {
        MutationOperation::None => base.to_vec(),
        MutationOperation::Replace | MutationOperation::Add => {
            let mut value = parse_strict(base).expect("valid base fixture");
            set_pointer(
                &mut value,
                control.pointer.as_deref().expect("mutation pointer"),
                control.value.clone().expect("mutation value"),
                control.operation == MutationOperation::Replace,
            );
            serde_json::to_vec_pretty(&value).expect("serialize mutated fixture")
        }
        MutationOperation::RawDuplicate => {
            assert_eq!(control.pointer.as_deref(), Some("/revision"));
            let text = std::str::from_utf8(base).expect("UTF-8 base fixture");
            text.replacen(
                "  \"revision\": \"r1\",",
                "  \"revision\": \"r1\",\n  \"revision\": \"duplicate\",",
                1,
            )
            .into_bytes()
        }
        MutationOperation::Truncate => {
            let count = control.byte_count.expect("truncate byte count");
            base[..base.len().checked_sub(count).expect("bounded truncation")].to_vec()
        }
        MutationOperation::PadString => {
            let minimum = control
                .minimum_canonical_bytes
                .expect("minimum padded byte count");
            let mut value = parse_strict(base).expect("valid base fixture");
            set_pointer(
                &mut value,
                control.pointer.as_deref().expect("padding pointer"),
                Value::String("x".repeat(minimum)),
                true,
            );
            serde_json::to_vec(&value).expect("serialize padded fixture")
        }
    }
}

#[test]
fn frozen_platform_profile_schema_controls_have_exact_classes() {
    let root = fixture_root();
    let manifest: ControlManifest = serde_json::from_slice(
        &fs::read(root.join("controls.json")).expect("read schema control manifest"),
    )
    .expect("parse schema control manifest");
    assert_eq!(manifest.schema, CONTROL_SCHEMA);
    assert_eq!(manifest.controls.len(), 9);

    let base = fs::read(root.join(&manifest.base)).expect("read base schema fixture");
    let mut ids = HashSet::new();
    for control in &manifest.controls {
        assert!(ids.insert(&control.id), "duplicate control id");
        let input = apply_control(&base, control);
        assert_eq!(validate_profile(&input), control.expected, "{}", control.id);
    }
}

#[test]
fn strict_json_rejects_nested_duplicate_members() {
    let input =
        br#"{"schema":"ferris.platform-profile/v1","nested":{"state":"pass","state":"unknown"}}"#;
    let error = parse_strict(input).expect_err("nested duplicate must fail");
    assert!(error.to_string().contains("duplicate JSON object member"));
}
