# Ferris Action Plan Execution Contract

Status: Implemented subset for GO-WP-003 plus bounded explicit preparation

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

## Preparation

`ferris prepare-action-plan` is a non-executable preparation boundary over the
existing V1 records. Its direct mode accepts one explicit
`ferris.owner-entrypoints/v1` file, selects exactly one named entrypoint, and
requires the caller to provide the lane ID, owner gate ID, repository ID,
topology ID, required status, timeout, and both output bounds.

Its multi-lane mode additionally accepts one strict
`ferris.action-plan-lanes/v1` record. That record supplies repository and
topology identity plus an ordered non-empty lane list. Every lane explicitly
names an entrypoint, lane ID, owner gate, required status, dependencies,
timeout, and both output bounds. Dependencies may name only earlier lanes and
lane IDs must be unique. Ferris does not derive any of those values.

Direct single-lane mode may replace the explicit entrypoint argument with one
complete repository-local `ferris.failure-policy-decision/v1` command result.
Ferris validates the decision record and outer command-result identities,
requires disposition `prepare_action`, and uses `owner_action_id` only as the
entrypoint selector. `halt` and `route` decisions are not preparable. Every
other lane policy value remains explicit.

Before writing, preparation validates the declaration and entrypoint content
identities, current Git revision, repository-local working directory, and every
bound file. A supplied failure decision is bounded, repository-local, and
rechecked for byte equality before commit. Ferris copies each exact declared
command into the explicitly ordered lanes, computes the existing Action Plan
identity, and atomically creates the requested repository-local output without
overwriting an existing file.
Repeated calls over unchanged inputs produce byte-identical output. Direct
mode remains a one-lane shorthand with no dependencies.

The prepared plan has an empty `approval_id`. This field is excluded from the
Action Plan identity projection, but `ferris go` rejects it until an owner
supplies a valid independent approval and binds that approval ID. Preparation
does not inspect environment values, launch a process, create an approval,
discover commands, stage executables, or infer dependencies. Multi-lane plan
policy remains owner-authored; Ferris only validates and compiles it.
Action Plan V1 binds the selected entrypoint but has no failure decision field;
the owner must retain the validated decision separately for provenance.

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
`blocked_by_dependency` result rather than omission. The receipt MUST preserve
both the full declared dependency list and, for a blocked lane, the exact subset
of earlier dependencies whose terminal status was not `succeeded`. It MUST also
preserve the transitive root blockers: earlier non-blocked lanes at which each
dependency failure chain terminated.

After cancellation is observed, Ferris terminates the active execution
containment and
does not launch another lane. Unlaunched lanes with a non-successful dependency
become `blocked_by_dependency`; other unlaunched lanes become `cancelled`.

Each launched process receives:

- the exact owner executable and argv;
- the exact declared working directory;
- an empty environment plus only approved inherited names;
- a bounded stdout and stderr capture; and
- the declared timeout.

Ferris owns the launched process under a platform-specific containment
primitive. Windows uses a Job Object and terminates processes assigned to that
job. Unix uses a dedicated process group and signals that group; owner commands
must not daemonize, call `setsid`, or otherwise escape the group. Other
platforms guarantee only direct-child termination. On timeout, cancellation,
output overflow, or capture failure, Ferris applies that primitive and waits
for the direct child and captured streams before the lane becomes terminal. A
cleanup state of `complete` means that operation completed; it does not claim
containment beyond the platform primitive. A cleanup state other than
`complete` prevents overall success.

Environment values are never serialized. Each lane records only a deterministic
identity over its present allowlisted names and value digests so later replay
cannot equate materially different execution environments. If a captured stream
contains an inherited value, Ferris redacts the value, records `leaked_secret`,
terminates the lane, and does not persist the unredacted bytes. Empty and
trivially short values are not treated as redaction tokens and environment names
commonly associated with credentials are rejected by this credential-free
slice.

## Receipt

Every invocation emits `ferris.execution-receipt/v2`. Each selected lane has
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
output digests, elapsed milliseconds, cleanup state, declared dependencies, and
the dependency identities that blocked any unlaunched lane. A blocked lane's
`blocked_by` field is the exact ordered subset of its `depends_on` field whose
terminal status was not `succeeded`. Its `root_blocked_by` field is the unique
set of non-blocked terminal lanes reached by expanding those blockers, ordered
as those lanes appear in the receipt. This is structural dependency evidence,
not an inference about the semantic root cause of an owner-command failure.
The aggregate succeeds only when every
required lane succeeded and every cleanup completed.
An observed cancellation produces aggregate `cancelled` and the fixed cancelled
process code.

Ferris continues to verify and replay legacy `ferris.execution-receipt/v1`
records. V1 derives blocked-lane causality from `depends_on` and earlier lane
statuses; V2 materializes that same causality in `blocked_by`. New receipts are
V2 and also materialize transitive origins in `root_blocked_by`; V1 receipts
cannot carry either V2 field.

The receipt identity covers immutable lineage and semantic results, excluding
elapsed time. `ferris verify <RECEIPT>` validates strict structure, content
identity, lane completeness, dependency-terminal consistency, blocked-lane
causality, and aggregate classification. Verification does not rerun commands
or authenticate who created the receipt.

`go` and `verify` preserve JSON as their default output for automation. With
`--format human`, both commands render every lane terminal state, direct
blockers, transitive root blockers and their terminal states, cleanup state,
owner exit code when present, and bounded redacted diagnostic tails. Human
verification renders only after the complete receipt has passed the same
structural and identity checks as JSON verification.

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

The preparation command additionally does not implement PATH-resolved
executables or interpreter discovery. V1 declarations still require a
repository-relative executable included in the command's file bindings.

An environment-readiness executable observation cannot fill this gap. Its V1
report intentionally retains neither the selected path nor executable content
identity, so it supplies presence evidence rather than execution authority.
Supporting platform-local tools requires a separately versioned execution
contract; it MUST NOT reinterpret Action Plan V1 fields.
