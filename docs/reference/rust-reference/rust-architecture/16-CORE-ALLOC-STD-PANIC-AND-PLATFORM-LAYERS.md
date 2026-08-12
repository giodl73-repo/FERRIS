---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:core-alloc-std-platform
kind: guide
module: rust-architecture
section: rust-architecture
title: The Standard Library - core, alloc, std, panic, and Platform Layers
status: source-custody
source_custody: partial
current_path: rust-architecture/16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md
canonical_path: rust-architecture/16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md
backsource_ids: [mdloom-backfill:rust-architecture:16-core-alloc-std-platform]
concepts: [core, alloc, std, no_std, lang items, panic runtime, global allocator, platform abstraction]
root_concepts: [standard library]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# The Standard Library - core, alloc, std, panic, and Platform Layers

## The Big Picture

Rust's standard library is a separate authority from `rustc`, even though the
two ship together and co-evolve on the same release train. The libs teams, with
libs-api owning the stable public API surface, define what `core`, `alloc`, and
`std` expose. For published targets, rustup installs the available precompiled
libraries through the `rust-std` component described in
[02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md). Some Tier 2 targets ship
only `core`; Tier 3 and custom targets may require an unstable `-Z build-std`
or a separately built sysroot. The layer model below describes API dependencies,
not a promise that every target has a prebuilt artifact.

```
+===========================================================================+
|                         RUST LIBRARY LAYERS                               |
|                                                                           |
|  #![no_std]                                                               |
|  core: primitives, traits, Option/Result, slices, iterators               |
|        no heap or OS; base of each viable Rust sysroot                    |
|                 | requires a global allocator                             |
|                 v                                                         |
|  alloc: Box, Vec, String, Rc; Arc requires pointer-width atomics          |
|        heap allocation, but no hosted OS services                         |
|                 | requires hosted platform services                       |
|                 v                                                         |
|  std: re-exports core + alloc; files, threads, net, time, process         |
|       facade over platform shims and OS-backed services                   |
+===========================================================================+
```

The stable contract is the public library API marked stable. The implementation glue between rustc and the library is much more volatile.

---

## Authority Map

| Authority | Owns | Boundary |
|---|---|---|
| Rust Reference / language teams | `no_std`, panic semantics, language attributes | Not the private layout of libstd |
| rustc | Compiler hooks, codegen requests, lang-item consumption | Not the stable public std API |
| Cargo | Profile settings such as `panic = "abort"` | Not allocator implementation |
| rustup | `rust-std` components per toolchain/target | Not compilation semantics |
| libs/libs-api teams | Stable `core`, `alloc`, and `std` APIs | Not LLVM lowering |
| LLVM/backends | Machine-code lowering for calls and unwind tables | Not public library design |
| Ecosystem tools | Embedded templates, allocator crates, docs, Miri checks | Not compiler/library private contracts |

---

## `core`: The Freestanding Base

`core` is the Rust library you get when there is no heap and no operating system. It is not a miniature `std`; it is the semantic substrate for Rust programs.

| `core` provides | Why it can exist without an OS |
|---|---|
| Primitive traits such as `Copy`, `Drop`, `Sized`, `Deref`, operators | Compiler-known semantics plus pure code |
| `Option`, `Result`, tuples, slices, iterators | No allocation required |
| `fmt` traits and machinery | Formatting abstraction, not necessarily I/O |
| Atomics and pointer utilities where target-supported | Lower to target instructions or compiler intrinsics |
| Panic interfaces used by `no_std` | Hook point, not the hosted panic runtime |

Every target that Rust supports has `core`, including bare-metal targets. A `#![no_std]` crate opts out of automatic `std` linkage and uses `core` directly.

---

## `alloc`: Heap Without Hosted OS

`alloc` is the middle layer: it assumes dynamic allocation, but not files, sockets, processes, environment variables, or threads supplied by an OS.

```
+--------------------+       +------------------------+
| no_std crate       | ----> | extern crate alloc     |
+--------------------+       +------------------------+
          |                             |
          | requires                    v
          |                   Box / Vec / String / Rc
          v
+--------------------+
| #[global_allocator]|
| GlobalAlloc impl   |
+--------------------+
```

| Type family | Lives in | Requirement |
|---|---|---|
| `Box<T>` | `alloc` | Heap allocator |
| `Vec<T>` / `String` | `alloc` | Growable heap allocation |
| `Rc<T>` | `alloc` | Heap control block |
| `Arc<T>` | `alloc` | Heap plus `target_has_atomic = "ptr"` support |
| `BTreeMap`, `VecDeque` | `alloc` | Heap-backed collections |

In embedded or kernel environments, `alloc` is common once you have a heap. You
still provide the allocator and panic story yourself. `Arc` is additionally
conditional: `alloc::sync` is unavailable when the target lacks pointer-width
atomic loads and stores, detectable with
`#[cfg(target_has_atomic = "ptr")]`.

