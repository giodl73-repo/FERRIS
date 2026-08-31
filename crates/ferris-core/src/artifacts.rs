use super::{CoreError, ResultClass, StrictJsonValue, digest_bytes, valid_portable_id};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::Path;

pub const ARTIFACT_REUSE_REQUEST_SCHEMA: &str = "ferris.artifact-reuse-request/v1";
pub const ARTIFACT_REUSE_REPORT_SCHEMA: &str = "ferris.artifact-reuse-report/v1";
pub const ARTIFACT_QUALIFICATION_REPORT_SCHEMA: &str = "ferris.artifact-qualification-report/v1";

const MAX_ARTIFACT_REQUEST_BYTES: u64 = 1024 * 1024;
const MAX_MEASURED_ARTIFACT_BYTES: u64 = 16 * 1024 * 1024 * 1024;
const MAX_MEASURED_MANIFEST_BYTES: u64 = 16 * 1024 * 1024;
const MAX_ARTIFACT_CONSUMERS: usize = 1024;
const MAX_ARTIFACT_FEATURES: usize = 256;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactProducerTerminalStatus {
    Succeeded,
    Failed,
    Cancelled,
    Unavailable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityEnvelope {
    pub repository_id: String,
    pub source_revision: String,
    pub toolchain_identity: String,
    pub platform_os: String,
    pub platform_architecture: String,
    pub target: String,
    pub profile: String,
    pub features: Vec<String>,
    pub configuration_identity: String,
    pub manifest_identity: String,
    pub command_identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactProducer {
    pub producer_node_id: String,
    pub attempt_id: String,
    pub terminal_status: ArtifactProducerTerminalStatus,
    pub artifact_id: String,
    pub artifact_digest: String,
    pub compatibility: CompatibilityEnvelope,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactConsumer {
    pub consumer_node_id: String,
    pub required: bool,
    pub expected_artifact_id: String,
    pub expected_artifact_digest: String,
    pub required_compatibility: CompatibilityEnvelope,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactReuseRequest {
    pub schema: String,
    pub producer: ArtifactProducer,
    pub consumers: Vec<ArtifactConsumer>,
    pub expected_consumer_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactConsumerClassification {
    Compatible,
    ProducerFailed,
    ProducerCancelled,
    ProducerUnavailable,
    ArtifactIdentityMismatch,
    CompatibilityMismatch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityDimension {
    Repository,
    SourceRevision,
    Toolchain,
    PlatformOs,
    PlatformArchitecture,
    Target,
    Profile,
    Features,
    Configuration,
    Manifest,
    Command,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactConsumerResult {
    pub consumer_node_id: String,
    pub required: bool,
    pub expected_artifact_id: String,
    pub expected_artifact_digest: String,
    pub classification: ArtifactConsumerClassification,
    pub compatibility_mismatches: Vec<CompatibilityDimension>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FanInReport {
    pub expected_consumer_ids: Vec<String>,
    pub missing_consumer_ids: Vec<String>,
    pub incompatible_consumer_ids: Vec<String>,
    pub succeeded: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReuseReportStatus {
    ObservationOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactReuseReport {
    pub schema: String,
    pub report_id: String,
    pub status: ArtifactReuseReportStatus,
    pub producer: ArtifactProducer,
    pub consumers: Vec<ArtifactConsumerResult>,
    pub fan_in: FanInReport,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactMeasurement {
    pub artifact_digest: String,
    pub artifact_size_bytes: u64,
    pub manifest_identity: String,
    pub manifest_size_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactProducerBinding {
    pub artifact_digest_matches: bool,
    pub manifest_identity_matches: bool,
    pub succeeded: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactQualificationStatus {
    Qualified,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactQualificationReport {
    pub schema: String,
    pub qualification_id: String,
    pub status: ArtifactQualificationStatus,
    pub measurement: ArtifactMeasurement,
    pub producer_binding: ArtifactProducerBinding,
    pub reuse_report: ArtifactReuseReport,
}

pub fn create_artifact_reuse_report(request_path: &Path) -> Result<ArtifactReuseReport, CoreError> {
    let canonical_request = request_path.canonicalize().map_err(|_| {
        artifacts_error(
            ResultClass::Blocked,
            "FERRIS-ARTIFACT-REQUEST-UNAVAILABLE",
            "The artifact reuse request is unavailable.",
        )
    })?;
    let request = read_request(&canonical_request)?;
    let request = validate_and_canonicalize_request(request)?;

    let consumers = request
        .consumers
        .iter()
        .map(|consumer| classify_consumer(&request.producer, consumer))
        .collect::<Vec<_>>();
    let consumers_by_id = consumers
        .iter()
        .map(|consumer| (consumer.consumer_node_id.as_str(), consumer))
        .collect::<BTreeMap<_, _>>();
    let missing_consumer_ids = request
        .expected_consumer_ids
        .iter()
        .filter(|consumer_id| !consumers_by_id.contains_key(consumer_id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let incompatible_consumer_ids = request
        .expected_consumer_ids
        .iter()
        .filter_map(|consumer_id| {
            consumers_by_id
                .get(consumer_id.as_str())
                .filter(|consumer| {
                    !consumer.required
                        || consumer.classification != ArtifactConsumerClassification::Compatible
                })
                .map(|_| consumer_id.clone())
        })
        .collect::<Vec<_>>();
    let succeeded = missing_consumer_ids.is_empty() && incompatible_consumer_ids.is_empty();

    let mut report = ArtifactReuseReport {
        schema: ARTIFACT_REUSE_REPORT_SCHEMA.to_owned(),
        report_id: String::new(),
        status: ArtifactReuseReportStatus::ObservationOnly,
        producer: request.producer,
        consumers,
        fan_in: FanInReport {
            expected_consumer_ids: request.expected_consumer_ids,
            missing_consumer_ids,
            incompatible_consumer_ids,
            succeeded,
        },
    };
    report.report_id = artifact_reuse_report_identity(&report);
    Ok(report)
}

pub fn create_artifact_qualification_report(
    request_path: &Path,
    artifact_path: &Path,
    manifest_path: &Path,
) -> Result<ArtifactQualificationReport, CoreError> {
    let reuse_report = create_artifact_reuse_report(request_path)?;
    let (artifact_digest, artifact_size_bytes) =
        measure_file(artifact_path, "artifact", MAX_MEASURED_ARTIFACT_BYTES)?;
    let (manifest_identity, manifest_size_bytes) =
        measure_file(manifest_path, "manifest", MAX_MEASURED_MANIFEST_BYTES)?;
    let artifact_digest_matches = artifact_digest == reuse_report.producer.artifact_digest;
    let manifest_identity_matches =
        manifest_identity == reuse_report.producer.compatibility.manifest_identity;
    let producer_binding_succeeded = artifact_digest_matches && manifest_identity_matches;
    let qualified = producer_binding_succeeded && reuse_report.fan_in.succeeded;
    let mut report = ArtifactQualificationReport {
        schema: ARTIFACT_QUALIFICATION_REPORT_SCHEMA.to_owned(),
        qualification_id: String::new(),
        status: if qualified {
            ArtifactQualificationStatus::Qualified
        } else {
            ArtifactQualificationStatus::Rejected
        },
        measurement: ArtifactMeasurement {
            artifact_digest,
            artifact_size_bytes,
            manifest_identity,
            manifest_size_bytes,
        },
        producer_binding: ArtifactProducerBinding {
            artifact_digest_matches,
            manifest_identity_matches,
            succeeded: producer_binding_succeeded,
        },
        reuse_report,
    };
    report.qualification_id = artifact_qualification_report_identity(&report);
    Ok(report)
}

pub fn artifact_reuse_report_identity(report: &ArtifactReuseReport) -> String {
    let mut semantic = report.clone();
    semantic.report_id.clear();
    digest_bytes(
        &serde_json::to_vec(&semantic).expect("typed artifact reuse reports must serialize"),
    )
}

pub fn artifact_qualification_report_identity(report: &ArtifactQualificationReport) -> String {
    let mut semantic = report.clone();
    semantic.qualification_id.clear();
    digest_bytes(
        &serde_json::to_vec(&semantic)
            .expect("typed artifact qualification reports must serialize"),
    )
}

fn measure_file(path: &Path, label: &str, maximum_bytes: u64) -> Result<(String, u64), CoreError> {
    let path_metadata = fs::metadata(path).map_err(|_| {
        artifacts_error(
            ResultClass::Blocked,
            "FERRIS-ARTIFACT-MEASUREMENT-UNAVAILABLE",
            format!("The measured {label} is unavailable."),
        )
    })?;
    if !path_metadata.is_file() {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-MEASUREMENT-BOUND-INVALID",
            format!("The measured {label} is not a bounded regular file."),
        ));
    }
    let mut file = fs::File::open(path).map_err(|_| {
        artifacts_error(
            ResultClass::Blocked,
            "FERRIS-ARTIFACT-MEASUREMENT-UNAVAILABLE",
            format!("The measured {label} is unavailable."),
        )
    })?;
    let metadata = file.metadata().map_err(|_| {
        artifacts_error(
            ResultClass::Blocked,
            "FERRIS-ARTIFACT-MEASUREMENT-UNAVAILABLE",
            format!("Ferris could not inspect the measured {label}."),
        )
    })?;
    if !metadata.is_file() || metadata.len() > maximum_bytes {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-MEASUREMENT-BOUND-INVALID",
            format!("The measured {label} is not a bounded regular file."),
        ));
    }

    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut measured_bytes = 0_u64;
    loop {
        let count = file.read(&mut buffer).map_err(|_| {
            artifacts_error(
                ResultClass::Blocked,
                "FERRIS-ARTIFACT-MEASUREMENT-READ-FAILED",
                format!("Ferris could not read the measured {label}."),
            )
        })?;
        if count == 0 {
            break;
        }
        measured_bytes = measured_bytes.checked_add(count as u64).ok_or_else(|| {
            artifacts_invalid(
                "FERRIS-ARTIFACT-MEASUREMENT-BOUND-INVALID",
                format!("The measured {label} exceeded its supported size bound."),
            )
        })?;
        if measured_bytes > maximum_bytes {
            return Err(artifacts_invalid(
                "FERRIS-ARTIFACT-MEASUREMENT-BOUND-INVALID",
                format!("The measured {label} exceeded its supported size bound."),
            ));
        }
        hasher.update(&buffer[..count]);
    }
    let digest = hasher.finalize();
    let digest = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok((format!("sha256:{digest}"), measured_bytes))
}

fn read_request(path: &Path) -> Result<ArtifactReuseRequest, CoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        artifacts_error(
            ResultClass::Blocked,
            "FERRIS-ARTIFACT-REQUEST-UNAVAILABLE",
            "The artifact reuse request is unavailable.",
        )
    })?;
    if !metadata.is_file() || metadata.len() > MAX_ARTIFACT_REQUEST_BYTES {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-REQUEST-BOUND-INVALID",
            "The artifact reuse request is not a bounded regular file.",
        ));
    }
    let bytes = fs::read(path).map_err(|_| {
        artifacts_error(
            ResultClass::Blocked,
            "FERRIS-ARTIFACT-REQUEST-READ-FAILED",
            "Ferris could not read the artifact reuse request.",
        )
    })?;
    let value = serde_json::from_slice::<StrictJsonValue>(&bytes)
        .map(StrictJsonValue::into_inner)
        .map_err(|_| {
            artifacts_invalid(
                "FERRIS-ARTIFACT-REQUEST-JSON-INVALID",
                "The artifact reuse request is not strict JSON.",
            )
        })?;
    serde_json::from_value(value).map_err(|_| {
        artifacts_invalid(
            "FERRIS-ARTIFACT-REQUEST-SHAPE-INVALID",
            "The artifact reuse request does not match its strict schema.",
        )
    })
}

fn validate_and_canonicalize_request(
    mut request: ArtifactReuseRequest,
) -> Result<ArtifactReuseRequest, CoreError> {
    if request.schema != ARTIFACT_REUSE_REQUEST_SCHEMA {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-REQUEST-SCHEMA-INVALID",
            "The artifact reuse request schema is unsupported.",
        ));
    }
    validate_identity(&request.producer.producer_node_id, "producer node")?;
    validate_identity(&request.producer.attempt_id, "producer attempt")?;
    validate_identity(&request.producer.artifact_id, "producer artifact")?;
    validate_sha256(
        &request.producer.artifact_digest,
        "producer artifact digest",
    )?;
    validate_envelope(&mut request.producer.compatibility)?;

    if request.consumers.is_empty() || request.consumers.len() > MAX_ARTIFACT_CONSUMERS {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-CONSUMER-BOUND-INVALID",
            "The artifact reuse request must contain a bounded non-empty consumer list.",
        ));
    }
    let mut consumer_ids = BTreeSet::new();
    for consumer in &mut request.consumers {
        validate_identity(&consumer.consumer_node_id, "consumer node")?;
        validate_identity(&consumer.expected_artifact_id, "expected artifact")?;
        validate_sha256(
            &consumer.expected_artifact_digest,
            "expected artifact digest",
        )?;
        validate_envelope(&mut consumer.required_compatibility)?;
        if !consumer_ids.insert(consumer.consumer_node_id.clone()) {
            return Err(artifacts_invalid(
                "FERRIS-ARTIFACT-CONSUMER-DUPLICATE",
                "The artifact reuse request contains a duplicate consumer identity.",
            ));
        }
    }
    request
        .consumers
        .sort_by(|left, right| left.consumer_node_id.cmp(&right.consumer_node_id));

    if request.expected_consumer_ids.is_empty()
        || request.expected_consumer_ids.len() > MAX_ARTIFACT_CONSUMERS
    {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-EXPECTED-CONSUMER-BOUND-INVALID",
            "The artifact reuse request must contain a bounded non-empty expected consumer list.",
        ));
    }
    for consumer_id in &request.expected_consumer_ids {
        validate_identity(consumer_id, "expected consumer")?;
    }
    request.expected_consumer_ids.sort();
    if request
        .expected_consumer_ids
        .windows(2)
        .any(|pair| pair[0] == pair[1])
    {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-EXPECTED-CONSUMER-DUPLICATE",
            "The artifact reuse request contains a duplicate expected consumer identity.",
        ));
    }
    Ok(request)
}

