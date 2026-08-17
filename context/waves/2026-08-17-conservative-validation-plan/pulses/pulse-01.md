# Pulse 01: Conservative Validation Plan

Status: Complete
Implementation authority: Bounded to this document
Budget: One implementation attempt, one review record, no successor

## Goal

Implement a local read-only Ferris command that:

1. accepts one explicit workspace manifest plus explicit changed workspace
   paths and package names;
2. invokes only
   `cargo metadata --format-version 1 --no-deps --offline --locked`;
3. emits a stable `ferris.validation-plan/v0` record and matching human view;
4. explains selected Cargo package closure separately from full-workspace
   fallback; and
5. never executes Cargo validation commands or claims full-suite equivalence.

## Authorized files

- root Cargo workspace files;
- `crates/ferris-core/`;
- `crates/ferris-cli/`;
- development fixtures under `tests/fixtures/`;
- this wave, its review record, and directly related public documentation.

## Required behavior

- `ferris validation-plan --workspace-id <portable-id> --manifest-path <Cargo.toml> (--changed-path <PATH> | --changed-package <PACKAGE>)+ [--format human|json]`;
- explicit local inputs only; no git, sibling, or network discovery;
- Cargo remains authoritative for workspace/package discovery and dependency
  closure;
- explicit package names, exact package roots, and existing non-build `.rs`
  paths inside exactly one workspace package may narrow package scope;
- all other explicit workspace paths widen to a visible full-workspace
  fallback;
- selected package closure, selected Cargo activity families, fallback
  boundary, reasons, unknowns, and limitations remain visible;
- machine output uses the existing typed command envelope and avoids
  checkout-absolute paths; and
- human output explains why each package/activity was selected or why fallback
  is required.

## Prohibited behavior

- executing `cargo check`, `cargo build`, `cargo test`, Clippy, formatting, or
  repository gates;
- inventing repository-owned validation declarations, mandatory gates, or
  confidence scores;
- contacting a network or discovering sibling repositories/workspaces;
- mutating manifests, lockfiles, sources, configuration, evidence, or
  environment state;
- creating a pulse chain, successor, or diagnostic custody layer; or
- claiming full-suite, release, support, platform, correctness, or CI
  equivalence.

## Acceptance

- formatting: run `cargo fmt` checks and report any pre-existing failure in
  unrelated diagnostic-pulse files instead of widening scope to reformat them;
- targeted core tests for supported-input selection, unknown-path fallback, and
  workspace-boundary safety;
- relevant CLI tests for JSON/human validation-plan output;
- build: `cargo check --workspace`;
- `git diff --check` passes;
- selected output stays non-executable and redacts checkout-absolute paths; and
- unsupported/out-of-workspace inputs remain typed non-success results.

## Stop conditions

Stop the pulse rather than widening scope if it requires:

- repository-owned validation declarations or hidden mappings;
- execution of validation commands or any owner action beyond Cargo metadata;
- git, sibling, or network discovery;
- a successor pulse, another architectural layer, or custody infrastructure;
- a second implementation attempt; or
- a new command family beyond the bounded `validation-plan` slice.

## Removal

Removal requires deleting the Ferris executable, library artifacts, transient
target output, this pulse's command code, and directly related documentation.
It MUST NOT require changing a fixture Cargo manifest, lockfile, source file,
workspace membership, or ordinary Cargo command.

## Review

Measured outcome is recorded in
`docs/plans/reviews/FERRIS-CONSERVATIVE-VALIDATION-PLAN-REVIEW.md`.
