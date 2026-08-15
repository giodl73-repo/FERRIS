"""Fail-closed exact imports for the Pulse 53 terminal replacement.

Pulse 53 verifies the complete Pulse 52 release tree before importing its
bounded phase engine, then invokes Pulse 52's own exact Pulse 51 loader.  The
production wrapper never substitutes an unsealed predecessor or callback.
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


PULSE51_COMMIT = "d09c923c1e2cd2be003026597f4ad2a0e2d3764f"
PULSE51_SOURCE_SHA256 = (
    "sha256:97c404dbf29d387561878772403c7fbd2672e97283b0620e838e7126ecbdd637"
)
PULSE52_COMMIT = "e4ef9617f227670f3911be42ca63df4b2e66d24f"
PULSE52_DIRECTORY = "pulse-52-ordered-materialization-executor-release"
PULSE52_MANIFEST_RAW_SHA256 = (
    "sha256:e585d6baaf83783ff1a1c65e1d3f281ce1d3afd9806f9cb9811b328eff9811da"
)
PULSE52_MANIFEST_AGGREGATE = (
    "sha256:3da8401a52d020ead7b9c6854461da5f28dfb9d1117385cd6943592f74e8aaec"
)
PULSE52_RECEIPT_RAW_SHA256 = (
    "sha256:1eaf50c293e4c44f9312b28efa581912ed4165e8f77014c703cfc54496b37192"
)
PULSE52_RECEIPT_PAYLOAD_SHA256 = (
    "sha256:183a7c6f0ebbab38bbe5b29efc4c1ebd3c5e1e8ca8ca84a5cc5d29107798a7ac"
)
PULSE52_SEAL_RAW_SHA256 = (
    "sha256:febee1ea581a3564da89714aaeae1c909b0a9345676958bbb6e2fe4ec2d72ca6"
)
PULSE52_SEAL_PAYLOAD_SHA256 = (
    "sha256:46d9e8bb1aa75780fb7397fd4833e13c5e28c0ec79254185ef6da793e4ed7f84"
)
PULSE52_SOURCE_SHA256 = (
    "sha256:768f4dc3af1009515e2e28ebc211af76215f434cee209b547d7be923a1bcec73"
)
PULSE52_FILE_COUNT = 10
PULSE52_TREE_FILE_COUNT = 12
PULSE52_SCHEMA = "ferris.pulse-52-ordered-materialization-executor-public-manifest/v1"

_PULSE52: ModuleType | None = None
_PULSE51: ModuleType | None = None


class SealedDependencyFailure(RuntimeError):
    """An exact Pulse 51/Pulse 52 identity or callable binding failed."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def _digest(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    value: dict[str, object] = {}
    for key, member in pairs:
        if key in value:
            raise SealedDependencyFailure("P53-SEALED-JSON-DUPLICATE")
        value[key] = member
    return value


def _safe_regular(path: Path, code: str, maximum: int = 4_194_304) -> bytes:
    try:
        before = os.lstat(path)
        if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
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
        after = os.fstat(descriptor)
        if not stat.S_ISREG(after.st_mode) or (before.st_dev, before.st_ino) != (
            after.st_dev,
            after.st_ino,
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


def _sealed_json(raw: bytes, code: str) -> dict[str, object]:
    try:
        value = json.loads(raw, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, SealedDependencyFailure) as error:
        raise SealedDependencyFailure(code) from error
    if type(value) is not dict:
        raise SealedDependencyFailure(code)
    return value


def _safe_root(repo_root: Path) -> Path:
    try:
        if not repo_root.is_absolute():
            raise SealedDependencyFailure("P53-SEALED-ROOT")
        root = repo_root.resolve(strict=True)
        probe = root
        while True:
            metadata = os.lstat(probe)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise SealedDependencyFailure("P53-SEALED-ROOT")
            if probe == probe.parent:
                return root
            probe = probe.parent
    except SealedDependencyFailure:
        raise
    except OSError as error:
        raise SealedDependencyFailure("P53-SEALED-ROOT") from error


def _release_root(repo_root: Path) -> Path:
    root = _safe_root(repo_root)
    release = root / "docs" / "simulations" / "profile-diff-held-out" / PULSE52_DIRECTORY
    try:
        release.relative_to(root)
        metadata = os.lstat(release)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise SealedDependencyFailure("P53-P52-IDENTITY")
    except SealedDependencyFailure:
        raise
    except (OSError, ValueError) as error:
        raise SealedDependencyFailure("P53-P52-IDENTITY") from error
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
            candidate = Path(entry.path)
            try:
                metadata = os.lstat(candidate)
                if stat.S_ISLNK(metadata.st_mode):
                    raise SealedDependencyFailure(code)
                relative = candidate.relative_to(root).as_posix()
                if stat.S_ISDIR(metadata.st_mode):
                    visit(candidate)
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


def _manifest_entries(manifest: dict[str, object], code: str) -> dict[str, dict[str, object]]:
    files = manifest.get("files")
    if type(files) is not list:
        raise SealedDependencyFailure(code)
    entries: dict[str, dict[str, object]] = {}
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
            or path in entries
            or type(digest) is not str
            or len(digest) != 71
            or not digest.startswith("sha256:")
            or type(size) is not int
            or size < 0
        ):
            raise SealedDependencyFailure(code)
        try:
            if len(bytes.fromhex(digest.removeprefix("sha256:"))) != 32:
                raise SealedDependencyFailure(code)
        except ValueError as error:
            raise SealedDependencyFailure(code) from error
        entries[path] = entry
    return entries


