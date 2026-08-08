# Rust Performance Research Questions

## Purpose

This registry decomposes Rust build and edit-to-confidence performance into
independently executable research questions. Each question has its own file,
hypothesis, evidence plan, intervention boundary, and decision.

Run a question with:

```text
/research PERF-Q01
/research PERF-Q01 --status
/research PERF-Q01 --resume
```

Research findings continue the global `FERRIUM-XX` sequence. `PERF-Qxx`
identifies a question, not a finding.

## Execution order

The questions are organized as a dependency ladder:

1. Establish trustworthy measurement and artifact identity.
2. Study Cargo-level duplication and orchestration.
3. Study rustc frontend and incremental reuse.
4. Study compile-time execution, generics, codegen, and linking.
5. Study advanced reuse and repository-level intervention.
6. Convert isolated cases into upstream contributions.

Questions may proceed in parallel only when they do not share mutable fixtures
or depend on unresolved terminology.

## Registry

| ID | Area | Question | Depends on | Primary path |
|---|---|---|---|---|
| PERF-Q01 | Foundation | What telemetry can explain Rust latency accurately? | None | External now |
| PERF-Q02 | Cargo | What forms the true Cargo build-unit identity? | Q01 | External now |
| PERF-Q03 | Cargo | How much latency comes from graph scheduling and critical paths? | Q01, Q02 | External now |
| PERF-Q04 | Cargo | How much work is multiplied by features, profiles, targets, and test modes? | Q01, Q02 | External now |
| PERF-Q05 | Cargo cache | What ordinary artifacts can be reused across workspaces safely? | Q02, Q04 | Compatibility boundary |
| PERF-Q06 | CI | Which cache topologies duplicate or discard useful work? | Q02, Q05 | External now |
| PERF-Q07 | IDE loop | Where do rust-analyzer, Cargo, and concurrent commands duplicate work or contend? | Q01, Q02 | External/upstream |
| PERF-Q08 | rustc startup | How much time goes to compiler startup and metadata loading? | Q01 | Upstream |
| PERF-Q09 | Frontend | Can parsing and tokenization reuse or parallelize more work? | Q01, Q08 | Upstream |
| PERF-Q10 | Frontend | Where does declarative macro expansion dominate or invalidate broadly? | Q01, Q09 | Upstream |
| PERF-Q11 | Frontend | Can name resolution and HIR lowering become more incremental or parallel? | Q09, Q10 | Upstream |
| PERF-Q12 | Semantics | Which type-inference and type-checking patterns dominate latency? | Q01, Q11 | External/upstream |
| PERF-Q13 | Semantics | Which trait-solving patterns create disproportionate work? | Q01, Q12 | External/upstream |
| PERF-Q14 | Semantics | When does borrow checking materially dominate builds? | Q01, Q12 | External/upstream |
| PERF-Q15 | MIR | Which MIR construction and optimization work repeats unnecessarily? | Q12, Q14 | Upstream |
| PERF-Q16 | Parallelism | Where does frontend parallelism help, stall, or regress? | Q08 through Q15 | Upstream |
| PERF-Q17 | Incremental | Which query dependencies cause false or broad invalidation? | Q01, Q12 through Q15 | Upstream |
| PERF-Q18 | Incremental | What does hashing, fingerprinting, serialization, and cache loading cost? | Q01, Q17 | Upstream |
| PERF-Q19 | Incremental | Which early compiler phases can become more incremental? | Q09 through Q11, Q17 | Upstream |
| PERF-Q20 | Cross-crate | Which edits are safe for Relink-Don't-Rebuild? | Q02, Q17 | Upstream |
| PERF-Q21 | Artifact reuse | Can check, build, lint, test, and doctest share more work? | Q02, Q04, Q17 | Cargo/upstream |
| PERF-Q22 | Proc macros | How can procedural-macro cost and inputs become observable and reusable? | Q01, Q10 | External/research |
| PERF-Q23 | Build scripts | How can build-script inputs, reruns, and outputs become precise? | Q01, Q02 | External/research |
| PERF-Q24 | Generics | Where do monomorphization and duplicate generic instances dominate? | Q01, Q17 | External/upstream |
| PERF-Q25 | Codegen | How should codegen-unit partitioning balance reuse, parallelism, and optimization? | Q24 | Configure/upstream |
| PERF-Q26 | LLVM | Which LLVM passes dominate development and release builds? | Q01, Q24, Q25 | Configure/upstream |
| PERF-Q27 | Backends | When should development builds use Cranelift or another supported backend? | Q01, Q26 | Configure/upstream |
| PERF-Q28 | Emission | How much latency comes from debug information and object emission? | Q01, Q25 | Configure/upstream |
| PERF-Q29 | Linking | When does linking dominate, and how much can incremental linking help? | Q01, Q28 | Configure/upstream |
| PERF-Q30 | Remote reuse | What provenance and identity model permits prewarmed or remote artifacts? | Q05, Q23 | Research |
| PERF-Q31 | Function reuse | Can function-level machine-code caching be correct and worthwhile? | Q18, Q24 through Q27 | Advanced research |
| PERF-Q32 | Crate slicing | Can dependencies compile only the metadata and code actually consumed? | Q17, Q20, Q24 | Advanced research |
| PERF-Q33 | System effects | How much latency comes from filesystem, memory, antivirus, virtualization, and hardware? | Q01 | External now |
| PERF-Q34 | Modularization | When does splitting or combining crates improve total iteration time? | Q03, Q17, Q20, Q24 | External advisor |
| PERF-Q35 | Validation | Can impact analysis reduce validation time without hiding coverage loss? | Q03, Q17, Q20, Q21 | External advisor |
| PERF-Q36 | Contribution | How can fixtures and evidence increase Rust performance contribution throughput? | All measured questions | Upstream program |

