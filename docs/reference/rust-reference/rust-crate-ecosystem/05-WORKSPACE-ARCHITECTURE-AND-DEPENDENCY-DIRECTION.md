---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:workspace-architecture-dependency-direction
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Workspace Architecture and Dependency Direction
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md
canonical_path: rust-crate-ecosystem/05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:05-workspace-architecture-dependency-direction]
concepts: [cargo workspace, crate architecture, dependency direction, internal crates, workspace dependencies]
root_concepts: [cargo workspace]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Workspace Architecture and Dependency Direction

## The Big Picture

A Cargo workspace is a build and policy boundary, not an architecture. Crate
edges create the architecture. The useful default is inward dependency flow:
applications and adapters depend on stable domain boundaries, never the reverse.

```
+===========================================================================+
|                        WORKSPACE DEPENDENCY FLOW                          |
+===========================================================================+
|                                                                           |
|  binaries / services / CLI                                                |
|            |                                                              |
|            v                                                              |
|  orchestration / use cases <------ adapters (db, http, queue)             |
|            |                         |                                    |
|            v                         | implements ports                   |
|  domain API / traits / types <-------+                                    |
|            |                                                              |
|            v                                                              |
|  small primitives / shared contracts                                      |
|                                                                           |
|  Forbidden: domain -> adapter, primitives -> application, dependency cycle |
+===========================================================================+
```

Cargo rejects package dependency cycles, but an acyclic graph can still be
poorly directed, overly granular, or coupled through public re-exports.

## Workspace Mechanics

```toml
[workspace]
members = [
  "crates/domain",
  "crates/application",
  "crates/adapter-sql",
  "apps/server",
  "tools/xtask",
]
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tracing = "0.1"

[workspace.lints.rust]
unsafe_code = "deny"
```

Member use:

```toml
[package]
name = "application"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
serde.workspace = true

[lints]
workspace = true
```

Workspace inheritance centralizes policy; it does not make member APIs private
or enforce architecture. Tool support for specific inheritable fields depends
on Cargo version, so keep the workspace toolchain explicit.

## Choosing Crate Boundaries

Create a crate boundary when it buys an independent property:

| Boundary reason | Evidence |
|-----------------|----------|
| Different dependency direction | Domain must not import adapter stack |
| Different target/profile | `no_std` core versus `std` integration |
| Different release/publication unit | Stable public API versus internal app |
| Different build trust | Proc macro/build tool isolated from runtime library |
| Different ownership/review | Security-sensitive parser with named owners |
| Replaceable backend | Adapter implements a stable port |

Do not create a crate only because a folder grew. Every crate adds manifest
policy, compilation units, version coordination, and public-boundary decisions.

```
module: cheap lexical boundary inside one crate
crate:  compile, visibility, dependency, feature, and publication boundary
repo:   source-control and operational ownership boundary
```

Rust has no workspace-private visibility. `pub(crate)` stops at one crate.
Cross-crate APIs are public to any code that can depend on the crate, even if
the package is `publish = false`.

## Direction Before Reuse

"Shared" crates often become dependency magnets. Prefer stable concepts, not
miscellaneous helpers.

| Smell | Better shape |
|-------|--------------|
| `common` imports database, HTTP, config, and domain types | Split stable primitives from adapters |
| Domain takes concrete client from an SDK crate | Define a narrow domain port; adapter owns SDK |
| All crates depend on one giant prelude | Explicit imports and purpose-specific types |
| Facade re-exports every dependency | Re-export only intentional public contract |
| Test helpers force production dependencies | Dev-only support crate or local test module |

Example port:

```rust
pub trait RouteStore {
    type Error;
    fn load(&self, id: RouteId) -> Result<Route, Self::Error>;
}
```

The SQL adapter depends on this trait and implements it. The domain does not
depend on SQL, the SQL client crate, or its feature graph.

## Dependency Policy in the Manifest

Use workspace dependencies for consistency, but remember inheritance is not a
global override. A member opts in with `.workspace = true`.

```toml
[workspace.dependencies]
domain = { path = "crates/domain", version = "0.4.0" }
serde = { version = "1.0.210", default-features = false }
```

```toml
[dependencies]
domain.workspace = true
serde = { workspace = true, features = ["derive", "std"] }
```

The member can add features to an inherited dependency but cannot contradict
every inherited field. Inspect the resolved graph rather than assuming central
declaration means central activation.

## Architecture Evidence

```text
cargo metadata --format-version 1
cargo tree --workspace
cargo tree -p domain
cargo tree -i adapter-sql
```

A lightweight policy table is often more durable than a tool-specific rule:

| From layer | May depend on | Must not depend on |
|------------|---------------|--------------------|
| primitives | `core`, selected leaf crates | domain, adapters, apps |
| domain | primitives | adapters, runtimes, apps |
| application | domain, primitives | concrete infrastructure unless intentionally coupled |
| adapters | domain ports, vendor SDKs | apps |
| apps | all composition layers | N/A; app is composition root |
| tools | tool libraries and metadata | Runtime crates unless intentionally shared |

Automated graph checks can enforce this, but keep the human-readable policy as
the authority and pin the checker version.

## Old World -> New World Bridge

The universal bridge is layered architecture expressed as package edges.

| Familiar architecture unit | Cargo workspace unit |
|----------------------------|----------------------|
| Solution/repository | Workspace |
| Project/assembly | Package with one or more crate targets |
| Internal namespace/module | Rust module; not a separate package |
| Interface project | Domain port/types crate |
| Infrastructure project | Adapter crate |
| Directory-wide package policy | `[workspace.dependencies]`, inherited fields/lints |

For MSBuild readers, a workspace resembles a solution plus central props, but
Cargo's shared lockfile and feature unification add graph-wide behavior that
project references alone do not capture.

## Common Confusion Points

- **"Workspace membership permits access."** Rust visibility remains crate
  scoped; packages depend explicitly.
- **"More crates means better modularity."** Boundaries without independent
  policy create coordination cost.
- **"`publish = false` makes APIs private."** It blocks registry publication,
  not source-level consumers.
- **"Central dependency declaration fixes one version."** The resolver and
  member requirements still decide the graph.
- **"A DAG is clean architecture."** Direction and boundary semantics matter,
  not only absence of cycles.

## Decision Cheat Sheet

| Need | Boundary |
|------|----------|
| Hide implementation inside one package | Module |
| Enforce compile-time dependency direction | Separate crate |
| Support `no_std` core and `std` adapters | Separate crates with inward flow |
| Swap infrastructure | Domain port plus adapter crate |
| Share versions/lints/metadata | Workspace inheritance |
| Publish stable API but keep apps internal | Public library package plus `publish = false` app packages |

## Primary Sources

- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Workspace inheritance: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-package-table
- Cargo manifests: https://doc.rust-lang.org/cargo/reference/manifest.html
- Cargo metadata: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- Rust visibility: https://doc.rust-lang.org/reference/visibility-and-privacy.html

## Related Guides

- Previous: [04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md](04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md)
- Next: [06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md](06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md)
- Internal/public boundaries: [14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md](14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md)
