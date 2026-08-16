"""Byte-bound exact imports for Pulse 58's ordered capability executor."""
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


class SealedDependencyFailure(RuntimeError):
    """A Pulse 58 predecessor was not the exact sealed release."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


P39 = ReleaseIdentity(
    "pulse-39-checkout-verifier-release", "checkout_verifier.py",
    "ferris.pulse-39-checkout-verifier-public-manifest/v1",
    "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c",
    "sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c",
    5, 8,
    "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8",
    "sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546",
    "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c",
    "sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b",
)
P41 = ReleaseIdentity(
    "pulse-41-transactional-copy-release", "transactional_copy.py",
    "ferris.pulse-41-transactional-copy-public-manifest/v1",
    "sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8",
    "sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755",
    5, 8,
    "sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c",
    "sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f",
    "sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a",
    "sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf",
)
P52 = ReleaseIdentity(
    "pulse-52-ordered-materialization-executor-release", "ordered_materialization_executor.py",
    "ferris.pulse-52-ordered-materialization-executor-public-manifest/v1",
    "sha256:e585d6baaf83783ff1a1c65e1d3f281ce1d3afd9806f9cb9811b328eff9811da",
    "sha256:3da8401a52d020ead7b9c6854461da5f28dfb9d1117385cd6943592f74e8aaec",
    10, 12,
    "sha256:1eaf50c293e4c44f9312b28efa581912ed4165e8f77014c703cfc54496b37192",
    "sha256:183a7c6f0ebbab38bbe5b29efc4c1ebd3c5e1e8ca8ca84a5cc5d29107798a7ac",
    "sha256:febee1ea581a3564da89714aaeae1c909b0a9345676958bbb6e2fe4ec2d72ca6",
    "sha256:46d9e8bb1aa75780fb7397fd4833e13c5e28c0ec79254185ef6da793e4ed7f84",
)
P57 = ReleaseIdentity(
    "pulse-57-capability-bound-diagnostic-executor-release", "capability_bound_executor.py",
    "ferris.pulse-57-capability-bound-diagnostic-executor-manifest/v1",
    "sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291",
    "sha256:ea0afe873e138cbe3ab9148ac0effd4b5defc94ebdf33ada4a6a3c0b468b1b46",
    12, 14,
    "sha256:8f0b35bd61bd147bbb43e898f0f817936b035688e3eb889d7530d8fc1b6a3a5d",
    "sha256:5cedec87b57e350d3ab11245c09b9cd7be1f485682d88cb9c1190a939f6bd134",
    "sha256:b18407fd2def541486405d18e2dd92b9bb343e5e9aeaa2899f3ed4f312b68ea8",
    "sha256:727e1806ede5ca9b5438b5f3b00bec3a59b4e36404ccc1b72e6adb661ebee144",
)
P35_MANIFEST_RAW = "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1"
P35_MANIFEST_AGGREGATE = "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69"
P35_MATERIALIZER_HASHES = frozenset((
    "sha256:7f74a642ce27f5742e87870e4d39d375cfa9223a40f92d253916db81260db6ba",
    "sha256:f531028a10127e7bc5f989eeffee45f89ffcfbe74660b3aa9eb4e8913aa3f73a",
))
P35_VERIFIER_HASHES = frozenset((
    "sha256:352d35202c0bef1a2294daa21bc4f6151db8f86a1bc1a0465914474981c1e301",
    "sha256:911fb069627a0c0bf657d7af974271f50b827cab34f326f7e09bff8045815221",
))


def canonical_bytes(value: object) -> bytes:
    return json.dumps(value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise SealedDependencyFailure("P58-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _safe_regular(path: Path, code: str = "P58-SEALED-TREE", maximum: int = 4_194_304) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise SealedDependencyFailure(code)
        descriptor = os.open(path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0))
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (opened.st_dev, opened.st_ino):
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


def _release_root(repo_root: Path, directory: str) -> Path:
    try:
        root = repo_root.resolve(strict=True)
        if not root.is_absolute():
            raise SealedDependencyFailure("P58-SEALED-ROOT")
        release = root / "docs" / "simulations" / "profile-diff-held-out" / directory
        release.relative_to(root)
        metadata = os.lstat(release)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise SealedDependencyFailure("P58-SEALED-ROOT")
        return release
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P58-SEALED-ROOT") from error


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
        path, digest = entry.get("path"), entry.get("sha256")
        if type(path) is not str or not path or Path(path).is_absolute() or ".." in Path(path).parts or type(digest) is not str:
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


def _verified_release(repo_root: Path, identity: ReleaseIdentity) -> _VerifiedRelease:
    code = f"P58-{identity.directory.upper().replace('-', '-')}-IDENTITY"
    release = _release_root(repo_root, identity.directory)
    manifest_raw = _safe_regular(release / "public-manifest.json", code)
    receipt_raw = _safe_regular(release / "qualification-receipt.json", code)
    seal_raw = _safe_regular(release / "release-seal.json", code)
    if (sha256_bytes(manifest_raw) != identity.manifest_raw_sha256 or sha256_bytes(receipt_raw) != identity.receipt_raw_sha256 or sha256_bytes(seal_raw) != identity.seal_raw_sha256):
        raise SealedDependencyFailure(code)
    manifest = _sealed_json(manifest_raw, code)
    receipt = _sealed_json(receipt_raw, code)
    seal = _sealed_json(seal_raw, code)
    files = manifest.get("files")
    if (type(files) is not list or manifest.get("schema") != identity.manifest_schema or manifest.get("aggregate") != identity.manifest_aggregate or manifest.get("file_count") != identity.manifest_file_count or len(files) != identity.manifest_file_count or _aggregate(files, code) != identity.manifest_aggregate or receipt.get("payload_sha256") != identity.receipt_payload_sha256 or receipt.get("receipt_id") != identity.receipt_payload_sha256 or seal.get("payload_sha256") != identity.seal_payload_sha256 or (seal.get("seal_id") != identity.seal_payload_sha256 and seal.get("receipt_id") != identity.seal_payload_sha256)):
        raise SealedDependencyFailure(code)
    if (type(receipt.get("payload")) is not dict or sha256_bytes(canonical_bytes(receipt["payload"])) != identity.receipt_payload_sha256 or type(seal.get("payload")) is not dict or sha256_bytes(canonical_bytes(seal["payload"])) != identity.seal_payload_sha256):
        raise SealedDependencyFailure(code)
    bound: dict[str, bytes] = {"public-manifest.json": manifest_raw, "release-seal.json": seal_raw}
    expected = set(bound)
    total = 0
    for entry in files:
        assert type(entry) is dict
        path, size, digest = entry.get("path"), entry.get("size"), entry.get("sha256")
        if type(path) is not str or type(size) is not int or size < 0 or type(digest) is not str or path in expected:
            raise SealedDependencyFailure(code)
        content = receipt_raw if path == "qualification-receipt.json" else _safe_regular(release.joinpath(*path.split("/")), code)
        if len(content) != size or sha256_bytes(content) != digest:
            raise SealedDependencyFailure(code)
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
    if type(manifest.get("total_bytes")) is int and manifest["total_bytes"] != total:
        raise SealedDependencyFailure(code)
    if sha256_bytes(bound[identity.source]) != next((str(item["sha256"]) for item in files if type(item) is dict and item.get("path") == identity.source), ""):
        raise SealedDependencyFailure(code)
    return _VerifiedRelease(release, MappingProxyType(bound))


def _exec_bound_module(name: str, source: Path, content: bytes, code: str) -> ModuleType:
    module = ModuleType(name)
    module.__file__ = os.fspath(source)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[name] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except BaseException as error:
        sys.modules.pop(name, None)
        if isinstance(error, (ImportError, OSError, RuntimeError, SyntaxError, ValueError)):
            raise SealedDependencyFailure(code) from error
        raise
    return module


def _signature(module: ModuleType, name: str, parameters: tuple[str, ...], code: str) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure(code)


def _load_with_bound_dependencies(prefix: str, bound: _VerifiedRelease, source: str, code: str) -> ModuleType:
    dependency_path = "sealed_dependencies.py"
    if dependency_path not in bound.files:
        raise SealedDependencyFailure(code)
    dependencies = _exec_bound_module(f"{prefix}_dependencies", bound.root / dependency_path, bound.files[dependency_path], code)
    previous = sys.modules.get("sealed_dependencies")
    sys.modules["sealed_dependencies"] = dependencies
    try:
        return _exec_bound_module(prefix, bound.root / source, bound.files[source], code)
    finally:
        if previous is None:
            sys.modules.pop("sealed_dependencies", None)
        else:
            sys.modules["sealed_dependencies"] = previous


def load_exact_p39_and_p41(repo_root: Path) -> tuple[ModuleType, ModuleType]:
    p39 = _verified_release(repo_root, P39)
    p41 = _verified_release(repo_root, P41)
    p39_module = _exec_bound_module("p58_exact_p39", p39.root / P39.source, p39.files[P39.source], "P58-P39-IMPORT")
    p41_module = _exec_bound_module("p58_exact_p41", p41.root / P41.source, p41.files[P41.source], "P58-P41-IMPORT")
    for name, parameters in (("verify", ("checkout_root_value", "pulse_25_root", "pulse_27_root", "git")), ("resolve_checkout_root", ("value",)), ("enumerate_release_paths", ("checkout_root", "release_roots"))):
        _signature(p39_module, name, parameters, "P58-P39-API")
    if getattr(p39_module, "EXPECTED_CARDINALITY", None) != 36 or len(getattr(p39_module, "EXPECTED_PATHS", ())) != 36 or not isinstance(getattr(p39_module, "PublicFailure", None), type):
        raise SealedDependencyFailure("P58-P39-API")
    _signature(p41_module, "copy_release", ("source_root_value", "final_root_value", "synchronizer", "copier", "renamer", "remover", "post_rename"), "P58-P41-API")
    _signature(p41_module, "verify_bound_tree", ("root", "phase"), "P58-P41-API")
    _signature(p41_module, "_safe_absolute", ("value",), "P58-P41-API")
    if getattr(p41_module, "EXPECTED_COUNT", None) != 8 or len(getattr(p41_module, "EXPECTED_PATHS", ())) != 8 or not isinstance(getattr(p41_module, "PublicFailure", None), type):
        raise SealedDependencyFailure("P58-P41-API")
    return p39_module, p41_module


def load_exact_p52_stage_reader(repo_root: Path) -> ModuleType:
    bound = _verified_release(repo_root, P52)
    module = _load_with_bound_dependencies("p58_exact_p52", bound, P52.source, "P58-P52-IMPORT")
    _signature(module, "_validate_materialization_summary", ("p51", "summary"), "P58-P52-API")
    _signature(module, "_validate_verification_summary", ("p51", "summary"), "P58-P52-API")
    _signature(module, "_validate_materialized_descriptor_root", ("p51", "descriptor_root", "runtime_root"), "P58-P52-API")
    _signature(module, "_verify_public_prelaunch_custody", ("p51", "p39", "p41", "runtime_root", "p39_checkout_root", "p41_final_root", "private_record"), "P58-P52-API")
    _signature(module, "_remove_private_tree", ("p51", "path", "code"), "P58-P52-API")
    _signature(module, "_remove_seed", ("p51", "seed_path", "private_record"), "P58-P52-API")
    return module


def load_exact_p57(repo_root: Path) -> ModuleType:
    bound = _verified_release(repo_root, P57)
    module = _load_with_bound_dependencies("p58_exact_p57", bound, P57.source, "P58-P57-IMPORT")
    _signature(module, "run_capability_bound_diagnostic_executor", ("repo_root", "descriptor_root", "private_runtime_root", "p27_cycle_root", "ubuntu_runtime_parent"), "P58-P57-API")
    for name, parameters in (("_freeze_descriptors", ("p51", "descriptor_root", "descriptors", "runtime_root")), ("_prelaunch_dispatch", ("p51", "frozen", "runtime_root")), ("_normalize_result", ("p51", "frozen", "capture")), ("_close_handles", ("controls", "windows_handle", "windows_launches", "wsl")), ("_known_failure", ("error", "controls")), ("_with_terminal_failure_types", ("controls", "modules"))):
        _signature(module, name, parameters, "P58-P57-API")
    if not isinstance(getattr(module, "_Controls", None), type) or not isinstance(getattr(module, "_NativeWslSession", None), type) or not isinstance(getattr(module, "ExecutorFailure", None), type) or getattr(module, "REQUEST_COUNT", None) != 69:
        raise SealedDependencyFailure("P58-P57-API")
    return module


def load_exact_p35_materializer_and_verifier(repo_root: Path) -> tuple[ModuleType, ModuleType]:
    release = _release_root(repo_root, "pulse-35-corpus-materializer-release")
    manifest_raw = _safe_regular(release / "public-manifest.json", "P58-P35-IDENTITY")
    materializer_raw = _safe_regular(release / "corpus_materializer.py", "P58-P35-IDENTITY", 131_072)
    verifier_raw = _safe_regular(release / "verify_materialization.py", "P58-P35-IDENTITY", 131_072)
    manifest = _sealed_json(manifest_raw, "P58-P35-IDENTITY")
    if (sha256_bytes(manifest_raw) != P35_MANIFEST_RAW or sha256_bytes(materializer_raw) not in P35_MATERIALIZER_HASHES or sha256_bytes(verifier_raw) not in P35_VERIFIER_HASHES or manifest.get("schema") != "ferris.pulse-35-public-corpus-materializer-manifest/v1" or manifest.get("aggregate") != P35_MANIFEST_AGGREGATE or manifest.get("file_count") != 8 or manifest.get("total_bytes") != 403316):
        raise SealedDependencyFailure("P58-P35-IDENTITY")
    materializer = _exec_bound_module("p58_exact_p35_materializer", release / "corpus_materializer.py", materializer_raw, "P58-P35-IMPORT")
    verifier = _exec_bound_module("p58_exact_p35_verifier", release / "verify_materialization.py", verifier_raw, "P58-P35-IMPORT")
    for module in (materializer, verifier):
        if getattr(module, "REQUIRED_CASE_COUNT", None) != 70 or getattr(module, "MAX_LOGICAL_CASES", None) != 512 or getattr(module, "DERIVATION", None) != "hmac-sha256-seed-key-domain-purpose-counter-v1" or not isinstance(getattr(module, "MaterializationError", None), type):
            raise SealedDependencyFailure("P58-P35-API")
    _signature(materializer, "materialize", ("seed_path", "output", "case_count"), "P58-P35-API")
    _signature(materializer, "seed_commitment", ("seed",), "P58-P35-API")
    _signature(verifier, "verify", ("output", "seed_path"), "P58-P35-API")
    return materializer, verifier


def load_exact_p57_stack(repo_root: Path) -> tuple[ModuleType, ModuleType, ModuleType]:
    """Return exact P57 plus its own byte-bound P51/P56 modules."""
    p57 = load_exact_p57(repo_root)
    try:
        p51 = p57.load_exact_p51(repo_root)
        p56 = p57.load_exact_p56(repo_root)
    except SealedDependencyFailure:
        raise
    except BaseException as error:
        raise SealedDependencyFailure("P58-P57-STACK") from error
    return p57, p51, p56


def release_identities() -> dict[str, dict[str, str]]:
    return {
        "pulse_35": {"manifest": P35_MANIFEST_RAW, "aggregate": P35_MANIFEST_AGGREGATE},
        "pulse_39": {"manifest": P39.manifest_raw_sha256, "seal": P39.seal_payload_sha256},
        "pulse_41": {"manifest": P41.manifest_raw_sha256, "seal": P41.seal_payload_sha256},
        "pulse_52": {"manifest": P52.manifest_raw_sha256, "seal": P52.seal_payload_sha256},
        "pulse_56": {
            "manifest": "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a",
            "seal": "sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747",
        },
        "pulse_57": {"manifest": P57.manifest_raw_sha256, "seal": P57.seal_payload_sha256},
    }
