# Public `ferris.profile-evidence/v0` Input Contract

Status: Normative public input contract
Dialect: JSON Schema Draft 2020-12 plus normative byte, framing, and parsing
rules
Schema raw SHA-256:
`sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b`

This contract publishes the complete input acceptance boundary used by the
existing experimental `profile-diff` command. It changes no production code,
does not generate a profile, and grants no diagnostic relaunch, score, fix,
support, or PLATFORM-001 authority.

An independent generator can construct accepted and declared-invalid inputs
from this document, the schema, the six positive fixtures, and the 33 negative
controls without reading Ferris source or tests.

## Normative artifacts

- [Draft 2020-12 schema](schemas/ferris.profile-evidence.v0.schema.json)
- [Positive scalar fixture](fixtures/profile-evidence-v0-positive-scalars.json)
- [Positive array fixture](fixtures/profile-evidence-v0-positive-arrays.json)
- [Positive object fixture](fixtures/profile-evidence-v0-positive-objects.json)
- [Positive nested fixture](fixtures/profile-evidence-v0-positive-nested-mixed.json)
- [Positive minimum-boundary fixture](fixtures/profile-evidence-v0-positive-boundary-minimum.json)
- [Positive maximum-boundary fixture](fixtures/profile-evidence-v0-positive-boundary-maximum.json)
- [Negative mutation controls](fixtures/profile-evidence-v0-mutations.json)

All published JSON files are UTF-8, LF-terminated, and contain no CR bytes.

## Complete accepted value

The parsed root value MUST be an object with exactly these five members, all
required:

| Member | Accepted value |
|---|---|
| `schema` | the exact string `ferris.profile-evidence/v0` |
| `profile_id` | 1 through 256 visible ASCII characters |
| `revision` | 1 through 256 visible ASCII characters |
| `consumer` | 1 through 256 visible ASCII characters |
| `sections` | the exact closed object defined below |

Visible ASCII means every decoded character is one byte in the inclusive
range `!` (`0x21`) through `~` (`0x7e`). Space, tabs, line breaks, control
characters, non-ASCII characters, empty strings, and 257-character values are
invalid in `profile_id`, `revision`, and `consumer`. Because the permitted
alphabet is ASCII, the character and UTF-8 byte counts are identical.

The parsed `sections` object MUST contain exactly these twelve members, all
required, with no additional member:

1. `identity`
2. `closure`
3. `features`
4. `toolchain`
5. `targets`
6. `providers`
7. `native`
8. `stages`
9. `assurance`
10. `stewardship`
11. `support`
12. `lifecycle`

Each section value MAY be any recursively composed JSON value: null, boolean,
number, string, array, or object. Arrays MAY contain any JSON values. Objects
MAY contain any number of members, but every decoded object member name at
every depth MUST use 1 through 256 visible ASCII characters under the same
`0x21` through `0x7e` rule. String values inside sections are not metadata
names and MAY contain any valid JSON string content.

Every JSON object at every depth MUST have unique member names. Duplicate
members are prohibited even if their values are equal. This rule applies to
the root, `sections`, objects inside arrays, and every deeper object.

## Normative byte and parsing rules

The JSON Schema validates a parsed JSON value. It cannot validate filesystem
state, raw byte length, malformed framing, encoding errors, or duplicate
member occurrences that a non-strict parser has already discarded. The
following companion rules are therefore normative and MUST be applied before
or alongside schema validation:

1. The supplied path MUST exist, be readable, and identify a regular file.
2. The complete file, including leading or trailing JSON whitespace, MUST be
   at most **1,048,576 bytes**.
3. Exactly 1,048,576 bytes is permitted. Any size of 1,048,577 bytes or more
   is oversized before JSON parsing.
4. The bytes MUST form one complete UTF-8 JSON value. Invalid UTF-8, an empty
   file, truncation, trailing non-whitespace, and other malformed JSON are
   invalid JSON.
5. Parsing MUST preserve and reject duplicate object member occurrences
   rather than silently applying first-member-wins or last-member-wins.
6. During that same recursive parse, every object member name MUST satisfy the
   visible-ASCII 1-through-256 rule.
7. After parsing, the schema string, closed root shape, closed `sections`
   shape, and metadata values MUST satisfy the parsed-value schema.

Generators SHOULD emit UTF-8 without a byte-order mark and SHOULD use LF for
portable public bytes. Whitespace outside the JSON value counts toward the
file-size limit.

