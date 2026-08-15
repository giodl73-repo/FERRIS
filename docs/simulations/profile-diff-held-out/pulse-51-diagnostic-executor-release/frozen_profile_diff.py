"""Exact frozen public ``profile-diff`` semantics and identity primitives.

This is a pure, standard-library transcription of the public Rust contract.
It accepts only already-validated profile evidence and never reads a path,
launches a process, or handles private material.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass


PROFILE_SCHEMA = "ferris.profile-evidence/v0"
PROFILE_DIFF_SCHEMA = "ferris.profile-diff/v0"
COMMAND_RESULT_SCHEMA = "ferris.command-result/v2"
MAX_CHANGES = 10_000
SECTION_NAMES = (
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
_INVOCATION_PARTS = (
    "profile-diff",
    "profile-schema=ferris.profile-evidence/v0",
    "input-max-bytes=1048576",
    "change-max=10000",
    "owner-tools=false",
    "network=false",
    "mutation=false",
)
_BOUNDED_OUTPUT_FIELDS = (
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
)


@dataclass(frozen=True)
class ProfileDiffSemantics:
    """The independently derived result for one explicit input pair."""

    result_class: str
    selection_identity: str
    invocation_identity: str
    record: dict[str, object] | None


class ChangeBoundExceeded(RuntimeError):
    """The frozen 10,000-change profile-diff bound was exceeded."""


def _compact_json(value: object) -> bytes:
    """Serialize already ordered values like compact ``serde_json::to_vec``."""

    return json.dumps(
        value,
        allow_nan=False,
        ensure_ascii=False,
        separators=(",", ":"),
        sort_keys=False,
    ).encode("utf-8")


def _canonical_value(value: object) -> object:
    """Normalize a ``serde_json::Value`` object tree to BTreeMap key order."""

    if type(value) is dict:
        return {key: _canonical_value(value[key]) for key in sorted(value)}
    if type(value) is list:
        return [_canonical_value(item) for item in value]
    return value


def canonical_value_json(value: object) -> bytes:
    """Return compact Rust ``serde_json::Value`` bytes for a JSON value."""

    return _compact_json(_canonical_value(value))


def _digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def value_digest(value: object) -> str:
    """Return the frozen canonical JSON-value digest."""

    return _digest(canonical_value_json(value))


def canonical_profile_json(profile: dict[str, object]) -> bytes:
    """Return compact ``ProfileEvidence`` bytes in Rust declaration order."""

    sections = profile["sections"]
    assert type(sections) is dict
    return _compact_json(
        {
            "schema": profile["schema"],
            "profile_id": profile["profile_id"],
            "revision": profile["revision"],
            "consumer": profile["consumer"],
            "sections": {
                section: _canonical_value(sections[section]) for section in SECTION_NAMES
            },
        }
    )


def profile_content_digest(profile: dict[str, object]) -> str:
    """Return the frozen canonical ``ProfileEvidence`` content digest."""

    return _digest(canonical_profile_json(profile))


def _profile_reference(profile: dict[str, object]) -> dict[str, object]:
    return {
        "profile_id": profile["profile_id"],
        "revision": profile["revision"],
        "consumer": profile["consumer"],
        "content_digest": profile_content_digest(profile),
    }


def _reference_payload(reference: dict[str, object]) -> dict[str, object]:
    return {
        "profile_id": reference["profile_id"],
        "revision": reference["revision"],
        "consumer": reference["consumer"],
        "content_digest": reference["content_digest"],
    }


def _change_payload(change: dict[str, object]) -> dict[str, object]:
    return {
        "path": change["path"],
        "change_kind": change["change_kind"],
        "before_value_digest": change["before_value_digest"],
        "after_value_digest": change["after_value_digest"],
    }


def record_payload(
    record: dict[str, object], *, empty_diff_id: bool = False
) -> dict[str, object]:
    """Build the exact ``ProfileDiffRecord`` serialization payload."""

    return {
        "schema": record["schema"],
        "diff_id": "" if empty_diff_id else record["diff_id"],
        "before": _reference_payload(record["before"]),  # type: ignore[arg-type]
        "after": _reference_payload(record["after"]),  # type: ignore[arg-type]
        "changed_sections": list(record["changed_sections"]),  # type: ignore[arg-type]
        "changes": [
            _change_payload(change) for change in record["changes"]  # type: ignore[arg-type]
        ],
        "unchanged_sections": list(record["unchanged_sections"]),  # type: ignore[arg-type]
        "unknowns": list(record["unknowns"]),  # type: ignore[arg-type]
        "limitations": list(record["limitations"]),  # type: ignore[arg-type]
        "executable": record["executable"],
    }


def diff_identity(record: dict[str, object]) -> str:
    """Recompute the self-excluding frozen ``profile-diff`` identity."""

    return "profile-diff:" + hashlib.sha256(
        _compact_json(record_payload(record, empty_diff_id=True))
    ).hexdigest()


def _bounded_output_payload(value: dict[str, object]) -> dict[str, object]:
    return {field: value[field] for field in _BOUNDED_OUTPUT_FIELDS}


def _diagnostic_payload(value: dict[str, object]) -> dict[str, object]:
    payload = {
        "code": value["code"],
        "severity": value["severity"],
        "result_class": value["result_class"],
        "message": value["message"],
        "source_digest": value["source_digest"],
    }
    if "bounded_output" in value:
        bounded = value["bounded_output"]
        assert type(bounded) is dict
        payload["bounded_output"] = _bounded_output_payload(bounded)
    payload["next_actions"] = list(value["next_actions"])  # type: ignore[arg-type]
    return payload


def result_identity(envelope: dict[str, object]) -> str:
    """Recompute the self-excluding frozen command-result identity."""

    record = envelope["record"]
    assert record is None or type(record) is dict
    diagnostics = envelope["diagnostics"]
    assert type(diagnostics) is list
    payload = {
        "schema": envelope["schema"],
        "command_version": envelope["command_version"],
        "semantic_command_id": envelope["semantic_command_id"],
        "selection_identity": envelope["selection_identity"],
        "invocation_identity": envelope["invocation_identity"],
        "result_class": envelope["result_class"],
        "process_exit_code": envelope["process_exit_code"],
        "diagnostics": [
            _diagnostic_payload(diagnostic) for diagnostic in diagnostics  # type: ignore[arg-type]
        ],
        "record": None if record is None else record_payload(record),
    }
    return "result:" + hashlib.sha256(_compact_json(payload)).hexdigest()


def _framed_identity(prefix: str, parts: tuple[str, ...]) -> str:
    hasher = hashlib.sha256()
    for part in parts:
        hasher.update(part.encode("utf-8"))
        hasher.update(b"\0")
    return prefix + ":" + hasher.hexdigest()


def selection_identity_from_content(before_digest: str, after_digest: str) -> str:
    return _framed_identity(
        "selection", ("profile-diff-selection", before_digest, after_digest)
    )


def invocation_identity(selection_identity: str) -> str:
    return _framed_identity(
        "invocation",
        (
            _INVOCATION_PARTS[0],
            selection_identity,
            *_INVOCATION_PARTS[1:],
        ),
    )


def lexically_normalize_path(path: str) -> str:
    """Apply the frozen path algorithm without filesystem access."""

    value = path.removeprefix("\\\\?\\").replace("\\", "/")
    unc = value.startswith("//")
    rooted = unc or value.startswith("/")
    prefix = ""
    if unc:
        prefix, value = "//", value[2:]
    elif value.startswith("/"):
        prefix, value = "/", value[1:]
    elif len(value) >= 3 and value[1:3] == ":/":
        prefix, value, rooted = value[:3], value[3:], True

    parts: list[str] = []
    for component in value.split("/"):
        if not component or component == ".":
            continue
        if component == "..":
            if parts and parts[-1] != "..":
                parts.pop()
            elif not rooted:
                parts.append(component)
            continue
        parts.append(component)
    joined = "/".join(parts)
    if not prefix:
        return joined or "."
    if not joined:
        return prefix
    return prefix + joined if prefix.endswith("/") else prefix + "/" + joined


def request_digest(path: str) -> str:
    return _digest(lexically_normalize_path(path).encode("utf-8"))


def selection_identity_from_requests(before_path: str, after_path: str) -> str:
    material = (
        f"before-request={request_digest(before_path)};"
        f"after-request={request_digest(after_path)}"
    )
    return _framed_identity("selection", ("profile-diff-selection", material))


def selection_identity_from_second_input(
    before_digest: str, after_path: str
) -> str:
    material = f"before={before_digest};after-request={request_digest(after_path)}"
    return _framed_identity("selection", ("profile-diff-selection", material))


def _json_equal(left: object, right: object) -> bool:
    """Match ``serde_json::Value`` structural equality, including number kind."""

    if type(left) is not type(right):
        return False
    if type(left) is dict:
        return (
            set(left) == set(right)
            and all(_json_equal(left[key], right[key]) for key in left)
        )
    if type(left) is list:
        return len(left) == len(right) and all(
            _json_equal(before, after) for before, after in zip(left, right, strict=True)
        )
    return left == right


def _append_change(
    changes: list[dict[str, object]],
    path: str,
    change_kind: str,
    before: object | None,
    after: object | None,
) -> None:
    if len(changes) >= MAX_CHANGES:
        raise ChangeBoundExceeded()
    changes.append(
        {
            "path": path,
            "change_kind": change_kind,
            "before_value_digest": None if before is None else value_digest(before),
            "after_value_digest": None if after is None else value_digest(after),
        }
    )


def _escape_pointer_token(token: str) -> str:
    return token.replace("~", "~0").replace("/", "~1")


def _add_subtree(
    value: object, path: str, change_kind: str, changes: list[dict[str, object]]
) -> None:
    if type(value) is dict and value:
        for key in sorted(value):
            _add_subtree(
                value[key],
                path + "/" + _escape_pointer_token(key),
                change_kind,
                changes,
            )
        return
    if type(value) is list and value:
        for index, item in enumerate(value):
            _add_subtree(item, f"{path}/{index}", change_kind, changes)
        return
    if change_kind == "added":
        _append_change(changes, path, change_kind, None, value)
    else:
        _append_change(changes, path, change_kind, value, None)


def _diff_value(
    before: object, after: object, path: str, changes: list[dict[str, object]]
) -> None:
    if _json_equal(before, after):
        return
    if type(before) is dict and type(after) is dict:
        keys = sorted(set(before) | set(after))
        if not keys:
            _append_change(changes, path, "changed", before, after)
            return
        for key in keys:
            child_path = path + "/" + _escape_pointer_token(key)
            if key in before and key in after:
                _diff_value(before[key], after[key], child_path, changes)
            elif key in before:
                _add_subtree(before[key], child_path, "removed", changes)
            else:
                _add_subtree(after[key], child_path, "added", changes)
        return
    if type(before) is list and type(after) is list:
        if not before and not after:
            _append_change(changes, path, "changed", before, after)
            return
        for index in range(max(len(before), len(after))):
            child_path = f"{path}/{index}"
            if index < len(before) and index < len(after):
                _diff_value(before[index], after[index], child_path, changes)
            elif index < len(before):
                _add_subtree(before[index], child_path, "removed", changes)
            else:
                _add_subtree(after[index], child_path, "added", changes)
        return
    _append_change(changes, path, "changed", before, after)


def profile_diff_record(
    before: dict[str, object], after: dict[str, object]
) -> dict[str, object]:
    """Build the complete ordered public record for matching accepted inputs."""

    changes: list[dict[str, object]] = []
    if before["revision"] != after["revision"]:
        _append_change(changes, "/revision", "changed", before["revision"], after["revision"])

    before_sections = before["sections"]
    after_sections = after["sections"]
    assert type(before_sections) is dict and type(after_sections) is dict
    changed_sections: list[str] = []
    unchanged_sections: list[str] = []
    for section in SECTION_NAMES:
        before_value = before_sections[section]
        after_value = after_sections[section]
        if _json_equal(before_value, after_value):
            unchanged_sections.append(section)
        else:
            changed_sections.append(section)
            _diff_value(
                before_value,
                after_value,
                "/sections/" + _escape_pointer_token(section),
                changes,
            )
    changed_sections.sort()
    unchanged_sections.sort()
    changes.sort(key=lambda change: str(change["path"]))
    record = {
        "schema": PROFILE_DIFF_SCHEMA,
        "diff_id": "",
        "before": _profile_reference(before),
        "after": _profile_reference(after),
        "changed_sections": changed_sections,
        "changes": changes,
        "unchanged_sections": unchanged_sections,
        "unknowns": list(UNKNOWN_LINES),
        "limitations": list(LIMITATION_LINES),
        "executable": False,
    }
    record["diff_id"] = diff_identity(record)
    return record


def derive_profile_diff(
    before_path: str,
    before_outcome: str,
    before: dict[str, object] | None,
    after_path: str,
    after_outcome: str,
    after: dict[str, object] | None,
) -> ProfileDiffSemantics:
    """Derive result class, identities, and record from explicit input state."""

    if before_outcome != "accepted":
        selection = selection_identity_from_requests(before_path, after_path)
        return ProfileDiffSemantics(
            before_outcome, selection, invocation_identity(selection), None
        )
    assert before is not None
    before_digest = profile_content_digest(before)
    if after_outcome != "accepted":
        selection = selection_identity_from_second_input(before_digest, after_path)
        return ProfileDiffSemantics(
            after_outcome, selection, invocation_identity(selection), None
        )
    assert after is not None
    after_digest = profile_content_digest(after)
    selection = selection_identity_from_content(before_digest, after_digest)
    invocation = invocation_identity(selection)
    if before["profile_id"] != after["profile_id"] or before["consumer"] != after["consumer"]:
        return ProfileDiffSemantics("invalid", selection, invocation, None)
    try:
        record = profile_diff_record(before, after)
    except ChangeBoundExceeded:
        return ProfileDiffSemantics("blocked", selection, invocation, None)
    return ProfileDiffSemantics(
        "success" if not record["changes"] else "difference",
        selection,
        invocation,
        record,
    )
