"""Pulse 86 stage-capture/bootstrap-argv successor to exact Pulse 75."""

from __future__ import annotations

import hashlib
import json
import os
import socket
import stat
import subprocess
import sys
import threading
import uuid
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType


LOCAL_SEALED_DEPENDENCIES_SHA256 = (
    "sha256:ed17b71f2a4ccd2850c8b2290a70ef95ba3355252045830d23802e84eceff247"
)
_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX = (
    "ferris.pulse-86.local-sealed-dependencies.runtime"
)
_LOCAL_SLOT_MISSING = object()
_CROSS_INSTANCE_REENTRY_STATE_KEY = (
    "_ferris_p86_cross_instance_reentry_advisory_state_v1"
)


class _Pulse86LinuxLockManager:
    def __init__(self) -> None:
        self._active_states: dict[int, object] = {}
        self._guard = threading.Lock()
        self._fork_hook_registrations = 0
        if os.name == "posix" and sys.platform.startswith("linux"):
            register_at_fork = getattr(os, "register_at_fork", None)
            if not callable(register_at_fork):
                raise RuntimeError("P86-LOCAL-SEALED-FORK-HOOK")
            register_at_fork(after_in_child=self._after_in_child)
            self._fork_hook_registrations += 1

    def _cross_instance_reentry_state(self) -> dict[str, object]:
        state = getattr(threading, _CROSS_INSTANCE_REENTRY_STATE_KEY, None)
        guard = state.get("guard") if type(state) is dict else None
        owners = state.get("owners") if type(state) is dict else None
        if callable(getattr(guard, "acquire", None)) and callable(
            getattr(guard, "release", None)
        ) and type(owners) is dict:
            return state
        fresh = {"guard": threading.Lock(), "owners": {}}
        setattr(threading, _CROSS_INSTANCE_REENTRY_STATE_KEY, fresh)
        return fresh

    def advisory_conflict(self, lock_name: str, owner_pid: int, owner_thread_id: int) -> bool:
        state = self._cross_instance_reentry_state()
        key = (lock_name, owner_pid, owner_thread_id)
        with state["guard"]:
            return key in state["owners"]

    def advisory_mark(self, active_state: object) -> None:
        state = self._cross_instance_reentry_state()
        key = (
            getattr(getattr(active_state, "lock_state", None), "name", None),
            getattr(active_state, "owner_pid", None),
            getattr(active_state, "owner_thread_id", None),
        )
        with state["guard"]:
            state["owners"][key] = getattr(active_state, "owner_token", active_state)

    def advisory_clear(self, active_state: object) -> None:
        state = self._cross_instance_reentry_state()
        key = (
            getattr(getattr(active_state, "lock_state", None), "name", None),
            getattr(active_state, "owner_pid", None),
            getattr(active_state, "owner_thread_id", None),
        )
        owner_token = getattr(active_state, "owner_token", active_state)
        with state["guard"]:
            if state["owners"].get(key) is owner_token:
                state["owners"].pop(key, None)

    def register_active_lock_state(self, active_state: object) -> None:
        with self._guard:
            self._active_states[id(active_state)] = active_state

    def unregister_active_lock_state(self, active_state: object) -> None:
        with self._guard:
            self._active_states.pop(id(active_state), None)

    def _after_in_child(self) -> None:
        states = tuple(self._active_states.values())
        self._active_states = {}
        self._guard = threading.Lock()
        setattr(
            threading,
            _CROSS_INSTANCE_REENTRY_STATE_KEY,
            {"guard": threading.Lock(), "owners": {}},
        )
        for active_state in states:
            try:
                owner_token = getattr(active_state, "owner_token", None)
                if owner_token is not None:
                    owner_token.live = False
                lock_state = getattr(active_state, "lock_state", None)
                if lock_state is not None:
                    handle = getattr(lock_state, "handle", None)
                    lock_state.handle = None
                    if isinstance(handle, socket.socket):
                        handle.close()
                if hasattr(active_state, "depth"):
                    active_state.depth = 0
            except BaseException:
                os._exit(97)


_P86_INTERNAL_LOCK_MANAGER = _Pulse86LinuxLockManager()


class _LocalSealedBootstrapFailure(RuntimeError):
    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)



def _local_sealed_dependencies_path() -> Path:
    try:
        return Path(__file__).resolve(strict=True).with_name("sealed_dependencies.py")
    except OSError as error:
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-PATH") from error



