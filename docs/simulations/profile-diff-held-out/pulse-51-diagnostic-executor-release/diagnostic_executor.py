"""Fail-closed Pulse 51 execution infrastructure.

The exported runtime has fixed predecessor identities, fixed P33 expectations,
and fixed Windows/Ubuntu dispatch.  Authority is deliberately external
governance: this module neither accepts nor attempts to authenticate a
caller-selected grant.  The underscore-prefixed qualification entry point is
private test infrastructure and is not a runtime authorization surface.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import stat
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Mapping, Protocol

import frozen_profile_diff as frozen_profile_diff
from p31_contract_verifier import (
    P31Failure,
    Outcome,
    parse_accepted_profile,
    validate_bytes,
    validate_path,
    verify_bound_contract,
)
from p35_p37_custody import CustodyFailure, verify_p35_p37_custody
from sealed_dependencies import (
    DependencyFailure,
    load_p27_exact_runner,
    load_terminal_dependencies,
    verify_p27_summary,
)


P43_CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
P43_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
P33_CUTOFF = "29517d732db13cc2ffa304684b344f3538ab587d"
COMMAND_VERSION = "0.1.0"
CANONICAL_PLATFORMS = ("windows-x86_64", "ubuntu-24.04-x86_64")
WSL_PLATFORM_ALIAS = "ubuntu-24.04-wsl2-x86_64"
WSL_DISTRIBUTION = "Ubuntu-24.04"
P47_SUMMARY_SCHEMA = "ferris.pulse-47-publication-outcome-witness-summary/v1"
P50_GATE_IDS = (
    "pulse-41-pulse-39-public-custody",
    "windows-retained-binary-custody",
    "ubuntu-retained-binary-custody",
    "exact-adapter-preflight",
    "pulse-31-public-input",
    "pulse-35-pulse-37-normalization",
    "bounded-materialization",
    "bounded-process-exit-search",
)
RESULT_MAP = {
    "success": {
        "exit": 0,
        "stream": "stdout-only",
        "record": "non-null",
        "diagnostics": "empty",
    },
    "difference": {
        "exit": 1,
        "stream": "stdout-only",
        "record": "non-null",
        "diagnostics": "empty",
    },
    "invalid": {
        "exit": 2,
        "stream": "stderr-only",
        "record": "null",
        "diagnostics": "exactly-one-matching-class",
    },
    "unsupported": {
        "exit": 4,
        "stream": "stderr-only",
        "record": "null",
        "diagnostics": "exactly-one-matching-class",
    },
    "incomplete": {
        "exit": 5,
        "stream": "stderr-only",
        "record": "null",
        "diagnostics": "exactly-one-matching-class",
    },
    "blocked": {
        "exit": 7,
        "stream": "stderr-only",
        "record": "null",
        "diagnostics": "exactly-one-matching-class",
    },
}
TARGET = re.compile(r"^artifacts/[0-9]{3}-(before|after)\.(bin|missing|directory)$")
TOKEN = re.compile(r"^[0-9a-f]{64}$")
SHA256 = re.compile(r"^sha256:[0-9a-f]{64}$")
IDENTITY = re.compile(r"^(selection|invocation|result):[0-9a-f]{64}$")
DIFF_ID = re.compile(r"^profile-diff:[0-9a-f]{64}$")
DIAGNOSTIC_CODE = re.compile(r"^FERRIS-[A-Z0-9-]+$")
VISIBLE_METADATA = re.compile(r"^[!-~]{1,256}$")
SECTIONS = (
    "assurance",
    "closure",
    "features",
    "identity",
    "lifecycle",
    "native",
    "providers",
    "stages",
    "stewardship",
    "support",
    "targets",
    "toolchain",
)
UNKNOWN_LINES = (
    "Semantic equivalence and compatibility are not assessed.",
    "Support, freshness, approval, and decision authority are not assessed.",
)
LIMITATION_LINES = (
    "This record compares only explicit caller-provided evidence and does not interpret support, compatibility, approval, correctness, freshness, or readiness.",
    "Ferris did not generate either profile, invoke an owner tool, discover files, contact a network, select packages, or mutate input, repository, or environment state.",
    "Value digests identify compared JSON values; raw section values are intentionally omitted.",
    "Profile identifiers, revisions, consumers, and JSON object keys are output-visible metadata; callers must not place secrets in those fields.",
)
DIAGNOSTIC_CODES_BY_CLASS = {
    "invalid": frozenset(
        {
            "FERRIS-PROFILE-DIFF-PROFILE-ID-MISMATCH",
            "FERRIS-PROFILE-DIFF-CONSUMER-MISMATCH",
            "FERRIS-PROFILE-IDENTITY-INVALID",
            "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER",
            "FERRIS-PROFILE-JSON-INVALID",
            "FERRIS-PROFILE-METADATA-INVALID",
            "FERRIS-PROFILE-SHAPE-INVALID",
        }
    ),
    "unsupported": frozenset({"FERRIS-PROFILE-SCHEMA-UNSUPPORTED"}),
    "incomplete": frozenset(
        {
            "FERRIS-PROFILE-INPUT-NOT-FILE",
            "FERRIS-PROFILE-INPUT-OVERSIZED",
            "FERRIS-PROFILE-INPUT-UNAVAILABLE",
        }
    ),
    "blocked": frozenset({"FERRIS-PROFILE-DIFF-BOUND-EXCEEDED"}),
}


@dataclass(frozen=True)
class BinaryExpectation:
    """Private qualification-only shape for harmless fake retained binaries."""

    platform: str
    logical_filename: str
    size: int
    sha256: str
    cargo_version: str
    rustc_version: str
    rustc_host: str
    reproducibility_controls: tuple[str, ...]
    published_receipt_payload_sha256: str


P33_EXPECTATIONS = {
    "windows-x86_64": BinaryExpectation(
        platform="windows-x86_64",
        logical_filename="ferris-windows-x86_64-29517d732db13cc2ffa304684b344f3538ab587d.exe",
        size=1_436_672,
        sha256="sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8",
        cargo_version="cargo 1.95.0 (f2d3ce0bd 2026-03-21)",
        rustc_version="rustc 1.95.0 (59807616e 2026-04-14)",
        rustc_host="x86_64-pc-windows-msvc",
        reproducibility_controls=("CARGO_INCREMENTAL=0", "RUSTFLAGS=-C link-arg=/Brepro"),
        published_receipt_payload_sha256="sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a",
    ),
    "ubuntu-24.04-x86_64": BinaryExpectation(
        platform="ubuntu-24.04-x86_64",
        logical_filename="ferris-ubuntu-24.04-x86_64-29517d732db13cc2ffa304684b344f3538ab587d",
        size=1_945_448,
        sha256="sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4",
        cargo_version="cargo 1.97.1 (c980f4866 2026-06-30)",
        rustc_version="rustc 1.97.1 (8bab26f4f 2026-07-14)",
        rustc_host="x86_64-unknown-linux-gnu",
        reproducibility_controls=("CARGO_INCREMENTAL=0",),
        published_receipt_payload_sha256="sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae",
    ),
}


class ExecutorFailure(RuntimeError):
    """A bounded runtime failure whose code is safe for a Pulse 43 terminal."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class _TerminalPreconditionFailure(RuntimeError):
    """A bounded P47 precondition failure with no path-bearing message."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


@dataclass(frozen=True)
class P44CustodyBinding:
    """One independently retained P44 custody outcome and its final tree."""

    platform: str
    final_root: Path
    work_root: Path
    summary: object


@dataclass(frozen=True)
class Descriptor:
    ordinal: int
    case_id: str
    output_format: str
    expected: dict[str, object]
    before: Path | None
    after: Path | None
    execution_mode: str


@dataclass(frozen=True)
class LaunchCapture:
    returncode: int
    stdout: bytes
    stderr: bytes


@dataclass(frozen=True)
class Dispatch:
    """One fully constructed no-shell process launch."""

    platform: str
    executable: Path
    application_argv: tuple[str, ...]
    host_cwd: Path
    command: tuple[str, ...]
    wsl_cwd: str | None


class ProcessRunner(Protocol):
    def __call__(self, dispatch: Dispatch) -> LaunchCapture:
        """Run a completely constructed command at the final subprocess boundary."""


@dataclass(frozen=True)
class ExecutorResult:
    catalog: dict[str, object]
    events: list[dict[str, object]]
    private_record: dict[str, object]


@dataclass(frozen=True)
class _RuntimeControls:
    expectations: Mapping[str, BinaryExpectation]
    p27_runner: Callable[[Path], dict[str, object]] | None
    process_runner: ProcessRunner


def canonical_platform_id(value: str) -> str:
    """Map the sole internal WSL label before any P45 or P43 record exists."""

    if value == WSL_PLATFORM_ALIAS:
        return "ubuntu-24.04-x86_64"
    if value in CANONICAL_PLATFORMS:
        return value
    raise ExecutorFailure("P51-PLATFORM-UNSUPPORTED")


def resolve_python_launcher(
    platform: str, *, which: Callable[[str], str | None]
) -> tuple[str, ...]:
    """Resolve only the documented private synthetic-fixture interpreter."""

    canonical = canonical_platform_id(platform)
    if canonical == "ubuntu-24.04-x86_64":
        for name in ("python3", "python"):
            located = which(name)
            if located:
                return (located,)
    else:
        located = which("python")
        if located:
            return (located,)
        located = which("py")
        if located:
            return (located, "-3")
    raise ExecutorFailure("P51-PYTHON-UNAVAILABLE")


def _canonical_json(value: object) -> bytes:
    return json.dumps(
        value, allow_nan=False, ensure_ascii=True, separators=(",", ":"), sort_keys=True
    ).encode("ascii")


def _digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def _duplicate_free_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ExecutorFailure("P51-DUPLICATE-JSON-MEMBER")
        result[key] = value
    return result


def _safe_regular_bytes(path: Path, code: str, maximum: int = 4_194_304) -> bytes:
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
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
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


def _read_json(path: Path, code: str, maximum: int = 4_194_304) -> dict[str, object]:
    try:
        result = json.loads(
            _safe_regular_bytes(path, code, maximum),
            object_pairs_hook=_duplicate_free_object,
        )
    except (UnicodeDecodeError, json.JSONDecodeError, ExecutorFailure) as error:
        raise ExecutorFailure(code) from error
    if type(result) is not dict:
        raise ExecutorFailure(code)
    return result


def _path_text(path: Path, code: str) -> str:
    value = os.fspath(path)
    if type(value) is not str or "\x00" in value or any(character in value for character in "\r\n"):
        raise ExecutorFailure(code)
    normalized = value.replace("/", "\\")
    if normalized.startswith(("\\\\", "//")) or normalized.startswith("\\\\.\\"):
        raise ExecutorFailure(code)
    if normalized.startswith("\\\\?\\"):
        raise ExecutorFailure(code)
    return value


def _absolute_windows_path(path: Path, code: str) -> Path:
    _path_text(path, code)
    if not path.is_absolute() or ".." in path.parts:
        raise ExecutorFailure(code)
    drive = path.drive
    if len(drive) != 2 or drive[1] != ":":
        raise ExecutorFailure(code)
    return path


def _safe_directory(path: Path, code: str) -> Path:
    _absolute_windows_path(path, code)
    try:
        metadata = os.lstat(path)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise ExecutorFailure(code)
        return path.resolve(strict=True)
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure(code) from error


def _safe_runtime_root(path: Path) -> Path:
    root = _safe_directory(path, "P51-RUNTIME-ROOT")
    probe = root
    try:
        while True:
            metadata = os.lstat(probe)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise ExecutorFailure("P51-RUNTIME-ROOT")
            if probe == probe.parent:
                break
            probe = probe.parent
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure("P51-RUNTIME-ROOT") from error
    return root


def _runtime_path(
    runtime_root: Path,
    value: Path,
    code: str,
    *,
    require_regular: bool = False,
    require_directory: bool = False,
    allow_absent_leaf: bool = False,
) -> Path:
    requested = _absolute_windows_path(value, code)
    try:
        lexical_relative = requested.relative_to(runtime_root)
        resolved = requested.resolve(strict=False)
        resolved.relative_to(runtime_root)
    except (OSError, ValueError) as error:
        raise ExecutorFailure(code) from error
    current = runtime_root
    try:
        for index, part in enumerate(lexical_relative.parts):
            current = current / part
            final_part = index == len(lexical_relative.parts) - 1
            if not os.path.lexists(current):
                if final_part and allow_absent_leaf:
                    break
                raise ExecutorFailure(code)
            metadata = os.lstat(current)
            if stat.S_ISLNK(metadata.st_mode):
                raise ExecutorFailure(code)
            if not final_part and not stat.S_ISDIR(metadata.st_mode):
                raise ExecutorFailure(code)
        metadata = os.lstat(requested) if os.path.lexists(requested) else None
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure(code) from error
    if metadata is None:
        if not allow_absent_leaf:
            raise ExecutorFailure(code)
        return resolved
    if require_regular and not stat.S_ISREG(metadata.st_mode):
        raise ExecutorFailure(code)
    if require_directory and not stat.S_ISDIR(metadata.st_mode):
        raise ExecutorFailure(code)
    if not require_regular and not require_directory and not (
        stat.S_ISREG(metadata.st_mode) or stat.S_ISDIR(metadata.st_mode)
    ):
        raise ExecutorFailure(code)
    return resolved


def windows_to_wsl_absolute(path: Path, runtime_root: Path) -> str:
    """Translate one verified non-UNC Windows path to its exact WSL mount path."""

    checked = _runtime_path(
        runtime_root,
        path,
        "P51-WSL-PATH",
        allow_absent_leaf=True,
    )
    drive = checked.drive
    rendered = checked.as_posix()
    if (
        len(drive) != 2
        or drive[1] != ":"
        or not rendered.startswith(drive)
        or len(rendered) < 3
        or rendered[2] != "/"
    ):
        raise ExecutorFailure("P51-WSL-PATH")
    translated = f"/mnt/{drive[0].lower()}{rendered[2:]}"
    if not translated.startswith(f"/mnt/{drive[0].lower()}/") or "\x00" in translated:
        raise ExecutorFailure("P51-WSL-PATH")
    return translated


def _confined(root: Path, target: str) -> Path:
    if TARGET.fullmatch(target) is None:
        raise ExecutorFailure("P51-DESCRIPTOR-TARGET")
    candidate = root.joinpath(*target.split("/"))
    try:
        candidate.relative_to(root)
        resolved_parent = candidate.parent.resolve(strict=True)
        resolved_parent.relative_to(root.resolve(strict=True))
    except (OSError, ValueError) as error:
        raise ExecutorFailure("P51-DESCRIPTOR-PATH-CONFINEMENT") from error
    if candidate.parent != root / "artifacts":
        raise ExecutorFailure("P51-DESCRIPTOR-PATH-CONFINEMENT")
    return candidate


def _normalize_lexical(path: str) -> str:
    if type(path) is not str:
        raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    value = path.removeprefix("\\\\?\\").replace("\\", "/")
    unc = value.startswith("//")
    rooted = unc or value.startswith("/")
    drive = ""
    if not unc and len(value) >= 3 and value[1:3] == ":/":
        drive, value, rooted = value[:3], value[3:], True
    elif unc:
        value = value[2:]
    elif rooted:
        value = value[1:]
    parts: list[str] = []
    for part in value.split("/"):
        if not part or part == ".":
            continue
        if part == "..":
            if parts and parts[-1] != "..":
                parts.pop()
            elif not rooted:
                parts.append(part)
            continue
        parts.append(part)
    if unc:
        return "//" + "/".join(parts)
    if drive:
        return drive + "/".join(parts)
    if rooted:
        return "/" + "/".join(parts)
    return "/".join(parts) or "."


def _validate_request(request: object, target: str) -> None:
    if type(request) is not dict:
        raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    required = {
        "spelling",
        "platform_namespace",
        "request_template",
        "substitution_rule",
        "resolved_output_relative_target",
        "relative_resolution_base",
    }
    if set(request) != required or request["substitution_rule"] != (
        "replace-target-placeholders-then-lexically-normalize-v1"
    ):
        raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    spelling = request["spelling"]
    template = request["request_template"]
    namespace = request["platform_namespace"]
    base = request["relative_resolution_base"]
    if (
        type(spelling) is not str
        or type(template) is not str
        or request["resolved_output_relative_target"] != target
    ):
        raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    expected = template.replace("{target}", target).replace(
        "{target_suffix}", target.removeprefix("artifacts/")
    )
    if _normalize_lexical(spelling) != _normalize_lexical(expected):
        raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    relative_namespaces = {"output-relative-v1", "relative-child-custody-root-v1"}
    absolute_namespaces = {
        "windows-drive-custody-root-v1",
        "windows-extended-custody-root-v1",
        "windows-unc-custody-root-v1",
        "unix-custody-root-v1",
    }
    if namespace in relative_namespaces:
        if namespace == "output-relative-v1" and base != "":
            raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
        if namespace == "relative-child-custody-root-v1" and base != "request-child":
            raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
        combined = _normalize_lexical((base + "/" if base else "") + _normalize_lexical(spelling))
        if combined != target:
            raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    elif namespace in absolute_namespaces:
        if base is not None:
            raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")
    else:
        raise ExecutorFailure("P51-DESCRIPTOR-REQUEST")


def _expected_result(value: object) -> dict[str, object]:
    if type(value) is not dict or set(value) != {
        "result_class",
        "exit",
        "stream",
        "record",
        "diagnostics",
    }:
        raise ExecutorFailure("P51-DESCRIPTOR-RESULT")
    result_class = value.get("result_class")
    if result_class not in RESULT_MAP or value != {
        "result_class": result_class,
        **RESULT_MAP[result_class],
    }:
        raise ExecutorFailure("P51-DESCRIPTOR-RESULT")
    return value


def _role_path(root: Path, role: object, *, is_final: bool) -> Path | None:
    if type(role) is not dict or set(role) != {
        "state",
        "target",
        "raw_size",
        "raw_sha256",
        "request",
        "expected_input",
    }:
        raise ExecutorFailure("P51-DESCRIPTOR-ROLE")
    state = role["state"]
    expected_input = role["expected_input"]
    if type(expected_input) is not dict or set(expected_input) != {"class", "diagnostic"}:
        raise ExecutorFailure("P51-DESCRIPTOR-ROLE")
    if state == "not-materialized":
        if not is_final or any(
            role[key] is not None for key in ("target", "raw_size", "raw_sha256", "request")
        ):
            raise ExecutorFailure("P51-DESCRIPTOR-ROLE")
        if expected_input != {
            "class": "incomplete",
            "diagnostic": "FERRIS-PROFILE-INPUT-UNAVAILABLE",
        }:
            raise ExecutorFailure("P51-DESCRIPTOR-ROLE")
        return None
    target = role["target"]
    if type(target) is not str:
        raise ExecutorFailure("P51-DESCRIPTOR-ROLE")
    _validate_request(role["request"], target)
    path = _confined(root, target)
    if state == "regular-file":
        data = _safe_regular_bytes(path, "P51-DESCRIPTOR-ARTIFACT", maximum=1_048_577)
        if (
            type(role["raw_size"]) is not int
            or role["raw_size"] != len(data)
            or role["raw_sha256"] != _digest(data)
        ):
            raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
        outcome = validate_path(path)
    elif state == "missing":
        if os.path.lexists(path) or role["raw_size"] is not None or role["raw_sha256"] is not None:
            raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
        outcome = Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE")
    elif state == "directory":
        try:
            metadata = os.lstat(path)
        except OSError as error:
            raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT") from error
        if (
            stat.S_ISLNK(metadata.st_mode)
            or not stat.S_ISDIR(metadata.st_mode)
            or role["raw_size"] is not None
            or role["raw_sha256"] is not None
        ):
            raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
        outcome = Outcome("incomplete", "FERRIS-PROFILE-INPUT-NOT-FILE")
    else:
        raise ExecutorFailure("P51-DESCRIPTOR-ROLE")
    observed_class = "valid" if outcome.accepted else outcome.result_class
    if expected_input != {"class": observed_class, "diagnostic": outcome.diagnostic}:
        raise ExecutorFailure("P51-DESCRIPTOR-INPUT-CONTRACT")
    return path


def _artifact_aggregate(root: Path) -> str:
    artifacts = root / "artifacts"
    try:
        metadata = os.lstat(artifacts)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
        with os.scandir(artifacts) as directory:
            entries = sorted((Path(entry.path) for entry in directory), key=lambda item: item.name)
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT") from error
    hasher = hashlib.sha256()
    for path in entries:
        try:
            metadata = os.lstat(path)
            if stat.S_ISLNK(metadata.st_mode):
                raise ExecutorFailure("P51-DESCRIPTOR-PATH-CONFINEMENT")
            if stat.S_ISDIR(metadata.st_mode):
                with os.scandir(path) as children:
                    if next(children, None) is not None:
                        raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
                continue
            if not stat.S_ISREG(metadata.st_mode):
                raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
        except ExecutorFailure:
            raise
        except OSError as error:
            raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT") from error
        relative = path.relative_to(root).as_posix().encode("utf-8")
        hasher.update(len(relative).to_bytes(8, "big"))
        hasher.update(relative)
        hasher.update(
            hashlib.sha256(
                _safe_regular_bytes(path, "P51-DESCRIPTOR-ARTIFACT")
            ).digest()
        )
    return "sha256:" + hasher.hexdigest()


def validate_descriptor_root(
    descriptor_root: Path, private_runtime_root: Path | None = None
) -> tuple[Path, tuple[Descriptor, ...]]:
    """Validate exact 70/69/1 topology without using descriptor spellings as paths."""

    if private_runtime_root is None:
        root = _safe_directory(descriptor_root, "P51-DESCRIPTOR-ROOT")
    else:
        root = _runtime_path(
            private_runtime_root,
            descriptor_root,
            "P51-DESCRIPTOR-ROOT",
            require_directory=True,
        )
    try:
        with os.scandir(root) as directory:
            entries = {entry.name for entry in directory}
    except OSError as error:
        raise ExecutorFailure("P51-DESCRIPTOR-ROOT") from error
    if entries != {"artifacts", "case-manifest.json", "coverage-manifest.json"}:
        raise ExecutorFailure("P51-DESCRIPTOR-ROOT")
    manifest_path = root / "case-manifest.json"
    manifest_raw = _safe_regular_bytes(manifest_path, "P51-DESCRIPTOR-MANIFEST")
    manifest = _read_json(manifest_path, "P51-DESCRIPTOR-MANIFEST")
    if (
        manifest.get("schema") != "ferris.pulse-35-corpus-case-manifest/v1"
        or manifest.get("derivation") != "hmac-sha256-seed-key-domain-purpose-counter-v1"
        or manifest.get("logical_case_max") != 512
        or manifest.get("required_case_count") != 70
        or manifest.get("case_count") != 70
        or manifest.get("diagnostic_execution") is not False
        or manifest.get("product_files_modified") is not False
        or manifest.get("logical_retries") != 0
        or type(manifest.get("cases")) is not list
        or len(manifest["cases"]) != 70
    ):
        raise ExecutorFailure("P51-DESCRIPTOR-MANIFEST")
    coverage = _read_json(root / "coverage-manifest.json", "P51-DESCRIPTOR-COVERAGE")
    if (
        coverage.get("schema") != "ferris.pulse-35-corpus-coverage-manifest/v1"
        or coverage.get("case_manifest_sha256") != _digest(manifest_raw)
        or coverage.get("case_count") != 70
        or coverage.get("coverage_domains_closed") != "18/18"
        or coverage.get("coverage_interactions_closed") != "8/8"
        or coverage.get("diagnostic_execution") is not False
        or coverage.get("product_files_modified") is not False
        or coverage.get("logical_retries") != 0
        or type(coverage.get("derived_catalog")) is not dict
    ):
        raise ExecutorFailure("P51-DESCRIPTOR-COVERAGE")

    descriptors: list[Descriptor] = []
    expected_artifacts: set[str] = set()
    tokens: set[str] = set()
    for ordinal, case in enumerate(manifest["cases"], start=1):
        if type(case) is not dict:
            raise ExecutorFailure("P51-DESCRIPTOR-CASE")
        if case.get("ordinal") != ordinal:
            raise ExecutorFailure("P51-DESCRIPTOR-ORDER")
        case_id = case.get("case_id")
        order_token = case.get("order_token")
        profile_token = case.get("profile_token")
        if (
            type(case_id) is not str
            or type(order_token) is not str
            or type(profile_token) is not str
            or TOKEN.fullmatch(case_id) is None
            or TOKEN.fullmatch(order_token) is None
            or TOKEN.fullmatch(profile_token) is None
            or {case_id, order_token, profile_token} & tokens
        ):
            raise ExecutorFailure("P51-DESCRIPTOR-ORDER")
        tokens.update({case_id, order_token, profile_token})
        execution = case.get("execution")
        if type(execution) is not dict or set(execution) != {"mode", "format", "expected"}:
            raise ExecutorFailure("P51-DESCRIPTOR-CASE")
        mode = execution["mode"]
        output_format = execution["format"]
        expected = _expected_result(execution["expected"])
        final = ordinal == 70
        if final:
            if (
                mode != "no-launch"
                or output_format != "no-launch"
                or expected["result_class"] != "blocked"
                or case.get("external_prerequisite") != "external-immutable-binary-freeze"
            ):
                raise ExecutorFailure("P51-DESCRIPTOR-NO-LAUNCH")
        elif mode != "launch-ready" or output_format not in {"json", "human"}:
            raise ExecutorFailure("P51-DESCRIPTOR-TOPOLOGY")
        if output_format == "human" and expected["result_class"] not in {"success", "difference"}:
            raise ExecutorFailure("P51-DESCRIPTOR-RESULT")
        before = _role_path(root, case.get("before"), is_final=final)
        after = _role_path(root, case.get("after"), is_final=final)
        for role in (case["before"], case["after"]):
            target = role["target"]
            if type(target) is str:
                expected_artifacts.add(target)
        if type(case.get("semantic_witnesses")) is not dict:
            raise ExecutorFailure("P51-DESCRIPTOR-CASE")
        descriptors.append(Descriptor(ordinal, case_id, output_format, expected, before, after, mode))

    artifacts = root / "artifacts"
    try:
        with os.scandir(artifacts) as directory:
            actual_artifacts = {"artifacts/" + entry.name for entry in directory}
    except OSError as error:
        raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT") from error
    if actual_artifacts - expected_artifacts:
        raise ExecutorFailure("P51-DESCRIPTOR-ARTIFACT")
    if manifest.get("artifact_aggregate") != _artifact_aggregate(root):
        raise ExecutorFailure("P51-DESCRIPTOR-AGGREGATE")
    if sum(item.execution_mode == "launch-ready" for item in descriptors) != 69:
        raise ExecutorFailure("P51-DESCRIPTOR-TOPOLOGY")
    if sum(item.execution_mode == "no-launch" for item in descriptors) != 1:
        raise ExecutorFailure("P51-DESCRIPTOR-TOPOLOGY")
    return root, tuple(descriptors)


def profile_diff_argv(descriptor: Descriptor) -> list[str]:
    if (
        descriptor.execution_mode != "launch-ready"
        or descriptor.before is None
        or descriptor.after is None
        or descriptor.output_format not in {"json", "human"}
    ):
        raise ExecutorFailure("P51-ARGV-NO-LAUNCH")
    return [
        "profile-diff",
        "--before",
        str(descriptor.before),
        "--after",
        str(descriptor.after),
        "--format",
        descriptor.output_format,
    ]


def build_platform_dispatch(
    platform: str,
    executable: Path,
    descriptor: Descriptor,
    runtime_root: Path,
) -> Dispatch:
    """Build the exact native or WSL argv after checking every private path."""

    canonical = canonical_platform_id(platform)
    if descriptor.before is None or descriptor.after is None:
        raise ExecutorFailure("P51-ARGV-NO-LAUNCH")
    executable = _runtime_path(
        runtime_root, executable, "P51-DISPATCH-EXECUTABLE", require_regular=True
    )
    before = _runtime_path(
        runtime_root,
        descriptor.before,
        "P51-DISPATCH-BEFORE",
        allow_absent_leaf=True,
    )
    after = _runtime_path(
        runtime_root,
        descriptor.after,
        "P51-DISPATCH-AFTER",
        allow_absent_leaf=True,
    )
    native_argv = (
        "profile-diff",
        "--before",
        str(before),
        "--after",
        str(after),
        "--format",
        descriptor.output_format,
    )
    if canonical == "windows-x86_64":
        return Dispatch(
            platform=canonical,
            executable=executable,
            application_argv=native_argv,
            host_cwd=runtime_root,
            command=(str(executable), *native_argv),
            wsl_cwd=None,
        )
    wsl_executable = windows_to_wsl_absolute(executable, runtime_root)
    wsl_before = windows_to_wsl_absolute(before, runtime_root)
    wsl_after = windows_to_wsl_absolute(after, runtime_root)
    wsl_cwd = windows_to_wsl_absolute(runtime_root, runtime_root)
    return Dispatch(
        platform=canonical,
        executable=executable,
        application_argv=native_argv,
        host_cwd=runtime_root,
        command=(
            "wsl.exe",
            "--distribution",
            WSL_DISTRIBUTION,
            "--cd",
            wsl_cwd,
            "--exec",
            wsl_executable,
            "profile-diff",
            "--before",
            wsl_before,
            "--after",
            wsl_after,
            "--format",
            descriptor.output_format,
        ),
        wsl_cwd=wsl_cwd,
    )


def _subprocess_process_runner(dispatch: Dispatch) -> LaunchCapture:
    try:
        completed = subprocess.run(
            list(dispatch.command),
            cwd=dispatch.host_cwd,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=60,
        )
    except subprocess.TimeoutExpired as error:
        raise ExecutorFailure("P51-PROCESS-TIMEOUT") from error
    except OSError as error:
        code = "P51-WSL-UNAVAILABLE" if dispatch.platform == "ubuntu-24.04-x86_64" else "P51-PROCESS-LAUNCH"
        raise ExecutorFailure(code) from error
    return LaunchCapture(completed.returncode, completed.stdout, completed.stderr)


def _require_digest(value: object, code: str) -> str:
    if type(value) is not str or SHA256.fullmatch(value) is None:
        raise ExecutorFailure(code)
    return value


def _require_visible_metadata(value: object, code: str) -> str:
    if type(value) is not str or VISIBLE_METADATA.fullmatch(value) is None:
        raise ExecutorFailure(code)
    return value


def _validate_bounded_output(value: object) -> None:
    required = {
        "schema",
        "owner_output_framing",
        "stdout_retained_bytes",
        "stdout_observed_bytes",
        "stdout_omitted_observed_bytes",
        "stdout_unobserved_bytes_unknown",
        "stdout_complete",
        "stdout_truncated",
        "stdout_read_failed",
        "stderr_retained_bytes",
        "stderr_observed_bytes",
        "stderr_omitted_observed_bytes",
        "stderr_unobserved_bytes_unknown",
        "stderr_complete",
        "stderr_truncated",
        "stderr_read_failed",
        "output_digest",
        "termination",
        "termination_scope",
        "termination_cleanup_complete",
    }
    if type(value) is not dict or set(value) != required:
        raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    if (
        value["schema"] != "ferris.bounded-output-evidence/v0"
        or value["owner_output_framing"] != "length-prefixed-stdout-stderr/v1"
        or value["termination"] not in {"completed", "read-failed", "timeout", "output-bound"}
        or value["termination_scope"] != "direct-child"
        or type(value["termination_cleanup_complete"]) is not bool
    ):
        raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    for key in (
        "stdout_retained_bytes",
        "stdout_observed_bytes",
        "stdout_omitted_observed_bytes",
        "stderr_retained_bytes",
        "stderr_observed_bytes",
        "stderr_omitted_observed_bytes",
    ):
        if type(value[key]) is not int or value[key] < 0:
            raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    for key in (
        "stdout_unobserved_bytes_unknown",
        "stdout_complete",
        "stdout_truncated",
        "stdout_read_failed",
        "stderr_unobserved_bytes_unknown",
        "stderr_complete",
        "stderr_truncated",
        "stderr_read_failed",
    ):
        if type(value[key]) is not bool:
            raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    _require_digest(value["output_digest"], "P51-RESULT-DIAGNOSTIC")


def _validate_diagnostic(value: object, result_class: str) -> dict[str, object]:
    required = {
        "code",
        "severity",
        "result_class",
        "message",
        "source_digest",
        "next_actions",
    }
    optional = required | {"bounded_output"}
    if type(value) is not dict or (set(value) != required and set(value) != optional):
        raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    if (
        type(value["code"]) is not str
        or DIAGNOSTIC_CODE.fullmatch(value["code"]) is None
        or value["code"] not in DIAGNOSTIC_CODES_BY_CLASS.get(result_class, frozenset())
        or value["severity"] != "error"
        or value["result_class"] != result_class
        or type(value["message"]) is not str
        or not value["message"]
        or type(value["next_actions"]) is not list
        or len(value["next_actions"]) != 1
        or type(value["next_actions"][0]) is not str
        or not value["next_actions"][0]
    ):
        raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    if value["source_digest"] is not None:
        _require_digest(value["source_digest"], "P51-RESULT-DIAGNOSTIC")
    if "bounded_output" in value:
        _validate_bounded_output(value["bounded_output"])
    return value


def _semantic_profile(path: Path) -> tuple[Outcome, dict[str, object] | None]:
    """Load one current descriptor input through the frozen P31 contract."""

    outcome = validate_path(path)
    if not outcome.accepted:
        return outcome, None
    data = _safe_regular_bytes(path, "P51-RESULT-INPUT", maximum=1_048_577)
    if not validate_bytes(data).accepted:
        raise ExecutorFailure("P51-RESULT-INPUT")
    try:
        return outcome, parse_accepted_profile(data)
    except P31Failure as error:
        raise ExecutorFailure("P51-RESULT-INPUT") from error


def _descriptor_semantics(
    descriptor: Descriptor,
) -> frozen_profile_diff.ProfileDiffSemantics:
    """Derive the output contract from the explicit before/after inputs only."""

    if descriptor.before is None or descriptor.after is None:
        raise ExecutorFailure("P51-RESULT-INPUT")
    before_outcome, before = _semantic_profile(descriptor.before)
    after_outcome, after = _semantic_profile(descriptor.after)
    semantics = frozen_profile_diff.derive_profile_diff(
        str(descriptor.before),
        before_outcome.result_class,
        before,
        str(descriptor.after),
        after_outcome.result_class,
        after,
    )
    if semantics.result_class != descriptor.expected["result_class"]:
        raise ExecutorFailure("P51-DESCRIPTOR-RESULT")
    return semantics


def _validate_profile_reference(value: object) -> dict[str, object]:
    fields = {"profile_id", "revision", "consumer", "content_digest"}
    if type(value) is not dict or set(value) != fields:
        raise ExecutorFailure("P51-RESULT-RECORD")
    for key in ("profile_id", "revision", "consumer"):
        _require_visible_metadata(value[key], "P51-RESULT-RECORD")
    _require_digest(value["content_digest"], "P51-RESULT-RECORD")
    return value


def _validate_change(value: object) -> dict[str, object]:
    fields = {"path", "change_kind", "before_value_digest", "after_value_digest"}
    if type(value) is not dict or set(value) != fields:
        raise ExecutorFailure("P51-RESULT-RECORD")
    path = value["path"]
    kind = value["change_kind"]
    if type(path) is not str or not path.startswith("/") or "\x00" in path:
        raise ExecutorFailure("P51-RESULT-RECORD")
    if kind not in {"added", "removed", "changed"}:
        raise ExecutorFailure("P51-RESULT-RECORD")
    before = value["before_value_digest"]
    after = value["after_value_digest"]
    if kind == "added":
        if before is not None:
            raise ExecutorFailure("P51-RESULT-RECORD")
        _require_digest(after, "P51-RESULT-RECORD")
    elif kind == "removed":
        if after is not None:
            raise ExecutorFailure("P51-RESULT-RECORD")
        _require_digest(before, "P51-RESULT-RECORD")
    else:
        _require_digest(before, "P51-RESULT-RECORD")
        _require_digest(after, "P51-RESULT-RECORD")
    return value


def _validate_record(
    value: object,
    result_class: str,
    expected_record: dict[str, object],
) -> dict[str, object]:
    required = {
        "schema",
        "diff_id",
        "before",
        "after",
        "changed_sections",
        "changes",
        "unchanged_sections",
        "unknowns",
        "limitations",
        "executable",
    }
    if type(value) is not dict or set(value) != required:
        raise ExecutorFailure("P51-RESULT-RECORD")
    if (
        value["schema"] != "ferris.profile-diff/v0"
        or type(value["diff_id"]) is not str
        or DIFF_ID.fullmatch(value["diff_id"]) is None
        or value["executable"] is not False
        or value["unknowns"] != list(UNKNOWN_LINES)
        or value["limitations"] != list(LIMITATION_LINES)
    ):
        raise ExecutorFailure("P51-RESULT-RECORD")
    _validate_profile_reference(value["before"])
    _validate_profile_reference(value["after"])
    changed = value["changed_sections"]
    unchanged = value["unchanged_sections"]
    changes = value["changes"]
    if (
        type(changed) is not list
        or type(unchanged) is not list
        or type(changes) is not list
        or len(changed) > len(SECTIONS)
        or len(unchanged) > len(SECTIONS)
        or len(changes) > 10_000
        or changed != sorted(changed)
        or unchanged != sorted(unchanged)
        or len(set(changed)) != len(changed)
        or len(set(unchanged)) != len(unchanged)
        or set(changed) | set(unchanged) != set(SECTIONS)
        or set(changed) & set(unchanged)
        or any(section not in SECTIONS for section in [*changed, *unchanged])
    ):
        raise ExecutorFailure("P51-RESULT-RECORD")
    validated_changes = [_validate_change(change) for change in changes]
    paths = [str(change["path"]) for change in validated_changes]
    if paths != sorted(paths) or len(set(paths)) != len(paths):
        raise ExecutorFailure("P51-RESULT-RECORD")
    if result_class == "success" and (changed or changes):
        raise ExecutorFailure("P51-RESULT-RECORD")
    if result_class == "difference" and not changes:
        raise ExecutorFailure("P51-RESULT-RECORD")
    if value["diff_id"] != frozen_profile_diff.diff_identity(value):
        raise ExecutorFailure("P51-RESULT-DIFF-ID")
    if value != expected_record:
        raise ExecutorFailure("P51-RESULT-SEMANTICS")
    return value


def _machine_semantic_projection(envelope: dict[str, object]) -> dict[str, object]:
    diagnostics = envelope["diagnostics"]
    projected_diagnostics: list[dict[str, object]] = []
    for diagnostic in diagnostics:
        assert type(diagnostic) is dict
        projected_diagnostics.append(
            {
                "code": diagnostic["code"],
                "message": diagnostic["message"],
                "next_actions": diagnostic["next_actions"],
                "result_class": diagnostic["result_class"],
                "severity": diagnostic["severity"],
                "source_digest": diagnostic["source_digest"],
                **(
                    {"bounded_output": diagnostic["bounded_output"]}
                    if "bounded_output" in diagnostic
                    else {}
                ),
            }
        )
    return {
        "semantic_command_id": envelope["semantic_command_id"],
        "result_class": envelope["result_class"],
        "process_exit_code": envelope["process_exit_code"],
        "diagnostics": projected_diagnostics,
        "record": envelope["record"],
    }


def _json_normalized(
    capture: LaunchCapture,
    expected: dict[str, object],
    semantics: frozen_profile_diff.ProfileDiffSemantics,
) -> dict[str, object]:
    selected = capture.stdout if expected["stream"] == "stdout-only" else capture.stderr
    other = capture.stderr if expected["stream"] == "stdout-only" else capture.stdout
    if other or b"\r" in selected or not selected.endswith(b"\n") or selected.endswith(b"\n\n"):
        raise ExecutorFailure("P51-RESULT-ROUTE")
    try:
        envelope = json.loads(selected, object_pairs_hook=_duplicate_free_object)
    except (UnicodeDecodeError, json.JSONDecodeError, ExecutorFailure) as error:
        raise ExecutorFailure("P51-RESULT-JSON") from error
    fields = {
        "schema",
        "command_version",
        "semantic_command_id",
        "selection_identity",
        "invocation_identity",
        "result_identity",
        "result_class",
        "process_exit_code",
        "diagnostics",
        "record",
    }
    if type(envelope) is not dict or set(envelope) != fields:
        raise ExecutorFailure("P51-RESULT-ENVELOPE")
    if (
        envelope["schema"] != "ferris.command-result/v2"
        or envelope["command_version"] != COMMAND_VERSION
        or envelope["semantic_command_id"] != "profile-diff"
        or envelope["result_class"] != expected["result_class"]
        or envelope["result_class"] != semantics.result_class
        or envelope["process_exit_code"] != expected["exit"]
        or capture.returncode != expected["exit"]
    ):
        raise ExecutorFailure("P51-RESULT-EXPECTATION")
    for name in ("selection_identity", "invocation_identity", "result_identity"):
        if type(envelope[name]) is not str or IDENTITY.fullmatch(envelope[name]) is None:
            raise ExecutorFailure("P51-RESULT-ENVELOPE")
    if envelope["selection_identity"] != semantics.selection_identity:
        raise ExecutorFailure("P51-RESULT-SELECTION-IDENTITY")
    if envelope["invocation_identity"] != semantics.invocation_identity:
        raise ExecutorFailure("P51-RESULT-INVOCATION-IDENTITY")
    diagnostics = envelope["diagnostics"]
    if type(diagnostics) is not list:
        raise ExecutorFailure("P51-RESULT-DIAGNOSTIC")
    if expected["diagnostics"] == "empty":
        if diagnostics:
            raise ExecutorFailure("P51-RESULT-EXPECTATION")
    elif len(diagnostics) != 1:
        raise ExecutorFailure("P51-RESULT-EXPECTATION")
    for diagnostic in diagnostics:
        _validate_diagnostic(diagnostic, str(expected["result_class"]))
    record = envelope["record"]
    if expected["record"] == "non-null":
        if semantics.record is None:
            raise ExecutorFailure("P51-RESULT-SEMANTICS")
        _validate_record(record, str(expected["result_class"]), semantics.record)
    elif record is not None:
        raise ExecutorFailure("P51-RESULT-EXPECTATION")
    if envelope["result_identity"] != frozen_profile_diff.result_identity(envelope):
        raise ExecutorFailure("P51-RESULT-IDENTITY")
    projection = _machine_semantic_projection(envelope)
    return {
        "format": "json",
        "result_class": expected["result_class"],
        "exit": expected["exit"],
        "stream": expected["stream"],
        "raw_sha256": _digest(selected),
        "normalized_sha256": _digest(_canonical_json(envelope)),
        "semantic_projection": projection,
        "semantic_projection_sha256": _digest(_canonical_json(projection)),
    }


def _human_list(
    lines: list[str],
    index: int,
    header: str,
    *,
    required_values: tuple[str, ...] | None = None,
) -> tuple[list[str], int]:
    if index >= len(lines) or lines[index] != header:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    index += 1
    values: list[str] = []
    while index < len(lines) and lines[index].startswith("  - "):
        values.append(lines[index][4:])
        index += 1
    if not values:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    if values == ["none"]:
        values = []
    elif "none" in values:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    if required_values is not None and tuple(values) != required_values:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    return values, index


def _parse_human_reference(line: str, prefix: str) -> dict[str, object]:
    if not line.startswith(prefix):
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    body = line[len(prefix) :]
    markers = (", revision=", ", consumer=", ", content_digest=")
    try:
        profile_id, remainder = body.split(markers[0], 1)
        revision, remainder = remainder.split(markers[1], 1)
        consumer, content_digest = remainder.split(markers[2], 1)
    except ValueError as error:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR") from error
    reference = {
        "profile_id": profile_id,
        "revision": revision,
        "consumer": consumer,
        "content_digest": content_digest,
    }
    _validate_profile_reference(reference)
    return reference


def _parse_human_change(line: str) -> dict[str, object]:
    prefix = "  - "
    suffix = ")"
    marker = " (before_digest="
    if not line.startswith(prefix) or not line.endswith(suffix) or marker not in line:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    path_and_kind, digests = line[len(prefix) : -1].split(marker, 1)
    try:
        path, kind = path_and_kind.rsplit(": ", 1)
        before, after = digests.split(", after_digest=", 1)
    except ValueError as error:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR") from error
    return _validate_change(
        {
            "path": path,
            "change_kind": kind,
            "before_value_digest": None if before == "none" else before,
            "after_value_digest": None if after == "none" else after,
        }
    )


def _human_normalized(
    capture: LaunchCapture,
    expected: dict[str, object],
    semantics: frozen_profile_diff.ProfileDiffSemantics,
) -> dict[str, object]:
    if (
        capture.returncode != expected["exit"]
        or capture.stderr
        or not capture.stdout
        or b"\r" in capture.stdout
        or not capture.stdout.endswith(b"\n")
        or capture.stdout.endswith(b"\n\n")
    ):
        raise ExecutorFailure("P51-RESULT-ROUTE")
    try:
        text = capture.stdout.decode("utf-8")
    except UnicodeDecodeError as error:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR") from error
    lines = text[:-1].split("\n")
    if len(lines) < 14 or not lines[0].startswith("Ferris profile diff "):
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    diff_id = lines[0].removeprefix("Ferris profile diff ")
    if DIFF_ID.fullmatch(diff_id) is None:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    if (
        lines[1] != "Schema: ferris.profile-diff/v0"
        or lines[2] != f"Result: {expected['result_class']}"
        or lines[3] != "Executable: false"
    ):
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    before = _parse_human_reference(lines[4], "Before: profile_id=")
    after = _parse_human_reference(lines[5], "After: profile_id=")
    index = 6
    changed, index = _human_list(lines, index, "Changed sections:")
    changes: list[dict[str, object]] = []
    if index >= len(lines) or lines[index] != "Changes:":
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    index += 1
    change_lines: list[str] = []
    while index < len(lines) and lines[index].startswith("  - "):
        change_lines.append(lines[index])
        index += 1
    if not change_lines:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    if change_lines == ["  - none"]:
        pass
    elif "  - none" in change_lines:
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    else:
        changes = [_parse_human_change(line) for line in change_lines]
    unchanged, index = _human_list(lines, index, "Unchanged sections:")
    unknowns, index = _human_list(
        lines, index, "Unknowns:", required_values=UNKNOWN_LINES
    )
    limitations, index = _human_list(
        lines, index, "Limitations:", required_values=LIMITATION_LINES
    )
    if index != len(lines):
        raise ExecutorFailure("P51-HUMAN-GRAMMAR")
    record = {
        "schema": "ferris.profile-diff/v0",
        "diff_id": diff_id,
        "before": before,
        "after": after,
        "changed_sections": changed,
        "changes": changes,
        "unchanged_sections": unchanged,
        "unknowns": unknowns,
        "limitations": limitations,
        "executable": False,
    }
    if (
        semantics.result_class != expected["result_class"]
        or semantics.record is None
    ):
        raise ExecutorFailure("P51-RESULT-SEMANTICS")
    _validate_record(record, str(expected["result_class"]), semantics.record)
    projection = {
        "semantic_command_id": "profile-diff",
        "result_class": expected["result_class"],
        "process_exit_code": expected["exit"],
        "diagnostics": [],
        "record": record,
    }
    return {
        "format": "human",
        "result_class": expected["result_class"],
        "exit": expected["exit"],
        "stream": "stdout-only",
        "raw_sha256": _digest(capture.stdout),
        "normalized_sha256": _digest(_canonical_json(record)),
        "semantic_projection": projection,
        "semantic_projection_sha256": _digest(_canonical_json(projection)),
    }


def _run_descriptor(
    descriptor: Descriptor,
    platform: str,
    executable: Path,
    runtime_root: Path,
    process_runner: ProcessRunner,
) -> dict[str, object]:
    semantics = _descriptor_semantics(descriptor)
    dispatch = build_platform_dispatch(platform, executable, descriptor, runtime_root)
    capture = process_runner(dispatch)
    normalized = (
        _json_normalized(capture, descriptor.expected, semantics)
        if descriptor.output_format == "json"
        else _human_normalized(capture, descriptor.expected, semantics)
    )
    return {
        "argv": list(dispatch.application_argv),
        "command": list(dispatch.command),
        "case_id": descriptor.case_id,
        "host_cwd": str(dispatch.host_cwd),
        "ordinal": descriptor.ordinal,
        "platform": dispatch.platform,
        "process_launched": True,
        "result": normalized,
        "wsl_cwd": dispatch.wsl_cwd,
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


def _execution_event(gate_id: str, kind: str, outcome: str) -> dict[str, object]:
    return {
        "classification": "ordered-execution",
        "event_kind": kind,
        "gate_id": gate_id,
        "outcome": outcome,
        "schema": P43_EVENT_SCHEMA,
    }


def _catalog() -> dict[str, object]:
    return {"schema": P43_CATALOG_SCHEMA, "gate_ids": list(P50_GATE_IDS)}


def _terminal_result(
    p43: object | None,
    events: list[dict[str, object]],
    gate: str,
    code: str,
    private_record: dict[str, object],
) -> ExecutorResult:
    events.append(_execution_event(gate, "terminal-stop", "failed"))
    if p43 is not None:
        p43.validate_catalog(_catalog())
        p43.validate_events(tuple(P50_GATE_IDS), events)
    private_record["failure_code"] = code
    private_record["outcome"] = "failed"
    return ExecutorResult(_catalog(), events, private_record)


def _normalize_custodies(
    custodies: Mapping[str, P44CustodyBinding],
) -> dict[str, P44CustodyBinding]:
    result: dict[str, P44CustodyBinding] = {}
    for supplied_platform, custody in custodies.items():
        canonical = canonical_platform_id(supplied_platform)
        if not isinstance(custody, P44CustodyBinding):
            raise ExecutorFailure("P51-P44-CUSTODY-BINDING")
        if canonical_platform_id(custody.platform) != canonical or canonical in result:
            raise ExecutorFailure("P51-P44-CUSTODY-BINDING")
        result[canonical] = custody
    if set(result) != set(CANONICAL_PLATFORMS):
        raise ExecutorFailure("P51-P44-CUSTODY-BINDING")
    return result


def _verify_custody_binary(
    custody: P44CustodyBinding,
    expectation: BinaryExpectation,
    runtime_root: Path,
) -> Path:
    final_root = _runtime_path(
        runtime_root,
        custody.final_root,
        "P51-P44-CUSTODY-ROOT",
        require_directory=True,
    )
    _runtime_path(
        runtime_root,
        custody.work_root,
        "P51-P44-WORK-ROOT",
        allow_absent_leaf=True,
    )
    try:
        with os.scandir(final_root) as directory:
            entries = sorted(entry.name for entry in directory)
    except OSError as error:
        raise ExecutorFailure("P51-P33-BINARY-IDENTITY") from error
    receipt_name = expectation.logical_filename + ".receipt.json"
    if entries != [expectation.logical_filename, receipt_name]:
        raise ExecutorFailure("P51-P33-BINARY-IDENTITY")
    executable = _runtime_path(
        runtime_root,
        final_root / expectation.logical_filename,
        "P51-P33-BINARY-IDENTITY",
        require_regular=True,
    )
    receipt_path = _runtime_path(
        runtime_root,
        final_root / receipt_name,
        "P51-P33-RECEIPT",
        require_regular=True,
    )
    executable_bytes = _safe_regular_bytes(
        executable, "P51-P33-BINARY-IDENTITY", maximum=16_777_216
    )
    if len(executable_bytes) != expectation.size or _digest(executable_bytes) != expectation.sha256:
        raise ExecutorFailure("P51-P33-BINARY-IDENTITY")
    receipt = _read_json(receipt_path, "P51-P33-RECEIPT", maximum=65_536)
    if set(receipt) != {"payload", "payload_sha256", "schema"}:
        raise ExecutorFailure("P51-P33-RECEIPT")
    payload = receipt["payload"]
    if (
        receipt["schema"] != "ferris.public-build-freeze-envelope/v1"
        or type(payload) is not dict
        or receipt["payload_sha256"] != _digest(_canonical_json(payload))
    ):
        raise ExecutorFailure("P51-P33-RECEIPT")
    if set(payload) != {"artifact", "build", "checkout", "cutoff", "platform", "safety", "schema"}:
        raise ExecutorFailure("P51-P33-RECEIPT")
    artifact = payload["artifact"]
    build = payload["build"]
    if (
        payload["schema"] != "ferris.public-build-freeze-receipt/v1"
        or payload["cutoff"] != P33_CUTOFF
        or payload["platform"] != expectation.platform
        or payload["checkout"]
        != {
            "core_autocrlf": False,
            "exact_commit": True,
            "tracked_files_clean": True,
        }
        or payload["safety"] != {"diagnostic_execution": False, "product_files_modified": False}
        or type(artifact) is not dict
        or artifact
        != {
            "discovery": "cargo-compiler-artifact-json",
            "logical_filename": expectation.logical_filename,
            "retained_in_public_bundle": True,
            "sha256": expectation.sha256,
            "size": expectation.size,
        }
        or type(build) is not dict
    ):
        raise ExecutorFailure("P51-P33-RECEIPT")
    expected_build = {
        "binary": "ferris",
        "cargo_version": expectation.cargo_version,
        "command": [
            "cargo",
            "build",
            "--locked",
            "--release",
            "--package",
            "ferris-cli",
            "--bin",
            "ferris",
            "--message-format=json-render-diagnostics",
        ],
        "package": "ferris-cli",
        "profile": "release",
        "reproducibility_controls": list(expectation.reproducibility_controls),
        "rustc_host": expectation.rustc_host,
        "rustc_version": expectation.rustc_version,
    }
    if build != expected_build:
        raise ExecutorFailure("P51-P33-TOOLCHAIN")
    if receipt["payload_sha256"] != expectation.published_receipt_payload_sha256:
        raise ExecutorFailure("P51-P33-RECEIPT")
    return executable


def _p45_bridge_identities(p45: object) -> dict[str, str]:
    names = {
        "manifest_aggregate": "P44_MANIFEST_AGGREGATE",
        "manifest_raw_sha256": "P44_MANIFEST_RAW_SHA256",
        "qualification_receipt_payload_sha256": "P44_RECEIPT_PAYLOAD_SHA256",
        "qualification_receipt_raw_sha256": "P44_RECEIPT_RAW_SHA256",
        "release_seal_payload_sha256": "P44_SEAL_PAYLOAD_SHA256",
        "release_seal_raw_sha256": "P44_SEAL_RAW_SHA256",
    }
    identities: dict[str, str] = {}
    for field, name in names.items():
        value = getattr(p45, name, None)
        if type(value) is not str or SHA256.fullmatch(value) is None:
            raise ExecutorFailure("P51-P45-IDENTITY")
        identities[field] = value
    return identities


def _validate_p45_bridge_result(p45: object, bridged: object, platform: str) -> None:
    canonical = canonical_platform_id(platform)
    expected = {
        "bridge": {
            "invocation_count": 1,
            "platform": canonical,
            "pulse_44_release": _p45_bridge_identities(p45),
            "retries": 0,
        },
        "ordered_execution_event": _execution_event(
            p45.PLATFORM_GATES[canonical], "gate-complete", "passed"
        ),
        "outcome": "passed",
        "schema": p45.SUMMARY_SCHEMA,
    }
    if bridged != expected:
        raise ExecutorFailure("P51-P45-CUSTODY")


def _bridge_p44_once(
    p45: object,
    repo_root: Path,
    custody: P44CustodyBinding,
    platform: str,
    runtime_root: Path,
) -> None:
    canonical = canonical_platform_id(platform)
    work_root = _runtime_path(
        runtime_root,
        custody.work_root,
        "P51-P44-WORK-ROOT",
        allow_absent_leaf=True,
    )
    final_root = _runtime_path(
        runtime_root,
        custody.final_root,
        "P51-P44-CUSTODY-ROOT",
        require_directory=True,
    )
    calls = 0

    def supplied_summary(
        repo: Path, cutoff: str, supplied_platform: str, work: Path, final: Path
    ) -> object:
        nonlocal calls
        calls += 1
        if (
            calls != 1
            or repo != repo_root
            or cutoff != P33_CUTOFF
            or supplied_platform != canonical
            or Path(work) != work_root
            or Path(final) != final_root
        ):
            raise RuntimeError("Pulse 45 callback binding mismatch")
        return custody.summary

    try:
        bridged = p45.bridge_pulse_44(
            repo_root,
            P33_CUTOFF,
            canonical,
            work_root,
            final_root,
            invoker=supplied_summary,
        )
    except (OSError, subprocess.SubprocessError) as error:
        raise ExecutorFailure("P51-P45-BRIDGE") from error
    if calls != 1:
        raise ExecutorFailure("P51-P45-CUSTODY")
    _validate_p45_bridge_result(p45, bridged, canonical)


def _private_cycle_root(runtime_root: Path, path: Path) -> Path:
    return _runtime_path(
        runtime_root,
        path,
        "P51-P27-CYCLE-ROOT",
        allow_absent_leaf=True,
    )


def _remove_private_tree(path: Path) -> None:
    if not os.path.lexists(path):
        return
    try:
        metadata = os.lstat(path)
        if stat.S_ISLNK(metadata.st_mode):
            raise ExecutorFailure("P51-P27-CLEANUP")
        if stat.S_ISDIR(metadata.st_mode):
            with os.scandir(path) as directory:
                for entry in directory:
                    _remove_private_tree(Path(entry.path))
            os.rmdir(path)
        elif stat.S_ISREG(metadata.st_mode):
            os.unlink(path)
        else:
            raise ExecutorFailure("P51-P27-CLEANUP")
    except ExecutorFailure:
        raise
    except OSError as error:
        raise ExecutorFailure("P51-P27-CLEANUP") from error


def _cleanup_failed_p27_cycle(cycle_root: Path) -> None:
    _remove_private_tree(cycle_root)
    if os.path.lexists(cycle_root):
        raise ExecutorFailure("P51-P27-CLEANUP")


def _run_p27_once(
    runtime_root: Path,
    cycle_root_value: Path,
    runner: Callable[[Path], dict[str, object]],
) -> None:
    cycle_root = _private_cycle_root(runtime_root, cycle_root_value)
    if os.path.lexists(cycle_root):
        raise ExecutorFailure("P51-P27-CYCLE-ROOT")
    invoked = False
    try:
        if invoked:
            raise ExecutorFailure("P51-P27-ONE-CALL")
        invoked = True
        summary = runner(cycle_root)
    except (ValueError, RuntimeError, OSError, subprocess.SubprocessError) as error:
        try:
            _cleanup_failed_p27_cycle(cycle_root)
        except ExecutorFailure:
            raise
        raise ExecutorFailure("P51-P27-EXECUTION") from error
    try:
        verify_p27_summary(summary)
    except DependencyFailure as error:
        try:
            _cleanup_failed_p27_cycle(cycle_root)
        except ExecutorFailure:
            raise
        raise ExecutorFailure("P51-P27-SUMMARY") from error
    if not os.path.lexists(cycle_root):
        raise ExecutorFailure("P51-P27-RETENTION")
    try:
        _runtime_path(
            runtime_root,
            cycle_root,
            "P51-P27-RETENTION",
            require_directory=True,
        )
    except ExecutorFailure:
        _cleanup_failed_p27_cycle(cycle_root)
        raise
    # P27 owns its successful durable cycle record.  It remains private and
    # retained; only a failed or malformed cycle is removed above.


def _execute(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    retained_custodies: Mapping[str, P44CustodyBinding],
    controls: _RuntimeControls,
) -> ExecutorResult:
    events: list[dict[str, object]] = []
    private_record: dict[str, object] = {
        "schema": "ferris.pulse-51-private-execution-record/v2",
        "outcome": "in-progress",
        "platform_records": {platform: [] for platform in CANONICAL_PLATFORMS},
        "no_launch_records": [],
        "process_counts": {platform: 0 for platform in CANONICAL_PLATFORMS},
        "p27_cycle_retention": "not-attempted",
    }
    current_gate = P50_GATE_IDS[0]
    p43: object | None = None
    try:
        p43, p45, _p47 = load_terminal_dependencies(repo_root)
        runtime_root = _safe_runtime_root(private_runtime_root)
        if set(controls.expectations) != set(CANONICAL_PLATFORMS):
            raise ExecutorFailure("P51-P33-BINARY-PLATFORM")
        if p45.PLATFORM_GATES != {
            "windows-x86_64": "windows-retained-binary-custody",
            "ubuntu-24.04-x86_64": "ubuntu-retained-binary-custody",
        }:
            raise ExecutorFailure("P51-P45-PLATFORM-BINDING")
        custodies = _normalize_custodies(retained_custodies)
        executable_by_platform = {
            platform: _verify_custody_binary(
                custodies[platform], controls.expectations[platform], runtime_root
            )
            for platform in CANONICAL_PLATFORMS
        }
        events.append(_validation_event("public-catalog-prevalidation", 5))

        # This event does not authenticate an authority.  P41/P39 custody and
        # authority status are external governance preconditions; no caller
        # controlled event or grant is accepted by this runtime.
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[1]
        _bridge_p44_once(
            p45,
            repo_root,
            custodies["windows-x86_64"],
            "windows-x86_64",
            runtime_root,
        )
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[2]
        _bridge_p44_once(
            p45,
            repo_root,
            custodies["ubuntu-24.04-x86_64"],
            "ubuntu-24.04-x86_64",
            runtime_root,
        )
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[3]
        p27_runner = controls.p27_runner or load_p27_exact_runner(repo_root)
        _run_p27_once(runtime_root, p27_cycle_root, p27_runner)
        private_record["p27_cycle_retention"] = "retained-private-cycle-root"
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[4]
        p31_summary = verify_bound_contract(repo_root)
        if p31_summary != {
            "artifact_count": 9,
            "positive_fixture_count": 6,
            "mutation_control_count": 33,
            "public_input_checks": 39,
        }:
            raise ExecutorFailure("P51-P31-CONTROL-COUNT")
        events.append(_validation_event("public-input-contract", 39))
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[5]
        p35_summary = verify_p35_p37_custody(repo_root)
        if p35_summary != {
            "bound_file_count": 11,
            "p35_release_tree_file_count": 10,
            "machine_schema_count": 1,
            "canonical_lf_file_count": 11,
            "git_clean_checks": 11,
        }:
            raise ExecutorFailure("P51-P35-CUSTODY-COUNT")
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[6]
        root, descriptors = validate_descriptor_root(descriptor_root, runtime_root)
        events.append(_execution_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[7]
        for descriptor in descriptors:
            if descriptor.execution_mode == "no-launch":
                for platform in CANONICAL_PLATFORMS:
                    private_record["no_launch_records"].append(
                        {
                            "case_id": descriptor.case_id,
                            "ordinal": descriptor.ordinal,
                            "platform": platform,
                            "process_launched": False,
                            "reason": "blocked-no-launch-external-immutable-binary-freeze",
                        }
                    )
                continue
            windows = _run_descriptor(
                descriptor,
                "windows-x86_64",
                executable_by_platform["windows-x86_64"],
                runtime_root,
                controls.process_runner,
            )
            private_record["platform_records"]["windows-x86_64"].append(windows)
            private_record["process_counts"]["windows-x86_64"] += 1
            ubuntu = _run_descriptor(
                descriptor,
                "ubuntu-24.04-x86_64",
                executable_by_platform["ubuntu-24.04-x86_64"],
                runtime_root,
                controls.process_runner,
            )
            private_record["platform_records"]["ubuntu-24.04-x86_64"].append(ubuntu)
            private_record["process_counts"]["ubuntu-24.04-x86_64"] += 1
            if windows["result"]["semantic_projection"] != ubuntu["result"]["semantic_projection"]:
                private_record["first_mismatch_ordinal"] = descriptor.ordinal
                raise ExecutorFailure("P51-FIRST-TARGET-MISMATCH")

        if (
            private_record["process_counts"]
            != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
            or len(private_record["no_launch_records"]) != 2
        ):
            raise ExecutorFailure("P51-TOPOLOGY-ACCOUNTING")
        events.append(_execution_event(current_gate, "terminal-stop", "completed"))
        p43.validate_catalog(_catalog())
        p43.validate_events(tuple(P50_GATE_IDS), events)
        private_record["outcome"] = "completed"
        return ExecutorResult(_catalog(), events, private_record)
    except (ExecutorFailure, P31Failure, CustodyFailure, DependencyFailure) as error:
        return _terminal_result(
            p43,
            events,
            current_gate,
            getattr(error, "code", "P51-UNCLASSIFIED"),
            private_record,
        )


def run_diagnostic_executor(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    retained_custodies: Mapping[str, P44CustodyBinding],
) -> ExecutorResult:
    """Run the fixed production executor; authority is verified externally.

    Windows always launches the retained `.exe` directly.  Ubuntu always
    launches the retained ELF through `wsl.exe --distribution Ubuntu-24.04
    --cd <translated-root> --exec <translated-elf> ...`.  This function has no
    synthetic, launcher, P27, expectation, event, or grant injection point.
    """

    return _execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        retained_custodies,
        _RuntimeControls(
            expectations=P33_EXPECTATIONS,
            p27_runner=None,
            process_runner=_subprocess_process_runner,
        ),
    )


def _run_qualification_executor(
    repo_root: Path,
    descriptor_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    retained_custodies: Mapping[str, P44CustodyBinding],
    *,
    p27_runner: Callable[[Path], dict[str, object]],
    process_runner: ProcessRunner,
    expectations: Mapping[str, BinaryExpectation],
) -> ExecutorResult:
    """Private qualification seam; never a documented or exported runtime API."""

    return _execute(
        repo_root,
        descriptor_root,
        private_runtime_root,
        p27_cycle_root,
        retained_custodies,
        _RuntimeControls(
            expectations=expectations,
            p27_runner=p27_runner,
            process_runner=process_runner,
        ),
    )


def _terminal_sync_not_attempted() -> dict[str, object]:
    return {
        "attempted": False,
        "error_category": "not-attempted",
        "mechanism": "not-attempted",
        "status": "not-attempted",
    }


def _p47_failure_summary(p47: object | None, code: str) -> dict[str, object]:
    return {
        "failure_code": code,
        "schema": getattr(p47, "SUMMARY_SCHEMA", P47_SUMMARY_SCHEMA),
        "witness_publication": {
            "final_files_present": False,
            "rename_attempts": 0,
            "retries": 0,
            "state": "absent",
            "sync": {
                "final_parent": _terminal_sync_not_attempted(),
                "rollback_parent": _terminal_sync_not_attempted(),
                "stage": _terminal_sync_not_attempted(),
            },
        },
    }


def _safe_terminal_parent(path: Path) -> Path:
    try:
        return _safe_runtime_root(path)
    except ExecutorFailure as error:
        raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-PARENT-UNSAFE") from error


def _terminal_lexical_candidate(parent: Path, value: Path) -> tuple[Path, tuple[str, ...]]:
    try:
        requested = _absolute_windows_path(value, "P47-WITNESS-FINAL-ROOT-INVALID")
        lexical_relative = requested.relative_to(parent)
        resolved = requested.resolve(strict=False)
        resolved.relative_to(parent)
    except ExecutorFailure as error:
        raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-ROOT-INVALID") from error
    except (OSError, ValueError) as error:
        raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-ROOT-INVALID") from error
    if not lexical_relative.parts or os.path.lexists(requested) or os.path.lexists(resolved):
        raise _TerminalPreconditionFailure(
            "P47-WITNESS-FINAL-EXISTS"
            if os.path.lexists(requested) or os.path.lexists(resolved)
            else "P47-WITNESS-FINAL-ROOT-INVALID"
        )
    return resolved, lexical_relative.parts


def _terminal_candidate(parent: Path, value: Path) -> Path:
    resolved, lexical_parts = _terminal_lexical_candidate(parent, value)
    current = parent
    try:
        for part in lexical_parts[:-1]:
            current = current / part
            metadata = os.lstat(current)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-PARENT-UNSAFE")
    except _TerminalPreconditionFailure:
        raise
    except OSError as error:
        raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-PARENT-UNSAFE") from error
    return resolved


def _terminal_root(parent: Path, value: Path) -> Path:
    resolved = _terminal_candidate(parent, value)
    relative = resolved.relative_to(parent)
    current = parent
    try:
        for index, part in enumerate(relative.parts[:-1]):
            current = current / part
            metadata = os.lstat(current)
            if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
                raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-PARENT-UNSAFE")
        parent_metadata = os.lstat(resolved.parent)
        if stat.S_ISLNK(parent_metadata.st_mode) or not stat.S_ISDIR(parent_metadata.st_mode):
            raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-PARENT-UNSAFE")
    except _TerminalPreconditionFailure:
        raise
    except OSError as error:
        raise _TerminalPreconditionFailure("P47-WITNESS-FINAL-PARENT-UNSAFE") from error
    return resolved


def _non_overlapping(first: Path, second: Path) -> bool:
    try:
        first.relative_to(second)
        return False
    except ValueError:
        try:
            second.relative_to(first)
            return False
        except ValueError:
            return True


@dataclass
class TerminalPulse47Once:
    """One-use terminal integration constrained to one safe private parent."""

    repo_root: Path
    safe_terminal_parent: Path
    _used: bool = False

    def invoke(
        self,
        result: ExecutorResult,
        p43_final_root: Path,
        witness_final_root: Path,
    ) -> object:
        if self._used:
            raise ExecutorFailure("P51-P47-ALREADY-INVOKED")
        self._used = True
        try:
            _p43, _p45, p47 = load_terminal_dependencies(self.repo_root)
        except DependencyFailure as error:
            return _p47_failure_summary(None, error.code)
        try:
            parent = _safe_terminal_parent(self.safe_terminal_parent)
            p43_candidate, _p43_parts = _terminal_lexical_candidate(parent, p43_final_root)
            witness_candidate, _witness_parts = _terminal_lexical_candidate(
                parent, witness_final_root
            )
            if not _non_overlapping(p43_candidate, witness_candidate):
                raise _TerminalPreconditionFailure("P47-WITNESS-ROOTS-OVERLAP")
            p43_root = _terminal_root(parent, p43_final_root)
            witness_root = _terminal_root(parent, witness_final_root)
        except _TerminalPreconditionFailure as error:
            return _p47_failure_summary(p47, error.code)
        witness_failure = getattr(p47, "WitnessFailure", None)
        if not isinstance(witness_failure, type) or not issubclass(witness_failure, Exception):
            raise ExecutorFailure("P51-P47-SEALED-CALLABLE")
        try:
            return p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=p47.invoke_real_pulse_43,
            )
        except witness_failure as error:
            return _p47_failure_summary(p47, getattr(error, "code", "P47-P43-INVOCATION-FAILURE"))
        except OSError:
            return _p47_failure_summary(p47, "P47-P43-INVOCATION-FAILURE")


def invoke_terminal_pulse47_once(
    terminal: TerminalPulse47Once,
    result: ExecutorResult,
    p43_final_root: Path,
    witness_final_root: Path,
) -> object:
    """Use the supplied one-use terminal object exactly once."""

    if not isinstance(terminal, TerminalPulse47Once):
        raise ExecutorFailure("P51-P47-TERMINAL-OBJECT")
    return terminal.invoke(result, p43_final_root, witness_final_root)


__all__ = [
    "ExecutorFailure",
    "ExecutorResult",
    "P44CustodyBinding",
    "TerminalPulse47Once",
    "canonical_platform_id",
    "invoke_terminal_pulse47_once",
    "run_diagnostic_executor",
]
