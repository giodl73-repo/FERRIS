# Pulse 01: Owner-Native Validation Metadata

Status: Complete

Implementation authority: Bounded to this document

Approval: Explicit repository-owner direction, "Start implementing.", on
2026-09-11 after review of the proposed bounded capability

Budget: One production implementation and one localized corrective pass

## Product outcome

Extend the read-only `validation-plan` command so a selected opaque owner
entrypoint can report owner-declared validation breadth, preparation identity,
and working directory without Ferris interpreting or executing owner commands.

## Authorized files

- `crates/ferris-core/src/lib.rs`;
- `crates/ferris-cli/tests/`;
- `docs/schemas/validation-plan/`;
- `tests/fixtures/simple-workspace/`;
- `AGENTS.md`, `CONTEXT.md`, `README.md`, and directly related program docs;
- the research note, this wave, this pulse, and one role-review record.

## Required behavior

- support `ferris.owner-validation-domains/v2` alongside unchanged v1;
- require every v2 entrypoint to declare an opaque entrypoint ID, one of
  `focused`, `subsystem`, or `comprehensive`, an opaque preparation ID, and a
  normalized workspace-relative working directory;
- preserve selected v2 metadata in deterministic machine and human output;
- bind v2 declarations into contract and validation-plan identity;
- reject duplicate entrypoint IDs, invalid IDs, unsafe working directories,
  unsupported breadth, unknown fields, and mixed v1/v2 entrypoint shapes;
- preserve existing Cargo selection and conservative fallback behavior; and
- preserve byte-identical v1 and no-contract results.

## Prohibited behavior

- command, argument, recipe, workflow, toolchain, or environment-value parsing;
- preparation or validation execution;
- inference of native commands, coverage, sufficiency, cost, or CI equivalence;
- automatic replacement or disabling of `just`, Cargo wrappers, or owner gates;
- Git, network, hosted-provider, or adopter changes; and
- performance or realized-savings claims.

## Acceptance

- focused `ferris-core --lib` tests for v1 compatibility and v2 semantics;
- focused `ferris-cli` behavior tests;
- published v2 structural schema and semantic-conformance tests;
- all existing validation-plan schema tests;
- role dispositions from all eleven repository roles;
- `cargo fmt --check`; and
- `git diff --check`.

## Removal

Remove v2 parsing, selected metadata, its schema, fixture, tests, and
documentation. V1 and no-contract behavior remain unchanged, so repositories
can return to opaque entrypoint selection without changing Cargo or owner
workflows.

## Stop conditions

Stop rather than expanding the pulse if validation requires command execution,
workflow interpretation, adopter mutation, performance measurement, or another
contract layer.

## Closeout evidence

The implementation added:

- one optional closed `ferris.owner-validation-domains/v2` input schema;
- one development fixture with a focused entrypoint, opaque preparation ID, and
  workspace-relative working directory;
- deterministic JSON and human projection of selected v2 metadata;
- runtime rejection for unsafe working directories and mixed version shapes;
- schema rejection for unsupported breadth and command-shaped unknown fields;
  and
- unchanged v1 and no-contract compatibility paths.

Focused validation on stock Rust 1.95 completed:

- 15 owner-domain core tests passed;
- the pinned no-contract identity test passed independently;
- 12 validation-plan CLI tests passed;
- all four validation-plan structural and semantic schema tests passed;
- file-scoped Rust formatting passed for every changed Rust file; and
- `git diff --check` passed.

The repository-wide `cargo fmt --all --check` remains blocked by pre-existing
formatter drift in untouched historical Pulse 54 and later diagnostic-release
tests. The formatter-generated changes to those files were reversed, and no
historical release fixture is modified by this pulse.

The completed role dispositions are recorded in
[`FERRIS Owner-Native Focused Validation Role Review`](../../../../docs/plans/reviews/FERRIS_OWNER_NATIVE_FOCUSED_VALIDATION_ROLE_REVIEW.md).

## Final authority

The production, corrective, and review budgets are exhausted. No command
interpretation, preparation or validation execution, workflow parsing,
automatic native-command generation, adopter migration, CI narrowing,
performance measurement, or savings claim follows.
