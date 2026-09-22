# Public Synthetic Action Plan Preparation Evaluation

Date: 2026-09-21

Status: completed bounded evaluation

## Question

Can Ferris remove manual Action Plan assembly from an already valid owner
entrypoint declaration without discovering commands, choosing owner policy,
creating approval, or executing work?

## Controls

- Consumer: `ferris-synthetic-chain` at
  `b3d38baf47c8477b4d07ad71b24d6e3c0d71784c`.
- Owner command source: the repository README's authoritative
  `cargo test --workspace` oracle.
- One temporary `ferris.owner-entrypoints/v1` declaration and one selected
  entrypoint.
- Every lane, gate, required-status, timeout, and output-bound value was
  explicit.
- No workflow parsing, Cargo discovery, command inference, approval creation,
  execution by preparation, or retained consumer mutation was allowed.

## Implementation

`ferris prepare-action-plan` validates the existing declaration schema and
identities, requires its source revision to equal the current Git revision,
checks the repository-local executable, working directory, and bound file
identities, and copies the exact selected command into one dependency-free V1
lane. It emits an empty `approval_id`, computes the normal Action Plan identity,
and atomically creates a new repository-local output.

The preparer intentionally accepts no defaults for owner gate semantics or
execution bounds. It does not read inherited environment values.

## Result

- Both preparations succeeded on Windows.
- Both output files were byte-identical.
- Output SHA-256:
  `bee748f321325a725ea3819a058beb2b3959a6d141c8be7d679e4655da5359f6`.
- Action Plan ID:
  `sha256:4077a14ac9dc5921e89fdb2ddd539f37ba899be8a5c1ef28fb1477e548b0d972`.
- The plan preserved executable `.ferris/bin/cargo.exe` and argv
  `test --workspace` exactly as declared.
- `ferris go` rejected the unsigned plan before launch with
  `FERRIS-EXECUTION-IDENTITY-INVALID`, exit 2, and no receipt.
- The unchanged owner command `cargo test --workspace` passed independently.
- The consumer returned to a clean tracked and untracked state after removal
  of the temporary `.ferris/` tree.

## Product Finding

The boundary is justified only as a mechanical compiler from an already valid
declaration to one unsigned lane. It removes duplicate Action Plan authoring
without taking owner authority.

It does not solve the larger onboarding problem. The synthetic repository's
normal command resolves `cargo` from `PATH`, while V1 execution requires a
repository-relative, content-bound executable. This evaluation therefore
staged the resolved Cargo binary temporarily. The declaration was
evaluator-authored, not adopter-maintained, so this is public synthetic product
evidence rather than owner adoption.

Further preparation features stop here. The next evidence should come from one
adopter-maintained declaration pair or from a separately authorized executable
resolution contract; neither should be inferred from CI YAML or shell scripts.

## Reproduction

Ferris tests:

```console
cargo test -p ferris-cli --test execution prepares_deterministic_unsigned_action_plan_without_launching -- --exact
cargo test -p ferris-cli --test execution preparation_rejects_unknown_entrypoint_and_existing_output -- --exact
```

Consumer oracle:

```console
cargo test --workspace
```
