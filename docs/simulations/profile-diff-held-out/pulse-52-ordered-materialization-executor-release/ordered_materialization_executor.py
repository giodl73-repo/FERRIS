"""Pulse 52 ordered materialization executor.

The sole production callable starts with public custody inputs, executes the
first six sealed public gates, and only then creates a fresh private seed and
P35 descriptor corpus.  It is infrastructure only: authority remains external
and a future authority must bind this sealed release together with Pulse 51.
"""

from __future__ import annotations

import os
import re
import secrets
import stat
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Mapping

from sealed_dependencies import (
    CANONICAL_PLATFORMS,
    P50_GATE_IDS,
    PULSE39_RELEASE_ROOT,
    SealedDependencyFailure,
    load_p35_materializer_and_verifier,
    load_p39_and_p41,
    load_pulse51,
)


P43_CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
P43_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
PRIVATE_LAUNCH_DIRECTORY = ".pulse52-private-launch"
SEED_FILENAME = "seed.bin"
DESCRIPTOR_DIRECTORY = "descriptors"
TERMINAL_DIRECTORY = ".pulse52-terminal-publication"
P43_FINAL_DIRECTORY = "pulse-52-p43-result"
WITNESS_FINAL_DIRECTORY = "pulse-52-p47-witness"
TERMINAL_CLEANUP_DELAYS = (0.02, 0.05, 0.10, 0.20)
TERMINAL_CLEANUP_FATAL_SCHEMA = (
    "ferris.pulse-52-terminal-publication-cleanup-indeterminate/v1"
)


@dataclass(frozen=True)
class OrderedMaterializationResult:
    """The P43 event record, terminal-publication disposition, and private record."""

    catalog: dict[str, object]
    events: list[dict[str, object]]
    publication: dict[str, object]
    private_record: dict[str, object]


class TerminalPublicationCleanupIndeterminate(RuntimeError):
    """A non-returning, public-safe terminal cleanup failure."""

    code = "terminal-publication-cleanup-indeterminate"

    def __init__(self) -> None:
        self.public_posture = {
            "schema": TERMINAL_CLEANUP_FATAL_SCHEMA,
            "state": self.code,
            "cleanup_owner": "caller-public-custodian",
            "cleanup_posture": "unresolved",
        }
        super().__init__(self.code)


@dataclass(frozen=True)
class _QualificationControls:
    """Underscored fake-only controls; they are never a production surface."""

    seed_bytes: bytes
    process_runner: Callable[[object], object]
    expectations: Mapping[str, object]
    force_materializer_destination_conflict: bool = False


def _catalog() -> dict[str, object]:
    return {"schema": P43_CATALOG_SCHEMA, "gate_ids": list(P50_GATE_IDS)}


def _event(gate_id: str, kind: str, outcome: str) -> dict[str, object]:
    return {
        "classification": "ordered-execution",
        "event_kind": kind,
        "gate_id": gate_id,
        "outcome": outcome,
        "schema": P43_EVENT_SCHEMA,
    }


def _private_record() -> dict[str, object]:
    return {
        "schema": "ferris.pulse-52-private-execution-record/v1",
        "outcome": "in-progress",
        "execution_outcome": "not-started",
        "publication_disposition": "not-attempted",
        "product_conclusion": None,
        "category_conclusion": None,
        "fix_conclusion": None,
        "private_launch_started": False,
        "prelaunch_private_namespace_absence_checks": 0,
        "p39_checkout_verifications": 0,
        "p41_transactional_copy_invocations": 0,
        "p41_post_copy_binding": "not-attempted",
        "p27_cycle_retention": "not-attempted",
        "p27_invocations": 0,
        "materializer_invocations": 0,
        "verifier_invocations": 0,
        "seed_cleanup": "not-created",
        "descriptor_cleanup": "not-created",
        "private_launch_cleanup": "not-attempted",
        "platform_records": {platform: [] for platform in CANONICAL_PLATFORMS},
        "no_launch_records": [],
        "process_counts": {platform: 0 for platform in CANONICAL_PLATFORMS},
        "terminal_p47_invocation_count": 0,
        "terminal_publication_cleanup": "not-attempted",
    }


def _terminal_publication(disposition: str, posture: dict[str, object]) -> dict[str, object]:
    return {
        "schema": "ferris.pulse-52-terminal-publication-disposition/v1",
        "disposition": disposition,
        "product_conclusion": None,
        "category_conclusion": None,
        "fix_conclusion": None,
        "posture": posture,
    }


def _not_attempted_publication() -> dict[str, object]:
    return _terminal_publication("not-attempted", {"state": "not-attempted"})


def _failure_code(error: BaseException, fallback: str) -> str:
    value = getattr(error, "code", None)
    return value if type(value) is str else fallback


def _terminal_result(
    p43: object | None,
    events: list[dict[str, object]],
    gate: str,
    code: str,
    private_record: dict[str, object],
) -> OrderedMaterializationResult:
    events.append(_event(gate, "terminal-stop", "failed"))
    if p43 is not None:
        p43.validate_catalog(_catalog())
        p43.validate_events(P50_GATE_IDS, events)
    private_record["failure_code"] = code
    private_record["outcome"] = "failed"
    private_record["execution_outcome"] = "failed"
    return OrderedMaterializationResult(
        _catalog(), events, _not_attempted_publication(), private_record
    )


def _raise(p51: object, code: str) -> None:
    raise p51.ExecutorFailure(code)


def _runtime_path(
    p51: object,
    runtime_root: Path,
    value: Path,
    code: str,
    *,
    require_directory: bool = False,
    allow_absent_leaf: bool = False,
) -> Path:
    return p51._runtime_path(
        runtime_root,
        value,
        code,
        require_directory=require_directory,
        allow_absent_leaf=allow_absent_leaf,
    )


def _strictly_below(root: Path, value: Path, p51: object, code: str) -> Path:
    checked = _runtime_path(p51, root, value, code, require_directory=True)
    if checked == root:
        _raise(p51, code)
    return checked


def _not_overlapping(first: Path, second: Path) -> bool:
    try:
        first.relative_to(second)
        return False
    except ValueError:
        try:
            second.relative_to(first)
            return False
        except ValueError:
            return True


