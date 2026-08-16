"""Pulse 69 cleanup-owning successor to the exact Pulse 57 executor."""

from __future__ import annotations

import json
import os
import subprocess
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType

from sealed_dependencies import (
    SealedDependencyFailure,
    load_exact_p57_stack,
    release_identities,
)


ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
_P57, _P51, _P56 = load_exact_p57_stack(REPO_ROOT)

ExecutorFailure = _P57.ExecutorFailure
ExecutorResult = _P57.ExecutorResult
P43_CATALOG_SCHEMA = _P57.P43_CATALOG_SCHEMA
P43_EVENT_SCHEMA = _P57.P43_EVENT_SCHEMA
WSL_PLATFORM = _P57.WSL_PLATFORM
WSL_SCHEMA = _P57.WSL_SCHEMA
REQUEST_COUNT = _P57.REQUEST_COUNT
CANONICAL_PLATFORMS = _P57.CANONICAL_PLATFORMS
P57_GATE_IDS = _P57.P57_GATE_IDS
MAX_PROTOCOL_BYTES = _P57.MAX_PROTOCOL_BYTES
PROTOCOL_TIMEOUT_SECONDS = _P57.PROTOCOL_TIMEOUT_SECONDS
_Controls = _P57._Controls
_copy_expected = _P57._copy_expected
_frozen_descriptor = _P57._frozen_descriptor
_freeze_descriptors = _P57._freeze_descriptors
_prelaunch_dispatch = _P57._prelaunch_dispatch
_with_terminal_failure_types = _P57._with_terminal_failure_types
_known_failure = _P57._known_failure
_close_handles = _P57._close_handles
_normalize_result = _P57._normalize_result
load_exact_p51 = _P57.load_exact_p51
load_exact_p56 = _P57.load_exact_p56

_BUNDLE_IDENTITY_SCHEMA = "ferris.pulse-69-staged-bundle-identity/v1"
_BUNDLE_CLEANUP_SCHEMA = "ferris.pulse-69-staged-bundle-cleanup/v1"


@dataclass(frozen=True)
class _BundleLayout:
    expected_files: tuple[str, ...]
    expected_directories: tuple[str, ...]
    expected_children: dict[str, tuple[str, ...]]


@dataclass(frozen=True)
class _OwnedBundle:
    root: str
    runtime_parent: str
    name: str
    python_identity: dict[str, object]
    root_device: int
    root_inode: int
    parent_device: int
    parent_inode: int
    layout: _BundleLayout


def _expected_bundle_layout(repo_root: Path) -> _BundleLayout:
    files = [
        "worker/wsl_session_worker.py",
        "worker/sealed_dependencies.py",
        *(
            str(entry["path"])
            for entry in _P57._p56_bundle_files(repo_root)
        ),
    ]
    directories: set[str] = set()
    children: dict[str, set[str]] = {"": set()}
    for relative in files:
        parts = relative.split("/")
        prefix = ""
        for index, part in enumerate(parts):
            children.setdefault(prefix, set()).add(part)
            if index == len(parts) - 1:
                break
            prefix = part if not prefix else prefix + "/" + part
            directories.add(prefix)
            children.setdefault(prefix, set())
    return _BundleLayout(
        expected_files=tuple(sorted(files)),
        expected_directories=tuple(sorted(directories)),
        expected_children={
            prefix: tuple(sorted(names)) for prefix, names in sorted(children.items())
        },
    )


def _run_wsl_json(
    script: str,
    arguments: tuple[str, ...],
    payload: object | None,
    code: str,
) -> dict[str, object]:
    command = (
        _P57._wsl_executable(),
        "--distribution",
        "Ubuntu-24.04",
        "--exec",
        "/usr/bin/python3",
        "-I",
        "-S",
        "-B",
        "-c",
        script,
        *arguments,
    )
    try:
        completed = subprocess.run(
            command,
            check=False,
            env=_P57._wsl_environment(),
            input=b"" if payload is None else _P57.canonical_bytes(payload),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=PROTOCOL_TIMEOUT_SECONDS,
        )
    except (OSError, subprocess.SubprocessError, TimeoutError) as error:
        raise ExecutorFailure(code) from error
    if completed.returncode != 0 or completed.stderr:
        raise ExecutorFailure(code)
    return _P57._parse_line(completed.stdout, MAX_PROTOCOL_BYTES)


