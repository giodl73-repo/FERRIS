---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:cargo-resolution-workspaces
kind: guide
module: rust-architecture
section: rust-architecture
title: Cargo - Resolution, Workspaces, Features, and Fingerprinting
status: source-custody
source_custody: partial
current_path: rust-architecture/17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md
canonical_path: rust-architecture/17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md
backsource_ids: [mdloom-backfill:rust-architecture:17-cargo-resolution-workspaces]
concepts: [cargo, dependency resolution, lockfile, workspaces, feature unification, fingerprinting]
root_concepts: [cargo]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Cargo - Resolution, Workspaces, Features, and Fingerprinting

## The Big Picture

Cargo is the Rust build orchestrator. It reads manifests, resolves packages,
builds a unit graph, schedules work, invokes `rustc` once per compile unit, and
coordinates linking. Cargo does **not** compile Rust; `rustc` does that work. It
also does not own crates.io; crates.io is the default public registry service,
and Cargo can resolve from alternative registries, git repositories, and path
dependencies.

```
+===========================================================================+
|                          CARGO'S AUTHORITY                                |
|                                                                           |
| Inputs: Cargo.toml + Cargo.lock + workspace + profiles + CLI flags        |
|                                                                           |
|  manifests -> resolver -> concrete package graph / Cargo.lock             |
|       |            |                         |                            |
|       v            v                         v                            |
|  unit graph -> scheduler -> rustc invocations                             |
|                                      |                                    |
|                                      v                                    |
|                       target artifacts + fingerprints                     |
+===========================================================================+
        |                     |                       |
        v                     v                       v
 crates.io/default       alternative registries      git/path deps
 separate service        supported by Cargo          local or remote source
```

Read this as an ownership diagram. Cargo owns the graph, resolver result,
lockfile, scheduling, profiles, and build cache freshness decision. `rustc` owns
parsing, type checking, code generation, metadata, diagnostics, and most of the
work behind each command line Cargo prints. For the compiler side of that line,
see [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md) and
[13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md).

---

## Manifests, Registries, and Dependency Requirements

`Cargo.toml` is Cargo's stable declarative input. It names the package, targets,
dependencies, features, build dependencies, dev dependencies, profiles, and
workspace participation. The default version requirement is SemVer caret: `1.2`
means `^1.2`, compatible with `>=1.2.0, <2.0.0`. SemVer compatibility is an
ecosystem convention Cargo enforces during resolution; the governance and
compatibility story sits with the project process in
[01](01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md), not in `rustc`.

| Manifest area | Cargo meaning | Authority boundary |
|---------------|---------------|--------------------|
| `[package]` | This package's identity, edition, version, metadata | Cargo manifest contract |
| `[dependencies]` | Target dependencies for normal builds | Cargo resolver input |
| `[dev-dependencies]` | Tests, examples, benches | Cargo resolver input, boundary-sensitive under resolver v2/v3 |
| `[build-dependencies]` | Host-side build script dependencies | Cargo host unit input, see [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md) |
| `[features]` | Additive named switches and optional deps | Cargo feature unification contract |
| `[patch]` / `[source]` / registries | Override or redirect package source | Cargo registry/source contract |

```toml
[package]
name = "route-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tracing = "0.1"
regex = { version = "1.10", default-features = false, features = ["std"] }
serde_json = { version = "1", optional = true }

[dev-dependencies]
insta = "1"

[features]
default = ["json"]
json = ["dep:serde_json"]
```

crates.io is only the default registry. Corporate/private registries, vendored
sources, git dependencies, and path dependencies are first-class Cargo source
options. Cargo decides where a package comes from; `rustc` sees source files and
flags after Cargo has made that decision.

---

## Resolution, Lockfiles, and Workspaces

The resolver chooses concrete package versions satisfying all requirements. The
important contrast with NuGet/Maven-style mental models is that Cargo allows
multiple SemVer-incompatible versions of the same crate to coexist in the same
build. If one dependency needs `foo 1.x` and another needs `foo 2.x`, Cargo can
build both, isolated by crate identity. It does not force a single global
winner. Within one SemVer-compatible line, Cargo still tries to share a version.

```
+-------------------+        +-------------------+
| app               |        | Cargo can resolve |
| foo = "1"         | -----> | foo v1.8.0        |
| bar = "2"         |        | bar v2.1.0        |
+-------------------+        |   -> foo v2.4.0   |
                             +-------------------+

Same name, different major versions: separate crate identities, both compile.
Same SemVer line: resolver tries to unify to one concrete version.
```

