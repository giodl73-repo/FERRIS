"""Private-only synthetic inputs for Pulse 51 qualification.

The helpers create harmless generated descriptors and fake retained-binary
custody trees underneath a caller-selected disposable runtime root.  They are
not part of the exported production executor surface.
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import time
from pathlib import Path

from diagnostic_executor import (
    BinaryExpectation,
    P33_CUTOFF,
    P44CustodyBinding,
    RESULT_MAP,
    _canonical_json,
    _digest,
)


SECTIONS = (
    "identity",
    "closure",
    "features",
    "toolchain",
    "targets",
    "providers",
    "native",
    "stages",
    "assurance",
    "stewardship",
    "support",
    "lifecycle",
)


SCRATCH_CLEANUP_DELAYS = (0.02, 0.05, 0.10, 0.20)


def _retryable_scratch_cleanup_error(error: OSError) -> bool:
    return isinstance(error, PermissionError) or getattr(error, "winerror", None) == 32


def cleanup_synthetic_runtime_root(root: Path) -> int:
    """Remove only a disposable synthetic/test root with bounded lock retries."""

    attempts = 0
    for delay in (*SCRATCH_CLEANUP_DELAYS, None):
        if not os.path.lexists(root):
            return attempts
        attempts += 1
        try:
            shutil.rmtree(root)
        except OSError as error:
            if delay is not None and _retryable_scratch_cleanup_error(error):
                time.sleep(delay)
                continue
            raise RuntimeError("P51-SYNTHETIC-SCRATCH-CLEANUP") from error
        if not os.path.lexists(root):
            return attempts
        raise RuntimeError("P51-SYNTHETIC-SCRATCH-CLEANUP")
    raise AssertionError("unreachable bounded scratch cleanup")


def _token(label: str, ordinal: int) -> str:
    return hashlib.sha256(f"pulse51-public-{label}-{ordinal}".encode("ascii")).hexdigest()


def _profile(ordinal: int, *, different: bool = False, unsupported: bool = False) -> bytes:
    value = {
        "schema": "ferris.profile-evidence/v1" if unsupported else "ferris.profile-evidence/v0",
        "profile_id": f"synthetic-{ordinal}",
        "revision": "two" if different else "one",
        "consumer": "public-fixture",
        "sections": {
            name: {"value": ordinal + 1 if different and name == "identity" else ordinal}
            for name in SECTIONS
        },
    }
    return json.dumps(value, ensure_ascii=True, indent=2, sort_keys=True).encode("utf-8") + b"\n"


def _request(target: str) -> dict[str, object]:
    return {
        "spelling": target,
        "platform_namespace": "output-relative-v1",
        "request_template": "{target}",
        "substitution_rule": "replace-target-placeholders-then-lexically-normalize-v1",
        "resolved_output_relative_target": target,
        "relative_resolution_base": "",
    }


def _role(
    root: Path,
    ordinal: int,
    side: str,
    state: str,
    content: bytes | None,
    expected_class: str,
    diagnostic: str | None,
) -> dict[str, object]:
    suffix = "bin" if state == "regular-file" else "missing" if state == "missing" else "directory"
    target = f"artifacts/{ordinal:03d}-{side}.{suffix}"
    path = root / target
    if state == "regular-file":
        assert content is not None
        path.write_bytes(content)
        size = len(content)
        digest = _digest(content)
    elif state == "directory":
        path.mkdir()
        size = None
        digest = None
    else:
        size = None
        digest = None
    return {
        "state": state,
        "target": target,
        "raw_size": size,
        "raw_sha256": digest,
        "request": _request(target),
        "expected_input": {"class": expected_class, "diagnostic": diagnostic},
    }


def _expected(result_class: str) -> dict[str, object]:
    return {"result_class": result_class, **RESULT_MAP[result_class]}


def _case(root: Path, ordinal: int) -> dict[str, object]:
    mode = ordinal % 5
    before = _profile(ordinal)
    result_class = "success"
    format_name = "human" if ordinal % 10 == 0 else "json"
    before_role = _role(root, ordinal, "before", "regular-file", before, "valid", None)
    after_role = _role(root, ordinal, "after", "regular-file", before, "valid", None)
    if mode == 1:
        result_class = "difference"
        format_name = "human" if ordinal % 2 else "json"
        after_role = _role(
            root,
            ordinal,
            "after",
            "regular-file",
            _profile(ordinal, different=True),
            "valid",
            None,
        )
    elif mode == 2:
        result_class = "invalid"
        format_name = "json"
        before_role = _role(
            root,
            ordinal,
            "before",
            "regular-file",
            b"{",
            "invalid",
            "FERRIS-PROFILE-JSON-INVALID",
        )
    elif mode == 3:
        result_class = "unsupported"
        format_name = "json"
        before_role = _role(
            root,
            ordinal,
            "before",
            "regular-file",
            _profile(ordinal, unsupported=True),
            "unsupported",
            "FERRIS-PROFILE-SCHEMA-UNSUPPORTED",
        )
    elif mode == 4:
        result_class = "incomplete"
        format_name = "json"
        (root / "artifacts" / f"{ordinal:03d}-before.bin").unlink()
        before_role = _role(
            root,
            ordinal,
            "before",
            "missing",
            None,
            "incomplete",
            "FERRIS-PROFILE-INPUT-UNAVAILABLE",
        )
    return {
        "ordinal": ordinal,
        "case_id": _token("case", ordinal),
        "order_token": _token("order", ordinal),
        "profile_token": _token("profile", ordinal),
        "execution": {
            "mode": "launch-ready",
            "format": format_name,
            "expected": _expected(result_class),
        },
        "before": before_role,
        "after": after_role,
        "semantic_witnesses": {"synthetic_public_fixture": True},
    }


def _aggregate(root: Path) -> str:
    aggregate = hashlib.sha256()
    with os.scandir(root / "artifacts") as directory:
        paths = sorted((Path(entry.path) for entry in directory), key=lambda item: item.name)
    for path in paths:
        if path.is_file():
            relative = path.relative_to(root).as_posix().encode("utf-8")
            aggregate.update(len(relative).to_bytes(8, "big"))
            aggregate.update(relative)
            aggregate.update(hashlib.sha256(path.read_bytes()).digest())
    return "sha256:" + aggregate.hexdigest()


def create_descriptor_root(root: Path) -> Path:
    if root.exists():
        raise FileExistsError(root)
    (root / "artifacts").mkdir(parents=True)
    cases = [_case(root, ordinal) for ordinal in range(1, 70)]
    cases.append(
        {
            "ordinal": 70,
            "case_id": _token("case", 70),
            "order_token": _token("order", 70),
            "profile_token": _token("profile", 70),
            "execution": {
                "mode": "no-launch",
                "format": "no-launch",
                "expected": _expected("blocked"),
            },
            "before": {
                "state": "not-materialized",
                "target": None,
                "raw_size": None,
                "raw_sha256": None,
                "request": None,
                "expected_input": {
                    "class": "incomplete",
                    "diagnostic": "FERRIS-PROFILE-INPUT-UNAVAILABLE",
                },
            },
            "after": {
                "state": "not-materialized",
                "target": None,
                "raw_size": None,
                "raw_sha256": None,
                "request": None,
                "expected_input": {
                    "class": "incomplete",
                    "diagnostic": "FERRIS-PROFILE-INPUT-UNAVAILABLE",
                },
            },
            "semantic_witnesses": {"synthetic_public_fixture": True},
            "external_prerequisite": "external-immutable-binary-freeze",
        }
    )
    manifest = {
        "schema": "ferris.pulse-35-corpus-case-manifest/v1",
        "derivation": "hmac-sha256-seed-key-domain-purpose-counter-v1",
        "seed_commitment_algorithm": "synthetic-public-fixture",
        "seed_commitment_sha256": "sha256:" + "0" * 64,
        "logical_case_max": 512,
        "required_case_count": 70,
        "case_count": 70,
        "artifact_aggregate_algorithm": "sha256-length-path-filedigest-v1",
        "artifact_aggregate": _aggregate(root),
        "cases": cases,
        "staging_directory_sync_records": [],
        "diagnostic_execution": False,
        "product_files_modified": False,
        "logical_retries": 0,
    }
    manifest_raw = _canonical_json(manifest)
    (root / "case-manifest.json").write_bytes(manifest_raw)
    coverage = {
        "schema": "ferris.pulse-35-corpus-coverage-manifest/v1",
        "authority_result_receipt": "sha256:" + "1" * 64,
        "case_manifest_sha256": _digest(manifest_raw),
        "case_count": 70,
        "coverage_domains_closed": "18/18",
        "coverage_interactions_closed": "8/8",
        "derived_catalog": {},
        "diagnostic_execution": False,
        "product_files_modified": False,
        "logical_retries": 0,
    }
    (root / "coverage-manifest.json").write_bytes(_canonical_json(coverage))
    return root


def _synthetic_receipt(expectation: BinaryExpectation) -> dict[str, object]:
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
        "cutoff": P33_CUTOFF,
        "platform": expectation.platform,
        "safety": {"diagnostic_execution": False, "product_files_modified": False},
        "schema": "ferris.public-build-freeze-receipt/v1",
    }
    return {
        "payload": payload,
        "payload_sha256": _digest(_canonical_json(payload)),
        "schema": "ferris.public-build-freeze-envelope/v1",
    }


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


def create_synthetic_custodies(
    root: Path, fake_source: Path
) -> tuple[dict[str, P44CustodyBinding], dict[str, BinaryExpectation]]:
    root.mkdir(parents=True)
    custodies: dict[str, P44CustodyBinding] = {}
    expectations: dict[str, BinaryExpectation] = {}
    raw = fake_source.read_bytes()
    digest = _digest(raw)
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
        filename = f"ferris-{platform}-{P33_CUTOFF}{suffix}"
        final_root = root / f"p44-final-{platform}"
        final_root.mkdir()
        executable = final_root / filename
        shutil.copyfile(fake_source, executable)
        initial = BinaryExpectation(
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
        receipt = _synthetic_receipt(initial)
        expectation = BinaryExpectation(
            **{
                **initial.__dict__,
                "published_receipt_payload_sha256": str(receipt["payload_sha256"]),
            }
        )
        receipt = _synthetic_receipt(expectation)
        (final_root / f"{filename}.receipt.json").write_bytes(_canonical_json(receipt))
        custodies[platform] = P44CustodyBinding(
            platform=platform,
            final_root=final_root,
            work_root=root / f"p44-work-{platform}",
            summary=synthetic_p44_summary(),
        )
        expectations[platform] = expectation
    return custodies, expectations
