---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:bootstrap-ci-perf-distribution
kind: guide
module: rust-architecture
section: rust-architecture
title: Bootstrap, CI, Testing, Performance, and Distribution
status: source-custody
source_custody: partial
current_path: rust-architecture/20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md
canonical_path: rust-architecture/20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md
backsource_ids: [mdloom-backfill:rust-architecture:20-bootstrap-ci-perf-distribution]
concepts: [bootstrap, x.py, compiler testing, rustc-perf, crater, release distribution]
root_concepts: [bootstrap]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Bootstrap, CI, Testing, Performance, and Distribution

## The Big Picture

This is the supply chain for **rustc itself**, not the path by which a Rust user
builds an application. A user build is Cargo planning crates and invoking rustc
([17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md)). A compiler build is
bootstrap: use an already-existing Rust compiler to build the next rustc, test it
hard, benchmark it, regression-test the ecosystem, then publish toolchains that
rustup installs ([02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md)).

```
+===========================================================================+
|              RUSTC SELF-HOSTING + QUALITY + DISTRIBUTION                  |
|                                                                           |
|  BOOTSTRAP: build the compiler with a compiler                            |
|                                                                           |
|  stage0: downloaded beta rustc + std/library                              |
|      | compiles in-tree compiler crates                                   |
|      v                                                                    |
|  stage1: rustc built by stage0                                            |
|      | good for most contributor build/test cycles                        |
|      v                                                                    |
|  stage2: rustc built by stage1                                            |
|      | self-hosted candidate for distribution                             |
|      v                                                                    |
|  stage3: optional rebuild by stage2                                       |
|      | checks reproducibility / determinism: stage2 ~= stage3             |
+---------------------------------------------------------------------------+
|  CI / RELEASE FUNNEL                                                       |
|                                                                           |
|  PR -> x.py check/test -> try build -> bors full CI -> merge               |
|       |                 |                                                  |
|       |                 +-> rustc-perf: compile-time / memory evidence     |
|       |                                                                  |
|       +-> crater: ecosystem compile/test regression runs                  |
|                                                                           |
|  merged main / beta / stable branch                                       |
|       | scheduled nightly job or controlled release promotion             |
|       v                                                                   |
|  dist artifacts -> signed channel manifests -> static.rust-lang.org        |
|       |                                                                   |
|       v                                                                   |
|  rustup installs toolchains                                                |
+===========================================================================+
```

The stable contract at the far right is the released toolchain. Everything in
the middle -- `x.py`, bootstrap stages, `config.toml` / `bootstrap.toml`, test
layout, and runner YAML -- is project-internal machinery, useful to know and not
a user-facing guarantee.

---

## Authority Boundaries

| Layer | What it owns | Authority |
|-------|--------------|-----------|
| Bootstrap / `x.py` | Building rustc, std, LLVM, and tools from the rust-lang/rust tree | Rust project's bootstrap and infra teams |
| Compiler implementation | Front end, MIR, codegen, diagnostics, artifacts | `rustc` teams; see [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md), [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md), [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md) |
| User builds | Dependency graph, features, lockfiles, per-crate rustc invocations | Cargo; conceptually guide [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md) |
| Toolchain install | Channels, components, target libraries, proxies | rustup; consumes released artifacts, does not build them ([02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md)) |
| Ecosystem regression | Build/test crates.io and GitHub crates against candidate compilers | crater (`rust-lang/crater`) |
| Compiler performance | Compile-time and memory benchmark lab | rustc-perf (`rust-lang/rustc-perf`) |
| Release train | Nightly, beta, stable cadence and publication | Rust release and infra teams; policy context in [01](01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md) |

This separation matters because a failure in Cargo resolution is not a bootstrap
failure, and a crater regression is not a rustc-perf regression. They sit in the
same release funnel but answer different questions.

---

## Bootstrap Stages: What Is Built By What

`rustc` is written in Rust. That is the chicken-and-egg problem: to build rustc,
you already need a Rust compiler. Rust solves it the conventional self-hosting
way: start from a previously released compiler, then rebuild inward.

```
+---------+       +---------+       +---------+       +---------+
| stage0  | ----> | stage1  | ----> | stage2  | ----> | stage3  |
| beta    |       | built   |       | built   |       | built   |
| rustc + |       | by      |       | by      |       | by      |
| std/lib |       | stage0  |       | stage1  |       | stage2  |
+---------+       +---------+       +---------+       +---------+
   seed            fast dev          shippable         compare
```

| Stage | Built by | Good for | Not good for |
|-------|----------|----------|--------------|
| **stage0** | Downloaded prebuilt beta rustc plus std/library artifacts | Seeding the build | Proving current source can self-host |
| **stage1** | stage0 compiler | Most contributor `build`, `check`, and many `test` loops | Final performance/profile confidence |
| **stage2** | stage1 compiler | Self-hosted compiler candidate, release validation, dist builds | Fast inner-loop iteration |
| **stage3** | stage2 compiler | Determinism/reproducibility sanity check: stage2 and stage3 should match where expected | Routine development |

