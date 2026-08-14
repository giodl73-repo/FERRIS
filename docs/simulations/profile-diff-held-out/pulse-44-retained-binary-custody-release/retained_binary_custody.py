#!/usr/bin/env python3
"""Freeze and publish one retained public binary and its receipt as one custody tree."""

from __future__ import annotations

import argparse
import errno
import hashlib
import importlib.util
import json
import os
import re
import stat
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Iterable


P33_CUTOFF = "29517d732db13cc2ffa304684b344f3538ab587d"
P33_MANIFEST_RAW_SHA256 = (
    "sha256:9082bb18ab72e5e5ced2ec43811ecc5ce14ac43c9cd2878e4db0f10bf7a741fd"
)
P33_MANIFEST_AGGREGATE = (
    "sha256:07df7bc02cab288adccbdc0f87e45f3fd52939ebc9d22c8d064f773843e861a4"
)
P33_BUILD_FREEZE_SHA256 = (
    "sha256:43bb31210175ceacba2431a238608d9973672a08de57572543ad0f9dae41cbe6"
)
P33_RELEASE_FILE_COUNT = 37
P33_RELEASE_TOTAL_BYTES = 59_895
PLATFORMS = frozenset({"ubuntu-24.04-x86_64", "windows-x86_64"})
EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
SUMMARY_SCHEMA = "ferris.pulse-44-retained-binary-custody-summary/v1"
SYNC_MECHANISM = "os.open+os.fsync-directory-v1"
GATE_ID = "retained-binary-custody"
SHA256 = re.compile(r"^sha256:[0-9a-f]{64}$")


