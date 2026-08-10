# FOREST-003: Query Forest Projection and Consistency Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: FOREST-002, IDENTITY-001, and EVIDENCE-001

## Purpose

This specification defines maps, ledgers, projection identity, consistency,
snapshot isolation, reproducibility, and failure behavior over canonical Query
Forest roots.

Maps answer bounded structural questions. Ledgers answer identity, lifecycle,
coverage, trust, and accounting questions over time. Neither is an independent
source of truth.

## Projection request

Every projection request MUST identify:

- immutable Forest root;
- projection kind and schema version;
- question or view;
- selected SCOPE-001 coordinates;
- filters and ordering;
- policy-visible exclusions;
- projection engine and version;
- resource budget; and
- requested output format.

Projection identity MUST include all inputs that can change the result.

## Projection result

Every result MUST include:

- projection identity;
- source root;
- canonical query parameters;
- engine and schema versions;
- creation time;
- selected and omitted scope;
- records returned;
- unknown, stale, unsupported, or unavailable evidence;
- truncation or budget fallback;
- consistency diagnostics; and
- source links for every material claim.

A projection MUST NOT mutate the source root or owner systems.

## Maps

The first schema MUST support:

- Scope Map;
- Dependency Map;
- Change Map;
- Invalidation Map;
- Demand Map;
- Critical-Path Map;
- Environment Map;
- Capability Map;
- Ownership Map; and
- Interop Boundary Map.

Maps MUST retain edge direction, scope, condition, owner, claim class, source,
confidence, time, expiry, and limitations.

## Ledgers

The first schema MUST support:

- Scope Mapping Ledger;
- Identity Ledger;
- Reuse Ledger;
- Artifact Ledger;
- Input and Side-Effect Ledger;
- Cost Ledger;
- Validation Coverage Ledger;
- Capability Ledger;
- Provenance and Trust Ledger;
- Assurance Ledger;
- Crate Ecosystem Ledger;
- Lineage Ledger; and
- Adoption and Operations Ledger.

Ledgers MUST preserve event time, observation time, owner, scope, source,
generation, lifecycle, expiry, supersession, and retained history.

## Consistency rules

For one root and projection version:

1. repeated equivalent requests MUST produce equivalent canonical results;
2. every projected node and edge MUST resolve to canonical source records;
3. every material aggregate MUST identify its contributing records;
4. selected scope MUST be a subset of or explicitly widened from the declared
   request;
5. omitted records MUST retain a reason;
6. map and ledger representations of the same identity or relationship MUST
   agree or report a conflict;
7. unknown, stale, unsupported, failed, and not-observed states MUST not become
   success through aggregation;
8. predictions MUST not appear as observations;
9. mutable ref movement MUST not change a result already bound to an immutable
   root; and
10. cross-tenant records MUST never join without explicit authorized scope.

## Snapshot and generation behavior

A projection runs against one immutable root or an explicit ordered root set.
Evidence arriving during projection MUST create or contribute to a later root.

Ref generations MAY select a root before execution. The resolved root and
generation MUST be retained in the projection request so later ref movement
cannot change the result.

Time-series ledgers MAY span roots. They MUST define root order, deduplication,
late-arrival, correction, supersession, and deletion semantics.

## Aggregation

Aggregates MUST state:

- measure and unit;
- population and exclusions;
- grouping dimensions;
- missing and unknown treatment;
- weighting;
- time window;
- variance or uncertainty where applicable; and
- source records.

Composite safety, trust, quality, portability, compatibility, maintenance, or
approval scores are prohibited unless a later specification defines the
individual claims, policy authority, and non-lossy drill-down.

## Failure and fallback

Projection failures MUST distinguish:

- unsupported projection or schema;
- missing root;
- incomplete evidence;
- invalid source record;
- inconsistency;
- budget exhaustion;
- permission denial;
- stale input;
- engine failure; and
- unknown.

Fallback MAY return a coarser projection if the omitted detail and consequence
are explicit. It MUST NOT return a success-shaped partial result.

A material inconsistency MUST make the affected projection ineligible for
planning, resolution, trust, approval, or action. A coarser fallback is
eligible only when it:

- is derived independently of the inconsistent result;
- excludes the conflicting records or dimensions explicitly;
- preserves the consequence of the omission; and
- does not convert unknown, failed, or conflicting evidence into success.

Conflicting canonical owner evidence is an incomplete or blocked evidence
condition. A projection engine result that contradicts canonical source
records, violates an equivalent-request invariant, or disagrees with another
required representation without reporting the conflict is an internal
projection invariant violation.

## Portability and removal

Canonical projection fixtures MUST be engine-independent. A replacement engine
must reproduce required canonical results or declare an explicit,
reviewed version difference.

Removing a projection engine or view MUST NOT remove canonical roots or make
ordinary Cargo, owner tools, or raw evidence inaccessible.

## Acceptance criteria

FOREST-003 may advance to Proposed only when:

1. every initial map and ledger has a canonical fixture;
2. repeated projections are deterministic;
3. all projected claims trace to canonical source records;
4. cross-map and cross-ledger consistency violations are detected;
5. ref movement, concurrent evidence, late arrival, correction, supersession,
   and deletion preserve snapshot rules;
6. truncation, missing evidence, permission failure, and engine failure cannot
   produce silent partial success;
7. two independent projection implementations reproduce the required fixture
   subset or report reviewed differences; and
8. all nine roles record a disposition.
