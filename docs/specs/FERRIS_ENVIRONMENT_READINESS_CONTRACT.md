# READINESS-001: Ferris Environment Readiness Contract

Status: Draft after eleven-role review
Implementation authority: None
Depends on: PRODUCT-001 and VIEW-001

## Purpose

This specification defines a product-neutral, read-only contract for comparing
explicit owner requirements with bounded local environment observations.

The contract does not authorize an implementation, installation, repair,
command execution, shell evaluation, lifecycle execution, network access,
secret handling, resource probe, or owner validation. It defines the records a
separately approved extension of `doctor` would have to accept and emit.

## Authority

Repository owners declare requirements and whether each is `required` or
`advisory`. Ferris observes only the explicitly supported local property and
does not establish that an owner command will pass.

Cargo retains package, workspace, dependency, lock, feature, target, and build
authority. Rustup retains Rust toolchain selection and installation authority.
Environment managers, container formats, Devfile, CI systems, operating-system
package managers, and repository documentation retain their native semantics.

Ferris MUST NOT:

- infer requirements from commands, workflows, logs, source code, or failures;
- select one owner-native source over another;
- combine source records into synthetic owner truth;
- resolve or install packages, toolchains, components, targets, or executables;
- execute an executable to discover its version or capability;
- evaluate a shell, task, hook, lifecycle command, or environment template;
- access a network, service, daemon, port, credential provider, or package
  registry; or
- treat presence as proof of behavioral correctness, support, or CI
  equivalence.

## Requirement declaration

An input record is `ferris.environment-requirements/v1`. It contains:

- one portable workspace identity;
- one owner-defined declaration identity;
- one owner identity;
- 1-64 source references;
- 1-256 requirements; and
- no executable commands or environment values.

The canonical schema is
[`ferris.environment-requirements.v1.schema.json`](../schemas/environment-readiness/ferris.environment-requirements.v1.schema.json).
Input is strict JSON: duplicate members, unknown members, malformed JSON,
unsupported schema versions, and records larger than 1 MiB are not accepted.

Every requirement has a unique owner-defined `requirement_id`, one
`criticality`, optional platform applicability, and at least one source
reference. Requirements MUST be sorted by `requirement_id`; sources MUST be
sorted by `source_id`; each requirement's source IDs MUST be sorted. IDs are
compared as case-sensitive ASCII.

`required` means a non-satisfied applicable observation can prevent readiness.
`advisory` means the observation remains visible but cannot alone make the
aggregate non-ready.

V1 supports exactly four requirement kinds:

| Kind | Owner declaration | Passive observation |
|---|---|---|
| `platform` | Allowed operating systems and optional architectures | Compare with the current process OS and architecture |
| `executable` | One executable leaf name and `process_path` resolution | Resolve by the current process search rules without launching it |
| `environment` | One canonical uppercase environment name | Observe name presence only |
| `path` | One workspace-relative path and expected `file` or `directory` kind | Read bounded filesystem metadata below the canonical workspace root |

Executable names MUST be leaf names. Paths, separators, drive qualifiers,
shell syntax, arguments, and version constraints are invalid. Environment names
MUST use uppercase ASCII canonical form. Their values, lengths, digests, and
derived content MUST NOT be retained.

Workspace-relative paths use `/`, MUST NOT be absolute or drive-qualified, and
MUST NOT contain empty, `.` or `..` components. The single value `.` MAY name
the workspace root only when the expected kind is `directory`. Symlinks and
reparse points are not followed outside the canonical workspace root.

## Source provenance

A source reference names:

- a unique source ID;
- source kind;
- claimed owner authority;
- optional workspace-relative source path;
- optional already-known content digest; and
- interpretation status.

V1 interpretation is always `declared_only`, and `claimed_authority` MUST equal
the declaration's top-level `owner`. It is an owner assertion, not verification
that Cargo, rustup, or another external system endorsed the requirement. Source
references explain why the owner declared a requirement; Ferris does not parse
or validate the referenced format in V1. Missing optional source files do not
silently remove a requirement.

Known source kinds are `owner_declaration`, `cargo_metadata`,
`rust_toolchain`, `asdf_tool_versions`, `mise`, `devcontainer`, `devfile`,
`documentation`, and `other`. Naming a kind does not claim compatibility with
that format.

Future adapters MUST produce separately bound observations, preserve exact
source provenance, and report incompatible or conflicting declarations. V1
cannot emit source conflict because it interprets no source. Adapters MUST NOT
outrank this explicit declaration without a later owner policy contract.

## Passive observation algorithms

The observed platform is the current Ferris process platform. Operating system
is `windows`, `linux`, `macos`, or `other`; architecture is the lowercase
target-architecture token reported by the compiled Ferris binary. `other`
allows an unsupported host to emit a typed report.

Executable resolution never launches a candidate:

- empty search-path elements are ignored and never mean current directory;
- current directory is never searched in V1 because every accepted search-path
  entry must be absolute;
- on Unix, Ferris joins the exact leaf name to each process `PATH` directory in
  order and requires a regular file with at least one execute bit;
