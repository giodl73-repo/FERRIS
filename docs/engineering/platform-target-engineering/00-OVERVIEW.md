# Platform and Target Engineering Overview

Status: Guidance
Implementation authority: None

## Purpose

This guide set defines how Ferris engineers describe, plan, qualify, adopt,
service, and remove Rust platform support. It is Ferris-owned engineering
guidance. It does not copy authority from the mirrored Rust references, replace
Cargo or rustc, approve a platform, or authorize installation or mutation of a
developer, CI, signing, device, or deployment environment.

The governing rules are:

> The plan is global; the work is local.

> Compatibility is renewable evidence for an exact consumer profile, not a
> universal portability label.

Ferris coordinates owner truth across Cargo, rustc, rustup, linkers, SDKs,
sysroots, runners, native providers, packaging systems, signing services,
debuggers, deployment systems, and product operators. Each owner remains
authoritative for its own work and state.

## Guide map

| Guide | Question answered |
|---|---|
| [01 - Boundary and ownership](01-BOUNDARY-AND-OWNERSHIP.md) | Who owns each platform decision and artifact? |
| [02 - Operating workflow](02-OPERATING-WORKFLOW.md) | How does a platform profile move from intent to qualified evidence? |
| [03 - Evidence and identity](03-EVIDENCE-AND-IDENTITY.md) | Which identities and states must be retained? |
| [04 - Failure modes and controls](04-FAILURE-MODES-AND-CONTROLS.md) | How are negative, missing, stale, and unsafe conditions handled? |
| [05 - Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md) | How is support introduced and later reversed without trapping a repository? |
| [06 - Validation roadmap](06-VALIDATION-ROADMAP.md) | Which matrices and evidence gates are required before support claims advance? |

## Platform scope

A Ferris platform profile is a versioned, expiring record for a named
application or consumer. It joins, without collapsing:

- package source, exact release, lock universe, target-active closure, and
  requested and effective features;
- Cargo, rustc, toolchain channel, compiler host, target triple, target
  specification, and Rust target tier;
- `core`, `alloc`, and `std` requirements;
- pointer width, endianness, atomic widths and CAS, unwind, SIMD, thread,
  allocator, clock, entropy, filesystem, process, socket, and other
  architecture or environment capabilities;
- build scripts, procedural macros, code generators, native libraries, ABIs,
  providers, SDKs, sysroots, linkers, archivers, package discovery, and
  generated outputs;
- resolve, check, build, link, execute, test, package, sign, deploy, debug,
  operate, service, rollback, and removal evidence; and
- owners, approvals, support dates, limitations, expiry, replacement, and
  residual unknowns.

