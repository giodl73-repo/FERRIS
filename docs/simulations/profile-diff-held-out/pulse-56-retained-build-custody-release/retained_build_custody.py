#!/usr/bin/env python3
"""Pulse 56 retained builds, evidence custody, and live in-process launch handles."""

from __future__ import annotations

import argparse
import copy
import ctypes
import hashlib
import json
import os
import re
import secrets
import shutil
import stat
import subprocess
import sys
import threading
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Iterable


CUTOFF = "29517d732db13cc2ffa304684b344f3538ab587d"
PLATFORMS = frozenset({"windows-x86_64", "ubuntu-24.04-x86_64"})
RECEIPT_SCHEMA = "ferris.pulse-56-retained-build-receipt/v1"
RECEIPT_ENVELOPE_SCHEMA = "ferris.pulse-56-retained-build-receipt-envelope/v1"
SUMMARY_SCHEMA = "ferris.pulse-56-retained-build-custody-summary/v1"
HANDOFF_SCHEMA = "ferris.pulse-56-verified-launch-handoff/v2"
SYNC_MECHANISM = "os.open+os.fsync-directory-v1"
MAX_RECEIPT_BYTES = 131_072
DEFAULT_LAUNCH_USES = 69
_HANDLE_CONSTRUCTOR_KEY = object()
_REGISTRY_LOCK = threading.RLock()
_LIVE_HANDLES: dict["CustodyHandle", "_LiveCustody"] = {}


