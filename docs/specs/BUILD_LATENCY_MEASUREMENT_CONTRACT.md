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

## System-environment vocabulary

Environment evidence distinguishes:

1. **Host identity:** operating-system build, machine class, CPU model and
   topology, physical memory, storage device class, and firmware where relevant.
2. **Execution substrate:** native host, VM, container, WSL version, guest
   kernel, and configured CPU, memory, and swap limits.
3. **Source placement:** filesystem, mount, device, and execution-boundary path
   containing source, manifests, lockfiles, and generated inputs.
4. **Target placement:** filesystem, mount, device, and execution-boundary path
   containing Cargo fingerprints, metadata, objects, archives, incremental
   state, linker state, and outputs.
5. **Auxiliary placement:** Cargo home, registry, Git cache, temp, linker cache,
   and other read/write roots.
6. **Filesystem crossing:** access that crosses a VM, container, network,
   remote, Windows/Linux, translation, or filter boundary.
7. **Project-cold:** a new empty Cargo target directory with registry and
   toolchain acquisition excluded.
8. **Process-cold:** relevant compiler, linker, daemon, and helper processes
   were not already resident.
9. **Page-cache state:** operating-system and guest cache state, or unknown.
10. **Physical-cold:** storage-device cache and operating-system page state were
    deliberately controlled through a safe dedicated benchmark procedure.
11. **CPU policy:** Cargo jobs, rustc jobs, jobserver domains, explicit
    affinity, processor classes, logical processors, power plan, and frequency
    observations.
12. **Memory policy:** host and guest limits, current availability, swap,
    reserved capacity, process-tree peak RSS, and concurrent session demand.
13. **Security state:** antivirus or endpoint product, real-time status,
    supported performance mode, exclusions if owner-approved, and trace
    reference.
14. **Indexing state:** indexer service, configured scope when known, and
    measured path activity.
15. **Thermal state:** sensor, throttle, frequency, fan, battery, or power-source
    evidence, or unknown.
16. **Background pressure:** competing builds, editors, agents, CI helpers,
    services, and system activity observed during the run.
17. **Environment-equivalent comparison:** source, toolchain, command,
    placement, cache assumptions, CPU, memory, security, indexing, power, and
    background state are matched closely enough for the claimed cause.
18. **Attribution confidence:** measured cause, correlated state, uncontrolled
    confounder, or unknown.

Project-cold must not be called hardware-cold or physical-cold. Reports record
which cache layers were actually controlled. Cache dropping, device flushing,
forced memory exhaustion, or service termination are prohibited on shared
workstations unless a dedicated owner-approved procedure explicitly permits
them.

Source, target, Cargo home, temp, incremental, linker-state, and cache placement
are independently recorded. A path string alone is insufficient when the path
crosses WSL, a container mount, a network share, a VM shared folder, or another
translation layer.

Job-count evidence records a response curve with wall, CPU, memory, variance,
and graph context. Logical processor count is not accepted as an automatic
optimum.

Antivirus, indexing, power, or thermal impact is not inferred from wall time.
Use supported operating-system traces where available. Real-time protection,
indexing, services, power plans, VM limits, affinity, and process priority are
not changed automatically.

Wall time remains primary for user-visible environment impact. Compiler-change
claims intended for upstream require rustc-perf-compatible stable-work evidence
on controlled hardware before small differences are attributed to compiler
algorithms.

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

## Remote artifact and forest-root vocabulary

Remote reuse reports distinguish:

1. **Action identity:** canonical build type, external parameters, resolved
   dependencies, and platform properties used before content lookup.
2. **Content identity:** cryptographic digest and size of an immutable blob or
   canonical forest root.
3. **Artifact class:** supported Cargo unit, complete rustc incremental
   generation, final output, evidence packet, or validation result.
4. **Compatibility envelope:** compiler and sysroot, target and ABI, source,
   dependencies, profile, features, flags, platform, and declared execution
   inputs required by the class.
5. **Execution-cone disposition:** included, isolated and declared, or excluded
   because build scripts, proc macros, native tools, SDKs, environment,
   filesystem, clocks, or network make the identity incomplete.
6. **Producer trust:** accepted signer or workload identity, builder ID, build
   type, permissions, and publication scope.
7. **Consumer expectation:** required action, source, platform, dependency,
   validation, and trust values checked before installation.
8. **Immutable root:** signed canonical manifest of subjects, provenance,
   lineage, validation, and transport references.
9. **Mutable label:** signed name, sequence, expiry, and root digest. A label is
   not a cache key or correctness claim.
10. **Atomic publication:** stage and verify one successful finalized producer
    state before making the root visible.
11. **Isolated installation:** verify immutable content, then materialize it
    into a private mutable consumer directory before Cargo or rustc use.
12. **Revocation:** deny future resolution or use by signer, builder, label, or
    root policy; physical deletion is a separate retention action.
13. **Net benefit:** avoided compilation minus hashing, compression, transfer,
    extraction, verification, locking, contention, and miss cost.

Reports must test exact acceptance and intentional rejection for source,
compiler, sysroot, target, ABI, flags, profile, features, dependencies, and
declared execution inputs relevant to the artifact class. Corruption,
manifest substitution, missing blobs, mix-and-match state, rollback, expiry,
revocation, interrupted publication, and ordinary rebuild recovery are required
negative cases before artifact-bearing automation.

A complete rustc incremental generation may be measured only as opaque atomic
state. Individual internal files are never portable entries or composition
units. A verified immutable generation must be copied or materialized into an
isolated mutable directory because rustc advances compiler-private state during
use.

One signed transport digest is sufficient for normal monolithic-archive
integrity verification. Rehashing the extracted tree is reported separately as
an audit mode. Neither signature validity, provenance, Cargo freshness, cache
hit, label resolution, nor compiler success is accepted as behavioral
correctness evidence.

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

## Parsing and tokenization vocabulary

Parser evidence distinguishes:

1. **Root source load:** opening, reading, hashing, and registering the crate
   root with the source map.
2. **Raw lexing:** minimal token kind and length recognition.
3. **Token cooking:** identifier interning, literal handling, whitespace and
   comment treatment, and conversion into compiler tokens.
4. **Token-tree construction:** delimiter matching, spacing, nested token
   streams, and diagnostic state.
5. **Root AST parse:** recursive-descent parsing of the crate-root file and
   inline modules.
6. **Outline module declaration:** `mod foo;` represented as an unloaded module
   at the root boundary.
7. **Outline module parse:** file location, source loading, lexing, token-tree
   construction, and parsing performed during expansion.
8. **Source shape:** bytes, line count, token density, delimiter depth, item
   count, expression shape, generated origin, and literal or comment payload.
9. **Module topology:** root, inline module, outline module, module-file count,
   nesting depth, and total source bytes.
10. **Persistent parse reuse:** token or tree state reused across compiler
    invocations. Cargo skipping rustc is invocation reuse, not a parser cache.
11. **Incremental reparse:** a bounded token, block, file, or tree region
    reconstructed after an edit.
12. **Parser concurrency:** lexer, root parser, module loader, or parser tasks
    that demonstrably overlap; frontend job count alone is not evidence.
13. **Parse failure behavior:** error position, recovery, fatal abort, emitted
    diagnostics, and source suffix that was or was not processed.
14. **Parser-attributed time:** external or internal time assigned to a named
    parser boundary with its included source loading and diagnostics stated.

The rustc `parse_crate` event is not accepted as a whole-crate parse timer.
It includes crate-root loading and root or inline-module parsing. Outline
module files are loaded and parsed later during expansion and require separate
attribution.

Source bytes and line count are not accepted as parser cost estimates without
token, item, expression, literal, comment, and module shape. A generated
source report records the generator revision and whether changing the
generator would alter diagnostics, API, compile-time execution, or runtime
behavior.

Stable end-to-end compilation remains the primary workflow evidence. Nightly
root-parse, no-analysis, time-passes, and self-profile runs are diagnostic
boundaries and retain their toolchain, event set, observer effect, and missing
coverage.

Incremental compiler experiments distinguish:

- Cargo freshness that skips rustc;
- rustc invocation with a fresh incremental directory;
- rustc invocation with a reused directory and untouched source;
- identical source bytes rewritten;
- a controlled semantic or non-semantic edit.

Rewriting a file can change source-loading, page-cache, antivirus, indexing,
and filesystem behavior even when the bytes remain identical. That cost is
not assigned to token or AST invalidation without further evidence.

Parser failures retain expected exit status and stderr. An early fatal error
can be faster because it processes less source; failed-fast latency is not a
throughput improvement.

rust-analyzer's lossless incremental syntax tree is not assumed to be reusable
by rustc. Any tree-sharing claim must establish token equivalence, editions,
cfg, macro and attribute behavior, diagnostics, source maps, mutation,
lifetime, and compiler ownership.

## Declarative macro expansion vocabulary

Macro-by-example evidence distinguishes:

1. **Macro origin:** local, imported, exported, built-in, attribute, derive, or
   procedural. This section applies to `macro_rules!`-style declarative
   expansion.
2. **Invocation topology:** macro definition, invocation count, nesting depth,
   recursive depth, call sites, and generated invocations.
3. **Input shape:** token count, delimiters, separators, token-tree depth,
   literal prefixes, named nonterminal fragments, and source bytes.
4. **Matcher arms:** arm count, declaration order, shared prefix, success arm,
   failed candidates, repetition nesting, and ambiguity behavior.
5. **Named nonterminal parse:** calls from the matcher into Rust expression,
   type, pattern, path, item, statement, or other fragment parsers.
6. **Named matches:** metavariable bindings constructed by the matcher,
   including repetition depth and captured token trees.
7. **Transcription:** selected right-hand-side traversal, repetition,
   metavariable substitution, token construction, and cumulative intermediate
   output.
8. **Expansion ratio:** cumulative successful expansion output bytes or tokens
   divided by invocation input. Recursive intermediate invocations are not
   final crate size.
9. **Hygiene:** expansion IDs, syntax contexts, transparency, and span marking
   required to preserve macro name semantics.
10. **Output reparse:** expanded tokens parsed into the required AST fragment
    and checked for trailing or malformed tokens.
11. **AST integration:** node and definition identity assignment, reduced
    graph updates, invocation collection, and newly introduced macros.
12. **Generated output:** final items, expressions, statements, types, and
    later validation, resolution, lowering, query, metadata, and codegen work.
13. **Edit fanout:** macro definition edit, one invocation edit, generated
    output delta, dependent crate invalidation, and later query invalidation.
14. **Expansion failure:** no matching arm, local ambiguity, recursion-limit
    failure, output parse failure, unresolved macro, recovery, and diagnostics.

Macro invocation count, input tokens, output bytes, arm count, and recursion
limit are not accepted as standalone cost estimates. Reports preserve matcher
prefix overlap, success position, repetition shape, cumulative intermediate
output, final output, and later generated-item work.

Stable repeated wall time remains primary. Nightly `macro-stats`,
`parse-crate-root-only`, `no-analysis`, time-passes, self-profile, expanded
output, and trace diagnostics remain separate observer-affected evidence.

`macro-stats` output is cumulative across successful expansions. Recursive TT
munchers can report large output even when their final expansion emits
nothing, because each invocation transcribes the remaining tail into another
invocation.

`macro_expand_crate` and `expand_crate` are compiler timing regions rather
than persistent incremental queries. Cargo skipping rustc is invocation reuse;
a reused rustc incremental directory is not assumed to cache declarative
expansion.

An identical source rewrite is required as a control before assigning an edit
delta to macro matching or invalidation. Definition edits and invocation edits
also record their different generated-output fanout.

Raising `recursion_limit`, reordering arms, replacing fragments, flattening
recursion, checking in expanded source, or changing generated APIs can alter
accepted syntax, diagnostics, hygiene, semantics, maintenance, and downstream
work. Such changes require explicit consumer validation and are not automatic
performance recommendations.

Declarative and procedural macro work remain separate. Process execution,
proc-macro server lifecycle, dynamic libraries, serialization, external I/O,
and proc-macro caching belong to PERF-Q22.

## Procedural macro vocabulary

Procedural-macro evidence distinguishes:

1. **Macro crate identity:** source, dependency graph, host artifact, compiler,
   profile, feature, target, and dynamic-library identity.
2. **Macro entry point:** derive, attribute, or function-like name and kind.
3. **Invocation topology:** invocation count, call sites, nesting, ordering,
   repeated identical inputs, and generated invocations.