The normative profile fields and lifecycle are defined by
[PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md). The seven
program ownership model is defined by the
[Ferris seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Required platform families

Ferris must be able to represent distinct profiles for at least these families:

| Family | Typical boundary that must remain explicit |
|---|---|
| Linux | GNU versus musl, libc floor, distro/system packages, ELF, linker, loader, DWARF, container or host runtime |
| Windows | MSVC versus GNU ABI, Windows SDK and CRT, import libraries, PDB, signing, installer and servicing channel |
| macOS | Apple SDK and deployment target, Mach-O, codesigning, notarization, entitlements, dSYM, supported hardware architecture |
| Android | NDK/API level, ABI, linker/sysroot, JNI boundary, package format, signing, emulator/device runner |
| iOS | Xcode SDK, deployment target, simulator versus device, Apple signing/provisioning, framework/package boundary |
| WASM/WASI/browser | `wasm32` target identity, browser/worker/Node/WASI/component host, JavaScript or WASI provider, runtime and packaging |
| Embedded/bare metal/RTOS | board/MCU, memory map, linker script, startup/runtime, panic, allocator, interrupts, probe/runner, image and flashing |

These are families, not support claims. "WASM", "ARM", "Unix", "mobile", and
"embedded" are too broad to be profile identities.

## Stage model

Every result is stage-specific:

```text
resolve -> check -> build -> link -> execute -> test
                                      |
                                      v
package -> sign/attest -> deploy -> operate -> service
                                      |
                                      v
                              rollback -> remove
```

- A cross-check proves that selected Rust units reached checking for a target.
- A cross-build may produce target objects or libraries.
- A cross-link proves that a target linker and its inputs produced an image.
- A cross-run proves that a runner, emulator, simulator, device, browser, or
  target host executed that image.
- A cross-test proves only the selected test scope in that execution context.
- A deployment observation adds packaging, installation, configuration,
  policy, runtime, health, and recovery evidence.

No earlier stage promotes a later stage. A successful `cargo check --target`
does not prove linking, runtime behavior, packaging, signing, deployment, or
support.

## State vocabulary

Use typed states rather than one pass/fail or portable/not-portable flag:

| State | Meaning |
|---|---|
| `pass` | The named stage completed under the recorded identity and acceptance criteria |
| `fail` | The stage ran and violated its expected result |
| `expected-rejection` | The stage produced the specifically required rejection for a negative case |
| `unsupported` | An owner or approved profile explicitly rejects the combination |
| `unavailable` | Required infrastructure or capability was not present |
| `not-observed` | The stage was intentionally not executed or no evidence was collected |
| `stale` | Evidence existed but exceeded age or was invalidated by a material change |
| `unknown` | The available records cannot establish a more specific state |

`expected-rejection` is a stage result, while `unsupported` is an owner or
profile support state. A required rejection can pass its negative test while
the tested combination remains unsupported. Do not collapse those records or
turn missing evidence into success-shaped output.

## Safety and mutation boundary

These guides authorize documentation and planning only. Ferris must not,
without a separately approved implementation and action authority:

- install or update rustup toolchains or target components;
- install compilers, linkers, SDKs, Xcode, Visual Studio workloads, NDKs,
  sysroots, package managers, system libraries, runtimes, emulators, or device
  tools;
- modify Cargo manifests, lockfiles, features, target configuration, CI,
  signing policy, deployment configuration, or provider selection;
- accept licenses, provisioning profiles, certificates, entitlements, or
  external terms;
- upload, sign, notarize, flash, publish, deploy, or post externally; or
- infer approval from a green check, build, or demonstration.

Plans may identify missing prerequisites and owner-native commands. Execution
requires explicit owner approval, policy, credentials handling, audit,
rollback, and a separate authority record.

## Role-derived review lens

The nine repository roles require the platform program to preserve:

- Rust safety and explicit unsafe, ABI, panic, concurrency, and allocator
  assumptions;
- representative performance measurements without weakening correctness;
- actionable host/target, FFI, linker, generated-code, and deployment
  diagnostics;
- evidence that distinguishes observation from assertion;
- upstream ownership and contribution before weaker duplication;
- ordinary Cargo and editor workflows plus removability;
- real native, operational, support, and training constraints;
- bounded scope and visible non-goals; and
- reproducible commands, environments, negative cases, and actual results.

Role descriptions are in
[`.roles`](../../../.roles/ROLE.md); they do not themselves approve any profile.

## Sources

- [Ferris context](../../../CONTEXT.md)
- [Ferris agent instructions](../../../AGENTS.md)
- [Ferris product plan](../../../PRODUCT_PLAN.md)
- [Rust platform compatibility research](../../research/2026-08-09-rust-platform-compatibility.md)
- [Rust native dependency boundary research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Ferris seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
- [Mirrored Rust target and toolchain guide](../../reference/rust-reference/rust-architecture/02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md)
- [Mirrored target compatibility guide](../../reference/rust-reference/rust-crate-ecosystem/11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)
- Role lenses:
  [Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md),
  [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md),
  [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md),
  [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md),
  [Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md),
  [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md),
  [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md),
  [Scope Keeper](../../../.roles/editorial/scope-keeper.md), and
  [Validation Checker](../../../.roles/editorial/validation-checker.md)
