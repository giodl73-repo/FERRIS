# FSIM-013: Held-Out Outcome Leakage

Wave: W04
Revision: 1
State: Simulated
Claim state: simulated

## Question

Can a fixture still count as held out after its observed outcome is used to
revise the prompt, mapping, threshold, or heuristic?

## Locked fixture

- application: `forge`
- repositories and workspaces: three Rust workspaces
- source and change: frozen edit `H17`
- contracts and profiles: unchanged
- environment: supported Windows host
- policy: held-out outcomes must remain unavailable until prediction freeze
- available evidence: predictor `P3` omits workspace `cli`; full-reference
  observation later shows `cli` was affected
- explicit unknowns: performance on other unseen edits
- negative or matched control: a second fixture `H18` whose outcome remains
  sealed

Changing the fixture requires a new revision.

## Governing specifications

- PREDICTION-001 reference and held-out data;
- PREDICTION-001 comparison with observation; and
- VALIDATION-001 full-reference and held-out controls.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | `H17` identity is frozen before prediction | Held-out contract |
| Scope | Original prediction omits `cli` | Immutable Prediction Record |
| Evidence | Full-reference root reveals the omission after cutoff | Comparison record |
| Causality | Omission is measured, not rewritten | Original remains immutable |
| Prediction | Revising `P3` with the outcome reclassifies `H17` as development evidence | No leakage |
| Validation | False omission is recorded separately | VALIDATION-001 |
| Planning | Revised predictor cannot cite `H17` as held-out proof | Evidence class changed |
| Resolution | Claim narrows or awaits `H18` and other sealed fixtures | PREDICTION-001 disposition |
| Trust/action | No approval follows from leaked evaluation | Prediction is not authority |
| Public view | Shows original error, revision, and changed evidence class | Attributable history |

## Assertions

- [x] the original Prediction Record is not rewritten;
- [x] `H17` remains a recorded false omission;
- [x] tuned results on `H17` are development or calibration evidence;
- [x] `H18` may remain held out; and
- [x] leakage cannot improve the reported held-out score.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original specifications reject held-out relabeling unambiguously.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