4. **Token input:** token-tree shape, rendered diagnostic size, delimiters,
   spans, source mapping, hygiene, attributes, and annotated item where
   applicable.
5. **Declared external input:** environment variables and paths reported
   through compiler-supported tracked APIs.
6. **Hidden external input:** undeclared environment, filesystem, process,
   time, randomness, network, native-library, or working-directory state.
7. **Bridge execution:** compiler-to-macro crossing, execution strategy,
   server or process lifecycle, panic, diagnostics, and returned token stream.
8. **Token output:** rendered diagnostic size, token-tree shape, spans,
   diagnostics, and expansion success or failure.
9. **Generated shape:** emitted items, impls, expressions, statements, types,
   constants, functions, trait obligations, and other later compiler work.
10. **Rerun cause:** Cargo freshness miss, macro-crate change, invocation input
    change, declared input change, hidden input discovered only after another
    edit, or compiler-option change.
11. **Expansion reuse:** macro executed, derive output loaded from a rustc
    query cache, Cargo skipped rustc, or no supported reuse.
12. **Capability boundary:** same thread, cross thread, native server process,
    operating-system sandbox, capability sandbox, or deterministic runtime.

Invocation count, token characters, self-profile `expand_proc_macro` time,
macro-crate compile time, generated line count, and final wall time are not
accepted as standalone procedural-macro cost estimates. Reports preserve
native execution, output parsing and integration, generated Rust work, and
later semantic and backend cost separately.

Primary timings run without macro logging. Token-size and macro-internal
instrumentation are separate observer-affected diagnostics because converting
token streams, recording timestamps, and writing logs can change execution.
Self-profile is another separate diagnostic boundary.

Cargo skipping rustc, rustc invoking a procedural macro, and rustc loading a
cached derive output are distinct reuse events. A crate-level rebuild edge is
not accepted as proof that a cached macro result depends on the same declared
inputs.

Tracked environment and path controls must be paired with ordinary untracked
reads. A changed hidden input that leaves stale output is recorded as a
correctness failure, not a cache hit.

The rustc `-Zcache-proc-macros` option is unsupported evidence only. It must
remain disabled in ordinary runs and must not be recommended: PERF-Q22
observed stale derive output after both hidden and tracked input changes.

A proposed procedural-macro cache identity must name macro artifact and entry
point, token trees, observable span and hygiene state, declared environment and
file inputs, compiler and bridge protocol, edition, target, cfg, relevant
options, diagnostics, and output compatibility. Unknown capabilities require
no-cache or explicit rejection; they must not be silently omitted.

Thread and process separation are not called sandboxes without explicit
capability restrictions. Deterministic WebAssembly or other restricted
execution remains an opt-in compatibility class until ecosystem, tooling,
performance, provenance, and rollback evidence exists.

Macro consolidation, source rewriting, checked-in expansion, cache activation,
and sandbox enforcement can change API, diagnostics, hygiene, security,
maintenance, and performance. They require explicit consumer validation and
are not automatic recommendations.

Build-script execution, `rerun-if-*` directives, output directories, native
toolchains, and build-script-specific caching remain PERF-Q23.

## Build-script vocabulary

Build-script evidence distinguishes:

1. **Script compile identity:** package, script source, build dependencies,
   host compiler, profile, features, flags, and executable artifact.
2. **Script run identity:** package, target or host context, profile, features,
   configuration, output directory, and `RunCustomBuild` unit.
3. **Detection mode:** package-wide default, declared paths and environment,
   target configuration override, or unknown.
4. **Declared path input:** file, directory, missing path, symlink, path
   normalization, and mtime evidence named by `rerun-if-changed`.
5. **Declared environment input:** variable name and Cargo-received value
   named by `rerun-if-env-changed`.
6. **Hidden input:** undeclared file, environment, working directory,
   filesystem metadata, process, native library, tool discovery, time,
   randomness, network, temporary directory, or other external state.
7. **Rerun cause:** script executable change, declared path, declared
   environment, package-wide package change, dependency output, profile,
   target, configuration, forced rebuild, or unknown.
8. **Saved output replay:** Cargo parsed previously stored stdout and stderr
   while the process remained fresh.
9. **Instruction output:** link search, linked library, linker argument, cfg,
   check-cfg, rustc environment, immediate-dependent metadata, warning, error,
   and their ordering where relevant.
10. **Generated output:** files and directories created, rewritten, retained,
    removed, or left stale under `OUT_DIR`.
11. **Effective output:** instruction output, generated output, and any
    externally visible effect relevant to compile or link correctness.
12. **Output ownership:** script identity, declared path, content identity,
    retained or ephemeral policy, cleanup owner, atomic publication, and
    failure recovery.
13. **Fan-out:** owning crate compile, immediate dependent build script,
    transitive crate, codegen, native link, relink, test target, or external
    build-system work caused by the run.
14. **Capability boundary:** unrestricted process, custom runner, container,
    operating-system sandbox, capability sandbox, deterministic runtime, or
    unknown.
15. **Execution evidence:** external script log, Cargo build-analysis dirty run
    unit, process trace, or another direct execution signal.

A `build-script-executed` Cargo JSON message, saved warning, or effective
instruction set is not accepted as proof that the script process executed in
that invocation. Cargo can replay saved output for a fresh run unit.

If a script emits no rerun instruction, reports record the package-wide scan
root, include/exclude scope, file count where practical, and relevant
filesystem limitations. This mode is conservative compatibility behavior, not
automatically unnecessary work.

Once any rerun instruction exists, reports treat the declared file and
environment set as the runtime dependency contract. Narrower is not
automatically better: an incomplete declaration that leaves stale output is a
correctness failure.

Declared file freshness and Rust source checksum freshness are separate. A
same-content declared-file rewrite remains a required control even when
`-Zchecksum-freshness` is enabled. Mtime false positives and false negatives
are reported separately.

Write-if-changed and Cargo compile freshness are separate. Preserving generated
bytes and mtime does not prove that Cargo will keep the owning or downstream
units fresh after the script run unit changed.

`OUT_DIR` is persistent. A stale file is not automatically removed, and its
presence is not automatically a Cargo defect. Reports identify the producing
script and expected lifecycle. Whole-directory cleanup is prohibited as an
experimental recommendation unless the script's ownership contract explicitly
permits it.

`rustc-env`, `rustc-cfg`, `rustc-link-*`, warning, and `links` metadata are not
one generic output class. Reports preserve the receiving compiler or immediate
dependent edge and the observed downstream fan-out.

A `links` target override is recorded as a supported configuration boundary
that prevents the original script from compiling or running. Reports preserve
the target triple and complete supplied link, cfg, environment, and metadata
values.

Process separation, a custom runner, a container, and `OUT_DIR` convention are
not called sandboxes without explicit capability restrictions. Filesystem,
environment, process, network, native toolchain, temporary-directory, time,
randomness, IPC, fallback, and rollback semantics remain visible.
Portable sandbox claims require explicit per-platform capability enforcement
and fallback behavior.

Nightly Cargo build-analysis may provide run identifiers, unit graphs,
fingerprint causes, root and cascading rebuilds, unit durations, and unblocking
events. It remains optional, observer-affected, unstable evidence behind an
exact Cargo-version and schema boundary. Stable Cargo JSON and repeated wall
time remain the ordinary baseline. Nightly build analysis must not be required
for correct operation.

A proposed build-script cache or unchanged-output decision must bind script
compile and run identity, every declared input, capability policy, complete
effective output, output ownership, diagnostics, target and profile context,
toolchain and native dependencies, failure state, and compatibility. Unknown
inputs or effects require no-cache or explicit rejection.

Automatic declaration rewriting, output deletion, rerun suppression, script
caching, sandbox enforcement, and target overrides can change correctness,
security, portability, native integration, diagnostics, and maintenance. They
require explicit consumer validation and are not automatic recommendations.

## Monomorphization and generic-instance vocabulary

Generic evidence distinguishes:

1. **Generic definition:** crate, DefId or stable diagnostic name, item kind,
   source body, bounds, attributes, visibility, and defining-crate metadata.
2. **Concrete substitution:** type, lifetime-erased, and const arguments used
   for one compiler instance, including shared type identity across crates.
3. **Instance family:** all concrete instances attributed to one generic
   definition in one measured configuration.
4. **Collection mode:** lazy or eager collection and the incremental state
   that selected it.
5. **Collected mono item:** a function, method, closure, static, or drop-glue
   item selected for code generation in one crate. Constants, vtables, and
   some shims may be generated on demand and remain separate.
6. **Instance owner:** the crate and codegen unit that collected an instance
   locally.
7. **Upstream provider:** an ancestor dependency whose exported exact instance
   allowed a downstream crate to avoid local collection.
8. **Effective sharing mode:** compiler default or explicit override, paired
   with optimization level and relevant attributes such as `#[inline]` and
   `#[inline(never)]`.
9. **Sibling duplication:** equivalent concrete instances emitted by crates
   that share a dependency but are not upstream of each other.
10. **Cross-workspace duplication:** equivalent concrete instances emitted by
    separate workspace roots even when an ordinary dependency artifact is
    fresh or shared.
11. **Mono estimate:** rustc's per-definition size estimate, instantiation
    count, and total estimate. It is not LLVM IR lines, object bytes, machine
    instructions, or final binary bytes.
12. **Emitted symbol:** an object or archive symbol with linkage, visibility,
    section, object owner, and byte evidence where available.
13. **Selected object:** an archive member or object accepted as a link input.
    Archive presence does not prove selection.
14. **Link-equivalent class:** symbols assigned one final address through
    COMDAT selection, identical-code folding, LTO, or another linker decision.
15. **Final retention:** retained address, section bytes, exported symbol, or
    other final-image evidence after linking.
16. **Generic shell:** type-dependent conversion, validation, dispatch, or
    wrapper logic that remains instantiated per concrete substitution.
17. **Non-generic core:** type-independent work called by one or more generic
    shells and emitted independently of their substitutions.
18. **Runtime control:** representative execution evidence paired with a
    sharing, inlining, erasure, LTO, or source-structure change.

Mono-item count is not accepted as a standalone compile-time, binary-size, or
runtime claim. Reports pair family counts with compiler estimates, relevant
phase timing, emitted bytes, and final-link evidence where the decision
depends on them.

Collected items, emitted symbols, rlib bytes, selected archive members, folded
aliases, and retained final code are different states. A report must name the
state it calls duplicate. Duplicate symbols in intermediate artifacts are not
accepted as final binary duplication without linker or image evidence.

Generic-sharing reports preserve dependency direction. An upstream provider
can satisfy a downstream instance; sibling crates do not become upstream of
each other. A later dependent may reuse one sibling's exported instance while
both siblings still emit copies.

The effective sharing mode records exact rustc revision and optimization
level. Current unstable defaults or flags are not assumed stable across
toolchains. `#[inline(never)]` reuse and local-copy behavior are recorded
separately from the global sharing setting.

Collection mode, target triple, ABI, codegen backend, optimization, LTO,
codegen units, target features, panic behavior, overflow checks,
instrumentation, debuginfo, symbol mangling, linkage, visibility, dependency
metadata, and compiler revision remain part of the measured instance context.
Two equal source-level substitutions are not assumed to be reusable machine
code when these dimensions differ.

Unused generic parameters require an explicit control. A compiler may still
collect one item per substitution even when the source body does not use a
parameter. Historical or WIP polymorphization behavior is not assumed.

A generic-shell/non-generic-core comparison preserves:

- public API and trait-bound behavior;
- diagnostics and type inference;
- panic, overflow, layout, drop, and allocation behavior;
- inlining and final-link behavior;
- object and binary bytes; and
- representative runtime.

Core extraction may reduce repeated IR while leaving one shell per
substitution. It is a review candidate, not an automatic source rewrite.
Trait objects, function pointers, erased adapters, and non-generic APIs change
dispatch or type contracts and require separate semantic and runtime review.

Sharing, LTO, codegen-unit, inlining, and visibility changes can exchange
compile time, parallelism, archive size, linker work, binary size, and runtime
optimization. No one axis is an automatic recommendation.

Nightly `-Zprint-mono-items` and `-Zdump-mono-stats` may provide item, family,
estimate, codegen-unit, and linkage evidence. They remain optional,
observer-affected, unstable diagnostics behind an exact rustc-version and
schema adapter. Stable Cargo and ordinary compilation must remain available
without them.

