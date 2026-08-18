"""Identity-bound exact P56 loader for Pulse 86's WSL worker."""

from __future__ import annotations

import hashlib
import inspect
import json
import os
import stat
import sys
from dataclasses import dataclass
from pathlib import Path
from types import MappingProxyType, ModuleType
from typing import Mapping


class SealedDependencyFailure(RuntimeError):
    """A bundled predecessor was not the exact sealed release."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass(frozen=True)
class ReleaseIdentity:
    directory: str
    source: str
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


P56 = ReleaseIdentity(
    directory="pulse-56-retained-build-custody-release",
    source="retained_build_custody.py",
    manifest_schema="ferris.pulse-56-retained-build-custody-manifest/v1",
    manifest_raw_sha256="sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a",
    manifest_aggregate="sha256:728cdbf64c520869b36cb902a9ca2dcadb88e5ff4ff734ff054ff05e9851a400",
    manifest_file_count=8,
    release_tree_file_count=10,
    receipt_raw_sha256="sha256:9fd2368dc6c123707da40b6f9eefd04f8a680e40635c4f539db6818d34f19d98",
    receipt_payload_sha256="sha256:6006f98a103cd822dc51fb2e8297e3755848fea72e4ec50e15ca6cb04a83f8d5",
    seal_raw_sha256="sha256:cbb2fc8eeaf82b90f5275dd1e8ed406c0ab215d52d8233824dd9c9af390755a4",
    seal_payload_sha256="sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747",
)


def canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise SealedDependencyFailure("P86-WORKER-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _relative_parts(path: str) -> tuple[str, ...]:
    parts = tuple(path.split("/"))
    if not parts or any(not part or part in {".", ".."} for part in parts):
        raise SealedDependencyFailure("P86-WORKER-SEALED-TREE")
    return parts


def _open_directory(parent_fd: int, name: str, code: str) -> int:
    flags = os.O_RDONLY | getattr(os, "O_DIRECTORY", 0) | getattr(os, "O_NOFOLLOW", 0)
    try:
        metadata = os.lstat(name, dir_fd=parent_fd)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise SealedDependencyFailure(code)
        descriptor = os.open(name, flags, dir_fd=parent_fd)
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure(code) from error
    opened = os.fstat(descriptor)
    if not stat.S_ISDIR(opened.st_mode) or (
        opened.st_dev,
        opened.st_ino,
    ) != (metadata.st_dev, metadata.st_ino):
        os.close(descriptor)
        raise SealedDependencyFailure(code)
    return descriptor


def _safe_regular(
    bundle_root_fd: int, relative: str, code: str, maximum: int = 4_194_304
) -> bytes:
    parts = _relative_parts(relative)
    current_fd = bundle_root_fd
    opened: list[int] = []
    try:
        for directory in parts[:-1]:
            next_fd = _open_directory(current_fd, directory, code)
            opened.append(next_fd)
            current_fd = next_fd
        try:
            metadata = os.lstat(parts[-1], dir_fd=current_fd)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
                raise SealedDependencyFailure(code)
            descriptor = os.open(
                parts[-1],
                os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0),
                dir_fd=current_fd,
            )
        except SealedDependencyFailure:
            raise
        except OSError as error:
            raise SealedDependencyFailure(code) from error
        try:
            opened_stat = os.fstat(descriptor)
            if not stat.S_ISREG(opened_stat.st_mode) or (
                opened_stat.st_dev,
                opened_stat.st_ino,
            ) != (metadata.st_dev, metadata.st_ino):
                raise SealedDependencyFailure(code)
            content = bytearray()
            while chunk := os.read(descriptor, 65_536):
                content.extend(chunk)
                if len(content) > maximum:
                    raise SealedDependencyFailure(code)
            return bytes(content)
        except OSError as error:
            raise SealedDependencyFailure(code) from error
        finally:
            os.close(descriptor)
    finally:
        for handle in reversed(opened):
            os.close(handle)


def _sealed_json(content: bytes, code: str) -> dict[str, object]:
    try:
        value = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, SealedDependencyFailure) as error:
        raise SealedDependencyFailure(code) from error
    if type(value) is not dict:
        raise SealedDependencyFailure(code)
    return value


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


def _tree_paths(current_fd: int, prefix: str, result: set[str], code: str) -> None:
    try:
        listing = tuple(sorted(os.listdir("/proc/self/fd/" + str(current_fd))))
    except OSError as error:
        raise SealedDependencyFailure(code) from error
    for child in listing:
        try:
            metadata = os.lstat(child, dir_fd=current_fd)
        except OSError as error:
            raise SealedDependencyFailure(code) from error
        relative = child if not prefix else prefix + "/" + child
        if stat.S_ISLNK(metadata.st_mode):
            raise SealedDependencyFailure(code)
        if stat.S_ISDIR(metadata.st_mode):
            child_fd = _open_directory(current_fd, child, code)
            try:
                _tree_paths(child_fd, relative, result, code)
            finally:
                os.close(child_fd)
        elif stat.S_ISREG(metadata.st_mode):
            result.add(relative)
        else:
            raise SealedDependencyFailure(code)


def _open_release_root(bundle_root_fd: int, directory: str, code: str) -> int:
    current_fd = bundle_root_fd
    opened: list[int] = []
    try:
        for part in _relative_parts(
            "repository/docs/simulations/profile-diff-held-out/" + directory
        ):
            next_fd = _open_directory(current_fd, part, code)
            opened.append(next_fd)
            current_fd = next_fd
        return opened[-1]
    finally:
        for handle in reversed(opened[:-1]):
            os.close(handle)


def _verified_release(
    bundle_root_fd: int, proc_prefix: str, identity: ReleaseIdentity, code: str
) -> _VerifiedRelease:
    prefix = "repository/docs/simulations/profile-diff-held-out/" + identity.directory
    manifest_raw = _safe_regular(bundle_root_fd, prefix + "/public-manifest.json", code)
    receipt_raw = _safe_regular(bundle_root_fd, prefix + "/qualification-receipt.json", code)
    seal_raw = _safe_regular(bundle_root_fd, prefix + "/release-seal.json", code)
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
    bound: dict[str, bytes] = {
        "public-manifest.json": manifest_raw,
        "qualification-receipt.json": receipt_raw,
        "release-seal.json": seal_raw,
    }
    expected_paths = {"public-manifest.json", "release-seal.json"}
    total = 0
    for entry in files:
        if type(entry) is not dict:
            raise SealedDependencyFailure(code)
        path = entry.get("path")
        size = entry.get("size")
        digest = entry.get("sha256")
        if (
            type(path) is not str
            or Path(path).is_absolute()
            or ".." in Path(path).parts
            or type(size) is not int
            or size < 0
            or type(digest) is not str
            or path in expected_paths
        ):
            raise SealedDependencyFailure(code)
        content = _safe_regular(bundle_root_fd, prefix + "/" + path, code)
        if len(content) != size or sha256_bytes(content) != digest:
            raise SealedDependencyFailure(code)
        expected_paths.add(path)
        bound[path] = content
        total += size
    expected_paths.add("qualification-receipt.json")
    release_fd = _open_release_root(bundle_root_fd, identity.directory, code)
    try:
        actual_paths: set[str] = set()
        _tree_paths(release_fd, "", actual_paths, code)
    finally:
        os.close(release_fd)
    if actual_paths != expected_paths or manifest.get("total_bytes") != total:
        raise SealedDependencyFailure(code)
    return _VerifiedRelease(
        Path(proc_prefix) / prefix,
        MappingProxyType(bound),
    )


def _exec_bound_module(name: str, source: Path, content: bytes) -> ModuleType:
    module = ModuleType(name)
    module.__file__ = os.fspath(source)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[name] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except (ImportError, OSError, RuntimeError, SyntaxError, ValueError) as error:
        sys.modules.pop(name, None)
        raise SealedDependencyFailure("P86-WORKER-SEALED-IMPORT") from error
    return module


def _expect_signature(module: ModuleType, name: str, parameters: tuple[str, ...]) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure("P86-WORKER-SEALED-API")


def load_exact_p56(bundle_root_fd: int, proc_prefix: str) -> ModuleType:
    bound = _verified_release(bundle_root_fd, proc_prefix, P56, "P86-WORKER-SEALED-IDENTITY")
    module = _exec_bound_module("p75_worker_exact_p56", bound.root / P56.source, bound.files[P56.source])
    _expect_signature(
        module, "publish_retained_build_and_custody", ("platform", "runtime_parent")
    )
    _expect_signature(module, "launch_verified", ("handle", "platform", "arguments"))
    _expect_signature(module, "close_custody", ("handle",))
    if getattr(module, "DEFAULT_LAUNCH_USES", None) != 69:
        raise SealedDependencyFailure("P86-WORKER-SEALED-API")
    return module


__all__ = ["SealedDependencyFailure", "canonical_bytes", "load_exact_p56", "sha256_bytes"]
