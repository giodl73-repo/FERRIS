# Ferris Specification Simulations

Status: Complete at Draft after final convergence
Implementation authority: None

Ferris specification simulations hand-trace frozen fixtures through the Draft
specification spine. They predict records, decisions, failures, fallbacks, and
public views without executing Ferris or writing product code.

The **Ferris Wheel** is the cross-wave regression turn that retraces every
earlier fixture affected by a Specification Change Record.

## Governing method

- [Simulation program](FERRIS_SPECIFICATION_SIMULATION_PROGRAM.md)
- [Simulation issues and change records](ISSUES.md)
- [Scenario template](_template/SCENARIO.md)
- [Final convergence review](FERRIS_SIMULATION_CONVERGENCE_REVIEW.md)
- [Held-out implementation fixture set](held-out/README.md)

## Wave registry

| Wave | Scope | Scenarios | Status |
|---|---|---:|---|
| [W01 Foundations](waves/W01-foundations/WAVE.md) | Change records, scope, validation, planning, and command defaults | 4 | Complete after first retrace |
| [W02 Cross-workspace contracts](waves/W02-cross-workspace-contracts/WAVE.md) | Exact source identity, layered contracts, profile renewal, and hidden native inputs | 4 | Complete after retrace |
| [W03 Identity and evidence](waves/W03-identity-evidence/WAVE.md) | Roots, refs, generations, adapters, projections, stale and conflicting evidence | 4 | Complete after retrace |
| [W04 Prediction and AI](waves/W04-prediction-ai/WAVE.md) | Held-out prediction, narrowing, abstention, budgets, and model accountability | 4 | Complete after retrace |
| [W05 Governance and action](waves/W05-governance-action/WAVE.md) | Resolution, approval, isolation, execution, rollback, cleanup, and audit | 4 | Complete after retrace |
| [W06 Connectors and MCP](waves/W06-connectors-mcp/WAVE.md) | Owner failures, parity, prompt injection, tool poisoning, and revocation | 4 | Complete after retrace |
| [W07 Lifecycle and removal](waves/W07-lifecycle-removal/WAVE.md) | Renewal, substitution, incident, packet, connector removal, and complete Ferris removal | 4 | Complete after retrace |
| [W08 Adversarial composition](waves/W08-adversarial-composition/WAVE.md) | Cross-gate races, partial failures, tenant boundaries, and unsupported platforms | 4 | Complete after retrace |
| [W09 Operational continuity](waves/W09-operational-continuity/WAVE.md) | Rolling schema upgrades, offline operation, disaster recovery, clock skew, and evidence-service loss | 5 | Complete after retrace |
| [W10 Human and scale limits](waves/W10-human-scale/WAVE.md) | Diagnostic actionability, giant graphs, truncation, accessibility, localization, and operator error | 5 | Complete after retrace |
| [W11 Coverage closure](waves/W11-coverage-closure/WAVE.md) | Direct component boundaries, doctor safety, explanation completeness, and command semantics | 4 | Complete after retrace |

## Final result

- 11 complete waves;
- 46 frozen simulated scenarios;
- 25 Simulation Issues;
- 25 applied and retraced Specification Change Records;
- all 22 specifications and all nine public commands covered;
- no open Simulation Issue.

## Claim boundary

Scenario outcomes are `simulated`. They are not observed runtime behavior,
implementation conformance, performance evidence, support commitments, or
authorization to implement.
