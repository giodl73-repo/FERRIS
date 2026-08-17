"""Pulse 76 ordered capability/materialization stage-bootstrap/worker-identity successor.

The production surface owns all ordering. It byte-binds sealed predecessors,
serializes every local and transitive sealed load with the final P74/P59
kernel-lock discipline, runs P39/P41 and every public P57 gate, then creates
one private P35 corpus and reuses Pulse 75's exact P57-compatible frozen
dispatch and semantic helpers over live P56 capabilities. It publishes
nothing.
"""
from __future__ import annotations

import hashlib
import os
import secrets
import socket
import stat
import sys
import threading
import uuid
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Callable


LOCAL_SEALED_DEPENDENCIES_SHA256 = (
    "sha256:6ef03c07692595814f433ef9c86cec943215deb68ad94086fd9f3aa1d0cffcbd"
)
_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX = (
    "ferris.pulse-76.local-sealed-dependencies.runtime"
)
_CROSS_INSTANCE_REENTRY_STATE_KEY = (
    "_ferris_p76_cross_instance_reentry_advisory_state_v1"
)


class _LocalSealedBootstrapFailure(RuntimeError):
    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class _Pulse76LinuxLockManager:
    def __init__(self) -> None:
        self._active_states: dict[int, object] = {}
        self._guard = threading.Lock()
        self._fork_hook_registrations = 0
        if os.name == "posix" and sys.platform.startswith("linux"):
            register_at_fork = getattr(os, "register_at_fork", None)
            if not callable(register_at_fork):
                raise RuntimeError("P76-LOCAL-SEALED-FORK-HOOK")
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


_P76_INTERNAL_LOCK_MANAGER = _Pulse76LinuxLockManager()


def _local_sealed_dependencies_path() -> Path:
    try:
        return Path(__file__).resolve(strict=True).with_name("sealed_dependencies.py")
    except OSError as error:
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-PATH") from error



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
    content = _safe_local_regular(path, "P76-LOCAL-SEALED-IMPORT")
    if _local_sealed_source_digest(content) != LOCAL_SEALED_DEPENDENCIES_SHA256:
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-IDENTITY")
    return content


def _bind_local_sealed_lock_manager_module(module: ModuleType) -> ModuleType:
    if not isinstance(module, ModuleType):
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
    try:
        module_path = Path(module.__file__).resolve(strict=True)
    except (AttributeError, OSError, TypeError) as error:
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE") from error
    if module_path != _local_sealed_dependencies_path():
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
    binder = getattr(module, "_bind_internal_lock_manager", None)
    if not callable(binder):
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
    binder(_P76_INTERNAL_LOCK_MANAGER)
    if getattr(module, "_P76_INTERNAL_LOCK_MANAGER", None) is not _P76_INTERNAL_LOCK_MANAGER:
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
    return module


def _exec_local_sealed_module(path: Path, content: bytes) -> ModuleType:
    name = (
        f"{_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX}."
        f"{os.getpid()}.{uuid.uuid4().hex}"
    )
    if sys.modules.get(name) is not None:
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
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
            raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE") from error
        if isinstance(error, _LocalSealedBootstrapFailure):
            raise
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-IMPORT") from error
    current = sys.modules.get(name)
    if current is not module:
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
    sys.modules.pop(name, None)
    required = (
        "load_exact_p35_materializer_and_verifier",
        "load_exact_p39_and_p41",
        "load_exact_p52_stage_reader",
        "load_exact_p75_stack",
        "release_identities",
        "_sealed_loading_lock",
    )
    if not isinstance(getattr(module, "SealedDependencyFailure", None), type) or any(
        not callable(getattr(module, name, None)) for name in required
    ):
        raise _LocalSealedBootstrapFailure("P76-LOCAL-SEALED-STATE")
    return _bind_local_sealed_lock_manager_module(module)



def _load_local_sealed_dependencies() -> ModuleType:
    path = _local_sealed_dependencies_path()
    content = _verified_local_sealed_content(path)
    return _exec_local_sealed_module(path, content)


