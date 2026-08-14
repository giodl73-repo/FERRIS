from __future__ import annotations

import argparse
import ctypes
import errno
import hashlib
import hmac
import json
import os
import re
import shutil
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
SECTION_NAMES = (
    "identity", "closure", "features", "toolchain", "targets", "providers",
    "native", "stages", "assurance", "stewardship", "support", "lifecycle",
)
PATH_FORMS = (
    "relative-simple", "relative-dot", "relative-reducible-dotdot",
    "relative-unreducible-dotdot", "windows-drive-absolute",
    "windows-extended-absolute", "windows-unc", "unix-absolute",
    "mixed-separators",
)
PATH_STATES = ("regular-file", "missing", "directory")
METADATA_SITES = ("profile_id", "revision", "consumer", "object-key")
METADATA_BOUNDARIES = (0, 1, 255, 256, 257)
METADATA_KINDS = ("visible-ascii", "ascii-control", "non-ascii")
JSON_VALUE_KINDS = (
    "null", "false", "true", "string", "number", "array-empty",
    "array-nonempty", "object-empty", "object-nonempty", "nested-array",
    "nested-object",
)
DUPLICATE_DEPTHS = (0, 1, 2, 8, 32)
FAILURE_POSITIONS = (
    "before-only", "after-only-after-valid-before", "both-before-precedence",
    "relocated-valid-before-same-after-failure",
)
MEMBER_ORDERINGS = (
    "before-reordered", "after-reordered", "both-reordered-equivalent",
)
CHANGE_COUNT_ALGORITHM = "recursive-json-leaf-difference-v1"
RESULT_MAP = {
    "success": {"exit": 0, "stream": "stdout-only", "record": "non-null", "diagnostics": "empty"},
    "difference": {"exit": 1, "stream": "stdout-only", "record": "non-null", "diagnostics": "empty"},
    "invalid": {"exit": 2, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
    "unsupported": {"exit": 4, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
    "incomplete": {"exit": 5, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
    "blocked": {"exit": 7, "stream": "stderr-only", "record": "null", "diagnostics": "exactly-one-matching-class"},
}
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
    ("interaction_requirements", (
        "metadata-site-by-metadata-byte-boundary",
        "metadata-site-by-character-kind-for-nonempty-values",
        "input-position-by-path-state-by-path-form",
        "input-position-by-input-byte-boundary",
        "json-value-kind-by-member-ordering",
        "duplicate-depth-by-failure-position",
        "expected-result-class-by-exact-json-route",
        "success-difference-by-json-human-format-pair",
    )),
)
INTERACTIONS = tuple(value for value in DOMAINS[-1][1])


class MaterializationError(ValueError):
    pass


class PublicationIndeterminateError(MaterializationError):
    pass


class ObjectPairs(list[tuple[str, Any]]):
    pass


def canonical_json(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n"


def sha256(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def seed_commitment(seed: bytes) -> str:
    return sha256(SEED_COMMITMENT_DOMAIN + seed)


def derive(seed: bytes, purpose: str, counter: int) -> bytes:
    if len(seed) != 32 or counter < 0 or not purpose.isascii() or "\0" in purpose:
        raise MaterializationError("invalid HMAC derivation inputs")
    message = DERIVATION_DOMAIN + purpose.encode("ascii") + b"\0" + counter.to_bytes(8, "big")
    return hmac.new(seed, message, hashlib.sha256).digest()


def _pairs(pairs: list[tuple[str, Any]]) -> ObjectPairs:
    return ObjectPairs(pairs)


def _load_relaxed(value: bytes) -> Any:
    return json.loads(value.decode("utf-8"), object_pairs_hook=_pairs, parse_constant=lambda item: (_ for _ in ()).throw(ValueError(item)))


def _plain(value: Any) -> Any:
    if isinstance(value, ObjectPairs):
        return {key: _plain(child) for key, child in value}
    if isinstance(value, list):
        return [_plain(child) for child in value]
    return value


def _visible(value: Any) -> bool:
    return isinstance(value, str) and VISIBLE_ASCII.fullmatch(value) is not None


def _character_kind(value: str) -> str:
    if _visible(value):
        return "visible-ascii"
    return "non-ascii" if any(ord(character) > 0x7E for character in value) else "ascii-control"


def _inspect(value: Any) -> dict[str, Any]:
    duplicates: list[int] = []
    keys: list[str] = []
    kinds: set[str] = set()
    pointer_kinds: set[str] = set()
    reordered = False

    def walk(member: Any, depth: int) -> None:
        nonlocal reordered
        if member is None:
            kinds.add("null")
        elif member is False:
            kinds.add("false")
        elif member is True:
            kinds.add("true")
        elif isinstance(member, str):
            kinds.add("string")
        elif isinstance(member, (int, float)):
            kinds.add("number")
        elif isinstance(member, ObjectPairs):
            kinds.add("object-empty" if not member else "object-nonempty")
            names = [key for key, _ in member]
            reordered = reordered or names != sorted(names)
            seen: set[str] = set()
            for key, child in member:
                keys.append(key)
                if key in seen:
                    duplicates.append(depth)
                seen.add(key)
                if "/" in key and "~" in key:
                    pointer_kinds.add("slash-and-tilde")
                elif "/" in key:
                    pointer_kinds.add("slash")
                elif "~" in key:
                    pointer_kinds.add("tilde")
                if isinstance(child, (ObjectPairs, list)):
                    pointer_kinds.add("nested")
                walk(child, depth + 1)
        elif isinstance(member, list):
            kinds.add("array-empty" if not member else "array-nonempty")
            if any(isinstance(child, list) for child in member):
                kinds.add("nested-array")
            if any(isinstance(child, ObjectPairs) for child in member):
                kinds.add("nested-object")
            for child in member:
                walk(child, depth)

    walk(value, 0)
    return {
        "duplicates": sorted(set(duplicates)), "keys": keys,
        "json_value_kinds": sorted(kinds), "pointer_key_kinds": sorted(pointer_kinds),
        "member_reordered": reordered,
    }


def _masked_json(value: bytes) -> str:
    output: list[str] = []
    quoted = escaped = False
    for character in value.decode("utf-8"):
        if quoted:
            output.append(" ")
            if escaped:
                escaped = False
            elif character == "\\":
                escaped = True
            elif character == '"':
                quoted = False
        elif character == '"':
            quoted = True
            output.append(" ")
        else:
            output.append(character)
    return "".join(output)


def _classify_regular(value: bytes) -> tuple[dict[str, Any], Any | None, dict[str, Any] | None]:
    if len(value) > MAX_PROFILE_BYTES:
        return ({"class": "incomplete", "diagnostic": "FERRIS-PROFILE-INPUT-OVERSIZED"}, None, None)
    try:
        parsed = _load_relaxed(value)
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
    schema = root.get("schema")
    if isinstance(schema, str) and schema != PROFILE_SCHEMA:
        return ({"class": "unsupported", "diagnostic": "FERRIS-PROFILE-SCHEMA-UNSUPPORTED"}, parsed, inspection)
    if set(root) != {"schema", "profile_id", "revision", "consumer", "sections"} or schema != PROFILE_SCHEMA:
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-SHAPE-INVALID"}, parsed, inspection)
    if any(not _visible(root.get(name)) for name in ("profile_id", "revision", "consumer")):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-IDENTITY-INVALID"}, parsed, inspection)
    if not isinstance(root["sections"], ObjectPairs) or set(dict(root["sections"])) != set(SECTION_NAMES):
        return ({"class": "invalid", "diagnostic": "FERRIS-PROFILE-SHAPE-INVALID"}, parsed, inspection)
    return ({"class": "valid", "diagnostic": None}, parsed, inspection)


def _path_form(spelling: str) -> str:
    if spelling.startswith("\\\\?\\"):
        return "windows-extended-absolute"
    if spelling.startswith("\\\\") or spelling.startswith("//"):
        return "windows-unc"
    if re.match(r"^[A-Za-z]:[\\/]", spelling):
        return "windows-drive-absolute"
    if spelling.startswith("/"):
        return "unix-absolute"
    if "\\" in spelling and "/" in spelling:
        return "mixed-separators"
    if spelling.startswith("./") or spelling.startswith(".\\"):
        return "relative-dot"
    if spelling.startswith("../") or spelling.startswith("..\\"):
        return "relative-unreducible-dotdot"
    if "/../" in spelling or "\\..\\" in spelling:
        return "relative-reducible-dotdot"
    return "relative-simple"


def normalize_lexical_path(spelling: str) -> tuple[str, list[str]]:
    steps: list[str] = []
    text = spelling
    if text.startswith("\\\\?\\"):
        text = text[4:]
        steps.append("extended-prefix-strip")
    if "\\" in text:
        text = text.replace("\\", "/")
        steps.append("backslash-to-slash")
    unc = text.startswith("//")
    drive = ""
    rooted = text.startswith("/")
    if unc:
        authority = [part for part in text[2:].split("/") if part]
        if len(authority) < 2:
            raise MaterializationError("UNC request lacks server/share authority")
        prefix = f"//{authority[0]}/{authority[1]}"
        text = "/".join(authority[2:])
        rooted = True
        steps.append("unc-authority-preserve")
    else:
        prefix = ""
        match = re.match(r"^([A-Za-z]:)(/)?", text)
        if match:
            drive, rooted = match.group(1), match.group(2) == "/"
            text = text[len(match.group(0)):]
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
    if not drive and rooted:
        prefix = "/"
    return prefix + "/".join(parts), sorted(set(steps))


def _request(target: str, form: str) -> dict[str, Any]:
    suffix = target.removeprefix("artifacts/")
    if form == "relative-simple":
        spelling, namespace, template, base = target, "output-relative-v1", "{target}", ""
    elif form == "relative-dot":
        spelling, namespace, template, base = f"./artifacts//{suffix}", "output-relative-v1", "./artifacts//{target_suffix}", ""
    elif form == "relative-reducible-dotdot":
        spelling, namespace, template, base = f"artifacts/hold/../{suffix}", "output-relative-v1", "artifacts/hold/../{target_suffix}", ""
    elif form == "relative-unreducible-dotdot":
        spelling, namespace, template, base = f"../{target}", "relative-child-custody-root-v1", "../{target}", "request-child"
    elif form == "mixed-separators":
        spelling, namespace, template, base = f".\\artifacts/hold\\..\\{suffix}", "output-relative-v1", "./artifacts/hold/../{target_suffix}", ""
    elif form == "windows-drive-absolute":
        spelling, namespace, template, base = f"C:\\ferris-p35-custody\\artifacts\\hold\\..\\{suffix}", "windows-drive-custody-root-v1", "C:/ferris-p35-custody/artifacts/hold/../{target_suffix}", None
    elif form == "windows-extended-absolute":
        spelling, namespace, template, base = f"\\\\?\\C:\\ferris-p35-custody\\artifacts\\hold\\..\\{suffix}", "windows-extended-custody-root-v1", "C:/ferris-p35-custody/artifacts/hold/../{target_suffix}", None
    elif form == "windows-unc":
        spelling, namespace, template, base = f"\\\\ferris-p35-custody\\corpus\\artifacts\\hold\\..\\{suffix}", "windows-unc-custody-root-v1", "//ferris-p35-custody/corpus/artifacts/hold/../{target_suffix}", None
    elif form == "unix-absolute":
        spelling, namespace, template, base = f"/../ferris-p35-custody/artifacts/hold/../{suffix}", "unix-custody-root-v1", "/../ferris-p35-custody/artifacts/hold/../{target_suffix}", None
    else:
        raise MaterializationError("unknown path form")
    return {
        "spelling": spelling, "platform_namespace": namespace, "request_template": template,
        "substitution_rule": "replace-target-placeholders-then-lexically-normalize-v1",
        "resolved_output_relative_target": target, "relative_resolution_base": base,
    }


def resolve_request(request: dict[str, Any]) -> tuple[str, list[str]]:
    required = {"spelling", "platform_namespace", "request_template", "substitution_rule", "resolved_output_relative_target", "relative_resolution_base"}
    if set(request) != required or request["substitution_rule"] != "replace-target-placeholders-then-lexically-normalize-v1":
        raise MaterializationError("request resolution contract is not closed")
    target = request["resolved_output_relative_target"]
    if not isinstance(target, str) or not re.fullmatch(r"artifacts/[0-9]{3}-(before|after)\.(bin|missing|directory)", target):
        raise MaterializationError("request target is not output-relative")
    expected = request["request_template"].replace("{target}", target).replace("{target_suffix}", target.removeprefix("artifacts/"))
    normalized, steps = normalize_lexical_path(request["spelling"])
    template_normalized, _ = normalize_lexical_path(expected)
    form = _path_form(request["spelling"])
    relative = form in {"relative-simple", "relative-dot", "relative-reducible-dotdot", "relative-unreducible-dotdot", "mixed-separators"}
    if relative:
        base = request["relative_resolution_base"]
        if not isinstance(base, str) or request["platform_namespace"] not in {"output-relative-v1", "relative-child-custody-root-v1"}:
            raise MaterializationError("relative request namespace is invalid")
        resolved, _ = normalize_lexical_path((base + "/" if base else "") + normalized)
        if normalized != template_normalized or resolved != target:
            raise MaterializationError("relative request does not resolve to its declared target")
    elif request["relative_resolution_base"] is not None or normalized != template_normalized:
        raise MaterializationError("absolute request does not resolve to its custody-root target")
    return normalized, steps


def _role_semantics(role: dict[str, Any], raw: bytes | None) -> dict[str, Any]:
    if role["state"] == "not-materialized":
        return {"input_class": "incomplete", "diagnostic": "FERRIS-PROFILE-INPUT-UNAVAILABLE", "metadata": [], "json_value_kinds": [], "number_representations": [], "pointer_key_kinds": [], "duplicate_depths": [], "member_reordered": False, "input_byte_boundary": None, "path_form": None, "path_normalized": None, "path_transformations": []}
    normalized, transforms = resolve_request(role["request"])
    if role["state"] != "regular-file":
        diagnostic = "FERRIS-PROFILE-INPUT-NOT-FILE" if role["state"] == "directory" else "FERRIS-PROFILE-INPUT-UNAVAILABLE"
        return {"input_class": "incomplete", "diagnostic": diagnostic, "metadata": [], "json_value_kinds": [], "number_representations": [], "pointer_key_kinds": [], "duplicate_depths": [], "member_reordered": False, "input_byte_boundary": None, "path_form": _path_form(role["request"]["spelling"]), "path_normalized": normalized, "path_transformations": transforms}
    assert raw is not None
    observed, parsed, inspection = _classify_regular(raw)
    common = {"input_class": observed["class"], "diagnostic": observed["diagnostic"], "input_byte_boundary": len(raw) if len(raw) in {1_048_575, 1_048_576, 1_048_577} else None, "path_form": _path_form(role["request"]["spelling"]), "path_normalized": normalized, "path_transformations": transforms}
    if parsed is None or inspection is None:
        return {**common, "metadata": [], "json_value_kinds": [], "number_representations": [], "pointer_key_kinds": [], "duplicate_depths": [], "member_reordered": False}
    metadata: list[dict[str, Any]] = []
    if isinstance(parsed, ObjectPairs):
        root = dict(parsed)
        for site in ("profile_id", "revision", "consumer"):
            if isinstance(root.get(site), str):
                metadata.append({"site": site, "byte_length": len(root[site].encode("utf-8")), "character_kind": _character_kind(root[site])})
    reserved = set(SECTION_NAMES) | {"schema", "profile_id", "revision", "consumer", "sections"}
    for key in inspection["keys"]:
        if key not in reserved:
            metadata.append({"site": "object-key", "byte_length": len(key.encode("utf-8")), "character_kind": _character_kind(key)})
    return {**common, "metadata": sorted(metadata, key=canonical_json), "json_value_kinds": inspection["json_value_kinds"], "number_representations": sorted(set(match.group(0) for match in NUMBER_TOKEN.finditer(_masked_json(raw)))), "pointer_key_kinds": inspection["pointer_key_kinds"], "duplicate_depths": inspection["duplicates"], "member_reordered": inspection["member_reordered"]}


def _change_count(left: Any, right: Any) -> int:
    if isinstance(left, ObjectPairs) and isinstance(right, ObjectPairs):
        first, second = dict(left), dict(right)
        return sum(_change_count(first.get(key, _MISSING), second.get(key, _MISSING)) for key in sorted(set(first) | set(second)))
    if isinstance(left, list) and isinstance(right, list):
        return sum(_change_count(left[index] if index < len(left) else _MISSING, right[index] if index < len(right) else _MISSING) for index in range(max(len(left), len(right))))
    return 0 if left == right else 1


_MISSING = object()


def _expected(before: dict[str, Any], after: dict[str, Any], before_raw: bytes | None, after_raw: bytes | None, mode: str) -> dict[str, Any]:
    if mode == "no-launch":
        result = "blocked"
    elif before["input_class"] != "valid":
        result = before["input_class"]
    elif after["input_class"] != "valid":
        result = after["input_class"]
    else:
        assert before_raw is not None and after_raw is not None
        result = "success" if _plain(_load_relaxed(before_raw)) == _plain(_load_relaxed(after_raw)) else "difference"
    return {"result_class": result, **RESULT_MAP[result]}


def _semantic_witnesses(before: dict[str, Any], after: dict[str, Any], before_raw: bytes | None, after_raw: bytes | None, mode: str) -> dict[str, Any]:
    if before["input_class"] == after["input_class"] == "valid":
        assert before_raw is not None and after_raw is not None
        count = _change_count(_load_relaxed(before_raw), _load_relaxed(after_raw))
        boundary = count if count in {9_999, 10_000, 10_001} else None
    else:
        count = boundary = None
    return {
        "before": before, "after": after,
        "comparison": "no-launch" if mode == "no-launch" else _expected(before, after, before_raw, after_raw, mode)["result_class"],
        "pair_change_count": count, "pair_change_count_boundary": boundary,
        "pair_change_count_algorithm": CHANGE_COUNT_ALGORITHM,
    }


def _base_sections(marker: str = "base") -> dict[str, Any]:
    return {"identity": {"marker": marker}, "closure": {}, "features": [], "toolchain": {}, "targets": [], "providers": {}, "native": {}, "stages": [], "assurance": {}, "stewardship": {}, "support": "", "lifecycle": {}}


def _profile_bytes(profile_id: str, *, schema: str = PROFILE_SCHEMA, profile_id_value: str | None = None, revision: str = "r", consumer: str = "c", sections: dict[str, Any] | None = None, ordered: bool = False) -> bytes:
    value = {"schema": schema, "profile_id": profile_id if profile_id_value is None else profile_id_value, "revision": revision, "consumer": consumer, "sections": _base_sections() if sections is None else sections}
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=not ordered).encode("utf-8") + b"\n"


def _rich_profile(profile_id: str, *, alternate_root_order: bool = False) -> bytes:
    sections = ('{"identity":{"z":"last","a/b":{"a~b":{"a/b~c":[[null],{"nested":true}]}}},'
                '"closure":{},"features":[null,false,true,"","ordinary",0,-0,1,-1,1.0,1e0,1E+0,1e-0,9007199254740991,-9007199254740991,[],[false],{},{"nested":[true]}],'
                '"toolchain":{},"targets":[],"providers":{},"native":{},"stages":[],"assurance":{},"stewardship":{},"support":"","lifecycle":{}}')
    fields = {"schema": json.dumps(PROFILE_SCHEMA), "profile_id": json.dumps(profile_id), "revision": '"r"', "consumer": '"c"', "sections": sections}
    names = ("consumer", "sections", "revision", "profile_id", "schema") if alternate_root_order else ("schema", "profile_id", "revision", "consumer", "sections")
    return ("{" + ",".join(f"{json.dumps(name)}:{fields[name]}" for name in names) + "}\n").encode("utf-8")


def _sorted_rich_profile(profile_id: str) -> bytes:
    return json.dumps(json.loads(_rich_profile(profile_id)), separators=(",", ":"), sort_keys=True).encode("utf-8") + b"\n"


def _duplicate_profile(profile_id: str, depth: int) -> bytes:
    fields = _base_sections()
    if depth == 0:
        rest = _profile_bytes(profile_id).decode("utf-8").strip()[1:-1]
        return ('{"schema":"ferris.profile-evidence/v0","schema":"ferris.profile-evidence/v0",' + rest + "}\n").encode("utf-8")
    identity = '{"duplicate":0,"duplicate":1}'
    for _ in range(depth - 2):
        identity = '{"nest":' + identity + "}"
    parts = [f'"identity":{identity}' if name == "identity" else f"{json.dumps(name)}:{json.dumps(fields[name], separators=(',', ':'))}" for name in SECTION_NAMES]
    if depth == 1:
        parts[0:1] = ['"identity":{}', '"identity":{}']
    root = '"schema":"ferris.profile-evidence/v0","profile_id":' + json.dumps(profile_id) + ',"revision":"r","consumer":"c","sections":{' + ",".join(parts) + "}"
    return ("{" + root + "}\n").encode("utf-8")


def _metadata_profile(profile_id: str, site: str, value: str) -> bytes:
    if site == "object-key":
        sections = _base_sections()
        sections["identity"] = {value: "witness"}
        return _profile_bytes(profile_id, sections=sections)
    values: dict[str, Any] = {"profile_id_value": None, "revision": "r", "consumer": "c"}
    values["profile_id_value" if site == "profile_id" else site] = value
    return _profile_bytes(profile_id, **values)


def _padded_profile(profile_id: str, size: int) -> bytes:
    value = _profile_bytes(profile_id)
    if len(value) > size:
        raise MaterializationError("profile padding target is too small")
    return value + b" " * (size - len(value))


def _change_profile(profile_id: str, count: int, value: str) -> bytes:
    sections = _base_sections()
    sections["identity"] = {f"change-{index:05d}": value for index in range(count)}
    return _profile_bytes(profile_id, sections=sections)


def _role(state: str, raw: bytes | None = None, form: str = "relative-simple") -> dict[str, Any]:
    return {"state": state, "raw": raw, "form": form}


def _drafts(seed: bytes) -> list[dict[str, Any]]:
    drafts: list[dict[str, Any]] = []
    profile = lambda ordinal: "p35-" + derive(seed, "profile-token", ordinal).hex()
    ordering_profile = "p35-" + derive(seed, "member-ordering-profile-token", 0).hex()
    combinations = [(state, form) for state in PATH_STATES for form in PATH_FORMS]
    for index, (state, form) in enumerate(combinations, start=1):
        value = _profile_bytes(profile(index), sections=_base_sections(f"path-{index}"))
        before = _role(state, value if state == "regular-file" else None, form)
        after_value = value
        output_format = "json"
        if index == 1:
            after_value, output_format = _profile_bytes(profile(index), sections=_base_sections("path-difference")), "human"
        elif index == 2:
            output_format = "human"
        elif index == 3:
            after_value = _profile_bytes(profile(index), sections=_base_sections("path-difference-json"))
        elif index == 4:
            after_value = value
        elif index == 5:
            before, after_value = _role(state, _padded_profile(profile(index), 1_048_575), form), _padded_profile(profile(index), 1_048_576)
        elif index == 6:
            before, after_value = _role(state, _padded_profile(profile(index), 1_048_576), form), _padded_profile(profile(index), 1_048_577)
        elif index == 7:
            before, after_value = _role(state, _padded_profile(profile(index), 1_048_577), form), _padded_profile(profile(index), 1_048_575)
        elif index == 8:
            before, after_value = _role(state, _profile_bytes(profile(index), schema="ferris.profile-evidence/v1"), form), value
        drafts.append({"before": before, "after": _role(state, after_value if state == "regular-file" else None, form), "format": output_format})
    for relation in MEMBER_ORDERINGS:
        ordinal = len(drafts) + 1
        if relation == "before-reordered":
            first, second = _rich_profile(ordering_profile), _sorted_rich_profile(ordering_profile)
        elif relation == "after-reordered":
            first, second = _sorted_rich_profile(ordering_profile), _rich_profile(ordering_profile)
        else:
            first, second = _rich_profile(ordering_profile), _rich_profile(ordering_profile, alternate_root_order=True)
        drafts.append({"before": _role("regular-file", first), "after": _role("regular-file", second), "format": "json"})
    for depth in DUPLICATE_DEPTHS:
        valid = _profile_bytes(profile(len(drafts) + 1), sections=_base_sections(f"duplicate-{depth}"))
        drafts.extend((
            {"before": _role("regular-file", _duplicate_profile(profile(len(drafts) + 1), depth)), "after": _role("regular-file", valid), "format": "json"},
            {"before": _role("regular-file", valid), "after": _role("regular-file", _duplicate_profile(profile(len(drafts) + 2), depth)), "format": "json"},
            {"before": _role("regular-file", _duplicate_profile(profile(len(drafts) + 3), depth)), "after": _role("regular-file", _duplicate_profile(profile(len(drafts) + 3), depth)), "format": "json"},
            {"before": _role("regular-file", valid), "after": _role("regular-file", _duplicate_profile(profile(len(drafts) + 4), depth)), "format": "json"},
        ))
    for count in (9_999, 10_000, 10_001):
        ordinal = len(drafts) + 1
        drafts.append({"before": _role("regular-file", _change_profile(profile(ordinal), count, "before")), "after": _role("regular-file", _change_profile(profile(ordinal), count, "after")), "format": "json"})
    metadata = [(site, "A" * boundary) for site in METADATA_SITES for boundary in METADATA_BOUNDARIES]
    metadata.extend((site, value) for site in METADATA_SITES for value in ("A", "\x01", "é"))
    for left, right in zip(metadata[0::2], metadata[1::2]):
        ordinal = len(drafts) + 1
        drafts.append({"before": _role("regular-file", _metadata_profile(profile(ordinal), *left)), "after": _role("regular-file", _metadata_profile(profile(ordinal), *right)), "format": "json"})
    drafts.append({"before": _role("not-materialized"), "after": _role("not-materialized"), "format": "no-launch", "external_prerequisite": "external-immutable-binary-freeze"})
    if len(drafts) != REQUIRED_CASE_COUNT:
        raise MaterializationError("complete descriptor plan cardinality changed")
    for ordinal, draft in enumerate(drafts, start=1):
        for role_name in ("before", "after"):
            if draft[role_name]["state"] != "not-materialized":
                draft[role_name]["form"] = PATH_FORMS[(ordinal * 2 + (0 if role_name == "before" else 1)) % len(PATH_FORMS)] if ordinal > 27 else draft[role_name]["form"]
    return drafts


def _file_aggregate(files: list[tuple[str, bytes]]) -> str:
    digest = hashlib.sha256()
    for path, value in sorted(files):
        digest.update(str(len(value)).encode("ascii"))
        digest.update(b"\0")
        digest.update(path.encode("utf-8"))
        digest.update(b"\0")
        digest.update(hashlib.sha256(value).hexdigest().encode("ascii"))
        digest.update(b"\n")
    return "sha256:" + digest.hexdigest()


def _key(value: Any) -> bytes:
    return canonical_json(value)


def _failure_positions(cases: list[dict[str, Any]]) -> dict[str, set[str]]:
    values = {case["case_id"]: set() for case in cases}
    relocatable: dict[tuple[str, str, tuple[int, ...]], list[dict[str, Any]]] = {}
    for case in cases:
        before, after = case["semantic_witnesses"]["before"], case["semantic_witnesses"]["after"]
        if before["input_class"] != "valid" and after["input_class"] == "valid":
            values[case["case_id"]].add("before-only")
        elif before["input_class"] != "valid" and after["input_class"] != "valid":
            values[case["case_id"]].add("both-before-precedence")
        elif before["input_class"] == "valid" and after["input_class"] != "valid":
            key = (case["before"]["raw_sha256"], after["diagnostic"], tuple(after["duplicate_depths"]))
            relocatable.setdefault(key, []).append(case)
    for group in relocatable.values():
        for index, case in enumerate(sorted(group, key=lambda item: item["ordinal"])):
            values[case["case_id"]].add("after-only-after-valid-before" if index == 0 else "relocated-valid-before-same-after-failure")
    return values


def _interaction_tuples(cases: list[dict[str, Any]]) -> dict[str, dict[bytes, list[str]]]:
    output = {name: {} for name in INTERACTIONS}
    def add(name: str, value: dict[str, Any], case_id: str) -> None:
        output[name].setdefault(_key(value), []).append(case_id)
    failure = _failure_positions(cases)
    for case in cases:
        witness = case["semantic_witnesses"]
        for position in ("before", "after"):
            role, semantic = case[position], witness[position]
            for metadata in semantic["metadata"]:
                add("metadata-site-by-metadata-byte-boundary", {"metadata_site": metadata["site"], "metadata_byte_boundary": metadata["byte_length"]}, case["case_id"])
                if metadata["byte_length"] > 0:
                    add("metadata-site-by-character-kind-for-nonempty-values", {"metadata_site": metadata["site"], "character_kind": metadata["character_kind"]}, case["case_id"])
            if role["state"] != "not-materialized":
                add("input-position-by-path-state-by-path-form", {"input_position": position, "path_state": {"regular-file": "regular-file", "missing": "missing", "directory": "non-file"}[role["state"]], "path_form": semantic["path_form"]}, case["case_id"])
            if semantic["input_byte_boundary"] is not None:
                add("input-position-by-input-byte-boundary", {"input_position": position, "input_byte_boundary": semantic["input_byte_boundary"]}, case["case_id"])
            for kind in semantic["json_value_kinds"]:
                for ordering in _member_orderings_for_case(witness):
                    add("json-value-kind-by-member-ordering", {"json_value_kind": kind, "member_ordering": ordering}, case["case_id"])
            for depth in semantic["duplicate_depths"]:
                for position_value in failure[case["case_id"]]:
                    add("duplicate-depth-by-failure-position", {"duplicate_depth": depth, "failure_position": position_value}, case["case_id"])
        expected = case["execution"]["expected"]
        add("expected-result-class-by-exact-json-route", {"expected_result_class": expected["result_class"], "json_route": expected["stream"]}, case["case_id"])
        if expected["result_class"] in {"success", "difference"} and case["execution"]["format"] in {"json", "human"}:
            add("success-difference-by-json-human-format-pair", {"result_class": expected["result_class"], "format": case["execution"]["format"]}, case["case_id"])
    return output


def _member_orderings_for_case(witness: dict[str, Any]) -> set[str]:
    before, after = witness["before"], witness["after"]
    if before["member_reordered"] and after["member_reordered"] and witness["comparison"] == "success":
        return {"both-reordered-equivalent"}
    if before["member_reordered"] and not after["member_reordered"]:
        return {"before-reordered"}
    if after["member_reordered"] and not before["member_reordered"]:
        return {"after-reordered"}
    return set()


def _required_interaction_tuples() -> dict[str, list[dict[str, Any]]]:
    return {
        "metadata-site-by-metadata-byte-boundary": [{"metadata_site": site, "metadata_byte_boundary": boundary} for site in METADATA_SITES for boundary in METADATA_BOUNDARIES],
        "metadata-site-by-character-kind-for-nonempty-values": [{"metadata_site": site, "character_kind": kind} for site in METADATA_SITES for kind in METADATA_KINDS],
        "input-position-by-path-state-by-path-form": [{"input_position": position, "path_state": state, "path_form": form} for position in ("before", "after") for state in ("missing", "non-file", "regular-file") for form in PATH_FORMS],
        "input-position-by-input-byte-boundary": [{"input_position": position, "input_byte_boundary": boundary} for position in ("before", "after") for boundary in (1_048_575, 1_048_576, 1_048_577)],
        "json-value-kind-by-member-ordering": [{"json_value_kind": kind, "member_ordering": ordering} for kind in JSON_VALUE_KINDS for ordering in MEMBER_ORDERINGS],
        "duplicate-depth-by-failure-position": [{"duplicate_depth": depth, "failure_position": position} for depth in DUPLICATE_DEPTHS for position in FAILURE_POSITIONS],
        "expected-result-class-by-exact-json-route": [{"expected_result_class": name, "json_route": result["stream"]} for name, result in RESULT_MAP.items()],
        "success-difference-by-json-human-format-pair": [{"result_class": result, "format": output_format} for result in ("success", "difference") for output_format in ("json", "human")],
    }


def derive_coverage_catalog(cases: list[dict[str, Any]], raw_by_target: dict[str, bytes]) -> dict[str, Any]:
    domains = {name: {_key(value): [] for value in values} for name, values in DOMAINS}
    reverse_pairs: dict[tuple[str, str], list[dict[str, Any]]] = {}
    failures = _failure_positions(cases)
    for case in cases:
        witness, expected = case["semantic_witnesses"], case["execution"]["expected"]
        result = {"class": expected["result_class"], "exit": expected["exit"], "json_route": expected["stream"], "record": expected["record"], "diagnostics": expected["diagnostics"]}
        domains["result_classes"][_key(result)].append(case["case_id"])
        if expected["result_class"] in {"success", "difference"} and case["execution"]["format"] == "human":
            domains["human_parity_classes"][_key(expected["result_class"])].append(case["case_id"])
        if case["execution"]["mode"] == "launch-ready":
            domains["input_role_orderings"][_key("before-then-after-read")].append(case["case_id"])
        for position in ("before", "after"):
            role, semantic = case[position], witness[position]
            if role["state"] == "not-materialized":
                continue
            domains["path_states"][_key({"regular-file": "regular-file", "missing": "missing", "directory": "non-file"}[role["state"]])].append(case["case_id"])
            domains["path_forms"][_key(semantic["path_form"])].append(case["case_id"])
            for value in semantic["path_transformations"]:
                domains["lexical_normalization"][_key(value)].append(case["case_id"])
            for metadata in semantic["metadata"]:
                for name, value in (("metadata_sites", metadata["site"]), ("metadata_byte_boundaries", metadata["byte_length"]), ("metadata_character_kinds", metadata["character_kind"])):
                    if _key(value) in domains[name]:
                        domains[name][_key(value)].append(case["case_id"])
            for name in ("json_value_kinds", "number_representations", "pointer_key_kinds", "duplicate_depths"):
                for value in semantic[name]:
                    if _key(value) in domains[name]:
                        domains[name][_key(value)].append(case["case_id"])
            if semantic["input_byte_boundary"] is not None:
                domains["input_byte_boundaries"][_key(semantic["input_byte_boundary"])].append(case["case_id"])
        for ordering in _member_orderings_for_case(witness):
            domains["member_orderings"][_key(ordering)].append(case["case_id"])
        for position in failures[case["case_id"]]:
            domains["failure_positions"][_key(position)].append(case["case_id"])
        if witness["pair_change_count_boundary"] is not None:
            domains["change_count_boundaries"][_key(witness["pair_change_count_boundary"])].append(case["case_id"])
        if witness["before"]["input_class"] == witness["after"]["input_class"] == "valid":
            reverse_pairs.setdefault((case["before"]["raw_sha256"], case["after"]["raw_sha256"]), []).append(case)
    for pair, entries in reverse_pairs.items():
        if inverse := reverse_pairs.get((pair[1], pair[0])):
            for case in entries:
                value = "difference-pair-original-roles" if case["ordinal"] < min(item["ordinal"] for item in inverse) else "difference-pair-swapped-roles"
                domains["input_role_orderings"][_key(value)].append(case["case_id"])
    interaction_values = _interaction_tuples(cases)
    required_interactions = _required_interaction_tuples()
    interaction_records = []
    for name in INTERACTIONS:
        entries = [{"required_tuple": value, "case_ids": sorted(set(interaction_values[name].get(_key(value), [])))} for value in required_interactions[name]]
        for entry in entries:
            domains["interaction_requirements"][_key(name)].extend(entry["case_ids"])
        interaction_records.append({"name": name, "tuples": entries})
    return {
        "domains": [{"name": name, "required": list(values), "witness_case_ids": [sorted(set(domains[name][_key(value)])) for value in values]} for name, values in DOMAINS],
        "interactions": interaction_records,
    }


def _unsupported(error: OSError) -> bool:
    return error.errno in {errno.EINVAL, errno.ENOSYS, getattr(errno, "ENOTSUP", -1), getattr(errno, "EOPNOTSUPP", -1)}


def _sync_directory(directory: Path) -> dict[str, Any]:
    if not directory.is_dir():
        raise NotADirectoryError(directory)
    if os.name != "nt":
        flags = os.O_RDONLY | getattr(os, "O_DIRECTORY", 0) | getattr(os, "O_CLOEXEC", 0)
        try:
            descriptor = os.open(directory, flags)
        except OSError as error:
            if _unsupported(error):
                return {"state": "unsupported", "mechanism": "posix-fsync-directory", "error_code": error.errno}
            raise
        try:
            os.fsync(descriptor)
            return {"state": "synced", "mechanism": "posix-fsync-directory", "error_code": None}
        except OSError as error:
            if _unsupported(error):
                return {"state": "unsupported", "mechanism": "posix-fsync-directory", "error_code": error.errno}
            raise
        finally:
            os.close(descriptor)
    from ctypes import wintypes
    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    create = kernel32.CreateFileW
    create.argtypes = [wintypes.LPCWSTR, wintypes.DWORD, wintypes.DWORD, wintypes.LPVOID, wintypes.DWORD, wintypes.DWORD, wintypes.HANDLE]
    create.restype = wintypes.HANDLE
    flush, close = kernel32.FlushFileBuffers, kernel32.CloseHandle
    unsupported_codes, last_error, invalid = {1, 5, 6, 50, 87}, None, ctypes.c_void_p(-1).value
    for access in (0x80000000 | 0x40000000, 0x80000000):
        handle = create(str(directory.resolve()), access, 7, None, 3, 0x02000000, None)
        if handle == invalid:
            last_error = ctypes.get_last_error()
            if last_error in unsupported_codes:
                continue
            raise ctypes.WinError(last_error)
        try:
            if flush(handle):
                return {"state": "synced", "mechanism": "win32-directory-flush", "error_code": None}
            last_error = ctypes.get_last_error()
            if last_error not in unsupported_codes:
                raise ctypes.WinError(last_error)
        finally:
            close(handle)
    return {"state": "unsupported", "mechanism": "win32-directory-flush", "error_code": last_error}


def _record_sync(records: list[dict[str, Any]], step: str, directory: Path) -> dict[str, Any]:
    status = _sync_directory(directory)
    records.append({"step": step, **status})
    return status


def atomic_write(path: Path, value: bytes, records: list[dict[str, Any]] | None = None, step: str = "atomic-write-parent") -> None:
    temporary = path.with_name(f".{path.name}.partial-{os.getpid()}")
    if temporary.exists():
        raise MaterializationError("atomic-write residue already exists")
    descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, stat.S_IRUSR | stat.S_IWUSR)
    try:
        with os.fdopen(descriptor, "wb", closefd=False) as stream:
            stream.write(value)
            stream.flush()
            os.fsync(stream.fileno())
    finally:
        os.close(descriptor)
    try:
        os.replace(temporary, path)
        if records is None:
            _sync_directory(path.parent)
        else:
            _record_sync(records, step, path.parent)
    finally:
        if temporary.exists():
            temporary.unlink()


def find_residue(root: Path) -> list[str]:
    return [] if not root.exists() else sorted(path.relative_to(root).as_posix() for path in root.rglob("*") if ".partial-" in path.name)


def _attach(error: Exception, records: list[dict[str, Any]]) -> Exception:
    setattr(error, "directory_sync_records", list(records))
    return error


def _prepare_destination(output: Path, records: list[dict[str, Any]]) -> Path:
    if output.exists():
        raise MaterializationError("output destination already exists")
    parent = output.parent
    if not parent.is_dir():
        raise MaterializationError("output parent is unavailable")
    prefix = f".{output.name}.partial-"
    if any(path.name.startswith(prefix) for path in parent.iterdir()):
        raise MaterializationError("output parent contains materialization residue")
    stage = parent / f"{prefix}{os.getpid()}"
    stage.mkdir()
    try:
        _record_sync(records, "stage-created-parent", parent)
        return stage
    except Exception as sync_error:
        try:
            shutil.rmtree(stage)
            _record_sync(records, "stage-create-rollback-parent", parent)
        except Exception as rollback_error:
            raise _attach(PublicationIndeterminateError("staging directory sync failed and cleanup is indeterminate"), records) from rollback_error
        raise _attach(MaterializationError("staging directory sync failed; staging residue was rolled back"), records) from sync_error


def _publish_stage(stage: Path, output: Path, records: list[dict[str, Any]]) -> None:
    os.replace(stage, output)
    try:
        _record_sync(records, "published-parent", output.parent)
    except Exception as sync_error:
        try:
            shutil.rmtree(output)
            _record_sync(records, "publication-rollback-parent", output.parent)
        except Exception as rollback_error:
            raise _attach(PublicationIndeterminateError("publication directory sync failed and rollback is indeterminate"), records) from rollback_error
        raise _attach(MaterializationError("publication directory sync failed; published output was rolled back"), records) from sync_error


def _read_seed(seed_path: Path) -> bytes:
    try:
        mode = seed_path.stat().st_mode
        if not stat.S_ISREG(mode):
            raise MaterializationError("seed path is not a regular file")
        seed = seed_path.read_bytes()
    except MaterializationError:
        raise
    except OSError as error:
        raise MaterializationError("seed file is unavailable or unreadable") from error
    if len(seed) != 32:
        raise MaterializationError("seed file must contain exactly 32 bytes of CSPRNG material")
    return seed


def materialize(seed_path: Path, output: Path, case_count: int = REQUIRED_CASE_COUNT) -> dict[str, Any]:
    if case_count != REQUIRED_CASE_COUNT:
        raise MaterializationError(f"case count must be exactly {REQUIRED_CASE_COUNT} complete-coverage descriptors")
    seed, records = _read_seed(seed_path), []
    stage = _prepare_destination(output, records)
    try:
        (stage / "artifacts").mkdir()
        _record_sync(records, "artifacts-created-parent", stage)
        raw_by_target: dict[str, bytes] = {}
        files: list[tuple[str, bytes]] = []
        descriptors: list[dict[str, Any]] = []
        for ordinal, draft in enumerate(_drafts(seed), start=1):
            roles: dict[str, dict[str, Any]] = {}
            witnesses: dict[str, dict[str, Any]] = {}
            for name in ("before", "after"):
                draft_role, state = draft[name], draft[name]["state"]
                extension = "bin" if state == "regular-file" else "directory" if state == "directory" else "missing"
                target = None if state == "not-materialized" else f"artifacts/{ordinal:03d}-{name}.{extension}"
                raw = draft_role["raw"]
                if state == "regular-file":
                    assert target is not None and raw is not None
                    atomic_write(stage / target, raw, records, f"artifact-{ordinal:03d}-{name}-parent")
                    files.append((target, raw))
                    raw_by_target[target] = raw
                    role = {"state": state, "target": target, "raw_size": len(raw), "raw_sha256": sha256(raw), "request": _request(target, draft_role["form"])}
                elif state == "directory":
                    assert target is not None
                    (stage / target).mkdir()
                    _record_sync(records, f"directory-{ordinal:03d}-{name}-parent", (stage / target).parent)
                    role = {"state": state, "target": target, "raw_size": None, "raw_sha256": None, "request": _request(target, draft_role["form"])}
                elif state == "missing":
                    assert target is not None
                    role = {"state": state, "target": target, "raw_size": None, "raw_sha256": None, "request": _request(target, draft_role["form"])}
                else:
                    role = {"state": state, "target": None, "raw_size": None, "raw_sha256": None, "request": None}
                roles[name] = role
                witnesses[name] = _role_semantics(role, raw)
                role["expected_input"] = {"class": witnesses[name]["input_class"], "diagnostic": witnesses[name]["diagnostic"]}
            mode = "no-launch" if draft["format"] == "no-launch" else "launch-ready"
            descriptor = {
                "ordinal": ordinal, "case_id": derive(seed, "case-id", ordinal).hex(),
                "order_token": derive(seed, "case-order-token", ordinal).hex(),
                "profile_token": derive(seed, "profile-token", ordinal).hex(),
                "execution": {"mode": mode, "format": draft["format"], "expected": _expected(witnesses["before"], witnesses["after"], draft["before"]["raw"], draft["after"]["raw"], mode)},
                "before": roles["before"], "after": roles["after"],
                "semantic_witnesses": _semantic_witnesses(witnesses["before"], witnesses["after"], draft["before"]["raw"], draft["after"]["raw"], mode),
            }
            if mode == "no-launch":
                descriptor["external_prerequisite"] = draft["external_prerequisite"]
            descriptors.append(descriptor)
        if find_residue(stage):
            raise MaterializationError("materialization created atomic-write residue")
        _record_sync(records, "artifacts-stage", stage / "artifacts")
        case_manifest = {
            "schema": "ferris.pulse-35-corpus-case-manifest/v1", "derivation": DERIVATION,
            "seed_commitment_algorithm": "sha256(ferris-p35-seed-commitment-v1\\0 || seed)",
            "seed_commitment_sha256": seed_commitment(seed), "logical_case_max": MAX_LOGICAL_CASES,
            "required_case_count": REQUIRED_CASE_COUNT, "case_count": REQUIRED_CASE_COUNT,
            "artifact_aggregate_algorithm": "sha256-length-path-filedigest-v1",
            "artifact_aggregate": _file_aggregate(files), "cases": descriptors,
            "staging_directory_sync_records": records, "diagnostic_execution": False,
            "product_files_modified": False, "logical_retries": 0,
        }
        case_bytes = canonical_json(case_manifest)
        atomic_write(stage / "case-manifest.json", case_bytes, records, "case-manifest-stage-parent")
        catalog = derive_coverage_catalog(descriptors, raw_by_target)
        if any(not ids for domain in catalog["domains"] for ids in domain["witness_case_ids"]) or any(not entry["case_ids"] for interaction in catalog["interactions"] for entry in interaction["tuples"]):
            raise MaterializationError("concrete descriptors did not close the exact interaction catalog")
        coverage = {
            "schema": "ferris.pulse-35-corpus-coverage-manifest/v1",
            "authority_result_receipt": PULSE_34_RESULT_RECEIPT,
            "case_manifest_sha256": sha256(case_bytes), "case_count": REQUIRED_CASE_COUNT,
            "coverage_domains_closed": "18/18", "coverage_interactions_closed": "8/8",
            "derived_catalog": catalog, "diagnostic_execution": False,
            "product_files_modified": False, "logical_retries": 0,
        }
        atomic_write(stage / "coverage-manifest.json", canonical_json(coverage), records, "coverage-manifest-stage-parent")
        _record_sync(records, "stage-complete", stage)
        _publish_stage(stage, output, records)
    except Exception as error:
        if stage.exists():
            try:
                shutil.rmtree(stage)
                _record_sync(records, "stage-failure-rollback-parent", stage.parent)
            except Exception as cleanup_error:
                raise _attach(PublicationIndeterminateError("materialization cleanup is indeterminate"), records) from cleanup_error
        raise _attach(error, records)
    states = sorted({record["state"] for record in records})
    return {
        "schema": "ferris.pulse-35-corpus-materialization-summary/v1",
        "case_count": REQUIRED_CASE_COUNT, "coverage_domains_closed": "18/18",
        "coverage_interactions_closed": "8/8", "directory_sync_posture": "unsupported" if "unsupported" in states else "synced",
        "directory_sync_records": records, "logical_retries": 0, "residue_count": 0,
        "diagnostic_execution": False,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Materialize a public-rule-only Pulse 35 profile-evidence corpus.")
    parser.add_argument("--seed-file", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--case-count", default=REQUIRED_CASE_COUNT, type=int)
    arguments = parser.parse_args()
    try:
        summary = materialize(arguments.seed_file, arguments.output, arguments.case_count)
    except (MaterializationError, OSError) as error:
        print(f"materialization rejected: {error}", file=sys.stderr)
        return 2
    print(canonical_json(summary).decode("ascii"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