PERF-Q01 is complete. Its telemetry decision and findings are recorded in
`docs/research/2026-08-07-rust-latency-telemetry.md`.

PERF-Q02 is complete. Its layered Cargo identity model and cache safety
boundary are recorded in
`docs/research/2026-08-07-cargo-build-unit-identity.md`.

PERF-Q03 is complete. Its Cargo scheduling model, queue-delay vocabulary, and
critical-path advisor boundary are recorded in
`docs/research/2026-08-08-cargo-graph-scheduling.md`.

PERF-Q04 is complete. Its required-versus-suspicious unit-variant model and
command-multiplication evidence are recorded in
`docs/research/2026-08-08-cargo-build-unit-multiplication.md`.

PERF-Q05 is complete. Its immutable-unit eligibility, provenance, integrity,
isolation, locking, cleanup, and upstream-deference decision is recorded in
`docs/research/2026-08-08-cross-workspace-artifact-reuse.md`.

PERF-Q06 is complete. Its CI key, payload, immutable-entry, transport,
retention, trust, command-placement, and cache-economics model is recorded in
`docs/research/2026-08-08-ci-cache-topology.md`.

## Status vocabulary

- **Planned:** question and hypothesis exist; evidence collection has not begun.
- **In progress:** at least one investigation is active.
- **Evidence collected:** planned evidence exists; synthesis or review remains.
- **Proposed:** answer and recommendation are ready for role review.
- **Complete:** role review and decision disposition are recorded.
- **Blocked:** a named dependency, source, fixture, or tool is unavailable.
- **Deferred:** evidence does not justify current investment.

## Shared requirements

Every question must:

1. Follow the
   [FERRIUM research skill](../../../.claude/skills/research/SKILL.md).
