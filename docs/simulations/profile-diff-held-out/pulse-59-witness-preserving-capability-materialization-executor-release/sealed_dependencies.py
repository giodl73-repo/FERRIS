"""Byte-bound exact imports for Pulse 59's witness-preserving P58 wrapper."""
from __future__ import annotations

import contextlib
import contextvars
import errno
import hashlib
import inspect
import json
import os
import stat
import sys
import time
from collections.abc import Mapping as ABCMapping
from dataclasses import dataclass
from pathlib import Path
from types import MappingProxyType, ModuleType
from typing import Mapping

if os.name == "nt":  # pragma: no cover - covered by Windows validation
    import msvcrt
else:  # pragma: no cover - covered by non-Windows validation
    import fcntl


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
_SEALED_LOCK_TIMEOUT_SECONDS = 30.0
_SEALED_LOCK_POLL_SECONDS = 0.05
_SEALED_LOCK_CONTENT = b"pulse59-sealed-load-lock\n"
_SEALED_LOCK_TARGET_DIRECTORY = "target"
_SEALED_LOCK_DIRECTORY = "pulse-59-sealed-loader-locks"
_WINDOWS_REPARSE_POINT_ATTRIBUTE = getattr(stat, "FILE_ATTRIBUTE_REPARSE_POINT", 0)
_ACTIVE_SEALED_LOADING_LOCK: contextvars.ContextVar[Mapping[str, object] | None] = (
    contextvars.ContextVar("p59_active_sealed_loading_lock", default=None)
)


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


@dataclass(frozen=True)
class _LockPathContext:
    repo_root: Path
    target_root: Path
    lock_root: Path
    lock_file: Path