def _aggregate(entries: dict[str, dict[str, object]], code: str) -> str:
    hasher = hashlib.sha256()
    for path in sorted(entries, key=lambda item: item.encode("utf-8")):
        try:
            digest = bytes.fromhex(str(entries[path]["sha256"]).removeprefix("sha256:"))
        except ValueError as error:
            raise SealedDependencyFailure(code) from error
        encoded = path.encode("utf-8")
        hasher.update(len(encoded).to_bytes(8, "big"))
        hasher.update(encoded)
        hasher.update(digest)
    return "sha256:" + hasher.hexdigest()


def _verify_pulse52_tree(release: Path, manifest: dict[str, object]) -> None:
    code = "P53-P52-IDENTITY"
    entries = _manifest_entries(manifest, code)
    if (
        manifest.get("schema") != PULSE52_SCHEMA
        or manifest.get("aggregate") != PULSE52_MANIFEST_AGGREGATE
        or manifest.get("file_count") != PULSE52_FILE_COUNT
        or manifest.get("release_tree_file_count") != PULSE52_TREE_FILE_COUNT
        or len(entries) != PULSE52_FILE_COUNT
        or _aggregate(entries, code) != PULSE52_MANIFEST_AGGREGATE
    ):
        raise SealedDependencyFailure(code)
    expected_tree = set(entries) | {
        "public-manifest.json",
        "qualification-receipt.json",
        "release-seal.json",
    }
    if len(expected_tree) != PULSE52_TREE_FILE_COUNT or _tree_paths(release, code) != expected_tree:
        raise SealedDependencyFailure(code)
    for relative, entry in entries.items():
        raw = _safe_regular(release / relative, code)
        if len(raw) != entry["size"] or _digest(raw) != entry["sha256"]:
            raise SealedDependencyFailure(code)


def _import_pulse52(release: Path) -> ModuleType:
    """Import P52 against its own verified dependency module, not this wrapper."""

    generic_name = "sealed_dependencies"
    previous_generic = sys.modules.get(generic_name)
    dependency_name = "pulse53_exact_pulse52_dependencies"
    executor_name = "pulse53_exact_pulse52_ordered_materialization_executor"
    sys.modules.pop(dependency_name, None)
    sys.modules.pop(executor_name, None)
    try:
        dependency_spec = importlib.util.spec_from_file_location(
            dependency_name, release / "sealed_dependencies.py"
        )
        if dependency_spec is None or dependency_spec.loader is None:
            raise SealedDependencyFailure("P53-P52-IMPORT")
        dependency_module = importlib.util.module_from_spec(dependency_spec)
        sys.modules[dependency_name] = dependency_module
        sys.modules[generic_name] = dependency_module
        dependency_spec.loader.exec_module(dependency_module)

        executor_spec = importlib.util.spec_from_file_location(
            executor_name, release / "ordered_materialization_executor.py"
        )
        if executor_spec is None or executor_spec.loader is None:
            raise SealedDependencyFailure("P53-P52-IMPORT")
        executor_module = importlib.util.module_from_spec(executor_spec)
        sys.modules[executor_name] = executor_module
        executor_spec.loader.exec_module(executor_module)
        return executor_module
    except SealedDependencyFailure:
        raise
    except (ImportError, OSError, SyntaxError) as error:
        raise SealedDependencyFailure("P53-P52-IMPORT") from error
    finally:
        if previous_generic is None:
            sys.modules.pop(generic_name, None)
        else:
            sys.modules[generic_name] = previous_generic


def _signature(module: ModuleType, name: str, parameters: tuple[str, ...]) -> None:
    value = getattr(module, name, None)
    if not callable(value) or tuple(inspect.signature(value).parameters) != parameters:
        raise SealedDependencyFailure("P53-P52-CALLABLE")


