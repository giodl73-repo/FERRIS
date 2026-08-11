# W01: Foundations

Status: Complete after first retrace
Claim state: simulated

## Goal

Test whether the completed Draft spine can produce deterministic read-only
planning behavior for basic Rust changes before introducing execution,
connectors, remote artifacts, or complex lifecycle state.

## Locked specification baseline

Baseline commit: `83b164f`

The initial trace used the specifications at that commit. The retrace includes
FSIM-SCR-001 through FSIM-SCR-003.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-001](FSIM-001-private-body-edit.md) | Private body edit in a shared library | Behavioral consumer scope versus source API stability | Pass after SCR-001 and SCR-002 |
| [FSIM-002](FSIM-002-public-api-change.md) | Breaking public API change | Cross-workspace migration and blocked resolution | Pass after SCR-001 |
| [FSIM-003](FSIM-003-unmapped-runtime-input.md) | Unmapped shared runtime input | Safe widening and full-reference validation | Pass after SCR-002 |
| [FSIM-004](FSIM-004-entrypoint-defaults.md) | CLI entrypoint defaults | Plan-first `check`/`test` and explicit scope parity | Pass after SCR-003; numeric exit blocker remains |

## Wave issues

- FSIM-SI-001: canonical Change Record gap;
- FSIM-SI-002: widening precedence ambiguity;
- FSIM-SI-003: `check` and `test` phase ambiguity; and
- FSIM-SI-004: numeric exit codes required a fixed process contract.

## Role review

- Rust Safety Steward: accepted after unknown and behavior-affecting changes
  could not silently narrow.
- Compiler Performance Engineer: accepted because no latency claim is made and
  selected/full-reference work remains explicit.
- Interop Boundary Auditor: not directly exercised; later wave required.
- AI Assurance Skeptic: accepted because all outcomes remain simulated and
  interpretive choices are cited.
- Ecosystem Strategist: accepted because Cargo remains authoritative.
- Rust Maintainer: accepted after command phase behavior became explicit.
- Native Platform Adopter: not directly exercised; Windows/Unix held-out wave
  remains required.
- Scope Keeper: accepted as one read-only foundation slice.
- Validation Checker: accepted after negative and unknown-input controls were
  frozen and retraced.

## Disposition

Close W01. FSIM-SI-004 was later resolved by FSIM-SCR-025 and retraced in
FSIM-004 revision 2. Open W02 only against the amended specification baseline.
