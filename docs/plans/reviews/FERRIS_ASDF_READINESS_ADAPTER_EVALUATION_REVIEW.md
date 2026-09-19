# FERRIS asdf Readiness Adapter Evaluation Review

Date: 2026-09-19
Status: Complete
Pulse: Environment Readiness Pulse 04
Disposition: No product adapter; retain explicit READINESS-001 input

## Executive disposition

The eleven roles accept the bounded evaluation and its no-go result.
`.tool-versions` can be parsed lexically without execution, but it cannot be
translated losslessly into READINESS-001 V1. Plugin IDs do not determine
executable leaves, and all documented version-selection forms exceed V1.

## Role dispositions

| Role | Disposition | Finding |
|---|---|---|
| Rust Safety Steward | `accept-no-safety-claim` | The test-only parser uses safe Rust and the evaluation makes no toolchain or behavioral correctness claim. |
| Compiler Performance Engineer | `accept-no-performance-claim` | The evaluation records no timing, savings, or build-performance conclusion. |
| Interop Boundary Auditor | `veto-lossy-plugin-mapping` | Plugin-owned executable and environment semantics cannot be flattened into an executable leaf without semantic loss. |
| AI Assurance Skeptic | `accept-observed-no-go` | Primary sources and frozen fixtures support the no-go; absent mappings are not inferred. |
| Ecosystem Strategist | `accept-owner-aligned-no-go` | asdf retains plugin and version authority; Ferris does not duplicate its resolver. |
| Rust Maintainer | `accept-no-product-burden` | No runtime parser, dependency, CLI flag, or maintenance surface is added. |
| Native Platform Adopter | `accept-no-host-inference` | Parent files, home configuration, environment overrides, plugins, and host installations are not discovered. |
| Scope Keeper | `accept-test-only-evaluation` | The pulse changes research, fixtures, a test-only harness, and governance only. |
| Validation Checker | `accept-frozen-public-cases` | Exact upstream bytes and ordinary, fallback, `system`, `path:`, and `ref:` cases are executable test evidence. |
| Product Value Governor | `stop-value-exhausted` | A V1 adapter would misdiagnose readiness; further implementation has no value without a new owner-mapping contract decision. |
| Autonomy Supervisor | `stop-after-pulse-04` | The authorized evaluation is complete; no contract revision, adapter, or Pulse 05 follows automatically. |

## Evidence

The public fixture is bound to asdf revision
`ca98e44ff49cb0a38966b23b42db962203478b59` and LF SHA-256
`383996cf07387f46a209f3739c189a18106de9a4d24baaed0a5ddb3030db1d64`.

```powershell
cargo test -p ferris-core --test asdf_readiness_adapter_evaluation --quiet
```

The two test-only cases pass: all three fixture groups reproduce their frozen
lexical rows and expected V1 semantic-loss classifications, and the upstream
fixture demonstrates the `golang` plugin ID versus `go` executable distinction.

## Remaining gates

- A product adapter requires a new contract with explicit owner mapping and
  non-success adapter dispositions.
- Version observation requires a separate product decision and cannot invoke a
  tool under current passive-readiness authority.
- No other owner-native format has been evaluated by this pulse.
- Unix and Windows product readiness behavior remains bounded by Pulse 03
  evidence; this test-only parser adds no platform claim.

## Final authority

Pulse 04 is complete and its evaluation authority is exhausted. The
dependency-free harness may be removed with its fixtures and documents without
changing Ferris behavior. Product code, schemas, dependencies, adopters, asdf,
plugins, and owner files remain unchanged. Pulse 05 is not authorized.
