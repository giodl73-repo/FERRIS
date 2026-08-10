# Wave: Lab Foundation

## Goal

Establish FERRIUM as an AI-native systems engineering lab with explicit research
lanes, repository-local review roles, and a measured decision process for
whether to begin a first compiler-grounded prototype.

## Thesis

Rust's ecosystem moat makes focused tooling more credible than an immediate new
general-purpose language. FERRIUM should first identify measurable gaps, define
compatibility boundaries, and prove that compiler-grounded AI can produce
stronger evidence than text-only code generation.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Research foundation | complete | Created the docs, skills, roles, and implementation gate. |
| 02 | Native landscape benchmark | in progress | The performance sequence is complete; execute the Crates Series next, then finish cross-lane scoring. |
| 03 | OSPREY and FERRIS contracts | pending | Define the Query Forest architecture, controlled-action model, and evidence packet after the Crates Series. |
| 04 | First bounded proof | pending | Implement only one separately approved, held-out OSPREY or interop proof. |

## Success criteria

- README explains FERRIUM, Ferris, and the research lanes.
- Product plan records consumers, measures, and non-goals.
- `.roles` covers safety, performance, interop, AI assurance, scope, validation,
  and adopter concerns.
- The next pulse has a cited research question and selection criteria.
- The repo contains no implementation package before the research gate.
- Documentation validation passes.

## Non-goals

- Selecting a product before the opportunity benchmark.
- Writing product code in the foundation pulse.
- Creating a new language in the foundation wave.
- Claiming autonomous correctness or formal verification.
- Integrating consumer repos before a compatibility contract exists.
