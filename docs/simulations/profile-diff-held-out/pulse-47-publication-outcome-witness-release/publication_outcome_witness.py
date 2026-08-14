#!/usr/bin/env python3
"""Witness one complete public Pulse 43 publication outcome with zero retries."""

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
from typing import Callable, Iterable, NoReturn


P43_RELEASE_DIRECTORY = "pulse-43-ordered-result-publisher-release"
P43_MANIFEST_RAW_SHA256 = (
    "sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4"
)
P43_MANIFEST_AGGREGATE = (
    "sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346"
)
P43_RECEIPT_RAW_SHA256 = (
    "sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c"
)
P43_RECEIPT_PAYLOAD_SHA256 = (
    "sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2"
)
P43_SEAL_RAW_SHA256 = (
    "sha256:4445a0b181419b303c28f1d91e1700a594d2a040fbae1cd0dc526fa7fc8e8f05"
)
P43_SEAL_PAYLOAD_SHA256 = (
    "sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1"
)
P43_SOURCE_SHA256 = (
    "sha256:38ebc7ce84ae29c2ad20ada593d8baeb0352b59e7c48438c4a9c224a0ea4a6c6"
)
P43_SUMMARY_SCHEMA = "ferris.pulse-43-ordered-result-publication-summary/v1"
P43_IDENTIFIER = re.compile(r"^[a-z][a-z0-9-]{0,47}$")
P43_FAILURE_CODE = re.compile(r"^P43-[A-Z-]+$")

WITNESS_SCHEMA = "ferris.pulse-47-publication-outcome-witness/v1"
WITNESS_ENVELOPE_SCHEMA = "ferris.pulse-47-publication-outcome-witness-envelope/v1"
RECEIPT_SCHEMA = "ferris.pulse-47-publication-outcome-witness-receipt/v1"
RECEIPT_ENVELOPE_SCHEMA = (
    "ferris.pulse-47-publication-outcome-witness-receipt-envelope/v1"
)
SUMMARY_SCHEMA = "ferris.pulse-47-publication-outcome-witness-summary/v1"
SYNC_MECHANISM = "os.open+os.fsync-directory-v1"
OUTPUT_FILES = ("publication-witness.json", "release-receipt.json")


