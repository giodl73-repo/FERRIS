"""Harmless route-equivalence helpers for the Pulse 67 WSL probe worker."""

from __future__ import annotations

import json
import os
import stat
import subprocess
from dataclasses import dataclass
from pathlib import Path


PROBE_SCHEMA = "ferris.pulse-67-wsl-probe-session/v1"
PROBE_RESULT_SCHEMA = "ferris.pulse-67-wsl-probe-result/v1"
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
P56_RELEASE_ROOT = (
    "docs/simulations/profile-diff-held-out/"
    "pulse-56-retained-build-custody-release"
)
EXACT_P56_RELEASE_FILES = (
    "README.md",
    "generate_release.py",
    "public-manifest.json",
    "qualification-receipt.json",
    "qualify.py",
    "release-seal.json",
    "retained_build_custody.py",
    "root-cause-report.md",
    "schemas/ferris.pulse-56-retained-build-receipt.v1.schema.json",
    "tests/test_retained_build_custody.py",
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


def _canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


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


def _exact_p56_paths(root: Path) -> set[str]:
    discovered: set[str] = set()
    for directory, _, files in os.walk(root):
        current = Path(directory)
        try:
            metadata = os.lstat(current)
        except OSError as error:
            raise ReleaseFailure("P67-PROBE-P56-TREE") from error
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise ReleaseFailure("P67-PROBE-P56-TREE")
        for name in files:
            path = current / name
            try:
                file_meta = os.lstat(path)
            except OSError as error:
                raise ReleaseFailure("P67-PROBE-P56-TREE") from error
            if stat.S_ISLNK(file_meta.st_mode) or not stat.S_ISREG(file_meta.st_mode):
                raise ReleaseFailure("P67-PROBE-P56-TREE")
            discovered.add(path.relative_to(root).as_posix())
    return discovered


def _p57_bundle_name(name: str) -> bool:
    return (
        name.startswith(".p57-")
        and len(name) == len(".p57-") + 32
        and all(character in "0123456789abcdef" for character in name[len(".p57-"):])
    )


def bind_probe_context(bundle_root: Path, p56_root: Path, runtime_parent: Path) -> ProbeContext:
    bundle_root = _native_directory(bundle_root, "P67-PROBE-BUNDLE-ROOT")
    runtime_parent = _native_directory(runtime_parent, "P67-PROBE-RUNTIME-PARENT")
    p56_root = _native_directory(p56_root, "P67-PROBE-P56-ROOT")
    if bundle_root.parent != runtime_parent or not _p57_bundle_name(bundle_root.name):
        raise ReleaseFailure("P67-PROBE-BUNDLE-ROOT")
    expected_p56_root = bundle_root / "repository" / Path(P56_RELEASE_ROOT)
    if p56_root != expected_p56_root:
        raise ReleaseFailure("P67-PROBE-P56-ROOT")
    if _exact_p56_paths(p56_root) != set(EXACT_P56_RELEASE_FILES):
        raise ReleaseFailure("P67-PROBE-P56-TREE")
    return ProbeContext(bundle_root=bundle_root, p56_root=p56_root, runtime_parent=runtime_parent)


def launch_harmless_probe(
    context: ProbeContext, platform: str, arguments: tuple[str, ...] | list[str]
) -> subprocess.CompletedProcess[bytes]:
    if type(context) is not ProbeContext or context.closed or context.probe_invocations != 0:
        raise ReleaseFailure("P67-PROBE-STATE")
    if platform != PLATFORM:
        raise ReleaseFailure("P67-PROBE-PLATFORM")
    if (
        type(arguments) not in {tuple, list}
        or len(arguments) != 7
        or any(type(value) is not str or "\x00" in value for value in arguments)
    ):
        raise ReleaseFailure("P67-PROBE-REQUEST")
    context.probe_invocations += 1
    stdout = _canonical_bytes(
        {
            "platform": PLATFORM,
            "probe": "p57-route-equivalence",
            "production_dependency_sha256": PRODUCTION_P57_SEALED_DEPENDENCIES_SHA256,
            "production_worker_sha256": PRODUCTION_P57_WORKER_SHA256,
            "request_argument_count": len(arguments),
            "schema": PROBE_RESULT_SCHEMA,
            "staged_p56_release_file_count": len(EXACT_P56_RELEASE_FILES),
            "type": "probe-result",
        }
    ) + b"\n"
    return subprocess.CompletedProcess(list(arguments), 0, stdout, b"")


def close_probe_context(context: ProbeContext) -> None:
    if type(context) is not ProbeContext or context.closed:
        raise ReleaseFailure("P67-PROBE-HANDLE")
    context.closed = True