A Build Forest may record generic-family summaries, instance owners, repeated
roots, and evidence references. It must not treat compiler objects or generic
machine code as portable cache entries. Any cross-workspace publication or
restoration follows the remote artifact and forest-root vocabulary above;
function-level machine-code caching follows the dedicated vocabulary below.

Automatic generic API rewriting, dispatch conversion, sharing overrides,
inlining changes, LTO changes, codegen-unit changes, cross-workspace writable
targets, and machine-code restoration can change semantics, correctness,
portability, performance, reproducibility, and maintenance. They require
explicit consumer validation and are not automatic recommendations.

## Codegen-unit partitioning vocabulary

Backend partition evidence distinguishes:

1. **Requested maximum:** the explicit or compiler-default upper bound passed
   to partitioning. It is not the actual emitted count.
2. **Request origin:** user, Cargo profile, target default, or rustc default.
   Current rustc defaults are toolchain behavior, not stable FERRIUM policy.
3. **Initial partition:** the source-derived CGU before count-based merging.
4. **Stable partition:** incremental placement for non-generic module code.
5. **Volatile partition:** incremental placement for generic instances whose
   reference topology may change independently of the defining body.
6. **Fallback partition:** items for which the compiler cannot derive a more
   specific characteristic source module.
7. **Root placement:** the primary globally shared placement of one mono item.
8. **Local-copy placement:** an internal copy made available in a consuming CGU
   for inlining or required glue.
9. **Placement multiplicity:** how many CGUs contain one mono-item identity,
   with linkage and visibility in each.
10. **CGU size estimate:** rustc's pre-LLVM estimate. It is not LLVM
    instructions, optimization duration, object bytes, memory, or runtime.
11. **Inline overlap:** estimated size of local-copy items shared by two CGUs
    and used by the current merge heuristic.
12. **Merge lineage:** the ordered initial partitions consumed into one final
    CGU and the reason for each merge.
13. **Actual CGU:** a final partition presented to backend code generation
    after count and minimum-size merging.
14. **Work-product identity:** the incremental backend cache identity associated
    with one CGU and its compiler context.
15. **Pre-LTO reuse:** reuse of a CGU before local or whole-graph LTO imports
    are applied.
16. **Post-LTO reuse:** reuse after LTO import and optimization dependencies are
    considered. Current unstable reporting may be incomplete.
17. **Partition stability:** unchanged common mono items whose final CGU
    placement remains identical after a controlled edit.
18. **Partition churn:** unrelated common items whose final CGU name or
    composition changes after an edit.
19. **Backend makespan:** wall time from backend availability until every CGU
    required for linking finishes, distinct from summed CPU work.
20. **Backend resource envelope:** CPU, peak memory, temporary and object bytes,
    worker concurrency, and simultaneous crate pressure.
21. **LTO scope:** none, local ThinLTO across one crate's CGUs, explicit
    whole-graph ThinLTO, or fat LTO.
22. **Final controls:** link duration, selected inputs, executable or library
    bytes, representative runtime, and behavior checks.

Every report preserves requested maximum and actual CGUs separately. A
requested count is not accepted as evidence that the compiler emitted that
many units. Default, explicit, incremental, and non-incremental configurations
are not interchangeable.

Initial source partitions and final merged CGUs are separate states. Reports
record stable, volatile, fallback, and upstream-derived initial units where the
diagnostic exposes them, then retain merge lineage into actual units.

One mono-item identity can have root and local-copy placements. Duplicate item
identity count is not placement multiplicity, estimated duplicate cost, object
duplication, or final retained code. Inline-copy evidence records all consuming
CGUs and is joined with final-link evidence where size or retention matters.

CGU size estimates are not accepted as observed LLVM cost. A scheduling or
merge recommendation requires measured per-unit or whole-backend time, estimate
error, peak memory, object size, and long-pole evidence. More equal estimated
bins are not assumed to improve makespan.

Incremental comparisons include unchanged, one-function body, generic
reference, inline body, module topology, and broad interface controls where
applicable. A high work-product hit rate is not accepted as a latency
improvement without load, proof, copy, coordination, link, and storage cost.

Partition stability comparisons exclude directly edited items and explicitly
name the remaining common-item denominator. A merge-name change is preserved
as work-product identity churn even when source bodies are unchanged.

Compiler backend parallelism is distinct from Cargo job parallelism, rustc
frontend parallelism, and simultaneous independent rustc processes. Reports
record logical processors, jobserver context, other active builds, and memory
pressure where available.

Local ThinLTO and whole-graph ThinLTO are separate modes. Reports preserve
bitcode, import topology, codegen units, incremental state, linker, and pre- or
post-LTO work-product evidence. They do not infer post-LTO reuse from an
unstable diagnostic known to be incomplete.

One CGU is not called maximum runtime performance. Fewer CGUs can increase
cross-function optimization, but inlining, register pressure, code layout,
vectorization, LTO, target, and workload can reverse the result. Any profile
recommendation requires representative runtime and output-size controls.

Nightly `-Zprint-mono-items`, human-readable CGU names, time passes, self
profiles, and saved temporary files are optional, observer-affected, unstable
evidence behind an exact rustc revision and output-mode adapter. Output
selection can itself change the effective CGU count. The adapter fails closed
on warnings, schema drift, incompatible output modes, or missing data.

A Build Forest may record partition summaries, merge lineage, work-product
dispositions, profile comparisons, and evidence references. It must not treat
CGU object bytes as portable or independently restorable cache entries.
Publication and provenance follow the remote artifact and forest-root
vocabulary above; function-level reuse follows the dedicated vocabulary below.

Automatic codegen-unit, incremental, LTO, inlining, source-module, crate,
linker, or Cargo profile changes can exchange compile time, CPU, memory,
storage, link time, binary size, runtime, reproducibility, and maintenance.
They require representative held-out validation, explicit rollback, and human
approval and are not automatic recommendations.

## LLVM optimization-cost vocabulary

LLVM evidence distinguishes:

1. **IR translation:** rustc conversion of mono items in one CGU into LLVM IR,
   before LLVM's optimization pipeline.
2. **Requested optimization label:** rustc or Cargo policy `0`, `1`, `2`, `3`,
   `s`, or `z`, including its user, profile, or default origin.
3. **LLVM pipeline level:** the actual LLVM pipeline selected by the pinned
   rustc/LLVM build. It is not assumed identical to the requested label.
4. **Function size policy:** `optsize`, `minsize`, or neither, recorded
   separately from pipeline level.
5. **Optimization stage:** pre-link no-LTO, pre-link ThinLTO, pre-link fat-LTO,
   post-import ThinLTO, or merged fat-LTO.
6. **IR scope:** module, strongly connected component, function, loop, or
   analysis unit named by the diagnostic.
7. **Rust shape:** source owner, generic family and instance, inline copy,
   loop, branch graph, aggregate, intrinsic, target-feature use, or generated
   body associated with an IR scope where supported.
8. **Pass invocation:** one execution of a named LLVM analysis or
   transformation over one IR scope.
9. **Pass class:** the named LLVM analysis, transformation, wrapper, adaptor,
   or machine pass aggregated across invocations.
10. **Inclusive event duration:** elapsed time for one event including nested
    child events.
11. **Child event work:** nested pass or analysis duration. It is not added to
    its parent as independent wall time.
12. **Pass event tree:** parent-child structure across module, SCC, function,
    loop, analysis, adaptor, and transformation events.
13. **LTO import:** a function or summary made visible across a CGU or crate
    boundary for post-link optimization.
14. **Machine pass:** target-specific work after LLVM IR optimization,
    including instruction selection, scheduling, register allocation, frame
    lowering, and assembly printing.
15. **Instruction selection:** conversion of target-independent LLVM IR into
    target machine instructions.
16. **Register allocation:** assignment of virtual values to physical
    registers and spill locations.
17. **Emission:** object, bitcode, assembly, IR, debug, or related file writing
    after optimization and machine-code work.
18. **Coarse LLVM region:** a rustc timer around a backend region. It is not
    accepted as summed pass work.
19. **Diagnostic work:** summed or hierarchical trace activity collected under
    an unstable profiler. It is not primary compile wall time.
20. **Observer effect:** wall, CPU, memory, and output change caused by the
    profiling mode itself.
21. **Intermediate size:** LLVM IR, bitcode, object, archive, PDB, or another
    non-final artifact size.
22. **Final controls:** behavior checks, representative runtime, final binary
    bytes, peak memory, link cost, and relevant deployment properties.

IR translation, pre-link optimization, LTO optimization, machine passes,
emission, and linking remain separate regions. A report does not call their
sum "LLVM time" unless the exact boundaries and overlap semantics are defined.

Requested optimization labels do not define a stable LLVM pipeline across
toolchains. Reports record rustc revision, LLVM revision, target, backend,
pipeline level, size attributes, tuning options, LTO stage, CGUs, incremental
state, debuginfo, panic behavior, and relevant target features.

Pass events are hierarchical. Inclusive wrapper, adaptor, module, function,
loop, analysis, and child transformation durations are not added as independent
work. Reports preserve invocation count, scope, parent, thread or CGU where
available, and whether a value is wall time, summed diagnostic work, or a
coarse region.

Pass name or IR-line count is not accepted as a standalone optimization
diagnosis. Reports join the expensive scope to Rust shape, generic and inline
topology, CGU, imports, target, machine passes, and final controls where the
decision depends on them.

Minimally instrumented complete compilation remains primary. Nightly
`-Zllvm-time-trace`, `-Ztime-llvm-passes`, `-Zself-profile`, raw IR, and
machine-pass diagnostics are optional, observer-affected evidence behind an
exact-version adapter. Every diagnostic comparison records calibration and
trace bytes and fails closed on missing events, schema drift, or unknown
overlap.

Intermediate LLVM IR, bitcode, object, archive, or debug size is not final
binary size. Runtime and size recommendations require consumer-representative
final artifacts and workloads.

A Build Forest may record LLVM stage summaries, pass-event references,
expensive Rust shapes, machine-pass regions, and alternative supported-profile
roots. It must not inject compiler flags, mutate profiles, or treat trace data
as a correctness proof.

Automatic optimization-level, size-policy, vectorization, unrolling, inlining,
LTO, target-feature, debuginfo, codegen-unit, backend, linker, or source changes
can exchange compile time, runtime, size, memory, ABI behavior,
reproducibility, and maintenance. They require held-out validation, rollback,
and human approval and are not automatic recommendations.

## Development codegen-backend vocabulary

Development-backend evidence distinguishes:

1. **Backend component:** the exact codegen implementation, rustc revision,
   distribution channel, file identity, and capability version.
2. **Default backend:** the backend selected by the target and toolchain without
   an explicit override.
3. **Alternative backend:** a non-default implementation selected for one
   command or profile, such as Cranelift.
4. **Shared compiler work:** parsing, expansion, semantic analysis, MIR,
   monomorphization, metadata, and other work that remains before backend
   codegen.
5. **Replaceable codegen share:** the measured portion of a workflow whose work
   can change when the backend changes.
6. **Backend eligibility:** target, architecture, operating system, crate type,
   panic, intrinsic, target-feature, ABI, debuginfo, sanitizer, coverage,
   profiler, LTO, and distribution requirements that a backend must satisfy.
7. **Backend selection scope:** command, Cargo profile, target, package graph,
   environment, configuration, or tool invocation affected by the override.
8. **Backend artifact identity:** object, metadata, archive, executable, debug,
   and incremental outputs produced under one exact backend identity.
9. **Isolated backend root:** a target and incremental directory that contains
   artifacts from only one backend identity.
10. **Clean backend comparison:** matched compilation from absent target state.
11. **Warm backend comparison:** matched no-op or fresh command after the
    backend's own artifacts already exist.
12. **Incremental backend comparison:** matched controlled edit using only that
    backend's prior incremental generation.
13. **Test compilation:** creation of test libraries and harness executables
    without executing them.
14. **Test execution:** running tests, including passing, failing, panic,
    timeout, ignored, subprocess, and diagnostic behavior.
15. **Failure semantics:** exit status, panic behavior, timeout, termination,
    harness summary, diagnostic output, cleanup, and subprocess propagation.
16. **Effective panic strategy:** the panic behavior actually supported and
    emitted for the target and backend, not merely the requested profile value.
17. **Development runtime control:** representative execution time and behavior
    of an alternative-backend artifact used during iteration.
18. **LLVM validation control:** ordinary default-backend build, test, release,
    or capability checks retained for evidence not established by the
    alternative backend.
