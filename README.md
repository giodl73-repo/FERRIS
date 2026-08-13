# Ferris

**The cross-workspace enterprise build system for Rust.**

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
5. Ferris application modeling plus dependency, build, validation, and change
   intelligence.

RUNE remains a product-neutral standards repository. FERRIS consumes and
contributes to it rather than copying it into this repository.

## Ferris

Ferris is a Cargo-native cross-workspace build and application-control system.
One semantic engine has two entrypoints:

```console
ferris
cargo ferris
```

`ferris` exposes complete application, repository, multi-workspace, contract,
profile, policy, CI, deployment, root, and ref scope. `cargo ferris`, provided
by `cargo-ferris`, defaults to the current Cargo workspace through Cargo's
external-subcommand convention.

Ferris defines the missing application layer above Cargo packages and
workspaces. Blueprint is its internal normalized model and planning engine:

```text
Cargo graph truth
  + application definition
  + RUNE contracts
  + platform and support profile
  + validation and lifecycle evidence
  -> FERRIS Application Contract
```

For each proposed change, Blueprint may generate a non-executable **Blueprint
Plan**: a dynamic, application-level DAG that composes the affected Cargo,
compiler, contract, native, link, validation, cache, and resource closures.
The plan is global; the work is local. Cargo and every other owner retain their
own resolver, graph, freshness, scheduling, and execution rules.

Blueprint scope is a coordinate set rather than one tree. Package, target,
activity, feature, profile, platform, compilation, runtime test, validation,
contract, service, native, deployment, lifecycle, and evidence scopes remain
distinct and are joined through typed mappings. AI may propose finer scopes,
but deterministic policy controls narrowing and unknowns widen safely.

Cargo remains authoritative for packages, targets, features, sources, and
resolution. Blueprint adds consumer-owned application intent, component and
service relationships, contracts, providers, platforms, validation, support,
renewal, removal, and rollback. The Query Forest remains its internal evidence
model.

Query Forest roots are immutable. Blueprint uses typed branches, write-once
tags, promotion channels, local aliases, and retention pins to navigate them;
leases and tombstones are policy records, while labels are metadata only.
These refs support compare, promotion, rollback, and retention but are never
cache keys or correctness evidence.

See the [Ferris program](docs/plans/FERRIS_PROGRAM.md),
[seven-program architecture](docs/plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md),
[Microsoft enterprise integration](docs/plans/FERRIS_MICROSOFT_ENTERPRISE_INTEGRATION.md),
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

## Microsoft Rust leadership package

- [Microsoft Rust investment brief](docs/leadership/MICROSOFT_RUST_INVESTMENT_BRIEF.md)
- [Upstream and differentiated opportunity map](docs/leadership/MICROSOFT_RUST_UPSTREAM_OPPORTUNITY_MAP.md)
- [Leadership package scorecard](docs/leadership/MICROSOFT_RUST_LEADERSHIP_PACKAGE_SCORECARD.md)
- [Leadership PowerPoint](docs/leadership/MICROSOFT_RUST_INVESTMENT_DECK.pptx)
- [PowerPoint source](docs/leadership/MICROSOFT_RUST_INVESTMENT_DECK_SOURCE.ps1)

## Rust reference library

Ferris carries a generated mirror of MAXIM's reviewed Rust references so
research, specifications, and blueprint work can cite a repository-local
source:

- [Compact Rust card](docs/reference/rust-reference/languages/09-RUST.md)
- [Rust language guide](docs/reference/rust-reference/rust-language/00-OVERVIEW.md)
- [Rust implementation architecture](docs/reference/rust-reference/rust-architecture/00-OVERVIEW.md)
- [Rust application blueprints](docs/reference/rust-reference/rust-application-blueprints/00-OVERVIEW.md)
- [Rust production engineering](docs/reference/rust-reference/rust-production-engineering/00-OVERVIEW.md)
- [Rust crate ecosystem](docs/reference/rust-reference/rust-crate-ecosystem/00-OVERVIEW.md)
- [Rust interop and migration](docs/reference/rust-reference/rust-interop-migration/00-OVERVIEW.md)
- [Rust security assurance](docs/reference/rust-reference/rust-security-assurance/00-OVERVIEW.md)
- [Rust performance](docs/reference/rust-reference/rust-performance/00-OVERVIEW.md)
- [Mirror policy and synchronization](docs/reference/rust-reference/README.md)

MAXIM remains canonical. Mirrored files are synchronized and hash-checked
rather than edited independently in Ferris.

## Rust engineering library

Ferris owns the applied operating guidance that joins Rust code to application
intent, platform support, renewable profiles, generated-change evidence,
upstream ownership, and conformance:

- [AI-assisted Rust engineering](docs/engineering/ai-assisted-rust/00-OVERVIEW.md)
- [Platform and target engineering](docs/engineering/platform-target-engineering/00-OVERVIEW.md)
- [Validated stack profiles](docs/engineering/validated-stack-profiles/00-OVERVIEW.md)
- [Maintainer and upstream contribution](docs/engineering/maintainer-upstream/00-OVERVIEW.md)
- [Reference implementations](docs/engineering/reference-implementations/00-OVERVIEW.md)
- [Engineering library index](docs/engineering/README.md)
- [Placement and gap-closure decision](docs/research/2026-08-12-ferris-rust-engineering-gaps.md)
- [Nine-role engineering-library review](docs/engineering/FERRIS-RUST-ENGINEERING-LIBRARY-ROLE-REVIEW.md)

These guides translate the existing research and Draft specifications into
repeatable workflows. They do not authorize product code or turn examples,
profiles, or AI proposals into correctness or support claims.

## Foundation state

FERRIS has completed the separately approved read-only implementation wave
through Pulse 19's ordinary-Cargo preservation control. The bounded product
surface includes
local `plan`, `explain`, declared-workspace `graph`, passive local `doctor`,
and non-executable `profile-diff` over two explicit experimental evidence
files. Pulse 13 adds a typed single-threaded process boundary for catchable
panics and output write failures. Its immutable cutoff passed the sealed
FHIF-030 held-out score; no held-out profile-diff claim is made.

The research corpus and 22-specification spine remain at Draft status.
Affected-only scope, query, execution, mutation, connectors, MCP, AI narrowing,
approval, deployment, remote evidence, and production claims remain
unauthorized. Profile diffing does not generate profiles, invoke Cargo or
owner tools, interpret evidence states, expose raw section values, or establish
compatibility, support, certification, or approval. Profile identifiers,
revisions, consumers, and JSON object keys are validated output-visible
metadata and must not contain secrets.

The initial command boundaries are recorded in
[`Pulse 01: Local Plan and Explain`](context/waves/2026-08-11-read-only-planning/pulses/pulse-01.md),
[`Pulse 02: Declared Workspace Graph`](context/waves/2026-08-11-read-only-planning/pulses/pulse-02.md),
and [`Pulse 04: Passive Doctor`](context/waves/2026-08-11-read-only-planning/pulses/pulse-04.md).
The current process boundary and held-out result are recorded in
[`Pulse 13`](context/waves/2026-08-11-read-only-planning/pulses/pulse-13.md)
and
[`Pulse 14`](context/waves/2026-08-11-read-only-planning/pulses/pulse-14.md).
The nine-family development conformance matrix and its role review are
recorded in
[`Pulse 15`](context/waves/2026-08-11-read-only-planning/pulses/pulse-15.md),
the
[fixture matrix](tests/fixtures/profile-evidence/MATRIX.md), and the
[Pulse 15 review](docs/plans/reviews/PULSE-15-ROLE-REVIEW.md).
The public profile-diff held-out design is recorded in
[`Pulse 16`](context/waves/2026-08-11-read-only-planning/pulses/pulse-16.md),
the
[held-out program](docs/simulations/profile-diff-held-out/README.md), and the
[Pulse 16 review](docs/plans/reviews/PULSE-16-ROLE-REVIEW.md). No executable
fixture, oracle, score, or pass is claimed.

