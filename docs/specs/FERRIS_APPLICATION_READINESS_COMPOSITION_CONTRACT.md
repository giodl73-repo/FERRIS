# APP-READINESS-001: Ferris Application Readiness Composition Contract

Status: Implemented V1 under Environment Readiness Pulse 07
Implementation authority: Exhausted
Depends on: READINESS-001 and APPLICATION-001

## Purpose

This specification defines a passive, fail-closed composition of existing
workspace READINESS-001 observations across one explicit multi-workspace
application. It prevents one selected Cargo workspace report from being
presented as application-wide readiness.

Pulse 06 froze this contract, its schemas, fixtures, and review. Pulse 07
implemented the optional explicit-file composition path. Neither pulse
authorizes adopter changes.

## Authority

The repository owner remains authoritative for the Application Definition and
each READINESS-001 declaration. Cargo remains authoritative for every
independent workspace. APP-READINESS-001 only checks explicit identities and
paths and composes typed readiness results.

Ferris MUST NOT:

- discover an application, workspace, or manifest through Cargo;
- invoke Cargo, a candidate executable, or an owner command;
- merge independent Cargo workspaces or lock boundaries;
- infer a requirement from an application definition, manifest, command,
  workflow, source file, or prior failure;
- reinterpret a workspace-root-relative V1 path as application-root-relative;
- conceal a non-ready workspace behind another workspace's ready result; or
- claim owner-command success, CI equivalence, support, or production
  readiness.

V1 `unknowns` MUST be empty. V1 `limitations` MUST contain exactly
`FERRIS-APPLICATION-READINESS-WORKSPACE-ONLY`, which states that the aggregate
does not observe application-root requirements.

Invalid inputs and inconsistent aggregates use these stable diagnostics:

| Code | Condition |
|---|---|
| `FERRIS-APPLICATION-READINESS-APPLICATION-ID-MISMATCH` | Request and Application Definition IDs differ |
| `FERRIS-APPLICATION-READINESS-DEFINITION-DIGEST-MISMATCH` | Application Definition bytes do not match the request |
| `FERRIS-APPLICATION-READINESS-WORKSPACE-COVERAGE-MISMATCH` | Request and definition workspace sets differ |
| `FERRIS-APPLICATION-READINESS-MANIFEST-MISMATCH` | A workspace manifest path differs across records |
| `FERRIS-APPLICATION-READINESS-WORKSPACE-ID-MISMATCH` | Requirements and request workspace IDs differ |
| `FERRIS-APPLICATION-READINESS-REQUIREMENTS-DIGEST-MISMATCH` | Requirements bytes do not match the request |
| `FERRIS-APPLICATION-READINESS-WORKSPACE-ROOT-DUPLICATE` | Two entries resolve to one workspace root |
| `FERRIS-APPLICATION-READINESS-WORKSPACE-ROOT-NESTED` | One workspace root contains another |
| `FERRIS-APPLICATION-READINESS-WORKSPACE-RESULT-DUPLICATE` | An aggregate repeats a workspace result |
| `FERRIS-APPLICATION-READINESS-AGGREGATE-INCONSISTENT` | Aggregate status does not match exact precedence |

## Request

An input record is `ferris.application-readiness-request/v1`. Its canonical
schema is
[`ferris.application-readiness-request.v1.schema.json`](../schemas/application-readiness/ferris.application-readiness-request.v1.schema.json).

The directory containing the request is the application root. The referenced
Application Definition MUST be a direct child of that root, so its existing
manifest-path base and the request root are identical. The request contains:

- one portable application ID;
- one application-definition path and exact byte digest;
- two to sixteen workspace entries; and
- no command, environment value, resolved path, or private machine identity.

Each workspace entry contains one portable workspace ID, one application-root-
relative Cargo manifest path, one application-root-relative READINESS-001
declaration path, and the declaration's exact byte digest. Entries MUST be
sorted by `workspace_id`.

All paths use `/`, MUST be relative files, and MUST NOT contain empty, `.`, or
`..` components. Components MUST NOT end in `.` or a space and MUST NOT be
Windows reserved device names. Application-root selection is lexical first and
then component-wise filesystem validation. Symlinks and Windows reparse points
are unsupported and MUST NOT be followed.

The request, application definition, and every requirements file are
independently limited to 1 MiB. JSON loading MUST reject duplicate members,
unknown members, malformed input, explicit nulls where absent values are
required, and unsupported schema versions.

## Application binding

The referenced application definition MUST be a strict
`ferris.application/v0` record. Loading for readiness composition MUST be
passive and MUST NOT call the existing resolved-application loader or Cargo
metadata.

The request `application_id` MUST equal the definition `application_id`.
Application workspace IDs and manifest paths MUST be unique. The request MUST
cover every definition workspace exactly once and MUST NOT add a workspace.
Each request manifest path MUST exactly equal the corresponding definition
manifest path.

Workspace roots MUST be distinct and MUST NOT be prefixes of one another.
Uniqueness and non-nesting MUST be enforced on validated filesystem identity,
not only portable spelling. Two entries resolving to the same file or root are
invalid, including platform-equivalent spellings.

The digest is `sha256:` followed by lowercase SHA-256 over the exact bounded
file bytes. A pre-observation mismatch is invalid input. The definition and
request bytes MUST be read again after workspace observations. A changed or
unreadable revalidation produces a typed stale aggregate rather than a ready
result.

## Workspace binding and observation

Every referenced requirements file MUST conform to
`ferris.environment-requirements/v1`. Its `workspace_id` MUST equal the request
entry, and its exact byte digest MUST equal `requirements_digest`.

