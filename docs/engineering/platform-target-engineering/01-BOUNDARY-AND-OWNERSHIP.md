# Platform Boundary and Ownership

Status: Guidance
Implementation authority: None

## Purpose

Platform engineering fails when a target triple is treated as the owner of the
whole outcome. This guide assigns responsibility across the complete
host-to-deployment chain. Ferris records and coordinates those responsibilities
but does not absorb them.

## Ownership map

| Boundary | Primary authority | Ferris responsibility |
|---|---|---|
| Package sources, resolution, lock state, features, workspace units | Cargo and repository maintainers | Retain selected identities and owner evidence; do not resolve independently |
| Rust language and code generation | rustc and Rust project target owners | Record exact compiler, target specification, tier, flags, outputs, and limits |
| Toolchain installation and selection | rustup or organization toolchain owner | Observe declared/active toolchain; never auto-install from this guidance |
| `core`, `alloc`, `std` APIs and target libraries | Rust library and distribution owners | Record required layer and actual component availability |
| Host build scripts and procedural macros | Cargo plus package owners | Preserve host identity, inputs, outputs, permissions, and target effects |
| Native compilation and discovery | Native tool, SDK, package, and crate owners | Record discovered executable/artifact, not only intended configuration |
| Final linking | rustc driver plus platform linker/toolchain owner | Separate link plan, linker identity, inputs, diagnostics, and produced image |
| Execution and testing | Runner, emulator, simulator, device, browser, OS, or RTOS owner | Record execution substrate and test scope |
| Packaging, signing, deployment, servicing | Product release and platform operations owners | Coordinate approved plan and evidence; never infer release authority |
| Profile approval and support | Consumer organization and named support owner | Enforce only declared policy and expiry |

The source ownership model follows the
[Ferris product boundary](../../../CONTEXT.md) and
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Host and target are separate identities

The host is where Cargo, rustc, build scripts, procedural macros, generators,
and many discovery tools execute. The target is where the produced artifact is
intended to execute. A complete record includes:

```text
host toolchain
  -> host build units and tools
  -> target compiler and target libraries
  -> target native compiler/linker/sysroot/SDK
  -> target artifact
  -> runner or deployment environment
```

For example, a Windows host checking a Linux target has not established a Linux
link or runtime. A macOS host building for iOS simulator has not established an
iOS device build. A Linux host producing a Cortex-M library has not established
firmware startup, memory layout, flashing, or hardware behavior.

Host identity must include the operating system, architecture, toolchain
manager, selected Cargo/rustc pair, environment policy, filesystem and path
semantics, and any host-executed dependency. Target identity must include the
triple, target specification, architecture capabilities, ABI, target tier,
library layer, linker and sysroot, runtime, and deployment class.

## Target tiers are upstream claims, not product support

Rust target tiers describe Rust project build, test, distribution, and support
expectations. They do not prove:

- that the selected crate closure supports the target;
- that required features avoid `std`;
- that an allocator, atomics, entropy, clocks, threads, or sockets exist;
- that a target linker, SDK, sysroot, or runtime is installed;
- that native libraries or providers are available;
- that product packaging, signing, deployment, debugging, or servicing works;
  or
- that Ferris or a consumer supports the combination.

Store the tier and its observation date as one input. Store consumer support as
a separate, approved and expiring profile decision.

## Library and architecture capability ownership

`core`, `alloc`, and `std` are layered capabilities:

| Layer | Required platform contribution |
|---|---|
| `core` | Viable target/compiler support and any required panic/runtime hooks in the final image |
| `alloc` | A valid global allocator, allocation failure policy, and required architecture capabilities |
| `std` | Target-specific hosted services implemented by the standard library and underlying OS/runtime |

Crate prose or `#![no_std]` does not establish closure-wide support. The
effective feature and dependency closure must be checked. Architecture
capabilities such as pointer-width atomics, compare-and-swap, endianness,
alignment, SIMD, unwind, thread-local storage, and interrupt or critical
section semantics remain target/profile facts.

Provider selection is consumer-owned. A fallback atomic implementation,
JavaScript entropy source, TLS backend, allocator, executor, HAL, RTOS adapter,
or system library can change safety, operational, licensing, and support
assumptions. Ferris may explain choices and missing prerequisites but must not
silently choose one.