fn validate_envelope(envelope: &mut CompatibilityEnvelope) -> Result<(), CoreError> {
    for (value, label) in [
        (&envelope.repository_id, "repository"),
        (&envelope.platform_os, "platform operating system"),
        (&envelope.platform_architecture, "platform architecture"),
        (&envelope.target, "target"),
        (&envelope.profile, "profile"),
    ] {
        validate_identity(value, label)?;
    }
    validate_source_revision(&envelope.source_revision)?;
    validate_sha256(&envelope.toolchain_identity, "toolchain")?;
    validate_sha256(&envelope.configuration_identity, "configuration")?;
    validate_sha256(&envelope.manifest_identity, "manifest")?;
    validate_sha256(&envelope.command_identity, "command")?;
    if envelope.features.len() > MAX_ARTIFACT_FEATURES {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-FEATURE-BOUND-INVALID",
            "An artifact compatibility feature set exceeds the supported bound.",
        ));
    }
    for feature in &envelope.features {
        validate_identity(feature, "feature")?;
    }
    envelope.features.sort();
    if envelope.features.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(artifacts_invalid(
            "FERRIS-ARTIFACT-FEATURE-DUPLICATE",
            "An artifact compatibility envelope contains a duplicate feature.",
        ));
    }
    Ok(())
}