The practical rule: if you are modifying compiler logic, stage1 is usually where
you start. If you are validating the exact compiler to ship, stage2 is the line
that matters.

---

## `x.py` Is the Compiler Build Orchestrator, Not Cargo

`./x.py` (and the shorter `./x`) is a Python bootstrap shim that drives a Rust
`bootstrap` tool. Bootstrap uses Cargo underneath because the compiler is a Rust
workspace, but Cargo alone does not understand stages, host/target compiler
pairs, in-tree LLVM policy, or distribution packaging.

```
+------------------------------------------------------------------+
| ./x.py / ./x                                                     |
|  Python entry point -> Rust bootstrap binary                     |
+------------------------------------------------------------------+
        | reads config.toml / bootstrap.toml
        v
+------------------------------------------------------------------+
| bootstrap plan                                                   |
|  LLVM: build bundled LLVM or use system LLVM                     |
|  rustc: stage0 -> stage1 -> stage2                               |
|  library/: core/alloc/std for host + targets ([16])              |
|  tools: cargo, rustdoc, clippy, rustfmt, etc. ([19])             |
|  dist: rust-installer tarballs + manifests                       |
+------------------------------------------------------------------+
        |
        v
+------------------------------------------------------------------+
| Cargo invocations, compiler invocations, test harnesses          |
+------------------------------------------------------------------+
```

`config.toml` / `bootstrap.toml` options are version-sensitive: build profile,
LLVM choice, download-rustc behavior, host/target sets, paths, and dist knobs are
all internal developer controls. Do not treat them like Cargo's user-facing
manifest model.

---

## Testing: From Compiletest to Full CI

Rust's CI is not one test suite. It is a stack of harnesses with different
failure modes, plus bors as the gated merge authority. The rule is the old
"not rocket science" rule: keep main in a state where the full test matrix
passes; do not merge known-red code.

| Suite / harness | What it catches |
|-----------------|-----------------|
| `tests/ui` via compiletest | Diagnostics and compiler output snapshots; `--bless` updates expected stderr/stdout |
| `tests/run-make` | End-to-end build orchestration cases that need makefiles, linkers, env, or files |
| `tests/codegen` | LLVM IR expectations, normally FileCheck-based ([12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md)) |
| `tests/assembly` | Target assembly expectations for backend-sensitive behavior |
| `tests/mir-opt` | MIR transform output and optimization pass changes ([09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md)) |
| `tests/incremental` | Dependency tracking, fingerprints, and cache reuse ([14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md)) |
| `tests/debuginfo` | Debugger-visible layout and metadata |
| `tests/rustdoc` | Documentation generation, doctests, search, and rendering |
| `library/` tests | Standard library unit/integration behavior ([16](16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md)) |
| `tidy` | Repository style, licensing, error-code, and metadata hygiene |

```
# local compiler-development loop
$ git clone https://github.com/rust-lang/rust
$ cd rust
$ ./x.py setup
$ ./x.py check
$ ./x.py build --stage 1
$ ./x.py test tests/ui --bless
$ ./x.py test tidy

# PR control-plane examples
@bors try
@rust-timer queue
@craterbot run name=my-regression-check mode=check-only start=master end=try
@bors r+

# consumer side: install what CI/release produced
$ rustup toolchain install nightly
```

Try builds answer "does this survive CI before merge?" Bors answers "can this be
merged while preserving green main?" Local `./x.py test tests/ui` is only a
small slice of that funnel.

---

## Performance and Ecosystem Regression: Two Orthogonal Farms

| Question | Tool | Input | Output |
|----------|------|-------|--------|
| "Did rustc get slower or use more memory?" | rustc-perf | A compiler build compared against baseline on curated benchmarks | perf.rust-lang.org deltas, PR comments, gating evidence |
| "Does the ecosystem still compile and test?" | crater | Candidate compiler vs baseline over a huge crates.io + GitHub sample | Regression reports for compatibility triage |

```
          +--------------------+          +----------------------+
PR -----> | @rust-timer queue  | -------> | rustc-perf dashboard |
          | compile-time/mem   |          | regression deltas    |
          +--------------------+          +----------------------+

          +--------------------+          +----------------------+
change -> | crater fleet       | -------> | ecosystem failures   |
          | check/build/test   |          | hours/days of data   |
          +--------------------+          +----------------------+
```

For someone who has run perf labs and gated check-ins, the mapping is direct:
rustc-perf is the benchmark gate; crater is the giant integration/regression
farm. They are deliberately separate because "the compiler is 3% slower" and
"200 crates no longer compile" require different owners and different fixes.

---

## Release Distribution Funnel