_WSL_BUNDLE_IDENTITY_BOOTSTRAP = r"""
import json,os,stat,sys
parent=sys.argv[1]
name=sys.argv[2]
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
flags=os.O_RDONLY|getattr(os,"O_DIRECTORY",0)|getattr(os,"O_NOFOLLOW",0)
parent_fd=os.open(parent,flags)
try:
 parent_stat=os.fstat(parent_fd)
 root_stat=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(root_stat.st_mode) or not stat.S_ISDIR(root_stat.st_mode): raise SystemExit(2)
 root_fd=os.open(name,flags,dir_fd=parent_fd)
 try:
  opened=os.fstat(root_fd)
  if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(root_stat.st_dev,root_stat.st_ino): raise SystemExit(2)
 finally:
  os.close(root_fd)
 response={
  "bundle_root": parent.rstrip("/")+"/"+name,
  "parent_device": parent_stat.st_dev,
  "parent_inode": parent_stat.st_ino,
  "root_device": root_stat.st_dev,
  "root_inode": root_stat.st_ino,
  "schema": "ferris.pulse-69-staged-bundle-identity/v1",
 }
 sys.stdout.buffer.write(json.dumps(response,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")+b"\n")
 sys.stdout.buffer.flush()
finally:
 os.close(parent_fd)
"""


