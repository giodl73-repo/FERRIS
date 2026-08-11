# FSIM-034: Offline Local Planning

Wave: W09
Revision: 1
State: Retraced
Claim state: simulated

## Question

What may a developer learn and plan while explicitly offline with retained
local roots but unavailable connector, policy, and revocation refresh?

## Locked fixture

- application: `forge`
- repositories and workspaces: one local Cargo workspace
- source and change: private body edit
- contracts and profiles: locally retained profile was current at cutoff `T1`
- environment: network disabled by explicit offline invocation
- policy: read-only local planning is allowed; mutation requires current remote
  revocation and policy checks
- available evidence: local root, manifests, lockfile, prior validation, and
  policy snapshot at `T1`
- explicit unknowns: changes after `T1`
- negative or matched control: network becomes available and evidence renews

Changing the fixture requires a new revision.

## Governing specifications

- VIEW-001 offline operation;
- EXECUTION-001 offline mutation; and
- PLANNING-001 evidence cutoff.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Local Change Record is available | FOREST-002 |
| Scope | Local workspace with named unknown external dimensions | Offline envelope |
| Evidence | Local sources retain cutoff `T1`; refresh is unavailable | FSIM-SCR-018 |
| Causality | Network absence does not imply no external change | Unknown remains |
| Prediction | Local affected scope may be forecast with cutoff limitations | PREDICTION-001 |
| Validation | Prior results remain historical or stale | VALIDATION-001 |
| Planning | Non-executable plan may be incomplete and conservative | VIEW-001 |
| Resolution | Request online evidence, use local full reference, or defer | Safe alternatives |
| Trust/action | Mutation is blocked because current revocation is not locally verifiable | EXECUTION-001 |
| Public view | Shows offline mode, cutoff, prohibited network, unknowns, and next evidence | VIEW-001 |

## Assertions

- [x] offline mode never contacts a connector;
- [x] unavailable refresh is not an empty result;
- [x] local Cargo evidence remains usable;
- [x] stale authority cannot enable mutation; and
- [x] the online control creates a distinct renewed result.

## Simulation issues

- `FSIM-SI-019`.

## Specification changes

- `FSIM-SCR-018`.

## Retrace

The fixture now permits bounded read-only planning while blocking any action
that depends on unavailable current authority.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