- on Windows, Ferris joins the exact leaf name first; when the leaf has no
  extension it then tests process `PATHEXT` entries in declared order after
  uppercase normalization;
- non-absolute search-path entries are `unsupported` in V1;
- an absent `PATH`, or absent `PATHEXT` when Windows extension expansion is
  needed, makes the executable observation `unavailable`;
- access denial or metadata failure is `unavailable`; and
- no candidate path, search-path value, or metadata is retained.

Repository-path observation starts from the already canonical workspace root,
processes each declared component with non-following metadata, and returns
`unsupported` if any component is a symlink or Windows reparse point. Absence
is `missing`, an unexpected final kind is `mismatched`, and access or metadata
failure is `unavailable`. V1 does not claim race-free authorization from this
diagnostic observation.

Environment observation compares the canonical declared ASCII name with the
current process environment using Windows case-insensitive lookup and Unix
case-sensitive lookup. It records presence only. Enumeration or access failure
is `unavailable`.

## Observation states

Every declared requirement produces exactly one observation with the same
requirement ID, kind, and criticality.

| State | Meaning |
|---|---|
| `satisfied` | The supported passive check matched the declaration |
| `missing` | The declared executable, environment name, file, or directory was absent |
| `mismatched` | An observed platform or filesystem kind did not match |
| `not_applicable` | The current platform is outside the requirement's explicit applicability |
| `unsupported` | Ferris does not support the declared observation on this platform or contract version |
| `unavailable` | A required local owner or OS observation could not be obtained |
| `not_observed` | The check was deliberately not attempted or no admissible observation exists |
| `stale` | A bound declaration or source identity changed before observation completed |
| `unknown` | Evidence exists but cannot establish any more specific state |

`missing`, `unavailable`, `not_observed`, and `unknown` are distinct. A missing
item was successfully checked and found absent. An unavailable observation
could not be obtained. A not-observed check was not attempted. Unknown evidence
was observed but is insufficient to classify.

No V1 passive check executes owner work, so `failed` and `conflicting` are not
V1 observation states. A later owner command failure remains an execution
result and MUST NOT be rewritten as readiness evidence. A future adapter
contract must add conflict evidence and choose `incomplete` or `blocked`
according to VIEW-001 rather than assuming one class.

Each kind has an exact semantic matrix:

| Kind | Active method | Allowed states | Diagnostic prefix |
|---|---|---|---|
| `platform` | `current_platform` | `satisfied`, `mismatched`, `not_applicable`, `unsupported`, `unavailable`, `not_observed`, `stale`, `unknown` | `FERRIS-READINESS-PLATFORM-` |
| `executable` | `process_path_resolution` | `satisfied`, `missing`, `not_applicable`, `unsupported`, `unavailable`, `not_observed`, `stale`, `unknown` | `FERRIS-READINESS-EXECUTABLE-` |
| `environment` | `environment_name_presence` | `satisfied`, `missing`, `not_applicable`, `unsupported`, `unavailable`, `not_observed`, `stale`, `unknown` | `FERRIS-READINESS-ENVIRONMENT-` |
| `path` | `repository_path_metadata` | `satisfied`, `missing`, `mismatched`, `not_applicable`, `unsupported`, `unavailable`, `not_observed`, `stale`, `unknown` | `FERRIS-READINESS-PATH-` |

`satisfied`, `missing`, and `mismatched` use the active method. All other
states use `none`. The diagnostic code is the kind prefix followed by the
uppercase status with `_` converted to `-`. Kind, criticality, and requirement
ID MUST exactly match the declaration.

## Readiness report

An output record is `ferris.environment-readiness-report/v1`. Its canonical
schema is
[`ferris.environment-readiness-report.v1.schema.json`](../schemas/environment-readiness/ferris.environment-readiness-report.v1.schema.json).
It is the `record` payload of the existing `ferris.command-result/v2`
envelope. The readiness specialization is
[`ferris.environment-readiness-command-result.v2.schema.json`](../schemas/environment-readiness/ferris.environment-readiness-command-result.v2.schema.json).

The report binds:

- deterministic report identity;
- digest of the exact strict requirement bytes;
- workspace identity;
- observed OS and architecture;
- one observation per requirement;
- one disposition per declared source;
- aggregate status;
- typed unknown and limitation codes; and
- evidence-retention assertions.

Observations retain no executable path, environment value, path metadata
content, command output, username, home directory, or reusable secret. Evidence
records explicitly assert that values, resolved paths, and content were not
retained.

Every V1 source disposition is `declared_only` because V1 has no source
adapters.

## Aggregate and process result

`not_applicable` is non-blocking. Advisory observations never change the
aggregate from `ready`, but remain in the report.

Applicable required observations map to the aggregate and the existing
`ferris.command-result/v2` process contract:

