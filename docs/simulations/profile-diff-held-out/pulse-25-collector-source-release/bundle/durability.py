from __future__ import annotations

import ctypes
import errno
import json
import os
import stat
import uuid
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Callable


TEMP_MARKER = ".atomic-write-"
TEMP_SUFFIX = ".tmp"
Observer = Callable[[str], None]


@dataclass(frozen=True)
class DirectorySyncStatus:
    state: str
    mechanism: str
    detail: str
    error_code: int | None = None

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass(frozen=True)
class AtomicWriteReceipt:
    byte_count: int
    file_sync: str
    replaced_existing: bool
    directory_sync: DirectorySyncStatus
    residue_clean: bool

    def to_dict(self) -> dict:
        value = asdict(self)
        value["directory_sync"] = self.directory_sync.to_dict()
        return value


def canonical_json(value: object) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=True,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def _emit(observer: Observer | None, event: str) -> None:
    if observer is not None:
        observer(event)


def _unsupported_error(error: OSError) -> bool:
    unsupported = {errno.EINVAL, errno.ENOSYS}
    if hasattr(errno, "ENOTSUP"):
        unsupported.add(errno.ENOTSUP)
    if hasattr(errno, "EOPNOTSUPP"):
        unsupported.add(errno.EOPNOTSUPP)
    return error.errno in unsupported


def _sync_directory_posix(
    directory: Path, observer: Observer | None
) -> DirectorySyncStatus:
    flags = os.O_RDONLY
    if hasattr(os, "O_DIRECTORY"):
        flags |= os.O_DIRECTORY
    if hasattr(os, "O_CLOEXEC"):
        flags |= os.O_CLOEXEC

    try:
        descriptor = os.open(directory, flags)
    except OSError as error:
        if _unsupported_error(error):
            return DirectorySyncStatus(
                state="unsupported",
                mechanism="posix-fsync-directory",
                detail="directory descriptor sync is unsupported by this filesystem",
                error_code=error.errno,
            )
        raise

    _emit(observer, "directory-open")
    try:
        os.fsync(descriptor)
        _emit(observer, "directory-sync")
        return DirectorySyncStatus(
            state="synced",
            mechanism="posix-fsync-directory",
            detail="containing directory descriptor was synchronized",
        )
    except OSError as error:
        if _unsupported_error(error):
            return DirectorySyncStatus(
                state="unsupported",
                mechanism="posix-fsync-directory",
                detail="directory descriptor sync is unsupported by this filesystem",
                error_code=error.errno,
            )
        raise
    finally:
        os.close(descriptor)
        _emit(observer, "directory-close")


def _sync_directory_windows(
    directory: Path, observer: Observer | None
) -> DirectorySyncStatus:
    from ctypes import wintypes

    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    create_file = kernel32.CreateFileW
    create_file.argtypes = [
        wintypes.LPCWSTR,
        wintypes.DWORD,
        wintypes.DWORD,
        wintypes.LPVOID,
        wintypes.DWORD,
        wintypes.DWORD,
        wintypes.HANDLE,
    ]
    create_file.restype = wintypes.HANDLE
    flush_file_buffers = kernel32.FlushFileBuffers
    flush_file_buffers.argtypes = [wintypes.HANDLE]
    flush_file_buffers.restype = wintypes.BOOL
    close_handle = kernel32.CloseHandle
    close_handle.argtypes = [wintypes.HANDLE]
    close_handle.restype = wintypes.BOOL

    generic_read = 0x80000000
    generic_write = 0x40000000
    share_all = 0x00000001 | 0x00000002 | 0x00000004
    open_existing = 3
    backup_semantics = 0x02000000
    invalid_handle = ctypes.c_void_p(-1).value
    unsupported_codes = {1, 5, 6, 50, 87}
    last_error = None

    for access in (generic_read | generic_write, generic_read):
        handle = create_file(
            str(directory.resolve()),
            access,
            share_all,
            None,
            open_existing,
            backup_semantics,
            None,
        )
        if handle == invalid_handle:
            last_error = ctypes.get_last_error()
            if last_error in unsupported_codes:
                continue
            raise ctypes.WinError(last_error)

        _emit(observer, "directory-open")
        try:
            if flush_file_buffers(handle):
                _emit(observer, "directory-sync")
                return DirectorySyncStatus(
                    state="synced",
                    mechanism="win32-directory-flush",
                    detail="containing directory handle was synchronized",
                )
            last_error = ctypes.get_last_error()
            if last_error not in unsupported_codes:
                raise ctypes.WinError(last_error)
        finally:
            close_handle(handle)
            _emit(observer, "directory-close")

    return DirectorySyncStatus(
        state="unsupported",
        mechanism="win32-directory-flush",
        detail="the active Windows filesystem/API denied directory-handle flushing",
        error_code=last_error,
    )


