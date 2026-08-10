# IDENTITY-001: Ferris Identity and Lineage Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: CONTRACT-001, APPLICATION-001, SCOPE-001, FOREST-002, and BLUE-Q01

## Purpose

This specification defines identity domains, comparison, derivation,
supersession, immutable roots, typed refs, update generations, retention, and
lineage.

Identity answers which thing a record names. It does not by itself prove
compatibility, trust, integrity, freshness, validation, availability, or
economic reuse.

## Identity domains

Ferris MUST preserve distinct identities for:

- source repository, revision, path, and change set;
- Cargo package source, release, workspace, target, feature selection,
  profile, platform, lock state, and unit;
- rustc invocation, semantic state, incremental generation, compiler query,
  generic instance, codegen unit, and backend work product;
- native source, tool, ABI, object, archive, link plan, linker state, debug
  package, and final artifact;
- Typebook, Rust API, WIT, wire, data, projection, and adapter contracts;
- environment, runtime process, deployment, and provider;
- validation plan, activity, result, capability, and repository gate;
- application definition, Blueprint Model, Application Contract, and profile;
- observation, prediction, plan, resolution, action, execution, and outcome;
- evidence packet and Forest root; and
- human ref, generation, pin, lease, tombstone, and label.

No digest, package version, path, ref name, artifact name, or cache key may
stand in for all domains.

## Identity record

Every identity record MUST include:

- identity domain and schema version;
- owner authority;
- canonical domain-local key;
- source and revision;
- applicable scope and conditions;
- creation or observation time;
- lifecycle state;
- predecessor, successor, or replacement where applicable;
- evidence references; and
- unknowns and limitations.

Identity comparison MUST produce exact-equal, distinct, unresolved, stale,
unsupported, failed, or unknown. Compatibility is evaluated separately under
CONTRACT-001 and PLATFORM-001.

## Lineage

Lineage relationships MUST distinguish:

- derived from;
- parent of;
- generated from;
- assembled from;
- copied from;
- restored from;
- migrated from;
- supersedes;
- replaces;
- promoted from;
- rolled back from; and
- observed in.

A derivation edge MUST name the operation, inputs, tool and environment,
evidence, owner, time, and limitations. Lineage MUST NOT imply bit equality,
semantic equivalence, support, trust, or validation.

## Immutable roots

A Forest root is an immutable identity over one canonical observed evidence
state. Root creation MUST:

- use FOREST-002 canonical serialization;
- retain source evidence and parent roots;
- record scope, environment, tools, omissions, and unknowns;
- reject mutable or secret-bearing inputs; and
- never alter a previously published root.

Missing or collected root material MUST be reported separately from root
identity.

## Typed refs

The canonical ref vocabulary is:

| Type | Rule |
|---|---|
| branch | moving development ref constrained to a lineage |
| tag | write-once published ref |
| channel | policy-controlled promotion ref |
| alias | local convenience ref without support meaning |
| pin | retention decision preserving a root |
| lease | expiring active-use retention claim |
| tombstone | future-resolution denial retaining audit history |
| label | searchable, non-dereferenceable metadata |

Git remains authoritative for source branches and tags. Ferris records an
association rather than creating duplicate source-control authority.

## Ref updates and generations

Every dereferenceable ref update MUST record:

- ref identity and type;
- expected prior root and generation;
- new root;
- actor and authority;
- policy and approval where applicable;
- reason;
- time;
- expiry where applicable; and
- resulting generation.

Updates MUST use compare-and-set semantics. Concurrent or replayed updates with
the wrong prior value MUST fail without losing history.

Tags MUST reject movement. Channels require promotion policy. Aliases remain
local. Tombstones deny future resolution but MUST preserve prior generations
for audit.

## Retention and collection

Reachability MUST account for:

- current refs;
- retained generations;
- pins;
- unexpired leases;
- policy retention;
- legal or audit holds;
- parent lineage requirements; and
- shared roots referenced by another tenant-authorized record.

Age alone MUST NOT authorize collection. Collection removes retained material,
not historical identity or revocation evidence, unless an approved deletion
policy requires otherwise.

## Reuse boundary

A ref or root MAY identify candidate prior evidence. Reuse additionally
requires separate:

- input and environment compatibility;
- integrity and provenance;
- freshness;
- validation;
- capability preservation;
- authorization; and
- restore-benefit economics.

Failure of any check MUST fall back to an owner-native rebuild or named blocked
state.

## Acceptance criteria

IDENTITY-001 may advance to Proposed only when:

1. all listed identity domains remain independently queryable;
2. exact equality cannot be confused with compatibility or trust;
3. derivation, migration, supersession, promotion, and rollback differ;
4. immutable roots reject mutation;
5. branches, tags, channels, aliases, pins, leases, tombstones, and labels have
   positive and negative fixtures;
6. concurrent, replayed, expired, missing, corrupt, revoked, and collected
   states preserve history and safe fallback;
7. ref removal leaves ordinary Git and Cargo operation intact; and
8. all nine roles record a disposition.
