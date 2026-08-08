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
cargo metadata --format-version 1 --no-deps
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
- Candidate root manifests inspected during corpus discovery:
  `METIS-CORE/Cargo.toml`, `PARLOR/Cargo.toml`, `RUNE/Cargo.toml`,
  `RLINE/Cargo.toml`, `ICELINES/Cargo.toml`, and `BISECT/Cargo.toml`.