def _audit_only_public_custody(
    p51: object,
    runtime_root: Path,
    custodies: Mapping[str, object],
    p27_cycle_root: Path,
) -> None:
    """Require a fresh container containing only declared public P44 custody."""

    cycle = _runtime_path(
        p51, runtime_root, p27_cycle_root, "P52-P27-CYCLE-ROOT", allow_absent_leaf=True
    )
    if cycle.parent != runtime_root or os.path.lexists(cycle):
        _raise(p51, "P52-P27-CYCLE-ROOT")
    custody_roots: list[Path] = []
    for platform in CANONICAL_PLATFORMS:
        custody = custodies[platform]
        final_root = _strictly_below(
            runtime_root, Path(custody.final_root), p51, "P52-P44-CUSTODY-ROOT"
        )
        work_value = Path(custody.work_root)
        work_root = _runtime_path(
            p51,
            runtime_root,
            work_value,
            "P52-P44-WORK-ROOT",
            allow_absent_leaf=True,
        )
        if work_root == runtime_root:
            _raise(p51, "P52-P44-WORK-ROOT")
        if os.path.lexists(work_root):
            work_root = _runtime_path(
                p51, runtime_root, work_root, "P52-P44-WORK-ROOT", require_directory=True
            )
        custody_roots.extend((final_root, work_root))
    for index, first in enumerate(custody_roots):
        for second in custody_roots[index + 1 :]:
            if first == second or not _not_overlapping(first, second):
                _raise(p51, "P52-P44-CUSTODY-ROOT")

    allowed_roots = tuple(path for path in custody_roots if os.path.lexists(path))

    def descend(directory: Path) -> None:
        try:
            with os.scandir(directory) as entries:
                ordered = sorted(entries, key=lambda entry: entry.name)
        except OSError as error:
            raise p51.ExecutorFailure("P52-RUNTIME-NOT-FRESH") from error
        for entry in ordered:
            path = Path(entry.path)
            try:
                metadata = os.lstat(path)
            except OSError as error:
                raise p51.ExecutorFailure("P52-RUNTIME-NOT-FRESH") from error
            if stat.S_ISLNK(metadata.st_mode):
                _raise(p51, "P52-RUNTIME-NOT-FRESH")
            if path in allowed_roots:
                if not stat.S_ISDIR(metadata.st_mode):
                    _raise(p51, "P52-RUNTIME-NOT-FRESH")
                continue
            if any(root != path and root.is_relative_to(path) for root in allowed_roots):
                if not stat.S_ISDIR(metadata.st_mode):
                    _raise(p51, "P52-RUNTIME-NOT-FRESH")
                descend(path)
                continue
            _raise(p51, "P52-RUNTIME-NOT-FRESH")

    descend(runtime_root)


def _assert_private_namespace_absent(
    p51: object, runtime_root: Path, private_record: dict[str, object]
) -> None:
    namespace = _runtime_path(
        p51,
        runtime_root,
        runtime_root / PRIVATE_LAUNCH_DIRECTORY,
        "P52-PRIVATE-LAUNCH-ROOT",
        allow_absent_leaf=True,
    )
    terminal = _runtime_path(
        p51,
        runtime_root,
        runtime_root / TERMINAL_DIRECTORY,
        "P52-TERMINAL-ROOT",
        allow_absent_leaf=True,
    )
    if os.path.lexists(namespace) or os.path.lexists(terminal):
        _raise(p51, "P52-PRIVATE-LAUNCH-NOT-FRESH")
    private_record["prelaunch_private_namespace_absence_checks"] = (
        int(private_record["prelaunch_private_namespace_absence_checks"]) + 1
    )


def _validate_p39_summary(p51: object, p39: object, value: object) -> None:
    if (
        type(value) is not dict
        or set(value)
        != {
            "attribute_files",
            "count",
            "files",
            "git_version",
            "lf_files",
            "status",
            "zero_cr_files",
        }
        or value["attribute_files"] != p39.EXPECTED_CARDINALITY
        or value["count"] != p39.EXPECTED_CARDINALITY
        or value["files"] != list(p39.EXPECTED_PATHS)
        or type(value["git_version"]) is not str
        or not value["git_version"].startswith("git version ")
        or value["lf_files"] != p39.EXPECTED_CARDINALITY
        or value["status"] != "pass"
        or value["zero_cr_files"] != p39.EXPECTED_CARDINALITY
    ):
        _raise(p51, "P52-P41-P39-PRELAUNCH")


def _p41_attempted_sync(value: object, mechanism: str) -> bool:
    if type(value) is not dict or set(value) != {
        "attempted",
        "error_category",
        "mechanism",
        "status",
    }:
        return False
    if value["attempted"] is not True or value["mechanism"] != mechanism:
        return False
    return (
        value["status"] == "synced" and value["error_category"] is None
    ) or (
        value["status"] == "unsupported"
        and value["error_category"] == "unsupported-by-platform-or-filesystem"
    )


def _p41_staging_sync(value: object, mechanism: str) -> bool:
    if type(value) is not dict or set(value) != {
        "attempts",
        "directories",
        "error_category",
        "mechanism",
        "operational_failures",
        "status",
        "synced",
        "unsupported",
        "unsupported_error_categories",
    }:
        return False
    if (
        value["attempts"] != 2
        or value["directories"] != 2
        or value["mechanism"] != mechanism
        or value["operational_failures"] != 0
        or type(value["synced"]) is not int
        or type(value["unsupported"]) is not int
        or value["synced"] < 0
        or value["unsupported"] < 0
        or value["synced"] + value["unsupported"] != 2
        or type(value["unsupported_error_categories"]) is not list
    ):
        return False
    if value["status"] == "synced":
        return (
            value["synced"] == 2
            and value["unsupported"] == 0
            and value["error_category"] is None
            and value["unsupported_error_categories"] == []
        )
    return (
        value["status"] == "unsupported"
        and value["unsupported"] > 0
        and value["error_category"] == "unsupported-by-platform-or-filesystem"
        and value["unsupported_error_categories"]
        == ["unsupported-by-platform-or-filesystem"]
    )


