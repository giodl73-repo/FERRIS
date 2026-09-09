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
`FERRIS-FEDERATED-VALIDATION-INPUT-BOUND-EXCEEDED` before application loading
or Cargo metadata. Removing one generated owner manifest immediately before
the overflow request proves that ordering.

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

## Role review

- **Scope Keeper — pass:** one test-only boundary is added; no product behavior,
  owner workflow, schema, or dependency changes.
- **Validation Checker — pass:** the accepted maximum, canonical orderings,
  deduplicated selection count, exact overflow diagnostic, process exit, and
  pre-metadata failure ordering are executable assertions.
- **Rust Maintainer — pass:** the fixture uses existing integration-test
  patterns and standard-library process/file APIs; removal is one helper and one
  test.
- **Product Value Governor — `continue-within-budget`:** the regression converts
  already-observed maximum-scale behavior into a durable public contract check
  without adding another product layer.

## Claim boundary

- generated workspaces are synthetic Tier 0 controls;
- identical JSON proves deterministic planning output for these permutations,
  not every possible input ordering;
- selected-package counts are planning scope, not build-time or cost savings;
- accepted requests still use ordinary Cargo metadata independently per
  workspace; and
- no validation command executes.
