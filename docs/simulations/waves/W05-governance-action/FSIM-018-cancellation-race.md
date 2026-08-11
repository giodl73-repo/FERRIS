# FSIM-018: Cancellation During Owner Operation

Wave: W05
Revision: 1
State: Retraced
Claim state: simulated

## Question

What does Ferris report when cancellation is requested while a
non-interruptible linker or external owner operation is already past its safe
stop point?

## Locked fixture

- application: `forge`
- repositories and workspaces: Rust workspace with native finalization
- source and change: approved release build
- contracts and profiles: unchanged and approved
- environment: supported host
- policy: stop launching dependent work immediately; respect owner safe points
- available evidence: linker operation `L5` is running and reports no safe
  interruption until output finalization
- explicit unknowns: whether `L5` completes before cancellation deadline
- negative or matched control: cancellation before `L5` starts

Changing the fixture requires a new revision.

## Governing specifications

- EXECUTION-001 cancellation protocol;
- EXECUTION-001 execution behavior; and
- GOVERNANCE-001 audit and attestation.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Authorized cancellation request is recorded | FSIM-SCR-010 |
| Scope | Remaining action and owner operation `L5` | Cancellation scope |
| Evidence | `L5` reports next safe point after finalization | Owner semantics |
| Causality | Request does not prove process termination | Cancellation protocol |
| Prediction | Completion timing remains unknown | No fabricated stop |
| Validation | Downstream validation is not launched | Stop dependent work |
| Planning | Original plan and Action Plan remain immutable | No rewrite |
| Resolution | Follow approved safe-point and cleanup rule | Declared action behavior |
| Trust/action | State progresses requested, acknowledged, owner-deferred, then cancelled or completed-before-stop | FSIM-SCR-010 |
| Public view | Shows completed effects, remaining work, and recovery separately | VIEW-001 |

## Assertions

- [x] request and acknowledgement are not effective cancellation;
- [x] no new dependent work starts;
- [x] owner interruptibility remains authoritative;
- [x] effects completed before the stop remain recorded; and
- [x] the pre-start control can cancel without owner deferral.

## Simulation issues

- `FSIM-SI-011`.

## Specification changes

- `FSIM-SCR-010`.

## Retrace

The fixture now has explicit states whether `L5` reaches a safe stop or
completes before that stop; neither path can be reported merely as
“cancelled.”

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
