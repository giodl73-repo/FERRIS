"""Pulse 88 witness-preserving terminal wrapper over exact Pulse 87."""

from __future__ import annotations

import hashlib
import os
import socket
import stat
import sys
import threading
import uuid
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Callable

P43_CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
P58_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
P58_GATE_IDS = (
    "pulse-41-pulse-39-public-custody",
    "sealed-predecessor-binding",
    "windows-capability-build-custody",
    "ubuntu-capability-build-custody",
    "exact-adapter-preflight",
    "pulse-31-public-input",
    "pulse-35-pulse-37-normalization",
    "bounded-materialization",
    "descriptor-validation",
    "bounded-process-exit-search",
)
TERMINAL_ROOT_SUFFIX = ".pulse88-terminal-publication"
P43_FINAL_DIRECTORY = "pulse-88-p43-result"
WITNESS_FINAL_DIRECTORY = "pulse-88-p47-witness"
TERMINAL_CLEANUP_FATAL_SCHEMA = (
    "ferris.pulse-88-terminal-publication-cleanup-indeterminate/v1"
)
TRANSFER_DESCRIPTOR_SCHEMA = "ferris.pulse-88-public-transfer-descriptor/v1"
TERMINAL_ROOT_POLICY = "fresh-sibling-of-private-runtime-root"
LOCAL_SEALED_DEPENDENCIES_SHA256 = (
    "sha256:8223a219fcfaa4217930904f23097c8d4af57794d28197e44d3c9b2f9c6dd094"
)
_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX = (
    "ferris.pulse-88.local-sealed-dependencies.runtime"
)
_LOCAL_SLOT_MISSING = object()
_CROSS_INSTANCE_REENTRY_STATE_KEY = (
    "_ferris_p88_cross_instance_reentry_advisory_state_v1"
)


class _Pulse88LinuxLockManager:
    def __init__(self) -> None:
        self._active_states: dict[int, object] = {}
        self._guard = threading.Lock()
        self._fork_hook_registrations = 0
        if os.name == "posix" and sys.platform.startswith("linux"):
            register_at_fork = getattr(os, "register_at_fork", None)
            if not callable(register_at_fork):
                raise RuntimeError("P88-LOCAL-SEALED-FORK-HOOK")
            register_at_fork(after_in_child=self._after_in_child)
            self._fork_hook_registrations += 1

    def _cross_instance_reentry_state(self) -> dict[str, object]:
        state = getattr(threading, _CROSS_INSTANCE_REENTRY_STATE_KEY, None)
        guard = state.get("guard") if type(state) is dict else None
        owners = state.get("owners") if type(state) is dict else None
        if callable(getattr(guard, "acquire", None)) and callable(
            getattr(guard, "release", None)
        ) and type(owners) is dict:
            return state
        fresh = {"guard": threading.Lock(), "owners": {}}
        setattr(threading, _CROSS_INSTANCE_REENTRY_STATE_KEY, fresh)
        return fresh

    def advisory_conflict(self, lock_name: str, owner_pid: int, owner_thread_id: int) -> bool:
        state = self._cross_instance_reentry_state()
        key = (lock_name, owner_pid, owner_thread_id)
        with state["guard"]:
            return key in state["owners"]

    def advisory_mark(self, active_state: object) -> None:
        state = self._cross_instance_reentry_state()
        key = (
            getattr(getattr(active_state, "lock_state", None), "name", None),
            getattr(active_state, "owner_pid", None),
            getattr(active_state, "owner_thread_id", None),
        )
        with state["guard"]:
            state["owners"][key] = getattr(active_state, "owner_token", active_state)

    def advisory_clear(self, active_state: object) -> None:
        state = self._cross_instance_reentry_state()
        key = (
            getattr(getattr(active_state, "lock_state", None), "name", None),
            getattr(active_state, "owner_pid", None),
            getattr(active_state, "owner_thread_id", None),
        )
        owner_token = getattr(active_state, "owner_token", active_state)
        with state["guard"]:
            if state["owners"].get(key) is owner_token:
                state["owners"].pop(key, None)

    def advisory_snapshot(self) -> tuple[tuple[str, int, int], ...]:
        state = self._cross_instance_reentry_state()
        with state["guard"]:
            keys = tuple(state["owners"])
        return tuple(sorted(keys))

    def register_active_lock_state(self, active_state: object) -> None:
        with self._guard:
            self._active_states[id(active_state)] = active_state

    def unregister_active_lock_state(self, active_state: object) -> None:
        with self._guard:
            self._active_states.pop(id(active_state), None)

    def _after_in_child(self) -> None:
        with self._guard:
            states = tuple(self._active_states.values())
            self._active_states.clear()
        for active_state in states:
            try:
                self.advisory_clear(active_state)
                owner_token = getattr(active_state, "owner_token", None)
                if owner_token is not None:
                    owner_token.live = False
                lock_state = getattr(active_state, "lock_state", None)
                if lock_state is not None:
                    handle = getattr(lock_state, "handle", None)
                    lock_state.handle = None
                    if isinstance(handle, socket.socket):
                        handle.close()
                if hasattr(active_state, "depth"):
                    active_state.depth = 0
            except BaseException:
                os._exit(97)


