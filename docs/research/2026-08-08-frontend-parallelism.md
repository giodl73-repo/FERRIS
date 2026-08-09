# Frontend Parallelism

Date: 2026-08-08
Question: PERF-Q16
Status: Complete
Decision: adopt frontend-owner and machine-session budgeting vocabulary now;
prototype nightly diagnostics and rustc-perf fixtures behind compatibility
boundaries; defer production flags, automatic job tuning, source rewrites, and
upstream activity.

## Executive conclusion

Rustc frontend parallelism is effective when a crate exposes thousands of
independent semantic owners. It is not a general acceleration of parsing, macro
expansion, name resolution, or one large function body.

At eight frontend jobs, 10,000 generic owners improved from 708.22 to
492.31 ms, 10,000 coercion owners from 646.90 to 468.73 ms, 5,000 borrow
owners from 438.57 to 336.90 ms, 5,000 const owners from 337.35 to 250.90 ms,
and 12,800 owners across 64 modules from 852.43 to 613.02 ms. Tiny compilation
regressed from 79.64 to 89.86 ms and one 10,000-operation body from 417.33 to
429.93 ms.

The gain trades machine resources for latency. Eight jobs used about
1.49x–1.87x CPU and 1.22x–1.51x peak memory on the useful controls. Sixteen
jobs generally added CPU without improving the best wall result. For generic
owners it used 2.77x the one-job CPU and was slower than eight jobs; for
modules it used 2.90x CPU and regressed from 613.02 to 654.46 ms.

Break-even depended on owner width. One hundred trivial owners regressed,
1,000 were approximately neutral, and 5,000–10,000 benefited from four to
eight jobs. This is a fixture-specific scheduling threshold, not a source-code
recommendation to split functions or create items.

The key operational gap is above one compiler or one Cargo process tree.
Rustc correctly acquires worker permits from an inherited jobserver, and one
four-crate Cargo build improved from 1,029.17 to 902.77 ms at eight frontend
jobs. Four independent Cargo sessions, however, created four independent
jobserver domains. With isolated targets, eight frontend jobs made the
machine-level batch 10.8% slower than one job, used 21.6% more CPU, and raised
peak memory from 1.62 to 2.11 GiB. Multiple terminals, worktrees, editors, CI
helpers, and AI agents therefore need a machine-level budget, not merely a
per-rustc thread setting.

Correctness held in the bounded controls. Metadata hashes matched across all
job counts; incremental untouched and local-edit frontiers matched; and every
diagnostic run emitted all 20 intended errors. Parallel diagnostics did not
retain source order: one-job stderr was byte-stable, while ten eight-job runs
produced ten owner orders. This is presentation nondeterminism, not observed
semantic loss, but it matters to tests and tooling.

Self-profile was a severe observer. It reversed useful minimally instrumented
speedups and produced overlapping event totals far above wall time. Provider
counts remain useful; profiled parallel wall time and summed query durations
do not become the primary benchmark.

The production recommendation remains closed. `--jobs-frontend` is unstable,
the accepted 2026 project goal still names incremental races, query-cycle
consistency, test infrastructure, Cargo and rustc-perf support, data
contention, finer-grained work, and future parallel name resolution and macro
expansion. FERRIUM should first explain owner granularity, serial regions,
resource cost, jobserver domain, and concurrent-session pressure.

## Decision supported

This research determines:

- which frontend work shapes benefited on the tested compiler;
- where serial and one-owner controls did not benefit;
- the CPU and memory price of wall-time reduction;
- the owner-count break-even in one parametric fixture;
- how rustc worker pools inherit one Cargo jobserver;
- why independent top-level build sessions still need machine coordination;
- whether successful outputs, incremental frontiers, and failures remained
  complete;
- which rustc-perf and upstream test gaps are defensible.

It does not authorize unstable production flags, automatic source
restructuring, compiler forks, a global scheduler implementation, or upstream
activity.

## Evidence reviewed

### Local evidence

