"""Exact public Pulse 31 input-contract verifier.

This module deliberately binds the frozen Pulse 31 artifacts by path, size,
and digest.  It does not inspect the current schema inventory or invoke the
historical Rust test.
"""

from __future__ import annotations

import hashlib
import json
import os
import stat
from dataclasses import dataclass
from pathlib import Path
from typing import Any


PROFILE_SCHEMA = "ferris.profile-evidence/v0"
MAX_INPUT_BYTES = 1_048_576
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

BOUND_ARTIFACTS = {
    "contract": {
        "path": "docs/simulations/profile-diff-held-out/INPUT_PROFILE_EVIDENCE.md",
        "sha256": "sha256:26fdb4b9eed558f1f03a66eaec13749bfbad7ea4612c6f7e58bb8e7b79e69295",
        "size": 9129,
    },
    "schema": {
        "path": "docs/simulations/profile-diff-held-out/schemas/ferris.profile-evidence.v0.schema.json",
        "sha256": "sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b",
        "size": 3108,
    },
    "mutations": {
        "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-mutations.json",
        "sha256": "sha256:b33985e51f54c2ed0121b94571b622ee47bbd00450c8ab1c3d65d0f463276158",
        "size": 8818,
    },
    "positive": (
        {
            "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-positive-scalars.json",
            "sha256": "sha256:650b15ffcadb46ff3673f889ef35900a14e9f8bc09e824739d0117f5ebeadf69",
            "size": 487,
        },
        {
            "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-positive-arrays.json",
            "sha256": "sha256:4f2c99b179253eb7ea6aa3ba227321dffaabbb3a81a11c786018ff8217d56cc8",
            "size": 587,
        },
        {
            "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-positive-objects.json",
            "sha256": "sha256:57bbd633d73ee9633b030d79f3452797928e93bc8766c84d2aeaea3d013bdd7a",
            "size": 785,
        },
        {
            "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-positive-nested-mixed.json",
            "sha256": "sha256:b4b682a0673899ac3cdf757bbd5280d2330cd3e72a423922ffe8cdfda1ad5dcd",
            "size": 1064,
        },
        {
            "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-positive-boundary-minimum.json",
            "sha256": "sha256:a07134d0ebf010515c3a057a3f7498105e23b351dc1b67fa030b79dec2cd68fd",
            "size": 407,
        },
        {
            "path": "docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-positive-boundary-maximum.json",
            "sha256": "sha256:75dbf918361795865dbe83f1b25b8aec1f23aad4475ce88f6620dd4d3efa0665",
            "size": 1427,
        },
    ),
}


