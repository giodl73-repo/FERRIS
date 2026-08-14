#!/usr/bin/env python3
"""Copy the exact public Pulse 39 release through one transactional publication."""

from __future__ import annotations

import argparse
import errno
import hashlib
import json
import os
import stat
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Iterable


@dataclass(frozen=True)
class Binding:
    path: str
    size: int
    sha256: str


CANONICAL_FILES = (
    Binding(
        "README.md",
        1786,
        "sha256:9e19afae44aa5c112ddcde67fbdaf501903b5cb39ce3757e5bc6fea8554c7989",
    ),
    Binding(
        "checkout_verifier.py",
        9685,
        "sha256:783283fd127170460ce52106a7a1158054cdc2608475e53899ff45a7a6a31d12",
    ),
    Binding(
        "public-manifest.json",
        1387,
        "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c",
    ),
    Binding(
        "qualification-receipt.json",
        2057,
        "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8",
    ),
    Binding(
        "release-seal.json",
        1901,
        "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c",
    ),
    Binding(
        "root-cause-report.json",
        1266,
        "sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd",
    ),
    Binding(
        "root-cause-report.md",
        1727,
        "sha256:9cfedd9a239bc869c35b728564267c206db981126c502121bce43a68b533b92e",
    ),
    Binding(
        "tests/test_checkout_verifier.py",
        11991,
        "sha256:02a57858dbb65cb678b614e0a906a8bab6f9437d69efd2cbc60fac0d4b689440",
    ),
)
EXPECTED_PATHS = tuple(binding.path for binding in CANONICAL_FILES)
EXPECTED_DIRECTORIES = frozenset({"tests"})
EXPECTED_COUNT = len(CANONICAL_FILES)
SYNC_MECHANISM = "os.open+os.fsync-directory-v1"


class PublicFailure(Exception):
    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass(frozen=True)
class SyncPosture:
    status: str
    mechanism: str
    error_category: str | None
    attempted: bool = True

    def public(self) -> dict[str, object]:
        return {
            "attempted": self.attempted,
            "error_category": self.error_category,
            "mechanism": self.mechanism,
            "status": self.status,
        }


@dataclass(frozen=True)
class StagingSyncPosture:
    directory_count: int
    attempts: int
    synced: int
    unsupported: int
    operational_failures: int
    mechanism: str
    status: str
    error_category: str | None
    unsupported_error_categories: tuple[str, ...]

    def public(self) -> dict[str, object]:
        return {
            "attempts": self.attempts,
            "directories": self.directory_count,
            "error_category": self.error_category,
            "mechanism": self.mechanism,
            "operational_failures": self.operational_failures,
            "status": self.status,
            "synced": self.synced,
            "unsupported": self.unsupported,
            "unsupported_error_categories": list(self.unsupported_error_categories),
        }


def _not_attempted_sync() -> SyncPosture:
    return SyncPosture(
        status="unsupported",
        mechanism="not-attempted",
        error_category="not-attempted",
        attempted=False,
    )


def _not_attempted_staging_sync() -> StagingSyncPosture:
    return StagingSyncPosture(
        directory_count=0,
        attempts=0,
        synced=0,
        unsupported=0,
        operational_failures=0,
        mechanism="not-attempted",
        status="not-attempted",
        error_category="not-attempted",
        unsupported_error_categories=(),
    )


def _failed_sync() -> SyncPosture:
    return SyncPosture(
        status="unsupported",
        mechanism=SYNC_MECHANISM,
        error_category="sync-operation-failed",
    )


def public_json(value: dict[str, object]) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def _lexists(path: Path) -> bool:
    return os.path.lexists(path)


def _absolute_input(value: str | os.PathLike[str]) -> Path:
    try:
        raw = os.fspath(value)
    except TypeError as error:
        raise PublicFailure("P41-ARGUMENT") from error
    if not isinstance(raw, str) or "\x00" in raw:
        raise PublicFailure("P41-ARGUMENT")
    path = Path(raw)
    if not path.is_absolute() or ".." in path.parts:
        raise PublicFailure("P41-ARGUMENT")
    return path


