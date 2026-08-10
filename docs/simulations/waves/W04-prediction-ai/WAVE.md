# W04: Prediction and AI

Status: Complete after retrace
Claim state: simulated

## Goal

Test held-out integrity, work-reducing prediction admission, abstention under
distribution shift, and incomplete model invocation behavior.

## Locked specification baseline

Baseline commit: `f6060f7`

The retrace includes FSIM-SCR-008 and FSIM-SCR-009.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-013](FSIM-013-held-out-leakage.md) | A held-out result is used to revise the predictor | Evidence leakage and immutable scoring | Pass without spec change |
| [FSIM-014](FSIM-014-prediction-narrowing.md) | AI proposes removing a reverse dependent | Admission before work reduction | Pass after FSIM-SCR-008 |
| [FSIM-015](FSIM-015-distribution-shift.md) | Windows-trained predictor sees an unsupported Unix native boundary | Out-of-population abstention | Pass without spec change |
| [FSIM-016](FSIM-016-budget-exhausted-output.md) | Model budget ends after a parseable partial list | Partial output and safe fallback | Pass after FSIM-SCR-009 |

## Wave issues

- FSIM-SI-009: prediction confidence and calibration lacked a separate
  deterministic admission gate before reducing owner work; and
- FSIM-SI-010: parseable partial model output lacked an explicit ineligible
  completion state.

## Role review

- Rust Safety Steward: accepted after no prediction can reduce deterministic
  owner work without a current admission record.
- Compiler Performance Engineer: accepted because the unreduced baseline and
  measured false omissions remain available for comparison.
- Interop Boundary Auditor: accepted because unsupported native and contract
  populations trigger abstention instead of transfer by analogy.
- AI Assurance Skeptic: accepted after held-out leakage, confidence, partial
  output, and model explanations were denied owner authority.
- Ecosystem Strategist: accepted because predictor admission is replaceable
  and owner-native fallback remains canonical.
- Rust Maintainer: accepted because narrowing shows its floor, omitted work,
  expiry, and disable conditions.
- Native Platform Adopter: accepted for Draft after Windows evidence could not
  silently authorize Unix native-boundary narrowing.
- Scope Keeper: accepted as a bounded predictive wave without model or runtime
  implementation.
- Validation Checker: accepted after full-reference controls and partial-model
  failure were retraced.

## Disposition

Close W04 with no open P0 or P1 issues. Continue to W05 governance and action.
