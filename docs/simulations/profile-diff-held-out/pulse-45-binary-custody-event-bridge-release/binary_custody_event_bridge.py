#!/usr/bin/env python3
"""Translate one sealed Pulse 44 platform result into one Pulse 43 gate event."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import stat
import sys
from pathlib import Path
from typing import Callable, Iterable, NoReturn


P44_RELEASE_DIRECTORY = "pulse-44-retained-binary-custody-release"
P44_MANIFEST_RAW_SHA256 = (
    "sha256:eae4db6c4add7f20a919cd301dc307cc7845f808f458219b5627c135ed5f0c94"
)
P44_MANIFEST_AGGREGATE = (
    "sha256:a22efbbb233ee53550c8ac9771a83af3829c16ce8f7f7a2ff15638adf2f58f94"
)
P44_RECEIPT_RAW_SHA256 = (
    "sha256:d17ac162d7e8d5afb9f41fa789afe43c2512f2ee1dd30b4afaae4bde16491f1b"
)
P44_RECEIPT_PAYLOAD_SHA256 = (
    "sha256:a5a5be3d0832476ba0addb4edda2790d3e02acda49a1266601e6065bc0f9cf29"
)
P44_SEAL_RAW_SHA256 = (
    "sha256:97598062129317e89862407cc00971aa11ac179420088f4d508678b535cab2a8"
)
P44_SEAL_PAYLOAD_SHA256 = (
    "sha256:4b90c678255fe3567760ce2ef253192a5489ee684ae57a4eb15446f038c189b5"
)
P44_SOURCE_SHA256 = (
    "sha256:101951fed6006b390499ba6400c828a0c0e902f018ec75bdb30bde9eb23f0942"
)
P44_SUMMARY_SCHEMA = "ferris.pulse-44-retained-binary-custody-summary/v1"
P43_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
SUMMARY_SCHEMA = "ferris.pulse-45-binary-custody-event-bridge-summary/v1"
PLATFORM_GATES = {
    "ubuntu-24.04-x86_64": "ubuntu-retained-binary-custody",
    "windows-x86_64": "windows-retained-binary-custody",
}
SYNC_STATUSES = frozenset({"failed", "not-attempted", "synced", "unsupported"})
SYNC_MECHANISM = "os.open+os.fsync-directory-v1"
FAILURE_STATES = frozenset({"absent", "indeterminate", "rolled-back"})


class BridgeFailure(Exception):
    """A bounded public bridge failure."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class SummaryMalformed(Exception):
    """The returned Pulse 44 summary was not an exact public record."""


Pulse44Invoker = Callable[[Path, str, str, str | os.PathLike[str], str | os.PathLike[str]], object]


def canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return f"sha256:{hashlib.sha256(value).hexdigest()}"


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


def _valid_sync(value: object, allowed_statuses: frozenset[str] = SYNC_STATUSES) -> None:
    record = _exact_object(
        value, frozenset({"attempted", "error_category", "mechanism", "status"})
    )
    attempted = _exact_boolean(record["attempted"])
    error_category = record["error_category"]
    if error_category is not None:
        error_category = _exact_string(error_category)
    mechanism = _exact_string(record["mechanism"])
    status = _exact_string(record["status"])
    if status not in allowed_statuses:
        raise SummaryMalformed()
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
    if (attempted, error_category, mechanism) != expected[status]:
        raise SummaryMalformed()


def _valid_sync_set(value: object, *, published: bool) -> None:
    record = _exact_object(value, frozenset({"final_parent", "rollback_parent", "stage"}))
    if published:
        completed = frozenset({"synced", "unsupported"})
        _valid_sync(record["final_parent"], completed)
        _valid_sync(record["rollback_parent"], frozenset({"not-attempted"}))
        _valid_sync(record["stage"], completed)
        return
    _valid_sync(record["final_parent"])
    _valid_sync(record["rollback_parent"])
    _valid_sync(record["stage"])


def _valid_event(value: object, outcome: str) -> None:
    if value != {
        "classification": "ordered-execution",
        "event_kind": "terminal-stop",
        "gate_id": "retained-binary-custody",
        "outcome": outcome,
        "schema": P43_EVENT_SCHEMA,
    }:
        raise SummaryMalformed()