def _safe_absolute(value: str | os.PathLike[str]) -> Path:
    try:
        return _absolute_input(value).resolve(strict=False)
    except OSError as error:
        raise PublicFailure("P41-ARGUMENT") from error


def _is_within(first: Path, second: Path) -> bool:
    try:
        first.relative_to(second)
    except ValueError:
        return False
    return True


def _validate_source_root(value: str | os.PathLike[str]) -> Path:
    input_root = _absolute_input(value)
    root = _safe_absolute(value)
    try:
        metadata = os.lstat(input_root)
    except OSError as error:
        raise PublicFailure("P41-SOURCE-ROOT-INVALID") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        raise PublicFailure("P41-SOURCE-ROOT-INVALID")
    return root


def _validate_final_root(
    value: str | os.PathLike[str], source_root: Path
) -> tuple[Path, Path]:
    input_final_root = _absolute_input(value)
    final_root = _safe_absolute(value)
    if _lexists(input_final_root) or _lexists(final_root):
        raise PublicFailure("P41-FINAL-EXISTS")
    if _is_within(final_root, source_root) or _is_within(source_root, final_root):
        raise PublicFailure("P41-ROOTS-OVERLAP")
    parent = final_root.parent
    try:
        metadata = os.lstat(input_final_root.parent)
    except OSError as error:
        raise PublicFailure("P41-FINAL-PARENT-UNSAFE") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        raise PublicFailure("P41-FINAL-PARENT-UNSAFE")
    return final_root, parent


def _validate_regular(path: Path) -> os.stat_result:
    try:
        metadata = os.lstat(path)
    except OSError as error:
        raise PublicFailure("P41-TREE-UNSAFE") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        raise PublicFailure("P41-TREE-UNSAFE")
    return metadata


def _scan_tree(root: Path, phase: str) -> dict[str, Path]:
    try:
        root_metadata = os.lstat(root)
    except OSError as error:
        raise PublicFailure(f"P41-{phase}-ROOT-INVALID") from error
    if stat.S_ISLNK(root_metadata.st_mode) or not stat.S_ISDIR(root_metadata.st_mode):
        raise PublicFailure(f"P41-{phase}-ROOT-INVALID")

    directories: set[str] = set()
    files: dict[str, Path] = {}
    pending = [root]
    while pending:
        directory = pending.pop()
        try:
            entries = sorted(os.scandir(directory), key=lambda entry: entry.name)
        except OSError as error:
            raise PublicFailure(f"P41-{phase}-TREE-UNSAFE") from error
        for entry in entries:
            path = Path(entry.path)
            relative = path.relative_to(root).as_posix()
            try:
                metadata = os.lstat(path)
            except OSError as error:
                raise PublicFailure(f"P41-{phase}-TREE-UNSAFE") from error
            if stat.S_ISLNK(metadata.st_mode):
                raise PublicFailure(f"P41-{phase}-TREE-UNSAFE")
            if stat.S_ISDIR(metadata.st_mode):
                directories.add(relative)
                pending.append(path)
            elif stat.S_ISREG(metadata.st_mode):
                files[relative] = path
            else:
                raise PublicFailure(f"P41-{phase}-TREE-UNSAFE")

    if directories != EXPECTED_DIRECTORIES or set(files) != set(EXPECTED_PATHS):
        raise PublicFailure(f"P41-{phase}-PATH-SET")
    return files


def _digest_regular_file(path: Path) -> tuple[int, str]:
    initial = _validate_regular(path)
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        raise PublicFailure("P41-TREE-UNSAFE") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode):
            raise PublicFailure("P41-TREE-UNSAFE")
        if (opened.st_dev, opened.st_ino) != (initial.st_dev, initial.st_ino):
            raise PublicFailure("P41-TREE-UNSAFE")
        digest = hashlib.sha256()
        size = 0
        while chunk := os.read(descriptor, 65536):
            digest.update(chunk)
            size += len(chunk)
    finally:
        os.close(descriptor)
    return size, f"sha256:{digest.hexdigest()}"


def verify_bound_tree(root: Path, phase: str) -> None:
    files = _scan_tree(root, phase)
    for binding in CANONICAL_FILES:
        size, digest = _digest_regular_file(files[binding.path])
        if size != binding.size or digest != binding.sha256:
            raise PublicFailure(f"P41-{phase}-BINDING")


