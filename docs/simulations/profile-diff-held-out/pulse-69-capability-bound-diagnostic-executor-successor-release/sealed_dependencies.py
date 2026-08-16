"""Verify and import the complete immutable Pulse 57 stack for Pulse 69."""

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


sys.dont_write_bytecode = True


class SealedDependencyFailure(RuntimeError):
    """A predecessor release was not the exact sealed release."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


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


P57 = ReleaseIdentity(
    directory="pulse-57-capability-bound-diagnostic-executor-release",
    source="capability_bound_executor.py",
    source_sha256="sha256:bcb5eac2cd5aa0abd271dec2e93963ec855faa1c5ecbd628dfef61f52358c2c0",
    manifest_schema="ferris.pulse-57-capability-bound-diagnostic-executor-manifest/v1",
    manifest_raw_sha256="sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291",
    manifest_aggregate="sha256:ea0afe873e138cbe3ab9148ac0effd4b5defc94ebdf33ada4a6a3c0b468b1b46",
    manifest_file_count=12,
    release_tree_file_count=14,
    receipt_raw_sha256="sha256:8f0b35bd61bd147bbb43e898f0f817936b035688e3eb889d7530d8fc1b6a3a5d",
    receipt_payload_sha256="sha256:5cedec87b57e350d3ab11245c09b9cd7be1f485682d88cb9c1190a939f6bd134",
    seal_raw_sha256="sha256:b18407fd2def541486405d18e2dd92b9bb343e5e9aeaa2899f3ed4f312b68ea8",
    seal_payload_sha256="sha256:727e1806ede5ca9b5438b5f3b00bec3a59b4e36404ccc1b72e6adb661ebee144",
)


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
            raise SealedDependencyFailure("P69-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _safe_regular(path: Path) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise SealedDependencyFailure("P69-SEALED-TREE")
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P69-SEALED-TREE") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise SealedDependencyFailure("P69-SEALED-TREE")
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
        return bytes(content)
    except OSError as error:
        raise SealedDependencyFailure("P69-SEALED-TREE") from error
    finally:
        os.close(descriptor)


def _sealed_json(content: bytes) -> dict[str, object]:
    try:
        value = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, SealedDependencyFailure) as error:
        raise SealedDependencyFailure("P69-SEALED-IDENTITY") from error
    if type(value) is not dict:
        raise SealedDependencyFailure("P69-SEALED-IDENTITY")
    return value


def _release_root(repo_root: Path, identity: ReleaseIdentity) -> Path:
    try:
        root = repo_root.resolve(strict=True)
        release = (
            root
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / identity.directory
        )
        release.relative_to(root)
        metadata = os.lstat(release)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise SealedDependencyFailure("P69-SEALED-ROOT")
        return release
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P69-SEALED-ROOT") from error


def _tree_paths(root: Path, current: Path, values: set[str]) -> None:
    try:
        entries = list(os.scandir(current))
    except OSError as error:
        raise SealedDependencyFailure("P69-SEALED-TREE") from error
    for entry in entries:
        path = Path(entry.path)
        try:
            metadata = os.lstat(path)
        except OSError as error:
            raise SealedDependencyFailure("P69-SEALED-TREE") from error
        if stat.S_ISLNK(metadata.st_mode):
            raise SealedDependencyFailure("P69-SEALED-TREE")
        if stat.S_ISDIR(metadata.st_mode):
            _tree_paths(root, path, values)
        elif stat.S_ISREG(metadata.st_mode):
            values.add(path.relative_to(root).as_posix())
        else:
            raise SealedDependencyFailure("P69-SEALED-TREE")


def _aggregate(entries: list[object]) -> str:
    ordered = sorted(entries, key=lambda item: str(item["path"]).encode("utf-8"))  # type: ignore[index]
    digest = hashlib.sha256()
    for entry in ordered:
        assert type(entry) is dict
        path = entry.get("path")
        file_digest = entry.get("sha256")
        if (
            type(path) is not str
            or type(file_digest) is not str
            or not file_digest.startswith("sha256:")
            or len(file_digest) != 71
        ):
            raise SealedDependencyFailure("P69-SEALED-MANIFEST")
        try:
            raw_digest = bytes.fromhex(file_digest.removeprefix("sha256:"))
        except ValueError as error:
            raise SealedDependencyFailure("P69-SEALED-MANIFEST") from error
        encoded = path.encode("utf-8")
        digest.update(len(encoded).to_bytes(8, "big"))
        digest.update(encoded)
        digest.update(raw_digest)
    return "sha256:" + digest.hexdigest()


def _verified_release(repo_root: Path, identity: ReleaseIdentity) -> _VerifiedRelease:
    release = _release_root(repo_root, identity)
    bound = {
        "public-manifest.json": _safe_regular(release / "public-manifest.json"),
        "qualification-receipt.json": _safe_regular(release / "qualification-receipt.json"),
        "release-seal.json": _safe_regular(release / "release-seal.json"),
    }
    manifest_raw = bound["public-manifest.json"]
    receipt_raw = bound["qualification-receipt.json"]
    seal_raw = bound["release-seal.json"]
    if (
        sha256_bytes(manifest_raw) != identity.manifest_raw_sha256
        or sha256_bytes(receipt_raw) != identity.receipt_raw_sha256
        or sha256_bytes(seal_raw) != identity.seal_raw_sha256
    ):
        raise SealedDependencyFailure("P69-SEALED-IDENTITY")
    manifest = _sealed_json(manifest_raw)
    receipt = _sealed_json(receipt_raw)
    seal = _sealed_json(seal_raw)
    files = manifest.get("files")
    if (
        manifest.get("schema") != identity.manifest_schema
        or manifest.get("aggregate") != identity.manifest_aggregate
        or manifest.get("file_count") != identity.manifest_file_count
        or manifest.get("release_tree_file_count") != identity.release_tree_file_count
        or type(files) is not list
        or len(files) != identity.manifest_file_count
        or _aggregate(files) != identity.manifest_aggregate
        or receipt.get("payload_sha256") != identity.receipt_payload_sha256
        or receipt.get("receipt_id") != identity.receipt_payload_sha256
        or seal.get("payload_sha256") != identity.seal_payload_sha256
        or seal.get("seal_id") != identity.seal_payload_sha256
    ):
        raise SealedDependencyFailure("P69-SEALED-IDENTITY")
    expected_paths = {"public-manifest.json", "release-seal.json"}
    total = 0
    for entry in files:
        if type(entry) is not dict:
            raise SealedDependencyFailure("P69-SEALED-MANIFEST")
        path, size, digest = entry.get("path"), entry.get("size"), entry.get("sha256")
        if (
            type(path) is not str
            or Path(path).is_absolute()
            or ".." in Path(path).parts
            or type(size) is not int
            or size < 0
            or type(digest) is not str
            or path in expected_paths
        ):
            raise SealedDependencyFailure("P69-SEALED-MANIFEST")
        content = bound.get(path)
        if content is None:
            content = _safe_regular(release.joinpath(*path.split("/")))
        if len(content) != size or sha256_bytes(content) != digest:
            raise SealedDependencyFailure("P69-SEALED-IDENTITY")
        expected_paths.add(path)
        bound[path] = content
        total += size
    expected_paths.add("qualification-receipt.json")
    paths: set[str] = set()
    _tree_paths(release, release, paths)
    if paths != expected_paths or manifest.get("total_bytes") != total:
        raise SealedDependencyFailure("P69-SEALED-TREE")
    return _VerifiedRelease(release, MappingProxyType(bound))


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
        raise SealedDependencyFailure("P69-SEALED-IMPORT") from error
    return module


def _load_exact_p57(bound: _VerifiedRelease) -> tuple[ModuleType, ModuleType]:
    source = bound.files[P57.source]
    if sha256_bytes(source) != P57.source_sha256:
        raise SealedDependencyFailure("P69-SEALED-IDENTITY")
    dependencies = _exec_bound_module(
        "p69_bound_p57_sealed_dependencies",
        bound.root / "sealed_dependencies.py",
        bound.files["sealed_dependencies.py"],
    )
    saved: dict[str, ModuleType] = {}
    current = sys.modules.get("sealed_dependencies")
    if current is not None:
        saved["sealed_dependencies"] = current
    current = sys.modules.get("p69_exact_p57")
    if current is not None:
        saved["p69_exact_p57"] = current
    sys.modules["sealed_dependencies"] = dependencies
    try:
        module = _exec_bound_module("p69_exact_p57", bound.root / P57.source, source)
    finally:
        sys.modules.pop("sealed_dependencies", None)
        sys.modules.pop("p69_exact_p57", None)
        sys.modules.update(saved)
    return module, dependencies


def _expect_signature(module: ModuleType, name: str, parameters: tuple[str, ...]) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure("P69-SEALED-API")


def load_exact_p57_stack(repo_root: Path) -> tuple[ModuleType, ModuleType, ModuleType]:
    bound = _verified_release(repo_root, P57)
    module, dependencies = _load_exact_p57(bound)
    _expect_signature(
        module,
        "run_capability_bound_diagnostic_executor",
        (
            "repo_root",
            "descriptor_root",
            "private_runtime_root",
            "p27_cycle_root",
            "ubuntu_runtime_parent",
        ),
    )
    _expect_signature(
        module,
        "_run_qualification_executor",
        (
            "repo_root",
            "descriptor_root",
            "private_runtime_root",
            "p27_cycle_root",
            "controls",
        ),
    )
    for name in (
        "_close_handles",
        "_known_failure",
        "_copy_expected",
        "_frozen_descriptor",
        "_normalize_result",
        "_with_terminal_failure_types",
        "_execute",
        "_stage_wsl_bundle",
        "_native_wsl_parent",
        "_wsl_environment",
        "_wsl_executable",
        "_parse_line",
        "_canonical_line",
        "bound_release_files",
        "load_exact_p51",
        "load_exact_p56",
    ):
        if not callable(getattr(module, name, None)):
            raise SealedDependencyFailure("P69-SEALED-API")
    if getattr(module, "REQUEST_COUNT", None) != 69:
        raise SealedDependencyFailure("P69-SEALED-API")
    if getattr(module, "WSL_PLATFORM", None) != "ubuntu-24.04-x86_64":
        raise SealedDependencyFailure("P69-SEALED-API")
    if getattr(module, "WSL_SCHEMA", None) != "ferris.pulse-57-wsl-capability-session/v1":
        raise SealedDependencyFailure("P69-SEALED-API")
    p51 = module.load_exact_p51(repo_root)
    p56 = module.load_exact_p56(repo_root)
    if not callable(getattr(dependencies, "load_p51_synthetic_fixture", None)):
        raise SealedDependencyFailure("P69-SEALED-API")
    module._p69_bound_sealed_dependencies = dependencies
    return module, p51, p56


def load_p51_synthetic_fixture(repo_root: Path, p51: ModuleType) -> ModuleType:
    bound = _verified_release(repo_root, P57)
    _module, dependencies = _load_exact_p57(bound)
    loader = getattr(dependencies, "load_p51_synthetic_fixture", None)
    if not callable(loader):
        raise SealedDependencyFailure("P69-SEALED-API")
    fixture = loader(repo_root, p51)
    if not callable(getattr(fixture, "create_descriptor_root", None)):
        raise SealedDependencyFailure("P69-SEALED-API")
    return fixture


def release_identities() -> dict[str, dict[str, str]]:
    """Return only public sealed identities; never roots, receipts, or handles."""

    return {
        "pulse_57": {
            "manifest": P57.manifest_raw_sha256,
            "seal": P57.seal_payload_sha256,
            "source": P57.source_sha256,
        },
        "pulse_51": {
            "manifest": "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc",
            "seal": "sha256:1d22ad1248a2f47c78984d8020c3c6507253c468b53f30073efcfb5ab880c0d4",
        },
        "pulse_56": {
            "manifest": "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a",
            "seal": "sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747",
        },
    }
