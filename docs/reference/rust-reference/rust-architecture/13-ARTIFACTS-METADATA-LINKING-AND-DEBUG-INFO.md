---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:artifacts-metadata-linking
kind: guide
module: rust-architecture
section: rust-architecture
title: Artifacts, Metadata, Linking, and Debug Info
status: source-custody
source_custody: partial
current_path: rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md
canonical_path: rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md
backsource_ids: [proof-backfill:rust-architecture:13-artifacts-metadata-linking]
concepts: [crate types, rlib, rmeta, dylib, staticlib, cdylib, linking, debug info]
root_concepts: [rust artifacts]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Artifacts, Metadata, Linking, and Debug Info

## The Big Picture

A Rust build produces two different kinds of information: native machine-code artifacts for the platform linker, and Rust metadata for downstream Rust compilation. Keeping those separate prevents most artifact confusion. The platform owns object formats, linkers, C runtimes, DWARF, and PDB. rustc owns crate metadata and Rust-specific dependency knowledge. Cargo owns the build graph that decides which rustc invocations run [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md).

```
+===========================================================================+
|                         RUST ARTIFACT PIPELINE                            |
|                                                                           |
|  source crate                                                             |
|      |                                                                    |
|      v                                                                    |
|  rustc front/middle: HIR, types, MIR, mono items                          |
|      |                                                                    |
|      +-------------------+------------------------------------------------+
|                          |                                                |
|                          v                                                |
|              crate metadata (.rmeta)                                      |
|              DefIds, types, MIR-for-generics, exports, deps       [06]    |
|                          |                                                |
|                          v                                                |
|              downstream Rust crates type-check without recompiling code    |
|                                                                           |
|      |                                                                    |
|      v                                                                    |
|  backend objects from LLVM/other backend                          [12]    |
|      |                                                                    |
|      +--> rlib / staticlib archives (archive outputs; linked later)       |
|      |                                                                    |
|      +--> rustc link driver -> platform linker                            |
|              link.exe / lld / ld / compiler driver                       |
|                          |                                                |
|                          v                                                |
|              bin / dylib / cdylib + DWARF/PDB debug information          |
+===========================================================================+
```

The key non-.NET fact: Rust has no stable Rust binary ABI. Rust libraries are
normally distributed as source crates and compiled together by the same
toolchain. The conventional stable interop route is an explicit platform C ABI
surface plus the data-layout, ownership, panic, and versioning discipline below.

---

## Crate Types and Who Consumes Them

| `--crate-type` | Produced artifact | Who consumes it | Stability story |
|----------------|-------------------|-----------------|-----------------|
| `bin` | Executable | OS loader / user | Native platform artifact |
| `lib` | Compiler-selected Rust library, currently usually `rlib` | Rust dependents | Stable spelling, selected format is rustc choice |
| `rlib` | Rust static library: object files plus `.rmeta` | rustc compiling downstream crates | Existence stable; internal layout unstable |
| `dylib` | Rust dynamic library using Rust ABI | Rust crates built with matching compiler | Rust ABI unstable/version-locked |
| `staticlib` | Native static archive containing Rust code and dependencies | C/C++/other native linkers in a later final link | Platform archive container; exported ABI depends on the source declarations |
| `cdylib` | Native shared library intended for non-Rust hosts | C ABI hosts, plugins, wasm embedders | Platform shared-library container; only explicit FFI exports have a C-compatible ABI |
| `proc-macro` | Special host dynamic library | rustc loads it while compiling another crate | Host toolchain artifact; see [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md) |

```
Rust-to-Rust dependency path:
    crate A --rlib/rmeta--> rustc compiling crate B

Native embedding path:
    Rust crate --staticlib/cdylib + extern "C"--> C ABI host

Executable path:
    Rust crate --bin--> platform loader
```

`rlib` is the ordinary dependency format because it carries both compiled code
and Rust metadata. `staticlib` and `cdylib` deliberately cross out of the
Rust-to-Rust dependency world into native artifact formats; only explicitly
declared exports choose a C or other platform ABI.

