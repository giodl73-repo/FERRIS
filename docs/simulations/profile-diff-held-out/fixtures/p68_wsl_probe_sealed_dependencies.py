"""Harmless route-equivalence helpers for the Pulse 68 WSL probe worker."""

from __future__ import annotations

import hashlib
import inspect
import json
import os
import stat
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from types import MappingProxyType, ModuleType
from typing import Callable, Mapping


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


P51 = ReleaseIdentity(
    directory="pulse-51-diagnostic-executor-release",
    source="diagnostic_executor.py",
    manifest_schema="ferris.pulse-51-diagnostic-executor-public-manifest/v1",
    manifest_raw_sha256="sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc",
    manifest_aggregate="sha256:18d61962245d75e42fed30f581555a5b436e0a83d89e3383d059dca035e978e6",
    manifest_file_count=14,
    release_tree_file_count=17,
    receipt_raw_sha256="sha256:ef2b423520e1f2680c0cadd246a51c0af1a4502f45d757f018982f42c326f1c9",
    receipt_payload_sha256="sha256:77408aabd377801c3c578a889523c18ee95eb286ac55b04df6c30f74d45ef452",
    seal_raw_sha256="sha256:968f495555b4617329318686b5adb460faf3fe95a07c8da160e163c9395eb767",
    seal_payload_sha256="sha256:1d22ad1248a2f47c78984d8020c3c6507253c468b53f30073efcfb5ab880c0d4",
)

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
            raise SealedDependencyFailure("P57-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _safe_regular(path: Path) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise SealedDependencyFailure("P57-SEALED-TREE")
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P57-SEALED-TREE") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise SealedDependencyFailure("P57-SEALED-TREE")
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
        return bytes(content)
    except OSError as error:
        raise SealedDependencyFailure("P57-SEALED-TREE") from error
    finally:
        os.close(descriptor)


def _sealed_json(content: bytes) -> dict[str, object]:
    try:
        value = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, SealedDependencyFailure) as error:
        raise SealedDependencyFailure("P57-SEALED-IDENTITY") from error
    if type(value) is not dict:
        raise SealedDependencyFailure("P57-SEALED-IDENTITY")
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
            raise SealedDependencyFailure("P57-SEALED-ROOT")
        return release
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P57-SEALED-ROOT") from error


def _tree_paths(root: Path, current: Path, values: set[str]) -> None:
    try:
        entries = list(os.scandir(current))
    except OSError as error:
        raise SealedDependencyFailure("P57-SEALED-TREE") from error
    for entry in entries:
        path = Path(entry.path)
        try:
            metadata = os.lstat(path)
        except OSError as error:
            raise SealedDependencyFailure("P57-SEALED-TREE") from error
        if stat.S_ISLNK(metadata.st_mode):
            raise SealedDependencyFailure("P57-SEALED-TREE")
        if stat.S_ISDIR(metadata.st_mode):
            _tree_paths(root, path, values)
        elif stat.S_ISREG(metadata.st_mode):
            values.add(path.relative_to(root).as_posix())
        else:
            raise SealedDependencyFailure("P57-SEALED-TREE")


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
            raise SealedDependencyFailure("P57-SEALED-MANIFEST")
        try:
            raw_digest = bytes.fromhex(file_digest.removeprefix("sha256:"))
        except ValueError as error:
            raise SealedDependencyFailure("P57-SEALED-MANIFEST") from error
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
        raise SealedDependencyFailure("P57-SEALED-IDENTITY")
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
        raise SealedDependencyFailure("P57-SEALED-IDENTITY")
    expected_paths = {"public-manifest.json", "release-seal.json"}
    total = 0
    for entry in files:
        if type(entry) is not dict:
            raise SealedDependencyFailure("P57-SEALED-MANIFEST")
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
            raise SealedDependencyFailure("P57-SEALED-MANIFEST")
        content = bound.get(path)
        if content is None:
            content = _safe_regular(release.joinpath(*path.split("/")))
        if len(content) != size or sha256_bytes(content) != digest:
            raise SealedDependencyFailure("P57-SEALED-IDENTITY")
        expected_paths.add(path)
        bound[path] = content
        total += size
    expected_paths.add("qualification-receipt.json")
    paths: set[str] = set()
    _tree_paths(release, release, paths)
    if paths != expected_paths or manifest.get("total_bytes") != total:
        raise SealedDependencyFailure("P57-SEALED-TREE")
    return _VerifiedRelease(release, MappingProxyType(bound))


def verify_release(repo_root: Path, identity: ReleaseIdentity) -> Path:
    """Verify every public release-tree file before it can be imported."""

    return _verified_release(repo_root, identity).root


def bound_release_files(repo_root: Path, identity: ReleaseIdentity) -> Mapping[str, bytes]:
    """Return complete verified release bytes for staging without a path reopen."""

    return _verified_release(repo_root, identity).files


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
        raise SealedDependencyFailure("P57-SEALED-IMPORT") from error
    return module


