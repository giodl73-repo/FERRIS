# Cargo Graph Scheduling and Critical Paths

Date: 2026-08-08
Question: PERF-Q03
Status: Complete
Decision: define the scheduling evidence and product boundary FERRIUM will use
to explain Cargo queueing, graph constraints, and critical-path candidates.

## Executive conclusion

Cargo does not currently schedule ready units using measured compile duration.
At source revision
[`21c2a90636b4a1991eacd14eca439e7e308c1af4`](https://github.com/rust-lang/cargo/commit/21c2a90636b4a1991eacd14eca439e7e308c1af4),
every unit receives the same placeholder cost. Cargo gives higher priority to a
ready unit when more total fixed-cost work depends on it transitively.

That heuristic is useful for exposing parallel work, but it can delay a slow
unit that gates the requested root when shorter chains have more transitive
dependents. In the controlled fixture at two jobs, the compilation of a
deliberately slow direct dependency was graph-ready from the start, began
thirteenth, and waited 4.627 seconds after unit-graph construction before
starting.

This does not establish that starting the slow unit first would improve the
build. Manually prebuilding it before the workspace removed useful overlap and
increased the exploratory median from 8.838 seconds to 10.214 seconds.
Scheduling, resource contention, and compiler-internal parallelism interact;
queue delay is optimization headroom to investigate, not automatically
recoverable time.

FERRIUM should provide a read-only critical-path and queue-delay explanation
over observed Cargo units. A duration-aware counterfactual scheduler may be
prototyped behind a compatibility boundary using historical evidence. FERRIUM
should not add manifest edges, rewrite workspace structure, split commands,
override Cargo scheduling, or claim a faster schedule from one diagnostic run.

No issue, comment, branch, or pull request was created during this research.

## Decision supported

This research determines:

- the scheduling vocabulary inherited by later build-intelligence questions;
- which Cargo decisions are observable separately from graph constraints;
- whether job count alone is a sufficient recommendation;
- the safe first critical-path advisor boundary;
- which scheduling experiments require Cargo coordination or upstream work.

It does not authorize a Cargo fork, scheduler override, automatic command
reordering, workspace rewrite, or upstream filing.

## Research question

How much iteration latency comes from dependency topology, ready-queue
scheduling, serial critical paths, and target ordering?

## Starting and competing hypotheses

The starting hypothesis was that a small number of high-fan-out or slow crates
dominate many workspace builds while graph dependencies prevent additional
parallelism.

The investigation also tested these competing explanations:

1. Cargo already prioritizes units using observed duration.
2. Configuring more jobs removes most scheduling latency.
3. A known slow gating unit should be built separately before the workspace.
4. Apparent scheduling delay is primarily process or machine contention rather
   than queue choice.

The evidence refined rather than fully confirmed the starting hypothesis.
High-fan-out and slow units are different scheduling signals. Cargo currently
models the first through transitive fixed costs, not the second through
duration. More job slots reduce queue delay, but wall-clock improvement can
flatten as contention and limited graph width dominate.

## Evidence reviewed

### Local evidence

- `docs/research/2026-08-07-rust-latency-telemetry.md`
- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/research/questions/PERF-Q03-cargo-graph-scheduling.md`
- `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

### Cargo source

Source revision:
[`21c2a90636b4a1991eacd14eca439e7e308c1af4`](https://github.com/rust-lang/cargo/commit/21c2a90636b4a1991eacd14eca439e7e308c1af4)

- [job queue overview and scheduling model](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L1-L71)
- [fixed unit cost supplied by `JobQueue::enqueue`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L471-L487)
- [ready-job priority and jobserver-token handling](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L557-L590)
- [`DependencyQueue::queue_finished`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/util/dependency_queue.rs#L98-L145)
- [`DependencyQueue::dequeue`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/util/dependency_queue.rs#L148-L161)

### Cargo interfaces and issue state

- [scheduling-hints issue #7437](https://github.com/rust-lang/cargo/issues/7437)
- [`-Zbuild-analysis` tracking issue #15844](https://github.com/rust-lang/cargo/issues/15844)
- [Cargo unstable build-analysis documentation](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-analysis)
- [Cargo build-performance guidance](https://doc.rust-lang.org/cargo/guide/build-performance.html)

## Scheduling model

FERRIUM distinguishes these scopes:

| Scope | Question answered | Evidence |
| --- | --- | --- |
| Unit graph | Which units must finish before another unit can run? | Cargo unit registration and dependency indexes |
| Graph readiness | When did all dependencies of one unit finish? | Maximum dependency finish time |
| Ready-queue scheduling | How long did a ready unit wait before Cargo started it? | Unit start minus graph-ready time |
| Cargo process parallelism | How many unit jobs overlapped? | Unit start and finish events |
| Resource contention | Did concurrent units become slower or unstable? | Repeated wall clock plus diagnostic duration changes |
| Compiler-internal parallelism | How did one rustc process use additional tokens? | rustc or backend evidence, not the Cargo unit graph |
| Observed gating chain | Which completed units and queue waits determined the requested root in this run? | Dependency chain plus observed start and finish events |
| Counterfactual critical path | Which schedule might finish sooner under estimated costs? | Simulation; not directly observed fact |

Elapsed build time is not interchangeable with summed unit work. Their ratio
describes average active Cargo jobs, but it does not measure CPU utilization or
prove that unused configured slots were available to a particular rustc
process.

## Findings

### FERRIUM-51: Cargo prioritizes fixed-cost transitive fan-out, not measured
duration

**Sources**

- [job queue scheduling model](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L58-L71)
- [fixed placeholder cost](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L471-L475)
- [`DependencyQueue::queue_finished`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/util/dependency_queue.rs#L98-L145)

**Observation**

`JobQueue::enqueue` supplies cost `100` for every unit. When the graph is
finalized, `DependencyQueue` gives a node the sum of its own cost and the costs
of every distinct transitive dependent. Among ready nodes, Cargo dequeues the
highest resulting priority.

The source explicitly identifies persisted historical timing as a possible
future input rather than current behavior.

**Implication**

FERRIUM must describe the current rule as transitive fan-out priority, not
duration-aware critical-path scheduling. Historical duration simulation is a
compatible research opportunity because it tests a signal Cargo does not
currently use.

**Confidence:** high.

### FERRIUM-52: readiness, queueing, contention, and compiler parallelism are
different causes

**Sources**

- [ready-job dequeue and pending queue](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L557-L590)
- [Cargo and rustc jobserver relationship](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L31-L71)
- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observation**

A unit can be blocked by graph dependencies, ready but waiting in Cargo's
pending queue, running concurrently with slower peers, or internally consuming
additional rustc/backend tokens. Cargo prioritizes spawning more rustc
processes before allocating optional tokens for more parallelism inside an
existing rustc process.

The controlled diagnostic runs showed increasing summed unit duration as the
configured job count rose. That duration inflation cannot be attributed to the
dependency graph and is consistent with instrumentation and shared-resource
contention.

**Implication**

FERRIUM reports graph wait, queue delay, unit duration, overlap, and
compiler-internal parallelism separately. A queue-delay value is not presented
as recoverable wall-clock savings.

**Confidence:** high for the boundary; medium for the specific causes of
duration inflation because no hardware-counter or process-resource trace was
collected.

### FERRIUM-53: a slow root-gating unit can wait behind shorter high-fan-out
chains

**Source**

- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observed behavior**

The synthetic workspace contained two three-crate dependency chains and one
direct application dependency whose build script slept for 1.6 seconds. All
other build scripts slept for 150 milliseconds.

At two jobs, the slow dependency's build-script compilation had no graph
dependencies, but it started thirteenth and 4.627 seconds after unit-graph
construction. Cargo first advanced units in the two chains because those units
had more transitive dependents. The delayed unit then formed part of the
observed chain gating the final application.

**Implication**

Transitive fan-out is not always the same signal as observed duration or
root-gating risk. FERRIUM can identify ready units whose queue delay lies on the
observed completion chain and mark them as scheduling candidates without
claiming a better order yet.

**Confidence:** high for this controlled fixture.

### FERRIUM-54: more job slots reduce queue delay but do not guarantee
proportional wall-clock improvement

**Source**

- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observed behavior**

In separate diagnostic runs of the 22-unit synthetic graph:

| Jobs | Makespan | Summed unit work | Average active jobs | Peak | Slow-unit queue delay |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 10.9063 s | 10.8991 s | 0.999 | 1 | 5.3564 s |
| 2 | 8.1545 s | 15.9163 s | 1.952 | 2 | 4.6270 s |
| 4 | 8.0843 s | 25.7672 s | 3.187 | 4 | 3.7010 s |
| 24 | 6.4846 s | 39.2653 s | 6.055 | 7 | 0.0474 s |

Twenty-four configured jobs produced only seven simultaneous Cargo units
because graph width and completion timing limited ready work. Summed diagnostic
unit duration also increased substantially with concurrency.

**Implication**

FERRIUM may show job-count sensitivity and unused Cargo slots, but should not
recommend maximum parallelism from topology alone. Resource and compiler
behavior require repeated wall-clock evidence.

**Confidence:** high for the observed schedules; medium for generalization.

### FERRIUM-55: both fixtures showed diminishing exploratory returns after two
jobs

**Source**

- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observed behavior**

Cold, minimally instrumented `cargo check` medians were:

| Fixture | 1 job | 2 jobs | 4 jobs | 24 jobs |
| --- | ---: | ---: | ---: | ---: |
| Synthetic | 11.644 s | 8.838 s | 8.412 s | 8.414 s |
| METIS-CORE | 11.098 s | 8.079 s | 8.025 s | 7.716 s |

The synthetic four-job series was unstable under the measurement contract
because MAD divided by median exceeded 10%. Every series contained only three
exploratory samples, so none supports a promoted optimization claim.

The qualitative result was consistent across the two fixtures: parallelism
materially improved over one job, while gains after two jobs were much smaller
and not proportional to configured capacity.

**Implication**

FERRIUM should present a measured job-count curve, not a universal "use more
cores" recommendation. Larger and different workload classes remain necessary.

**Confidence:** medium because the direction repeated across two fixtures, but
the sample count and corpus remain small.

### FERRIUM-56: manually prebuilding the apparent gate can lose useful overlap

**Source**

- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observed behavior**

At two jobs, timing the slow dependency build followed by the workspace build
produced samples of 9.628, 10.214, and 10.983 seconds, with a 10.214-second
median. The ordinary workspace command's median was 8.838 seconds.

The manual split forced the slow dependency to finish before Cargo could
overlap it with independent chain work. It was about 15.6% slower in this
exploratory comparison.

**Implication**

FERRIUM must not implement command splitting or "prebuild the slow crate"
advice from queue delay alone. Counterfactual scheduling must preserve
available overlap and evaluate the whole completion time.

**Confidence:** high for the fixture; no general claim.

### FERRIUM-57: build scripts are schedulable units whose future cost is opaque
to Cargo

**Sources**

- [job definition](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L1-L19)
- [fixed placeholder cost](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/job_queue/mod.rs#L471-L475)
- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observation**

Cargo schedules build-script compilation and build-script execution as
separate units. Their current invocation cost is not known to the queue; each
receives the same placeholder cost as an ordinary rustc unit.

Build scripts can also launch external work through the shared jobserver. Their
duration, inputs, and resource behavior can differ sharply from rustc work.

**Implication**

Historical estimates need unit role, identity, command context, and uncertainty.
FERRIUM should not transfer one build-script duration across changed inputs,
platforms, or toolchains without compatibility evidence. PERF-Q23 remains the
owner of build-script input precision and reuse.

**Confidence:** high.

### FERRIUM-58: explanation is ready; automatic scheduling intervention is not

**Sources**

- [scheduling-hints issue #7437](https://github.com/rust-lang/cargo/issues/7437)
- [`-Zbuild-analysis` tracking issue #15844](https://github.com/rust-lang/cargo/issues/15844)
- Experiment:
  `docs/research/perf-q03-cargo-scheduling/results/EXP-01-critical-path-scheduling.md`

**Observation**

Current nightly build analysis exposes registered units, dependencies,
fingerprints, starts, finishes, and unblocked units in structured JSONL. Its
schema remains unstable. Stable Cargo surfaces do not expose equivalent queue
events or scheduler controls.

The experiment found a credible scheduling candidate and also demonstrated
that a simplistic intervention was slower.

**Implication**

The immediate FERRIUM opportunity is a read-only evidence consumer:

- reconstruct the observed unit schedule;
- distinguish graph wait from ready-queue delay;
- show observed gating chains and job-count sensitivity;
- explain why high fan-out and high duration differ;
- export minimized evidence for Cargo discussion.

A later prototype may simulate duration-aware schedules behind a versioned
nightly adapter. Scheduler overrides, Cargo patches, or upstream proposals
require a broader corpus, counterfactual validation, maintainer coordination,
and explicit owner approval.

**Confidence:** high.

## Model evolution

The starting hypothesis was refined in four ways:

1. High fan-out and high duration are separate properties; Cargo currently
   prioritizes the first.
2. A delayed slow unit can gate completion, but its queue delay is not
   automatically recoverable.
3. Additional job slots reduce scheduler queueing only while the graph exposes
   work and the machine can execute it efficiently.
4. Manual command ordering can worsen the build by removing overlap.

The evidence did not establish a universally superior scheduler, a preferred
historical-cost model, or a safe source/workspace rewrite.

## Recommendations

### Adopt now

- Add graph readiness, queue delay, observed gating chain, summed unit work,
  makespan, average active jobs, and peak Cargo jobs to the evidence vocabulary.
- Explain current Cargo priority as fixed-cost transitive fan-out.
- Present job-count curves and variance rather than maximum-core advice.
- Preserve the distinction between observed schedules and counterfactual
  critical paths.

Owner: FERRIUM.

Validation: PERF-Q04, PERF-Q34, and PERF-Q35 must retain unit-level schedule
identity and separate topology from resource contention.

### Prototype behind a compatibility boundary

- A read-only schedule reconstruction over nightly `-Zbuild-analysis`.
- A stable fallback that reports topology and observed artifacts without
  inventing queue events.
- Historical duration storage keyed by the PERF-Q02 identity model.
- Offline counterfactual simulation that compares whole-build completion time
  and preserves overlap.

Owner: FERRIUM.

Validation:

- schema-version fixtures;
- synthetic schedules with known readiness and queueing;
- held-out runs rather than fitting and evaluating the same trace;
- several public workspace shapes and job counts;
- no command execution, manifest edits, or scheduler control;
- uncertainty when historical identity or machine context differs.

### Reject or defer

- Automatically splitting Cargo commands to prebuild selected crates.
- Adding artificial manifest dependencies to influence order.
- Rewriting crate structure before PERF-Q34.
- Treating queue delay as guaranteed savings.
- Recommending one global job count from two fixtures.
- Depending on nightly build-analysis as the only usable mode.
- Filing scheduling issues or patches without explicit owner approval.

## Contribution path

1. **Explain externally now:** reconstruct schedules and produce minimized
   evidence without modifying Cargo.
2. **Configure or wrap later:** offer diagnostic job-count comparison while
   preserving ordinary Cargo commands.
3. **Research further:** validate duration estimation and counterfactual
   scheduling across larger public workspaces.
4. **Contribute upstream only with approval:** coordinate a fixture, report, or
   targeted Cargo change after the evidence identifies a repeatable failure in
   the current heuristic.

## Non-goals

- Replacing Cargo's job queue.
- Controlling rustc's internal parallelism.
- Claiming CPU utilization from Cargo unit overlap.
- Recommending crate splits or merges.
- Recommending reduced validation.
- Publishing private workspace topology or timing.

## Open questions

- How stable are per-unit duration estimates across warm state, machine load,
  toolchain updates, and source changes?
- Which scheduling objective best predicts developer-visible completion:
  requested root, first diagnostics, metadata availability, or all targets?
- How often do larger public workspaces exhibit a root-gating ready unit with
  material queue delay?
- Can a stable Cargo surface expose enough schedule information for useful
  partial explanation?
- What feedback would Cargo maintainers want before discussing historical
  costs or scheduling hints?

## Role review

| Role | Verdict | Required discipline |
| --- | --- | --- |
| Rust Safety Steward | Approve | No correctness edge, build-script dependency, or validation step may be removed to alter scheduling. |
| Compiler Performance Engineer | Approve | Wall clock, variance, unit work, queueing, overlap, and contention remain separate; exploratory samples are not promoted claims. |
| Interop Boundary Auditor | Approve | Build scripts and external jobserver work remain explicit opaque boundaries; no ABI or native-build claim is made. |
| AI Assurance Skeptic | Approve | Observed delay is separated from counterfactual savings, and the slower manual-prebuild result remains visible. |
| Ecosystem Strategist | Approve | The first capability consumes Cargo evidence and preserves an upstream contribution path rather than replacing Cargo. |
| Rust Maintainer | Approve | Output is read-only, ordinary Cargo commands remain valid, and no scheduling ritual or source rewrite is prescribed. |
| Native Platform Adopter | Approve with restriction | Advice must record machine, job count, toolchain, variance, and rollback to ordinary Cargo behavior. |
| Scope Keeper | Approve | Q03 ends at explanation and simulation boundaries; modularization, build scripts, and validation retain separate questions. |
| Validation Checker | Approve | Commands, fixtures, samples, unstable series, failed acquisition attempt, diagnostic limitations, and negative intervention are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