def _safe_local_regular(path: Path, code: str, maximum: int = 4_194_304) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise _LocalSealedBootstrapFailure(code)
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except _LocalSealedBootstrapFailure:
        raise
    except OSError as error:
        raise _LocalSealedBootstrapFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise _LocalSealedBootstrapFailure(code)
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > maximum:
                raise _LocalSealedBootstrapFailure(code)
        return bytes(content)
    except _LocalSealedBootstrapFailure:
        raise
    except OSError as error:
        raise _LocalSealedBootstrapFailure(code) from error
    finally:
        os.close(descriptor)



def _local_sealed_source_digest(content: bytes) -> str:
    return "sha256:" + hashlib.sha256(content).hexdigest()



def _verified_local_sealed_content(path: Path) -> bytes:
    content = _safe_local_regular(path, "P86-LOCAL-SEALED-IMPORT")
    if _local_sealed_source_digest(content) != LOCAL_SEALED_DEPENDENCIES_SHA256:
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-IDENTITY")
    return content



def _bind_local_sealed_lock_manager_module(module: ModuleType) -> ModuleType:
    if not isinstance(module, ModuleType):
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE")
    try:
        module_path = Path(module.__file__).resolve(strict=True)
    except (AttributeError, OSError, TypeError) as error:
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE") from error
    if module_path != _local_sealed_dependencies_path():
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE")
    binder = getattr(module, "_bind_internal_lock_manager", None)
    if not callable(binder):
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE")
    binder(_P86_INTERNAL_LOCK_MANAGER)
    return module



def _exec_local_sealed_module(path: Path, content: bytes) -> ModuleType:
    name = (
        f"{_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX}."
        f"{os.getpid()}.{uuid.uuid4().hex}"
    )
    if sys.modules.get(name) is not None:
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE")
    module = ModuleType(name)
    module.__file__ = os.fspath(path)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[name] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except BaseException as error:
        current = sys.modules.get(name)
        if current is module:
            sys.modules.pop(name, None)
        else:
            raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE") from error
        if isinstance(error, _LocalSealedBootstrapFailure):
            raise
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-IMPORT") from error
    current = sys.modules.get(name)
    if current is not module:
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE")
    sys.modules.pop(name, None)
    if not callable(getattr(module, "load_exact_p75_stack", None)) or not isinstance(
        getattr(module, "SealedDependencyFailure", None), type
    ):
        raise _LocalSealedBootstrapFailure("P86-LOCAL-SEALED-STATE")
    return _bind_local_sealed_lock_manager_module(module)



def _load_local_sealed_dependencies() -> ModuleType:
    path = _local_sealed_dependencies_path()
    content = _verified_local_sealed_content(path)
    return _exec_local_sealed_module(path, content)


ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
_SEALED = _load_local_sealed_dependencies()
_P75, _P57, _P51, _P56 = _SEALED.load_exact_p75_stack(REPO_ROOT)
SealedDependencyFailure = _SEALED.SealedDependencyFailure
release_identities = _SEALED.release_identities

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

_BUNDLE_STAGE_SCHEMA = "ferris.pulse-86-wsl-bundle-staged/v1"
_BUNDLE_REVALIDATION_SCHEMA = "ferris.pulse-86-staged-bundle-revalidation/v1"
_BUNDLE_CLEANUP_SCHEMA = "ferris.pulse-86-staged-bundle-cleanup/v1"
WSL_WORKER_SHA256 = "sha256:8e2e2f222fe0dfb4f7859e3a7a34bf03e7cf4fcfbf4a9104946d3acb5beccbef"
WORKER_SEALED_DEPENDENCIES_SHA256 = "sha256:7faf5df3e98fe27678ee8f1d541839e4b2c885e42e35e0b3171c02d9b3f56b56"
_P56_RELEASE_DIRECTORY = (
    "repository/docs/simulations/profile-diff-held-out/"
    "pulse-56-retained-build-custody-release"
)


@dataclass(frozen=True)
class _BundleLayout:
    expected_files: tuple[str, ...]
    expected_directories: tuple[str, ...]
    expected_children: dict[str, tuple[str, ...]]


@dataclass(frozen=True)
class _OwnedBundle:
    root: str
    runtime_parent: str
    owner_username: str
    owner_uid: int
    name: str
    python_identity: dict[str, object]
    root_device: int
    root_inode: int
    root_type: str
    parent_device: int
    parent_inode: int
    parent_type: str
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


