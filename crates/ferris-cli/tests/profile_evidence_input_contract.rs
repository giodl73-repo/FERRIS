use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const PROFILE_SCHEMA: &str = "ferris.profile-evidence/v0";
const MAX_INPUT_BYTES: usize = 1_048_576;
const MAX_METADATA_BYTES: usize = 256;
const SCHEMA_SHA256: &str =
    "sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b";
const SECTION_NAMES: [&str; 12] = [
    "identity",
    "closure",
    "features",
    "toolchain",
    "targets",
    "providers",
    "native",
    "stages",
    "assurance",
    "stewardship",
    "support",
    "lifecycle",
];
const POSITIVE_FIXTURES: [&str; 6] = [
    "profile-evidence-v0-positive-scalars.json",
    "profile-evidence-v0-positive-arrays.json",
    "profile-evidence-v0-positive-objects.json",
    "profile-evidence-v0-positive-nested-mixed.json",
    "profile-evidence-v0-positive-boundary-minimum.json",
    "profile-evidence-v0-positive-boundary-maximum.json",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn held_out_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out")
}

fn fixture_root() -> PathBuf {
    held_out_root().join("fixtures")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR bytes");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end with LF");
    bytes
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn string_set(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string item"))
        .collect()
}

fn object_key_set(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Accepted,
    Rejected {
        result_class: String,
        diagnostic: String,
    },
}

fn rejected(result_class: &str, diagnostic: &str) -> Outcome {
    Outcome::Rejected {
        result_class: result_class.to_owned(),
        diagnostic: diagnostic.to_owned(),
    }
}

enum InputSource {
    Bytes(Vec<u8>),
    Missing,
    NonFile,
    Unreadable,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileEvidence {
    schema: String,
    profile_id: String,
    revision: String,
    consumer: String,
    sections: ProfileSections,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileSections {
    identity: Value,
    closure: Value,
    features: Value,
    toolchain: Value,
    targets: Value,
    providers: Value,
    native: Value,
    stages: Value,
    assurance: Value,
    stewardship: Value,
    support: Value,
    lifecycle: Value,
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
        formatter.write_str("a JSON value with unique visible-ASCII object member names")
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
            if !valid_visible_ascii(&key) {
                return Err(de::Error::custom(
                    "invalid output-visible JSON object member",
                ));
            }
            if values.contains_key(&key) {
                return Err(de::Error::custom("duplicate JSON object member"));
            }
            let value = map.next_value::<StrictJsonValue>()?;
            values.insert(key, value.0);
        }
        Ok(StrictJsonValue(Value::Object(values)))
    }
}

fn valid_visible_ascii(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_METADATA_BYTES
        && value.bytes().all(|byte| matches!(byte, b'!'..=b'~'))
}

fn validate(source: InputSource) -> Outcome {
    let bytes = match source {
        InputSource::Missing | InputSource::Unreadable => {
            return rejected("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE");
        }
        InputSource::NonFile => {
            return rejected("incomplete", "FERRIS-PROFILE-INPUT-NOT-FILE");
        }
        InputSource::Bytes(bytes) => bytes,
    };

    if bytes.len() > MAX_INPUT_BYTES {
        return rejected("incomplete", "FERRIS-PROFILE-INPUT-OVERSIZED");
    }

    let value = match serde_json::from_slice::<StrictJsonValue>(&bytes) {
        Ok(value) => value.into_inner(),
        Err(error) if error.to_string().contains("duplicate JSON object member") => {
            return rejected("invalid", "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER");
        }
        Err(error)
            if error
                .to_string()
                .contains("invalid output-visible JSON object member") =>
        {
            return rejected("invalid", "FERRIS-PROFILE-METADATA-INVALID");
        }
        Err(_) => {
            return rejected("invalid", "FERRIS-PROFILE-JSON-INVALID");
        }
    };

    if value
        .get("schema")
        .and_then(Value::as_str)
        .is_some_and(|schema| schema != PROFILE_SCHEMA)
    {
        return rejected("unsupported", "FERRIS-PROFILE-SCHEMA-UNSUPPORTED");
    }

    let evidence: ProfileEvidence = match serde_json::from_value(value) {
        Ok(evidence) => evidence,
        Err(_) => {
            return rejected("invalid", "FERRIS-PROFILE-SHAPE-INVALID");
        }
    };
    if evidence.schema != PROFILE_SCHEMA {
        return rejected("unsupported", "FERRIS-PROFILE-SCHEMA-UNSUPPORTED");
    }
    if [
        evidence.profile_id.as_str(),
        evidence.revision.as_str(),
        evidence.consumer.as_str(),
    ]
    .iter()
    .any(|value| !valid_visible_ascii(value))
    {
        return rejected("invalid", "FERRIS-PROFILE-IDENTITY-INVALID");
    }

    Outcome::Accepted
}

