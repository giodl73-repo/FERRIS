# Pulse 01: Eight-Workspace Value Measurement

Status: Complete
Implementation authority: Bounded to this document
Budget: One measurement pass, one correction if validation finds a defect

## Outcome

Measure how much workspace validation scope the existing
`federated-validation-plan` can avoid on a fixed eight-workspace application
while preserving transitive and application-wide fallback.

## Scenarios

| Scenario | Explicit input | Expected required scopes | Expected avoided scopes |
|---|---|---:|---:|
| Leaf | `ferris.benchmark/cli:fvb-cli` | 1 | 7 |
| Shared | `ferris.benchmark/domain:fvb-domain` | 5 | 3 |
| Application | `application-policy.txt` | 8 | 0 |

The shared scenario requires the directly changed `domain` workspace plus
`api`, `worker`, `cli`, and `admin` through explicit reverse relationships.
The application-owned path has no workspace owner and must widen all eight.

## Measured result

The deterministic test passed all three scenarios against the unchanged
`ferris.federated-validation-plan/v0` output:

| Scenario | Required scopes | Avoided scopes | Structural reduction |
|---|---:|---:|---:|
| Leaf | 1 | 7 | 87.5% |
| Shared | 5 | 3 | 37.5% |
| Application | 8 | 0 | 0% |

The explicit local overhead run used seven samples per scenario and reported:

| Scenario | Median planning overhead |
|---|---:|
| Leaf | 635.766 ms |
| Shared | 626.885 ms |
| Application | 635.402 ms |

Command:

```console
cargo +nightly test -p ferris-cli \
  --test federated_validation_value \
  report_local_planning_overhead_and_scope_reduction \
  --locked -- --ignored --nocapture
```

Environment:

- Windows 11 Enterprise Insider Preview `10.0.26310`;
- Intel Core i7-12800HX, 24 logical processors;
- 34,042,929,152 visible memory bytes;
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`;
- `cargo 1.99.0-nightly (c79e8f894 2026-08-04)`; and
- local NTFS checkout under `C:\src\FERRIS-value-benchmark`.

Each scenario still loaded Cargo metadata for all eight workspaces. Similar
planning medians across the scenarios are therefore expected and confirm that
the current value is scope explanation and conservative owner-work avoidance,
not reduced planner work.

Validation:

```console
rustfmt +nightly --edition 2024 --check crates/ferris-cli/tests/federated_validation_value.rs
cargo +nightly test -p ferris-cli --test federated_validation_value --locked --quiet
cargo +nightly test -p ferris-cli --test federated_validation_plan --locked --quiet
cargo +nightly test -p ferris-cli --test federated_validation_value report_local_planning_overhead_and_scope_reduction --locked -- --ignored --nocapture
cargo +nightly check --workspace --locked --quiet
cargo +nightly clippy -p ferris-cli --test federated_validation_value --locked -- -D warnings
git diff --check
```

The existing federated-validation suite passed eight tests. The normal value
test passed one structural test with the measurement test ignored. The
explicit measurement test passed all 21 planner invocations. Workspace check,
targeted Clippy, focused rustfmt, JSON parsing, and diff hygiene passed.

## Boundaries

- every fixture workspace contains one package, so workspace and package-scope
  counts are equal only for this fixture;
- scope reduction is not a build-time, test-time, command-count, or cost claim;
- the planner still loads Cargo metadata for all declared workspaces;
- timings describe local planning overhead only;
- validation remains non-executable and owner-controlled; and
- the fixture is synthetic Tier 0 evidence, not production adoption evidence.
