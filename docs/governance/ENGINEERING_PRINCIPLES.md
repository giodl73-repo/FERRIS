# FERRIS Engineering Principles

## Purpose

These principles govern how FERRIS researches, prototypes, and evaluates
native tooling. They turn the lab's research-first posture into decision rules
that can reject attractive but weak ideas before they become products.

They apply across every research lane. A later wave may strengthen them, but it
must not silently weaken them.

## Principles

### FP-01: Evidence before narrative

Every material claim must be supported by a cited source, reproducible command,
or measured result. Compiler acceptance, a persuasive model explanation, and a
single successful run are evidence surfaces, not proof.

**Decision rule:** Label unmeasured claims as hypotheses and keep them out of
product promises.

### FP-02: Optimize representative workflows

FERRIS optimizes the time and effort users actually experience, not an isolated
microbenchmark. Cold, warm, incremental, check, build, test, codegen, and link
workloads must remain distinguishable.

**Decision rule:** No performance recommendation advances without a named
consumer workflow, baseline, variance, and known limitation.

### FP-03: Explain causality before prescribing change

Recommendations should identify why work occurred before proposing workspace,
feature, caching, compiler, or linker changes. Correlation alone is not a safe
basis for source rewrites or architecture changes.

**Decision rule:** Prefer an actionable explanation over an automatic rewrite.

### FP-04: Never hide a correctness trade

Performance, convenience, or AI autonomy must not silently weaken safety,
reproducibility, diagnostics, or behavioral validation. Rust guarantees stop at
explicit boundaries, especially around `unsafe`, FFI, build scripts, procedural
macros, caches, and external toolchains.

**Decision rule:** State which guarantees hold, where they stop, and what
evidence covers the gap.

### FP-05: Preserve ordinary Rust workflows

FERRIS should complement Cargo, rustc, rust-analyzer, Clippy, Miri, debuggers,
and established build systems. Core value must not depend on unstable compiler
internals when exported metadata or a compatibility adapter is sufficient.

**Decision rule:** Stable interfaces are the default; experimental integration
must sit behind a replaceable compatibility boundary.

### FP-06: Contribute before competing

Existing Rust initiatives, maintainers, standards, and tools must be examined
before FERRIS creates a replacement. Upstream fixtures, measurements,
diagnostics, documentation, and targeted patches are preferred when they solve
the measured problem.

**Decision rule:** Every build decision records why adopting, contributing,
wrapping, or competing is appropriate.

### FP-07: Make adoption incremental and reversible

A team must be able to evaluate and remove FERRIS without rewriting its system,
changing source semantics, or losing ordinary tool access. Interoperability work
must preserve rollback and make ABI, ownership, allocation, panic, exception,
threading, and lifetime rules explicit.

**Decision rule:** No prototype advances without an exit path and failure
recovery story.

### FP-08: Measure the whole system

Compiler frontend time is only one part of native iteration latency. Cargo graph
shape, features, profiles, procedural macros, build scripts, codegen, debug
information, linking, hardware, filesystem, cache state, and toolchain identity
must be considered where relevant.

**Decision rule:** Attribute cost to components before selecting an optimization
target.

### FP-09: Treat caches as correctness systems

Precompilation and caching require explicit identities, inputs, outputs,
provenance, compatibility rules, and invalidation behavior. A fast cache that
can return the wrong artifact is not an optimization.

**Decision rule:** Cache experiments require positive hits, intentional misses,
poisoning resistance, and reproducibility checks.

### FP-10: Keep failures visible and useful

Failed commands, cache misses, unsupported targets, inconclusive measurements,
and negative test results are first-class evidence. FERRIS must not convert
them into success-shaped summaries.

**Decision rule:** Diagnostics must identify the failing boundary and preserve
enough context for a maintainer to reproduce it.

### FP-11: AI proposes; evidence and accountable humans decide

Ferris may gather evidence, generate hypotheses, and propose patches, but model
confidence is not technical assurance. High-risk changes require explicit human
approval, and generated work must record relevant inputs, actions, revisions,
commands, and results.

**Decision rule:** Automation authority must be narrower than its evidence.

### FP-12: Bound experiments and publish stop conditions

Each prototype must answer one research question for a named consumer using
representative fixtures. Success criteria, non-goals, compatibility boundaries,
and rejection conditions are defined before implementation.

**Decision rule:** Stop, defer, or upstream work that fails its gate rather than
expanding scope to protect the idea.