fn set_pointer(value: &mut Value, pointer: &str, replacement: Value) {
    let (parent, key) = pointer.rsplit_once('/').expect("pointer parent");
    let parent = if parent.is_empty() {
        value
    } else {
        value.pointer_mut(parent).expect("pointer parent value")
    };
    parent
        .as_object_mut()
        .expect("pointer parent object")
        .insert(key.to_owned(), replacement);
}

fn remove_pointer(value: &mut Value, pointer: &str) {
    let (parent, key) = pointer.rsplit_once('/').expect("pointer parent");
    let parent = if parent.is_empty() {
        value
    } else {
        value.pointer_mut(parent).expect("pointer parent value")
    };
    parent
        .as_object_mut()
        .expect("pointer parent object")
        .remove(key)
        .expect("removed value");
}

fn insert_member(value: &mut Value, pointer: &str, key: String, member: Value) {
    let object = if pointer.is_empty() {
        value
    } else {
        value.pointer_mut(pointer).expect("member pointer")
    };
    object
        .as_object_mut()
        .expect("member object")
        .insert(key, member);
}

fn repeated(mutation: &Value) -> String {
    let character = mutation["character"].as_str().expect("repeat character");
    assert_eq!(character.chars().count(), 1);
    character.repeat(mutation["count"].as_u64().expect("repeat count") as usize)
}

fn apply_mutation(base_bytes: &[u8], base_value: &Value, mutation: &Value) -> InputSource {
    match mutation["operation"].as_str().expect("mutation operation") {
        "replace" | "add" => {
            let mut value = base_value.clone();
            set_pointer(
                &mut value,
                mutation["pointer"].as_str().expect("mutation pointer"),
                mutation["value"].clone(),
            );
            InputSource::Bytes(serde_json::to_vec_pretty(&value).expect("serialize mutation"))
        }
        "remove" => {
            let mut value = base_value.clone();
            remove_pointer(
                &mut value,
                mutation["pointer"].as_str().expect("mutation pointer"),
            );
            InputSource::Bytes(serde_json::to_vec_pretty(&value).expect("serialize mutation"))
        }
        "replace-repeat" => {
            let mut value = base_value.clone();
            set_pointer(
                &mut value,
                mutation["pointer"].as_str().expect("mutation pointer"),
                Value::String(repeated(mutation)),
            );
            InputSource::Bytes(serde_json::to_vec_pretty(&value).expect("serialize mutation"))
        }
        "insert-member" => {
            let mut value = base_value.clone();
            insert_member(
                &mut value,
                mutation["pointer"].as_str().expect("mutation pointer"),
                mutation["key"].as_str().expect("mutation key").to_owned(),
                mutation["value"].clone(),
            );
            InputSource::Bytes(serde_json::to_vec_pretty(&value).expect("serialize mutation"))
        }
        "insert-repeated-member" => {
            let mut value = base_value.clone();
            insert_member(
                &mut value,
                mutation["pointer"].as_str().expect("mutation pointer"),
                repeated(mutation),
                mutation["value"].clone(),
            );
            InputSource::Bytes(serde_json::to_vec_pretty(&value).expect("serialize mutation"))
        }
        "raw-replace" => {
            let text = std::str::from_utf8(base_bytes).expect("UTF-8 base fixture");
            let needle = mutation["needle"].as_str().expect("raw needle");
            assert_eq!(text.matches(needle).count(), 1, "unique raw needle");
            InputSource::Bytes(
                text.replacen(
                    needle,
                    mutation["replacement"].as_str().expect("raw replacement"),
                    1,
                )
                .into_bytes(),
            )
        }
        "raw-content" => InputSource::Bytes(
            mutation["content"]
                .as_str()
                .expect("raw content")
                .as_bytes()
                .to_vec(),
        ),
        "pad-to-size" => {
            let size = mutation["size"].as_u64().expect("pad size") as usize;
            assert!(size >= base_bytes.len());
            let mut bytes = base_bytes.to_vec();
            bytes.resize(size, b' ');
            InputSource::Bytes(bytes)
        }
        "source-state" => match mutation["state"].as_str().expect("source state") {
            "missing" => InputSource::Missing,
            "non_file" => InputSource::NonFile,
            "unreadable" => InputSource::Unreadable,
            state => panic!("unsupported source state {state}"),
        },
        operation => panic!("unsupported mutation operation {operation}"),
    }
}

