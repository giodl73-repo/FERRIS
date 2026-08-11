# FSIM-032: Post-Preflight Owner-State Race

Wave: W08
Revision: 1
State: Retraced
Claim state: simulated

## Question

May an approved rollback overwrite a newer deployment when the owner state
changes after preflight but before the mutation request?

## Locked fixture

- application: `forge`
- repositories and workspaces: release producer and deployment owner
- source and change: approved rollback from deployment generation `D4` to `D3`
- contracts and profiles: rollback profile is valid for expected current `D4`
- environment: external deployment API supports conditional generation writes
- policy: never overwrite a concurrent deployment
- available evidence: preflight observes `D4`; another actor promotes `D5`
  before the rollback mutation
- explicit unknowns: intent and validity of `D5`
- negative or matched control: generation remains `D4`

Changing the fixture requires a new revision.

## Governing specifications

- EXECUTION-001 side-effect and revocation barriers;
- IDENTITY-001 compare-and-set generations; and
- GOVERNANCE-001 approval binding.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Owner generation changes from `D4` to `D5` after preflight | Concurrent owner event |
| Scope | Exact deployment and rollback operation | Action Plan |
| Evidence | Immediate conditional guard observes generation mismatch | FSIM-SCR-017 |
| Causality | Concurrent promotion causes stale rollback precondition | Owner state |
| Prediction | `D5` safety and intent remain unknown | No overwrite inference |
| Validation | Prior rollback validation is stale for current state | Exact profile |
| Planning | Existing plan remains historical; replan from `D5` | PLANNING-001 |
| Resolution | Prior rollback disposition is stale | RESOLUTION-001 |
| Trust/action | Conditional mutation fails without overwriting `D5` | FSIM-SCR-017 |
| Public view | Shows stale expected generation and no side effect | VIEW-001 |

## Assertions

- [x] successful preflight is not sufficient at mutation time;
- [x] rollback request binds expected generation `D4`;
- [x] `D5` is not overwritten;
- [x] unknown intent does not authorize retry; and
- [x] the unchanged-generation control may perform the conditional rollback.

## Simulation issues

- `FSIM-SI-018`.

## Specification changes

- `FSIM-SCR-017`.

## Retrace

The fixture now uses the owner-native conditional generation immediately at
mutation time and fails stale without side effects.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
