---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:cargo-build-scripts-proc-macros-native
kind: guide
module: rust-architecture
section: rust-architecture
title: Build Scripts, Proc Macros, and Native Tool Integration
status: source-custody
source_custody: partial
current_path: rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md
canonical_path: rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md
backsource_ids: [mdloom-backfill:rust-architecture:18-cargo-build-scripts-proc-macros-native]
concepts: [build scripts, proc macros, host target split, native libraries, links key, cross compilation]
root_concepts: [build scripts]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Build Scripts, Proc Macros, and Native Tool Integration

## The Big Picture

The organizing idea is the host/target split. Some code must run on the build
machine at compile time: build scripts, procedural macros, code generators, C
probes, bindgen-style tools. Cargo compiles those units for the **host**. The
crate you are shipping is compiled for the **target**. On a normal laptop build
host and target are often the same triple; in cross-compilation they diverge.

```
+===========================================================================+
|                         CARGO HOST/TARGET BUILD                           |
+---------------------------------------------------------------------------+
| HOST SIDE: runs during the build                                           |
|                                                                           |
|  build.rs source  -> rustc(host) -> build-script exe -> prints cargo::*   |
|  proc-macro crate -> rustc(host) -> special dylib   -> loaded by rustc    |
|  cc/pkg-config/vcpkg probes run here                                      |
+---------------------------------------------------------------------------+
                  v  stdout directives, env, generated files, macro tokens
+---------------------------------------------------------------------------+
| TARGET SIDE: product being built                                           |
|                                                                           |
|  lib/bin/test crate -> rustc(target) -> rlib/bin/cdylib/staticlib         |
|  links to native libraries discovered by host-side build logic            |
+---------------------------------------------------------------------------+
                  v  selected by rustup target std/sysroot, see [02]
+---------------------------------------------------------------------------+
| EXAMPLE: host=x86_64-pc-windows-msvc, target=wasm32-unknown-unknown       |
+===========================================================================+
```

Cargo owns this split in the unit graph. `rustc` owns compilation once invoked.
`rustup` supplies toolchains and target standard libraries, see
[02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md). Native registries and
system package managers may supply C libraries, but Cargo's build-script
protocol is the stable bridge into the Rust build.

---

## Build Scripts: The Stable Stdout Protocol

A `build.rs` file is compiled for the host and run before the package's normal
targets. It communicates with Cargo by printing directives to stdout. Modern
Cargo accepts the `cargo::` prefix; older Cargo used `cargo:`. The protocol is a
stable contract. The internal details of how Cargo stores the result in unit
fingerprints are not; see
[17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md).

| Directive | Effect |
|-----------|--------|
| `cargo::rustc-link-lib=foo` | Ask rustc/linker to link `foo` |
| `cargo::rustc-link-search=native=PATH` | Add a native library search path |
| `cargo::rustc-cfg=has_foo` | Add a cfg for conditional Rust compilation |
| `cargo::rustc-env=KEY=VALUE` | Set compile-time environment for `env!` |
| `cargo::rerun-if-changed=PATH` | Re-run script when a file changes |
| `cargo::rerun-if-env-changed=VAR` | Re-run script when an env var changes |
| `cargo::warning=message` | Emit a Cargo warning |
| `cargo::metadata=KEY=VALUE` | Pass metadata to dependent build scripts for `links` crates |

```toml
[package]
name = "z-wrap"
version = "0.1.0"
edition = "2021"
links = "z"
build = "build.rs"

[build-dependencies]
cc = "1"
```

```rust
// build.rs
fn main() {
    println!("cargo::rerun-if-changed=native/zshim.c");
    println!("cargo::rerun-if-env-changed=ZLIB_DIR");
    println!("cargo::rustc-check-cfg=cfg(has_zlib)");

    cc::Build::new()
        .file("native/zshim.c")
        .compile("zshim");

    if let Ok(dir) = std::env::var("ZLIB_DIR") {
        println!("cargo::rustc-link-search=native={dir}/lib");
    }

    println!("cargo::rustc-link-lib=z");
    println!("cargo::rustc-cfg=has_zlib");
    println!("cargo::metadata=include=include");
}
```

Generated Rust belongs under `OUT_DIR`, then the target crate includes it
explicitly:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

That line is the boundary. Build scripts may generate files, probe systems, and
compile C, but they should not silently rewrite source files.

---

