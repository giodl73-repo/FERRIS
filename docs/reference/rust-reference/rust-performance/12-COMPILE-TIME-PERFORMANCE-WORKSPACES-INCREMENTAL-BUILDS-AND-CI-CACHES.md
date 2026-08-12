---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:compile-time-performance-workspaces-incremental-builds-and-ci-caches
kind: guide
module: rust-performance
section: rust-performance
title: Compile-Time Performance, Workspaces, Incremental Builds, and CI Caches
status: source-custody
source_custody: partial
current_path: rust-performance/12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md
canonical_path: rust-performance/12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md
backsource_ids: [mdloom-backfill:rust-performance:12-compile-time-performance-workspaces-incremental-builds-and-ci-caches]
concepts: [compile time, cargo workspaces, incremental compilation, ci caches, sccache, build timings]
root_concepts: [compile-time performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Compile-Time Performance, Workspaces, Incremental Builds, and CI Caches

## The Big Picture

Compile-time performance is a delivery-system problem. Cargo schedules a unit
graph; rustc performs front-end analysis, monomorphization, codegen, and
metadata work per crate; native tools and the linker add their own critical
paths; caches help only when keys and reuse boundaries match.

```
+=============================================================================+
|                          BUILD CRITICAL PATH                                |
|                                                                             |
| resolve graph -> build scripts/proc macros -> crate checks/codegen -> link  |
|                       |                     |                   |           |
|                       v                     v                   v           |
|                 host dependencies     parallel crate DAG       linker       |
|                                             |                               |
|                                             +-> incremental query/CGU reuse |
|                                             +-> compiler cache reuse        |
|                                                                             |
| clean build | warm no-op | scripted edit-build | test | release are separate|
+=============================================================================+
```

## Measure Distinct Build Scenarios

| Scenario | Question |
|----------|----------|
| Clean check | How fast can a fresh contributor get diagnostics? |
| Clean build | What is full codegen/link cost? |
| Warm no-op | Is Cargo freshness cheap and correct? |
| Leaf edit | How fast is a common local change? |
| Core crate edit | How wide is invalidation? |
| Feature/target change | How much cache duplication occurs? |
| Release build | What is the delivery critical path with LTO/PGO policy? |
| Test run | Is compilation or test execution dominant? |

Script the edit so comparisons touch the same file and semantic surface. A clean
build improvement can regress the daily leaf-edit loop, and vice versa.

## Cargo Timings

```
# Stable HTML timings report on current Cargo.
cargo build --timings

# Separate front-end diagnostics from full code generation.
cargo check --workspace --all-targets
cargo build --workspace --all-targets

# See commands and freshness decisions.
cargo build -vv
```

Cargo timings expose crate-unit durations, concurrency, and the critical path.
They do not decompose rustc internals. Machine-readable timing formats have had
different stability than HTML; verify your Cargo version before automating.

Look for:

- one long crate that serializes the graph;
- build scripts or proc macros gating many dependents;
- duplicate builds for host/target or feature/profile combinations;
- long final links;
- poor CPU utilization caused by dependency shape;
- repeated native compilation.

## Workspace and Crate Boundaries

Crates are compilation and incremental-reuse boundaries.

```
monolith:
[---------------- one large crate ----------------]
 edit -> analyze/codegen broad internal surface

factored workspace:
[core] -> [domain] -> [service] -> [bin]
 edit leaf --------------------------^ rebuild narrow
 edit core -> invalidates downstream chain
```

| More crates can help | More crates can hurt |
|----------------------|----------------------|
| Parallel compilation | Per-crate metadata and scheduling overhead |
| Narrow invalidation | More link inputs and dependency management |
| Stable API boundaries | Cross-crate optimization may need LTO/inlining |
| Team ownership | Deep serial dependency chains |

Do not split crates solely to chase timings. Choose boundaries that represent
stable ownership/API seams, then verify edit patterns.

## Incremental Compilation

Rustc incremental compilation reuses query results and codegen work products.
Cargo freshness decides whether a crate invocation is needed at all. They are
different layers.

| Layer | Reuse decision |
|-------|----------------|
| Cargo | Has this package unit's inputs/configuration changed? |
| rustc incremental | Which compiler queries/codegen units can be reused inside a rebuilt crate? |
| Compiler cache | Can a matching compilation output be restored from local/remote storage? |

Incremental artifacts are toolchain-, target-, profile-, flag-, and path-sensitive
and can be large. They are excellent for local edit-build loops. In ephemeral CI,
upload/download and low hit rates can outweigh reuse, so compare CI with
incremental disabled and a compiler cache enabled.

```
# One controlled CI experiment.
$env:CARGO_INCREMENTAL="0"    # PowerShell
cargo build --locked --workspace
Remove-Item Env:CARGO_INCREMENTAL
```

On POSIX shells use `CARGO_INCREMENTAL=0 cargo build ...`. Cargo profile defaults
already differ; record the effective profile rather than assuming this variable
is necessary everywhere.

## Proc Macros, Build Scripts, and Native Dependencies

Proc macros and build scripts compile for and execute on the host. They can:

- gate downstream crates;
- scan large directory trees;
- rerun too often because `rerun-if-*` directives are incomplete;
- invoke C/C++ compilers repeatedly;
- generate large token streams;
- duplicate work across feature/target variants.

Inspect `cargo build -vv` and timings. Build scripts should emit precise
`cargo::rerun-if-changed`/`rerun-if-env-changed` directives according to the
current Cargo contract. The modern `cargo::` syntax has an MSRV requirement;
repositories supporting older Cargo releases may need the older accepted
`cargo:` spelling. Generated code that creates enormous Rust syntax trees can
move cost into parsing, expansion, type checking, and monomorphization.

## Generics, Macros, and Codegen

Compile-time cost often comes from abstraction volume:

| Cause | Mitigation to evaluate |
|-------|------------------------|
| Many generic instantiations | Move heavy logic behind a non-generic internal core |
| Large derive/proc-macro output | Reduce generated surface; inspect expanded code carefully |
| Deep trait solving | Simplify bounds/types at hot compile sites |
| Large debug info | Tune dev debug level if debugging requirements allow |
| Many codegen units | Balance parallel codegen and downstream optimization |
| Slow link | Faster compatible linker, dynamic dev linking where appropriate, fewer artifacts |

Nightly self-profile (`-Z self-profile`) and related tools can attribute rustc
internals, but they are unstable compiler diagnostics. Pin the nightly version
and use them only after stable Cargo timings identify a target.

## CI Cache Layers

```
dependency download cache -> compiler output cache -> target/incremental cache
          high reuse              key-sensitive             large/brittle
```

| Cache | Usually worth it | Key inputs |
|-------|------------------|------------|
| Cargo registry/git downloads | Yes | lockfile/source config |
| `sccache` compiler outputs | Often | compiler, target, flags, source, environment |
| Entire `target` directory | Sometimes | OS, toolchain, target, profile, features, flags, paths |
| Test data/tool downloads | Often | version/checksum |

`sccache` is external. Configure it as `RUSTC_WRAPPER`, verify hit rates, and
secure remote cache credentials and namespaces.

```
# External installation.
cargo install sccache --locked
$env:RUSTC_WRAPPER="sccache"
sccache --zero-stats
cargo build --locked --workspace
sccache --show-stats
Remove-Item Env:RUSTC_WRAPPER
```

A cache hit statistic without elapsed-time improvement is not success. Include
network transfer and compression cost. Treat remote compiler caches as part of
the build trust boundary: authenticate writes, separate untrusted pull-request
workloads from release namespaces, and do not promote cached output without the
same provenance and verification policy as a local compilation.

### Azure Pipelines Supplement

Azure Pipelines `Cache@2` can cache Cargo downloads or an `sccache` directory.
Use a key containing OS, architecture, toolchain identity, and `Cargo.lock`.
Caching the whole `target` directory needs stricter profile/feature/flag keys and
often poor size economics. Universal cache-key discipline comes first; Azure is
only one implementation.

Windows Defender or enterprise endpoint scanning can affect many-small-file
builds. Do not add exclusions casually. Measure, use organization-approved build
paths/policies, and preserve security controls.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| MSBuild project graph / critical path | Cargo unit graph and `--timings` |
| Roslyn incremental compilation | rustc query/work-product incremental reuse |
| ccache / remote build cache | `sccache` |
| NuGet package cache | Cargo registry/git download cache |
| Solution decomposition | Workspace/crate boundaries |
| VSTS/Azure Pipelines cache task | Cache Cargo downloads and compiler outputs with complete keys |

The major Rust-specific costs are monomorphization, proc macros, trait solving,
and native link work.

## Common Confusion Points

- **`cargo check` time is not link or release-build time.**
- **Clean and incremental builds answer different questions.**
- **More crates can increase a serial dependency chain.**
- **A restored `target` directory is not automatically reusable.**
- **Compiler caches are accelerators, not source-of-truth artifact stores.**
- **Cache hit rate is not elapsed-time savings.**
- **LTO/one CGU runtime tuning can heavily regress release build time.**
- **Nightly self-profile output is version-sensitive.**
- **Security exclusions are not a default performance knob.**

## Decision Cheat Sheet

| Observation | First action |
|-------------|--------------|
| Slow diagnostics loop | Measure `cargo check`; inspect long gating crates |
| Slow leaf edit | Script the edit; inspect invalidation and crate boundaries |
| Low CPU utilization | Examine dependency critical path/build scripts |
| Proc macro/build script dominates | Narrow rerun inputs and generated surface |
| Final link dominates | Evaluate compatible linker and artifact/profile policy |
| Ephemeral CI is slow | Cache downloads, test `sccache`, compare incremental off |
| Huge cache, low benefit | Tighten keys or remove `target` caching |
| Need rustc internal attribution | Pinned nightly self-profile after stable timings |

## Primary Sources

- Cargo build cache: https://doc.rust-lang.org/cargo/reference/build-cache.html
- Cargo timings: https://doc.rust-lang.org/cargo/reference/timings.html
- rustc incremental compilation: https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html
- sccache: https://github.com/mozilla/sccache
- Cargo build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html

## Related Guides

- Build profiles: [02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md](02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md)
- Generic/codegen cost: [06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md](06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md)
- Link-stage cost: [11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md](11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md)
