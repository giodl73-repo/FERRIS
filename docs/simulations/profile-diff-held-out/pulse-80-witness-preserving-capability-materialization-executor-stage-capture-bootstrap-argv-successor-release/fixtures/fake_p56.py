"""Private fake-only Pulse 56 capability implementation for qualification."""

from __future__ import annotations

import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from types import SimpleNamespace
from typing import Callable


class FakeReleaseFailure(RuntimeError):
    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass
class FakeHandle:
    platform: str
    remaining: int = 69
    closed: bool = False


class FakeP56:
    """Matches only the three P56 capability calls with harmless fake output."""

    DEFAULT_LAUNCH_USES = 69
    ReleaseFailure = FakeReleaseFailure

    def __init__(
        self,
        fake_ferris: Path,
        *,
        fail_cleanup: bool = False,
        uses: int = 69,
        on_launch: Callable[[str, tuple[str, ...], int], None] | None = None,
        result_mutator: Callable[[subprocess.CompletedProcess[bytes]], subprocess.CompletedProcess[bytes]]
        | None = None,
        runner: Callable[[object], object] | None = None,
        runtime_root: Path | None = None,
        p51: object | None = None,
    ) -> None:
        self.fake_ferris = fake_ferris
        self.fail_cleanup = fail_cleanup
        self.uses = uses
        self.on_launch = on_launch
        self.result_mutator = result_mutator
        self.runner = runner
        self.runtime_root = runtime_root
        self.p51 = p51
        self.publishes: list[str] = []
        self.launches: list[tuple[str, tuple[str, ...]]] = []
        self.closes: list[str] = []

    def publish_retained_build_and_custody(
        self, platform: str, runtime_parent: str | Path
    ) -> FakeHandle:
        if platform not in {"windows-x86_64", "ubuntu-24.04-x86_64"}:
            raise FakeReleaseFailure("P56-UNSUPPORTED-PLATFORM")
        self.publishes.append(platform)
        return FakeHandle(platform, self.uses)

    def launch_verified(
        self, handle: FakeHandle, platform: str, arguments: tuple[str, ...] | list[str]
    ) -> subprocess.CompletedProcess[bytes]:
        if (
            type(handle) is not FakeHandle
            or handle.platform != platform
            or type(arguments) not in {tuple, list}
            or any(type(argument) is not str for argument in arguments)
        ):
            raise FakeReleaseFailure("P56-HANDLE-FORGERY")
        if handle.closed or handle.remaining == 0:
            raise FakeReleaseFailure("P56-HANDLE-EXPIRED")
        argv = tuple(arguments)
        self.launches.append((platform, argv))
        if self.on_launch is not None:
            self.on_launch(platform, argv, len(self.launches))
        if self.runner is not None:
            if self.runtime_root is None or self.p51 is None:
                raise FakeReleaseFailure("P58-FAKE-RUNNER")
            executable = self.runtime_root / "fake-ferris"
            if platform == "windows-x86_64":
                dispatch = SimpleNamespace(
                    application_argv=argv,
                    command=(str(executable), *argv),
                    executable=executable,
                    host_cwd=self.runtime_root,
                    platform=platform,
                    wsl_cwd=None,
                )
            else:
                wsl_cwd = self.p51.windows_to_wsl_absolute(self.runtime_root, self.runtime_root)
                dispatch = SimpleNamespace(
                    application_argv=argv,
                    command=(
                        "wsl.exe",
                        "--distribution",
                        "Ubuntu-24.04",
                        "--cd",
                        wsl_cwd,
                        "--exec",
                        self.p51.windows_to_wsl_absolute(executable, self.runtime_root),
                        "profile-diff",
                        "--before",
                        self.p51.windows_to_wsl_absolute(
                            Path(argv[2]), self.runtime_root
                        ),
                        "--after",
                        self.p51.windows_to_wsl_absolute(
                            Path(argv[4]), self.runtime_root
                        ),
                        "--format",
                        argv[6],
                    ),
                    executable=executable,
                    host_cwd=self.runtime_root,
                    platform=platform,
                    wsl_cwd=wsl_cwd,
                )
            completed = self.runner(dispatch)
        else:
            completed = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(self.fake_ferris),
                    "--synthetic-platform",
                    platform,
                    *argv,
                ],
                check=False,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
        handle.remaining -= 1
        if handle.remaining == 0:
            handle.closed = True
        if self.result_mutator is not None:
            return self.result_mutator(completed)
        return completed

    def close_custody(self, handle: FakeHandle) -> None:
        if type(handle) is not FakeHandle or handle.closed:
            raise FakeReleaseFailure("P56-HANDLE-EXPIRED")
        self.closes.append(handle.platform)
        handle.closed = True
        if self.fail_cleanup:
            raise FakeReleaseFailure("P56-INDETERMINATE-CLEANUP")
