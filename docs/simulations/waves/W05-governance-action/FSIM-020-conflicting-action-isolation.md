# FSIM-020: Conflicting Writable Actions

Wave: W05
Revision: 1
State: Simulated
Claim state: simulated

## Question

Can two separately approved Action Plans execute concurrently against the same
writable Cargo worktree and target state?

## Locked fixture

- application: `forge`
- repositories and workspaces: one Cargo workspace and one writable worktree
- source and change: Action Plans `A10` and `A11` use different revisions
- contracts and profiles: each plan is independently eligible
- environment: both requests arrive on one host
- policy: mutable worktree and target state must be isolated per action
- available evidence: `A10` acquired the approved writable paths first
- explicit unknowns: when `A10` will release its paths
- negative or matched control: `A11` uses a separate worktree and target state

Changing the fixture requires a new revision.

## Governing specifications

- EXECUTION-001 Action Plan, preflight, and execution behavior;
- GOVERNANCE-001 tenant isolation; and
- PLANNING-001 resource envelope.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Two exact approved actions request conflicting writable state | Action identities remain separate |
| Scope | Same worktree and target paths | Mutation boundary |
| Evidence | `A10` holds the paths | Preflight conflict |
| Causality | Approval does not remove physical conflict | Authorization is not availability |
| Prediction | Wait duration remains unknown | No invented release time |
| Validation | Neither plan may validate against mixed outputs | Isolation |
| Planning | `A11` may wait, replan to isolated paths, or block | Resource fallback |
| Resolution | Existing approval cannot silently change `A11` paths | Exact Action Plan binding |
| Trust/action | `A11` preflight blocks before mutation | Conflicting running action |
| Public view | Shows holder, conflict boundary, and safe alternatives | VIEW-001 |

## Assertions

- [x] two approvals do not authorize shared writable state;
- [x] `A11` cannot mutate before conflict resolution;
- [x] changing paths requires a new exact Action Plan and approval;
- [x] mixed artifacts cannot become validation evidence; and
- [x] the isolated-path control may proceed independently.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original preflight and isolation rules block the conflicting action.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
