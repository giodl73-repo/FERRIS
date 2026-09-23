use super::{CoreError, ResultClass, digest_bytes};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};

pub const OWNER_EXECUTABLE_STAGING_RECEIPT_SCHEMA: &str =
    "ferris.owner-executable-staging-receipt/v1";

const MAX_STAGED_EXECUTABLE_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct OwnerExecutableStagingRequest<'a> {
    pub repository_root: &'a Path,
    pub source_path: &'a Path,
    pub destination: &'a str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerExecutableStagingReceipt {
    pub schema: String,
    pub staging_id: String,
    pub destination: String,
    pub content_identity: String,
    pub byte_length: u64,
    pub source_path_retained: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnerExecutableStagingOutcome {
    pub receipt: OwnerExecutableStagingReceipt,
    pub destination_path: PathBuf,
}

#[derive(Serialize)]
struct StagingIdentityProjection<'a> {
    schema: &'a str,
    destination: &'a str,
    content_identity: &'a str,
    byte_length: u64,
    source_path_retained: bool,
}

pub fn stage_owner_executable(
    request: OwnerExecutableStagingRequest<'_>,
) -> Result<OwnerExecutableStagingOutcome, CoreError> {
    let root = canonical_repository_root(request.repository_root)?;
    validate_destination(request.destination)?;
    let source_path = canonical_source(&root, request.source_path)?;
    let source_metadata = fs::metadata(&source_path).map_err(|_| {
        staging_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-UNAVAILABLE",
            "The explicitly selected source executable is unavailable.",
        )
    })?;
    if !source_metadata.is_file() {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-INVALID",
            "The explicitly selected source executable must be a regular file.",
        ));
    }
    if source_metadata.len() == 0 || source_metadata.len() > MAX_STAGED_EXECUTABLE_BYTES {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-BOUND-INVALID",
            "The source executable must be non-empty and no larger than 512 MiB.",
        ));
    }
    #[cfg(unix)]
    if source_metadata.mode() & 0o111 == 0 {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-NOT-EXECUTABLE",
            "The source executable has no executable permission bit.",
        ));
    }
    let destination = resolve_destination(&root, request.destination)?;
    if let Destination::Existing(destination_path) = destination {
        let (source_identity, source_length) = hash_source(&source_path)?;
        let (current_source_identity, current_source_length) = hash_source(&source_path)?;
        let (destination_identity, destination_length) =
            hash_existing_destination(&destination_path)?;
        if current_source_identity != source_identity
            || current_source_length != source_length
            || !source_metadata_unchanged(&source_path, &source_metadata)
        {
            return Err(source_changed());
        }
        #[cfg(unix)]
        let destination_permissions_match = fs::metadata(&destination_path)
            .is_ok_and(|metadata| metadata.mode() & 0o7777 == source_metadata.mode() & 0o777);
        #[cfg(not(unix))]
        let destination_permissions_match = true;
        if destination_identity != source_identity
            || destination_length != source_length
            || !destination_permissions_match
        {
            return Err(staging_error(
                ResultClass::Invalid,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS",
                "The executable staging destination already exists with different content or permissions.",
            ));
        }
        return Ok(staging_outcome(
            request.destination,
            source_identity,
            source_length,
            destination_path,
        ));
    }
    let Destination::Absent(destination_path) = destination else {
        unreachable!("existing executable staging destination returned above")
    };

    let parent = destination_path
        .parent()
        .expect("resolved executable staging destination parent");
    let (mut destination_file, temporary) = create_temporary_destination(parent)?;
    let (content_identity, byte_length) = copy_source(
        &source_path,
        &mut destination_file,
        #[cfg(unix)]
        source_metadata.mode(),
    )?;
    drop(destination_file);

    let (current_source_identity, current_source_length) = hash_source(&source_path)?;
    let (temporary_identity, temporary_length) = hash_temporary(&temporary.0)?;
    if current_source_identity != content_identity
        || current_source_length != byte_length
        || !source_metadata_unchanged(&source_path, &source_metadata)
        || temporary_identity != content_identity
        || temporary_length != byte_length
    {
        return Err(source_changed());
    }

    fs::hard_link(&temporary.0, &destination_path).map_err(|error| {
        if error.kind() == io::ErrorKind::AlreadyExists {
            staging_error(
                ResultClass::Invalid,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS",
                "The executable staging destination already exists.",
            )
        } else {
            staging_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
                "The staged executable could not be committed atomically.",
            )
        }
    })?;
    drop(temporary);

    Ok(staging_outcome(
        request.destination,
        content_identity,
        byte_length,
        destination_path,
    ))
}