class SealedDependencyFailure(RuntimeError):
    """A sealed Pulse 58 predecessor or callable binding failed."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


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
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH") from error


def _is_symlink_or_reparse(metadata: os.stat_result) -> bool:
    if stat.S_ISLNK(metadata.st_mode):
        return True
    attributes = getattr(metadata, "st_file_attributes", None)
    return (
        os.name == "nt"
        and type(attributes) is int
        and bool(attributes & _WINDOWS_REPARSE_POINT_ATTRIBUTE)
    )


def _same_identity(left: os.stat_result, right: os.stat_result) -> bool:
    return (
        stat.S_IFMT(left.st_mode),
        left.st_dev,
        left.st_ino,
    ) == (
        stat.S_IFMT(right.st_mode),
        right.st_dev,
        right.st_ino,
    )


def _checked_lstat(path: Path, code: str) -> os.stat_result:
    try:
        return os.lstat(path)
    except OSError as error:
        raise SealedDependencyFailure(code) from error


def _verified_directory(
    path: Path, code: str, *, create: bool, mode: int = 0o700
) -> os.stat_result:
    before = _checked_lstat(path, code) if os.path.lexists(path) else None
    if before is None:
        if not create:
            raise SealedDependencyFailure(code)
        try:
            os.mkdir(path, mode)
        except FileExistsError:
            pass
    else:
        if _is_symlink_or_reparse(before) or not stat.S_ISDIR(before.st_mode):
            raise SealedDependencyFailure(code)
    after = _checked_lstat(path, code)
    if _is_symlink_or_reparse(after) or not stat.S_ISDIR(after.st_mode):
        raise SealedDependencyFailure(code)
    if before is not None and not _same_identity(before, after):
        raise SealedDependencyFailure(code)
    return after


def _verified_regular(path: Path, code: str) -> os.stat_result:
    metadata = _checked_lstat(path, code)
    if _is_symlink_or_reparse(metadata) or not stat.S_ISREG(metadata.st_mode):
        raise SealedDependencyFailure(code)
    return metadata


def _lock_path_context() -> _LockPathContext:
    path = _self_path()
    try:
        repo_root = path.parents[4]
    except IndexError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH") from error
    target_root = repo_root / _SEALED_LOCK_TARGET_DIRECTORY
    lock_root = target_root / _SEALED_LOCK_DIRECTORY
    lock_file = lock_root / f"{hashlib.sha256(os.fsencode(os.fspath(path))).hexdigest()}.lock"
    return _LockPathContext(repo_root, target_root, lock_root, lock_file)


def _lock_file_context(path: Path) -> _LockPathContext:
    try:
        lock_root = path.parent
        target_root = lock_root.parent
        repo_root = target_root.parent
    except IndexError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH") from error
    if (
        not path.is_absolute()
        or target_root.name != _SEALED_LOCK_TARGET_DIRECTORY
        or lock_root.name != _SEALED_LOCK_DIRECTORY
        or path.suffix != ".lock"
    ):
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")
    return _LockPathContext(repo_root, target_root, lock_root, path)


def _lock_file_path() -> Path:
    return _lock_path_context().lock_file


def _verified_lock_ancestors(
    context: _LockPathContext,
) -> tuple[os.stat_result, os.stat_result, os.stat_result]:
    repo_root = _verified_directory(
        context.repo_root, "P59-SEALED-LOCK-PATH", create=False
    )
    target_root = _verified_directory(
        context.target_root, "P59-SEALED-LOCK-PATH", create=True
    )
    lock_root = _verified_directory(
        context.lock_root, "P59-SEALED-LOCK-PATH", create=True
    )
    return repo_root, target_root, lock_root


def _reverify_lock_ancestors(
    context: _LockPathContext, expected: tuple[os.stat_result, os.stat_result, os.stat_result]
) -> None:
    actual = (
        _verified_directory(context.repo_root, "P59-SEALED-LOCK-PATH", create=False),
        _verified_directory(context.target_root, "P59-SEALED-LOCK-PATH", create=False),
        _verified_directory(context.lock_root, "P59-SEALED-LOCK-PATH", create=False),
    )
    for before, after in zip(expected, actual):
        if not _same_identity(before, after):
            raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")


def _open_lock_descriptor(path: Path) -> int:
    context = _lock_file_context(path)
    ancestor_state = _verified_lock_ancestors(context)
    descriptor: int | None = None
    success = False
    try:
        initial = _verified_regular(path, "P59-SEALED-LOCK-PATH") if os.path.lexists(path) else None
        flags = os.O_RDWR | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        if initial is None:
            try:
                descriptor = os.open(path, flags | os.O_CREAT | os.O_EXCL, 0o600)
            except FileExistsError:
                initial = _verified_regular(path, "P59-SEALED-LOCK-PATH")
        if descriptor is None:
            descriptor = os.open(path, flags, 0o600)
        opened_metadata = os.fstat(descriptor)
        if not stat.S_ISREG(opened_metadata.st_mode):
            raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")
        current = _verified_regular(path, "P59-SEALED-LOCK-PATH")
        if initial is not None and not _same_identity(initial, current):
            raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")
        if not _same_identity(current, opened_metadata):
            raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")
        _reverify_lock_ancestors(context, ancestor_state)
        if opened_metadata.st_size == 0:
            os.write(descriptor, _SEALED_LOCK_CONTENT)
        os.lseek(descriptor, 0, os.SEEK_SET)
        success = True
        return descriptor
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH") from error
    finally:
        if descriptor is not None and not success:
            try:
                os.close(descriptor)
            except OSError:
                pass


def _lock_contention(error: BaseException) -> bool:
    if isinstance(error, BlockingIOError):
        return True
    if not isinstance(error, OSError):
        return False
    return (
        error.errno in {errno.EACCES, errno.EAGAIN, errno.EDEADLK}
        or getattr(error, "winerror", None) in {33, 36, 158}
    )


def _acquire_descriptor_lock(descriptor: int) -> None:
    deadline = time.monotonic() + _SEALED_LOCK_TIMEOUT_SECONDS
    while True:
        try:
            if os.name == "nt":
                os.lseek(descriptor, 0, os.SEEK_SET)
                msvcrt.locking(descriptor, msvcrt.LK_NBLCK, 1)
            else:
                fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
            return
        except OSError as error:
            if not _lock_contention(error) or time.monotonic() >= deadline:
                raise SealedDependencyFailure("P59-SEALED-LOCK-ACQUIRE") from error
            time.sleep(_SEALED_LOCK_POLL_SECONDS)


def _release_descriptor_lock(descriptor: int) -> None:
    try:
        if os.name == "nt":
            os.lseek(descriptor, 0, os.SEEK_SET)
            msvcrt.locking(descriptor, msvcrt.LK_UNLCK, 1)
        else:
            fcntl.flock(descriptor, fcntl.LOCK_UN)
    except OSError as error:
        raise SealedDependencyFailure("P59-SEALED-LOCK-RELEASE") from error


def _revalidate_locked_path(path: Path, descriptor: int) -> None:
    context = _lock_file_context(path)
    _verified_lock_ancestors(context)
    opened = os.fstat(descriptor)
    if not stat.S_ISREG(opened.st_mode):
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")
    current = _verified_regular(path, "P59-SEALED-LOCK-PATH")
    if not _same_identity(current, opened):
        raise SealedDependencyFailure("P59-SEALED-LOCK-PATH")


def _close_lock_descriptor(descriptor: int, *, locked: bool) -> None:
    release_error: BaseException | None = None
    if locked:
        try:
            _release_descriptor_lock(descriptor)
        except BaseException as error:  # pragma: no cover - fail-closed cleanup
            release_error = error
    try:
        os.close(descriptor)
    except OSError as error:  # pragma: no cover - fail-closed cleanup
        if release_error is None:
            release_error = SealedDependencyFailure("P59-SEALED-LOCK-RELEASE")
    if release_error is not None:
        raise release_error


@contextlib.contextmanager
def _sealed_loading_lock() -> Mapping[str, object]:
    active = _ACTIVE_SEALED_LOADING_LOCK.get()
    if active is not None:
        yield active
        return

    path = _lock_file_path()
    descriptor = _open_lock_descriptor(path)
    locked = False
    token: contextvars.Token[Mapping[str, object] | None] | None = None
    lock_state: Mapping[str, object] | None = None
    try:
        _acquire_descriptor_lock(descriptor)
        locked = True
        _revalidate_locked_path(path, descriptor)
        lock_state = MappingProxyType({"descriptor": descriptor, "path": path})
        token = _ACTIVE_SEALED_LOADING_LOCK.set(lock_state)
    except BaseException as error:
        try:
            _close_lock_descriptor(descriptor, locked=locked)
        except BaseException as cleanup_error:
            raise cleanup_error from error
        raise
    try:
        assert lock_state is not None
        yield lock_state
    finally:
        if token is not None:
            _ACTIVE_SEALED_LOADING_LOCK.reset(token)
        _close_lock_descriptor(descriptor, locked=True)


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