The crate type chooses a container, not an FFI contract. A durable native
boundary also needs:

| Concern | Required discipline |
|---------|---------------------|
| Exported function ABI | `pub extern "C" fn` and an exported symbol such as `#[unsafe(no_mangle)]` in edition 2024 |
| Data layout | FFI-safe scalars/pointers and `#[repr(C)]` aggregates; no Rust-owned `String`, `Vec`, trait object, or unwritten enum layout |
| Ownership | Document who allocates, frees, and retains each pointer; pair constructors with matching Rust-side destructors |
| Panic/unwind | Do not let ordinary Rust unwinding cross a non-unwind FFI boundary; catch, abort, or deliberately use a supported unwind ABI |
| Versioning | Version the exported API and honor the platform ABI independently of the Rust compiler version |

---

## rmeta: Metadata Is the Rust Dependency Interface

| Metadata carries | Why downstream rustc needs it |
|------------------|-------------------------------|
| DefIds and DefPathHashes | Stable identity across sessions; see [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md) |
| Types, generics, trait impls | Type-check uses of dependency APIs |
| MIR for generic and inlineable functions | Monomorphization and inlining in downstream crates [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) |
| Exported symbols and linkage data | Later artifact and link decisions |
| Dependency and feature information | Coherence, crate disambiguation, and rebuild logic |

```
Dependency crate:
  rustc --emit=metadata --------------> libdep.rmeta
              |                              |
              |                              v
              |                  dependent crate can type-check
              |                  before dependency codegen finishes
              v
  backend codegen still running
```

This is the mechanism behind Cargo pipelining: once a dependency's metadata is ready, Cargo can start compiling dependents before the dependency's backend work has finished. That improves wall-clock parallelism without changing the semantic dependency order.

The caveat is severe: `.rmeta` is not the .NET assembly metadata story. It is serialized rustc implementation state, version-locked to the exact rustc that produced it. A prebuilt `rlib` from one toolchain is generally not usable by another. That is why crates.io distributes source, not long-lived Rust-ABI binaries.

---

## Rust ABI, C ABI, and Distribution Reality

| Boundary | Use | Contract |
|----------|-----|----------|
| Rust source crate | Normal library distribution | Semver plus Cargo resolver; compiled by consumer |
| `rlib` between crates | Build artifact inside one toolchain | rustc-version-locked |
| `dylib` Rust ABI | Rust dynamic link within matching compiler world | Unstable Rust ABI |
| `extern "C"` | FFI declarations | C ABI owned by platform/toolchain |
| `staticlib` / `cdylib` | Native library distribution | Platform object/shared-library conventions |

```
Do not publish this as a stable Rust ABI contract:
    libfoo.rlib  or  libfoo.dylib   -> arbitrary future Rust toolchain

Do publish this when you need a binary boundary:
    extern "C" API + cdylib/staticlib -> C/Python/.NET/PInvoke/host program
```

Generic instantiation, trait layout, vtables, symbol mangling, and metadata all participate in the Rust compilation model. Without a stable Rust ABI, the safe distribution unit is source. The explicit FFI unit is C ABI plus headers/bindings/documented calling convention.

---

## Linking: rustc Delegates to the Platform Toolchain

```
+-------------------+      +-------------------+      +-------------------+
| backend object    | ---> | rustc link driver | ---> | platform linker   |
| files + archives  |      | selects args      |      | link.exe/lld/ld   |
+-------------------+      +-------------------+      +-------------------+
          |                         |                          |
          |                         | -L, -l, #[link]          | CRT, libc, SDK libs
          |                         | -C linker, link-arg      | relocations, image
          v                         v                          v
                    final executable / dylib / cdylib
```

`staticlib` stops before the final link: rustc invokes an archiver to package
the crate's objects and Rust dependency code. The consuming native build later
links that archive into an image and must also supply any required system/native
libraries (inspectable with `--print native-static-libs`). The external linker
step above applies to `bin`, `dylib`, and `cdylib` outputs.