fn classify_consumer(
    producer: &ArtifactProducer,
    consumer: &ArtifactConsumer,
) -> ArtifactConsumerResult {
    let (classification, compatibility_mismatches) = match producer.terminal_status {
        ArtifactProducerTerminalStatus::Failed => {
            (ArtifactConsumerClassification::ProducerFailed, Vec::new())
        }
        ArtifactProducerTerminalStatus::Cancelled => (
            ArtifactConsumerClassification::ProducerCancelled,
            Vec::new(),
        ),
        ArtifactProducerTerminalStatus::Unavailable => (
            ArtifactConsumerClassification::ProducerUnavailable,
            Vec::new(),
        ),
        ArtifactProducerTerminalStatus::Succeeded
            if producer.artifact_id != consumer.expected_artifact_id
                || producer.artifact_digest != consumer.expected_artifact_digest =>
        {
            (
                ArtifactConsumerClassification::ArtifactIdentityMismatch,
                Vec::new(),
            )
        }
        ArtifactProducerTerminalStatus::Succeeded => {
            let mismatches =
                compatibility_mismatches(&producer.compatibility, &consumer.required_compatibility);
            let classification = if mismatches.is_empty() {
                ArtifactConsumerClassification::Compatible
            } else {
                ArtifactConsumerClassification::CompatibilityMismatch
            };
            (classification, mismatches)
        }
    };
    ArtifactConsumerResult {
        consumer_node_id: consumer.consumer_node_id.clone(),
        required: consumer.required,
        expected_artifact_id: consumer.expected_artifact_id.clone(),
        expected_artifact_digest: consumer.expected_artifact_digest.clone(),
        classification,
        compatibility_mismatches,
    }
}

