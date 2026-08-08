# Rust Editor and Cargo Contention

Date: 2026-08-08
Question: PERF-Q07
Status: Complete
Decision: define an edit-loop topology diagnostic that separates
rust-analyzer semantic analysis, Cargo build data, flycheck, lock classes,
artifact compatibility, foreground priority, and isolated-target duplication.

## Executive conclusion

The Rust editor loop is not one compiler invocation. It contains at least:

1. rust-analyzer's own incremental semantic database;
2. Cargo metadata and rustc probes used to construct the crate graph;
3. a selective Cargo check that builds and runs build scripts and compiles
   procedural macros;
4. check-on-save flycheck for rustc diagnostics;
5. developer-initiated check, build, test, run, or debugger commands.

Those layers overlap in source and dependencies but do not produce one
interchangeable cache.

Cargo 1.95 materially changes the old contention model. In the controlled
fixture, two simultaneous identical checks sharing one target completed in
about the same time as one check. One process compiled all units while the
other waited on the build directory and then reported fresh. A separate target
removed that wait but compiled and stored the whole check twice.

The harder case is check versus build. With a shared target, the build waited
on the artifact directory, reused one compile-time unit, and the pair completed
in a median 8.172 seconds. With separate targets, they completed concurrently
in 5.641 seconds but compiled seventeen rustc units and retained 13.7 MB rather
than sixteen units and 10.7 MB.

There is no universal fastest target-directory setting:

- sharing minimizes successful duplicate work and disk;
- isolation prioritizes foreground overlap at the cost of duplicate work,
  storage, memory pressure, and CPU competition;
- even isolated targets still touch Cargo's global package-cache lock.

rust-analyzer also performs substantial work outside Cargo's target directory.
On the public diagnostics, its internal analysis reached about 1.3 GB for
METIS-CORE and 933 MB for RUNE after inspecting 1.95 million and 1.56 million
dependency lines respectively. Changing `cargo.targetDir` cannot address that
memory or semantic-analysis cost.

FERRIUM should not replace rust-analyzer, Cargo, or flycheck. Its opportunity is
a read-only edit-loop diagnostic that explains which engine is active, which
lock is blocking which user action, what work is compatible, what a separate
target duplicates, and which latency objective the repository is choosing.

No upstream activity was created.

## Decision supported

This research determines:

- whether editor and terminal work is actually duplicated or merely adjacent;
- when current Cargo locking coalesces work;
- when target isolation improves foreground latency;
- which correctness information is lost by disabling build scripts, proc
  macros, or check-on-save;
- the bounded diagnostic and contribution opportunities for FERRIUM.

It does not authorize editor configuration changes, a custom language server,
a Cargo wrapper, or automatic command cancellation.

## Evidence reviewed

### Local evidence