19. **Backend rollback:** removal of the optional override and isolated outputs
    without source, manifest, or shared-cache recovery.
20. **Backend outcome:** wall, CPU, memory, artifact, behavior, failure,
    runtime, debug, ABI, and validation evidence for one workflow.

Check, clean build, warm build, incremental edit, test compilation, test
execution, run, benchmark, and release remain separate workflows. A clean
codegen improvement is not applied to check, fresh, incremental, test, runtime,
or release claims without direct evidence.

Backend selection is part of build and artifact identity. Reports record rustc
revision, backend component and hash, target, profile, panic strategy,
codegen units, optimization, LTO, debuginfo, target features, command,
environment origin, target root, incremental root, and relevant native tools.

Backend roots do not share objects, archives, executables, debug data, or
incremental generations. Matching source and metadata do not make machine-code
artifacts interchangeable across backends.

Passing compilation and happy-path tests are not backend equivalence. Reports
include intentional failure and panic controls plus consumer-required runtime,
debugger, sanitizer, coverage, profiler, intrinsic, inline assembly, ABI,
dynamic-library, FFI, and native-link evidence.

Intermediate archive or executable bytes are not accepted as code quality.
Runtime, final size, debug, deployment, and release controls remain separate.

Nightly profile backend selection and unstable backend components remain behind
an exact-version compatibility adapter. Missing components, unsupported
targets, capability drift, unexpected panic behavior, artifact mixing, or
failure-diagnostic differences fail closed.

A Build Forest may compare sibling roots produced by supported backends under
one source parent. It must preserve backend identity and validation
dispositions and must not restore or relabel one backend's artifacts as
another's.

Automatic `Cargo.toml`, `.cargo/config.toml`, environment, CI, editor,
backend, panic, target-feature, LTO, release, or validation changes require
held-out evidence, rollback, and human approval and are not automatic
recommendations.

## Function-level machine-code cache vocabulary

Function-cache evidence distinguishes:

1. **Rust semantic envelope:** the concrete mono item, substitutions, type and
   layout identity, MIR and lowering revision, dependency metadata, imported
   or inlined bodies, calling convention, ABI, panic and overflow behavior,
   instrumentation, symbol requirements, and other rustc-owned inputs that
   make backend lowering valid.
2. **Backend function stencil:** the compilation-relevant backend IR and
   metadata used as the backend's authoritative function cache key.
3. **Finalization parameters:** function-specific names, external identities,
   relocations, or fixups applied after restoring a compiled stencil.
4. **Backend cache key:** the cryptographic digest computed by the backend from
   the stencil, ISA, target, shared flags, ISA-specific flags, and backend
   version policy.
5. **Cache value:** the serialized or in-memory compiled stencil returned for
   one exact backend key.
6. **Admission decision:** cache, bypass, or evict, with the estimated compile
   cost, serialized bytes, expected reuse, memory pressure, and policy reason.
7. **Cold population:** compilation plus keying, serialization, insertion, and
   accounting when an entry is absent.
8. **Exact hit:** retrieval and successful finalization for the exact backend
   key under an accepted Rust semantic envelope.
9. **Semantic miss:** rustc determines that the function must be lowered under
   a different semantic envelope before backend lookup.
10. **Backend miss:** exact lowering reaches the backend but the stencil or ISA
    key is absent or different.
11. **Integrity miss:** bytes are absent, malformed, unauthenticated,
    corrupted, revoked, or not bound to the requested key and are rejected.
12. **Restoration cost:** key computation, lookup, synchronization, retrieval,
    deserialization, parameter application, relocation or fixup work, and
    validation on a hit.
13. **Avoided compilation:** backend function compilation work not performed
    because restoration succeeded. It excludes unchanged frontend and linking
    work.
14. **Invalidated CGU:** the compiler work product that ordinary incremental
    reuse could not copy after the edit.
15. **Stable neighboring function:** a function inside an invalidated CGU whose
    accepted semantic envelope and backend stencil remain unchanged.
16. **Cross-function dependency:** imported body, LTO state, whole-program
    decision, profile summary, or other input that makes one function's output
    depend on another function or larger optimization unit.
17. **Capability disposition:** target, debug, unwind, panic, ABI, intrinsic,
    inline assembly, sanitizer, coverage, profiler, PGO, relocation, symbol,
    linker, debugger, and runtime evidence required for the consumer workflow.
18. **Trusted in-process store:** compiler-owned memory whose isolation and
    lifecycle satisfy the backend API's key-to-bytes precondition without
    claiming persistence integrity.
19. **Persistent store:** disk, daemon, shared memory, service, or remote
    storage that must authenticate the binding among key, bytes, size,
    producer, version, and policy.
20. **Cache-check mode:** diagnostic recompilation that compares restored and
    freshly compiled results. It is validation evidence and removes the
    performance benefit while enabled.
21. **Daemon identity:** compiler, backend, protocol, user or workspace,
    target, capability, and policy identity for one persistent process.
22. **Daemon lifecycle:** startup, discovery, connection, version negotiation,
    memory accounting, eviction, concurrency, isolation, crash recovery,
    restart, upgrade, shutdown, and rollback behavior.
23. **Net function-cache benefit:** avoided backend compilation minus
    population, lookup, restoration, integrity, memory, lifecycle, and
    additional link or validation costs.

rustc semantic acceptance precedes backend lookup. Equal source text, symbols,
MIR text, object bytes, or function names are not accepted as substitute cache
keys. After rustc has produced exact backend IR, the backend's own stencil key
remains authoritative.

Function-cache reports record rustc revision, backend component and version,
target, ISA, target features, profile, optimization, panic, overflow, CGUs,
LTO, debuginfo, instrumentation, dependency identity, command, source
revision, store type, admission policy, and cache-check mode.

Population, exact hit, local edit, broad semantic edit, backend-flag mismatch,
version mismatch, corruption, stale value, cache absence, and ordinary rebuild
are separate cases. Reports do not infer correctness from a hit or failure from
a miss.

Hit rate is not accepted as an outcome. Reports preserve wall, CPU, memory,
serialized bytes, emitted code bytes, entry count, lookup and restoration cost,
avoided compile work, eviction, and end-to-end compile and link results.

The reusable unit is not assumed to remain one function under LLVM, LTO,
cross-function optimization, imported bodies, instrumentation summaries, or
PGO. Those modes require a separately demonstrated identity and capability
boundary.

Code bytes are not a complete object. Debug, unwind, relocations, symbols,
visibility, ABI, object integration, linking, debugger behavior, panic,
runtime, sanitizer, coverage, and profiler controls remain explicit where the
workflow depends on them.

A trusted in-process store may rely on compiler process isolation. Persistent,
shared, or remote stores follow the remote artifact and forest-root integrity,
provenance, quarantine, revocation, and recovery vocabulary. Deserialization
or a backend version marker alone is not authenticated integrity.

Minimally instrumented complete compilation remains primary. Optimized-MIR
dumps, mono-item output, self profiles, backend cache counters, and cache-check
mode are optional, observer-affected, unstable evidence behind an exact
toolchain and schema adapter.

A Build Forest may record invalidated CGUs, stable and changed function
stencils, admission decisions, hits, misses, restoration cost, integrity
dispositions, capability evidence, and upstream experiment references. It must
not compute an independent key, retain restorable function blobs as ordinary
forest artifacts, launch a daemon, or restore machine code.

Automatic daemon startup, backend selection, admission policy, persistence,
restoration, eviction, Cargo configuration, source changes, profile changes,
LLVM or LTO reuse, release use, or remote transport require upstream ownership,
held-out evidence, rollback, and human approval and are not automatic
recommendations.

## Partial dependency compilation vocabulary

Partial-dependency evidence distinguishes:

1. **Declared dependency surface:** public non-generic functions, generic
   definitions, traits, impls, macros, statics, private items, generated items,
   and exported metadata available from one dependency.
2. **Consumer demand:** items referenced, instantiated, dynamically reached,
   selected for a vtable, exported again, or retained for one consumer target.
3. **Metadata demand:** namespace, signature, trait, macro, layout, inline-body,
   and related metadata actually decoded by a consumer.
4. **Whole-crate frontend work:** parsing, expansion, resolution, HIR lowering,
   type checking, trait solving, borrow checking, and MIR construction that
   still runs for the dependency.
5. **Hint eligibility:** exact toolchain, Cargo unstable feature, rustc support,
   crate type, profile, optimization, inline policy, target, and item shape
   under which codegen may be deferred.
6. **Dependency-owned codegen:** mono items, backend work, objects, and archive
   bytes emitted while compiling the dependency.
7. **Consumer-owned codegen:** dependency bodies emitted in a downstream
   consumer after deferral.
8. **Repeated consumer emission:** the same eligible body emitted by multiple
   binaries, libraries, tests, examples, benches, profiles, or targets.
9. **Already-lazy generic:** a generic definition instantiated only for
   concrete demand without relying on the partial-compilation hint.
10. **Already-unreachable private item:** private code excluded from ordinary
    codegen because no reachable root uses it.
11. **Codegen slicing:** dependency codegen deferred to consumers while
    whole-crate frontend correctness remains in the dependency compilation.
12. **Full crate slicing:** proposed compiler-owned deferral of reachable
    frontend, MIR, and codegen work from dependency compilation into a later
    root compilation.
13. **Whole-crate correctness control:** an unused-body type error, coherence
    obligation, macro or generated-code requirement, dynamic-dispatch case, or
    other semantic condition that must not disappear silently.
14. **Final retention:** object members and symbols selected, folded, stripped,
    or retained after archive and linker processing.
15. **Net partial-compilation benefit:** avoided dependency work minus consumer
    codegen, duplicated consumer work, hint overhead, artifact handling,
    linking, runtime, and validation costs.

Partial-dependency reports record the exact nightly rustc and Cargo versions,
unstable feature, dependency and consumer revisions, target, profile,
optimization, inline attributes, crate types, features, consumer count, target
roots, clean or incremental state, and command.

Sparse, dense, generic, private, multi-consumer, release, inline-policy, and
whole-crate-error cases are separate. Reports do not infer sparse use from
crate size, public item count, rlib bytes, or one consumer.

Primary outcomes include complete-build wall and CPU time, peak memory,
dependency and consumer artifact bytes, mono-item ownership, repeated emission,
final binary size, runtime where relevant, exit status, and output identity.
Rlib shrinkage alone is not accepted as an optimization result.

Minimally instrumented complete builds remain primary. Mono-item output and
self profiles are optional unstable diagnostics used to distinguish frontend,
dependency-codegen, and consumer-codegen causes.

Current `hint-mostly-unused` evidence is described as codegen slicing. It must
not be generalized to skipped parsing, type checking, borrow checking, MIR,
coherence, macro expansion, generated code, dynamic dispatch, or diagnostics
unless those properties are separately demonstrated by an upstream compiler
implementation.

A Build Forest may record dependency surfaces, consumer demand,
hint eligibility, frontend work, dependency-owned and consumer-owned codegen,
duplication, final retention, and experiment outcomes. It must not rewrite
profiles, transform source, construct stub rlibs, consume compiler-private
metadata as a stable format, or skip whole-crate correctness work.

Automatic hint adoption, manifest or source changes, release use, stub-rlib or
frontend slicing, compiler forks, and production nightly dependence require
upstream ownership, held-out public evidence, cross-platform validation,
rollback, and human approval and are not automatic recommendations.

## Workspace modularization and crate-boundary vocabulary

Crate-boundary evidence distinguishes:

1. **Logical boundary:** modules, visibility, ownership, and concepts inside
   one crate.
2. **Compilation boundary:** one rustc crate invocation and its metadata,
   archive, incremental state, and codegen ownership.
3. **Package boundary:** one Cargo package with manifest, features, targets,
   tests, publication, and semver identity.
4. **Workspace boundary:** packages coordinated by one root, lockfile, target
   directory, and command selection.
5. **Parallel width:** independent ready crates Cargo can compile concurrently.
6. **Serial depth:** dependency edges that force crate completion order.
7. **Edit containment:** work avoided inside the edited crate and across its
   dependent cone.
8. **Downstream fan-out:** dependent crates Cargo invokes after the edited
   crate changes.
9. **Invocation multiplication:** additional rustc startup, sysroot, metadata,
   hashing, archive, and process work.
10. **Generic ownership:** the crate that collects and emits each concrete
    generic instance.
11. **Target multiplication:** additional library, binary, test, example,
    benchmark, build-script, and documentation targets.