def _validate_pulse52_callable(module: ModuleType) -> None:
    if tuple(getattr(module, "__all__", ())) != (
        "OrderedMaterializationResult",
        "TerminalPublicationCleanupIndeterminate",
        "run_ordered_materialization_executor",
    ):
        raise SealedDependencyFailure("P53-P52-CALLABLE")
    if (
        getattr(module, "P50_GATE_IDS", None)
        != (
            "pulse-41-pulse-39-public-custody",
            "windows-retained-binary-custody",
            "ubuntu-retained-binary-custody",
            "exact-adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "bounded-materialization",
            "bounded-process-exit-search",
        )
        or getattr(module, "CANONICAL_PLATFORMS", None)
        != ("windows-x86_64", "ubuntu-24.04-x86_64")
        or not isinstance(getattr(module, "OrderedMaterializationResult", None), type)
    ):
        raise SealedDependencyFailure("P53-P52-CALLABLE")
    for name, parameters in (
        (
            "run_ordered_materialization_executor",
            (
                "repo_root",
                "private_runtime_root",
                "p27_cycle_root",
                "p39_checkout_root",
                "p41_final_root",
                "retained_custodies",
            ),
        ),
        (
            "_run_loaded",
            (
                "p51",
                "p39",
                "p41",
                "repo_root",
                "private_runtime_root",
                "p27_cycle_root",
                "p39_checkout_root",
                "p41_final_root",
                "retained_custodies",
                "controls",
            ),
        ),
        ("_prepare_terminal", ("p51", "runtime_root", "repo_root")),
        (
            "_cleanup_terminal_publication",
            ("p51", "parent", "p43_root", "witness_root", "private_record"),
        ),
        (
            "_published_terminal_summary",
            ("p43", "p47", "summary", "p43_root", "witness_root"),
        ),
        ("_p47_failure_posture", ("p47", "summary")),
        ("_published_witness_posture", ("value",)),
        ("_private_record", ()),
        ("_catalog", ()),
        ("_event", ("gate_id", "kind", "outcome")),
    ):
        _signature(module, name, parameters)


def _validate_pulse51(module: ModuleType) -> None:
    if (
        getattr(module, "P33_CUTOFF", None) != "29517d732db13cc2ffa304684b344f3538ab587d"
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
        raise SealedDependencyFailure("P53-P51-CALLABLE")


def load_pulse52(repo_root: Path) -> tuple[ModuleType, ModuleType]:
    """Return P52 and P51 only after full exact-tree and callable binding."""

    release = _release_root(repo_root)
    manifest_raw = _safe_regular(release / "public-manifest.json", "P53-P52-IDENTITY")
    receipt_raw = _safe_regular(
        release / "qualification-receipt.json", "P53-P52-IDENTITY", maximum=1_048_576
    )
    seal_raw = _safe_regular(release / "release-seal.json", "P53-P52-IDENTITY")
    source_raw = _safe_regular(
        release / "ordered_materialization_executor.py", "P53-P52-IDENTITY"
    )
    if (
        _digest(manifest_raw) != PULSE52_MANIFEST_RAW_SHA256
        or _digest(receipt_raw) != PULSE52_RECEIPT_RAW_SHA256
        or _digest(seal_raw) != PULSE52_SEAL_RAW_SHA256
        or _digest(source_raw) != PULSE52_SOURCE_SHA256
    ):
        raise SealedDependencyFailure("P53-P52-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P53-P52-IDENTITY")
    receipt = _sealed_json(receipt_raw, "P53-P52-IDENTITY")
    seal = _sealed_json(seal_raw, "P53-P52-IDENTITY")
    _verify_pulse52_tree(release, manifest)
    if (
        receipt.get("schema")
        != "ferris.pulse-52-ordered-materialization-executor-qualification-envelope/v1"
        or receipt.get("payload_sha256") != PULSE52_RECEIPT_PAYLOAD_SHA256
        or receipt.get("receipt_id") != PULSE52_RECEIPT_PAYLOAD_SHA256
        or seal.get("schema")
        != "ferris.pulse-52-ordered-materialization-executor-release-seal/v1"
        or seal.get("payload_sha256") != PULSE52_SEAL_PAYLOAD_SHA256
        or seal.get("receipt_id") != PULSE52_SEAL_PAYLOAD_SHA256
    ):
        raise SealedDependencyFailure("P53-P52-IDENTITY")

    global _PULSE52, _PULSE51
    module = _PULSE52 or _import_pulse52(release)
    _validate_pulse52_callable(module)
    try:
        p51 = _PULSE51 or module.load_pulse51(repo_root)
    except module.SealedDependencyFailure as error:
        raise SealedDependencyFailure("P53-P51-IDENTITY") from error
    _validate_pulse51(p51)
    _PULSE52 = module
    _PULSE51 = p51
    return module, p51


__all__ = [
    "PULSE51_COMMIT",
    "PULSE51_SOURCE_SHA256",
    "PULSE52_COMMIT",
    "PULSE52_MANIFEST_AGGREGATE",
    "PULSE52_MANIFEST_RAW_SHA256",
    "PULSE52_SOURCE_SHA256",
    "SealedDependencyFailure",
    "load_pulse52",
]