Each `manifest_path` MUST name the workspace-root `Cargo.toml`; its parent is
the workspace observation root. A member-package manifest is invalid input.
Ferris MUST NOT use Cargo to verify membership, so this is an explicit owner
assertion checked against the Application Definition and the distinct,
non-nested root invariants. No path requirement from that workspace's
READINESS-001 declaration may resolve outside the validated workspace root.

Ferris MUST apply the unchanged READINESS-001 algorithms separately to each
workspace. It MUST preserve one complete workspace report per entry. The
application result references, but does not replace, those reports.

Workspace observations MUST be ordered by `workspace_id`. The environment is
observed independently for each declaration even when requirements repeat;
the contract defines no cache or shared-success inference.

## Aggregate report

An output record is `ferris.application-readiness-report/v1`. Its canonical
schema is
[`ferris.application-readiness-report.v1.schema.json`](../schemas/application-readiness/ferris.application-readiness-report.v1.schema.json).

The report binds:

- a deterministic application-readiness report ID;
- the exact request digest;
- the application ID;
- current or stale composition-input status;
- one result reference per workspace;
- the fail-closed aggregate status; and
- typed unknowns and limitations.

Each workspace result reference MUST exactly match the corresponding
READINESS-001 report's workspace ID, requirements digest, report ID, and
aggregate status. References MUST be sorted by `workspace_id` and cover the
request exactly once.

The application aggregate uses this precedence:

1. stale composition input or any `stale` workspace;
2. any `blocked` workspace;
3. any `incomplete` workspace;
4. any `unsupported` workspace;
5. otherwise `ready`.

No majority, advisory override, or ready workspace can lower a higher-
precedence result. The aggregate is a readiness composition only. It does not
assert that application-root files, cross-workspace behavior, dependencies,
services, resources, credentials, or owner commands are ready.

## Result and exit mapping

The optional `doctor --application-readiness <REQUEST_JSON>` integration uses
the existing VIEW-001 mapping:

| Aggregate | Result | Exit |
|---|---|---:|
| `ready` | `success` | 0 |
| `unsupported` | `unsupported` | 4 |
| `incomplete` | `incomplete` | 5 |
| `stale` | `stale` | 6 |
| `blocked` | `blocked` | 7 |

Invalid input before an admissible aggregate is `invalid`, exit 2, without an
application-readiness report. Internal invariant failure is `internal_error`,
exit 11.

## Identity and determinism

`request_digest` is SHA-256 over the exact accepted request bytes.
`report_id` is `application-readiness:` plus lowercase SHA-256 over canonical
JSON for the complete report with `report_id` omitted.

Canonical JSON uses UTF-8, lexicographically sorted object keys, array order as
specified by this contract, no insignificant whitespace, and no trailing
newline. Identical accepted bytes, child report references, and composition
state MUST produce byte-identical reports.

`workspace_results` MUST be sorted by `workspace_id`. `unknowns` and
`limitations` MUST be sorted lexicographically.

Absolute paths MUST NOT enter either identity. A future invocation identity MAY
bind exact platform-native request-path argument bytes under VIEW-001, but that
path identity MUST remain separate from the portable report identity.

## Stale and failure semantics

`composition_status` is `current` only when the exact request and application
definition bytes remain readable and unchanged after all child observations.
Otherwise it is `stale`, `aggregate_status` MUST be `stale`, and
`diagnostic_code` MUST be `FERRIS-APPLICATION-READINESS-INPUT-STALE`.

When composition input is current and any workspace report is stale,
`aggregate_status` MUST be `stale` and `diagnostic_code` MUST be
`FERRIS-APPLICATION-READINESS-WORKSPACE-STALE`. For other current composition,
the code is `FERRIS-APPLICATION-READINESS-CURRENT`.

Workspace declaration changes retain READINESS-001 stale behavior in the child
report. A missing, malformed, mismatched, or unsupported input detected before
observation is invalid input, not a fabricated missing or ready report.

## Privacy

The request and report MUST NOT retain:

- absolute application, manifest, requirements, or executable paths;
- environment values or search-path values;
- resolved executable paths;
- usernames, machine names, home directories, or temporary directories;
- file content other than owner-authored portable input; or
- credentials, tokens, connection strings, or reusable secrets.

The report retains portable IDs, non-reversible digests, child report IDs,
typed states, and diagnostic codes only.

## Compatibility, removal, and limits

READINESS-001 V1 is unchanged. Applications with one workspace continue to use
ordinary workspace readiness and are outside this V1 composition contract.
Removing APP-READINESS-001 records restores independent workspace readiness
without changing Cargo manifests, lockfiles, owner commands, application
definitions, or requirements declarations.

V1 intentionally does not observe application-root-relative paths. Such a
feature requires a separately versioned owner declaration; a workspace V1
declaration MUST NOT be repurposed. V1 implements only the optional existing
`doctor` CLI and command envelope. It does not implement a cache, parallel
scheduler, source adapter, installer, repair action, or support policy.

## Conformance

The frozen fixtures are under
[`tests/fixtures/application-readiness`](../../tests/fixtures/application-readiness/).
Conformance requires:

- valid request and mixed ready/blocked aggregate acceptance;
- stale input precedence;
- duplicate-workspace rejection;
- traversal rejection;
- application and workspace identity mismatch rejection;
- definition workspace coverage;
- declaration digest binding;
- deterministic report identity; and
- privacy-field exclusion.

Schema validity alone is insufficient for semantic controls.
