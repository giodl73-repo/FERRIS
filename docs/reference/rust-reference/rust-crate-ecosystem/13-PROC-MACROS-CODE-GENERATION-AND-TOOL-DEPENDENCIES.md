---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:proc-macros-code-generation-tool-dependencies
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Proc Macros, Code Generation, and Tool Dependencies
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md
canonical_path: rust-crate-ecosystem/13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:13-proc-macros-code-generation-tool-dependencies]
concepts: [procedural macros, code generation, build dependencies, xtask, host tools]
root_concepts: [rust code generation]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Proc Macros, Code Generation, and Tool Dependencies

## The Big Picture

Rust builds can execute code before the product runs. Procedural macros,
build scripts, generators, and repository tools have different interfaces, but
all expand the host-side trust and reproducibility boundary.

```
+===========================================================================+
|                       BUILD-TIME EXECUTION                                |
+===========================================================================+
|                                                                           |
| proc-macro crate -> host dylib -> rustc token expansion -> target source  |
| build.rs         -> host exe   -> files/cfg/link args    -> target build  |
| generator CLI    -> host exe   -> committed/OUT_DIR files                |
| xtask/tool       -> host exe   -> repository workflow/release artifacts   |
|                                                                           |
| Inputs + tool version + environment -> generated result -> product graph  |
+===========================================================================+
```

Review host-executed dependencies at least as carefully as runtime libraries.
They can read files, environment variables, and credentials available to the
build process unless the environment restricts them.

## Procedural Macros

A proc-macro package compiles for the host and exports compiler-loaded macro
entry points.

```toml
[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1"
quote = "1"
syn = { version = "2", features = ["derive"] }
```

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Route)]
pub fn derive_route(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl #name {
            pub fn route_name() -> &'static str {
                stringify!(#name)
            }
        }
    }
    .into()
}
```

Keep parsing and semantic logic in normal functions where possible. This makes
the transformation unit-testable without loading it as a compiler plugin.

| Proc-macro concern | Review question |
|--------------------|-----------------|
| Token handling | Does it preserve spans and produce actionable diagnostics? |
| API stability | Is generated public code part of the downstream SemVer surface? |
| Compiler/MSRV | Which stable compiler range is tested? |
| Performance | Does expansion dominate clean/incremental builds? |
| Security | What source/config/environment does the macro read? |
| Debuggability | Can users inspect or reason about generated code? |

The stable public interface is `proc_macro`; compiler internals beneath it are
version-sensitive. Avoid relying on nightly expansion internals in supported
workflows.

## Separate Facade, Runtime, and Macro Crates

Proc-macro crates have special crate-type constraints. A common ecosystem shape:

```
example-core      normal library: traits/types/runtime helpers
example-derive    proc-macro: token transformation
example           facade: re-exports stable user surface
```

This separation:

- keeps runtime users from importing parser/generator dependencies unless needed;
- gives macro and runtime layers independent internal structure;
- makes generated paths explicit;
- prevents a proc-macro crate from becoming the only home for runtime API.

Do not re-export everything automatically. Re-exports become public API and can
couple release versions.

## Build-Time Code Generation

Generated artifacts have three common custody models.

| Model | Location | Prefer when | Main risk |
|-------|----------|-------------|-----------|
| Generate in `OUT_DIR` | Build output | Input/tool are available and deterministic | Harder review/debug, host execution every build |
| Commit generated source | Repository | Reviewability and downstream simplicity matter | Drift between source schema and output |
| Generate release artifact | Packaging pipeline | Consumers should not run generator | Release process becomes authority |

For `OUT_DIR`:

```rust
include!(concat!(env!("OUT_DIR"), "/protocol.rs"));
```

Build scripts should not rewrite tracked source files. Declare inputs with
`rerun-if-changed` and make output deterministic for the supported environment.

## Tool Dependencies and `xtask`

Cargo has no dedicated general "tool dependency" section for repository CLIs.
Common patterns:

| Pattern | Example | Tradeoff |
|---------|---------|----------|
| Workspace tool package | `tools/xtask` | Versioned with repo, normal Cargo graph |
| Pinned external binary install | `cargo install tool --version X --locked` | Bootstrap and binary cache needed |
| Toolchain component | rustfmt/clippy | Coupled to selected Rust toolchain |
| Container/dev image | Preinstalled tool versions | Image becomes reproducibility boundary |

`xtask` pattern:

```toml
[workspace]
members = ["crates/*", "tools/xtask"]
```

```text
cargo run -p xtask -- codegen
cargo run -p xtask -- package
```

An `xtask` is a convention, not a Cargo feature. Keep it out of runtime
dependency direction and give it explicit input/output contracts.

## Inspecting and Testing Generated Behavior

```
input fixtures -> transformation -> expected tokens/files -> compile test
                                      |
                                      v
                            diagnostics/failure fixtures