CI's final product is not "a compiler binary" in isolation. It is a set of
componentized toolchains: `rustc`, Cargo, standard libraries per target, rustdoc,
sources, clippy/rustfmt where applicable, installers, and channel manifests.
PR CI can build and test distribution-shaped artifacts, but it does not publish
them as a channel. Publication starts from a merged mainline revision for
nightly, or from the controlled beta/stable promotion process, and runs the
release builders, signing, manifest generation, and upload steps.

```
+------------------+     +------------------+     +-------------------+
| merged/channel   | --> | scheduled or     | --> | rust-installer    |
| release revision |     | release builders |     | dist components   |
+------------------+     +------------------+     +-------------------+
                                  | PGO / BOLT optimized rustc builds
                                  v
                         +-------------------+
                         | channel manifests |
                         |nightly/beta/stable|
                         +-------------------+
                                  |
                                  v
                         +-------------------+
                         | static.rust-lang. |
                         | org CDN/feed      |
                         +-------------------+
                                  |
                                  v
                         +-------------------+
                         | rustup install /  |
                         | update / override |
                         +-------------------+
```

Nightly is published from the mainline cadence, beta is the stabilization branch,
and stable ships on the six-week train described in [01](01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md).
The released artifacts and manifests are the user-facing surface. The exact CI
runner topology, `x.py dist` internals, and packaging steps are not.

---

## Old World -> New World

| Old world anchor | Rust equivalent |
|------------------|-----------------|
| GCC three-stage bootstrap | stage0 -> stage1 -> stage2 rustc; optional stage3 reproducibility check |
| Roslyn self-hosting | rustc compiling the next rustc, with a release compiler as the seed |
| Giant MSBuild/CMake repo orchestrator | `x.py` / bootstrap wrapping Cargo plus LLVM, std, tools, tests, dist |
| VSTS gated check-in / merge queue | bors try builds and `@bors r+` full-CI merge discipline |
| Perf lab with benchmark gates | rustc-perf and `@rust-timer` |
| Large compatibility/integration farm | crater over crates.io + GitHub projects |
| SDK package feed + CDN installer | static.rust-lang.org manifests consumed by rustup |

The philosophy is familiar. The names and boundaries are the new part.

---

## Reading Paths

| You want to understand... | Read |
|---------------------------|------|
| Channel policy and compatibility | [01](01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md) -> this guide |
| How a downloaded toolchain is selected | [02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md) -> this guide |
| Why a compiler test changed output | [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md) -> `tests/ui` here |
| Why a build cache bug matters | [14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md) -> incremental tests here |
| How LLVM/PGO/BOLT fit release builds | [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md) -> release funnel here |

---

## Decision Cheat Sheet

| Situation | Use / think |
|-----------|-------------|
| I am changing rustc internals and want a fast sanity pass | `./x.py check`, then targeted `./x.py test ...`, usually stage1 |
| I changed diagnostics | `tests/ui`, update snapshots intentionally with `--bless` |
| I changed MIR or incremental behavior | `tests/mir-opt` or `tests/incremental`, then broader CI |
| I changed backend/codegen | `tests/codegen`, `tests/assembly`, LLVM-aware review, perf run |
| I suspect compile-time regression | `@rust-timer queue` / rustc-perf data |
| I suspect ecosystem compatibility fallout | crater run, usually team-mediated and expensive |
| I need to merge a PR | bors try if needed, then `@bors r+`; never knowingly red main |
| I just want to build my app | Cargo, not bootstrap; see guide [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md) |
| I just want the shipped compiler | rustup from static.rust-lang.org manifests ([02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md)) |

---

## Common Confusion Points

- **Stage1 is useful but not the shipped compiler.** It is the contributor workhorse.
  Stage2 is the self-hosted candidate.
- **Cargo is involved but not in charge.** Bootstrap invokes Cargo; Cargo does
  not model compiler stages, dist artifacts, or CI policy.
- **Crater is not rustc-perf.** Crater asks compatibility; rustc-perf asks speed
  and memory.
- **`--bless` is not a magic fix.** It accepts new expected output. Use it only
  when the new diagnostic or output is intentional.
- **Nightly does not mean untested.** It means published from the nightly channel;
  it still passed that channel's CI gates.
- **Bootstrap internals are not Rust guarantees.** The language contract lives in
  the Reference/RFC/stability process and stable libraries, not in `x.py` flags,
  compiletest layout, or CI runner configuration. For language semantics, see
  `../rust-language/` where that module exists.

---

## Primary Sources

- **rustc-dev-guide** — Bootstrapping; How to build and run the compiler; The
  compiler testing framework / compiletest; Suggested workflows; Walkthrough: a
  typical contribution.
- **rust-lang/rust bootstrap docs** — the in-tree bootstrap implementation and
  configuration documentation.
- **rust-lang/rust CI docs** — builders, try builds, bors integration, and dist
  jobs.
- **rust-lang/rustc-perf** — benchmark suite, collector, and perf.rust-lang.org.
- **rust-lang/crater** — ecosystem regression testing infrastructure and docs.
- **Rust Forge release-process docs** — release train, promotion, channel
  manifests, and distribution process.
