# FSIM-031: Unsupported Native Platform

Wave: W08
Revision: 1
State: Simulated
Claim state: simulated

## Question

Can successful Rust compilation on a supported host become degraded success
for an unsupported native target and deployment platform?

## Locked fixture

- application: `forge`
- repositories and workspaces: one Rust service with native TLS provider
- source and change: public API change
- contracts and profiles: source contract is compatible; no approved profile
  exists for target `vendor-os-arm64`
- environment: host compilation succeeds; target SDK and runtime are absent
- policy: release requires native link, execute, deploy, and rollback evidence
- available evidence: host `cargo check` passes
- explicit unknowns: target ABI, loader, runtime, and provider behavior
- negative or matched control: approved Linux ARM64 profile with exact evidence

Changing the fixture requires a new revision.

## Governing specifications

- PLATFORM-001 environment and validation stages;
- CONTRACT-001 layered compatibility; and
- CONFORMANCE-001 C-PLATFORM and C-INTEROP.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Public API Change Record | FOREST-002 |
| Scope | Source, target, native, runtime, deployment, and rollback | SCOPE-001 |
| Evidence | Host check is observed; target stages are unsupported or not observed | PLATFORM-001 |
| Causality | Host compilation proves no target runtime behavior | Claim separation |
| Prediction | Target success is unsupported | No transfer |
| Validation | Link, execute, deploy, operational, and rollback gates remain unmet | Stage independence |
| Planning | Select supported platform, obtain a profile, or block | Safe fallback |
| Resolution | Unsupported target candidate is ineligible | Hard constraint |
| Trust/action | No deployment Action Plan is eligible | Governance |
| Public view | Result is unsupported, not degraded success | VIEW-001 |

## Assertions

- [x] passing host compilation remains visible;
- [x] source compatibility does not imply native or deployment support;
- [x] absent target evidence is not failure or pass;
- [x] no release action is approved; and
- [x] the supported Linux control remains independently eligible.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original platform, validation, and conformance rules require an
unsupported result.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
