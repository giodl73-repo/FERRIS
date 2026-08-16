"""Byte-bound exact imports for Pulse 59's witness-preserving P58 wrapper."""
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
from collections.abc import Mapping as ABCMapping
from dataclasses import dataclass
from pathlib import Path
from types import MappingProxyType, ModuleType
from typing import Mapping


P58_COMMIT = "7c66d70800edd06642274ed4f2e4aee224b7583e"
P58_RELEASE_ROOT = (
    "docs/simulations/profile-diff-held-out/"
    "pulse-58-ordered-capability-materialization-executor-release"
)
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
_SEALED_LOADING_SLOT = "sealed_dependencies"
_MISSING = object()
_SEALED_LOCK_TIMEOUT_SECONDS = 300.0
_SEALED_LOCK_POLL_SECONDS = 0.05
_KERNEL_LOCK_NAMESPACE_PREFIX = "ferris-p59"
_WINDOWS_WAIT_OBJECT_0 = 0x00000000
_WINDOWS_WAIT_ABANDONED = 0x00000080
_WINDOWS_WAIT_TIMEOUT = 0x00000102
_WINDOWS_WAIT_FAILED = 0xFFFFFFFF
_P59_INTERNAL_LOCK_MANAGER: object | None = None


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
    contextvars.ContextVar("p59_active_sealed_loading_lock", default=None)
)