The linker is external. On Windows MSVC targets that usually means `link.exe` or `lld-link` and PDB-aware toolchains. On Unix-like targets it is commonly `cc`, `ld`, or `lld`. The C runtime, SDK libraries, system import libraries, and native archive formats come from the OS toolchain, not from rustc.

| Mechanism | Layer | Use |
|-----------|-------|-----|
| `-L native=...` | rustc/link | Add native library search path |
| `-l static=foo`, `-l dylib=foo` | rustc/link | Request native library |
| `#[link(name = "foo")]` | Rust source | Attach native link requirement to extern block/module |
| Cargo `links` key | Cargo/build script | Declare one crate owns a native library link name [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md) |
| `-C linker=...` | rustc codegen/link option | Select linker executable |
| `-C link-arg=...` | rustc codegen/link option | Pass one raw argument to linker |

---

## Debug Info: DWARF, PDB, and Source Spans

| Target family | Debug format | Rust knob | Reader/tooling |
|---------------|--------------|-----------|----------------|
| ELF Linux | DWARF | `-C debuginfo=1/2`, `-g` | gdb, lldb, perf tooling |
| Mach-O macOS | DWARF/dSYM | `-C debuginfo`, `-C split-debuginfo` | lldb, dsymutil ecosystem |
| MSVC Windows | PDB | `-C debuginfo`, `-C split-debuginfo` where supported | Visual Studio, WinDbg, debuggers |
| Any target | Stripped symbols/debug | `-C strip=symbols` or `-C strip=debuginfo` | Deployment-size control |

```
Rust source span [04]
        |
        v
HIR/MIR/codegen location info
        |
        v
object debug sections or PDB records
        |
        v
native debugger maps instruction pointer -> Rust source
```

`rust-gdb` and `rust-lldb` are wrappers that load Rust pretty-printers and make Rust names/types less hostile in native debuggers. On MSVC Windows, PDB is not an alien layer; it is the same debug-information universe a .NET/VC++ veteran already knows, except the code is native Rust rather than IL or managed code.

---

## Commands and Traces

| Goal | Command |
|------|---------|
| Emit a cdylib container and metadata | `rustc --crate-type=cdylib --emit=link,metadata src/lib.rs` |
| See Cargo's rustc invocations and link-affecting flags | `cargo build -v` |
| Print rustc's actual platform-linker argument vector | `rustc --print link-args src/main.rs` |
| Inspect an rlib archive | `ar t libfoo.rlib` or `llvm-ar t libfoo.rlib` |
| Request full debug info | `rustc -C debuginfo=2 src/main.rs` or `cargo build` in dev profile |
| Override linker | `RUSTFLAGS="-C linker=lld" cargo build` |

```
# Produce a native shared-library container and Rust metadata.
# The source still needs explicit extern "C" exports and an FFI-safe API.
rustc --crate-name foo --crate-type=cdylib --emit=link,metadata -C debuginfo=2 src/lib.rs

# Show each rustc invocation and the flags that shape linking.
cargo build -v

# Print the external linker executable and arguments for one direct rustc build.
rustc --print link-args src/main.rs

# rlib is an archive; names vary by target and hash.
ar t target/debug/deps/libfoo-*.rlib
# Typical contents include object files plus lib.rmeta.

# Metadata-only compilation, the basis for Cargo pipelining.
rustc --crate-type=lib --emit=metadata src/lib.rs

# Pass one native linker argument. Prefer Cargo config/profile policy in real builds.
RUSTFLAGS="-C link-arg=/DEBUG:FULL" cargo build
```

The exact command syntax for linker arguments is platform-specific. `/DEBUG:FULL` is MSVC-linker-shaped; ELF targets will use different flags.

---

## Old world -> New World Bridge