`Cargo.lock` records the exact resolved graph, including registry checksums and
git revisions. Binaries and applications should commit it. Libraries
historically often did not, because downstream applications resolve the final
graph; current practice is more nuanced, but the key point is that the lockfile
is a reproducibility artifact owned by Cargo.

Workspaces put multiple packages under one resolution and build umbrella.
Members share one lockfile, one `target/` directory by default, and one
`[workspace]` policy surface. A virtual manifest is a `Cargo.toml` with
`[workspace]` and no local `[package]`.

| Workspace feature | What it does | Old-world shape |
|-------------------|--------------|-----------------|
| `members` | Names packages participating in one workspace | Solution projects |
| `Cargo.lock` at root | Records the unified resolved graph | `packages.lock.json` at repo root |
| shared `target/` | Reuses build artifacts across members | Common output/intermediate tree |
| `[workspace.dependencies]` | Centralizes versions, inherited by members | `Directory.Packages.props` / central package management |

```toml
[workspace]
members = ["crates/api", "crates/core", "tools/xtask"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tracing = "0.1"
```

Resolver v2, the default for edition 2021 packages, made feature unification
less global across build-dependency/proc-macro host units, target units, and
dev-only edges. Resolver v3 has shipped since Rust 1.84 and is implied by
edition 2024 (except that virtual workspaces must set `resolver = "3"`
explicitly). It retains resolver-v2 feature behavior and adds
`rust-version`-aware dependency selection with incompatible versions treated as
fallback candidates. Cargo's shipping resolver is still the classic resolver;
PubGrub-style replacement work is not the stable baseline.

---

## Features, Units, Profiles, and Scheduling

Cargo features are named, additive switches. Optional dependencies create
features unless hidden behind `dep:` syntax. `default` is just a feature selected
unless disabled. The footgun is feature unification: a crate is built once for a
given unit with the union of all requested features in that boundary.

```
crate A asks: serde features = ["derive"]
crate B asks: serde features = ["rc"]
                 |
                 v
Cargo builds serde once with: ["derive", "rc"]
```

Do not model features as mutually exclusive configurations. They are closer to
capability bits that only turn on. Resolver v2 stopped unifying some features
across build-dependency/proc-macro host units, target units, and dev-only edges,
which removed several accidental activations. It did not make features private.

A Cargo **unit** is roughly:

| Unit dimension | Examples |
|----------------|----------|
| package | `serde`, `route-engine`, `api` |
| target | lib, bin, test, bench, example, build script, proc macro |
| profile | dev, release, test, bench; `opt-level`, debug, LTO, codegen-units, panic |
| features | the unified feature set for this package in this boundary |
| platform side | host or target |

Cargo builds the unit DAG in parallel, respecting dependency edges. Profiles
control flags passed to `rustc`; monomorphization/codegen implications are in
[11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md), backend implications in
[12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md), and panic/platform
layering in [16](16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md).

Cargo also pipelines. When a dependency's metadata (`.rmeta`, emitted by
`rustc --emit=metadata`) is ready, Cargo can start compiling dependents before
that dependency's full codegen finishes. That is a real scheduler optimization,
not a language feature. See [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md)
for the artifact boundary.

---

## Fingerprints and Freshness

Cargo has its own incremental build layer, separate from `rustc` incremental.
For each unit, Cargo computes a freshness fingerprint from declared and observed
inputs: sources, dependency outputs, `rustc` version, flags, profiles, features,
environment variables, build script `rerun-if-*` declarations, and similar
state. If the fingerprint matches, Cargo declares the unit fresh and skips the
`rustc` invocation.

```
+----------------------+        +--------------------------+
| Cargo fingerprint    | -----> | skip or invoke rustc     |
| target/.fingerprint/ |        | whole unit freshness     |
+----------------------+        +--------------------------+
          |
          | distinct from
          v
+----------------------+        +--------------------------+
| rustc incremental    | -----> | reuse query/codegen work |
| target/.../incr.     |        | inside one crate         |
+----------------------+        +--------------------------+
          |
          | distinct from
          v
+----------------------+        +--------------------------+
| sccache/cache tool   | -----> | reuse compiler outputs   |
| external layer       |        | across invocations       |
+----------------------+        +--------------------------+
```