class WitnessFailure(Exception):
    """A bounded public Pulse 47 failure."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class SummaryMalformed(Exception):
    """A returned Pulse 43 value was not one complete closed summary."""


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
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return f"sha256:{hashlib.sha256(value).hexdigest()}"


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise WitnessFailure("P47-DUPLICATE-JSON-MEMBER")
        result[key] = value
    return result


def _exact_object(value: object, fields: frozenset[str]) -> dict[str, object]:
    if type(value) is not dict or set(value) != fields:
        raise SummaryMalformed()
    return value


def _exact_string(value: object) -> str:
    if type(value) is not str:
        raise SummaryMalformed()
    return value


def _exact_integer(value: object) -> int:
    if type(value) is not int:
        raise SummaryMalformed()
    return value


def _exact_boolean(value: object) -> bool:
    if type(value) is not bool:
        raise SummaryMalformed()
    return value


def _valid_digest(value: object) -> str:
    digest = _exact_string(value)
    if re.fullmatch(r"sha256:[0-9a-f]{64}", digest) is None:
        raise SummaryMalformed()
    return digest


def _valid_p43_identifier(value: object) -> str:
    identifier = _exact_string(value)
    if P43_IDENTIFIER.fullmatch(identifier) is None:
        raise SummaryMalformed()
    return identifier


def _valid_sync(value: object, allowed: frozenset[str] | None = None) -> dict[str, object]:
    record = _exact_object(
        value, frozenset({"attempted", "error_category", "mechanism", "status"})
    )
    attempted = _exact_boolean(record["attempted"])
    error_category = record["error_category"]
    if error_category is not None:
        error_category = _exact_string(error_category)
    mechanism = _exact_string(record["mechanism"])
    status = _exact_string(record["status"])
    expected = {
        "failed": (True, "sync-operation-failed", SYNC_MECHANISM),
        "not-attempted": (False, "not-attempted", "not-attempted"),
        "synced": (True, None, SYNC_MECHANISM),
        "unsupported": (
            True,
            "unsupported-by-platform-or-filesystem",
            SYNC_MECHANISM,
        ),
    }
    if status not in expected or (allowed is not None and status not in allowed):
        raise SummaryMalformed()
    if (attempted, error_category, mechanism) != expected[status]:
        raise SummaryMalformed()
    return record


def _valid_sync_set(value: object, *, published: bool) -> dict[str, object]:
    record = _exact_object(value, frozenset({"final_parent", "rollback_parent", "stage"}))
    if published:
        complete = frozenset({"synced", "unsupported"})
        _valid_sync(record["final_parent"], complete)
        _valid_sync(record["rollback_parent"], frozenset({"not-attempted"}))
        _valid_sync(record["stage"], complete)
    else:
        _valid_sync(record["final_parent"])
        _valid_sync(record["rollback_parent"])
        _valid_sync(record["stage"])
    return record


def _valid_p43_ordered(value: object) -> dict[str, object]:
    record = _exact_object(
        value,
        frozenset(
            {
                "attempted_gate_count",
                "catalog_gate_count",
                "completed_gate_count",
                "terminal",
            }
        ),
    )
    attempted = _exact_integer(record["attempted_gate_count"])
    catalog = _exact_integer(record["catalog_gate_count"])
    completed = _exact_integer(record["completed_gate_count"])
    terminal = _exact_object(
        record["terminal"], frozenset({"event_kind", "gate_id", "outcome"})
    )
    if (
        not 1 <= catalog <= 24
        or not 1 <= attempted <= catalog
        or not 0 <= completed <= attempted
        or terminal["event_kind"] != "terminal-stop"
    ):
        raise SummaryMalformed()
    _valid_p43_identifier(terminal["gate_id"])
    outcome = _exact_string(terminal["outcome"])
    if outcome == "completed":
        if attempted != catalog or completed != catalog:
            raise SummaryMalformed()
    elif outcome in {"failed", "stopped"}:
        if completed != attempted - 1:
            raise SummaryMalformed()
    else:
        raise SummaryMalformed()
    return record


def _valid_p43_validation(value: object) -> dict[str, object]:
    record = _exact_object(
        value,
        frozenset(
            {
                "all_declared_checks_completed",
                "completed_checks",
                "event_count",
                "expected_checks",
            }
        ),
    )
    completed = _exact_integer(record["completed_checks"])
    expected = _exact_integer(record["expected_checks"])
    count = _exact_integer(record["event_count"])
    all_complete = _exact_boolean(record["all_declared_checks_completed"])
    if (
        not 0 <= completed <= expected <= 1_000_000
        or not 0 <= count <= 128
        or all_complete != (completed == expected)
    ):
        raise SummaryMalformed()
    return record


def _p43_identities() -> dict[str, str]:
    return {
        "manifest_aggregate": P43_MANIFEST_AGGREGATE,
        "manifest_raw_sha256": P43_MANIFEST_RAW_SHA256,
        "qualification_receipt_payload_sha256": P43_RECEIPT_PAYLOAD_SHA256,
        "qualification_receipt_raw_sha256": P43_RECEIPT_RAW_SHA256,
        "release_seal_payload_sha256": P43_SEAL_PAYLOAD_SHA256,
        "release_seal_raw_sha256": P43_SEAL_RAW_SHA256,
        "source_sha256": P43_SOURCE_SHA256,
    }


def _capture_p43_summary(value: object) -> dict[str, object]:
    if type(value) is not dict:
        raise SummaryMalformed()
    if value.get("schema") != P43_SUMMARY_SCHEMA:
        raise SummaryMalformed()
    publication_value = value.get("publication")
    if type(publication_value) is not dict:
        raise SummaryMalformed()
    if publication_value.get("state") == "published":
        summary = _exact_object(
            value,
            frozenset(
                {
                    "ordered_execution",
                    "publication",
                    "public_self_validation",
                    "schema",
                }
            ),
        )
        publication = _exact_object(
            summary["publication"],
            frozenset(
                {
                    "final_files_present",
                    "files",
                    "raw_hashes",
                    "rename_attempts",
                    "retries",
                    "state",
                    "sync",
                }
            ),
        )
        if (
            publication["final_files_present"] is not True
            or publication["files"] != "2/2"
            or _exact_integer(publication["rename_attempts"]) != 1
            or _exact_integer(publication["retries"]) != 0
            or publication["state"] != "published"
        ):
            raise SummaryMalformed()
        hashes = _exact_object(
            publication["raw_hashes"],
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "result_payload_sha256",
                    "result_raw_sha256",
                }
            ),
        )
        for digest in hashes.values():
            _valid_digest(digest)
        sync = _valid_sync_set(publication["sync"], published=True)
        ordered = _valid_p43_ordered(summary["ordered_execution"])
        validation = _valid_p43_validation(summary["public_self_validation"])
        return {
            "kind": "published",
            "publication": {
                "final_files": "2/2",
                "raw_hashes": hashes,
                "rename_attempts": 1,
                "retries": 0,
                "sync": sync,
            },
            "ordered_execution": {
                "attempted_gate_count": ordered["attempted_gate_count"],
                "catalog_gate_count": ordered["catalog_gate_count"],
                "completed_gate_count": ordered["completed_gate_count"],
                "terminal_event_kind": "terminal-stop",
                "terminal_outcome": ordered["terminal"]["outcome"],
            },
            "public_self_validation": validation,
        }

    summary = _exact_object(
        value, frozenset({"failure_code", "publication", "schema"})
    )
    code = _exact_string(summary["failure_code"])
    if P43_FAILURE_CODE.fullmatch(code) is None:
        raise SummaryMalformed()
    publication = _exact_object(
        summary["publication"],
        frozenset(
            {
                "final_files_present",
                "rename_attempts",
                "retries",
                "state",
                "sync",
            }
        ),
    )
    if (
        publication["final_files_present"] is not False
        or _exact_integer(publication["rename_attempts"]) not in {0, 1}
        or _exact_integer(publication["retries"]) != 0
        or publication["state"] not in {"absent", "rolled-back", "indeterminate"}
    ):
        raise SummaryMalformed()
    sync = _valid_sync_set(publication["sync"], published=False)
    return {
        "failure_code": code,
        "kind": "failed",
        "publication": {
            "final_files_present": False,
            "rename_attempts": publication["rename_attempts"],
            "retries": 0,
            "state": publication["state"],
            "sync": sync,
        },
    }


def _witness_payload(captured: dict[str, object]) -> dict[str, object]:
    return {
        "publication_outcome": captured,
        "pulse_43": {
            "identities": _p43_identities(),
            "invocation_count": 1,
            "retries": 0,
        },
        "release_limits": {
            "diagnostic_authority": False,
            "private_data_access": False,
            "product_conclusion": None,
        },
        "schema": WITNESS_SCHEMA,
    }


def _validate_witness_payload(value: object) -> dict[str, object]:
    payload = _exact_object(
        value,
        frozenset({"publication_outcome", "pulse_43", "release_limits", "schema"}),
    )
    if payload["schema"] != WITNESS_SCHEMA:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    pulse = _exact_object(
        payload["pulse_43"], frozenset({"identities", "invocation_count", "retries"})
    )
    if pulse["identities"] != _p43_identities() or pulse["invocation_count"] != 1 or pulse["retries"] != 0:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    limits = _exact_object(
        payload["release_limits"],
        frozenset({"diagnostic_authority", "private_data_access", "product_conclusion"}),
    )
    if limits != {
        "diagnostic_authority": False,
        "private_data_access": False,
        "product_conclusion": None,
    }:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    try:
        captured = _capture_p43_summary(
            _expand_captured_for_validation(payload["publication_outcome"])
        )
    except (SummaryMalformed, WitnessFailure) as error:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE") from error
    if captured != payload["publication_outcome"]:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    return payload


def _expand_captured_for_validation(value: object) -> dict[str, object]:
    """Rebuild a closed P43-shaped record solely to validate the filtered witness."""

    record = _exact_object(value, frozenset({"kind", "publication", "ordered_execution", "public_self_validation"})) if type(value) is dict and value.get("kind") == "published" else None
    if record is not None:
        publication = _exact_object(
            record["publication"],
            frozenset({"final_files", "raw_hashes", "rename_attempts", "retries", "sync"}),
        )
        ordered = _exact_object(
            record["ordered_execution"],
            frozenset(
                {
                    "attempted_gate_count",
                    "catalog_gate_count",
                    "completed_gate_count",
                    "terminal_event_kind",
                    "terminal_outcome",
                }
            ),
        )
        if (
            publication["final_files"] != "2/2"
            or publication["rename_attempts"] != 1
            or publication["retries"] != 0
            or ordered["terminal_event_kind"] != "terminal-stop"
        ):
            raise SummaryMalformed()
        return {
            "ordered_execution": {
                "attempted_gate_count": ordered["attempted_gate_count"],
                "catalog_gate_count": ordered["catalog_gate_count"],
                "completed_gate_count": ordered["completed_gate_count"],
                "terminal": {
                    "event_kind": "terminal-stop",
                    "gate_id": "public-terminal",
                    "outcome": ordered["terminal_outcome"],
                },
            },
            "publication": {
                "final_files_present": True,
                "files": "2/2",
                "raw_hashes": publication["raw_hashes"],
                "rename_attempts": 1,
                "retries": 0,
                "state": "published",
                "sync": publication["sync"],
            },
            "public_self_validation": record["public_self_validation"],
            "schema": P43_SUMMARY_SCHEMA,
        }
    failure = _exact_object(
        value, frozenset({"failure_code", "kind", "publication"})
    )
    if failure["kind"] != "failed":
        raise SummaryMalformed()
    publication = _exact_object(
        failure["publication"],
        frozenset(
            {
                "final_files_present",
                "rename_attempts",
                "retries",
                "state",
                "sync",
            }
        ),
    )
    return {
        "failure_code": failure["failure_code"],
        "publication": publication,
        "schema": P43_SUMMARY_SCHEMA,
    }


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
        raise WitnessFailure("P47-INVALID-SYNC-POSTURE")
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
        raise WitnessFailure("P47-WITNESS-FINAL-ROOT-INVALID") from error
    if type(raw) is not str or "\x00" in raw:
        raise WitnessFailure("P47-WITNESS-FINAL-ROOT-INVALID")
    requested = Path(raw)
    if not requested.is_absolute() or ".." in requested.parts:
        raise WitnessFailure("P47-WITNESS-FINAL-ROOT-INVALID")
    try:
        final_root = requested.resolve(strict=False)
        parent_metadata = os.lstat(requested.parent)
    except OSError as error:
        raise WitnessFailure("P47-WITNESS-FINAL-PARENT-UNSAFE") from error
    if stat.S_ISLNK(parent_metadata.st_mode) or not stat.S_ISDIR(parent_metadata.st_mode):
        raise WitnessFailure("P47-WITNESS-FINAL-PARENT-UNSAFE")
    if _lexists(requested) or _lexists(final_root):
        raise WitnessFailure("P47-WITNESS-FINAL-EXISTS")
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
                raise OSError("short public witness write")
            offset += written
        os.fsync(descriptor)
    finally:
        if descriptor is not None:
            os.close(descriptor)


def _read_regular(path: Path) -> bytes:
    try:
        before = os.lstat(path)
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
            raise WitnessFailure("P47-WITNESS-OUTPUT-TREE-UNSAFE")
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except WitnessFailure:
        raise
    except OSError as error:
        raise WitnessFailure("P47-WITNESS-OUTPUT-TREE-UNSAFE") from error
    try:
        after = os.fstat(descriptor)
        if not stat.S_ISREG(after.st_mode) or (before.st_dev, before.st_ino) != (
            after.st_dev,
            after.st_ino,
        ):
            raise WitnessFailure("P47-WITNESS-OUTPUT-TREE-UNSAFE")
        parts: list[bytes] = []
        while chunk := os.read(descriptor, 65_536):
            parts.append(chunk)
        return b"".join(parts)
    finally:
        os.close(descriptor)


def _read_envelope(content: bytes, schema: str) -> dict[str, object]:
    if b"\r" in content or not content.endswith(b"\n"):
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    try:
        value = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (json.JSONDecodeError, WitnessFailure) as error:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE") from error
    envelope = _exact_object(
        value, frozenset({"payload", "payload_sha256", "schema"})
    )
    if envelope["schema"] != schema or type(envelope["payload"]) is not dict:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    if envelope["payload_sha256"] != sha256_bytes(canonical_bytes(envelope["payload"])):
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    return envelope


def verify_witness_directory(root: Path) -> dict[str, str]:
    try:
        metadata = os.lstat(root)
        entries = sorted(os.scandir(root), key=lambda entry: entry.name)
    except OSError as error:
        raise WitnessFailure("P47-WITNESS-FINAL-DIRECTORY-MISSING") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        raise WitnessFailure("P47-WITNESS-FINAL-DIRECTORY-MISSING")
    if [entry.name for entry in entries] != list(OUTPUT_FILES):
        raise WitnessFailure("P47-WITNESS-OUTPUT-TREE-UNSAFE")
    contents = {entry.name: _read_regular(Path(entry.path)) for entry in entries}
    witness = _read_envelope(
        contents["publication-witness.json"], WITNESS_ENVELOPE_SCHEMA
    )
    _validate_witness_payload(witness["payload"])
    receipt = _read_envelope(contents["release-receipt.json"], RECEIPT_ENVELOPE_SCHEMA)
    receipt_payload = _exact_object(
        receipt["payload"],
        frozenset(
            {
                "publication",
                "witness_payload_sha256",
                "witness_raw_sha256",
                "schema",
            }
        ),
    )
    if receipt_payload["schema"] != RECEIPT_SCHEMA:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    if receipt_payload["witness_raw_sha256"] != sha256_bytes(
        contents["publication-witness.json"]
    ) or receipt_payload["witness_payload_sha256"] != witness["payload_sha256"]:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    publication = _exact_object(
        receipt_payload["publication"],
        frozenset(
            {"file_count", "file_fsync", "rename_attempts", "retries", "transaction"}
        ),
    )
    if publication != {
        "file_count": 2,
        "file_fsync": "2/2-os.fsync-before-close",
        "rename_attempts": 1,
        "retries": 0,
        "transaction": "absent-staged-verified-one-rename-final-verified",
    }:
        raise WitnessFailure("P47-WITNESS-VERIFY-FAILURE")
    return {
        "receipt_payload_sha256": receipt["payload_sha256"],
        "receipt_raw_sha256": sha256_bytes(contents["release-receipt.json"]),
        "witness_payload_sha256": witness["payload_sha256"],
        "witness_raw_sha256": sha256_bytes(contents["publication-witness.json"]),
    }


@dataclass
class WitnessPublicationState:
    rename_attempts: int = 0
    retries: int = 0
    stage_sync: SyncPosture = field(default_factory=_not_attempted_sync)
    final_parent_sync: SyncPosture = field(default_factory=_not_attempted_sync)
    rollback_parent_sync: SyncPosture = field(default_factory=_not_attempted_sync)

    def failure(self, code: str, posture: str) -> dict[str, object]:
        return {
            "failure_code": code,
            "schema": SUMMARY_SCHEMA,
            "witness_publication": {
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
        }


Writer = Callable[[Path, bytes], None]
Renamer = Callable[[Path, Path], None]
Remover = Callable[[Path], None]
Synchronizer = Callable[[Path], SyncPosture]
Verifier = Callable[[Path], dict[str, str]]
PostRename = Callable[[Path], None]
Pulse43Invoker = Callable[[object, object, str | os.PathLike[str]], object]


def _clean_stage(
    stage: Path, remover: Remover, state: WitnessPublicationState, code: str
) -> dict[str, object]:
    try:
        remover(stage)
    except Exception:
        return state.failure("P47-INDETERMINATE-WITNESS-PUBLICATION", "indeterminate")
    if _lexists(stage):
        return state.failure("P47-INDETERMINATE-WITNESS-PUBLICATION", "indeterminate")
    return state.failure(code, "absent")


def _rollback_final(
    final_root: Path,
    final_parent: Path,
    synchronizer: Synchronizer,
    remover: Remover,
    state: WitnessPublicationState,
    code: str,
) -> dict[str, object]:
    try:
        remover(final_root)
    except Exception:
        return state.failure("P47-INDETERMINATE-WITNESS-PUBLICATION", "indeterminate")
    if _lexists(final_root):
        return state.failure("P47-INDETERMINATE-WITNESS-PUBLICATION", "indeterminate")
    try:
        state.rollback_parent_sync = _validate_sync_posture(synchronizer(final_parent))
    except Exception:
        state.rollback_parent_sync = _failed_sync()
        return state.failure("P47-INDETERMINATE-WITNESS-PUBLICATION", "indeterminate")
    return state.failure(code, "rolled-back")


def witness_pulse_43(
    catalog_value: object,
    events_value: object,
    p43_final_root: str | os.PathLike[str],
    witness_final_root: str | os.PathLike[str],
    *,
    invoker: Pulse43Invoker,
    writer: Writer = _write_fsynced,
    renamer: Renamer = os.replace,
    remover: Remover = remove_tree,
    synchronizer: Synchronizer = sync_directory,
    verifier: Verifier = verify_witness_directory,
    post_rename: PostRename | None = None,
) -> dict[str, object]:
    """Invoke Pulse 43 once and publish only a sealed public outcome witness."""

    state = WitnessPublicationState()
    try:
        final_root, final_parent = _absolute_final_root(witness_final_root)
    except WitnessFailure as error:
        return state.failure(error.code, "absent")
    try:
        p43_summary = invoker(catalog_value, events_value, p43_final_root)
    except Exception:
        return state.failure("P47-P43-INVOCATION-FAILURE", "absent")
    try:
        captured = _capture_p43_summary(p43_summary)
    except SummaryMalformed:
        return state.failure("P47-P43-SUMMARY-MALFORMED", "absent")

    payload = _witness_payload(captured)
    witness_bytes = (
        canonical_bytes(
            {
                "payload": payload,
                "payload_sha256": sha256_bytes(canonical_bytes(payload)),
                "schema": WITNESS_ENVELOPE_SCHEMA,
            }
        )
        + b"\n"
    )
    receipt_payload = {
        "publication": {
            "file_count": 2,
            "file_fsync": "2/2-os.fsync-before-close",
            "rename_attempts": 1,
            "retries": 0,
            "transaction": "absent-staged-verified-one-rename-final-verified",
        },
        "schema": RECEIPT_SCHEMA,
        "witness_payload_sha256": sha256_bytes(canonical_bytes(payload)),
        "witness_raw_sha256": sha256_bytes(witness_bytes),
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
    stage = final_parent / f".{final_root.name}.pulse-47-stage"
    if _lexists(stage):
        return state.failure("P47-WITNESS-STAGING-EXISTS", "absent")
    try:
        os.mkdir(stage)
        writer(stage / "publication-witness.json", witness_bytes)
        writer(stage / "release-receipt.json", receipt_bytes)
    except Exception:
        return _clean_stage(stage, remover, state, "P47-WITNESS-STAGE-COPY-FAILURE")
    try:
        verifier(stage)
    except Exception:
        return _clean_stage(stage, remover, state, "P47-WITNESS-STAGE-VERIFY-FAILURE")
    try:
        state.stage_sync = _validate_sync_posture(synchronizer(stage))
    except Exception:
        state.stage_sync = _failed_sync()
        return _clean_stage(stage, remover, state, "P47-WITNESS-STAGE-SYNC-FAILURE")

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
                "P47-WITNESS-RENAME-FAILURE",
            )
        return _clean_stage(stage, remover, state, "P47-WITNESS-RENAME-FAILURE")
    if _lexists(stage):
        if not _lexists(final_root):
            return _clean_stage(
                stage, remover, state, "P47-WITNESS-FINAL-DIRECTORY-MISSING"
            )
        return state.failure("P47-INDETERMINATE-WITNESS-PUBLICATION", "indeterminate")
    try:
        if post_rename is not None:
            post_rename(final_root)
        hashes = verifier(final_root)
    except Exception:
        return _rollback_final(
            final_root,
            final_parent,
            synchronizer,
            remover,
            state,
            "P47-WITNESS-FINAL-VERIFY-FAILURE",
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
            "P47-WITNESS-FINAL-SYNC-FAILURE",
        )
    return {
        "outcome": "published",
        "publication_outcome": captured,
        "schema": SUMMARY_SCHEMA,
        "witness_publication": {
            "files": "2/2",
            "final_files_present": True,
            "raw_hashes": hashes,
            "rename_attempts": 1,
            "retries": 0,
            "state": "published",
            "sync": {
                "final_parent": state.final_parent_sync.public(),
                "rollback_parent": state.rollback_parent_sync.public(),
                "stage": state.stage_sync.public(),
            },
        },
    }


def _regular_bytes(path: Path) -> bytes:
    try:
        before = os.lstat(path)
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
            raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except WitnessFailure:
        raise
    except OSError as error:
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (before.st_dev, before.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > 1_048_576:
                raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
        return bytes(content)
    finally:
        os.close(descriptor)


def _sealed_json(raw: bytes) -> dict[str, object]:
    try:
        value = json.loads(raw.decode("utf-8"), object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, WitnessFailure) as error:
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY") from error
    if type(value) is not dict:
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
    return value


def _p43_release_root() -> Path:
    return Path(__file__).resolve().parent.parent / P43_RELEASE_DIRECTORY


def _verify_exact_p43_release(root: Path) -> Path:
    manifest_raw = _regular_bytes(root / "public-manifest.json")
    receipt_raw = _regular_bytes(root / "qualification-receipt.json")
    seal_raw = _regular_bytes(root / "release-seal.json")
    source = root / "ordered_result_publisher.py"
    if (
        sha256_bytes(manifest_raw) != P43_MANIFEST_RAW_SHA256
        or sha256_bytes(receipt_raw) != P43_RECEIPT_RAW_SHA256
        or sha256_bytes(seal_raw) != P43_SEAL_RAW_SHA256
        or sha256_bytes(_regular_bytes(source)) != P43_SOURCE_SHA256
    ):
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
    manifest = _sealed_json(manifest_raw)
    receipt = _sealed_json(receipt_raw)
    seal = _sealed_json(seal_raw)
    if (
        manifest.get("aggregate") != P43_MANIFEST_AGGREGATE
        or manifest.get("file_count") != 6
        or manifest.get("release_tree_file_count") != 9
        or manifest.get("schema")
        != "ferris.pulse-43-ordered-result-public-manifest/v1"
        or receipt.get("payload_sha256") != P43_RECEIPT_PAYLOAD_SHA256
        or receipt.get("receipt_id") != P43_RECEIPT_PAYLOAD_SHA256
        or seal.get("payload_sha256") != P43_SEAL_PAYLOAD_SHA256
        or seal.get("seal_id") != P43_SEAL_PAYLOAD_SHA256
    ):
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
    return source


def _load_exact_p43_module() -> object:
    source = _verify_exact_p43_release(_p43_release_root())
    specification = importlib.util.spec_from_file_location("pulse_47_p43", source)
    if specification is None or specification.loader is None:
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
    module = importlib.util.module_from_spec(specification)
    sys.modules[specification.name] = module
    try:
        specification.loader.exec_module(module)
    except Exception as error:
        sys.modules.pop(specification.name, None)
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY") from error
    if not callable(getattr(module, "publish_result", None)):
        raise WitnessFailure("P47-P43-RELEASE-IDENTITY")
    return module


def invoke_real_pulse_43(
    catalog_value: object, events_value: object, final_root: str | os.PathLike[str]
) -> object:
    module = _load_exact_p43_module()
    return module.publish_result(catalog_value, events_value, final_root)


class PublicArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> NoReturn:
        del message
        raise WitnessFailure("P47-ARGUMENT")


def arguments(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = PublicArgumentParser(description=__doc__)
    parser.add_argument("--catalog", required=True)
    parser.add_argument("--events", required=True)
    parser.add_argument("--p43-final-root", required=True)
    parser.add_argument("--witness-final-root", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        parsed = arguments(argv)
        module = _load_exact_p43_module()
        catalog = module.load_public_json(parsed.catalog)
        events = module.load_public_json(parsed.events)
        result = witness_pulse_43(
            catalog,
            events,
            parsed.p43_final_root,
            parsed.witness_final_root,
            invoker=module.publish_result,
        )
    except WitnessFailure as error:
        result = WitnessPublicationState().failure(error.code, "absent")
    print(canonical_bytes(result).decode("ascii"))
    return 0 if result.get("outcome") == "published" else 1


if __name__ == "__main__":
    raise SystemExit(main())