_P88_INTERNAL_LOCK_MANAGER = _Pulse88LinuxLockManager()


@dataclass(frozen=True)
class WitnessPreservingCapabilityMaterializationResult:
    """P43-safe ordered events plus Pulse 88 terminal publication disposition."""

    catalog: dict[str, object]
    events: list[dict[str, object]]
    publication: dict[str, object]
    transfer_descriptor: dict[str, object] | None
    private_record: dict[str, object]


@dataclass(frozen=True)
class _QualificationControls:
    """Private harmless-fake seam layered strictly after exact P87 ordering."""

    seed_bytes: bytes
    p27_runner: Callable[[Path], dict[str, object]]
    p56: object
    open_wsl: Callable[[Path, str, object], object]
    terminal_call: Callable[[object, object, Path, Path], object] | None


class P88Failure(RuntimeError):
    """A bounded Pulse 88 failure outside exact Pulse 87 execution."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class _LocalSealedBootstrapFailure(RuntimeError):
    """The sibling sealed dependency binder could not be loaded safely."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


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


def _local_sealed_dependencies_path() -> Path:
    try:
        return Path(__file__).resolve(strict=True).with_name("sealed_dependencies.py")
    except OSError as error:
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-PATH") from error


def _safe_local_regular(path: Path, code: str, maximum: int = 4_194_304) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise _LocalSealedBootstrapFailure(code)
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except _LocalSealedBootstrapFailure:
        raise
    except OSError as error:
        raise _LocalSealedBootstrapFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise _LocalSealedBootstrapFailure(code)
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > maximum:
                raise _LocalSealedBootstrapFailure(code)
        return bytes(content)
    except _LocalSealedBootstrapFailure:
        raise
    except OSError as error:
        raise _LocalSealedBootstrapFailure(code) from error
    finally:
        os.close(descriptor)


def _local_sealed_source_digest(content: bytes) -> str:
    return "sha256:" + hashlib.sha256(content).hexdigest()


def _verified_local_sealed_content(path: Path) -> bytes:
    content = _safe_local_regular(path, "P88-LOCAL-SEALED-IMPORT")
    if _local_sealed_source_digest(content) != LOCAL_SEALED_DEPENDENCIES_SHA256:
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-IDENTITY")
    return content


def _bind_local_sealed_lock_manager_module(module: ModuleType) -> ModuleType:
    if not isinstance(module, ModuleType):
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    try:
        module_path = Path(module.__file__).resolve(strict=True)
    except (AttributeError, OSError, TypeError) as error:
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE") from error
    if module_path != _local_sealed_dependencies_path():
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    binder = getattr(module, "_bind_internal_lock_manager", None)
    if not callable(binder):
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    binder(_P88_INTERNAL_LOCK_MANAGER)
    if getattr(module, "_P88_INTERNAL_LOCK_MANAGER", None) is not _P88_INTERNAL_LOCK_MANAGER:
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    return module


