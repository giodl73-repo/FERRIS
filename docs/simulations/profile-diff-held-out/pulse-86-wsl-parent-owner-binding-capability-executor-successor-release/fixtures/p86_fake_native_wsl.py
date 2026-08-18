from __future__ import annotations

import base64
import hashlib
import io
import json
import shutil
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Callable


@dataclass
class BundleRecord:
    root: Path
    revalidate_calls: int = 0
    cleanup_calls: int = 0


class FakeBundleManager:
    def __init__(self, executor: object, root: Path) -> None:
        self._executor = executor
        self._root = root
        self._root.mkdir(parents=True, exist_ok=True)
        self.records: list[BundleRecord] = []
        self.before_revalidate: Callable[[Path, object, BundleRecord], None] | None = None
        self.revalidate_failure: BaseException | None = None
        self.before_cleanup: Callable[[Path, object, BundleRecord], None] | None = None
        self.cleanup_failure: BaseException | None = None

    def stage(self, repo_root: Path, _runtime_parent: str) -> object:
        layout = self._executor._expected_bundle_layout(repo_root)
        bundle_root = self._root / f".p57-fake-{len(self.records) + 1:02d}"
        for relative in layout.expected_files:
            target = bundle_root.joinpath(*relative.split("/"))
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(relative.encode("utf-8") + b"\n")
        root_stat = bundle_root.stat()
        parent_stat = bundle_root.parent.stat()
        record = BundleRecord(bundle_root)
        self.records.append(record)
        return self._executor._OwnedBundle(
            root=str(bundle_root),
            runtime_parent="/home/pulse75-fake",
            owner_username="pulse86-fake",
            owner_uid=0,
            name=bundle_root.name,
            python_identity={"executable": "/usr/bin/python3", "version": [3, 12, 0]},
            root_device=root_stat.st_dev,
            root_inode=root_stat.st_ino,
            root_type="directory",
            parent_device=parent_stat.st_dev,
            parent_inode=parent_stat.st_ino,
            parent_type="directory",
            layout=layout,
        )

    def _record(self, path: Path) -> BundleRecord:
        for record in self.records:
            if record.root == path:
                return record
        raise AssertionError(f"unknown bundle root {path}")

    def revalidate(self, bundle: object) -> None:
        path = Path(str(getattr(bundle, "root")))
        record = self._record(path)
        record.revalidate_calls += 1
        if self.before_revalidate is not None:
            self.before_revalidate(path, bundle, record)
        if self.revalidate_failure is not None:
            raise self.revalidate_failure
        if getattr(bundle, "parent_type") != "directory" or getattr(bundle, "root_type") != "directory":
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        if (
            path.parent.stat().st_dev != getattr(bundle, "parent_device")
            or path.parent.stat().st_ino != getattr(bundle, "parent_inode")
        ):
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        if path.is_symlink() or not path.is_dir():
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        root_stat = path.stat()
        if (
            root_stat.st_dev != getattr(bundle, "root_device")
            or root_stat.st_ino != getattr(bundle, "root_inode")
        ):
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")

    def cleanup(self, bundle: object) -> None:
        path = Path(str(getattr(bundle, "root")))
        record = self._record(path)
        record.cleanup_calls += 1
        if self.before_cleanup is not None:
            self.before_cleanup(path, bundle, record)
        if self.cleanup_failure is not None:
            raise self.cleanup_failure
        if getattr(bundle, "parent_type") != "directory" or getattr(bundle, "root_type") != "directory":
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        if (
            path.parent.stat().st_dev != getattr(bundle, "parent_device")
            or path.parent.stat().st_ino != getattr(bundle, "parent_inode")
        ):
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        if path.is_symlink() or not path.is_dir():
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        root_stat = path.stat()
        if (
            root_stat.st_dev != getattr(bundle, "root_device")
            or root_stat.st_ino != getattr(bundle, "root_inode")
        ):
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")
        shutil.rmtree(path)
        if path.exists():
            raise self._executor.ExecutorFailure("P57-WSL-CLEANUP")

    def residue(self) -> list[Path]:
        return [record.root for record in self.records if record.root.exists()]


class _QueueReader:
    def __init__(self, ready: bytes, read_error: BaseException | None = None) -> None:
        self._queue = [ready]
        self._read_error = read_error
        self._raised = False

    def enqueue(self, payload: bytes) -> None:
        self._queue.append(payload)

    def readline(self, _maximum: int) -> bytes:
        if self._read_error is not None and not self._raised:
            self._raised = True
            raise self._read_error
        if not self._queue:
            return b""
        return self._queue.pop(0)

    def read(self, _maximum: int) -> bytes:
        data = b"".join(self._queue)
        self._queue.clear()
        return data


class _QueueWriter:
    def __init__(self, process: "FakeWorkerProcess") -> None:
        self._process = process
        self._buffer = bytearray()
        self._closed = False

    def write(self, payload: bytes) -> int:
        if self._closed:
            raise ValueError("I/O operation on closed file")
        self._buffer.extend(payload)
        return len(payload)

    def flush(self) -> None:
        if self._closed:
            raise ValueError("I/O operation on closed file")
        if self._buffer:
            self._process.consume(bytes(self._buffer))
            self._buffer.clear()

    def close(self) -> None:
        self._closed = True


