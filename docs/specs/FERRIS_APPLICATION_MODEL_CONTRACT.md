# APPLICATION-001: Ferris Application Model Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: PRODUCT-001, CONTRACT-001, PLATFORM-001, BLUE-Q01, and GOVERNANCE-001

## Purpose

This specification defines the consumer-authored Application Definition, the
normalized Blueprint Model, and the resolved FERRIS Application Contract.

## Three-record rule

Ferris MUST retain three distinct records:

1. **Application Definition:** durable consumer intent.
2. **Blueprint Model:** normalized intent joined with owner truth and evidence.
3. **FERRIS Application Contract:** resolved and validated support state.

The records MUST have separate identity, schema, owner, lifecycle, and
authority.

A Blueprint Model MUST NOT silently rewrite the Application Definition.
A FERRIS Application Contract MUST NOT conceal unresolved or unsupported
inputs.

## Application Definition

An Application Definition MUST identify:

- application ID, revision, owner, and lifecycle state;
- repositories and source-selection rules;
- Cargo workspaces and expected lock boundaries;
- components, services, tools, libraries, jobs, tests, packages, and
  deployment units;
- relationships and containment;
- Typebook contracts and operations;
- profile requirements and selections;
- providers and alternatives;
- platforms, targets, runtimes, native prerequisites, and deployment
  environments;
- activities and validation obligations;
- artifacts and packaging intent;
- policy, governance, approval, and data classifications;
- support, renewal, removal, and rollback intent; and
- explicit unknown, optional, deferred, and excluded scope.

The definition MAY be projected into Cargo metadata or separate files. The
canonical schema MUST be versioned independently of any one projection.

## Cargo integration

Cargo remains authoritative for:

- workspace membership;
- package source and identity;
- dependency requirements and resolution;
- lockfile;
- features;
- targets;
- profiles;
- platform conditions; and
- build-unit construction and freshness.

Ferris MUST consume supported Cargo metadata and command evidence. It MUST NOT
reimplement the Cargo resolver or create a competing dependency manifest.

The Application Definition MAY add relationships Cargo does not own, including
services, contracts, deployments, policies, validation families, and
cross-workspace application containment.

Independent workspaces MUST retain independent Cargo resolution and lock
identity.

## Blueprint Model normalization

Normalization MUST be deterministic for identical:

- Application Definition;
- repository and source state;
- Cargo metadata and lock state;
- contracts;
- profiles;
- connector evidence;
- governance policy;
- tool and schema versions; and
- explicit environment selection.

The Blueprint Model MUST retain:

- all Application Definition declarations and their source locations;
- discovered owner records;
- normalized identifiers;
- Cargo graph truth;
- active closures by target and activity;
- contract, profile, provider, native, validation, governance, and lifecycle
  joins;
- conflicts and precedence;
- unknown, stale, unsupported, and unavailable records;
- evidence sources, confidence, and expiry; and
- normalization diagnostics.

Normalization MUST NOT convert missing declarations or failed discovery into
defaults that reduce required work.

## Components and mappings

Components MAY represent:

- Cargo packages and targets;
- non-Cargo native components;
- generated-code systems;
- services and processes;
- test and validation families;
- deployable artifacts;
- contracts and providers; and
- external systems.

Every component mapping MUST identify:

- declaring owner;
- discovered owner;
- subject;
- relationship type;
- cardinality;
- condition;
- applicable activity, configuration, platform, and lifecycle;
- source evidence;
- confidence; and
- fallback.

SCOPE-001 defines the complete coordinate and mapping algebra.

## Conflict and precedence

The model MUST distinguish:

- consumer declaration;
- owner-system truth;
- discovered evidence;
- profile policy;
- governance policy;
- inferred relationship; and
- AI proposal.

Owner-system truth prevails for owner-local facts. Governance policy may deny
an otherwise valid action but MUST NOT rewrite owner facts.

Conflicts MUST produce diagnostics naming both sources, affected scope,
authority, required decision, and conservative fallback.

AI proposals MUST NOT establish contract, profile, support, compatibility,
governance, or owner truth without deterministic policy or approval.

## Roots and references

An immutable application root MUST bind or reference:

- Application Definition identity;
- Blueprint Model identity;
- source snapshot and Cargo lock identities;
- contract and profile identities;
- toolchain, platform, provider, native, environment, and governance inputs;
- validation plans and outcomes;
- action and evidence records;
- limitations and unknowns; and
- parent root or roots.

Root identity MUST be immutable.

Typed refs MUST follow BLUE-Q01:

- branch;
- tag;
- channel;
- alias;
- pin;
- lease;
- tombstone; and
- metadata label.

Every mutable ref update MUST use an expected prior value and append history.
Refs MUST NOT establish compatibility, trust, integrity, validation, or reuse.

## FERRIS Application Contract

A resolved Application Contract MUST include:

- contract identity and application root;
- selected Application Definition and Blueprint Model;
- selected contract and profile revisions;
- component, service, provider, platform, and deployment selections;
- exact Cargo and non-Cargo owner identities;
- compatibility results and conditions;
- support and servicing state;
- required validation and observed outcomes;
- governance and approval requirements;
- evidence sources, dates, confidence, expiry, and unknowns;
- unsupported, degraded, exception, and deferred scope;
- renewal triggers;
- substitution and migration;
- removal and rollback; and
- decision owners.

The canonical application state MUST be one of:

- resolved and validated;
- resolved with named conditions;
- degraded;
- blocked;
- unsupported;
- stale;
- incomplete;
- failed; or
- unknown.

The Application Contract MUST NOT use a single “green” field as its canonical
state.

## Resolution boundary

APPLICATION-001 defines available application records. It does not select or
execute work.

- PLANNING-001 creates candidate Blueprint Plans.
- RESOLUTION-001 selects, widens, defers, or rejects a plan.
- EXECUTION-001 authorizes an Action Plan.
- CONFORMANCE-001 tests the complete workflow.

Observation and resolution MUST remain separate from mutation.

## Lifecycle

Application records MUST support:

- draft;
- active;
- deprecated;
- superseded;
- expired;
- revoked;
- removed; and
- retained historical state.

Renewal MUST re-evaluate changed source, lock, contract, profile, environment,
provider, connector, governance, validation, and support evidence.

Removal MUST restore ordinary Cargo and owner-system workflows without hidden
Ferris-owned correctness state.

## Acceptance criteria

APPLICATION-001 may advance to Proposed only when:

1. one small, one medium, and one federated multi-workspace application are
   represented;
2. Cargo truth and application declarations conflict in controlled fixtures;
3. normalization is deterministic and preserves unknowns;
4. contracts, profiles, providers, native components, validation, governance,
   and deployments are joined without collapsing ownership;
5. immutable roots and every ref kind have positive and negative fixtures;
6. moving refs use expected-value updates and preserve history;
7. resolved, conditional, degraded, blocked, unsupported, stale, incomplete,
   failed, and unknown states are exercised;
8. renewal, substitution, rollback, and complete removal are specified; and
9. all nine roles record a disposition.
