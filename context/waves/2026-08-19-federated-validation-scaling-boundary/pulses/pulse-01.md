# Pulse 01: Workspace Scaling Measurement

Status: Complete
Implementation authority: Bounded to this document
Budget: One measurement pass, one correction if validation finds a defect

## Outcome

Validate accepted application sizes through the declared 16-workspace maximum,
reject 17 workspaces before Cargo metadata loading, and measure local
sequential planning overhead.

## Structural expectations

For each accepted size 2, 4, 8, and 16:

- a change in the final chain workspace requires one direct scope and leaves
  every predecessor not selected; and
- a change in the first chain workspace requires one direct scope plus every
  transitive reverse dependent.

The 17-workspace definition must return result class `invalid`, process exit
code `2`, and diagnostic
`FERRIS-APPLICATION-WORKSPACE-COUNT-INVALID`.

## Measured result

All accepted sizes passed both structural cases. At 16 workspaces, a final
chain-leaf change required 1 direct scope and avoided 15 scopes, while a
first-workspace change required 1 direct scope plus 15 relationship fallbacks.
The 17-workspace definition returned the exact typed invalid result before
owner metadata loading.

The explicit local scaling run used five samples per accepted size:

| Workspaces | Median planning overhead | Approximate median per workspace |
|---:|---:|---:|
| 2 | 144.437 ms | 72.218 ms |
| 4 | 280.568 ms | 70.142 ms |
| 8 | 545.762 ms | 68.220 ms |
| 16 | 1,113.149 ms | 69.572 ms |

The 8x increase from 2 to 16 workspaces produced a 7.71x median increase in
this control. That is consistent with the current documented design: one
sequential Cargo metadata process per declared workspace, even when the
result requires only one leaf validation scope.

Command:

```console
cargo +nightly test -p ferris-cli \
  --test federated_validation_scaling \
  report_local_workspace_scaling \
  --locked -- --ignored --nocapture
```

Environment matches the first value proof:

- Windows 11 Enterprise Insider Preview `10.0.26310`;
- Intel Core i7-12800HX, 24 logical processors;
- 34,042,929,152 visible memory bytes;
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`; and
- `cargo 1.99.0-nightly (c79e8f894 2026-08-04)`.

Validation:

```console
rustfmt +nightly --edition 2024 --check crates/ferris-cli/tests/federated_validation_scaling.rs
cargo +nightly test -p ferris-cli --test federated_validation_scaling --locked --quiet
cargo +nightly test -p ferris-cli --test federated_validation_scaling report_local_workspace_scaling --locked -- --ignored --nocapture
cargo +nightly test -p ferris-cli --test federated_validation_value --test federated_validation_plan --locked --quiet
cargo +nightly check --workspace --locked --quiet
cargo +nightly clippy -p ferris-cli --test federated_validation_scaling --locked -- -D warnings
git diff --check
```

## Claim boundary

- generated workspaces are synthetic Tier 0 controls;
- the timing measures local planner process plus sequential Cargo metadata
  overhead, not validation, build, or test latency;
- one-package workspaces minimize metadata complexity and do not model large
  owner graphs;
- the test does not establish a support SLO or acceptable production latency;
  and
- no validation command executes.