def _validate_p41_summary(p51: object, p41: object, value: object) -> None:
    if (
        type(value) is not dict
        or set(value)
        != {
            "counts",
            "failure_code",
            "indeterminate_publication",
            "rename_attempts",
            "retries",
            "rollback_attempted",
            "rollback_path_absent",
            "rollback_verified_absent",
            "schema",
            "status",
            "sync",
        }
        or value["counts"]
        != {"final": "8/8", "source": "8/8", "stage": "8/8"}
        or value["failure_code"] is not None
        or value["indeterminate_publication"] is not False
        or value["rename_attempts"] != 1
        or value["retries"] != 0
        or value["rollback_attempted"] is not False
        or value["rollback_path_absent"] is not False
        or value["rollback_verified_absent"] is not False
        or value["schema"] != "ferris.pulse-41-transactional-copy-report/v1"
        or value["status"] != "pass"
        or type(value["sync"]) is not dict
        or set(value["sync"]) != {"final_parent", "rollback_parent", "staging"}
        or not _p41_staging_sync(value["sync"]["staging"], p41.SYNC_MECHANISM)
        or not _p41_attempted_sync(value["sync"]["final_parent"], p41.SYNC_MECHANISM)
        or value["sync"]["rollback_parent"]
        != {
            "attempted": False,
            "error_category": "not-attempted",
            "mechanism": "not-attempted",
            "status": "unsupported",
        }
    ):
        _raise(p51, "P52-P41-P39-PRELAUNCH")


def _verify_public_prelaunch_custody(
    p51: object,
    p39: object,
    p41: object,
    runtime_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    private_record: dict[str, object],
) -> None:
    """Verify fresh P39 checkout custody and create one exact P41 copy."""

    try:
        final_root = p41._safe_absolute(p41_final_root)
        stage_root = final_root.parent / f".{final_root.name}.pulse-41-stage"
        if any(
            not _not_overlapping(runtime_root, candidate)
            for candidate in (final_root, stage_root)
        ):
            _raise(p51, "P52-P41-P39-PRELAUNCH")

        private_record["p39_checkout_verifications"] = (
            int(private_record["p39_checkout_verifications"]) + 1
        )
        p39_summary = p39.verify(
            str(p39_checkout_root),
            p39.PULSE_25_ROOT,
            p39.PULSE_27_ROOT,
        )
        _validate_p39_summary(p51, p39, p39_summary)

        checkout_root = p39.resolve_checkout_root(str(p39_checkout_root))
        source_root = checkout_root.joinpath(*PULSE39_RELEASE_ROOT.split("/"))
        private_record["p41_transactional_copy_invocations"] = (
            int(private_record["p41_transactional_copy_invocations"]) + 1
        )
        p41_summary = p41.copy_release(source_root, p41_final_root)
        _validate_p41_summary(p51, p41, p41_summary)
        p41.verify_bound_tree(final_root, "FINAL")
        private_record["p41_post_copy_binding"] = "8/8"
    except p51.ExecutorFailure:
        raise
    except (p39.PublicFailure, p41.PublicFailure, OSError) as error:
        raise p51.ExecutorFailure("P52-P41-P39-PRELAUNCH") from error


def _begin_private_launch(p51: object, runtime_root: Path) -> Path:
    namespace = runtime_root / PRIVATE_LAUNCH_DIRECTORY
    _runtime_path(
        p51, runtime_root, namespace, "P52-PRIVATE-LAUNCH-ROOT", allow_absent_leaf=True
    )
    if os.path.lexists(namespace):
        _raise(p51, "P52-PRIVATE-LAUNCH-NOT-FRESH")
    try:
        os.mkdir(namespace, 0o700)
    except OSError as error:
        raise p51.ExecutorFailure("P52-PRIVATE-LAUNCH-ROOT") from error
    return _runtime_path(
        p51, runtime_root, namespace, "P52-PRIVATE-LAUNCH-ROOT", require_directory=True
    )


def _write_seed_atomically(p51: object, namespace: Path, seed: bytes) -> Path:
    if type(seed) is not bytes or len(seed) != 32:
        _raise(p51, "P52-SEED-GENERATION")
    path = namespace / SEED_FILENAME
    flags = (
        os.O_WRONLY
        | os.O_CREAT
        | os.O_EXCL
        | getattr(os, "O_BINARY", 0)
        | getattr(os, "O_NOFOLLOW", 0)
    )
    try:
        descriptor = os.open(path, flags, stat.S_IRUSR | stat.S_IWUSR)
    except OSError as error:
        raise p51.ExecutorFailure("P52-SEED-CREATION") from error
    try:
        offset = 0
        while offset < len(seed):
            written = os.write(descriptor, seed[offset:])
            if written <= 0:
                _raise(p51, "P52-SEED-CREATION")
            offset += written
        os.fsync(descriptor)
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or opened.st_size != 32:
            _raise(p51, "P52-SEED-CREATION")
    except p51.ExecutorFailure:
        raise
    except OSError as error:
        raise p51.ExecutorFailure("P52-SEED-CREATION") from error
    finally:
        os.close(descriptor)
    try:
        metadata = os.lstat(path)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
            _raise(p51, "P52-SEED-CREATION")
    except p51.ExecutorFailure:
        raise
    except OSError as error:
        raise p51.ExecutorFailure("P52-SEED-CREATION") from error
    return path


def _remove_private_tree(p51: object, path: Path, code: str = "P52-PRIVATE-CLEANUP") -> None:
    if not os.path.lexists(path):
        return
    try:
        metadata = os.lstat(path)
        if stat.S_ISLNK(metadata.st_mode):
            _raise(p51, code)
        if stat.S_ISREG(metadata.st_mode):
            os.unlink(path)
            return
        if not stat.S_ISDIR(metadata.st_mode):
            _raise(p51, code)
        with os.scandir(path) as entries:
            children = [Path(entry.path) for entry in entries]
        for child in children:
            _remove_private_tree(p51, child, code)
        os.rmdir(path)
    except p51.ExecutorFailure:
        raise
    except OSError as error:
        raise p51.ExecutorFailure(code) from error


