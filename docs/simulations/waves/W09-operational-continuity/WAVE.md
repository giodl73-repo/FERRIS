# W09: Operational Continuity

Status: Complete after retrace
Claim state: simulated

## Goal

Test rolling schema compatibility, explicit offline behavior, disaster
recovery, clock uncertainty, and evidence-service loss during approved work.

## Locked specification baseline

Baseline commit: `c311540`

The retrace includes FSIM-SCR-018 through FSIM-SCR-020 and a Ferris Wheel turn
over affected offline, identity, lifecycle, approval, and revocation fixtures.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-033](FSIM-033-rolling-schema-upgrade.md) | Old reader and new writer overlap during additive schema rollout | Version support without guessed fields | Pass without spec change |
| [FSIM-034](FSIM-034-offline-local-plan.md) | Developer plans from retained local evidence without network | Explicit offline evidence envelope | Pass after FSIM-SCR-018 |
| [FSIM-035](FSIM-035-forest-disaster-recovery.md) | Root store is lost and restored from backup plus packets | Restore identity versus reconstruction | Pass after FSIM-SCR-019 |
| [FSIM-036](FSIM-036-clock-skew-expiry.md) | Client and approval-authority clocks straddle expiry | Conservative time uncertainty | Pass after FSIM-SCR-020 |
| [FSIM-037](FSIM-037-evidence-service-loss.md) | Evidence and revocation service fails between action barriers | Read continuity versus mutation stop | Pass after FSIM-SCR-018 and FSIM-SCR-020 |

## Wave issues

- FSIM-SI-019: offline behavior lacked a semantic network and authority
  envelope;
- FSIM-SI-020: disaster recovery lacked a canonical restoration record and
  root-identity rule; and
- FSIM-SI-021: time-sensitive authority lacked clock uncertainty and skew
  semantics.

## Ferris Wheel retrace

- Offline changes preserved read-only local diagnosis while preventing cached
  authority from enabling mutation.
- Recovery changes preserved immutable root and ref semantics.
- Time changes made expiry and revocation fixtures fail conservatively at
  uncertain boundaries.

## Role review

- Rust Safety Steward: accepted after unavailable authority and clock
  uncertainty block mutation.
- Compiler Performance Engineer: accepted because offline reads may use
  retained evidence without pretending it is fresh.
- Interop Boundary Auditor: accepted after schema, backup, packet, owner-time,
  and service identities remain distinct.
- AI Assurance Skeptic: accepted because offline model or connector absence
  cannot be replaced by cached confidence.
- Ecosystem Strategist: accepted because ordinary Cargo remains usable during
  Ferris service loss.
- Rust Maintainer: accepted because recovery and offline diagnostics name the
  evidence needed to return online.
- Native Platform Adopter: accepted for Draft after local and remote platform
  evidence remain independently stale or unavailable.
- Scope Keeper: accepted as a bounded continuity wave without operating a
  service.
- Validation Checker: accepted after upgrade, offline, restore, clock, and
  service-loss controls completed a Wheel turn.

## Disposition

Close W09 with no open P0 or P1 issues. Continue to W10 human and scale limits.
