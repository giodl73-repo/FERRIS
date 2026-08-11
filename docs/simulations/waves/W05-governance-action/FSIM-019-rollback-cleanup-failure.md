# FSIM-019: Rollback Success with Cleanup Failure

Wave: W05
Revision: 1
State: Retraced
Claim state: simulated

## Question

How does Ferris represent an action whose deployment rollback restores the
prior version while cleanup fails and an external session remains open?

## Locked fixture

- application: `forge`
- repositories and workspaces: release producer and deployment owner
- source and change: approved deployment of artifact `A2`
- contracts and profiles: deployment contract requires rollback and cleanup
- environment: supported provider
- policy: any retained credential or external session requires escalation
- available evidence: deployment fails; rollback restores `A1`; temporary
  worktree deletion succeeds; connector session close fails
- explicit unknowns: whether the remote session can still mutate state
- negative or matched control: session close succeeds

Changing the fixture requires a new revision.

## Governing specifications

- EXECUTION-001 rollback and cleanup;
- EXECUTION-001 execution record; and
- TRUST-001 secrets and privacy.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Deployment attempt produces observed partial effects | Execution evidence |
| Scope | Deployment, rollback, local cleanup, and external session | Independent dimensions |
| Evidence | `A1` restored; session-close operation failed | Owner-local results |
| Causality | Rollback success does not cause cleanup success | Separate operations |
| Prediction | Remote mutation risk remains unknown | Unknown residual effect |
| Validation | Restored deployment may validate, but recovery remains open | Capability versus cleanup |
| Planning | Recovery action requires a new bounded plan if not preapproved | No silent extra action |
| Resolution | Escalate session revocation or owner recovery | TRUST-001 |
| Trust/action | Execution failed; rollback succeeded; cleanup failed; residual effect unknown | FSIM-SCR-011 |
| Public view | Summary is partial or failed and exposes all dimensions | VIEW-001 |

## Assertions

- [x] rollback success is retained;
- [x] cleanup failure is not overwritten by rollback;
- [x] unknown remote mutation capability remains visible;
- [x] overall result is not success; and
- [x] recovery owner and deadline are required.

## Simulation issues

- `FSIM-SI-012`.

## Specification changes

- `FSIM-SCR-011`.

## Retrace

The fixture now produces independent execution, rollback, cleanup, and
residual-effect states without forcing one state to erase another.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
