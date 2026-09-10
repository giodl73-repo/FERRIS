# Pulse 01: Current-Main Private Corpus Qualification

Status: Complete; stopped incomplete at Windows owner-integrity gate
Implementation authority: Evidence only; no product behavior
Budget: One five-family, two-platform qualification and at most one
infrastructure-only corrective rerun

## User outcome

Determine whether current Ferris preserves the complete existing private
enterprise-shape synthetic corpus before using that corpus as the primary
regression and demonstration suite.

## Frozen inputs

- Ferris: exact revision resolved from `origin/main` before corpus execution.
- Corpus: exactly five private repositories mapped in private custody to
  `EC-01` through `EC-05`.
- Scenarios: only immutable tags and commands already declared by each corpus.
- Platforms: local Windows and GitHub-hosted Ubuntu.
- Owner truth: each repository's existing full Cargo or validation command.

Exact private repository names, revisions, paths, commands containing private
paths, raw output, and identifiable timings remain outside this public tree.

## Procedure

1. Freeze the Ferris revision, private repository revisions, scenario tags,
   toolchain identities, and available CI definitions before execution.
2. Build Ferris once per platform from the frozen revision with its locked
   dependencies.
3. Run each repository's existing fixture-integrity or owner-validation command.
4. Run every already-declared Ferris scenario twice and compare plan,
   selection, and applicable change-set identities.
5. Run the existing selected and full owner commands for scenarios that declare
   them, preserving pass and failure outcomes separately.
6. Record fallback and typed rejection cases without converting them into
   successful selection.
7. Retain detailed records privately and publish only aggregate family-level
   dispositions.
8. Remove every temporary hosted-validation branch after evidence custody.

## Acceptance criteria

- all five frozen corpus revisions pass their own integrity and owner checks;
- every declared scenario has a complete disposition on both platforms;
- repeated deterministic identities match for every successful plan;
- expected typed rejections occur before owner execution;
- no selected-pass/full-fail outcome occurs;
- declared matched-failure controls remain failures in both lanes;
- no private identifier or identifiable timing enters the public evidence; and
- no product, schema, dependency, scenario, or owner-command change is needed.

Failure of an acceptance criterion closes the pulse as failed or incomplete; it
does not authorize a product fix or another campaign.

## Result

The private custody record froze Ferris revision
`e66795f4292a9e23a95fffc13a17bece8f5fff61` and five exact private corpus
revisions before execution. The Windows release binary built successfully with
stock Rust and Cargo 1.95.0.

The first owner-preflight attempt was invalid before any Ferris scenario
started. A globally configured compiler cache restored objects produced by a
different Rust 1.95.0 compiler identity, and owner doctests rejected the mixed
artifacts with `E0514`. The one budgeted infrastructure-only correction
disabled the wrapper and cleaned only the five private custody target trees.

On the corrected preflight:

- `EC-01`, `EC-02`, and `EC-03` passed formatting, Clippy, and their complete
  locked/offline owner test suites;
- `EC-05` passed all sixteen workspace formatting, Clippy, locked/offline test,
  and generated-corpus integrity checks; and
- `EC-04` passed formatting, Clippy, and its complete locked/offline 96-package
  owner test suite, but its repository-owned regeneration check produced a
  different `TOPOLOGY.json` hash on Windows.

The `EC-04` owner-integrity failure triggered the precommitted stop condition.
Zero Ferris scenarios, selected/full comparisons, typed-rejection campaigns, or
hosted Ubuntu jobs ran. No temporary hosted-validation branch was created.

The aggregate record is
[`Private enterprise-shape corpus qualification`](../../../../docs/research/2026-09-10-private-enterprise-corpus-qualification.md).
Exact private identities, revisions, paths, output, and hashes remain in private
custody.

## Validation

Public closeout validation:

```console
git diff --check
```

Applicable Markdown links and code fences must also be checked. Cargo-wide
validation is not required because this pulse changes no Rust source, schema,
fixture, or executable contract.

## Role review requirement

Closeout must record dispositions for:

- Rust Safety Steward;
- Compiler Performance Engineer;
- Interop Boundary Auditor;
- AI Assurance Skeptic;
- Ecosystem Strategist;
- Rust Maintainer;
- Native Platform Adopter;
- Scope Keeper;
- Validation Checker;
- Product Value Governor; and
- Autonomy Supervisor.

The completed dispositions are in the
[`post-qualification role review`](../../../../docs/plans/reviews/FERRIS_PRIVATE_ENTERPRISE_CORPUS_QUALIFICATION_ROLE_REVIEW.md).

## Non-goals

- changing Ferris or corpus behavior;
- adding scenarios, repositories, commands, or dependencies;
- replacing or narrowing owner CI;
- production, support, savings, or representativeness claims;
- publishing private identities or raw evidence; and
- authorizing a successor pulse.

## Closeout

- Completed work: frozen custody, one Windows owner-preflight attempt, and one
  permitted infrastructure-only correction.
- Unrun work: every Ferris scenario and the complete hosted Ubuntu campaign.
- Remaining gate: separately approved correction of the `EC-04` deterministic
  generator or checked-in topology contract, followed by separately approved
  qualification authority.
- Implementation authority: exhausted; no product or private-corpus correction
  is authorized by this closeout.
