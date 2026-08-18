# FERRIS RUNE Consumer Adoption Reconciliation

Date: 2026-08-18
Disposition: Accept second `consumer-pinned-experimental` projection
Implementation authority: None

## Evidence

RUNE is the second named external consumer to satisfy the FERRIS reuse gate,
and it supplies a topology materially different from PARLOR:

- RUNE PR
  [#1](https://github.com/giodl73-repo/RUNE/pull/1) merged the consumer
  contract at `3eae3c2f633f3c638308452029e199db6056d887`;
- its
  [`contract.json`](https://github.com/giodl73-repo/RUNE/blob/3eae3c2f633f3c638308452029e199db6056d887/tools/ferris-contract/contract.json)
  pins exact FERRIS merge
  `35f3518b6597acde34641a4f55e5111405334e70`, command version `0.1.0`,
  `ferris.command-result/v2`, and `ferris.validation-plan/v0`;
- RUNE uses Cargo resolver 3 and includes `rune-derive`, a procedural-macro
  package with `trybuild` compile tests;
- the accepted projection is the `rune-derive` anchor plus the
  `rune-adopter` and `rune-shape-calculator` reverse dependencies;
- the repository-file fallback retains all six RUNE workspace packages;
- RUNE's formatting, full workspace tests, runtime status command, and diff
  hygiene remain separately mandatory;
- the exact-pin proof passed on Windows and in the immutable-revision
  [Ubuntu consumer workflow](https://github.com/giodl73-repo/RUNE/actions/runs/32182143870);
  and
- migration from FERRIS `35f3518` to `5cd1aa9` and rollback to `35f3518`
  both passed without a retained pin change.

RUNE copies no FERRIS schema and depends on no FERRIS crate. Its checker builds
a clean exact pin in temporary custody, invokes the exact adapter directly,
and asserts only the consumer-owned projection.

## Product Value Governor

Pass. RUNE adds evidence that PARLOR cannot provide: resolver 3, a
procedural-macro anchor, compile-test dev dependencies, example adopters, and
a different three-package reverse cone. This justifies a second named
consumer record but not a general support claim or another implementation
pulse.

## Rust Safety Steward

Pass. This reconciliation changes documentation only. The consumer proof
builds the existing safe-Rust adapter and introduces no FERRIS runtime or
unsafe-code change.

## Compiler Performance Engineer

Pass with no performance claim. No compile-time reduction is inferred from the
selected closure. RUNE's full workspace tests remain mandatory, including the
procedural-macro compile tests.

## Interop Boundary Auditor

Pass. Cargo retains package and dependency authority. RUNE retains macro,
compile-test, runtime, and repository validation semantics. FERRIS retains
command and schema ownership. The consumer pins identifiers and behavior
without copying an owner schema.

## AI Assurance Skeptic

Pass. Exact commits, a machine-readable consumer contract, portable path
assertions, accepted and fallback outcomes, immutable Ubuntu CI, alias
resistance, and exercised migration/rollback ground the claim.

## Ecosystem Strategist

Pass. The second consumer demonstrates reuse across a distinct workspace
shape while retaining the correct dependency direction: compatibility
evidence lives with RUNE and points to the public FERRIS owner.

## Rust Maintainer

Pass. No crate API or FERRIS code is introduced. The claim remains bounded to
the exact fields and behavior asserted by the consumer contract and can evolve
only through explicit migration or versioning.

## Native Platform Adopter

Pass. The proof passed on Windows and Ubuntu, uses a portable workspace
identity and paths, and retains complete rollback and removal procedures.

## Scope Keeper

Pass. FERRIS does not claim to understand procedural-macro expansion,
individual `trybuild` cases, features, targets, doctests, runtime status, or
repository policy. It does not execute the selected activities.

## Validation Checker

Pass. RUNE verified exact source identity and cleanliness, locked FERRIS build,
direct adapter invocation, Cargo-alias resistance, portable inputs, selected
closure, full fallback, owner validation presence, fetched-pin CI, migration,
and rollback.

## Autonomy Supervisor

Pass. The work stopped at consumer evidence and documentation reconciliation.
No FERRIS implementation pulse, automatic pin advancement, support claim, or
successor authority follows.

## Decision

All eleven roles accept RUNE's exact `ferris.validation-plan/v0` projection as
a second `consumer-pinned-experimental` contract. Together PARLOR and RUNE
improve topology evidence, but FERRIS remains an unsupported incubation
platform. Only each consumer's explicitly asserted projection is protected;
wider compatibility or graduation requires separate evidence and role review.