class SealedDependencyFailure(RuntimeError):
    """A sealed Pulse 58 predecessor or callable binding failed."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def _bind_internal_lock_manager(manager: object) -> None:
    if not callable(getattr(manager, "register_active_lock_state", None)) or not callable(
        getattr(manager, "unregister_active_lock_state", None)
    ):
        raise SealedDependencyFailure("P59-SEALED-LOCK-STATE")
    global _P59_INTERNAL_LOCK_MANAGER
    _P59_INTERNAL_LOCK_MANAGER = manager


def _require_internal_lock_manager() -> object:
    if _P59_INTERNAL_LOCK_MANAGER is None:
        raise SealedDependencyFailure("P59-SEALED-LOCK-STATE")
    return _P59_INTERNAL_LOCK_MANAGER


P58 = ReleaseIdentity(
    "pulse-58-ordered-capability-materialization-executor-release",
    "ordered_capability_materialization_executor.py",
    "sha256:cd2f8e2ddb744f165c8e5357276f055c31efed494716ac11cde3f1de710547ef",
    "ferris.pulse-58-ordered-capability-materialization-executor-manifest/v1",
    "sha256:8e321041d69a5953aa73a2c67c344a55c25c2fec25008828151d5cb5e16f968f",
    "sha256:7f09008023e9bcf4d111e7b8b82320fe3b99467480a81d8ae452c9f04025a47c",
    13,
    15,
    "sha256:9ad46590136de604d6dc1bc9929ac60c3daf56af147905d33d811d4b42037289",
    "sha256:49fb8397ffbd344552b6a4ff1880e3816af35ace55cff3ba6793c3dfef91e7e6",
    "sha256:5dadcb7a6c7b926d2d1166865df8cec075a1cfa4da957637350e5851dade4e93",
    "sha256:9bd64239dca64d8facbe493d2c243d91b4e6d53014efa17fcb9eb4ae2eaffdd5",
)

P43_IDENTITIES = {
    "aggregate": "sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346",
    "manifest": "sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4",
    "seal": "sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1",
    "source": "sha256:38ebc7ce84ae29c2ad20ada593d8baeb0352b59e7c48438c4a9c224a0ea4a6c6",
}
P47_IDENTITIES = {
    "aggregate": "sha256:5cb97276ee2752888c40d44a50e45079c9e550f7e26398e5aa4841d98083143d",
    "manifest": "sha256:44d5c72b9eb09dc7e24b476a4535fed662eadde3edee6ecbfe1fdfa644082f8b",
    "seal": "sha256:a00478e73897781ddd88e8e0fcbca2d1453a72758cbbd8ec06ccd9d0c228f681",
    "source": "sha256:4a402d3c2e034597a574368e628af0b87966b74ec2cdef947b38db2881cf4760",
}
P58_PREDECESSORS = {
    "pulse_35": {
        "aggregate": "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69",
        "manifest": "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1",
    },
    "pulse_39": {
        "manifest": "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c",
        "seal": "sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b",
    },
    "pulse_41": {
        "manifest": "sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8",
        "seal": "sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf",
    },
    "pulse_52": {
        "manifest": "sha256:e585d6baaf83783ff1a1c65e1d3f281ce1d3afd9806f9cb9811b328eff9811da",
        "seal": "sha256:46d9e8bb1aa75780fb7397fd4833e13c5e28c0ec79254185ef6da793e4ed7f84",
    },
    "pulse_56": {
        "manifest": "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a",
        "seal": "sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747",
    },
    "pulse_57": {
        "manifest": "sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291",
        "seal": "sha256:727e1806ede5ca9b5438b5f3b00bec3a59b4e36404ccc1b72e6adb661ebee144",
    },
}

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
            raise SealedDependencyFailure("P59-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _safe_regular(
    path: Path, code: str = "P59-SEALED-TREE", maximum: int = 4_194_304
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


def _release_root(repo_root: Path, directory: str, code: str) -> Path:
    try:
        root = repo_root.resolve(strict=True)
        if not root.is_absolute():
            raise SealedDependencyFailure(code)
        release = root / "docs" / "simulations" / "profile-diff-held-out" / directory
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
    if len({path for path, _digest in normalized}) != len(normalized):
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
    release = _release_root(repo_root, identity.directory, code)
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
        type(files) is not list
        or manifest.get("schema") != identity.manifest_schema
        or manifest.get("aggregate") != identity.manifest_aggregate
        or manifest.get("file_count") != identity.manifest_file_count
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
        manifest.get("release_tree_file_count") != identity.release_tree_file_count
        or (
            type(manifest.get("total_bytes")) is int and manifest["total_bytes"] != total
        )
        or source_digest != identity.source_sha256
    ):
        raise SealedDependencyFailure(code)
    return _VerifiedRelease(release, MappingProxyType(bound))


def _self_path() -> Path:
    try:
        return Path(__file__).resolve(strict=True)
    except OSError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-OPEN") from error


_WINDOWS_KERNEL32: object | None = None


def _current_pid() -> int:
    return os.getpid()


def _current_thread_id() -> int:
    return threading.get_ident()


def _kernel_lock_name() -> str:
    path = _self_path()
    digest = hashlib.sha256()
    digest.update(os.fsencode(os.fspath(path)))
    digest.update(b"\0")
    digest.update(sha256_bytes(_safe_regular(path, "P59-SEALED-LOCK-OPEN")).encode("ascii"))
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
            raise SealedDependencyFailure("P59-SEALED-LOCK-OPEN") from error
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
        raise SealedDependencyFailure("P59-SEALED-LOCK-OPEN")
    try:
        return socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    except OSError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-OPEN") from error


def _open_kernel_lock(name: str) -> _KernelLockState:
    if os.name == "nt":
        library = _windows_kernel32()
        handle = library.CreateMutexW(None, 0, name)
        if not handle:
            raise SealedDependencyFailure("P59-SEALED-LOCK-OPEN")
        return _KernelLockState("windows-mutex", name, handle)
    if not _linux_socket_lock_supported():
        raise SealedDependencyFailure("P59-SEALED-LOCK-OPEN")
    _require_internal_lock_manager()
    return _KernelLockState("linux-abstract-unix-socket", name, None)


def _acquire_kernel_lock(lock_state: _KernelLockState) -> None:
    if lock_state.kind == "windows-mutex":
        library = _windows_kernel32()
        timeout = max(1, int(_SEALED_LOCK_TIMEOUT_SECONDS * 1000))
        result = library.WaitForSingleObject(lock_state.handle, timeout)
        if result in (_WINDOWS_WAIT_OBJECT_0, _WINDOWS_WAIT_ABANDONED):
            return
        raise SealedDependencyFailure("P59-SEALED-LOCK-ACQUIRE")

    deadline = time.monotonic() + _SEALED_LOCK_TIMEOUT_SECONDS
    while True:
        lock_socket = _open_linux_kernel_socket()
        try:
            lock_socket.bind(lock_state.name)
        except OSError as error:
            try:
                lock_socket.close()
            except OSError as close_error:
                raise SealedDependencyFailure("P59-SEALED-LOCK-ACQUIRE") from close_error
            if error.errno == errno.EINTR and time.monotonic() < deadline:
                continue
            if error.errno == errno.EADDRINUSE and time.monotonic() < deadline:
                time.sleep(_SEALED_LOCK_POLL_SECONDS)
                continue
            raise SealedDependencyFailure("P59-SEALED-LOCK-ACQUIRE") from error
        lock_state.handle = lock_socket
        return


def _release_kernel_lock(lock_state: _KernelLockState) -> None:
    if lock_state.kind != "windows-mutex":
        return
    library = _windows_kernel32()
    if not library.ReleaseMutex(lock_state.handle):
        raise SealedDependencyFailure("P59-SEALED-LOCK-RELEASE")


def _close_kernel_handle(lock_state: _KernelLockState) -> None:
    handle = lock_state.handle
    lock_state.handle = None
    if handle is None:
        return
    if lock_state.kind == "windows-mutex":
        library = _windows_kernel32()
        if not library.CloseHandle(handle):
            raise SealedDependencyFailure("P59-SEALED-LOCK-RELEASE")
        return
    if not isinstance(handle, socket.socket):
        raise SealedDependencyFailure("P59-SEALED-LOCK-RELEASE")
    try:
        handle.close()
    except OSError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-RELEASE") from error


def _close_kernel_lock(lock_state: _KernelLockState, *, acquired: bool) -> None:
    release_error: BaseException | None = None
    if acquired:
        try:
            _release_kernel_lock(lock_state)
        except BaseException as error:  # pragma: no cover - fail-closed cleanup
            release_error = error
    try:
        _close_kernel_handle(lock_state)
    except BaseException as error:  # pragma: no cover - fail-closed cleanup
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
                    raise SealedDependencyFailure("P59-SEALED-LOCK-STATE")
                active.depth -= 1
        return

    kernel_name = _kernel_lock_name()
    kernel_lock = _open_kernel_lock(kernel_name)
    acquired = False
    token: contextvars.Token[_ActiveSealedLoadingLock | None] | None = None
    active_state: _ActiveSealedLoadingLock | None = None
    release_error: BaseException | None = None
    try:
        _acquire_kernel_lock(kernel_lock)
        acquired = True
        active_state = _ActiveSealedLoadingLock(
            kernel_lock, _current_pid(), _current_thread_id(), _KernelLockToken()
        )
        _register_active_loading_lock(active_state)
        token = _ACTIVE_SEALED_LOADING_LOCK.set(active_state)
    except BaseException as error:
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
                    release_error = SealedDependencyFailure("P59-SEALED-LOCK-STATE")
                else:
                    detached_lock = _invalidate_active_loading_lock(active_state)
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
        if isinstance(
            error, (ImportError, OSError, RuntimeError, SyntaxError, ValueError)
        ):
            raise failure_type(code) from error
        raise
    current = sys.modules.get(name, _MISSING)
    if current is not module:
        raise failure_type(code)
    if not keep_loaded:
        sys.modules.pop(name, None)
    return module


def _patch_loaded_dependency_module(module: object, code: str) -> ModuleType:
    if not isinstance(module, ModuleType):
        raise SealedDependencyFailure(code)
    if getattr(module, "__p59_bound_dependency_patch__", False):
        return module
    failure_type = getattr(module, "SealedDependencyFailure", None)
    if not isinstance(failure_type, type):
        raise SealedDependencyFailure(code)

    def patched_exec(*arguments: object) -> ModuleType:
        keep_loaded = False
        if len(arguments) == 4:
            name, source, content, child_code = arguments
        elif len(arguments) == 3:
            name, source, content = arguments
            child_code = code
            keep_loaded = True
        else:
            raise failure_type(code)
        if (
            type(name) is not str
            or not isinstance(source, Path)
            or type(content) is not bytes
            or type(child_code) is not str
        ):
            raise failure_type(code)
        return _exec_bound_module(
            name,
            source,
            content,
            child_code,
            failure_type,
            keep_loaded=keep_loaded,
        )

    def patched_load_with_bound_dependencies(
        prefix: str, bound: object, source: str, child_code: str
    ) -> ModuleType:
        dependency_path = "sealed_dependencies.py"
        files = getattr(bound, "files", None)
        root = getattr(bound, "root", None)
        if (
            not isinstance(files, ABCMapping)
            or not isinstance(root, Path)
            or dependency_path not in files
        ):
            raise failure_type(child_code)
        dependencies = patched_exec(
            f"{prefix}_dependencies",
            root / dependency_path,
            files[dependency_path],
            child_code,
        )
        _patch_loaded_dependency_module(dependencies, child_code)
        with _installed_sealed_dependencies(dependencies, child_code):
            return patched_exec(prefix, root / source, files[source], child_code)

    module._exec_bound_module = patched_exec
    module._load_with_bound_dependencies = patched_load_with_bound_dependencies
    module.__p59_bound_dependency_patch__ = True
    return module


def _signature(
    module: ModuleType, name: str, parameters: tuple[str, ...], code: str
) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure(code)


def _load_with_bound_dependencies(
    prefix: str, bound: _VerifiedRelease, source: str, code: str
) -> ModuleType:
    dependency_path = "sealed_dependencies.py"
    if dependency_path not in bound.files:
        raise SealedDependencyFailure(code)
    dependencies = _patch_loaded_dependency_module(
        _exec_bound_module(
            f"{prefix}_dependencies",
            bound.root / dependency_path,
            bound.files[dependency_path],
            code,
        ),
        code,
    )
    with _installed_sealed_dependencies(dependencies, code):
        return _exec_bound_module(prefix, bound.root / source, bound.files[source], code)


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


def load_pulse58(
    repo_root: Path,
) -> tuple[ModuleType, ModuleType, ModuleType, ModuleType, ModuleType, ModuleType]:
    """Return fresh exact Pulse 58 modules after exact-tree verification."""
    with _sealed_loading_lock():
        bound = _verified_release(repo_root, P58, "P59-P58-IDENTITY")
        p58 = _load_with_bound_dependencies(
            "p59_exact_p58", bound, P58.source, "P59-P58-IMPORT"
        )
        _signature(
            p58,
            "run_ordered_capability_materialization_executor",
            (
                "repo_root",
                "private_runtime_root",
                "p27_cycle_root",
                "p39_checkout_root",
                "p41_final_root",
                "ubuntu_runtime_parent",
            ),
            "P59-P58-API",
        )
        _signature(
            p58,
            "_run_qualification_executor",
            (
                "repo_root",
                "private_runtime_root",
                "p27_cycle_root",
                "p39_checkout_root",
                "p41_final_root",
                "seed_bytes",
                "p27_runner",
                "p56",
                "open_wsl",
            ),
            "P59-P58-API",
        )
        _signature(
            p58,
            "_terminal",
            ("p43", "events", "gate", "code", "record"),
            "P59-P58-API",
        )
        if (
            tuple(getattr(p58, "P58_GATE_IDS", ())) != P58_GATE_IDS
            or getattr(p58, "P39_CALLER_AUTHORITY_PRECONDITION", None)
            != "future-authority-supplied-fresh-anonymous-exact-cutoff-root"
            or not isinstance(
                getattr(p58, "OrderedCapabilityMaterializationResult", None), type
            )
            or not isinstance(getattr(p58, "P58Failure", None), type)
            or not isinstance(getattr(p58, "IndeterminateCleanup", None), type)
        ):
            raise SealedDependencyFailure("P59-P58-API")

        try:
            p52 = p58.load_exact_p52_stage_reader(repo_root)
            p57, p51, _p56 = p58.load_exact_p57_stack(repo_root)
            p43, _p45, p47 = p51.load_terminal_dependencies(repo_root)
        except SealedDependencyFailure:
            raise
        except BaseException as error:
            raise SealedDependencyFailure("P59-P58-STACK") from error

        for module, name, parameters in (
            (
                p52,
                "_cleanup_terminal_publication",
                ("p51", "parent", "p43_root", "witness_root", "private_record"),
            ),
            (
                p52,
                "_published_terminal_summary",
                ("p43", "p47", "summary", "p43_root", "witness_root"),
            ),
            (p52, "_p47_failure_posture", ("p47", "summary")),
            (p52, "_published_witness_posture", ("value",)),
            (p51, "load_terminal_dependencies", ("repo_root",)),
            (
                p51,
                "invoke_terminal_pulse47_once",
                ("terminal", "result", "p43_final_root", "witness_final_root"),
            ),
            (p43, "validate_catalog", ("value",)),
            (p43, "validate_events", ("catalog", "value")),
            (p43, "verify_publication_directory", ("root",)),
            (p47, "verify_witness_directory", ("root",)),
        ):
            _signature(module, name, parameters, "P59-P58-STACK")
        if not isinstance(getattr(p51, "TerminalPulse47Once", None), type):
            raise SealedDependencyFailure("P59-P58-STACK")

        return p58, p52, p57, p51, p43, p47


def release_identities() -> dict[str, object]:
    return {
        "pulse_43": dict(P43_IDENTITIES),
        "pulse_47": dict(P47_IDENTITIES),
        "pulse_58": {
            "commit": P58_COMMIT,
            "manifest_aggregate": P58.manifest_aggregate,
            "manifest_raw_sha256": P58.manifest_raw_sha256,
            "receipt_payload_sha256": P58.receipt_payload_sha256,
            "receipt_raw_sha256": P58.receipt_raw_sha256,
            "release_root": P58_RELEASE_ROOT,
            "seal_payload_sha256": P58.seal_payload_sha256,
            "seal_raw_sha256": P58.seal_raw_sha256,
            "source_sha256": P58.source_sha256,
            "stack": json.loads(canonical_bytes(P58_PREDECESSORS)),
        },
    }


__all__ = [
    "P58_COMMIT",
    "P58_GATE_IDS",
    "P58_RELEASE_ROOT",
    "SealedDependencyFailure",
    "canonical_bytes",
    "load_pulse58",
    "release_identities",
    "sha256_bytes",
]
