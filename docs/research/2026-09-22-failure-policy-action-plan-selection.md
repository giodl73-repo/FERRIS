# Failure Policy Action Plan Selection

Status: Complete bounded successor

## Control Record

- Product outcome: turn an explicit `prepare_action` failure-policy decision
  into an unsigned owner Action Plan without command or policy inference.
- Prior work: passive Cargo diagnosis and owner failure-policy matching were
  complete; the decision could not select an entrypoint.
- Maximum effort: one single-lane bridge over existing V1 contracts and one
  focused execution-contract test expansion.
- Completion: deterministic preparation plus fail-closed controls for
  non-action, tampered, out-of-root, oversized, and unmapped decisions.
- Abandonment: any requirement for automatic approval, execution, a new plan
  schema, multi-lane inference, or another orchestration layer.
- Product Value Governor disposition: `continue-within-budget`.

## Implementation

`prepare-action-plan --failure-decision <JSON>` now replaces the explicit
`--entrypoint` selector in single-lane mode. The command:

1. requires a repository-local decision file no larger than 128 KiB;
2. validates the complete `failure-policy` command result, record identity,
   command identities, current Ferris version, and passive invariants;
3. accepts only `prepare_action`;
4. treats `owner_action_id` as an entrypoint ID in the separately supplied
   `ferris.owner-entrypoints/v1` declaration;
5. retains every other caller-supplied lane policy field;
6. rechecks decision bytes before atomic output creation; and
7. emits the existing unsigned `ferris.action-plan/v1`.

No Action Plan, approval, or process is created by `failure-policy` itself.
The plan is created only by the separate preparation command.

## Result

A real `diagnose-cargo` dependency report flowed through `failure-policy` to a
`prepare_action` decision selecting `owner/test`. Preparing from that decision
produced byte-identical Action Plan output to direct explicit selection of the
same entrypoint. The plan had an empty `approval_id`, and no receipt or owner
process was created.

Negative controls rejected:

- `halt` and `route` dispositions;
- a modified `owner_action_id` with stale identities;
- a valid `prepare_action` naming an undeclared owner action;
- a decision outside the repository root; and
- a decision larger than 128 KiB.

The contract catalog now marks `ferris.failure-policy-decision/v1` as accepted
and emitted.

## Boundaries

The bridge does not infer commands, lane identity, gate identity, requiredness,
dependencies, topology, timeouts, output bounds, approval, or execution.
Multi-lane preparation remains owner-authored through
`ferris.action-plan-lanes/v1`.

Action Plan V1 has no failure-decision field. It binds the selected entrypoint
and command, but not the policy, classification, or decision ID that selected
them. Owners must retain the decision and plan as separate audit artifacts.
Adding embedded provenance requires a separately reviewed versioned contract.

## Validation

Validated on Windows with Cargo
`1.95.0 (1.95.0-ms-20260618.5+ed80dadd6a)`:

| Command | Result |
|---|---|
| `cargo fmt -p ferris-core -- --check` | passed |
| `rustfmt --edition 2024 --check crates/ferris-cli/src/entrypoint.rs crates/ferris-cli/tests/execution.rs crates/ferris-cli/tests/failure_policy.rs` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed |
| `cargo test -p ferris-core --lib` | 131 passed, 2 ignored |
| `cargo test -p ferris-cli --bin ferris` | 10 passed |
| `cargo test -p ferris-cli --test cli` | 62 passed |
| `cargo test -p ferris-cli --test cargo_failure` | 5 passed |
| `cargo test -p ferris-cli --test contracts` | 7 passed |
| `cargo test -p ferris-cli --test failure_policy` | 5 passed |
| `cargo test -p ferris-cli --test execution` | 25 passed |

Workspace-wide `cargo fmt --all -- --check` remains excluded because Rust 1.95
reports unrelated historical CLI fixture formatting deltas. Those files were
not rewritten.

## Decision

The approved bridge is complete and stops at unsigned single-lane preparation.
Automatic approval, execution, multi-lane failure orchestration, and Action
Plan provenance expansion remain unauthorized follow-on work.