class CustodyFailure(Exception):
    """A bounded, path-free public failure classification."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass(frozen=True)
class Binding:
    name: str
    size: int
    sha256: str


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


def _not_attempted_sync() -> SyncPosture:
    return SyncPosture("not-attempted", "not-attempted", "not-attempted", False)


def _failed_sync() -> SyncPosture:
    return SyncPosture("failed", SYNC_MECHANISM, "sync-operation-failed")


def canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return f"sha256:{hashlib.sha256(value).hexdigest()}"


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise CustodyFailure("P44-DUPLICATE-JSON-MEMBER")
        result[key] = value
    return result


def _exact_object(
    value: object, required: frozenset[str], code: str
) -> dict[str, object]:
    if type(value) is not dict or set(value) != required:
        raise CustodyFailure(code)
    return value


def _lexists(path: Path) -> bool:
    return os.path.lexists(path)


def _safe_regular_digest(path: Path, code: str) -> tuple[int, str]:
    try:
        before = os.lstat(path)
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
            raise CustodyFailure(code)
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except CustodyFailure:
        raise
    except OSError as error:
        raise CustodyFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (before.st_dev, before.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise CustodyFailure(code)
        digest = hashlib.sha256()
        size = 0
        while chunk := os.read(descriptor, 65_536):
            digest.update(chunk)
            size += len(chunk)
    finally:
        os.close(descriptor)
    return size, f"sha256:{digest.hexdigest()}"


def _safe_regular_bytes(path: Path, code: str, maximum: int = 65_536) -> bytes:
    size, _ = _safe_regular_digest(path, code)
    if size > maximum:
        raise CustodyFailure(code)
    try:
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except OSError as error:
        raise CustodyFailure(code) from error
    try:
        content = b""
        while chunk := os.read(descriptor, 65_536):
            content += chunk
    finally:
        os.close(descriptor)
    if len(content) != size:
        raise CustodyFailure(code)
    return content


def _read_duplicate_free_json(path: Path, code: str) -> dict[str, object]:
    try:
        value = json.loads(_safe_regular_bytes(path, code), object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, CustodyFailure) as error:
        raise CustodyFailure(code) from error
    if type(value) is not dict:
        raise CustodyFailure(code)
    return value


def _validated_absolute_fresh_root(value: str | os.PathLike[str]) -> tuple[Path, Path]:
    try:
        raw = os.fspath(value)
    except TypeError as error:
        raise CustodyFailure("P44-ROOT-INVALID") from error
    if type(raw) is not str or "\x00" in raw:
        raise CustodyFailure("P44-ROOT-INVALID")
    requested = Path(raw)
    if not requested.is_absolute() or ".." in requested.parts:
        raise CustodyFailure("P44-ROOT-INVALID")
    if _lexists(requested):
        raise CustodyFailure("P44-ROOT-EXISTS")
    parent = requested.parent
    probe = parent
    try:
        while True:
            metadata = os.lstat(probe)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise CustodyFailure("P44-ROOT-PARENT-UNSAFE")
            if probe == probe.parent:
                break
            probe = probe.parent
        resolved = requested.resolve(strict=False)
    except CustodyFailure:
        raise
    except OSError as error:
        raise CustodyFailure("P44-ROOT-PARENT-UNSAFE") from error
    if _lexists(resolved):
        raise CustodyFailure("P44-ROOT-EXISTS")
    return resolved, resolved.parent


def _roots_are_separate(first: Path, second: Path) -> bool:
    try:
        first.relative_to(second)
        return False
    except ValueError:
        try:
            second.relative_to(first)
            return False
        except ValueError:
            return True


def _directory_error_category(error: OSError) -> str:
    unsupported = {
        errno.EACCES,
        errno.EINVAL,
        errno.EPERM,
        getattr(errno, "ENOTSUP", errno.EINVAL),
        getattr(errno, "EOPNOTSUPP", errno.EINVAL),
    }
    return "unsupported-by-platform-or-filesystem" if error.errno in unsupported else "os-error"


def sync_directory(path: Path) -> SyncPosture:
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_DIRECTORY", 0)
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        category = _directory_error_category(error)
        if category == "os-error":
            raise
        return SyncPosture("unsupported", SYNC_MECHANISM, category)
    try:
        os.fsync(descriptor)
    except OSError as error:
        category = _directory_error_category(error)
        if category == "os-error":
            raise
        return SyncPosture("unsupported", SYNC_MECHANISM, category)
    finally:
        os.close(descriptor)
    return SyncPosture("synced", SYNC_MECHANISM, None)


def _validated_sync(value: SyncPosture) -> SyncPosture:
    if (
        not isinstance(value, SyncPosture)
        or not value.attempted
        or value.status not in {"synced", "unsupported"}
        or (value.status == "synced" and value.error_category is not None)
        or (value.status == "unsupported" and value.error_category is None)
    ):
        raise CustodyFailure("P44-INVALID-SYNC-POSTURE")
    return value


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


def _p33_release_root() -> Path:
    return Path(__file__).resolve().parent.parent / "pulse-33-build-freeze-release"


def _verify_p33_identity(release: Path) -> Path:
    manifest_path = release / "public-manifest.json"
    manifest_bytes = _safe_regular_bytes(manifest_path, "P44-P33-IDENTITY")
    if sha256_bytes(manifest_bytes) != P33_MANIFEST_RAW_SHA256:
        raise CustodyFailure("P44-P33-IDENTITY")
    try:
        manifest = json.loads(manifest_bytes, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, CustodyFailure) as error:
        raise CustodyFailure("P44-P33-IDENTITY") from error
    if (
        type(manifest) is not dict
        or manifest.get("schema") != "ferris.pulse-33-public-build-freeze-manifest/v1"
        or manifest.get("cutoff") != P33_CUTOFF
        or manifest.get("aggregate") != P33_MANIFEST_AGGREGATE
        or manifest.get("file_count") != P33_RELEASE_FILE_COUNT
        or manifest.get("total_bytes") != P33_RELEASE_TOTAL_BYTES
    ):
        raise CustodyFailure("P44-P33-IDENTITY")
    source = release / "build_freeze.py"
    if _safe_regular_digest(source, "P44-P33-IDENTITY")[1] != P33_BUILD_FREEZE_SHA256:
        raise CustodyFailure("P44-P33-IDENTITY")
    return source


def _pulse33_builder(
    repo: Path,
    cutoff: str,
    platform: str,
    output: Path,
    *,
    retain_executable: bool,
) -> dict[str, object]:
    source = _verify_p33_identity(_p33_release_root())
    specification = importlib.util.spec_from_file_location("pulse_33_build_freeze", source)
    if specification is None or specification.loader is None:
        raise CustodyFailure("P44-P33-IDENTITY")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module.build_and_freeze(
        repo, cutoff, platform, output, retain_executable=retain_executable
    )


def _logical_filename(platform: str, cutoff: str) -> str:
    if platform not in PLATFORMS or cutoff != P33_CUTOFF:
        raise CustodyFailure("P44-BUILD-IDENTITY")
    suffix = ".exe" if platform == "windows-x86_64" else ""
    return f"ferris-{platform}-{cutoff}{suffix}"


def _verify_build_receipt(
    executable: Path, receipt_path: Path, cutoff: str, platform: str, code: str
) -> tuple[Binding, Binding]:
    filename = _logical_filename(platform, cutoff)
    executable_size, executable_sha256 = _safe_regular_digest(executable, code)
    receipt_bytes = _safe_regular_bytes(receipt_path, code)
    try:
        receipt = json.loads(receipt_bytes, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, CustodyFailure) as error:
        raise CustodyFailure(code) from error
    envelope = _exact_object(
        receipt, frozenset({"payload", "payload_sha256", "schema"}), code
    )
    payload = _exact_object(
        envelope["payload"],
        frozenset({"artifact", "build", "checkout", "cutoff", "platform", "safety", "schema"}),
        code,
    )
    if (
        envelope["schema"] != "ferris.public-build-freeze-envelope/v1"
        or payload["schema"] != "ferris.public-build-freeze-receipt/v1"
        or envelope["payload_sha256"] != sha256_bytes(canonical_bytes(payload))
        or payload["cutoff"] != cutoff
        or payload["platform"] != platform
    ):
        raise CustodyFailure(code)
    artifact = _exact_object(
        payload["artifact"],
        frozenset(
            {"discovery", "logical_filename", "retained_in_public_bundle", "sha256", "size"}
        ),
        code,
    )
    if (
        artifact["discovery"] != "cargo-compiler-artifact-json"
        or artifact["logical_filename"] != filename
        or artifact["retained_in_public_bundle"] is not True
        or artifact["size"] != executable_size
        or artifact["sha256"] != executable_sha256
        or not isinstance(artifact["size"], int)
        or not isinstance(artifact["sha256"], str)
        or SHA256.fullmatch(artifact["sha256"]) is None
    ):
        raise CustodyFailure(code)
    checkout = _exact_object(
        payload["checkout"],
        frozenset({"core_autocrlf", "exact_commit", "tracked_files_clean"}),
        code,
    )
    if checkout != {
        "core_autocrlf": False,
        "exact_commit": True,
        "tracked_files_clean": True,
    }:
        raise CustodyFailure(code)
    build = payload["build"]
    safety = payload["safety"]
    if (
        type(build) is not dict
        or build.get("profile") != "release"
        or build.get("package") != "ferris-cli"
        or build.get("binary") != "ferris"
        or type(build.get("command")) is not list
        or "--locked" not in build["command"]
        or "--release" not in build["command"]
        or safety != {"diagnostic_execution": False, "product_files_modified": False}
    ):
        raise CustodyFailure(code)
    return (
        Binding(filename, executable_size, executable_sha256),
        Binding(receipt_path.name, len(receipt_bytes), sha256_bytes(receipt_bytes)),
    )


def verify_build_output(root: Path, cutoff: str, platform: str, code: str) -> tuple[Binding, Binding]:
    filename = _logical_filename(platform, cutoff)
    receipt_name = f"{filename}.receipt.json"
    try:
        metadata = os.lstat(root)
        entries = sorted(os.scandir(root), key=lambda entry: entry.name)
    except OSError as error:
        raise CustodyFailure(code) from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        raise CustodyFailure(code)
    if [entry.name for entry in entries] != [filename, receipt_name]:
        raise CustodyFailure(code)
    return _verify_build_receipt(
        root / filename, root / receipt_name, cutoff, platform, code
    )


def _copy_fsynced(source: Path, destination: Path, file_synchronizer: Callable[[int], None]) -> None:
    source_descriptor: int | None = None
    destination_descriptor: int | None = None
    try:
        source_descriptor = os.open(
            source, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
        source_metadata = os.fstat(source_descriptor)
        if not stat.S_ISREG(source_metadata.st_mode):
            raise CustodyFailure("P44-STAGE-COPY-FAILURE")
        destination_descriptor = os.open(
            destination,
            os.O_WRONLY
            | os.O_CREAT
            | os.O_EXCL
            | getattr(os, "O_BINARY", 0)
            | getattr(os, "O_NOFOLLOW", 0),
            0o600,
        )
        while chunk := os.read(source_descriptor, 65_536):
            offset = 0
            while offset < len(chunk):
                written = os.write(destination_descriptor, chunk[offset:])
                if written <= 0:
                    raise OSError("short custody write")
                offset += written
        file_synchronizer(destination_descriptor)
    except CustodyFailure:
        raise
    except Exception as error:
        raise CustodyFailure("P44-STAGE-COPY-FAILURE") from error
    finally:
        for descriptor in (destination_descriptor, source_descriptor):
            if descriptor is not None:
                os.close(descriptor)


@dataclass
class CustodyState:
    rename_attempts: int = 0
    retries: int = 0
    work_verified: str = "0/2"
    stage_verified: str = "0/2"
    final_verified: str = "0/2"
    stage_sync: SyncPosture = field(default_factory=_not_attempted_sync)
    final_parent_sync: SyncPosture = field(default_factory=_not_attempted_sync)
    rollback_parent_sync: SyncPosture = field(default_factory=_not_attempted_sync)

    def failure(self, code: str, posture: str) -> dict[str, object]:
        return {
            "custody": {
                "final_files_present": False,
                "final_verified": self.final_verified,
                "rename_attempts": self.rename_attempts,
                "retries": self.retries,
                "stage_verified": self.stage_verified,
                "state": posture,
                "sync": {
                    "final_parent": self.final_parent_sync.public(),
                    "rollback_parent": self.rollback_parent_sync.public(),
                    "stage": self.stage_sync.public(),
                },
                "work_verified": self.work_verified,
            },
            "failure_code": code,
            "ordered_execution_event": _terminal_event("failed"),
            "outcome": "failed",
            "schema": SUMMARY_SCHEMA,
        }


def _terminal_event(outcome: str) -> dict[str, str]:
    return {
        "classification": "ordered-execution",
        "event_kind": "terminal-stop",
        "gate_id": GATE_ID,
        "outcome": outcome,
        "schema": EVENT_SCHEMA,
    }


def _remove_and_absent(paths: Iterable[Path], remover: Callable[[Path], None]) -> bool:
    try:
        for path in paths:
            remover(path)
        return all(not _lexists(path) for path in paths)
    except Exception:
        return False


def _pre_final_failure(
    stage: Path, work_root: Path, remover: Callable[[Path], None], state: CustodyState, code: str
) -> dict[str, object]:
    if not _remove_and_absent((stage, work_root), remover):
        return state.failure("P44-INDETERMINATE-PUBLICATION", "indeterminate")
    return state.failure(code, "absent")


def _rollback_final(
    final_root: Path,
    stage: Path,
    work_root: Path,
    final_parent: Path,
    synchronizer: Callable[[Path], SyncPosture],
    remover: Callable[[Path], None],
    state: CustodyState,
    code: str,
) -> dict[str, object]:
    if not _remove_and_absent((stage, work_root, final_root), remover):
        return state.failure("P44-INDETERMINATE-PUBLICATION", "indeterminate")
    try:
        state.rollback_parent_sync = _validated_sync(synchronizer(final_parent))
    except Exception:
        state.rollback_parent_sync = _failed_sync()
        return state.failure("P44-INDETERMINATE-PUBLICATION", "indeterminate")
    return state.failure(code, "rolled-back")


Builder = Callable[..., dict[str, object]]
Copier = Callable[[Path, Path], None]
Synchronizer = Callable[[Path], SyncPosture]
Renamer = Callable[[Path, Path], None]
Remover = Callable[[Path], None]
EventEmitter = Callable[[dict[str, str]], None]


def retain_binary_custody(
    repo: Path,
    cutoff: str,
    platform: str,
    work_root_value: str | os.PathLike[str],
    final_root_value: str | os.PathLike[str],
    *,
    builder: Builder = _pulse33_builder,
    copier: Copier | None = None,
    file_synchronizer: Callable[[int], None] = os.fsync,
    synchronizer: Synchronizer = sync_directory,
    renamer: Renamer = os.replace,
    remover: Remover = remove_tree,
    post_rename: Callable[[Path], None] | None = None,
    event_emitter: EventEmitter | None = None,
) -> dict[str, object]:
    """Call Pulse 33 exactly once, then custody its retained pair with one rename."""

    state = CustodyState()
    stage: Path | None = None
    try:
        if cutoff != P33_CUTOFF or platform not in PLATFORMS:
            raise CustodyFailure("P44-BUILD-IDENTITY")
        work_root, _ = _validated_absolute_fresh_root(work_root_value)
        final_root, final_parent = _validated_absolute_fresh_root(final_root_value)
        if not _roots_are_separate(work_root, final_root):
            raise CustodyFailure("P44-ROOTS-OVERLAP")
        stage = final_parent / f".{final_root.name}.pulse-44-stage"
        if _lexists(stage):
            raise CustodyFailure("P44-STAGING-EXISTS")
        _verify_p33_identity(_p33_release_root())
    except CustodyFailure as error:
        return state.failure(error.code, "absent")

    try:
        builder(repo, cutoff, platform, work_root, retain_executable=True)
    except Exception:
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-BUILD-FREEZE-FAILURE"
        )

    try:
        bindings = verify_build_output(work_root, cutoff, platform, "P44-WORK-VERIFY-FAILURE")
        state.work_verified = "2/2"
    except CustodyFailure:
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-WORK-VERIFY-FAILURE"
        )

    active_copier = copier
    if active_copier is None:
        active_copier = lambda source, destination: _copy_fsynced(
            source, destination, file_synchronizer
        )
    try:
        os.mkdir(stage)
        for binding in bindings:
            active_copier(work_root / binding.name, stage / binding.name)
    except Exception:
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-STAGE-COPY-FAILURE"
        )

    try:
        staged_bindings = verify_build_output(stage, cutoff, platform, "P44-STAGE-VERIFY-FAILURE")
        if staged_bindings != bindings:
            raise CustodyFailure("P44-STAGE-VERIFY-FAILURE")
        state.stage_verified = "2/2"
    except CustodyFailure:
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-STAGE-VERIFY-FAILURE"
        )

    if not _remove_and_absent((work_root,), remover):
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-WORK-CLEANUP-FAILURE"
        )
    try:
        state.stage_sync = _validated_sync(synchronizer(stage))
    except Exception:
        state.stage_sync = _failed_sync()
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-STAGE-SYNC-FAILURE"
        )

    state.rename_attempts = 1
    try:
        renamer(stage, final_root)
    except Exception:
        if _lexists(final_root):
            return _rollback_final(
                final_root, stage, work_root, final_parent, synchronizer, remover, state,
                "P44-RENAME-FAILURE",
            )
        return _pre_final_failure(
            stage, work_root, remover, state, "P44-RENAME-FAILURE"
        )

    if _lexists(stage):
        return _rollback_final(
            final_root, stage, work_root, final_parent, synchronizer, remover, state,
            "P44-FINAL-VERIFY-FAILURE",
        )
    try:
        reconstructed_final = Path(os.fspath(final_root_value)).resolve(strict=True)
        final_bindings = verify_build_output(
            reconstructed_final, cutoff, platform, "P44-FINAL-VERIFY-FAILURE"
        )
        if final_bindings != bindings:
            raise CustodyFailure("P44-FINAL-VERIFY-FAILURE")
        if post_rename is not None:
            post_rename(reconstructed_final)
            final_bindings = verify_build_output(
                reconstructed_final, cutoff, platform, "P44-FINAL-VERIFY-FAILURE"
            )
            if final_bindings != bindings:
                raise CustodyFailure("P44-FINAL-VERIFY-FAILURE")
        state.final_verified = "2/2"
    except Exception:
        return _rollback_final(
            final_root, stage, work_root, final_parent, synchronizer, remover, state,
            "P44-FINAL-VERIFY-FAILURE",
        )

    try:
        state.final_parent_sync = _validated_sync(synchronizer(final_parent))
    except Exception:
        state.final_parent_sync = _failed_sync()
        return _rollback_final(
            final_root, stage, work_root, final_parent, synchronizer, remover, state,
            "P44-FINAL-SYNC-FAILURE",
        )

    event = _terminal_event("completed")
    try:
        if event_emitter is not None:
            event_emitter(event)
    except Exception:
        return _rollback_final(
            final_root, stage, work_root, final_parent, synchronizer, remover, state,
            "P44-EVENT-EMIT-FAILURE",
        )
    return {
        "custody": {
            "final_files_present": True,
            "final_verified": state.final_verified,
            "files": "2/2",
            "rename_attempts": state.rename_attempts,
            "retries": state.retries,
            "stage_verified": state.stage_verified,
            "state": "published",
            "sync": {
                "final_parent": state.final_parent_sync.public(),
                "rollback_parent": state.rollback_parent_sync.public(),
                "stage": state.stage_sync.public(),
            },
            "work_verified": state.work_verified,
        },
        "ordered_execution_event": event,
        "outcome": "published",
        "schema": SUMMARY_SCHEMA,
    }


class PublicArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> None:
        del message
        raise CustodyFailure("P44-ARGUMENT")


def arguments(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = PublicArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True, type=Path)
    parser.add_argument("--cutoff", required=True)
    parser.add_argument("--platform", required=True, choices=sorted(PLATFORMS))
    parser.add_argument("--work-root", required=True)
    parser.add_argument("--final-root", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        parsed = arguments(argv)
        result = retain_binary_custody(
            parsed.repo,
            parsed.cutoff,
            parsed.platform,
            parsed.work_root,
            parsed.final_root,
        )
    except CustodyFailure as error:
        result = CustodyState().failure(error.code, "absent")
    print(canonical_bytes(result).decode("ascii"))
    return 0 if result["outcome"] == "published" else 1


if __name__ == "__main__":
    raise SystemExit(main())