## The `links` Key and `-sys` Crates

`links = "foo"` declares that a package links a native library named `foo`.
Cargo enforces at most one package per `links` value in the resolved graph. This
prevents two crates from both deciding how to link the same native library and
then handing incompatible linker flags to `rustc`.

```
+------------------+       metadata       +-------------------+
| foo-sys          | -------------------> | higher-level foo  |
| links = "foo"    | DEP_FOO_INCLUDE=...  | safe wrapper      |
| build.rs probes  |                      | no native probe   |
+------------------+                      +-------------------+
          |
          v
  one owner of native link decisions in the Cargo graph
```

The `-sys` naming convention is ecosystem practice, not a compiler rule. A
`foo-sys` crate is usually a thin FFI wrapper around native `foo`: it owns
headers, `extern "C"` declarations, and link discovery. Higher-level crates build
safe APIs above it. Cargo's `DEP_FOO_*` environment variables let the `links`
crate pass build-script metadata to direct dependents.

| Layer | Typical crate | Responsibility |
|-------|---------------|----------------|
| Native library | `libz`, OpenSSL, SQLite | C ABI and platform artifacts |
| `-sys` crate | `z-sys`, `openssl-sys`, `libsqlite3-sys` | Link/probe/generate FFI |
| Safe wrapper | `flate2`, `openssl`, `rusqlite` | Rust API and safety invariants |

---

## Native Tool Discovery and Linker Handoff

Build scripts run on the host, so discovery is host code even when producing
target artifacts. The executable doing the probing is a host program; the C
compiler, archiver, sysroot, headers, and libraries it selects must still match
`TARGET`. The `cc` crate uses Cargo's target environment to choose a cross
compiler when configured. `pkg-config` and `vcpkg` need target-aware roots and
must not accidentally report host libraries that cannot link into the target.
Cargo exposes `HOST`, `TARGET`, `OUT_DIR`, `CARGO_CFG_*`, `CARGO_FEATURE_*`,
and package metadata to make that distinction explicit.

| Input | Who reads it | Result |
|-------|--------------|--------|
| `CC_<target>`, `AR_<target>`, `CFLAGS_<target>` (or documented generic fallbacks) | `cc` crate / build.rs | Target-native objects or static archives |
| Target-aware `PKG_CONFIG_*` / sysroot settings | `pkg-config` crate | Target include paths and link flags |
| `VCPKG_ROOT` plus target triplet/configuration | `vcpkg` crate | Windows target library discovery |
| `TARGET`, `HOST` | build.rs | Branch between host and target logic |
| `cargo::rustc-link-*` | Cargo then `rustc` | Linker flags on the target unit |

```
build.rs(host)
      |
      +--> target C compiler / target sysroot / target libraries
      |
      +--> cargo::rustc-link-* -> rustc(target) -> platform linker [13]
```

`rustc` ultimately invokes the linker with the flags Cargo supplies; artifact
and linker mechanics are in [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md).

---

## Proc Macros: Host Code Loaded by the Compiler

Procedural macros are also host-side build-time code, but the boundary is
compiler-facing instead of stdout-facing. A crate with `proc-macro = true`
compiles to a special host dynamic library. During compilation of a target crate,
`rustc` loads that library and drives it over token streams. The stable public
bridge is the `proc_macro` API. The bridge implementation underneath is internal
and unstable.

```
+--------------------+       host dylib       +----------------------------+
| proc-macro crate   | ---------------------> | rustc compiling target     |
| #[proc_macro] fn   |                        | expands tokens in [05]     |
+--------------------+                        +----------------------------+
          ^                                                |
          | stable proc_macro API                          v
          +-------------------- internal bridge ---------------------------+
```

```toml
[lib]
proc-macro = true
```

```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    "pub const ANSWER: u32 = 42;".parse().unwrap()
}
```

Proc macros run arbitrary host code at build time. They can read files, inspect
environment, open sockets if allowed, or spin CPU. Treat them like compiler
plugins with supply-chain consequences. Macro expansion and hygiene are covered
in [05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md); ecosystem tooling
interaction is in [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md).

---

## Cross-Compilation Shape

`cargo build --target <triple>` adds target-side units while leaving build
scripts and proc macros on the host. The target standard library comes from the
selected rustup toolchain and installed target components. For exotic targets,
`-Z build-std` can build the standard library from source, but `-Z` means
nightly/unstable.