def _remove_seed(p51: object, seed_path: Path, private_record: dict[str, object]) -> None:
    try:
        metadata = os.lstat(seed_path)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
            _raise(p51, "P52-SEED-CLEANUP")
        os.unlink(seed_path)
        if os.path.lexists(seed_path):
            _raise(p51, "P52-SEED-CLEANUP")
    except p51.ExecutorFailure:
        raise
    except OSError as error:
        raise p51.ExecutorFailure("P52-SEED-CLEANUP") from error
    private_record["seed_cleanup"] = "removed-after-verification"


def _cleanup_private_launch(
    p51: object, namespace: Path, private_record: dict[str, object]
) -> None:
    _remove_private_tree(p51, namespace)
    if os.path.lexists(namespace):
        _raise(p51, "P52-PRIVATE-CLEANUP")
    if private_record["seed_cleanup"] == "not-created":
        private_record["seed_cleanup"] = "removed-on-terminal"
    private_record["descriptor_cleanup"] = "removed"
    private_record["private_launch_cleanup"] = "removed-and-verified"


def _cleanup_terminal_publication(
    p51: object,
    parent: Path,
    p43_root: Path,
    witness_root: Path,
    private_record: dict[str, object],
) -> None:
    """Remove terminal output only after an unsuccessful one-use publication."""

    stage_roots = (
        p43_root.parent / f".{p43_root.name}.pulse-43-stage",
        witness_root.parent / f".{witness_root.name}.pulse-47-stage",
    )
    paths = (parent, p43_root, witness_root, *stage_roots)
    for delay in (*TERMINAL_CLEANUP_DELAYS, None):
        try:
            _remove_private_tree(p51, parent, "P52-TERMINAL-CLEANUP")
            if any(os.path.lexists(path) for path in paths):
                _raise(p51, "P52-TERMINAL-CLEANUP")
        except (p51.ExecutorFailure, PermissionError, OSError):
            if delay is not None:
                time.sleep(delay)
                continue
            raise
        private_record["terminal_publication_cleanup"] = "removed-and-verified"
        return
    raise AssertionError("terminal cleanup retry state is unreachable")


def _digest(value: object) -> bool:
    return (
        type(value) is str
        and re.fullmatch(r"sha256:[0-9a-f]{64}", value) is not None
    )


def _sync(value: object, *, published: bool) -> dict[str, object] | None:
    if type(value) is not dict or set(value) != {
        "attempted",
        "error_category",
        "mechanism",
        "status",
    }:
        return None
    status = value["status"]
    if type(status) is not str:
        return None
    expected = {
        "failed": (True, "sync-operation-failed", "os.open+os.fsync-directory-v1"),
        "not-attempted": (False, "not-attempted", "not-attempted"),
        "synced": (True, None, "os.open+os.fsync-directory-v1"),
        "unsupported": (
            True,
            "unsupported-by-platform-or-filesystem",
            "os.open+os.fsync-directory-v1",
        ),
    }
    if (
        status not in expected
        or type(value["attempted"]) is not bool
        or (value["error_category"] is not None and type(value["error_category"]) is not str)
        or type(value["mechanism"]) is not str
        or tuple(value[key] for key in ("attempted", "error_category", "mechanism"))
        != expected[status]
    ):
        return None
    if published and status not in {"synced", "unsupported"}:
        return None
    return dict(value)


def _sync_set(value: object, *, published: bool) -> dict[str, object] | None:
    if type(value) is not dict or set(value) != {"final_parent", "rollback_parent", "stage"}:
        return None
    final_parent = _sync(value["final_parent"], published=published)
    stage = _sync(value["stage"], published=published)
    rollback_parent = _sync(value["rollback_parent"], published=False)
    if (
        final_parent is None
        or stage is None
        or rollback_parent is None
        or (published and rollback_parent["status"] != "not-attempted")
    ):
        return None
    return {
        "final_parent": final_parent,
        "rollback_parent": rollback_parent,
        "stage": stage,
    }


def _failed_publication_posture(value: object) -> dict[str, object] | None:
    if type(value) is not dict or set(value) != {
        "final_files_present",
        "rename_attempts",
        "retries",
        "state",
        "sync",
    }:
        return None
    sync = _sync_set(value["sync"], published=False)
    if (
        value["final_files_present"] is not False
        or type(value["rename_attempts"]) is not int
        or value["rename_attempts"] not in {0, 1}
        or type(value["retries"]) is not int
        or value["retries"] != 0
        or value["state"] not in {"absent", "rolled-back", "indeterminate"}
        or sync is None
    ):
        return None
    return {
        "final_files_present": False,
        "rename_attempts": value["rename_attempts"],
        "retries": 0,
        "state": value["state"],
        "sync": sync,
    }


def _published_witness_posture(value: object) -> dict[str, object] | None:
    if type(value) is not dict or set(value) != {
        "files",
        "final_files_present",
        "raw_hashes",
        "rename_attempts",
        "retries",
        "state",
        "sync",
    }:
        return None
    hashes = value["raw_hashes"]
    sync = _sync_set(value["sync"], published=True)
    if (
        value["files"] != "2/2"
        or value["final_files_present"] is not True
        or type(hashes) is not dict
        or set(hashes) != {
            "receipt_payload_sha256",
            "receipt_raw_sha256",
            "witness_payload_sha256",
            "witness_raw_sha256",
        }
        or not all(_digest(digest) for digest in hashes.values())
        or type(value["rename_attempts"]) is not int
        or value["rename_attempts"] != 1
        or type(value["retries"]) is not int
        or value["retries"] != 0
        or value["state"] != "published"
        or sync is None
    ):
        return None
    return {
        "files": "2/2",
        "final_files_present": True,
        "rename_attempts": 1,
        "retries": 0,
        "state": "published",
        "sync": sync,
    }


