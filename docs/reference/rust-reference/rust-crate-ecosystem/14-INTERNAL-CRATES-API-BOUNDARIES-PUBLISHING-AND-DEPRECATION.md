---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:internal-crates-api-publishing-deprecation
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Internal Crates, API Boundaries, Publishing, and Deprecation
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md
canonical_path: rust-crate-ecosystem/14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:14-internal-crates-api-publishing-deprecation]
concepts: [internal crates, rust API boundaries, cargo publish, deprecation, workspace publishing]
root_concepts: [rust crate API design]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Internal Crates, API Boundaries, Publishing, and Deprecation

## The Big Picture

A workspace boundary is not an API privacy boundary. Once one crate depends on
another, cross-crate items are public Rust API to that consumer. Publication
adds a distribution contract, but internal packages already need intentional
interfaces and migration rules.

```
+===========================================================================+
|                            CRATE API LIFECYCLE                            |
+===========================================================================+
| module-private -> pub(crate) -> cross-crate public -> published API       |
| implementation -> one crate -> internal migration -> SemVer policy        |
|                                                                           |
| publish = false controls registry publication, not Rust visibility.       |
+===========================================================================+
```

Design the narrowest stable boundary that supports independent compilation,
ownership, target policy, or replacement.

## Internal Package Configuration

```toml
[package]
name = "route-internal-storage"
version = "0.1.0"
publish = false
edition.workspace = true
rust-version.workspace = true
license.workspace = true
```

`publish = false` prevents accidental publication through Cargo. It does not
prevent another repository from using a path or git dependency if source access
exists.

| Boundary | Visibility/control |
|----------|--------------------|
| Private module item | Only parent/child visibility rules |
| `pub(crate)` | Anywhere in one crate |
| `pub` in internal package | Any dependent crate |
| `publish = false` | Cargo publication disabled |
| Private registry | Distribution/auth boundary, not language visibility |

There is no `pub(workspace)` stable visibility. Use crate boundaries only when
the public interface is acceptable within the consuming trust boundary.

## Public API Surface

Public API includes more than explicitly documented functions.

| Surface | Compatibility concern |
|---------|-----------------------|
| Public structs/enums/traits/functions | Direct source compatibility |
| Public trait implementations | Coherence and downstream behavior |
| Re-exported dependency types | Couples downstream API to dependency version |
| Feature names/defaults | Build and behavior compatibility |
| Error types | Pattern matching and source-chain expectations |
| Macro-generated API | Expansion can expose hidden dependencies/paths |
| MSRV/targets | Build compatibility |
| Serialization/wire formats | Data compatibility beyond Rust types |

Prefer newtypes, owned domain types, and narrow traits at boundaries. Returning a
vendor SDK type exports that SDK's version and feature decisions.

## Path Plus Version for Publishable Workspaces

```toml
[dependencies]
route-types = { path = "../route-types", version = "0.4.0" }
```

The path supports local workspace development. The version requirement is used
when publishing so registry consumers can resolve the dependency. Before
publishing, every non-development dependency that remains in the packaged
manifest must be resolvable under the target registry's rules. crates.io
packages cannot depend on alternate-registry packages.

Publish in dependency order:

```
route-types -> route-core -> route-client -> route-cli
```

Commands:

```text
cargo package -p route-types
cargo package -p route-types --list
cargo publish -p route-types --dry-run
```

A dry run validates packaging mechanics against current local/registry state; it
does not prove legal, security, or API readiness.

## Package Contents

Cargo determines package contents from manifest include/exclude rules, source
control state, and packaging conventions.

```toml
[package]
include = [
  "src/**",
  "Cargo.toml",
  "README.md",
  "LICENSE-*",
]
```

Inspect:

```text
cargo package --list
cargo package --allow-dirty
```

Use `--allow-dirty` only for deliberate local inspection; a clean release should
not depend on uncommitted state. Verify that generated files, license texts,
fixtures, native sources, and README links required by the package are present.

## Deprecation

Rust deprecation communicates migration at compile time:

```rust
#[deprecated(
    since = "0.8.0",
    note = "use RouteClient::connect_with instead"
)]
pub fn connect_legacy() {
    // compatibility implementation
}
```

Deprecation is a staged contract, not a warning dump.

```
announce replacement
      |
      v
ship replacement + migration examples
      |
      v
deprecate old API for stated window
      |
      v
measure/update internal consumers
      |
      v
remove in allowed breaking release
```

| Change | Internal crate | Published crate |
|--------|----------------|-----------------|
| Rename API | Coordinate consumers in one change if atomic | Deprecate/migrate under SemVer policy |
| Remove feature | Update all workspace consumers and support profiles | Usually breaking; document replacement |
| Split crate | Workspace migration | Publish compatibility facade if needed |
| Raise MSRV | Product policy decision | Public support-policy change |
| Yank version | Rarely relevant internally | Prevent new selection; existing locks remain |

## Publishing Controls

```text
release approval
   |
   +-> clean source and reviewed diff
   +-> package list and license/notices
   +-> tests/MSRV/targets/features
   +-> advisory/source policy
   +-> changelog/version
   +-> least-privilege registry credential
   +-> post-publish verification and rollback plan
```

Registry releases are generally immutable; publishing a corrected version and
yanking a bad one are typical recovery mechanisms. Yanking does not delete the
crate or break existing lockfiles automatically.

## Old World -> New World Bridge

| Familiar boundary | Rust/Cargo boundary |
|-------------------|----------------------|
| Internal assembly/project | `publish = false` package |
| `internal` visibility/friend assembly | No workspace equivalent; redesign boundary or keep one crate |
| Public NuGet package | Published crate |
| Obsolete attribute | `#[deprecated]` |
| Central package release train | Workspace dependency-order publish pipeline |
| Public DTO leaked from dependency | Re-exported/vendor type coupling |

The universal API principle remains: what crosses a component boundary becomes
someone else's dependency, even before public publication.

## Common Confusion Points

- **"`publish = false` makes a crate private."** It only blocks Cargo
  publication.
- **"`pub` is harmless inside one repository."** It is a cross-crate contract
  immediately.
- **"Re-export saves coupling."** It can hide imports while increasing semantic
  coupling to the re-exported crate.
- **"Yank is rollback."** Existing lockfiles can continue using a yanked version.
- **"Dry-run means release-ready."** It checks packaging/publish mechanics, not
  the complete governance record.

## Decision Cheat Sheet

| Need | Prefer |
|------|--------|
| Hide implementation details | Keep them in one crate with private/`pub(crate)` modules |
| Enforce dependency direction | Separate crate with narrow `pub` API |
| Prevent accidental public publication | `publish = false` |
| Develop locally and publish later | Path plus version dependency |
| Change published API | Replacement, deprecation window, SemVer release |
| Recover bad release | Publish fixed version; yank when appropriate; notify consumers |

## Primary Sources

- Cargo publishing: https://doc.rust-lang.org/cargo/reference/publishing.html
- Cargo package command: https://doc.rust-lang.org/cargo/commands/cargo-package.html
- Manifest package fields: https://doc.rust-lang.org/cargo/reference/manifest.html
- Rust deprecation attribute: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

## Related Guides

- Previous: [13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md](13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md)
- Next: [15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md](15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md)
- Workspace direction: [05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md](05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md)
