"""Byte-bound exact imports for Pulse 75's stage-bootstrap/worker-identity stack."""
from __future__ import annotations

import contextlib
import contextvars
import ctypes
import errno
import hashlib
import inspect
import json
import os
import socket
import stat
import sys
import threading
import time
from dataclasses import dataclass
from pathlib import Path
from types import MappingProxyType, ModuleType
from typing import Mapping


P72_COMMIT = "642285b1b7f45102718f31d1d3ce92336f78bf5c"
P72_RELEASE_ROOT = (
    "docs/simulations/profile-diff-held-out/"
    "pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-release"
)
_SEALED_LOADING_SLOT = "sealed_dependencies"
_MISSING = object()
_SEALED_LOCK_TIMEOUT_SECONDS = 300.0
_SEALED_LOCK_POLL_SECONDS = 0.05
_KERNEL_LOCK_NAMESPACE_PREFIX = "ferris-p75"
_WINDOWS_WAIT_OBJECT_0 = 0x00000000
_WINDOWS_WAIT_ABANDONED = 0x00000080
_P75_INTERNAL_LOCK_MANAGER: object | None = None
_WINDOWS_KERNEL32: object | None = None


@dataclass(frozen=True)
class ReleaseIdentity:
    directory: str
    source: str
    source_sha256: str
    manifest_schema: str
    manifest_raw_sha256: str
    manifest_aggregate: str
    manifest_file_count: int
    release_tree_file_count: int
    receipt_raw_sha256: str
    receipt_payload_sha256: str
    seal_raw_sha256: str
    seal_payload_sha256: str


@dataclass(frozen=True)
class _VerifiedRelease:
    root: Path
    files: Mapping[str, bytes]


@dataclass
class _KernelLockState:
    kind: str
    name: str
    handle: object | None


@dataclass
class _KernelLockToken:
    live: bool = True


@dataclass
class _ActiveSealedLoadingLock:
    lock_state: _KernelLockState
    owner_pid: int
    owner_thread_id: int
    owner_token: _KernelLockToken
    depth: int = 1


_ACTIVE_SEALED_LOADING_LOCK: contextvars.ContextVar[_ActiveSealedLoadingLock | None] = (
    contextvars.ContextVar("p75_active_sealed_loading_lock", default=None)
)


