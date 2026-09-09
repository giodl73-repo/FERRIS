# Pulse 01: Maximum Mixed-Input Regression

Status: Complete
Implementation authority: Bounded to this document
Budget: One generated fixture, one focused regression, one correction if
validation finds a defect

## Outcome

Protect the existing 256-explicit-input limit, canonical input ordering, and
path/package deduplication at the maximum declared 16-workspace application
size.

## Structural expectations

The generated application contains 16 independent Cargo workspaces with eight
packages each. One request supplies:

- 128 workspace-qualified package names;
- the same 128 packages' source paths; and
- no relationship, application-policy, or owner-execution input.

The accepted request must produce 16 direct workspace plans, select exactly 128
packages, and serialize identically for forward, reverse, and rotated argument
orders.

Adding one application-policy path creates 257 explicit inputs. That request
must return result class `blocked`, process exit code `7`, and diagnostic
`FERRIS-FEDERATED-VALIDATION-INPUT-BOUND-EXCEEDED` before Cargo metadata.
Removing one generated owner manifest immediately before the overflow request
proves that ordering.

## Result

All three 256-input orderings produced identical complete JSON, 16 direct
workspace plans, and exactly 128 selected packages. The 257-input request
returned the exact typed blocked result after an owner manifest was removed,
proving the bound is enforced before Cargo metadata.

The child process runs from the generated application root and receives a
relative application filename and relative changed paths. This keeps the
maximum-input invocation below the Windows command-line limit without changing
the input semantics.

## Validation

```console
rustfmt --edition 2024 --check crates/ferris-cli/tests/federated_validation_scaling.rs
cargo test --locked -p ferris-cli --test federated_validation_scaling
cargo clippy --locked -p ferris-cli --test federated_validation_scaling -- -D warnings
git diff --check
```

The focused integration test passed with three tests successful and the
measurement-only test ignored. Targeted Clippy and focused rustfmt passed.
Repository-wide rustfmt also reported pre-existing formatting drift in
unrelated pulse tests; this pulse does not change those files.

Validation environment:

- Microsoft Windows NT `10.0.26310.0`;
- `rustc 1.95.0 (ed80dadd6a 2026-06-18)
  (1.95.0-ms-20260618.5+ed80dadd6a)`; and
- `cargo 1.95.0 (1.95.0-ms-20260618.5+ed80dadd6a)`.

## Role review

- **Product Value Governor — `continue-within-budget`:** the regression converts
  already-observed maximum-scale behavior into a durable public contract check
  without adding another product layer.
- **Rust Safety Steward — accept:** the change adds test-only safe Rust and
  makes no memory-safety or soundness claim.
- **Compiler Performance Engineer — accept without a performance claim:** the
  test asserts structure and typed failure only; it records no timing or
  representative iteration-speed conclusion.
- **Interop Boundary Auditor — accept as not applicable:** no FFI, ABI,
  language boundary, binding, or migration behavior changes.
- **AI Assurance Skeptic — accept with bounded claims:** executable assertions
  establish only the named generated-fixture behaviors, and the claim boundary
  excludes universal determinism, performance, and correctness claims.
- **Ecosystem Strategist — accept:** the regression protects existing Ferris
  behavior around Cargo-owned workspaces and adds no competing resolver, tool,
  language, or dependency.
- **Rust Maintainer — accept:** the fixture uses existing integration-test
  patterns and standard-library process/file APIs; removal is one helper and
  one test.
- **Native Platform Adopter — accept:** relative child-process arguments keep
  the maximum-input test viable on Windows without changing owner workflows or
  platform contracts.
- **Scope Keeper — accept:** one test-only boundary is added; no product
  behavior, owner workflow, schema, or dependency changes.
- **Validation Checker — accept:** the accepted maximum, canonical orderings,
  deduplicated selection count, exact overflow diagnostic, process exit, and
  pre-metadata failure ordering are executable assertions.
- **Autonomy Supervisor — accept and stop after publication:** the user's
  explicit continuation authorized this one bounded upstream regression; no
  successor pulse or adjacent hardening is implied.

## Closeout

- Completed revisions: authority `1804038`; regression `17a0ebb` and
  `22ceb4f`; this review correction.
- Remaining gates: clean branch review and ordinary pull-request CI before
  merge.
- Implementation authority: exhausted after this one generated fixture and
  focused regression; successor authority remains none.

## Claim boundary

- generated workspaces are synthetic Tier 0 controls;
- identical JSON proves deterministic planning output for these permutations,
  not every possible input ordering;
- selected-package counts are planning scope, not build-time or cost savings;
- accepted requests still use ordinary Cargo metadata independently per
  workspace; and
- no validation command executes.
