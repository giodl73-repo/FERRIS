"""Harmless fake-only loader for the Pulse 66 WSL worker bootstrap probe."""

from __future__ import annotations

import subprocess
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType


class ReleaseFailure(RuntimeError):
    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass
class ProbeHandle:
    platform: str
    closed: bool = False


def _p56_root(repo_root: Path) -> Path:
    return (
        repo_root
        / "docs"
        / "simulations"
        / "profile-diff-held-out"
        / "pulse-56-retained-build-custody-release"
    )


def load_exact_p56(repo_root: Path) -> ModuleType:
    root = _p56_root(repo_root)
    module = ModuleType("p66_worker_probe_p56")
    module.__file__ = str(root / "retained_build_custody.py")
    module.ReleaseFailure = ReleaseFailure

    def publish_retained_build_and_custody(
        platform: str, runtime_parent: str | Path
    ) -> ProbeHandle:
        if platform != "ubuntu-24.04-x86_64" or not isinstance(runtime_parent, (str, Path)):
            raise ReleaseFailure("P56-UNSUPPORTED-PLATFORM")
        return ProbeHandle(platform)

    def launch_verified(
        handle: ProbeHandle, platform: str, arguments: tuple[str, ...] | list[str]
    ) -> subprocess.CompletedProcess[bytes]:
        raise ReleaseFailure("P56-LAUNCH-FORBIDDEN")

    def close_custody(handle: ProbeHandle) -> None:
        if type(handle) is not ProbeHandle or handle.closed:
            raise ReleaseFailure("P56-HANDLE-EXPIRED")
        handle.closed = True

    module.publish_retained_build_and_custody = publish_retained_build_and_custody
    module.launch_verified = launch_verified
    module.close_custody = close_custody
    return module