```

| Test | What it proves |
|------|----------------|
| Parser/unit test | Transformation logic on explicit inputs |
| Snapshot test | Generated shape remains reviewed; requires intentional updates |
| Compile-pass test | Generated code compiles under named toolchain |
| Compile-fail test | Diagnostics and invalid inputs behave as intended |
| Re-run determinism | Same inputs/tool produce same normalized output |
| Cross-target check | Generated code respects target cfg and API availability |

Third-party compile-test/snapshot/expansion tools are themselves tool
dependencies. Pin them and record whether they require nightly. For direct
inspection, stable rustdoc/source design is preferable to making a nightly-only
expansion command part of the support contract.

## Build Environment Controls

```text
CI build identity
  |
  +-> minimal credentials
  +-> read-only source checkout where possible
  +-> controlled network
  +-> pinned toolchain and tool dependencies
  +-> clean generated-output comparison
```

Sandbox behavior differs by operating system and CI platform. Say which
permissions are actually restricted rather than claiming a generic sandbox.

## Old World -> New World Bridge

| Familiar mechanism | Rust mechanism |
|--------------------|----------------|
| Compiler annotation processor | Procedural macro |
| Source generator | Proc macro or generator CLI/build script |
| MSBuild task/target | `build.rs` or `xtask` |
| T4/codegen template | Committed or `OUT_DIR` generation |
| Build-only NuGet package | Build dependency or tool package |

The universal distinction is compile-time execution versus runtime linkage.
Cargo makes both appear in one dependency graph, so policy must label them.

## Common Confusion Points

- **"Proc macros are just syntax."** They are host-executed programs that emit
  syntax.
- **"Build dependency cannot affect runtime."** It can generate code, cfg,
  linker arguments, or embedded data.
- **"Committed generated code needs no generator policy."** Regeneration and
  source provenance still matter.
- **"`xtask` is built into Cargo."** It is a workspace convention.
- **"A passing macro test proves diagnostics."** Test invalid inputs and spans,
  not only happy-path expansion.

## Decision Cheat Sheet

| Need | Prefer |
|------|--------|
| Derive boilerplate from Rust syntax | Small proc macro with compile tests |
| Generate from external schema | Explicit generator with pinned inputs/tool |
| Maximize consumer transparency | Commit generated source or publish generated artifact |
| Keep build logic maintainable | `xtask` for workflows; `build.rs` only for package build needs |
| Reduce host trust | Avoid network, credentials, and opaque generators during build |
| Support multiple targets | Cross-target compile tests for emitted code |

## Primary Sources

- Rust procedural macros: https://doc.rust-lang.org/reference/procedural-macros.html
- Cargo build dependencies: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies
- Cargo build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html
- Rust macro diagnostics/API: https://doc.rust-lang.org/proc_macro/
- Cargo package targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html

## Related Guides

- Previous: [12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md](12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md)
- Next: [14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md](14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md)
- Language macro model: [../rust-language/13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md](../rust-language/13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md)
