# FSIM-010: Concurrent Channel Promotion

Wave: W03
Revision: 1
State: Simulated
Claim state: simulated

## Question

What happens when two authorized actors concurrently promote the same channel
from one expected generation to different roots?

## Locked fixture

- application: `forge`
- repositories and workspaces: two producer workspaces
- source and change: candidate roots `R21` and `R22` are both available
- contracts and profiles: both candidates pass their separate eligibility
  checks
- environment: unchanged
- policy: both actors may request promotion; channel `stable` is at generation
  7 and root `R20`
- available evidence: both updates declare expected root `R20`, generation 7
- explicit unknowns: network arrival order
- negative or matched control: replay the losing request after generation 8

Changing the fixture requires a new revision.

## Governing specifications

- IDENTITY-001 ref updates and generations;
- GOVERNANCE-001 authorization; and
- TRUST-001 refs and roots.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Two promotion requests target one channel generation | Requests remain separate |
| Scope | Channel `stable` in one tenant and owner namespace | IDENTITY-001 ref identity |
| Evidence | `R20`, `R21`, and `R22` remain immutable | FOREST-002 |
| Causality | Arrival order determines which valid compare-and-set is first | No semantic preference inferred |
| Prediction | None authorizes a winner | Policy and CAS are deterministic |
| Validation | Candidate validation remains attached to each root | Trust and validation are separate |
| Planning | Both requests expect generation 7 | IDENTITY-001 |
| Resolution | First accepted update creates generation 8; second is stale | Compare-and-set |
| Trust/action | Losing request requires reread and new approval if retried | Changed prior value |
| Public view | One success and one stale conflict retain both request histories | No lost update |

## Assertions

- [x] exactly one request can update generation 7;
- [x] the losing request cannot overwrite generation 8;
- [x] replay remains rejected;
- [x] both candidate roots and request histories remain queryable; and
- [x] arrival order is not relabeled a quality or trust decision.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original specifications produce one unambiguous compare-and-set result.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