def _exec_local_sealed_module(path: Path, content: bytes) -> ModuleType:
    name = (
        f"{_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX}."
        f"{os.getpid()}.{uuid.uuid4().hex}"
    )
    if sys.modules.get(name) is not None:
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    module = ModuleType(name)
    module.__file__ = os.fspath(path)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[name] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except BaseException as error:
        current = sys.modules.get(name)
        if current is module:
            sys.modules.pop(name, None)
        else:
            raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE") from error
        if isinstance(error, _LocalSealedBootstrapFailure):
            raise
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-IMPORT") from error
    current = sys.modules.get(name)
    if current is not module:
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    sys.modules.pop(name, None)
    if not callable(getattr(module, "load_pulse87", None)) or not isinstance(
        getattr(module, "SealedDependencyFailure", None), type
    ):
        raise _LocalSealedBootstrapFailure("P88-LOCAL-SEALED-STATE")
    return _bind_local_sealed_lock_manager_module(module)


def _load_local_sealed_dependencies() -> ModuleType:
    path = _local_sealed_dependencies_path()
    content = _verified_local_sealed_content(path)
    return _exec_local_sealed_module(path, content)


def _catalog() -> dict[str, object]:
    return {"schema": P43_CATALOG_SCHEMA, "gate_ids": list(P58_GATE_IDS)}


def _event(gate_id: str, outcome: str) -> dict[str, object]:
    return {
        "classification": "ordered-execution",
        "event_kind": "terminal-stop",
        "gate_id": gate_id,
        "outcome": outcome,
        "schema": P58_EVENT_SCHEMA,
    }


def _terminal_publication(disposition: str, posture: dict[str, object]) -> dict[str, object]:
    return {
        "schema": "ferris.pulse-88-terminal-publication-disposition/v1",
        "disposition": disposition,
        "product_conclusion": None,
        "category_conclusion": None,
        "fix_conclusion": None,
        "posture": posture,
    }


def _not_attempted_publication() -> dict[str, object]:
    return _terminal_publication("not-attempted", {"state": "not-attempted"})


def _fallback_failure(code: str) -> WitnessPreservingCapabilityMaterializationResult:
    record = {
        "schema": "ferris.pulse-88-private-execution-record/v1",
        "outcome": "failed",
        "execution_outcome": "not-started",
        "p87_execution_outcome": "not-started",
        "publication_disposition": "not-attempted",
        "terminal_transfer": "not-created",
        "terminal_root_policy": TERMINAL_ROOT_POLICY,
        "terminal_root_absent_before_execution": None,
        "terminal_root_absent_before_publication": None,
        "terminal_runtime_absence_verified": False,
        "terminal_publication_cleanup": "not-attempted",
        "product_conclusion": None,
        "category_conclusion": None,
        "fix_conclusion": None,
        "failure_code": code,
    }
    return WitnessPreservingCapabilityMaterializationResult(
        _catalog(),
        [_event(P58_GATE_IDS[0], "failed")],
        _not_attempted_publication(),
        None,
        record,
    )


def _clone_private_record(p87_record: dict[str, object]) -> dict[str, object]:
    record = dict(p87_record)
    record["schema"] = "ferris.pulse-88-private-execution-record/v1"
    record["execution_outcome"] = record.get("outcome")
    record["p87_execution_outcome"] = record.get("outcome")
    record["publication_disposition"] = "not-attempted"
    record["terminal_transfer"] = "not-created"
    record["terminal_root_policy"] = TERMINAL_ROOT_POLICY
    record["terminal_root_absent_before_execution"] = None
    record["terminal_root_absent_before_publication"] = None
    record["terminal_runtime_absence_verified"] = False
    record["terminal_publication_cleanup"] = "not-attempted"
    return record


def _result(
    catalog: dict[str, object],
    events: list[dict[str, object]],
    publication: dict[str, object],
    transfer_descriptor: dict[str, object] | None,
    private_record: dict[str, object],
) -> WitnessPreservingCapabilityMaterializationResult:
    return WitnessPreservingCapabilityMaterializationResult(
        catalog, events, publication, transfer_descriptor, private_record
    )


def _hashes(value: object, keys: frozenset[str], p52: object) -> dict[str, str] | None:
    if type(value) is not dict or set(value) != set(keys):
        return None
    if not all(p52._digest(digest) for digest in value.values()):
        return None
    return {key: value[key] for key in sorted(keys)}


