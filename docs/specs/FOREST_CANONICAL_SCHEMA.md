# FOREST-002: Query Forest Canonical Schema

Status: Draft after nine-role review
Implementation authority: None
Depends on: FOREST-001, SCOPE-001, APPLICATION-001, BLUE-Q01, and the Crates Series

## Purpose

This specification defines the canonical node, edge, state, observation,
serialization, root, and extension envelope for the Query Forest.

The schema is a product-neutral evidence language. It is not a database,
service, planner, policy engine, cache protocol, or execution authority.

## Schema envelope

Every serialized Forest record MUST identify:

- schema family and version;
- record kind and version;
- record identity;
- producing component and version;
- tenant or isolation domain where applicable;
- creation time;
- source evidence references;
- lifecycle state; and
- limitations.

Canonical records MUST be self-describing enough to reject unsupported schema
versions without guessing.

## Node contract

Every node MUST contain:

- typed node kind;
- identity domain;
- domain-local identity;
- owner;
- applicable scope coordinates;
- lifecycle and evidence states;
- source records;
- observation time and expiry;
- attributes in a versioned namespace; and
- limitations and unknowns.

Node identifiers MUST be stable only within their declared identity domain.
The schema MUST NOT define one universal package, artifact, compatibility,
cache, contract, or application identity.

## Edge contract

Every edge MUST contain:

- typed edge kind;
- source and target node identities;
- direction;
- applicable scope and conditions;
- cardinality where relevant;
- declaring and evidence owners;
- observed, declared, inferred, predicted, proposed, or unknown claim class;
- source evidence;
- confidence;
- observation time and expiry;
- tool or adapter version; and
- limitations.

Edges MUST NOT imply their reverse. Structural, dependency, causality,
production, validation, history, governance, and ownership edges MUST remain
distinguishable.

## Observation contract

An observation MUST identify:

- subject and applicable scope;
- observer and owner boundary;
- command, query, API, file, or protocol used;
- environment;
- start and end time;
- result state;
- raw-evidence reference or digest;
- normalization result;
- diagnostics;
- expiry and renewal trigger; and
- unsupported, unavailable, failed, or unknown dimensions.

Declared owner metadata, direct observation, inference, prediction, proposal,
resolution, execution, and outcome MUST use separate record kinds.

## Change Record

Every triggering change consumed by scope, causality, prediction, validation,
or planning MUST use one canonical Change Record containing:

- change ID, schema, and version;
- prior and new source or owner-state identities;
- declaring owner and discovery method;
- changed subjects and SCOPE-001 coordinates;
- change class, including source body, public API, contract, configuration,
  dependency, feature, target, profile, generated, runtime data, environment,
  native, policy, validation, deployment, deletion, or unknown;
- added, modified, removed, renamed, moved, or unchanged-control operation;
- direct evidence and observation time;
- declared and discovered mappings;
- known and unknown effects;
- limitations; and
- supersession or correction.

One record MAY group an atomic owner change set. Unrelated changes MUST remain
separately selectable. A path diff or source-control label alone MUST NOT
establish semantic impact.

## State vocabulary

The canonical schema MUST support at least:

- planned;
- selected;
- queued;
- running;
- fresh;
- dirty;
- invalidated;
- reused;
- restored;
- rebuilt;
- relinked;
- skipped;
- passed;
- failed;
- cancelled;
- blocked;
- unsupported;
- unavailable;
- stale;
- unknown; and
- not observed.

Record-specific state machines MAY use additional namespaced states. They MUST
map losslessly to a canonical class or remain explicitly extension-only.

## Canonical serialization

Canonical serialization MUST:

- be deterministic for equivalent records;
- define field ordering or canonical encoding;
- define number, timestamp, string, Unicode, and byte semantics;
- reject duplicate or ambiguous fields;
- preserve unknown extension fields where safe;
- identify omitted versus explicit null or unknown values;
- support bounded streaming and size limits; and
- exclude credentials and reusable secrets.

A serialization digest MAY establish byte integrity for one canonical record.
It MUST NOT establish compatibility, trust, freshness, validation, or reuse.

## Immutable Forest root

Every Forest root MUST bind:

- schema version;
- normalized node, edge, observation, and limitation sets;
- source evidence identities;
- scope selection;
- environment and tool versions;
- parent root or roots;
- assembly diagnostics;
- omitted and unknown evidence;
- creation time; and
- retention classification.

Root identity MUST be immutable. A changed canonical record set produces a new
root. Parentage records lineage, not semantic compatibility.

Moving refs, generations, pins, leases, tombstones, and labels MUST remain
outside immutable root content except as observed historical evidence.

## Extensions

Extensions MUST use an owner-qualified namespace and declare:

- schema and version;
- applicable node, edge, or record kinds;
- owner and support lifecycle;
- canonicalization rules;
- compatibility and migration policy;
- validation fixtures; and
- removal behavior.

An extension MUST NOT redefine a canonical field or silently change canonical
state semantics.

## Versioning and migration

Schema evolution MUST classify:

- backward-readable additive change;
- conditionally readable change;
- migration-required change;
- breaking change;
- deprecation; and
- removal.

Migration MUST preserve source references, claim classes, unknowns,
limitations, identities, lineage, and old-schema evidence. Failed migration
MUST retain the original record and produce an explicit diagnostic.

## Acceptance criteria

FOREST-002 may advance to Proposed only when:

1. every FOREST-001 node and edge family has a typed fixture;
2. declared, observed, inferred, predicted, proposed, and unknown claims remain
   distinguishable;
3. canonical serialization is byte-stable across repeated runs;
4. malformed, oversized, ambiguous, unsupported-version, and secret-bearing
   records are rejected;
5. roots are immutable and parent lineage cannot rewrite prior roots;
6. additive, migration-required, breaking, and failed migrations are tested;
7. extensions can be removed without redefining canonical evidence; and
8. all nine roles record a disposition.
