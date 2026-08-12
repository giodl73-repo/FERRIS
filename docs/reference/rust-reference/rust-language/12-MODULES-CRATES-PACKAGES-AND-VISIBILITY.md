---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:modules-crates-packages-and-visibility
kind: guide
module: rust-language
section: languages
title: Modules, Crates, Packages, and Visibility
status: source-custody
source_custody: partial
current_path: rust-language/12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md
canonical_path: rust-language/12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md
backsource_ids: [mdloom-backfill:rust-language:12-modules-crates-packages-and-visibility]
concepts: [modules, crates, packages, workspaces, visibility, pub, use, crate root, prelude, re-exports, API layout]
root_concepts: [modules]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Modules, Crates, Packages, and Visibility

Rust's code-organization vocabulary — module, crate, package, workspace — is
precise and worth pinning down, because the words overlap with other ecosystems'
usage. A **crate** is the compilation unit; a **module** is a namespace *inside* a
crate; a **package** is a Cargo bundle of one or more crates; a **workspace**
groups packages. Visibility is private-by-default with `pub` opting into
exposure. This layer is where you shape your public API surface.

```
+===============================================================================+
|              PACKAGE > CRATE > MODULE  (three nesting levels)                 |
+===============================================================================+

  WORKSPACE  (Cargo.toml [workspace])          many packages, shared lock/target
  +-------------------------------------------------------------------+
  |  PACKAGE  (one Cargo.toml)                                        |
  |  +-------------------------------------------------------------+  |
  |  |  CRATE = compilation unit (rustc compiles ONE crate)        |  |
  |  |    - lib crate:  src/lib.rs   (0 or 1 per package)          |  |
  |  |    - bin crate:  src/main.rs, src/bin/*.rs (0..N per pkg)   |  |
  |  |  +-----------------------------------------------------+    |  |
  |  |  |  MODULE tree (namespaces inside the crate)          |    |  |
  |  |  |   crate (root) :: net :: http :: Client             |    |  |
  |  |  |   mod net { mod http { pub struct Client; } }       |    |  |
  |  |  +-----------------------------------------------------+    |  |
  |  +-------------------------------------------------------------+  |
  +-------------------------------------------------------------------+

  VISIBILITY               PATHS                       BRING INTO SCOPE
  ----------               -----                       ----------------
  (default)  private       crate::net::http::Client    use crate::net::http::Client;
  pub        public         self::sibling               use std::collections::HashMap;
  pub(crate) crate-wide     super::parent               use x::{A, B, C};
  pub(super) parent module  ::extern_crate::Item        pub use  (RE-EXPORT)
```

## Crate: The Compilation Unit

`rustc` compiles one **crate** at a time. A crate is either a **library** (root
`src/lib.rs`) or a **binary** (root with a `main`). Cargo's default library
artifact is a Rust `rlib`; optional `dylib`, `cdylib`, and `staticlib` crate
types select different linking contracts and target-specific file extensions.
The crate root file is the top of the module tree; everything else is reached by
module paths from it.

## Package and Workspace

A **package** is what `Cargo.toml` describes: metadata plus at most one library
crate and any number of binary crates. Conventions:

```
mypackage/
  Cargo.toml            # the package manifest
  src/lib.rs            # the library crate root (optional)
  src/main.rs           # a binary crate root (optional)
  src/bin/tool.rs       # an additional binary crate
  tests/                # each file = a separate integration-test crate
  benches/  examples/   # bench and example crates
```

A **workspace** ties multiple packages together with a shared `Cargo.lock` and
`target/` directory — the equivalent of a multi-project solution:

```toml
# top-level Cargo.toml
[workspace]
members = ["core", "cli", "server"]
resolver = "2"
```

Workspaces let a `cli` package depend on a `core` package by path, all built and
tested together with one `cargo test`.

## Modules: Namespaces Inside a Crate

Modules form a tree rooted at the crate root. Declare a module inline or point at
a file:

```rust
// src/lib.rs
pub mod net;           // loads src/net.rs OR src/net/mod.rs

mod util {             // inline module
    pub fn helper() {}
    pub(crate) fn internal() {}
}
```

Paths navigate the tree: `crate::` (absolute from root), `self::` (current
module), `super::` (parent). Bring names into scope with `use`:

```rust
use crate::net::http::Client;
use std::collections::{HashMap, HashSet};
use std::io::Write as _;                    // import trait for its methods, no name
```

## Visibility: Private by Default

Everything is **private to its module** unless marked `pub`. Privacy is by module
subtree — a child can see its ancestors' private items, but not vice versa. The
graduated `pub` forms let you widen exactly as far as needed:

| Modifier | Visible to | Use for |
|----------|-----------|---------|
| (none) | current module + descendants | internal helpers |
| `pub` | everyone (if the path is reachable) | the public API |
| `pub(crate)` | anywhere in this crate | cross-module internals |
| `pub(super)` | the parent module | sibling collaboration |
| `pub(in path)` | a specific ancestor module | precise scoping |

Note a subtlety: `pub` on an item only matters if every module on its path is
also reachable. A `pub fn` inside a private `mod` is not externally callable — the
module gate closes first.

## Preludes and Re-exports (Shaping the API)

The **prelude** is the set of names auto-imported into every module
(`Option`, `Result`, `Vec`, `String`, common traits). You cannot easily extend
the std prelude, but crates ship their own `mycrate::prelude` module that users
`use mycrate::prelude::*`.

**Re-exports** (`pub use`) are the primary API-design tool: implement code in
deeply nested private modules, then surface a flat, curated public path.

```rust
// src/lib.rs
mod engine { pub mod core { pub struct Session; } }
mod config { pub struct Settings; }

pub use engine::core::Session;   // users write `mycrate::Session`, not the deep path
pub use config::Settings;        // internal layout is free to change
```

This decouples internal file structure from the public API: reorganize modules
freely as long as the re-exported facade stays stable ([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)).
`cargo add` / crates.io dependencies and features are covered in
[01](01-TOOLCHAIN-AND-WORKFLOW.md) and [18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md).

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| .NET assembly (`.dll`) | crate | The unit of compilation and linking |
| .NET solution (`.sln`) | Cargo workspace | Groups projects/packages |
| .NET project (`.csproj`) | package (`Cargo.toml`) | Metadata + one lib + N bins |
| `namespace` (C#) | `mod` | Namespaces, but file-tree-linked |
| `internal` (C#) | `pub(crate)` | Crate-wide visibility |
| `public`/`private` | `pub`/(default) | Private is the default in Rust |
| Java package = directory | `mod` = file/dir | Similar file mapping |
| `using`/`import` | `use` | Same idea; supports grouping and `as` |
| C# `[assembly:InternalsVisibleTo]` | `pub(crate)` + workspace paths | Different mechanism |

The cleanest bridge for a .NET reader: **crate ~= assembly**, **package ~= project**,
**workspace ~= solution**, **`mod` ~= namespace**, **`pub(crate)` ~= `internal`**.
The one genuinely different default is private-by-default at the module level.

## Common Confusion Points

- **"crate vs package."** A package (`Cargo.toml`) can contain multiple crates
  (one lib + several bins); a crate is the thing `rustc` compiles. People say
  "crate" loosely to mean "library on crates.io," which is a package with one lib
  crate.
- **`pub` inside a private module.** Marking an item `pub` does nothing if an
  ancestor module is private — the path is unreachable. Re-export it (`pub use`)
  to surface it.
- **`mod foo;` vs `use foo;`.** `mod` *declares/loads* a module (defines the
  tree); `use` merely *brings a path into scope*. Declaring a module twice, or
  forgetting to declare it, is a common error.
- **Each `tests/*.rs` is its own crate.** Integration tests compile as separate
  crates that link your library as an external dependency — they only see `pub`
  items ([19](19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md)).
- **`src/bin/` for extra binaries.** Multiple executables live under `src/bin/`,
  each a crate; run with `cargo run --bin name`.
- **Workspace `resolver`.** Editions changed the default feature resolver; set
  `resolver = "2"` (or `"3"`) explicitly at the workspace root ([18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)).

## Decision Cheat Sheet

| I want to... | Do |
|--------------|-----|
| Split code into namespaces | `mod` (inline or file) |
| Expose an item publicly | `pub` (ensure the whole path is reachable) |
| Share internals across modules only | `pub(crate)` |
| Curate a flat public API | `pub use` re-exports facade |
| Offer a convenient import bundle | ship a `prelude` module |
| Group related packages | a Cargo `[workspace]` |
| Add an extra executable | `src/bin/<name>.rs` |
| Depend on a sibling package | path dependency in the workspace |
| Import a trait's methods only | `use Trait as _;` |

## Primary Sources

- The Book, Ch. 7 (Managing Growing Projects with Packages, Crates, and Modules): https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Reference — Crates and source files: https://doc.rust-lang.org/reference/crates-and-source-files.html
- Reference — Visibility and privacy: https://doc.rust-lang.org/reference/visibility-and-privacy.html
- The Cargo Book — Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Rust API Guidelines (naming, re-exports): https://rust-lang.github.io/api-guidelines/

## Related Guides

- Previous: [11-ERRORS-RESULT-OPTION-AND-PANIC.md](11-ERRORS-RESULT-OPTION-AND-PANIC.md)
- Next: [13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md](13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md)
- Features & editions: [18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)
- Public API design: [20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)
