# SCOPE-001: Ferris Scope Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: APPLICATION-001, FOREST-001, BLUE-Q03, and the Crates Series

## Purpose

This specification defines the multi-dimensional scope coordinates, mappings,
set operations, widening rules, and economics controls used by Ferris and
Blueprint.

Scope identifies the bounded domain over which a statement, observation,
selection, validation result, policy, prediction, or action applies. It is not
one containment hierarchy.

## Scope coordinate

A canonical scope coordinate MUST identify applicable:

- owner;
- subject;
- activity;
- configuration;
- platform and environment;
- lifecycle; and
- evidence state.

Each axis MUST be typed, independently versioned, and capable of representing:

- one exact value;
- a finite set;
- a declared range;
- all values within a named owner boundary;
- not applicable;
- unsupported;
- not observed;
- stale; or
- unknown.

An omitted axis MUST NOT silently mean all values.

## Owner-native anchors

Scope selection MUST begin from stable owner-native anchors, including:

- Git repository and revision;
- Cargo workspace, package, target, feature, profile, platform, and unit;
- rustc invocation and supported compiler evidence;
- linker or native build input;
- validation command and repository gate;
- Typebook contract and operation;
- application, component, service, provider, and deployment; and
- governance policy and approval boundary.

Ferris MUST NOT create a synthetic anchor that replaces the owning system's
identity or selection semantics.

## Distinct command scopes

Every command plan MUST retain separate:

- package selection;
- target selection;
- feature and configuration selection;
- build or compiler work;
- artifact production;
- runtime execution;
- validation and capability coverage;
- deployment or operational scope; and
- omitted or unresolved scope.

A package filter MUST NOT be treated as complete compilation, runtime, test,
deployment, or policy scope. A test name filter MUST NOT be treated as
compilation scope. Compilation success MUST NOT establish runtime or deployment
coverage.

## Scope mappings

Every mapping MUST record:

- mapping type and direction;
- source and target coordinates;
- cardinality: one-to-one, one-to-many, many-to-one, or many-to-many;
- condition;
- declaring and evidence owners;
- authority class;
- declared, resolved, observed, inferred, proposed, unsupported, stale,
  not-observed, or unknown state;
- source evidence and tool version;
- confidence and limitations;
- observation time and expiry;
- renewal trigger;
- complexity cost; and
- fallback boundary.

Mappings MUST NOT be assumed reversible. A reverse lookup requires its own
mapping or a proven invertibility rule.

## Scope algebra

The scope engine MUST provide deterministic:

- equality and containment;
- union and intersection;
- difference with retained exclusions;
- directional mapping;
- owner-specific affected closure;
- transitive closure with cycle reporting;
- widening;
- selected-versus-full comparison; and
- cardinality and complexity accounting.

Operations MUST preserve axis types, conditions, sources, unknowns, and omitted
scope. An empty result MUST distinguish proven-empty, unsupported,
not-observed, stale, failed, and unknown.

## Narrowing and widening

Narrowing that removes work, validation, capability coverage, or policy gates
MUST be justified by deterministic owner evidence or explicit approval.

AI MAY propose mappings, exclusions, and narrower scope. An AI proposal MUST:

- start from an owner-native anchor;
- cite the evidence available at proposal time;
- state confidence and unknowns;
- compare against the full-reference scope;
- preserve mandatory gates; and
- remain non-authoritative until policy or human approval accepts it.

Unknown, expired, conflicting, or failed mappings MUST widen to the smallest
safe named owner boundary. If no safe boundary is known, the result MUST be
blocked or require owner input rather than silently selecting less work.

## Scope-detail budget

Fine-grained evidence collection MUST have an explicit budget covering:

- collection latency;
- CPU, memory, I/O, storage, and transport;
- retained record count and bytes;
- toolchain stability;
- privacy and data classification;
- maintenance ownership; and
- expected work or validation avoided.

Budget exhaustion MUST produce a named coarser fallback. It MUST NOT truncate
scope without recording the omitted detail and consequence.

## Lifecycle

Scope mappings MUST support active, deprecated, expired, superseded, revoked,
and retained-historical states.

Renewal MUST occur when source, Cargo graph, contracts, profiles, owner tools,
platform, environment, validation, governance, or mapping evidence changes.

Removal MUST restore owner-native command selection without hidden Ferris
correctness state.

## Acceptance criteria

SCOPE-001 may advance to Proposed only when:

1. file, module, item, package, target, unit, compiler, test, contract, native,
   service, deployment, policy, and evidence scopes are represented;
2. at least six cross-command mappings exercise every cardinality;
3. directional and non-invertible mappings have negative fixtures;
4. selected and full-reference scopes are compared;
5. unknown, stale, unsupported, not-observed, conflict, and budget exhaustion
   widen or block correctly;
6. AI proposals cannot independently remove work or validation;
7. ordinary Cargo and owner-native filters remain available after removal; and
8. all nine roles record a disposition.