12. **Link multiplication:** additional archives, executables, test harnesses,
    and final links.
13. **Boundary stability:** how often both sides change together and whether
    the cross-crate interface is narrower than the implementation.
14. **Non-performance boundary:** independent reuse, publication, semver,
    features, platform, capability, unsafe review, ownership, security, or
    operational reasons for separation.
15. **Counterfactual topology:** a reversible synthetic or disposable
    alternative used for measurement rather than an owner-worktree patch.

Modularization reports record the exact source and target placement, toolchain,
profile, features, target, linker, jobs, crate graph, package and target count,
dependency depth, ready width, edit location, changed crate, dependent cone,
generic families, test targets, and non-performance constraints.

Required workloads include clean build, warm no-op, private local edit,
foundation edit, revert, and `cargo test --no-run`. Applicable reports also
include check, release, proc-macro, build-script, native dependency, feature,
and cross-target controls.

Primary outcomes include wall, CPU, memory, variance, rustc invocation count,
compiled and fresh artifacts, metadata and target bytes, generic-instance
ownership, link and test target count, output behavior, and validation
coverage. A clean wall-time improvement alone is not sufficient when CPU,
tests, storage, or dependent fan-out regress.

Current intra-crate incremental reuse is measured before attributing avoided
work to a proposed crate split. Current behavior and any future
Relink-Don't-Rebuild counterfactual remain separate.

A Build Forest may record boundary type, graph position, edit frequency,
dependent fan-out, current reuse, generic ownership, target multiplication,
counterfactual outcomes, non-performance constraints, and confidence. It must
not move source, rewrite manifests, change APIs, or split or combine packages.

Automatic source movement, crate splitting or combining, manifest, feature,
package, public-API, semver, ownership, unsafe-boundary, or validation changes
require owner-led design, held-out evidence, rollback, and human approval and
are not automatic recommendations.

## Debug information and native-emission vocabulary

Debug and emission evidence distinguishes:

1. **Requested debug level:** none, line directives, line tables, limited, or
   full as requested by rustc, Cargo profile, environment, or tool.
2. **Effective debug level:** the source, line, procedure, local, type, scope,
   and expression records actually emitted for the target and backend.
3. **Profile origin:** built-in Cargo profile, manifest profile, environment
   override, command flag, target policy, or wrapper.
4. **Debug format:** CodeView/PDB, DWARF, dSYM, DWO/DWP, platform directives,
   or another target representation.
5. **Debug construction:** rustc work that creates source, location, scope,
   procedure, variable, and type metadata during IR translation.
6. **Backend debug processing:** optimization, preservation, lowering,
   relocation, machine-code, and emission work influenced by debug metadata.
7. **Object emission:** target object creation including code, data, unwind,
   relocations, symbols, debug sections, padding, and file writing.
8. **Named debug-section bytes:** bytes in format-specific sections such as
   `.debug$S`, `.debug$T`, or DWARF sections. They are not total
   debug-induced object bytes.
9. **Total object bytes:** every emitted native object byte, including
   non-debug content and debug-induced relocation, symbol, layout, or padding
   changes.
10. **Archive bytes:** object, metadata, and archive-container bytes retained
    in an Rlib, static library, or equivalent intermediate.
11. **Incremental debug bytes:** backend work products, dep-graph state, query
    results, fingerprints, and related storage whose size changes with debug
    policy.
12. **Split-debug mode:** packed, unpacked, off, or another target-specific
    mode, including whether it is stable and supported.
13. **Packaged debug output:** PDB, dSYM, DWP, retained object, executable
    debug section, or another artifact produced for debugger consumption.
14. **Linker debug input:** CodeView, DWARF, symbols, objects, libraries, and
    native records consumed while producing the final image and packaged
    debug output.
15. **PDB baseline:** PDB bytes present without current-crate debug records
    because dependencies, native runtime inputs, public symbols, and linker
    records remain. It is not attributed to the current crate.
16. **Matched debug delta:** artifact or latency difference between otherwise
    identical debug policies. It is preferred over total PDB attribution.
17. **Strip policy:** final-link removal of debug or symbols. It is not assumed
    to suppress earlier debug construction or object emission.
18. **Saved temporary policy:** retained IR, bitcode, objects, split-debug
    files, or linker intermediates with explicit observer and storage effects.
19. **Debugger capability:** source stepping, line backtraces, locals, types,
    expressions, optimized frames, panic/unwind, crash, mixed-language, and
    remote-debug behavior required by the consumer.
20. **Debug capability contract:** the explicit set of source-location, local,
    type, profiling-symbol, crash-symbol, panic/unwind, mixed-language, and
    remote-debug capabilities required by one workflow.
21. **Debug rollback:** removal of an optional alternative profile and its
    isolated artifacts without source, manifest, shared-cache, or validation
    recovery.

Object-only, archive, complete-link, packaged-debug, executable, incremental,
and interactive-debugger evidence remain separate. A complete rustc command is
not called a linker timer, and object-only duration is not subtracted from a
separate link duration to manufacture one.

Requested debug labels do not guarantee distinct effective output on every
target. Reports inspect emitted records and artifacts and preserve rustc,
backend, target, object format, linker, CGUs, optimization, incremental state,
panic, strip, split mode, and profile origin.

Reports start from the workflow's debug capability contract rather than a
preferred compiler flag. A reduced level is acceptable only when the emitted
records, packaged symbols, retention, artifact identity, and interactive tools
demonstrate every required capability.

Named debug-section bytes are not accepted as total debug cost. Reports also
record total object and archive bytes, relocations, symbol-table effects,
incremental storage, packaged debug output, and complete-command resources.

Total PDB, dSYM, DWP, or executable debug bytes are not automatically
attributed to the current crate. Precompiled dependencies and native inputs
require a matched no-current-crate-debug baseline or per-input inspection.

CGU count can trade backend parallelism against repeated line, file, type,
symbol, relocation, object, and archive bytes. Debug and CGU guidance requires
joined wall, CPU, memory, storage, linker-input, final-size, runtime, and
debugger evidence.

Strip, split-debug, save-temps, and debugger choices are separate controls.
Stripping final output is not evidence that debug construction was avoided.
Record presence is not interactive debugger usability.

Self-profile, time-pass, CodeView, DWARF, PDB, saved-object, and linker
diagnostics are optional observer-affected evidence behind exact-version
adapters. Missing tools, unsupported target modes, schema drift, unknown
records, or unavailable debugger behavior fail closed.

A Build Forest may compare sibling debug-policy roots and record object,
archive, incremental, packaged-symbol, and debugger-capability evidence. It
must preserve debug identity and must not restore or relabel one root's
artifacts or validation claims as another's.

Automatic profile, environment, split-debug, strip, CGU, backend, linker,
source, CI, editor, artifact-sharing, or validation changes can exchange
latency, CPU, memory, storage, runtime, diagnostics, crash analysis, ABI,
reproducibility, and support burden. They require held-out evidence, explicit
rollback, human approval, and are not automatic recommendations.

## Linking and incremental-link-state vocabulary

Link evidence distinguishes:

1. **Link capability contract:** target and ABI compatibility, debug and symbol
   packaging, edit-to-runnable latency, release optimization and finalization,
   reproducibility, native-library and mixed-language support, signing,
   deployment, and rollback required by one workflow.
2. **Link input identity:** exact native objects, archives, dynamic and static
   libraries, exports, resources, response files, search paths, target,
   subsystem, ABI, panic/unwind, debug, LTO, and linker options.
3. **Link plan:** ordered inputs, engine, options, environment, working
   directory, output paths, debug packaging, and validation expected for one
   final image.
4. **Complete link:** a linker operation that constructs the output from the
   complete accepted input set without reusable prior link state.
5. **Incremental preparation:** a link that produces a padded or otherwise
   prepared image and engine-specific reusable state for later relinks.
6. **Incremental state identity:** linker version, target, link options, input
   set, output image, timestamps or content identity, debug output, and state
   artifact such as MSVC ILK.
7. **Incremental request:** a requested flag or mode. It is not evidence that
   an incremental link occurred.
8. **Incremental eligibility:** all engine-specific policy and state
   preconditions required before reuse can occur.
9. **State disposition:** prepared, reused, partially reused, invalidated,
   missing, corrupt, incompatible, rejected, or unknown.
10. **Changed module set:** exact retained, changed, added, and removed object
    or linker-module identities observed by the engine.
11. **Fallback reason:** optimization incompatibility, missing output or state,
    changed timestamp, changed option, added or omitted input, excessive
    changed modules, corruption, unsupported engine, or unknown.
12. **Linker optimization policy:** dead-code elimination, identical-code
    folding, branch optimization, LTO, padding, thunks, and other policies that
    can affect both incrementality and final image behavior.
13. **Link output set:** executable or library, PDB/DWARF/dSYM or symbols,
    ILK or other state, maps, imports, exports, manifests, resources, and
    signatures with separate byte and identity records.
14. **Release finalization:** required non-incremental optimization, debug
    packaging, reproducibility, signing, scanning, smoke, ABI, runtime,
    deployment, and rollback validation for the deliverable artifact.
15. **Link rollback:** removal of optional linker configuration and isolated
    prepared state without source, shared-cache, output, signing, deployment,
    or validation recovery.

Object emission, link-plan construction, complete linking, incremental
preparation, state reuse, fallback, debug packaging, and final validation
remain separate regions. A complete rustc command is not a native linker timer.

Reports record the effective engine and outcome rather than only requested
flags. They preserve engine and version, target, object format, input set,
response file, environment, `/OPT` or equivalent policy, debug capability,
incremental state, changed modules, output bytes, diagnostics, and release
identity.

Incremental-link claims require an engine diagnostic or equivalent artifact
evidence that state was reused. Missing, corrupt, incompatible, rejected, and
full-link fallback states remain visible.

Compiler CGU partition and object naming are part of linker reuse eligibility.
Reports compare object path and content identities across the controlled edit;
an unchanged source-level scope is not assumed to preserve linker modules.

Prepared development and final release outputs are distinct artifact and
validation identities. Larger padded images, jump thunks, disabled dead-code
elimination, PDB changes, and linker-state storage must be reported beside
latency.

No-link/link-only bundles, linker response files, ILK files, and engine-private
state are compatibility-bound diagnostic surfaces unless an upstream stable
contract states otherwise. They are not portable cache entries.

A Build Forest may compare complete, prepared-development, and final-release
roots. It must preserve input sets, linker state, outputs, debug packages,
signatures, validation, and rollback dispositions separately.

Automatic linker, profile, environment, optimization, CGU, source, CI, editor,
state-retention, signing, deployment, release, artifact-sharing, or validation
changes can exchange latency, CPU, memory, storage, runtime, ABI, diagnostics,
reproducibility, security, and support burden. They require held-out evidence,
explicit rollback, human approval, and are not automatic recommendations.

## Name resolution and HIR vocabulary

Resolution and lowering evidence distinguishes:

1. **Reduced graph:** modules, definitions, macro scopes, imports, and
   expansion fragments planted before the crate is fully expanded.
2. **Import kind:** named, renamed, glob, public re-export, extern crate,
   prelude, macro-generated, or implicit.
3. **Import dependency:** an import whose resolution depends on names made
   available by another import or expansion.
4. **Fixed-point batch:** one pass over currently indeterminate imports before
   newly determined bindings are committed.
5. **Dependency depth:** the longest observed chain of import or re-export
   availability.
6. **Propagated bindings:** names copied or made visible through imports and
   re-exports across modules. This is distinct from source item count.
7. **Ambiguity:** multiple candidates, shadowing, indeterminate resolution,
   privacy, and diagnostics preserved until finalization.
8. **Effective visibility:** the crate-wide public reachability and re-export
   state computed after imports are finalized.
9. **Late resolution:** paths, types, expressions, patterns, locals,
   lifetimes, labels, generic scopes, and ribs resolved over the fully
   expanded AST.
10. **AST owner:** a crate, item, trait item, impl item, foreign item, nested
    use tree, or other definition indexed for lowering.
11. **HIR owner:** one definition and its locally indexed HIR nodes, bodies,
    attributes, parenting, and trait candidates.
12. **Local HIR node:** a node identified within one owner. Local-node count
    and owner count are separate cost dimensions.
13. **Owner hash:** stable fingerprints for owner nodes and bodies, plus
    separately projected attributes and opaque definitions.