The successor
[Platform Profile Conformance wave](context/waves/2026-08-12-platform-profile-conformance/WAVE.md)
is active with documentation-only Pulse 01 complete. Its
[program map](docs/engineering/validated-stack-profiles/07-PLATFORM-PROPOSED-PROGRAM.md)
sequences all nine controlled families, lifecycle controls, independently
owned held-out gates, and the eventual PLATFORM-001 Proposed review. It adds
no current product, owner-execution, support, or status authority.
Pulse 02 now freezes the controlled-fixture
[`ferris.platform-profile/v1` schema](docs/schemas/platform-profile/README.md)
and exact negative controls. The schema is not a generated profile, support
catalog, completed family, product parser, RUNE v1 claim, or PLATFORM-001
status change.
Pulse 03 adds only a dependency-free Rust integration harness for those
controls. Its
[Windows and Unix receipt](docs/plans/validation/PULSE-03-SCHEMA-HARNESS.md)
records exact valid, unsupported, invalid, and blocked outcomes without adding
a production parser or family evidence.
Pulse 04 completes the first controlled v1 family:
[pure data](docs/plans/validation/PULSE-04-PURE-DATA-FAMILY.md). Its two
zero-dependency revisions preserve locked/offline owner Cargo workflows and
exact profile digests on Windows and Unix; no other family or lifecycle gate
is implied.
Pulse 05 completes the controlled
[CLI/configuration family](docs/plans/validation/PULSE-05-CLI-CONFIG-FAMILY.md)
with exact process precedence, bounded explicit-file failures, owner workflow
preservation, and stable profile digests on Windows and Unix.
Pulse 06 completes the controlled
[hosted-service family](docs/plans/validation/PULSE-06-HOSTED-SERVICE-FAMILY.md)
with in-process health, malformed-request, cancellation, readiness, and
unavailable evidence while preserving owner workflows and excluding network
and deployment authority.
Pulse 07 completes the controlled
[embedded/`no_std` family](docs/plans/validation/PULSE-07-EMBEDDED-NO-STD-FAMILY.md)
with host behavior tests and exact `thumbv7em-none-eabi` compilation while
retaining device execution as unavailable.
Pulse 08 completes the controlled
[browser-WASM family](docs/plans/validation/PULSE-08-BROWSER-WASM-FAMILY.md)
with exact escaping, language-metadata rejection, and
`wasm32-unknown-unknown` compilation while retaining browser execution as
unavailable.
Pulse 09 completes the controlled
[WebAssembly-component family](docs/plans/validation/PULSE-09-WASM-COMPONENT-FAMILY.md)
with exact WIT revisions and non-empty `wasm32-wasip2` artifacts while
retaining component-runtime execution as unavailable.
Windows and Ubuntu 24.04.4 WSL2 development validation is recorded in
[`Pulse 17`](context/waves/2026-08-11-read-only-planning/pulses/pulse-17.md),
the
[cross-platform receipt](docs/plans/validation/PULSE-17-CROSS-PLATFORM-VALIDATION.md),
and the
[Pulse 17 review](docs/plans/reviews/PULSE-17-ROLE-REVIEW.md). This is not
native Linux support or held-out evidence.
The public-CLI input and working-directory non-mutation proof is recorded in
[`Pulse 18`](context/waves/2026-08-11-read-only-planning/pulses/pulse-18.md),
the
[filesystem immutability receipt](docs/plans/validation/PULSE-18-FILESYSTEM-IMMUTABILITY.md),
and the
[Pulse 18 review](docs/plans/reviews/PULSE-18-ROLE-REVIEW.md). It is not a
whole-system sandbox or complete removal proof.
The representative owner-native before-and-after Cargo control is recorded in
[`Pulse 19`](context/waves/2026-08-11-read-only-planning/pulses/pulse-19.md),
the
[ordinary Cargo preservation receipt](docs/plans/validation/PULSE-19-ORDINARY-CARGO-PRESERVATION.md),
and the
[Pulse 19 review](docs/plans/reviews/PULSE-19-ROLE-REVIEW.md). It is one
zero-dependency development control, not universal lifecycle evidence.
The Pulse 13 held-out result is the
[public-safe FHIF-030 result](docs/simulations/held-out/PUBLIC_SAFE_DOCTOR_RESULT_022.md).

```console
cargo run -p ferris-cli -- plan --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- explain --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- graph --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- doctor --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- profile-diff --before <PROFILE_JSON> --after <PROFILE_JSON>
```

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
- [Rust build-state references](docs/research/2026-08-10-rust-build-state-references.md)
- [Blueprint federated execution planning](docs/research/2026-08-10-blueprint-federated-execution-planning.md)
- [Blueprint cross-command scope model](docs/research/2026-08-10-blueprint-cross-command-scope-model.md)
- [Blueprint competitive positioning and CLI strategy](docs/research/2026-08-10-blueprint-competitive-positioning.md)
- [Ferris product naming decision](docs/research/2026-08-10-ferris-product-naming.md)
- [Ferris seven-program synthesis](docs/research/2026-08-10-ferris-seven-program-synthesis.md)
- [Ferris Microsoft enterprise integration](docs/research/2026-08-10-ferris-microsoft-enterprise-integration.md)
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
- [Ferris specification simulations](docs/simulations/README.md)
- [Ferris specification simulation method](docs/research/2026-08-10-ferris-specification-simulation-method.md)
- [Query Forest component model](docs/specs/FOREST_COMPONENT_MODEL.md)
- [FOREST-001 nine-role review](docs/specs/reviews/FOREST-001-ROLE-REVIEW.md)
- [Ferris public-contract review](docs/specs/reviews/FERRIS-PUBLIC-CONTRACTS-ROLE-REVIEW.md)
- [Ferris seven-program review](docs/plans/reviews/FERRIS-SEVEN-PROGRAM-ROLE-REVIEW.md)
- [Ferris Microsoft integration review](docs/plans/reviews/FERRIS-MICROSOFT-INTEGRATION-ROLE-REVIEW.md)
- [Build intelligence research program](docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md)
- [Crates Series: ecosystem and library research](docs/plans/ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md)
- [Ferris program](docs/plans/FERRIS_PROGRAM.md)
- [Blueprint planning engine program](docs/plans/BLUEPRINT_PROGRAM.md)
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
5. Implement only capabilities explicitly selected by an approved bounded
   pulse.
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
