# PLANNING-001: Ferris Blueprint Planning Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: APPLICATION-001, SCOPE-001, IDENTITY-001, PREDICTION-001,
VALIDATION-001, BLUE-Q02, and BLUE-Q03

## Purpose

This specification defines the versioned, non-executable Blueprint Plan that
composes owner-specific affected closures, commands, validation, resources,
artifact economics, uncertainty, fallback, observation barriers, and replans.

The governing rule is:

> **The plan is global; the work is local.**

## Distinct records

Ferris MUST keep separate:

1. Application Definition;
2. Blueprint Model;
3. Prediction Record;
4. Blueprint Plan;
5. Resolution Record;
6. approved Action Plan;
7. Execution Record;
8. observed Forest root; and
9. Outcome Record.

A Blueprint Plan does not grant authority to execute. A later Action Plan is
the exact approved executable projection.

## Plan identity

Every Blueprint Plan MUST include:

- plan ID, schema, version, and parent plan;
- application and Blueprint Model identities;
- triggering FOREST-002 Change Record and prior Forest root;
- prediction and evidence cutoff;
- source, Cargo, contract, profile, platform, toolchain, environment,
  governance, and validation identities;
- requested user outcome;
- creation owner and time;
- lifecycle state; and
- expiry and replan triggers.

A changed plan input produces a new version. A replan MUST NOT rewrite the
original plan.

## Federated DAG

The plan MUST link, not flatten:

- Cargo invocation and unit closures;
- supported compiler and code-generation evidence;
- build-script, macro, generator, and native execution cones;
- linker and finalization work;
- contract, projection, compatibility, and migration work;
- validation and capability coverage;
- artifact eligibility, verification, transfer, and materialization;
- deployment or operational work where declared; and
- evidence collection.

Each node MUST identify owner, operation, input and output identities, scope,
conditions, dependencies, expected state, resource demand, validation,
fallback, and evidence.

Each edge MUST retain owner semantics, direction, condition, scope, source,
confidence, unknowns, and failure propagation.

## Owner-specific closures

Required work is:

```text
union(owner-specific affected closures)
  + mandatory capability and policy gates
  + explicit finalization work
  + conservative fallback for unknowns
```

Cargo owns resolution, unit construction, freshness, scheduling, and compiler
invocation. rustc owns compiler queries and incremental state. Linkers, test
runners, contract systems, native tools, and deployment systems retain their
local planning and execution rules.

## Owner freshness insufficiency

When canonical evidence identifies a changed input that an owner freshness
model does not declare, observe, or invalidate, selecting a wider owner scope
MUST NOT be presented as sufficient correction.

The plan MUST be blocked or present explicit alternatives:

- repair the owner declaration or mapping;
- use a supported owner-native invalidation operation;
- request an isolated empty-state rebuild;
- select the prior compatible input or environment; or
- defer pending owner guidance.

Deleting, cleaning, touching, or invalidating owner state is an action and
requires EXECUTION-001 approval. The non-executable Blueprint Plan may describe
the alternative and cost but MUST NOT perform it.

## Per-command Cargo plans

The Blueprint Plan MUST retain one Cargo invocation plan per activity,
including applicable:

- command;
- manifest and workspace;
- package and target selection;
- requested features;
- profile;
- host and target;
- configuration and environment;
- lock state;
- expected unit and artifact evidence;
- validation relationship;
- fallback; and
- limitations.

`check`, `build`, `test`, `clippy`, `doc`, and doctest MUST NOT be treated as
one interchangeable unit graph.

## Validation and capability

Every plan MUST embed or reference a VALIDATION-001 plan and retain:

- selected and full-reference coverage;
- mandatory gates;
- omitted scope;
- capability preserved, reduced, unverified, or lost;
- exceptions and expiry; and
- fallback.

Faster work that drops required features, targets, profiles, lints, tests,
symbols, ABI support, safety checks, policy, or deployment guarantees is not an
equivalent plan.

## Resource envelope

The plan MUST define applicable:

- CPU and job limits;
- memory and reserve;
- storage and retention;
- I/O and filesystem placement;
- network and transfer;
- security and indexing effects;
- foreground latency priority;
- concurrent session and worktree isolation;
- coalescing and cancellation;
- thermal or power constraints where material; and
- resource stop and replan thresholds.

The plan MUST NOT silently change repository profiles, codegen settings,
validation, security controls, host configuration, or CI policy to fit the
envelope.

## Artifact economics

Artifact candidates MUST retain:

- producer and input identities;
- compatibility, integrity, provenance, freshness, authorization, and
  capability checks;
- lookup, verification, transfer, materialization, reconstruction, and
  cleanup cost;
- expected work avoided;
- rejection reason; and
- owner-native rebuild fallback.

A hit or matching ref never establishes correctness or net benefit.

## Observation barriers and replanning

An observation barrier pauses dependent planning when owner-local execution
must reveal:

- resolved graph or unit details;
- generated inputs or side effects;
- native or linker state;
- artifact availability or validity;
- validation coverage;
- environment pressure; or
- failure details.

After a barrier, the plan may continue only when observed state matches its
declared conditions. Otherwise Ferris MUST widen, fall back, request a new
versioned plan, or require owner input.

Replan triggers include graph drift, new inputs, incompatible artifacts,
capability gaps, toolchain or environment changes, resource-envelope breach,
failure, cancellation, stale evidence, and policy change.

## Plan states

Canonical plan state MUST distinguish:

- draft;
- complete;
- incomplete;
- blocked;
- stale;
- superseded;
- selected;
- rejected;
- expired; and
- unknown.

Selected does not mean approved for execution.

## Removal

Removing Blueprint planning MUST leave:

- original Application Definition;
- Cargo manifests and lockfiles;
- owner-native commands and configuration;
- repository validation;
- source-control history; and
- ordinary build, test, and release workflows

usable without hidden Ferris correctness state.

## Acceptance criteria

PLANNING-001 may advance to Proposed only when:

1. one small, one medium, and one federated application produce versioned
   plans;
2. at least five owner systems retain separate closures and identities;
3. every applicable Cargo activity has a distinct invocation plan;
4. validation coverage, mandatory gates, capabilities, and omissions remain
   explicit;
5. resource pressure, artifact rejection, unknown input, graph drift, failure,
   and cancellation trigger correct fallback or replanning;
6. replans preserve prior versions and observation deviations;
7. no plan can execute without later resolution and approval;
8. complete removal preserves ordinary owner workflows; and
9. all nine roles record a disposition.