The stable contract is that documented Cargo commands rebuild when necessary and
reuse work when safe. The exact fingerprint hash composition, file layout under
`target/.fingerprint/`, and unit-graph internals are Cargo implementation
details. They are useful for debugging, not for building permanent tooling.

---

## Concrete Trace: Seeing Cargo's Graph

Use Cargo's documented commands when possible; treat verbose internals as
observations.

```powershell
cargo metadata --format-version=1 > metadata.json
cargo tree -f "{p} {f}"
cargo build -v
cargo build --timings
cargo update -p serde
```

A lockfile entry is the resolved result, not the manifest request:

```toml
# Cargo.lock
[[package]]
name = "serde"
version = "1.0.197"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "..."
dependencies = [
 "serde_derive",
]
```

A verbose build line shows the boundary clearly:

```text
Running `rustc --crate-name route_engine src\lib.rs --edition=2021 \
  --crate-type lib -C debuginfo=2 --emit=dep-info,metadata,link \
  --extern serde=...\libserde-....rlib`
```

Cargo chose that unit and command line. `rustc` owns what happens inside it.

---

## Old World -> New World

| Old-world concept | Cargo-side analogue | Important difference |
|-------------------|---------------------|----------------------|
| MSBuild + NuGet | Cargo fused build graph, restore, lockfile, and invocation planner | Less XML extensibility; more convention and manifest structure |
| NuGet binding redirects / one version wins | Multiple SemVer-incompatible crate versions can coexist | Isolation is solved at compile/link identity, not runtime binding policy |
| `packages.lock.json` / npm lockfile | `Cargo.lock` | Cargo owns exact resolved graph and checksums |
| MSBuild solution / npm workspaces | Cargo workspace | One lockfile and shared target directory by default |
| Conditional compilation / build configurations | Cargo features + profiles | Features are additive and unioned; profiles are build-mode policy |
| MSBuild incremental up-to-date check | Cargo fingerprints | Cargo unit freshness is distinct from rustc incremental and external caches |

The closest mental model is "MSBuild and NuGet intentionally fused around one
manifest and one graph." That fusion is why `cargo build`, `cargo test`,
`cargo tree`, and `cargo metadata` all see the same world.

---

## Decision Cheat Sheet

| Question | Use / answer | Authority |
|----------|--------------|-----------|
| Who compiles Rust? | `rustc`, invoked by Cargo | rustc |
| Who resolves dependency versions? | Cargo resolver | Cargo |
| Where is the exact resolved graph? | `Cargo.lock` | Cargo stable format |
| Can two major versions coexist? | Yes, if SemVer-incompatible | Cargo resolver behavior |
| Why did a feature turn on? | Feature unification; inspect `cargo tree -f "{p} {f}"` | Cargo |
| Why did a crate rebuild? | Fingerprint changed; use `cargo build -v` and build script output | Cargo implementation detail |
| Where do profiles affect codegen? | Cargo passes flags to rustc; see [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) | Cargo + rustc |
| Can I depend on `target/.fingerprint/` layout? | No | Cargo internal |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "Cargo compiles my code." | Cargo orchestrates; `rustc` compiles. |
| "crates.io is Cargo." | crates.io is the default registry service; Cargo is the client/orchestrator and supports alternatives. |
| "SemVer means one global version." | Cargo can compile multiple incompatible versions side by side. |
| "Features are configurations." | Features are additive and unioned, not exclusive modes. |
| "Resolver v2 removed feature unification." | It narrowed some host/target/dev unification, but features still unify within boundaries. |
| "Resolver v3 changes feature unification again." | It keeps resolver-v2 feature boundaries and adds MSRV-aware version fallback. |
| "Cargo incremental is rustc incremental." | Cargo fingerprints skip whole units; rustc incremental reuses compiler work inside a crate; sccache is a third layer. |
| "Cargo.lock is just advisory." | For applications it is the reproducibility source of truth. |

---

## Primary Sources

| Source | Why it matters |
|--------|----------------|
| The Cargo Book: manifest, dependency resolution, features, workspaces, profiles, build cache | Stable user contract for Cargo behavior |
| Cargo contributor docs: `rust-lang/cargo` `ARCHITECTURE.md`, "How Cargo works" | Implementation map for units, resolver, and scheduling |
| Cargo SemVer compatibility chapter | How Cargo interprets compatibility promises |
| RFCs and Cargo documentation for resolver v2/features | Feature unification boundary changes |
| [00](00-OVERVIEW.md), [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md), [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md) | Sibling guides around Cargo, build-time code, and tools |