---

## `std`: Hosted Facade and Platform Services

`std` is a facade crate. It re-exports most of `core` and `alloc` so normal users see one namespace, then adds hosted services.

| `std` area | Underlying dependency |
|---|---|
| `std::fs` | OS filesystem APIs |
| `std::thread` | OS or platform thread primitives |
| `std::net` | Sockets and platform networking |
| `std::process` | Process creation and handles |
| `std::time` | Platform clocks |
| `std::env` | Process environment and arguments |

Internally, `std` hides target differences behind `std::sys` implementations for Unix, Windows, WASI, and other platforms. That layer is an implementation detail. The stable promise is the public `std` API, not the layout or names of the shims that implement it.

---

## Lang Items and Compiler Intrinsics: The Coupling Boundary

The compiler and library cannot be completely independent. rustc must recognize a small set of special library items.

| Mechanism | Example territory | Stability |
|---|---|---|
| Lang items (`#[lang = "..."]`) | `Sized`, `Copy`, `Drop`, `Deref`, operator traits, panic and box machinery | Unstable internal contract |
| Compiler intrinsics (`#[rustc_intrinsic]`) | Low-level operations exposed through `core::intrinsics` wrappers | Mostly unstable; user-facing wrappers may be stable |
| `#[rustc_*]` attributes | Compiler-only annotations in libstd/libcore | Unstable implementation detail |
| Public APIs | `Option`, `Vec`, `std::fs::File`, `Iterator` | Stable when marked `#[stable]` |

This is the CLR-special-types analogy: the runtime knows about `System.Object`, strings, arrays, and delegates. Rust has no managed runtime, but rustc has special knowledge of certain library hooks. That is why fully replacing `core` is not a stable user-level operation.

---

## Panic Strategy and Panic Handlers

`panic!` is not a general exception mechanism. It is Rust's failure path for unrecoverable conditions, with two stable strategy knobs.

```
panic!(...) in user code
        |
        v
+-------------------------+
| panic handler/runtime   |
+-------------------------+
     |                 |
     v                 v
panic = "unwind"     panic = "abort"
run destructors       terminate process/image
needs unwind tables   smaller, no unwinding path
```

| Layer | Stable? | Notes |
|---|---|---|
| `[profile.*] panic = "abort"` or `"unwind"` | Yes | Cargo profile setting |
| `#[panic_handler]` in `no_std` | Yes | Required when `std` does not provide one |
| `panic_unwind` / `panic_abort` crates | Internal distribution pieces | Not a general application API |
| Panic lang items and runtime wiring | Unstable | rustc/libstd implementation detail |

Unwinding resembles C++ exception unwinding in the narrow sense that destructors run while stack frames are unwound. It is not meant to replace `Result` for expected errors. See [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) for MIR cleanup paths and [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) for linking/unwind artifacts.

---

## Allocators and Global Allocation

Rust's allocation policy is explicit at the library boundary. Hosted `std` programs get a default global allocator, typically a wrapper over the system allocator. Programs can replace it.

| API | Role | Stability |
|---|---|---|
| `#[global_allocator]` | Selects the process/global allocator | Stable |
| `GlobalAlloc` | Trait implemented by custom allocators | Stable, unsafe contract |
| `alloc::alloc` | Low-level allocation functions | Stable API with unsafe obligations |
| jemalloc/mimalloc crates | Ecosystem allocator choices | Crate-level support, not language authority |

A hosted allocator swap is deliberately small:

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

A `no_std` + `alloc` program must provide a global allocator. Without one, `Vec`, `Box`, and `String` have nowhere to obtain memory.

---

## Concrete Build Skeletons

A portable `no_std` **library** can be complete without choosing a device
runtime, entry symbol, memory map, panic handler, or allocator. This example
builds for the published bare-metal target using only `core`:

| File-level choice | Consequence |
|---|---|
| `#![no_std]` in a library | Do not link the hosted standard library |
| No `#![no_main]` | The crate is an rlib, not a final platform image |
| No panic handler | The final binary supplies one |
| No global allocator | `core` works; `alloc` remains unavailable to the final image until it supplies one |

```rust
// src/lib.rs
#![no_std]

pub fn checksum(bytes: &[u8]) -> u32 {
    bytes.iter().map(|&b| u32::from(b)).sum()
}
```

```toml
# Cargo.toml
[profile.release]
panic = "abort"
```

```text
$ rustup target add thumbv7em-none-eabihf
$ cargo build --lib --target thumbv7em-none-eabihf --release
```

That command produces a library, not a bootable image. A final bare-metal
binary must additionally choose a runtime/entry convention, linker script and
memory map, panic handler, and—if it uses `alloc`—a real allocator and
allocation-failure policy. Those choices are board/OS contracts, not portable
`core` architecture.