class SealedDependencyFailure(RuntimeError):
    """A sealed Pulse 72 predecessor or callable binding failed."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


P72 = ReleaseIdentity(
    "pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-release",
    "capability_bound_diagnostic_executor_successor.py",
    "sha256:408cc0013861c398e76e125d7e62ec6f24cfd0aec8e9083f2d699e3ad04b2901",
    "ferris.pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-manifest/v1",
    "sha256:ba65fa3e8b9363e3345b736d12152f9b9dd46bd5ac1ef8b718407868d9a749b9",
    "sha256:315f39c3eb03e9fe1d2171914d1f2d09d94c509e8cbb2b09e5911cffc2e83dfd",
    12,
    14,
    "sha256:0e316975e76d47d2abc807c6c00e361fe2f22c9e57bdf0bb686364e48840205e",
    "sha256:8c31c7d0e19d9298e6b072a137f6d10522b752d91ed8c8b2c5df8259c5a1fb26",
    "sha256:241fa954c14d002474120ce539894f881eeae9a4e2c38226f7a732807b76b8a7",
    "sha256:f80fdcc6b3d7b23cd2e81004c7e5243c75512234adc049805cb025c4f93ec7a3",
)
P72_STACK = {
    "pulse_69": {
        "manifest_aggregate": "sha256:070d56d191be0b743b34d4444350f6b0e78272088c9141a39ec2b60524216331",
        "manifest_raw_sha256": "sha256:dd52c64100cdcd91ae1ea92f91af112f56296faf803511832ce3db489e071463",
        "receipt_payload_sha256": "sha256:244f02e9fb5a164a747d3407696f4ef4a7fa318fa7ee5fe33d8ce1baa533def9",
        "receipt_raw_sha256": "sha256:5fe60dcabc27a3260439c302407079feb3f004e2463877c8ff133a055884cbd4",
        "release_root": "docs/simulations/profile-diff-held-out/pulse-69-capability-bound-diagnostic-executor-successor-release",
        "seal_payload_sha256": "sha256:2ac37cac02569d89cc068778cd956c8b5273c086685a1296dc568d13d191885f",
        "seal_raw_sha256": "sha256:c25b7b9868825734c7d649c1a2f4d303cf0de8da2ae7acebde386031fd689363",
        "source_sha256": "sha256:f07f10ccbafde98ba16292f6d35ec6611623b14aa736220ad2275ced3ecb316d",
        "stack": {
            "pulse_51": {
                "manifest": "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc",
                "seal": "sha256:1d22ad1248a2f47c78984d8020c3c6507253c468b53f30073efcfb5ab880c0d4",
            },
            "pulse_56": {
                "manifest": "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a",
                "seal": "sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747",
            },
            "pulse_57": {
                "manifest": "sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291",
                "seal": "sha256:727e1806ede5ca9b5438b5f3b00bec3a59b4e36404ccc1b72e6adb661ebee144",
                "source": "sha256:bcb5eac2cd5aa0abd271dec2e93963ec855faa1c5ecbd628dfef61f52358c2c0",
            },
        },
    },
}


def _bind_internal_lock_manager(manager: object) -> None:
    if not callable(getattr(manager, "register_active_lock_state", None)) or not callable(
        getattr(manager, "unregister_active_lock_state", None)
    ):
        raise SealedDependencyFailure("P75-SEALED-LOCK-STATE")
    global _P75_INTERNAL_LOCK_MANAGER
    _P75_INTERNAL_LOCK_MANAGER = manager



def _require_internal_lock_manager() -> object:
    if _P75_INTERNAL_LOCK_MANAGER is None:
        raise SealedDependencyFailure("P75-SEALED-LOCK-STATE")
    return _P75_INTERNAL_LOCK_MANAGER



def canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value,
        allow_nan=False,
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("ascii")



def sha256_bytes(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()



def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise SealedDependencyFailure("P75-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result



def _safe_regular(
    path: Path, code: str = "P75-SEALED-TREE", maximum: int = 4_194_304
) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise SealedDependencyFailure(code)
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise SealedDependencyFailure(code)
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > maximum:
                raise SealedDependencyFailure(code)
        return bytes(content)
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure(code) from error
    finally:
        os.close(descriptor)



def _sealed_json(content: bytes, code: str) -> dict[str, object]:
    try:
        value = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, SealedDependencyFailure) as error:
        raise SealedDependencyFailure(code) from error
    if type(value) is not dict:
        raise SealedDependencyFailure(code)
    return value



def _release_root(repo_root: Path, identity: ReleaseIdentity, code: str) -> Path:
    try:
        root = repo_root.resolve(strict=True)
        if not root.is_absolute():
            raise SealedDependencyFailure(code)
        release = root / "docs" / "simulations" / "profile-diff-held-out" / identity.directory
        release.relative_to(root)
        metadata = os.lstat(release)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise SealedDependencyFailure(code)
        return release
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure(code) from error



def _tree_paths(root: Path, current: Path, result: set[str], code: str) -> None:
    try:
        entries = sorted(os.scandir(current), key=lambda entry: entry.name)
    except OSError as error:
        raise SealedDependencyFailure(code) from error
    for entry in entries:
        path = Path(entry.path)
        try:
            metadata = os.lstat(path)
        except OSError as error:
            raise SealedDependencyFailure(code) from error
        if stat.S_ISLNK(metadata.st_mode):
            raise SealedDependencyFailure(code)
        if stat.S_ISDIR(metadata.st_mode):
            _tree_paths(root, path, result, code)
        elif stat.S_ISREG(metadata.st_mode):
            result.add(path.relative_to(root).as_posix())
        else:
            raise SealedDependencyFailure(code)



def _aggregate(entries: list[object], code: str) -> str:
    normalized: list[tuple[str, str]] = []
    for entry in entries:
        if type(entry) is not dict:
            raise SealedDependencyFailure(code)
        path = entry.get("path")
        digest = entry.get("sha256")
        if (
            type(path) is not str
            or not path
            or Path(path).is_absolute()
            or ".." in Path(path).parts
            or type(digest) is not str
        ):
            raise SealedDependencyFailure(code)
        normalized.append((path, digest))
    if len({path for path, _ in normalized}) != len(normalized):
        raise SealedDependencyFailure(code)
    digest = hashlib.sha256()
    for path, value in sorted(normalized, key=lambda item: item[0].encode("utf-8")):
        try:
            raw = bytes.fromhex(value.removeprefix("sha256:"))
        except ValueError as error:
            raise SealedDependencyFailure(code) from error
        if len(raw) != 32:
            raise SealedDependencyFailure(code)
        encoded = path.encode("utf-8")
        digest.update(len(encoded).to_bytes(8, "big"))
        digest.update(encoded)
        digest.update(raw)
    return "sha256:" + digest.hexdigest()



def _verified_release(
    repo_root: Path, identity: ReleaseIdentity, code: str
) -> _VerifiedRelease:
    release = _release_root(repo_root, identity, code)
    manifest_raw = _safe_regular(release / "public-manifest.json", code)
    receipt_raw = _safe_regular(release / "qualification-receipt.json", code)
    seal_raw = _safe_regular(release / "release-seal.json", code)
    if (
        sha256_bytes(manifest_raw) != identity.manifest_raw_sha256
        or sha256_bytes(receipt_raw) != identity.receipt_raw_sha256
        or sha256_bytes(seal_raw) != identity.seal_raw_sha256
    ):
        raise SealedDependencyFailure(code)
    manifest = _sealed_json(manifest_raw, code)
    receipt = _sealed_json(receipt_raw, code)
    seal = _sealed_json(seal_raw, code)
    files = manifest.get("files")
    if (
        manifest.get("schema") != identity.manifest_schema
        or manifest.get("aggregate") != identity.manifest_aggregate
        or manifest.get("file_count") != identity.manifest_file_count
        or manifest.get("release_tree_file_count") != identity.release_tree_file_count
        or type(files) is not list
        or len(files) != identity.manifest_file_count
        or _aggregate(files, code) != identity.manifest_aggregate
        or receipt.get("payload_sha256") != identity.receipt_payload_sha256
        or receipt.get("receipt_id") != identity.receipt_payload_sha256
        or seal.get("payload_sha256") != identity.seal_payload_sha256
        or seal.get("seal_id") != identity.seal_payload_sha256
    ):
        raise SealedDependencyFailure(code)
    if (
        type(receipt.get("payload")) is not dict
        or sha256_bytes(canonical_bytes(receipt["payload"]))
        != identity.receipt_payload_sha256
        or type(seal.get("payload")) is not dict
        or sha256_bytes(canonical_bytes(seal["payload"])) != identity.seal_payload_sha256
    ):
        raise SealedDependencyFailure(code)
    bound: dict[str, bytes] = {
        "public-manifest.json": manifest_raw,
        "release-seal.json": seal_raw,
    }
    expected = set(bound)
    total = 0
    source_digest: str | None = None
    for entry in files:
        assert type(entry) is dict
        path = entry.get("path")
        size = entry.get("size")
        digest = entry.get("sha256")
        if (
            type(path) is not str
            or type(size) is not int
            or size < 0
            or type(digest) is not str
            or path in expected
        ):
            raise SealedDependencyFailure(code)
        content = (
            receipt_raw
            if path == "qualification-receipt.json"
            else _safe_regular(release.joinpath(*path.split("/")), code)
        )
        if len(content) != size or sha256_bytes(content) != digest:
            raise SealedDependencyFailure(code)
        if path == identity.source:
            source_digest = digest
        bound[path] = content
        expected.add(path)
        total += size
    if "qualification-receipt.json" not in expected:
        expected.add("qualification-receipt.json")
        bound["qualification-receipt.json"] = receipt_raw
    actual: set[str] = set()
    _tree_paths(release, release, actual, code)
    if actual != expected or len(actual) != identity.release_tree_file_count:
        raise SealedDependencyFailure(code)
    if (
        manifest.get("total_bytes") != total
        or source_digest != identity.source_sha256
    ):
        raise SealedDependencyFailure(code)
    return _VerifiedRelease(release, MappingProxyType(bound))



def _self_path() -> Path:
    try:
        return Path(__file__).resolve(strict=True)
    except OSError as error:
        raise SealedDependencyFailure("P75-SEALED-LOCK-OPEN") from error



def _current_pid() -> int:
    return os.getpid()



def _current_thread_id() -> int:
    return threading.get_ident()



def _kernel_lock_name() -> str:
    path = _self_path()
    digest = hashlib.sha256()
    digest.update(os.fsencode(os.fspath(path)))
    digest.update(b"\0")
    digest.update(sha256_bytes(_safe_regular(path, "P75-SEALED-LOCK-OPEN")).encode("ascii"))
    value = digest.hexdigest()
    if os.name == "nt":
        return f"Local\\{_KERNEL_LOCK_NAMESPACE_PREFIX}-{value}"
    return f"\0{_KERNEL_LOCK_NAMESPACE_PREFIX}-{value}"



def _windows_kernel32() -> object:
    global _WINDOWS_KERNEL32
    if _WINDOWS_KERNEL32 is None:
        try:
            library = ctypes.WinDLL("kernel32", use_last_error=True)
        except OSError as error:
            raise SealedDependencyFailure("P75-SEALED-LOCK-OPEN") from error
        library.CreateMutexW.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_wchar_p]
        library.CreateMutexW.restype = ctypes.c_void_p
        library.WaitForSingleObject.argtypes = [ctypes.c_void_p, ctypes.c_ulong]
        library.WaitForSingleObject.restype = ctypes.c_ulong
        library.ReleaseMutex.argtypes = [ctypes.c_void_p]
        library.ReleaseMutex.restype = ctypes.c_int
        library.CloseHandle.argtypes = [ctypes.c_void_p]
        library.CloseHandle.restype = ctypes.c_int
        _WINDOWS_KERNEL32 = library
    return _WINDOWS_KERNEL32



def _linux_socket_lock_supported() -> bool:
    return os.name == "posix" and sys.platform.startswith("linux")



def _open_linux_kernel_socket() -> socket.socket:
    if not _linux_socket_lock_supported() or not hasattr(socket, "AF_UNIX"):
        raise SealedDependencyFailure("P75-SEALED-LOCK-OPEN")
    try:
        return socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    except OSError as error:
        raise SealedDependencyFailure("P75-SEALED-LOCK-OPEN") from error



def _open_kernel_lock(name: str) -> _KernelLockState:
    if os.name == "nt":
        library = _windows_kernel32()
        handle = library.CreateMutexW(None, 0, name)
        if not handle:
            raise SealedDependencyFailure("P75-SEALED-LOCK-OPEN")
        return _KernelLockState("windows-mutex", name, handle)
    if not _linux_socket_lock_supported():
        raise SealedDependencyFailure("P75-SEALED-LOCK-OPEN")
    _require_internal_lock_manager()
    return _KernelLockState("linux-abstract-unix-socket", name, None)



def _acquire_kernel_lock(lock_state: _KernelLockState) -> None:
    if lock_state.kind == "windows-mutex":
        library = _windows_kernel32()
        timeout = max(1, int(_SEALED_LOCK_TIMEOUT_SECONDS * 1000))
        result = library.WaitForSingleObject(lock_state.handle, timeout)
        if result in (_WINDOWS_WAIT_OBJECT_0, _WINDOWS_WAIT_ABANDONED):
            return
        raise SealedDependencyFailure("P75-SEALED-LOCK-ACQUIRE")

    deadline = time.monotonic() + _SEALED_LOCK_TIMEOUT_SECONDS
    while True:
        lock_socket = _open_linux_kernel_socket()
        try:
            lock_socket.bind(lock_state.name)
        except OSError as error:
            try:
                lock_socket.close()
            except OSError as close_error:
                raise SealedDependencyFailure("P75-SEALED-LOCK-ACQUIRE") from close_error
            if error.errno == errno.EINTR and time.monotonic() < deadline:
                continue
            if error.errno == errno.EADDRINUSE and time.monotonic() < deadline:
                time.sleep(_SEALED_LOCK_POLL_SECONDS)
                continue
            raise SealedDependencyFailure("P75-SEALED-LOCK-ACQUIRE") from error
        lock_state.handle = lock_socket
        return



def _release_kernel_lock(lock_state: _KernelLockState) -> None:
    if lock_state.kind != "windows-mutex":
        return
    library = _windows_kernel32()
    if not library.ReleaseMutex(lock_state.handle):
        raise SealedDependencyFailure("P75-SEALED-LOCK-RELEASE")



def _close_kernel_handle(lock_state: _KernelLockState) -> None:
    handle = lock_state.handle
    lock_state.handle = None
    if handle is None:
        return
    if lock_state.kind == "windows-mutex":
        library = _windows_kernel32()
        if not library.CloseHandle(handle):
            raise SealedDependencyFailure("P75-SEALED-LOCK-RELEASE")
        return
    if not isinstance(handle, socket.socket):
        raise SealedDependencyFailure("P75-SEALED-LOCK-RELEASE")
    try:
        handle.close()
    except OSError as error:
        raise SealedDependencyFailure("P75-SEALED-LOCK-RELEASE") from error



def _close_kernel_lock(lock_state: _KernelLockState, *, acquired: bool) -> None:
    release_error: BaseException | None = None
    if acquired:
        try:
            _release_kernel_lock(lock_state)
        except BaseException as error:
            release_error = error
    try:
        _close_kernel_handle(lock_state)
    except BaseException as error:
        if release_error is None:
            release_error = error
    if release_error is not None:
        raise release_error



def _same_lock_owner(active: _ActiveSealedLoadingLock) -> bool:
    return (
        active.owner_pid == _current_pid()
        and active.owner_thread_id == _current_thread_id()
        and active.owner_token.live
        and active.lock_state.handle is not None
    )



def _register_active_loading_lock(active: _ActiveSealedLoadingLock) -> None:
    _require_internal_lock_manager().register_active_lock_state(active)



def _unregister_active_loading_lock(active: _ActiveSealedLoadingLock) -> None:
    _require_internal_lock_manager().unregister_active_lock_state(active)



def _reject_cross_instance_reentry(lock_name: str) -> None:
    if _require_internal_lock_manager().advisory_conflict(
        lock_name, _current_pid(), _current_thread_id()
    ):
        raise SealedDependencyFailure("P75-SEALED-LOCK-CROSS-INSTANCE-REENTRY")



def _mark_cross_instance_reentry(active: _ActiveSealedLoadingLock) -> None:
    _require_internal_lock_manager().advisory_mark(active)



def _clear_cross_instance_reentry(active: _ActiveSealedLoadingLock) -> None:
    _require_internal_lock_manager().advisory_clear(active)



def _invalidate_active_loading_lock(active: _ActiveSealedLoadingLock) -> _KernelLockState:
    active.owner_token.live = False
    detached = _KernelLockState(
        active.lock_state.kind, active.lock_state.name, active.lock_state.handle
    )
    active.lock_state.handle = None
    active.depth = 0
    return detached



def _active_lock_view(active: _ActiveSealedLoadingLock) -> Mapping[str, object]:
    return MappingProxyType(
        {
            "depth": active.depth,
            "kind": active.lock_state.kind,
            "name": active.lock_state.name,
            "owner_pid": active.owner_pid,
            "owner_thread_id": active.owner_thread_id,
            "live": active.owner_token.live,
        }
    )



def _normalize_active_loading_lock() -> _ActiveSealedLoadingLock | None:
    active = _ACTIVE_SEALED_LOADING_LOCK.get()
    if active is None:
        return None
    if _same_lock_owner(active):
        return active
    detached: _KernelLockState | None = None
    try:
        if active.owner_pid != _current_pid() and active.lock_state.handle is not None:
            detached = _invalidate_active_loading_lock(active)
            _clear_cross_instance_reentry(active)
            _unregister_active_loading_lock(active)
    finally:
        _ACTIVE_SEALED_LOADING_LOCK.set(None)
    if detached is not None:
        _close_kernel_lock(detached, acquired=False)
    return None


@contextlib.contextmanager
def _sealed_loading_lock() -> Mapping[str, object]:
    active = _normalize_active_loading_lock()
    if active is not None:
        active.depth += 1
        try:
            yield _active_lock_view(active)
        finally:
            if _same_lock_owner(active):
                current = _normalize_active_loading_lock()
                if current is not active or active.depth < 2:
                    raise SealedDependencyFailure("P75-SEALED-LOCK-STATE")
                active.depth -= 1
        return

    kernel_name = _kernel_lock_name()
    _reject_cross_instance_reentry(kernel_name)
    kernel_lock = _open_kernel_lock(kernel_name)
    acquired = False
    token: contextvars.Token[_ActiveSealedLoadingLock | None] | None = None
    active_state: _ActiveSealedLoadingLock | None = None
    release_error: BaseException | None = None
    advisory_marked = False
    registered_active = False
    try:
        _acquire_kernel_lock(kernel_lock)
        acquired = True
        active_state = _ActiveSealedLoadingLock(
            kernel_lock, _current_pid(), _current_thread_id(), _KernelLockToken()
        )
        _mark_cross_instance_reentry(active_state)
        advisory_marked = True
        _register_active_loading_lock(active_state)
        registered_active = True
        token = _ACTIVE_SEALED_LOADING_LOCK.set(active_state)
    except BaseException as error:
        if active_state is not None:
            try:
                if advisory_marked:
                    _clear_cross_instance_reentry(active_state)
                if registered_active:
                    _unregister_active_loading_lock(active_state)
            except BaseException as cleanup_error:
                error = cleanup_error
        try:
            _close_kernel_lock(kernel_lock, acquired=acquired)
        except BaseException as cleanup_error:
            raise cleanup_error from error
        raise
    try:
        assert active_state is not None
        yield _active_lock_view(active_state)
    finally:
        current_pid = _current_pid()
        current_thread_id = _current_thread_id()
        detached_lock: _KernelLockState | None = None
        try:
            if (
                active_state is not None
                and active_state.owner_pid == current_pid
                and active_state.owner_thread_id == current_thread_id
            ):
                current = _normalize_active_loading_lock()
                if current is not active_state or active_state.depth != 1:
                    release_error = SealedDependencyFailure("P75-SEALED-LOCK-STATE")
                else:
                    detached_lock = _invalidate_active_loading_lock(active_state)
                    _clear_cross_instance_reentry(active_state)
                    _unregister_active_loading_lock(active_state)
            else:
                _normalize_active_loading_lock()
        except BaseException as error:
            release_error = error
        try:
            if token is not None:
                _ACTIVE_SEALED_LOADING_LOCK.reset(token)
        except BaseException as error:
            if release_error is None:
                release_error = error
        try:
            _close_kernel_lock(
                detached_lock if detached_lock is not None else kernel_lock,
                acquired=bool(
                    acquired
                    and active_state is not None
                    and active_state.owner_pid == current_pid
                    and active_state.owner_thread_id == current_thread_id
                    and detached_lock is not None
                ),
            )
        except BaseException as error:
            if release_error is None:
                release_error = error
        if release_error is not None:
            raise release_error



def _exec_bound_module(
    name: str,
    source: Path,
    content: bytes,
    code: str,
    failure_type: type[BaseException] = SealedDependencyFailure,
    keep_loaded: bool = False,
) -> ModuleType:
    module = ModuleType(name)
    module.__file__ = os.fspath(source)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[name] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except BaseException as error:
        current = sys.modules.get(name, _MISSING)
        if current is module:
            sys.modules.pop(name, None)
        else:
            raise failure_type(code) from error
        if isinstance(error, (ImportError, OSError, RuntimeError, SyntaxError, ValueError)):
            raise failure_type(code) from error
        raise
    current = sys.modules.get(name, _MISSING)
    if current is not module:
        raise failure_type(code)
    if not keep_loaded:
        sys.modules.pop(name, None)
    return module



def _signature(
    module: ModuleType, name: str, parameters: tuple[str, ...], code: str
) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure(code)


@contextlib.contextmanager
def _installed_sealed_dependencies(
    dependencies: ModuleType, code: str
) -> Mapping[str, object]:
    with _sealed_loading_lock():
        previous = sys.modules.get(_SEALED_LOADING_SLOT, _MISSING)
        sys.modules[_SEALED_LOADING_SLOT] = dependencies
        try:
            yield {"previous": previous, "slot": _SEALED_LOADING_SLOT}
        finally:
            current = sys.modules.get(_SEALED_LOADING_SLOT, _MISSING)
            if current is not dependencies:
                raise SealedDependencyFailure(code)
            if previous is _MISSING:
                sys.modules.pop(_SEALED_LOADING_SLOT, None)
            else:
                sys.modules[_SEALED_LOADING_SLOT] = previous



def _load_exact_p72(bound: _VerifiedRelease, code: str) -> ModuleType:
    return _exec_bound_module(
        "p75_exact_p72",
        bound.root / P72.source,
        bound.files[P72.source],
        code,
    )


def load_exact_p72_stack(repo_root: Path) -> tuple[ModuleType, ModuleType, ModuleType, ModuleType]:
    with _sealed_loading_lock():
        bound = _verified_release(repo_root, P72, "P75-P72-IDENTITY")
        p72 = _load_exact_p72(bound, "P75-P72-IMPORT")
        _signature(
            p72,
            "run_capability_bound_diagnostic_executor",
            (
                "repo_root",
                "descriptor_root",
                "private_runtime_root",
                "p27_cycle_root",
                "ubuntu_runtime_parent",
            ),
            "P75-P72-API",
        )
        _signature(
            p72,
            "_run_qualification_executor",
            (
                "repo_root",
                "descriptor_root",
                "private_runtime_root",
                "p27_cycle_root",
                "controls",
            ),
            "P75-P72-API",
        )
        for name in (
            "_close_handles",
            "_known_failure",
            "_copy_expected",
            "_frozen_descriptor",
            "_freeze_descriptors",
            "_normalize_result",
            "_prelaunch_dispatch",
            "_with_terminal_failure_types",
            "load_exact_p51",
            "load_exact_p56",
        ):
            if not callable(getattr(p72, name, None)):
                raise SealedDependencyFailure("P75-P72-API")
        if (
            not isinstance(getattr(p72, "_NativeWslSession", None), type)
            or not isinstance(getattr(p72, "ExecutorFailure", None), type)
            or getattr(p72, "REQUEST_COUNT", None) != 69
            or getattr(p72, "WSL_PLATFORM", None) != "ubuntu-24.04-x86_64"
            or getattr(p72, "WSL_SCHEMA", None)
            != "ferris.pulse-57-wsl-capability-session/v1"
        ):
            raise SealedDependencyFailure("P75-P72-API")
        try:
            p57 = getattr(p72, "_P57", None)
            p51 = p72.load_exact_p51(repo_root)
            p56 = p72.load_exact_p56(repo_root)
        except SealedDependencyFailure:
            raise
        except BaseException as error:
            raise SealedDependencyFailure("P75-P72-STACK") from error
        dependencies = getattr(p72, "_SEALED", None)
        if (
            not isinstance(p57, ModuleType)
            or not isinstance(dependencies, ModuleType)
            or not callable(getattr(dependencies, "load_p51_synthetic_fixture", None))
        ):
            raise SealedDependencyFailure("P75-P72-API")
        p72._p75_bound_sealed_dependencies = dependencies
        return p72, p57, p51, p56


def load_p51_synthetic_fixture(repo_root: Path, p51: ModuleType) -> ModuleType:
    with _sealed_loading_lock():
        p72, _p57, _p51, _p56 = load_exact_p72_stack(repo_root)
        dependencies = getattr(p72, "_SEALED", None)
        loader = getattr(dependencies, "load_p51_synthetic_fixture", None)
        if not callable(loader):
            raise SealedDependencyFailure("P75-P72-API")
        fixture = loader(repo_root, p51)
        if not callable(getattr(fixture, "create_descriptor_root", None)):
            raise SealedDependencyFailure("P75-P72-API")
        return fixture



def release_identities() -> dict[str, object]:
    return {
        "pulse_72": {
            "commit": P72_COMMIT,
            "manifest_aggregate": P72.manifest_aggregate,
            "manifest_raw_sha256": P72.manifest_raw_sha256,
            "receipt_payload_sha256": P72.receipt_payload_sha256,
            "receipt_raw_sha256": P72.receipt_raw_sha256,
            "release_root": P72_RELEASE_ROOT,
            "seal_payload_sha256": P72.seal_payload_sha256,
            "seal_raw_sha256": P72.seal_raw_sha256,
            "source_sha256": P72.source_sha256,
            "stack": json.loads(canonical_bytes(P72_STACK)),
        }
    }


__all__ = [
    "SealedDependencyFailure",
    "_bind_internal_lock_manager",
    "canonical_bytes",
    "load_exact_p72_stack",
    "load_p51_synthetic_fixture",
    "release_identities",
    "sha256_bytes",
]
