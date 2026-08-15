"""Exact, fail-closed imports for the Pulse 52 ordered executor.

This module verifies complete sealed Pulse 39, Pulse 41, and Pulse 51 trees
before importing their implementations. Pulse 35 source is loaded only after
Pulse 51's gate-six P35/P37 custody check has passed.
"""

from __future__ import annotations

import hashlib
import importlib.util
import inspect
import json
import os
import stat
import sys
from pathlib import Path
from types import ModuleType


PULSE51_DIRECTORY = "pulse-51-diagnostic-executor-release"
PULSE51_MANIFEST_RAW_SHA256 = (
    "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc"
)
PULSE51_MANIFEST_AGGREGATE = (
    "sha256:18d61962245d75e42fed30f581555a5b436e0a83d89e3383d059dca035e978e6"
)
PULSE51_RECEIPT_RAW_SHA256 = (
    "sha256:ef2b423520e1f2680c0cadd246a51c0af1a4502f45d757f018982f42c326f1c9"
)
PULSE51_RECEIPT_PAYLOAD_SHA256 = (
    "sha256:77408aabd377801c3c578a889523c18ee95eb286ac55b04df6c30f74d45ef452"
)
PULSE51_SEAL_RAW_SHA256 = (
    "sha256:968f495555b4617329318686b5adb460faf3fe95a07c8da160e163c9395eb767"
)
PULSE51_SEAL_PAYLOAD_SHA256 = (
    "sha256:1d22ad1248a2f47c78984d8020c3c6507253c468b53f30073efcfb5ab880c0d4"
)
PULSE51_SOURCE_SHA256 = (
    "sha256:97c404dbf29d387561878772403c7fbd2672e97283b0620e838e7126ecbdd637"
)

PULSE39_DIRECTORY = "pulse-39-checkout-verifier-release"
PULSE39_RELEASE_ROOT = (
    "docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release"
)
PULSE39_MANIFEST_RAW_SHA256 = (
    "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c"
)
PULSE39_MANIFEST_AGGREGATE = (
    "sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c"
)
PULSE39_RECEIPT_RAW_SHA256 = (
    "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8"
)
PULSE39_RECEIPT_PAYLOAD_SHA256 = (
    "sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546"
)
PULSE39_SEAL_RAW_SHA256 = (
    "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c"
)
PULSE39_SEAL_PAYLOAD_SHA256 = (
    "sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b"
)
PULSE39_SOURCE_SHA256 = (
    "sha256:783283fd127170460ce52106a7a1158054cdc2608475e53899ff45a7a6a31d12"
)

PULSE41_DIRECTORY = "pulse-41-transactional-copy-release"
PULSE41_MANIFEST_RAW_SHA256 = (
    "sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8"
)
PULSE41_MANIFEST_AGGREGATE = (
    "sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755"
)
PULSE41_RECEIPT_RAW_SHA256 = (
    "sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c"
)
PULSE41_RECEIPT_PAYLOAD_SHA256 = (
    "sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f"
)
PULSE41_SEAL_RAW_SHA256 = (
    "sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a"
)
PULSE41_SEAL_PAYLOAD_SHA256 = (
    "sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf"
)
PULSE41_SOURCE_SHA256 = (
    "sha256:900a89de3401f78558970d896214568f851ca644def28639476e66154235c8cf"
)

PULSE35_DIRECTORY = "pulse-35-corpus-materializer-release"
PULSE35_MANIFEST_RAW_SHA256 = (
    "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1"
)
PULSE35_MANIFEST_AGGREGATE = (
    "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69"
)
PULSE35_MATERIALIZER_SHA256 = (
    "sha256:7f74a642ce27f5742e87870e4d39d375cfa9223a40f92d253916db81260db6ba"
)
PULSE35_VERIFIER_SHA256 = (
    "sha256:352d35202c0bef1a2294daa21bc4f6151db8f86a1bc1a0465914474981c1e301"
)
PULSE35_MATERIALIZER_RAW_VARIANTS = frozenset(
    {
        PULSE35_MATERIALIZER_SHA256,
        "sha256:f531028a10127e7bc5f989eeffee45f89ffcfbe74660b3aa9eb4e8913aa3f73a",
    }
)
PULSE35_VERIFIER_RAW_VARIANTS = frozenset(
    {
        PULSE35_VERIFIER_SHA256,
        "sha256:911fb069627a0c0bf657d7af974271f50b827cab34f326f7e09bff8045815221",
    }
)