14. **Edit class:** body, signature, import, visibility, module, re-export,
    macro output, attribute, or identical rewrite.

Source bytes, line count, item count, import count, path count, HIR record
count, and module count are not accepted as standalone resolution or lowering
estimates. Reports preserve import kind, dependency depth, propagated
bindings, effective visibility fanout, path and scope shape, owner count, and
local nodes per owner.

Stable complete compilation remains primary. Nightly root parse, no-analysis,
time passes, self-profile, input stats, query events, and dep-graph diagnostics
remain separate observer-affected evidence.

The no-analysis boundary is not a pure expansion timer. In the current
frontend, `configure_and_expand` completes crate resolution before later
analysis is skipped. Reports therefore preserve import finalization,
effective visibility, late resolution, and aggregate resolution when using
that boundary.

`resolver_for_lowering_raw`, `index_ast`, and `lower_to_hir` are not assumed
to persist frontend products across compiler invocations. A reused
incremental directory must demonstrate which queries reran, which owner
results compared equal, and which downstream queries reused cached results.

`hir_owner` and projected attribute results are stable-hashed boundaries.
Reconstructing and hashing an unchanged owner is not the same as skipping
lowering, and downstream reuse is not evidence that the AST or HIR was loaded
unchanged from disk.

Frontend job count is recorded with import topology. Parallel work within an
import batch does not imply that dependency batches, effective visibility,
late crate walking, AST indexing, or all owner lowering run in parallel.

Ambiguous glob names, private re-exports, unresolved paths, and changed
visibility retain exit status and diagnostics. A failed-fast result is not a
resolution throughput improvement.

Replacing globs, renaming imports, reordering declarations, changing
visibility, moving modules, or changing macro output can alter ambiguity,
privacy, lints, diagnostics, edition behavior, hygiene, public API, and
downstream invalidation. Such changes require explicit semantic and consumer
validation and are not automatic performance recommendations.

## Type inference and type checking vocabulary

Type-checking evidence distinguishes:

1. **Item collection:** signature, generics, predicates, and top-level type
   information collected without checking function innards.
2. **Item well-formedness:** crate and per-item checks represented by
   `check_type_wf` and `check_well_formed`.
3. **Type-check root:** one function, const, static, or enclosing root whose
   nested closures and inline bodies share an inference environment.
4. **Inference variable:** an unresolved type, integer, float, region-adjacent,
   or const variable created while checking a body.
5. **Expected type:** a type propagated from an annotation, return position,
   call argument, branch, pattern, or surrounding expression.
6. **Equality and subtype constraint:** relationships unified or related
   inside the root inference context.
7. **Generic argument inference:** type or const arguments inferred at a
   generic use site.
8. **Coercion:** adjustments between expression and expected types, including
   function-item, reference, pointer, unsizing, branch, and return coercions.
9. **Coercion accumulator:** least-upper-bound work across branch, match,
   break, or return expressions.
10. **Fallback:** final integer, float, diverging, or other defaulting after
    expression constraints are collected.
11. **Deferred check:** cast, repeat, closure, coroutine, sized, transmute, or
    assembly work completed after initial expression traversal.
12. **Trait-obligation boundary:** predicates registered and selected during
    type checking. This work is interleaved with inference and belongs to
    PERF-Q13 when trait topology is the independent variable.
13. **Writeback:** replacing inference variables with resolved types and
    recording node types, generic arguments, adjustments, captures,
    coercions, and hidden types.
14. **Type-check result:** one stable-hashed `TypeckResults` value for a root,
    eligible for on-disk incremental reuse.
15. **Owner width:** the number and distribution of independently schedulable
    type-check roots.
16. **Type edit class:** body, annotation, expected type, helper body,
    signature, alias, generic predicate, coercion target, pattern, closure, or
    identical rewrite.

Source bytes, expression count, annotation count, generic call count, HIR
records, inference variables, owner count, and obligation count are not
accepted as standalone type-check estimates. Reports preserve body-local
shape, owner distribution, expected types, coercions, patterns, fallback,
trait confounds, and shared type dependencies.

Stable complete compilation remains primary. No-analysis excludes type
checking but is not subtracted and relabeled as inference time. Time passes,
self-profile, input stats, debug logs, query events, and incremental cache
statistics remain separate observer-affected evidence.

`type_check_crate` is a crate-level region. `typeck_root` is the body-owner
query boundary and may include expression checking, inference, obligation
selection, coercion, fallback, closure analysis, and writeback. A
`typeck_root` event is not a pure unification or trait-solving timer.

`typeck_root` is cacheable on disk. Reused incremental measurements record
provider execution, cache hits and misses, result loading, changed owner,
shared signature or alias dependencies, and downstream MIR or borrow work.
No event for an unchanged owner is not evidence that the complete compiler
did no work.

Frontend job count is recorded with owner width. More owners can expose
parallel body checking while adding query, HIR, MIR, borrow-check, metadata,
and maintenance overhead. Automatic function splitting is not a valid
inference optimization.

Unconstrained generic values, untyped closures, incompatible branches,
coercion failures, fallback changes, and ambiguity retain exit status and
diagnostics. A failed-fast result is not type-check throughput improvement.

Adding annotations, changing aliases, modifying generic signatures, replacing
coercions, changing patterns, splitting functions, or simplifying trait bounds
can alter inference, fallback, diagnostics, borrow behavior, public API, code
generation, and downstream invalidation. Such changes require explicit
semantic and consumer validation and are not automatic performance
recommendations.

## Trait-solving cost and reuse vocabulary

Trait-solving evidence distinguishes:

1. **Solver mode:** old everywhere, next solver for coherence only, or next
   solver globally. The exact compiler revision and effective option are part
   of benchmark identity.
2. **Goal:** one predicate evaluated in a parameter environment.
3. **Canonical goal:** a goal normalized over inference variables for solver
   evaluation and eligible cache reuse.
4. **Goal identity:** repeated-equivalent, unique concrete, generic,
   projection, normalization, method, coherence, or recursive goal shape.
5. **Candidate set:** trait implementations or method candidates considered
   before applicability is resolved.
6. **Candidate width:** the number of relevant same-name or same-trait
   candidates visible at a goal site.
7. **Applicable candidate count:** candidates that remain viable after
   receiver, type, predicate, and environment constraints.
8. **Call count:** method or predicate use sites evaluated for a controlled
   candidate topology.
9. **Supertrait depth:** the length and branching of inherited trait
   obligations.
10. **Projection depth:** nested associated-type normalization depth and
    branching.
11. **Solver reuse:** in-process or query-level cache access for an equivalent
    canonical goal. Solver architectures can expose this through different
    events.
12. **Provisional cycle reuse:** temporary search-graph results used while
    evaluating recursive goals toward a fixpoint.
13. **Stalled goal:** an obligation deferred until inference progress can
    change its result.
14. **Query visibility:** which solver work appears as a named query and which
    remains folded into `typeck_root`, method probing, fulfillment, or another
    event.
15. **Trait edit class:** caller body, impl body, impl header, trait definition,
    impl-set membership, shared bound, associated type, or identical rewrite.
16. **Impl-set invalidation:** recomputation caused by adding, removing, or
    changing an implementation even when the edited impl is not selected for
    the controlled concrete goal.

Trait count, impl count, source bytes, obligation count, cache hits, candidate
width, call count, supertrait depth, and projection depth are not accepted as
standalone solver-cost estimates. Reports preserve goal identity, solver mode,
parameter environment, candidate applicability, owner topology, and edit
class.

Stable complete compilation remains primary. No-analysis is not subtracted and
relabeled as trait-solving time. Time passes, self-profile, proof-tree, debug
logs, query events, cache statistics, and incremental dep-graph evidence
remain separate observer-affected diagnostics.

Old-solver `evaluate_obligation` and new-solver proof-tree or search-graph
events are not equivalent complete-cost timers. `typeck_root` can include
method lookup, candidate assembly, normalization, fulfillment, inference, and
solver cache access that is absent from the standalone event's self time.

Incremental measurements record whether unchanged body `TypeckResults` load,
which solver or impl-set queries re-evaluate, and whether caller roots miss
after body, impl-body, impl-header, impl-set, or shared-bound edits. A query
miss without owner misses and owner misses without a corresponding standalone
solver query are both valid architectural outcomes.

Unsatisfied bounds, ambiguity, no solution, recursive overflow, normalization
failure, and solver divergence retain exit status and diagnostics. Failed-fast
latency is not solver throughput.

Changing trait bounds, splitting or merging traits, renaming methods, replacing
associated types, changing imports, selecting an unstable solver mode, or
rewriting generic APIs can alter coherence, resolution, inference, diagnostics,
public API, code generation, semver behavior, and downstream invalidation.
Such changes require explicit semantic and consumer validation and are not
automatic performance recommendations.

## Borrow-checking cost and incrementality vocabulary

Borrow-checking evidence distinguishes:

1. **Borrow-check root:** one type-check root whose nested closures and
   coroutines are coordinated by one `mir_borrowck` evaluation.
2. **Promoted MIR:** the MIR body and promoted constants supplied to borrow
   checking after construction and adjacent checks.
3. **Move path:** one local or projected place tracked for moves,
   initialization, and descendant state.
4. **Move event:** move-out, initialization, reassignment, storage death,
   partial move, or related access attached to a MIR location.
5. **Loan:** one borrow with reservation location, optional two-phase
   activation, kind, region, borrowed place, and assigned place.
6. **Loan lifetime:** the MIR points over which a loan remains live.
7. **Active loan overlap:** the number and topology of loans simultaneously
   relevant to accesses or dataflow state.
8. **Place projection:** fields, dereferences, variants, indexes, subslices,
   casts, unions, and other components compared for conflict.
9. **MIR type-check constraint:** liveness, outlives, type-test, universe,
   placeholder, or closure requirement generated while borrow-checking MIR.
10. **Region graph:** region variables, liveness points, outlives edges,
    strongly connected components, inferred values, and universal relations.
11. **Borrow dataflow:** iterative `Borrows`, `MaybeUninitializedPlaces`, and
    `EverInitializedPlaces` analyses over the MIR CFG.
12. **CFG topology:** statement and local volume plus blocks, joins,
    backedges, switch fanout, cleanup edges, and yields.
13. **Nested body topology:** closure, coroutine, capture, await, yield,
    opaque-type, and propagated requirement shape inside one root.
14. **Borrow-query time:** `mir_borrowck` self time separated from total time,
    nested dependencies, broad `MIR_borrow_checking`, and complete wall time.
15. **Polonius mode:** ordinary NLL/off, legacy fact-based analysis, or
    experimental next implementation, with exact compiler revision.
16. **Borrow edit class:** untouched, identical rewrite, caller body, helper
    body, signature, shared type, nested body, or ownership/lifetime change.

Source bytes, `&` count, loan count, move count, owner count, projection depth,
region count, block count, and await count are not accepted as standalone
borrow-check cost estimates. Reports preserve live ranges, active overlap,
move and place topology, region constraints, CFG, nested bodies, mode, and
neighboring compiler phases.

Stable complete compilation remains primary. No-analysis is not subtracted and
relabeled as borrow-check time. Time passes, self-profile, MIR dumps, dataflow
graphs, NLL facts, Polonius facts, query events, and incremental dep-graph
evidence remain separate observer-affected diagnostics.

`thir_body`, `mir_built`, `mir_promoted`, `mir_borrowck`, and
`optimized_mir` are separate boundaries. `mir_borrowck` self time is not its
total nested query time, and the broad `MIR_borrow_checking` pass is not one
owner's provider time.

`mir_borrowck` is not `cache_on_disk`. Incremental measurements record whether
the dependency graph skips a green provider and which body, MIR, shared type,
signature, or nested-body changes cause misses. No event for an unchanged
root is not evidence that rustc loaded a serialized borrow set or region
solution.

Use-after-move, conflicting access, invalid return lifetime, borrow-across-
yield, closure capture, and drop errors retain exit status and diagnostics.
Failed-fast latency is not successful borrow-check throughput.

Shortening borrows, inserting scopes, cloning, changing ownership or
lifetimes, adding interior mutability, splitting functions, extracting
closures, changing async shape, adding `unsafe`, or selecting experimental
Polonius modes can alter safety, semantics, destruction order, concurrency,
memory, runtime cost, public API, and diagnostics. Such changes require
explicit behavioral and consumer validation and are not automatic performance
recommendations.

## MIR construction and optimization vocabulary

MIR evidence distinguishes:

1. **MIR owner:** one function, const, static, closure, coroutine, shim, or
   promoted body with its own MIR lifecycle.
2. **Built MIR:** the `mir_built` result produced by THIR-to-MIR lowering.
3. **Promoted body:** a compiler-created MIR body extracted for constant
   promotion and handled separately from the main body.
4. **Runtime-ready MIR:** MIR after borrow checking, cleanup, drop elaboration,
   unwind handling, and other required semantic transforms.
5. **CTFE MIR:** `mir_for_ctfe` or promoted CTFE MIR prepared for interpreter
   execution without ordinary runtime MIR optimization.
6. **Optimized MIR:** disk-cached codegen-ready MIR after the configured
   optional and required pass schedule.
7. **Steal boundary:** ownership transfer of a large THIR or MIR value between
   queries without cloning the main body.
8. **Pass traversal:** one named `mir_pass_*` walk or analysis over a body,
   whether or not the pass changes that body.
9. **Pass schedule:** ordered required and optional transformations selected
   by compiler revision, optimization level, profile, attributes, incremental
   mode, and explicit unstable overrides.
10. **Body topology:** locals, statements, basic blocks, edges, switches,
    cleanups, projections, calls, drops, yields, and generated-body count.
11. **Drop topology:** move paths, drop-needing places, partial initialization,
    conditional drops, unwind edges, drop flags, and drop shims.
12. **Inlining topology:** caller sites, candidate bodies, thresholds,
    recursion/depth limits, accepted expansions, expanded size, and later-pass
    input.
13. **Coroutine topology:** suspension points, saved locals, simultaneous
    liveness, storage conflicts, borrows, state dispatch, and sync/async drop
    shims.
14. **Optimization level:** explicit MIR level 0 through 4, kept separate from
    LLVM `-Copt-level` and complete check/debug/release workflows.
15. **MIR edit class:** untouched, identical rewrite, owner body, shared const
    or type, inline callee, promoted dependency, coroutine dependency, or
    pass-policy change.
16. **Observer mode:** stable metadata, no-analysis, binary encoded MIR,
    textual MIR dump, time passes, self-profile, validation, or complete
    backend workflow.

Source bytes, owner count, statement count, block count, call count, move
count, promotion count, await count, pass self time, and encoded output size
are not accepted as standalone MIR-cost estimates. Reports preserve body
topology, pass policy, required versus optional work, generated bodies,
optimization level, incremental mode, edit frontier, output mode, and
neighboring phases.

Stable complete compilation remains primary. `-Zalways-encode-mir`,
`-Zmir-opt-level`, `-Zmir-enable-passes`, `-Zvalidate-mir`, MIR dumps,
time passes, self-profile, and incremental query events are separately labeled
nightly diagnostics. Textual MIR formatting and binary encoding are output
work, not pass time.

`thir_body`, `mir_built`, `mir_promoted`,
`mir_drops_elaborated_and_const_checked`, `mir_for_ctfe`, `promoted_mir`, and
`optimized_mir` are separate boundaries. Query self time is not the sum of
nested pass activities. A pass event is not evidence that the pass changed the
body.

`optimized_mir`, `mir_for_ctfe`, and `promoted_mir` can be cached on disk.
Incremental measurements record dependency-graph reuse, query-result loading,
provider hits and misses, profile and inlining policy, and the exact edit
frontier. No provider event for a green owner is not evidence that every
intermediate analysis result was serialized.

CTFE interpretation, borrow checking, monomorphization, LLVM optimization,
object/debug emission, and linking remain separate from MIR construction and
optimization. Nested query totals may overlap and are not added.

Changing function boundaries, match structure, aggregates, const placement,
drop order, async shape, inlining attributes, MIR optimization levels,
validation, panic strategy, or pass policy can alter semantics, diagnostics,
runtime performance, code size, incremental reuse, or compiler correctness.
Such changes require dedicated behavioral and consumer validation and are not
automatic performance recommendations.

## Frontend parallelism vocabulary

Frontend parallelism evidence distinguishes:

1. **Frontend job:** one configured rustc frontend worker capacity, separate
   from Cargo process jobs and backend codegen jobs.
2. **Serial compiler mode:** the default non-parallel implementation used when
   the dynamically thread-safe frontend is not selected.
3. **Parallel one-job mode:** the dynamically thread-safe implementation and
   worker-pool path configured with one frontend job.
4. **Schedulable owner:** one function, const, static, closure, coroutine, item,
   or module unit that a compiler parallel iterator can dispatch
   independently.
5. **Owner granularity:** schedulable owner count and work per owner, including
   shared dependencies and skew between owners.
6. **Serial region:** parsing, expansion, resolution, global coordination, or
   another interval that does not expose useful concurrent work in the tested
   revision.
7. **Response curve:** wall, CPU, memory, and output observations across at
   least two frontend-job counts.
8. **Break-even width:** the smallest tested owner/work shape whose parallel
   wall improvement exceeds variance and resource-policy thresholds.
9. **Diminishing-return point:** the tested job count after which added workers
   provide little or negative wall improvement.
10. **Jobserver domain:** one inherited or locally created token-coordination
    tree shared by cooperating Cargo and rustc processes.
11. **Independent session:** a top-level terminal, worktree, editor, CI helper,
    or AI-agent build with a separate Cargo process, target context, and
    jobserver domain.
12. **Machine-session pressure:** aggregate makespan, CPU, memory, runnable
    work, artifact locking, and foreground responsiveness across independent
    sessions.
13. **Diagnostic order:** the observed presentation order of complete
    diagnostics, kept separate from error count, content, exit status, and
    semantic correctness.
14. **Parallel observer effect:** instrumentation synchronization, event
    overlap, allocation, or serialization that changes the minimally
    instrumented response curve.

Frontend-job count, logical cores, source bytes, owner count, query-event
duration, and fastest single-build wall time are not accepted as standalone
parallelism recommendations. Reports preserve compiler mode, Cargo jobs,
backend jobs, cache state, owner topology, serial regions, jobserver domain,
independent-session count, CPU, memory, output identity, diagnostics, and
observer mode.

Stable/default Cargo and rustc workflows remain primary.
`--jobs-frontend`, self-profile, and compiler-internal scheduling events are
nightly compatibility-bound evidence until their upstream interfaces and
correctness requirements stabilize.

One Cargo jobserver coordinates one inherited process tree. It is not evidence
of a machine-global budget across independent top-level sessions. Session
experiments record target-directory identity because shared targets can wait
or coalesce while isolated targets can duplicate work and memory.

Parallel query events overlap and must not be summed into elapsed time.
Self-profile is calibrated against equivalent minimally instrumented runs.
Successful output hashes, incremental provider frontiers, exit status,
diagnostic completeness and order, hangs, timeouts, and ICEs remain visible.

Changing frontend jobs, Cargo jobs, process priority, target-directory layout,
function or module boundaries, session concurrency, cancellation, or memory
limits can alter latency, throughput, diagnostics, artifact reuse, editor
responsiveness, and machine stability. Such changes require explicit consumer
and operational validation and are not automatic performance recommendations.

## Query invalidation vocabulary

Incremental query evidence distinguishes:

1. **Untouched baseline:** an unchanged compile using the same toolchain,
   output mode, flags, and incremental cache topology as the edit run.
2. **Edit frontier:** the smallest owner, query, artifact, or process set whose
   current result may differ because of the controlled edit.
3. **Provider execution:** a query provider ran in the current compiler
   session; this does not alone prove that its result changed.
4. **Green result:** the current provider result matched the prior stable
   fingerprint, allowing downstream reuse.
5. **Red result:** the current provider result differed from the prior stable
   fingerprint and can invalidate dependent work.
6. **Downstream containment:** the furthest compiler or artifact stage that
   actually executed after a provider re-ran.
7. **Persisted result:** a result or fingerprint available across compiler
   sessions through the incremental cache.
8. **Session work:** provider, decoding, graph, metadata, or coordination work
   that also executes in an untouched compile or is not persisted.
9. **Semantic propagation:** re-execution required because a body, type,
   trait candidate set, constant identity, optimization input, or other Rust
   contract changed.
10. **Diagnostic dependency:** work required or conservatively tracked because
    lint levels, spans, errors, warnings, suggestions, or another observable
    diagnostic can change.
11. **Source-layout movement:** an edit changes byte, line, column, or span
    positions of otherwise reusable owners.
12. **Moved-owner count:** the number of existing owners whose source range
    changed after the edit.
13. **Fixed-offset control:** a paired edit with equal source length and stable
    following owner offsets, used to separate meaning from movement.
14. **Insertion-position control:** the same inserted item or text placed
    before versus after existing owners.
15. **Output sensitivity:** whether metadata, check, debug, full-debuginfo,
    coverage, documentation, or another output mode requires different
    source-facing work.
16. **Impl-set frontier:** owners whose trait solving is invalidated by a
    same-trait or unrelated-trait implementation change under a named solver.
17. **Span-ignore control:** testing-only use of
    `-Zincremental-ignore-spans=yes` to identify span-hashing causality, never
    a production recommendation.

Reports do not equate cache misses, provider invocations, self-profile event
counts, red results, or machine-code regeneration. They preserve the sequence:

```text
source edit
  -> changed or moved owner
  -> provider executed or reused
  -> result red or green when observable
  -> downstream provider frontier
  -> artifact and validation outcome
```

Every broad invalidation claim requires:

1. an untouched baseline;
2. one semantic negative or local control;
3. a moved-offset versus fixed-offset control when source positions differ;
4. provider counts for the suspected frontier and at least one downstream
   stage;
5. output mode, lint policy, solver mode, and debug/coverage state;
6. failure or diagnostic evidence when observable output may justify breadth;
7. minimally instrumented wall distributions before a latency claim.

Provider work that also appears in the untouched baseline is classified as
session or non-persisted work unless the edit measurably changes its count,
duration, result, or downstream effect.

Source spans support diagnostics, debuginfo, coverage, metadata,
documentation, and other observable behavior. A fixed-offset or span-ignore
control can identify layout sensitivity but cannot prove that removing span
dependencies is correct or beneficial.

Changing source order, adding padding, reserving byte slots, suppressing lint
dependencies, ignoring spans, selecting a trait solver, reading compiler cache
internals, or changing query edges can alter correctness, diagnostics,
artifacts, reproducibility, and maintenance cost. Such changes require
dedicated compiler and consumer validation and are not automatic performance
recommendations.

## Incremental cache economics vocabulary

Incremental cache evidence distinguishes:

1. **Incremental disabled:** rustc receives no incremental directory and pays
   no cross-session graph, result-cache, or work-product persistence cost.
2. **Incremental cold:** rustc receives an empty incremental directory and
   creates reusable state without prior-session reuse.
3. **Warm unchanged:** rustc receives a valid prior generation and unchanged
   source; Cargo freshness must be reported separately because Cargo may skip
   rustc entirely.
4. **Warm local edit:** a controlled edit changes a bounded owner or codegen
   frontier while preserving a reusable remainder.
5. **Warm broad edit:** a shared type, bound, trait set, optimization input, or
   other contract removes most of the reusable frontier.
6. **Proof cost:** stable hashing, dependency traversal, red-green marking,
   cache lookup, and other work required to establish reuse.
7. **Load cost:** dependency-graph loading, query-cache mapping, query-result
   deserialization, and backend work-product access.
8. **Persistence cost:** current graph encoding, result-cache serialization,
   cache promotion, work-product writing, and session finalization.
9. **Avoided work:** provider, optimization, codegen, emission, or link work
   that an equivalent disabled compile would otherwise execute.
10. **Frontend result reuse:** persisted query values or fingerprints reused
    before backend code generation.
11. **Backend work-product reuse:** compiled codegen units or other backend
    outputs reused independently of frontend query results.
12. **Cache generation:** one working or finalized rustc session directory.
    Working, failed, prior finalized, and current finalized generations remain
    distinct states.
13. **Logical cache bytes:** the sum of directory-entry file lengths,
    including hard-linked content more than once.
14. **Unique cache bytes:** bytes deduplicated by filesystem file identity,
    reported separately from logical size.
15. **Recovery boundary:** the whole isolated incremental directory. Internal
    graph, query-cache, metadata, and work-product files are not independently
    repaired, pruned, copied, or transported.

Incremental benefit is reported as a comparison against an equivalent
disabled compile for the same source state. An unchanged incremental compile
is not compared only with incremental cold, and a local edit is not compared
only with a broad edit.