_WSL_BUNDLE_CLEANUP_BOOTSTRAP = r"""
import errno,json,os,stat,sys
maximum=262144
raw=sys.stdin.buffer.read(maximum+1)
if len(raw)>maximum: raise SystemExit(2)
request=json.loads(raw)
if json.dumps(request,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")!=raw: raise SystemExit(2)
required={"bundle_root","expected_children","expected_directories","expected_files","name","parent_device","parent_inode","root_device","root_inode","runtime_parent","schema"}
if type(request) is not dict or set(request)!=required or request["schema"]!="ferris.pulse-69-staged-bundle-cleanup/v1": raise SystemExit(2)
parent=request["runtime_parent"]
name=request["name"]
expected_root=request["bundle_root"]
if type(parent) is not str or type(name) is not str or type(expected_root) is not str: raise SystemExit(2)
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
if expected_root!=parent.rstrip("/")+"/"+name: raise SystemExit(2)
expected_files=request["expected_files"]
expected_directories=request["expected_directories"]
expected_children=request["expected_children"]
if type(expected_files) is not list or type(expected_directories) is not list or type(expected_children) is not dict: raise SystemExit(2)
file_set={path for path in expected_files if type(path) is str and path and not path.startswith("/") and ".." not in path.split("/")}
directory_set={path for path in expected_directories if type(path) is str and path and not path.startswith("/") and ".." not in path.split("/")}
if len(file_set)!=len(expected_files) or len(directory_set)!=len(expected_directories) or file_set&directory_set: raise SystemExit(2)
normalized_children={}
for prefix, names in expected_children.items():
 if type(prefix) is not str or type(names) is not list: raise SystemExit(2)
 if prefix and (prefix.startswith("/") or ".." in prefix.split("/")): raise SystemExit(2)
 if len(names)!=len({name for name in names if type(name) is str and name and "/" not in name and name not in {".",".."}}): raise SystemExit(2)
 normalized_children[prefix]=tuple(sorted(names))
if "" not in normalized_children: raise SystemExit(2)
flags=os.O_RDONLY|getattr(os,"O_DIRECTORY",0)|getattr(os,"O_NOFOLLOW",0)
def _sync_parent(fd):
 try:
  os.fsync(fd)
  return "synced"
 except OSError as error:
  if error.errno in {errno.EINVAL,getattr(errno,"ENOTSUP",errno.EINVAL),getattr(errno,"EOPNOTSUPP",errno.EINVAL)}:
   return "unsupported"
  raise
def _remove(fd,prefix):
 listing=tuple(sorted(os.listdir("/proc/self/fd/"+str(fd))))
 expected=normalized_children.get(prefix)
 if expected is None or listing!=expected: raise SystemExit(2)
 files_removed=0
 directories_removed=0
 for child in listing:
  relative=child if not prefix else prefix+"/"+child
  metadata=os.lstat(child,dir_fd=fd)
  if stat.S_ISLNK(metadata.st_mode): raise SystemExit(2)
  if relative in directory_set:
   if not stat.S_ISDIR(metadata.st_mode): raise SystemExit(2)
   child_fd=os.open(child,flags,dir_fd=fd)
   try:
    opened=os.fstat(child_fd)
    if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(metadata.st_dev,metadata.st_ino): raise SystemExit(2)
    child_files,child_directories=_remove(child_fd,relative)
   finally:
    os.close(child_fd)
   os.rmdir(child,dir_fd=fd)
   files_removed+=child_files
   directories_removed+=child_directories+1
  elif relative in file_set:
   if not stat.S_ISREG(metadata.st_mode): raise SystemExit(2)
   os.unlink(child,dir_fd=fd)
   files_removed+=1
  else:
   raise SystemExit(2)
 return files_removed,directories_removed
parent_fd=os.open(parent,flags)
try:
 parent_stat=os.fstat(parent_fd)
 if (parent_stat.st_dev,parent_stat.st_ino)!=(request["parent_device"],request["parent_inode"]): raise SystemExit(2)
 root_stat=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(root_stat.st_mode) or not stat.S_ISDIR(root_stat.st_mode): raise SystemExit(2)
 if (root_stat.st_dev,root_stat.st_ino)!=(request["root_device"],request["root_inode"]): raise SystemExit(2)
 root_fd=os.open(name,flags,dir_fd=parent_fd)
 try:
  opened=os.fstat(root_fd)
  if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(root_stat.st_dev,root_stat.st_ino): raise SystemExit(2)
  files_removed,directories_removed=_remove(root_fd,"")
 finally:
  os.close(root_fd)
 os.rmdir(name,dir_fd=parent_fd)
 sync_status=_sync_parent(parent_fd)
 try:
  os.lstat(name,dir_fd=parent_fd)
  raise SystemExit(2)
 except FileNotFoundError:
  pass
 response={
  "bundle_absent": True,
  "bundle_root": expected_root,
  "directories_removed": directories_removed,
  "files_removed": files_removed,
  "parent_sync_status": sync_status,
  "schema": "ferris.pulse-69-staged-bundle-cleanup/v1",
 }
 sys.stdout.buffer.write(json.dumps(response,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")+b"\n")
 sys.stdout.buffer.flush()
finally:
 os.close(parent_fd)
"""