- `docs/research/2026-08-07-rustc-compiler-performance.md`
- `docs/research/2026-08-07-rust-incremental-reuse-boundaries.md`
- `docs/research/2026-08-08-cargo-build-unit-multiplication.md`
- `docs/research/2026-08-08-cross-workspace-artifact-reuse.md`
- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/research/perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md`

### External evidence

- [2025 Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)
- [rust-analyzer configuration](https://rust-analyzer.github.io/book/configuration.html)
- [rust-analyzer build-lock FAQ](https://rust-analyzer.github.io/book/faq.html#rust-analyzer-and-cargo-compete-over-the-build-lock)
- [rust-analyzer build-dependency source at `478b893`](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/project-model/src/build_dependencies.rs)
- [rust-analyzer flycheck source at `478b893`](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/rust-analyzer/src/flycheck.rs)
- [rust-analyzer project loading source at `478b893`](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/rust-analyzer/src/reload.rs)
- [Cargo granular-locking tracker #4282](https://github.com/rust-lang/cargo/issues/4282)
- [Cargo build-unit locking PR #16155](https://github.com/rust-lang/cargo/pull/16155)
- [Cargo check artifact-lock PR #16307](https://github.com/rust-lang/cargo/pull/16307)
- [Historical rust-analyzer contention issue #4616](https://github.com/rust-lang/rust-analyzer/issues/4616)
- [Open custom-target stall report #22891](https://github.com/rust-lang/rust-analyzer/issues/22891)

The rust-analyzer source revision was
`478b8936bb221e84718ba2aa90906c3b32dfd3c8`. The Cargo tracker was reviewed
against master `b99aa3ee16b96480b74b83cf866218e720257317`.

## Edit-loop model

| Layer | Owner | Reusable state | Main latency risks |
| --- | --- | --- | --- |
| Project discovery | rust-analyzer and Cargo | metadata, sysroot, crate graph | process startup, filesystem, manifest reload |
| Build data | rust-analyzer through Cargo | build-script output and proc-macro dylibs | compile-time dependencies, execution, target locks |
| Semantic database | rust-analyzer | syntax, name, type, inference, IDE queries | dependency source volume, invalidation, memory |
| Flycheck | rust-analyzer through Cargo/rustc | Cargo check artifacts and rustc diagnostics | command scope, restart, target lock, incompatible mode |
| Foreground command | Cargo/rustc/linker/test runner | command-specific artifacts and results | lock priority, codegen, linking, tests |
| Global acquisition | Cargo | registry, Git, package cache | package-cache lock and network |

Every diagnosis must identify the layer. “rust-analyzer is compiling” is too
coarse to distinguish build-data compilation, flycheck, or internal semantic
analysis.

## Findings

### FERRIUM-88: the editor loop uses multiple analysis engines and caches

**Sources**

- [rust-analyzer build-dependency source](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/project-model/src/build_dependencies.rs)
- [rust-analyzer project loading source](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/rust-analyzer/src/reload.rs)
- [rust-analyzer flycheck source](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/rust-analyzer/src/flycheck.rs)

**Observed constraint**

rust-analyzer constructs and incrementally updates its own semantic database.
It separately invokes Cargo for metadata, build scripts and procedural macros,
and flycheck diagnostics. Developer Cargo commands are another process and
artifact graph.

The semantic database cannot be installed into Cargo's target directory, and
Cargo artifacts cannot replace rust-analyzer's IDE queries.

**Implication**

FERRIUM must report duplicate purpose separately from duplicate artifact.
Two tools reading the same source is not enough to declare one redundant.

**Confidence:** high.

### FERRIUM-89: build-data Cargo is selective but correctness-critical

**Sources**

- [rust-analyzer build-dependency source](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/project-model/src/build_dependencies.rs)
- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

**Observed behavior**

rust-analyzer ran:

```text
cargo check --quiet --workspace --message-format=json \
  --keep-going --compile-time-deps --all-targets
