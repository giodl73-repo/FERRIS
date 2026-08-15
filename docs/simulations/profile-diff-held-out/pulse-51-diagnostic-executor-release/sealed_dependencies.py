"""Verified imports of the sealed Pulse 27, 43, 45, and 47 public modules."""

from __future__ import annotations

import hashlib
import importlib.util
import inspect
import json
import os
import stat
import sys
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Callable


@dataclass(frozen=True)
class SealedRelease:
    directory: str
    source: str
    source_sha256: str
    manifest_raw_sha256: str
    manifest_aggregate: str
    receipt_raw_sha256: str
    receipt_payload_sha256: str
    seal_raw_sha256: str
    seal_payload_sha256: str
    file_count: int
    tree_file_count: int
    manifest_schema: str


P43 = SealedRelease(
    directory="pulse-43-ordered-result-publisher-release",
    source="ordered_result_publisher.py",
    source_sha256="sha256:38ebc7ce84ae29c2ad20ada593d8baeb0352b59e7c48438c4a9c224a0ea4a6c6",
    manifest_raw_sha256="sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4",
    manifest_aggregate="sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346",
    receipt_raw_sha256="sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c",
    receipt_payload_sha256="sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2",
    seal_raw_sha256="sha256:4445a0b181419b303c28f1d91e1700a594d2a040fbae1cd0dc526fa7fc8e8f05",
    seal_payload_sha256="sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1",
    file_count=6,
    tree_file_count=9,
    manifest_schema="ferris.pulse-43-ordered-result-public-manifest/v1",
)

P45 = SealedRelease(
    directory="pulse-45-binary-custody-event-bridge-release",
    source="binary_custody_event_bridge.py",
    source_sha256="sha256:3d903500c7e61123ed9e3248e03d2196663b652227440064c8ab765d2cb16860",
    manifest_raw_sha256="sha256:f8574972a8dc7791580d26dcf17a0ffcb0c55024e8d753616dcbba7c592dd544",
    manifest_aggregate="sha256:4a6c3fb5093aeff681c62636e36b78dc581e2491672207bbc64ecf0e01bd434d",
    receipt_raw_sha256="sha256:40b9dac86b496be10dd550e9119fa250f70a0acd6f63b019fd66c6496c1086ce",
    receipt_payload_sha256="sha256:fb7049852a417baaa2afd41decd26b508ad5727d6e2252a05d4f79ab44989bd9",
    seal_raw_sha256="sha256:7a087787d040103643436c2b6bee5bb58f803d1a5c0a897d9cb9f8e935f75c86",
    seal_payload_sha256="sha256:f39e38597f479467bc5f154a17edb8b1a97e5df8aa7d6c3dca0e755019dc4588",
    file_count=6,
    tree_file_count=9,
    manifest_schema="ferris.pulse-45-binary-custody-event-bridge-public-manifest/v1",
)

P47 = SealedRelease(
    directory="pulse-47-publication-outcome-witness-release",
    source="publication_outcome_witness.py",
    source_sha256="sha256:4a402d3c2e034597a574368e628af0b87966b74ec2cdef947b38db2881cf4760",
    manifest_raw_sha256="sha256:44d5c72b9eb09dc7e24b476a4535fed662eadde3edee6ecbfe1fdfa644082f8b",
    manifest_aggregate="sha256:5cb97276ee2752888c40d44a50e45079c9e550f7e26398e5aa4841d98083143d",
    receipt_raw_sha256="sha256:be73ee9a87377e58a87c04308557ef118afbb7ed0fb117b039cc569f9040b265",
    receipt_payload_sha256="sha256:dbe44afbb9f0ad43549113028da8dc5d2d0ca5fe9faa15824d7cd80e3edea355",
    seal_raw_sha256="sha256:4300f5ba89bdaefb938b91092adf7d1c62dbf11ba6e1a4350c9a34c03cce1a8e",
    seal_payload_sha256="sha256:a00478e73897781ddd88e8e0fcbca2d1453a72758cbbd8ec06ccd9d0c228f681",
    file_count=6,
    tree_file_count=9,
    manifest_schema="ferris.pulse-47-publication-outcome-witness-public-manifest/v1",
)