def _terminal_parent_shape(parent: Path, expected: frozenset[str]) -> bool:
    try:
        metadata = os.lstat(parent)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            return False
        with os.scandir(parent) as entries:
            names = frozenset(entry.name for entry in entries)
    except OSError:
        return False
    return names == expected


def _stage_root(root: Path, pulse: str) -> Path:
    return root.parent / f".{root.name}.{pulse}-stage"


def _transfer_descriptor(
    kind: str,
    *,
    result_hashes: dict[str, str] | None,
    witness_hashes: dict[str, str],
) -> dict[str, object]:
    counts: dict[str, int] = {"witness": 2, "total": 2}
    hashes: dict[str, dict[str, str]] = {"witness": dict(witness_hashes)}
    if result_hashes is not None:
        counts = {"result": 2, "witness": 2, "total": 4}
        hashes = {"result": dict(result_hashes), "witness": dict(witness_hashes)}
    return {
        "schema": TRANSFER_DESCRIPTOR_SCHEMA,
        "expected_public_tree_kind": kind,
        "exact_file_counts": counts,
        "verified_raw_payload_hashes": hashes,
    }


def _published_result_descriptor(
    p52: object,
    p43: object,
    p47: object,
    summary: object,
    terminal_parent: Path,
    p43_root: Path,
    witness_root: Path,
) -> dict[str, object] | None:
    if not p52._published_terminal_summary(p43, p47, summary, p43_root, witness_root):
        return None
    if not _terminal_parent_shape(
        terminal_parent, frozenset({p43_root.name, witness_root.name})
    ):
        return None
    try:
        result_hashes = _hashes(
            p43.verify_publication_directory(p43_root),
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "result_payload_sha256",
                    "result_raw_sha256",
                }
            ),
            p52,
        )
        witness_hashes = _hashes(
            p47.verify_witness_directory(witness_root),
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "witness_payload_sha256",
                    "witness_raw_sha256",
                }
            ),
            p52,
        )
    except (p43.PublicFailure, p47.WitnessFailure, OSError):
        return None
    if result_hashes is None or witness_hashes is None:
        return None
    return _transfer_descriptor(
        "result-and-witness",
        result_hashes=result_hashes,
        witness_hashes=witness_hashes,
    )


def _failure_witness_descriptor(
    p52: object,
    p43: object,
    p47: object,
    summary: object,
    terminal_parent: Path,
    p43_root: Path,
    witness_root: Path,
) -> tuple[dict[str, object], dict[str, object]] | None:
    posture = p52._p47_failure_posture(p47, summary)
    if posture.get("source") != "pulse-43":
        return None
    if (
        os.path.lexists(p43_root)
        or os.path.lexists(_stage_root(p43_root, "pulse-43"))
        or os.path.lexists(_stage_root(witness_root, "pulse-47"))
        or not _terminal_parent_shape(terminal_parent, frozenset({witness_root.name}))
    ):
        return None
    if type(summary) is not dict:
        return None
    witness = p52._published_witness_posture(summary.get("witness_publication"))
    if witness is None:
        return None
    expected_hashes = _hashes(
        summary.get("witness_publication", {}).get("raw_hashes")
        if type(summary.get("witness_publication")) is dict
        else None,
        frozenset(
            {
                "receipt_payload_sha256",
                "receipt_raw_sha256",
                "witness_payload_sha256",
                "witness_raw_sha256",
            }
        ),
        p52,
    )
    if expected_hashes is None:
        return None
    try:
        witness_hashes = _hashes(
            p47.verify_witness_directory(witness_root),
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "witness_payload_sha256",
                    "witness_raw_sha256",
                }
            ),
            p52,
        )
    except (p47.WitnessFailure, OSError):
        return None
    if witness_hashes is None or witness_hashes != expected_hashes:
        return None
    return (
        posture,
        _transfer_descriptor(
            "failure-witness-only",
            result_hashes=None,
            witness_hashes=witness_hashes,
        ),
    )


def _invalid_witness_posture(p52: object, p47: object, summary: object) -> dict[str, object]:
    return p52._p47_failure_posture(p47, summary)