```

With rust-analyzer as `RUSTC_WRAPPER`, ordinary crate checking was skipped
while build scripts ran and procedural macros were compiled.

Disabling this work removed target artifacts and reduced the synthetic median
from 11.157 to 8.216 seconds, but generated `OUT_DIR` and proc-macro
diagnostics and unknown types.

**Implication**

Build-data cost is a separate visible component, not removable duplicate
correctness work. FERRIUM may identify expensive or imprecisely invalidated
build dependencies, but must not recommend disabling them generally.

**Confidence:** high.

### FERRIUM-90: current shared-target checks can coalesce instead of duplicating

**Sources**

- [Cargo granular-locking tracker #4282](https://github.com/rust-lang/cargo/issues/4282)
- [Cargo build-unit locking PR #16155](https://github.com/rust-lang/cargo/pull/16155)
- [Cargo check artifact-lock PR #16307](https://github.com/rust-lang/cargo/pull/16307)
- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

**Observed behavior**

Cargo now uses shared build-directory and per-build-unit locking for supported
check work. Two identical cold checks in one target had a 5.374-second median
versus 5.348 seconds for one. One process completed all eleven rustc
invocations; the waiter performed only the compiler capability probe.

The waiter explicitly reported the build-directory lock. The first launched
process was not always the producer.

Cargo's open tracker still records an unresolved dirty-shared-unit case that
can block and rebuild, so the controlled success is not a universal guarantee.

**Implication**

Do not diagnose every lock wait as wasted compile time. Record wait owner,
producer, waiter, completed units, and whether the waiter became fresh.

**Confidence:** high for Cargo 1.95 and the fixture; medium for other filesystems
and command shapes.

### FERRIUM-91: target isolation exchanges waiting for duplicate work

**Sources**

- [rust-analyzer target-directory configuration](https://rust-analyzer.github.io/book/configuration.html#cargo.targetDir)
- [rust-analyzer build-lock FAQ](https://rust-analyzer.github.io/book/faq.html#rust-analyzer-and-cargo-compete-over-the-build-lock)
- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

**Observed behavior**

Two checks in separate targets completed without the build-directory wait, but
each executed eleven rustc processes and retained a full target. Combined bytes
were twice the shared-target result.

Both still briefly waited on Cargo's global package-cache lock.

**Implication**

`cargo.targetDir` is a foreground-latency policy, not a free performance
optimization. Recommendations require target bytes, successful rustc work,
machine headroom, disk policy, and the user's foreground priority.

**Confidence:** high for the controlled fixture.

### FERRIUM-92: check and build expose the real latency-versus-reuse trade

**Sources**

- [2025 performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#type-checking-and-ide-performance)
- [Historical rust-analyzer issue #4616](https://github.com/rust-lang/rust-analyzer/issues/4616)
- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

**Observed behavior**

A shared-target check followed by build completed in a median 8.172 seconds.
The build waited on the artifact directory and reused one compatible
compile-time unit.

Separate targets completed in 5.641 seconds but compiled all check and build
units and retained about 28% more bytes.

This is consistent with the survey's broader finding that check and build do
not share all artifacts and with the long-standing rust-analyzer workaround.

**Implication**

The configuration choice depends on whether the desired outcome is minimum
foreground build latency, minimum total work, minimum disk, or lower machine
contention. FERRIUM should present the Pareto trade rather than one default.

**Confidence:** high for topology; absolute timing is exploratory.

### FERRIUM-93: flycheck restarts prioritize current diagnostics

**Sources**

- [rust-analyzer flycheck source](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/rust-analyzer/src/flycheck.rs)
- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

**Observed behavior**

rust-analyzer gives restart requests priority, cancels the previous process,
and debounces restarts for 50 ms. The LSP trace observed two flycheck spawns
around startup and save but only one cold set of eleven completed rustc
processes.

**Implication**

Restart count is not duplicate-work count. Diagnostics should record cancelled
processes, completed artifacts before cancellation, generation, save trigger,
and eventual ready time.

**Confidence:** high for source behavior and the trace.

### FERRIUM-94: target-directory tuning cannot reduce rust-analyzer's semantic
database

**Sources**

- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)
- [2025 performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#type-checking-and-ide-performance)

**Observed behavior**

The batch diagnostics reported approximately:

- 561 MB after analyzing the synthetic fixture and standard-library sources;
- 1.3 GB for METIS-CORE;
- 933 MB for RUNE.

Build-data-disabled runs wrote no Cargo target artifacts but retained most
semantic analysis.

**Implication**

Editor memory, cache priming, dependency source, proc-macro expansion, and
semantic invalidation need their own evidence. Cargo target isolation only
addresses subprocess artifacts and locks.

**Confidence:** high for the diagnostic runs; memory figures are
rust-analyzer's summaries rather than operating-system working sets.

### FERRIUM-95: editor latency telemetry has an attribution gap

**Source**

- [Experiment](perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

**Observed behavior**

METIS-CORE's single cold diagnostic took 80.620 seconds. The summary named
1.99 seconds of metadata, 21.38 seconds of Cargo build data, and 13.92 seconds
of later analysis, leaving a large project-loading interval without a durable
component label.

**Implication**

FERRIUM needs a lifecycle timeline joining:

- project discovery and reload cause;
- metadata and toolchain probes;
- build-data command;
- proc-macro loading;
- VFS loading;
- cache priming and semantic readiness;
- flycheck start, cancellation, completion, and diagnostics;
- child-process and Cargo lock intervals.

Unknown time remains unknown rather than being assigned to Cargo, rustc, or
rust-analyzer.

**Confidence:** high that the gap exists; low on its cause.

### FERRIUM-96: configuration axes can change both coverage and contention

**Sources**

- [rust-analyzer configuration](https://rust-analyzer.github.io/book/configuration.html)
- [`config.rs` at `478b893`](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/rust-analyzer/src/config.rs)

**Observed constraint**

Defaults enable cache priming, all targets, build scripts, proc macros,
workspace check-on-save, and build-data rebuild-on-save. Users may change
target directory, check command, workspace scope, target set, features,
environment, and override commands.

These settings change diagnostic coverage, artifact identity, target size,
process overlap, and lock behavior.

**Implication**

An edit-loop report must capture effective rust-analyzer configuration and
must not compare two sessions as equivalent when coverage differs.

**Confidence:** high.

### FERRIUM-97: FERRIUM's opportunity is an edit-loop topology diagnostic

**Sources**

- Findings FERRIUM-88 through FERRIUM-96
- [Cargo tracker #4282](https://github.com/rust-lang/cargo/issues/4282)
- [rust-analyzer FAQ](https://rust-analyzer.github.io/book/faq.html#rust-analyzer-and-cargo-compete-over-the-build-lock)

**Observed constraint**

Cargo owns artifact compatibility and locks. rust-analyzer owns its semantic
database, build-data loading, flycheck lifecycle, and editor integration.
Existing configuration already supports shared and isolated targets.

The missing surface is a joined explanation of:

- engine and purpose;
- process and command;
- lock class, owner, waiter, and wait duration;
- compatible work reused after waiting;
- duplicate rustc units and target bytes under isolation;
- diagnostic-ready and foreground-command completion;
- correctness and coverage lost by narrowing the editor workload.

**Implication**

FERRIUM should diagnose and compare existing configurations. It should not
create another analyzer, compiler daemon, or target cache.

**Confidence:** high for the boundary; consumer demand still requires a wider
editor and repository census.

## Recommendations

### Adopt now

- Record the effective rust-analyzer Cargo, check, target, feature, proc-macro,
  build-script, cache-priming, and workspace settings.
- Separate project discovery, build data, semantic readiness, flycheck, and
  foreground command completion.
- Classify package-cache, build-directory, build-unit, artifact-directory, and
  unknown waits separately.
- Treat a waiting command that becomes fresh differently from duplicate
  compilation.
- Report both foreground latency and total successful work.
- Keep build scripts and procedural macros enabled unless a repository has an
  explicit correctness-compatible alternative.

### Prototype behind a compatibility boundary

- A read-only rust-analyzer log and Cargo process timeline parser.
- A shared-versus-isolated target what-if report using observed command mix,
  target bytes, lock waits, and machine headroom.
- Flycheck generation and cancellation accounting.
- Project-load phase instrumentation that preserves unattributed intervals.
- A portable fixture suite for identical check, check/build, proc-macro, and
  build-script overlap.

The implementation gate remains closed.

### Reject or defer

- Reject disabling check-on-save, build scripts, proc macros, all targets, or
  workspace coverage as an automatic optimization.
- Reject a universal separate-target recommendation.
- Reject lock-message count as a latency or duplicate-work metric.
- Reject treating rust-analyzer memory as Cargo target-cache size.
- Defer automatic foreground-priority cancellation to rust-analyzer and Cargo
  owners.
- Defer compiler-daemon or shared semantic-cache design to later research.
- Defer the open custom-target stall report as an optimization blocker until
  its root cause and affected versions are confirmed.
- Defer upstream activity until explicit owner approval.

## Potential contribution paths

Without creating upstream activity, Q07 identifies:

1. Cargo fixtures for identical-check coalescing and check/build lock behavior;
2. rust-analyzer lifecycle events for build data, proc-macro loading, VFS,
   cache priming, flycheck generations, and diagnostic readiness;
3. lock-owner and wait-duration evidence that distinguishes productive waiting
   from duplicate compilation;
4. documentation connecting `cargo.targetDir` to foreground latency, total
   work, disk, and machine-pressure trade-offs;
5. minimized cases when a current Cargo or rust-analyzer revision violates the
   observed topology.

Any contribution first requires an owner-approved, current-version
reproduction and project-specific contribution policy review.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: build scripts, proc macros, rustc diagnostics, and validation coverage are correctness inputs rather than removable overhead. |
| Compiler Performance Engineer | Accepted: foreground latency, total rustc work, target bytes, lock waits, internal semantic work, warm-up, variance, and unknown time remain separate. |
| Interop Boundary Auditor | Accepted: build scripts, native tools, targets, environment, and procedural execution remain explicit boundaries. |
| AI Assurance Skeptic | Accepted: the cold outlier, unstable build baseline, unclassified METIS interval, unresolved Cargo edge case, and unconfirmed rust-analyzer issue remain visible. |
| Ecosystem Strategist | Accepted: Cargo and rust-analyzer remain the implementation owners; FERRIUM supplies joined diagnosis and fixtures. |
| Rust Maintainer | Accepted: ordinary editor and Cargo behavior remains valid, recommendations are reversible, and no repository configuration is rewritten. |
| Native Platform Adopter | Accepted: disk, memory, machine headroom, target isolation, rollback, Windows behavior, and missing cross-platform evidence are explicit. |
| Scope Keeper | Accepted: Q07 covers editor/Cargo orchestration; compiler query sharing, validation impact, and remote artifacts remain later questions. |
| Validation Checker | Accepted: source revisions, toolchains, commands, synthetic controls, public fixtures, three-sample topology results, negative controls, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q07 is complete.

FERRIUM should model the entire edit-to-diagnostic and edit-to-foreground
command loop, but only as a read-only topology and evidence surface. The next
question is PERF-Q08: isolate rustc startup, metadata loading, and fixed
per-invocation cost before studying deeper frontend phases.