class FakeWorkerProcess:
    def __init__(
        self,
        executor: object,
        fake: object,
        runtime_root: Path,
        bundle_root: Path,
        *,
        startup_error: BaseException | None = None,
        ready_mutator: Callable[[bytes], bytes] | None = None,
        launch_mutator: Callable[[dict[str, object], bytes], bytes] | None = None,
        status_after_close: int = 0,
        close_timeout: bool = False,
        extra_stdout_after_close: bytes = b"",
        extra_stderr_after_close: bytes = b"",
    ) -> None:
        self._executor = executor
        self._fake = fake
        self._runtime_root = runtime_root
        self._bundle_root = bundle_root
        self._launch_mutator = launch_mutator
        self._status_after_close = status_after_close
        self._close_timeout = close_timeout
        self._extra_stdout_after_close = extra_stdout_after_close
        self.stdin = _QueueWriter(self)
        ready = self._executor._P57._canonical_line(
            {
                "count": self._executor.REQUEST_COUNT,
                "platform": self._executor.WSL_PLATFORM,
                "python": {"executable": "/usr/bin/python3", "version": [3, 12, 0]},
                "schema": self._executor.WSL_SCHEMA,
                "type": "ready",
            }
        )
        self.stdout = _QueueReader(
            ready if ready_mutator is None else ready_mutator(ready), startup_error
        )
        self.stderr = io.BytesIO(extra_stderr_after_close)
        self.close_requested = False
        self.terminated = False
        self.killed = False
        self.requests: list[tuple[int, tuple[str, ...]]] = []
        self.handle = self._fake.publish_retained_build_and_custody(
            "ubuntu-24.04-x86_64",
            self._runtime_root,
        )

    @staticmethod
    def _from_wsl(value: str) -> str:
        return "C:/" + value.removeprefix("/mnt/c/") if value.startswith("/mnt/c/") else value

    def _result_line(self, request: dict[str, object]) -> bytes:
        if not self._bundle_root.exists():
            raise AssertionError("owned bundle vanished before worker close")
        arguments = tuple(self._from_wsl(value) for value in request["arguments"])  # type: ignore[index]
        capture = self._fake.launch_verified(self.handle, "ubuntu-24.04-x86_64", arguments)
        self.requests.append((int(request["ordinal"]), arguments))
        stdout = bytes(capture.stdout)
        stderr = bytes(capture.stderr)
        stdout_sha256 = "sha256:" + hashlib.sha256(stdout).hexdigest()
        stderr_sha256 = "sha256:" + hashlib.sha256(stderr).hexdigest()
        response = {
            "ordinal": request["ordinal"],
            "platform": self._executor.WSL_PLATFORM,
            "request_id": request["request_id"],
            "returncode": int(capture.returncode),
            "result_id": self._executor._P57._worker_result_id(
                int(request["ordinal"]),
                str(request["request_id"]),
                int(capture.returncode),
                stdout_sha256,
                stderr_sha256,
            ),
            "schema": self._executor.WSL_SCHEMA,
            "stderr_b64": base64.b64encode(stderr).decode("ascii"),
            "stderr_sha256": stderr_sha256,
            "stdout_b64": base64.b64encode(stdout).decode("ascii"),
            "stdout_sha256": stdout_sha256,
            "type": "result",
        }
        raw = self._executor._P57._canonical_line(response)
        return raw if self._launch_mutator is None else self._launch_mutator(request, raw)

    def consume(self, raw: bytes) -> None:
        request = json.loads(raw)
        if request["type"] == "close":
            self.close_requested = True
            if len(self.requests) != self._executor.REQUEST_COUNT:
                self._fake.close_custody(self.handle)
            if self._extra_stdout_after_close:
                self.stdout.enqueue(self._extra_stdout_after_close)
            return
        self.stdout.enqueue(self._result_line(request))

    def wait(self, timeout: int) -> int:
        if self._close_timeout and not self.killed:
            raise subprocess.TimeoutExpired("worker", timeout)
        if self.killed:
            return -9
        if self.close_requested:
            return self._status_after_close
        raise subprocess.TimeoutExpired("worker", timeout)

    def terminate(self) -> None:
        self.terminated = True

    def kill(self) -> None:
        self.killed = True


class FakeWorkerProcessFactory:
    def __init__(
        self,
        executor: object,
        fake: object,
        runtime_root: Path,
        *,
        startup_error: BaseException | None = None,
        ready_mutator: Callable[[bytes], bytes] | None = None,
        launch_mutator: Callable[[dict[str, object], bytes], bytes] | None = None,
        status_after_close: int = 0,
        close_timeout: bool = False,
        extra_stdout_after_close: bytes = b"",
        extra_stderr_after_close: bytes = b"",
    ) -> None:
        self._executor = executor
        self._fake = fake
        self._runtime_root = runtime_root
        self._startup_error = startup_error
        self._ready_mutator = ready_mutator
        self._launch_mutator = launch_mutator
        self._status_after_close = status_after_close
        self._close_timeout = close_timeout
        self._extra_stdout_after_close = extra_stdout_after_close
        self._extra_stderr_after_close = extra_stderr_after_close
        self.processes: list[FakeWorkerProcess] = []

    def __call__(self, _bundle: object, _ubuntu_runtime_parent: str) -> FakeWorkerProcess:
        process = FakeWorkerProcess(
            self._executor,
            self._fake,
            self._runtime_root,
            Path(str(getattr(_bundle, "root"))),
            startup_error=self._startup_error,
            ready_mutator=self._ready_mutator,
            launch_mutator=self._launch_mutator,
            status_after_close=self._status_after_close,
            close_timeout=self._close_timeout,
            extra_stdout_after_close=self._extra_stdout_after_close,
            extra_stderr_after_close=self._extra_stderr_after_close,
        )
        self.processes.append(process)
        return process