fn staging_outcome(
    destination: &str,
    content_identity: String,
    byte_length: u64,
    destination_path: PathBuf,
) -> OwnerExecutableStagingOutcome {
    let projection = StagingIdentityProjection {
        schema: OWNER_EXECUTABLE_STAGING_RECEIPT_SCHEMA,
        destination,
        content_identity: &content_identity,
        byte_length,
        source_path_retained: false,
    };
    let staging_id = format!(
        "owner-executable-stage:{}",
        digest_bytes(
            &serde_json::to_vec(&projection)
                .expect("typed owner executable staging identity must serialize")
        )
        .trim_start_matches("sha256:")
    );
    let receipt = OwnerExecutableStagingReceipt {
        schema: OWNER_EXECUTABLE_STAGING_RECEIPT_SCHEMA.to_owned(),
        staging_id,
        destination: destination.to_owned(),
        content_identity,
        byte_length,
        source_path_retained: false,
    };
    OwnerExecutableStagingOutcome {
        receipt,
        destination_path,
    }
}

pub fn render_owner_executable_staging_human(outcome: &OwnerExecutableStagingOutcome) -> String {
    format!(
        "Ferris owner executable staged\nStaging: {}\nDestination: {}\nIdentity: {}\nBytes: {}\nSource path retained: no\n",
        outcome.receipt.staging_id,
        outcome.receipt.destination,
        outcome.receipt.content_identity,
        outcome.receipt.byte_length,
    )
}

fn canonical_repository_root(repository_root: &Path) -> Result<PathBuf, CoreError> {
    let root = repository_root.canonicalize().map_err(|_| {
        staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-ROOT-INVALID",
            "The executable staging repository root could not be canonicalized.",
        )
    })?;
    if !root.is_dir() {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-ROOT-INVALID",
            "The executable staging repository root is not a directory.",
        ));
    }
    Ok(root)
}

fn canonical_source(root: &Path, source: &Path) -> Result<PathBuf, CoreError> {
    let candidate = if source.is_absolute() {
        source.to_owned()
    } else {
        root.join(source)
    };
    candidate.canonicalize().map_err(|_| {
        staging_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-UNAVAILABLE",
            "The explicitly selected source executable is unavailable.",
        )
    })
}

fn validate_destination(destination: &str) -> Result<(), CoreError> {
    if destination.is_empty()
        || destination.contains('\\')
        || destination.contains(':')
        || destination
            .chars()
            .any(|character| character.is_control() || character == '\u{7f}')
        || Path::new(destination).is_absolute()
        || destination
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        || Path::new(destination)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-INVALID",
            "The executable staging destination must be a portable repository-relative file path.",
        ));
    }
    Ok(())
}

enum Destination {
    Absent(PathBuf),
    Existing(PathBuf),
}

fn resolve_destination(root: &Path, destination: &str) -> Result<Destination, CoreError> {
    let candidate = root.join(destination);
    match fs::symlink_metadata(&candidate) {
        Ok(_) => {
            let canonical = candidate.canonicalize().map_err(|_| {
                staging_error(
                    ResultClass::Invalid,
                    "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS",
                    "The executable staging destination already exists and is not reusable.",
                )
            })?;
            if !canonical.starts_with(root) || !canonical.is_file() {
                return Err(staging_error(
                    ResultClass::Invalid,
                    "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS",
                    "The executable staging destination already exists and is not a repository-local regular file.",
                ));
            }
            return Ok(Destination::Existing(canonical));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(_) => {
            return Err(staging_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
                "The executable staging destination could not be inspected.",
            ));
        }
    }
    let parent = candidate.parent().ok_or_else(|| {
        staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-INVALID",
            "The executable staging destination has no parent directory.",
        )
    })?;
    let canonical_parent = parent.canonicalize().map_err(|_| {
        staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-PARENT-INVALID",
            "The executable staging destination parent must already exist.",
        )
    })?;
    if !canonical_parent.starts_with(root) || !canonical_parent.is_dir() {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-ESCAPE",
            "The executable staging destination must remain inside the repository.",
        ));
    }
    Ok(Destination::Absent(
        canonical_parent.join(
            candidate
                .file_name()
                .expect("validated executable staging destination filename"),
        ),
    ))
}

struct TemporaryExecutable(PathBuf);

impl Drop for TemporaryExecutable {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn create_temporary_destination(
    parent: &Path,
) -> Result<(fs::File, TemporaryExecutable), CoreError> {
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);
    (0..128)
        .find_map(|_| {
            let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(
                ".ferris-owner-executable-{}-{counter}.tmp",
                std::process::id()
            ));
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            options.mode(0o600);
            match options.open(&path) {
                Ok(file) => Some(Ok((file, TemporaryExecutable(path)))),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => None,
                Err(_) => Some(Err(staging_error(
                    ResultClass::Blocked,
                    "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
                    "A temporary executable staging file could not be created.",
                ))),
            }
        })
        .unwrap_or_else(|| {
            Err(staging_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
                "A unique temporary executable staging file could not be created.",
            ))
        })
}