_WSL_PARENT_OWNER_SCHEMA = "ferris.pulse-86-wsl-parent-owner/v1"
_WSL_PARENT_OWNER_BOOTSTRAP = r"""
import json,os,pwd,stat,sys
parent=sys.argv[1]
if not parent.startswith("/") or parent.startswith("/mnt/") or "\x00" in parent or "\r" in parent or "\n" in parent: raise SystemExit(2)
metadata=os.stat(parent,follow_symlinks=False)
if not stat.S_ISDIR(metadata.st_mode): raise SystemExit(2)
account=pwd.getpwuid(metadata.st_uid)
if account.pw_uid!=metadata.st_uid or not account.pw_name or "\x00" in account.pw_name or "\r" in account.pw_name or "\n" in account.pw_name: raise SystemExit(2)
response={"owner_uid":metadata.st_uid,"schema":"ferris.pulse-86-wsl-parent-owner/v1","username":account.pw_name}
sys.stdout.buffer.write(json.dumps(response,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")+b"\n")
"""


@dataclass(frozen=True)
class _WslOwner:
    username: str
    uid: int


def _wsl_python_command(
    username: str,
    script: str,
    arguments: tuple[str, ...] = (),
) -> tuple[str, ...]:
    if (
        type(username) is not str
        or not username
        or "\x00" in username
        or "\r" in username
        or "\n" in username
    ):
        raise ExecutorFailure("P86-WSL-OWNER")
    return (
        _P57._wsl_executable(),
        "--distribution",
        "Ubuntu-24.04",
        "--user",
        username,
        "--exec",
        "/usr/bin/python3",
        "-I",
        "-S",
        "-B",
        "-c",
        script,
        *arguments,
    )


