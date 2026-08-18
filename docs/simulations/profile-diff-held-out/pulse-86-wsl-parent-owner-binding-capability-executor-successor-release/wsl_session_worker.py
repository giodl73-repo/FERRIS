"""Pulse 86 identity-bound WSL capability session worker."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
import stat
import sys
from pathlib import Path
from types import ModuleType
from typing import BinaryIO


sys.dont_write_bytecode = True


SCHEMA = "ferris.pulse-57-wsl-capability-session/v1"
PLATFORM = "ubuntu-24.04-x86_64"
REQUEST_COUNT = 69
MAX_REQUEST_BYTES = 16_384
MAX_RESPONSE_BYTES = 2_800_000
MAX_STREAM_BYTES = 1_048_576


class WorkerFailure(RuntimeError):
    """Protocol or custody failure without private path disclosure."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    value: dict[str, object] = {}
    for key, item in pairs:
        if key in value:
            raise WorkerFailure("P57-WSL-PROTOCOL")
        value[key] = item
    return value


def _canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def _line_value(raw: bytes, maximum: int) -> dict[str, object]:
    if not raw.endswith(b"\n") or len(raw) > maximum:
        raise WorkerFailure("P57-WSL-PROTOCOL")
    try:
        value = json.loads(raw[:-1], object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, WorkerFailure) as error:
        raise WorkerFailure("P57-WSL-PROTOCOL") from error
    if type(value) is not dict or _canonical_bytes(value) + b"\n" != raw:
        raise WorkerFailure("P57-WSL-PROTOCOL")
    return value