fn copy_source(
    source_path: &Path,
    destination: &mut fs::File,
    #[cfg(unix)] source_mode: u32,
) -> Result<(String, u64), CoreError> {
    let mut source = fs::File::open(source_path).map_err(|_| {
        staging_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-READ-FAILED",
            "Ferris could not read the explicitly selected source executable.",
        )
    })?;
    let mut hasher = Sha256::new();
    let mut byte_length = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = source.read(&mut buffer).map_err(|_| {
            staging_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTABLE-STAGING-SOURCE-READ-FAILED",
                "Ferris could not read the explicitly selected source executable.",
            )
        })?;
        if count == 0 {
            break;
        }
        byte_length = byte_length.saturating_add(count as u64);
        if byte_length > MAX_STAGED_EXECUTABLE_BYTES {
            return Err(staging_error(
                ResultClass::Invalid,
                "FERRIS-EXECUTABLE-STAGING-SOURCE-BOUND-INVALID",
                "The source executable must be non-empty and no larger than 512 MiB.",
            ));
        }
        hasher.update(&buffer[..count]);
        destination.write_all(&buffer[..count]).map_err(|_| {
            staging_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
                "Ferris could not write the staged executable.",
            )
        })?;
    }
    if byte_length == 0 {
        return Err(staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-SOURCE-BOUND-INVALID",
            "The source executable must be non-empty and no larger than 512 MiB.",
        ));
    }
    #[cfg(unix)]
    destination
        .set_permissions(fs::Permissions::from_mode(source_mode & 0o777))
        .map_err(|_| {
            staging_error(
                ResultClass::Blocked,
                "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
                "Ferris could not preserve the source executable permissions.",
            )
        })?;
    destination.sync_all().map_err(|_| {
        staging_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
            "Ferris could not synchronize the staged executable.",
        )
    })?;
    Ok((sha256_identity(hasher.finalize()), byte_length))
}

enum HashError {
    Unavailable,
    BoundExceeded,
}

fn hash_bounded_file(path: &Path) -> Result<(String, u64), HashError> {
    let mut file = fs::File::open(path).map_err(|_| HashError::Unavailable)?;
    let mut hasher = Sha256::new();
    let mut byte_length = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|_| HashError::Unavailable)?;
        if count == 0 {
            break;
        }
        byte_length = byte_length.saturating_add(count as u64);
        if byte_length > MAX_STAGED_EXECUTABLE_BYTES {
            return Err(HashError::BoundExceeded);
        }
        hasher.update(&buffer[..count]);
    }
    Ok((sha256_identity(hasher.finalize()), byte_length))
}

fn hash_source(path: &Path) -> Result<(String, u64), CoreError> {
    hash_bounded_file(path).map_err(|_| source_changed())
}

fn hash_existing_destination(path: &Path) -> Result<(String, u64), CoreError> {
    hash_bounded_file(path).map_err(|error| match error {
        HashError::Unavailable => staging_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
            "The existing executable staging destination could not be read.",
        ),
        HashError::BoundExceeded => staging_error(
            ResultClass::Invalid,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS",
            "The executable staging destination already exists with different content or permissions.",
        ),
    })
}

fn hash_temporary(path: &Path) -> Result<(String, u64), CoreError> {
    hash_bounded_file(path).map_err(|_| {
        staging_error(
            ResultClass::Blocked,
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-UNAVAILABLE",
            "The temporary executable staging file could not be verified.",
        )
    })
}

fn source_metadata_unchanged(path: &Path, initial: &fs::Metadata) -> bool {
    fs::metadata(path).is_ok_and(|current| {
        let unchanged = current.is_file() && current.len() == initial.len();
        #[cfg(unix)]
        let unchanged = unchanged && current.mode() & 0o777 == initial.mode() & 0o777;
        unchanged
    })
}

fn source_changed() -> CoreError {
    staging_error(
        ResultClass::Stale,
        "FERRIS-EXECUTABLE-STAGING-SOURCE-CHANGED",
        "The source executable changed while Ferris was staging it.",
    )
}

fn sha256_identity(digest: impl AsRef<[u8]>) -> String {
    format!(
        "sha256:{}",
        digest
            .as_ref()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    )
}

fn staging_error(class: ResultClass, code: &str, message: &str) -> CoreError {
    CoreError::new(
        class,
        code,
        message,
        vec![
            "Select an explicit readable executable and an absent repository-local destination, then retry."
                .to_owned(),
        ],
    )
}