def _p47_failure_posture(p47: object, summary: object) -> dict[str, object]:
    """Filter a terminal failure to P43/P47's public failure vocabulary."""

    absent = {
        "final_files_present": False,
        "rename_attempts": 0,
        "retries": 0,
        "state": "absent",
        "sync": {
            key: {
                "attempted": False,
                "error_category": "not-attempted",
                "mechanism": "not-attempted",
                "status": "not-attempted",
            }
            for key in ("final_parent", "rollback_parent", "stage")
        },
    }
    if (
        type(summary) is not dict
        or summary.get("schema") != getattr(p47, "SUMMARY_SCHEMA", None)
    ):
        return {
            "source": "pulse-47",
            "failure_code": "P52-P47-SUMMARY",
            "witness_publication": absent,
        }
    if set(summary) == {"failure_code", "schema", "witness_publication"}:
        witness = _failed_publication_posture(summary["witness_publication"])
        code = summary["failure_code"]
        if (
            witness is not None
            and type(code) is str
            and re.fullmatch(r"P(?:47|51)-[A-Z-]+", code) is not None
        ):
            return {
                "source": "pulse-47",
                "failure_code": code,
                "witness_publication": witness,
            }
    if set(summary) == {
        "outcome",
        "publication_outcome",
        "schema",
        "witness_publication",
    }:
        captured = summary["publication_outcome"]
        witness = _published_witness_posture(summary["witness_publication"])
        if (
            summary["outcome"] == "published"
            and type(captured) is dict
            and set(captured) == {"failure_code", "kind", "publication"}
            and captured["kind"] == "failed"
            and type(captured["failure_code"]) is str
            and re.fullmatch(r"P43-[A-Z-]+", captured["failure_code"]) is not None
            and witness is not None
        ):
            publication = _failed_publication_posture(captured["publication"])
            if publication is not None:
                return {
                    "source": "pulse-43",
                    "failure_code": captured["failure_code"],
                    "publication": publication,
                    "witness_publication": witness,
                }
    return {
        "source": "pulse-47",
        "failure_code": "P52-P47-SUMMARY",
        "witness_publication": absent,
    }


def _published_terminal_summary(
    p43: object,
    p47: object,
    summary: object,
    p43_root: Path,
    witness_root: Path,
) -> bool:
    """Require independently verified P43 and P47 published shapes."""

    if (
        type(summary) is not dict
        or set(summary) != {
            "outcome",
            "publication_outcome",
            "schema",
            "witness_publication",
        }
        or summary["outcome"] != "published"
        or summary["schema"] != getattr(p47, "SUMMARY_SCHEMA", None)
    ):
        return False
    captured = summary["publication_outcome"]
    witness = _published_witness_posture(summary["witness_publication"])
    if (
        type(captured) is not dict
        or set(captured) != {
            "kind",
            "publication",
            "ordered_execution",
            "public_self_validation",
        }
        or captured["kind"] != "published"
        or witness is None
    ):
        return False
    publication = captured["publication"]
    ordered = captured["ordered_execution"]
    validation = captured["public_self_validation"]
    if (
        type(publication) is not dict
        or set(publication)
        != {"final_files", "raw_hashes", "rename_attempts", "retries", "sync"}
        or publication["final_files"] != "2/2"
        or type(publication["raw_hashes"]) is not dict
        or set(publication["raw_hashes"])
        != {
            "receipt_payload_sha256",
            "receipt_raw_sha256",
            "result_payload_sha256",
            "result_raw_sha256",
        }
        or not all(_digest(digest) for digest in publication["raw_hashes"].values())
        or type(publication["rename_attempts"]) is not int
        or publication["rename_attempts"] != 1
        or type(publication["retries"]) is not int
        or publication["retries"] != 0
        or _sync_set(publication["sync"], published=True) is None
        or type(ordered) is not dict
        or set(ordered)
        != {
            "attempted_gate_count",
            "catalog_gate_count",
            "completed_gate_count",
            "terminal_event_kind",
            "terminal_outcome",
        }
        or ordered["terminal_event_kind"] != "terminal-stop"
        or ordered["terminal_outcome"] != "completed"
        or type(validation) is not dict
    ):
        return False
    try:
        p43_hashes = p43.verify_publication_directory(p43_root)
        witness_hashes = p47.verify_witness_directory(witness_root)
    except (p43.PublicFailure, p47.WitnessFailure, OSError):
        return False
    return (
        p43_hashes == publication["raw_hashes"]
        and witness_hashes == summary["witness_publication"]["raw_hashes"]
    )


def _validate_materialization_summary(p51: object, summary: object) -> None:
    if type(summary) is not dict or set(summary) != {
        "schema",
        "case_count",
        "coverage_domains_closed",
        "coverage_interactions_closed",
        "directory_sync_posture",
        "directory_sync_records",
        "logical_retries",
        "residue_count",
        "diagnostic_execution",
    }:
        _raise(p51, "P52-MATERIALIZATION-SUMMARY")
    if (
        summary["schema"] != "ferris.pulse-35-corpus-materialization-summary/v1"
        or summary["case_count"] != 70
        or summary["coverage_domains_closed"] != "18/18"
        or summary["coverage_interactions_closed"] != "8/8"
        or summary["directory_sync_posture"] not in {"synced", "unsupported"}
        or type(summary["directory_sync_records"]) is not list
        or not summary["directory_sync_records"]
        or summary["logical_retries"] != 0
        or summary["residue_count"] != 0
        or summary["diagnostic_execution"] is not False
    ):
        _raise(p51, "P52-MATERIALIZATION-SUMMARY")


def _validate_verification_summary(p51: object, summary: object) -> None:
    if type(summary) is not dict or set(summary) != {
        "case_count",
        "coverage_domains_closed",
        "coverage_interactions_closed",
        "fresh_process_reload",
        "residue_count",
        "logical_retries",
        "directory_sync_records_validated",
    }:
        _raise(p51, "P52-PRIVATE-VERIFIER")
    if (
        summary["case_count"] != 70
        or summary["coverage_domains_closed"] != "18/18"
        or summary["coverage_interactions_closed"] != "8/8"
        or summary["fresh_process_reload"] is not True
        or summary["residue_count"] != 0
        or summary["logical_retries"] != 0
        or type(summary["directory_sync_records_validated"]) is not int
        or summary["directory_sync_records_validated"] <= 0
    ):
        _raise(p51, "P52-PRIVATE-VERIFIER")


