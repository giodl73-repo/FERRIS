"""Fake-only inputs and cleanup for Pulse 52 qualification."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import time
from pathlib import Path
from typing import Mapping


SCRATCH_CLEANUP_DELAYS = (0.02, 0.05, 0.10, 0.20)


def cleanup_synthetic_runtime_root(root: Path) -> int:
    """Remove only a test-owned root with a bounded Windows-sharing retry."""

    attempts = 0
    for delay in (*SCRATCH_CLEANUP_DELAYS, None):
        if not os.path.lexists(root):
            return attempts
        attempts += 1
        try:
            shutil.rmtree(root)
        except OSError as error:
            retryable = isinstance(error, PermissionError) or getattr(error, "winerror", None) == 32
            if delay is not None and retryable:
                time.sleep(delay)
                continue
            raise RuntimeError("P52-SYNTHETIC-SCRATCH-CLEANUP") from error
        if not os.path.lexists(root):
            return attempts
        raise RuntimeError("P52-SYNTHETIC-SCRATCH-CLEANUP")
    raise AssertionError("unreachable bounded synthetic cleanup")


def synthetic_seed(cycle: int) -> bytes:
    """Deterministic fake-only bytes; production always calls secrets.token_bytes."""

    return hashlib.sha256(f"ferris-p52-synthetic-seed-{cycle}".encode("ascii")).digest()


def _sync(status: str) -> dict[str, object]:
    if status == "not-attempted":
        return {
            "attempted": False,
            "error_category": "not-attempted",
            "mechanism": "not-attempted",
            "status": status,
        }
    return {
        "attempted": True,
        "error_category": None,
        "mechanism": "os.open+os.fsync-directory-v1",
        "status": status,
    }


def synthetic_p44_summary() -> dict[str, object]:
    return {
        "custody": {
            "files": "2/2",
            "final_files_present": True,
            "final_verified": "2/2",
            "rename_attempts": 1,
            "retries": 0,
            "stage_verified": "2/2",
            "state": "published",
            "sync": {
                "final_parent": _sync("synced"),
                "rollback_parent": _sync("not-attempted"),
                "stage": _sync("synced"),
            },
            "work_verified": "2/2",
        },
        "ordered_execution_event": {
            "classification": "ordered-execution",
            "event_kind": "terminal-stop",
            "gate_id": "retained-binary-custody",
            "outcome": "completed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        },
        "outcome": "published",
        "schema": "ferris.pulse-44-retained-binary-custody-summary/v1",
    }


def _synthetic_receipt(p51: object, expectation: object) -> dict[str, object]:
    payload = {
        "artifact": {
            "discovery": "cargo-compiler-artifact-json",
            "logical_filename": expectation.logical_filename,
            "retained_in_public_bundle": True,
            "sha256": expectation.sha256,
            "size": expectation.size,
        },
        "build": {
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
        },
        "checkout": {
            "core_autocrlf": False,
            "exact_commit": True,
            "tracked_files_clean": True,
        },
        "cutoff": p51.P33_CUTOFF,
        "platform": expectation.platform,
        "safety": {"diagnostic_execution": False, "product_files_modified": False},
        "schema": "ferris.public-build-freeze-receipt/v1",
    }
    return {
        "payload": payload,
        "payload_sha256": p51._digest(p51._canonical_json(payload)),
        "schema": "ferris.public-build-freeze-envelope/v1",
    }


def create_synthetic_custodies(
    runtime_root: Path, p51: object
) -> tuple[dict[str, object], dict[str, object]]:
    """Create harmless public fake-binary custody beneath a test runtime root."""

    custody_root = runtime_root / "custodies"
    custody_root.mkdir()
    fake_source = Path(p51.__file__).resolve().parent / "fixtures" / "fake_ferris.py"
    raw = fake_source.read_bytes()
    digest = p51._digest(raw)
    custodies: dict[str, object] = {}
    expectations: dict[str, object] = {}
    for platform, suffix, cargo, rustc, host, controls in (
        (
            "windows-x86_64",
            ".exe",
            "cargo synthetic",
            "rustc synthetic windows",
            "x86_64-pc-windows-msvc",
            ("CARGO_INCREMENTAL=0", "RUSTFLAGS=-C link-arg=/Brepro"),
        ),
        (
            "ubuntu-24.04-x86_64",
            "",
            "cargo synthetic",
            "rustc synthetic ubuntu",
            "x86_64-unknown-linux-gnu",
            ("CARGO_INCREMENTAL=0",),
        ),
    ):
        filename = f"ferris-{platform}-{p51.P33_CUTOFF}{suffix}"
        final_root = custody_root / f"p44-final-{platform}"
        final_root.mkdir()
        executable = final_root / filename
        shutil.copyfile(fake_source, executable)
        initial = p51.BinaryExpectation(
            platform=platform,
            logical_filename=filename,
            size=len(raw),
            sha256=digest,
            cargo_version=cargo,
            rustc_version=rustc,
            rustc_host=host,
            reproducibility_controls=controls,
            published_receipt_payload_sha256="",
        )
        receipt = _synthetic_receipt(p51, initial)
        expectation = p51.BinaryExpectation(
            platform=initial.platform,
            logical_filename=initial.logical_filename,
            size=initial.size,
            sha256=initial.sha256,
            cargo_version=initial.cargo_version,
            rustc_version=initial.rustc_version,
            rustc_host=initial.rustc_host,
            reproducibility_controls=initial.reproducibility_controls,
            published_receipt_payload_sha256=receipt["payload_sha256"],
        )
        receipt = _synthetic_receipt(p51, expectation)
        (final_root / f"{filename}.receipt.json").write_bytes(p51._canonical_json(receipt))
        custodies[platform] = p51.P44CustodyBinding(
            platform=platform,
            final_root=final_root,
            work_root=custody_root / f"p44-work-{platform}",
            summary=synthetic_p44_summary(),
        )
        expectations[platform] = expectation
    return custodies, expectations


class QualificationProcessRunner:
    """Replace only P51's final subprocess boundary with a harmless fake."""

    def __init__(self, runtime_root: Path, p51: object) -> None:
        self.runtime_root = runtime_root
        self.p51 = p51
        self.counts = {"windows-x86_64": 0, "ubuntu-24.04-x86_64": 0}
        self.dispatches: list[object] = []

    def __call__(self, dispatch: object) -> object:
        self._assert_dispatch(dispatch)
        before_path = Path(dispatch.application_argv[2])
        after_path = Path(dispatch.application_argv[4])
        before_outcome, before_profile = self.p51._semantic_profile(before_path)
        after_outcome, after_profile = self.p51._semantic_profile(after_path)
        semantics = self.p51.frozen_profile_diff.derive_profile_diff(
            str(before_path),
            before_outcome.result_class,
            before_profile,
            str(after_path),
            after_outcome.result_class,
            after_profile,
        )
        exit_code = self.p51.RESULT_MAP[semantics.result_class]["exit"]
        diagnostics: list[dict[str, object]] = []
        if semantics.record is None:
            code = before_outcome.diagnostic
            if before_outcome.accepted:
                code = after_outcome.diagnostic
            if code is None and before_profile is not None and after_profile is not None:
                if before_profile["profile_id"] != after_profile["profile_id"]:
                    code = "FERRIS-PROFILE-DIFF-PROFILE-ID-MISMATCH"
                elif before_profile["consumer"] != after_profile["consumer"]:
                    code = "FERRIS-PROFILE-DIFF-CONSUMER-MISMATCH"
                elif semantics.result_class == "blocked":
                    code = "FERRIS-PROFILE-DIFF-BOUND-EXCEEDED"
            if code is None:
                raise AssertionError("synthetic output did not derive a diagnostic code")
            diagnostics.append(
                {
                    "code": code,
                    "severity": "error",
                    "result_class": semantics.result_class,
                    "message": "Synthetic qualification diagnostic.",
                    "source_digest": None,
                    "next_actions": ["Use only the declared synthetic fixture."],
                }
            )
        payload = {
            "schema": "ferris.command-result/v2",
            "command_version": "0.1.0",
            "semantic_command_id": "profile-diff",
            "selection_identity": semantics.selection_identity,
            "invocation_identity": semantics.invocation_identity,
            "result_identity": "",
            "result_class": semantics.result_class,
            "process_exit_code": exit_code,
            "diagnostics": diagnostics,
            "record": semantics.record,
        }
        payload["result_identity"] = self.p51.frozen_profile_diff.result_identity(payload)
        if dispatch.application_argv[6] == "json":
            serialized = json.dumps(payload, ensure_ascii=True, indent=2).encode("ascii") + b"\n"
            stdout, stderr = (
                (serialized, b"")
                if semantics.result_class in {"success", "difference"}
                else (b"", serialized)
            )
        else:
            if semantics.record is None:
                raise AssertionError("synthetic human output requires a record")
            stdout, stderr = self._human(semantics.record, semantics.result_class), b""
        self.counts[dispatch.platform] += 1
        self.dispatches.append(dispatch)
        return self.p51.LaunchCapture(exit_code, stdout, stderr)

    @staticmethod
    def _human(record: dict[str, object], result_class: str) -> bytes:
        before = record["before"]
        after = record["after"]
        assert type(before) is dict and type(after) is dict
        lines = [
            f"Ferris profile diff {record['diff_id']}",
            f"Schema: {record['schema']}",
            f"Result: {result_class}",
            "Executable: false",
            "Before: profile_id={profile_id}, revision={revision}, consumer={consumer}, content_digest={content_digest}".format(
                **before
            ),
            "After: profile_id={profile_id}, revision={revision}, consumer={consumer}, content_digest={content_digest}".format(
                **after
            ),
            "Changed sections:",
        ]
        changed_sections = record["changed_sections"]
        assert type(changed_sections) is list
        lines.extend(f"  - {value}" for value in changed_sections or ["none"])
        lines.append("Changes:")
        changes = record["changes"]
        assert type(changes) is list
        if changes:
            for change in changes:
                assert type(change) is dict
                before_digest = change["before_value_digest"] or "none"
                after_digest = change["after_value_digest"] or "none"
                lines.append(
                    f"  - {change['path']}: {change['change_kind']} "
                    f"(before_digest={before_digest}, after_digest={after_digest})"
                )
        else:
            lines.append("  - none")
        lines.append("Unchanged sections:")
        unchanged_sections = record["unchanged_sections"]
        assert type(unchanged_sections) is list
        lines.extend(f"  - {value}" for value in unchanged_sections or ["none"])
        lines.append("Unknowns:")
        lines.extend(f"  - {value}" for value in record["unknowns"])
        lines.append("Limitations:")
        lines.extend(f"  - {value}" for value in record["limitations"])
        return ("\n".join(lines) + "\n").encode("utf-8")

    def _assert_dispatch(self, dispatch: object) -> None:
        if (
            dispatch.host_cwd != self.runtime_root
            or tuple(dispatch.application_argv[:2]) != ("profile-diff", "--before")
            or len(dispatch.application_argv) != 7
            or dispatch.application_argv[3] != "--after"
            or dispatch.application_argv[5] != "--format"
            or dispatch.application_argv[6] not in {"json", "human"}
        ):
            raise AssertionError("qualification received an inexact application dispatch")
        for path in (
            dispatch.executable,
            Path(dispatch.application_argv[2]),
            Path(dispatch.application_argv[4]),
        ):
            path.relative_to(self.runtime_root)
        if dispatch.platform == "windows-x86_64":
            if dispatch.wsl_cwd is not None or dispatch.command != (
                str(dispatch.executable),
                *dispatch.application_argv,
            ):
                raise AssertionError("qualification received a non-native Windows dispatch")
            return
        expected = (
            "wsl.exe",
            "--distribution",
            "Ubuntu-24.04",
            "--cd",
            self.p51.windows_to_wsl_absolute(self.runtime_root, self.runtime_root),
            "--exec",
            self.p51.windows_to_wsl_absolute(dispatch.executable, self.runtime_root),
            "profile-diff",
            "--before",
            self.p51.windows_to_wsl_absolute(
                Path(dispatch.application_argv[2]), self.runtime_root
            ),
            "--after",
            self.p51.windows_to_wsl_absolute(
                Path(dispatch.application_argv[4]), self.runtime_root
            ),
            "--format",
            dispatch.application_argv[6],
        )
        if (
            dispatch.platform != "ubuntu-24.04-x86_64"
            or dispatch.wsl_cwd != expected[4]
            or dispatch.command != expected
        ):
            raise AssertionError("qualification received an inexact Ubuntu WSL dispatch")


def public_result_text(result: object) -> str:
    """Render only P43-safe return fields for privacy assertions."""

    return repr(
        {
            "catalog": result.catalog,
            "events": result.events,
            "publication": result.publication,
            "transfer_descriptor": result.transfer_descriptor,
        }
    )
