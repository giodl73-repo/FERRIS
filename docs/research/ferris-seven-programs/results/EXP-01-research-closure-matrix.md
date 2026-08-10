# EXP-01: Ferris Research Closure Matrix

Date: 2026-08-10
Question: How should the completed Ferris research corpus map into seven
bounded programs?
Result: all 36 PERF, 12 ECOS, and 5 BLUE questions map to one or more programs
without requiring one monolithic product or transferring owner authority.

Program abbreviations:

- **FE:** Ferris;
- **TY:** Typebook;
- **PR:** Profiles;
- **BP:** Blueprint;
- **QF:** Query Forest;
- **CO:** Conformance; and
- **EB:** Ecosystem Bridge.

## Performance research

| Questions | Durable learning retained | Primary programs | Secondary programs |
|---|---|---|---|
| PERF-Q01 | Measurement requires exact workload, environment, commands, identity, failures, variance, and limitations | CO | QF, FE |
| PERF-Q02-Q04 | Cargo unit identity, critical paths, and feature/profile/target/activity multiplication remain distinct | BP, QF | FE, CO |
| PERF-Q05-Q07 | Reuse needs compatibility, isolation, topology, economics, contention, and foreground-latency evidence | QF, BP | FE, CO, EB |
| PERF-Q08-Q16 | Compiler startup, parsing, macros, resolution, semantics, MIR, and parallelism remain rustc-owned stages with explicit evidence | QF, EB | BP, CO |
| PERF-Q17-Q21 | Incremental precision, proof cost, early reuse, cross-crate interfaces, and cross-command stages require owner-specific identities | QF, BP | CO, EB |
| PERF-Q22-Q23 | Procedural macros and build scripts are native execution boundaries with declared, hidden, generated, replayed, and downstream effects | EB, QF | BP, CO |
| PERF-Q24-Q29 | Generics, CGUs, LLVM, development backends, debug emission, and linking have separate owners, capabilities, costs, and validation | QF, EB | BP, CO, PR |
| PERF-Q30-Q32 | Remote artifacts, function reuse, and crate slicing require provenance, compatibility, integrity, economics, and conservative deferral | QF, CO | BP, EB |
| PERF-Q33 | Filesystem, VM, memory, security, indexing, power, thermal, and concurrent pressure constrain attribution | CO, QF | FE |
| PERF-Q34 | Crate and workspace boundaries trade containment against graph, invocation, generic, test, and link multiplication | BP, CO | FE |
| PERF-Q35 | Package selection is not validation coverage; unknown inputs widen and selected evidence differs from full-reference evidence | BP, CO | FE, QF |
| PERF-Q36 | Upstream work must reduce to one owner-aligned, licensed, reproducible, issue-specific contribution packet | EB | CO, FE |

## Ecosystem research

| Questions | Durable learning retained | Primary programs | Secondary programs |
|---|---|---|---|
| ECOS-Q01-Q02 | Broad crate availability does not provide one renewable application contract; foundational status is role-based, not popularity approval | EB, PR | TY |
| ECOS-Q03 | Type, trait, version, feature, conversion, wrapper, serialization, and runtime interchange need explicit semantic contracts | TY | PR, EB, CO |
| ECOS-Q04 | Async portability is operation-specific across executor, spawn, I/O, time, cancellation, blocking, synchronization, context, and platform | TY, EB | PR, CO |
| ECOS-Q05-Q06 | Stewardship and assurance are renewable joined evidence, not release-age or zero-advisory verdicts | EB, QF | PR, CO |
| ECOS-Q07 | Compatibility is exact and stage-specific across compiler, features, host, target, provider, native tools, check, link, execute, and test | PR | EB, CO |
| ECOS-Q08 | Fragmentation evidence must retain requesters, versions, effective features, public exposure, cost, ownership, and renewal | EB, QF | PR, BP |
| ECOS-Q09 | Native source mode, discovery, ABI, generation, artifacts, deployment, and reproducibility shift ownership without removing the boundary | EB, TY | PR, CO |
| ECOS-Q10 | Discovery, evidence, eligibility, tradeoffs, decision, and renewal are separate; popularity only generates candidates | EB, PR | QF |
| ECOS-Q11 | Profiles are expiring consumer contracts over exact stacks, environments, stages, assurance, renewal, removal, and rollback | PR | TY, EB, CO |
| ECOS-Q12 | Interventions divide into document, adapt, standardize, contribute, support stewardship, bounded prototype, or reject/defer | EB, FE | CO, QF |

## Blueprint and product research

| Question | Durable learning retained | Primary programs | Secondary programs |
|---|---|---|---|
| BLUE-Q01 | Immutable roots use typed refs, compare-and-set updates, history, retention, revocation, and ordinary-Cargo fallback | QF | FE, CO |
| BLUE-Q02 | The global plan composes owner-local closures, resources, uncertainty, fallback, and replanning without replacing local planners | BP | FE, QF, CO |
| BLUE-Q03 | Scope is multi-dimensional, typed, conditional, owner-native, evidence-gated, and conservatively widened | BP, QF | CO |
| BLUE-Q04 | The competitive wedge is Cargo-native affected work across workspaces with one engine and bounded claims | FE | BP, CO, EB |
| BLUE-Q05 | Ferris is the public product; Blueprint is internal; `ferris` and `cargo ferris` share one engine | FE | BP, CO |
| BLUE-Q06 | The complete input corpus closes through seven bounded programs with one public product | FE | TY, PR, BP, QF, CO, EB |
| BLUE-Q07 | Enterprise governance belongs in Ferris; governed Microsoft and MCP connectors belong in Ecosystem Bridge | FE, EB | QF, CO |

## Coverage result

All 53 input questions plus the BLUE-Q06 synthesis and BLUE-Q07 enterprise
integration decisions have a primary program owner and at least one
specification path. Overlap is intentional where evidence, planning,
validation, contracts, profiles, governance, connectors, and external
ownership intersect.

The matrix does not authorize implementation or imply that every program must
ship as a separate package or service.
