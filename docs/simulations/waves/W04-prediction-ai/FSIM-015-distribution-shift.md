# FSIM-015: Distribution-Shift Abstention

Wave: W04
Revision: 1
State: Simulated
Claim state: simulated

## Question

What happens when a predictor calibrated on Windows-only Rust workspaces sees
a Unix workspace with a native linker and generated C input?

## Locked fixture

- application: `forge`
- repositories and workspaces: one Rust workspace with `build.rs` and native C
- source and change: header and linker-script changes on Unix
- contracts and profiles: Unix native profile exists but was absent from model
  calibration
- environment: supported Unix host
- policy: out-of-population predictions must abstain
- available evidence: model `M8` has calibrated Windows Cargo-only results
- explicit unknowns: native dependency and relink closure
- negative or matched control: a Windows Cargo-only private body edit within
  the calibrated population

Changing the fixture requires a new revision.

## Governing specifications

- PREDICTION-001 uncertainty, safe fallback, and lifecycle;
- PLANNING-001 owner-specific closures; and
- VALIDATION-001 failure and fallback.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Native and environment Change Records are retained | FOREST-002 |
| Scope | Rust, build-script, native, linker, and validation dimensions apply | SCOPE-001 |
| Evidence | Calibration population lacks Unix native cases | PREDICTION-001 |
| Causality | No Windows result establishes Unix native mechanism | CAUSALITY-001 |
| Prediction | `M8` is out of population and abstains | Safe fallback |
| Validation | Native compile, link, load, and capability checks remain | VALIDATION-001 |
| Planning | Uses deterministic owner closures or blocks for owner input | PLANNING-001 |
| Resolution | No AI narrowing candidate is eligible | Unsupported population |
| Trust/action | No approval can convert abstention into prediction evidence | Authority separation |
| Public view | Shows unsupported population, unknown native closure, and fallback | VIEW-001 |

## Assertions

- [x] Windows calibration does not transfer silently;
- [x] confidence is not reported for an unsupported population;
- [x] native and linker unknowns remain visible;
- [x] safe fallback is owner-native and conservative; and
- [x] the matched Windows control remains independently evaluable.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original safe-fallback and lifecycle rules require abstention without a
specification amendment.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