def _derived_terminal_root(p51: object, private_runtime_root: Path) -> Path:
    try:
        runtime_root = p51._safe_runtime_root(private_runtime_root)
        # Terminal custody is native-host-only; WSL path translation belongs to
        # Pulse 86 and must fail closed rather than leak into this witness layer.
        parent = p51._safe_runtime_root(runtime_root.parent)
    except BaseException as error:
        raise P88Failure("P88-TERMINAL-ROOT") from error
    candidate = parent / f"{runtime_root.name}{TERMINAL_ROOT_SUFFIX}"
    if candidate == runtime_root or os.path.lexists(candidate):
        raise P88Failure("P88-TERMINAL-ROOT-NOT-FRESH")
    return candidate


def _create_terminal_parent(p51: object, terminal_parent: Path) -> Path:
    if os.path.lexists(terminal_parent):
        raise P88Failure("P88-TERMINAL-ROOT-NOT-FRESH")
    try:
        os.mkdir(terminal_parent, 0o700)
    except OSError as error:
        raise P88Failure("P88-TERMINAL-ROOT") from error
    try:
        return p51._safe_runtime_root(terminal_parent)
    except BaseException as error:
        raise P88Failure("P88-TERMINAL-ROOT") from error


def _pre_p87_failure_result(p87: object, p43: object, code: str) -> object:
    record = p87._private_record()
    return p87._terminal(p43, [], P58_GATE_IDS[0], code, record)


