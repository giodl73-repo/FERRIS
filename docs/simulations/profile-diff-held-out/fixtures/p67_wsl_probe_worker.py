"""Harmless exact-route probe worker for Pulse 67 WSL preflight."""

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


SCHEMA = "ferris.pulse-67-wsl-probe-session/v1"
PLATFORM = "ubuntu-24.04-x86_64"
REQUEST_COUNT = 1
MAX_REQUEST_BYTES = 16_384
MAX_RESPONSE_BYTES = 2_800_000
MAX_STREAM_BYTES = 1_048_576
SEALED_DEPENDENCIES_SHA256 = "sha256:317d11a7468647dfafef26db5079a0422ca39cc8228cf389ebc6949be4a4236f"


class WorkerFailure(RuntimeError):
    """Protocol or custody failure without a path-bearing public message."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    value: dict[str, object] = {}
    for key, item in pairs:
        if key in value:
            raise WorkerFailure("P67-WSL-PROTOCOL")
        value[key] = item
    return value


def _canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def _line_value(raw: bytes, maximum: int) -> dict[str, object]:
    if not raw.endswith(b"\n") or len(raw) > maximum:
        raise WorkerFailure("P67-WSL-PROTOCOL")
    try:
        value = json.loads(raw[:-1], object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, WorkerFailure) as error:
        raise WorkerFailure("P67-WSL-PROTOCOL") from error
    if type(value) is not dict or _canonical_bytes(value) + b"\n" != raw:
        raise WorkerFailure("P67-WSL-PROTOCOL")
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


def _safe_regular(path: Path, code: str) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise WorkerFailure(code)
        descriptor = os.open(path, os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0))
    except WorkerFailure:
        raise
    except OSError as error:
        raise WorkerFailure(code) from error
    try:
        opened = os.fstat(descriptor)
        if (opened.st_dev, opened.st_ino) != (initial.st_dev, initial.st_ino):
            raise WorkerFailure(code)
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > MAX_RESPONSE_BYTES:
                raise WorkerFailure(code)
        return bytes(content)
    except OSError as error:
        raise WorkerFailure(code) from error
    finally:
        os.close(descriptor)


def _load_sealed_dependencies(bundle_root: Path) -> ModuleType:
    source = bundle_root / "worker" / "sealed_dependencies.py"
    content = _safe_regular(source, "P67-WSL-BUNDLE")
    if _sha256_bytes(content) != SEALED_DEPENDENCIES_SHA256:
        raise WorkerFailure("P67-WSL-BUNDLE")
    module = ModuleType("p67_probe_sealed_dependencies")
    module.__file__ = os.fspath(source)
    module.__package__ = ""
    module.__loader__ = None
    module.__spec__ = None
    sys.modules[module.__name__] = module
    try:
        exec(compile(content, module.__file__, "exec"), module.__dict__)
    except (ImportError, OSError, RuntimeError, SyntaxError, ValueError) as error:
        sys.modules.pop(module.__name__, None)
        raise WorkerFailure("P67-WSL-BUNDLE") from error
    return module


def _dependency_failure(module: ModuleType, error: BaseException) -> bool:
    candidate = getattr(module, "ReleaseFailure", None)
    return isinstance(candidate, type) and isinstance(error, candidate)


class ProbeProtocol:
    """Perform one harmless probe over the exact bounded line protocol."""

    def __init__(
        self,
        dependencies: ModuleType,
        runtime_parent: Path,
        bundle_root: Path,
        p56_root: Path,
    ) -> None:
        if (
            getattr(dependencies, "PROBE_SCHEMA", None) != SCHEMA
            or getattr(dependencies, "PLATFORM", None) != PLATFORM
            or getattr(dependencies, "PROBE_REQUEST_COUNT", None) != REQUEST_COUNT
        ):
            raise WorkerFailure("P67-WSL-BUNDLE")
        try:
            self._context = dependencies.bind_probe_context(bundle_root, p56_root, runtime_parent)
        except BaseException as error:
            if _dependency_failure(dependencies, error) or isinstance(error, (OSError, ValueError)):
                raise WorkerFailure("P67-WSL-BUNDLE") from error
            raise
        self._dependencies = dependencies
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
        try:
            self._dependencies.close_probe_context(self._context)
        except BaseException as error:
            if _dependency_failure(self._dependencies, error) or isinstance(error, (OSError, ValueError)):
                raise WorkerFailure("P67-WSL-CLEANUP") from error
            raise

    def _close_after_failure(self) -> None:
        try:
            self._close()
        except BaseException as cleanup:
            raise WorkerFailure("P67-INDETERMINATE-CLEANUP") from cleanup

    def consume(self, raw: bytes) -> bytes | None:
        if self._closed:
            raise WorkerFailure("P67-WSL-PROTOCOL")
        request = _line_value(raw, MAX_REQUEST_BYTES)
        if request == {"schema": SCHEMA, "type": "close"}:
            self._close()
            return None
        required = {"schema", "type", "ordinal", "platform", "request_id", "arguments"}
        if (
            set(request) != required
            or request.get("schema") != SCHEMA
            or request.get("type") != "launch"
        ):
            raise WorkerFailure("P67-WSL-PROTOCOL")
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
            raise WorkerFailure("P67-WSL-PROTOCOL")
        try:
            completed = self._dependencies.launch_harmless_probe(self._context, PLATFORM, arguments)
            stdout = completed.stdout
            stderr = completed.stderr
            if (
                type(completed.returncode) is not int
                or type(stdout) is not bytes
                or type(stderr) is not bytes
                or len(stdout) > MAX_STREAM_BYTES
                or len(stderr) > MAX_STREAM_BYTES
            ):
                raise WorkerFailure("P67-WSL-OUTPUT-BOUND")
        except WorkerFailure:
            self._close_after_failure()
            raise
        except BaseException as error:
            self._close_after_failure()
            if _dependency_failure(self._dependencies, error) or isinstance(error, (OSError, ValueError)):
                raise WorkerFailure("P67-WSL-PROBE-LAUNCH") from error
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
            raise WorkerFailure("P67-WSL-OUTPUT-BOUND")
        return encoded

    def close_after_failure(self) -> None:
        self._close()


def _read_line(stream: BinaryIO) -> bytes:
    line = stream.readline(MAX_REQUEST_BYTES + 1)
    if len(line) > MAX_REQUEST_BYTES:
        raise WorkerFailure("P67-WSL-PROTOCOL")
    return line


def _serve(protocol: ProbeProtocol, stdin: BinaryIO, stdout: BinaryIO) -> int:
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
            raise WorkerFailure("P67-INDETERMINATE-CLEANUP") from cleanup
        raise


def _arguments(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--runtime-parent", required=True)
    parser.add_argument("--bundle-root", required=True)
    parser.add_argument("--p56-root", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        arguments = _arguments(argv)
        runtime_parent = _native_directory(arguments.runtime_parent, "P67-WSL-NATIVE-ROOT")
        bundle_root = _native_directory(arguments.bundle_root, "P67-WSL-BUNDLE")
        p56_root = _native_directory(arguments.p56_root, "P67-WSL-P56-ROOT")
        expected_p56 = (
            bundle_root
            / "repository"
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / "pulse-56-retained-build-custody-release"
        )
        if p56_root != expected_p56:
            raise WorkerFailure("P67-WSL-P56-ROOT")
        dependencies = _load_sealed_dependencies(bundle_root)
        return _serve(
            ProbeProtocol(dependencies, runtime_parent, bundle_root, p56_root),
            sys.stdin.buffer,
            sys.stdout.buffer,
        )
    except (WorkerFailure, OSError, ValueError):
        return 2
    except BaseException as error:
        code = getattr(error, "code", None)
        if type(code) is str and code.startswith(("P67-",)):
            return 2
        raise


if __name__ == "__main__":
    raise SystemExit(main())
