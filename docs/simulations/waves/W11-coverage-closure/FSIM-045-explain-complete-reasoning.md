# FSIM-045: Complete Maintainer Explanation

Wave: W11
Revision: 1
State: Simulated
Claim state: simulated

## Question

Does `ferris explain` preserve why work was selected, omitted, reused,
blocked, widened, and left unknown without upgrading prediction to fact?

## Locked fixture

- application: `forge`
- repositories and workspaces: `core`, `service`, `cli`, and native installer
- source and change: public API edit in `core`
- contracts and profiles: `service` contract is mandatory; installer mapping
  is stale
- environment: supported host
- policy: mandatory validation and unknown native input widen conservatively
- available evidence: observed dependency to `service`, admitted prediction
  omitting advisory `cli`, reusable artifact for `core`, stale installer map
- explicit unknowns: exact installer native closure
- negative or matched control: explanation rendered without internal Forest terms

Changing the fixture requires a new revision.

## Governing specifications

- VIEW-001 explanation view;
- CAUSALITY-001 user explanations;
- PREDICTION-001 claim boundary; and
- PLANNING-001 owner-specific closures.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Public API Change Record | FOREST-002 |
| Scope | `core`, mandatory `service`, advisory `cli`, widened installer | SCOPE-001 |
| Evidence | Observed, predicted, stale, and reusable claims remain typed | FOREST-003 |
| Causality | `service` inclusion cites observed dependency | CAUSALITY-001 |
| Prediction | `cli` omission is admitted prediction, not observation | PREDICTION-001 |
| Validation | Mandatory service and installer fallback remain | VALIDATION-001 |
| Planning | Artifact reuse and widened native scope are separate | PLANNING-001 |
| Resolution | Explanation reports selected candidate and rejected narrower alternative | RESOLUTION-001 |
| Trust/action | No action or approval follows from explanation | VIEW-001 |
| Public view | Answers selected, why, omitted, unknown, owner, fallback, and changing evidence | Explanation contract |

## Assertions

- [x] every material reason cites evidence;
- [x] prediction is not relabeled observation;
- [x] reuse does not imply validation;
- [x] stale native mapping exposes widening and needed evidence; and
- [x] maintainer language does not require Forest internals.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The existing explanation contract covers every required question and claim
class.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