| Unit kind | Compiled for host? | Compiled for target? |
|-----------|--------------------|----------------------|
| build script executable | yes | no |
| build dependencies | yes | no |
| proc-macro crate | yes | no |
| normal library/bin | no, unless also used as a host dep | yes |
| tests | host when running locally; target when cross-building harnesses | depends on command/target |

```powershell
rustup target add wasm32-unknown-unknown
rustup target add aarch64-unknown-linux-gnu
cargo build --lib --target wasm32-unknown-unknown -v
cargo build --lib --target aarch64-unknown-linux-gnu -v

# Building core from source requires rust-src in the same nightly toolchain.
rustup component add rust-src --toolchain nightly
cargo +nightly build --lib -Z build-std=core --target thumbv7em-none-eabihf
```

In verbose output, look for two triples. If a proc macro appears under the host
triple while your library appears under the target triple, Cargo is doing the
right thing. A final cross-linked executable may still require a configured
target linker, sysroot, C runtime, runner, or board runtime; the `--lib` commands
above deliberately stop short of those platform-specific final-link/run steps.

---

## Old World -> New World

| Old-world concept | Rust analogue | Difference that matters |
|-------------------|---------------|-------------------------|
| MSBuild pre-build target | `build.rs` | Rust gives a narrow stdout protocol rather than arbitrary project-file graph mutation |
| Custom MSBuild task / npm preinstall | build script host executable | Same supply-chain risk: code runs during build |
| Roslyn Source Generator | proc macro | Strong parallel: host code runs in the compiler pipeline; Rust exposes tokens through `proc_macro` |
| RID/TFM cross-build | Cargo host/target split | Build tools stay host while product artifacts target another triple |
| P/Invoke/native NuGet package | `links` + `-sys` crate | Cargo enforces one link owner per native library name |
| CMake/vcpkg/pkg-config discovery | `cc`, `vcpkg`, `pkg-config` crates in build.rs | Discovery is explicit host code feeding Cargo directives |

This is not a second build system hidden inside Cargo. It is Cargo admitting
that some builds need host programs, native probes, and compiler-time code, then
standardizing the narrow seams where they influence the target build.

---

## Decision Cheat Sheet

| Question | Use / answer | Stable? |
|----------|--------------|---------|
| Need to generate Rust at build time? | `build.rs` writes into `OUT_DIR`, target uses `include!` | protocol stable |
| Need to link a native library? | `links`, `cargo::rustc-link-lib`, possibly a `-sys` crate | stable Cargo contract |
| Need a C compiler? | `cc` crate from build.rs | crate API, not Cargo core |
| Need system library discovery? | `pkg-config` or `vcpkg` crate | crate API |
| Need syntax-driven code generation? | proc macro using stable `proc_macro` | API stable; bridge internal |
| Cross-compiling? | `cargo build --target`, install target with rustup | documented command stable |
| Need to build std for an unusual target? | `-Z build-std` | unstable nightly feature |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "A build script builds my crate." | It runs before the crate and emits directives; `rustc` still compiles the crate. |
| "Proc macros are target code." | Proc macros are host dylibs loaded by `rustc` during compilation. |
| "Host equals target." | Only on native builds. Cross-compilation makes the distinction visible. |
| "`links` links the library by itself." | `links` declares ownership; build script directives supply actual flags. |
| "`cargo::rerun-if-changed` is optional bookkeeping." | Without precise rerun directives, Cargo's freshness model can be too broad or too stale. |
| "The proc-macro bridge is a stable plugin API." | The stable surface is `proc_macro`; the bridge implementation is internal. |
| "`-Z build-std` is normal Cargo." | `-Z` flags are nightly/unstable. |

---

## Primary Sources

| Source | Why it matters |
|--------|----------------|
| The Cargo Book: Build Scripts, Build Script Examples, `links`, environment variables | Stable build-script and native-linking contract |
| Cargo reference on cross-compilation | Host/target behavior and target selection |
| `proc_macro` standard library documentation | Stable public API for procedural macros |
| rustc-dev-guide: macro expansion and proc-macro bridge | Internal compiler boundary and caveats |
| `cc`, `pkg-config`, and `vcpkg` crate docs | De facto native discovery/build APIs |
| Siblings: [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md), [05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md), [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md), [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md) | Adjacent graph, macro, artifact, and tool layers |