- [Experiment](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md)
- [Parsing and tokenization](2026-08-08-parsing-tokenization.md)
- [Declarative macro expansion](2026-08-08-declarative-macro-expansion.md)
- [Name resolution and HIR lowering](2026-08-08-name-resolution-hir-lowering.md)
- [Type inference and type checking](2026-08-08-type-inference-checking.md)
- [Borrow-checking cost and incrementality](2026-08-08-borrow-checking-cost-incrementality.md)
- [MIR construction and optimization](2026-08-08-mir-construction-optimization.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [thread-pool construction and frontend default](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/util.rs#L176-L305)
- [jobserver acquisition and buffering](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/jobserver.rs#L15-L173)
- [parallel work grouping](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/sync/parallel.rs#L137-L208)
- [HIR owner and module scheduling](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/map.rs#L374-L519)
- [type-check owner scheduling](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_analysis/src/lib.rs#L155-L185)
- [borrow-check and module scheduling](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs#L1110-L1274)
- [currently serial slice helper](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/sync/parallel.rs#L188-L195)

### Performance and project-goal sources

rustc-perf revision:
`58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

- [frontend-thread model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/parallel_frontend.rs)
- [benchmark frontend axis](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs)
- [collector integration](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/execute/mod.rs)
- [accepted 2026 parallel frontend goal](https://github.com/rust-lang/goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [2023 parallel frontend announcement](https://blog.rust-lang.org/2023/11/09/parallel-rustc/)

## Findings

### FERRIUM-177: frontend jobs schedule owners, not arbitrary source volume

**Sources:** [HIR owner scheduling](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/map.rs#L374-L519) and [primary matrix](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#primary-wall-time-matrix).

**Observed behavior:** Thousands of independent generic, coercion, borrow,
const, and module owners accelerated. One large body and tiny source did not.

**Implication:** FERRIUM must report schedulable owner count, work per owner,
shared dependencies, and serial regions rather than source bytes or total item
count alone.

**Confidence:** High for the generated controls.

### FERRIUM-178: parsing, expansion, and current resolution controls remain serial

**Sources:** [serial slice helper](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/sync/parallel.rs#L188-L195), [no-analysis control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#serial-and-no-analysis-controls), and PERF-Q09 through PERF-Q11.

**Observed behavior:** Tiny, one-body, and 64-module no-analysis boundaries
were 5–9% slower at eight jobs. Prior parser, macro, and resolver controls also
showed no acceleration.

**Implication:** Frontend parallelism must not be described as whole-frontend
parallelism. Early global phases and their dependencies remain first-class
limits.

**Confidence:** High for the tested revision and controls.

### FERRIUM-179: useful break-even requires enough independent work

**Sources:** [owner scaling](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#owner-count-break-even).

**Observed behavior:** One hundred trivial owners regressed; 1,000 were
approximately neutral; 5,000 and 10,000 benefited from four to eight jobs.

**Implication:** Job selection needs owner granularity and estimated work, not
logical-core count alone. Function or module splitting is not implied.

**Confidence:** High for the parametric fixture, low as a universal threshold.

### FERRIUM-180: four to eight jobs captured most bounded gains

**Sources:** [primary matrix](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#primary-wall-time-matrix).

**Observed behavior:** Eight jobs was best for generic, coercion, and module
owners. Sixteen was only narrowly best for borrow and const owners and
regressed elsewhere.

**Implication:** FERRIUM should present a measured response curve and
diminishing-return point, not recommend the maximum available thread count.

**Confidence:** High on this 24-logical-processor host.

### FERRIUM-181: parallel latency gains consume more CPU and memory

**Sources:** [resource tradeoff](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#cpu-and-memory-tradeoff).

**Observed behavior:** Useful eight-job speedups used 1.49x–1.87x CPU and
1.22x–1.51x peak memory. Sixteen jobs sharply increased CPU on several
fixtures.

**Implication:** Reports need wall time, CPU work, occupied cores, memory, and
machine contention. Fastest single build is not always best system throughput
or interactive policy.

**Confidence:** High for wall and peak memory; medium for timer-quantized CPU.

### FERRIUM-182: explicit one-job parallel mode is close to serial, not free

**Sources:** [serial control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#serial-and-no-analysis-controls) and [thread-pool construction](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/util.rs#L176-L305).

**Observed behavior:** Most default-versus-one-job differences were small; the
const-owner median regressed about 6%.

**Implication:** Compatibility overhead should be measured separately from
multi-job speedup and should not be assumed to be exactly zero.

**Confidence:** Medium because several pairs were noisy.

### FERRIUM-183: one Cargo jobserver coordinates one process tree

**Sources:** [jobserver proxy](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/jobserver.rs#L15-L173) and [Cargo control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#process-cargo-and-session-contention).

**Observed behavior:** One Cargo build ran four rustc processes while inherited
tokens bounded active workers. Eight jobs improved wall time but increased
tree memory from 425.22 to 557.63 MiB.

**Implication:** Rustc and Cargo already have the correct local coordination
primitive. FERRIUM should explain and preserve it rather than introduce
competing per-process heuristics.

**Confidence:** High.

### FERRIUM-184: independent build sessions lack one machine budget

**Sources:** [multi-session control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#process-cargo-and-session-contention).

**Observed behavior:** Four isolated Cargo sessions at eight frontend jobs
were 10.8% slower than one frontend job, used 21.6% more CPU, and used 30.5%
more peak memory. Each top-level Cargo process owned a separate jobserver.

**Implication:** A credible FERRIUM gap is read-only machine/session topology
and, later, a bounded shared budget for terminals, worktrees, editors, CI
helpers, and AI agents. Shared-target locking, artifact reuse, foreground
priority, cancellation, and memory reserve must remain explicit.

**Confidence:** High for the synthetic four-session batch.

### FERRIUM-185: warm incremental work may leave little to parallelize

**Sources:** [incremental control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#incremental-control).

**Observed behavior:** Untouched and identical rewrites ran no body providers;
one owner edit produced one body frontier in both modes. Eight jobs did not
improve the warm scenarios. Output hashes matched.

**Implication:** Parallel settings need cache state and edit frontier. Cold
owner-width gains must not be projected onto incremental edits.

**Confidence:** High for provider counts and hashes.

### FERRIUM-186: parallel diagnostics can be complete but nondeterministic

**Sources:** [diagnostic control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#diagnostics-and-correctness) and [2026 goal](https://github.com/rust-lang/goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md).

**Observed behavior:** All runs emitted all 20 intended errors. One job used
one stable source order; eight jobs produced ten different orders.

**Implication:** Tooling and tests should consume structured diagnostics and
avoid relying on incidental stderr order. Query-cycle consistency remains an
upstream test requirement.

**Confidence:** High.

### FERRIUM-187: self-profile is not neutral under parallel execution

**Sources:** [self-profile control](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#self-profile-observer-effect).

**Observed behavior:** Profiling reversed useful wall-time speedups and summed
overlapping query durations exceeded wall time.

**Implication:** Minimally instrumented wall distributions remain primary.
Self-profile supports provider counts, blocked time, and scheduling diagnosis
only after observer calibration.

**Confidence:** High.

### FERRIUM-188: upstream coverage has an axis but still needs representative cases

**Sources:** [rustc-perf integration](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/execute/mod.rs), [experiment rustc-perf review](perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md#rustc-perf-and-project-goal-boundary), and [2026 goal](https://github.com/rust-lang/goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md).

**Observed behavior:** rustc-perf carries a frontend-thread axis and
jobserver-aware preparation, while the accepted goal still requests tool
support and more benchmarks.

**Implication:** Defensible candidates include owner-width break-even,
one-large-body negative control, memory/CPU response curves, incremental
parallel correctness, and diagnostic/query-cycle tests. No upstream activity
is authorized.

**Confidence:** High.

## Recommendations

### Adopt now

- Add owner granularity, serial region, jobserver domain, CPU work, peak
  memory, and concurrent-session pressure to FERRIUM build explanations.
- Distinguish fastest single build from machine throughput and interactive
  responsiveness.
- Preserve Cargo's inherited jobserver model and identify independent
  top-level coordination domains.
- Keep stable/default workflows primary; label frontend jobs as nightly and
  compatibility-bound.
- Preserve output hashes, incremental provider counts, diagnostic completeness
  and order, timeouts, failures, and observer overhead.

### Prototype behind a compatibility boundary

- A nightly adapter that reports frontend-job count, schedulable owner width,
  serial early phases, CPU/memory response, and diminishing returns.
- A read-only machine-session census joining Cargo/rustc process trees, target
  directories, inherited versus independent jobservers, foreground/background
  intent, and memory reserve.
- A bounded session-budget experiment that coordinates cooperating developer
  and AI-agent builds without changing Cargo artifact identity or hiding
  validation.
- rustc-perf candidates for owner-width scaling, one-body negative controls,
  independent-session pressure, and parallel incremental correctness.
- Structured diagnostic-order and query-cycle tests with maintainer guidance.

### Reject or defer

- Enabling unstable frontend jobs in production by default.
- Setting jobs from logical-core count alone or always selecting 8/16.
- Automatic function, item, or module splitting to create parallel owners.
- Ignoring CPU, memory, other sessions, editor responsiveness, or Cargo
  jobserver inheritance.
- Treating self-profile event sums as wall time.
- Global process killing, hidden priority changes, compiler forks, or upstream
  filing without approval.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: successful output identity, incremental frontiers, failures, and unstable boundaries remain explicit. |
| Compiler Performance Engineer | Accepted: interleaved wall distributions, owner scaling, CPU, memory, serial controls, and session contention are separated. |
| Interop Boundary Auditor | Accepted: no ABI or cross-language recommendation follows from compiler scheduling evidence. |
| AI Assurance Skeptic | Accepted: parallel diagnostic nondeterminism, profiler observer effect, and synthetic limitations remain visible. |
| Ecosystem Strategist | Accepted: the opportunity complements Cargo's jobserver and upstream rustc-perf rather than replacing them. |
| Rust Maintainer | Accepted: ordinary Cargo remains primary; no source restructuring or unexplained compiler ritual is proposed. |
| Native Platform Adopter | Accepted: production flags stay closed and machine-resource impact is part of the adoption contract. |
| Scope Keeper | Accepted: the work answers PERF-Q16 and leaves general invalidation, hashing, codegen, and a scheduler implementation to later gates. |
| Validation Checker | Accepted: commands, revisions, distributions, hashes, incremental edits, failures, contention, and limitations are recorded. |

## Non-goals

- Selecting one universal frontend-job count.
- Measuring codegen, linking, runtime performance, or all Cargo graphs.
- Proving stabilization readiness from synthetic Windows fixtures.
- Recommending source rewrites or unstable production configuration.
- Implementing a machine-wide scheduler in this research question.
- Creating upstream issues, branches, comments, or pull requests.
