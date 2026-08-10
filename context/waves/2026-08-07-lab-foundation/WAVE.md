# Wave: Lab Foundation

## Goal

Establish FERRIS as an enterprise Rust application-platform lab with explicit
research lanes, repository-local review roles, product-neutral contracts, and
a measured decision process for whether to begin a first compiler-grounded
prototype.

## Thesis

Rust's ecosystem moat makes focused tooling more credible than an immediate new
general-purpose language. FERRIS should first identify measurable gaps, define
compatibility boundaries, and prove that compiler-grounded AI can produce
stronger evidence than text-only code generation.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Research foundation | complete | Created the docs, skills, roles, and implementation gate. |
| 02 | Native landscape benchmark | complete | The performance and Crates Series research sequences selected evidence coordination and contract support as the wedge. |
| 03 | FERRIS application contracts | in progress | Integrate RUNE, Cargo Blueprint, layered Rust/ABI/WIT/wire contracts, supported profiles, Query Forest architecture, and evidence packets. |
| 04 | First bounded proof | pending | Implement only one separately approved, held-out Blueprint or interop proof. |

## Success criteria

- README explains FERRIS, RUNE, the enterprise platform, and the research lanes.
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