| Required observation | Aggregate | Result class | Exit |
|---|---|---|---:|
| all `satisfied` or `not_applicable` | `ready` | `success` | 0 |
| any `unsupported` and no higher-precedence result | `unsupported` | `unsupported` | 4 |
| any `not_observed` and no higher-precedence result | `incomplete` | `incomplete` | 5 |
| any `stale` and no higher-precedence result | `stale` | `stale` | 6 |
| any `unknown` and no higher-precedence result | `incomplete` | `incomplete` | 5 |
| any `missing`, `mismatched`, or `unavailable` | `blocked` | `blocked` | 7 |

When several required states apply, VIEW-001 precedence remains authoritative:
`stale`, then `blocked`, then `incomplete`, then `unsupported`, then `success`.
The numeric values are stable process codes, not severity ordering.

Invalid or unsupported input fails before a readiness report exists:

- malformed, duplicate-member, structurally invalid, semantically invalid, or
  unsafe input uses `invalid` and exit 2;
- a recognized `ferris.environment-requirements/` version that is not V1 uses
  `unsupported` and exit 4; and
- an unavailable or oversized requirements file uses `blocked` and exit 7.

Report-bearing non-success results retain one typed diagnostic matching the
result class. Pre-report invalid, unsupported, blocked, and internal outcomes
have `record: null` and one typed diagnostic. `internal` is reserved for a
Ferris invariant failure and uses exit 11. Diagnostic messages and next actions
MUST be generic bounded text and MUST NOT include environment values, search
paths, resolved paths, file content, usernames, machine names, or credentials.

## Identity and determinism

Requirements digest is SHA-256 over the exact accepted strict JSON bytes.
Report identity covers the schema, requirements digest, workspace identity,
observed platform, sorted observations, source dispositions, aggregate,
unknowns, and limitations. It excludes wall-clock time and host-specific paths.
The report ID is `report:` followed by lowercase SHA-256 over the UTF-8 JSON
record with `report_id` omitted, object members sorted lexicographically at
every depth, arrays in their required canonical order, no insignificant
whitespace, and JSON string escaping. V1 records contain no JSON numbers, so
numeric canonicalization is not part of this contract.

The same accepted declaration bytes and equivalent observations MUST produce
the same report identity on the same OS and architecture. Array order is canonical:
sources by source ID, requirements and observations by requirement ID, source
dispositions by source ID, and diagnostic-code arrays lexicographically.

The command selection identity binds semantic command `doctor`, workspace ID,
selected manifest identity, and requirements digest. Invocation identity also
binds command version and explicit invocation inputs. Result identity follows
the existing command-envelope identity and therefore binds result class,
process code, diagnostics, and the optional report.

## Conflict and precedence

V1 has no implicit source precedence. Duplicate requirement IDs, duplicate
source IDs, missing source references, contradictory exact requirements, and
non-canonical ordering are invalid declarations.

Future adapter disagreements require a later schema and are not representable
as V1 observations. That later contract MUST preserve both claims and select
`incomplete` when additional safe evidence can resolve the conflict or
`blocked` when owner action is required. Ferris MUST NOT choose a source,
weaken a required requirement, or convert the result to an advisory warning
without explicit owner policy.

## Privacy and secrets

Workspace IDs contain exactly one namespace separator and use lowercase opaque
segments; all other IDs exclude `/`, `\`, `:`, and `@`. IDs MUST be
owner-assigned stable identifiers, not discovered usernames, machine names,
emails, drives, or home paths.

The contract contains names and presence only. It MUST reject fields that
attempt to carry:

- environment values;
- command arguments or output;
- executable or resolved absolute paths;
- credentials, tokens, certificates, or private keys;
- file content or metadata beyond expected kind; or
- user, machine, or home-directory identities.

Environment names associated with credentials MAY be declared because checking
name presence does not reveal a value. The report MUST NOT retain the value or
a digest of the value.

## Compatibility, adoption, and removal

V1 is optional. Existing `doctor` behavior without an explicit requirements
input remains unchanged. Adoption adds only one owner-controlled declaration.
Removal deletes that declaration and any Ferris-only report; Cargo, owner
commands, owner-native environment files, and ordinary IDE or CI behavior
remain unchanged.

Unknown schema versions fail closed. New requirement kinds or source adapters
require a new compatible schema version or separately versioned adapter
contract. They MUST NOT change V1 interpretation.

## Frozen fixtures

The
[`environment-readiness` fixtures](../../tests/fixtures/environment-readiness/)
freeze:

- one complete requirements declaration;
- one ready report plus advisory-missing, not-applicable, required-missing,
  required-mismatched, required-unsupported, required-not-observed,
  required-stale, required-unavailable, and required-unknown state vectors;
- complete success, report-bearing blocked, and pre-report invalid examples
  plus the remaining `ferris.command-result/v2` mappings; and
- exact structural and semantic negative-control mutations.

The fixtures are contract evidence only. They do not prove an implementation,
platform support, or owner-command success.

## Non-goals

READINESS-001 does not define:

- version probing or comparison;
- package, component, or target installation;
- environment activation or repair;
- service, network, port, credential, or capacity probes;
- owner command or validation execution;
- workflow, shell, task, or lifecycle interpretation;
- adapter support for owner-native formats;
- CI equivalence, support certification, or production readiness; or
- automatic Action Plan eligibility.