## Common pitfalls and countermeasures

| ID | Pitfall | Countermeasure |
|---|---|---|
| PF-01 | Benchmark theater: optimizing a tiny or favorable case | Use a workload matrix, representative fixtures, repeated runs, and variance. |
| PF-02 | Treating all compile time as one problem | Separate Cargo scheduling, frontend, queries, macros, codegen, debug data, and linking. |
| PF-03 | Tuning before understanding invalidation | Record edit scenarios and explain rebuild causality before recommending changes. |
| PF-04 | Trading correctness for speed without saying so | Compare validation and reproducibility before and after every optimization. |
| PF-05 | Assuming a cache hit is inherently safe | Define complete cache identity, provenance, invalidation, negative tests, and corruption behavior. |
| PF-06 | Building a rustc fork, backend, linker, or package manager too early | Measure the gap and evaluate upstream work, adapters, and contributions first. |
| PF-07 | Depending directly on unstable compiler internals | Start with stable Cargo outputs; isolate optional nightly or `rustc_private` integration. |
| PF-08 | Overfitting to one repository, machine, or toolchain | Use fixture categories and record hardware, OS, filesystem, toolchain, profiles, and cache state. |
| PF-09 | Letting AI-generated confidence replace assurance | Require behavioral tests, failure evidence, provenance, and human approval proportional to risk. |
| PF-10 | Producing opaque recommendations that increase maintenance cost | Explain the cause, expected gain, tradeoff, rollback, and ordinary Rust alternative. |
| PF-11 | Concealing failures behind summaries or fallbacks | Preserve command results and surface unsupported or inconclusive cases explicitly. |
| PF-12 | Expanding a prototype until it cannot fail | Set one question, a time-bounded gate, non-goals, and explicit stop conditions. |
| PF-13 | Breaking incremental adoption at a language boundary | Specify ABI and semantic contracts, test negative cases, and preserve rollback. |
| PF-14 | Mistaking local novelty for ecosystem opportunity | Inventory current tools and upstream plans, then name the missing consumer capability. |

## Required gate for a prototype

A prototype may begin only when its proposal records:

1. The research question, named consumer, and representative fixtures.
2. The current tools and upstream initiatives relevant to the problem.
3. Baseline commands, environment identity, variance, and limitations.
4. The compatibility, safety, and correctness boundaries.
5. Success measures, negative cases, stop conditions, and non-goals.
6. Adoption, removal, rollback, and maintenance implications.
7. The evidence Ferris may produce and the decisions reserved for humans.
8. A completed `.roles` review with unresolved objections visible.

## Role review

The initial doctrine was reviewed against every FERRIS role on 2026-08-07.

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted after FP-04 made guarantee boundaries and hidden correctness trades explicit. |
| Compiler Performance Engineer | Accepted after FP-02 and FP-08 required workload separation, representative baselines, and variance. |
| Interop Boundary Auditor | Accepted after FP-07 required reversible adoption and explicit cross-language semantics. |
| AI Assurance Skeptic | Accepted after FP-01, FP-10, and FP-11 separated evidence from assertion and preserved failures. |
| Ecosystem Strategist | Accepted after FP-06 required an adopt/contribute/wrap/compete decision. |
| Rust Maintainer | Accepted after FP-03, FP-05, and FP-07 protected explainability, ordinary tooling, and removability. |
| Native Platform Adopter | Accepted after FP-07 and the prototype gate required migration, rollback, operational, and maintenance implications. |
| Scope Keeper | Accepted after FP-12 bounded each experiment and required stop conditions and non-goals. |
| Validation Checker | Accepted after the prototype gate required commands, fixtures, environment identity, negative cases, and honest status. |

No role raised a blocking objection after these changes. Acceptance means the
doctrine addresses the role's current questions; it does not pre-approve a
future prototype.

## Evidence basis

- [`2026-08-07-rustc-compiler-performance.md`](../research/2026-08-07-rustc-compiler-performance.md):
  findings FERRIUM-01 through FERRIUM-11 and the proposed benchmark protocol.
- [`2026-08-07-rust-latency-component-roadmap.md`](../research/2026-08-07-rust-latency-component-roadmap.md):
  findings FERRIUM-12 through FERRIUM-23, the precompilation ladder, and the
  phased contribution roadmap.
- [FERRIS role index](../../.roles/ROLE.md) and the nine role definitions
  referenced by the review table.
