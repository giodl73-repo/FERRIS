# FSIM-039: Confusable Application Selection

Wave: W10
Revision: 1
State: Retraced
Claim state: simulated

## Question

Can a destructive action use a display name when two tenants have applications
that render as the same human-readable `forge-prod`?

## Locked fixture

- application: tenant `T1` and tenant `T2` each display `forge-prod`
- repositories and workspaces: independent private repositories
- source and change: operator requests deployment rollback
- contracts and profiles: both applications have rollback plans
- environment: interactive CLI with `T1` used most recently
- policy: mutation requires explicit canonical tenant and application IDs
- available evidence: operator supplies only display name `forge-prod`
- explicit unknowns: operator's intended tenant
- negative or matched control: operator supplies exact `T1/app-7`

Changing the fixture requires a new revision.

## Governing specifications

- VIEW-001 selection safety;
- EXECUTION-001 action request; and
- GOVERNANCE-001 tenant isolation.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Rollback request lacks canonical target | FSIM-SCR-022 |
| Scope | Two candidates remain | Ambiguous selector |
| Evidence | Last-used `T1` is context, not authority | Selection safety |
| Causality | Display equality establishes no identity | IDENTITY-001 |
| Prediction | Intent remains unknown | No guess |
| Validation | No target-specific rollback validation is selected | Invalid request |
| Planning | No mutating plan or action request is created | VIEW-001 |
| Resolution | Request explicit canonical tenant and application | Safe next action |
| Trust/action | No cross-tenant disclosure or mutation | GOVERNANCE-001 |
| Public view | Shows ambiguity without exposing unauthorized tenant details | Redaction |

## Assertions

- [x] last-used context does not select `T1`;
- [x] display name and prefix matching are insufficient;
- [x] no action request is created;
- [x] ambiguity does not disclose private candidate metadata; and
- [x] exact `T1/app-7` control may proceed to normal governance.

## Simulation issues

- `FSIM-SI-023`.

## Specification changes

- `FSIM-SCR-022`.

## Retrace

The fixture now rejects the display-only selector before planning or approval.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