#[test]
fn pulse_31_schema_is_closed_recursive_and_exact() {
    let schema_path = held_out_root()
        .join("schemas")
        .join("ferris.profile-evidence.v0.schema.json");
    let bytes = read_lf(schema_path);
    assert_eq!(sha256(&bytes), SCHEMA_SHA256);
    let schema: Value = serde_json::from_slice(&bytes).expect("parse schema");

    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["$id"], "urn:ferris:schema:profile-evidence:v0");
    assert_eq!(schema["type"], "object");
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        string_set(&schema["required"]),
        ["schema", "profile_id", "revision", "consumer", "sections"]
            .into_iter()
            .collect()
    );
    assert_eq!(
        object_key_set(&schema["properties"]),
        ["schema", "profile_id", "revision", "consumer", "sections"]
            .into_iter()
            .collect()
    );
    assert_eq!(schema["properties"]["schema"]["const"], PROFILE_SCHEMA);

    for definition in ["visibleAsciiMetadata", "visibleAsciiName"] {
        let value = &schema["$defs"][definition];
        assert_eq!(value["type"], "string");
        assert_eq!(value["minLength"], 1);
        assert_eq!(value["maxLength"], 256);
        assert_eq!(value["pattern"], "^[!-~]{1,256}$");
    }

    let sections = &schema["properties"]["sections"];
    assert_eq!(sections["type"], "object");
    assert_eq!(sections["additionalProperties"], false);
    assert_eq!(
        string_set(&sections["required"]),
        SECTION_NAMES.into_iter().collect()
    );
    assert_eq!(
        object_key_set(&sections["properties"]),
        SECTION_NAMES.into_iter().collect()
    );
    for name in SECTION_NAMES {
        assert_eq!(sections["properties"][name]["$ref"], "#/$defs/jsonValue");
    }

    let variants = schema["$defs"]["jsonValue"]["oneOf"]
        .as_array()
        .expect("JSON value variants");
    assert_eq!(variants.len(), 6);
    assert_eq!(
        variants
            .iter()
            .map(|variant| variant["type"].as_str().expect("variant type"))
            .collect::<BTreeSet<_>>(),
        ["null", "boolean", "number", "string", "array", "object"]
            .into_iter()
            .collect()
    );
    let array = variants
        .iter()
        .find(|variant| variant["type"] == "array")
        .expect("array variant");
    assert_eq!(array["items"]["$ref"], "#/$defs/jsonValue");
    let object = variants
        .iter()
        .find(|variant| variant["type"] == "object")
        .expect("object variant");
    assert_eq!(object["propertyNames"]["$ref"], "#/$defs/visibleAsciiName");
    assert_eq!(object["additionalProperties"]["$ref"], "#/$defs/jsonValue");
}

#[test]
fn pulse_31_positive_fixtures_cover_values_and_inclusive_boundaries() {
    for name in POSITIVE_FIXTURES {
        let bytes = read_lf(fixture_root().join(name));
        assert_eq!(
            validate(InputSource::Bytes(bytes)),
            Outcome::Accepted,
            "{name}"
        );
    }

    let minimum: Value = serde_json::from_slice(&read_lf(
        fixture_root().join("profile-evidence-v0-positive-boundary-minimum.json"),
    ))
    .expect("minimum fixture");
    for field in ["profile_id", "revision", "consumer"] {
        assert_eq!(minimum[field].as_str().expect("minimum metadata").len(), 1);
    }
    assert_eq!(
        minimum["sections"]["identity"]
            .as_object()
            .expect("minimum object")
            .keys()
            .next()
            .expect("minimum key")
            .len(),
        1
    );

    let maximum: Value = serde_json::from_slice(&read_lf(
        fixture_root().join("profile-evidence-v0-positive-boundary-maximum.json"),
    ))
    .expect("maximum fixture");
    for field in ["profile_id", "revision", "consumer"] {
        assert_eq!(
            maximum[field].as_str().expect("maximum metadata").len(),
            256
        );
    }
    assert_eq!(
        maximum["sections"]["identity"]
            .as_object()
            .expect("maximum object")
            .keys()
            .next()
            .expect("maximum key")
            .len(),
        256
    );

    let mut exact_limit =
        read_lf(fixture_root().join("profile-evidence-v0-positive-nested-mixed.json"));
    exact_limit.resize(MAX_INPUT_BYTES, b' ');
    assert_eq!(
        validate(InputSource::Bytes(exact_limit.clone())),
        Outcome::Accepted
    );
    exact_limit.push(b' ');
    assert_eq!(
        validate(InputSource::Bytes(exact_limit)),
        rejected("incomplete", "FERRIS-PROFILE-INPUT-OVERSIZED")
    );
}

