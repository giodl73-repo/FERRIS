# FSIM-037: Evidence Service Loss During Action

Wave: W09
Revision: 1
State: Retraced
Claim state: simulated

## Question

Can an approved multi-step action continue mutating after the evidence and
revocation service becomes unavailable between side-effect barriers?

## Locked fixture

- application: `forge`
- repositories and workspaces: build producer and deployment owner
- source and change: approved build, sign, and deploy action
- contracts and profiles: exact supported identities
- environment: local build evidence plus remote trust and revocation service
- policy: current revocation state is required before signing and deployment
- available evidence: build completes; service becomes unavailable before sign
- explicit unknowns: current credential, signer, and approval revocation state
- negative or matched control: service recovers with current non-revoked state

Changing the fixture requires a new revision.

## Governing specifications

- EXECUTION-001 side-effect and revocation barriers;
- TRUST-001 running-action revocation; and
- VIEW-001 offline operation.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Service availability changes after local build | Environment event |
| Scope | Remaining sign and deploy side effects | Action barrier |
| Evidence | Local build result exists; current trust refresh is unavailable | Distinct evidence |
| Causality | Service loss causes unknown authority, not build failure | Claim separation |
| Prediction | Revocation state remains unknown | No cached success |
| Validation | Build validation remains observed; release validation incomplete | Stage independence |
| Planning | Wait, recover service, or request a new permitted offline plan | Explicit alternatives |
| Resolution | Existing action blocks before signing | Side-effect barrier |
| Trust/action | No sign or deploy begins | FSIM-SCR-018 and FSIM-SCR-020 |
| Public view | Shows retained build, blocked remaining work, and required refresh | VIEW-001 |

## Assertions

- [x] completed local build evidence is retained;
- [x] service loss is not relabeled revocation or success;
- [x] cached trust cannot authorize later side effects;
- [x] no signing or deployment starts; and
- [x] recovered current authority may resume only under the exact plan conditions.

## Simulation issues

- `FSIM-SI-019`;
- `FSIM-SI-021`.

## Specification changes

- `FSIM-SCR-018`;
- `FSIM-SCR-020`.

## Retrace

The fixture now preserves completed read/local evidence while blocking every
later mutation whose current authority cannot be verified.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
