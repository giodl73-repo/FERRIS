"""Pulse 57 capability-bound successor to the sealed Pulse 51 executor."""

from __future__ import annotations

import base64
import hashlib
import json
import os
import secrets
import stat
import subprocess
import threading
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Callable, Protocol

from sealed_dependencies import (
    P56,
    SealedDependencyFailure,
    bound_release_files,
    canonical_bytes,
    load_exact_p51,
    load_exact_p56,
    release_identities,
)


P43_CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
P43_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
WSL_PLATFORM = "ubuntu-24.04-x86_64"
WSL_SCHEMA = "ferris.pulse-57-wsl-capability-session/v1"
REQUEST_COUNT = 69
CANONICAL_PLATFORMS = ("windows-x86_64", WSL_PLATFORM)
P57_GATE_IDS = (
    "sealed-predecessor-binding",
    "windows-capability-build-custody",
    "ubuntu-capability-build-custody",
    "exact-adapter-preflight",
    "pulse-31-public-input",
    "pulse-35-pulse-37-normalization",
    "descriptor-validation",
    "bounded-process-exit-search",
)
MAX_PROTOCOL_BYTES = 2_800_000
MAX_BUNDLE_BYTES = 1_048_576
PROTOCOL_TIMEOUT_SECONDS = 15
CLOSE_TIMEOUT_SECONDS = 5
BUNDLE_SCHEMA = "ferris.pulse-57-wsl-bundle/v1"
WSL_WORKER_SHA256 = "sha256:9b0d91f7c4e2aed57d7dc40b95f5860f017138717364d3399d132884047904cb"
SEALED_DEPENDENCIES_SHA256 = "sha256:fe36a56a10d5d3659fae9cfacc3cd48075aaf0e3327ae029a2470d1107da6c8d"