def _resolve_wsl_parent_owner(runtime_parent: str) -> _WslOwner:
    _P57._native_wsl_parent(runtime_parent)
    command = _wsl_python_command(
        "root",
        _WSL_PARENT_OWNER_BOOTSTRAP,
        (runtime_parent,),
    )
    try:
        completed = subprocess.run(
            command,
            check=False,
            env=_P57._wsl_environment(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=PROTOCOL_TIMEOUT_SECONDS,
        )
    except (OSError, subprocess.SubprocessError, TimeoutError) as error:
        raise ExecutorFailure("P86-WSL-OWNER") from error
    if completed.returncode != 0 or completed.stderr:
        raise ExecutorFailure("P86-WSL-OWNER")
    try:
        response = _P57._parse_line(completed.stdout, MAX_PROTOCOL_BYTES)
    except ExecutorFailure as error:
        raise ExecutorFailure("P86-WSL-OWNER") from error
    if (
        set(response) != {"owner_uid", "schema", "username"}
        or response["schema"] != _WSL_PARENT_OWNER_SCHEMA
        or type(response["owner_uid"]) is not int
        or response["owner_uid"] < 0
        or type(response["username"]) is not str
        or not response["username"]
        or "\x00" in response["username"]
        or "\r" in response["username"]
        or "\n" in response["username"]
    ):
        raise ExecutorFailure("P86-WSL-OWNER")
    return _WslOwner(response["username"], response["owner_uid"])


def _run_wsl_json(
    username: str,
    script: str,
    arguments: tuple[str, ...],
    payload: object | None,
    code: str,
) -> dict[str, object]:
    command = _wsl_python_command(username, script, arguments)
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


_WSL_BUNDLE_STAGE_BOOTSTRAP = r"""
import base64,json,os,stat,sys
maximum=1048576
raw=sys.stdin.buffer.read(maximum+1)
if len(raw)>maximum: raise SystemExit(2)
request=json.loads(raw)
if json.dumps(request,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")!=raw: raise SystemExit(2)
if type(request) is not dict or set(request)!={"files","schema"} or request["schema"]!="ferris.pulse-57-wsl-bundle/v1" or type(request["files"]) is not list: raise SystemExit(2)
parent=sys.argv[1]
name=sys.argv[2]
expected_owner_uid=int(sys.argv[3])
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
flags=os.O_RDONLY|getattr(os,"O_DIRECTORY",0)|getattr(os,"O_NOFOLLOW",0)
def _emit(value):
 sys.stdout.buffer.write(json.dumps(value,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")+b"\n")
 sys.stdout.buffer.flush()
def _stage_failure(root_path,failure_code,bundle_absent_verified,cleanup_posture):
 return {
  "bundle_absent_verified":bundle_absent_verified,
  "bundle_root":root_path,
  "cleanup_posture":cleanup_posture,
  "failure_code":failure_code,
  "schema":"ferris.pulse-86-wsl-bundle-staged/v1",
  "status":"failed",
 }
def _open_directory(parent_fd,name,create):
 if create:
  try:
   os.mkdir(name,0o700,dir_fd=parent_fd)
  except FileExistsError:
   pass
 metadata=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode): raise SystemExit(2)
 fd=os.open(name,flags,dir_fd=parent_fd)
 opened=os.fstat(fd)
 if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(metadata.st_dev,metadata.st_ino):
  os.close(fd)
  raise SystemExit(2)
 return fd
def _capture_owned_root(parent_fd,name):
 metadata=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode): raise RuntimeError
 fd=os.open(name,flags,dir_fd=parent_fd)
 try:
  opened=os.fstat(fd)
  if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(metadata.st_dev,metadata.st_ino): raise RuntimeError
  return fd,(opened.st_dev,opened.st_ino)
 except BaseException:
  os.close(fd)
  raise
def _remove_tree(fd):
 listing=tuple(sorted(os.listdir("/proc/self/fd/"+str(fd))))
 files_removed=0
 directories_removed=0
 for child in listing:
  metadata=os.lstat(child,dir_fd=fd)
  if stat.S_ISLNK(metadata.st_mode): raise RuntimeError
  if stat.S_ISDIR(metadata.st_mode):
   child_fd=os.open(child,flags,dir_fd=fd)
   try:
    opened=os.fstat(child_fd)
    if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(metadata.st_dev,metadata.st_ino): raise RuntimeError
    child_files,child_directories=_remove_tree(child_fd)
   finally:
    os.close(child_fd)
   os.rmdir(child,dir_fd=fd)
   files_removed+=child_files
   directories_removed+=child_directories+1
  elif stat.S_ISREG(metadata.st_mode):
   os.unlink(child,dir_fd=fd)
   files_removed+=1
  else:
   raise RuntimeError
 return files_removed,directories_removed
def _cleanup_created_root(parent_fd,root_fd,name,root_path,created_identity):
 try:
  reopened=os.fstat(root_fd)
  if not stat.S_ISDIR(reopened.st_mode) or (reopened.st_dev,reopened.st_ino)!=created_identity: raise RuntimeError
  _remove_tree(root_fd)
  current=os.lstat(name,dir_fd=parent_fd)
  if stat.S_ISLNK(current.st_mode) or not stat.S_ISDIR(current.st_mode): raise RuntimeError
  if (current.st_dev,current.st_ino)!=created_identity: raise RuntimeError
  os.rmdir(name,dir_fd=parent_fd)
  try:
   os.lstat(name,dir_fd=parent_fd)
   raise RuntimeError
  except FileNotFoundError:
   pass
  return _stage_failure(root_path,"P57-WSL-BUNDLE",True,"removed")
 except BaseException:
  return _stage_failure(root_path,"P57-INDETERMINATE-CLEANUP",False,"indeterminate")
parent_fd=os.open(parent,flags)
try:
 parent_stat=os.fstat(parent_fd)
 if os.geteuid()!=expected_owner_uid or parent_stat.st_uid!=expected_owner_uid: raise SystemExit(2)
 os.mkdir(name,0o700,dir_fd=parent_fd)
 root_path=parent.rstrip("/")+"/"+name
 try:
  root_fd,created_identity=_capture_owned_root(parent_fd,name)
 except BaseException:
  _emit(_stage_failure(root_path,"P86-INDETERMINATE-STAGE-CLEANUP",False,"indeterminate"))
  raise SystemExit(0)
 try:
  try:
   for entry in request["files"]:
    if type(entry) is not dict or set(entry)!={"bytes_b64","path"} or type(entry["path"]) is not str or type(entry["bytes_b64"]) is not str:
     raise SystemExit(2)
    parts=entry["path"].split("/")
    if not parts or any(not part or part in {".",".."} for part in parts): raise SystemExit(2)
    data=base64.b64decode(entry["bytes_b64"],validate=True)
    current_fd=root_fd
    opened=[]
    try:
     for directory in parts[:-1]:
      next_fd=_open_directory(current_fd,directory,True)
      opened.append(next_fd)
      current_fd=next_fd
     descriptor=os.open(parts[-1],os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o500,dir_fd=current_fd)
     try:
      offset=0
      while offset<len(data):
       offset+=os.write(descriptor,data[offset:])
     finally:
      os.close(descriptor)
    finally:
     for handle in reversed(opened):
      os.close(handle)
   final_root=os.lstat(name,dir_fd=parent_fd)
   final_parent=os.fstat(parent_fd)
   if stat.S_ISLNK(final_root.st_mode) or not stat.S_ISDIR(final_root.st_mode): raise SystemExit(2)
   if (final_parent.st_dev,final_parent.st_ino)!=(parent_stat.st_dev,parent_stat.st_ino): raise SystemExit(2)
   if (final_root.st_dev,final_root.st_ino)!=created_identity: raise SystemExit(2)
   reopened=os.fstat(root_fd)
   if not stat.S_ISDIR(reopened.st_mode) or (reopened.st_dev,reopened.st_ino)!=created_identity: raise SystemExit(2)
   _emit({
    "bundle_root": root_path,
    "parent_device": final_parent.st_dev,
    "parent_inode": final_parent.st_ino,
    "parent_type": "directory",
    "python": {"executable": sys.executable, "version": list(sys.version_info[:3])},
    "root_device": reopened.st_dev,
    "root_inode": reopened.st_ino,
    "root_type": "directory",
    "schema": "ferris.pulse-86-wsl-bundle-staged/v1",
    "status": "staged",
   })
  except BaseException:
   _emit(_cleanup_created_root(parent_fd,root_fd,name,root_path,created_identity))
 finally:
  os.close(root_fd)
finally:
 os.close(parent_fd)
"""


_WSL_BUNDLE_REVALIDATION_BOOTSTRAP = r"""
import json,os,stat,sys
maximum=131072
raw=sys.stdin.buffer.read(maximum+1)
if len(raw)>maximum: raise SystemExit(2)
request=json.loads(raw)
if json.dumps(request,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")!=raw: raise SystemExit(2)
parent=sys.argv[1]
name=sys.argv[2]
expected_root=sys.argv[3]
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
if expected_root!=parent.rstrip("/")+"/"+name: raise SystemExit(2)
required={"owner_uid","parent_device","parent_inode","parent_type","root_device","root_inode","root_type","schema"}
if type(request) is not dict or set(request)!=required or request["schema"]!="ferris.pulse-86-staged-bundle-revalidation/v1": raise SystemExit(2)
if request["parent_type"]!="directory" or request["root_type"]!="directory": raise SystemExit(2)
flags=os.O_RDONLY|getattr(os,"O_DIRECTORY",0)|getattr(os,"O_NOFOLLOW",0)
parent_fd=os.open(parent,flags)
try:
 parent_stat=os.fstat(parent_fd)
 if os.geteuid()!=request["owner_uid"] or not stat.S_ISDIR(parent_stat.st_mode) or parent_stat.st_uid!=request["owner_uid"] or (parent_stat.st_dev,parent_stat.st_ino)!=(request["parent_device"],request["parent_inode"]): raise SystemExit(2)
 root_stat=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(root_stat.st_mode) or not stat.S_ISDIR(root_stat.st_mode): raise SystemExit(2)
 if (root_stat.st_dev,root_stat.st_ino)!=(request["root_device"],request["root_inode"]): raise SystemExit(2)
 root_fd=os.open(name,flags,dir_fd=parent_fd)
 try:
  opened=os.fstat(root_fd)
  if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(request["root_device"],request["root_inode"]): raise SystemExit(2)
 finally:
  os.close(root_fd)
 response={"bundle_root": expected_root, "schema": "ferris.pulse-86-staged-bundle-revalidation/v1", "verified": True}
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
required={"bundle_root","expected_children","expected_directories","expected_files","name","owner_uid","parent_device","parent_inode","parent_type","root_device","root_inode","root_type","runtime_parent","schema"}
if type(request) is not dict or set(request)!=required or request["schema"]!="ferris.pulse-86-staged-bundle-cleanup/v1": raise SystemExit(2)
parent=request["runtime_parent"]
name=request["name"]
expected_root=request["bundle_root"]
if type(parent) is not str or type(name) is not str or type(expected_root) is not str: raise SystemExit(2)
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
if expected_root!=parent.rstrip("/")+"/"+name: raise SystemExit(2)
if request["parent_type"]!="directory" or request["root_type"]!="directory": raise SystemExit(2)
expected_files=request["expected_files"]
expected_directories=request["expected_directories"]
expected_children=request["expected_children"]
if type(expected_files) is not list or type(expected_directories) is not list or type(expected_children) is not dict: raise SystemExit(2)
file_set={path for path in expected_files if type(path) is str and path and not path.startswith("/") and ".." not in path.split("/")}
directory_set={path for path in expected_directories if type(path) is str and path and not path.startswith("/") and ".." not in path.split("/")}
if len(file_set)!=len(expected_files) or len(directory_set)!=len(expected_directories) or file_set&directory_set: raise SystemExit(2)
normalized_children={}
for prefix,names in expected_children.items():
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
 if os.geteuid()!=request["owner_uid"] or not stat.S_ISDIR(parent_stat.st_mode) or parent_stat.st_uid!=request["owner_uid"] or (parent_stat.st_dev,parent_stat.st_ino)!=(request["parent_device"],request["parent_inode"]): raise SystemExit(2)
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
  "schema": "ferris.pulse-86-staged-bundle-cleanup/v1",
 }
 sys.stdout.buffer.write(json.dumps(response,allow_nan=False,ensure_ascii=True,separators=(",",":"),sort_keys=True).encode("ascii")+b"\n")
 sys.stdout.buffer.flush()
finally:
 os.close(parent_fd)
"""


def _stage_owned_bundle(repo_root: Path, ubuntu_runtime_parent: str) -> _OwnedBundle:
    _P57._native_wsl_parent(ubuntu_runtime_parent)
    owner = _resolve_wsl_parent_owner(ubuntu_runtime_parent)
    release_root = ROOT
    files = [
        {
            "path": "worker/wsl_session_worker.py",
            "bytes_b64": _P57.base64.b64encode(
                _P57._source_with_identity(release_root / "wsl_session_worker.py", WSL_WORKER_SHA256)
            ).decode("ascii"),
        },
        {
            "path": "worker/sealed_dependencies.py",
            "bytes_b64": _P57.base64.b64encode(
                _P57._source_with_identity(
                    release_root / "worker_sealed_dependencies.py",
                    WORKER_SEALED_DEPENDENCIES_SHA256,
                )
            ).decode("ascii"),
        },
        *_P57._p56_bundle_files(repo_root),
    ]
    payload = _P57.canonical_bytes({"files": files, "schema": _P57.BUNDLE_SCHEMA})
    if len(payload) > _P57.MAX_BUNDLE_BYTES:
        raise ExecutorFailure("P57-WSL-BUNDLE")
    name = ".p57-" + _P57.secrets.token_hex(16)
    command = _wsl_python_command(
        owner.username,
        _WSL_BUNDLE_STAGE_BOOTSTRAP,
        (ubuntu_runtime_parent, name, str(owner.uid)),
    )
    try:
        completed = subprocess.run(
            command,
            check=False,
            env=_P57._wsl_environment(),
            input=payload,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=PROTOCOL_TIMEOUT_SECONDS,
        )
    except (OSError, subprocess.SubprocessError, TimeoutError) as error:
        raise ExecutorFailure("P57-WSL-BUNDLE") from error
    if completed.returncode != 0 or completed.stderr:
        raise ExecutorFailure("P57-WSL-BUNDLE")
    response = _P57._parse_line(completed.stdout, MAX_PROTOCOL_BYTES)
    expected_root = ubuntu_runtime_parent.rstrip("/") + "/" + name
    python_identity = response.get("python")
    if (
        set(response)
        == {
            "bundle_absent_verified",
            "bundle_root",
            "cleanup_posture",
            "failure_code",
            "schema",
            "status",
        }
        and response["schema"] == _BUNDLE_STAGE_SCHEMA
        and response["status"] == "failed"
        and response["bundle_root"] == expected_root
        and response["cleanup_posture"] in {"removed", "indeterminate"}
        and type(response["bundle_absent_verified"]) is bool
        and type(response["failure_code"]) is str
    ):
        if response["failure_code"] == "P86-INDETERMINATE-STAGE-CLEANUP":
            raise ExecutorFailure("P86-INDETERMINATE-STAGE-CLEANUP")
        if response["failure_code"] == "P57-INDETERMINATE-CLEANUP":
            raise ExecutorFailure("P57-INDETERMINATE-CLEANUP")
        if (
            response["failure_code"] == "P57-WSL-BUNDLE"
            and response["cleanup_posture"] == "removed"
            and response["bundle_absent_verified"] is True
        ):
            raise ExecutorFailure("P57-WSL-BUNDLE")
        raise ExecutorFailure("P57-INDETERMINATE-CLEANUP")
    if (
        set(response)
        != {
            "bundle_root",
            "parent_device",
            "parent_inode",
            "parent_type",
            "python",
            "root_device",
            "root_inode",
            "root_type",
            "schema",
            "status",
        }
        or response["schema"] != _BUNDLE_STAGE_SCHEMA
        or response["status"] != "staged"
        or response["bundle_root"] != expected_root
        or response["parent_type"] != "directory"
        or response["root_type"] != "directory"
        or any(
            type(response[field]) is not int or response[field] < 0
            for field in ("parent_device", "parent_inode", "root_device", "root_inode")
        )
        or type(python_identity) is not dict
        or set(python_identity) != {"executable", "version"}
        or python_identity["executable"] != "/usr/bin/python3"
        or type(python_identity["version"]) is not list
        or len(python_identity["version"]) != 3
        or any(type(part) is not int or part < 0 for part in python_identity["version"])
    ):
        raise ExecutorFailure("P57-WSL-BUNDLE")
    return _OwnedBundle(
        root=expected_root,
        runtime_parent=ubuntu_runtime_parent,
        owner_username=owner.username,
        owner_uid=owner.uid,
        name=name,
        python_identity=python_identity,
        root_device=response["root_device"],
        root_inode=response["root_inode"],
        root_type=response["root_type"],
        parent_device=response["parent_device"],
        parent_inode=response["parent_inode"],
        parent_type=response["parent_type"],
        layout=_expected_bundle_layout(repo_root),
    )



def _revalidate_staged_bundle(bundle: _OwnedBundle) -> None:
    response = _run_wsl_json(
        bundle.owner_username,
        _WSL_BUNDLE_REVALIDATION_BOOTSTRAP,
        (bundle.runtime_parent, bundle.name, bundle.root),
        {
            "owner_uid": bundle.owner_uid,
            "parent_device": bundle.parent_device,
            "parent_inode": bundle.parent_inode,
            "parent_type": bundle.parent_type,
            "root_device": bundle.root_device,
            "root_inode": bundle.root_inode,
            "root_type": bundle.root_type,
            "schema": _BUNDLE_REVALIDATION_SCHEMA,
        },
        "P57-WSL-CLEANUP",
    )
    if (
        set(response) != {"bundle_root", "schema", "verified"}
        or response["schema"] != _BUNDLE_REVALIDATION_SCHEMA
        or response["bundle_root"] != bundle.root
        or response["verified"] is not True
    ):
        raise ExecutorFailure("P57-WSL-CLEANUP")



def _cleanup_owned_bundle(bundle: _OwnedBundle) -> None:
    response = _run_wsl_json(
        bundle.owner_username,
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
            "owner_uid": bundle.owner_uid,
            "parent_device": bundle.parent_device,
            "parent_inode": bundle.parent_inode,
            "parent_type": bundle.parent_type,
            "root_device": bundle.root_device,
            "root_inode": bundle.root_inode,
            "root_type": bundle.root_type,
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


_WSL_WORKER_BOOTSTRAP = r"""
import hashlib,os,stat,sys
maximum=1048576
parent=sys.argv[1]
name=sys.argv[2]
expected_root=sys.argv[3]
expected_owner_uid=int(sys.argv[4])
expected_parent_device=int(sys.argv[5])
expected_parent_inode=int(sys.argv[6])
expected_root_device=int(sys.argv[7])
expected_root_inode=int(sys.argv[8])
worker_relative=sys.argv[9]
expected_worker_sha256=sys.argv[10]
dependency_relative=sys.argv[11]
expected_dependency_sha256=sys.argv[12]
flags=os.O_RDONLY|getattr(os,"O_DIRECTORY",0)|getattr(os,"O_NOFOLLOW",0)
if not parent.startswith("/") or parent.startswith("/mnt/") or "/" in name or not name.startswith(".p57-"): raise SystemExit(2)
if expected_root!=parent.rstrip("/")+"/"+name: raise SystemExit(2)
if worker_relative!="worker/wsl_session_worker.py": raise SystemExit(2)
if dependency_relative!="worker/sealed_dependencies.py": raise SystemExit(2)
def _open_directory(parent_fd,name):
 metadata=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode): raise SystemExit(2)
 fd=os.open(name,flags,dir_fd=parent_fd)
 opened=os.fstat(fd)
 if not stat.S_ISDIR(opened.st_mode) or (opened.st_dev,opened.st_ino)!=(metadata.st_dev,metadata.st_ino):
  os.close(fd)
  raise SystemExit(2)
 return fd
parent_fd=os.open(parent,flags)
try:
 parent_stat=os.fstat(parent_fd)
 if os.geteuid()!=expected_owner_uid or not stat.S_ISDIR(parent_stat.st_mode) or parent_stat.st_uid!=expected_owner_uid or (parent_stat.st_dev,parent_stat.st_ino)!=(expected_parent_device,expected_parent_inode): raise SystemExit(2)
 root_stat=os.lstat(name,dir_fd=parent_fd)
 if stat.S_ISLNK(root_stat.st_mode) or not stat.S_ISDIR(root_stat.st_mode): raise SystemExit(2)
 if (root_stat.st_dev,root_stat.st_ino)!=(expected_root_device,expected_root_inode): raise SystemExit(2)
 root_fd=os.open(name,flags,dir_fd=parent_fd)
 try:
  opened_root=os.fstat(root_fd)
  if not stat.S_ISDIR(opened_root.st_mode) or (opened_root.st_dev,opened_root.st_ino)!=(expected_root_device,expected_root_inode): raise SystemExit(2)
  worker_dir_fd=_open_directory(root_fd,"worker")
  try:
   dependency_stat=os.lstat("sealed_dependencies.py",dir_fd=worker_dir_fd)
   if stat.S_ISLNK(dependency_stat.st_mode) or not stat.S_ISREG(dependency_stat.st_mode): raise SystemExit(2)
   dependency_fd=os.open("sealed_dependencies.py",os.O_RDONLY|getattr(os,"O_NOFOLLOW",0),dir_fd=worker_dir_fd)
   try:
    opened_dependency=os.fstat(dependency_fd)
    if not stat.S_ISREG(opened_dependency.st_mode) or (opened_dependency.st_dev,opened_dependency.st_ino)!=(dependency_stat.st_dev,dependency_stat.st_ino): raise SystemExit(2)
    dependency=bytearray()
    while True:
     chunk=os.read(dependency_fd,65536)
     if not chunk: break
     dependency.extend(chunk)
     if len(dependency)>maximum: raise SystemExit(2)
    if "sha256:"+hashlib.sha256(bytes(dependency)).hexdigest()!=expected_dependency_sha256: raise SystemExit(2)
   finally:
    os.close(dependency_fd)
   worker_stat=os.lstat("wsl_session_worker.py",dir_fd=worker_dir_fd)
   if stat.S_ISLNK(worker_stat.st_mode) or not stat.S_ISREG(worker_stat.st_mode): raise SystemExit(2)
   worker_fd=os.open("wsl_session_worker.py",os.O_RDONLY|getattr(os,"O_NOFOLLOW",0),dir_fd=worker_dir_fd)
   try:
    opened_worker=os.fstat(worker_fd)
    if not stat.S_ISREG(opened_worker.st_mode) or (opened_worker.st_dev,opened_worker.st_ino)!=(worker_stat.st_dev,worker_stat.st_ino): raise SystemExit(2)
    content=bytearray()
    while True:
     chunk=os.read(worker_fd,65536)
     if not chunk: break
     content.extend(chunk)
     if len(content)>maximum: raise SystemExit(2)
    source=bytes(content)
    if "sha256:"+hashlib.sha256(source).hexdigest()!=expected_worker_sha256: raise SystemExit(2)
    filename="/proc/self/fd/"+str(worker_fd)
    sys.argv=[expected_root+"/"+worker_relative,*sys.argv[13:]]
    namespace={"__name__":"__main__","__file__":filename,"__package__":None}
    exec(compile(source,filename,"exec"),namespace)
   finally:
    os.close(worker_fd)
  finally:
   os.close(worker_dir_fd)
 finally:
  os.close(root_fd)
finally:
 os.close(parent_fd)
 """


def _spawn_wsl_worker(bundle: _OwnedBundle, ubuntu_runtime_parent: str) -> subprocess.Popen[bytes]:
    command = _wsl_python_command(
        bundle.owner_username,
        _WSL_WORKER_BOOTSTRAP,
        (
            bundle.runtime_parent,
            bundle.name,
            bundle.root,
            str(bundle.owner_uid),
            str(bundle.parent_device),
            str(bundle.parent_inode),
            str(bundle.root_device),
            str(bundle.root_inode),
            "worker/wsl_session_worker.py",
            WSL_WORKER_SHA256,
            "worker/sealed_dependencies.py",
            WORKER_SEALED_DEPENDENCIES_SHA256,
            "--runtime-parent",
            ubuntu_runtime_parent,
            "--bundle-root",
            bundle.root,
            "--p56-root",
            bundle.root + "/" + _P56_RELEASE_DIRECTORY,
            "--expected-parent-device",
            str(bundle.parent_device),
            "--expected-parent-inode",
            str(bundle.parent_inode),
            "--expected-root-device",
            str(bundle.root_device),
            "--expected-root-inode",
            str(bundle.root_inode),
            "--expected-sealed-dependencies-sha256",
            WORKER_SEALED_DEPENDENCIES_SHA256,
        ),
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
            assert self._bundle is not None
            _revalidate_staged_bundle(self._bundle)
            self._process = _spawn_wsl_worker(self._bundle, ubuntu_runtime_parent)
        except ExecutorFailure as error:
            try:
                self._cleanup_bundle()
            except BaseException as cleanup:
                raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from cleanup
            raise ExecutorFailure("P57-INDETERMINATE-CLEANUP") from error
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
    """Run exact Pulse 57 semantics with stage-time identity binding and cleanup."""

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
    """Private fake-only seam used by Pulse 86 qualification and successors."""

    return _P57._execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        "/home/pulse78-qualification",
        controls,
    )


__all__ = [
    "ExecutorFailure",
    "ExecutorResult",
    "release_identities",
    "run_capability_bound_diagnostic_executor",
]
