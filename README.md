# FERRIS

**An enterprise application platform and evidence-driven engineering system
for Rust.**

FERRIS, formerly FERRIUM, is a research and engineering platform for the
unfinished parts of enterprise Rust: supported crate profiles, versioned
contracts, compiler-grounded AI assistance, fast builds, trustworthy language
boundaries, supply-chain assurance, concurrency observability, and portable
native execution.

Historical `FERRIUM-*` findings remain stable citation identifiers. New
findings use `FERRIS-*`.

## Enterprise platform architecture

FERRIS combines five replaceable layers:

1. idiomatic Rust crate APIs and Cargo SemVer;
2. [RUNE](https://github.com/giodl73-repo/RUNE) semantic descriptors,
   registries, compatibility reports, profiles, and adapters;
3. explicit C ABI, WIT/component, or wire-schema contracts at independently
   versioned boundaries;
4. renewable enterprise crate profiles with support, security, platform,
   stewardship, renewal, removal, and rollback evidence; and
5. OSPREY dependency, build, validation, and change intelligence.

RUNE remains a product-neutral standards repository. FERRIS consumes and
contributes to it rather than copying it into this repository.

See the
[enterprise Rust application-platform plan](docs/plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md)
and
[Rust contract and interface strategy](docs/research/2026-08-10-rust-contract-interface-strategy.md).

## Initial research lanes

| Lane | Question |
|---|---|
| Boundary | How can Rust enter C and C++ systems without weakening safety at the boundary? |
| Hammer | How can build causality, caching, linking, and workspace structure reduce iteration time? |
| Temper | How can generated native code carry auditable safety, provenance, and compliance evidence? |
| Lens | How can async and concurrent Rust become easier to observe, explain, and replay? |
| Furnace | How can ownership-aware native code target CPUs, GPUs, and accelerators portably? |

These are research lanes, not promised products or separate repositories.
FERRIS promotes a lane into implementation only after a cited research note,
measurable baseline, and bounded validation contract exist.

## Foundation state

FERRIS remains a research-and-governance repository. It contains no product
code, crates, packages, or implementation commitments yet. The first research
wave will compare the opportunity lanes, establish measurable baselines, and
recommend what—if anything—should be prototyped.

## Research

- [What the first seven performance questions established](docs/research/2026-08-08-first-seven-performance-questions.md)
- [rustc startup and metadata loading](docs/research/2026-08-08-rustc-startup-metadata.md)
- [Parsing and tokenization](docs/research/2026-08-08-parsing-tokenization.md)
- [Declarative macro expansion](docs/research/2026-08-08-declarative-macro-expansion.md)
- [Name resolution and HIR lowering](docs/research/2026-08-08-name-resolution-hir-lowering.md)
- [Type inference and type checking](docs/research/2026-08-08-type-inference-checking.md)
- [Trait-solving cost and reuse](docs/research/2026-08-08-trait-solving-cost-reuse.md)
- [Borrow-checking cost and incrementality](docs/research/2026-08-08-borrow-checking-cost-incrementality.md)
- [MIR construction and optimization](docs/research/2026-08-08-mir-construction-optimization.md)
- [Frontend parallelism](docs/research/2026-08-08-frontend-parallelism.md)
- [Query dependency precision and false invalidation](docs/research/2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead and reuse economics](docs/research/2026-08-08-incremental-cache-overhead.md)
- [Early-phase incrementality](docs/research/2026-08-08-early-phase-incrementality.md)
- [Relink-Don't-Rebuild and cross-crate interfaces](docs/research/2026-08-08-relink-dont-rebuild.md)
- [Reuse across check, build, lint, test, and doctest](docs/research/2026-08-08-command-artifact-reuse.md)
- [Procedural-macro cost, inputs, and reuse](docs/research/2026-08-08-procedural-macro-cost-input-reuse.md)
- [Build-script input, output, and rerun precision](docs/research/2026-08-09-build-script-input-output-precision.md)
- [Monomorphization and generic-instance reuse](docs/research/2026-08-09-monomorphization-generic-instance-reuse.md)
- [Codegen-unit partitioning](docs/research/2026-08-09-codegen-unit-partitioning.md)
- [LLVM optimization cost](docs/research/2026-08-09-llvm-optimization-cost.md)
- [Development codegen backends](docs/research/2026-08-09-development-codegen-backends.md)
- [Debug information and object emission](docs/research/2026-08-09-debug-information-object-emission.md)
- [Linking and incremental linking](docs/research/2026-08-09-linking-incremental-linking.md)
- [Remote artifact provenance and Rust Build Forest roots](docs/research/2026-08-09-remote-artifact-provenance.md)
- [Function-level machine-code caching](docs/research/2026-08-09-function-level-machine-code-caching.md)
- [Crate slicing and partial dependency compilation](docs/research/2026-08-09-crate-slicing-partial-compilation.md)
- [System effects on Rust build latency](docs/research/2026-08-09-system-effects-build-latency.md)
- [Workspace modularization and crate boundaries](docs/research/2026-08-09-workspace-modularization-crate-boundaries.md)
- [Impact-aware validation selection](docs/research/2026-08-09-impact-aware-validation-selection.md)
- [Rust performance contribution program closeout](docs/research/2026-08-09-rust-performance-contribution-program-closeout.md)
- [Rust capability coverage](docs/research/2026-08-09-rust-capability-coverage.md)
- [Rust foundational crate census](docs/research/2026-08-09-rust-foundational-crate-census.md)
- [Rust interchange contracts](docs/research/2026-08-09-rust-interchange-contracts.md)
- [Rust async portability](docs/research/2026-08-09-rust-async-portability.md)
- [Rust maintenance and stewardship](docs/research/2026-08-09-rust-maintenance-stewardship.md)
- [Rust security and provenance](docs/research/2026-08-09-rust-security-provenance.md)
- [Rust platform compatibility](docs/research/2026-08-09-rust-platform-compatibility.md)
- [Rust feature and version fragmentation](docs/research/2026-08-09-rust-feature-version-fragmentation.md)
- [Rust native dependency boundary](docs/research/2026-08-10-rust-native-dependency-boundary.md)
- [Rust crate discovery and selection](docs/research/2026-08-10-rust-crate-discovery-selection.md)
- [Rust compatibility-tested stack profiles](docs/research/2026-08-10-rust-compatibility-stack-profiles.md)
- [Rust ecosystem intervention decisions](docs/research/2026-08-10-rust-ecosystem-intervention-decisions.md)
- [Rust contract and interface strategy](docs/research/2026-08-10-rust-contract-interface-strategy.md)
- [Performance program role checkpoint](docs/research/2026-08-08-performance-program-role-checkpoint.md)
- [Rust Build Forest opportunity](docs/research/2026-08-08-rust-build-forest-opportunity.md)
- [Rust compiler performance: architecture, bottlenecks, and FERRIS opportunities](docs/research/2026-08-07-rustc-compiler-performance.md)
- [Rust latency component roadmap](docs/research/2026-08-07-rust-latency-component-roadmap.md)
- [Rust incremental reuse scopes and contribution boundaries](docs/research/2026-08-07-rust-incremental-reuse-boundaries.md)
- [Rust performance research-question registry](docs/research/questions/README.md)
- [Crates Series research-question registry](docs/research/questions/ecosystem/README.md)
- [Build latency measurement contract](docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
- [Rust performance contribution packet](docs/specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md)
- [FERRIS specification registry](docs/specs/README.md)
- [Query Forest component model](docs/specs/FOREST_COMPONENT_MODEL.md)
- [FOREST-001 nine-role review](docs/specs/reviews/FOREST-001-ROLE-REVIEW.md)
- [Build intelligence research program](docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md)
- [Crates Series: ecosystem and library research](docs/plans/ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md)
- [OSPREY program](docs/plans/OSPREY_PROGRAM.md)
- [FERRIS enterprise Rust application platform](docs/plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md)

## Review model

FERRIS uses the
[ROLES](https://github.com/giodl73-repo/ROLES) `.roles` convention. Rust safety,
compiler performance, interoperability, AI assurance, ecosystem strategy,
scope, validation, and adopter concerns are represented as explicit review
lenses.

The [FERRIS engineering principles](docs/governance/ENGINEERING_PRINCIPLES.md)
define the lab's decision rules, common failure modes, prototype gate, and
initial review disposition from every repository role.

## Repository skills

- `/research` runs hypothesis-led, cited compiler and native-tooling research.
- `/ferrium-wave` plans research-led capability waves.
- `/ferrium-pulse` executes bounded research or implementation pulses.
- `/ferrium-research` remains a compatibility alias for `/research`.

## Operating rules

1. Research before standardizing a language, protocol, benchmark, or product.
2. Treat compiler success as evidence, not proof of behavioral correctness.
3. Keep shared contracts product-neutral.
4. Measure build, runtime, safety, and migration claims.
5. Do not create implementation packages before the research gate selects a
   bounded prototype.
6. Record non-goals and rejected approaches.

## Non-goals

- Creating a general-purpose Rust replacement before a defensible wedge exists.
- Building another text-only coding assistant without compiler-grounded checks.
- Claiming formal verification, memory safety, or performance without evidence.
- Embedding portfolio-product semantics in shared crates.
- Treating experimental lane names as committed products.

## Validation

```powershell
git grep -n "FERRIS\\|FERRIUM-" -- README.md PRODUCT_PLAN.md docs context
git diff --check
```

## License

MIT.