def _sha256_bytes(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def _request_id(ordinal: int, arguments: list[str]) -> str:
    return _sha256_bytes(_canonical_bytes({"arguments": arguments, "ordinal": ordinal}))


def _result_id(
    ordinal: int,
    request_id: str,
    returncode: int,
    stdout_sha256: str,
    stderr_sha256: str,
) -> str:
    return _sha256_bytes(
        _canonical_bytes(
            {
                "ordinal": ordinal,
                "platform": PLATFORM,
                "request_id": request_id,
                "returncode": returncode,
                "stderr_sha256": stderr_sha256,
                "stdout_sha256": stdout_sha256,
            }
        )
    )


def _native_directory(value: str, code: str) -> Path:
    if (
        type(value) is not str
        or not value.startswith("/")
        or value.startswith("/mnt/")
        or "\x00" in value
        or "\r" in value
        or "\n" in value
    ):
        raise WorkerFailure(code)
    try:
        path = Path(value).resolve(strict=True)
        metadata = os.lstat(path)
        if (
            stat.S_ISLNK(metadata.st_mode)
            or not stat.S_ISDIR(metadata.st_mode)
            or str(path).startswith("/mnt/")
        ):
            raise WorkerFailure(code)
        return path
    except WorkerFailure:
        raise
    except OSError as error:
        raise WorkerFailure(code) from error


class _BoundBundleRoot:
    def __init__(
        self,
        runtime_parent: str,
        bundle_root: str,
        expected_parent_device: int,
        expected_parent_inode: int,
        expected_root_device: int,
        expected_root_inode: int,
    ) -> None:
        if (
            type(bundle_root) is not str
            or not bundle_root.startswith("/")
            or bundle_root.startswith("/mnt/")
            or "\x00" in bundle_root
            or "\r" in bundle_root
            or "\n" in bundle_root
        ):
            raise WorkerFailure("P57-WSL-BUNDLE")
        if not bundle_root.startswith(runtime_parent.rstrip("/") + "/"):
            raise WorkerFailure("P57-WSL-BUNDLE")
        name = bundle_root.rsplit("/", 1)[-1]
        if "/" in name or not name.startswith(".p57-"):
            raise WorkerFailure("P57-WSL-BUNDLE")
        flags = os.O_RDONLY | getattr(os, "O_DIRECTORY", 0) | getattr(os, "O_NOFOLLOW", 0)
        try:
            parent_fd = os.open(runtime_parent, flags)
        except OSError as error:
            raise WorkerFailure("P57-WSL-BUNDLE") from error
        self._flags = flags
        self._runtime_parent = runtime_parent
        self._bundle_root = bundle_root
        self._name = name
        self._parent_fd = parent_fd
        self._root_fd: int | None = None
        try:
            parent_stat = os.fstat(parent_fd)
            if not stat.S_ISDIR(parent_stat.st_mode) or (
                parent_stat.st_dev,
                parent_stat.st_ino,
            ) != (expected_parent_device, expected_parent_inode):
                raise WorkerFailure("P57-WSL-BUNDLE")
            root_stat = os.lstat(name, dir_fd=parent_fd)
            if stat.S_ISLNK(root_stat.st_mode) or not stat.S_ISDIR(root_stat.st_mode):
                raise WorkerFailure("P57-WSL-BUNDLE")
            if (root_stat.st_dev, root_stat.st_ino) != (
                expected_root_device,
                expected_root_inode,
            ):
                raise WorkerFailure("P57-WSL-BUNDLE")
            root_fd = os.open(name, flags, dir_fd=parent_fd)
            self._root_fd = root_fd
            opened = os.fstat(root_fd)
            if not stat.S_ISDIR(opened.st_mode) or (
                opened.st_dev,
                opened.st_ino,
            ) != (expected_root_device, expected_root_inode):
                raise WorkerFailure("P57-WSL-BUNDLE")
        except BaseException:
            self.close()
            raise

    @property
    def bundle_root(self) -> str:
        return self._bundle_root

    @property
    def root_fd(self) -> int:
        if self._root_fd is None:
            raise WorkerFailure("P57-WSL-BUNDLE")
        return self._root_fd

    def proc_prefix(self) -> str:
        return "/proc/self/fd/" + str(self.root_fd)

    def proc_path(self, relative: str) -> str:
        return self.proc_prefix() + "/" + relative

    def _open_directory(self, parent_fd: int, name: str, code: str) -> int:
        try:
            metadata = os.lstat(name, dir_fd=parent_fd)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise WorkerFailure(code)
            descriptor = os.open(name, self._flags, dir_fd=parent_fd)
        except WorkerFailure:
            raise
        except OSError as error:
            raise WorkerFailure(code) from error
        opened = os.fstat(descriptor)
        if not stat.S_ISDIR(opened.st_mode) or (
            opened.st_dev,
            opened.st_ino,
        ) != (metadata.st_dev, metadata.st_ino):
            os.close(descriptor)
            raise WorkerFailure(code)
        return descriptor

    def read_regular(self, relative: str, code: str, maximum: int = MAX_RESPONSE_BYTES) -> bytes:
        parts = relative.split("/")
        if not parts or any(not part or part in {".", ".."} for part in parts):
            raise WorkerFailure(code)
        current_fd = self.root_fd
        opened: list[int] = []
        try:
            for directory in parts[:-1]:
                next_fd = self._open_directory(current_fd, directory, code)
                opened.append(next_fd)
                current_fd = next_fd
            try:
                metadata = os.lstat(parts[-1], dir_fd=current_fd)
                if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
                    raise WorkerFailure(code)
                descriptor = os.open(
                    parts[-1],
                    os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0),
                    dir_fd=current_fd,
                )
            except WorkerFailure:
                raise
            except OSError as error:
                raise WorkerFailure(code) from error
            try:
                opened_stat = os.fstat(descriptor)
                if not stat.S_ISREG(opened_stat.st_mode) or (
                    opened_stat.st_dev,
                    opened_stat.st_ino,
                ) != (metadata.st_dev, metadata.st_ino):
                    raise WorkerFailure(code)
                content = bytearray()
                while chunk := os.read(descriptor, 65_536):
                    content.extend(chunk)
                    if len(content) > maximum:
                        raise WorkerFailure(code)
                return bytes(content)
            except OSError as error:
                raise WorkerFailure(code) from error
            finally:
                os.close(descriptor)
        finally:
            for handle in reversed(opened):
                os.close(handle)

    def close(self) -> None:
        if self._root_fd is not None:
            os.close(self._root_fd)
            self._root_fd = None
        if self._parent_fd is not None:
            os.close(self._parent_fd)
            self._parent_fd = None


