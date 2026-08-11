# FSIM-004: Entrypoint and Command Defaults

Wave: W01
Revision: 2
State: Retraced
Claim state: simulated

## Question

Can a maintainer predict whether `ferris check`, `cargo ferris check`, and an
MCP planning tool execute work or only produce a plan?

## Locked fixture

- Current directory is the `ledger-app` Cargo workspace.
- Application `ledger-cli` contains both `ledger-app` and `ledger-lib`.
- Explicit source, configuration, tool, and evidence identities are equal
  where parity is compared.
- No Approval Record or Action Plan exists.
- The MCP client has read/planning authorization only.

Negative control: `ferris run --action-plan AP-42` with a valid exact approval
is described but not executed.

## Governing specifications

- PRODUCT-001 entrypoints;
- VIEW-001 commands, scope defaults, and safety defaults;
- GOVERNANCE-001 authorization and approval;
- CONNECTOR-001 MCP adapter; and
- EXECUTION-001 exact action projection.

## Initial hand trace

`check` and `test` were described as “plan and optionally execute,” which did
not define whether execution was the default, a flag, an action request, or an
approved-plan operation.

Initial issue: FSIM-SI-003.

## Retraced expected behavior

| Invocation | Predicted behavior |
|---|---|
| `cargo ferris check` | Produce a non-executable check plan scoped by default to the current Cargo workspace |
| `ferris check --workspace ledger-app` | Produce the equivalent semantic plan for the same explicit workspace |
| `ferris check --application ledger-cli` | Produce an application-scoped plan that may include both workspaces |
| `ferris check --application ledger-cli --request-action` | Display the plan and create an action request; do not execute |
| MCP `ferris.plan` for check | Produce the same semantic plan for identical explicit inputs |
| MCP action tool without approval | Create or return an action request; execution denied |
| `ferris run --action-plan AP-42` | Eligible for preflight only when exact plan, resolution, trust, policy, and approval remain valid |

Process results use the fixed VIEW-001 mapping:

- successful plan production returns `0`;
- an invalid selector or invocation returns `2`;
- the unapproved MCP action attempt returns `3`;
- stale Action Plan evidence returns `6`; and
- an unavailable required precondition returns `7`.

## Assertions

- [x] `check` and `test` are plan-only by default.
- [x] `--request-action` does not imply approval or execution.
- [x] `run --action-plan` is the initial explicit execution form.
- [x] `cargo ferris` does not discover sibling workspaces by default.
- [x] Explicit identical scope produces semantic parity.
- [x] MCP discovery and protocol authorization cannot bypass Ferris approval.
- [x] Exact numeric process exit codes are predictable.

## Simulation issues

- FSIM-SI-003 resolved by FSIM-SCR-003.
- FSIM-SI-004 resolved by FSIM-SCR-025.

## Specification changes

- FSIM-SCR-003; and
- FSIM-SCR-025.

## Claim boundary

The table predicts semantic behavior and fixed process results only. No
executable or MCP server behavior is observed by this simulation.
