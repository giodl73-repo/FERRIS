# W10: Human and Scale Limits

Status: Complete after retrace
Claim state: simulated

## Goal

Test giant result sets, truncation, confusable identities, accessible and
localized views, actionable diagnostics, and stale operator input.

## Locked specification baseline

Baseline commit: `f4cfa45`

The retrace includes FSIM-SCR-021 through FSIM-SCR-023 and a Ferris Wheel turn
over affected view, failure, identity, and operator scenarios.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-038](FSIM-038-million-edge-truncation.md) | A graph projection exceeds its million-edge budget | Severity-preserving bounded output | Pass after FSIM-SCR-021 |
| [FSIM-039](FSIM-039-confusable-application-selection.md) | Two tenants have visually identical application names | Canonical mutation selection | Pass after FSIM-SCR-022 |
| [FSIM-040](FSIM-040-localized-accessible-output.md) | A blocked plan is rendered in localized, non-color, screen-reader output | Semantic and accessibility parity | Pass after FSIM-SCR-023 |
| [FSIM-041](FSIM-041-actionable-owner-diagnostic.md) | Native owner failure produces thousands of log lines | Bounded diagnosis and safe next action | Pass after FSIM-SCR-021 and FSIM-SCR-023 |
| [FSIM-042](FSIM-042-stale-cross-tenant-action-id.md) | Operator pastes an old Action Plan ID from another tenant | Exact identity, authorization, and non-disclosure | Pass without spec change |

## Wave issues

- FSIM-SI-022: truncation could hide mandatory failures or unknowns from the
  public summary;
- FSIM-SI-023: mutation selection lacked an explicit prohibition on fuzzy and
  remembered identities; and
- FSIM-SI-024: diagnostics and localized output lacked durable semantic and
  accessibility requirements.

## Ferris Wheel retrace

- Bounded-output rules preserved earlier projection, model, connector, tenant,
  and partial-result failures.
- Selection rules preserved typed refs, worktree isolation, and cross-tenant
  boundaries.
- Diagnostic rules preserved failure ownership while improving safe recovery
  guidance without changing authority.

## Role review

- Rust Safety Steward: accepted after truncation and presentation cannot hide
  a mandatory failure or select a confusable mutation target.
- Compiler Performance Engineer: accepted because scale limits use stable
  pagination rather than unbounded memory or logs.
- Interop Boundary Auditor: accepted after diagnostic owner, native stage,
  tenant, and target identities remain explicit.
- AI Assurance Skeptic: accepted because summaries and localization cannot
  rewrite machine semantics or invent remediation authority.
- Ecosystem Strategist: accepted because owner-native diagnostics and commands
  remain primary.
- Rust Maintainer: accepted because failures identify safe next actions and
  evidence instead of requiring raw-log archaeology.
- Native Platform Adopter: accepted for Draft after native failure detail is
  bounded without losing ABI, linker, loader, or platform state.
- Scope Keeper: accepted as a bounded human-interface wave without product
  implementation.
- Validation Checker: accepted after scale, identity, accessibility,
  localization, diagnostics, and operator-error controls completed a Wheel
  turn.

## Disposition

Close W10 with no open P0 or P1 issues. The scenario corpus is ready for a
final convergence review and held-out implementation-fixture freeze.