## Platform family boundaries

### Linux

Distinguish GNU from musl and other environments. Record libc and loader
requirements, minimum distro or image, system package identity, linker,
sysroot, kernel/runtime assumptions, container boundary, debug format, package
format, service manager, and patch owner. "Linux" alone is not an ABI or
servicing contract.

### Windows

Distinguish MSVC and GNU targets, Windows SDK version, MSVC toolset and CRT,
import libraries, linker, subsystem, minimum supported Windows release,
PDB/debugger workflow, Authenticode or package signing, installer format, and
enterprise servicing channel. Visual Studio discovery outside `PATH` must be
recorded from the tool that actually ran.

### macOS

Record host architecture, target architecture, universal-binary policy, Apple
SDK and deployment target, linker, frameworks, entitlements, codesigning,
notarization, dSYM handling, package form, and update mechanism. Apple SDK
absence is `unavailable`, not evidence that the Rust target is unsupported.

### Android and iOS

Mobile profiles must distinguish build host, device ABI, simulator/emulator
from physical device, SDK/NDK or Xcode version, API/deployment level, JNI or
Objective-C/Swift boundary, package format, signing/provisioning, store or
enterprise distribution, device tests, crash symbol handling, and update
policy. Platform vendor policy remains authoritative.

### WASM, WASI, and browser

Do not treat WebAssembly as one runtime. Record the Rust target and host
contract: browser main thread, worker, Node, WASI preview/version, component
host, plugin embedder, or custom runtime. Record JavaScript glue, WIT/component
interfaces, entropy/time/network/filesystem providers, bundler, optimizer,
runtime version, browser matrix, and deployment packaging.

### Embedded, bare metal, and RTOS

Record MCU/SoC, board, target specification, floating-point ABI, atomic
capabilities, memory map, linker script, startup/runtime, HAL, PAC, allocator,
panic behavior, interrupt model, RTOS and version, probe, runner, flashing
tool, image format, bootloader, device test, debugging, field update, and
recovery. A library check is not a bootable firmware result.

## External tool and native provider boundary

Native integration is an execution and artifact chain, not a `links` label:

```text
Rust package
  -> host build script or generator
  -> discovery inputs
  -> native compiler/assembler/archiver
  -> target headers/libraries/SDK/sysroot
  -> Cargo directives
  -> target linker
  -> load/runtime behavior
  -> packaged and serviced component
```

System, bundled-source, prebuilt-object, generated-binding, vendored-tool, and
externally supplied modes move responsibility but do not remove it. Record who
owns installation, patching, license notices, advisories, ABI, deployment, and
rollback in each mode. See the
[native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md).

## Ferris boundaries

Ferris owns:

- profile schemas, identity joins, evidence state, expiry, explanations, and
  cross-owner plans;
- conservative qualification matrices and unsupported/unknown visibility;
- approval, audit, lifecycle, rollback, and removal records; and
- conformance requirements for Ferris claims.

Ferris does not own:

- Cargo resolution or rustc target semantics;
- toolchain, SDK, linker, package-manager, runner, signing, or deployment
  installation;
- platform vendor licenses or credentials;
- automatic feature, provider, manifest, CI, or environment mutation; or
- a universal Rust distribution, portability certificate, or replacement
  platform taxonomy.

## Review questions

1. Is every host-executed and target-executed component assigned to an owner?
2. Are compiler tier, crate compatibility, environment availability, and
   consumer support separate claims?
3. Are ABI, allocator, panic, atomic, threading, entropy, clock, and provider
   assumptions explicit?
4. Are link, package, sign, deploy, debug, and servicing owners named?
5. Can ordinary Cargo and owner-native workflows continue without Ferris?
6. Does any proposed action mutate an environment without separate authority?

## Sources

- [Ferris context](../../../CONTEXT.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Platform compatibility research](../../research/2026-08-09-rust-platform-compatibility.md)
- [Native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [Mirrored toolchain and target guide](../../reference/rust-reference/rust-architecture/02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md)
- [Mirrored core/alloc/std guide](../../reference/rust-reference/rust-architecture/16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md)
- [Mirrored host/target native tools guide](../../reference/rust-reference/rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md)