class P31Failure(RuntimeError):
    """A fail-closed public-contract validation failure."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class DuplicateMember(ValueError):
    """Raised while preserving duplicate JSON object members."""


class InvalidMemberName(ValueError):
    """Raised for a recursively invalid public JSON object member name."""


@dataclass(frozen=True)
class Outcome:
    result_class: str
    diagnostic: str | None

    @property
    def accepted(self) -> bool:
        return self.result_class == "accepted"


def _digest(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def _visible_ascii(value: object) -> bool:
    return (
        type(value) is str
        and 1 <= len(value) <= 256
        and all("!" <= character <= "~" for character in value)
    )


def _pairs_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateMember()
        if not _visible_ascii(key):
            raise InvalidMemberName()
        result[key] = value
    return result


def _reject_constant(_: str) -> object:
    raise ValueError("non-finite JSON number")


def _parse_strict(data: bytes) -> object:
    text = data.decode("utf-8")
    return json.loads(
        text,
        object_pairs_hook=_pairs_object,
        parse_constant=_reject_constant,
    )


def _safe_regular_bytes(path: Path, *, maximum: int) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise P31Failure("P51-P31-ARTIFACT-TYPE")
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except P31Failure:
        raise
    except OSError as error:
        raise P31Failure("P51-P31-ARTIFACT-UNAVAILABLE") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise P31Failure("P51-P31-ARTIFACT-TYPE")
        content = bytearray()
        while chunk := os.read(descriptor, 65_536):
            content.extend(chunk)
            if len(content) > maximum:
                raise P31Failure("P51-P31-ARTIFACT-TOO-LARGE")
        return bytes(content)
    except OSError as error:
        raise P31Failure("P51-P31-ARTIFACT-UNAVAILABLE") from error
    finally:
        os.close(descriptor)


def _bound_path(repo_root: Path, relative: str) -> Path:
    try:
        root = repo_root.resolve(strict=True)
    except OSError as error:
        raise P31Failure("P51-P31-ARTIFACT-UNAVAILABLE") from error
    candidate = root.joinpath(*relative.split("/"))
    try:
        candidate.relative_to(root)
    except ValueError as error:
        raise P31Failure("P51-P31-ARTIFACT-PATH") from error
    return candidate


def _verify_artifact(repo_root: Path, binding: dict[str, object]) -> bytes:
    path = _bound_path(repo_root, str(binding["path"]))
    content = _safe_regular_bytes(path, maximum=MAX_INPUT_BYTES + 16_384)
    if (
        len(content) != binding["size"]
        or _digest(content) != binding["sha256"]
        or b"\r" in content
        or not content.endswith(b"\n")
    ):
        raise P31Failure("P51-P31-ARTIFACT-IDENTITY")
    return content


def validate_bytes(data: bytes) -> Outcome:
    """Apply only the frozen Pulse 31 parsing and shape contract."""

    if len(data) > MAX_INPUT_BYTES:
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-OVERSIZED")
    try:
        value = _parse_strict(data)
    except DuplicateMember:
        return Outcome("invalid", "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER")
    except InvalidMemberName:
        return Outcome("invalid", "FERRIS-PROFILE-METADATA-INVALID")
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError):
        return Outcome("invalid", "FERRIS-PROFILE-JSON-INVALID")

    if type(value) is not dict:
        return Outcome("invalid", "FERRIS-PROFILE-SHAPE-INVALID")
    schema = value.get("schema")
    if type(schema) is str and schema != PROFILE_SCHEMA:
        return Outcome("unsupported", "FERRIS-PROFILE-SCHEMA-UNSUPPORTED")
    required_root = {"schema", "profile_id", "revision", "consumer", "sections"}
    if set(value) != required_root or schema != PROFILE_SCHEMA:
        return Outcome("invalid", "FERRIS-PROFILE-SHAPE-INVALID")
    sections = value["sections"]
    if type(sections) is not dict or set(sections) != set(SECTION_NAMES):
        return Outcome("invalid", "FERRIS-PROFILE-SHAPE-INVALID")
    if not all(_visible_ascii(value[field]) for field in ("profile_id", "revision", "consumer")):
        return Outcome("invalid", "FERRIS-PROFILE-IDENTITY-INVALID")
    return Outcome("accepted", None)


def parse_accepted_profile(data: bytes) -> dict[str, object]:
    """Return one strictly parsed profile only after the frozen P31 gate accepts it."""

    if not validate_bytes(data).accepted:
        raise P31Failure("P51-P31-PROFILE-NOT-ACCEPTED")
    value = _parse_strict(data)
    if type(value) is not dict:
        raise P31Failure("P51-P31-PROFILE-NOT-ACCEPTED")
    return value


def validate_source_state(state: str, data: bytes | None = None) -> Outcome:
    if state in {"missing", "unreadable"}:
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE")
    if state == "non_file":
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-NOT-FILE")
    if state != "bytes" or data is None:
        raise P31Failure("P51-P31-SOURCE-STATE")
    return validate_bytes(data)


def validate_path(path: Path) -> Outcome:
    """Validate one caller-supplied path without following a symlink."""

    try:
        metadata = os.lstat(path)
    except OSError:
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-NOT-FILE")
    try:
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except OSError:
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE")
    try:
        data = bytearray()
        while chunk := os.read(descriptor, 65_536):
            data.extend(chunk)
            if len(data) > MAX_INPUT_BYTES:
                return Outcome("incomplete", "FERRIS-PROFILE-INPUT-OVERSIZED")
    except OSError:
        return Outcome("incomplete", "FERRIS-PROFILE-INPUT-UNAVAILABLE")
    finally:
        os.close(descriptor)
    return validate_bytes(bytes(data))


def _pointer_parts(pointer: str) -> list[str]:
    if pointer == "":
        return []
    if not pointer.startswith("/"):
        raise P31Failure("P51-P31-MUTATION-POINTER")
    return [part.replace("~1", "/").replace("~0", "~") for part in pointer[1:].split("/")]


def _pointer_parent(value: dict[str, object], pointer: str) -> tuple[dict[str, object], str]:
    parts = _pointer_parts(pointer)
    if not parts:
        raise P31Failure("P51-P31-MUTATION-POINTER")
    current: object = value
    for part in parts[:-1]:
        if type(current) is dict:
            if part not in current:
                raise P31Failure("P51-P31-MUTATION-POINTER")
            current = current[part]
        elif type(current) is list and part.isdecimal():
            index = int(part)
            if index >= len(current):
                raise P31Failure("P51-P31-MUTATION-POINTER")
            current = current[index]
        else:
            raise P31Failure("P51-P31-MUTATION-POINTER")
    if type(current) is not dict:
        raise P31Failure("P51-P31-MUTATION-POINTER")
    return current, parts[-1]


def _pointer_value(value: dict[str, object], pointer: str) -> object:
    current: object = value
    for part in _pointer_parts(pointer):
        if type(current) is dict:
            if part not in current:
                raise P31Failure("P51-P31-MUTATION-POINTER")
            current = current[part]
        elif type(current) is list and part.isdecimal():
            index = int(part)
            if index >= len(current):
                raise P31Failure("P51-P31-MUTATION-POINTER")
            current = current[index]
        else:
            raise P31Failure("P51-P31-MUTATION-POINTER")
    return current


def _repeat(mutation: dict[str, object]) -> str:
    character = mutation.get("character")
    count = mutation.get("count")
    if type(character) is not str or len(character) != 1 or type(count) is not int:
        raise P31Failure("P51-P31-MUTATION-SHAPE")
    return character * count


def _mutation_source(
    base_bytes: bytes, base_value: dict[str, object], mutation: dict[str, object]
) -> tuple[str, bytes | None]:
    operation = mutation.get("operation")
    if type(operation) is not str:
        raise P31Failure("P51-P31-MUTATION-SHAPE")
    if operation in {"replace", "add", "remove", "replace-repeat"}:
        value = json.loads(json.dumps(base_value))
        parent, key = _pointer_parent(value, str(mutation.get("pointer")))
        if operation == "remove":
            if key not in parent:
                raise P31Failure("P51-P31-MUTATION-POINTER")
            del parent[key]
        elif operation == "replace-repeat":
            parent[key] = _repeat(mutation)
        else:
            parent[key] = mutation.get("value")
        return "bytes", json.dumps(value, ensure_ascii=False, indent=2).encode("utf-8")
    if operation in {"insert-member", "insert-repeated-member"}:
        value = json.loads(json.dumps(base_value))
        pointer = str(mutation.get("pointer"))
        current = _pointer_value(value, pointer)
        if type(current) is not dict:
            raise P31Failure("P51-P31-MUTATION-POINTER")
        key = _repeat(mutation) if operation == "insert-repeated-member" else mutation.get("key")
        if type(key) is not str:
            raise P31Failure("P51-P31-MUTATION-SHAPE")
        current[key] = mutation.get("value")
        return "bytes", json.dumps(value, ensure_ascii=False, indent=2).encode("utf-8")
    if operation == "raw-replace":
        needle = mutation.get("needle")
        replacement = mutation.get("replacement")
        if type(needle) is not str or type(replacement) is not str:
            raise P31Failure("P51-P31-MUTATION-SHAPE")
        source = base_bytes.decode("utf-8")
        if source.count(needle) != 1:
            raise P31Failure("P51-P31-MUTATION-SHAPE")
        return "bytes", source.replace(needle, replacement, 1).encode("utf-8")
    if operation == "raw-content":
        content = mutation.get("content")
        if type(content) is not str:
            raise P31Failure("P51-P31-MUTATION-SHAPE")
        return "bytes", content.encode("utf-8")
    if operation == "pad-to-size":
        size = mutation.get("size")
        if type(size) is not int or size < len(base_bytes):
            raise P31Failure("P51-P31-MUTATION-SHAPE")
        return "bytes", base_bytes + b" " * (size - len(base_bytes))
    if operation == "source-state":
        state = mutation.get("state")
        if state not in {"missing", "non_file", "unreadable"}:
            raise P31Failure("P51-P31-MUTATION-SHAPE")
        return str(state), None
    raise P31Failure("P51-P31-MUTATION-OPERATION")


def _verify_schema(schema: object) -> None:
    if type(schema) is not dict:
        raise P31Failure("P51-P31-SCHEMA-SHAPE")
    properties = schema.get("properties")
    definitions = schema.get("$defs")
    if (
        schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema"
        or schema.get("$id") != "urn:ferris:schema:profile-evidence:v0"
        or schema.get("type") != "object"
        or schema.get("additionalProperties") is not False
        or type(properties) is not dict
        or type(definitions) is not dict
        or set(properties) != {"schema", "profile_id", "revision", "consumer", "sections"}
        or set(schema.get("required", [])) != set(properties)
        or properties["schema"] != {"const": PROFILE_SCHEMA}
    ):
        raise P31Failure("P51-P31-SCHEMA-SHAPE")
    for name in ("visibleAsciiMetadata", "visibleAsciiName"):
        item = definitions.get(name)
        if item != {
            "type": "string",
            "minLength": 1,
            "maxLength": 256,
            "pattern": "^[!-~]{1,256}$",
        }:
            raise P31Failure("P51-P31-SCHEMA-SHAPE")
    sections = properties["sections"]
    if (
        type(sections) is not dict
        or sections.get("type") != "object"
        or sections.get("additionalProperties") is not False
        or set(sections.get("required", [])) != set(SECTION_NAMES)
        or set(sections.get("properties", {})) != set(SECTION_NAMES)
    ):
        raise P31Failure("P51-P31-SCHEMA-SHAPE")


def verify_bound_contract(repo_root: Path) -> dict[str, int]:
    """Verify the frozen artifacts, six positives, and all 33 mutations."""

    contract = _verify_artifact(repo_root, BOUND_ARTIFACTS["contract"])
    schema_bytes = _verify_artifact(repo_root, BOUND_ARTIFACTS["schema"])
    mutations_bytes = _verify_artifact(repo_root, BOUND_ARTIFACTS["mutations"])
    if PROFILE_SCHEMA.encode("ascii") not in contract:
        raise P31Failure("P51-P31-CONTRACT-CONTENT")
    try:
        _verify_schema(json.loads(schema_bytes))
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
        raise P31Failure("P51-P31-SCHEMA-SHAPE") from error

    positive_count = 0
    for binding in BOUND_ARTIFACTS["positive"]:
        if not validate_bytes(_verify_artifact(repo_root, binding)).accepted:
            raise P31Failure("P51-P31-POSITIVE-REJECTED")
        positive_count += 1

    try:
        mutations = json.loads(mutations_bytes)
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
        raise P31Failure("P51-P31-MUTATION-REGISTRY") from error
    if (
        type(mutations) is not dict
        or mutations.get("schema") != "ferris.profile-evidence-v0-mutations/v1"
        or type(mutations.get("base_fixture")) is not str
        or type(mutations.get("mutations")) is not list
        or len(mutations["mutations"]) != 33
    ):
        raise P31Failure("P51-P31-MUTATION-REGISTRY")
    base_path = _bound_path(
        repo_root,
        "docs/simulations/profile-diff-held-out/fixtures/" + mutations["base_fixture"],
    )
    base_bytes = _safe_regular_bytes(base_path, maximum=MAX_INPUT_BYTES)
    if not validate_bytes(base_bytes).accepted:
        raise P31Failure("P51-P31-BASE-FIXTURE")
    try:
        base_value = json.loads(base_bytes)
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
        raise P31Failure("P51-P31-BASE-FIXTURE") from error
    if type(base_value) is not dict:
        raise P31Failure("P51-P31-BASE-FIXTURE")

    mutation_ids: set[str] = set()
    for mutation_value in mutations["mutations"]:
        if type(mutation_value) is not dict:
            raise P31Failure("P51-P31-MUTATION-REGISTRY")
        mutation_id = mutation_value.get("id")
        expected = mutation_value.get("expected")
        if (
            type(mutation_id) is not str
            or mutation_id in mutation_ids
            or type(expected) is not dict
            or type(expected.get("result_class")) is not str
            or type(expected.get("diagnostic")) is not str
        ):
            raise P31Failure("P51-P31-MUTATION-REGISTRY")
        mutation_ids.add(mutation_id)
        source_state, data = _mutation_source(base_bytes, base_value, mutation_value)
        outcome = validate_source_state(source_state, data)
        if (
            outcome.result_class != expected["result_class"]
            or outcome.diagnostic != expected["diagnostic"]
        ):
            raise P31Failure("P51-P31-MUTATION-CLASSIFICATION")

    return {
        "artifact_count": 9,
        "positive_fixture_count": positive_count,
        "mutation_control_count": len(mutation_ids),
        "public_input_checks": positive_count + len(mutation_ids),
    }
