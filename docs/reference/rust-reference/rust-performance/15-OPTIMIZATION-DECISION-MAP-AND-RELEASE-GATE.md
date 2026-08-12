---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:optimization-decision-map-and-release-gate
kind: guide
module: rust-performance
section: rust-performance
title: Optimization Decision Map and Release Gate
status: source-custody
source_custody: partial
current_path: rust-performance/15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md
canonical_path: rust-performance/15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md
backsource_ids: [mdloom-backfill:rust-performance:15-optimization-decision-map-and-release-gate]
concepts: [optimization decisions, release gate, performance evidence, rollback, workload matrix, risk]
root_concepts: [performance release gate]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Optimization Decision Map and Release Gate

## The Big Picture

Optimization is complete only when the final artifact improves a defined outcome
across the required workload matrix without unacceptable correctness,
portability, build-time, operational, or maintenance regressions.

```
+=============================================================================+
|                         OPTIMIZATION DECISION                               |
|                                                                             |
| problem/SLO -> reproduce -> measure baseline -> profile limiting resource   |
|                                                    |                        |
|                              no clear cause? <------+                       |
|                                  | stop / improve evidence                  |
|                                  v                                          |
| choose lowest-risk mechanism -> implement -> verify correctness             |
|                                  |                                          |
|                                  v                                          |
| A/B workload matrix: runtime + memory + build + size + platform             |
|                                  |                                          |
|                   no material win? -> revert/record                         |
|                                  |                                          |
|                                  v                                          |
| final artifact -> canary -> guardrails -> release or rollback               |
+=============================================================================+
```

## Decision Map

```
START
  |
  v
Is there a measured user, capacity, cost, or delivery problem?
  | no -> do not optimize; record folklore as unproven
  |
 yes
  v
Is the workload and baseline reproducible?
  | no -> fix methodology [01]
  |
 yes
  v
What resource limits the outcome?
  |
  +-> CPU [03] -> layout/SIMD [05] or codegen [06]
  +-> allocation/RSS [04]
  +-> async queue/wait [07]
  +-> lock/cache coherence [08]
  +-> file/network I/O [09]
  +-> parse/copy/compress [10]
  +-> final code layout/size [11]
  +-> compile/link/CI [12]
  |
  v
Can a simpler algorithm, ownership, batching, or boundary change solve it?
  | yes -> prefer it
  | no  -> consider lower-level tuning
  v
Benchmark [13] -> production canary [14] -> RELEASE GATE
```

## Evidence Packet

Every proposed performance release should carry:

| Field | Required content |
|-------|------------------|
| Problem | User/SLO/capacity/cost/build outcome and owner |
| Baseline | Commit/artifact, toolchain, target, profile, features, environment |
| Workload | Corpus/traffic matrix, warm/cold state, concurrency, duration |
| Attribution | Profile/trace/counter proving the limiting resource |
| Change | Mechanism and why it should affect that resource |
| Results | Absolute and relative metrics, uncertainty, repetitions |
| Trade-offs | Memory, size, compile time, portability, complexity, safety |
| Final artifact | Exact build and symbol identity |
| Rollout | Canary scope, thresholds, owner, rollback artifact |
| Disposition | Ship, hold, revert, or accepted trade with rationale |

This packet separates measured behavior from folklore. A claim such as "Rust
iterators are zero-cost" is not evidence. A paired result showing a specific
iterator pipeline compiles to one loop and improves the defined workload is.

## Optimization Order

Prefer changes in descending generality:

| Order | Change class | Why |
|-------|--------------|-----|
| 1 | Remove unnecessary work / improve algorithm | Usually largest and most portable |
| 2 | Change ownership/data flow to remove copies/queues | Structural and broadly durable |
| 3 | Improve batching, locality, and contention | Mechanism-level but portable |
| 4 | Tune profiles, LTO, allocator, runtime | Configuration complexity |
| 5 | Target-specific SIMD, PGO, BOLT | Highest custody/portability burden |
| 6 | Unsafe micro-optimization | Adds proof and maintenance obligations |

This is not dogma. A profile may point directly to a target-specific kernel. The
order defines the burden of proof: lower-level changes need stronger evidence.

## Workload and Platform Matrix

```
                 small/median/large input
                         x
              low/normal/peak concurrency
                         x
                 cold/warm/steady state
                         x
              supported target/CPU/OS tier
```

Select a bounded risk-based matrix. At minimum include the workload that
motivated the change and one counterexample likely to regress. SIMD needs old
and new supported CPUs. Buffering needs small and large messages. PGO needs
training and holdout workloads. Allocator changes need burst/recovery and RSS.

## Release Gate

| Gate | Pass condition |
|------|----------------|
| Correctness | Existing tests plus optimization-specific invariants pass |
| Safety | No known new unsoundness; unsafe preconditions independently reviewed and challenged |
| Performance | Practical improvement on target workload; no unexplained red gate |
| Tails | p95/p99/timeout policy remains within SLO |
| Memory | Peak/steady RSS and allocation policy acceptable |
| Artifact | Size, startup, symbols, signing, and target compatibility acceptable |
| Delivery | Compile/link/CI cost fits budget |
| Stability | Stable Rust preferred; nightly dependency explicitly approved/pinned |
| Operations | Telemetry, canary, rollback, and on-call ownership ready |
| Documentation | Workload, commands, caveats, and known non-wins recorded |