#[test]
fn pulse_31_all_33_declared_invalid_controls_match_normative_classification() {
    let mutations: Value = serde_json::from_slice(&read_lf(
        fixture_root().join("profile-evidence-v0-mutations.json"),
    ))
    .expect("mutation controls");
    assert_eq!(
        mutations["schema"],
        "ferris.profile-evidence-v0-mutations/v1"
    );
    let base_name = mutations["base_fixture"].as_str().expect("base fixture");
    let base_bytes = read_lf(fixture_root().join(base_name));
    let base_value: Value = serde_json::from_slice(&base_bytes).expect("base JSON");
    assert_eq!(
        validate(InputSource::Bytes(base_bytes.clone())),
        Outcome::Accepted
    );

    let controls = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(controls.len(), 33);
    let mut ids = BTreeSet::new();
    for mutation in controls {
        let id = mutation["id"].as_str().expect("mutation id");
        assert!(ids.insert(id), "duplicate mutation id {id}");
        let expected = &mutation["expected"];
        assert_eq!(
            validate(apply_mutation(&base_bytes, &base_value, mutation)),
            rejected(
                expected["result_class"]
                    .as_str()
                    .expect("expected result class"),
                expected["diagnostic"]
                    .as_str()
                    .expect("expected diagnostic"),
            ),
            "{id}"
        );
    }
}

#[test]
fn pulse_31_contract_indexes_and_review_publish_the_complete_boundary() {
    let contract = fs::read_to_string(held_out_root().join("INPUT_PROFILE_EVIDENCE.md"))
        .expect("input contract");
    for required in [
        PROFILE_SCHEMA,
        SCHEMA_SHA256,
        "1,048,576 bytes",
        "Exactly 1,048,576 bytes is permitted",
        "JSON Schema validates a parsed JSON value",
        "FERRIS-PROFILE-INPUT-UNAVAILABLE",
        "FERRIS-PROFILE-INPUT-NOT-FILE",
        "FERRIS-PROFILE-INPUT-OVERSIZED",
        "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER",
        "FERRIS-PROFILE-METADATA-INVALID",
        "FERRIS-PROFILE-JSON-INVALID",
        "FERRIS-PROFILE-SCHEMA-UNSUPPORTED",
        "FERRIS-PROFILE-SHAPE-INVALID",
        "FERRIS-PROFILE-IDENTITY-INVALID",
        "six positive fixtures",
        "33 negative",
    ] {
        assert!(
            contract.contains(required),
            "missing contract term {required}"
        );
    }

    let schema_index =
        fs::read_to_string(held_out_root().join("schemas/README.md")).expect("schema index");
    assert!(schema_index.contains("ferris.profile-evidence.v0.schema.json"));
    assert!(schema_index.contains("All 18 schemas"));

    let fixture_index =
        fs::read_to_string(held_out_root().join("fixtures/README.md")).expect("fixture index");
    for name in POSITIVE_FIXTURES {
        assert!(fixture_index.contains(name), "missing fixture index {name}");
    }
    assert!(fixture_index.contains("profile-evidence-v0-mutations.json"));
    assert!(fixture_index.contains("961 total declared mutations"));

    for relative in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-31.md",
        "docs/plans/reviews/PULSE-31-PROFILE-EVIDENCE-INPUT-CONTRACT-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 31 document");
        assert!(text.contains(SCHEMA_SHA256), "{relative} schema digest");
        assert!(text.contains("33"), "{relative} mutation count");
        assert!(text.contains("six"), "{relative} fixture count");
        assert!(
            text.contains("no diagnostic"),
            "{relative} authority boundary"
        );
    }
}