def sync_directory(
    directory: Path | str, observer: Observer | None = None
) -> DirectorySyncStatus:
    path = Path(directory)
    if not path.is_dir():
        raise NotADirectoryError(path)
    if os.name == "nt":
        return _sync_directory_windows(path, observer)
    return _sync_directory_posix(path, observer)


def temporary_path(target: Path, token: str | None = None) -> Path:
    unique = token or f"{os.getpid()}-{uuid.uuid4().hex}"
    return target.with_name(f".{target.name}{TEMP_MARKER}{unique}{TEMP_SUFFIX}")


def is_temporary_path(path: Path) -> bool:
    return TEMP_MARKER in path.name and path.name.endswith(TEMP_SUFFIX)


def find_residue(root: Path | str) -> list[Path]:
    base = Path(root)
    if not base.exists():
        return []
    return sorted(
        (path for path in base.rglob("*") if path.is_file() and is_temporary_path(path)),
        key=lambda path: path.as_posix(),
    )


def atomic_write_bytes(
    target: Path | str,
    data: bytes,
    *,
    observer: Observer | None = None,
    token: str | None = None,
) -> AtomicWriteReceipt:
    path = Path(target)
    parent = path.parent
    if not parent.is_dir():
        raise FileNotFoundError(f"containing directory does not exist: {parent}")

    replaced_existing = path.exists()
    mode = 0o666
    if replaced_existing:
        mode = stat.S_IMODE(path.stat().st_mode)
    temporary = temporary_path(path, token)
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_BINARY"):
        flags |= os.O_BINARY
    if hasattr(os, "O_CLOEXEC"):
        flags |= os.O_CLOEXEC

    descriptor: int | None = None
    handle = None
    try:
        descriptor = os.open(temporary, flags, mode)
        _emit(observer, "temp-open")
        handle = os.fdopen(descriptor, "wb", closefd=True)
        descriptor = None
        try:
            handle.write(data)
            _emit(observer, "userspace-write")
            handle.flush()
            _emit(observer, "userspace-flush")
            os.fsync(handle.fileno())
            _emit(observer, "file-sync")
        finally:
            handle.close()
            handle = None
            _emit(observer, "temp-close")

        os.replace(temporary, path)
        _emit(observer, "atomic-replace")
        directory_status = sync_directory(parent, observer)
        _emit(observer, "complete")
        return AtomicWriteReceipt(
            byte_count=len(data),
            file_sync="synced",
            replaced_existing=replaced_existing,
            directory_sync=directory_status,
            residue_clean=not temporary.exists(),
        )
    finally:
        if handle is not None:
            handle.close()
            _emit(observer, "temp-close")
        if descriptor is not None:
            os.close(descriptor)
            _emit(observer, "temp-close")
        if temporary.exists():
            temporary.unlink()
            _emit(observer, "temp-cleanup")


def atomic_write_json(
    target: Path | str,
    value: object,
    *,
    observer: Observer | None = None,
    token: str | None = None,
) -> AtomicWriteReceipt:
    return atomic_write_bytes(
        target,
        canonical_json(value) + b"\n",
        observer=observer,
        token=token,
    )
