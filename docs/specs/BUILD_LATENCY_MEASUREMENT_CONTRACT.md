# Build Latency Measurement Contract

Status: Draft research specification

Version: 0.1

Owner: FERRIUM Hammer lane

## Decision supported

This specification determines whether FERRIUM should advance from research into
a bounded build-causality prototype, and which Rust build shapes that prototype
must explain.

It does not authorize implementation. Measurements and the Pulse 02
cross-lane selection gate must be completed first.

## Goals

1. Produce comparable evidence for representative Rust iteration workflows.
2. Separate compiler, Cargo graph, macro, build-script, codegen, debug, and link
   costs where the available evidence permits.
3. Observe which packages rebuild after controlled edits.
4. Test whether a useful causal explanation can be produced from stable Cargo
   surfaces before using compiler-internal instrumentation.
5. Select fixtures by build shape rather than by repository popularity.

## Governing principles

This contract applies
[FP-01 through FP-12](../governance/ENGINEERING_PRINCIPLES.md). In particular:

- representative workflows outrank convenient microbenchmarks;
- causality must precede recommendations;
- caches are correctness systems;
- failures and inconclusive results remain visible;
- experiments require explicit stop conditions; and
- private repositories must not leak source, names, paths, dependencies, or
  identifiable timing records into public artifacts.

## Corpus model

FERRIUM uses three fixture tiers.

### Tier 0: Synthetic controls

Small purpose-built fixtures isolate one variable at a time. They may model a
single crate, a dependency diamond, a procedural macro, a build script, a
generic-heavy library, or a link-heavy binary.

Synthetic controls establish whether the measurement method can detect a known
cause. They cannot establish real-world impact by themselves.

### Tier 1: Public portfolio fixtures

Public repositories provide realistic graphs while allowing commands, source
revisions, and raw results to be published. Initial discovery candidates are:

| Candidate | Observed build shape | Candidate use |
|---|---|---|
| [METIS-CORE](https://github.com/giodl73-repo/METIS-CORE) | Single-crate pure Rust algorithm library with property tests and benchmarks | Small baseline and generic/optimization-sensitive control |
| [PARLOR](https://github.com/giodl73-repo/PARLOR) | Seven-manifest workspace with a shared core, multiple game kernels, and a CLI | Medium fan-out and sibling-crate edits |
| [RUNE](https://github.com/giodl73-repo/RUNE) | Seven-manifest workspace with a procedural-macro crate and `trybuild` tests | Procedural-macro and compile-test workload |
| [RLINE](https://github.com/giodl73-repo/RLINE) | Twelve-manifest graph, statistics, math, history, optimization, and CLI workspace | Shared-library fan-out and layered graph workload |
| [ICELINES](https://github.com/giodl73-repo/ICELINES) | Seven-manifest async application workspace with web, CLI, SQLite, and a build script | Dependency-heavy application, native dependency, and link workload |
| [BISECT](https://github.com/giodl73-repo/BISECT) | Forty-one tracked manifests spanning libraries, CLIs, web/TUI surfaces, algorithms, and a Python binding | Deep workspace, broad fan-out, build-script, and mixed-target workload |

These observations are corpus-discovery evidence, not frozen benchmark
revisions. Each selected fixture must later record its complete commit SHA,
manifest path, package selection, features, lockfile hash, and licensing status.

### Tier 2: Authorized private fixtures

Owner-nominated private or enterprise repositories may be measured when they
represent build shapes absent from the public corpus.

Private fixtures have stricter rules:

1. Raw source, repository names, remotes, paths, manifests, dependency names,
   logs, and per-package timing data remain private.
2. The public corpus record uses an anonymous fixture ID and a coarse build-shape
   classification.
3. Only aggregate, disclosure-reviewed results may leave the authorized
   environment.
4. Public conclusions must also be reproducible on a Tier 0 or Tier 1 fixture.
5. Private evidence may strengthen confidence but cannot be the sole basis for
   an open FERRIUM product claim.

## Required fixture classes

The first census must cover all of these classes, although one repository may
cover more than one:

| Class | Required characteristic |
|---|---|
| Small baseline | One crate, ordinary dependencies, short iteration cycle |
| Medium workspace | Multiple sibling crates with a clear shared core |
| Deep workspace | Large package graph with several target types |
| Generic-heavy | Meaningful monomorphization or optimization sensitivity |
| Procedural-macro-heavy | At least one proc-macro crate and downstream users |
| Build-script-sensitive | At least one `build.rs` with declared rerun inputs |
| Dependency-heavy application | Async, web, database, native, or similarly broad dependencies |
| Link-heavy | One or more substantial binaries or mixed target types |
| Mixed topology | Workspace members plus intentionally isolated packages or build contexts |

The initial census is incomplete until every class has either a selected fixture
or a documented reason for deferral.

## Workload matrix

Every selected fixture runs the applicable workflows below.

| Workload | Purpose |
|---|---|
| `cargo check` | Frontend-oriented development feedback |
| `cargo build` | Development codegen and link path |
| `cargo test --no-run` | Test-target compilation without test execution noise |
| Selected test command | End-to-end developer verification loop |
| Release build | Optimized codegen and linking, only where consumer-relevant |

Each workflow is measured in these cache states:

1. **Project-cold:** a new empty `CARGO_TARGET_DIR`; global registry and Git
   caches are not deleted.
2. **Warm no-op:** repeat the identical command with the same target directory
   and unchanged source.
3. **Warm incremental edit:** apply one controlled edit and reuse the target
   directory.
4. **Warm revert:** revert the controlled edit and repeat, recording whether
   work is reused or repeated.

Dependency download time is a separate acquisition workload and must not be
mixed into compiler latency claims.

## Controlled edit scenarios

Each fixture must support a documented subset of:

| ID | Scenario | Intended observation |
|---|---|---|
| ES-01 | Private function-body change | Local recompilation and downstream relink behavior |
| ES-02 | Public non-generic API change | Cross-crate metadata invalidation and fan-out |
| ES-03 | Generic function-body or bound change | Downstream monomorphization and optimization impact |
| ES-04 | `#[inline]` body change | Cross-crate codegen invalidation |
| ES-05 | Feature-set change | Graph, dependency, and artifact identity changes |
| ES-06 | Procedural-macro input change | Macro execution and downstream invalidation |
| ES-07 | Build-script declared input change | Build-script rerun and dependent work |
| ES-08 | Build-script unrelated-file change | Correct negative behavior: no rerun when inputs are precise |
| ES-09 | Test-only change | Separation of test-target and product-target work |
| ES-10 | Binary-only change | Link-path cost without library API invalidation |

Edits must be deterministic, behavior-preserving where stated, and applied only
inside a disposable worktree or fixture copy. Active owner branches are never
mutated for benchmark execution.

## Environment record

Every run records:

- operating system and version;
- CPU model, logical core count, and memory;
- filesystem type and whether the workspace is local, virtualized, or remote;
- power mode when it can materially affect results;
- `rustc -Vv`;
- `cargo -V`;
- active default linker and relevant linker flags;
- repository commit SHA and dirty state;
- root manifest and selected package or workspace;
- `Cargo.lock` hash;
- enabled features, target triple, profile, and environment overrides;
- `CARGO_TARGET_DIR` identity and cache-state label; and
- whether antivirus, indexing, or another known source of interference was
  observed.

Unknown values are recorded as unknown rather than inferred.

## Stable measurement tier

The required tier uses stable Cargo and rustc surfaces:

```powershell
rustc -Vv
cargo -V
cargo metadata --format-version 1
cargo check --timings
cargo build --timings
cargo test --no-run --timings
```

Wall-clock duration, exit status, Cargo JSON messages where requested, and the
Cargo timing artifact are retained together. Commands may add package, feature,
profile, target, or manifest selectors, but the complete command must be
recorded.

The measurement harness must not parse human terminal text when a structured
Cargo output exists.

## Telemetry layers and observer effect

No single telemetry mode is the benchmark.

Use the following layers:

1. repeated minimally instrumented wall-clock runs for the primary latency
   distribution;
2. `cargo metadata --format-version 1` once per fixture configuration for the
   declared package, target, feature, and dependency graph;
3. Cargo JSON messages for representative workload states to record observed
   artifacts, freshness, cached or current build-script output, diagnostics,
   and outcome;
4. separately labelled `cargo --timings` diagnostic runs when unit duration,
   dependency unblocking, concurrency, or frontend/codegen split is needed;
5. separately labelled rustc self-profile runs when query execution, cache
   hits, blocked time, or incremental loading is needed; and
6. rustc-perf-compatible evidence before promoting a compiler-change claim
   intended for upstream Rust.

Do not include instrumentation cost silently in the primary latency claim.
Calibrate every diagnostic mode against an otherwise equivalent minimally
instrumented command on the fixture. Record its sample count, median, MAD, and
known limitations.

Cargo timing reports are diagnostic evidence, not a substitute for repeated
wall-clock samples. Undocumented report internals must not be the sole durable
machine interface.

Cargo emits `build-script-executed` JSON messages for current and cached build
script output. That message alone is not evidence that a script ran. Use timing
or other dirty-unit evidence for an execution claim.

Stable Cargo operation must remain possible without nightly. Nightly
self-profile integration is an optional, versioned compatibility boundary.

If a fixture revision lacks a committed lockfile, generate the lockfile before
measurement, record its cryptographic hash, fetch dependencies, and then run
with `--locked --offline`. A failed lock or acquisition precondition is not a
build-latency sample.

## Optional compiler-detail tier

Nightly rustc self-profile summaries may be collected for selected runs when
stable evidence cannot distinguish relevant causes.

This tier:

- is optional and separately labeled;
- records the exact nightly toolchain;
- summarizes compiler query categories rather than exposing repository source;
- cannot become a runtime dependency of the stable prototype; and
- must not make a public conclusion unreproducible without nightly.

Direct `rustc_private` integration is outside this specification.

## Evidence record

Each run produces a logical record with:

| Field | Meaning |
|---|---|
| `run_id` | Stable identifier for this attempt |
| `fixture_id` | Public repository/revision or anonymous private fixture ID |
| `workload` | Check, build, test-no-run, test, or release |
| `cache_state` | Project-cold, warm-no-op, warm-edit, or warm-revert |
| `edit_scenario` | `ES-01` through `ES-10`, or none |
| `command` | Exact executed command and working manifest |
| `environment` | Environment-record reference |
| `started_at` / `finished_at` | Monotonic measurement bounds plus wall time |
| `exit_status` | Success or preserved failure |
| `observed_packages` | Cargo package/target work observed from structured output |
| `timing_artifacts` | Paths or hashes of retained Cargo timing data |
| `limitations` | Noise, unsupported surfaces, missing evidence, or anomalies |

The schema is logical at this stage. Choosing JSON, JSONL, SQLite, or another
storage format belongs to the later evidence-contract decision.

## Statistical contract

1. Exploratory runs use at least three repetitions and are labeled exploratory.
2. A promoted quantitative claim uses at least five measured repetitions after
   any declared warm-up.
3. Results report every sample, the median, and median absolute deviation.
4. If median absolute deviation divided by the median exceeds 10%, the result is
   unstable and cannot support a promoted optimization claim without an
   explanation and rerun.
5. Failed and interrupted samples remain in the run record but are excluded from
   the successful-duration statistic with the exclusion reason visible.
6. Comparisons use the same environment and fixture revision unless explicitly
   labeled cross-environment.

## Causal explanation target

The census tests whether stable evidence can support explanations in this
vocabulary:

- package or target changed directly;
- downstream metadata invalidation;
- downstream codegen or monomorphization;
- feature or profile identity changed;
- procedural macro reran;
- build script reran;
- dependency artifact unavailable or incompatible;
- codegen repeated;
- relink occurred without upstream recompilation;
- test-only target work occurred;
- cause unknown from available evidence.

`Cause unknown` is a valid and required result. FERRIUM must not guess.

## Identity vocabulary

Every rebuild explanation distinguishes:

1. **Graph unit identity:** package, target, profile, host/target kind, compile
   mode, features, flags, role, and dependency identity inside one invocation.
2. **Artifact identity:** the namespace and output filename Cargo selected for
   a simultaneously reusable result.
3. **Freshness identity:** source, dependency, environment, build-script, and
   configuration evidence that decides whether the existing artifact must be
   rebuilt.
4. **Propagation:** a unit became dirty because a dependency fingerprint or
   build-script output changed.

A source edit may rebuild the same artifact identity. A feature, profile, mode,
target, toolchain, flag, or dependency-identity change may create a separate
artifact identity. Reports must name the changed layer.

Package count and manifest count are not accepted proxies for build work. Use
observed units where available.

Do not use one shared writable `CARGO_TARGET_DIR` for unrelated repositories in
FERRIUM experiments. Cargo's relocation-compatible local path identity can
collide when unrelated workspaces contain equivalent path-package identities.
Cross-workspace reuse requires a later provenance and isolation contract.

## Scheduling vocabulary

When a question analyzes Cargo orchestration, every observed unit may record:

1. **Graph-ready time:** when every dependency of the unit had finished.
2. **Ready-queue delay:** unit start minus graph-ready time.
3. **Unit duration:** unit finish minus unit start in that diagnostic run.
4. **Summed unit work:** the sum of all observed unit durations.
5. **Makespan:** final unit finish minus unit-graph completion.
6. **Average active Cargo jobs:** summed unit work divided by makespan.
7. **Peak active Cargo jobs:** the maximum overlapping observed units.
8. **Observed gating chain:** the dependency and queueing chain that determined
   completion of the requested root in the measured run.
9. **Counterfactual critical path:** a simulated schedule using estimated
   costs; never reported as observed fact.

These values describe Cargo units, not CPU utilization. A unit can be blocked
by graph dependencies, ready but waiting for a Cargo job slot, slowed by
resource contention, or using compiler-internal parallelism. Reports must keep
those causes separate.

Queue delay is not automatically recoverable latency. A proposed schedule must
preserve independent overlap and compare whole-command wall time. Diagnostic
unit durations and nightly build-analysis traces remain separate from the
primary repeated wall-clock distribution.

## Unit multiplication vocabulary

Repeated package or target names are classified by:

1. target kind and target name;
2. compile mode, including test-harness and doctest roles;
3. effective profile fields rather than profile label alone;
4. host or target platform and explicit-target namespace;
5. enabled features and normal, build, or dev dependency role;
6. compiler, rustdoc, Clippy driver, wrapper, and relevant flags;
7. selected validation coverage;
8. observed artifact freshness.

Every variant receives one disposition:

- required by a named semantic, platform, tool, or coverage boundary;
- compatible and observed reused;
- suspicious repeated compilation;
- unknown because the evidence surface omits the differing identity.

Package-version reports are not accepted as complete duplicate-work evidence.
Planned unit count and observed dirty artifact count remain separate because
Cargo JSON, build scripts, rustdoc, doctests, and unstable unit graphs do not
have one-to-one event coverage.

Fewer units are not inherently better. Feature unification, profile changes,
target removal, and validation reduction can change semantics or coverage and
require their own explicit decision and validation gate.

## Cross-workspace reuse vocabulary

Cross-workspace reuse reports distinguish:

1. **Producer:** the Cargo invocation and environment that created an artifact
   set.
2. **Consumer:** the invocation considering that artifact set for reuse.
3. **Candidate unit:** an immutable registry or Git unit whose visible identity
   appears compatible; candidate does not mean proven reusable.
4. **Provenance:** source checksum or revision, Cargo unit identity, dependency
   identities, compiler, target, flags, configuration, and producer context.
5. **Execution cone:** build-script, proc-macro, native-tool, or other
   compile-time execution that can introduce undeclared inputs.
6. **Artifact set:** every compiler output, fingerprint, dep-info record, and
   Cargo sidecar required for one reusable unit.
7. **Integrity:** cryptographic verification that imported bytes match the
   declared entry. Cargo freshness alone is not an integrity check.
8. **Isolation domain:** the repositories and path packages permitted to write
   one namespace.
9. **Publication:** atomic installation of a complete verified entry rather
   than in-place partial mutation.
10. **Retention owner:** the component responsible for access tracking,
    garbage collection, size limits, and poisoned-entry recovery.

One shared writable target or build directory across unrelated repositories is
not an accepted cache topology. It can collapse distinct path-package
provenance, broaden lock contention, and make one workspace's cleanup delete
another's state.

Compiler output files are not assumed to be self-contained cache entries.
Experiments that copy or corrupt artifacts must use disposable isolated
targets, retain failure output, and never touch the global registry or Git
source caches.

Exact package-name overlap is not an accepted hit-rate estimate. Reports must
compare version, target, mode, effective profile, platform, features, flags,
toolchain, dependency identities, and excluded execution cones. Restore,
verification, locking, and cleanup costs remain separate from compilation
saved.

Remote producer trust, signing, revocation, transport, and cross-platform
compatibility remain PERF-Q30 scope.

## CI cache vocabulary

CI cache evidence distinguishes:

1. **Cargo compatibility:** whether restored state is fresh for the consumer
   unit.
2. **Transport key:** the user-visible key requested from the cache service.
3. **Cache version:** service metadata derived from paths, compression, or
   action implementation.
4. **Match class:** exact, fallback, default-branch fallback, or absent.
5. **Producer job:** the trusted job that assembled and published the payload.
6. **Consumer job:** the job that restored and validated the payload.
7. **Payload:** the registry, Git, target, profile, tool, and workspace state
   included in the archive.
8. **Entry lifecycle:** immutable first-writer state, later exact-hit behavior,
   and the cache schema/version used to force replacement.
9. **Scope and trust:** branch, pull-request, tag, event, and writer
   permissions controlling visibility and publication.
10. **Retention:** idle expiry, last-access eviction, quota, and manual cleanup.
11. **Transport cost:** lookup, download, extraction, verification, pack, and
    upload time.
12. **Saved work:** Cargo work avoided after restore, measured with fresh and
    dirty artifact evidence.

`cache-hit=true` is not accepted as a build-reuse result. Every promoted cache
claim records the match class, payload bytes, restore duration, compile after
restore, and observed Cargo freshness.

An immutable key needs an explicit cache schema/version. If command coverage,
cleanup semantics, target, profile, feature, or tool variants change without a
new key or designated producer, an exact hit can preserve an incomplete
payload indefinitely.

Matrix jobs may share a job ID. Target triples, profiles, and features supplied
only as command arguments are not assumed to be present in a cache key. Reports
must identify the single writer, explicit variant key, or central producer that
prevents first-writer ambiguity.

Consumer value is:

```text
cold compile - restore - verification - compile after restore
```

Portfolio value also amortizes producer pack and upload, storage, eviction,
and expected future hit count. A cache with positive compile reuse can still be
net negative.

Same-job command reuse and transported reuse remain separate. A cache cannot
make check, test, Clippy, release, target, or feature artifacts compatible when
Cargo identity differs.

## IDE loop vocabulary

Editor-loop evidence distinguishes:

1. **Project discovery:** workspace manifests, Cargo metadata, sysroot,
   toolchain probes, crate graph, source loading, and reload cause.
2. **Build data:** Cargo and rustc work needed for build-script outputs,
   `OUT_DIR`, generated cfg values, and procedural-macro dynamic libraries.
3. **Analyzer-native work:** parsing, indexing, name resolution, inference,
   cache priming, and IDE queries performed by rust-analyzer rather than rustc.
4. **Flycheck:** the rust-analyzer-managed Cargo or custom command that
   produces compiler diagnostics.
5. **Foreground command:** a developer-initiated check, build, test, run,
   debugger, lint, or validation command.
6. **Diagnostic generation:** the editor state, save or reload trigger, and
   flycheck process associated with one current diagnostic result.
7. **Diagnostic-ready latency:** edit or save until current analyzer and
   compiler diagnostics are available.
8. **Foreground latency:** foreground command request until the requested
   result is available.
9. **Target topology:** shared, editor-isolated, command-isolated, or unknown
   target and build directories.
10. **Lock class:** package cache, build directory, build unit, artifact
    directory, global cache, or unknown.
11. **Producer and waiter:** the process that creates compatible state and the
    process blocked while considering that state.
12. **Productive wait:** a waiter that becomes fresh after another process
    publishes compatible work.
13. **Duplicate work:** separate successful compiler or linker work for an
    equivalent requested unit, not merely a second command or lock message.
14. **Cancellation:** the process generation, trigger, cancellation request,
    completed child work, and eventual replacement result.
15. **Coverage delta:** changed workspace, target, feature, build-script,
    proc-macro, lint, or validation coverage between two configurations.
16. **Resource duplication:** additional compiler processes, target bytes,
    memory, CPU demand, and I/O caused by isolation or overlapping commands.

Reports must capture effective rust-analyzer and Cargo configuration. Shared
and isolated sessions are not equivalent when command, workspace, targets,
features, build scripts, proc macros, environment, or diagnostics differ.

A lock message is not accepted as duplicate-work evidence. Reports identify
the lock class, owner, waiter, duration where available, completed work, and
whether waiting enabled reuse.

Lower diagnostic-ready or foreground latency is not accepted as a system-wide
speedup when it increases successful compiler work, target bytes, resource
competition, or later validation cost. Reports present foreground latency and
total machine work separately.

Disabling check-on-save, build scripts, proc macros, workspace coverage,
targets, features, or validation requires a named correctness or coverage
disposition. Missing `OUT_DIR`, unavailable macro expansion, and absent
diagnostics remain failures or coverage changes rather than speedups.

Target-directory isolation does not isolate Cargo's global package cache or
rust-analyzer's in-memory semantic database. Every recommendation states which
layer it can affect.

Unknown editor-loading intervals remain unknown. They are not assigned to
Cargo, rustc, proc macros, filesystem behavior, or rust-analyzer without a
corresponding event, process, trace, or controlled experiment.

## rustc invocation and metadata vocabulary

Direct compiler evidence distinguishes:

1. **Launcher:** Cargo, rustup proxy, direct toolchain executable, wrapper, or
   another process that selected rustc.
2. **Process lower bound:** a non-compiling direct-rustc command used to bound
   process creation, image loading, and early command handling. It is not a
   complete or automatically recoverable startup cost.
3. **Session initialization:** target, sysroot, backend, diagnostics, options,
   source map, query system, and other compiler-session setup not assigned to
   parsing or later work.
4. **Parse boundary:** opening and parsing the crate root, explicitly labeled
   when a nightly stop boundary is used.
5. **Expansion boundary:** macro expansion and the work required to reach the
   selected no-analysis boundary.
6. **Crate location:** filesystem and candidate work used to discover direct
   and transitive metadata.
7. **Metadata registration:** crates admitted to the compiler session,
   separated into sysroot, injected, direct, and transitive origins where
   evidence permits.
8. **Metadata blob availability:** bytes mapped or read so the process can
   validate and access a metadata artifact.
9. **Metadata demand:** lazy tables, entries, exported children, traits,
   implementations, macros, generics, MIR, or other information actually
   decoded for queries.
10. **Dependency-count cost:** location, validation, registration, and session
    work attributable to more crates rather than more metadata bytes.
11. **Emitted metadata:** encoding and writing the current crate's `.rmeta`.
12. **Backend and archive output:** object, bitcode, archive, or other codegen
    work after the selected frontend boundary.
13. **Profiled event time:** CPU or wall evidence represented by the selected
    self-profile event set.
14. **Unclassified invocation time:** external wall time not assigned by the
    available compiler events. It remains unclassified rather than being
    labeled startup, I/O, or operating-system cost.

Every direct-rustc experiment records the resolved executable path, launcher,
toolchain revision, target, sysroot, crate type, emit mode, output paths,
incremental state, profiler event set, working directory, and created
directories.

Manual `rustc` from `PATH` is not assumed to match the executable Cargo passes
to a wrapper. Rustup-proxy overhead is reported separately and is not
multiplied by Cargo unit count without direct process evidence.

Metadata size is not accepted as a latency proxy. Reports pair artifact bytes
with crate count and demand shape, including unused extern, named-item use,
namespace enumeration or glob reexport, and any relevant trait, macro,
generic, or MIR demand.

Self-profile summaries do not replace external wall time. The difference
between external wall time and profiled totals can include process startup,
early compiler work, uninstrumented work, parallel or idle intervals,
profiler overhead and output, and operating-system scheduling. The difference
must remain unclassified unless another event source resolves it.

For direct rustc, omitting `-C incremental` leaves incremental compilation
disabled by default. `-C incremental=off` names a directory called `off`; it
does not disable incremental compilation. Cargo experiments use
`CARGO_INCREMENTAL=0` when they require an explicit override.

Diagnostic flags can change tiny workloads materially. Query-argument
profiling, timing output, profiler serialization, output deletion, and
directory persistence are calibrated in separate runs rather than included
silently in the primary distribution.

## Acceptance gate

The build-causality prototype may be proposed only if the census demonstrates:

1. At least one Tier 0 control and three Tier 1 repositories can be measured
   reproducibly.
2. The selected corpus covers small, medium, deep, macro/build-script, and
   dependency- or link-heavy shapes.
3. At least six controlled edit scenarios have successful evidence records.
4. Stable surfaces correctly distinguish direct work from downstream work on
   the known synthetic controls.
5. At least one real fixture produces a costly or surprising rebuild that can
   be attributed more precisely than total wall time alone.
6. The proposed explanation reduces maintainer investigation effort without
   requiring source rewriting or unstable compiler integration.
7. Privacy review confirms that no Tier 2 identity or sensitive detail appears
   in public output.

If these conditions are not met, FERRIUM improves the measurement contract,
contributes fixtures upstream, or defers the prototype.

## Role review

| Role | Specification disposition |
|---|---|
| Rust Safety Steward | Accepted: the contract forbids hidden correctness trades and preserves explicit unsafe, FFI, macro, build-script, and cache boundaries. |
| Compiler Performance Engineer | Accepted: workloads, cache states, repetitions, variance, and component attribution are separated. |
| Interop Boundary Auditor | Accepted: mixed-language and private fixtures require explicit boundaries, anonymization, and reversible handling. |
| AI Assurance Skeptic | Accepted: unknown causes and failed runs remain visible; AI explanation is not treated as proof. |
| Ecosystem Strategist | Accepted: the census evaluates stable Cargo surfaces and upstream contribution before replacement tooling. |
| Rust Maintainer | Accepted: active branches are not mutated, ordinary Cargo workflows remain valid, and explanations must reduce maintenance burden. |
| Native Platform Adopter | Accepted: private enterprise fixtures are optional, controlled, anonymized, and removable. |
| Scope Keeper | Accepted: the contract measures one bounded build-causality question and defers storage and implementation choices. |
| Validation Checker | Accepted: fixtures, commands, environments, edit cases, negative behavior, statistics, and acceptance criteria are explicit. |

No role pre-approves a prototype. Each selected fixture and the completed census
must be reviewed again before Pulse 02 closes.

## Adopt now

- Freeze this measurement contract before collecting benchmark numbers.
- Select exact public fixture revisions and controlled edits.
- Begin with stable Cargo surfaces and synthetic controls.
- Keep private fixtures anonymous and supplementary.

## Prototype behind a compatibility boundary

- Optional nightly self-profile collection for unresolved compiler categories.
- A logical evidence-record serializer after the measurement fields stabilize.
- Automated edit application only inside disposable fixture worktrees.

## Defer

- Product recommendations before the census.
- Remote artifact distribution or shared binary caches.
- Automatic source or workspace rewrites.
- Direct rustc-internal dependencies.
- Publishing raw evidence from private repositories.

## Evidence basis

- [Rust compiler performance research](../research/2026-08-07-rustc-compiler-performance.md),
  especially FERRIUM-01 through FERRIUM-11 and its benchmark protocol.
- [Rust latency component roadmap](../research/2026-08-07-rust-latency-component-roadmap.md),
  especially FERRIUM-12 through FERRIUM-23 and Phases 0 through 2.
- [FERRIUM engineering principles](../governance/ENGINEERING_PRINCIPLES.md).
- [Rust latency telemetry](../research/2026-08-07-rust-latency-telemetry.md),
  especially FERRIUM-35 through FERRIUM-41.
- [Cargo build-unit identity](../research/2026-08-07-cargo-build-unit-identity.md),
  especially FERRIUM-42 through FERRIUM-50.
- [Cargo graph scheduling and critical paths](../research/2026-08-08-cargo-graph-scheduling.md),
  especially FERRIUM-51 through FERRIUM-58.
- [Cargo build-unit multiplication](../research/2026-08-08-cargo-build-unit-multiplication.md),
  especially FERRIUM-59 through FERRIUM-67.
- [Cross-workspace Cargo artifact reuse](../research/2026-08-08-cross-workspace-artifact-reuse.md),
  especially FERRIUM-68 through FERRIUM-77.
- [CI cache topology and duplicate Rust work](../research/2026-08-08-ci-cache-topology.md),
  especially FERRIUM-78 through FERRIUM-87.
- [Editor and Cargo contention](../research/2026-08-08-editor-cargo-contention.md),
  especially FERRIUM-88 through FERRIUM-97.
- [rustc startup and metadata loading](../research/2026-08-08-rustc-startup-metadata.md),
  especially FERRIUM-98 through FERRIUM-107.
- Candidate root manifests inspected during corpus discovery:
  `METIS-CORE/Cargo.toml`, `PARLOR/Cargo.toml`, `RUNE/Cargo.toml`,
  `RLINE/Cargo.toml`, `ICELINES/Cargo.toml`, and `BISECT/Cargo.toml`.
