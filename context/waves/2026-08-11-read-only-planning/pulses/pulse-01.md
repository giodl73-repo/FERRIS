# Pulse 01: Local Plan and Explain

Status: Complete on Windows and Unix; applicable held-out fixtures passed
Implementation authority: Bounded to this document

## Goal

Implement a local read-only Ferris core and CLI that:

1. invokes official
   `cargo metadata --format-version 1 --no-deps --offline --locked`;
2. accepts one explicit workspace manifest;
3. emits a non-executable `ferris.blueprint-plan/v0` record;
4. emits a `ferris.explanation/v0` record for that plan; and
5. uses the fixed VIEW-001 process codes.

## Authorized files

- root Cargo workspace files;
- `crates/ferris-core/`;
- `crates/ferris-cli/`;
- development fixtures under `tests/fixtures/`;
- this wave and directly related public documentation.

## Required behavior

- `ferris plan --workspace-id <portable-id> --manifest-path <Cargo.toml> [--format human|json]`;
- `ferris explain --workspace-id <portable-id> --manifest-path <Cargo.toml> [--format human|json]`;
- exact explicit manifest selection;
- Cargo remains authoritative for workspace and package discovery;
- stable ordering independent of Cargo JSON order;
- non-executable output with visible evidence source and unknown limitations;
- no path content outside owner metadata in machine output unless required for
  explicit selection identity;
- deterministic process codes;
- separate human and JSON renderers over the same typed record; and
- actionable diagnostics for invalid manifests and Cargo failures.

## Prohibited behavior

- invoking build scripts, rustc, linkers, tests, generators, or owner actions;
- contacting a network;
- discovering sibling repositories or workspaces;
- writing application, Cargo, lock, source, configuration, or evidence state;
- executing a plan;
- using AI, predictions, connectors, MCP, approvals, credentials, or secrets;
- reading held-out edit or oracle packages; or
- claiming affected-only correctness, performance gain, conformance, or
  production support.

## Acceptance

- formatting: `cargo fmt --all -- --check`;
- lint: `cargo clippy --workspace --all-targets -- -D warnings`;
- tests: `cargo test --workspace`;
- invalid manifest returns `2`;
- unsupported Cargo metadata version returns `4`;
- missing required metadata returns `5`;
- Cargo unavailable or offline resolution blocked returns `7`;
- malformed Cargo output or an internal invariant returns `11`;
- plan and explanation fixtures prove stable JSON and human semantics;
- a negative fixture proves no sibling workspace discovery; and
- `git diff --check` passes.

## Stop conditions

Stop the pulse rather than widening scope if it requires:

- nightly Cargo internals or `-Z` flags;
- owner execution beyond metadata;
- network access;
- mutation;
- a hidden manifest or parallel resolver;
- an unstable output field presented as owner truth;
- more than two product crates; or
- any command outside `plan` and `explain`.

## Removal

Removal requires deleting the Ferris executable, library artifacts, and
transient target output. It MUST NOT require changing a fixture Cargo
manifest, lockfile, source file, workspace membership, or ordinary Cargo
command.

## Review

Approved by the nine-role entry review in
`docs/plans/reviews/FERRIS-READ-ONLY-IMPLEMENTATION-ENTRY-REVIEW.md`.

Completed under the nine-role measured review in
`docs/plans/reviews/FERRIS-READ-ONLY-IMPLEMENTATION-COMPLETION-REVIEW.md`.