def _validate_materialized_descriptor_root(
    p51: object, descriptor_root: Path, runtime_root: Path
) -> tuple[tuple[object, ...], int]:
    """Stage P51's descriptor execution API over P35's 4.6-MiB exact manifest.

    Pulse 51's public executor binds an intentionally smaller four-MiB
    descriptor JSON ceiling.  Exact P35 output is slightly larger because it
    carries its complete derived catalog.  P35's exact private verifier has
    already authenticated every byte and semantic witness; this narrow stage
    retains P51's root confinement, role/input, aggregate, topology, and
    dispatch descriptor checks with an explicit eight-MiB bounded manifest
    ceiling rather than weakening either sealed release.
    """

    root = _runtime_path(
        p51,
        runtime_root,
        descriptor_root,
        "P52-DESCRIPTOR-ROOT",
        require_directory=True,
    )
    try:
        with os.scandir(root) as directory:
            entries = {entry.name for entry in directory}
    except OSError as error:
        raise p51.ExecutorFailure("P52-DESCRIPTOR-ROOT") from error
    if entries != {"artifacts", "case-manifest.json", "coverage-manifest.json"}:
        _raise(p51, "P52-DESCRIPTOR-ROOT")
    manifest_path = root / "case-manifest.json"
    manifest_raw = p51._safe_regular_bytes(
        manifest_path, "P52-DESCRIPTOR-MANIFEST", maximum=8_388_608
    )
    manifest = p51._read_json(
        manifest_path, "P52-DESCRIPTOR-MANIFEST", maximum=8_388_608
    )
    if (
        manifest.get("schema") != "ferris.pulse-35-corpus-case-manifest/v1"
        or manifest.get("derivation") != "hmac-sha256-seed-key-domain-purpose-counter-v1"
        or manifest.get("logical_case_max") != 512
        or manifest.get("required_case_count") != 70
        or manifest.get("case_count") != 70
        or manifest.get("diagnostic_execution") is not False
        or manifest.get("product_files_modified") is not False
        or manifest.get("logical_retries") != 0
        or type(manifest.get("cases")) is not list
        or len(manifest["cases"]) != 70
    ):
        _raise(p51, "P52-DESCRIPTOR-MANIFEST")
    coverage = p51._read_json(root / "coverage-manifest.json", "P52-DESCRIPTOR-COVERAGE")
    if (
        coverage.get("schema") != "ferris.pulse-35-corpus-coverage-manifest/v1"
        or coverage.get("case_manifest_sha256") != p51._digest(manifest_raw)
        or coverage.get("case_count") != 70
        or coverage.get("coverage_domains_closed") != "18/18"
        or coverage.get("coverage_interactions_closed") != "8/8"
        or coverage.get("diagnostic_execution") is not False
        or coverage.get("product_files_modified") is not False
        or coverage.get("logical_retries") != 0
        or type(coverage.get("derived_catalog")) is not dict
    ):
        _raise(p51, "P52-DESCRIPTOR-COVERAGE")
    descriptors: list[object] = []
    p35_to_p51_projection_variances = 0
    expected_artifacts: set[str] = set()
    tokens: set[str] = set()
    for ordinal, case in enumerate(manifest["cases"], start=1):
        if type(case) is not dict or case.get("ordinal") != ordinal:
            _raise(p51, "P52-DESCRIPTOR-ORDER")
        case_id = case.get("case_id")
        order_token = case.get("order_token")
        profile_token = case.get("profile_token")
        if (
            type(case_id) is not str
            or type(order_token) is not str
            or type(profile_token) is not str
            or p51.TOKEN.fullmatch(case_id) is None
            or p51.TOKEN.fullmatch(order_token) is None
            or p51.TOKEN.fullmatch(profile_token) is None
            or {case_id, order_token, profile_token} & tokens
        ):
            _raise(p51, "P52-DESCRIPTOR-ORDER")
        tokens.update({case_id, order_token, profile_token})
        execution = case.get("execution")
        if type(execution) is not dict or set(execution) != {"mode", "format", "expected"}:
            _raise(p51, "P52-DESCRIPTOR-CASE")
        mode = execution["mode"]
        output_format = execution["format"]
        expected = p51._expected_result(execution["expected"])
        is_final = ordinal == 70
        if is_final:
            if (
                mode != "no-launch"
                or output_format != "no-launch"
                or expected["result_class"] != "blocked"
                or case.get("external_prerequisite") != "external-immutable-binary-freeze"
            ):
                _raise(p51, "P52-DESCRIPTOR-NO-LAUNCH")
        elif mode != "launch-ready" or output_format not in {"json", "human"}:
            _raise(p51, "P52-DESCRIPTOR-TOPOLOGY")
        if output_format == "human" and expected["result_class"] not in {"success", "difference"}:
            _raise(p51, "P52-DESCRIPTOR-RESULT")
        before_role = case.get("before")
        after_role = case.get("after")
        before = p51._role_path(root, before_role, is_final=is_final)
        after = p51._role_path(root, after_role, is_final=is_final)
        for role in (before_role, after_role):
            if type(role) is not dict:
                _raise(p51, "P52-DESCRIPTOR-ROLE")
            target = role.get("target")
            if type(target) is str:
                expected_artifacts.add(target)
        if type(case.get("semantic_witnesses")) is not dict:
            _raise(p51, "P52-DESCRIPTOR-CASE")
        dispatch_expected = expected
        if not is_final:
            if before is None or after is None:
                _raise(p51, "P52-DESCRIPTOR-ROLE")
            before_outcome, before_profile = p51._semantic_profile(before)
            after_outcome, after_profile = p51._semantic_profile(after)
            semantics = p51.frozen_profile_diff.derive_profile_diff(
                str(before),
                before_outcome.result_class,
                before_profile,
                str(after),
                after_outcome.result_class,
                after_profile,
            )
            dispatch_expected = {
                "result_class": semantics.result_class,
                **p51.RESULT_MAP[semantics.result_class],
            }
            if dispatch_expected != expected:
                p35_to_p51_projection_variances += 1
        descriptors.append(
            p51.Descriptor(ordinal, case_id, output_format, dispatch_expected, before, after, mode)
        )
    artifacts = root / "artifacts"
    try:
        with os.scandir(artifacts) as directory:
            actual_artifacts = {"artifacts/" + entry.name for entry in directory}
    except OSError as error:
        raise p51.ExecutorFailure("P52-DESCRIPTOR-ARTIFACT") from error
    # P35's sealed verifier has already recomputed its independently defined
    # artifact aggregate.  P51's synthetic descriptor aggregate is a distinct
    # framing and is therefore not substituted for the exact P35 aggregate.
    if actual_artifacts - expected_artifacts:
        _raise(p51, "P52-DESCRIPTOR-ARTIFACT")
    if sum(item.execution_mode == "launch-ready" for item in descriptors) != 69:
        _raise(p51, "P52-DESCRIPTOR-TOPOLOGY")
    if sum(item.execution_mode == "no-launch" for item in descriptors) != 1:
        _raise(p51, "P52-DESCRIPTOR-TOPOLOGY")
    return tuple(descriptors), p35_to_p51_projection_variances


