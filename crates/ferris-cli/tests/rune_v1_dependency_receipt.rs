use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "ferris.rune-v1-dependency-receipt/v1";
const REVISION: &str = "194449444624fb10add4137cb0da8d0327164fa7";

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn validation_root() -> PathBuf {
    repository_root().join("docs/plans/validation")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn receipt_identity(value: &Value) -> String {
    let mut payload = value.clone();
    payload["receipt_identity"] = Value::String(String::new());
    let mut bytes = format!("{DOMAIN}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).expect("serialize receipt"));
    sha256(&bytes)
}

fn exact_keys(value: &Value, expected: &[&str]) -> bool {
    value.as_object().is_some_and(|object| {
        object.keys().map(String::as_str).collect::<BTreeSet<_>>()
            == expected.iter().copied().collect::<BTreeSet<_>>()
    })
}

fn strings(value: &Value) -> Option<BTreeSet<String>> {
    value.as_array().and_then(|items| {
        items
            .iter()
            .map(|item| item.as_str().map(ToOwned::to_owned))
            .collect()
    })
}

fn expected(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn digest(value: &Value) -> bool {
    value.as_str().is_some_and(|text| {
        text.len() == 71
            && text.starts_with("sha256:")
            && text[7..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    })
}

fn validate(value: &Value) -> bool {
    const ROOT: [&str; 12] = [
        "schema",
        "receipt_identity",
        "decision_id",
        "recorded_on",
        "dependency",
        "rune_repository",
        "accepted_baseline",
        "cli_v1_surface",
        "validation",
        "fixture_binding",
        "change_control",
        "limitations",
    ];
    if !exact_keys(value, &ROOT)
        || value["schema"] != DOMAIN
        || value["decision_id"] != "FERRIS-P21-RUNE-V1-DEPENDENCY"
        || value["recorded_on"] != "2026-08-13"
        || !digest(&value["receipt_identity"])
        || receipt_identity(value) != value["receipt_identity"]
    {
        return false;
    }

    let dependency = &value["dependency"];
    if !exact_keys(
        dependency,
        &[
            "contract_id",
            "requirement",
            "status",
            "decision_scope",
            "platform_status_effect",
        ],
    ) || dependency["contract_id"] != "CONTRACT-001"
        || dependency["requirement"] != "Typebook/RUNE v1 contract baseline"
        || dependency["status"] != "satisfied"
        || dependency["decision_scope"] != "accepted contract/release-readiness baseline"
        || dependency["platform_status_effect"]
            != "RUNE dependency closed; PLATFORM-001 remains Draft solely for the valid Pulse 17 process-exit-agreement failure"
    {
        return false;
    }

    let repository = &value["rune_repository"];
    if !exact_keys(
        repository,
        &[
            "url",
            "revision",
            "cargo_workspace_version",
            "semver_1_0_0_published",
            "git_v1_0_0_tag_present",
        ],
    ) || repository["url"] != "https://github.com/giodl73-repo/RUNE.git"
        || repository["revision"] != REVISION
        || repository["cargo_workspace_version"] != "0.1.0"
        || repository["semver_1_0_0_published"] != false
        || repository["git_v1_0_0_tag_present"] != false
    {
        return false;
    }

    let baseline = &value["accepted_baseline"];
    if !exact_keys(
        baseline,
        &["name", "kind", "accepted_specification_rows", "evidence"],
    ) || baseline["name"] != "RUNE v1"
        || baseline["kind"] != "contract-release-readiness"
        || baseline["accepted_specification_rows"] != 8
    {
        return false;
    }
    let evidence = match baseline["evidence"].as_array() {
        Some(evidence) if evidence.len() == 3 => evidence,
        _ => return false,
    };
    if evidence
        .iter()
        .any(|item| !exact_keys(item, &["path", "url", "disposition"]))
    {
        return false;
    }
    let evidence_rows = evidence
        .iter()
        .filter_map(|item| {
            Some(format!(
                "{}|{}|{}",
                item["path"].as_str()?,
                item["url"].as_str()?,
                item["disposition"].as_str()?
            ))
        })
        .collect::<BTreeSet<_>>();
    if evidence_rows
        != expected(&[
            "context/waves/2026-06-02-v1-release-readiness/WAVE.md|https://github.com/giodl73-repo/RUNE/blob/194449444624fb10add4137cb0da8d0327164fa7/context/waves/2026-06-02-v1-release-readiness/WAVE.md|v1-release-readiness-closed",
            "docs/release-readiness.md|https://github.com/giodl73-repo/RUNE/blob/194449444624fb10add4137cb0da8d0327164fa7/docs/release-readiness.md|ready-as-publishable-contract-infrastructure",
            "docs/vtrace/SPECIFICATION_BASELINE.md|https://github.com/giodl73-repo/RUNE/blob/194449444624fb10add4137cb0da8d0327164fa7/docs/vtrace/SPECIFICATION_BASELINE.md|8-accepted-specification-rows",
        ])
    {
        return false;
    }

    let cli = &value["cli_v1_surface"];
    if !exact_keys(
        cli,
        &[
            "crates",
            "approved_stage",
            "contract_kinds",
            "approved_commands",
            "profiles",
            "adapters",
        ],
    ) || cli["approved_stage"] != "v1 implementation waves"
        || strings(&cli["crates"])
            != Some(expected(&[
                "rune-core",
                "rune-derive",
                "rune-cli",
                "rune-adapters",
            ]))
        || strings(&cli["contract_kinds"])
            != Some(expected(&[
                "entity", "event", "command", "state", "artifact", "source", "evidence", "other",
            ]))
        || strings(&cli["approved_commands"])
            != Some(expected(&[
                "status",
                "inspect --fixture <path>",
                "inspect-collection --fixture <path>",
                "inventory-collection --fixture <path>",
                "discover --manifest <path>",
                "evidence-collection --profile rune.neutral_descriptor_json (--fixture <path> | --manifest <path>)",
                "adapt-collection --adapter rune.review_packet_json --fixture <path>",
                "adapter list",
                "check --profile <profile-id> --fixture <path>",
                "check-collection --profile <profile-id> --fixture <path>",
                "check-registry --fixture <path>",
                "inspect-registry --fixture <path>",
                "check-state-graph --fixture <path> --registry <path>",
                "check-evidence-packet --fixture <path> --registry <path>",
                "check-agent-protocol --fixture <path> --registry <path>",
                "check-compatibility --fixture <path> --registry <path>",
                "generate --profile <profile-id> --fixture <path>",
                "generate-collection --profile <profile-id> --fixture <path>",
                "profile list",
            ]))
        || strings(&cli["profiles"])
            != Some(expected(&[
                "rune.neutral_descriptor_json",
                "rune.documentation_packet_json",
                "rune.data_contract_json",
            ]))
        || strings(&cli["adapters"]) != Some(expected(&["rune.review_packet_json"]))
    {
        return false;
    }

    let validation = &value["validation"];
    if !exact_keys(
        validation,
        &[
            "exact_revision",
            "all_required_passed",
            "commands",
            "registry_result",
            "compatibility_result",
        ],
    ) || validation["exact_revision"] != REVISION
        || validation["all_required_passed"] != true
    {
        return false;
    }
    let commands = match validation["commands"].as_array() {
        Some(commands) if commands.len() == 7 => commands,
        _ => return false,
    };
    if commands.iter().any(|command| {
        !exact_keys(command, &["id", "command", "disposition", "summary"])
            || command["disposition"] != "pass"
            || command["command"].as_str().is_none_or(str::is_empty)
            || command["summary"].as_str().is_none_or(str::is_empty)
    }) || commands
        .iter()
        .filter_map(|command| command["id"].as_str().map(ToOwned::to_owned))
        .collect::<BTreeSet<_>>()
        != expected(&[
            "fmt",
            "workspace-test",
            "status",
            "registry",
            "compatibility",
            "diff-check",
            "clean-status",
        ])
    {
        return false;
    }

    let registry = &validation["registry_result"];
    let capabilities = &registry["capabilities"];
    if !exact_keys(
        registry,
        &[
            "status",
            "collections",
            "profiles",
            "adapters",
            "capabilities",
        ],
    ) || registry["status"] != "ok"
        || registry["collections"] != 2
        || registry["profiles"] != 2
        || registry["adapters"] != 1
        || !exact_keys(
            capabilities,
            &["read", "query", "generate", "mutate", "runtime"],
        )
        || capabilities["read"] != true
        || capabilities["query"] != true
        || capabilities["generate"] != true
        || capabilities["mutate"] != false
        || capabilities["runtime"] != false
    {
        return false;
    }

    let compatibility = &validation["compatibility_result"];
    if !exact_keys(
        compatibility,
        &[
            "status",
            "compatibility",
            "supported",
            "unsupported",
            "degraded",
            "diagnostics",
        ],
    ) || compatibility["status"] != "ok"
        || compatibility["compatibility"] != "compatible"
        || compatibility["supported"] != 2
        || compatibility["unsupported"] != 0
        || compatibility["degraded"] != 0
        || compatibility["diagnostics"] != 0
    {
        return false;
    }

    let fixture = &value["fixture_binding"];
    if !exact_keys(
        fixture,
        &[
            "source_document",
            "controlled_fixture",
            "revision",
            "crate_version",
            "descriptor_collection_version",
            "neutral_profile_id",
            "neutral_profile_version",
            "fixture_revision_matches_receipt",
        ],
    ) || fixture["source_document"] != "docs/schemas/platform-profile/README.md"
        || fixture["controlled_fixture"]
            != "tests/fixtures/platform-profiles/schema/valid/pure-data-r1.json"
        || fixture["revision"] != REVISION
        || fixture["crate_version"] != "0.1.0"
        || fixture["descriptor_collection_version"] != "v0"
        || fixture["neutral_profile_id"] != "rune.neutral_descriptor_json"
        || fixture["neutral_profile_version"] != "v0"
        || fixture["fixture_revision_matches_receipt"] != true
    {
        return false;
    }

    let change = &value["change_control"];
    if !exact_keys(
        change,
        &[
            "semantic_fixture_regeneration_performed",
            "semantic_fixture_bytes_changed",
            "profile_identities_unchanged",
            "profile_digests_unchanged",
            "production_behavior_changed",
            "rune_repository_changed",
            "pulse_17_result_changed",
        ],
    ) || change["semantic_fixture_regeneration_performed"] != false
        || change["semantic_fixture_bytes_changed"] != false
        || change["profile_identities_unchanged"] != true
        || change["profile_digests_unchanged"] != true
        || change["production_behavior_changed"] != false
        || change["rune_repository_changed"] != false
        || change["pulse_17_result_changed"] != false
    {
        return false;
    }

    value["limitations"].as_array().is_some_and(|items| {
        items.len() == 7
            && items
                .iter()
                .all(|item| item.as_str().is_some_and(|text| !text.is_empty()))
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("not Cargo SemVer 1.0.0"))
            })
            && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|text| text.contains("valid Pulse 17 failure"))
            })
    })
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
            parent
                .as_object_mut()
                .expect("mutation object")
                .insert(key.to_owned(), mutation["value"].clone());
        }
        operation => panic!("unsupported mutation {operation}"),
    }
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

