# Owner-Native Focused Validation

Status: Accepted input for one bounded implementation pulse

Date: 2026-09-11

## Question

What additional information does Ferris need to distinguish a focused
owner-native validation entrypoint from a subsystem or comprehensive entrypoint
without interpreting commands or taking ownership from the repository?

## Evidence

The private enterprise repository reviewed for this decision is recorded here
as `ER-01`. Exact repository, pull-request, author, revision, and raw-diff
identities remain in session custody and MUST NOT enter this public repository.

The 2026-09-11 read-only review established:

- `FERRIS-FOCUSED-001`: `ER-01` first changed contributor guidance from broad
  per-surface recipe bundles to the smallest change-specific local command while
  retaining comprehensive required CI.
- `FERRIS-FOCUSED-002`: a later change machine-enforced that local default and
  rejected legacy broad fallback patterns.
- `FERRIS-FOCUSED-003`: expensive owner recipes were changed to require either
  a focused selector or an explicit broad mode; an omitted selector no longer
  launched the largest suite.
- `FERRIS-FOCUSED-004`: native Cargo, npm, TypeScript, and Python commands were
  then preferred for focused validation, but owner wrappers and recipes remained
  authoritative for toolchain selection, private feeds, platform setup,
  compiler settings, orchestration, images, generators, and CI-equivalent lanes.
- `FERRIS-FOCUSED-005`: a separate owner change removed one duplicate Rust
  front-end pass from an aggregate test recipe while preserving its declared
  feature and test coverage.

Current Ferris already selects opaque owner entrypoint IDs through
`ferris.owner-validation-domains/v1`; see
[`FERRIS_OWNER_VALIDATION_DOMAINS_PLAN.md`](../plans/FERRIS_OWNER_VALIDATION_DOMAINS_PLAN.md).
That contract does not state whether an entrypoint is focused, subsystem-wide,
or comprehensive, and it does not identify the owner preparation procedure or
working directory required before execution.

## Decision

Add an optional `ferris.owner-validation-domains/v2` contract. Each declared
entrypoint MUST provide:

- its existing opaque entrypoint ID;
- one explicit validation breadth: `focused`, `subsystem`, or `comprehensive`;
- one opaque preparation ID; and
- one normalized Cargo-workspace-root-relative working directory.

Ferris MUST preserve those declarations in a non-executable validation plan.
It MUST NOT parse commands, infer a native command, inspect recipes or
workflows, expand environment values, execute preparation, or claim that a
focused entrypoint is sufficient.

V1 remains supported unchanged. Supplying no owner-domain contract continues to
produce the existing bytes and identity.

## Measurable decision

Proceed only if a v2 fixture proves all of the following:

1. a selected entrypoint reports its breadth and preparation reference;
2. changing preparation or breadth changes contract and plan identity;
3. invalid breadth, unsafe working directories, duplicate IDs, and unknown
   fields fail deterministically;
4. v1 output and no-contract output remain unchanged; and
5. no command text or executable behavior is added.

## Limits

This evidence does not establish build-time savings, CI equivalence, command
correctness, or a safe automatic replacement for owner recipes. Duplicate-work
observation, cost measurement, command interpretation, and execution remain
separate future decisions.
