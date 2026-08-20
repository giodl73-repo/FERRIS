---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:cpp-interop
kind: guide
module: rust-interop-migration
section: computing-software
title: C++ Interop
status: source-custody
source_custody: partial
current_path: rust-interop-migration/04-CPP-INTEROP.md
canonical_path: rust-interop-migration/04-CPP-INTEROP.md
backsource_ids: [proof-backfill:rust-interop-migration:04-cpp-interop]
concepts: [C++ interop, cxx, autocxx, bindgen, C shim, exception translation, unique_ptr, C++ ABI]
root_concepts: [C++ interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# C++ Interop

C++ is not one portable ABI. Name mangling, exceptions, standard-library types,
class layout, compiler flags, and CRT selection create a toolchain-specific
binary world. Start with a C-shaped boundary when durability matters; use a
C++-aware bridge when one controlled toolchain makes richer types worth the
coupling.

## The Big Picture

```
+============================================================================+
|                         RUST <-> C++ OPTIONS                               |
+============================================================================+
|  MOST DURABLE                                                              |
|                                                                            |
|  C++ facade -> versioned C ABI -> Rust core                                |
|      simple contract, broad compiler/language reach                        |
|                                                                            |
|  cxx bridge <-> generated C++/Rust glue                                    |
|      checked shared subset, unique_ptr/shared types, controlled builds     |
|                                                                            |
|  autocxx / bindgen + handwritten safety facade                             |
|      adapt a larger existing header surface                                |
|                                                                            |
|  direct C++ ABI guesses / class layout / Rust trait-object vtable          |
|      DO NOT use as a durable boundary                                      |
|                                                                            |
|  MOST TOOLCHAIN-COUPLED                                                    |
+============================================================================+
```

## Choose the Bridge Shape

| Situation | Preferred shape |
|-----------|-----------------|
| Public SDK or multiple compiler families | C facade with opaque handles |
| One product build controls Rust and C++ toolchains | `cxx` bridge |
| Large C++ API must be consumed with selective automation | `autocxx`, then narrow wrappers |
| Plain C subset in headers | `bindgen` plus safe Rust facade |
| Template-heavy/header-only library | C++ wrapper translation unit that instantiates a stable narrow API |
| Cross-process opportunity exists | Protocol boundary instead of in-process C++ ABI |

## A `cxx` Bridge

`cxx` defines a shared, checked bridge language and generates glue for both
sides. It supports a deliberate subset rather than pretending arbitrary C++ and
Rust types are layout-compatible.

```rust
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("legacy/include/parser.h");

        type Parser;
        fn new_parser() -> UniquePtr<Parser>;
        fn parse(self: Pin<&mut Parser>, input: &CxxString) -> Result<u64>;
    }

    extern "Rust" {
        fn normalize(input: &str) -> String;
    }
}
```

```cpp
#pragma once
#include <cstdint>
#include <memory>
#include <string>

class Parser {
public:
  std::uint64_t parse(const std::string& input);
};
std::unique_ptr<Parser> new_parser();
```

The bridge encodes ownership (`UniquePtr`), mutability/pinning, and supported
string conversions. It still requires a compatible C++ compiler, standard
library, exception policy, and package layout.

## C Facade as the Stability Firewall

```
  arbitrary C++ callers
          |
          v
  +-----------------------------------------------------------+
  | C++ RAII facade: std::string, exceptions, classes         |
  +-----------------------------------------------------------+
          |
          v
  +-----------------------------------------------------------+
  | versioned C ABI: opaque handles, ptr+len, status codes    |
  +-----------------------------------------------------------+
          |
          v
  +-----------------------------------------------------------+
  | Rust adapter + core: idiomatic Rust stays private         |
  +-----------------------------------------------------------+
```

This pattern makes the C++ layer replaceable and keeps compiler-specific ABI out
of the durable contract. The RAII facade can translate C status codes to C++
exceptions or `std::expected`, but exceptions terminate at the facade.

## Exceptions, Panics, and Destructors

Ordinary `extern "C"` frames are not a corridor for C++ exceptions or Rust
panics. Catch C++ exceptions before returning to Rust; catch/translate Rust
panics before returning to C++. `cxx` maps supported C++ exceptions to Rust
errors for declared fallible calls, but that is bridge behavior, not permission
for arbitrary cross-language unwinding.

Destruction must follow ownership:

- `std::unique_ptr<T>` is destroyed by C++ glue with the correct complete type.
- Rust-owned opaque handles are released by Rust exports.
- C++ objects borrowed by Rust must outlive every borrow and callback.
- A destructor that can throw is already hazardous in C++; never allow it to
  unwind through Rust.

## Build and Link Topology

| Layer | Typical tool |
|-------|--------------|
| Compile generated/handwritten C++ glue | Cargo `cc` crate or outer CMake/MSBuild |
| Generate bridge | `cxx-build`, `autocxx-build`, or `bindgen` |
| Outer native graph | CMake, Meson, Bazel, MSBuild, or Cargo |
| Artifact | Rust `staticlib`/`cdylib`, C++ static/shared library |
| ABI verification | Export scan, header compile tests, per-target integration executable |

Choose one build system as the authority for target/compiler flags. Two build
systems may cooperate, but they must agree on architecture, runtime library,
debug/release mode, C++ standard library, exception/RTTI flags, and link order.

## Boundary Hazard Register

| Hazard | C++ boundary rule |
|--------|-------------------|
| ABI | Treat C++ ABI as compiler/platform-specific; use C ABI or generated checked glue; never export Rust ABI or trait objects. |
| Allocator | Destroy C++ objects in C++; destroy Rust objects in Rust; avoid passing owning `std::string`/`vector` or `Vec` across unmatched runtimes. |
| Panic/unwind | Catch C++ exceptions before Rust and Rust panics before C++; use explicit error mapping. |
| Lifetime | Encode unique/shared/borrowed semantics in wrapper types; pin when address stability is required. |
| Threading | Document object thread safety, callback reentrancy, and destruction thread; C++ types do not become `Send`/`Sync` automatically. |
| Target | Pin compiler family/version, standard library, architecture, CRT, exception model, and flags. |
| Packaging | Ship bridge library, dependent runtime libraries, headers, debug symbols, and loader metadata as one tested closure. |

## Old World -> New World Bridge

| C++ concept | Rust interop reading |
|-------------|----------------------|
| PIMPL | Opaque handle keeps implementation layout private |
| RAII | `Drop` and `UniquePtr` encode deterministic destruction on the owning side |
| `std::span` | Pointer-plus-length borrow with explicit call lifetime |
| `std::expected` / error code | Rust `Result`, translated at the bridge |
| Template instantiation firewall | C++ wrapper TU exposes concrete operations to Rust |
| COM interface | Versioned function surface, but do not substitute a Rust trait-object vtable |

## Common Confusion Points

- **"`extern \"C++\"` means portable C++ ABI."** It describes bridge intent;
  the compiled result remains tied to a target and C++ toolchain.
- **"`bindgen` understands all C++ semantics."** Generated declarations cannot
  prove ownership, exception, template, or virtual-dispatch contracts.
- **"The same compiler name means ABI-compatible."** Standard library, CRT,
  flags, build mode, and compiler version also matter.
- **"Move semantics line up automatically."** Rust moves and C++ move
  constructors have different language contracts; use bridge-owned wrappers.
- **"Zero-copy string exchange is free."** It imposes encoding, lifetime,
  mutability, and allocator constraints.
- **"Trait objects are Rust's COM."** The analogy is conceptual only. Rust
  trait-object ABI/vtable layout is not a durable foreign interface.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Long-lived multi-toolchain SDK | C ABI plus C++ RAII facade |
| Controlled monorepo with rich Rust/C++ calls | `cxx` |
| Consume a broad existing C++ API | `autocxx` selectively, with handwritten boundary wrappers |
| Consume plain C declarations in C++ headers | `bindgen` |
| Call template-heavy library | Instantiate concrete C++ wrapper functions |
| Pass ownership | `UniquePtr`/opaque handle with owner-side destructor |
| Translate failure | `Result`/status/`std::expected`; no cross-boundary unwind |

## Primary Sources

- `cxx` guide: https://cxx.rs/
- `autocxx` documentation: https://google.github.io/autocxx/
- bindgen user guide: https://rust-lang.github.io/rust-bindgen/
- Rustonomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- Itanium C++ ABI: https://itanium-cxx-abi.github.io/cxx-abi/

## Related Guides

- Previous: [03-C-INTEROP.md](03-C-INTEROP.md)
- Next: [05-DOTNET-CSHARP-INTEROP.md](05-DOTNET-CSHARP-INTEROP.md)
- Packaging matrix: [14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md](14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md)
