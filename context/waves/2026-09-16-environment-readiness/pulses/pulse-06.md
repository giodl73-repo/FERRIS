# Pulse 06: Application Readiness Composition Contract

Status: Complete

## User outcome

Define the smallest fail-closed contract that can report readiness across an
explicit multi-workspace application without presenting one Cargo workspace
observation as application-wide readiness.

## Authority

The user's fresh `continue` after Pulse 05 authorizes one
documentation/schema/fixture-only contract decision:

- define one strict application-readiness request over an explicit
  `ferris.application/v0` definition;
- bind two to sixteen independent Cargo workspace manifests and one existing
  READINESS-001 declaration per workspace;
- define one aggregate report that preserves each workspace report identity,
  requirements digest, and typed status;
- define deterministic identity, stale-input handling, privacy, removal, and
  fail-closed aggregation;
- freeze positive, mixed-status, stale, and negative-control fixtures; and
- complete an eleven-role review.

This pulse does not authorize Rust product code, CLI behavior, Cargo metadata
execution, owner-command execution, an application-root requirement kind,
application-definition mutation, source adapters, dependency changes, adopter
changes, support claims, or a successor pulse.

## Maximum effort

One request schema, one aggregate-report schema, one normative contract, one
bounded fixture family, one eleven-role review, registry and wave updates, and
documentation validation.

## Completion test

- the request root and every referenced path are explicit and traversal-safe;
- every declared application workspace is covered exactly once;
- each workspace keeps its own manifest, READINESS-001 declaration, report
  identity, requirements digest, and status;
- aggregate precedence cannot hide `blocked`, `incomplete`, `unsupported`, or
  `stale` workspace state;
- loading and observation require no Cargo or owner command;
- application-definition and request changes become typed stale results;
- schemas and fixtures validate, including expected negative controls;
- privacy, identity, removal, and future implementation gates are explicit;
- all eleven role dispositions are recorded; and
- Markdown validation and `git diff --check` pass.

## Stop condition

Stop if the contract would duplicate Cargo workspace truth, reinterpret a V1
workspace declaration as an application-root declaration, require executable
discovery, conceal an independently blocked workspace, or depend on private
consumer content. Do not implement the contract inside this pulse.

## Result

APP-READINESS-001 is Draft with strict request and aggregate-report schemas.
The request binds one explicit `ferris.application/v0` definition and three
fixture workspaces without Cargo discovery. The report preserves each
READINESS-001 result and applies `stale`, `blocked`, `incomplete`,
`unsupported`, then `ready` precedence.

Independent review corrected absent base fixtures, member-manifest ambiguity,
missing nested-root rejection, child-stale diagnostics, diagnostic-array
ordering, an ineffective coverage control, and placeholder identities. Final
fixtures bind exact file digests and computed canonical aggregate identities.

The
[eleven-role review](../../../../docs/plans/reviews/FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT_REVIEW.md)
accepts the contract as Draft and withholds implementation. Application-root
requirements remain deferred because a workspace V1 declaration cannot be
reinterpreted safely. Pulse 06 authority is exhausted.