P27_MANIFEST_RAW_SHA256 = "sha256:7a6e61dacb3d58ab6d8c75cf1267a70f7919219baadd34329b835640931e8d5e"
P27_MANIFEST_AGGREGATE = "sha256:531113c7c8a50f1c71c446bc708e44549702623114625ea46f5aa874b6aea721"
P27_SOURCE_SHA256 = "sha256:c969a8522df46eee0a5809eb20652d7e4b1ff54e90f8a8d9f09810f4eb7a7442"
P27_DIRECTORY = "pulse-27-preflight-adapter-release"


class DependencyFailure(RuntimeError):
    """A sealed predecessor cannot be imported as its exact public release."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def _digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise DependencyFailure("P51-SEALED-JSON-DUPLICATE")
        result[key] = value
    return result


def _safe_regular(path: Path, code: str) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise DependencyFailure(code)
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except DependencyFailure:
        raise
    except OSError as error:
        raise DependencyFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise DependencyFailure(code)
        chunks: list[bytes] = []
        while chunk := os.read(descriptor, 65_536):
            chunks.append(chunk)
        return b"".join(chunks)
    except OSError as error:
        raise DependencyFailure(code) from error
    finally:
        os.close(descriptor)


def _sealed_json(data: bytes, code: str) -> dict[str, object]:
    try:
        value = json.loads(data, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, DependencyFailure) as error:
        raise DependencyFailure(code) from error
    if type(value) is not dict:
        raise DependencyFailure(code)
    return value


def _release_root(repo_root: Path, directory: str) -> Path:
    try:
        root = repo_root.resolve(strict=True)
    except OSError as error:
        raise DependencyFailure("P51-SEALED-ROOT") from error
    release = root / "docs" / "simulations" / "profile-diff-held-out" / directory
    try:
        release.relative_to(root)
    except ValueError as error:
        raise DependencyFailure("P51-SEALED-ROOT") from error
    return release


def _load_module(name: str, source: Path) -> ModuleType:
    specification = importlib.util.spec_from_file_location(name, source)
    if specification is None or specification.loader is None:
        raise DependencyFailure("P51-SEALED-IMPORT")
    module = importlib.util.module_from_spec(specification)
    sys.modules[name] = module
    try:
        specification.loader.exec_module(module)
    except (ImportError, OSError) as error:
        sys.modules.pop(name, None)
        raise DependencyFailure("P51-SEALED-IMPORT") from error
    return module


def load_sealed_release(repo_root: Path, release: SealedRelease) -> ModuleType:
    """Verify all seal identities before importing a terminal dependency."""

    root = _release_root(repo_root, release.directory)
    manifest_raw = _safe_regular(root / "public-manifest.json", "P51-SEALED-IDENTITY")
    receipt_raw = _safe_regular(root / "qualification-receipt.json", "P51-SEALED-IDENTITY")
    seal_raw = _safe_regular(root / "release-seal.json", "P51-SEALED-IDENTITY")
    source = root / release.source
    source_raw = _safe_regular(source, "P51-SEALED-IDENTITY")
    if (
        _digest(manifest_raw) != release.manifest_raw_sha256
        or _digest(receipt_raw) != release.receipt_raw_sha256
        or _digest(seal_raw) != release.seal_raw_sha256
        or _digest(source_raw) != release.source_sha256
    ):
        raise DependencyFailure("P51-SEALED-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P51-SEALED-IDENTITY")
    receipt = _sealed_json(receipt_raw, "P51-SEALED-IDENTITY")
    seal = _sealed_json(seal_raw, "P51-SEALED-IDENTITY")
    if (
        manifest.get("schema") != release.manifest_schema
        or manifest.get("aggregate") != release.manifest_aggregate
        or manifest.get("file_count") != release.file_count
        or manifest.get("release_tree_file_count") != release.tree_file_count
        or receipt.get("payload_sha256") != release.receipt_payload_sha256
        or receipt.get("receipt_id") != release.receipt_payload_sha256
        or seal.get("payload_sha256") != release.seal_payload_sha256
        or seal.get("seal_id") != release.seal_payload_sha256
    ):
        raise DependencyFailure("P51-SEALED-IDENTITY")
    return _load_module("pulse51_" + release.directory.replace("-", "_"), source)


def load_p27_exact_runner(repo_root: Path) -> Callable[[Path], dict[str, object]]:
    """Load only the exact P27 callable, never its qualification CLI."""

    root = _release_root(repo_root, P27_DIRECTORY)
    manifest_raw = _safe_regular(root / "public-manifest.json", "P51-P27-IDENTITY")
    source = root / "adapter.py"
    if _digest(manifest_raw) != P27_MANIFEST_RAW_SHA256 or _digest(
        _safe_regular(source, "P51-P27-IDENTITY")
    ) != P27_SOURCE_SHA256:
        raise DependencyFailure("P51-P27-IDENTITY")
    manifest = _sealed_json(manifest_raw, "P51-P27-IDENTITY")
    if (
        manifest.get("schema") != "exact-two-preflight-public-manifest-v1"
        or type(manifest.get("digests")) is not dict
        or manifest["digests"].get("release_aggregate") != P27_MANIFEST_AGGREGATE
        or manifest.get("file_count") != 20
        or manifest.get("total_bytes") != 80579
    ):
        raise DependencyFailure("P51-P27-IDENTITY")
    module = _load_module("pulse51_p27_adapter", source)
    runner = getattr(module, "run_exact_two_cycle", None)
    if not callable(runner):
        raise DependencyFailure("P51-P27-CALLABLE")
    signature = inspect.signature(runner)
    parameters = tuple(signature.parameters.values())
    if len(parameters) != 1 or parameters[0].name != "cycle_root":
        raise DependencyFailure("P51-P27-CALLABLE")
    return runner


def verify_p27_summary(value: object) -> None:
    if type(value) is not dict:
        raise DependencyFailure("P51-P27-SUMMARY")
    if (
        value.get("schema") != "exact-two-preflight-cycle-v1"
        or value.get("outcome") != "pass"
        or value.get("pair_ids") != ["preflight-pair-000", "preflight-pair-001"]
        or value.get("pair_count") != 2
        or value.get("windows_record_count") != 2
        or value.get("ubuntu_record_count") != 2
        or value.get("process_record_count") != 4
        or value.get("pair_seal_count") != 2
        or value.get("durable_write_count") != 6
        or value.get("fresh_process_reload_count") != 2
        or value.get("residue_count") != 0
        or value.get("retries") != 0
    ):
        raise DependencyFailure("P51-P27-SUMMARY")
    verifiers = value.get("fresh_verifiers")
    if type(verifiers) is not dict or set(verifiers) != {"windows", "ubuntu"}:
        raise DependencyFailure("P51-P27-SUMMARY")


def load_terminal_dependencies(repo_root: Path) -> tuple[ModuleType, ModuleType, ModuleType]:
    """Return verified P43, P45, and P47 modules without invoking publication."""

    p43 = load_sealed_release(repo_root, P43)
    p45 = load_sealed_release(repo_root, P45)
    p47 = load_sealed_release(repo_root, P47)
    if (
        not callable(getattr(p43, "validate_catalog", None))
        or not callable(getattr(p43, "validate_events", None))
        or not callable(getattr(p43, "publish_result", None))
        or not callable(getattr(p45, "bridge_pulse_44", None))
        or not isinstance(getattr(p45, "PLATFORM_GATES", None), dict)
        or not callable(getattr(p47, "witness_pulse_43", None))
    ):
        raise DependencyFailure("P51-SEALED-CALLABLE")
    return p43, p45, p47
