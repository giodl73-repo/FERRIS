# FSIM-036: Clock Skew at Approval Expiry

Wave: W09
Revision: 1
State: Retraced
Claim state: simulated

## Question

May execution begin when the client clock says approval remains valid but the
authority clock and uncertainty interval straddle expiry?

## Locked fixture

- application: `forge`
- repositories and workspaces: one release workspace
- source and change: approved publication action
- contracts and profiles: unchanged
- environment: client clock is 90 seconds slow; authority clock uncertainty is
  plus or minus 20 seconds
- policy: approval expires at authority time `18:00:00Z`
- available evidence: client reports `17:59:20Z`; authority evidence interval
  is `17:59:50Z` through `18:00:30Z`
- explicit unknowns: exact authority instant
- negative or matched control: synchronized interval wholly before expiry

Changing the fixture requires a new revision.

## Governing specifications

- FOREST-002 time evidence;
- GOVERNANCE-001 approval; and
- TRUST-001 time-sensitive eligibility.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Time evidence conflicts around the approval boundary | FSIM-SCR-020 |
| Scope | Exact approval and publication action | Governance |
| Evidence | Clock sources, skew, sync, and uncertainty are retained | Time evidence |
| Causality | Client clock cannot extend authority-issued approval | Owner authority |
| Prediction | Exact validity is unknown | Boundary overlap |
| Validation | Preflight cannot establish current approval | GOVERNANCE-001 |
| Planning | Existing plan remains historical | No rewrite |
| Resolution | Renew approval or defer | Conservative result |
| Trust/action | Publication is blocked before side effect | FSIM-SCR-020 |
| Public view | Shows uncertain expiry rather than expired or valid guess | VIEW-001 |

## Assertions

- [x] the favorable client clock does not win;
- [x] uncertainty interval is part of the decision;
- [x] mutation is blocked at the overlapping boundary;
- [x] approval history remains intact; and
- [x] the wholly-before-expiry control may proceed.

## Simulation issues

- `FSIM-SI-021`.

## Specification changes

- `FSIM-SCR-020`.

## Retrace

The fixture now selects the conservative blocked result when accepted time
uncertainty overlaps expiry.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
