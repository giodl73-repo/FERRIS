# FSIM-017: Approved Plan with Preflight Drift

Wave: W05
Revision: 1
State: Simulated
Claim state: simulated

## Question

Can an approved Action Plan execute after the exact rustc toolchain identity
changes between approval and preflight?

## Locked fixture

- application: `forge`
- repositories and workspaces: two Cargo workspaces
- source and change: approved source revisions remain unchanged
- contracts and profiles: exact profile approved for rustc `1.91.0`
- environment: rustc changes to `1.91.1` before execution
- policy: approval binds exact tool versions and environment
- available evidence: valid Resolution Record, Action Plan, and unexpired
  approval for `1.91.0`
- explicit unknowns: compatibility of `1.91.1`
- negative or matched control: preflight with original rustc `1.91.0`

Changing the fixture requires a new revision.

## Governing specifications

- GOVERNANCE-001 approval;
- EXECUTION-001 approval binding and preflight; and
- PLATFORM-001 renewal.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Environment identity changes after approval | Material plan input |
| Scope | Exact approved application scope remains | No scope expansion |
| Evidence | Toolchain observation differs from Action Plan | Preflight evidence |
| Causality | Toolchain drift causes stale precondition | Direct identity comparison |
| Prediction | No compatibility is inferred | Profile identity is exact |
| Validation | Existing profile support is stale for `1.91.1` | PLATFORM-001 |
| Planning | Existing plan remains immutable; replan or prior environment is required | PLANNING-001 |
| Resolution | Prior resolution is stale for changed material environment | RESOLUTION-001 |
| Trust/action | Preflight blocks; approval is not rewritten | EXECUTION-001 |
| Public view | Shows stale toolchain, blocked execution, and alternatives | VIEW-001 |

## Assertions

- [x] unexpired approval does not override changed tool identity;
- [x] `1.91.1` compatibility is unknown;
- [x] the Action Plan and approval remain historical;
- [x] execution does not start; and
- [x] the matched control may pass this preflight condition.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original exact binding and preflight rules block the drift unambiguously.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