## Classification and precedence

Ferris applies these boundaries in the following order. A generator declaring
an invalid control MUST use the first applicable row.

| Boundary | Result class | Diagnostic |
|---|---|---|
| path missing, metadata unreadable, open failure, or incomplete read | `incomplete` | `FERRIS-PROFILE-INPUT-UNAVAILABLE` |
| path exists but is not a regular file | `incomplete` | `FERRIS-PROFILE-INPUT-NOT-FILE` |
| file size is greater than 1,048,576 bytes | `incomplete` | `FERRIS-PROFILE-INPUT-OVERSIZED` |
| duplicate object member at any depth | `invalid` | `FERRIS-PROFILE-JSON-DUPLICATE-MEMBER` |
| object member name violates the recursive metadata-name rule | `invalid` | `FERRIS-PROFILE-METADATA-INVALID` |
| malformed JSON, invalid UTF-8, empty input, or other parse failure | `invalid` | `FERRIS-PROFILE-JSON-INVALID` |
| `schema` is present as a string but is not `ferris.profile-evidence/v0` | `unsupported` | `FERRIS-PROFILE-SCHEMA-UNSUPPORTED` |
| missing/non-string schema, unknown or missing root member, non-object or incomplete/extra `sections` | `invalid` | `FERRIS-PROFILE-SHAPE-INVALID` |
| `profile_id`, `revision`, or `consumer` violates its metadata rule | `invalid` | `FERRIS-PROFILE-IDENTITY-INVALID` |

A supported schema string is checked before closed-shape deserialization.
Therefore a present unsupported string is `unsupported` even if another shape
defect also exists. A missing or non-string `schema` cannot select another
schema and is instead an invalid shape.

The file-size gate precedes parsing. An oversized malformed file is
`incomplete`/`FERRIS-PROFILE-INPUT-OVERSIZED`; an exactly 1,048,576-byte
malformed file reaches parsing and is `invalid`/`FERRIS-PROFILE-JSON-INVALID`.

## Positive fixture coverage

The six positive files are complete standalone inputs:

| Fixture | Coverage |
|---|---|
| `positive-scalars` | null, booleans, integer, floating-point number, empty string value, ordinary string, and Unicode string value |
| `positive-arrays` | empty and non-empty arrays, nested arrays, objects in arrays, and arrays in objects |
| `positive-objects` | all twelve sections as ordinary objects |
| `positive-nested-mixed` | recursive arrays/objects/scalars and the base for byte mutations |
| `positive-boundary-minimum` | one-character metadata and one-character object member name |
| `positive-boundary-maximum` | 256-character `profile_id`, `revision`, `consumer`, and nested object member name |

The test-only validator also appends JSON space bytes to a valid fixture and
verifies acceptance at exactly 1,048,576 bytes.

## Negative-control construction

`profile-evidence-v0-mutations.json` contains 33 controls. Unless an operation
states otherwise, start from the parsed `base_fixture`.

- `replace`, `remove`, and `add` apply the supplied JSON Pointer operation.
- `replace-repeat` replaces the pointer value with `character` repeated
  `count` times.
- `insert-member` inserts `key` and `value` into the object at `pointer`; the
  empty pointer selects the root.
- `insert-repeated-member` inserts a name made by repeating `character`
  `count` times.
- Parsed-value mutations are serialized as one complete UTF-8 JSON value
  before validation.
- `raw-replace` starts from the original LF base bytes and replaces exactly
  the first occurrence of `needle` with `replacement`. The needle MUST occur
  exactly once.
- `raw-content` uses the UTF-8 bytes of `content` as the complete file.
- `pad-to-size` appends ASCII space bytes after the complete base JSON until
  the byte count equals `size`.
- `source-state` creates no bytes and presents the declared `missing`,
  `non_file`, or `unreadable` filesystem state.

Each control declares its expected result class and diagnostic. These controls
cover unsupported and missing schema, root/section closure, every metadata
boundary, recursive invalid keys, duplicate members at root/section/nested
depths, malformed and empty JSON, oversized input, and missing/non-file/
unreadable source states.

## Limits and authority

This is a syntax, framing, and input-acceptance contract only. Acceptance does
not establish semantic correctness, compatibility, support, freshness,
approval, safety, security, or readiness. Section meanings remain caller
evidence. Ferris still does not generate profiles, invoke owner tools, discover
inputs, access a network, mutate repositories, or execute a diagnostic under
this release.
