# W07: Lifecycle and Removal

Status: Complete after retrace
Claim state: simulated

## Goal

Test profile renewal, connector disablement, packet revocation and deletion,
incident recovery, retained history, and complete Ferris removal.

## Locked specification baseline

Baseline commit: `6cd9f35`

The retrace includes FSIM-SCR-014 and FSIM-SCR-015.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-025](FSIM-025-profile-advisory-renewal.md) | A critical advisory arrives before scheduled profile expiry | Early renewal and support state | Pass without spec change |
| [FSIM-026](FSIM-026-connector-removal-active-session.md) | Connector removal begins with an active external session | Phased disablement and residual state | Pass after FSIM-SCR-014 |
| [FSIM-027](FSIM-027-revoked-packet-deletion.md) | An accepted packet is revoked and one replica cannot be deleted | Historical publication versus current lifecycle | Pass after FSIM-SCR-015 |
| [FSIM-028](FSIM-028-complete-ferris-removal.md) | Ferris is removed from a federated application | Owner-native correctness and completion proof | Pass after FSIM-SCR-014 |

## Wave issues

- FSIM-SI-015: removal requirements lacked one phased record and completion
  invariant across product, application, and connectors; and
- FSIM-SI-016: packet lifecycle collapsed historical publication, current
  eligibility, and deletion progress into one state.

## Role review

- Rust Safety Steward: accepted after removal cannot complete with hidden
  correctness state, active mutation, credentials, or unknown residual effects.
- Compiler Performance Engineer: accepted because retained caches and evidence
  are inventoried rather than silently treated as correctness dependencies.
- Interop Boundary Auditor: accepted after connector and provider removal
  includes external state, credentials, mappings, and owner-native fallback.
- AI Assurance Skeptic: accepted because lifecycle summaries cannot erase
  revocation, partial deletion, or failed verification.
- Ecosystem Strategist: accepted because Ferris, profiles, connectors, and
  packets remain removable without replacing Cargo or owner systems.
- Rust Maintainer: accepted because ordinary Cargo and repository workflows
  are explicitly verified after removal.
- Native Platform Adopter: accepted for Draft after native artifacts,
  deployments, credentials, and rollback boundaries remain in the inventory.
- Scope Keeper: accepted as a bounded lifecycle wave without performing
  deletion or uninstallation.
- Validation Checker: accepted after renewal, disablement, partial deletion,
  retained history, rollback, and complete-removal controls were retraced.

## Disposition

Close W07 with no open P0 or P1 issues. Continue to W08 adversarial
composition, then W09 and W10 recommended proof waves.
