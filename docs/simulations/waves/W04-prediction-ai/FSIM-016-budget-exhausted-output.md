# FSIM-016: Budget-Exhausted Model Output

Wave: W04
Revision: 1
State: Retraced
Claim state: simulated

## Question

May Ferris normalize and use a parseable package list when the model stops
because its token or cost budget is exhausted before completing the response?

## Locked fixture

- application: `forge`
- repositories and workspaces: `core`, `service`, `cli`, and `installer`
- source and change: shared configuration edit
- contracts and profiles: all four workspaces are in the deterministic
  application closure
- environment: supported host
- policy: model budget is fixed and cannot be exceeded automatically
- available evidence: model output parses as `core, service, cli` but reports
  budget exhaustion before its final item and explanation
- explicit unknowns: whether `installer` would have been included
- negative or matched control: a complete schema-valid response containing all
  four workspaces

Changing the fixture requires a new revision.

## Governing specifications

- PREDICTION-001 model invocation outcome and safe fallback;
- GOVERNANCE-001 resource and cost budget; and
- VIEW-001 model failure display.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Shared configuration Change Record | FOREST-002 |
| Scope | Deterministic application closure includes all four workspaces | SCOPE-001 |
| Evidence | Invocation state is budget-exhausted | FSIM-SCR-009 |
| Causality | Partial text proves no complete affected-set claim | AI boundary |
| Prediction | No model-produced Prediction Record is admitted | FSIM-SCR-009 |
| Validation | Deterministic validation floor remains unchanged | VALIDATION-001 |
| Planning | Uses baseline, requests another approved invocation, or blocks | Safe fallback |
| Resolution | Partial list is ineligible as a narrowing candidate | Completion prerequisite |
| Trust/action | Budget policy is not bypassed and no work executes | GOVERNANCE-001 |
| Public view | Shows incomplete attempt, diagnostics, and selected fallback | VIEW-001 |

## Assertions

- [x] parseability does not imply completeness;
- [x] `installer` cannot be omitted using the partial response;
- [x] budget exhaustion remains distinct from provider failure;
- [x] retry requires a separately permitted budget or fallback; and
- [x] the complete negative control may proceed to deterministic normalization.

## Simulation issues

- `FSIM-SI-010`.

## Specification changes

- `FSIM-SCR-009`.

## Retrace

The locked fixture now classifies the invocation as budget-exhausted, records
no complete model-produced Prediction Record, and retains the deterministic
baseline.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