def _stage_owned_bundle(repo_root: Path, ubuntu_runtime_parent: str) -> _OwnedBundle:
    _P57._native_wsl_parent(ubuntu_runtime_parent)
    staged = _P57._stage_wsl_bundle(repo_root, ubuntu_runtime_parent)
    name = staged.root.rsplit("/", 1)[-1]
    if (
        not name.startswith(".p57-")
        or staged.root != ubuntu_runtime_parent.rstrip("/") + "/" + name
    ):
        raise ExecutorFailure("P57-WSL-BUNDLE")
    identity = _run_wsl_json(
        _WSL_BUNDLE_IDENTITY_BOOTSTRAP,
        (ubuntu_runtime_parent, name),
        None,
        "P57-WSL-BUNDLE",
    )
    if (
        set(identity)
        != {
            "bundle_root",
            "parent_device",
            "parent_inode",
            "root_device",
            "root_inode",
            "schema",
        }
        or identity["schema"] != _BUNDLE_IDENTITY_SCHEMA
        or identity["bundle_root"] != staged.root
        or any(
            type(identity[field]) is not int or identity[field] < 0
            for field in ("parent_device", "parent_inode", "root_device", "root_inode")
        )
    ):
        raise ExecutorFailure("P57-WSL-BUNDLE")
    return _OwnedBundle(
        root=staged.root,
        runtime_parent=ubuntu_runtime_parent,
        name=name,
        python_identity=staged.python_identity,
        root_device=identity["root_device"],
        root_inode=identity["root_inode"],
        parent_device=identity["parent_device"],
        parent_inode=identity["parent_inode"],
        layout=_expected_bundle_layout(repo_root),
    )


def _cleanup_owned_bundle(bundle: _OwnedBundle) -> None:
    response = _run_wsl_json(
        _WSL_BUNDLE_CLEANUP_BOOTSTRAP,
        (),
        {
            "bundle_root": bundle.root,
            "expected_children": {
                prefix: list(names) for prefix, names in bundle.layout.expected_children.items()
            },
            "expected_directories": list(bundle.layout.expected_directories),
            "expected_files": list(bundle.layout.expected_files),
            "name": bundle.name,
            "parent_device": bundle.parent_device,
            "parent_inode": bundle.parent_inode,
            "root_device": bundle.root_device,
            "root_inode": bundle.root_inode,
            "runtime_parent": bundle.runtime_parent,
            "schema": _BUNDLE_CLEANUP_SCHEMA,
        },
        "P57-WSL-CLEANUP",
    )
    if (
        set(response)
        != {
            "bundle_absent",
            "bundle_root",
            "directories_removed",
            "files_removed",
            "parent_sync_status",
            "schema",
        }
        or response["schema"] != _BUNDLE_CLEANUP_SCHEMA
        or response["bundle_root"] != bundle.root
        or response["bundle_absent"] is not True
        or response["parent_sync_status"] not in {"synced", "unsupported"}
        or response["files_removed"] != len(bundle.layout.expected_files)
        or response["directories_removed"] != len(bundle.layout.expected_directories)
    ):
        raise ExecutorFailure("P57-WSL-CLEANUP")


def _spawn_wsl_worker(bundle: _OwnedBundle, ubuntu_runtime_parent: str) -> subprocess.Popen[bytes]:
    command = (
        _P57._wsl_executable(),
        "--distribution",
        "Ubuntu-24.04",
        "--exec",
        "/usr/bin/python3",
        "-I",
        "-S",
        "-B",
        "-c",
        _P57._WSL_WORKER_BOOTSTRAP,
        bundle.root + "/worker/wsl_session_worker.py",
        _P57.WSL_WORKER_SHA256,
        "--runtime-parent",
        ubuntu_runtime_parent,
        "--bundle-root",
        bundle.root,
        "--p56-root",
        bundle.root
        + "/repository/docs/simulations/profile-diff-held-out/"
        + "pulse-56-retained-build-custody-release",
    )
    return subprocess.Popen(
        command,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=_P57._wsl_environment(),
    )