| Old world | Rust mapping | Difference that matters |
|-----------|--------------|-------------------------|
| `.lib` / `.a` static library | `rlib` superficially resembles an archive | `rlib` also carries rustc metadata and is not stable across toolchains |
| .NET assembly metadata | `.rmeta` is the Rust compile-time metadata analog | .NET metadata is a stable runtime/loader contract; rmeta is rustc-private |
| P/Invoke to native DLL | `cdylib`/`staticlib` plus `extern "C"` | This is the stable binary boundary Rust intentionally exposes |
| PDB debugging | Rust MSVC target PDBs | Same debug-info family, native code instead of managed IL |
| MSBuild item/link settings | Cargo + rustc link flags/build scripts | Cargo plans; rustc invokes the platform linker |

If you expect NuGet-style prebuilt binary Rust libraries, you will fight the model. Cargo packages source; rustc compiles the whole Rust dependency world with one coherent toolchain; binary boundaries are made explicit through C ABI artifacts.

---

## Stability and Ownership Boundaries

| Thing | Treat as |
|-------|----------|
| Existence of `rlib`, `dylib`, `staticlib`, `cdylib`, `bin`, `proc-macro` crate types | Stable rustc surface |
| `--emit` output kinds | Stable rustc surface |
| `.rmeta` serialized format | rustc internal, version-locked |
| `rlib` archive member layout | rustc internal |
| Rust `dylib` ABI | Unstable Rust ABI, compiler-version-sensitive |
| `extern "C"` ABI | Platform/toolchain ABI contract |
| Object files, archives, shared libraries | Platform/linker-owned formats |
| DWARF/PDB | Platform/debugger ecosystem formats, emitted by rustc/backend |

The stable promise is not "this binary artifact will link forever." The promise is that documented stable Rust source and documented stable flags keep working subject to the target platform's ABI rules.

---

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Build an ordinary Rust dependency | `rlib` through Cargo's default flow |
| Start dependents before dependency codegen finishes | Cargo pipelining via metadata emission |
| Ship a plugin or native shared library | `cdylib` plus `extern "C"` API |
| Embed Rust into a C/C++ program statically | `staticlib` plus C ABI declarations |
| Build a Rust dynamic library for Rust consumers only | `dylib`, but only inside one coherent toolchain world |
| Debug on Windows/MSVC | `-C debuginfo=2` and PDB-aware debugger |
| Debug on Linux/macOS | DWARF through gdb/lldb, possibly `rust-gdb`/`rust-lldb` |
| Diagnose native link failure | `cargo build -v`, inspect `-L`, `-l`, `#[link]`, build scripts |
| Understand backend object emission first | Read [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md) |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "An rlib is just a C static library." | It is an archive-like file plus Rust metadata, for rustc consumption. |
| "rmeta is like stable .NET metadata." | It is the analog, not the contract. It is rustc-private and version-locked. |
| "Rust has a stable ABI because it makes dylibs." | Rust `dylib` exists, but the Rust ABI is unstable. Use C ABI for stable binary boundaries. |
| "Cargo links everything itself." | Cargo schedules; rustc drives final links for `bin`/`dylib`/`cdylib`, while `staticlib` is archived for a later consumer link. |
| "The C runtime comes from Rust." | It comes from the target platform/toolchain. |
| "PDB means managed code." | PDB is a debug-info container used by native MSVC Rust targets too. |
| "If `--emit=llvm-ir` works, LLVM IR is the artifact contract." | It is an output kind, but the IR content is LLVM/rustc-version-sensitive. |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| rustc-dev-guide: Crate metadata / rmeta | What metadata exists and how downstream rustc uses it |
| rustc-dev-guide: Linking | rustc's link orchestration and native library handling |
| rustc-dev-guide: Debugging support / debuginfo | DWARF/PDB generation and debugging model |
| The rustc book: `--crate-type`, `--emit` | Stable command-line artifact controls |
| The rustc book: codegen/link options, debuginfo | `-C linker`, `-C link-arg`, `-C debuginfo`, `-C strip` |
| The Rust Reference: linkage, `extern`, ABI | Language-level FFI and ABI spelling |
| The Cargo Book: build scripts and `links` | Native library discovery and Cargo link coordination |

*Cross-links:* place this guide after backend codegen in [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md), identity and DefPathHash in [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md), monomorphization in [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md), Cargo scheduling in [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md), and platform/std layering in [16](16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md).