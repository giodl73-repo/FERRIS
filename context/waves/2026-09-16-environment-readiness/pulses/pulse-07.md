# Pulse 07: Application Readiness Composition Implementation

Status: Complete

## User outcome

Let a developer obtain one fail-closed readiness result for an explicit
multi-workspace application before owner work starts, while preserving each
workspace's independent READINESS-001 result.

## Control record

- Product outcome: expose the frozen APP-READINESS-001 composition through the
  existing `doctor` surface.
- Work completed: workspace READINESS-001 is implemented and application
  composition is frozen as a reviewed Draft contract.
- Value obtained: Ferris can identify missing prerequisites per workspace but
  cannot yet prevent one workspace result from being mistaken for complete
  multi-workspace readiness.
- Remaining risk: passive application loading, filesystem-root identity,
  fail-closed aggregation, and stale composition have no product proof.
- Pulses consumed: six environment-readiness pulses; no prior implementation
  attempt for APP-READINESS-001.
- Proposed next action: one bounded implementation and conformance pass.
- Product Value Governor: `continue-within-budget`.

## Authority

The user's fresh `go` after Pulse 06 authorizes one optional
`doctor --application-readiness <REQUEST_JSON>` implementation of the frozen
APP-READINESS-001 contract:

- strict bounded request and passive `ferris.application/v0` loading;
- explicit application-root, manifest, and requirements binding without Cargo;
- unchanged READINESS-001 observation per declared workspace;
- deterministic aggregate report, command envelope, stale handling, privacy,
  and VIEW-001 result mapping;
- targeted core, CLI, schema, filesystem, non-execution, identity, privacy,
  failure, and legacy-compatibility tests; and
- an eleven-role implementation closeout.

This pulse does not authorize application-root requirements, a new source
adapter, Cargo metadata or owner-command execution, installation, repair,
parallel scheduling, cache reuse, adopter changes, support, production,
performance, savings, or a successor pulse.

## Maximum effort

One core module, one optional `doctor` input, targeted tests, directly related
contract and status corrections, one review, and one commit. No dependency
change.

## Completion test

- the implementation accepts the frozen positive request and report shapes;
- it rejects every frozen structural and semantic negative control at the
  intended gate;
- it never invokes Cargo, a candidate executable, or owner work;
- child reports remain independently bound and worst-state precedence is exact;
- request, application-definition, and child declaration changes become stale
  rather than ready;
- output retains no absolute path, environment value, or resolved path;
- repeated equivalent runs are byte-identical;
- legacy `doctor` and `doctor --requirements` behavior remains unchanged;
- targeted tests, schema proof, formatting, and `git diff --check` pass; and
- eleven roles accept or explicitly block the result.

## Stop condition

Stop on any need to alter READINESS-001 semantics, invoke Cargo to verify a
workspace, add application-root requirements, expose private paths, change an
adopter, add a dependency, or widen beyond the frozen contract. One corrective
review pass is allowed inside this pulse; further work requires fresh approval.

## Result

`doctor --application-readiness <REQUEST_JSON>` implements the frozen
APP-READINESS-001 V1 composition without Cargo discovery, owner execution, or
new dependencies. Strict binding, independent READINESS-001 observation,
filesystem identity, exact stale revalidation, deterministic aggregation,
privacy-safe output, and legacy compatibility passed targeted core and CLI
tests.

The initial independent review found four fail-closed defects: unsupported
child schemas reached report-bearing observation, declaration replacement could
become internal error, hard-linked manifests could alias workspace roots, and
same-byte replacement could escape stale classification. All four received
focused corrections and regression tests. The authorized final review found no
remaining material issue.

The
[implementation review](../../../../docs/plans/reviews/FERRIS_APPLICATION_READINESS_IMPLEMENTATION_REVIEW.md)
records all eleven dispositions, evidence, and remaining limits. Pulse 07 is
complete and exhausted. No application-root requirements, adapter, adopter,
execution, support, production, performance, savings, or successor authority
follows.