fn compatibility_mismatches(
    producer: &CompatibilityEnvelope,
    consumer: &CompatibilityEnvelope,
) -> Vec<CompatibilityDimension> {
    let mut mismatches = Vec::new();
    if producer.repository_id != consumer.repository_id {
        mismatches.push(CompatibilityDimension::Repository);
    }
    if producer.source_revision != consumer.source_revision {
        mismatches.push(CompatibilityDimension::SourceRevision);
    }
    if producer.toolchain_identity != consumer.toolchain_identity {
        mismatches.push(CompatibilityDimension::Toolchain);
    }
    if producer.platform_os != consumer.platform_os {
        mismatches.push(CompatibilityDimension::PlatformOs);
    }
    if producer.platform_architecture != consumer.platform_architecture {
        mismatches.push(CompatibilityDimension::PlatformArchitecture);
    }
    if producer.target != consumer.target {
        mismatches.push(CompatibilityDimension::Target);
    }
    if producer.profile != consumer.profile {
        mismatches.push(CompatibilityDimension::Profile);
    }
    if producer.features != consumer.features {
        mismatches.push(CompatibilityDimension::Features);
    }
    if producer.configuration_identity != consumer.configuration_identity {
        mismatches.push(CompatibilityDimension::Configuration);
    }
    if producer.manifest_identity != consumer.manifest_identity {
        mismatches.push(CompatibilityDimension::Manifest);
    }
    if producer.command_identity != consumer.command_identity {
        mismatches.push(CompatibilityDimension::Command);
    }
    mismatches
}

