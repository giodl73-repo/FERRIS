# W02: Cross-workspace Contracts

Status: Complete after retrace
Claim state: simulated

## Goal

Test exact source identity, semantic and generated projection compatibility,
profile renewal, and build-script/native freshness across multiple workspaces.

## Locked specification baseline

Baseline commit: `b7c36c5`

The retrace includes FSIM-SCR-004 and FSIM-SCR-005.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-005](FSIM-005-exact-source-identity.md) | Same package name, different source identity | Prevent false cross-workspace fan-out | Pass without spec change |
| [FSIM-006](FSIM-006-layered-contract-change.md) | Typebook change with stale Rust projection | Layered compatibility aggregation | Pass after FSIM-SCR-004 |
| [FSIM-007](FSIM-007-profile-toolchain-renewal.md) | Exact compiler changes after profile approval | Expiry, stale support, and renewal | Pass without spec change |
| [FSIM-008](FSIM-008-hidden-build-input.md) | Undeclared build-script environment input changes | Scope widening versus owner freshness | Pass after FSIM-SCR-005 |

## Wave issues

- FSIM-SI-005: mandatory layered compatibility lacked deterministic
  application eligibility composition; and
- FSIM-SI-006: scope widening alone could appear sufficient when Cargo
  freshness did not observe a changed hidden build input.

## Role review

- Rust Safety Steward: accepted after hidden build/native inputs could not be
  treated as corrected by wider package selection.
- Compiler Performance Engineer: accepted because isolated rebuild is a
  fallback with explicit cost, not a claimed optimization.
- Interop Boundary Auditor: accepted after semantic and Rust projection
  compatibility remained separately visible and jointly required.
- AI Assurance Skeptic: accepted because no AI-selected default can synthesize
  semantic equivalence or freshness.
- Ecosystem Strategist: accepted because Cargo and Typebook retain authority.
- Rust Maintainer: accepted because false package-name fan-out is prohibited
  and hidden-input remediation names owner actions.
- Native Platform Adopter: accepted for Draft after exact SDK, compiler, and
  native environment identity remained explicit.
- Scope Keeper: accepted as a bounded cross-workspace contract wave.
- Validation Checker: accepted after stale profiles, negative projection, and
  hidden-input controls were retraced.

## Disposition

Close W02 with no open P0 or P1 issues. Continue to W03 identity and evidence.