def _prepare_terminal(
    p51: object, runtime_root: Path, repo_root: Path
) -> tuple[object, Path, Path, Path]:
    parent = runtime_root / TERMINAL_DIRECTORY
    try:
        os.mkdir(parent, 0o700)
    except OSError as error:
        raise p51.ExecutorFailure("P52-TERMINAL-ROOT") from error
    try:
        metadata = os.lstat(parent)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            _raise(p51, "P52-TERMINAL-ROOT")
    except p51.ExecutorFailure:
        raise
    except OSError as error:
        raise p51.ExecutorFailure("P52-TERMINAL-ROOT") from error
    return (
        p51.TerminalPulse47Once(repo_root, parent),
        parent / P43_FINAL_DIRECTORY,
        parent / WITNESS_FINAL_DIRECTORY,
        parent,
    )


def _run_loaded(
    p51: object,
    p39: object,
    p41: object,
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
    controls: _QualificationControls | None,
) -> OrderedMaterializationResult:
    events: list[dict[str, object]] = []
    private_record = _private_record()
    current_gate = P50_GATE_IDS[0]
    p43: object | None = None
    p47: object | None = None
    namespace: Path | None = None
    private_started = False
    failure: str | None = None
    try:
        p43, p45, p47 = p51.load_terminal_dependencies(repo_root)
        runtime_root = p51._safe_runtime_root(private_runtime_root)
        if p45.PLATFORM_GATES != {
            "windows-x86_64": P50_GATE_IDS[1],
            "ubuntu-24.04-x86_64": P50_GATE_IDS[2],
        }:
            _raise(p51, "P52-P45-PLATFORM-BINDING")
        custodies = p51._normalize_custodies(retained_custodies)
        _audit_only_public_custody(p51, runtime_root, custodies, p27_cycle_root)
        expectations = p51.P33_EXPECTATIONS if controls is None else controls.expectations
        if set(expectations) != set(CANONICAL_PLATFORMS):
            _raise(p51, "P52-P33-BINARY-PLATFORM")
        executable_by_platform = {
            platform: p51._verify_custody_binary(
                custodies[platform], expectations[platform], runtime_root
            )
            for platform in CANONICAL_PLATFORMS
        }

        _assert_private_namespace_absent(p51, runtime_root, private_record)
        _verify_public_prelaunch_custody(
            p51,
            p39,
            p41,
            runtime_root,
            p39_checkout_root,
            p41_final_root,
            private_record,
        )
        events.append(p51._validation_event("public-catalog-prevalidation", 5))
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[1]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p51._bridge_p44_once(
            p45, repo_root, custodies["windows-x86_64"], "windows-x86_64", runtime_root
        )
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[2]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p51._bridge_p44_once(
            p45,
            repo_root,
            custodies["ubuntu-24.04-x86_64"],
            "ubuntu-24.04-x86_64",
            runtime_root,
        )
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[3]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p51._run_p27_once(
            runtime_root, p27_cycle_root, p51.load_p27_exact_runner(repo_root)
        )
        private_record["p27_cycle_retention"] = "retained-private-cycle-root"
        private_record["p27_invocations"] = 1
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[4]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p31_summary = p51.verify_bound_contract(repo_root)
        if p31_summary != {
            "artifact_count": 9,
            "positive_fixture_count": 6,
            "mutation_control_count": 33,
            "public_input_checks": 39,
        }:
            _raise(p51, "P52-P31-CONTROL-COUNT")
        events.append(p51._validation_event("public-input-contract", 39))
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[5]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p35_summary = p51.verify_p35_p37_custody(repo_root)
        if p35_summary != {
            "bound_file_count": 11,
            "p35_release_tree_file_count": 10,
            "machine_schema_count": 1,
            "canonical_lf_file_count": 11,
            "git_clean_checks": 11,
        }:
            _raise(p51, "P52-P35-CUSTODY-COUNT")
        materializer, verifier = load_p35_materializer_and_verifier(repo_root)
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[6]
        namespace = _begin_private_launch(p51, runtime_root)
        private_started = True
        private_record["private_launch_started"] = True
        seed = secrets.token_bytes(32) if controls is None else controls.seed_bytes
        seed_path = _write_seed_atomically(p51, namespace, seed)
        private_record["seed_byte_count"] = 32
        descriptor_root = namespace / DESCRIPTOR_DIRECTORY
        if controls is not None and controls.force_materializer_destination_conflict:
            try:
                conflict = os.open(
                    descriptor_root,
                    os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, "O_BINARY", 0),
                    stat.S_IRUSR | stat.S_IWUSR,
                )
                os.fsync(conflict)
                os.close(conflict)
            except OSError as error:
                raise p51.ExecutorFailure("P52-MATERIALIZATION") from error
        private_record["materializer_invocations"] = 1
        try:
            materialization_summary = materializer.materialize(seed_path, descriptor_root)
        except materializer.MaterializationError as error:
            raise p51.ExecutorFailure("P52-MATERIALIZATION") from error
        _validate_materialization_summary(p51, materialization_summary)
        private_record["verifier_invocations"] = 1
        try:
            verification_summary = verifier.verify(descriptor_root, seed_path)
        except verifier.MaterializationError as error:
            raise p51.ExecutorFailure("P52-PRIVATE-VERIFIER") from error
        _validate_verification_summary(p51, verification_summary)
        commitment = materializer.seed_commitment(seed)
        if type(commitment) is not str or not commitment.startswith("sha256:"):
            _raise(p51, "P52-SEED-COMMITMENT")
        private_record["seed_commitment_sha256"] = commitment
        _remove_seed(p51, seed_path, private_record)
        descriptors, projection_variances = _validate_materialized_descriptor_root(
            p51, descriptor_root, runtime_root
        )
        private_record["p35_to_p51_semantic_projection_variance_count"] = projection_variances
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[7]
        runner = p51._subprocess_process_runner if controls is None else controls.process_runner
        for descriptor in descriptors:
            if descriptor.execution_mode == "no-launch":
                for platform in CANONICAL_PLATFORMS:
                    private_record["no_launch_records"].append(
                        {
                            "case_id": descriptor.case_id,
                            "ordinal": descriptor.ordinal,
                            "platform": platform,
                            "process_launched": False,
                            "reason": "blocked-no-launch-external-immutable-binary-freeze",
                        }
                    )
                continue
            windows = p51._run_descriptor(
                descriptor,
                "windows-x86_64",
                executable_by_platform["windows-x86_64"],
                runtime_root,
                runner,
            )
            private_record["platform_records"]["windows-x86_64"].append(windows)
            private_record["process_counts"]["windows-x86_64"] += 1
            ubuntu = p51._run_descriptor(
                descriptor,
                "ubuntu-24.04-x86_64",
                executable_by_platform["ubuntu-24.04-x86_64"],
                runtime_root,
                runner,
            )
            private_record["platform_records"]["ubuntu-24.04-x86_64"].append(ubuntu)
            private_record["process_counts"]["ubuntu-24.04-x86_64"] += 1
            if windows["result"]["semantic_projection"] != ubuntu["result"]["semantic_projection"]:
                private_record["first_mismatch_ordinal"] = descriptor.ordinal
                _raise(p51, "P52-FIRST-TARGET-MISMATCH")
        if (
            private_record["process_counts"]
            != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
            or len(private_record["no_launch_records"]) != 2
        ):
            _raise(p51, "P52-TOPOLOGY-ACCOUNTING")
        _cleanup_private_launch(p51, namespace, private_record)
        terminal, p43_root, witness_root, terminal_parent = _prepare_terminal(
            p51, runtime_root, repo_root
        )
    except (
        p51.ExecutorFailure,
        p51.P31Failure,
        p51.CustodyFailure,
        p51.DependencyFailure,
        SealedDependencyFailure,
        OSError,
        subprocess.SubprocessError,
    ) as error:
        failure = _failure_code(error, "P52-OPERATION")

    if failure is not None:
        if private_started and namespace is not None:
            try:
                _cleanup_private_launch(p51, namespace, private_record)
            except p51.ExecutorFailure as cleanup_error:
                failure = cleanup_error.code
        return _terminal_result(p43, events, current_gate, failure, private_record)

    events.append(_event(P50_GATE_IDS[7], "terminal-stop", "completed"))
    p43.validate_catalog(_catalog())
    p43.validate_events(P50_GATE_IDS, events)
    private_record["execution_outcome"] = "completed"
    publication = _not_attempted_publication()
    result = OrderedMaterializationResult(_catalog(), events, publication, private_record)
    private_record["terminal_p47_invocation_count"] = 1
    try:
        terminal_summary = p51.invoke_terminal_pulse47_once(
            terminal, result, p43_root, witness_root
        )
    except (p43.PublicFailure, p47.WitnessFailure):
        terminal_summary = None
    if _published_terminal_summary(p43, p47, terminal_summary, p43_root, witness_root):
        publication.update(
            _terminal_publication(
                "published",
                {
                    "p43_result": "published-and-verified",
                    "p47_witness": "published-and-verified",
                },
            )
        )
        private_record["outcome"] = "published"
        private_record["publication_disposition"] = "published"
        private_record["terminal_p47_outcome"] = "published"
        private_record["terminal_publication_cleanup"] = "retained-published"
        return result

    publication.update(
        _terminal_publication(
            "invalid-publication-integrity",
            _p47_failure_posture(p47, terminal_summary),
        )
    )
    private_record["outcome"] = "invalid-publication-integrity"
    private_record["publication_disposition"] = "invalid-publication-integrity"
    private_record["terminal_p47_outcome"] = (
        terminal_summary.get("outcome") if type(terminal_summary) is dict else None
    )
    try:
        _cleanup_terminal_publication(
            p51, terminal_parent, p43_root, witness_root, private_record
        )
    except (p51.ExecutorFailure, PermissionError, OSError):
        raise TerminalPublicationCleanupIndeterminate() from None
    return result