#[test]
fn rune_v1_dependency_receipt_is_closed_exact_and_mutation_resistant() {
    let schema =
        read_json(validation_root().join("ferris.rune-v1-dependency-receipt.v1.schema.json"));
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["properties"]["schema"]["const"], DOMAIN);
    assert_closed_object_schemas(&schema);

    let receipt = read_json(validation_root().join("PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT.json"));
    assert!(validate(&receipt));

    let schema_readme =
        fs::read_to_string(repository_root().join("docs/schemas/platform-profile/README.md"))
            .expect("read platform profile schema README");
    assert!(schema_readme.contains(&format!("| Revision | `{REVISION}` |")));
    assert!(schema_readme.contains("| Crate version | `0.1.0` |"));
    assert!(schema_readme.contains("| Descriptor collection | `v0` |"));
    assert!(schema_readme.contains("| Neutral profile version | `v0` |"));

    let fixture = read_json(
        repository_root().join("tests/fixtures/platform-profiles/schema/valid/pure-data-r1.json"),
    );
    let rune_source = fixture["contracts"]
        .as_array()
        .expect("contract list")
        .iter()
        .map(|contract| &contract["source"])
        .find(|source| source["identity"] == "rune.neutral_descriptor_json")
        .expect("RUNE fixture source");
    assert_eq!(rune_source["revision"], REVISION);
    assert_eq!(
        receipt["fixture_binding"]["revision"],
        rune_source["revision"]
    );

    let mutations =
        read_json(validation_root().join("PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT-MUTATIONS.json"));
    assert_eq!(
        mutations["schema"],
        "ferris.rune-v1-dependency-receipt-mutations/v1"
    );
    let mutations = mutations["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 13);
    for mutation in mutations {
        assert!(exact_keys(
            mutation,
            &["id", "operation", "pointer", "value", "recompute_identity",]
        ));
        let mut candidate = receipt.clone();
        apply_mutation(&mut candidate, mutation);
        if mutation["recompute_identity"] == true {
            candidate["receipt_identity"] = Value::String(receipt_identity(&candidate));
        }
        assert!(
            !validate(&candidate),
            "mutation unexpectedly accepted: {}",
            mutation["id"]
        );
    }
}