def _copy_exact_bytes(source: Path, destination: Path) -> None:
    source_descriptor: int | None = None
    destination_descriptor: int | None = None
    source_file = None
    destination_file = None
    try:
        source_descriptor = os.open(
            source,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
        source_metadata = os.fstat(source_descriptor)
        if not stat.S_ISREG(source_metadata.st_mode):
            raise PublicFailure("P41-STAGE-COPY-FAILURE")
        destination_descriptor = os.open(
            destination,
            os.O_WRONLY
            | os.O_CREAT
            | os.O_EXCL
            | getattr(os, "O_BINARY", 0)
            | getattr(os, "O_NOFOLLOW", 0),
            0o600,
        )
        source_file = os.fdopen(source_descriptor, "rb", closefd=False)
        destination_file = os.fdopen(destination_descriptor, "wb", closefd=False)
        while chunk := source_file.read(65536):
            destination_file.write(chunk)
        destination_file.flush()
        os.fsync(destination_descriptor)
    except PublicFailure:
        raise
    except Exception as error:
        raise PublicFailure("P41-STAGE-COPY-FAILURE") from error
    finally:
        close_error: OSError | None = None
        for file in (destination_file, source_file):
            if file is None:
                continue
            try:
                file.close()
            except OSError as error:
                close_error = close_error or error
        for descriptor in (destination_descriptor, source_descriptor):
            if descriptor is None:
                continue
            try:
                os.close(descriptor)
            except OSError as error:
                close_error = close_error or error
        if close_error is not None:
            raise PublicFailure("P41-STAGE-COPY-FAILURE") from close_error


def _directory_error_category(error: OSError) -> str:
    unsupported = {
        errno.EACCES,
        errno.EINVAL,
        errno.EPERM,
        getattr(errno, "ENOTSUP", errno.EINVAL),
        getattr(errno, "EOPNOTSUPP", errno.EINVAL),
    }
    if error.errno in unsupported:
        return "unsupported-by-platform-or-filesystem"
    return "os-error"


def sync_directory(path: Path) -> SyncPosture:
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_DIRECTORY", 0)
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        category = _directory_error_category(error)
        if category == "os-error":
            raise
        return SyncPosture(
            status="unsupported",
            mechanism=SYNC_MECHANISM,
            error_category=category,
        )
    try:
        os.fsync(descriptor)
    except OSError as error:
        category = _directory_error_category(error)
        if category == "os-error":
            raise
        return SyncPosture(
            status="unsupported",
            mechanism=SYNC_MECHANISM,
            error_category=category,
        )
    finally:
        os.close(descriptor)
    return SyncPosture(status="synced", mechanism=SYNC_MECHANISM, error_category=None)


def _validate_sync_posture(value: SyncPosture) -> SyncPosture:
    if (
        not isinstance(value, SyncPosture)
        or not value.attempted
        or value.status not in {"synced", "unsupported"}
        or (value.status == "synced" and value.error_category is not None)
        or (value.status == "unsupported" and value.error_category is None)
        or value.error_category in {"not-attempted", "sync-operation-failed"}
    ):
        raise ValueError("invalid public sync posture")
    return value


def _staging_sync_posture(
    directories: list[Path], outcomes: list[SyncPosture], operational_failures: int
) -> StagingSyncPosture:
    synced_count = sum(outcome.status == "synced" for outcome in outcomes)
    unsupported_outcomes = [
        outcome for outcome in outcomes if outcome.status == "unsupported"
    ]
    unsupported_categories = tuple(
        sorted(
            {
                outcome.error_category
                for outcome in unsupported_outcomes
                if outcome.error_category is not None
            }
        )
    )
    mechanisms = {outcome.mechanism for outcome in outcomes}
    mechanism = (
        next(iter(mechanisms))
        if len(mechanisms) == 1
        else ("multiple-directory-sync-mechanisms" if mechanisms else SYNC_MECHANISM)
    )
    if operational_failures:
        status = "failed"
        error_category = "sync-operation-failed"
    elif unsupported_outcomes:
        status = "unsupported"
        error_category = (
            unsupported_categories[0]
            if len(unsupported_categories) == 1
            else "multiple-unsupported-postures"
        )
    else:
        status = "synced"
        error_category = None
    return StagingSyncPosture(
        directory_count=len(directories),
        attempts=len(outcomes) + operational_failures,
        synced=synced_count,
        unsupported=len(unsupported_outcomes),
        operational_failures=operational_failures,
        mechanism=mechanism,
        status=status,
        error_category=error_category,
        unsupported_error_categories=unsupported_categories,
    )


def _sync_staging_directories(
    directories: list[Path], synchronizer: SyncFunction, state: TransactionState
) -> None:
    outcomes: list[SyncPosture] = []
    for directory in reversed(directories):
        try:
            outcomes.append(_validate_sync_posture(synchronizer(directory)))
        except Exception:
            state.staging_sync = _staging_sync_posture(directories, outcomes, 1)
            raise
    state.staging_sync = _staging_sync_posture(directories, outcomes, 0)


def remove_tree(path: Path) -> None:
    if not _lexists(path):
        return
    metadata = os.lstat(path)
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        os.unlink(path)
        return
    for entry in os.scandir(path):
        remove_tree(Path(entry.path))
    os.rmdir(path)


@dataclass
class TransactionState:
    source: str = "0/8"
    stage: str = "0/8"
    final: str = "0/8"
    rename_attempts: int = 0
    retries: int = 0
    staging_sync: StagingSyncPosture = _not_attempted_staging_sync()
    final_parent_sync: SyncPosture = _not_attempted_sync()
    rollback_parent_sync: SyncPosture = _not_attempted_sync()
    rollback_attempted: bool = False
    rollback_path_absent: bool = False
    rollback_verified_absent: bool = False
    indeterminate_publication: bool = False

    def report(self, status: str, failure_code: str | None) -> dict[str, object]:
        return {
            "counts": {
                "final": self.final,
                "source": self.source,
                "stage": self.stage,
            },
            "failure_code": failure_code,
            "indeterminate_publication": self.indeterminate_publication,
            "rename_attempts": self.rename_attempts,
            "retries": self.retries,
            "rollback_attempted": self.rollback_attempted,
            "rollback_path_absent": self.rollback_path_absent,
            "rollback_verified_absent": self.rollback_verified_absent,
            "schema": "ferris.pulse-41-transactional-copy-report/v1",
            "status": status,
            "sync": {
                "final_parent": self.final_parent_sync.public(),
                "rollback_parent": self.rollback_parent_sync.public(),
                "staging": self.staging_sync.public(),
            },
        }


SyncFunction = Callable[[Path], SyncPosture]
CopyFunction = Callable[[Path, Path], None]
RenameFunction = Callable[[Path, Path], None]
RemoveFunction = Callable[[Path], None]
PostRenameFunction = Callable[[Path], None]


def _clean_stage(
    staging_root: Path, remover: RemoveFunction, state: TransactionState, code: str
) -> dict[str, object]:
    try:
        remover(staging_root)
    except OSError:
        return state.report("fail", "P41-STAGE-CLEANUP-FAILED")
    if _lexists(staging_root):
        return state.report("fail", "P41-STAGE-CLEANUP-FAILED")
    return state.report("fail", code)


def _rollback_final(
    final_root: Path,
    final_parent: Path,
    synchronizer: SyncFunction,
    remover: RemoveFunction,
    state: TransactionState,
) -> bool:
    state.rollback_attempted = True
    try:
        remover(final_root)
    except Exception:
        state.indeterminate_publication = True
        return False
    try:
        state.rollback_path_absent = not _lexists(final_root)
    except Exception:
        state.indeterminate_publication = True
        return False
    if not state.rollback_path_absent:
        state.indeterminate_publication = True
        return False
    try:
        state.rollback_parent_sync = _validate_sync_posture(synchronizer(final_parent))
    except Exception:
        state.rollback_parent_sync = _failed_sync()
        state.indeterminate_publication = True
        return False
    state.rollback_verified_absent = True
    return True


def _final_failure(
    final_root: Path,
    final_parent: Path,
    synchronizer: SyncFunction,
    remover: RemoveFunction,
    state: TransactionState,
    code: str,
) -> dict[str, object]:
    if not _rollback_final(final_root, final_parent, synchronizer, remover, state):
        return state.report("fail", "P41-INDETERMINATE-PUBLICATION")
    return state.report("fail", code)


def copy_release(
    source_root_value: str | os.PathLike[str],
    final_root_value: str | os.PathLike[str],
    *,
    synchronizer: SyncFunction = sync_directory,
    copier: CopyFunction = _copy_exact_bytes,
    renamer: RenameFunction = os.replace,
    remover: RemoveFunction = remove_tree,
    post_rename: PostRenameFunction | None = None,
) -> dict[str, object]:
    """Copy one exact source tree with zero logical retries and one rename."""

    state = TransactionState()
    try:
        source_root = _validate_source_root(source_root_value)
        verify_bound_tree(source_root, "SOURCE")
        state.source = "8/8"
        final_before_publication, final_parent = _validate_final_root(
            final_root_value, source_root
        )
    except PublicFailure as error:
        return state.report("fail", error.code)

    staging_root = final_parent / f".{final_before_publication.name}.pulse-41-stage"
    if _lexists(staging_root):
        return state.report("fail", "P41-STAGING-EXISTS")

    created_staging_directories: list[Path] = []
    try:
        os.mkdir(staging_root)
        created_staging_directories.append(staging_root)
        for directory in sorted(EXPECTED_DIRECTORIES):
            created_directory = staging_root.joinpath(*directory.split("/"))
            os.mkdir(created_directory)
            created_staging_directories.append(created_directory)
        for binding in CANONICAL_FILES:
            copier(
                source_root.joinpath(*binding.path.split("/")),
                staging_root.joinpath(*binding.path.split("/")),
            )
    except Exception:
        return _clean_stage(staging_root, remover, state, "P41-STAGE-COPY-FAILURE")

    try:
        verify_bound_tree(staging_root, "STAGE")
        state.stage = "8/8"
    except PublicFailure:
        return _clean_stage(staging_root, remover, state, "P41-STAGE-VERIFY-FAILURE")

    try:
        _sync_staging_directories(created_staging_directories, synchronizer, state)
    except Exception:
        return _clean_stage(staging_root, remover, state, "P41-STAGE-SYNC-FAILURE")

    state.rename_attempts = 1
    try:
        renamer(staging_root, final_before_publication)
    except Exception:
        stage_report = _clean_stage(staging_root, remover, state, "P41-RENAME-FAILURE")
        if not _lexists(final_before_publication):
            return stage_report
        return _final_failure(
            final_before_publication,
            final_parent,
            synchronizer,
            remover,
            state,
            "P41-RENAME-FAILURE",
        )

    del staging_root
    try:
        final_after_rename = _safe_absolute(final_root_value)
        if _lexists(final_after_rename) and os.path.islink(final_after_rename):
            raise PublicFailure("P41-FINAL-VERIFY-FAILURE")
        if post_rename is not None:
            post_rename(final_after_rename)
        verify_bound_tree(final_after_rename, "FINAL")
        state.final = "8/8"
    except Exception:
        return _final_failure(
            final_before_publication,
            final_parent,
            synchronizer,
            remover,
            state,
            "P41-FINAL-VERIFY-FAILURE",
        )

    try:
        state.final_parent_sync = _validate_sync_posture(
            synchronizer(final_after_rename.parent)
        )
    except Exception:
        state.final_parent_sync = _failed_sync()
        return _final_failure(
            final_after_rename,
            final_parent,
            synchronizer,
            remover,
            state,
            "P41-FINAL-SYNC-FAILURE",
        )
    return state.report("pass", None)


class PublicArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> None:
        del message
        raise PublicFailure("P41-ARGUMENT")


def arguments(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = PublicArgumentParser(description=__doc__)
    parser.add_argument("--source-root", required=True)
    parser.add_argument("--final-root", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        parsed = arguments(argv)
        result = copy_release(parsed.source_root, parsed.final_root)
    except PublicFailure as error:
        result = TransactionState().report("fail", error.code)
    except Exception:
        result = TransactionState().report("fail", "P41-INTERNAL-ERROR")
    print(public_json(result))
    return 0 if result["status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