_SEALED = _load_local_sealed_dependencies()
SealedDependencyFailure = _SEALED.SealedDependencyFailure
load_exact_p35_materializer_and_verifier = _SEALED.load_exact_p35_materializer_and_verifier
load_exact_p39_and_p41 = _SEALED.load_exact_p39_and_p41
load_exact_p52_stage_reader = _SEALED.load_exact_p52_stage_reader
load_exact_p75_stack = _SEALED.load_exact_p75_stack
release_identities = _SEALED.release_identities


P43_CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
P43_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
PRIVATE_NAMESPACE = ".pulse58-private-launch"
SEED_FILENAME = "seed.bin"
CANONICAL_PLATFORMS = ("windows-x86_64", "ubuntu-24.04-x86_64")
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
P39_CALLER_AUTHORITY_PRECONDITION = (
    "future-authority-supplied-fresh-anonymous-exact-cutoff-root"
)


class P58Failure(RuntimeError):
    """A bounded Pulse 58 failure that can be truthfully terminalized."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class IndeterminateCleanup(RuntimeError):
    """Cleanup uncertainty is fatal and has priority over all other outcomes."""

    code = "P58-INDETERMINATE-CLEANUP"

    def __init__(self) -> None:
        self.public_posture = {
            "schema": "ferris.pulse-58-cleanup-indeterminate/v1",
            "state": self.code,
            "publication": "not-attempted",
        }
        super().__init__(self.code)


@dataclass(frozen=True)
class OrderedCapabilityMaterializationResult:
    """Private execution accounting and privacy-safe ordered events only."""

    catalog: dict[str, object]
    events: list[dict[str, object]]
    private_record: dict[str, object]


@dataclass(frozen=True)
class _QualificationControls:
    """Unexported harmless-fake seam used only by sealed qualification."""

    seed_bytes: bytes
    p27_runner: Callable[[Path], dict[str, object]]
    p56: object
    open_wsl: Callable[[Path, str, ModuleType], object]


@dataclass
class _ExecutionState:
    """Mutable custody accounting shared with terminal cleanup."""

    windows_launches: int = 0


@dataclass(frozen=True)
class _LexicalInputIdentity:
    """A lexical no-follow snapshot of one P35 input state."""

    path: Path
    device: int | None
    inode: int | None
    size: int | None
    sha256: str | None
    state: str


@dataclass(frozen=True)
class _P58FrozenDescriptor:
    """P57-compatible frozen semantics with P58-local input identities."""

    descriptor: object
    semantics: object | None
    before: _LexicalInputIdentity | None
    after: _LexicalInputIdentity | None
    dispatch_probe: Path | None
    windows_arguments: tuple[str, ...] | None
    ubuntu_arguments: tuple[str, ...] | None


def _catalog() -> dict[str, object]:
    return {"schema": P43_CATALOG_SCHEMA, "gate_ids": list(P58_GATE_IDS)}


def _event(gate_id: str, kind: str, outcome: str) -> dict[str, object]:
    return {
        "classification": "ordered-execution",
        "event_kind": kind,
        "gate_id": gate_id,
        "outcome": outcome,
        "schema": P43_EVENT_SCHEMA,
    }


def _validation(validation_id: str, checks: int) -> dict[str, object]:
    return {
        "classification": "public-artifact-self-validation",
        "completed_checks": checks,
        "event_kind": "validation-complete",
        "expected_checks": checks,
        "schema": P43_EVENT_SCHEMA,
        "validation_id": validation_id,
    }


def _private_record() -> dict[str, object]:
    return {
        "schema": "ferris.pulse-58-private-execution-record/v1",
        "outcome": "in-progress",
        "product_conclusion": None,
        "category_conclusion": None,
        "fix_conclusion": None,
        "publication": "not-attempted",
        "p39_checkout_verifications": 0,
        "p39_caller_authority_precondition": P39_CALLER_AUTHORITY_PRECONDITION,
        "p39_execution_scope": "exact-p39-semantics-only",
        "p41_transactional_copy_invocations": 0,
        "p41_post_copy_binding": "not-attempted",
        "public_preflight_namespace_absence_checks": 0,
        "p27_invocations": 0,
        "p27_cycle_retention": "not-attempted",
        "seed_calls": 0,
        "seed_byte_count": 0,
        "seed_cleanup": "not-created",
        "materializer_invocations": 0,
        "verifier_invocations": 0,
        "descriptor_cleanup": "not-created",
        "private_root_cleanup": "not-attempted",
        "platform_records": {platform: [] for platform in CANONICAL_PLATFORMS},
        "process_counts": {platform: 0 for platform in CANONICAL_PLATFORMS},
        "no_launch_records": [],
    }


def _failure_code(error: BaseException, fallback: str = "P58-OPERATION") -> str:
    value = getattr(error, "code", None)
    return value if type(value) is str else fallback


def _terminal(
    p43: ModuleType | None,
    events: list[dict[str, object]],
    gate: str,
    code: str | None,
    record: dict[str, object],
) -> OrderedCapabilityMaterializationResult:
    events.append(_event(gate, "terminal-stop", "completed" if code is None else "failed"))
    if p43 is not None:
        p43.validate_catalog(_catalog())
        p43.validate_events(tuple(P58_GATE_IDS), events)
    record["outcome"] = "completed" if code is None else "failed"
    if code is not None:
        record["failure_code"] = code
    return OrderedCapabilityMaterializationResult(_catalog(), events, record)


def _runtime_path(p51: ModuleType, runtime_root: Path, value: Path, code: str, *, directory: bool = False, absent: bool = False) -> Path:
    return p51._runtime_path(
        runtime_root,
        value,
        code,
        require_directory=directory,
        allow_absent_leaf=absent,
    )


def _assert_runtime_fresh(p51: ModuleType, value: Path) -> Path:
    runtime_root = p51._safe_runtime_root(value)
    try:
        if next(os.scandir(runtime_root), None) is not None:
            raise P58Failure("P58-RUNTIME-NOT-FRESH")
    except P58Failure:
        raise
    except OSError as error:
        raise P58Failure("P58-RUNTIME-NOT-FRESH") from error
    return runtime_root


def _private_paths(p51: ModuleType, runtime_root: Path, p27_cycle_root: Path) -> tuple[Path, Path]:
    namespace = _runtime_path(p51, runtime_root, runtime_root / PRIVATE_NAMESPACE, "P58-PRIVATE-ROOT", absent=True)
    cycle = _runtime_path(p51, runtime_root, p27_cycle_root, "P58-P27-CYCLE-ROOT", absent=True)
    if namespace.parent != runtime_root or cycle.parent != runtime_root or os.path.lexists(namespace) or os.path.lexists(cycle):
        raise P58Failure("P58-PRIVATE-ROOT-NOT-FRESH")
    return namespace, cycle


def _assert_before_seed(p51: ModuleType, runtime_root: Path, namespace: Path, cycle: Path, record: dict[str, object]) -> None:
    _runtime_path(p51, runtime_root, namespace, "P58-PRIVATE-ROOT", absent=True)
    _runtime_path(p51, runtime_root, cycle, "P58-P27-CYCLE-ROOT", absent=True)
    if os.path.lexists(namespace):
        raise P58Failure("P58-PRIVATE-ROOT-NOT-FRESH")
    record["public_preflight_namespace_absence_checks"] = int(record["public_preflight_namespace_absence_checks"]) + 1


def _begin_private_namespace(p51: ModuleType, runtime_root: Path, namespace: Path) -> Path:
    _runtime_path(p51, runtime_root, namespace, "P58-PRIVATE-ROOT", absent=True)
    try:
        os.mkdir(namespace, 0o700)
    except OSError as error:
        raise P58Failure("P58-PRIVATE-ROOT") from error
    return _runtime_path(p51, runtime_root, namespace, "P58-PRIVATE-ROOT", directory=True)


def _remove_runtime(p52: ModuleType, p51: ModuleType, runtime_root: Path, namespace: Path, record: dict[str, object]) -> None:
    try:
        if os.path.lexists(namespace):
            p52._remove_private_tree(p51, namespace, "P58-PRIVATE-CLEANUP")
        if os.path.lexists(runtime_root):
            p52._remove_private_tree(p51, runtime_root, "P58-PRIVATE-CLEANUP")
        if os.path.lexists(namespace) or os.path.lexists(runtime_root):
            raise P58Failure("P58-PRIVATE-CLEANUP")
    except BaseException as error:
        if isinstance(error, P58Failure):
            raise
        raise P58Failure("P58-PRIVATE-CLEANUP") from error
    record["descriptor_cleanup"] = "removed-and-verified"
    record["private_root_cleanup"] = "removed-and-verified"
    if record["seed_cleanup"] == "not-created":
        record["seed_cleanup"] = "removed-on-terminal"


def _close_and_cleanup(
    p75: ModuleType,
    p52: ModuleType,
    controls: object | None,
    windows_handle: object | None,
    execution: _ExecutionState,
    wsl: object | None,
    p51: ModuleType | None,
    runtime_root: Path | None,
    namespace: Path | None,
    record: dict[str, object],
) -> None:
    try:
        if controls is not None:
            p75._close_handles(controls, windows_handle, execution.windows_launches, wsl)
        if p51 is not None and runtime_root is not None and namespace is not None:
            _remove_runtime(p52, p51, runtime_root, namespace, record)
    except BaseException as error:
        raise IndeterminateCleanup() from error


def _known(p75: ModuleType, error: BaseException, controls: object | None) -> bool:
    return isinstance(error, (P58Failure, SealedDependencyFailure)) or p75._known_failure(error, controls)


def _materialize_once(
    p52: ModuleType,
    p51: ModuleType,
    materializer: ModuleType,
    verifier: ModuleType,
    namespace: Path,
    seed: bytes,
    record: dict[str, object],
) -> tuple[Path, tuple[object, ...]]:
    if type(seed) is not bytes or len(seed) != 32:
        raise P58Failure("P58-SEED-GENERATION")
    try:
        seed_path = p52._write_seed_atomically(p51, namespace, seed)
    except BaseException as error:
        raise P58Failure("P58-SEED-CREATION") from error
    record["seed_byte_count"] = 32
    descriptor_root = namespace / "descriptors"
    record["materializer_invocations"] = 1
    try:
        materialization = materializer.materialize(seed_path, descriptor_root)
    except materializer.MaterializationError as error:
        raise P58Failure("P58-MATERIALIZATION") from error
    p52._validate_materialization_summary(p51, materialization)
    record["verifier_invocations"] = 1
    try:
        verification = verifier.verify(descriptor_root, seed_path)
    except verifier.MaterializationError as error:
        raise P58Failure("P58-PRIVATE-VERIFIER") from error
    p52._validate_verification_summary(p51, verification)
    try:
        commitment = materializer.seed_commitment(seed)
    except BaseException as error:
        raise P58Failure("P58-SEED-COMMITMENT") from error
    if type(commitment) is not str or not commitment.startswith("sha256:"):
        raise P58Failure("P58-SEED-COMMITMENT")
    record["seed_commitment_sha256"] = commitment
    p52._remove_seed(p51, seed_path, record)
    descriptors, projection_variances = p52._validate_materialized_descriptor_root(p51, descriptor_root, namespace.parent)
    record["p35_to_p51_semantic_projection_variance_count"] = projection_variances
    return descriptor_root, descriptors


def _is_link_or_reparse(metadata: os.stat_result) -> bool:
    return stat.S_ISLNK(metadata.st_mode) or bool(
        getattr(metadata, "st_file_attributes", 0)
        & getattr(stat, "FILE_ATTRIBUTE_REPARSE_POINT", 0)
    )


def _lexical_runtime_path(runtime_root: Path, value: Path, code: str) -> Path:
    """Check lexical components without resolving a link before rejecting it."""

    try:
        if not value.is_absolute():
            raise P58Failure(code)
        relative = value.relative_to(runtime_root)
        if not relative.parts:
            raise P58Failure(code)
        current = runtime_root
        metadata = os.lstat(current)
        if _is_link_or_reparse(metadata) or not stat.S_ISDIR(metadata.st_mode):
            raise P58Failure(code)
        for index, part in enumerate(relative.parts):
            current = current / part
            try:
                metadata = os.lstat(current)
            except FileNotFoundError:
                if index == len(relative.parts) - 1:
                    return value
                raise P58Failure(code)
            if _is_link_or_reparse(metadata):
                raise P58Failure(code)
            if index != len(relative.parts) - 1 and not stat.S_ISDIR(metadata.st_mode):
                raise P58Failure(code)
        return value
    except P58Failure:
        raise
    except (OSError, ValueError) as error:
        raise P58Failure(code) from error


def _directory_identity(path: Path, code: str = "P58-P51-INPUT-IDENTITY") -> _LexicalInputIdentity:
    """Capture a directory by lexical file ID, never by a resolved target."""

    try:
        initial = os.lstat(path)
        if _is_link_or_reparse(initial) or not stat.S_ISDIR(initial.st_mode):
            raise P58Failure(code)
        flags = os.O_RDONLY | getattr(os, "O_BINARY", 0)
        directory_flag = getattr(os, "O_DIRECTORY", 0)
        nofollow_flag = getattr(os, "O_NOFOLLOW", 0)
        if directory_flag:
            descriptor = os.open(path, flags | directory_flag | nofollow_flag)
        else:
            descriptor = None
    except P58Failure:
        raise
    except OSError as error:
        raise P58Failure(code) from error
    try:
        if descriptor is not None:
            opened = os.fstat(descriptor)
            if (
                not stat.S_ISDIR(opened.st_mode)
                or (opened.st_dev, opened.st_ino) != (initial.st_dev, initial.st_ino)
            ):
                raise P58Failure(code)
        repeated = os.lstat(path)
        if (
            _is_link_or_reparse(repeated)
            or not stat.S_ISDIR(repeated.st_mode)
            or (repeated.st_dev, repeated.st_ino) != (initial.st_dev, initial.st_ino)
        ):
            raise P58Failure(code)
        return _LexicalInputIdentity(
            path, initial.st_dev, initial.st_ino, None, None, "directory"
        )
    except P58Failure:
        raise
    except OSError as error:
        raise P58Failure(code) from error
    finally:
        if descriptor is not None:
            os.close(descriptor)


def _lexical_input_identity(
    runtime_root: Path, value: Path, code: str = "P58-P51-INPUT-IDENTITY"
) -> _LexicalInputIdentity:
    """Bind regular, missing, and directory P35 input states without following links."""

    path = _lexical_runtime_path(runtime_root, value, code)
    try:
        initial = os.lstat(path)
    except FileNotFoundError:
        return _LexicalInputIdentity(path, None, None, None, None, "missing")
    except OSError as error:
        raise P58Failure(code) from error
    if _is_link_or_reparse(initial):
        raise P58Failure(code)
    if stat.S_ISDIR(initial.st_mode):
        return _directory_identity(path, code)
    if not stat.S_ISREG(initial.st_mode):
        raise P58Failure(code)
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        raise P58Failure(code) from error
    try:
        opened = os.fstat(descriptor)
        if (
            not stat.S_ISREG(opened.st_mode)
            or (opened.st_dev, opened.st_ino) != (initial.st_dev, initial.st_ino)
        ):
            raise P58Failure(code)
        digest = hashlib.sha256()
        size = 0
        while chunk := os.read(descriptor, 65_536):
            digest.update(chunk)
            size += len(chunk)
        repeated = os.lstat(path)
        if (
            _is_link_or_reparse(repeated)
            or not stat.S_ISREG(repeated.st_mode)
            or (repeated.st_dev, repeated.st_ino) != (opened.st_dev, opened.st_ino)
        ):
            raise P58Failure(code)
        return _LexicalInputIdentity(
            path,
            opened.st_dev,
            opened.st_ino,
            size,
            "sha256:" + digest.hexdigest(),
            "regular-file",
        )
    except P58Failure:
        raise
    except OSError as error:
        raise P58Failure(code) from error
    finally:
        os.close(descriptor)


def _same_lexical_identity(runtime_root: Path, expected: _LexicalInputIdentity) -> None:
    if _lexical_input_identity(runtime_root, expected.path) != expected:
        raise P58Failure("P57-P51-INPUT-SUBSTITUTION")


def _freeze_p58_descriptors(
    p75: ModuleType,
    p51: ModuleType,
    descriptor_root: Path,
    descriptors: tuple[object, ...],
    runtime_root: Path,
) -> tuple[_P58FrozenDescriptor, ...]:
    """Freeze P57 semantics while retaining P58's directory-capable identity."""

    dispatch_probe = _lexical_runtime_path(
        runtime_root, descriptor_root / "case-manifest.json", "P57-P51-DISPATCH"
    )
    probe_identity = _lexical_input_identity(runtime_root, dispatch_probe, "P57-P51-DISPATCH")
    if probe_identity.state != "regular-file":
        raise P58Failure("P57-P51-DISPATCH")
    frozen: list[_P58FrozenDescriptor] = []
    for original in descriptors:
        expected = p75._copy_expected(original.expected)
        if original.execution_mode == "no-launch":
            frozen.append(
                _P58FrozenDescriptor(
                    p75._frozen_descriptor(original, expected, None, None),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            )
            continue
        if original.before is None or original.after is None:
            raise P58Failure("P57-P51-SEMANTICS")
        before = _lexical_input_identity(runtime_root, original.before)
        after = _lexical_input_identity(runtime_root, original.after)
        descriptor = p75._frozen_descriptor(original, expected, before.path, after.path)
        semantics = p51._descriptor_semantics(descriptor)
        _same_lexical_identity(runtime_root, before)
        _same_lexical_identity(runtime_root, after)
        windows = p51.build_platform_dispatch(
            "windows-x86_64", dispatch_probe, descriptor, runtime_root
        )
        ubuntu = p51.build_platform_dispatch(
            "ubuntu-24.04-x86_64", dispatch_probe, descriptor, runtime_root
        )
        windows_arguments = tuple(windows.application_argv)
        ubuntu_arguments = tuple(ubuntu.command[-7:])
        if len(windows_arguments) != 7 or len(ubuntu_arguments) != 7:
            raise P58Failure("P57-P51-DISPATCH")
        _same_lexical_identity(runtime_root, before)
        _same_lexical_identity(runtime_root, after)
        frozen.append(
            _P58FrozenDescriptor(
                descriptor,
                semantics,
                before,
                after,
                dispatch_probe,
                windows_arguments,
                ubuntu_arguments,
            )
        )
    return tuple(frozen)


def _p58_prelaunch_dispatch(
    p51: ModuleType, frozen: _P58FrozenDescriptor, runtime_root: Path
) -> tuple[tuple[str, ...], tuple[str, ...]]:
    """Repeat local no-follow identities around P57's exact dispatch construction."""

    if (
        frozen.before is None
        or frozen.after is None
        or frozen.dispatch_probe is None
        or frozen.windows_arguments is None
        or frozen.ubuntu_arguments is None
    ):
        raise P58Failure("P57-P51-DISPATCH")
    _same_lexical_identity(runtime_root, frozen.before)
    _same_lexical_identity(runtime_root, frozen.after)
    windows = p51.build_platform_dispatch(
        "windows-x86_64", frozen.dispatch_probe, frozen.descriptor, runtime_root
    )
    ubuntu = p51.build_platform_dispatch(
        "ubuntu-24.04-x86_64", frozen.dispatch_probe, frozen.descriptor, runtime_root
    )
    if (
        tuple(windows.application_argv) != frozen.windows_arguments
        or tuple(ubuntu.command[-7:]) != frozen.ubuntu_arguments
    ):
        raise P58Failure("P57-P51-INPUT-SUBSTITUTION")
    _same_lexical_identity(runtime_root, frozen.before)
    _same_lexical_identity(runtime_root, frozen.after)
    return frozen.windows_arguments, frozen.ubuntu_arguments


def _execute_p57_semantics(
    p75: ModuleType,
    p51: ModuleType,
    p56: object,
    frozen: tuple[_P58FrozenDescriptor, ...],
    runtime_root: Path,
    windows_handle: object,
    wsl: object,
    record: dict[str, object],
    execution: _ExecutionState,
) -> None:
    for descriptor in frozen:
        if descriptor.descriptor.execution_mode == "no-launch":
            for platform in CANONICAL_PLATFORMS:
                record["no_launch_records"].append({
                    "ordinal": descriptor.descriptor.ordinal,
                    "platform": platform,
                    "process_launched": False,
                    "reason": "blocked-no-launch-external-immutable-binary-freeze",
                })
            continue
        windows_arguments, ubuntu_arguments = _p58_prelaunch_dispatch(
            p51, descriptor, runtime_root
        )
        windows_capture = p56.launch_verified(windows_handle, "windows-x86_64", windows_arguments)
        execution.windows_launches += 1
        _same_lexical_identity(runtime_root, descriptor.before)
        _same_lexical_identity(runtime_root, descriptor.after)
        windows = {
            "ordinal": descriptor.descriptor.ordinal,
            "platform": "windows-x86_64",
            "process_launched": True,
            "result": p75._normalize_result(p51, descriptor, windows_capture),
        }
        record["platform_records"]["windows-x86_64"].append(windows)
        record["process_counts"]["windows-x86_64"] += 1
        _p58_prelaunch_dispatch(p51, descriptor, runtime_root)
        ubuntu_capture = wsl.launch(descriptor.descriptor.ordinal, ubuntu_arguments)
        _same_lexical_identity(runtime_root, descriptor.before)
        _same_lexical_identity(runtime_root, descriptor.after)
        ubuntu = {
            "ordinal": descriptor.descriptor.ordinal,
            "platform": "ubuntu-24.04-x86_64",
            "process_launched": True,
            "result": p75._normalize_result(p51, descriptor, ubuntu_capture),
        }
        record["platform_records"]["ubuntu-24.04-x86_64"].append(ubuntu)
        record["process_counts"]["ubuntu-24.04-x86_64"] += 1
        if windows["result"]["semantic_projection"] != ubuntu["result"]["semantic_projection"]:
            record["first_mismatch_ordinal"] = descriptor.descriptor.ordinal
            raise p75.ExecutorFailure("P57-FIRST-TARGET-MISMATCH")
    if record["process_counts"] != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69} or len(record["no_launch_records"]) != 2 or execution.windows_launches != p75.REQUEST_COUNT:
        raise P58Failure("P58-TOPOLOGY-ACCOUNTING")


