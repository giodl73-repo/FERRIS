# Ferris Action Plan Execution Contract

Status: Implemented subset for GO-WP-003

## Purpose

This contract defines the first bounded `ferris go` proof. It authorizes local,
linear execution of exact owner-declared validation commands. It does not
authorize selection, scheduling, provider APIs, publication, artifact
transport, deployment, or execution of a read-only topology plan.

## Plan and approval files

`ferris go --action-plan <ID>` resolves three files from the current repository:

- `.ferris/action-plans/<ID>.json`;
- the approval named by that Action Plan under `.ferris/approvals/`; and
- the owner entrypoint declaration named by that Action Plan under
  `.ferris/entrypoints/`.

The `.ferris/` layout is a prototype implementation detail, not a product
concept or user-managed store.

IDs are lowercase `sha256:<hex>` content identities. A file whose computed
identity differs from its filename or embedded identity is stale and MUST NOT
execute. To avoid a circular digest, the Action Plan identity projection
excludes `approval_id`, while the approval identity projection includes
`action_plan_id`. Changing the selected approval therefore does not change the
plan ID, but the loaded approval's identity and exact plan binding MUST both
validate before launch. The Action Plan binds the repository identity,
declaration identity, source revision, topology identity, owner gate identities,
command identities, working directories, environment-name allowlists, file
identities, limits, and ordered lanes.

The repository root is the current directory after canonicalization. All plan,
approval, declaration, working-directory, executable, and bound-file paths are
repository-relative and MUST remain inside that root after canonicalization.

## Owner entrypoints

An entrypoint declaration is `ferris.owner-entrypoints/v1`. Each entrypoint
contains:

- a stable owner reference;
- an executable repository-relative path;
- structured argv;
- a repository-relative working directory;
- an allowlist of inherited environment variable names; and
- a credential class.

The initial executor accepts only credential class `none`. It never invokes a
shell and never interprets a command string. The Action Plan repeats the exact
entrypoint identity; any declaration or command drift blocks execution.

## Approval

An approval file is `ferris.execution-approval/v1`. It binds the exact Action
Plan identity, principal, allowed environment names, expiry, and revocation
state. Expired, revoked, mismatched, or identity-invalid approvals block before
launch. Ferris does not create or authenticate approvals in GO-WP-003; the
repository owner supplies the approval file.

## Execution

An Action Plan is `ferris.action-plan/v1`. Lanes execute once, in declared
order. Dependencies may reference only earlier selected lanes. A failed,
timed-out, cancelled, or blocked dependency produces a typed
`blocked_by_dependency` result rather than omission.

After cancellation is observed, Ferris terminates the active process tree and
does not launch another lane. Unlaunched lanes with a non-successful dependency
become `blocked_by_dependency`; other unlaunched lanes become `cancelled`.

Each launched process receives:

- the exact owner executable and argv;
- the exact declared working directory;
- an empty environment plus only approved inherited names;
- a bounded stdout and stderr capture; and
- the declared timeout.

Ferris owns the launched process tree. On timeout, cancellation, output
overflow, or capture failure, the whole owned tree is terminated before the
lane becomes terminal. A cleanup state other than `complete` prevents overall
success.

Environment values are never serialized. Each lane records only a deterministic
identity over its present allowlisted names and value digests so later replay
cannot equate materially different execution environments. If a captured stream
contains an inherited value, Ferris redacts the value, records `leaked_secret`,
terminates the lane, and does not persist the unredacted bytes. Empty and
trivially short values are not treated as redaction tokens and environment names
commonly associated with credentials are rejected by this credential-free
slice.

## Receipt

Every invocation emits `ferris.execution-receipt/v1`. Each selected lane has
exactly one terminal result:

- `succeeded`;
- `failed`;
- `timed_out`;
- `cancelled`;
- `blocked_by_dependency`;
- `output_limit_exceeded`;
- `leaked_secret`; or
- `internal_error`.

The receipt binds the repository, source, actual operating-system and
architecture pair, topology, owner gate, entrypoint, and non-secret environment
identity. Results preserve owner exit codes, bounded redacted diagnostic tails,
output digests, elapsed milliseconds, and cleanup state. The aggregate succeeds
only when every required lane succeeded and every cleanup completed. An observed
cancellation produces aggregate `cancelled` and the fixed cancelled process
code.

The receipt identity covers immutable lineage and semantic results, excluding
elapsed time. `ferris verify <RECEIPT>` validates strict structure, content
identity, lane completeness, dependency-terminal consistency, and aggregate
classification. Verification does not rerun commands or authenticate who
created the receipt.

## Explicit non-goals

GO-WP-003 does not implement:

- `go --changed` or `go --full`;
- changed-path or package selection;
- parallel or historical scheduling;
- retries;
- external gates or provider APIs;
- secret injection;
- network, filesystem, or container sandboxing;
- artifact transfer or cache reuse;
- signing, publication, promotion, or deployment; or
- consumer-repository modification.
