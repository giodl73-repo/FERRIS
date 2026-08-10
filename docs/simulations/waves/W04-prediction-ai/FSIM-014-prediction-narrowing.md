# FSIM-014: Prediction-Based Work Narrowing

Wave: W04
Revision: 1
State: Retraced
Claim state: simulated

## Question

When may a high-confidence AI prediction remove a reverse-dependent workspace
from the deterministic owner closure?

## Locked fixture

- application: `forge`
- repositories and workspaces: `core`, `service`, and `cli`
- source and change: private body edit in `core`
- contracts and profiles: no declared public contract change
- environment: supported Windows and Unix population
- policy: mandatory gates remain; admitted predictor may reduce advisory
  reverse-dependent work when false omissions remain below threshold
- available evidence: deterministic baseline selects all three workspaces;
  model `M7` predicts `core` and `service` only with confidence 0.97
- explicit unknowns: whether `cli` uses an undeclared generated input
- negative or matched control: the same prediction with expired calibration

Changing the fixture requires a new revision.

## Governing specifications

- PREDICTION-001 work-reducing admission;
- VALIDATION-001 selection rule; and
- PLANNING-001 owner-specific closures.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Canonical private-body Change Record | FOREST-002 |
| Scope | Deterministic baseline selects `core`, `service`, and `cli` | SCOPE-001 |
| Evidence | `M7` result is prediction, not owner truth | PREDICTION-001 |
| Causality | No observed edge is removed | CAUSALITY-001 boundary |
| Prediction | Candidate reduction omits advisory `cli` work | Named prediction class |
| Validation | Mandatory floor and full-reference cadence remain | FSIM-SCR-008 |
| Planning | Reduction is eligible only with a current matching admission | FSIM-SCR-008 |
| Resolution | Policy or required human approval selects baseline or admitted candidate | Prediction is not approval |
| Trust/action | Approved Action Plan must bind the selected exact plan | GOVERNANCE-001 |
| Public view | Shows baseline, predicted reduction, floor, unknown, expiry, and fallback | PLANNING-001 |

## Assertions

- [x] confidence 0.97 alone cannot remove `cli`;
- [x] admission is versioned separately from the Prediction Record;
- [x] mandatory validation and deterministic minimum floor remain;
- [x] the unreduced baseline remains queryable; and
- [x] expired calibration restores the baseline or blocks.

## Simulation issues

- `FSIM-SI-009`.

## Specification changes

- `FSIM-SCR-008`.

## Retrace

The locked fixture now has one eligibility rule: use the reduction only under
a current admission matching the predictor, population, thresholds, floor,
policy, and expiry. The negative control cannot narrow.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