def _load_p51(bound: _VerifiedRelease) -> ModuleType:
    helpers = (
        ("frozen_profile_diff", "frozen_profile_diff.py"),
        ("p31_contract_verifier", "p31_contract_verifier.py"),
        ("p35_p37_custody", "p35_p37_custody.py"),
        ("sealed_dependencies", "sealed_dependencies.py"),
    )
    names = (*tuple(name for name, _ in helpers), "p57_exact_p51")
    saved = {name: sys.modules.pop(name) for name in names if name in sys.modules}
    try:
        for name, path in helpers:
            _exec_bound_module(name, bound.root / path, bound.files[path])
        module = _exec_bound_module(
            "p57_exact_p51", bound.root / P51.source, bound.files[P51.source]
        )
        module._p57_sealed_dependencies = sys.modules["sealed_dependencies"]
        _bind_p51_dependency_loaders(module)
        return module
    finally:
        for name in names:
            sys.modules.pop(name, None)
        sys.modules.update(saved)


def _bound_p51_dependency_call(p51: ModuleType, callback: Callable[[], object]) -> object:
    dependencies = p51._p57_sealed_dependencies
    original_read = dependencies._safe_regular
    original_load = dependencies._load_module
    contents: dict[str, bytes] = {}

    def read(path: Path, code: str) -> bytes:
        content = original_read(path, code)
        contents[os.fspath(path)] = content
        return content

    def load(name: str, source: Path) -> ModuleType:
        content = contents.get(os.fspath(source))
        if content is None:
            raise dependencies.DependencyFailure("P51-SEALED-IMPORT")
        try:
            return _exec_bound_module(name, source, content)
        except SealedDependencyFailure as error:
            raise dependencies.DependencyFailure("P51-SEALED-IMPORT") from error

    dependencies._safe_regular = read
    dependencies._load_module = load
    try:
        return callback()
    finally:
        dependencies._safe_regular = original_read
        dependencies._load_module = original_load


def _bind_p51_dependency_loaders(p51: ModuleType) -> None:
    original_terminal = p51.load_terminal_dependencies
    original_p27 = p51.load_p27_exact_runner

    def load_terminal_dependencies(repo_root: Path) -> tuple[ModuleType, ModuleType, ModuleType]:
        result = _bound_p51_dependency_call(p51, lambda: original_terminal(repo_root))
        assert type(result) is tuple
        return result  # type: ignore[return-value]

    def load_p27_exact_runner(repo_root: Path) -> Callable[[Path], dict[str, object]]:
        result = _bound_p51_dependency_call(p51, lambda: original_p27(repo_root))
        if not callable(result):
            raise p51.DependencyFailure("P51-P27-CALLABLE")
        return result

    p51.load_terminal_dependencies = load_terminal_dependencies
    p51.load_p27_exact_runner = load_p27_exact_runner


def _expect_signature(module: ModuleType, name: str, parameters: tuple[str, ...]) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure("P57-SEALED-API")


def load_exact_p51(repo_root: Path) -> ModuleType:
    bound = _verified_release(repo_root, P51)
    module = _load_p51(bound)
    _expect_signature(
        module,
        "run_diagnostic_executor",
        ("repo_root", "descriptor_root", "private_runtime_root", "p27_cycle_root", "retained_custodies"),
    )
    _expect_signature(module, "validate_descriptor_root", ("descriptor_root", "private_runtime_root"))
    _expect_signature(module, "profile_diff_argv", ("descriptor",))
    _expect_signature(module, "canonical_platform_id", ("value",))
    for name in (
        "_descriptor_semantics",
        "_json_normalized",
        "_human_normalized",
        "_safe_runtime_root",
        "_run_p27_once",
        "verify_bound_contract",
        "verify_p35_p37_custody",
        "load_p27_exact_runner",
        "load_terminal_dependencies",
    ):
        if not callable(getattr(module, name, None)):
            raise SealedDependencyFailure("P57-SEALED-API")
    return module


def load_exact_p56(repo_root: Path) -> ModuleType:
    bound = _verified_release(repo_root, P56)
    module = _exec_bound_module("p57_exact_p56", bound.root / P56.source, bound.files[P56.source])
    _expect_signature(
        module, "publish_retained_build_and_custody", ("platform", "runtime_parent")
    )
    _expect_signature(module, "launch_verified", ("handle", "platform", "arguments"))
    _expect_signature(module, "close_custody", ("handle",))
    if getattr(module, "DEFAULT_LAUNCH_USES", None) != 69:
        raise SealedDependencyFailure("P57-SEALED-API")
    return module


