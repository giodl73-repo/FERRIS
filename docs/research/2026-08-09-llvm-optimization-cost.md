# LLVM Optimization Cost

Date: 2026-08-09
Question: PERF-Q26
Status: Complete
Decision: adopt a read-only LLVM cost ledger that separates rustc IR
translation, pre-link optimization, LTO/import optimization, machine-code
passes, object and debug emission, and linking; join pass events to exact
toolchain, CGU, IR scope, Rust shape, observer-effect calibration, runtime, and
size controls; prototype exact-nightly trace and self-profile adapters plus
isolated profile comparison; contribute minimized pass-pathology fixtures
upstream; defer automatic profile, vectorization, inlining, LTO, target-feature,
pass-pipeline, backend, and source changes.

## Executive conclusion

"LLVM time" is not one stage.

rustc first translates each codegen unit into LLVM IR. LLVM then runs a
pre-link optimization pipeline whose shape depends on optimization level, LTO
stage, size attributes, target, and compiler revision. ThinLTO or fat LTO can
add a second optimization phase after imports or module merging. A separate
machine pipeline performs instruction selection, scheduling, register
allocation, prologue and epilogue work, assembly printing, and object emission.
The external linker remains another system.

These boundaries matter because their costs respond to different inputs.
Monomorphized IR volume can amplify every function pass. Loop and branch shape
changes which analyses and transformations execute. Codegen-unit count changes
parallel wall time and repeated pipeline setup. LTO changes the set of
functions visible together. Debug information increases machine and emission
work. Target features can make one vectorizer case pathological without
implicating the rest of the compiler.

The PERF-Q26 synthetic fixture made that separation visible. At one CGU, O0
compiled in 385.3 ms while O3 took 959.8 ms, but O0 runtime was 3.27 times the
O3 median. Removing optimization was therefore a large compile-time win and a
large runtime loss, not a free development setting.

O2 and O3 were nearly indistinguishable in end-to-end compilation and had
overlapping runtime distributions in this fixture. O3 added an observable SLP
vectorizer region, but other pass costs moved in both directions. An
optimization level is a policy bundle, not an ordinal prediction that every
higher level costs more or always produces faster code.

The size modes were more striking. Os and Oz shortened synthetic compilation
30.9% and 32.8%, reduced executable bytes about 20%, and produced only small,
overlapping runtime differences. On METIS-CORE, they shortened isolated
root-crate compilation while producing larger Rlib archives. Intermediate
archive size therefore did not predict final executable size or runtime.

LLVM 23 also changed what the size labels mean internally. Separate Os and Oz
pass pipelines were removed. rustc maps both labels to LLVM's O2 pipeline and
relies on `optsize` and `minsize` function attributes. The public control
confirmed that O2 and Os contained the same named pass classes but performed
different amounts of work. FERRIUM must record both pipeline level and
function policy.

Pass dominance was shape-specific. The loop family dominated named function
optimization under O1 through O3. `ModuleInlinerWrapperPass` was the largest
inclusive IR region in both the synthetic and public controls.
`InstCombinePass`, loop vectorization, SLP vectorization, and machine
instruction selection were separately material. Under explicit ThinLTO and
fat LTO, dependency and runtime functions outside the generated fixture
families became the largest additional region.

The timing hierarchy is nested. The inliner wrapper contains call-graph and
function passes; `OptModule` contains function optimization; loop managers
contain loop passes. Adding those durations double counts work. A useful report
must preserve parent-child scope rather than manufacture a flat total.

The diagnostic itself was expensive. The combined LLVM time-trace and rustc
time-pass mode increased synthetic wall time from 12% to 43% depending on
configuration. Its time-trace file reached 43.5 MiB, and one METIS-CORE trace
reached 99.9 MiB. Diagnostic wall time cannot be the primary benchmark.

The coarse rustc `LLVM_passes` timer also did not numerically equal nested pass
work, especially under multi-CGU and LTO modes. It is a region timer, not a
sum-of-passes interface.

Existing tools remain necessary:

- Cargo timings locate crate and target cost;
- rustc self-profile locates compiler regions and can expose LLVM pass events;
- LLVM time traces expose nested per-thread events;
- `cargo-llvm-lines` exposes IR volume and generic copies;
- `opt`, `llc`, and pass bisection minimize LLVM cases;
- native profilers expose implementation hotspots;
- rustc-perf validates an upstream compiler change.

The gap is the join across them. A maintainer still lacks one explanation that
says:

- which Rust and mono-item shapes entered LLVM;
- which CGU and optimization stage owned them;
- which pass and IR scope became expensive;
- whether that time was parallel work or wall-critical work;
- whether size, runtime, memory, debug output, or link cost improved;
- whether the evidence came from a minimally instrumented run or an
  observer-affected trace.

FERRIUM should add that join to the compiler query plan and labeled Build
Forest as a read-only LLVM cost ledger. It should not disable vectorization,
lower optimization, enable LTO, change target features, rewrite source, or
select another backend automatically.

The measured platform is x86_64 Windows MSVC. Nightly pass names and trace
schemas are unstable. The compatibility adapter must bind to the exact rustc
and LLVM revision, calibrate observer effect, preserve unknown events, and fail
closed. Ordinary Cargo and minimally instrumented compilation remain
authoritative. Disabling the diagnostic is the rollback.

## Decision supported

This research determines:

- which LLVM optimization, LTO, machine-code, emission, and link boundaries
  belong in the compiler query plan;
- why optimization level, size policy, LTO stage, CGU topology, target, and IR
  shape must remain separate dimensions;
- which pass events can explain a costly compile without being added
  incorrectly;
- which runtime, size, memory, and behavior controls every recommendation must
  retain;
- how LLVM 23 changes Os and Oz interpretation;
- where existing profiling tools stop and a FERRIUM evidence join can begin;
- which minimized upstream fixtures are defensible.

It does not authorize automatic optimization-level, size-policy, LTO,
vectorization, unrolling, inlining, target-feature, codegen-unit, debuginfo,
backend, linker, Cargo profile, or source changes.

## Evidence reviewed

### Local evidence

