---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:target-platform-compatibility-no-std
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Target and Platform Compatibility, Including no_std
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md
canonical_path: rust-crate-ecosystem/11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:11-target-platform-compatibility-no-std]
concepts: [rust targets, platform compatibility, no_std, alloc, target-specific dependencies]
root_concepts: [rust target support]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Target and Platform Compatibility, Including no_std

## The Big Picture

"Supports Rust" says nothing about the target, standard-library layer, allocator,
OS API, ABI, or active feature set. Compatibility is a matrix.

```
+===========================================================================+
|                       PLATFORM SUPPORT MATRIX                             |
+===========================================================================+
|                                                                           |
| target triple + Rust target tier + toolchain component                    |
|       |                                                                   |
|       v                                                                   |
| core only -> core + alloc -> std                                          |
|       |             |          |                                          |
|       v             v          v                                          |
| no OS assumptions  allocator  threads/files/network/process              |
|       |             |          |                                          |
|       +-------------+----------+--> crate features and target deps         |
|                                      |                                    |
|                                      v                                    |
|                          native/build/link environment                     |
+===========================================================================+
```

A Rust target tier describes Rust project support for the toolchain target. It
does not guarantee that a third-party crate, native library, runner, or product
feature supports that target.

## `core`, `alloc`, and `std`

| Layer | Provides | Assumption |
|-------|----------|------------|
| `core` | Fundamental language/library types and traits | No allocator or OS |
| `alloc` | `Vec`, `String`, `Box`, and allocation-backed types | Global allocator supplied by environment |
| `std` | OS-facing facilities, threads, files, networking, process, synchronization | Supported standard-library target/platform |

Minimal library shape:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub fn checksum(bytes: &[u8]) -> u32 {
    bytes.iter().map(|b| u32::from(*b)).sum()
}
```

```toml
[features]
default = ["std"]
std = ["alloc"]
alloc = []
```

This only establishes the crate's own layering. Every dependency must also
support the selected `no_std`/`alloc` profile without re-enabling `std`.
Some `alloc` APIs have additional target requirements: for example,
`alloc::sync::Arc` requires pointer-width atomic operations and is not
available on every `alloc` target.

## Target-Specific Dependencies

```toml
[target.'cfg(unix)'.dependencies]
nix = { version = "0.29", default-features = false }

[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.59", features = ["Win32_Foundation"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
getrandom = { version = "0.2", features = ["js"] }
```

Target tables use Cargo's supported `cfg` expressions. They are evaluated for
dependency selection, but build scripts still execute on the host and may
inspect `HOST`/`TARGET`.

Avoid putting target-specific dependency declarations behind `cfg(feature =
"...")` in target table keys; Cargo feature selection and target dependency
syntax have distinct rules. Use features on the dependency entry and source
`#[cfg]` where appropriate.

## Proving a Support Claim

Install target libraries when available:

```text
rustup target add wasm32-unknown-unknown
rustup target add thumbv7em-none-eabihf
```

Check profiles:

```text
cargo check --target wasm32-unknown-unknown --no-default-features
cargo check --target wasm32-unknown-unknown --no-default-features --features alloc
cargo check --target x86_64-unknown-linux-gnu --all-targets
cargo check --target x86_64-pc-windows-msvc --all-targets
```

`cargo check` proves compilation through metadata/code analysis for the selected
units; it does not run tests or prove linker/runtime behavior. Cross-target
tests need an emulator, device, runner, simulator, or target CI host.

```
check -> link -> execute -> integrate
  |        |        |          |
Rust API  linker   runtime    product behavior
```

## Support Matrix Example

| Profile | Toolchain | Features | Evidence | Claim |
|---------|-----------|----------|----------|-------|
| Linux server | MSRV and stable | `std,tls-rustls` | check/test/release build | Supported |
| Windows service | Stable | `std,tls-native` | native CI test | Supported |
| Browser WASM | Stable | `alloc,js` | check plus browser integration | Supported |
| Embedded Cortex-M | Pinned stable | no defaults | check plus hardware test | Experimental |

Use "experimental" or "best effort" when evidence does not justify "supported."
Name whether the target, crate, or complete product is being described.

## Portability Traps in Dependencies

| Trap | Detection |
|------|-----------|
| Default feature pulls `std` | `cargo tree -e features` under no-default profile |
| Unconditional filesystem/network use | Source/API review and target check |
| Architecture-specific atomics | Compile matrix and target docs |
| Entropy/clock assumptions | Runtime integration on target |
| Native library unavailable | [12](12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md) review |
| Proc macro works on host but emits target-invalid code | Cross-target expansion/check |
| `usize`/endianness/layout assumption | Tests on architecture variants and explicit serialization |

## Old World -> New World Bridge

| Familiar portability concept | Rust equivalent |
|------------------------------|-----------------|
| Target framework/platform moniker | Target triple plus `cfg` |
| Base class library subset | `core` / `alloc` / `std` layers |
| Conditional project reference | Target-specific dependency table |
| Cross compiler SDK/sysroot | Rust target component plus linker/native sysroot |
| Portable class library contract | Feature/target profile tested across consumers |

For .NET readers, `no_std` is not simply "no CLR." Rust has no CLR; it means the
crate declines the OS-oriented `std` facade and may use only `core` and
optionally `alloc`.

## Common Confusion Points

- **"`no_std` means no allocation."** `alloc` is available when the environment
  supplies an allocator.
- **"Rust supports the target, so the crate does."** Toolchain and crate support
  are separate.
- **"`cargo check --target` proves runtime support."** It does not execute.
- **"WASM is one platform."** Browser, WASI, embedded, and custom hosts expose
  different capabilities.
- **"Disabling defaults removes `std`."** Another dependency edge may re-enable
  defaults or require `std` directly.

## Decision Cheat Sheet

| Need | Profile |
|------|---------|
| Desktop/server only | `std` default, explicit OS target matrix |
| Library usable in embedded and server | `no_std` core, optional `alloc`, additive `std` |
| Browser WASM | Dedicated feature/dependency profile plus browser execution test |
| Cross-compiled native product | Target check, linker/sysroot configuration, target runtime test |
| Claim a target as supported | Named toolchain/features plus compile, link, execute evidence |
| Unsupported optional target | Label experimental; do not let docs imply parity |

## Primary Sources

- Rust platform support: https://doc.rust-lang.org/rustc/platform-support.html
- Embedded Rust `no_std`: https://docs.rust-embedded.org/book/intro/no-std.html
- Cargo target dependencies: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies
- Conditional compilation: https://doc.rust-lang.org/reference/conditional-compilation.html
- `alloc` crate: https://doc.rust-lang.org/alloc/

## Related Guides

- Previous: [10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md](10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md)
- Next: [12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md](12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md)
- Feature layering: [04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md](04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md)