def _valid_p44_summary(value: object) -> tuple[str, str | None, str | None]:
    if type(value) is not dict:
        raise SummaryMalformed()
    if value.get("outcome") == "published":
        record = _exact_object(
            value,
            frozenset({"custody", "ordered_execution_event", "outcome", "schema"}),
        )
        if record["schema"] != P44_SUMMARY_SCHEMA or record["outcome"] != "published":
            raise SummaryMalformed()
        custody = _exact_object(
            record["custody"],
            frozenset(
                {
                    "files",
                    "final_files_present",
                    "final_verified",
                    "rename_attempts",
                    "retries",
                    "stage_verified",
                    "state",
                    "sync",
                    "work_verified",
                }
            ),
        )
        rename_attempts = _exact_integer(custody["rename_attempts"])
        retries = _exact_integer(custody["retries"])
        if (
            custody["files"] != "2/2"
            or custody["final_files_present"] is not True
            or custody["final_verified"] != "2/2"
            or rename_attempts != 1
            or retries != 0
            or custody["stage_verified"] != "2/2"
            or custody["state"] != "published"
            or custody["work_verified"] != "2/2"
        ):
            raise SummaryMalformed()
        _valid_sync_set(custody["sync"], published=True)
        _valid_event(record["ordered_execution_event"], "completed")
        return "published", None, None

    record = _exact_object(
        value,
        frozenset(
            {
                "custody",
                "failure_code",
                "ordered_execution_event",
                "outcome",
                "schema",
            }
        ),
    )
    if record["schema"] != P44_SUMMARY_SCHEMA or record["outcome"] != "failed":
        raise SummaryMalformed()
    failure_code = _exact_string(record["failure_code"])
    suffix = failure_code.removeprefix("P44-")
    if (
        not failure_code.startswith("P44-")
        or not suffix
        or any(character not in "ABCDEFGHIJKLMNOPQRSTUVWXYZ-" for character in suffix)
    ):
        raise SummaryMalformed()
    custody = _exact_object(
        record["custody"],
        frozenset(
            {
                "final_files_present",
                "final_verified",
                "rename_attempts",
                "retries",
                "stage_verified",
                "state",
                "sync",
                "work_verified",
            }
        ),
    )
    state = _exact_string(custody["state"])
    rename_attempts = _exact_integer(custody["rename_attempts"])
    retries = _exact_integer(custody["retries"])
    if (
        custody["final_files_present"] is not False
        or custody["final_verified"] not in {"0/2", "2/2"}
        or rename_attempts not in {0, 1}
        or retries != 0
        or custody["stage_verified"] not in {"0/2", "2/2"}
        or state not in FAILURE_STATES
        or custody["work_verified"] not in {"0/2", "2/2"}
    ):
        raise SummaryMalformed()
    _valid_sync_set(custody["sync"], published=False)
    _valid_event(record["ordered_execution_event"], "failed")
    return "failed", failure_code, state


def _gate_for(platform: str) -> str:
    try:
        return PLATFORM_GATES[platform]
    except KeyError as error:
        raise BridgeFailure("P45-UNSUPPORTED-PLATFORM") from error


def _bridge_record(platform: str) -> dict[str, object]:
    return {
        "invocation_count": 1,
        "platform": platform,
        "pulse_44_release": {
            "manifest_aggregate": P44_MANIFEST_AGGREGATE,
            "manifest_raw_sha256": P44_MANIFEST_RAW_SHA256,
            "qualification_receipt_payload_sha256": P44_RECEIPT_PAYLOAD_SHA256,
            "qualification_receipt_raw_sha256": P44_RECEIPT_RAW_SHA256,
            "release_seal_payload_sha256": P44_SEAL_PAYLOAD_SHA256,
            "release_seal_raw_sha256": P44_SEAL_RAW_SHA256,
        },
        "retries": 0,
    }


def _event(platform: str, event_kind: str, outcome: str) -> dict[str, str]:
    return {
        "classification": "ordered-execution",
        "event_kind": event_kind,
        "gate_id": _gate_for(platform),
        "outcome": outcome,
        "schema": P43_EVENT_SCHEMA,
    }


def _failure(
    platform: str, code: str, source_failure: dict[str, str] | None = None
) -> dict[str, object]:
    result: dict[str, object] = {
        "bridge": _bridge_record(platform),
        "failure_code": code,
        "ordered_execution_event": _event(platform, "terminal-stop", "failed"),
        "outcome": "failed",
        "schema": SUMMARY_SCHEMA,
    }
    if source_failure is not None:
        result["source_failure"] = source_failure
    return result