- [EXP-01 LLVM pass cost matrix](perf-q26-llvm-optimization/results/EXP-01-llvm-pass-cost-matrix.md)
- [Codegen-unit partitioning](2026-08-09-codegen-unit-partitioning.md)
- [Monomorphization and generic-instance reuse](2026-08-09-monomorphization-generic-instance-reuse.md)
- [MIR construction and optimization](2026-08-08-mir-construction-optimization.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler and LLVM sources

- [Pinned rustc LLVM write path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_llvm/src/back/write.rs)
- [Pinned LLVM pass wrapper](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp)
- [Pinned rustc LLVM self-profile bridge](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_llvm/src/back/profiling.rs)
- [Pinned rustc LTO path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_llvm/src/back/lto.rs)
- [LLVM pass reference](https://llvm.org/docs/Passes.html)
- [LLVM PR removing separate Os and Oz pipelines](https://github.com/llvm/llvm-project/pull/191363)

### Profiling and upstream controls

- [rustc developer guide: profiling](https://rustc-dev-guide.rust-lang.org/profiling.html)
- [rustc developer guide: debugging LLVM](https://rustc-dev-guide.rust-lang.org/backend/debugging.html)
- [rustc self-profile flag](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/self-profile.html)
- [measureme](https://github.com/rust-lang/measureme)
- [cargo-llvm-lines](https://github.com/dtolnay/cargo-llvm-lines)
- [SLP compile-time regression, rust-lang/rust #157302](https://github.com/rust-lang/rust/issues/157302)
- [Undesirable loop vectorization, rust-lang/rust #102709](https://github.com/rust-lang/rust/issues/102709)
- [rustc-perf](https://perf.rust-lang.org/)

## Current compiler model

### Pre-link optimization and machine code are separate pipelines

rustc calls LLVM's new pass manager for module optimization. The pinned
compiler maps:

- `PreLinkNoLTO` to the per-module default pipeline;
- `PreLinkThinLTO` and `PreLinkFatLTO` to the ThinLTO pre-link pipeline;
- `ThinLTO` to the post-import ThinLTO pipeline;
- `FatLTO` to the full LTO pipeline.

Object generation uses a separate output pass manager. Instruction selection,
machine scheduling, register allocation, exception preparation, assembly
printing, and object writing therefore need a machine and emission region
beside the IR-pass region.

### Optimization level selects policy, not one pass

O0, O1, O2, and O3 select different pipeline policies. The pinned bridge also
wires loop unrolling, loop interleaving, loop vectorization, SLP vectorization,
and merge-functions controls into LLVM's pipeline tuning options.

One level can add a pass, change a threshold, alter an attribute, or transform
IR so that later passes have less work. Pass cost is not monotonic by level.

### LLVM 23 expresses size policy through attributes

LLVM PR `#191363` removed separate Os and Oz pipeline definitions. The pinned
rustc bridge maps Os and Oz to O2 when built with LLVM 23 or later. The intended
difference comes from `optsize` and `minsize` attributes.

This means a report needs:

- the rustc label requested by Cargo or the user;
- the LLVM pipeline level actually selected;
- relevant function attributes;
- which pass classes executed;
- how much work each scope performed.

"Os uses a smaller pipeline" is no longer an accurate LLVM 23 explanation.

### Pass events are hierarchical

LLVM time trace events represent modules, functions, loops, strongly connected
components, analyses, adaptors, wrappers, transformations, and machine passes.

An inclusive wrapper duration answers "how long was this region active?" A
child duration answers "how much time was attributed to this nested event?"
They are not independent samples.

### Pass cost needs a Rust-side shape

Pass name alone is not actionable. `InstCombine` can be expensive because of
many generic copies, one very large function, repeated simplification after
inlining, or imported LTO code. The vectorizers respond to loops, target
features, memory access, and dependence shape. Instruction selection and
register allocation respond to the transformed machine problem.

FERRIUM must connect:

```text
Rust owner / generic instance / inline copy
  -> CGU
  -> LLVM module / function / loop / SCC
  -> optimization stage
  -> pass event hierarchy
  -> machine pass region
  -> emitted artifact
  -> runtime and size control
```

## Findings

### FERRIUM-336: LLVM backend cost has distinct ownership boundaries

**Sources:** pinned `write.rs`, `PassWrapper.cpp`, `lto.rs`, and EXP-01.

**Observed behavior:** IR translation, pre-link optimization, LTO
optimization, instruction selection, register allocation, emission, and
linking used different paths and responded differently to the matrix.

**Implication:** Never report one undifferentiated "LLVM time." Preserve stage,
CGU, thread, artifact, and linker boundaries.

**Confidence:** High.

### FERRIUM-337: optimization level is a policy bundle, not an intensity scalar

**Sources:** pinned pipeline selection, pipeline tuning options, and EXP-01.

**Observed behavior:** O2 and O3 had nearly equal synthetic and METIS wall
medians. O3 added SLP work, while other pass durations moved in both
directions.

**Implication:** Compare complete supported profiles. Do not infer cost or
runtime from the ordinal level alone.

**Confidence:** High for the mechanism; medium for portable profile outcomes.

### FERRIUM-338: O0 removes optimizer cost but not backend cost

**Sources:** EXP-01 primary, pass, and runtime matrices.

**Observed behavior:** O0 ran no material middle-end transform pipeline but
still spent time in X86 instruction selection and emission. It compiled 59.9%
faster than one-CGU O3 while running 227.2% slower.

**Implication:** A development-speed recommendation must preserve consumer
runtime needs and distinguish "compile only" from executable iteration.

**Confidence:** High for the fixture.

### FERRIUM-339: IR shape determines which passes dominate

**Sources:** EXP-01 function-shape attribution.

**Observed behavior:** generated loop functions dominated named
`OptFunction` time under O1 through O3. Size policy sharply reduced loop-region
work, while scalar-region work changed much less.

**Implication:** A pass report needs Rust owner, loop and control-flow shape,
generic multiplicity, attributes, and target context. Pass name alone is not a
diagnosis.

**Confidence:** High for the fixture; medium for prevalence.

### FERRIUM-340: inliner regions can dominate without identifying one culprit

**Sources:** EXP-01 synthetic and METIS traces.

**Observed behavior:** `ModuleInlinerWrapperPass` was the largest inclusive IR
region. It contained nested devirtualization, inlining, simplification, loop,
and analysis work.

**Implication:** Preserve the event tree and identify expensive child scopes.
Do not label the wrapper itself as a source-level inlining defect.

**Confidence:** High.

### FERRIUM-341: vectorization is both a compile-time and runtime trade

**Sources:** EXP-01, rust issue `#157302`, and rust issue `#102709`.

**Observed behavior:** O2 introduced material loop-vectorizer time; O3 added
SLP time. Upstream has both a version-specific SLP compile-time pathology and
a loop whose vectorized output lost to a scalar size-optimized form.

**Implication:** Report loop and SLP vectorization separately with target
features, exact LLVM revision, runtime, and size. Do not disable either
globally.

**Confidence:** High.

### FERRIUM-342: instruction selection remains material after IR optimization

**Sources:** EXP-01 machine-pass traces.

**Observed behavior:** X86 DAG-to-DAG instruction selection was the largest
named machine pass in every measured optimized mode and remained present at
O0.

**Implication:** A middle-end-only trace can miss the long pole. The ledger
needs machine pass and target ownership.

**Confidence:** High for x86_64 MSVC; medium cross-target.

### FERRIUM-343: coarse LLVM region timing is not pass work

**Sources:** EXP-01 `LLVM_passes` and trace comparison.

**Observed behavior:** the rustc `LLVM_passes` region did not equal the sum of
nested trace events. Under multi-CGU and LTO configurations, parallel and
multi-stage work made the mismatch especially large.

**Implication:** Keep region wall time, summed event work, and backend makespan
as separate metrics.

**Confidence:** High for the pinned toolchain.

### FERRIUM-344: pass diagnostics have substantial observer effect

**Sources:** EXP-01 primary-versus-diagnostic calibration.

**Observed behavior:** the combined LLVM trace and time-pass mode changed
synthetic wall time by 12% to 43% and emitted trace files up to 43.5 MiB; one
public trace reached 99.9 MiB.

**Implication:** Benchmark minimally instrumented runs. Collect pass traces
separately and publish calibration, trace bytes, and limitations.

**Confidence:** High.

### FERRIUM-345: LLVM 23 size modes use O2 plus function policy

**Sources:** pinned `PassWrapper.cpp`, LLVM PR `#191363`, and METIS pass sets.

**Observed behavior:** rustc mapped Os and Oz to LLVM O2. O2 and Os exposed the
same named pass classes in METIS, but size attributes changed pass work and
output.

**Implication:** Record requested label, actual pipeline level, and
`optsize`/`minsize` policy separately. Toolchain upgrades can change the
meaning of a stable Cargo profile label.

**Confidence:** High.

### FERRIUM-346: intermediate archive size is not final size

**Sources:** EXP-01 synthetic executable and METIS Rlib controls.

**Observed behavior:** Os and Oz reduced synthetic executable bytes by about
20% but increased the METIS Rlib archive relative to O2 and O3.

**Implication:** Preserve object, archive, selected-link-input, debug, and final
binary bytes separately. Never promote an archive-only size claim.

**Confidence:** High.

### FERRIUM-347: backend parallelism exchanges wall time for resources

**Sources:** EXP-01 CGU controls and PERF-Q25.

**Observed behavior:** 16 automatic CGUs shortened O3 wall time 17.2% while
consuming 27.1% more CPU. Disabling local ThinLTO shortened wall further in the
fixture but increased peak RSS.

**Implication:** Join pass cost to CGU topology, backend makespan, CPU, memory,
and concurrent crate pressure. A pass sum cannot choose partition count.

**Confidence:** High for the fixture; medium for portable resource ratios.

### FERRIUM-348: LTO multiplied compiler work without a measured runtime win

**Sources:** EXP-01 ThinLTO, fat LTO, runtime, and size controls.

**Observed behavior:** explicit ThinLTO added 47.2% wall time and 283.1% CPU;
fat LTO added 131.1% wall time. Both expanded pass and machine-code regions.
Optimized runtime distributions still overlapped.

**Implication:** LTO requires an application-specific release benchmark.
Compile cost, imports, executable size, emitted bytes, and runtime remain
separate.

**Confidence:** High for the fixture; low on general runtime benefit.

### FERRIUM-349: debuginfo changes optimization-adjacent and emission cost

**Sources:** EXP-01 debuginfo control.

**Observed behavior:** full debuginfo added 19.7% wall time, 17.0% peak RSS,
and 17.9% emitted bytes, while several pass and machine regions also grew.

**Implication:** PERF-Q28 must measure debug records and object emission
directly. PERF-Q26 records the coupling but does not select debuginfo policy.

**Confidence:** High for the fixture.

### FERRIUM-350: pass timings are nested and cannot be flattened safely

**Sources:** LLVM trace hierarchy and EXP-01 parser.

**Observed behavior:** wrapper, adaptor, module, function, loop, analysis, and
transformation events overlapped.

**Implication:** Store parent-child event scope and inclusive duration.
Summed child time is diagnostic work, not automatically CPU or wall time.

**Confidence:** High.

### FERRIUM-351: existing tools are complementary, not interchangeable

**Sources:** rustc developer guide, measureme, cargo-llvm-lines, LLVM tools,
and rustc-perf.

**Observed behavior:** each tool answered a different layer: crate scheduling,
compiler queries, pass timelines, IR volume, native implementation hotspots,
or before-and-after regression.

**Implication:** FERRIUM should join evidence references and causal identities,
not replace Cargo, measureme, LLVM tools, or rustc-perf.

**Confidence:** High.

### FERRIUM-352: the immediate opportunity is a read-only LLVM cost ledger

**Sources:** findings 336 through 351 and role review.

**Observed behavior:** no stable surface joined Rust shape, generic instance,
CGU, optimization stage, pass hierarchy, machine passes, observer effect,
runtime, size, memory, and link controls.

**Implication:** Add an exact-version LLVM overlay to the compiler query plan
and labeled Build Forest. Use it to explain measured alternatives and generate
minimized upstream cases.

**Confidence:** High for the observability gap; medium for product adoption.

### FERRIUM-353: automatic pass and profile changes remain gated

**Sources:** EXP-01 mixed outcomes, upstream vectorization cases, and role
review.

**Observed behavior:** different rows optimized compile wall, CPU, memory,
intermediate bytes, final bytes, or runtime. No configuration won every axis.

**Implication:** Do not rewrite profiles or inject LLVM arguments. Require
held-out repositories, targets, runtime workloads, rollback, and human
approval before advisory guidance.

**Confidence:** High.

## Recommendations

### Adopt now

- Separate IR translation, pre-link optimization, LTO/import optimization,
  machine passes, emission, and linking in every backend report.
- Record requested rustc optimization label, LLVM pipeline level, function
  size policy, LTO stage, target features, backend, CGUs, and debuginfo.
- Preserve pass event hierarchy, IR scope, invocation count, inclusive
  duration, summed diagnostic work, and observer-effect calibration.
- Join expensive function events to Rust owner, mono-item family, inline-copy
  placement, CGU, and generated or imported origin where the exact adapter
  exposes them.
- Pair every profile comparison with CPU, peak RSS, object/archive/final
  bytes, behavior checks, and representative runtime.
- Contribute minimized LLVM IR and Rust fixtures when a pass pathology survives
  exact-version and target controls.

### Prototype behind a compatibility boundary

- An exact-nightly adapter for LLVM time traces and self-profile LLVM events.
- A read-only LLVM cost ledger and compiler query-plan overlay.
- A Build Forest comparison of immutable roots built under supported
  alternative profiles in isolated target directories.
- A source/MIR/mono-item-to-LLVM-scope correlation experiment.
- A pass-pathology detector that produces evidence and a minimized candidate,
  not an automatic flag change.

### Reject or defer

- A universal development or release optimization level.
- Automatic `RUSTFLAGS`, Cargo profile, LTO, vectorization, unrolling,
  inlining, target-feature, debuginfo, or CGU changes.
- Disabling SLP or loop vectorization from historical issue evidence.
- Adding or reordering LLVM passes from FERRIUM.
- Source rewrites designed only to manipulate optimizer shape.
- A compiler fork, custom optimizer, backend, linker, or package manager.
- Treating trace duration, IR lines, archive bytes, or one runtime microbenchmark
  as a complete optimization result.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because the ledger changes no Rust safety rule; future profile or target-feature advice must retain behavior, panic, overflow, ABI, concurrency, and unsafe-boundary tests and cannot treat optimized compilation as proof. |
| Compiler Performance Engineer | Accepted with primary and diagnostic timing separated, event nesting preserved, CPU and memory retained, runtime and size controls visible, and synthetic results barred from selecting a universal profile. |
| Interop Boundary Auditor | Accepted provisionally because target features, calling convention, native objects, exception handling, allocation, linker, and mixed-language LTO remain explicit unmeasured controls before adopter guidance. |
| AI Assurance Skeptic | Accepted because exact commands, revision, pass hierarchy, raw variance, observer effect, negative runtime results, and historical fixed regressions remain visible; the model cannot inject flags automatically. |
| Ecosystem Strategist | Accepted because FERRIUM joins Cargo, measureme, LLVM, cargo-llvm-lines, and rustc-perf evidence and prioritizes minimized upstream cases rather than duplicating those tools or forking the compiler. |
| Rust Maintainer | Accepted because the proposed output explains an existing build without source or manifest churn, preserves ordinary Cargo, and can be disabled by removing the optional diagnostic. |
| Native Platform Adopter | Accepted provisionally for local read-only use; Windows and Linux targets, enterprise linkers, CI memory, support cost, rollback, and audit evidence remain required before operational guidance. |
| Scope Keeper | Accepted because PERF-Q26 owns optimization and machine-pass attribution; development backends, debug emission, linking, remote reuse, and function caching remain in PERF-Q27 through PERF-Q31. |
| Validation Checker | Accepted because the synthetic and public fixtures, commands, repetitions, MAD, checksums, runtime, size, observer calibration, failures, and limitations are recorded; cross-platform and broader Tier 1 evidence remain open gates. |

No role authorizes implementation or automatic compiler changes.

## Product implication

The compiler query plan should add an LLVM backend region:

```text
mono items and inline copies
  -> actual CGU
  -> LLVM IR translation
  -> pre-link optimization stage
       -> module / SCC / function / loop event tree
       -> analyses and transforms
  -> optional ThinLTO import or fat-LTO merge
  -> post-link optimization event tree
  -> instruction selection
  -> machine scheduling and register allocation
  -> object / bitcode / debug emission
  -> linker
  -> final behavior, runtime, and size controls
```

Each event records the exact rustc and LLVM revisions, target, requested
profile, actual pipeline level, size attributes, CGU, thread, IR scope,
duration semantics, trace mode, and evidence reference.

The Build Forest may compare immutable roots produced by supported alternative
profiles, but it does not mutate a manifest, restore LLVM output, or claim one
root is universally better.

## Prototype gate

No LLVM cost implementation or profile advisor may begin until:

1. at least three Tier 1 repositories reproduce the ledger;
2. loop, branch, generic, inline, target-feature, and LTO held-out shapes are
   evaluated;
3. Windows and Linux evidence exists, with macOS or a documented deferral;
4. x86_64 plus one non-x86 target is measured;
5. runtime, final size, object/debug bytes, memory, and link controls are
   consumer-representative;
6. primary-versus-diagnostic observer effect is calibrated for every adapter;
7. the trace and self-profile adapters fail closed on schema or pass-name
   drift;
8. any proposed flag guidance has explicit rollback and maintainer approval;
9. any compiler-facing claim has a minimized fixture and rustc-perf plan;
10. PERF-Q27, PERF-Q28, and PERF-Q29 retain backend, emission, and linker
    ownership.

The implementation gate remains closed.