def _run(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    ubuntu_runtime_parent: str,
    controls: _QualificationControls | None,
) -> WitnessPreservingCapabilityMaterializationResult:
    try:
        sealed = _load_local_sealed_dependencies()
    except _LocalSealedBootstrapFailure as error:
        return _fallback_failure(error.code)

    try:
        p87, p52, _p86, p51, p43, p47 = sealed.load_pulse87(repo_root)
    except sealed.SealedDependencyFailure as error:
        return _fallback_failure(error.code)

    try:
        terminal_parent = _derived_terminal_root(p51, private_runtime_root)
    except P88Failure as error:
        p87_result = _pre_p87_failure_result(p87, p43, error.code)
        private_record = _clone_private_record(p87_result.private_record)
        private_record["failure_code"] = error.code
        return _result(
            p87_result.catalog,
            p87_result.events,
            _not_attempted_publication(),
            None,
            private_record,
        )

    try:
        if controls is None:
            p87_result = p87.run_ordered_capability_materialization_executor(
                repo_root,
                private_runtime_root,
                p27_cycle_root,
                p39_checkout_root,
                p41_final_root,
                ubuntu_runtime_parent,
            )
            terminal_call = p51.invoke_terminal_pulse47_once
        else:
            p87_result = p87._run_qualification_executor(
                repo_root,
                private_runtime_root,
                p27_cycle_root,
                p39_checkout_root,
                p41_final_root,
                seed_bytes=controls.seed_bytes,
                p27_runner=controls.p27_runner,
                p56=controls.p56,
                open_wsl=controls.open_wsl,
            )
            terminal_call = (
                p51.invoke_terminal_pulse47_once
                if controls.terminal_call is None
                else controls.terminal_call
            )
    except sealed.SealedDependencyFailure as error:
        return _fallback_failure(error.code)

    private_record = _clone_private_record(p87_result.private_record)
    private_record["terminal_root_absent_before_execution"] = True
    if private_record["p87_execution_outcome"] != "completed":
        return _result(
            p87_result.catalog,
            p87_result.events,
            _not_attempted_publication(),
            None,
            private_record,
        )

    if os.path.lexists(private_runtime_root):
        private_record["outcome"] = "failed"
        private_record["failure_code"] = "P88-P87-PRIVATE-RUNTIME-RESIDUE"
        return _result(
            p87_result.catalog,
            p87_result.events,
            _not_attempted_publication(),
            None,
            private_record,
        )
    if os.path.lexists(terminal_parent):
        private_record["outcome"] = "failed"
        private_record["failure_code"] = "P88-TERMINAL-ROOT-NOT-FRESH"
        return _result(
            p87_result.catalog,
            p87_result.events,
            _not_attempted_publication(),
            None,
            private_record,
        )
    private_record["terminal_runtime_absence_verified"] = True
    private_record["terminal_root_absent_before_publication"] = True

    try:
        terminal_parent = _create_terminal_parent(p51, terminal_parent)
    except P88Failure as error:
        private_record["outcome"] = "failed"
        private_record["failure_code"] = error.code
        return _result(
            p87_result.catalog,
            p87_result.events,
            _not_attempted_publication(),
            None,
            private_record,
        )
    terminal = p51.TerminalPulse47Once(repo_root, terminal_parent)
    p43_root = terminal_parent / P43_FINAL_DIRECTORY
    witness_root = terminal_parent / WITNESS_FINAL_DIRECTORY

    publication = _not_attempted_publication()
    result = _result(
        p87_result.catalog,
        p87_result.events,
        publication,
        None,
        private_record,
    )
    private_record["terminal_p47_invocation_count"] = 1
    try:
        terminal_summary = terminal_call(terminal, p87_result, p43_root, witness_root)
    except (p43.PublicFailure, p47.WitnessFailure):
        terminal_summary = None

    published = _published_result_descriptor(
        p52, p43, p47, terminal_summary, terminal_parent, p43_root, witness_root
    )
    if published is not None:
        publication.update(
            _terminal_publication(
                "published-result",
                {
                    "p43_result": "published-and-verified",
                    "p47_witness": "published-and-verified",
                },
            )
        )
        private_record["outcome"] = "published-result"
        private_record["publication_disposition"] = "published-result"
        private_record["terminal_transfer"] = "retained-result-and-witness"
        private_record["terminal_publication_cleanup"] = "retained-published-result"
        return _result(
            p87_result.catalog,
            p87_result.events,
            publication,
            published,
            private_record,
        )

    witnessed_failure = _failure_witness_descriptor(
        p52, p43, p47, terminal_summary, terminal_parent, p43_root, witness_root
    )
    if witnessed_failure is not None:
        posture, descriptor = witnessed_failure
        publication.update(_terminal_publication("published-failure-witness", posture))
        private_record["outcome"] = "published-failure-witness"
        private_record["publication_disposition"] = "published-failure-witness"
        private_record["terminal_transfer"] = "retained-failure-witness-only"
        private_record["terminal_publication_cleanup"] = "retained-failure-witness"
        return _result(
            p87_result.catalog,
            p87_result.events,
            publication,
            descriptor,
            private_record,
        )

    publication.update(
        _terminal_publication(
            "invalid-witness-publication",
            _invalid_witness_posture(p52, p47, terminal_summary),
        )
    )
    private_record["outcome"] = "invalid-witness-publication"
    private_record["publication_disposition"] = "invalid-witness-publication"
    private_record["terminal_p47_outcome"] = (
        terminal_summary.get("outcome") if type(terminal_summary) is dict else None
    )
    try:
        p52._cleanup_terminal_publication(
            p51, terminal_parent, p43_root, witness_root, private_record
        )
    except (p51.ExecutorFailure, PermissionError, OSError):
        raise TerminalPublicationCleanupIndeterminate() from None
    return _result(
        p87_result.catalog,
        p87_result.events,
        publication,
        None,
        private_record,
    )


def run_witness_preserving_capability_materialization_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    ubuntu_runtime_parent: str,
) -> WitnessPreservingCapabilityMaterializationResult:
    """Run exact Pulse 87 and terminalize only after Pulse 87 completes.

    The production surface mirrors Pulse 87's six concrete inputs. It accepts
    no seed, fake capability, callback, publication-root, or trust injection.
    """

    return _run(
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        ubuntu_runtime_parent,
        None,
    )


def _run_qualification_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    *,
    seed_bytes: bytes,
    p27_runner: Callable[[Path], dict[str, object]],
    p56: object,
    open_wsl: Callable[[Path, str, object], object],
    terminal_call: Callable[[object, object, Path, Path], object] | None = None,
) -> WitnessPreservingCapabilityMaterializationResult:
    """Private fake-only seam layered on exact Pulse 87 qualification."""

    return _run(
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        "/home/pulse88-qualification",
        _QualificationControls(seed_bytes, p27_runner, p56, open_wsl, terminal_call),
    )


__all__ = [
    "P88Failure",
    "TerminalPublicationCleanupIndeterminate",
    "WitnessPreservingCapabilityMaterializationResult",
    "run_witness_preserving_capability_materialization_executor",
]
