# W03: Identity and Evidence

Status: Complete after retrace
Claim state: simulated

## Goal

Test typed ref resolution, immutable-root generations, concurrent channel
promotion, revocation over historical evidence, and projection consistency.

## Locked specification baseline

Baseline commit: `fdaa630`

The retrace includes FSIM-SCR-006 and FSIM-SCR-007.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-009](FSIM-009-ref-namespace-switch.md) | Branch movement and colliding typed ref names | Exact ref qualification and immutable generations | Pass after FSIM-SCR-006 |
| [FSIM-010](FSIM-010-concurrent-channel-promotion.md) | Two actors promote one channel concurrently | Compare-and-set history | Pass without spec change |
| [FSIM-011](FSIM-011-revoked-evidence.md) | Previously accepted evidence is later revoked | Historical truth versus future eligibility | Pass without spec change |
| [FSIM-012](FSIM-012-projection-inconsistency.md) | Map and ledger disagree over one immutable root | Blocking material consistency failure | Pass after FSIM-SCR-007 |

## Wave issues

- FSIM-SI-007: typed refs lacked deterministic qualification and collision
  behavior; and
- FSIM-SI-008: a detected material projection inconsistency did not explicitly
  block downstream decisions or classify engine failure separately.

## Role review

- Rust Safety Steward: accepted after inconsistent projections could not feed
  action and ref ambiguity could not silently select a root.
- Compiler Performance Engineer: accepted because consistency failure blocks
  reuse rather than inventing a performance claim.
- Interop Boundary Auditor: accepted because owner conflicts remain distinct
  from projection-engine contradictions.
- AI Assurance Skeptic: accepted because neither AI nor display precedence may
  resolve ambiguous refs or suppress material conflicts.
- Ecosystem Strategist: accepted because Git retains source ref authority and
  Ferris only records typed associations.
- Rust Maintainer: accepted because branch movement creates generations
  without rewriting roots or ordinary Git history.
- Native Platform Adopter: accepted for Draft because revocation and stale
  evidence remain operation-scoped and visible.
- Scope Keeper: accepted as a bounded identity and evidence wave.
- Validation Checker: accepted after collision, concurrency, revocation, and
  projection-negative controls were retraced.

## Disposition

Close W03 with no open P0 or P1 issues. Continue to W04 prediction and AI.
