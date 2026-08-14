#!/usr/bin/env python3
"""Publish one complete, ordered public result with zero retries."""

from __future__ import annotations

import argparse
import errno
import hashlib
import json
import os
import re
import stat
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Iterable


CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
RESULT_SCHEMA = "ferris.pulse-43-ordered-public-result/v1"
RESULT_ENVELOPE_SCHEMA = "ferris.pulse-43-ordered-public-result-envelope/v1"
RECEIPT_SCHEMA = "ferris.pulse-43-ordered-result-receipt/v1"
RECEIPT_ENVELOPE_SCHEMA = "ferris.pulse-43-ordered-result-receipt-envelope/v1"
SUMMARY_SCHEMA = "ferris.pulse-43-ordered-result-publication-summary/v1"
SYNC_MECHANISM = "os.open+os.fsync-directory-v1"
MAX_CATALOG_GATES = 24
MAX_EVENTS = 128
MAX_VALIDATION_COUNT = 1_000_000
MAX_INPUT_BYTES = 65_536
IDENTIFIER = re.compile(r"^[a-z][a-z0-9-]{0,47}$")
PRIVACY_BEARING_IDENTIFIER_PARTS = frozenset(
    {
        "candidate",
        "corpus",
        "credential",
        "home",
        "password",
        "private",
        "seed",
        "secret",
        "token",
        "user",
        "workspace",
    }
)
OUTPUT_FILES = ("public-result.json", "release-receipt.json")


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


def _not_attempted_sync() -> SyncPosture:
    return SyncPosture(
        "not-attempted", "not-attempted", "not-attempted", attempted=False
    )


def _failed_sync() -> SyncPosture:
    return SyncPosture("failed", SYNC_MECHANISM, "sync-operation-failed")


def canonical_bytes(value: object) -> bytes:
    """Return the sole canonical JSON representation used for payload hashes."""

    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return f"sha256:{hashlib.sha256(value).hexdigest()}"


def public_json(value: object) -> str:
    return canonical_bytes(value).decode("ascii")


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise PublicFailure("P43-DUPLICATE-JSON-MEMBER")
        result[key] = value
    return result