2. Use the
   [build latency measurement contract](../../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
   for experiments.
3. Preserve the scopes in
   [Rust incremental reuse boundaries](../2026-08-07-rust-incremental-reuse-boundaries.md).
4. Distinguish external diagnosis, supported configuration, compatibility-bound
   research, and upstream compiler work.
5. Record `.roles` review before moving to Complete.
6. Keep the implementation gate closed unless a later pulse explicitly opens
   one bounded prototype.

## Initial prioritization

Start with:

1. PERF-Q01 - measurement and causal telemetry.
2. PERF-Q02 through PERF-Q04 - Cargo identity, graph, and multiplication.
3. PERF-Q22 and PERF-Q23 - macros and build scripts, using the shared evidence
   model.
4. PERF-Q08 through PERF-Q15 - frontend and semantic component baselines.
5. PERF-Q17 through PERF-Q21 - incremental and cross-crate reuse.
6. PERF-Q24 through PERF-Q29 - generics, codegen, backends, emission, and
   linking.
7. PERF-Q30 through PERF-Q36 - advanced reuse, modularization, validation, and
   contribution strategy.

This sequence tests whether FERRIUM can explain meaningful latency before
researching more speculative caches or compiler architecture.

## Coverage audit

| Existing findings | Covered by |
|---|---|
| FERRIUM-01: distinct build workloads | Q01, Q04, Q21, Q35 |
| FERRIUM-02: semantic correctness work | Q12, Q13, Q14, Q15 |
| FERRIUM-03: query causal model | Q17, Q18, Q19 |
| FERRIUM-04: incremental tradeoffs | Q17, Q18 |
| FERRIUM-05: monomorphization | Q24, Q25, Q31 |
| FERRIUM-06: LLVM cost | Q26, Q27 |
| FERRIUM-07: serial regions and parallel limits | Q03, Q16 |
| FERRIUM-08: debug information and linking | Q28, Q29 |
| FERRIUM-09: Cargo workflow and cache topology | Q02 through Q07, Q21 |
| FERRIUM-10: interpretation gap | Q01, Q03, Q35 |
| FERRIUM-11: contributor and review constraints | Q36 |
| FERRIUM-12 and FERRIUM-13: observability and Cargo graph | Q01 through Q04 |
| FERRIUM-14: cross-workspace cache | Q05, Q06, Q30 |
| FERRIUM-15: macros and build scripts | Q10, Q22, Q23 |
| FERRIUM-16: frontend parallelism and early phases | Q09 through Q11, Q16, Q19 |
| FERRIUM-17: query precision | Q17 through Q19 |
| FERRIUM-18: Relink-Don't-Rebuild | Q20 |
| FERRIUM-19: generic reuse | Q24, Q31, Q32 |
| FERRIUM-20: development backends | Q27 |
| FERRIUM-21: debug and object emission | Q28 |
| FERRIUM-22: linking | Q29 |
| FERRIUM-23: IDE and verification loop | Q07, Q21, Q35 |
| FERRIUM-24 through FERRIUM-34: reuse boundaries | Q02, Q08 through Q35 |

Every existing performance finding maps to at least one dedicated question.
Overlap is intentional where a boundary must be examined from Cargo, compiler,
and user-workflow perspectives.

## Registry role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted: correctness-sensitive caching, RDR, generic reuse, validation, and compiler changes remain separate questions with explicit gates. |
| Compiler Performance Engineer | Accepted: workloads and components are decomposed rather than hidden behind aggregate compile time. |
| Interop Boundary Auditor | Accepted: build scripts, native dependencies, emission, linking, targets, and private fixtures retain explicit boundaries. |
| AI Assurance Skeptic | Accepted: hypotheses, counter-evidence, failed measurements, and uncertainty are required for every question. |
| Ecosystem Strategist | Accepted: configuration, external tooling, upstream contribution, and advanced research are distinct paths. |
| Rust Maintainer | Accepted: the backlog prioritizes explanation and minimized fixtures before invasive source or compiler changes. |
| Native Platform Adopter | Accepted: CI, environment, cache provenance, reversibility, and operational constraints are represented. |
| Scope Keeper | Accepted: each issue has one separately executable question and decision. |
| Validation Checker | Accepted: Q01 establishes shared telemetry and every question inherits the measurement contract. |

No role pre-approves an intervention. Each question receives a new role review
after its evidence is collected.
