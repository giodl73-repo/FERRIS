use super::{
    CommandEnvelope, CoreError, ResultClass, classify_cargo_failure, digest_bytes,
    invocation_identity, record_id, selection_identity, success_envelope,
};
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const CARGO_FAILURE_REPORT_SCHEMA: &str = "ferris.cargo-failure-report/v1";
pub const MAX_CARGO_FAILURE_INPUT_BYTES: u64 = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CargoFailureKind {
    Dependency,
    Lockfile,
    OfflinePolicy,
    Unclassified,
}

impl CargoFailureKind {
    fn name(self) -> &'static str {
        match self {
            Self::Dependency => "dependency",
            Self::Lockfile => "lockfile",
            Self::OfflinePolicy => "offline_policy",
            Self::Unclassified => "unclassified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CargoFailureReport {
    pub schema: String,
    pub report_id: String,
    pub classification: CargoFailureKind,
    pub classified: bool,
    pub diagnostic_code: String,
    pub input_digest: String,
    pub input_bytes: u64,
    pub input_bound_bytes: u64,
    pub input_complete: bool,
    pub evidence_source: String,
    pub raw_output_retained: bool,
    pub executable: bool,
    pub next_actions: Vec<String>,
    pub unknowns: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Serialize)]
struct CargoFailureIdentity<'a> {
    schema: &'a str,
    classification: CargoFailureKind,
    classified: bool,
    diagnostic_code: &'a str,
    input_digest: &'a str,
    input_bytes: u64,
    input_bound_bytes: u64,
    input_complete: bool,
    evidence_source: &'a str,
    raw_output_retained: bool,
    executable: bool,
    next_actions: &'a [String],
    unknowns: &'a [String],
    limitations: &'a [String],
}

pub fn create_cargo_failure_report(
    stderr_path: &Path,
) -> Result<CommandEnvelope<CargoFailureReport>, CoreError> {
    let file = File::open(stderr_path).map_err(|_| {
        cargo_failure_input_error(
            "FERRIS-CARGO-DIAGNOSTIC-UNAVAILABLE",
            "The explicit Cargo stderr input is unavailable.",
        )
    })?;
    create_cargo_failure_report_from_reader(file)
}

pub fn create_cargo_failure_report_from_reader(
    reader: impl Read,
) -> Result<CommandEnvelope<CargoFailureReport>, CoreError> {
    let bytes = read_cargo_failure_input(reader)?;
    let stderr = std::str::from_utf8(&bytes).map_err(|_| {
        cargo_failure_input_error(
            "FERRIS-CARGO-DIAGNOSTIC-INVALID",
            "The explicit Cargo stderr input is not UTF-8.",
        )
    })?;
    let observed = classify_cargo_failure(stderr);
    let (classification, classified, diagnostic_code, next_action) = match observed.code {
        "FERRIS-CARGO-DEPENDENCY-BLOCKED" => (
            CargoFailureKind::Dependency,
            true,
            observed.code,
            observed.next_action,
        ),
        "FERRIS-CARGO-LOCK-BLOCKED" => (
            CargoFailureKind::Lockfile,
            true,
            observed.code,
            observed.next_action,
        ),
        "FERRIS-CARGO-OFFLINE-BLOCKED" => (
            CargoFailureKind::OfflinePolicy,
            true,
            observed.code,
            observed.next_action,
        ),
        _ => (
            CargoFailureKind::Unclassified,
            false,
            "FERRIS-CARGO-FAILURE-UNCLASSIFIED",
            "Inspect the owner Cargo diagnostic directly; Ferris did not identify a supported dependency failure shape.",
        ),
    };
    let input_digest = digest_bytes(&bytes);
    let next_actions = vec![next_action.to_owned()];
    let unknowns = vec![
        "Ferris did not determine a failing package, dependency identity, source path, or root cause."
            .to_owned(),
    ];
    let limitations = vec![
        "Classification is lexical over caller-supplied UTF-8 stderr; Ferris did not prove that Cargo produced the input."
            .to_owned(),
        "Ferris does not interpret rustc diagnostics, test output, causal chains, or command success."
            .to_owned(),
        "No owner command was executed, and raw input is not retained in this report.".to_owned(),
    ];
    let evidence_source = "caller-supplied-cargo-stderr";
    let identity = CargoFailureIdentity {
        schema: CARGO_FAILURE_REPORT_SCHEMA,
        classification,
        classified,
        diagnostic_code,
        input_digest: &input_digest,
        input_bytes: bytes.len() as u64,
        input_bound_bytes: MAX_CARGO_FAILURE_INPUT_BYTES,
        input_complete: true,
        evidence_source,
        raw_output_retained: false,
        executable: false,
        next_actions: &next_actions,
        unknowns: &unknowns,
        limitations: &limitations,
    };
    let report_id = record_id("cargo-failure-report", &identity)?;
    let report = CargoFailureReport {
        schema: CARGO_FAILURE_REPORT_SCHEMA.to_owned(),
        report_id: report_id.clone(),
        classification,
        classified,
        diagnostic_code: diagnostic_code.to_owned(),
        input_digest: input_digest.clone(),
        input_bytes: bytes.len() as u64,
        input_bound_bytes: MAX_CARGO_FAILURE_INPUT_BYTES,
        input_complete: true,
        evidence_source: evidence_source.to_owned(),
        raw_output_retained: false,
        executable: false,
        next_actions,
        unknowns,
        limitations,
    };
    Ok(success_envelope(
        "diagnose-cargo",
        selection_identity("diagnose-cargo", &input_digest),
        invocation_identity(&[
            "diagnose-cargo",
            &input_digest,
            "input=caller-supplied-cargo-stderr",
            "input-max-bytes=65536",
            "execution=false",
            "raw-output-retained=false",
        ]),
        report,
    ))
}

