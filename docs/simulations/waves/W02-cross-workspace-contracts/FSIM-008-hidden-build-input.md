# FSIM-008: Hidden Build-script and Native Environment Input

Wave: W02
Revision: 1
State: Retraced
Claim state: simulated

## Question

Can Ferris safely plan after a build script's observed native SDK input changes
when Cargo does not declare that input for freshness?

## Locked fixture

- Package `native-sys` has `build.rs`.
- Prior evidence observes reads of `NATIVE_SDK_ROOT` and native headers beneath
  that directory.
- `build.rs` does not emit `cargo:rerun-if-env-changed=NATIVE_SDK_ROOT`.
- The environment changes from SDK A to SDK B.
- Source files and Cargo manifests do not change.
- Cargo may consider prior build-script output fresh.
- The application requires exact native ABI and deployment validation.

Negative control: the build script declares the environment input and Cargo
owner evidence reports the changed input.

## Governing specifications

- EVIDENCE-001 side-effecting and observed evidence;
- SCOPE-001 environment and native widening;
- PLANNING-001 owner freshness and fallback;
- VALIDATION-001 native and ABI coverage; and
- EXECUTION-001 approved cleanup or isolated rebuild.

## Initial hand trace

The specs widened unknown scope but did not state that wider package selection
cannot repair an owner freshness mechanism that fails to observe the changed
input.

Initial issue: FSIM-SI-006.

## Retraced expected behavior

| Stage | Predicted result |
|---|---|
| Change | Environment Change Record identifies SDK identity change |
| Evidence | Prior directly observed build-script input conflicts with missing Cargo declaration |
| Scope | Native, generated, link, runtime, and deployment consumers widen |
| Freshness | Owner freshness is insufficient; wider selection alone is not corrective |
| Planning | Plan is blocked pending owner declaration fix or an explicit isolated clean rebuild alternative |
| Validation | Prior artifacts and passing selected tests are ineligible for the new SDK |
| Resolution | Alternatives are fix `rerun-if-env-changed`, approve isolated empty-state rebuild, select SDK A, or defer |
| Action boundary | Cleaning or invalidating state requires a later exact approved Action Plan |
| Control | Declared input allows Cargo owner freshness to rerun the build script normally |

## Assertions

- [x] Hidden input evidence remains distinct from Cargo declaration.
- [x] Scope widening does not claim to invalidate Cargo state.
- [x] Clean rebuild is an explicit costly action, not a silent planning side
  effect.
- [x] Native ABI and deployment validation are renewed.
- [x] The declared-input control follows owner-native freshness.

## Specification changes

- FSIM-SCR-005.

## Claim boundary

Cargo freshness behavior is simulated from the locked fixture. No build script,
SDK, compiler, linker, or cleanup command ran.
