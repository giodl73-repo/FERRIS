use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const EVALUATION_SCHEMA: &str = "ferris.asdf-tool-versions-evaluation/v1";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Evaluation {
    schema: String,
    upstream_revision: String,
    upstream_lf_sha256: String,
    cases: Vec<EvaluationCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvaluationCase {
    id: String,
    file: String,
    expected_rows: Vec<ToolVersions>,
    expected_reasons: Vec<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct ToolVersions {
    tool: String,
    versions: Vec<String>,
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/environment-readiness-adapters/asdf-tool-versions")
}

// Mirrors asdf's documented parser shape: '#' starts a comment and literal
// spaces delimit the tool name and ordered version tokens.
fn parse_tool_versions(content: &str) -> Vec<ToolVersions> {
    content
        .split('\n')
        .filter_map(|line| {
            let declaration = line.split_once('#').map_or(line, |(before, _)| before);
            let tokens = declaration
                .split(' ')
                .map(str::trim)
                .filter(|token| !token.is_empty())
                .collect::<Vec<_>>();
            (tokens.len() > 1).then(|| ToolVersions {
                tool: tokens[0].to_owned(),
                versions: tokens[1..]
                    .iter()
                    .map(|version| (*version).to_owned())
                    .collect(),
            })
        })
        .collect()
}

fn v1_semantic_losses(rows: &[ToolVersions]) -> Vec<String> {
    let mut reasons = BTreeSet::from(["plugin-executable-mapping-unavailable"]);
    for row in rows {
        if row.versions.len() > 1 {
            reasons.insert("ordered-fallback-unrepresentable");
        }
        for version in &row.versions {
            if version == "system" {
                reasons.insert("system-selection-requires-asdf-resolution");
            } else if version.starts_with("path:") {
                reasons.insert("path-selection-outside-readiness-v1");
            } else if version.starts_with("ref:") {
                reasons.insert("ref-selection-requires-asdf-plugin");
            } else {
                reasons.insert("version-selection-unrepresentable");
            }
        }
    }
    reasons.into_iter().map(str::to_owned).collect()
}

#[test]
fn frozen_asdf_cases_are_not_losslessly_translatable_to_readiness_v1() {
    let root = fixture_root();
    let evaluation: Evaluation =
        serde_json::from_slice(&fs::read(root.join("evaluation.json")).expect("read evaluation"))
            .expect("parse evaluation");

    assert_eq!(evaluation.schema, EVALUATION_SCHEMA);
    assert_eq!(
        evaluation.upstream_revision,
        "ca98e44ff49cb0a38966b23b42db962203478b59"
    );
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(
                fs::read_to_string(root.join("upstream-asdf.tool-versions"))
                    .expect("read exact upstream fixture")
                    .replace("\r\n", "\n")
                    .as_bytes()
            )
        ),
        evaluation.upstream_lf_sha256
    );
    assert_eq!(evaluation.cases.len(), 3);

    let mut ids = BTreeSet::new();
    for case in evaluation.cases {
        assert!(ids.insert(case.id.clone()), "duplicate case {}", case.id);
        let rows = parse_tool_versions(
            &fs::read_to_string(root.join(&case.file))
                .unwrap_or_else(|error| panic!("read {}: {error}", case.file)),
        );
        assert_eq!(rows, case.expected_rows, "{}", case.id);
        assert_eq!(
            v1_semantic_losses(&rows),
            case.expected_reasons,
            "{}",
            case.id
        );
    }
}

#[test]
fn upstream_fixture_exposes_plugin_and_executable_name_difference() {
    let rows = parse_tool_versions(
        &fs::read_to_string(fixture_root().join("upstream-asdf.tool-versions"))
            .expect("read upstream fixture"),
    );

    assert!(rows.iter().any(|row| row.tool == "golang"));
    assert!(!rows.iter().any(|row| row.tool == "go"));
}