class ExecutorFailure(RuntimeError):
    """A fail-closed bounded Pulse 57 executor failure."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass(frozen=True)
class ExecutorResult:
    catalog: dict[str, object]
    events: list[dict[str, object]]
    private_record: dict[str, object]


@dataclass(frozen=True)
class _InputIdentity:
    path: Path
    device: int | None
    inode: int | None
    size: int | None
    sha256: str | None


@dataclass(frozen=True)
class _FrozenDescriptor:
    descriptor: object
    semantics: object | None
    before: _InputIdentity | None
    after: _InputIdentity | None
    dispatch_probe: Path | None
    windows_arguments: tuple[str, ...] | None
    ubuntu_arguments: tuple[str, ...] | None


@dataclass(frozen=True)
class _StagedBundle:
    root: str
    python_identity: dict[str, object]


class _WslSession(Protocol):
    def launch(self, ordinal: int, arguments: tuple[str, ...]) -> object:
        """Launch one exact ordinal and return a Pulse 51 LaunchCapture."""

    def close(self) -> None:
        """Close a still-live session or confirm automatic use exhaustion."""


def _catalog() -> dict[str, object]:
    return {"gate_ids": list(P57_GATE_IDS), "schema": P43_CATALOG_SCHEMA}


def _execution_event(gate_id: str, kind: str, outcome: str) -> dict[str, object]:
    return {
        "classification": "ordered-execution",
        "event_kind": kind,
        "gate_id": gate_id,
        "outcome": outcome,
        "schema": P43_EVENT_SCHEMA,
    }


def _validation_event(validation_id: str, checks: int) -> dict[str, object]:
    return {
        "classification": "public-artifact-self-validation",
        "completed_checks": checks,
        "event_kind": "validation-complete",
        "expected_checks": checks,
        "schema": P43_EVENT_SCHEMA,
        "validation_id": validation_id,
    }


def _canonical_line(value: object) -> bytes:
    return canonical_bytes(value) + b"\n"


def _parse_line(raw: bytes, maximum: int) -> dict[str, object]:
    if not raw.endswith(b"\n") or len(raw) > maximum:
        raise ExecutorFailure("P57-WSL-PROTOCOL")
    try:
        value = json.loads(raw[:-1])
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ExecutorFailure("P57-WSL-PROTOCOL") from error
    if type(value) is not dict or _canonical_line(value) != raw:
        raise ExecutorFailure("P57-WSL-PROTOCOL")
    return value


def _request_id(ordinal: int, arguments: tuple[str, ...]) -> str:
    return "sha256:" + hashlib.sha256(
        canonical_bytes({"arguments": list(arguments), "ordinal": ordinal})
    ).hexdigest()


def _worker_result_id(
    ordinal: int,
    request_id: str,
    returncode: int,
    stdout_sha256: object,
    stderr_sha256: object,
) -> str:
    return "sha256:" + hashlib.sha256(
        canonical_bytes(
            {
                "ordinal": ordinal,
                "platform": WSL_PLATFORM,
                "request_id": request_id,
                "returncode": returncode,
                "stderr_sha256": stderr_sha256,
                "stdout_sha256": stdout_sha256,
            }
        )
    ).hexdigest()


def _bounded(operation: Callable[[], object], timeout: int) -> object:
    result: list[object] = []
    error: list[BaseException] = []
    complete = threading.Event()

    def invoke() -> None:
        try:
            result.append(operation())
        except BaseException as caught:
            error.append(caught)
        finally:
            complete.set()

    thread = threading.Thread(target=invoke, daemon=True)
    thread.start()
    if not complete.wait(timeout):
        raise TimeoutError("bounded operation timed out")
    if error:
        raise error[0]
    return result[0] if result else None


def _safe_bound_bytes(path: Path, code: str, maximum: int = MAX_BUNDLE_BYTES) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise ExecutorFailure(code)
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (opened.st_dev, opened.st_ino) != (
            initial.st_dev,
            initial.st_ino,
        ):
            raise ExecutorFailure(code)
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > maximum:
                raise ExecutorFailure(code)
        return bytes(content)
    except OSError as error:
        raise ExecutorFailure(code) from error
    finally:
        os.close(descriptor)


def _source_with_identity(path: Path, expected: str) -> bytes:
    content = _safe_bound_bytes(path, "P57-WSL-WORKER-IDENTITY")
    if "sha256:" + hashlib.sha256(content).hexdigest() != expected:
        raise ExecutorFailure("P57-WSL-WORKER-IDENTITY")
    return content


def _native_wsl_parent(value: str) -> None:
    if (
        type(value) is not str
        or not value.startswith("/")
        or value.startswith("/mnt/")
        or "\x00" in value
        or "\r" in value
        or "\n" in value
    ):
        raise ExecutorFailure("P57-WSL-NATIVE-ROOT")


def _wsl_environment() -> dict[str, str]:
    system_root = os.environ.get("SystemRoot") or os.environ.get("SYSTEMROOT")
    if not system_root:
        raise ExecutorFailure("P57-WSL-UNAVAILABLE")
    system32 = os.fspath(Path(system_root) / "System32")
    return {
        "ComSpec": os.fspath(Path(system32) / "cmd.exe"),
        "PATH": system32,
        "SystemRoot": system_root,
    }


def _wsl_executable() -> str:
    system_root = os.environ.get("SystemRoot") or os.environ.get("SYSTEMROOT")
    if not system_root:
        raise ExecutorFailure("P57-WSL-UNAVAILABLE")
    return os.fspath(Path(system_root) / "System32" / "wsl.exe")


_WSL_BUNDLE_BOOTSTRAP = r"""
import base64,json,os,sys
maximum=1048576
raw=sys.stdin.buffer.read(maximum+1)
if len(raw)>maximum: raise SystemExit(2)
request=json.loads(raw)
if json.dumps(request,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")!=raw: raise SystemExit(2)
if type(request) is not dict or set(request)!={"files","schema"} or request["schema"]!="ferris.pulse-57-wsl-bundle/v1" or type(request["files"]) is not list: raise SystemExit(2)
parent=sys.argv[1]
name=sys.argv[2]
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
root=os.path.join(parent,name)
os.mkdir(root,0o700)
for entry in request["files"]:
 if type(entry) is not dict or set(entry)!={"bytes_b64","path"} or type(entry["path"]) is not str or type(entry["bytes_b64"]) is not str: raise SystemExit(2)
 parts=entry["path"].split("/")
 if not parts or any(not part or part in {".",".."} for part in parts): raise SystemExit(2)
 data=base64.b64decode(entry["bytes_b64"],validate=True)
 target=os.path.join(root,*parts)
 directory=os.path.dirname(target)
 os.makedirs(directory,mode=0o700,exist_ok=True)
 descriptor=os.open(target,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o500)
 try:
  offset=0
  while offset<len(data):
   offset+=os.write(descriptor,data[offset:])
 finally:
  os.close(descriptor)
response={"bundle_root":root,"python":{"executable":sys.executable,"version":list(sys.version_info[:3])},"schema":"ferris.pulse-57-wsl-bundle-staged/v1"}
sys.stdout.buffer.write(json.dumps(response,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")+b"\n")
sys.stdout.buffer.flush()
"""


_WSL_WORKER_BOOTSTRAP = r"""
import hashlib,os,stat,sys
maximum=1048576
worker=sys.argv[1]
expected=sys.argv[2]
sys.argv=[worker,*sys.argv[3:]]
descriptor=os.open(worker,os.O_RDONLY|getattr(os,"O_NOFOLLOW",0))
try:
 metadata=os.fstat(descriptor)
 if not stat.S_ISREG(metadata.st_mode): raise SystemExit(2)
 content=bytearray()
 while True:
  chunk=os.read(descriptor,65536)
  if not chunk: break
  content.extend(chunk)
  if len(content)>maximum: raise SystemExit(2)
 source=bytes(content)
 if "sha256:"+hashlib.sha256(source).hexdigest()!=expected: raise SystemExit(2)
 filename="/proc/self/fd/"+str(descriptor)
 namespace={"__name__":"__main__","__file__":filename,"__package__":None}
 exec(compile(source,filename,"exec"),namespace)
finally:
 os.close(descriptor)
"""


def _p56_bundle_files(repo_root: Path) -> list[dict[str, object]]:
    files = bound_release_files(repo_root, P56)
    prefix = (
        "repository/docs/simulations/profile-diff-held-out/"
        "pulse-56-retained-build-custody-release/"
    )
    return [
        {"path": prefix + name, "bytes_b64": base64.b64encode(content).decode("ascii")}
        for name, content in sorted(files.items())
    ]


def _stage_wsl_bundle(repo_root: Path, runtime_parent: str) -> _StagedBundle:
    _native_wsl_parent(runtime_parent)
    release_root = Path(__file__).resolve().parent
    files = [
        {
            "path": "worker/wsl_session_worker.py",
            "bytes_b64": base64.b64encode(
                _source_with_identity(release_root / "wsl_session_worker.py", WSL_WORKER_SHA256)
            ).decode("ascii"),
        },
        {
            "path": "worker/sealed_dependencies.py",
            "bytes_b64": base64.b64encode(
                _source_with_identity(
                    release_root / "sealed_dependencies.py", SEALED_DEPENDENCIES_SHA256
                )
            ).decode("ascii"),
        },
        *_p56_bundle_files(repo_root),
    ]
    payload = canonical_bytes({"files": files, "schema": BUNDLE_SCHEMA})
    if len(payload) > MAX_BUNDLE_BYTES:
        raise ExecutorFailure("P57-WSL-BUNDLE")
    name = ".p57-" + secrets.token_hex(16)
    command = (
        _wsl_executable(),
        "--distribution",
        "Ubuntu-24.04",
        "--exec",
        "/usr/bin/python3",
        "-I",
        "-S",
        "-B",
        "-c",
        _WSL_BUNDLE_BOOTSTRAP,
        runtime_parent,
        name,
    )
    try:
        completed = subprocess.run(
            command,
            check=False,
            env=_wsl_environment(),
            input=payload,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=PROTOCOL_TIMEOUT_SECONDS,
        )
    except (OSError, subprocess.SubprocessError, TimeoutError) as error:
        raise ExecutorFailure("P57-WSL-BUNDLE") from error
    if completed.returncode != 0 or completed.stderr:
        raise ExecutorFailure("P57-WSL-BUNDLE")
    response = _parse_line(completed.stdout, MAX_PROTOCOL_BYTES)
    expected_root = runtime_parent.rstrip("/") + "/" + name
    python_identity = response.get("python")
    if (
        set(response) != {"bundle_root", "python", "schema"}
        or response["schema"] != "ferris.pulse-57-wsl-bundle-staged/v1"
        or response["bundle_root"] != expected_root
        or type(python_identity) is not dict
        or set(python_identity) != {"executable", "version"}
        or python_identity["executable"] != "/usr/bin/python3"
        or type(python_identity["version"]) is not list
        or len(python_identity["version"]) != 3
        or any(type(part) is not int or part < 0 for part in python_identity["version"])
    ):
        raise ExecutorFailure("P57-WSL-BUNDLE")
    return _StagedBundle(expected_root, python_identity)


class _NativeWslSession:
    """The only production route to a native Ubuntu P56 live capability."""

    def __init__(self, repo_root: Path, ubuntu_runtime_parent: str, p51: ModuleType) -> None:
        _native_wsl_parent(ubuntu_runtime_parent)
        staged = _stage_wsl_bundle(repo_root, ubuntu_runtime_parent)
        command = (
            _wsl_executable(),
            "--distribution",
            "Ubuntu-24.04",
            "--exec",
            "/usr/bin/python3",
            "-I",
            "-S",
            "-B",
            "-c",
            _WSL_WORKER_BOOTSTRAP,
            staged.root + "/worker/wsl_session_worker.py",
            WSL_WORKER_SHA256,
            "--runtime-parent",
            ubuntu_runtime_parent,
            "--bundle-root",
            staged.root,
            "--p56-root",
            staged.root
            + "/repository/docs/simulations/profile-diff-held-out/"
            + "pulse-56-retained-build-custody-release",
        )
        try:
            self._process = subprocess.Popen(
                command,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                env=_wsl_environment(),
            )
        except OSError as error:
            raise ExecutorFailure("P57-WSL-UNAVAILABLE") from error
        self._p51 = p51
        self._closed = False
        self._launched = 0
        try:
            ready = self._read()
            if ready != {
                "count": REQUEST_COUNT,
                "platform": WSL_PLATFORM,
                "python": staged.python_identity,
                "schema": WSL_SCHEMA,
                "type": "ready",
            }:
                raise ExecutorFailure("P57-WSL-PROTOCOL")
        except (ExecutorFailure, OSError, ValueError, subprocess.SubprocessError, TimeoutError):
            try:
                self._abort()
            except ExecutorFailure as cleanup:
                raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
            raise ExecutorFailure("P57-WSL-PROTOCOL")
        except BaseException:
            try:
                self._abort()
            except BaseException as cleanup:
                raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
            raise

    def _read(self) -> dict[str, object]:
        assert self._process.stdout is not None
        line = _bounded(
            lambda: self._process.stdout.readline(MAX_PROTOCOL_BYTES + 1),
            PROTOCOL_TIMEOUT_SECONDS,
        )
        if type(line) is not bytes or not line or len(line) > MAX_PROTOCOL_BYTES:
            raise ExecutorFailure("P57-WSL-PROTOCOL")
        return _parse_line(line, MAX_PROTOCOL_BYTES)

    def _write(self, payload: bytes) -> None:
        assert self._process.stdin is not None
        _bounded(lambda: self._process.stdin.write(payload), PROTOCOL_TIMEOUT_SECONDS)
        _bounded(self._process.stdin.flush, PROTOCOL_TIMEOUT_SECONDS)

    def _wait(self) -> int | None:
        try:
            return self._process.wait(timeout=CLOSE_TIMEOUT_SECONDS)
        except subprocess.TimeoutExpired:
            return None
        except (OSError, ValueError) as error:
            raise ExecutorFailure("P57-WSL-CLEANUP") from error

    def _end_process(self) -> int:
        status = self._wait()
        if status is not None:
            return status
        try:
            _bounded(self._process.terminate, CLOSE_TIMEOUT_SECONDS)
        except (OSError, ValueError, TimeoutError):
            pass
        status = self._wait()
        if status is not None:
            return status
        try:
            _bounded(self._process.kill, CLOSE_TIMEOUT_SECONDS)
        except (OSError, ValueError, TimeoutError) as error:
            raise ExecutorFailure("P57-WSL-CLEANUP") from error
        status = self._wait()
        if status is None:
            raise ExecutorFailure("P57-WSL-CLEANUP")
        return status

    def _drain(self) -> tuple[bytes, bytes]:
        assert self._process.stdout is not None
        assert self._process.stderr is not None
        try:
            stdout = _bounded(
                lambda: self._process.stdout.read(MAX_PROTOCOL_BYTES + 1),
                CLOSE_TIMEOUT_SECONDS,
            )
            stderr = _bounded(
                lambda: self._process.stderr.read(MAX_PROTOCOL_BYTES + 1),
                CLOSE_TIMEOUT_SECONDS,
            )
        except (OSError, ValueError, TimeoutError) as error:
            raise ExecutorFailure("P57-WSL-CLEANUP") from error
        if type(stdout) is not bytes or type(stderr) is not bytes:
            raise ExecutorFailure("P57-WSL-CLEANUP")
        return stdout, stderr

    def _close_stdin(self) -> None:
        assert self._process.stdin is not None
        try:
            _bounded(self._process.stdin.close, CLOSE_TIMEOUT_SECONDS)
        except (OSError, ValueError, TimeoutError):
            pass

    def _abort(self) -> None:
        self._closed = True
        self._close_stdin()
        self._end_process()
        self._drain()

    def _fatal_close(self, code: str) -> None:
        try:
            self.close()
        except BaseException as cleanup:
            raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
        raise ExecutorFailure(code)

    def launch(self, ordinal: int, arguments: tuple[str, ...]) -> object:
        if self._closed or ordinal != self._launched + 1 or len(arguments) != 7:
            raise ExecutorFailure("P57-WSL-PROTOCOL")
        request_id = _request_id(ordinal, arguments)
        request = {
            "arguments": list(arguments),
            "ordinal": ordinal,
            "platform": WSL_PLATFORM,
            "request_id": request_id,
            "schema": WSL_SCHEMA,
            "type": "launch",
        }
        try:
            self._write(_canonical_line(request))
            response = self._read()
            required = {
                "ordinal",
                "platform",
                "request_id",
                "returncode",
                "result_id",
                "schema",
                "stderr_b64",
                "stderr_sha256",
                "stdout_b64",
                "stdout_sha256",
                "type",
            }
            if (
                set(response) != required
                or response["type"] != "result"
                or response["schema"] != WSL_SCHEMA
                or response["platform"] != WSL_PLATFORM
                or response["ordinal"] != ordinal
                or response["request_id"] != request_id
                or type(response["returncode"]) is not int
                or type(response["stdout_b64"]) is not str
                or type(response["stderr_b64"]) is not str
            ):
                self._fatal_close("P57-WSL-PROTOCOL")
            stdout = base64.b64decode(response["stdout_b64"], validate=True)
            stderr = base64.b64decode(response["stderr_b64"], validate=True)
            if (
                response["stdout_sha256"]
                != "sha256:" + hashlib.sha256(stdout).hexdigest()
                or response["stderr_sha256"]
                != "sha256:" + hashlib.sha256(stderr).hexdigest()
                or response["result_id"]
                != _worker_result_id(
                    ordinal,
                    request_id,
                    response["returncode"],
                    response["stdout_sha256"],
                    response["stderr_sha256"],
                )
            ):
                self._fatal_close("P57-WSL-PROTOCOL")
        except (
            ExecutorFailure,
            OSError,
            ValueError,
            subprocess.SubprocessError,
            TimeoutError,
        ):
            self._fatal_close("P57-WSL-PROTOCOL")
            raise AssertionError("unreachable")
        except BaseException:
            try:
                self._abort()
            except BaseException as cleanup:
                raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
            raise
        self._launched += 1
        return self._p51.LaunchCapture(response["returncode"], stdout, stderr)

    def close(self) -> None:
        if self._closed:
            return
        self._closed = True
        sent = True
        try:
            self._write(_canonical_line({"schema": WSL_SCHEMA, "type": "close"}))
        except (ExecutorFailure, OSError, ValueError, subprocess.SubprocessError, TimeoutError):
            sent = False
        self._close_stdin()
        status = self._end_process()
        stdout, stderr = self._drain()
        if not sent or stdout or stderr or status != 0:
            raise ExecutorFailure("P57-WSL-CLEANUP")


@dataclass(frozen=True)
class _Controls:
    p51: ModuleType
    p56: ModuleType
    p27_runner: Callable[[Path], dict[str, object]] | None
    open_wsl: Callable[[Path, str, ModuleType], _WslSession]
    bounded_failures: tuple[type[BaseException], ...] = ()


def _identity(path: Path) -> _InputIdentity:
    try:
        resolved = path.resolve(strict=False)
        if not os.path.lexists(resolved):
            return _InputIdentity(resolved, None, None, None, None)
        initial = os.lstat(resolved)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise ExecutorFailure("P57-P51-INPUT-IDENTITY")
        descriptor = os.open(
            resolved, os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0)
        )
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure("P57-P51-INPUT-IDENTITY") from error
    try:
        opened = os.fstat(descriptor)
        if (opened.st_dev, opened.st_ino) != (initial.st_dev, initial.st_ino):
            raise ExecutorFailure("P57-P51-INPUT-IDENTITY")
        digest = hashlib.sha256()
        size = 0
        while chunk := os.read(descriptor, 65_536):
            digest.update(chunk)
            size += len(chunk)
        return _InputIdentity(resolved, opened.st_dev, opened.st_ino, size, "sha256:" + digest.hexdigest())
    except OSError as error:
        raise ExecutorFailure("P57-P51-INPUT-IDENTITY") from error
    finally:
        os.close(descriptor)


def _copy_expected(value: object) -> dict[str, object]:
    try:
        copied = json.loads(canonical_bytes(value))
    except (TypeError, ValueError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ExecutorFailure("P57-P51-SEMANTICS") from error
    if type(copied) is not dict:
        raise ExecutorFailure("P57-P51-SEMANTICS")
    return copied


def _frozen_descriptor(descriptor: object, expected: dict[str, object], before: Path | None, after: Path | None) -> object:
    try:
        return type(descriptor)(
            descriptor.ordinal,
            descriptor.case_id,
            descriptor.output_format,
            expected,
            before,
            after,
            descriptor.execution_mode,
        )
    except (AttributeError, TypeError) as error:
        raise ExecutorFailure("P57-P51-SEMANTICS") from error


def _freeze_descriptors(
    p51: ModuleType, descriptor_root: Path, descriptors: tuple[object, ...], runtime_root: Path
) -> tuple[_FrozenDescriptor, ...]:
    frozen: list[_FrozenDescriptor] = []
    dispatch_probe = p51._runtime_path(
        runtime_root,
        descriptor_root / "case-manifest.json",
        "P57-P51-DISPATCH",
        require_regular=True,
    )
    for original in descriptors:
        expected = _copy_expected(original.expected)
        if original.execution_mode == "no-launch":
            frozen.append(
                _FrozenDescriptor(
                    _frozen_descriptor(original, expected, None, None),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            )
            continue
        if original.before is None or original.after is None:
            raise ExecutorFailure("P57-P51-SEMANTICS")
        before = _identity(
            p51._runtime_path(runtime_root, original.before, "P57-P51-PATH", allow_absent_leaf=True)
        )
        after = _identity(
            p51._runtime_path(runtime_root, original.after, "P57-P51-PATH", allow_absent_leaf=True)
        )
        descriptor = _frozen_descriptor(original, expected, before.path, after.path)
        semantics = p51._descriptor_semantics(descriptor)
        windows = p51.build_platform_dispatch(
            "windows-x86_64", dispatch_probe, descriptor, runtime_root
        )
        ubuntu = p51.build_platform_dispatch(WSL_PLATFORM, dispatch_probe, descriptor, runtime_root)
        windows_arguments = tuple(windows.application_argv)
        ubuntu_arguments = tuple(ubuntu.command[-7:])
        if len(windows_arguments) != 7 or len(ubuntu_arguments) != 7:
            raise ExecutorFailure("P57-P51-DISPATCH")
        frozen.append(
            _FrozenDescriptor(
                descriptor, semantics, before, after, dispatch_probe, windows_arguments, ubuntu_arguments
            )
        )
    return tuple(frozen)


def _same_identity(expected: _InputIdentity) -> None:
    observed = _identity(expected.path)
    if observed != expected:
        raise ExecutorFailure("P57-P51-INPUT-SUBSTITUTION")


def _prelaunch_dispatch(p51: ModuleType, frozen: _FrozenDescriptor, runtime_root: Path) -> tuple[tuple[str, ...], tuple[str, ...]]:
    if (
        frozen.before is None
        or frozen.after is None
        or frozen.dispatch_probe is None
        or frozen.windows_arguments is None
        or frozen.ubuntu_arguments is None
    ):
        raise ExecutorFailure("P57-P51-DISPATCH")
    _same_identity(frozen.before)
    _same_identity(frozen.after)
    windows = p51.build_platform_dispatch(
        "windows-x86_64", frozen.dispatch_probe, frozen.descriptor, runtime_root
    )
    ubuntu = p51.build_platform_dispatch(
        WSL_PLATFORM, frozen.dispatch_probe, frozen.descriptor, runtime_root
    )
    if (
        tuple(windows.application_argv) != frozen.windows_arguments
        or tuple(ubuntu.command[-7:]) != frozen.ubuntu_arguments
    ):
        raise ExecutorFailure("P57-P51-INPUT-SUBSTITUTION")
    return frozen.windows_arguments, frozen.ubuntu_arguments


def _normalize_result(p51: ModuleType, frozen: _FrozenDescriptor, capture: object) -> dict[str, object]:
    if frozen.semantics is None:
        raise ExecutorFailure("P57-P51-SEMANTICS")
    descriptor = frozen.descriptor
    if descriptor.output_format == "json":
        return p51._json_normalized(capture, descriptor.expected, frozen.semantics)
    return p51._human_normalized(capture, descriptor.expected, frozen.semantics)


def _failure_types(module: ModuleType, names: tuple[str, ...]) -> tuple[type[BaseException], ...]:
    result: list[type[BaseException]] = []
    for name in names:
        candidate = getattr(module, name, None)
        if isinstance(candidate, type) and issubclass(candidate, BaseException):
            result.append(candidate)
    return tuple(result)


def _with_terminal_failure_types(
    controls: _Controls, *modules: ModuleType
) -> _Controls:
    names = ("PublicFailure", "BridgeFailure", "WitnessFailure", "SummaryMalformed")
    discovered = tuple(
        candidate
        for module in modules
        for candidate in _failure_types(module, names)
    )
    return _Controls(
        controls.p51,
        controls.p56,
        controls.p27_runner,
        controls.open_wsl,
        (*controls.bounded_failures, *discovered),
    )


def _known_failure(error: BaseException, controls: _Controls | None = None) -> bool:
    known: tuple[type[BaseException], ...] = (
        ExecutorFailure,
        SealedDependencyFailure,
        OSError,
        ValueError,
        subprocess.SubprocessError,
        TimeoutError,
    )
    if controls is not None:
        known += _failure_types(
            controls.p51,
            (
                "ExecutorFailure",
                "DependencyFailure",
                "P31Failure",
                "CustodyFailure",
                "_TerminalPreconditionFailure",
            ),
        )
        known += _failure_types(controls.p56, ("ReleaseFailure",))
        known += controls.bounded_failures
    return isinstance(error, known)


def _close_handles(
    controls: _Controls,
    windows_handle: object | None,
    windows_launches: int,
    wsl: _WslSession | None,
) -> None:
    failure: BaseException | None = None
    if windows_handle is not None and windows_launches < REQUEST_COUNT:
        try:
            controls.p56.close_custody(windows_handle)
        except BaseException as error:
            failure = error
    if wsl is not None:
        try:
            wsl.close()
        except BaseException as error:
            failure = failure or error
    if failure is not None:
        raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from failure


def _terminal(
    p43: ModuleType | None,
    events: list[dict[str, object]],
    gate: str,
    code: str | None,
    private_record: dict[str, object],
    controls: _Controls | None = None,
) -> ExecutorResult:
    outcome = "completed" if code is None else "failed"
    candidate = _execution_event(gate, "terminal-stop", outcome)
    if p43 is not None:
        try:
            p43.validate_catalog(_catalog())
            p43.validate_events(tuple(P57_GATE_IDS), [*events, candidate])
        except BaseException as error:
            if not _terminal_validation_failure(error, controls):
                raise
            if code != "P57-INDETERMINATE-CLEANUP":
                code = _failure_code(error)
                candidate = _execution_event(gate, "terminal-stop", "failed")
    events.append(candidate)
    private_record["outcome"] = "completed" if code is None else "failed"
    if code is not None:
        private_record["failure_code"] = code
    return ExecutorResult(_catalog(), events, private_record)


def _failure_code(error: BaseException) -> str:
    code = getattr(error, "code", None)
    return (
        code
        if type(code) is str and code.startswith(("P27-", "P31-", "P35-", "P37-", "P43-", "P45-", "P47-", "P51-", "P56-", "P57-"))
        else "P57-OS-PROTOCOL"
    )


def _terminal_validation_failure(
    error: BaseException, controls: _Controls | None = None
) -> bool:
    code = getattr(error, "code", None)
    return _known_failure(error, controls) or (
        type(code) is str and code.startswith(("P43-", "P45-", "P47-", "P51-", "P56-", "P57-"))
    )


def _execute(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    ubuntu_runtime_parent: str,
    controls: _Controls,
) -> ExecutorResult:
    events: list[dict[str, object]] = []
    private_record: dict[str, object] = {
        "schema": "ferris.pulse-57-private-execution-record/v1",
        "outcome": "in-progress",
        "platform_records": {platform: [] for platform in CANONICAL_PLATFORMS},
        "no_launch_records": [],
        "process_counts": {platform: 0 for platform in CANONICAL_PLATFORMS},
        "p27_cycle_retention": "not-attempted",
    }
    current_gate = P57_GATE_IDS[0]
    p43: ModuleType | None = None
    windows_handle: object | None = None
    windows_launches = 0
    wsl: _WslSession | None = None
    code: str | None = None
    try:
        p43, _p45, _p47 = controls.p51.load_terminal_dependencies(repo_root)
        controls = _with_terminal_failure_types(controls, p43, _p45, _p47)
        runtime_root = controls.p51._safe_runtime_root(private_runtime_root)
        events.append(_validation_event("sealed-predecessor-identities", 2))
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[1]
        windows_handle = controls.p56.publish_retained_build_and_custody("windows-x86_64", runtime_root)
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[2]
        wsl = controls.open_wsl(repo_root, ubuntu_runtime_parent, controls.p51)
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[3]
        p27_runner = controls.p27_runner or controls.p51.load_p27_exact_runner(repo_root)
        controls.p51._run_p27_once(runtime_root, p27_cycle_root, p27_runner)
        private_record["p27_cycle_retention"] = "retained-private-cycle-root"
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[4]
        if controls.p51.verify_bound_contract(repo_root) != {
            "artifact_count": 9,
            "positive_fixture_count": 6,
            "mutation_control_count": 33,
            "public_input_checks": 39,
        }:
            raise ExecutorFailure("P57-P51-CONTROL-COUNT")
        events.append(_validation_event("public-input-contract", 39))
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[5]
        if controls.p51.verify_p35_p37_custody(repo_root) != {
            "bound_file_count": 11,
            "p35_release_tree_file_count": 10,
            "machine_schema_count": 1,
            "canonical_lf_file_count": 11,
            "git_clean_checks": 11,
        }:
            raise ExecutorFailure("P57-P51-CUSTODY-COUNT")
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[6]
        root, descriptors = controls.p51.validate_descriptor_root(descriptor_root, runtime_root)
        frozen = _freeze_descriptors(controls.p51, root, descriptors, runtime_root)
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P57_GATE_IDS[7]
        for descriptor in frozen:
            if descriptor.descriptor.execution_mode == "no-launch":
                for platform in CANONICAL_PLATFORMS:
                    private_record["no_launch_records"].append(
                        {
                            "ordinal": descriptor.descriptor.ordinal,
                            "platform": platform,
                            "process_launched": False,
                            "reason": "blocked-no-launch-external-immutable-binary-freeze",
                        }
                    )
                continue
            windows_arguments, ubuntu_arguments = _prelaunch_dispatch(
                controls.p51, descriptor, runtime_root
            )
            windows_capture = controls.p56.launch_verified(
                windows_handle, "windows-x86_64", windows_arguments
            )
            windows_launches += 1
            _same_identity(descriptor.before)  # type: ignore[arg-type]
            _same_identity(descriptor.after)  # type: ignore[arg-type]
            windows = {
                "ordinal": descriptor.descriptor.ordinal,
                "platform": "windows-x86_64",
                "process_launched": True,
                "result": _normalize_result(controls.p51, descriptor, windows_capture),
            }
            private_record["platform_records"]["windows-x86_64"].append(windows)
            private_record["process_counts"]["windows-x86_64"] += 1

            _prelaunch_dispatch(controls.p51, descriptor, runtime_root)
            ubuntu_capture = wsl.launch(descriptor.descriptor.ordinal, ubuntu_arguments)
            _same_identity(descriptor.before)  # type: ignore[arg-type]
            _same_identity(descriptor.after)  # type: ignore[arg-type]
            ubuntu = {
                "ordinal": descriptor.descriptor.ordinal,
                "platform": WSL_PLATFORM,
                "process_launched": True,
                "result": _normalize_result(controls.p51, descriptor, ubuntu_capture),
            }
            private_record["platform_records"][WSL_PLATFORM].append(ubuntu)
            private_record["process_counts"][WSL_PLATFORM] += 1
            if windows["result"]["semantic_projection"] != ubuntu["result"]["semantic_projection"]:
                private_record["first_mismatch_ordinal"] = descriptor.descriptor.ordinal
                raise ExecutorFailure("P57-FIRST-TARGET-MISMATCH")

        if (
            private_record["process_counts"] != {"windows-x86_64": 69, WSL_PLATFORM: 69}
            or len(private_record["no_launch_records"]) != 2
            or windows_launches != REQUEST_COUNT
        ):
            raise ExecutorFailure("P57-TOPOLOGY-ACCOUNTING")
    except BaseException as error:
        if not _known_failure(error, controls):
            try:
                _close_handles(controls, windows_handle, windows_launches, wsl)
            except BaseException as cleanup:
                raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
            raise
        code = _failure_code(error)

    try:
        _close_handles(controls, windows_handle, windows_launches, wsl)
    except BaseException:
        code = "P57-INDETERMINATE-CLEANUP"
    return _terminal(p43, events, current_gate, code, private_record, controls)


def run_capability_bound_diagnostic_executor(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    ubuntu_runtime_parent: str,
) -> ExecutorResult:
    """Run the fixed successor without any custody, executable, or runner seam."""

    try:
        p51 = load_exact_p51(repo_root)
        p56 = load_exact_p56(repo_root)
    except SealedDependencyFailure as error:
        return _terminal(None, [], P57_GATE_IDS[0], error.code, {"outcome": "in-progress"})
    return _execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        ubuntu_runtime_parent,
        _Controls(p51, p56, None, lambda root, parent, api: _NativeWslSession(root, parent, api)),
    )


def _run_qualification_executor(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    controls: _Controls,
) -> ExecutorResult:
    """Private source-bound test seam; it is not exported production authority."""

    return _execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        "/home/pulse57-qualification",
        controls,
    )


__all__ = [
    "ExecutorFailure",
    "ExecutorResult",
    "release_identities",
    "run_capability_bound_diagnostic_executor",
]