---

## No Managed Runtime, Minimal Startup

Rust has startup code and library initialization, but not a CLR/JVM-style managed runtime.

| Concept | Rust reality |
|---|---|
| Garbage collector | None in the language or std |
| JIT | None in normal Rust execution; rustc emits native code ahead of time |
| BCL-style runtime services | `std` is statically linked library code plus OS calls |
| Pre-main runtime | Minimal setup for args, environment, stack guard/panic integration |
| Reflection/metadata runtime | No CLR-equivalent runtime metadata system |

This is why Rust binaries can be small, static, and freestanding, but also why services such as allocation, panic handling, and platform I/O must be explicit library contracts rather than ambient runtime magic.

---

## Old World -> New World Bridge

| Old world | Rust analogue | Difference that matters |
|---|---|---|
| C freestanding vs hosted | `#![no_std]` vs `std` | Rust keeps strong language semantics in both modes |
| Layered BCL | `core` -> `alloc` -> `std` | No managed runtime underneath |
| Overriding `malloc` / custom heap | `#[global_allocator]` + `GlobalAlloc` | Unsafe trait contract; allocation is library-level |
| C++ exceptions unwind stack | `panic = "unwind"` | Panics are not ordinary recoverable errors |
| CLR special-known types | lang items | Compiler/library coupling, not user-stable API |
| NuGet-installed reference assemblies | rustup `rust-std` target component | Precompiled std crates per target |

For a .NET mind, the trap is calling `std` "the runtime." It is closer to BCL surface area statically linked into a native binary, with a few compiler-known hooks and OS shims.

---

## Stability Boundary

| Stable surface | Unstable implementation detail |
|---|---|
| Public `std`, `core`, `alloc` APIs marked stable | `std::sys` platform abstraction layout |
| `#![no_std]` and `#![no_main]` | lang item names and wiring |
| `#[panic_handler]` | panic runtime internals and panic lang items |
| `#[global_allocator]` and `GlobalAlloc` | `#[rustc_*]` attributes and compiler-only hooks |
| Cargo profile `panic = "abort"/"unwind"` | compiler intrinsics as direct user APIs |

The distinction is operational. You can design a product around `no_std`, `panic = "abort"`, and a custom global allocator. You should not design one around `std::sys` internals or private lang-item replacement.

---

## Decision Cheat Sheet

| I want to... | Use |
|---|---|
| Write ordinary hosted application code | `std` |
| Target bare metal or a kernel-like environment | `#![no_std]` with `core` |
| Use `Vec` or `String` without `std` | `extern crate alloc` plus a global allocator |
| Minimize binary size and remove unwinding | `[profile.release] panic = "abort"` |
| Define panic behavior in `no_std` | `#[panic_handler]` |
| Replace the default allocator | `#[global_allocator]` and a `GlobalAlloc` implementation or allocator crate |
| Understand platform-specific `std` behavior | Read public `std` docs first; inspect `std::sys` only as implementation context |
| Cross-compile with library support | `rustup target add <target>` and verify target tier/std availability |

---

## Common Confusion Points

| Confusion | Correction |
|---|---|
| "rustc owns the standard library." | No. libs/libs-api own the public library API; rustc and std co-evolve. |
| "`std` is Rust's runtime." | No. It is a statically linked library facade over `core`, `alloc`, and OS APIs. |
| "`no_std` means no Rust standard semantics." | No. You still have `core`, traits, `Option`, `Result`, slices, and iterators. |
| "`alloc` requires an operating system." | No. It requires a heap allocator, not necessarily hosted OS services. |
| "Lang items are a stable extension point." | No. They are compiler/library implementation details. |
| "Panic unwind is just exceptions." | Only mechanically similar in stack unwinding; Rust uses `Result` for expected errors. |
| "All targets have `std`." | No. Some targets support only `core` or `core` + `alloc`; target tier matters. |

---

## Primary Sources

| Source | Why it matters |
|---|---|
| Standard library docs: `std`, `core`, `alloc` | Public stable API surface and module boundaries |
| Rust Reference: `no_std`, panic behavior, attributes | Language-level contract |
| Unstable Book: lang items and intrinsics | Explicit warning that these are not stable user APIs |
| rustc-dev-guide: compiler/library boundary and panic implementation | Internal coupling model |
| The Embedonomicon | Practical `no_std`, panic handler, and embedded startup patterns |
| Rustonomicon | Unsafe allocator and low-level library obligations |
| libs team RFCs and API guidelines | How public library authority evolves |

*Cross-links:* start with [00](00-OVERVIEW.md), then [02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md) for installed target libraries, [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) for cleanup/unwind paths, [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) for generic code becoming library calls, [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md) for backend lowering, and [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) for link artifacts. For language-facing syntax and semantics, `../rust-language/` is the right sibling module where present.