def _load_sealed_dependencies(
    bundle: _BoundBundleRoot, expected_sha256: str
) -> ModuleType:
    relative = "worker/sealed_dependencies.py"
    content = bundle.read_regular(relative, "P57-WSL-BUNDLE")
    if _sha256_bytes(content) != expected_sha256:
        raise WorkerFailure("P57-WSL-BUNDLE")
    module = ModuleType("p75_worker_sealed_dependencies")
    module.__file__ = bundle.proc_path(relative)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[module.__name__] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except (ImportError, OSError, RuntimeError, SyntaxError, ValueError) as error:
        sys.modules.pop(module.__name__, None)
        raise WorkerFailure("P57-WSL-BUNDLE") from error
    if not callable(getattr(module, "load_exact_p56", None)):
        raise WorkerFailure("P57-WSL-BUNDLE")
    return module


def _p56_failure(p56: ModuleType, error: BaseException) -> bool:
    candidate = getattr(p56, "ReleaseFailure", None)
    return isinstance(candidate, type) and isinstance(error, candidate)


class WorkerProtocol:
    """Own exactly one P56 Ubuntu capability and exactly 69 ordered requests."""

    def __init__(self, p56: ModuleType, runtime_parent: Path) -> None:
        self._p56 = p56
        self._handle = p56.publish_retained_build_and_custody(PLATFORM, runtime_parent)
        self._ordinal = 0
        self._closed = False

    def ready(self) -> bytes:
        return _canonical_bytes(
            {
                "count": REQUEST_COUNT,
                "platform": PLATFORM,
                "python": {
                    "executable": sys.executable,
                    "version": list(sys.version_info[:3]),
                },
                "schema": SCHEMA,
                "type": "ready",
            }
        ) + b"\n"

    def _close(self) -> None:
        if self._closed:
            return
        self._closed = True
        if self._ordinal < REQUEST_COUNT:
            try:
                self._p56.close_custody(self._handle)
            except BaseException as error:
                if _p56_failure(self._p56, error) or isinstance(error, (OSError, ValueError)):
                    raise WorkerFailure("P57-WSL-INDETERMINATE-CLEANUP") from error
                raise

    def _close_after_failure(self) -> None:
        try:
            self._close()
        except BaseException as cleanup:
            raise WorkerFailure("P57-INDETERMINATE-CLEANUP") from cleanup

    def consume(self, raw: bytes) -> bytes | None:
        if self._closed:
            raise WorkerFailure("P57-WSL-PROTOCOL")
        request = _line_value(raw, MAX_REQUEST_BYTES)
        if request == {"schema": SCHEMA, "type": "close"}:
            self._close()
            return None
        required = {"schema", "type", "ordinal", "platform", "request_id", "arguments"}
        if set(request) != required or request.get("schema") != SCHEMA or request.get("type") != "launch":
            raise WorkerFailure("P57-WSL-PROTOCOL")
        ordinal = request["ordinal"]
        arguments = request["arguments"]
        if (
            type(ordinal) is not int
            or ordinal != self._ordinal + 1
            or ordinal > REQUEST_COUNT
            or request["platform"] != PLATFORM
            or type(arguments) is not list
            or any(type(value) is not str or "\x00" in value for value in arguments)
            or request["request_id"] != _request_id(ordinal, arguments)
        ):
            raise WorkerFailure("P57-WSL-PROTOCOL")
        try:
            completed = self._p56.launch_verified(self._handle, PLATFORM, arguments)
            stdout = completed.stdout
            stderr = completed.stderr
            if (
                type(completed.returncode) is not int
                or type(stdout) is not bytes
                or type(stderr) is not bytes
                or len(stdout) > MAX_STREAM_BYTES
                or len(stderr) > MAX_STREAM_BYTES
            ):
                raise WorkerFailure("P57-WSL-OUTPUT-BOUND")
        except WorkerFailure:
            self._close_after_failure()
            raise
        except BaseException as error:
            self._close_after_failure()
            if _p56_failure(self._p56, error) or isinstance(error, (OSError, ValueError)):
                raise WorkerFailure("P57-WSL-LAUNCH") from error
            raise
        self._ordinal = ordinal
        stdout_sha256 = _sha256_bytes(stdout)
        stderr_sha256 = _sha256_bytes(stderr)
        response = {
            "ordinal": ordinal,
            "platform": PLATFORM,
            "request_id": request["request_id"],
            "returncode": completed.returncode,
            "result_id": _result_id(
                ordinal,
                str(request["request_id"]),
                completed.returncode,
                stdout_sha256,
                stderr_sha256,
            ),
            "schema": SCHEMA,
            "stderr_b64": base64.b64encode(stderr).decode("ascii"),
            "stderr_sha256": stderr_sha256,
            "stdout_b64": base64.b64encode(stdout).decode("ascii"),
            "stdout_sha256": stdout_sha256,
            "type": "result",
        }
        encoded = _canonical_bytes(response) + b"\n"
        if len(encoded) > MAX_RESPONSE_BYTES:
            self._close_after_failure()
            raise WorkerFailure("P57-WSL-OUTPUT-BOUND")
        return encoded

    def close_after_failure(self) -> None:
        self._close()