def load_public_json(path: str | os.PathLike[str]) -> object:
    """Load one bounded, duplicate-free JSON public control without echoing its path."""

    try:
        with open(path, "rb") as handle:
            raw = handle.read(MAX_INPUT_BYTES + 1)
    except OSError as error:
        raise PublicFailure("P43-PUBLIC-INPUT-UNAVAILABLE") from error
    if len(raw) > MAX_INPUT_BYTES:
        raise PublicFailure("P43-PUBLIC-INPUT-TOO-LARGE")
    try:
        text = raw.decode("utf-8")
        return json.loads(text, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PublicFailure("P43-PUBLIC-INPUT-INVALID") from error


def _exact_object(
    value: object, required: frozenset[str], code: str
) -> dict[str, object]:
    if type(value) is not dict or set(value) != required:
        raise PublicFailure(code)
    return value


def _public_identifier(value: object, code: str) -> str:
    if type(value) is not str or IDENTIFIER.fullmatch(value) is None:
        raise PublicFailure(code)
    if PRIVACY_BEARING_IDENTIFIER_PARTS.intersection(value.split("-")):
        raise PublicFailure("P43-PRIVACY-BEARING-IDENTIFIER")
    return value


def _bounded_count(value: object, code: str) -> int:
    if type(value) is not int or value < 0 or value > MAX_VALIDATION_COUNT:
        raise PublicFailure(code)
    return value


def validate_catalog(value: object) -> tuple[str, ...]:
    catalog = _exact_object(value, frozenset({"schema", "gate_ids"}), "P43-CATALOG-SCHEMA")
    if catalog["schema"] != CATALOG_SCHEMA:
        raise PublicFailure("P43-CATALOG-SCHEMA")
    gate_ids = catalog["gate_ids"]
    if type(gate_ids) is not list or not 1 <= len(gate_ids) <= MAX_CATALOG_GATES:
        raise PublicFailure("P43-CATALOG-CARDINALITY")
    normalized = tuple(_public_identifier(gate, "P43-CATALOG-GATE-ID") for gate in gate_ids)
    if len(set(normalized)) != len(normalized):
        raise PublicFailure("P43-DUPLICATE-CATALOG-GATE")
    return normalized


def _validation_event(
    event: object, validation_ids: set[str], summary: dict[str, int]
) -> None:
    record = _exact_object(
        event,
        frozenset(
            {
                "classification",
                "completed_checks",
                "event_kind",
                "expected_checks",
                "schema",
                "validation_id",
            }
        ),
        "P43-EVENT-SCHEMA",
    )
    if (
        record["schema"] != EVENT_SCHEMA
        or record["classification"] != "public-artifact-self-validation"
        or record["event_kind"] != "validation-complete"
    ):
        raise PublicFailure("P43-EVENT-SCHEMA")
    validation_id = _public_identifier(record["validation_id"], "P43-VALIDATION-ID")
    if validation_id in validation_ids:
        raise PublicFailure("P43-DUPLICATE-VALIDATION")
    completed = _bounded_count(record["completed_checks"], "P43-VALIDATION-COUNT")
    expected = _bounded_count(record["expected_checks"], "P43-VALIDATION-COUNT")
    if completed > expected:
        raise PublicFailure("P43-VALIDATION-COUNT")
    validation_ids.add(validation_id)
    summary["event_count"] += 1
    summary["completed_checks"] += completed
    summary["expected_checks"] += expected


def _execution_event(
    event: object,
    catalog: tuple[str, ...],
    seen: list[str],
    terminal: dict[str, str] | None,
) -> dict[str, str] | None:
    record = _exact_object(
        event,
        frozenset({"classification", "event_kind", "gate_id", "outcome", "schema"}),
        "P43-EVENT-SCHEMA",
    )
    if record["schema"] != EVENT_SCHEMA or record["classification"] != "ordered-execution":
        raise PublicFailure("P43-EVENT-SCHEMA")
    if terminal is not None:
        raise PublicFailure("P43-ORDERED-AFTER-TERMINAL")
    gate_id = _public_identifier(record["gate_id"], "P43-EXECUTION-GATE-ID")
    if gate_id not in catalog:
        raise PublicFailure("P43-UNKNOWN-EXECUTION-GATE")
    if gate_id in seen:
        raise PublicFailure("P43-DUPLICATE-EXECUTION-GATE")
    if len(seen) >= len(catalog) or gate_id != catalog[len(seen)]:
        raise PublicFailure("P43-MISSING-PRIOR-EXECUTION-GATE")

    event_kind = record["event_kind"]
    outcome = record["outcome"]
    if event_kind == "gate-complete":
        if outcome != "passed":
            raise PublicFailure("P43-EVENT-SCHEMA")
        seen.append(gate_id)
        return None
    if event_kind != "terminal-stop" or outcome not in {"completed", "failed", "stopped"}:
        raise PublicFailure("P43-EVENT-SCHEMA")
    if outcome == "completed" and len(seen) != len(catalog) - 1:
        raise PublicFailure("P43-INCOMPLETE-CATALOG")
    seen.append(gate_id)
    return {"event_kind": event_kind, "gate_id": gate_id, "outcome": outcome}


def validate_events(catalog: tuple[str, ...], value: object) -> dict[str, object]:
    if type(value) is not list or not 1 <= len(value) <= MAX_EVENTS:
        raise PublicFailure("P43-EVENT-CARDINALITY")
    validation_summary = {
        "completed_checks": 0,
        "event_count": 0,
        "expected_checks": 0,
    }
    validation_ids: set[str] = set()
    seen: list[str] = []
    terminal: dict[str, str] | None = None
    for event in value:
        if type(event) is not dict:
            raise PublicFailure("P43-EVENT-SCHEMA")
        classification = event.get("classification")
        if classification == "public-artifact-self-validation":
            _validation_event(event, validation_ids, validation_summary)
        elif classification == "ordered-execution":
            terminal = _execution_event(event, catalog, seen, terminal)
        else:
            raise PublicFailure("P43-EVENT-CLASSIFICATION")
    if terminal is None:
        raise PublicFailure("P43-MISSING-TERMINAL")
    completed = len(seen) if terminal["outcome"] == "completed" else len(seen) - 1
    return {
        "ordered_execution": {
            "attempted_gate_count": len(seen),
            "catalog_gate_count": len(catalog),
            "completed_gate_count": completed,
            "terminal": terminal,
        },
        "public_self_validation": {
            **validation_summary,
            "all_declared_checks_completed": (
                validation_summary["completed_checks"] == validation_summary["expected_checks"]
            ),
        },
    }


def build_result(catalog_value: object, events_value: object) -> tuple[dict[str, object], bytes]:
    catalog = validate_catalog(catalog_value)
    execution = validate_events(catalog, events_value)
    payload: dict[str, object] = {
        "catalog": {"gate_ids": list(catalog), "schema": CATALOG_SCHEMA},
        "ordered_execution": execution["ordered_execution"],
        "privacy": {
            "diagnostic_authority": False,
            "paths_included": False,
            "private_data_included": False,
        },
        "public_self_validation": execution["public_self_validation"],
        "release_limits": {
            "fallbacks": 0,
            "product_conclusion": None,
            "retries": 0,
        },
        "schema": RESULT_SCHEMA,
    }
    envelope = {
        "payload": payload,
        "payload_sha256": sha256_bytes(canonical_bytes(payload)),
        "schema": RESULT_ENVELOPE_SCHEMA,
    }
    return payload, canonical_bytes(envelope) + b"\n"


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


def _validate_sync_posture(value: SyncPosture) -> SyncPosture:
    if (
        not isinstance(value, SyncPosture)
        or not value.attempted
        or value.status not in {"synced", "unsupported"}
        or (value.status == "synced" and value.error_category is not None)
        or (value.status == "unsupported" and value.error_category is None)
    ):
        raise PublicFailure("P43-INVALID-SYNC-POSTURE")
    return value


def _lexists(path: Path) -> bool:
    return os.path.lexists(path)


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


def _absolute_final_root(value: str | os.PathLike[str]) -> tuple[Path, Path]:
    try:
        raw = os.fspath(value)
    except TypeError as error:
        raise PublicFailure("P43-FINAL-ROOT-INVALID") from error
    if type(raw) is not str or "\x00" in raw:
        raise PublicFailure("P43-FINAL-ROOT-INVALID")
    requested = Path(raw)
    if not requested.is_absolute() or ".." in requested.parts:
        raise PublicFailure("P43-FINAL-ROOT-INVALID")
    try:
        final_root = requested.resolve(strict=False)
        parent_metadata = os.lstat(requested.parent)
    except OSError as error:
        raise PublicFailure("P43-FINAL-PARENT-UNSAFE") from error
    if stat.S_ISLNK(parent_metadata.st_mode) or not stat.S_ISDIR(parent_metadata.st_mode):
        raise PublicFailure("P43-FINAL-PARENT-UNSAFE")
    if _lexists(requested) or _lexists(final_root):
        raise PublicFailure("P43-FINAL-EXISTS")
    return final_root, final_root.parent


def _write_fsynced(path: Path, content: bytes) -> None:
    descriptor: int | None = None
    try:
        descriptor = os.open(
            path,
            os.O_WRONLY
            | os.O_CREAT
            | os.O_EXCL
            | getattr(os, "O_BINARY", 0)
            | getattr(os, "O_NOFOLLOW", 0),
            0o600,
        )
        offset = 0
        while offset < len(content):
            written = os.write(descriptor, content[offset:])
            if written <= 0:
                raise OSError("short public record write")
            offset += written
        os.fsync(descriptor)
    finally:
        if descriptor is not None:
            os.close(descriptor)


def _read_regular(path: Path) -> bytes:
    try:
        before = os.lstat(path)
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
            raise PublicFailure("P43-OUTPUT-TREE-UNSAFE")
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except PublicFailure:
        raise
    except OSError as error:
        raise PublicFailure("P43-OUTPUT-TREE-UNSAFE") from error
    try:
        after = os.fstat(descriptor)
        if not stat.S_ISREG(after.st_mode) or (before.st_dev, before.st_ino) != (
            after.st_dev,
            after.st_ino,
        ):
            raise PublicFailure("P43-OUTPUT-TREE-UNSAFE")
        chunks: list[bytes] = []
        while chunk := os.read(descriptor, 65536):
            chunks.append(chunk)
        return b"".join(chunks)
    finally:
        os.close(descriptor)


def _read_envelope(content: bytes, schema: str) -> dict[str, object]:
    if b"\r" in content or not content.endswith(b"\n"):
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    try:
        value = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (json.JSONDecodeError, PublicFailure) as error:
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE") from error
    envelope = _exact_object(
        value, frozenset({"payload", "payload_sha256", "schema"}), "P43-OUTPUT-VERIFY-FAILURE"
    )
    if envelope["schema"] != schema or type(envelope["payload"]) is not dict:
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    if envelope["payload_sha256"] != sha256_bytes(canonical_bytes(envelope["payload"])):
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    return envelope


def verify_publication_directory(root: Path) -> dict[str, str]:
    try:
        metadata = os.lstat(root)
        entries = sorted(os.scandir(root), key=lambda entry: entry.name)
    except OSError as error:
        raise PublicFailure("P43-FINAL-DIRECTORY-MISSING") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        raise PublicFailure("P43-FINAL-DIRECTORY-MISSING")
    if [entry.name for entry in entries] != list(OUTPUT_FILES):
        raise PublicFailure("P43-OUTPUT-TREE-UNSAFE")
    contents = {entry.name: _read_regular(Path(entry.path)) for entry in entries}
    result = _read_envelope(contents["public-result.json"], RESULT_ENVELOPE_SCHEMA)
    receipt = _read_envelope(contents["release-receipt.json"], RECEIPT_ENVELOPE_SCHEMA)
    receipt_payload = _exact_object(
        receipt["payload"],
        frozenset(
            {
                "publication",
                "result_payload_sha256",
                "result_raw_sha256",
                "schema",
            }
        ),
        "P43-OUTPUT-VERIFY-FAILURE",
    )
    if receipt_payload["schema"] != RECEIPT_SCHEMA:
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    if receipt_payload["result_raw_sha256"] != sha256_bytes(contents["public-result.json"]):
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    if receipt_payload["result_payload_sha256"] != result["payload_sha256"]:
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    publication = _exact_object(
        receipt_payload["publication"],
        frozenset(
            {
                "file_count",
                "file_fsync",
                "rename_attempts",
                "retries",
                "transaction",
            }
        ),
        "P43-OUTPUT-VERIFY-FAILURE",
    )
    if publication != {
        "file_count": 2,
        "file_fsync": "2/2-os.fsync-before-close",
        "rename_attempts": 1,
        "retries": 0,
        "transaction": "absent-staged-verified-one-rename-final-verified",
    }:
        raise PublicFailure("P43-OUTPUT-VERIFY-FAILURE")
    return {
        "receipt_payload_sha256": receipt["payload_sha256"],
        "receipt_raw_sha256": sha256_bytes(contents["release-receipt.json"]),
        "result_payload_sha256": result["payload_sha256"],
        "result_raw_sha256": sha256_bytes(contents["public-result.json"]),
    }


@dataclass
class PublicationState:
    rename_attempts: int = 0
    retries: int = 0
    stage_sync: SyncPosture = field(default_factory=_not_attempted_sync)
    final_parent_sync: SyncPosture = field(default_factory=_not_attempted_sync)
    rollback_parent_sync: SyncPosture = field(default_factory=_not_attempted_sync)

    def failure(self, code: str, posture: str) -> dict[str, object]:
        return {
            "failure_code": code,
            "publication": {
                "final_files_present": False,
                "rename_attempts": self.rename_attempts,
                "retries": self.retries,
                "state": posture,
                "sync": {
                    "final_parent": self.final_parent_sync.public(),
                    "rollback_parent": self.rollback_parent_sync.public(),
                    "stage": self.stage_sync.public(),
                },
            },
            "schema": SUMMARY_SCHEMA,
        }


Writer = Callable[[Path, bytes], None]
Renamer = Callable[[Path, Path], None]
Remover = Callable[[Path], None]
Synchronizer = Callable[[Path], SyncPosture]
PostRename = Callable[[Path], None]


def _clean_stage(
    stage: Path, remover: Remover, state: PublicationState, code: str
) -> dict[str, object]:
    try:
        remover(stage)
    except Exception:
        return state.failure("P43-INDETERMINATE-PUBLICATION", "indeterminate")
    if _lexists(stage):
        return state.failure("P43-INDETERMINATE-PUBLICATION", "indeterminate")
    return state.failure(code, "absent")


def _rollback_final(
    final_root: Path,
    final_parent: Path,
    synchronizer: Synchronizer,
    remover: Remover,
    state: PublicationState,
    code: str,
) -> dict[str, object]:
    try:
        remover(final_root)
    except Exception:
        return state.failure("P43-INDETERMINATE-PUBLICATION", "indeterminate")
    if _lexists(final_root):
        return state.failure("P43-INDETERMINATE-PUBLICATION", "indeterminate")
    try:
        state.rollback_parent_sync = _validate_sync_posture(synchronizer(final_parent))
    except Exception:
        state.rollback_parent_sync = _failed_sync()
        return state.failure("P43-INDETERMINATE-PUBLICATION", "indeterminate")
    return state.failure(code, "rolled-back")


def publish_result(
    catalog_value: object,
    events_value: object,
    final_root_value: str | os.PathLike[str],
    *,
    writer: Writer = _write_fsynced,
    renamer: Renamer = os.replace,
    remover: Remover = remove_tree,
    synchronizer: Synchronizer = sync_directory,
    post_rename: PostRename | None = None,
) -> dict[str, object]:
    """Publish only a fully verified result directory; never retry or fall back."""

    state = PublicationState()
    try:
        result_payload, result_bytes = build_result(catalog_value, events_value)
        final_root, final_parent = _absolute_final_root(final_root_value)
    except PublicFailure as error:
        return state.failure(error.code, "absent")

    receipt_payload: dict[str, object] = {
        "publication": {
            "file_count": 2,
            "file_fsync": "2/2-os.fsync-before-close",
            "rename_attempts": 1,
            "retries": 0,
            "transaction": "absent-staged-verified-one-rename-final-verified",
        },
        "result_payload_sha256": sha256_bytes(canonical_bytes(result_payload)),
        "result_raw_sha256": sha256_bytes(result_bytes),
        "schema": RECEIPT_SCHEMA,
    }
    receipt_bytes = (
        canonical_bytes(
            {
                "payload": receipt_payload,
                "payload_sha256": sha256_bytes(canonical_bytes(receipt_payload)),
                "schema": RECEIPT_ENVELOPE_SCHEMA,
            }
        )
        + b"\n"
    )
    stage = final_parent / f".{final_root.name}.pulse-43-stage"
    if _lexists(stage):
        return state.failure("P43-STAGING-EXISTS", "absent")

    try:
        os.mkdir(stage)
        writer(stage / "public-result.json", result_bytes)
        writer(stage / "release-receipt.json", receipt_bytes)
        verify_publication_directory(stage)
    except Exception:
        return _clean_stage(stage, remover, state, "P43-STAGE-COPY-FAILURE")

    try:
        state.stage_sync = _validate_sync_posture(synchronizer(stage))
    except Exception:
        state.stage_sync = _failed_sync()
        return _clean_stage(stage, remover, state, "P43-STAGE-SYNC-FAILURE")

    state.rename_attempts = 1
    try:
        renamer(stage, final_root)
    except Exception:
        if _lexists(final_root):
            return _rollback_final(
                final_root,
                final_parent,
                synchronizer,
                remover,
                state,
                "P43-RENAME-FAILURE",
            )
        return _clean_stage(stage, remover, state, "P43-RENAME-FAILURE")

    if _lexists(stage):
        if not _lexists(final_root):
            return _clean_stage(stage, remover, state, "P43-FINAL-DIRECTORY-MISSING")
        return state.failure("P43-INDETERMINATE-PUBLICATION", "indeterminate")
    try:
        if post_rename is not None:
            post_rename(final_root)
        hashes = verify_publication_directory(final_root)
    except Exception:
        return _rollback_final(
            final_root,
            final_parent,
            synchronizer,
            remover,
            state,
            "P43-FINAL-VERIFY-FAILURE",
        )

    try:
        state.final_parent_sync = _validate_sync_posture(synchronizer(final_parent))
    except Exception:
        state.final_parent_sync = _failed_sync()
        return _rollback_final(
            final_root,
            final_parent,
            synchronizer,
            remover,
            state,
            "P43-FINAL-SYNC-FAILURE",
        )
    return {
        "ordered_execution": result_payload["ordered_execution"],
        "publication": {
            "final_files_present": True,
            "files": "2/2",
            "raw_hashes": hashes,
            "rename_attempts": state.rename_attempts,
            "retries": state.retries,
            "state": "published",
            "sync": {
                "final_parent": state.final_parent_sync.public(),
                "rollback_parent": state.rollback_parent_sync.public(),
                "stage": state.stage_sync.public(),
            },
        },
        "public_self_validation": result_payload["public_self_validation"],
        "schema": SUMMARY_SCHEMA,
    }


class PublicArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> None:
        del message
        raise PublicFailure("P43-ARGUMENT")


def arguments(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = PublicArgumentParser(description=__doc__)
    parser.add_argument("--catalog", required=True)
    parser.add_argument("--events", required=True)
    parser.add_argument("--final-root", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        parsed = arguments(argv)
        report = publish_result(
            load_public_json(parsed.catalog),
            load_public_json(parsed.events),
            parsed.final_root,
        )
    except PublicFailure as error:
        report = PublicationState().failure(error.code, "absent")
    print(public_json(report))
    return 0 if report["publication"]["state"] == "published" else 1


if __name__ == "__main__":
    raise SystemExit(main())
