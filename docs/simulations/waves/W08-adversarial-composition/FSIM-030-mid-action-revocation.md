# FSIM-030: Mid-Action Emergency Revocation

Wave: W08
Revision: 1
State: Retraced
Claim state: simulated

## Question

What happens when approval and credential eligibility are revoked while a
non-interruptible signing operation is running but before publication begins?

## Locked fixture

- application: `forge`
- repositories and workspaces: release producer
- source and change: approved sign-and-publish action
- contracts and profiles: signing and publication are separate owner operations
- environment: signing service and package registry connector
- policy: emergency revocation stops future side effects immediately
- available evidence: signing operation `S8` has passed its safe stop point;
  approval and credential class are revoked before publish operation `P8`
- explicit unknowns: whether `S8` will finish successfully
- negative or matched control: revocation after publication completes

Changing the fixture requires a new revision.

## Governing specifications

- TRUST-001 revocation;
- EXECUTION-001 side-effect and revocation barriers; and
- EXECUTION-001 cancellation protocol.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Emergency revocation event is recorded | FSIM-SCR-016 |
| Scope | Approval, credential, signing, and publication operations | Exact action |
| Evidence | `S8` non-interruptibility and revocation time remain visible | Owner state |
| Causality | Revocation blocks later publication but may not erase completed signing | Safe point |
| Prediction | Signing completion remains unknown | No invented stop |
| Validation | Signed artifact is not publication-eligible | Revoked authority |
| Planning | Original plan remains immutable; recovery or disposal may need a new plan | No rewrite |
| Resolution | Stop before `P8`; isolate any signed output | Revocation barrier |
| Trust/action | Enter cancellation and cleanup; never start publication | FSIM-SCR-016 |
| Public view | Shows revocation, owner-deferred signing, isolated effect, and blocked publish | VIEW-001 |

## Assertions

- [x] no publication begins after applicable revocation;
- [x] non-interruptible signing follows its safe point;
- [x] a completed signature does not restore approval;
- [x] signed output remains isolated and ineligible; and
- [x] the post-publication control records an external residual effect.

## Simulation issues

- `FSIM-SI-017`.

## Specification changes

- `FSIM-SCR-016`.

## Retrace

The fixture now checks revocation before publication, stops new work, and
preserves the independent signing effect and cleanup path.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
