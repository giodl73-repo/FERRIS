# Pulse 02: Native Landscape Benchmark

## Goal

Select FERRIUM's first bounded technical wedge using cited ecosystem evidence
and measurable adoption criteria.

## Changes

- Compare interop, build intelligence, assurance, observability, and accelerator
  opportunities.
- Record existing tools, unmet needs, likely consumers, and defensibility.
- Define representative fixtures and benchmark commands.
- Recommend adopt now, prototype, and defer categories.
- Codify engineering principles, common pitfalls, and the required prototype
  gate.
- Review the doctrine against every parliament, stakeholder, and editorial
  role.
- Define public, synthetic, and privacy-preserving fixture tiers plus the build
  workload, edit-scenario, evidence, and statistical contracts.
- Record the leading Hammer-Temper opportunity: build-impact forecasting,
  rebuild explanation, validation planning, cache diagnosis, and evidence
  packets.
- Decompose the continuing latency research agenda by Cargo, frontend,
  incrementality, macros, generics, codegen, debug emission, linking, caches,
  and IDE/validation loops.
- Map reuse scopes from Cargo build units through rustc queries, cross-crate
  metadata, generics, codegen units, linking, workspace caches, and validation.
- Classify improvement paths as external now, compatibility-boundary
  prototypes, upstream contributions, or deferred research.
- Add the canonical `/research` skill with hypothesis-first planning,
  experiments, evidence capture, finding synthesis, privacy boundaries, and
  `.roles` review.
- Retain `/ferrium-research` as a compatibility alias rather than a divergent
  protocol.
- Decompose every identified Rust performance issue into a separately
  executable `PERF-Qxx` research question with dependencies, hypotheses,
  intervention paths, and role gates.
- Complete PERF-Q01 and freeze the latency telemetry stack: minimally
  instrumented wall clock, Cargo metadata, Cargo JSON, separate Cargo timing
  diagnostics, optional rustc self-profile, and rustc-perf for upstream claims.
- Capture the first public-fixture experiment, including instrumentation
  observer effect, artifact freshness, replayed build-script output, and
  immutable lockfile acquisition requirements.
- Complete PERF-Q02 and define Cargo graph-unit, artifact, symbol, freshness,
  and propagation identities.
- Validate feature, profile, mode, target, rustflag, source, build-script input,
  and workspace-relocation effects with a disposable synthetic fixture.
- Reject unrelated shared writable target directories pending a stronger
  provenance and isolation model, while retaining a read-only identity/session
  diff as the leading tool boundary.
- Complete PERF-Q03 and define Cargo graph readiness, ready-queue delay, summed
  unit work, makespan, active-job, observed-gating-chain, and counterfactual
  critical-path vocabulary.
- Confirm from Cargo source that ready-unit priority uses fixed unit costs plus
  transitive dependent fan-out rather than measured duration.
- Demonstrate on a controlled fixture that a slow root-gating unit can wait
  behind shorter high-fan-out chains, while preserving the negative result that
  manually prebuilding the apparent gate loses overlap and is slower.
- Retain read-only schedule explanation as the external opportunity; place
  duration-aware simulation behind a nightly compatibility boundary and defer
  command splitting, manifest rewrites, scheduler overrides, and upstream
  filing.

## Validation

- `git grep -n "Source\\|Confidence\\|Adopt now\\|Prototype\\|Defer" -- docs/research`
- `git grep -n "FP-\\|PF-\\|Accepted after" -- docs/governance/ENGINEERING_PRINCIPLES.md`
- `git grep -n "ES-\\|Tier 0\\|Tier 1\\|Tier 2\\|Acceptance gate" -- docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `git grep -n "BI-\\|RQ-\\|Stage [A-E]" -- docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md`
- `git grep -n "FERRIUM-2[4-9]\\|FERRIUM-3[0-4]\\|Help externally now\\|Contribute upstream" -- docs/research/2026-08-07-rust-incremental-reuse-boundaries.md`
- `git grep -n "FERRIUM-5[1-8]\\|queue delay\\|observed gating chain\\|counterfactual" -- docs/research/2026-08-08-cargo-graph-scheduling.md docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `git grep -n "hypothesis-led\\|FERRIUM-XX\\|Role review\\|Gate criteria" -- .claude/skills/research/SKILL.md`
- `git grep -n "PERF-Q0[1-9]\\|PERF-Q[12][0-9]\\|PERF-Q3[0-6]" -- docs/research/questions`
- `git diff --check`

## Status

In progress. The rustc architecture and build-performance research slice is
complete. The engineering doctrine and full `.roles` review are also complete;
the build-latency measurement contract now defines candidate fixture classes,
commands, edit scenarios, privacy boundaries, statistics, and the prototype
gate. The build-intelligence research program now records the leading
Hammer-Temper opportunity and the next component-level latency questions. The
incremental-reuse research now identifies the compiler and build boundaries,
their invalidators, evidence surfaces, and contribution paths. The canonical
`/research` skill now supports deeper hypothesis-led investigation without
opening the implementation gate. The performance registry now defines 36
separate research questions spanning measurement, Cargo, rustc, incrementality,
macros, generics, backends, linking, caching, modularization, validation, and
upstream contribution. PERF-Q01 is complete and establishes the evidence model
for the remaining questions. PERF-Q02 is complete and establishes the Cargo
identity and cache-safety model. PERF-Q03 is complete and establishes the Cargo
scheduling model. Cargo uses fixed-cost transitive fan-out rather than measured
duration. A controlled fixture exposed a slow direct dependency that waited in
the ready queue and gated final completion, but a manual prebuild intervention
was slower because it removed overlap. The adopted boundary is read-only
schedule and critical-path explanation; duration-aware simulation remains
compatibility-bound research. Automatic ordering, manifest, scheduler, and
upstream interventions remain closed. Exact larger-workspace snapshots, later
question execution, and cross-lane scoring remain.