def _run(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
    controls: _QualificationControls | None,
) -> OrderedMaterializationResult:
    try:
        p51 = load_pulse51(repo_root)
        p39, p41 = load_p39_and_p41(repo_root)
    except SealedDependencyFailure as error:
        private_record = _private_record()
        return _terminal_result(None, [], P50_GATE_IDS[0], error.code, private_record)
    return _run_loaded(
        p51,
        p39,
        p41,
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        retained_custodies,
        controls,
    )


def run_ordered_materialization_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
) -> OrderedMaterializationResult:
    """Execute P52's fixed public-to-private order with no runtime injection.

    The caller supplies a fresh P39 checkout root, a fresh absent P41 final
    root, and retained P44 custody inputs.  P52 invokes and validates exact
    P39/P41 itself before constructing gate one.  This surface accepts no
    prelaunch event, seed, descriptor root, materializer, launcher,
    expectation, fake binary, or trust mode.
    """

    return _run(
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        retained_custodies,
        None,
    )


def _run_qualification_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
    *,
    seed_bytes: bytes,
    process_runner: Callable[[object], object],
    expectations: Mapping[str, object],
    force_materializer_destination_conflict: bool = False,
) -> OrderedMaterializationResult:
    """Private fake-only qualification seam; never a production API."""

    return _run(
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        retained_custodies,
        _QualificationControls(
            seed_bytes,
            process_runner,
            expectations,
            force_materializer_destination_conflict,
        ),
    )


__all__ = [
    "OrderedMaterializationResult",
    "TerminalPublicationCleanupIndeterminate",
    "run_ordered_materialization_executor",
]