Define non-waivable blockers before review. A known correctness failure,
unsoundness, invalid artifact identity, or unacceptable security/safety risk
normally blocks release rather than becoming a routine performance waiver.
Eligible trade-offs require a documented exception by the accountable risk
owner, expiry/revisit trigger, and rollback - never an implicit merge.

## Stable vs Nightly Release Policy

Stable compiler options and ecosystem tools can cover most optimization work.
Nightly is appropriate when its product value exceeds:

- six-week and toolchain migration risk;
- unstable option/output churn;
- dependency compatibility constraints;
- expanded CI matrix;
- incident response complexity.

Pin nightly by date, record required features/`-Z` options, and define an exit
plan. Nightly diagnostics used during investigation do not require shipping a
nightly binary.

## Reproducible Final Check

```
# Toolchain and graph identity.
rustc -Vv
cargo -V
cargo tree -e features

# Correctness and final artifact.
cargo test --locked --workspace --all-targets
cargo build --locked --profile profiling
cargo build --locked --release

# Repository-specific benchmark and final release-artifact smoke commands follow.
cargo bench
```

`--all-targets` compiles benchmark/example targets and may require external
services or platform tools in some repositories; tailor the documented gate.
`cargo test` normally executes test-profile artifacts, not the final release
binary. Run repository-specific smoke/integration tests against the exact
release, PGO, or BOLT-rewritten bytes and verify their digest and symbols.
Do not run a Linux profiler command on Windows or assume Azure-hosted agents
expose hardware counters. The command record must state the platform/tool
boundary.

## Stop Rules

Stop or revert when:

- the effect is below practical significance;
- results do not reproduce;
- the profile shows the target is not limiting;
- complexity or unsafe surface exceeds the benefit;
- one supported platform regresses beyond policy;
- PGO training data is unrepresentative;
- build/CI or operational custody cost exceeds saved runtime/cost;
- a simpler change produces equivalent benefit.

Recording a non-win prevents future teams from repeating folklore-driven work.

## Rollback Design

```
release N baseline artifact -------------------------------+
                                                           |
release N+1 optimized -> canary -> expand -> regression? --+-> restore N
                              |
                              +-> preserve traces/results for diagnosis
```

Feature flags can help for algorithm/runtime choices, but compile/link options,
allocators, PGO, and BOLT may require separate artifacts. Keep the known-good
artifact deployable until the observation window closes.

## Old World -> New World Bridge

| Prior art | Rust release gate |
|-----------|-------------------|
| Perf review board | Evidence packet and accountable waiver |
| VSTS gated check-in | Benchmark/CI gate with dedicated perf lane |
| Flighting/rings | Canary and guarded expansion |
| Native code security review | Unsafe/FFI/SIMD precondition review |
| Symbol server and rollback package | Exact Rust binary/PDB/DWARF custody |
| Configuration matrix | Cargo profile/features/target/toolchain matrix |

The universal bridge is disciplined change control: performance is one release
quality dimension alongside correctness, safety, operability, and delivery.

## Common Confusion Points

- **A faster microbenchmark is not a release decision.**
- **A profile attributes one run; it does not prove improvement.**
- **Stable output on one CPU does not prove portability.**
- **"No regression in mean" does not protect tails.**
- **PGO/BOLT artifacts require the same testing as source-built artifacts.**
- **Passing `cargo test` does not execute the final release binary by default.**
- **Unsafe optimization has a permanent proof cost.**
- **Nightly investigation tools do not imply a nightly release.**
- **Accepted trade-offs must name an owner and rollback.**

## Decision Cheat Sheet

| Situation | Decision |
|-----------|----------|
| No measured problem | Do not optimize |
| Workload not reproducible | Build the measurement contract first |
| No limiting resource identified | Profile/trace before editing |
| Algorithmic fix exists | Prefer it over codegen tuning |
| Small target-specific win | Weigh fleet coverage and custody cost |
| Clear benchmark win, product neutral | Hold until end-to-end/cost evidence |
| Product win, build-time regression | Compare total delivery/operational value and document waiver |
| Canary violates guardrail | Roll back, preserve evidence, investigate |
| All gates pass | Release with monitoring and retained rollback |

## Primary Sources

- Rust Performance Book: https://nnethercote.github.io/perf-book/
- Cargo profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- rustc PGO: https://doc.rust-lang.org/rustc/profile-guided-optimization.html
- Criterion.rs book: https://criterion-rs.github.io/book/
- Rust API Guidelines, safety and predictability context: https://rust-lang.github.io/api-guidelines/

## Related Guides

- Module map: [00-OVERVIEW.md](00-OVERVIEW.md)
- Methodology: [01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md)
- Benchmark gates: [13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md](13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md)
- Production validation: [14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md)
