# Wave: Federated Application Plan

Status: Complete
Implementation authority: One bounded pulse
Successor authority: None

## Frame

Working owner system:

- Cargo resolves, locks, and reports each workspace independently;
- FERRIS already emits portable non-executable `ferris.blueprint-plan/v0`
  records for one workspace;
- PARLOR and RUNE prove that exact read-only projections can remain
  consumer-owned.

Missing product capability:

FERRIS claims to be cross-workspace, but every shipped planning command accepts
one Cargo manifest. A user coordinating several independently locked
workspaces must invoke FERRIS repeatedly and manually reconstruct one
application-level view.

Thesis:

```text
independent Cargo workspace plans
+ one bounded consumer-authored federation request
-> one portable non-executable application-level plan
```

The deletion target is manual invocation and collation of one-workspace plan
records. The thesis is disproved if the slice needs a shared Cargo resolution,
cross-workspace dependency inference, execution, repository mutation, hidden
discovery, absolute paths in output, or a broad Application Definition schema.

## Approved V0 slice

Add one `federated-plan` command:

```console
ferris federated-plan --request <REQUEST_JSON> --format json
```

The strict bounded request contains:

- schema `ferris.federated-plan-request/v0`;
- portable application ID, revision, and owner;
- 2-16 explicit workspace records;
- one portable workspace ID and one request-relative Cargo manifest path per
  workspace;
- manifest paths use forward slashes, contain no `..`, and remain below the
  canonical request parent;
- Cargo's reported workspace root also remains below the canonical request
  parent, which must be a common ancestor for every complete selected
  workspace;
- root and member manifests from one Cargo workspace are not independent; and
- revision and owner use 1-256 ASCII bytes with interior spaces allowed but no
  leading/trailing spaces or control characters.

The result:

- uses schema `ferris.federated-plan/v0`;
- is explicitly non-executable;
- retains one unchanged portable `ferris.blueprint-plan/v0` record per
  workspace;
- sorts workspaces deterministically by portable workspace ID;
- links but never flattens independent Cargo evidence;
- retains unknowns and limitations at both levels; and
- contains no request path, absolute workspace path, lock digest or identity,
  or inferred relationship.

Cargo metadata runs exactly once per requested workspace and runs
sequentially. Each invocation has a 30-second timeout and separate 4 MiB
stdout and stderr bounds. Its captured bytes supply both canonical Cargo
workspace-root duplicate validation and the retained Blueprint Plan.
At the maximum cardinality, the sequential timeout ceiling is 480 seconds
plus owner-process startup and cleanup. Termination covers the direct Cargo
child, not descendants of a custom wrapper. The common-ancestor syntax cannot
group workspaces on different Windows drives.

## Ownership boundaries

- Cargo owns membership, packages, resolution, lockfiles, features, targets,
  and metadata in each workspace.
- The request author owns application identity and the decision to group the
  workspaces.
- FERRIS owns bounded parsing, validation, orchestration, portable identity,
  and presentation.
- Repository validation, contracts, services, native relationships, support,
  and lifecycle remain owner-defined and unmodeled in V0.

## Compare

| Analogue | Classification | Decision |
| ---- | ---- | ---- |
| Existing `plan` and `PlanRecord` | reuse | Preserve it unchanged per workspace |
| Cargo `--manifest-path` and `metadata` | reuse | Invoke once per independent workspace |
| APPLICATION-001 full definition | avoid for V0 | Too broad; this request is not the canonical Application Definition |
| PLANNING-001 federated DAG | adapt | Link workspace plans without claiming owner closures not observed |
| Bazel-style unified graph | avoid | Would replace independent Cargo resolution and lock boundaries |
| Repeated shell invocations | delete | Replace manual collation with one typed result |

Cargo's workspace documentation defines a workspace as a set of packages that
share a `Cargo.lock` and output directory. FERRIS therefore rejects multiple
selected manifests that Cargo reports under one canonical workspace root and
must not pretend that independent workspaces are one Cargo workspace. Cargo
metadata's explicit manifest selection is the owner seam reused here. The
existing PlanRecord does not retain a lock digest, so the federation preserves
separate invocation and workspace boundaries without claiming lock identity.

## Role evaluation

### Product Value Governor

Continue within one pulse. This closes the largest contradiction between the
product statement and shipped surface with one user-visible command.

### Rust Safety Steward

Accept. The command reads bounded JSON and Cargo metadata, performs no
execution beyond read-only owner discovery, and adds no unsafe code.

### Compiler Performance Engineer

Accept without a speed claim. Metadata runs remain independent and sequential;
parallelism, caching, and affected-only reduction are out of scope.

### Interop Boundary Auditor

Accept if each nested workspace record preserves its own Cargo evidence and no
shared lock or dependency graph is invented.

### AI Assurance Skeptic

Require strict JSON, duplicate rejection, portable output, deterministic
ordering, explicit unknowns, and negative fixtures for one workspace,
duplicates, unknown fields, and invalid manifests.

### Ecosystem Strategist

Accept. This builds above Cargo rather than replacing it and makes the public
cross-workspace category materially true for read-only planning.

### Rust Maintainer

Accept one command and two V0 schemas. Reuse `PlanRecord`; do not introduce a
generic plugin system, public library target, or full application model.

### Native Platform Adopter

Require request-relative path resolution and path-free portable output on
Windows and Unix. Do not infer native or target relationships.

### Scope Keeper

Accept only federation. Components, contracts, validation composition,
relationships, profiles, governance, MCP, execution, and mutation remain out.

### Validation Checker

Require one accepted two-workspace result, one structured invalid request,
identity stability across relocated fixtures, CLI parity, formatting, and
workspace tests.

### Autonomy Supervisor

Authorize one implementation pulse. Stop after proof and review; adoption,
documentation/deck revision, or richer application semantics require separate
evidence.

## Completion condition

- strict bounded request and portable federated result;
- two independent fixture workspaces with separate lock boundaries;
- accepted and structured failure controls;
- stable identity after relocating the complete fixture;
- direct `ferris`, direct `cargo-ferris`, and Cargo-style parity;
- focused and workspace validation;
- all-eleven-role closeout; and
- no successor.

The measured implementation result is recorded in
[`Pulse 01`](pulses/pulse-01.md). The all-eleven-role closeout is recorded in
the
[Federated Application Plan review](../../../docs/plans/reviews/FERRIS-FEDERATED-APPLICATION-PLAN-REVIEW.md).

## Removal

Delete the command, its request/result types, tests, fixtures, and this wave.
Existing one-workspace commands, Cargo manifests, lockfiles, consumer
contracts, and owner workflows remain unchanged.
