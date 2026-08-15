"""Public synthetic fake; it is not a FERRIS binary and has no private inputs."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
from pathlib import Path


sys.dont_write_bytecode = True
RELEASE_ROOT = Path(__file__).resolve().parents[1]
if str(RELEASE_ROOT) not in sys.path:
    sys.path.insert(0, str(RELEASE_ROOT))

import frozen_profile_diff as frozen_profile_diff
from p31_contract_verifier import Outcome, parse_accepted_profile, validate_bytes


def _profile(path: Path) -> tuple[Outcome, dict[str, object] | None, bytes | None]:
    if not path.exists():
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE"), None, None
    if not path.is_file():
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-NOT-FILE"), None, None
    try:
        raw = path.read_bytes()
    except OSError:
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE"), None, None
    outcome = validate_bytes(raw)
    if not outcome.accepted:
        return outcome, None, raw
    return outcome, parse_accepted_profile(raw), raw


def _diagnostic(
    result_class: str,
    before: tuple[Outcome, dict[str, object] | None, bytes | None],
    after: tuple[Outcome, dict[str, object] | None, bytes | None],
) -> dict[str, object]:
    before_outcome, before_profile, before_raw = before
    after_outcome, after_profile, after_raw = after
    raw = before_raw if before_raw is not None else after_raw
    values = {
        "FERRIS-PROFILE-JSON-INVALID": (
            "An explicit profile evidence input is not valid JSON.",
            "Provide a complete JSON profile evidence fixture.",
        ),
        "FERRIS-PROFILE-SCHEMA-UNSUPPORTED": (
            "An explicit profile evidence input uses an unsupported schema.",
            "Use schema ferris.profile-evidence/v0.",
        ),
        "FERRIS-PROFILE-INPUT-UNAVAILABLE": (
            "An explicit profile evidence input is missing or unreadable.",
            "Pass two readable local files with --before and --after.",
        ),
        "FERRIS-PROFILE-INPUT-NOT-FILE": (
            "An explicit profile evidence input is not a regular file.",
            "Pass two readable local files with --before and --after.",
        ),
        "FERRIS-PROFILE-INPUT-OVERSIZED": (
            "An explicit profile evidence input exceeds the 1048576-byte bound.",
            "Reduce the explicit input below the documented bound.",
        ),
        "FERRIS-PROFILE-DIFF-PROFILE-ID-MISMATCH": (
            "The two profile evidence files declare different profile identities.",
            "Compare revisions of the same explicit profile identity.",
        ),
        "FERRIS-PROFILE-DIFF-CONSUMER-MISMATCH": (
            "The two profile evidence files declare different consumers.",
            "Compare evidence for the same explicit consumer.",
        ),
        "FERRIS-PROFILE-DIFF-BOUND-EXCEEDED": (
            "The profile diff exceeds the 10000-change bound.",
            "Use a more narrowly scoped explicit evidence fixture or owner-native comparison tools.",
        ),
    }
    code = before_outcome.diagnostic
    if before_outcome.accepted:
        code = after_outcome.diagnostic
    if code is None and before_profile is not None and after_profile is not None:
        if before_profile["profile_id"] != after_profile["profile_id"]:
            code = "FERRIS-PROFILE-DIFF-PROFILE-ID-MISMATCH"
        elif before_profile["consumer"] != after_profile["consumer"]:
            code = "FERRIS-PROFILE-DIFF-CONSUMER-MISMATCH"
        elif result_class == "blocked":
            code = "FERRIS-PROFILE-DIFF-BOUND-EXCEEDED"
    if code not in values:
        code = "FERRIS-PROFILE-JSON-INVALID"
    message, action = values[code]
    return {
        "code": code,
        "severity": "error",
        "result_class": result_class,
        "message": message,
        "source_digest": None
        if raw is None
        else "sha256:" + hashlib.sha256(raw).hexdigest(),
        "next_actions": [action],
    }


def envelope(before_path: Path, after_path: Path, variant: bool) -> dict[str, object]:
    before = _profile(before_path)
    after = _profile(after_path)
    semantics = frozen_profile_diff.derive_profile_diff(
        str(before_path),
        before[0].result_class,
        before[1],
        str(after_path),
        after[0].result_class,
        after[1],
    )
    exit_code = {
        "success": 0,
        "difference": 1,
        "invalid": 2,
        "unsupported": 4,
        "incomplete": 5,
        "blocked": 7,
    }[semantics.result_class]
    payload = {
        "schema": "ferris.command-result/v2",
        "command_version": "0.1.0",
        "semantic_command_id": "profile-diff",
        "selection_identity": semantics.selection_identity,
        "invocation_identity": semantics.invocation_identity,
        "result_identity": "",
        "result_class": semantics.result_class,
        "process_exit_code": exit_code,
        "diagnostics": []
        if semantics.record is not None
        else [_diagnostic(semantics.result_class, before, after)],
        "record": semantics.record,
    }
    payload["result_identity"] = frozen_profile_diff.result_identity(payload)
    if variant:
        record = payload["record"]
        assert type(record) is dict
        record["diff_id"] = "profile-diff:" + "f" * 64
        payload["result_identity"] = frozen_profile_diff.result_identity(payload)
    return payload


def human(record_value: dict[str, object], result_class: str) -> bytes:
    before = record_value["before"]
    after = record_value["after"]
    assert type(before) is dict and type(after) is dict
    lines = [
        f"Ferris profile diff {record_value['diff_id']}",
        f"Schema: {record_value['schema']}",
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
    changed_sections = record_value["changed_sections"]
    assert type(changed_sections) is list
    lines.extend(f"  - {value}" for value in changed_sections or ["none"])
    lines.append("Changes:")
    changes = record_value["changes"]
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
    unchanged_sections = record_value["unchanged_sections"]
    assert type(unchanged_sections) is list
    lines.extend(f"  - {value}" for value in unchanged_sections or ["none"])
    lines.append("Unknowns:")
    lines.extend(f"  - {value}" for value in record_value["unknowns"])  # type: ignore[arg-type]
    lines.append("Limitations:")
    lines.extend(f"  - {value}" for value in record_value["limitations"])  # type: ignore[arg-type]
    return ("\n".join(lines) + "\n").encode("utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--synthetic-platform", required=True)
    parser.add_argument("command")
    parser.add_argument("--before", required=True)
    parser.add_argument("--after", required=True)
    parser.add_argument("--format", required=True, choices=("json", "human"))
    arguments = parser.parse_args()
    if arguments.command != "profile-diff":
        return 11
    before = Path(arguments.before)
    after = Path(arguments.after)
    mismatch = os.environ.get("P51_SYNTHETIC_MISMATCH_PLATFORM")
    mismatch_ordinal = os.environ.get("P51_SYNTHETIC_MISMATCH_ORDINAL")
    variant = (
        mismatch == arguments.synthetic_platform
        and bool(mismatch_ordinal)
        and mismatch_ordinal in arguments.before
    )
    payload = envelope(before, after, variant)
    if arguments.format == "human":
        if payload["result_class"] not in {"success", "difference"}:
            return 11
        record = payload["record"]
        assert type(record) is dict
        sys.stdout.buffer.write(human(record, str(payload["result_class"])))
    else:
        output = json.dumps(payload, ensure_ascii=True, indent=2) + "\n"
        stream = (
            sys.stdout.buffer
            if payload["result_class"] in {"success", "difference"}
            else sys.stderr.buffer
        )
        stream.write(output.encode("ascii"))
    return int(payload["process_exit_code"])


if __name__ == "__main__":
    raise SystemExit(main())