pub fn render_cargo_failure_human(envelope: &CommandEnvelope<CargoFailureReport>) -> String {
    let report = envelope
        .record
        .as_ref()
        .expect("successful Cargo failure diagnosis has a report");
    let mut output = format!(
        "Ferris Cargo failure report {}\nClassification: {}\nClassified: {}\nDiagnostic code: {}\nInput digest: {}\nInput bytes: {} of {} (complete={})\nEvidence source: {}\nRaw output retained: {}\nExecutable: no\nNext actions:\n",
        report.report_id,
        report.classification.name(),
        report.classified,
        report.diagnostic_code,
        report.input_digest,
        report.input_bytes,
        report.input_bound_bytes,
        report.input_complete,
        report.evidence_source,
        report.raw_output_retained,
    );
    for action in &report.next_actions {
        output.push_str(&format!("  - {action}\n"));
    }
    output.push_str("Unknowns:\n");
    for unknown in &report.unknowns {
        output.push_str(&format!("  - {unknown}\n"));
    }
    output.push_str("Limitations:\n");
    for limitation in &report.limitations {
        output.push_str(&format!("  - {limitation}\n"));
    }
    output
}

fn read_cargo_failure_input(reader: impl Read) -> Result<Vec<u8>, CoreError> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_CARGO_FAILURE_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            cargo_failure_input_error(
                "FERRIS-CARGO-DIAGNOSTIC-UNAVAILABLE",
                "The explicit Cargo stderr input could not be read.",
            )
        })?;
    if bytes.is_empty() {
        return Err(cargo_failure_input_error(
            "FERRIS-CARGO-DIAGNOSTIC-INVALID",
            "The explicit Cargo stderr input is empty.",
        ));
    }
    if bytes.len() as u64 > MAX_CARGO_FAILURE_INPUT_BYTES {
        return Err(cargo_failure_input_error(
            "FERRIS-CARGO-DIAGNOSTIC-INVALID",
            "The explicit Cargo stderr input exceeds the 64 KiB bound.",
        ));
    }
    Ok(bytes)
}

fn cargo_failure_input_error(code: &str, message: &str) -> CoreError {
    CoreError::new(
        ResultClass::Invalid,
        code,
        message,
        vec![
            "Provide one complete UTF-8 Cargo stderr input no larger than 64 KiB and retry."
                .to_owned(),
        ],
    )
}