P50_GATE_IDS = (
    "pulse-41-pulse-39-public-custody",
    "windows-retained-binary-custody",
    "ubuntu-retained-binary-custody",
    "exact-adapter-preflight",
    "pulse-31-public-input",
    "pulse-35-pulse-37-normalization",
    "bounded-materialization",
    "bounded-process-exit-search",
)
CANONICAL_PLATFORMS = ("windows-x86_64", "ubuntu-24.04-x86_64")
_PULSE51_MODULE: ModuleType | None = None
_PULSE39_MODULE: ModuleType | None = None
_PULSE41_MODULE: ModuleType | None = None


class SealedDependencyFailure(RuntimeError):
    """An exact predecessor tree, source, or callable binding failed."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def _digest(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise SealedDependencyFailure("P52-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _safe_regular(path: Path, code: str, maximum: int = 4_194_304) -> bytes:
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
        if not stat.S_ISREG(opened.st_mode) or (opened.st_dev, opened.st_ino) != (
            initial.st_dev,
            initial.st_ino,
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


def _sealed_json(value: bytes, code: str) -> dict[str, object]:
    try:
        parsed = json.loads(value, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, SealedDependencyFailure) as error:
        raise SealedDependencyFailure(code) from error
    if type(parsed) is not dict:
        raise SealedDependencyFailure(code)
    return parsed


def _safe_root(repo_root: Path) -> Path:
    try:
        if not repo_root.is_absolute():
            raise SealedDependencyFailure("P52-SEALED-ROOT")
        resolved = repo_root.resolve(strict=True)
        probe = resolved
        while True:
            metadata = os.lstat(probe)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise SealedDependencyFailure("P52-SEALED-ROOT")
            if probe == probe.parent:
                return resolved
            probe = probe.parent
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P52-SEALED-ROOT") from error


def _release_root(repo_root: Path, directory: str) -> Path:
    root = _safe_root(repo_root)
    release = root / "docs" / "simulations" / "profile-diff-held-out" / directory
    try:
        release.relative_to(root)
    except ValueError as error:
        raise SealedDependencyFailure("P52-SEALED-ROOT") from error
    try:
        metadata = os.lstat(release)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise SealedDependencyFailure("P52-SEALED-ROOT")
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P52-SEALED-ROOT") from error
    return release


def _tree_paths(root: Path, code: str) -> set[str]:
    paths: set[str] = set()

    def visit(directory: Path) -> None:
        try:
            with os.scandir(directory) as entries:
                ordered = sorted(entries, key=lambda entry: entry.name)
        except OSError as error:
            raise SealedDependencyFailure(code) from error
        for entry in ordered:
            path = Path(entry.path)
            try:
                metadata = os.lstat(path)
                if stat.S_ISLNK(metadata.st_mode):
                    raise SealedDependencyFailure(code)
                relative = path.relative_to(root).as_posix()
                if stat.S_ISDIR(metadata.st_mode):
                    visit(path)
                elif stat.S_ISREG(metadata.st_mode):
                    paths.add(relative)
                else:
                    raise SealedDependencyFailure(code)
            except SealedDependencyFailure:
                raise
            except (OSError, ValueError) as error:
                raise SealedDependencyFailure(code) from error

    visit(root)
    return paths


def _manifest_paths(manifest: dict[str, object], code: str) -> dict[str, dict[str, object]]:
    files = manifest.get("files")
    if type(files) is not list:
        raise SealedDependencyFailure(code)
    result: dict[str, dict[str, object]] = {}
    for entry in files:
        if type(entry) is not dict or set(entry) != {"kind", "path", "sha256", "size"}:
            raise SealedDependencyFailure(code)
        path = entry["path"]
        digest = entry["sha256"]
        size = entry["size"]
        if (
            type(path) is not str
            or not path
            or Path(path).is_absolute()
            or "\\" in path
            or ".." in Path(path).parts
            or type(digest) is not str
            or not digest.startswith("sha256:")
            or type(size) is not int
            or size < 0
            or path in result
        ):
            raise SealedDependencyFailure(code)
        result[path] = entry
    return result


def _aggregate(entries: dict[str, dict[str, object]]) -> str:
    hasher = hashlib.sha256()
    for path in sorted(entries, key=lambda item: item.encode("utf-8")):
        digest = str(entries[path]["sha256"]).removeprefix("sha256:")
        try:
            digest_bytes = bytes.fromhex(digest)
        except ValueError as error:
            raise SealedDependencyFailure("P52-SEALED-MANIFEST") from error
        if len(digest_bytes) != 32:
            raise SealedDependencyFailure("P52-SEALED-MANIFEST")
        encoded = path.encode("utf-8")
        hasher.update(len(encoded).to_bytes(8, "big"))
        hasher.update(encoded)
        hasher.update(digest_bytes)
    return "sha256:" + hasher.hexdigest()


def _verify_manifest_tree(
    release: Path,
    manifest: dict[str, object],
    *,
    aggregate: str,
    expected_file_count: int,
    expected_tree_count: int,
    schema: str,
    code: str,
    tree_count_field: str | None = "release_tree_file_count",
) -> None:
    entries = _manifest_paths(manifest, code)
    if (
        manifest.get("schema") != schema
        or manifest.get("aggregate") != aggregate
        or manifest.get("file_count") != expected_file_count
        or len(entries) != expected_file_count
        or _aggregate(entries) != aggregate
    ):
        raise SealedDependencyFailure(code)
    if tree_count_field is not None and manifest.get(tree_count_field) != expected_tree_count:
        raise SealedDependencyFailure(code)
    expected_tree = set(entries) | {
        "public-manifest.json",
        "qualification-receipt.json",
        "release-seal.json",
    }
    if len(expected_tree) != expected_tree_count or _tree_paths(release, code) != expected_tree:
        raise SealedDependencyFailure(code)
    for path, entry in entries.items():
        raw = _safe_regular(release / path, code)
        if len(raw) != entry["size"] or _digest(raw) != entry["sha256"]:
            raise SealedDependencyFailure(code)


def _signature(
    module: ModuleType,
    name: str,
    parameters: tuple[str, ...],
    code: str = "P52-P51-CALLABLE",
) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure(code)


def _import_pulse51(release: Path) -> ModuleType:
    imported_names = (
        "frozen_profile_diff",
        "p31_contract_verifier",
        "p35_p37_custody",
        "sealed_dependencies",
    )
    prior_modules = {name: sys.modules.get(name) for name in imported_names}
    for name in imported_names:
        sys.modules.pop(name, None)
    sys.path.insert(0, str(release))
    name = "pulse52_exact_pulse51_diagnostic_executor"
    sys.modules.pop(name, None)
    try:
        specification = importlib.util.spec_from_file_location(name, release / "diagnostic_executor.py")
        if specification is None or specification.loader is None:
            raise SealedDependencyFailure("P52-P51-IMPORT")
        module = importlib.util.module_from_spec(specification)
        sys.modules[name] = module
        specification.loader.exec_module(module)
        return module
    except SealedDependencyFailure:
        raise
    except (ImportError, OSError) as error:
        raise SealedDependencyFailure("P52-P51-IMPORT") from error
    finally:
        sys.path.pop(0)
        for imported_name, prior in prior_modules.items():
            if prior is None:
                sys.modules.pop(imported_name, None)
            else:
                sys.modules[imported_name] = prior


def load_pulse51(repo_root: Path) -> ModuleType:
    """Return Pulse 51 only after full-tree, source, and callable binding."""

    release = _release_root(repo_root, PULSE51_DIRECTORY)
    manifest_raw = _safe_regular(release / "public-manifest.json", "P52-P51-IDENTITY")
    receipt_raw = _safe_regular(
        release / "qualification-receipt.json", "P52-P51-IDENTITY", maximum=1_048_576
    )
    seal_raw = _safe_regular(release / "release-seal.json", "P52-P51-IDENTITY")
    source_raw = _safe_regular(release / "diagnostic_executor.py", "P52-P51-IDENTITY")
    if (
        _digest(manifest_raw) != PULSE51_MANIFEST_RAW_SHA256
        or _digest(receipt_raw) != PULSE51_RECEIPT_RAW_SHA256
        or _digest(seal_raw) != PULSE51_SEAL_RAW_SHA256
        or _digest(source_raw) != PULSE51_SOURCE_SHA256
    ):
        raise SealedDependencyFailure("P52-P51-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P52-P51-IDENTITY")
    receipt = _sealed_json(receipt_raw, "P52-P51-IDENTITY")
    seal = _sealed_json(seal_raw, "P52-P51-IDENTITY")
    _verify_manifest_tree(
        release,
        manifest,
        aggregate=PULSE51_MANIFEST_AGGREGATE,
        expected_file_count=14,
        expected_tree_count=17,
        schema="ferris.pulse-51-diagnostic-executor-public-manifest/v1",
        code="P52-P51-IDENTITY",
    )
    if (
        receipt.get("schema") != "ferris.pulse-51-diagnostic-executor-qualification-envelope/v2"
        or receipt.get("payload_sha256") != PULSE51_RECEIPT_PAYLOAD_SHA256
        or receipt.get("receipt_id") != PULSE51_RECEIPT_PAYLOAD_SHA256
        or seal.get("schema") != "ferris.pulse-51-diagnostic-executor-release-seal/v2"
        or seal.get("payload_sha256") != PULSE51_SEAL_PAYLOAD_SHA256
        or seal.get("seal_id") != PULSE51_SEAL_PAYLOAD_SHA256
    ):
        raise SealedDependencyFailure("P52-P51-IDENTITY")
    global _PULSE51_MODULE
    module = _PULSE51_MODULE or _import_pulse51(release)
    if (
        getattr(module, "P50_GATE_IDS", None) != P50_GATE_IDS
        or getattr(module, "CANONICAL_PLATFORMS", None) != CANONICAL_PLATFORMS
        or getattr(module, "P33_CUTOFF", None)
        != "29517d732db13cc2ffa304684b344f3538ab587d"
        or tuple(getattr(module, "__all__", ()))
        != (
            "ExecutorFailure",
            "ExecutorResult",
            "P44CustodyBinding",
            "TerminalPulse47Once",
            "canonical_platform_id",
            "invoke_terminal_pulse47_once",
            "run_diagnostic_executor",
        )
    ):
        raise SealedDependencyFailure("P52-P51-CALLABLE")
    for name, parameters in (
        ("run_diagnostic_executor", ("repo_root", "descriptor_root", "private_runtime_root", "p27_cycle_root", "retained_custodies")),
        ("_safe_runtime_root", ("path",)),
        ("_runtime_path", ("runtime_root", "value", "code", "require_regular", "require_directory", "allow_absent_leaf")),
        ("_normalize_custodies", ("custodies",)),
        ("_verify_custody_binary", ("custody", "expectation", "runtime_root")),
        ("_bridge_p44_once", ("p45", "repo_root", "custody", "platform", "runtime_root")),
        ("_run_p27_once", ("runtime_root", "cycle_root_value", "runner")),
        ("validate_descriptor_root", ("descriptor_root", "private_runtime_root")),
        ("_run_descriptor", ("descriptor", "platform", "executable", "runtime_root", "process_runner")),
        ("_execution_event", ("gate_id", "kind", "outcome")),
        ("_validation_event", ("validation_id", "checks")),
        ("_safe_regular_bytes", ("path", "code", "maximum")),
        ("_read_json", ("path", "code", "maximum")),
        ("_expected_result", ("value",)),
        ("_role_path", ("root", "role", "is_final")),
        ("_artifact_aggregate", ("root",)),
        ("_semantic_profile", ("path",)),
        ("load_terminal_dependencies", ("repo_root",)),
        ("load_p27_exact_runner", ("repo_root",)),
        ("verify_bound_contract", ("repo_root",)),
        ("verify_p35_p37_custody", ("repo_root", "git")),
        ("invoke_terminal_pulse47_once", ("terminal", "result", "p43_final_root", "witness_final_root")),
    ):
        _signature(module, name, parameters, "P52-P51-CALLABLE")
    if not isinstance(getattr(module, "P44CustodyBinding", None), type) or not isinstance(
        getattr(module, "TerminalPulse47Once", None), type
    ):
        raise SealedDependencyFailure("P52-P51-CALLABLE")
    _PULSE51_MODULE = module
    return module


def _validate_envelope(
    value: dict[str, object],
    *,
    schema: str,
    payload_sha256: str,
    identifier: str,
    code: str,
) -> None:
    if (
        set(value) != {"payload", "payload_sha256", identifier, "schema"}
        or type(value.get("payload")) is not dict
        or value.get("schema") != schema
        or value.get("payload_sha256") != payload_sha256
        or value.get(identifier) != payload_sha256
    ):
        raise SealedDependencyFailure(code)


def _import_release_module(name: str, source: Path, code: str) -> ModuleType:
    specification = importlib.util.spec_from_file_location(name, source)
    if specification is None or specification.loader is None:
        raise SealedDependencyFailure(code)
    module = importlib.util.module_from_spec(specification)
    sys.modules.pop(name, None)
    sys.modules[name] = module
    try:
        specification.loader.exec_module(module)
    except (ImportError, OSError, SyntaxError) as error:
        sys.modules.pop(name, None)
        raise SealedDependencyFailure(code) from error
    return module


def _load_pulse39(repo_root: Path) -> ModuleType:
    release = _release_root(repo_root, PULSE39_DIRECTORY)
    manifest_raw = _safe_regular(release / "public-manifest.json", "P52-P39-IDENTITY")
    receipt_raw = _safe_regular(
        release / "qualification-receipt.json", "P52-P39-IDENTITY", maximum=1_048_576
    )
    seal_raw = _safe_regular(release / "release-seal.json", "P52-P39-IDENTITY")
    source_raw = _safe_regular(release / "checkout_verifier.py", "P52-P39-IDENTITY")
    if (
        _digest(manifest_raw) != PULSE39_MANIFEST_RAW_SHA256
        or _digest(receipt_raw) != PULSE39_RECEIPT_RAW_SHA256
        or _digest(seal_raw) != PULSE39_SEAL_RAW_SHA256
        or _digest(source_raw) != PULSE39_SOURCE_SHA256
    ):
        raise SealedDependencyFailure("P52-P39-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P52-P39-IDENTITY")
    receipt = _sealed_json(receipt_raw, "P52-P39-IDENTITY")
    seal = _sealed_json(seal_raw, "P52-P39-IDENTITY")
    _verify_manifest_tree(
        release,
        manifest,
        aggregate=PULSE39_MANIFEST_AGGREGATE,
        expected_file_count=5,
        expected_tree_count=8,
        schema="ferris.pulse-39-checkout-verifier-public-manifest/v1",
        code="P52-P39-IDENTITY",
        tree_count_field=None,
    )
    receipt_payload = receipt.get("payload")
    seal_payload = seal.get("payload")
    if (
        manifest.get("release_root") != PULSE39_RELEASE_ROOT
        or manifest.get("total_bytes") != 26_455
        or type(receipt_payload) is not dict
        or type(seal_payload) is not dict
        or receipt_payload.get("pulse") != 39
        or seal_payload.get("pulse") != 39
    ):
        raise SealedDependencyFailure("P52-P39-IDENTITY")
    _validate_envelope(
        receipt,
        schema="ferris.pulse-39-checkout-verifier-qualification-envelope/v1",
        payload_sha256=PULSE39_RECEIPT_PAYLOAD_SHA256,
        identifier="receipt_id",
        code="P52-P39-IDENTITY",
    )
    _validate_envelope(
        seal,
        schema="ferris.pulse-39-checkout-verifier-release-seal-envelope/v1",
        payload_sha256=PULSE39_SEAL_PAYLOAD_SHA256,
        identifier="seal_id",
        code="P52-P39-IDENTITY",
    )
    global _PULSE39_MODULE
    module = _PULSE39_MODULE or _import_release_module(
        "pulse52_exact_p39_checkout_verifier",
        release / "checkout_verifier.py",
        "P52-P39-IMPORT",
    )
    if (
        getattr(module, "PULSE_25_ROOT", None)
        != "docs/simulations/profile-diff-held-out/pulse-25-collector-source-release"
        or getattr(module, "PULSE_27_ROOT", None)
        != "docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release"
        or getattr(module, "EXPECTED_CARDINALITY", None) != 36
        or tuple(getattr(module, "EXPECTED_PATHS", ())) != tuple(sorted(module.EXPECTED_PATHS))
        or len(getattr(module, "EXPECTED_PATHS", ())) != 36
        or not isinstance(getattr(module, "PublicFailure", None), type)
    ):
        raise SealedDependencyFailure("P52-P39-CALLABLE")
    for name, parameters in (
        ("verify", ("checkout_root_value", "pulse_25_root", "pulse_27_root", "git")),
        ("resolve_checkout_root", ("value",)),
        ("enumerate_release_paths", ("checkout_root", "release_roots")),
    ):
        _signature(module, name, parameters, "P52-P39-CALLABLE")
    _PULSE39_MODULE = module
    return module


def _load_pulse41(repo_root: Path) -> ModuleType:
    release = _release_root(repo_root, PULSE41_DIRECTORY)
    manifest_raw = _safe_regular(release / "public-manifest.json", "P52-P41-IDENTITY")
    receipt_raw = _safe_regular(
        release / "qualification-receipt.json", "P52-P41-IDENTITY", maximum=1_048_576
    )
    seal_raw = _safe_regular(release / "release-seal.json", "P52-P41-IDENTITY")
    source_raw = _safe_regular(release / "transactional_copy.py", "P52-P41-IDENTITY")
    if (
        _digest(manifest_raw) != PULSE41_MANIFEST_RAW_SHA256
        or _digest(receipt_raw) != PULSE41_RECEIPT_RAW_SHA256
        or _digest(seal_raw) != PULSE41_SEAL_RAW_SHA256
        or _digest(source_raw) != PULSE41_SOURCE_SHA256
    ):
        raise SealedDependencyFailure("P52-P41-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P52-P41-IDENTITY")
    receipt = _sealed_json(receipt_raw, "P52-P41-IDENTITY")
    seal = _sealed_json(seal_raw, "P52-P41-IDENTITY")
    _verify_manifest_tree(
        release,
        manifest,
        aggregate=PULSE41_MANIFEST_AGGREGATE,
        expected_file_count=5,
        expected_tree_count=8,
        schema="ferris.pulse-41-transactional-copy-public-manifest/v1",
        code="P52-P41-IDENTITY",
    )
    receipt_payload = receipt.get("payload")
    seal_payload = seal.get("payload")
    if (
        manifest.get("manifest_payload_file_count") != 5
        or manifest.get("total_bytes") != 49_120
        or type(receipt_payload) is not dict
        or type(seal_payload) is not dict
        or receipt_payload.get("pulse") != 41
        or seal_payload.get("pulse") != 41
    ):
        raise SealedDependencyFailure("P52-P41-IDENTITY")
    _validate_envelope(
        receipt,
        schema="ferris.pulse-41-transactional-copy-qualification-envelope/v1",
        payload_sha256=PULSE41_RECEIPT_PAYLOAD_SHA256,
        identifier="receipt_id",
        code="P52-P41-IDENTITY",
    )
    _validate_envelope(
        seal,
        schema="ferris.pulse-41-transactional-copy-release-seal-envelope/v1",
        payload_sha256=PULSE41_SEAL_PAYLOAD_SHA256,
        identifier="seal_id",
        code="P52-P41-IDENTITY",
    )
    global _PULSE41_MODULE
    module = _PULSE41_MODULE or _import_release_module(
        "pulse52_exact_p41_transactional_copy",
        release / "transactional_copy.py",
        "P52-P41-IMPORT",
    )
    if (
        getattr(module, "EXPECTED_COUNT", None) != 8
        or len(getattr(module, "EXPECTED_PATHS", ())) != 8
        or getattr(module, "SYNC_MECHANISM", None) != "os.open+os.fsync-directory-v1"
        or not isinstance(getattr(module, "PublicFailure", None), type)
    ):
        raise SealedDependencyFailure("P52-P41-CALLABLE")
    for name, parameters in (
        (
            "copy_release",
            (
                "source_root_value",
                "final_root_value",
                "synchronizer",
                "copier",
                "renamer",
                "remover",
                "post_rename",
            ),
        ),
        ("verify_bound_tree", ("root", "phase")),
        ("_safe_absolute", ("value",)),
    ):
        _signature(module, name, parameters, "P52-P41-CALLABLE")
    _PULSE41_MODULE = module
    return module


def load_p39_and_p41(repo_root: Path) -> tuple[ModuleType, ModuleType]:
    """Bind exact P39/P41 trees before public checkout/copy custody runs."""

    return _load_pulse39(repo_root), _load_pulse41(repo_root)


def _import_single(name: str, path: Path) -> ModuleType:
    specification = importlib.util.spec_from_file_location(name, path)
    if specification is None or specification.loader is None:
        raise SealedDependencyFailure("P52-P35-IMPORT")
    module = importlib.util.module_from_spec(specification)
    sys.modules[name] = module
    try:
        specification.loader.exec_module(module)
    except (ImportError, OSError) as error:
        sys.modules.pop(name, None)
        raise SealedDependencyFailure("P52-P35-IMPORT") from error
    return module


def load_p35_materializer_and_verifier(repo_root: Path) -> tuple[ModuleType, ModuleType]:
    """Bind and import the exact P35 materializer and private verifier."""

    release = _release_root(repo_root, PULSE35_DIRECTORY)
    manifest_raw = _safe_regular(release / "public-manifest.json", "P52-P35-IDENTITY")
    materializer_raw = _safe_regular(
        release / "corpus_materializer.py", "P52-P35-IDENTITY", maximum=131_072
    )
    verifier_raw = _safe_regular(
        release / "verify_materialization.py", "P52-P35-IDENTITY", maximum=131_072
    )
    if (
        _digest(manifest_raw) != PULSE35_MANIFEST_RAW_SHA256
        or _digest(materializer_raw) not in PULSE35_MATERIALIZER_RAW_VARIANTS
        or _digest(verifier_raw) not in PULSE35_VERIFIER_RAW_VARIANTS
    ):
        raise SealedDependencyFailure("P52-P35-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P52-P35-IDENTITY")
    if (
        manifest.get("schema") != "ferris.pulse-35-public-corpus-materializer-manifest/v1"
        or manifest.get("aggregate") != PULSE35_MANIFEST_AGGREGATE
        or manifest.get("file_count") != 8
        or manifest.get("total_bytes") != 403316
    ):
        raise SealedDependencyFailure("P52-P35-IDENTITY")
    materializer = _import_single("pulse52_exact_p35_materializer", release / "corpus_materializer.py")
    verifier = _import_single("pulse52_exact_p35_verifier", release / "verify_materialization.py")
    for module in (materializer, verifier):
        if (
            getattr(module, "REQUIRED_CASE_COUNT", None) != 70
            or getattr(module, "MAX_LOGICAL_CASES", None) != 512
            or getattr(module, "DERIVATION", None)
            != "hmac-sha256-seed-key-domain-purpose-counter-v1"
            or not isinstance(getattr(module, "MaterializationError", None), type)
        ):
            raise SealedDependencyFailure("P52-P35-CALLABLE")
    _signature(materializer, "materialize", ("seed_path", "output", "case_count"))
    _signature(materializer, "seed_commitment", ("seed",))
    _signature(verifier, "verify", ("output", "seed_path"))
    return materializer, verifier
