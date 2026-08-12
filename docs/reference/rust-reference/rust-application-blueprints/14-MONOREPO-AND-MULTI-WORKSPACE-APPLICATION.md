---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:monorepo-and-multi-workspace-application
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Monorepo and Multi-Workspace Application Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/14-MONOREPO-AND-MULTI-WORKSPACE-APPLICATION.md
canonical_path: rust-application-blueprints/14-MONOREPO-AND-MULTI-WORKSPACE-APPLICATION.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:14-monorepo-and-multi-workspace-application]
concepts: [monorepo, cargo workspace, multi-workspace, dependency graph, release train, repository ownership, affected testing]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Monorepo and Multi-Workspace Application Blueprint

## The Big Picture

```
+============================================================================+
| repository authority                                                       |
| ownership | policy | dependency updates | CI graph | release records       |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| Cargo coordination                                                         |
| one workspace graph OR several explicit workspace graphs                   |
+----------------------+----------------------+------------------------------+
                       v                      v
                 shared crates          apps/services/tools
                       |                      |
                       +-----------+----------+
                                   v
                  independently named build/deploy/release units
```

A monorepo is a source and change-coordination boundary. A Cargo workspace is a
dependency resolution, lockfile, target-directory, and command-selection
boundary. They often coincide, but they are not synonyms. A repository may hold
one workspace, several workspaces, or Rust plus other build systems.

## One-Workspace Layout

```
platform/
|-- Cargo.toml
|-- Cargo.lock
|-- crates/
|   |-- domain-a/
|   |-- domain-b/
|   `-- protocol-primitives/
|-- apps/
|   |-- api/
|   |-- worker/
|   `-- cli/
|-- tools/
|   `-- xtask/
|-- tests/
|   `-- system-scenarios/
`-- ops/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tools/*", "tests/*"]
default-members = ["apps/api", "apps/worker", "apps/cli"]

[workspace.dependencies]
protocol-primitives = { path = "crates/protocol-primitives" }

[workspace.lints.rust]
unsafe_code = "deny"
```

Use workspace inheritance for policy that is genuinely common. A workspace
dependency declaration centralizes versions; each member still opts into the
dependency with `dependency-name.workspace = true`. Workspace lints likewise
require the member manifest to declare:

```toml
[lints]
workspace = true
```

## Multi-Workspace Layout

Use multiple workspaces when graphs require materially different lockfiles,
toolchains, targets, release authority, or dependency policy:

```
platform/
|-- workspaces/
|   |-- server/
|   |   |-- Cargo.toml
|   |   `-- Cargo.lock
|   |-- firmware/
|   |   |-- Cargo.toml
|   |   `-- Cargo.lock
|   `-- tools/
|       |-- Cargo.toml
|       `-- Cargo.lock
|-- shared-source/
|-- contracts/
`-- repo-tools/
```

Each package should belong to one Cargo workspace. Do not attempt overlapping
workspace membership. Cross-workspace sharing needs a deliberate package path,
published/git dependency, generated contract, or duplicated neutral source with
clear authority.

A cross-workspace path dependency is technically convenient but can recreate a
hidden lockstep graph without shared lockfile or root commands. Give it a named
compatibility/release policy or publish/version the boundary.

| Shape | Prefer when |
|-------|-------------|
| One workspace | compatible toolchain/target graph and coordinated lockfile |
| Multiple workspaces | firmware/server/tool graphs need independent policy |
| Multiple repositories | access, lifecycle, or authority must be independent |

## Dependency and Ownership Rules

```
apps ------> application crates ------> domain crates
  |                  |
  +--> adapters -----+

forbidden:
domain-a ---> app-b
service-a private model ---> service-b
generic "common" ---> every layer
```

| Policy | Evidence |
|--------|----------|
| Dependency direction | metadata/graph check plus review |
| Ownership | path-level owner and review rule |
| Unsafe/FFI | named crates and stricter audit |
| Version/MSRV | workspace or per-workspace CI matrix |
| Generated contracts | source owner and reproducible generation check |
| Releases | artifact-to-commit and dependency record |

Repository ownership is not runtime authority. A team may review a path while a
different service remains the sole production data owner; record both.

Path ownership is also not an access-control boundary. Proc macros, build
scripts, native tools, and repository automation can execute in CI under its
credentials. Partition release secrets and deployment identities by artifact,
review changes to executable build logic, and prevent an unrelated package from
receiving production credentials merely because it shares the repository.

## CI, Affected Testing, and Release

```
changed paths
   |
   v
map to packages/contracts/ops
   |
   v
reverse dependency closure
   |
   +--> fast affected checks
   `--> required global/integration gates
```

An affected-only system is an optimization, not proof that unrelated code cannot
be impacted. Workspace resolution, shared lockfiles, features, build scripts,
toolchain changes, and contract changes can widen the closure.

```text
cargo metadata --format-version 1
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

For multiple workspaces, run commands from each workspace root with its pinned
toolchain. Do not assume a root `cargo test --workspace` traverses them all. Add
an explicit supported feature matrix; `--all-features` is a valid gate only when
those features are designed to coexist.

| Release model | Rollback |
|---------------|----------|
| One application cohort | revert cohort artifact and compatible state |
| Independent binaries | retain per-artifact versions and contract matrix |
| Published libraries | publish corrected versions; cannot rewrite history |
| Firmware plus service | verify cross-workspace protocol compatibility |

Removing a package or workspace requires reverse-dependency and consumer
evidence, release/artifact retirement, cache and CI graph cleanup, credential
revocation, and preservation or transfer of any runtime data authority. Deleting
the directory first destroys the evidence needed to prove removal is safe.

## Universal Bridge First

The universal bridge is build-graph governance: source co-location reduces
change latency, while explicit package and release boundaries prevent the graph
from becoming one authority. Large C/C++ trees, Java builds, and polyglot
monorepos face the same source-versus-artifact distinction.

Supplementally, a Visual Studio solution and MSBuild project graph resemble a
repository view plus build graph. Cargo's workspace resolver and feature
unification make the exact dependency behavior different; inspect it rather
than assuming configuration semantics transfer.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Shared lockfile and coordinated Rust graph | one Cargo workspace |
| Firmware, server, tools with incompatible policies | multiple explicit workspaces |
| Independent access/security/lifecycle | separate repositories |
| Common versions/lints | workspace inheritance |
| Shared service behavior | protocol contract, not private domain crate |
| Faster CI | affected graph plus non-skippable global gates |
| Independent rollback | artifact and contract records per deployable |

## Common Confusion Points

- **Monorepo does not imply monolith.** Source can be co-located while services
  deploy independently.
- **Workspace does not imply one package.** It coordinates many packages.
- **A virtual workspace has no root package.** Commands need member/default
  selection.
- **Features unify within resolution.** A member may receive a feature enabled
  by another graph path.
- **One root command does not cover multiple workspaces.** Repository
  orchestration must enumerate them.
- **Shared code can erase service authority.** Reuse is not automatically worth
  lockstep evolution.
- **A monorepo is not a trust zone.** Build-time code and CI credentials still
  need least-privilege boundaries.

## Primary Sources

- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Cargo Features: https://doc.rust-lang.org/cargo/reference/features.html
- Cargo `metadata`: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- Cargo resolver: https://doc.rust-lang.org/cargo/reference/resolver.html
- Cargo Profiles: https://doc.rust-lang.org/cargo/reference/profiles.html

## Related Guides

- Distributed deployables: [13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md](13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md)
- Blueprint evolution: [15-BLUEPRINT-SELECTION-AND-EVOLUTION.md](15-BLUEPRINT-SELECTION-AND-EVOLUTION.md)