def _read_line(stream: BinaryIO) -> bytes:
    line = stream.readline(MAX_REQUEST_BYTES + 1)
    if len(line) > MAX_REQUEST_BYTES:
        raise WorkerFailure("P57-WSL-PROTOCOL")
    return line


def _serve(protocol: WorkerProtocol, stdin: BinaryIO, stdout: BinaryIO) -> int:
    stdout.write(protocol.ready())
    stdout.flush()
    try:
        while True:
            line = _read_line(stdin)
            if not line:
                protocol.close_after_failure()
                return 0
            response = protocol.consume(line)
            if response is None:
                return 0
            stdout.write(response)
            stdout.flush()
    except (WorkerFailure, OSError, ValueError):
        try:
            protocol.close_after_failure()
        except BaseException:
            return 2
        return 1
    except BaseException:
        try:
            protocol.close_after_failure()
        except BaseException as cleanup:
            raise WorkerFailure("P57-INDETERMINATE-CLEANUP") from cleanup
        raise


def _arguments(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--runtime-parent", required=True)
    parser.add_argument("--bundle-root", required=True)
    parser.add_argument("--p56-root", required=True)
    parser.add_argument("--expected-parent-device", required=True, type=int)
    parser.add_argument("--expected-parent-inode", required=True, type=int)
    parser.add_argument("--expected-root-device", required=True, type=int)
    parser.add_argument("--expected-root-inode", required=True, type=int)
    parser.add_argument("--expected-sealed-dependencies-sha256", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    bundle: _BoundBundleRoot | None = None
    try:
        arguments = _arguments(argv)
        runtime_parent = _native_directory(arguments.runtime_parent, "P57-WSL-NATIVE-ROOT")
        bundle = _BoundBundleRoot(
            arguments.runtime_parent,
            arguments.bundle_root,
            arguments.expected_parent_device,
            arguments.expected_parent_inode,
            arguments.expected_root_device,
            arguments.expected_root_inode,
        )
        expected_p56 = arguments.bundle_root + "/repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release"
        if arguments.p56_root != expected_p56:
            raise WorkerFailure("P57-WSL-P56-ROOT")
        dependencies = _load_sealed_dependencies(
            bundle, arguments.expected_sealed_dependencies_sha256
        )
        p56 = dependencies.load_exact_p56(bundle.root_fd, bundle.proc_prefix())
        if Path(p56.__file__).parent.as_posix() != bundle.proc_path(
            "repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release"
        ):
            raise WorkerFailure("P57-WSL-P56-ROOT")
        return _serve(WorkerProtocol(p56, runtime_parent), sys.stdin.buffer, sys.stdout.buffer)
    except (WorkerFailure, OSError, ValueError):
        return 2
    except BaseException as error:
        code = getattr(error, "code", None)
        if type(code) is str and code.startswith(("P56-", "P57-")):
            return 2
        raise
    finally:
        if bundle is not None:
            bundle.close()


if __name__ == "__main__":
    raise SystemExit(main())
