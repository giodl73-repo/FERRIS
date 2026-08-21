# FERRIS Role Index

FERRIS uses explicit review lenses for systems research and compiler-grounded
AI tooling.

## Parliament

| File | Role | Primary tension |
|---|---|---|
| `parliament/rust-safety-steward.md` | Rust Safety Steward | Useful low-level capability vs. weakened safety claims |
| `parliament/compiler-performance-engineer.md` | Compiler Performance Engineer | Measured iteration gains vs. benchmark theater |
| `parliament/interop-boundary-auditor.md` | Interop Boundary Auditor | Incremental adoption vs. unsafe semantic loss |
| `parliament/ai-assurance-skeptic.md` | AI Assurance Skeptic | Compiler evidence vs. unjustified correctness claims |
| `parliament/ecosystem-strategist.md` | Ecosystem Strategist | Defensible wedge vs. duplicating mature Rust tools |
| `parliament/product-value-governor.md` | Product Value Governor | Ferris outcomes vs. technically interesting detours |

## Editorial

| File | Role | Primary check |
|---|---|---|
| `editorial/scope-keeper.md` | Scope Keeper | Research lanes remain bounded and product-neutral |
| `editorial/validation-checker.md` | Validation Checker | Claims have commands, fixtures, and limitations |
| `editorial/autonomy-supervisor.md` | Autonomy Supervisor | Useful agent initiative vs. uncontrolled continuation |

## Stakeholders

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/rust-maintainer.md` | Rust Maintainer | Patch quality, explainability, and maintenance burden |
| `stakeholders/native-platform-adopter.md` | Native Platform Adopter | Migration cost, compatibility, and operational trust |

## Productive tensions

| Pulls | Against | Because |
|---|---|---|
| Rust Safety Steward | Compiler Performance Engineer | Low-level speedups can weaken safety boundaries or rely on unjustified assumptions. |
| Interop Boundary Auditor | Ecosystem Strategist | Incremental ecosystem adoption can pressure neutral boundaries into lossy adapters. |
| AI Assurance Skeptic | Product Value Governor | Strong assurance evidence can require work beyond the value of the proposed outcome. |
| Scope Keeper | Autonomy Supervisor | Useful follow-on investigation can exceed the bounded scope authorized for a pulse. |
| Validation Checker | Compiler Performance Engineer | Reproducible validation can expose benchmark gains as narrow or environment-specific. |

Safety and semantic-loss findings block first. Product Value Governor then decides whether fixing
a veto is worth the bounded effort. Resolve performance or ecosystem disputes with the smallest
representative fixture or benchmark; preserve unresolved dissent and require explicit user
approval before expanding scope.

## Review order

1. The Product Value Governor must approve the outcome, opportunity cost, and
   stop condition before a new wave, pulse family, or successor chain begins.
2. Use the relevant technical parliament roles while selecting or designing a
   capability.
3. Use stakeholder roles before fixing the onboarding contract.
4. The Autonomy Supervisor must review scope growth before any autonomous
   follow-on pulse is started.
5. Use editorial roles before treating a pulse as complete.

## Mandatory autonomous-work controls

- A technical finding does not automatically authorize another pulse.
- One corrective successor may be attempted inside the approved scope. A
  second successor requires explicit user approval.
- Stop and report when two consecutive attempts are invalid, when the work
  changes layers, or when validation infrastructure exceeds the product
  change it protects.
- Every pulse must state its Ferris user outcome, maximum effort, completion
  test, and abandonment condition before implementation.
- `stop`, `pause`, `move on`, or equivalent user direction immediately ends
  autonomous continuation. No cleanup commit, review pass, or successor may be
  started afterward unless the user explicitly requests it.
- Technical reviewers may veto unsafe work. Only the Product Value Governor
  may decide that fixing the veto is still worth doing.