fn validate_identity(value: &str, label: &str) -> Result<(), CoreError> {
    if valid_portable_id(value) {
        return Ok(());
    }
    Err(artifacts_invalid(
        "FERRIS-ARTIFACT-IDENTITY-INVALID",
        format!("The {label} identity is not a bounded stable portable identifier."),
    ))
}

fn validate_source_revision(value: &str) -> Result<(), CoreError> {
    if (40..=64).contains(&value.len())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Ok(());
    }
    Err(artifacts_invalid(
        "FERRIS-ARTIFACT-SOURCE-REVISION-INVALID",
        "The source revision must be a lowercase Git object identity.",
    ))
}

fn validate_sha256(value: &str, label: &str) -> Result<(), CoreError> {
    if value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Ok(());
    }
    Err(artifacts_invalid(
        "FERRIS-ARTIFACT-HASH-INVALID",
        format!("The {label} must be lowercase sha256:<64-hex>."),
    ))
}

fn artifacts_invalid(code: &str, message: impl Into<String>) -> CoreError {
    artifacts_error(ResultClass::Invalid, code, message)
}

fn artifacts_error(class: ResultClass, code: &str, message: impl Into<String>) -> CoreError {
    CoreError::new(
        class,
        code,
        message,
        vec!["Repair the artifact reuse request and retry.".to_owned()],
    )
}