def _run(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    ubuntu_runtime_parent: str,
    qualification: _QualificationControls | None,
) -> OrderedCapabilityMaterializationResult:
    events: list[dict[str, object]] = []
    record = _private_record()
    p43: ModuleType | None = None
    p75: ModuleType | None = None
    p52: ModuleType | None = None
    p51: ModuleType | None = None
    controls: object | None = None
    runtime_root: Path | None = None
    namespace: Path | None = None
    windows_handle: object | None = None
    execution = _ExecutionState()
    wsl: object | None = None
    current_gate = P58_GATE_IDS[0]
    try:
        p39, p41 = load_exact_p39_and_p41(repo_root)
        p52 = load_exact_p52_stage_reader(repo_root)
        materializer, verifier = load_exact_p35_materializer_and_verifier(repo_root)
        p75, p51, p56_real = load_exact_p75_stack(repo_root)
        p43, p45, p47 = p51.load_terminal_dependencies(repo_root)
        p56 = p56_real if qualification is None else qualification.p56
        open_wsl = (lambda root, parent, api: p75._NativeWslSession(root, parent, api)) if qualification is None else qualification.open_wsl
        p27_runner = None if qualification is None else qualification.p27_runner
        controls = p75._Controls(p51, p56, p27_runner, open_wsl)
        controls = p75._with_terminal_failure_types(controls, p43, p45, p47)
        runtime_root = _assert_runtime_fresh(p51, private_runtime_root)
        namespace, cycle = _private_paths(p51, runtime_root, p27_cycle_root)
        p43.validate_catalog(_catalog())
        record["p39_checkout_verifications"] = 0
        record["p41_transactional_copy_invocations"] = 0
        p52._verify_public_prelaunch_custody(p51, p39, p41, runtime_root, p39_checkout_root, p41_final_root, record)
        events.append(
            _validation(
                "p39-exact-semantics-authority-precondition", 10
            )
        )
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[1]
        events.append(_validation("sealed-predecessor-identities", 6))
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[2]
        _assert_before_seed(p51, runtime_root, namespace, cycle, record)
        windows_handle = p56.publish_retained_build_and_custody("windows-x86_64", runtime_root)
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[3]
        _assert_before_seed(p51, runtime_root, namespace, cycle, record)
        wsl = open_wsl(repo_root, ubuntu_runtime_parent, p51)
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[4]
        _assert_before_seed(p51, runtime_root, namespace, cycle, record)
        runner = p27_runner or p51.load_p27_exact_runner(repo_root)
        p51._run_p27_once(runtime_root, cycle, runner)
        record["p27_invocations"] = 1
        record["p27_cycle_retention"] = "private-cycle-root"
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[5]
        _assert_before_seed(p51, runtime_root, namespace, cycle, record)
        if p51.verify_bound_contract(repo_root) != {"artifact_count": 9, "positive_fixture_count": 6, "mutation_control_count": 33, "public_input_checks": 39}:
            raise P58Failure("P58-P31-CONTROL-COUNT")
        events.append(_validation("public-input-contract", 39))
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[6]
        _assert_before_seed(p51, runtime_root, namespace, cycle, record)
        if p51.verify_p35_p37_custody(repo_root) != {"bound_file_count": 11, "p35_release_tree_file_count": 10, "machine_schema_count": 1, "canonical_lf_file_count": 11, "git_clean_checks": 11}:
            raise P58Failure("P58-P35-CUSTODY-COUNT")
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[7]
        namespace = _begin_private_namespace(p51, runtime_root, namespace)
        seed = secrets.token_bytes(32) if qualification is None else qualification.seed_bytes
        record["seed_calls"] = 1
        descriptor_root, descriptors = _materialize_once(p52, p51, materializer, verifier, namespace, seed, record)
        del seed
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[8]
        frozen = _freeze_p58_descriptors(p75, p51, descriptor_root, descriptors, runtime_root)
        if len(frozen) != 70:
            raise P58Failure("P58-DESCRIPTOR-TOPOLOGY")
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P58_GATE_IDS[9]
        _execute_p57_semantics(
            p75, p51, p56, frozen, runtime_root, windows_handle, wsl, record, execution
        )
    except BaseException as error:
        if p75 is None and isinstance(error, SealedDependencyFailure):
            return _terminal(None, events, current_gate, error.code, record)
        if p75 is not None and _known(p75, error, controls):
            code = _failure_code(error)
            try:
                if p52 is not None:
                    _close_and_cleanup(p75, p52, controls, windows_handle, execution, wsl, p51, runtime_root, namespace, record)
            except IndeterminateCleanup:
                raise
            return _terminal(p43, events, current_gate, code, record)
        if p75 is not None and p52 is not None:
            _close_and_cleanup(p75, p52, controls, windows_handle, execution, wsl, p51, runtime_root, namespace, record)
        raise

    assert p75 is not None and p52 is not None
    _close_and_cleanup(p75, p52, controls, windows_handle, execution, wsl, p51, runtime_root, namespace, record)
    return _terminal(p43, events, current_gate, None, record)


def run_ordered_capability_materialization_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    ubuntu_runtime_parent: str,
) -> OrderedCapabilityMaterializationResult:
    """Run Pulse 76's fixed public-to-private capability sequence.

    Callers cannot provide descriptors, a seed, capability, custody receipt or
    root, process runner, callback, environment, or another control seam.
    """
    return _run(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent, None)


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
    open_wsl: Callable[[Path, str, ModuleType], object],
) -> OrderedCapabilityMaterializationResult:
    """Source-bound fake-only qualification seam, absent from production exports."""
    return _run(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, "/home/pulse58-qualification", _QualificationControls(seed_bytes, p27_runner, p56, open_wsl))


__all__ = [
    "IndeterminateCleanup",
    "OrderedCapabilityMaterializationResult",
    "P58Failure",
    "release_identities",
    "run_ordered_capability_materialization_executor",
]