class ReleaseFailure(RuntimeError):
    """A bounded failure. Its code is safe to surface; its cause is not."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass(frozen=True)
class Binding:
    path: str
    size: int
    sha256: str

    def public(self) -> dict[str, object]:
        return {"path": self.path, "sha256": self.sha256, "size": self.size}


class CustodyHandle:
    """Opaque, identity-bound capability for a future same-process executor.

    A receipt or custody directory is evidence only.  This object authorizes a
    launch only while this exact object remains in the private registry.
    """

    __slots__ = ("__token",)

    def __init__(self, key: object, token: bytes) -> None:
        if key is not _HANDLE_CONSTRUCTOR_KEY:
            raise ReleaseFailure("P56-HANDLE-FORGERY")
        self.__token = token

    def __copy__(self) -> "CustodyHandle":
        raise ReleaseFailure("P56-HANDLE-FORGERY")

    def __deepcopy__(self, memo: dict[int, object]) -> "CustodyHandle":
        raise ReleaseFailure("P56-HANDLE-FORGERY")

    @property
    def summary(self) -> dict[str, object]:
        with _REGISTRY_LOCK:
            record = _live_record(self)
            return copy.deepcopy(record.summary)

    @property
    def remaining_uses(self) -> int:
        with _REGISTRY_LOCK:
            return _live_record(self).remaining_uses


@dataclass
class _LiveCustody:
    token: bytes
    platform: str
    artifact: Binding
    artifact_bytes: bytes
    receipt: Binding
    receipt_document: dict[str, object]
    runtime_root: Path
    runtime_root_id: tuple[int, int]
    custody_root: Path
    custody_root_id: tuple[int, int]
    launch_parent: Path
    launch_parent_id: tuple[int, int]
    summary: dict[str, object]
    remaining_uses: int
    launch_roots: dict[Path, tuple[int, int]] = field(default_factory=dict)
    active_launches: int = 0
    retired: bool = False
    cleanup_started: bool = False


def canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def sha256_bytes(value: bytes) -> str:
    return f"sha256:{hashlib.sha256(value).hexdigest()}"


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ReleaseFailure("P56-DUPLICATE-JSON-MEMBER")
        result[key] = value
    return result


def _is_reparse_or_link(metadata: os.stat_result) -> bool:
    if stat.S_ISLNK(metadata.st_mode):
        return True
    attribute = getattr(stat, "FILE_ATTRIBUTE_REPARSE_POINT", 0)
    return bool(attribute and getattr(metadata, "st_file_attributes", 0) & attribute)


def _lexists(path: Path) -> bool:
    return os.path.lexists(path)


def _file_id(metadata: os.stat_result) -> tuple[int, int]:
    return (metadata.st_dev, metadata.st_ino)


def _read_regular_once(path: Path, code: str, maximum: int | None = None) -> tuple[bytes, Binding]:
    """Read, size, and hash one regular file through exactly one descriptor."""

    try:
        before = os.lstat(path)
        if _is_reparse_or_link(before) or not stat.S_ISREG(before.st_mode):
            raise ReleaseFailure(code)
        descriptor = os.open(
            path, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except ReleaseFailure:
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if (
            _is_reparse_or_link(opened)
            or not stat.S_ISREG(opened.st_mode)
            or _file_id(before) != _file_id(opened)
        ):
            raise ReleaseFailure(code)
        content = bytearray()
        digest = hashlib.sha256()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if maximum is not None and len(content) > maximum:
                raise ReleaseFailure(code)
            digest.update(chunk)
        binding = Binding(path.name, len(content), f"sha256:{digest.hexdigest()}")
        return bytes(content), binding
    finally:
        os.close(descriptor)


def _safe_regular_digest(path: Path, code: str) -> tuple[int, str]:
    _content, binding = _read_regular_once(path, code)
    return binding.size, binding.sha256


def _safe_existing_directory(path: Path, code: str) -> Path:
    try:
        if not path.is_absolute() or ".." in path.parts:
            raise ReleaseFailure(code)
        probe = path
        while True:
            metadata = os.lstat(probe)
            if _is_reparse_or_link(metadata) or not stat.S_ISDIR(metadata.st_mode):
                raise ReleaseFailure(code)
            if probe == probe.parent:
                break
            probe = probe.parent
        return path.resolve(strict=True)
    except ReleaseFailure:
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error


def _fresh_child(parent: Path, name: str, code: str) -> Path:
    if not name or "/" in name or "\\" in name or name in {".", ".."}:
        raise ReleaseFailure(code)
    candidate = parent / name
    if _lexists(candidate):
        raise ReleaseFailure(code)
    return candidate


def _mkdir_exclusive(path: Path, code: str, mode: int = 0o700) -> tuple[int, int]:
    try:
        os.mkdir(path, mode)
        metadata = os.lstat(path)
        if _is_reparse_or_link(metadata) or not stat.S_ISDIR(metadata.st_mode):
            raise ReleaseFailure(code)
        return _file_id(metadata)
    except ReleaseFailure:
        if _lexists(path):
            _remove_tree(path)
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error


def _remove_tree(path: Path) -> None:
    """Remove only the given tree and never follow a reparse point or symlink."""

    if not _lexists(path):
        return
    metadata = os.lstat(path)
    if _is_reparse_or_link(metadata) or not stat.S_ISDIR(metadata.st_mode):
        try:
            os.chmod(path, stat.S_IWRITE | stat.S_IREAD)
        except OSError:
            pass
        os.unlink(path)
        return
    with os.scandir(path) as entries:
        for entry in entries:
            _remove_tree(Path(entry.path))
    os.rmdir(path)


def _remove_owned_tree(path: Path, expected_id: tuple[int, int], code: str) -> None:
    """Refuse a substituted root instead of recursively deleting it."""

    try:
        metadata = os.lstat(path)
    except FileNotFoundError:
        raise ReleaseFailure(code)
    except OSError as error:
        raise ReleaseFailure(code) from error
    if _is_reparse_or_link(metadata) or not stat.S_ISDIR(metadata.st_mode) or _file_id(metadata) != expected_id:
        raise ReleaseFailure(code)
    _remove_tree(path)


def _sync_directory(path: Path) -> str:
    flags = os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_DIRECTORY", 0)
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        if error.errno in {getattr(os, "EINVAL", 22), getattr(os, "EPERM", 1), getattr(os, "EACCES", 13)}:
            return "unsupported"
        raise ReleaseFailure("P56-DIRECTORY-SYNC") from error
    try:
        os.fsync(descriptor)
    except OSError as error:
        if error.errno in {getattr(os, "EINVAL", 22), getattr(os, "EPERM", 1), getattr(os, "EACCES", 13)}:
            return "unsupported"
        raise ReleaseFailure("P56-DIRECTORY-SYNC") from error
    finally:
        os.close(descriptor)
    return "synced"


def _write_bytes_exclusive(path: Path, content: bytes, code: str, mode: int) -> None:
    try:
        descriptor = os.open(
            path,
            os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
            mode,
        )
    except OSError as error:
        raise ReleaseFailure(code) from error
    try:
        offset = 0
        while offset < len(content):
            written = os.write(descriptor, content[offset:])
            if written <= 0:
                raise ReleaseFailure(code)
            offset += written
        os.fsync(descriptor)
    except ReleaseFailure:
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error
    finally:
        os.close(descriptor)


def _inside(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
        return True
    except ValueError:
        return False


def _run(command: list[str], code: str, env: dict[str, str] | None = None) -> bytes:
    completed = subprocess.run(command, env=env, capture_output=True, check=False)
    output = completed.stdout + completed.stderr
    if completed.returncode != 0 or not output:
        raise ReleaseFailure(code)
    return output


def _tool_binding(path: Path, command: list[str], code: str, env: dict[str, str]) -> dict[str, object]:
    try:
        identity_path = path.resolve(strict=True)
    except OSError as error:
        raise ReleaseFailure(code) from error
    _content, binding = _read_regular_once(identity_path, code)
    return {
        "command_identity_sha256": sha256_bytes(_run(command, code, env)),
        "file_sha256": binding.sha256,
        "path_role": "absolute-path-verified-privately",
        "size": binding.size,
    }


def _tree_identity(root: Path, code: str) -> dict[str, object]:
    root = _safe_existing_directory(root, code)
    entries: list[Binding] = []
    for path in sorted(root.rglob("*"), key=lambda item: item.relative_to(root).as_posix()):
        metadata = os.lstat(path)
        if _is_reparse_or_link(metadata):
            raise ReleaseFailure(code)
        if stat.S_ISDIR(metadata.st_mode):
            continue
        if not stat.S_ISREG(metadata.st_mode):
            raise ReleaseFailure(code)
        _content, binding = _read_regular_once(path, code)
        entries.append(Binding(path.relative_to(root).as_posix(), binding.size, binding.sha256))
    digest = hashlib.sha256()
    for entry in entries:
        digest.update(entry.path.encode("utf-8"))
        digest.update(b"\0")
        digest.update(str(entry.size).encode("ascii"))
        digest.update(b"\0")
        digest.update(entry.sha256.encode("ascii"))
        digest.update(b"\n")
    return {
        "aggregate": f"sha256:{digest.hexdigest()}",
        "file_count": len(entries),
        "total_bytes": sum(entry.size for entry in entries),
    }


def _host_platform() -> str:
    if os.name == "nt":
        return "windows-x86_64"
    if sys.platform == "linux":
        return "ubuntu-24.04-x86_64"
    raise ReleaseFailure("P56-UNSUPPORTED-HOST")


def _find_tool(name: str, code: str) -> Path:
    found = shutil.which(name)
    if found is None:
        suffix = ".exe" if os.name == "nt" else ""
        fallback = Path.home() / ".cargo" / "bin" / f"{name}{suffix}"
        if fallback.is_file():
            found = os.fspath(fallback)
    if found is None:
        raise ReleaseFailure(code)
    try:
        return Path(os.path.abspath(found)).resolve(strict=True)
    except OSError as error:
        raise ReleaseFailure(code) from error


def _environment_identity(values: dict[str, str]) -> dict[str, object]:
    return {
        "algorithm": "sha256-name-value-v1",
        "values": [
            {
                "name": name,
                "value_sha256": sha256_bytes(value.encode("utf-8")),
                "value_utf8_bytes": len(value.encode("utf-8")),
            }
            for name, value in sorted(values.items())
        ],
    }


def _build_environment(checkout: Path, target: Path, runtime: Path) -> dict[str, str]:
    """Allowlist all discovery and build inputs; do not inherit tool overrides."""

    cargo_home = os.environ.get("CARGO_HOME", os.fspath(Path.home() / ".cargo"))
    rustup_home = os.environ.get("RUSTUP_HOME", os.fspath(Path.home() / ".rustup"))
    if os.name == "nt":
        required = [
            "APPDATA", "COMSPEC", "INCLUDE", "LIB", "LIBPATH", "LOCALAPPDATA", "PATHEXT",
            "PROCESSOR_ARCHITECTURE", "ProgramData", "SystemRoot", "TEMP", "TMP",
            "UCRTVersion", "UniversalCRTSdkDir", "USERPROFILE", "VCToolsInstallDir",
            "WindowsSdkDir", "WindowsSDKVersion",
        ]
        selected = {name: os.environ[name] for name in required if name in os.environ}
        selected["PATH"] = os.environ.get("PATH", "")
    else:
        selected = {"HOME": os.fspath(runtime / "home"), "PATH": "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"}
    selected.update(
        {
            "CARGO_HOME": cargo_home,
            "CARGO_INCREMENTAL": "0",
            "CARGO_TARGET_DIR": os.fspath(target),
            "GIT_ATTR_NOSYSTEM": "1",
            "GIT_CONFIG_GLOBAL": os.fspath(runtime / "empty-gitconfig"),
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_TERMINAL_PROMPT": "0",
            "RUSTUP_HOME": rustup_home,
        }
    )
    return selected


def _public_environment(environment: dict[str, str], checkout: Path, target: Path, runtime: Path) -> dict[str, object]:
    return _environment_identity(
        {
            name: value.replace(os.fspath(checkout), "$CHECKOUT")
            .replace(os.fspath(target), "$TARGET")
            .replace(os.fspath(runtime), "$RUNTIME")
            for name, value in environment.items()
        }
    )


def _direct_toolchain(
    checkout: Path, target: Path, runtime: Path
) -> tuple[Path, Path, Path, str, dict[str, str], dict[str, object]]:
    environment = _build_environment(checkout, target, runtime)
    rustup = _find_tool("rustup", "P56-RUSTUP-UNAVAILABLE")
    active = _run([os.fspath(rustup), "show", "active-toolchain", "-v"], "P56-TOOLCHAIN", environment)
    try:
        toolchain = active.decode("utf-8").splitlines()[0].split()[0]
    except (IndexError, UnicodeDecodeError) as error:
        raise ReleaseFailure("P56-TOOLCHAIN") from error
    if not toolchain:
        raise ReleaseFailure("P56-TOOLCHAIN")
    environment["RUSTUP_TOOLCHAIN"] = toolchain
    cargo = Path(
        _run(
            [os.fspath(rustup), "which", "--toolchain", toolchain, "cargo"],
            "P56-TOOLCHAIN",
            environment,
        ).decode("utf-8").strip()
    ).resolve(strict=True)
    rustc = Path(
        _run(
            [os.fspath(rustup), "which", "--toolchain", toolchain, "rustc"],
            "P56-TOOLCHAIN",
            environment,
        ).decode("utf-8").strip()
    ).resolve(strict=True)
    cargo_proxy = _find_tool("cargo", "P56-CARGO-UNAVAILABLE")
    rustc_proxy = _find_tool("rustc", "P56-RUSTC-UNAVAILABLE")
    for direct, proxy in ((cargo, cargo_proxy), (rustc, rustc_proxy)):
        _direct_bytes, direct_binding = _read_regular_once(direct, "P56-TOOLCHAIN")
        _proxy_bytes, proxy_binding = _read_regular_once(proxy, "P56-TOOLCHAIN")
        if direct == proxy or direct_binding.sha256 == proxy_binding.sha256:
            raise ReleaseFailure("P56-TOOLCHAIN-PROXY")
    environment["PATH"] = os.pathsep.join([os.fspath(cargo.parent), environment["PATH"]])
    return cargo, rustc, rustup, toolchain, environment, _public_environment(
        environment, checkout, target, runtime
    )


def _git_binding(git: Path, environment: dict[str, str]) -> dict[str, object]:
    return {
        "binary": _tool_binding(git, [os.fspath(git), "--version"], "P56-GIT-IDENTITY", environment),
        "version_sha256": sha256_bytes(_run([os.fspath(git), "--version"], "P56-GIT-IDENTITY", environment)),
    }


def _git_command(git: Path, checkout: Path, hooks: Path, command: list[str]) -> list[str]:
    return [
        os.fspath(git),
        "-c", "core.autocrlf=false",
        "-c", "core.eol=lf",
        "-c", f"core.hooksPath={hooks}",
        "-c", "filter.lfs.clean=",
        "-c", "filter.lfs.smudge=",
        "-c", "filter.lfs.process=",
        "-c", "filter.lfs.required=false",
        "-C", os.fspath(checkout),
        *command,
    ]


def _fresh_checkout(source: Path, checkout: Path, git: Path, environment: dict[str, str], runtime: Path) -> None:
    hooks = runtime / "empty-hooks"
    if not hooks.exists():
        _mkdir_exclusive(hooks, "P56-CHECKOUT")
    else:
        _safe_existing_directory(hooks, "P56-CHECKOUT")
    clone = [
        os.fspath(git),
        "-c", "core.autocrlf=false",
        "-c", "core.eol=lf",
        "-c", f"core.hooksPath={hooks}",
        "-c", "filter.lfs.clean=",
        "-c", "filter.lfs.smudge=",
        "-c", "filter.lfs.process=",
        "-c", "filter.lfs.required=false",
        "clone", "--no-checkout", "--config", "core.autocrlf=false",
        os.fspath(source), os.fspath(checkout),
    ]
    if subprocess.run(clone, env=environment, capture_output=True, check=False).returncode != 0:
        raise ReleaseFailure("P56-CHECKOUT")
    commands = (
        ["config", "--local", "core.autocrlf", "false"],
        ["config", "--local", "core.hooksPath", os.fspath(hooks)],
        ["checkout", "--detach", "--no-recurse-submodules", CUTOFF],
    )
    for command in commands:
        if subprocess.run(
            _git_command(git, checkout, hooks, command), env=environment, capture_output=True, check=False
        ).returncode != 0:
            raise ReleaseFailure("P56-CHECKOUT")
    config = subprocess.run(_git_command(git, checkout, hooks, ["config", "--get", "core.autocrlf"]), env=environment, capture_output=True)
    status = subprocess.run(_git_command(git, checkout, hooks, ["status", "--porcelain"]), env=environment, capture_output=True)
    head = subprocess.run(_git_command(git, checkout, hooks, ["rev-parse", "HEAD"]), env=environment, capture_output=True)
    if (
        config.returncode != 0
        or config.stdout.strip().lower() != b"false"
        or status.returncode != 0
        or status.stdout
        or head.returncode != 0
        or head.stdout.decode("ascii", "ignore").strip() != CUTOFF
    ):
        raise ReleaseFailure("P56-CHECKOUT")


def _valid_direct_path(path: Path, code: str) -> Path:
    try:
        resolved = path.resolve(strict=True)
        _content, binding = _read_regular_once(resolved, code)
        if binding.size < 1:
            raise ReleaseFailure(code)
        return resolved
    except ReleaseFailure:
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error


def _tool_output_path(raw: bytes, code: str, environment: dict[str, str]) -> Path:
    try:
        candidate = Path(raw.decode("utf-8").strip())
    except UnicodeDecodeError as error:
        raise ReleaseFailure(code) from error
    if not candidate.is_absolute():
        found = shutil.which(os.fspath(candidate), path=environment["PATH"])
        if found is None:
            raise ReleaseFailure(code)
        candidate = Path(found)
    return _valid_direct_path(candidate, code)


def _toolchain_and_controls(
    platform: str, checkout: Path, target: Path, runtime: Path
) -> tuple[Path, dict[str, object], dict[str, str]]:
    cargo, rustc, rustup, selected, environment, environment_identity = _direct_toolchain(
        checkout, target, runtime
    )
    sysroot = _safe_existing_directory(
        Path(_run([os.fspath(rustc), "--print", "sysroot"], "P56-TOOLCHAIN", environment).decode("utf-8").strip()),
        "P56-TOOLCHAIN",
    )
    target_libdir = _safe_existing_directory(
        Path(_run([os.fspath(rustc), "--print", "target-libdir"], "P56-TOOLCHAIN", environment).decode("utf-8").strip()),
        "P56-TOOLCHAIN",
    )
    if not _inside(target_libdir, sysroot):
        raise ReleaseFailure("P56-TOOLCHAIN")
    git = _find_tool("git", "P56-GIT-UNAVAILABLE")
    rustc_verbose = _run([os.fspath(rustc), "-vV"], "P56-TOOLCHAIN", environment).decode("utf-8")
    host_lines = [line for line in rustc_verbose.splitlines() if line.startswith("host: ")]
    if len(host_lines) != 1:
        raise ReleaseFailure("P56-TOOLCHAIN")
    if platform == "windows-x86_64":
        linker = _valid_direct_path(
            sysroot / "lib" / "rustlib" / "x86_64-pc-windows-msvc" / "bin" / "rust-lld.exe",
            "P56-LINKER-ROUTE",
        )
        linker_record: dict[str, object] = {
            "driver": _tool_binding(linker, [os.fspath(linker), "-flavor", "link", "/?"], "P56-LINKER-IDENTITY", environment),
            "path_under_sysroot": "lib/rustlib/x86_64-pc-windows-msvc/bin/rust-lld.exe",
            "route": "rust-toolchain-shipped-rust-lld",
        }
        linker_flags = [
            "-C", f"linker={linker}", "-C", "link-arg=/Brepro", "-C", "link-arg=/timestamp:0",
            "-C", "link-arg=/debug:none",
        ]
        public_linker_flags = [
            "-C", "linker=$SYSROOT/lib/rustlib/x86_64-pc-windows-msvc/bin/rust-lld.exe",
            "-C", "link-arg=/Brepro", "-C", "link-arg=/timestamp:0", "-C", "link-arg=/debug:none",
        ]
        route = "rust-toolchain-shipped-rust-lld"
    else:
        cc = _valid_direct_path(Path("/usr/bin/cc"), "P56-LINKER-ROUTE")
        collect2 = _tool_output_path(
            _run([os.fspath(cc), "-print-prog-name=collect2"], "P56-LINKER-IDENTITY", environment),
            "P56-LINKER-IDENTITY", environment,
        )
        ld = _tool_output_path(
            _run([os.fspath(cc), "-print-prog-name=ld"], "P56-LINKER-IDENTITY", environment),
            "P56-LINKER-IDENTITY", environment,
        )
        startup: list[dict[str, object]] = []
        for name in ("Scrt1.o", "crti.o", "crtbeginS.o", "crtendS.o", "crtn.o"):
            item = _valid_direct_path(
                Path(_run([os.fspath(cc), f"-print-file-name={name}"], "P56-LINKER-IDENTITY", environment).decode("utf-8").strip()),
                "P56-LINKER-IDENTITY",
            )
            startup.append({"name": name, "identity": _tool_binding(item, [os.fspath(cc), f"-print-file-name={name}"], "P56-LINKER-IDENTITY", environment)})
        linker_record = {
            "cc_driver": _tool_binding(cc, [os.fspath(cc), "--version"], "P56-LINKER-IDENTITY", environment),
            "collect2": _tool_binding(collect2, [os.fspath(collect2), "--version"], "P56-LINKER-IDENTITY", environment),
            "gnu_ld": _tool_binding(ld, [os.fspath(ld), "--version"], "P56-LINKER-IDENTITY", environment),
            "gnu_ld_search_identity_sha256": sha256_bytes(_run([os.fspath(ld), "--verbose"], "P56-LINKER-IDENTITY", environment)),
            "route": "bound-ubuntu-cc-collect2-gnu-ld-trace",
            "startup_objects": startup,
        }
        linker_flags = [
            "-C", f"linker={cc}", "-C", "link-arg=-Wl,--build-id=sha1",
            "-C", "link-arg=-Wl,-t", "-C", "debuginfo=0",
        ]
        public_linker_flags = linker_flags
        route = "bound-ubuntu-cc-collect2-gnu-ld-trace"
    rustflags = f"--remap-path-prefix={checkout}=/ferris " + " ".join(
        f"{linker_flags[index]} {linker_flags[index + 1]}" for index in range(0, len(linker_flags), 2)
    )
    environment.update({"RUSTC": os.fspath(rustc), "RUSTFLAGS": rustflags})
    binding = {
        "checkout": {"core_autocrlf": False, "exact_commit": CUTOFF, "fresh_clean_checkout": True},
        "controls": {
            "cargo_incremental": "0",
            "command": ["cargo", "build", "--release", "--locked", "--package", "ferris-cli", "--bin", "ferris", "--message-format=json"],
            "linker_route": route,
            "linker_rustflags": ["--remap-path-prefix=$CHECKOUT=/ferris", *public_linker_flags],
            "remap_path_prefix": "$CHECKOUT=/ferris",
        },
        "git": _git_binding(git, environment),
        "toolchain": {
            "cargo_direct": _tool_binding(cargo, [os.fspath(cargo), "-V"], "P56-TOOLCHAIN", environment),
            "environment": _public_environment(environment, checkout, target, runtime),
            "host": host_lines[0].removeprefix("host: "),
            "linker": linker_record,
            "rustc_direct": _tool_binding(rustc, [os.fspath(rustc), "-vV"], "P56-TOOLCHAIN", environment),
            "rustup_selector": _tool_binding(rustup, [os.fspath(rustup), "--version"], "P56-TOOLCHAIN", environment),
            "selected_toolchain": selected,
            "sysroot_target_libdir": target_libdir.relative_to(sysroot).as_posix(),
            "target_libdir_tree": _tree_identity(target_libdir, "P56-TOOLCHAIN"),
        },
    }
    return cargo, binding, environment


def _ubuntu_trace_inputs(output: bytes, checkout: Path, target: Path) -> list[dict[str, object]]:
    """Bind actual GNU-ld trace-selected startup/system inputs, not route labels."""

    found: set[Path] = set()
    for candidate in re.findall(rb"/[A-Za-z0-9_./+-]+", output):
        try:
            path = Path(candidate.decode("utf-8")).resolve(strict=True)
            if _inside(path, checkout) or _inside(path, target):
                continue
            metadata = os.lstat(path)
            if stat.S_ISREG(metadata.st_mode) and not _is_reparse_or_link(metadata):
                found.add(path)
        except (OSError, UnicodeDecodeError):
            continue
    entries: list[dict[str, object]] = []
    for path in sorted(found, key=os.fspath):
        _content, binding = _read_regular_once(path, "P56-LINKER-TRACE")
        entries.append(
            {
                "identity": {"file_sha256": binding.sha256, "size": binding.size},
                "role": "actual-ld-trace-selected-input",
            }
        )
    if len(entries) < 3:
        raise ReleaseFailure("P56-LINKER-TRACE")
    return entries


def _source_root() -> Path:
    return _safe_existing_directory(Path(__file__).resolve().parents[4], "P56-SOURCE-REPOSITORY")


def _build_one(
    platform: str, work: Path, runtime: Path, label: str
) -> tuple[bytes, Binding, dict[str, object]]:
    checkout = _fresh_child(work, f"checkout-{label}", "P56-WORK-ROOT")
    target = _fresh_child(work, f"target-{label}", "P56-WORK-ROOT")
    _mkdir_exclusive(target, "P56-WORK-ROOT")
    cargo, binding, environment = _toolchain_and_controls(platform, checkout, target, runtime)
    git = _find_tool("git", "P56-GIT-UNAVAILABLE")
    _fresh_checkout(_source_root(), checkout, git, environment, runtime)
    command = [os.fspath(cargo), "build", "--release", "--locked", "--package", "ferris-cli", "--bin", "ferris", "--message-format=json"]
    completed = subprocess.run(command, cwd=checkout, env=environment, capture_output=True, check=False)
    if completed.returncode != 0:
        raise ReleaseFailure("P56-BUILD")
    executable: Path | None = None
    try:
        for line in completed.stdout.decode("utf-8").splitlines():
            message = json.loads(line)
            target_data = message.get("target")
            if (
                message.get("reason") == "compiler-artifact"
                and type(target_data) is dict
                and target_data.get("name") == "ferris"
                and target_data.get("kind") == ["bin"]
                and type(message.get("executable")) is str
            ):
                executable = Path(message["executable"])
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ReleaseFailure("P56-ARTIFACT-DISCOVERY") from error
    if executable is None or not _inside(executable.resolve(strict=True), target):
        raise ReleaseFailure("P56-ARTIFACT-DISCOVERY")
    artifact_bytes, actual = _read_regular_once(executable, "P56-ARTIFACT")
    suffix = ".exe" if platform == "windows-x86_64" else ""
    artifact = Binding(f"ferris-{platform}-{CUTOFF}{suffix}", actual.size, actual.sha256)
    if platform == "ubuntu-24.04-x86_64":
        linker = binding["toolchain"]["linker"]
        assert type(linker) is dict
        linker["actual_trace_selected_inputs"] = _ubuntu_trace_inputs(
            completed.stdout + completed.stderr, checkout, target
        )
    return artifact_bytes, artifact, binding


def _receipt(
    platform: str, artifact: Binding, first: dict[str, object], second: dict[str, object]
) -> tuple[bytes, Binding, dict[str, object]]:
    if first != second:
        raise ReleaseFailure("P56-TOOLCHAIN-DRIFT")
    payload = {
        "artifact": {"logical_filename": artifact.path, "retained": True, "sha256": artifact.sha256, "size": artifact.size},
        "build": first,
        "cutoff": CUTOFF,
        "platform": platform,
        "reproducibility": {
            "artifact_bytes_identical": True, "builds": 2, "distinct_checkout_roots": True,
            "distinct_target_roots": True, "evidence": [artifact.public(), artifact.public()],
        },
        "safety": {
            "diagnostic_execution": False, "ferris_executed": False, "product_files_modified": False,
            "public_receipt_is_evidence_only": True,
        },
        "schema": RECEIPT_SCHEMA,
    }
    identity = sha256_bytes(canonical_bytes(payload))
    document = {"payload": payload, "payload_sha256": identity, "receipt_id": identity, "schema": RECEIPT_ENVELOPE_SCHEMA}
    content = canonical_bytes(document) + b"\n"
    return content, Binding(f"{artifact.path}.receipt.json", len(content), sha256_bytes(content)), document


def _valid_sha256(value: object) -> bool:
    return type(value) is str and bool(re.fullmatch(r"sha256:[0-9a-f]{64}", value))


def _valid_tool_identity(value: object) -> bool:
    return (
        type(value) is dict
        and set(value) == {"command_identity_sha256", "file_sha256", "path_role", "size"}
        and _valid_sha256(value["command_identity_sha256"])
        and _valid_sha256(value["file_sha256"])
        and value["path_role"] == "absolute-path-verified-privately"
        and type(value["size"]) is int
        and value["size"] > 0
    )


def _valid_build_binding(value: object, platform: str) -> bool:
    if type(value) is not dict or set(value) != {"checkout", "controls", "git", "toolchain"}:
        return False
    if value["checkout"] != {"core_autocrlf": False, "exact_commit": CUTOFF, "fresh_clean_checkout": True}:
        return False
    controls, git, toolchain = value["controls"], value["git"], value["toolchain"]
    if (
        type(controls) is not dict
        or controls.get("command") != ["cargo", "build", "--release", "--locked", "--package", "ferris-cli", "--bin", "ferris", "--message-format=json"]
        or controls.get("cargo_incremental") != "0"
        or controls.get("remap_path_prefix") != "$CHECKOUT=/ferris"
        or not isinstance(controls.get("linker_rustflags"), list)
        or type(git) is not dict
        or set(git) != {"binary", "version_sha256"}
        or not _valid_tool_identity(git["binary"])
        or not _valid_sha256(git["version_sha256"])
        or type(toolchain) is not dict
    ):
        return False
    required = {
        "cargo_direct", "environment", "host", "linker", "rustc_direct", "rustup_selector",
        "selected_toolchain", "sysroot_target_libdir", "target_libdir_tree",
    }
    if set(toolchain) != required or not _valid_tool_identity(toolchain["cargo_direct"]) or not _valid_tool_identity(toolchain["rustc_direct"]) or not _valid_tool_identity(toolchain["rustup_selector"]):
        return False
    if type(toolchain["selected_toolchain"]) is not str or not toolchain["selected_toolchain"]:
        return False
    environment = toolchain["environment"]
    tree = toolchain["target_libdir_tree"]
    if (
        type(environment) is not dict or set(environment) != {"algorithm", "values"}
        or environment["algorithm"] != "sha256-name-value-v1" or type(environment["values"]) is not list
        or not any(item.get("name") == "RUSTUP_TOOLCHAIN" for item in environment["values"] if type(item) is dict)
        or type(tree) is not dict or set(tree) != {"aggregate", "file_count", "total_bytes"}
        or not _valid_sha256(tree["aggregate"]) or type(tree["file_count"]) is not int
    ):
        return False
    linker = toolchain["linker"]
    if platform == "windows-x86_64":
        return (
            controls.get("linker_route") == "rust-toolchain-shipped-rust-lld"
            and type(linker) is dict
            and set(linker) == {"driver", "path_under_sysroot", "route"}
            and linker["route"] == "rust-toolchain-shipped-rust-lld"
            and _valid_tool_identity(linker["driver"])
        )
    return (
        controls.get("linker_route") == "bound-ubuntu-cc-collect2-gnu-ld-trace"
        and type(linker) is dict
        and set(linker) == {
            "actual_trace_selected_inputs", "cc_driver", "collect2", "gnu_ld",
            "gnu_ld_search_identity_sha256", "route", "startup_objects",
        }
        and linker["route"] == "bound-ubuntu-cc-collect2-gnu-ld-trace"
        and all(_valid_tool_identity(linker[name]) for name in ("cc_driver", "collect2", "gnu_ld"))
        and _valid_sha256(linker["gnu_ld_search_identity_sha256"])
        and type(linker["startup_objects"]) is list and len(linker["startup_objects"]) == 5
        and type(linker["actual_trace_selected_inputs"]) is list and len(linker["actual_trace_selected_inputs"]) >= 3
    )


def _parse_receipt_bytes(content: bytes, code: str) -> dict[str, object]:
    try:
        document = json.loads(content, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, ReleaseFailure) as error:
        raise ReleaseFailure(code) from error
    if type(document) is not dict:
        raise ReleaseFailure(code)
    return document


def _verify_receipt_and_pair(root: Path, platform: str) -> tuple[Binding, Binding, dict[str, object]]:
    root = _safe_existing_directory(root, "P56-CUSTODY-VERIFY")
    suffix = ".exe" if platform == "windows-x86_64" else ""
    artifact_name = f"ferris-{platform}-{CUTOFF}{suffix}"
    receipt_name = f"{artifact_name}.receipt.json"
    try:
        names = sorted(entry.name for entry in os.scandir(root))
    except OSError as error:
        raise ReleaseFailure("P56-CUSTODY-VERIFY") from error
    if names != [artifact_name, receipt_name]:
        raise ReleaseFailure("P56-CUSTODY-VERIFY")
    artifact_bytes, measured_artifact = _read_regular_once(root / artifact_name, "P56-CUSTODY-VERIFY")
    receipt_bytes, measured_receipt = _read_regular_once(
        root / receipt_name, "P56-CUSTODY-VERIFY", MAX_RECEIPT_BYTES
    )
    artifact = Binding(artifact_name, measured_artifact.size, measured_artifact.sha256)
    receipt = Binding(receipt_name, measured_receipt.size, measured_receipt.sha256)
    document = _parse_receipt_bytes(receipt_bytes, "P56-CUSTODY-VERIFY")
    if set(document) != {"payload", "payload_sha256", "receipt_id", "schema"} or document["schema"] != RECEIPT_ENVELOPE_SCHEMA:
        raise ReleaseFailure("P56-CUSTODY-VERIFY")
    payload = document["payload"]
    if (
        type(payload) is not dict
        or document["payload_sha256"] != sha256_bytes(canonical_bytes(payload))
        or document["receipt_id"] != document["payload_sha256"]
        or set(payload) != {"artifact", "build", "cutoff", "platform", "reproducibility", "safety", "schema"}
        or payload["schema"] != RECEIPT_SCHEMA
        or payload["cutoff"] != CUTOFF
        or payload["platform"] != platform
        or not _valid_build_binding(payload["build"], platform)
        or payload["artifact"] != {"logical_filename": artifact_name, "retained": True, "sha256": artifact.sha256, "size": artifact.size}
        or payload["reproducibility"] != {
            "artifact_bytes_identical": True, "builds": 2, "distinct_checkout_roots": True,
            "distinct_target_roots": True, "evidence": [artifact.public(), artifact.public()],
        }
        or payload["safety"] != {
            "diagnostic_execution": False, "ferris_executed": False, "product_files_modified": False,
            "public_receipt_is_evidence_only": True,
        }
    ):
        raise ReleaseFailure("P56-CUSTODY-VERIFY")
    if sha256_bytes(artifact_bytes) != artifact.sha256:
        raise ReleaseFailure("P56-CUSTODY-VERIFY")
    return artifact, receipt, document


def _publish_two_file_custody(
    work: Path, final: Path, platform: str, artifact_bytes: bytes, artifact: Binding,
    receipt_bytes: bytes, receipt: Binding,
) -> dict[str, object]:
    stage = _fresh_child(final.parent, f".{final.name}.stage", "P56-STAGE-ROOT")
    _mkdir_exclusive(stage, "P56-STAGE-ROOT")
    renamed = False
    try:
        _write_bytes_exclusive(stage / artifact.path, artifact_bytes, "P56-STAGE-COPY", 0o500)
        _write_bytes_exclusive(stage / receipt.path, receipt_bytes, "P56-STAGE-COPY", 0o400)
        staged_artifact, staged_receipt, _ = _verify_receipt_and_pair(stage, platform)
        if (staged_artifact, staged_receipt) != (artifact, receipt):
            raise ReleaseFailure("P56-STAGE-VERIFY")
        _remove_tree(work)
        if _lexists(work):
            raise ReleaseFailure("P56-WORK-CLEANUP")
        stage_sync = _sync_directory(stage)
        os.replace(stage, final)
        renamed = True
        final_artifact, final_receipt, document = _verify_receipt_and_pair(final, platform)
        if (final_artifact, final_receipt) != (artifact, receipt):
            raise ReleaseFailure("P56-FINAL-VERIFY")
        parent_sync = _sync_directory(final.parent)
        payload = {
            "artifacts": {"artifact": final_artifact.public(), "receipt": final_receipt.public(), "receipt_id": document["receipt_id"]},
            "custody": {
                "files": "2/2", "final_verified": "2/2", "rename_attempts": 1, "retries": 0,
                "stage_verified": "2/2", "sync": {"final_parent": parent_sync, "mechanism": SYNC_MECHANISM, "stage": stage_sync},
                "work_cleanup": "absent",
            },
            "schema": SUMMARY_SCHEMA,
        }
        return {"completion_id": sha256_bytes(canonical_bytes(payload)), **payload}
    except Exception as error:
        try:
            if _lexists(stage):
                _remove_tree(stage)
            if renamed and _lexists(final):
                _remove_tree(final)
            if _lexists(work):
                _remove_tree(work)
            _sync_directory(final.parent)
        except Exception as cleanup_error:
            raise ReleaseFailure("P56-INDETERMINATE-CLEANUP") from cleanup_error
        if isinstance(error, ReleaseFailure):
            raise
        raise ReleaseFailure("P56-CUSTODY") from error


def _live_record(handle: object) -> _LiveCustody:
    if type(handle) is not CustodyHandle:
        raise ReleaseFailure("P56-HANDLE-FORGERY")
    record = _LIVE_HANDLES.get(handle)
    if record is None or not secrets.compare_digest(handle._CustodyHandle__token, record.token):
        raise ReleaseFailure("P56-HANDLE-EXPIRED")
    return record


def publish_retained_build_and_custody(platform: str, runtime_parent: str | os.PathLike[str]) -> CustodyHandle:
    """Build twice and return an opaque live capability, never a launchable path."""

    if platform not in PLATFORMS or platform != _host_platform():
        raise ReleaseFailure("P56-UNSUPPORTED-PLATFORM")
    parent = _safe_existing_directory(Path(os.fspath(runtime_parent)), "P56-RUNTIME-PARENT")
    run = _fresh_child(parent, f".p56-{secrets.token_hex(12)}", "P56-RUNTIME-ROOT")
    run_id = _mkdir_exclusive(run, "P56-RUNTIME-ROOT")
    work = run / "work"
    final = run / "custody"
    launch_parent = run / "launches"
    _mkdir_exclusive(work, "P56-WORK-ROOT")
    launch_parent_id = _mkdir_exclusive(launch_parent, "P56-LAUNCH-ROOT")
    _mkdir_exclusive(run / "home", "P56-RUNTIME-ROOT")
    _mkdir_exclusive(run / "tmp", "P56-RUNTIME-ROOT")
    try:
        first_bytes, first_artifact, first_binding = _build_one(platform, work, run, "a")
        second_bytes, second_artifact, second_binding = _build_one(platform, work, run, "b")
        if first_artifact != second_artifact or not secrets.compare_digest(first_bytes, second_bytes):
            raise ReleaseFailure("P56-REPRODUCIBILITY")
        receipt_bytes, receipt, receipt_document = _receipt(platform, first_artifact, first_binding, second_binding)
        summary = _publish_two_file_custody(work, final, platform, first_bytes, first_artifact, receipt_bytes, receipt)
        custody_id = _file_id(os.lstat(final))
        token = secrets.token_bytes(32)
        handle = CustodyHandle(_HANDLE_CONSTRUCTOR_KEY, token)
        record = _LiveCustody(
            token, platform, first_artifact, first_bytes, receipt, receipt_document, run, run_id,
            final, custody_id, launch_parent, launch_parent_id, summary, DEFAULT_LAUNCH_USES,
        )
        with _REGISTRY_LOCK:
            _LIVE_HANDLES[handle] = record
        return handle
    except Exception:
        if _lexists(run):
            _remove_owned_tree(run, run_id, "P56-RUNTIME-CLEANUP")
        raise


def verify_custody(custody_root: str | os.PathLike[str], platform: str) -> dict[str, object]:
    """Return public evidence only. It cannot create or recover a live handle."""

    if platform not in PLATFORMS:
        raise ReleaseFailure("P56-UNSUPPORTED-PLATFORM")
    artifact, receipt, document = _verify_receipt_and_pair(
        _safe_existing_directory(Path(os.fspath(custody_root)), "P56-CUSTODY-VERIFY"), platform
    )
    return {
        "artifact": artifact.public(), "receipt": receipt.public(), "receipt_id": document["receipt_id"],
        "receipt_authorizes_launch": False, "schema": HANDOFF_SCHEMA,
    }


def _native_linux_root(path: Path) -> None:
    if sys.platform != "linux":
        return
    try:
        lines = Path("/proc/self/mountinfo").read_text(encoding="utf-8").splitlines()
        resolved = os.fspath(path.resolve(strict=True))
        mounts: list[tuple[str, str]] = []
        for line in lines:
            before, separator, after = line.partition(" - ")
            if not separator:
                continue
            fields = before.split()
            trailing = after.split()
            if len(fields) >= 5 and trailing:
                mounts.append((fields[4].replace("\\040", " "), trailing[0]))
        mount, filesystem = max(
            ((point, kind) for point, kind in mounts if resolved == point or resolved.startswith(point.rstrip("/") + "/")),
            key=lambda pair: len(pair[0]),
        )
    except (OSError, ValueError):
        raise ReleaseFailure("P56-NATIVE-LINUX-LAUNCH-ROOT")
    if filesystem not in {"ext4", "xfs", "btrfs", "tmpfs", "overlay"}:
        raise ReleaseFailure("P56-NATIVE-LINUX-LAUNCH-ROOT")


def _child_environment(record: _LiveCustody) -> dict[str, str]:
    home = record.runtime_root / "home"
    temporary = record.runtime_root / "tmp"
    if os.name == "nt":
        system_root = os.environ.get("SystemRoot") or os.environ.get("SYSTEMROOT")
        if not system_root:
            raise ReleaseFailure("P56-LAUNCH-ENVIRONMENT")
        return {
            "ComSpec": os.environ.get("ComSpec", str(Path(system_root) / "System32" / "cmd.exe")),
            "HOME": os.fspath(home),
            "HOMEDRIVE": Path(home).drive or "C:",
            "HOMEPATH": "\\",
            "LANG": "C",
            "LC_ALL": "C",
            "PATH": str(Path(system_root) / "System32"),
            "SystemRoot": system_root,
            "TEMP": os.fspath(temporary),
            "TMP": os.fspath(temporary),
            "USERPROFILE": os.fspath(home),
        }
    return {
        "HOME": os.fspath(home), "LANG": "C.UTF-8", "LC_ALL": "C.UTF-8",
        "PATH": "/usr/bin:/bin", "TMPDIR": os.fspath(temporary),
    }


def _register_launch_root(record: _LiveCustody) -> Path:
    root = _fresh_child(record.launch_parent, f"l-{secrets.token_hex(12)}", "P56-LAUNCH-ROOT")
    _native_linux_root(record.launch_parent)
    record.launch_roots[root] = _mkdir_exclusive(root, "P56-LAUNCH-ROOT")
    return root


def _remove_registered_launch_root(record: _LiveCustody, root: Path) -> None:
    root_id = record.launch_roots.get(root)
    if root_id is None:
        raise ReleaseFailure("P56-LAUNCH-ROOT")
    _remove_owned_tree(root, root_id, "P56-LAUNCH-CLEANUP")
    del record.launch_roots[root]


def _validate_owned_tree(path: Path, expected_id: tuple[int, int], code: str) -> None:
    try:
        metadata = os.lstat(path)
    except OSError as error:
        raise ReleaseFailure(code) from error
    if _is_reparse_or_link(metadata) or not stat.S_ISDIR(metadata.st_mode) or _file_id(metadata) != expected_id:
        raise ReleaseFailure(code)


def _clean_live_custody(record: _LiveCustody) -> None:
    """Exact-clean registry-owned roots after the handle is already retired."""

    try:
        _validate_owned_tree(record.runtime_root, record.runtime_root_id, "P56-RUNTIME-CLEANUP")
        _validate_owned_tree(record.custody_root, record.custody_root_id, "P56-CUSTODY-CLEANUP")
        _validate_owned_tree(record.launch_parent, record.launch_parent_id, "P56-LAUNCH-CLEANUP")
        for root, root_id in record.launch_roots.items():
            _validate_owned_tree(root, root_id, "P56-LAUNCH-CLEANUP")
        for root in tuple(record.launch_roots):
            _remove_registered_launch_root(record, root)
        _remove_owned_tree(record.custody_root, record.custody_root_id, "P56-CUSTODY-CLEANUP")
        _remove_owned_tree(record.launch_parent, record.launch_parent_id, "P56-LAUNCH-CLEANUP")
        _remove_owned_tree(record.runtime_root, record.runtime_root_id, "P56-RUNTIME-CLEANUP")
    except Exception as error:
        raise ReleaseFailure("P56-INDETERMINATE-CLEANUP") from error


def _retire_locked(handle: CustodyHandle, record: _LiveCustody) -> None:
    record.retired = True
    if _LIVE_HANDLES.get(handle) is record:
        del _LIVE_HANDLES[handle]


def _take_terminal_cleanup_locked(handle: CustodyHandle, record: _LiveCustody) -> _LiveCustody | None:
    if record.active_launches != 0 or record.cleanup_started:
        return None
    if not record.retired and record.remaining_uses != 0:
        return None
    _retire_locked(handle, record)
    record.cleanup_started = True
    return record


def _complete_launch(handle: CustodyHandle, record: _LiveCustody, root: Path | None) -> None:
    """Account for one launch and make terminal cleanup fatal rather than success-shaped."""

    launch_cleanup_failed = False
    terminal: _LiveCustody | None = None
    with _REGISTRY_LOCK:
        try:
            if root is not None:
                _remove_registered_launch_root(record, root)
        except Exception:
            launch_cleanup_failed = True
            _retire_locked(handle, record)
        finally:
            record.active_launches -= 1
            if record.active_launches < 0:
                record.active_launches = 0
                launch_cleanup_failed = True
                _retire_locked(handle, record)
            terminal = _take_terminal_cleanup_locked(handle, record)
    terminal_cleanup_failed = False
    if terminal is not None:
        try:
            _clean_live_custody(terminal)
        except ReleaseFailure:
            terminal_cleanup_failed = True
    if launch_cleanup_failed or terminal_cleanup_failed:
        raise ReleaseFailure("P56-INDETERMINATE-CLEANUP")


def _hash_open_descriptor(descriptor: int, code: str) -> Binding:
    try:
        opened = os.fstat(descriptor)
        if _is_reparse_or_link(opened) or not stat.S_ISREG(opened.st_mode):
            raise ReleaseFailure(code)
        os.lseek(descriptor, 0, os.SEEK_SET)
        digest = hashlib.sha256()
        size = 0
        while chunk := os.read(descriptor, 65_536):
            digest.update(chunk)
            size += len(chunk)
        os.lseek(descriptor, 0, os.SEEK_SET)
        return Binding("", size, f"sha256:{digest.hexdigest()}")
    except ReleaseFailure:
        raise
    except OSError as error:
        raise ReleaseFailure(code) from error


def _open_linux_verified_image(path: Path, expected: Binding, content: bytes) -> tuple[int, str]:
    _write_bytes_exclusive(path, content, "P56-LAUNCH-COPY", 0o500)
    try:
        descriptor = os.open(path, os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0))
    except OSError as error:
        raise ReleaseFailure("P56-LAUNCH-OPEN") from error
    observed = _hash_open_descriptor(descriptor, "P56-LAUNCH-VERIFY")
    if (observed.size, observed.sha256) != (expected.size, expected.sha256):
        os.close(descriptor)
        raise ReleaseFailure("P56-LAUNCH-VERIFY")
    return descriptor, f"/proc/self/fd/{descriptor}"


def _close_unowned_windows_handle(kernel32: Any, handle: int) -> None:
    kernel32.CloseHandle(handle)


def _open_windows_verified_image(path: Path, expected: Binding, content: bytes) -> tuple[int, Callable[[], None]]:
    _write_bytes_exclusive(path, content, "P56-LAUNCH-COPY", 0o500)
    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    create_file = kernel32.CreateFileW
    create_file.argtypes = [ctypes.c_wchar_p, ctypes.c_ulong, ctypes.c_ulong, ctypes.c_void_p, ctypes.c_ulong, ctypes.c_ulong, ctypes.c_void_p]
    create_file.restype = ctypes.c_void_p
    handle = create_file(os.fspath(path), 0x80000000, 0x00000001, None, 3, 0x80, None)
    invalid = ctypes.c_void_p(-1).value
    if handle == invalid:
        raise ReleaseFailure("P56-WINDOWS-IMAGE-LOCK")
    try:
        import msvcrt

        descriptor = msvcrt.open_osfhandle(handle, os.O_RDONLY | getattr(os, "O_BINARY", 0))
    except Exception:
        _close_unowned_windows_handle(kernel32, handle)
        raise
    try:
        observed = _hash_open_descriptor(descriptor, "P56-LAUNCH-VERIFY")
        if (observed.size, observed.sha256) != (expected.size, expected.sha256):
            raise ReleaseFailure("P56-LAUNCH-VERIFY")
    except Exception:
        os.close(descriptor)
        raise
    return descriptor, lambda: os.close(descriptor)


def close_custody(handle: CustodyHandle) -> None:
    """Retire one exact live handle and exact-clean its private runtime roots."""

    with _REGISTRY_LOCK:
        record = _live_record(handle)
        if record.active_launches:
            raise ReleaseFailure("P56-HANDLE-ACTIVE")
        _retire_locked(handle, record)
        if record.cleanup_started:
            raise ReleaseFailure("P56-HANDLE-EXPIRED")
        record.cleanup_started = True
    _clean_live_custody(record)


def launch_verified(
    handle: CustodyHandle, platform: str, arguments: tuple[str, ...] | list[str]
) -> subprocess.CompletedProcess[bytes]:
    """Launch only an exact live handle through its registered in-memory bytes."""

    if type(arguments) not in {tuple, list}:
        raise ReleaseFailure("P56-LAUNCH-ARGUMENTS")
    argv = tuple(arguments)
    if any(type(argument) is not str or "\0" in argument for argument in argv):
        raise ReleaseFailure("P56-LAUNCH-ARGUMENTS")
    if type(platform) is not str or platform not in PLATFORMS or platform != _host_platform():
        raise ReleaseFailure("P56-UNSUPPORTED-PLATFORM")
    record: _LiveCustody | None = None
    root: Path | None = None
    try:
        with _REGISTRY_LOCK:
            record = _live_record(handle)
            if platform != record.platform or record.remaining_uses < 1:
                raise ReleaseFailure("P56-HANDLE-EXPIRED")
            record.remaining_uses -= 1
            record.active_launches += 1
            root = _register_launch_root(record)
        executable = root / record.artifact.path
        environment = _child_environment(record)
        if os.name == "nt":
            descriptor, close_image = _open_windows_verified_image(executable, record.artifact, record.artifact_bytes)
            try:
                process = subprocess.Popen(
                    [os.fspath(executable), *argv], cwd=record.runtime_root, env=environment,
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                )
            finally:
                close_image()  # CreateProcess has opened the image; Windows lock may then release.
            stdout, stderr = process.communicate()
            return subprocess.CompletedProcess([os.fspath(executable), *argv], process.returncode, stdout, stderr)
        descriptor, executable_fd_path = _open_linux_verified_image(
            executable, record.artifact, record.artifact_bytes
        )
        try:
            process = subprocess.Popen(
                [executable_fd_path, *argv], cwd=record.runtime_root, env=environment,
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, pass_fds=(descriptor,), close_fds=True,
            )
        finally:
            os.close(descriptor)  # The child inherited the descriptor used by /proc/self/fd.
        stdout, stderr = process.communicate()
        return subprocess.CompletedProcess([executable_fd_path, *argv], process.returncode, stdout, stderr)
    finally:
        if record is not None:
            _complete_launch(handle, record, root)


def _test_only_register_synthetic_handle(
    platform: str, runtime_parent: str | os.PathLike[str], executable_bytes: bytes, uses: int = 1
) -> CustodyHandle:
    """Test-only synthetic handle factory; it neither reads custody evidence nor accepts a root."""

    if type(executable_bytes) is not bytes or uses < 1:
        raise ReleaseFailure("P56-TEST-HANDLE")
    parent = _safe_existing_directory(Path(os.fspath(runtime_parent)), "P56-TEST-HANDLE")
    run = _fresh_child(parent, f".p56-test-{secrets.token_hex(8)}", "P56-TEST-HANDLE")
    run_id = _mkdir_exclusive(run, "P56-TEST-HANDLE")
    launch_parent = run / "launches"
    custody = run / "custody"
    launch_parent_id = _mkdir_exclusive(launch_parent, "P56-TEST-HANDLE")
    custody_id = _mkdir_exclusive(custody, "P56-TEST-HANDLE")
    _mkdir_exclusive(run / "home", "P56-TEST-HANDLE")
    _mkdir_exclusive(run / "tmp", "P56-TEST-HANDLE")
    suffix = ".exe" if platform == "windows-x86_64" else ""
    artifact = Binding(f"synthetic{suffix}", len(executable_bytes), sha256_bytes(executable_bytes))
    handle = CustodyHandle(_HANDLE_CONSTRUCTOR_KEY, secrets.token_bytes(32))
    with _REGISTRY_LOCK:
        _LIVE_HANDLES[handle] = _LiveCustody(
            handle._CustodyHandle__token, platform, artifact, executable_bytes,
            Binding("synthetic.receipt.json", 1, sha256_bytes(b"x")), {}, run, run_id, custody,
            custody_id, launch_parent, launch_parent_id, {"schema": SUMMARY_SCHEMA, "test_only": True}, uses,
        )
    return handle


def _arguments(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--platform", choices=sorted(PLATFORMS), required=True)
    parser.add_argument("--runtime-parent", required=True)
    parser.add_argument("--include-receipt", action="store_true")
    return parser.parse_args(argv)


def main(argv: Iterable[str] | None = None) -> int:
    parsed = _arguments(argv)
    try:
        handle = publish_retained_build_and_custody(parsed.platform, parsed.runtime_parent)
        output: dict[str, object] = {"summary": handle.summary}
        if parsed.include_receipt:
            with _REGISTRY_LOCK:
                output["qualification_receipt"] = copy.deepcopy(_live_record(handle).receipt_document)
        print(canonical_bytes(output).decode("ascii"))
        return 0
    except ReleaseFailure as error:
        print(canonical_bytes({"failure_code": error.code, "schema": SUMMARY_SCHEMA}).decode("ascii"))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
