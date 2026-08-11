# FSIM-042: Stale Cross-Tenant Action Plan ID

Wave: W10
Revision: 1
State: Simulated
Claim state: simulated

## Question

What happens when an operator pastes a valid but expired Action Plan ID issued
for another tenant into `ferris run`?

## Locked fixture

- application: tenant `T1` application `forge`
- repositories and workspaces: private `T1` and `T2` workspaces
- source and change: operator authenticated only for `T1`
- contracts and profiles: unchanged
- environment: CLI receives `T2` Action Plan ID `A77`
- policy: exact tenant, principal, plan, approval, expiry, and disclosure checks
- available evidence: `A77` exists, is expired, and belongs to `T2`
- explicit unknowns: how operator obtained the opaque ID
- negative or matched control: current approved `T1` Action Plan ID

Changing the fixture requires a new revision.

## Governing specifications

- EXECUTION-001 approval binding and preflight;
- GOVERNANCE-001 tenant isolation; and
- VIEW-001 selection safety.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Invocation supplies an opaque foreign action identity | Input |
| Scope | Authenticated `T1` only | Governance |
| Evidence | Unauthorized `T2` details remain undisclosed | Data boundary |
| Causality | Possession of an ID creates no authority | Authentication separation |
| Prediction | Operator intent remains unknown | No inference |
| Validation | No `T2` plan details are evaluated for the user | Tenant isolation |
| Planning | No new plan is created | Exact run form |
| Resolution | Request is denied or invalid without existence disclosure | Governance |
| Trust/action | No preflight side effect or connector call | EXECUTION-001 |
| Public view | Generic inaccessible, invalid, or stale-safe diagnostic for the caller | Non-disclosure |

## Assertions

- [x] opaque ID possession grants no access;
- [x] response does not confirm private `T2` metadata;
- [x] expired state cannot be used to execute;
- [x] no action or external call begins; and
- [x] the current exact `T1` control proceeds to normal preflight.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

Existing exact binding, tenant isolation, expiry, and selection rules provide
one safe trace.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