def bridge_pulse_44(
    repo: Path,
    cutoff: str,
    platform: str,
    work_root: str | os.PathLike[str],
    final_root: str | os.PathLike[str],
    *,
    invoker: Pulse44Invoker,
) -> dict[str, object]:
    """Invoke Pulse 44 once and translate only a complete sealed summary."""

    _gate_for(platform)
    try:
        summary = invoker(repo, cutoff, platform, work_root, final_root)
    except Exception:
        return _failure(platform, "P45-P44-INVOCATION-FAILURE")
    try:
        outcome, source_code, source_state = _valid_p44_summary(summary)
    except SummaryMalformed:
        return _failure(platform, "P45-P44-SUMMARY-MALFORMED")
    if outcome == "failed":
        assert source_code is not None and source_state is not None
        return _failure(
            platform,
            source_code,
            {"custody_state": source_state, "failure_code": source_code},
        )
    return {
        "bridge": _bridge_record(platform),
        "ordered_execution_event": _event(platform, "gate-complete", "passed"),
        "outcome": "passed",
        "schema": SUMMARY_SCHEMA,
    }


def _regular_bytes(path: Path) -> bytes:
    try:
        before = os.lstat(path)
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
            raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except BridgeFailure:
        raise
    except OSError as error:
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (before.st_dev, before.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > 1_048_576:
                raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
        return bytes(content)
    finally:
        os.close(descriptor)


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
        result[key] = value
    return result


def _sealed_json(raw: bytes) -> dict[str, object]:
    try:
        value = json.loads(raw.decode("utf-8"), object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, BridgeFailure) as error:
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY") from error
    if type(value) is not dict:
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
    return value


def _p44_release_root() -> Path:
    return Path(__file__).resolve().parent.parent / P44_RELEASE_DIRECTORY


def _verify_exact_p44_release(root: Path) -> Path:
    manifest_raw = _regular_bytes(root / "public-manifest.json")
    receipt_raw = _regular_bytes(root / "qualification-receipt.json")
    seal_raw = _regular_bytes(root / "release-seal.json")
    source = root / "retained_binary_custody.py"
    if (
        sha256_bytes(manifest_raw) != P44_MANIFEST_RAW_SHA256
        or sha256_bytes(receipt_raw) != P44_RECEIPT_RAW_SHA256
        or sha256_bytes(seal_raw) != P44_SEAL_RAW_SHA256
        or sha256_bytes(_regular_bytes(source)) != P44_SOURCE_SHA256
    ):
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
    manifest = _sealed_json(manifest_raw)
    receipt = _sealed_json(receipt_raw)
    seal = _sealed_json(seal_raw)
    if (
        manifest.get("aggregate") != P44_MANIFEST_AGGREGATE
        or manifest.get("file_count") != 5
        or manifest.get("release_tree_file_count") != 8
        or manifest.get("schema") != "ferris.pulse-44-retained-binary-custody-public-manifest/v1"
        or receipt.get("payload_sha256") != P44_RECEIPT_PAYLOAD_SHA256
        or receipt.get("receipt_id") != P44_RECEIPT_PAYLOAD_SHA256
        or seal.get("payload_sha256") != P44_SEAL_PAYLOAD_SHA256
        or seal.get("seal_id") != P44_SEAL_PAYLOAD_SHA256
    ):
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
    return source


def _load_exact_pulse_44_invoker() -> Pulse44Invoker:
    source = _verify_exact_p44_release(_p44_release_root())
    specification = importlib.util.spec_from_file_location("pulse_45_p44", source)
    if specification is None or specification.loader is None:
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
    module = importlib.util.module_from_spec(specification)
    sys.modules[specification.name] = module
    try:
        specification.loader.exec_module(module)
        invoker = getattr(module, "retain_binary_custody")
    except Exception as error:
        sys.modules.pop(specification.name, None)
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY") from error
    if not callable(invoker):
        raise BridgeFailure("P45-P44-RELEASE-IDENTITY")
    return invoker


def invoke_real_pulse_44(
    repo: Path,
    cutoff: str,
    platform: str,
    work_root: str | os.PathLike[str],
    final_root: str | os.PathLike[str],
) -> object:
    return _load_exact_pulse_44_invoker()(repo, cutoff, platform, work_root, final_root)


class PublicArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> NoReturn:
        del message
        raise BridgeFailure("P45-ARGUMENT")


def arguments(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = PublicArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True, type=Path)
    parser.add_argument("--cutoff", required=True)
    parser.add_argument("--platform", required=True, choices=sorted(PLATFORM_GATES))
    parser.add_argument("--work-root", required=True)
    parser.add_argument("--final-root", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        parsed = arguments(argv)
        result = bridge_pulse_44(
            parsed.repo,
            parsed.cutoff,
            parsed.platform,
            parsed.work_root,
            parsed.final_root,
            invoker=invoke_real_pulse_44,
        )
    except BridgeFailure as error:
        result = {
            "failure_code": error.code,
            "outcome": "failed",
            "schema": SUMMARY_SCHEMA,
        }
    print(canonical_bytes(result).decode("ascii"))
    return 0 if result["outcome"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
