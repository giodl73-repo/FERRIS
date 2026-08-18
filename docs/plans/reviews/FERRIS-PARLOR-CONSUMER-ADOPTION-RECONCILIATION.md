# FERRIS PARLOR Consumer Adoption Reconciliation

Date: 2026-08-18
Disposition: Accept `consumer-pinned-experimental`
Implementation authority: None

## Evidence

PARLOR is the first named external consumer to satisfy the FERRIS reuse gate
for one bounded surface:

- PARLOR PR
  [#1](https://github.com/giodl73-repo/PARLOR/pull/1) added the exact consumer
  contract and merged at
  `5bd11f8db92fec0ff5e85efc746fcedecc0bd81d`;
- its
  [`contract.json`](https://github.com/giodl73-repo/PARLOR/blob/3ad7cd6b87aa4d248c6785c3f48b30d5c048d789/tools/ferris-contract/contract.json)
  pins exact FERRIS merge
  `5cd1aa99727a23de25c79d067090e7444bdfb5e8`, command version `0.1.0`,
  `ferris.command-result/v2`, and `ferris.validation-plan/v0`;
- the consumer checker verifies the accepted `parlor-go` plus `parlor-cli`
  closure and the repository-file full-workspace fallback;
- PARLOR's release tests, Clippy, and formatting remain separately mandatory;
- the
  [Ubuntu consumer workflow](https://github.com/giodl73-repo/PARLOR/actions/runs/32177692037)
  passed before merge; and
- PARLOR PR
  [#2](https://github.com/giodl73-repo/PARLOR/pull/2), merged at
  `3ad7cd6b87aa4d248c6785c3f48b30d5c048d789`, records a successful pin
  migration from FERRIS merge `5cd1aa9` to implementation `8c0d674` and
  rollback to `5cd1aa9`.

PARLOR copies no FERRIS schema and depends on no FERRIS crate. Its checker
builds a clean exact pin in temporary custody and asserts only the
consumer-owned projection.

## Product Value Governor

Pass. The prior stop condition required a named adopter before expanding the
reuse claim. PARLOR now supplies exact consumer evidence, migration, rollback,
and removal. The accepted value is narrower than support: one protected
experimental projection replaces an unprotected research observation.

## Rust Safety Steward

Pass. This reconciliation changes documentation only. The consumer proof
builds the existing safe-Rust adapter and introduces no FERRIS runtime or
unsafe-code change.

## Compiler Performance Engineer

Pass with no performance claim. PARLOR's historical 9.4% measurement remains
research context, not a renewed benchmark or guaranteed saving.

## Interop Boundary Auditor

Pass. Cargo retains workspace and dependency authority, PARLOR retains
validation policy, and FERRIS retains command/schema ownership. The consumer
pins identifiers and behavior without copying an owner schema.

## AI Assurance Skeptic

Pass. The claim is grounded in exact commits, a machine-readable consumer
contract, accepted and fallback assertions, two operating systems, immutable
CI, and an exercised migration/rollback. No model assertion creates adoption.

## Ecosystem Strategist

Pass. Compatibility evidence lives with the consumer. This is the correct
direction of dependency and avoids making FERRIS the owner of PARLOR policy.

## Rust Maintainer

Pass. No crate API or additional FERRIS code is introduced. The exact
projection can evolve through an explicit consumer migration or a new contract
version.

## Native Platform Adopter

Pass. The proof passed on local Windows and consumer Ubuntu CI, uses a
path-independent workspace identity, and retains a complete removal path.

## Scope Keeper

Pass. Only the exact `validation-plan` projection becomes
consumer-pinned-experimental. Other commands, record fields, crates, support,
execution, mutation, and automatic upgrade remain outside the claim.

## Validation Checker

Pass. PARLOR verified the exact pin, source cleanliness, locked FERRIS build,
Cargo-alias resistance, accepted reverse cone, full fallback, owner validation
presence, fetched-pin CI, migration, and rollback.

## Autonomy Supervisor

Pass. No new FERRIS implementation pulse was created. The work stopped at the
smallest evidence needed to reconcile the published reuse statement, and no
successor feature or support claim follows.

## Decision

All eleven roles accept `consumer-pinned-experimental` for PARLOR's exact
`ferris.validation-plan/v0` projection at the recorded pins. FERRIS remains an
unsupported incubation platform. Any wider graduation requires its own named
consumer evidence and role review.