PROBE_SCHEMA = "ferris.pulse-68-wsl-probe-session/v1"
PROBE_RESULT_SCHEMA = "ferris.pulse-68-wsl-probe-result/v1"
PLATFORM = "ubuntu-24.04-x86_64"
PROBE_REQUEST_COUNT = 1
PRODUCTION_P57_WORKER_SOURCE = (
    "docs/simulations/profile-diff-held-out/"
    "pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py"
)
PRODUCTION_P57_WORKER_SHA256 = (
    "sha256:9b0d91f7c4e2aed57d7dc40b95f5860f017138717364d3399d132884047904cb"
)
PRODUCTION_P57_SEALED_DEPENDENCIES_SOURCE = (
    "docs/simulations/profile-diff-held-out/"
    "pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py"
)
PRODUCTION_P57_SEALED_DEPENDENCIES_SHA256 = (
    "sha256:fe36a56a10d5d3659fae9cfacc3cd48075aaf0e3327ae029a2470d1107da6c8d"
)
PROBE_VALIDATED_P56_CALLABLES = (
    "publish_retained_build_and_custody",
    "launch_verified",
    "close_custody",
)


class ReleaseFailure(RuntimeError):
    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass
class ProbeContext:
    bundle_root: Path
    p56_root: Path
    runtime_parent: Path
    probe_invocations: int = 0
    closed: bool = False


def _native_directory(path: Path, code: str) -> Path:
    try:
        resolved = path.resolve(strict=True)
        metadata = os.lstat(resolved)
        if (
            stat.S_ISLNK(metadata.st_mode)
            or not stat.S_ISDIR(metadata.st_mode)
            or not str(resolved).startswith("/")
            or str(resolved).startswith("/mnt/")
        ):
            raise ReleaseFailure(code)
        return resolved
    except ReleaseFailure:
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error


def _p57_bundle_name(name: str) -> bool:
    return (
        name.startswith(".p57-")
        and len(name) == len(".p57-") + 32
        and all(character in "0123456789abcdef" for character in name[len(".p57-"):])
    )


def bind_probe_context(bundle_root: Path, p56_root: Path, runtime_parent: Path) -> ProbeContext:
    bundle_root = _native_directory(bundle_root, "P68-PROBE-BUNDLE-ROOT")
    runtime_parent = _native_directory(runtime_parent, "P68-PROBE-RUNTIME-PARENT")
    p56_root = _native_directory(p56_root, "P68-PROBE-P56-ROOT")
    if bundle_root.parent != runtime_parent or not _p57_bundle_name(bundle_root.name):
        raise ReleaseFailure("P68-PROBE-BUNDLE-ROOT")
    expected_p56_root = (
        bundle_root
        / "repository"
        / "docs"
        / "simulations"
        / "profile-diff-held-out"
        / P56.directory
    )
    if p56_root != expected_p56_root:
        raise ReleaseFailure("P68-PROBE-P56-ROOT")
    return ProbeContext(bundle_root=bundle_root, p56_root=p56_root, runtime_parent=runtime_parent)


def launch_harmless_probe(
    context: ProbeContext, platform: str, arguments: tuple[str, ...] | list[str]
) -> subprocess.CompletedProcess[bytes]:
    if type(context) is not ProbeContext or context.closed or context.probe_invocations != 0:
        raise ReleaseFailure("P68-PROBE-STATE")
    if platform != PLATFORM:
        raise ReleaseFailure("P68-PROBE-PLATFORM")
    if (
        type(arguments) not in {tuple, list}
        or len(arguments) != 7
        or any(type(value) is not str or "\x00" in value for value in arguments)
    ):
        raise ReleaseFailure("P68-PROBE-REQUEST")
    context.probe_invocations += 1
    stdout = canonical_bytes(
        {
            "platform": PLATFORM,
            "probe": "p57-route-equivalence-with-exact-p56-loader-leg",
            "production_dependency_sha256": PRODUCTION_P57_SEALED_DEPENDENCIES_SHA256,
            "production_worker_sha256": PRODUCTION_P57_WORKER_SHA256,
            "request_argument_count": len(arguments),
            "schema": PROBE_RESULT_SCHEMA,
            "staged_p56_release_file_count": P56.release_tree_file_count,
            "type": "probe-result",
            "validated_p56_callable_count": len(PROBE_VALIDATED_P56_CALLABLES),
            "validated_p56_callable_names": list(PROBE_VALIDATED_P56_CALLABLES),
            "validated_p56_default_launch_uses": 69,
            "validated_p56_loader_route": "exact-load_exact_p56(repo_root)-plus-Path(p56.__file__).parent",
        }
    ) + b"\n"
    return subprocess.CompletedProcess(list(arguments), 0, stdout, b"")


def close_probe_context(context: ProbeContext) -> None:
    if type(context) is not ProbeContext or context.closed:
        raise ReleaseFailure("P68-PROBE-HANDLE")
    context.closed = True
