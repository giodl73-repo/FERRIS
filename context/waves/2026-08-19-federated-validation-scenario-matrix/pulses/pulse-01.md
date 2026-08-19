# Pulse 01: Scenario Response Matrix

Status: Complete
Implementation authority: Bounded to this document
Budget: One measurement pass, one correction if validation finds a defect

## Outcome

Validate that the unchanged `federated-validation-plan` response is
monotonic across graph depth, unions independent branches, retains nested
workspace fallback, and has order-independent semantic identity.

## Measured structural matrix

| Scenario | Required scopes | Avoided scopes | Reduction |
|---|---:|---:|---:|
| Foundation package | 8 | 0 | 0% |
| Contracts package | 6 | 2 | 25% |
| Domain package | 5 | 3 | 37.5% |
| API package | 3 | 5 | 62.5% |
| CLI package | 1 | 7 | 87.5% |
| API + analytics packages | 4 | 4 | 50% |
| Domain manifest | 5 | 3 | 37.5% |
| Domain manifest + analytics package | 6 | 2 | 25% |

The package-depth curve is monotonic for this topology: moving a direct
change from the shared foundation toward a leaf never selects additional
reverse dependents. The independent API and analytics inputs produce the
exact union `{admin, analytics, api, cli}`. Reversing those two package
arguments preserves selection, invocation, and federated-validation-plan
identities.

The domain `Cargo.toml` is directly owned by the domain workspace. Its nested
single-workspace validation record requires the full-workspace fallback, then
the application relationship layer adds the four declared reverse dependents.
The top-level application fallback remains false because ownership is known.
Adding analytics selects that independent branch without widening foundation,
contracts, or the whole application.

## Claim boundary

These are deterministic structural results over one synthetic topology. They
do not measure owner command count, build/test latency, cache reuse, package
cost, production accuracy, or application relationship correctness. The
planner continues to load Cargo metadata for all eight workspaces.

## Validation

```console
rustfmt +nightly --edition 2024 --check crates/ferris-cli/tests/federated_validation_value.rs
cargo +nightly test -p ferris-cli --test federated_validation_value --test federated_validation_plan --locked --quiet
cargo +nightly check --workspace --locked --quiet
cargo +nightly clippy -p ferris-cli --test federated_validation_value --locked -- -D warnings
git diff --check
```

The focused value suite passed three structural tests with the opt-in timing
test ignored. The unchanged federated-validation regression suite passed all
eight tests. Workspace check, targeted Clippy, focused rustfmt, fixture JSON
parsing, and diff hygiene passed.
