from __future__ import annotations

import argparse
import hashlib
import hmac
import json
import os
import re
import stat
import sys
from pathlib import Path
from typing import Any


MAX_LOGICAL_CASES = 512
REQUIRED_CASE_COUNT = 70
MAX_PROFILE_BYTES = 1_048_576
PROFILE_SCHEMA = "ferris.profile-evidence/v0"
PULSE_34_RESULT_RECEIPT = "sha256:dca0ad1579257a6f265ada501533a4034070963267ef7c25478bf38267ee1588"
SEED_COMMITMENT_DOMAIN = b"ferris-p35-seed-commitment-v1\0"
DERIVATION_DOMAIN = b"ferris-p35-corpus-hmac-v1\0"
DERIVATION = "hmac-sha256-seed-key-domain-purpose-counter-v1"
VISIBLE_ASCII = re.compile(r"^[!-~]{1,256}$")
NUMBER_TOKEN = re.compile(r"(?<![0-9A-Za-z_.+-])-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?(?![0-9A-Za-z_.+-])")
SECTION_NAMES = ("identity", "closure", "features", "toolchain", "targets", "providers", "native", "stages", "assurance", "stewardship", "support", "lifecycle")
PATH_FORMS = ("relative-simple", "relative-dot", "relative-reducible-dotdot", "relative-unreducible-dotdot", "windows-drive-absolute", "windows-extended-absolute", "windows-unc", "unix-absolute", "mixed-separators")
METADATA_SITES = ("profile_id", "revision", "consumer", "object-key")
METADATA_BOUNDARIES = (0, 1, 255, 256, 257)
METADATA_KINDS = ("visible-ascii", "ascii-control", "non-ascii")
JSON_VALUE_KINDS = ("null", "false", "true", "string", "number", "array-empty", "array-nonempty", "object-empty", "object-nonempty", "nested-array", "nested-object")
DUPLICATE_DEPTHS = (0, 1, 2, 8, 32)
FAILURE_POSITIONS = ("before-only", "after-only-after-valid-before", "both-before-precedence", "relocated-valid-before-same-after-failure")
MEMBER_ORDERINGS = ("before-reordered", "after-reordered", "both-reordered-equivalent")
CHANGE_COUNT_ALGORITHM = "recursive-json-leaf-difference-v1"
RESULT_MAP = {
    "success": {"exit": 0, "stream": "stdout-only", "record": "non-null", "diagnostics": "empty"},
    "difference": {"exit": 1, "stream": "stdout-only", "record": "non-null", "diagnostics": "empty"},
    "invalid": {"exit": 2, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
    "unsupported": {"exit": 4, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
    "incomplete": {"exit": 5, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
    "blocked": {"exit": 7, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
}
INTERACTIONS = (
    "metadata-site-by-metadata-byte-boundary",
    "metadata-site-by-character-kind-for-nonempty-values",
    "input-position-by-path-state-by-path-form",
    "input-position-by-input-byte-boundary",
    "json-value-kind-by-member-ordering",
    "duplicate-depth-by-failure-position",
    "expected-result-class-by-exact-json-route",
    "success-difference-by-json-human-format-pair",
)
DOMAINS: tuple[tuple[str, tuple[Any, ...]], ...] = (
    ("result_classes", tuple({"class": name, "exit": value["exit"], "json_route": value["stream"], "record": value["record"], "diagnostics": value["diagnostics"]} for name, value in RESULT_MAP.items())),
    ("human_parity_classes", ("success", "difference")),
    ("metadata_sites", METADATA_SITES),
    ("metadata_byte_boundaries", METADATA_BOUNDARIES),
    ("metadata_character_kinds", METADATA_KINDS),
    ("json_value_kinds", JSON_VALUE_KINDS),
    ("number_representations", ("0", "-0", "1", "-1", "1.0", "1e0", "1E+0", "1e-0", "9007199254740991", "-9007199254740991")),
    ("pointer_key_kinds", ("slash", "tilde", "slash-and-tilde", "nested")),
    ("duplicate_depths", DUPLICATE_DEPTHS),
    ("member_orderings", MEMBER_ORDERINGS),
    ("input_role_orderings", ("before-then-after-read", "difference-pair-original-roles", "difference-pair-swapped-roles")),
    ("failure_positions", FAILURE_POSITIONS),
    ("path_states", ("missing", "non-file", "regular-file")),
    ("path_forms", PATH_FORMS),
    ("lexical_normalization", ("extended-prefix-strip", "backslash-to-slash", "unc-authority-preserve", "empty-component-removal", "dot-component-removal", "reducible-dotdot-pop", "rooted-dotdot-discard", "relative-dotdot-preserve", "repeated-separator-collapse", "drive-case-preserve", "drive-rooted-versus-relative")),
    ("input_byte_boundaries", (1_048_575, 1_048_576, 1_048_577)),
    ("change_count_boundaries", (9_999, 10_000, 10_001)),
    ("interaction_requirements", INTERACTIONS),
)


class MaterializationError(ValueError):
    pass


class ObjectPairs(list[tuple[str, Any]]):
    pass


def canonical_json(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n"


def sha256(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def _derive(seed: bytes, purpose: str, counter: int) -> bytes:
    return hmac.new(seed, DERIVATION_DOMAIN + purpose.encode("ascii") + b"\0" + counter.to_bytes(8, "big"), hashlib.sha256).digest()


def _read_seed(seed_path: Path) -> bytes:
    try:
        if not stat.S_ISREG(seed_path.stat().st_mode):
            raise MaterializationError("seed path is not a regular file")
        value = seed_path.read_bytes()
    except MaterializationError:
        raise
    except OSError as error:
        raise MaterializationError("seed file is unavailable or unreadable") from error
    if len(value) != 32:
        raise MaterializationError("seed file must contain exactly 32 bytes of CSPRNG material")
    return value


def _pairs(value: list[tuple[str, Any]]) -> ObjectPairs:
    return ObjectPairs(value)


def _load(value: bytes) -> Any:
    return json.loads(value.decode("utf-8"), object_pairs_hook=_pairs, parse_constant=lambda item: (_ for _ in ()).throw(ValueError(item)))


def _plain(value: Any) -> Any:
    if isinstance(value, ObjectPairs):
        return {key: _plain(child) for key, child in value}
    return [_plain(child) for child in value] if isinstance(value, list) else value


def _visible(value: Any) -> bool:
    return isinstance(value, str) and VISIBLE_ASCII.fullmatch(value) is not None


def _character_kind(value: str) -> str:
    return "visible-ascii" if _visible(value) else "non-ascii" if any(ord(item) > 0x7E for item in value) else "ascii-control"


def _inspect(value: Any) -> dict[str, Any]:
    duplicates: list[int] = []
    keys: list[str] = []
    kinds: set[str] = set()
    pointer: set[str] = set()
    reordered = False
    def walk(item: Any, depth: int) -> None:
        nonlocal reordered
        if item is None:
            kinds.add("null")
        elif item is False:
            kinds.add("false")
        elif item is True:
            kinds.add("true")
        elif isinstance(item, str):
            kinds.add("string")
        elif isinstance(item, (int, float)):
            kinds.add("number")
        elif isinstance(item, ObjectPairs):
            kinds.add("object-empty" if not item else "object-nonempty")
            names = [key for key, _ in item]
            reordered = reordered or names != sorted(names)
            seen: set[str] = set()
            for key, child in item:
                keys.append(key)
                if key in seen:
                    duplicates.append(depth)
                seen.add(key)
                if "/" in key and "~" in key:
                    pointer.add("slash-and-tilde")
                elif "/" in key:
                    pointer.add("slash")
                elif "~" in key:
                    pointer.add("tilde")
                if isinstance(child, (ObjectPairs, list)):
                    pointer.add("nested")
                walk(child, depth + 1)
        elif isinstance(item, list):
            kinds.add("array-empty" if not item else "array-nonempty")
            if any(isinstance(child, list) for child in item):
                kinds.add("nested-array")
            if any(isinstance(child, ObjectPairs) for child in item):
                kinds.add("nested-object")
            for child in item:
                walk(child, depth)
    walk(value, 0)
    return {"duplicates": sorted(set(duplicates)), "keys": keys, "json_value_kinds": sorted(kinds), "pointer_key_kinds": sorted(pointer), "member_reordered": reordered}


def _masked(value: bytes) -> str:
    output: list[str] = []
    quoted = escaped = False
    for item in value.decode("utf-8"):
        if quoted:
            output.append(" ")
            if escaped:
                escaped = False
            elif item == "\\":
                escaped = True
            elif item == '"':
                quoted = False
        elif item == '"':
            quoted = True
            output.append(" ")
        else:
            output.append(item)
    return "".join(output)


def _classify(value: bytes) -> tuple[dict[str, Any], Any | None, dict[str, Any] | None]:
    if len(value) > MAX_PROFILE_BYTES:
        return ({"class": "incomplete", "diagnostic": "FERRIS-PROFILE-INPUT-OVERSIZED"}, None, None)
    try:
        parsed = _load(value)
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-JSON-INVALID"}, None, None)
    inspection = _inspect(parsed)
    if inspection["duplicates"]:
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER"}, parsed, inspection)
    if any(not _visible(key) for key in inspection["keys"]):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-METADATA-INVALID"}, parsed, inspection)
    if not isinstance(parsed, ObjectPairs):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-SHAPE-INVALID"}, parsed, inspection)
    root = dict(parsed)
    if isinstance(root.get("schema"), str) and root["schema"] != PROFILE_SCHEMA:
        return ({"class": "unsupported", "diagnostic": "FERRIS-PROFILE-SCHEMA-UNSUPPORTED"}, parsed, inspection)
    if set(root) != {"schema", "profile_id", "revision", "consumer", "sections"} or root.get("schema") != PROFILE_SCHEMA:
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-SHAPE-INVALID"}, parsed, inspection)
    if any(not _visible(root.get(name)) for name in ("profile_id", "revision", "consumer")):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-IDENTITY-INVALID"}, parsed, inspection)
    if not isinstance(root["sections"], ObjectPairs) or set(dict(root["sections"])) != set(SECTION_NAMES):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-SHAPE-INVALID"}, parsed, inspection)
    return ({"class": "valid", "diagnostic": None}, parsed, inspection)


def _path_form(value: str) -> str:
    if value.startswith("\\\\?\\"):
        return "windows-extended-absolute"
    if value.startswith("\\\\") or value.startswith("//"):
        return "windows-unc"
    if re.match(r"^[A-Za-z]:[\\/]", value):
        return "windows-drive-absolute"
    if value.startswith("/"):
        return "unix-absolute"
    if "\\" in value and "/" in value:
        return "mixed-separators"
    if value.startswith("./") or value.startswith(".\\"):
        return "relative-dot"
    if value.startswith("../") or value.startswith("..\\"):
        return "relative-unreducible-dotdot"
    if "/../" in value or "\\..\\" in value:
        return "relative-reducible-dotdot"
    return "relative-simple"


def normalize_lexical_path(value: str) -> tuple[str, list[str]]:
    steps: list[str] = []
    text = value
    if text.startswith("\\\\?\\"):
        text = text[4:]
        steps.append("extended-prefix-strip")
    if "\\" in text:
        text = text.replace("\\", "/")
        steps.append("backslash-to-slash")
    unc, drive, rooted = text.startswith("//"), "", text.startswith("/")
    if unc:
        authority = [part for part in text[2:].split("/") if part]
        if len(authority) < 2:
            raise MaterializationError("UNC request lacks server/share authority")
        prefix, text, rooted = f"//{authority[0]}/{authority[1]}", "/".join(authority[2:]), True
        steps.append("unc-authority-preserve")
    else:
        prefix = ""
        match = re.match(r"^([A-Za-z]:)(/)?", text)
        if match:
            drive, rooted, text = match.group(1), match.group(2) == "/", text[len(match.group(0)):]
            steps.extend(("drive-case-preserve", "drive-rooted-versus-relative"))
    if "//" in text:
        steps.append("repeated-separator-collapse")
    parts: list[str] = []
    for part in text.split("/"):
        if not part:
            if text:
                steps.append("empty-component-removal")
        elif part == ".":
            steps.append("dot-component-removal")
        elif part == "..":
            if parts and parts[-1] != "..":
                parts.pop()
                steps.append("reducible-dotdot-pop")
            elif rooted:
                steps.append("rooted-dotdot-discard")
            else:
                parts.append(part)
                steps.append("relative-dotdot-preserve")
        else:
            parts.append(part)
    if unc:
        return prefix + ("/" + "/".join(parts) if parts else ""), sorted(set(steps))
    prefix = drive + ("/" if drive and rooted else "")
    return prefix + "/".join(parts) if drive else ("/" if rooted else "") + "/".join(parts), sorted(set(steps))


def _resolve_request(request: dict[str, Any]) -> tuple[str, list[str]]:
    required = {"spelling", "platform_namespace", "request_template", "substitution_rule", "resolved_output_relative_target", "relative_resolution_base"}
    if set(request) != required or request["substitution_rule"] != "replace-target-placeholders-then-lexically-normalize-v1":
        raise MaterializationError("request resolution contract is not closed")
    target = request["resolved_output_relative_target"]
    if not isinstance(target, str) or not re.fullmatch(r"artifacts/[0-9]{3}-(before|after)\.(bin|missing|directory)", target):
        raise MaterializationError("request target is not output-relative")
    expected = request["request_template"].replace("{target}", target).replace("{target_suffix}", target.removeprefix("artifacts/"))
    normalized, steps = normalize_lexical_path(request["spelling"])
    template, _ = normalize_lexical_path(expected)
    form = _path_form(request["spelling"])
    relative = form in {"relative-simple", "relative-dot", "relative-reducible-dotdot", "relative-unreducible-dotdot", "mixed-separators"}
    if relative:
        base = request["relative_resolution_base"]
        if not isinstance(base, str) or request["platform_namespace"] not in {"output-relative-v1", "relative-child-custody-root-v1"}:
            raise MaterializationError("relative request namespace is invalid")
        resolved, _ = normalize_lexical_path((base + "/" if base else "") + normalized)
        if normalized != template or resolved != target:
            raise MaterializationError("relative request does not resolve to its declared target")
    elif request["relative_resolution_base"] is not None or normalized != template:
        raise MaterializationError("absolute request does not resolve to its custody-root target")
    return normalized, steps


def _role_semantics(role: dict[str, Any], raw: bytes | None) -> dict[str, Any]:
    if role["state"] == "not-materialized":
        return {"input_class": "incomplete", "diagnostic": "FERRIS-PROFILE-INPUT-UNAVAILABLE", "metadata": [], "json_value_kinds": [], "number_representations": [], "pointer_key_kinds": [], "duplicate_depths": [], "member_reordered": False, "input_byte_boundary": None, "path_form": None, "path_normalized": None, "path_transformations": []}
    normalized, transforms = _resolve_request(role["request"])
    common = {"path_form": _path_form(role["request"]["spelling"]), "path_normalized": normalized, "path_transformations": transforms}
    if role["state"] != "regular-file":
        return {**common, "input_class": "incomplete", "diagnostic": "FERRIS-PROFILE-INPUT-NOT-FILE" if role["state"] == "directory" else "FERRIS-PROFILE-INPUT-UNAVAILABLE", "metadata": [], "json_value_kinds": [], "number_representations": [], "pointer_key_kinds": [], "duplicate_depths": [], "member_reordered": False, "input_byte_boundary": None}
    assert raw is not None
    observed, parsed, inspection = _classify(raw)
    common.update({"input_class": observed["class"], "diagnostic": observed["diagnostic"], "input_byte_boundary": len(raw) if len(raw) in {1_048_575, 1_048_576, 1_048_577} else None})
    if parsed is None or inspection is None:
        return {**common, "metadata": [], "json_value_kinds": [], "number_representations": [], "pointer_key_kinds": [], "duplicate_depths": [], "member_reordered": False}
    metadata: list[dict[str, Any]] = []
    if isinstance(parsed, ObjectPairs):
        root = dict(parsed)
        for site in ("profile_id", "revision", "consumer"):
            if isinstance(root.get(site), str):
                metadata.append({"site": site, "byte_length": len(root[site].encode("utf-8")), "character_kind": _character_kind(root[site])})
    reserved = set(SECTION_NAMES) | {"schema", "profile_id", "revision", "consumer", "sections"}
    metadata.extend({"site": "object-key", "byte_length": len(key.encode("utf-8")), "character_kind": _character_kind(key)} for key in inspection["keys"] if key not in reserved)
    return {**common, "metadata": sorted(metadata, key=canonical_json), "json_value_kinds": inspection["json_value_kinds"], "number_representations": sorted(set(match.group(0) for match in NUMBER_TOKEN.finditer(_masked(raw)))), "pointer_key_kinds": inspection["pointer_key_kinds"], "duplicate_depths": inspection["duplicates"], "member_reordered": inspection["member_reordered"]}


_MISSING = object()


def _change_count(left: Any, right: Any) -> int:
    if isinstance(left, ObjectPairs) and isinstance(right, ObjectPairs):
        first, second = dict(left), dict(right)
        return sum(_change_count(first.get(key, _MISSING), second.get(key, _MISSING)) for key in sorted(set(first) | set(second)))
    if isinstance(left, list) and isinstance(right, list):
        return sum(_change_count(left[index] if index < len(left) else _MISSING, right[index] if index < len(right) else _MISSING) for index in range(max(len(left), len(right))))
    return 0 if left == right else 1


def _expected(before: dict[str, Any], after: dict[str, Any], before_raw: bytes | None, after_raw: bytes | None, mode: str) -> dict[str, Any]:
    if mode == "no-launch":
        result = "blocked"
    elif before["input_class"] != "valid":
        result = before["input_class"]
    elif after["input_class"] != "valid":
        result = after["input_class"]
    else:
        assert before_raw is not None and after_raw is not None
        result = "success" if _plain(_load(before_raw)) == _plain(_load(after_raw)) else "difference"
    return {"result_class": result, **RESULT_MAP[result]}


def _witnesses(before: dict[str, Any], after: dict[str, Any], before_raw: bytes | None, after_raw: bytes | None, mode: str) -> dict[str, Any]:
    if before["input_class"] == after["input_class"] == "valid":
        assert before_raw is not None and after_raw is not None
        count = _change_count(_load(before_raw), _load(after_raw))
        boundary = count if count in {9_999, 10_000, 10_001} else None
    else:
        count = boundary = None
    return {"before": before, "after": after, "comparison": "no-launch" if mode == "no-launch" else _expected(before, after, before_raw, after_raw, mode)["result_class"], "pair_change_count": count, "pair_change_count_boundary": boundary, "pair_change_count_algorithm": CHANGE_COUNT_ALGORITHM}


def _member_orderings(witness: dict[str, Any]) -> set[str]:
    before, after = witness["before"], witness["after"]
    if before["member_reordered"] and after["member_reordered"] and witness["comparison"] == "success":
        return {"both-reordered-equivalent"}
    if before["member_reordered"] and not after["member_reordered"]:
        return {"before-reordered"}
    if after["member_reordered"] and not before["member_reordered"]:
        return {"after-reordered"}
    return set()


def _failure_positions(cases: list[dict[str, Any]]) -> dict[str, set[str]]:
    output = {case["case_id"]: set() for case in cases}
    grouped: dict[tuple[str, str, tuple[int, ...]], list[dict[str, Any]]] = {}
    for case in cases:
        before, after = case["semantic_witnesses"]["before"], case["semantic_witnesses"]["after"]
        if before["input_class"] != "valid" and after["input_class"] == "valid":
            output[case["case_id"]].add("before-only")
        elif before["input_class"] != "valid" and after["input_class"] != "valid":
            output[case["case_id"]].add("both-before-precedence")
        elif before["input_class"] == "valid" and after["input_class"] != "valid":
            grouped.setdefault((case["before"]["raw_sha256"], after["diagnostic"], tuple(after["duplicate_depths"])), []).append(case)
    for group in grouped.values():
        for index, case in enumerate(sorted(group, key=lambda item: item["ordinal"])):
            output[case["case_id"]].add("after-only-after-valid-before" if index == 0 else "relocated-valid-before-same-after-failure")
    return output


def _required_tuples() -> dict[str, list[dict[str, Any]]]:
    return {
        INTERACTIONS[0]: [{"metadata_site": site, "metadata_byte_boundary": boundary} for site in METADATA_SITES for boundary in METADATA_BOUNDARIES],
        INTERACTIONS[1]: [{"metadata_site": site, "character_kind": kind} for site in METADATA_SITES for kind in METADATA_KINDS],
        INTERACTIONS[2]: [{"input_position": position, "path_state": state, "path_form": form} for position in ("before", "after") for state in ("missing", "non-file", "regular-file") for form in PATH_FORMS],
        INTERACTIONS[3]: [{"input_position": position, "input_byte_boundary": boundary} for position in ("before", "after") for boundary in (1_048_575, 1_048_576, 1_048_577)],
        INTERACTIONS[4]: [{"json_value_kind": kind, "member_ordering": ordering} for kind in JSON_VALUE_KINDS for ordering in MEMBER_ORDERINGS],
        INTERACTIONS[5]: [{"duplicate_depth": depth, "failure_position": position} for depth in DUPLICATE_DEPTHS for position in FAILURE_POSITIONS],
        INTERACTIONS[6]: [{"expected_result_class": name, "json_route": result["stream"]} for name, result in RESULT_MAP.items()],
        INTERACTIONS[7]: [{"result_class": result, "format": output_format} for result in ("success", "difference") for output_format in ("json", "human")],
    }


def derive_interaction_catalog(cases: list[dict[str, Any]]) -> list[dict[str, Any]]:
    observed = {name: {} for name in INTERACTIONS}
    def add(name: str, value: dict[str, Any], identifier: str) -> None:
        observed[name].setdefault(canonical_json(value), []).append(identifier)
    failures = _failure_positions(cases)
    for case in cases:
        witness = case["semantic_witnesses"]
        for position in ("before", "after"):
            role, semantic = case[position], witness[position]
            for item in semantic["metadata"]:
                add(INTERACTIONS[0], {"metadata_site": item["site"], "metadata_byte_boundary": item["byte_length"]}, case["case_id"])
                if item["byte_length"] > 0:
                    add(INTERACTIONS[1], {"metadata_site": item["site"], "character_kind": item["character_kind"]}, case["case_id"])
            if role["state"] != "not-materialized":
                add(INTERACTIONS[2], {"input_position": position, "path_state": {"regular-file": "regular-file", "missing": "missing", "directory": "non-file"}[role["state"]], "path_form": semantic["path_form"]}, case["case_id"])
            if semantic["input_byte_boundary"] is not None:
                add(INTERACTIONS[3], {"input_position": position, "input_byte_boundary": semantic["input_byte_boundary"]}, case["case_id"])
            for kind in semantic["json_value_kinds"]:
                for ordering in _member_orderings(witness):
                    add(INTERACTIONS[4], {"json_value_kind": kind, "member_ordering": ordering}, case["case_id"])
            for depth in semantic["duplicate_depths"]:
                for failure in failures[case["case_id"]]:
                    add(INTERACTIONS[5], {"duplicate_depth": depth, "failure_position": failure}, case["case_id"])
        expected = case["execution"]["expected"]
        add(INTERACTIONS[6], {"expected_result_class": expected["result_class"], "json_route": expected["stream"]}, case["case_id"])
        if expected["result_class"] in {"success", "difference"} and case["execution"]["format"] in {"json", "human"}:
            add(INTERACTIONS[7], {"result_class": expected["result_class"], "format": case["execution"]["format"]}, case["case_id"])
    return [{"name": name, "tuples": [{"required_tuple": value, "case_ids": sorted(set(observed[name].get(canonical_json(value), [])))} for value in _required_tuples()[name]]} for name in INTERACTIONS]


def derive_domain_catalog(cases: list[dict[str, Any]]) -> list[dict[str, Any]]:
    values = {name: {canonical_json(value): [] for value in required} for name, required in DOMAINS}
    reverse_pairs: dict[tuple[str, str], list[dict[str, Any]]] = {}
    failures = _failure_positions(cases)
    for case in cases:
        witness, expected = case["semantic_witnesses"], case["execution"]["expected"]
        result = {"class": expected["result_class"], "exit": expected["exit"], "json_route": expected["stream"], "record": expected["record"], "diagnostics": expected["diagnostics"]}
        values["result_classes"][canonical_json(result)].append(case["case_id"])
        if expected["result_class"] in {"success", "difference"} and case["execution"]["format"] == "human":
            values["human_parity_classes"][canonical_json(expected["result_class"])].append(case["case_id"])
        if case["execution"]["mode"] == "launch-ready":
            values["input_role_orderings"][canonical_json("before-then-after-read")].append(case["case_id"])
        for position in ("before", "after"):
            role, semantic = case[position], witness[position]
            if role["state"] == "not-materialized":
                continue
            values["path_states"][canonical_json({"regular-file": "regular-file", "missing": "missing", "directory": "non-file"}[role["state"]])].append(case["case_id"])
            values["path_forms"][canonical_json(semantic["path_form"])].append(case["case_id"])
            for item in semantic["path_transformations"]:
                values["lexical_normalization"][canonical_json(item)].append(case["case_id"])
            for item in semantic["metadata"]:
                for name, value in (("metadata_sites", item["site"]), ("metadata_byte_boundaries", item["byte_length"]), ("metadata_character_kinds", item["character_kind"])):
                    if canonical_json(value) in values[name]:
                        values[name][canonical_json(value)].append(case["case_id"])
            for name in ("json_value_kinds", "number_representations", "pointer_key_kinds", "duplicate_depths"):
                for item in semantic[name]:
                    if canonical_json(item) in values[name]:
                        values[name][canonical_json(item)].append(case["case_id"])
            if semantic["input_byte_boundary"] is not None:
                values["input_byte_boundaries"][canonical_json(semantic["input_byte_boundary"])].append(case["case_id"])
        for item in _member_orderings(witness):
            values["member_orderings"][canonical_json(item)].append(case["case_id"])
        for item in failures[case["case_id"]]:
            values["failure_positions"][canonical_json(item)].append(case["case_id"])
        if witness["pair_change_count_boundary"] is not None:
            values["change_count_boundaries"][canonical_json(witness["pair_change_count_boundary"])].append(case["case_id"])
        if witness["before"]["input_class"] == witness["after"]["input_class"] == "valid":
            reverse_pairs.setdefault((case["before"]["raw_sha256"], case["after"]["raw_sha256"]), []).append(case)
    for pair, entries in reverse_pairs.items():
        if inverse := reverse_pairs.get((pair[1], pair[0])):
            for case in entries:
                item = "difference-pair-original-roles" if case["ordinal"] < min(value["ordinal"] for value in inverse) else "difference-pair-swapped-roles"
                values["input_role_orderings"][canonical_json(item)].append(case["case_id"])
    for interaction in derive_interaction_catalog(cases):
        for entry in interaction["tuples"]:
            values["interaction_requirements"][canonical_json(interaction["name"])].extend(entry["case_ids"])
    return [{"name": name, "required": list(required), "witness_case_ids": [sorted(set(values[name][canonical_json(item)])) for item in required]} for name, required in DOMAINS]


def _aggregate(files: list[tuple[str, bytes]]) -> str:
    digest = hashlib.sha256()
    for path, value in sorted(files):
        digest.update(str(len(value)).encode("ascii"))
        digest.update(b"\0")
        digest.update(path.encode("utf-8"))
        digest.update(b"\0")
        digest.update(hashlib.sha256(value).hexdigest().encode("ascii"))
        digest.update(b"\n")
    return "sha256:" + digest.hexdigest()


def _read_json(path: Path) -> tuple[bytes, dict[str, Any]]:
    try:
        raw, value = path.read_bytes(), json.loads(path.read_bytes())
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise MaterializationError(f"{path.name} is not complete UTF-8 JSON") from error
    if not isinstance(value, dict):
        raise MaterializationError(f"{path.name} root is not an object")
    return raw, value


def _validate_sync(records: Any) -> None:
    if not isinstance(records, list) or not records:
        raise MaterializationError("staging directory synchronization status is absent")
    mechanism = "win32-directory-flush" if os.name == "nt" else "posix-fsync-directory"
    for record in records:
        if set(record) != {"step", "state", "mechanism", "error_code"} or not isinstance(record["step"], str) or record["state"] not in {"synced", "unsupported"} or record["mechanism"] != mechanism or not (record["error_code"] is None or isinstance(record["error_code"], int)):
            raise MaterializationError("recorded directory synchronization status is invalid")


def verify(output: Path, seed_path: Path) -> dict[str, Any]:
    seed = _read_seed(seed_path)
    if not output.is_dir():
        raise MaterializationError("materialization output is unavailable")
    if any(".partial-" in item.name for item in output.rglob("*")):
        raise MaterializationError("materialization output contains atomic-write residue")
    manifest_bytes, manifest = _read_json(output / "case-manifest.json")
    _, coverage = _read_json(output / "coverage-manifest.json")
    manifest_keys = {"schema", "derivation", "seed_commitment_algorithm", "seed_commitment_sha256", "logical_case_max", "required_case_count", "case_count", "artifact_aggregate_algorithm", "artifact_aggregate", "cases", "staging_directory_sync_records", "diagnostic_execution", "product_files_modified", "logical_retries"}
    if set(manifest) != manifest_keys or manifest["schema"] != "ferris.pulse-35-corpus-case-manifest/v1" or manifest["derivation"] != DERIVATION:
        raise MaterializationError("case manifest has unknown, missing, or wrong schema members")
    if manifest["seed_commitment_algorithm"] != "sha256(ferris-p35-seed-commitment-v1\\0 || seed)" or manifest["seed_commitment_sha256"] != sha256(SEED_COMMITMENT_DOMAIN + seed):
        raise MaterializationError("private seed does not match the declared commitment")
    if manifest["logical_case_max"] != MAX_LOGICAL_CASES or manifest["required_case_count"] != REQUIRED_CASE_COUNT or manifest["case_count"] != REQUIRED_CASE_COUNT or manifest["artifact_aggregate_algorithm"] != "sha256-length-path-filedigest-v1":
        raise MaterializationError("case manifest does not require exact complete coverage")
    if manifest["diagnostic_execution"] is not False or manifest["product_files_modified"] is not False or manifest["logical_retries"] != 0:
        raise MaterializationError("case manifest widens the release authority")
    _validate_sync(manifest["staging_directory_sync_records"])
    cases = manifest["cases"]
    if not isinstance(cases, list) or len(cases) != REQUIRED_CASE_COUNT:
        raise MaterializationError("case descriptor cardinality mismatch")
    files, expected_files, expected_dirs, identifiers = [], {"case-manifest.json", "coverage-manifest.json"}, {"artifacts"}, set()
    for ordinal, case in enumerate(cases, start=1):
        expected_case_keys = {"ordinal", "case_id", "order_token", "profile_token", "execution", "before", "after", "semantic_witnesses"}
        if case.get("execution", {}).get("mode") == "no-launch":
            expected_case_keys.add("external_prerequisite")
        if set(case) != expected_case_keys or case.get("ordinal") != ordinal or case.get("case_id") != _derive(seed, "case-id", ordinal).hex() or case.get("order_token") != _derive(seed, "case-order-token", ordinal).hex() or case.get("profile_token") != _derive(seed, "profile-token", ordinal).hex() or case["case_id"] in identifiers:
            raise MaterializationError("seed-derived case identifier, order, or token mismatch")
        identifiers.add(case["case_id"])
        execution = case["execution"]
        if set(execution) != {"mode", "format", "expected"} or execution["mode"] not in {"launch-ready", "no-launch"} or (execution["mode"] == "no-launch" and (execution["format"] != "no-launch" or case.get("external_prerequisite") != "external-immutable-binary-freeze")) or (execution["mode"] == "launch-ready" and execution["format"] not in {"json", "human"}):
            raise MaterializationError("case execution descriptor is invalid")
        raws: dict[str, bytes | None] = {}
        semantics: dict[str, dict[str, Any]] = {}
        for name in ("before", "after"):
            role = case[name]
            role_keys = {"state", "target", "raw_size", "raw_sha256", "request", "expected_input"}
            if set(role) != role_keys or role["state"] not in {"regular-file", "missing", "directory", "not-materialized"}:
                raise MaterializationError("input role descriptor is not closed")
            if role["state"] == "not-materialized":
                if execution["mode"] != "no-launch" or any(role[key] is not None for key in ("target", "raw_size", "raw_sha256", "request")):
                    raise MaterializationError("no-launch role contains a request or materialized input")
                raw = None
            else:
                target = role["target"]
                expected_target = f"artifacts/{ordinal:03d}-{name}.{'bin' if role['state'] == 'regular-file' else 'directory' if role['state'] == 'directory' else 'missing'}"
                if target != expected_target or not isinstance(role["request"], dict) or role["request"].get("resolved_output_relative_target") != target:
                    raise MaterializationError("request spelling is not bound to its declared target")
                path = output / target
                if role["state"] == "regular-file":
                    if not path.is_file():
                        raise MaterializationError("regular-file input target is unavailable")
                    raw = path.read_bytes()
                    if role["raw_size"] != len(raw) or role["raw_sha256"] != sha256(raw):
                        raise MaterializationError("raw input size or digest mismatch")
                    expected_files.add(target)
                    files.append((target, raw))
                elif role["state"] == "directory":
                    if not path.is_dir() or role["raw_size"] is not None or role["raw_sha256"] is not None:
                        raise MaterializationError("directory input target mismatch")
                    expected_dirs.add(target)
                    raw = None
                else:
                    if path.exists() or role["raw_size"] is not None or role["raw_sha256"] is not None:
                        raise MaterializationError("missing input target mismatch")
                    raw = None
            semantic = _role_semantics(role, raw)
            if role["expected_input"] != {"class": semantic["input_class"], "diagnostic": semantic["diagnostic"]}:
                raise MaterializationError("expected public input classification is not derived from bytes and state")
            raws[name], semantics[name] = raw, semantic
        recomputed = _witnesses(semantics["before"], semantics["after"], raws["before"], raws["after"], execution["mode"])
        if case["semantic_witnesses"] != recomputed or execution["expected"] != _expected(semantics["before"], semantics["after"], raws["before"], raws["after"], execution["mode"]):
            raise MaterializationError("semantic witnesses, change count, or expected result are not independently derived")
    if manifest["artifact_aggregate"] != _aggregate(files):
        raise MaterializationError("artifact aggregate mismatch")
    actual_files = {path.relative_to(output).as_posix() for path in output.rglob("*") if path.is_file()}
    actual_dirs = {path.relative_to(output).as_posix() for path in output.rglob("*") if path.is_dir()}
    if actual_files != expected_files or actual_dirs != expected_dirs:
        raise MaterializationError("materialization output has missing or extra files or directories")
    coverage_keys = {"schema", "authority_result_receipt", "case_manifest_sha256", "case_count", "coverage_domains_closed", "coverage_interactions_closed", "derived_catalog", "diagnostic_execution", "product_files_modified", "logical_retries"}
    if set(coverage) != coverage_keys or coverage["schema"] != "ferris.pulse-35-corpus-coverage-manifest/v1" or coverage["authority_result_receipt"] != PULSE_34_RESULT_RECEIPT or coverage["case_manifest_sha256"] != sha256(manifest_bytes) or coverage["case_count"] != REQUIRED_CASE_COUNT:
        raise MaterializationError("coverage manifest identity mismatch")
    if coverage["coverage_domains_closed"] != "18/18" or coverage["coverage_interactions_closed"] != "8/8" or coverage["diagnostic_execution"] is not False or coverage["product_files_modified"] is not False or coverage["logical_retries"] != 0:
        raise MaterializationError("coverage manifest widens the release authority")
    catalog = coverage["derived_catalog"]
    if not isinstance(catalog, dict) or set(catalog) != {"domains", "interactions"} or not isinstance(catalog["domains"], list) or len(catalog["domains"]) != 18:
        raise MaterializationError("coverage domain catalog shape is invalid")
    if catalog != {"domains": derive_domain_catalog(cases), "interactions": derive_interaction_catalog(cases)}:
        raise MaterializationError("coverage catalog is not independently derived from bound artifacts and descriptors")
    if any(not tuple_entry["case_ids"] for interaction in catalog["interactions"] for tuple_entry in interaction["tuples"]):
        raise MaterializationError("exact interaction tuple closure is incomplete")
    return {"case_count": REQUIRED_CASE_COUNT, "coverage_domains_closed": "18/18", "coverage_interactions_closed": "8/8", "fresh_process_reload": True, "residue_count": 0, "logical_retries": 0, "directory_sync_records_validated": len(manifest["staging_directory_sync_records"])}


def main() -> int:
    parser = argparse.ArgumentParser(description="Independently verify a Pulse 35 materialization with its private seed.")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--seed-file", required=True, type=Path)
    arguments = parser.parse_args()
    try:
        result = verify(arguments.output, arguments.seed_file)
    except (MaterializationError, OSError) as error:
        print(f"verification rejected: {error}", file=sys.stderr)
        return 2
    print(canonical_json(result).decode("ascii"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