class _NativeWslSession(_P57._NativeWslSession):
    """The exact Pulse 57 worker route plus owned staged-bundle cleanup."""

    def __init__(self, repo_root: Path, ubuntu_runtime_parent: str, p51: ModuleType) -> None:
        _P57._native_wsl_parent(ubuntu_runtime_parent)
        self._bundle: _OwnedBundle | None = _stage_owned_bundle(repo_root, ubuntu_runtime_parent)
        try:
            self._process = _spawn_wsl_worker(self._bundle, ubuntu_runtime_parent)
        except OSError as error:
            try:
                self._cleanup_bundle()
            except BaseException as cleanup:
                raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
            raise ExecutorFailure("P57-WSL-UNAVAILABLE") from error
        self._p51 = p51
        self._closed = False
        self._launched = 0
        try:
            ready = self._read()
            if ready != {
                "count": REQUEST_COUNT,
                "platform": WSL_PLATFORM,
                "python": self._bundle.python_identity,
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

    def _cleanup_bundle(self) -> None:
        bundle = self._bundle
        self._bundle = None
        if bundle is not None:
            _cleanup_owned_bundle(bundle)

    def _abort(self) -> None:
        self._closed = True
        process_ended = False
        self._close_stdin()
        try:
            self._end_process()
            process_ended = True
            self._drain()
        except BaseException:
            if process_ended:
                try:
                    self._cleanup_bundle()
                except BaseException as cleanup:
                    raise ExecutorFailure("P57-WSL-CLEANUP") from cleanup
            raise
        self._cleanup_bundle()

    def close(self) -> None:
        if self._closed:
            return
        self._closed = True
        sent = True
        process_ended = False
        try:
            try:
                self._write(_P57._canonical_line({"schema": WSL_SCHEMA, "type": "close"}))
            except (
                ExecutorFailure,
                OSError,
                ValueError,
                subprocess.SubprocessError,
                TimeoutError,
            ):
                sent = False
            self._close_stdin()
            status = self._end_process()
            process_ended = True
            stdout, stderr = self._drain()
        except BaseException:
            if process_ended:
                try:
                    self._cleanup_bundle()
                except BaseException as cleanup:
                    raise ExecutorFailure("P57-WSL-CLEANUP") from cleanup
            raise
        try:
            self._cleanup_bundle()
        except BaseException as cleanup:
            raise ExecutorFailure("P57-WSL-CLEANUP") from cleanup
        if not sent or stdout or stderr or status != 0:
            raise ExecutorFailure("P57-WSL-CLEANUP")

    def launch(self, ordinal: int, arguments: tuple[str, ...]) -> object:
        if self._closed or ordinal != self._launched + 1 or len(arguments) != 7:
            raise ExecutorFailure("P57-WSL-PROTOCOL")
        request_id = _P57._request_id(ordinal, arguments)
        request = {
            "arguments": list(arguments),
            "ordinal": ordinal,
            "platform": WSL_PLATFORM,
            "request_id": request_id,
            "schema": WSL_SCHEMA,
            "type": "launch",
        }
        try:
            self._write(_P57._canonical_line(request))
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
            stdout = _P57.base64.b64decode(response["stdout_b64"], validate=True)
            stderr = _P57.base64.b64decode(response["stderr_b64"], validate=True)
            if (
                response["stdout_sha256"]
                != "sha256:" + _P57.hashlib.sha256(stdout).hexdigest()
                or response["stderr_sha256"]
                != "sha256:" + _P57.hashlib.sha256(stderr).hexdigest()
                or response["result_id"]
                != _P57._worker_result_id(
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
        ) as error:
            if getattr(error, "code", None) == "P57-INDETERMINATE-CLEANUP":
                raise
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


def run_capability_bound_diagnostic_executor(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    ubuntu_runtime_parent: str,
) -> ExecutorResult:
    """Run exact Pulse 57 semantics with owned native staged-bundle cleanup."""

    return _P57._execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        ubuntu_runtime_parent,
        _Controls(
            _P51,
            _P56,
            None,
            lambda root, parent, api: _NativeWslSession(root, parent, api),
        ),
    )


def _run_qualification_executor(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    controls: _Controls,
) -> ExecutorResult:
    """Private fake-only seam used by Pulse 69 qualification and successors."""

    return _P57._execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        "/home/pulse69-qualification",
        controls,
    )


__all__ = [
    "ExecutorFailure",
    "ExecutorResult",
    "release_identities",
    "run_capability_bound_diagnostic_executor",
]