Source bytes, line count, crate count, and owner count are not accepted as
benefit predictors by themselves. Reports include reusable owner cost, edit
frontier, output mode, backend-work-product policy, graph or query topology
when available, and the amount of state retained.

Cargo freshness precedes rustc incremental analysis. A fresh Cargo artifact
means rustc did not run and did not pay incremental load or persistence cost.
No-op process time is not attributed to rustc cache reuse.

Cache sizes report component, generation, logical, and unique bytes. Tools must
not inspect unstable internal values as a supported interface or delete one
internal file while retaining related graph state.

Cross-mode compiler output bytes may differ because incremental settings are
part of artifact identity. Correctness comparisons use successful behavior and
within-mode artifact stability unless a stronger public equivalence contract
exists.

Configuration guidance remains workload-specific. Cargo development and
release defaults are preserved unless repeated repository-level evidence
supports a reversible override. One-shot CI, active development, local edits,
broad regeneration, storage pressure, and backend codegen are separate
workload classes.

## Early-phase incrementality vocabulary

Early-phase reuse evidence distinguishes:

1. **Reuse unit:** the smallest proposed persisted result, such as one source
   file, syntax fragment, expansion, namespace component, AST owner, or HIR
   owner.
2. **Reconstruction cost:** parsing, expansion, resolution, indexing, lowering,
   and stable hashing repeated before the compiler can compare an output with
   the prior session.
3. **Stable output boundary:** the fingerprinted result that can protect
   downstream queries even when its provider reconstructs.
4. **Coupling frontier:** the edition, cfg, source map, attribute, hygiene,
   expansion-order, namespace, visibility, privacy, diagnostic, or owner state
   capable of invalidating a reuse unit.
5. **Structural parse reuse:** token replacement or bounded syntax reparse that
   shares unchanged tree structure. An IDE implementation is not evidence of
   rustc AST compatibility.
6. **Expansion-query reuse:** persistence for one fully identified macro
   operation. It is distinct from caching the iterative crate expansion loop.
7. **Namespace reuse:** persistence of import, visibility, macro-scope, or
   late-resolution results. Crate-wide resolver output is not assumed to have
   module-local invalidation.
8. **Owner reconstruction:** rebuilding an AST or HIR owner whose stable result
   may compare equal and contain downstream invalidation.
9. **Theoretical saved work:** the measured reconstruction region that a
   hypothetical perfect hit could avoid before paying identity, validation,
   load, hash, and persistence cost.
10. **Compiler query plan:** the planned or observed graph of Cargo units,
    driver passes, compiler queries, cache decisions, backend work, linking,
    validation, dependencies, cost, invalidation, and concurrency.

Reports separate repeated early work from downstream containment. A low count
of type-check, borrow-check, MIR, or codegen misses does not prove parsing,
expansion, resolution, indexing, or lowering was reused.

Candidate measurements include unchanged, identical rewrite, body, import,
visibility, module, macro invocation, macro definition, source-layout,
failure, and broad namespace states as applicable. They preserve diagnostics,
spans, suggestions, hygiene, privacy, cfg, target, toolchain, and stable owner
identity as correctness outputs.

Parallel execution and incremental reuse are reported separately. More
frontend jobs can overlap reconstruction without avoiding it; persistence can
avoid work without creating parallelism.

## Relink-Don't-Rebuild vocabulary

Cross-crate cutoff evidence distinguishes:

1. **Source freshness:** Cargo's decision that source content, declared inputs,
   environment, and configuration require rebuilding the edited graph unit.
   Timestamp or checksum freshness is not an interface decision.
2. **Compiler artifact identity:** the broad rustc and Cargo identity required
   for metadata, diagnostics, incremental state, and artifact compatibility.
3. **Cross-crate interface identity:** the conservative semantic inputs from
   one crate that can affect compilation of an unchanged dependent.
4. **Implementation-only edit:** a change to emitted code or private behavior
   that leaves the cross-crate compilation interface equal.
5. **Exported-body input:** inline, generic, const-evaluable, macro, layout, or
   other body or representation information consumed across the crate
   boundary.
6. **Retained-artifact compatibility:** proof that definition, symbol,
   metadata, target, compiler, profile, feature, and dependency identities
   referenced by an unchanged downstream artifact remain valid.
7. **Direct early cutoff:** pruning compilation of a direct dependent after
   the edited crate rebuilds with an equal effective interface.
8. **Transitive early cutoff:** pruning more distant compilation after an
   intermediate crate rebuilds and its effective interface remains equal.
9. **Downstream compile pruned:** an existing dependent artifact is retained;
   this does not imply that the edited upstream crate or final linker was
   skipped.
10. **Link-input identity:** code, data, native objects, metadata, linker
    options, and other inputs that determine whether a final link is required.
11. **RDR query plan:** the planned or observed sequence of source freshness,
    upstream rebuild, interface comparison, retained-artifact validation,
    downstream execution or pruning, and link execution or pruning.

RDR reports include at least:

- identical-content rewrite;
- comment or formatting edit;
- private non-generic body edit;
- public non-inline body edit;
- inline and generic body edits;
- constant, macro, and layout edits;
- private item insertion and reorder;
- public API addition or signature change; and
- a semantically equivalent public spelling control where practical.

Visibility, runtime equality, current `.rmeta` equality, crate hash equality,
or artifact filename equality is not sufficient reuse proof. A false-negative
cross-crate interface decision is a potential miscompilation.

Reports state separately whether:

- the edited upstream crate rebuilt;
- its cross-crate interface compared equal;
- retained downstream artifacts remained compatible;
- each direct or transitive downstream compile was pruned;
- link inputs changed; and
- the final linker ran.

Private and public non-inline body edits are leading positive fixtures, not
pre-approved implementations. Inline, generic, const, macro, layout,
definition-identity, ABI, target, native-link, proc-macro, build-script, LTO,
and dynamic-linking cases remain correctness boundaries.

## Cross-command reuse vocabulary

Cross-command evidence distinguishes:

1. **Compiler activity:** check, build, Clippy, test, documentation, doctest, or
   another consumer of rustc or rustdoc with its own semantics and outputs.
2. **Target stage:** the furthest compiler stage required by an activity, such
   as analysis, code generation, or linking.
3. **Stage dependency:** a flag, cfg, environment value, tool input, or
   dependency that first affects one named compiler stage. A later-stage option
   must not be assumed to invalidate an earlier stage, and a cfg-sensitive
   option must not be assumed to remain later-stage only.
4. **Exact artifact reuse:** Cargo reports the same unit output fresh.
5. **Compatible dependency reuse:** one activity reuses a dependency artifact
   built by another activity while retaining distinct roots.
6. **Common-stage reuse:** different activities reuse compatible earlier
   compiler work while executing their required later or tool-specific stages.
7. **Tool-specific work:** Clippy lint analysis, rustdoc extraction and
   rendering, doctest generation, or another tool contract that remains
   required after common-stage reuse.
8. **Coverage-specific work:** test cfg, harnesses, integration targets,
   examples, benches, dev dependencies, documentation targets, or selected
   validation not present in the other activity.
9. **Ephemeral output:** generated compilation output, such as current doctest
   crates, that is created in a temporary location and not represented as a
   persistent Cargo artifact.
10. **Cross-command direction:** reuse from activity A to B may differ from B
    to A because one stage, artifact, or coverage set can contain or extend
    another.
11. **Compilation reuse:** compiler work or outputs are retained. This is
    separate from test, benchmark, binary, or documentation execution.
12. **Cross-command query plan:** the planned or observed graph of command,
    coverage, Cargo units, activity, target stages, stage dependencies, exact
    artifacts, compatible dependencies, tool work, persistent and ephemeral
    outputs, and execution.

Required command-pair controls include:

- aligned check then build and build then check;
- matching-target check then Clippy and Clippy then check;
- check then test, build then test, and test then build;
- build then documentation and documentation then build;
- documentation then doctest and doctest then documentation;
- repeated test compilation and repeated doctest; and
- a failed lint or diagnostic case proving that tool-specific work is not
  interchangeable.

Reports state separately:

- selected packages and targets;
- compile activity and mode;
- exact fresh and dirty artifacts;
- compiler, Clippy, and rustdoc invocations;
- required tool- or coverage-specific work;
- persistent versus temporary outputs;
- execution performed after compilation; and
- theoretical common stages not currently reused.

Matching package names, selected sources, unit-graph shape, runtime output, or
target directory does not prove cross-command compatibility. A shared compiler
base must preserve activity-specific cfg, lints, diagnostics, test harnesses,
documentation, codegen, linking, and failure behavior.

Fine-grained locking and common-stage reuse are reported separately. Allowing
commands to overlap can reduce waiting while duplicating total work; sharing
stages can reduce work while changing resource and locking behavior.

## Validation-selection evidence

Any impact-aware validation experiment reports package selection separately
from validation dimensions. The record includes:

- changed paths and declared input owners;
- direct package owners and reverse dependency closure;
- selected and omitted packages;
- checks, lints, tests, test compilation, doctests, release modes, targets,
  feature sets, platforms, execution, formatting, and repository gates;
- mappings for shared, generated, build-script, macro, native, runtime,
  environment, root-policy, and cross-target inputs;
- uncertainty and the reason for any full fallback;
- exact selected commands and the full reference commands;
- selected-pass evidence separately from full-reference evidence;
- periodic full-run comparison and the next audit obligation; and
- held-out mutation classes, false negatives, and promotion budget.

Unknown or unmapped changes expand to the full reference plan. No empty
package selection is interpreted as no validation. Repository-mandated gates
remain authoritative unless the repository explicitly scopes them.

A selector is evaluated first by failure detection and false negatives, then
by latency. A faster policy that misses a held-out material failure does not
pass the promotion gate. A selected pass is labeled only as a selected-plan
pass and never as proof that the full repository contract passed.

## Upstream contribution packaging

A measured case promoted toward rustc, rustc-perf, Cargo, a backend, or a
linker follows the
[Rust performance contribution packet](RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

The contribution record adds:

- one upstream home, owner, issue, goal, and maintainer question;
- a licensed reproducer with minimization provenance;
- a mapping from FERRIUM workloads and edits to upstream profiles, scenarios,
  metrics, tests, or benchmarks;
- local stable metrics in addition to environment-sensitive wall time;
- correctness, negative, failure, and unsupported controls;
- the bounded requested upstream action;
- maintenance and retirement ownership; and
- approval state before any external issue, comment, or pull request.

Upstream acceptance, external disposition, and FERRIUM product authorization
are independent outcomes.

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
- [Parsing and tokenization](../research/2026-08-08-parsing-tokenization.md),
  especially FERRIUM-108 through FERRIUM-116.
- [Declarative macro expansion](../research/2026-08-08-declarative-macro-expansion.md),
  especially FERRIUM-117 through FERRIUM-126.
- [Name resolution and HIR lowering](../research/2026-08-08-name-resolution-hir-lowering.md),
  especially FERRIUM-127 through FERRIUM-136.
- [Type inference and type checking](../research/2026-08-08-type-inference-checking.md),
  especially FERRIUM-137 through FERRIUM-146.
- [Trait-solving cost and reuse](../research/2026-08-08-trait-solving-cost-reuse.md),
  especially FERRIUM-147 through FERRIUM-156.
- [Borrow-checking cost and incrementality](../research/2026-08-08-borrow-checking-cost-incrementality.md),
  especially FERRIUM-157 through FERRIUM-166.
- [MIR construction and optimization](../research/2026-08-08-mir-construction-optimization.md),
  especially FERRIUM-167 through FERRIUM-176.
- [Frontend parallelism](../research/2026-08-08-frontend-parallelism.md),
  especially FERRIUM-177 through FERRIUM-188.
- [Query dependency precision and false invalidation](../research/2026-08-08-query-dependency-precision.md),
  especially FERRIUM-189 through FERRIUM-202.
- [Incremental cache overhead and reuse economics](../research/2026-08-08-incremental-cache-overhead.md),
  especially FERRIUM-203 through FERRIUM-216.
- [Early-phase incrementality](../research/2026-08-08-early-phase-incrementality.md),
  especially FERRIUM-222 through FERRIUM-234.
- Candidate root manifests inspected during corpus discovery:
  `METIS-CORE/Cargo.toml`, `PARLOR/Cargo.toml`, `RUNE/Cargo.toml`,
  `RLINE/Cargo.toml`, `ICELINES/Cargo.toml`, and `BISECT/Cargo.toml`.
