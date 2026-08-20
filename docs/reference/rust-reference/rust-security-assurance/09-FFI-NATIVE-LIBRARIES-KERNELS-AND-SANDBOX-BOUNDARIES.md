---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:ffi-native-libraries-kernels-and-sandbox-boundaries
kind: guide
module: rust-security-assurance
section: security-engineering
title: FFI, Native Libraries, Kernels, and Sandbox Boundaries
status: source-custody
source_custody: partial
current_path: rust-security-assurance/09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md
canonical_path: rust-security-assurance/09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md
backsource_ids: [proof-backfill:rust-security-assurance:09-ffi-native-libraries-kernels-and-sandbox-boundaries]
concepts: [ffi, native libraries, kernels, sandboxing, abi, containment]
root_concepts: [ffi security]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# FFI, Native Libraries, Kernels, and Sandbox Boundaries

Rust's language guarantees stop at contracts the compiler cannot see. Across
FFI, the application must establish layout, calling convention, ownership,
lifetime, thread, and unwind rules. Below the process, native libraries, the
kernel, drivers, hypervisor, and sandbox policy remain part of the trusted
computing base.

## The Big Picture

```
+============================================================================+
|                           CONTAINMENT STACK                                |
+============================================================================+
| safe Rust API                                                              |
+------------------------- sound wrapper ------------------------------------+
| unsafe Rust + FFI declarations + marshaling                                |
+------------------------- ABI boundary -------------------------------------+
| C/C++/assembly library + allocator + language runtime                      |
+------------------------- syscall boundary ---------------------------------+
| kernel + drivers + namespaces/job/sandbox policy                           |
+------------------------- virtualization boundary --------------------------+
| hypervisor/firmware/hardware                                                |
+============================================================================+
```

Containment reduces consequence; it does not retroactively make an invalid FFI
contract sound.

## Specify the ABI Contract

| Contract | Questions |
|----------|-----------|
| Calling convention | Is `extern "C"` or another ABI correct for every target? |
| Layout | Are transferred types `#[repr(C)]`/`#[repr(transparent)]` and composed only of ABI-defined fields? |
| Ownership | Who allocates, frees, closes, and may retain each handle? |
| Lifetime | How long are pointers/callback contexts valid? |
| Mutability/aliasing | May native code retain or concurrently mutate memory? |
| Errors | Error code, out parameter, nullable pointer, thread-local error, exception? |
| Unwinding | Can panic/exception cross? Usually contain and translate at the boundary. |
| Threads | Which thread may call, callback, initialize, or destroy? |

Rust has no stable Rust-native ABI for independently built foreign consumers.
Default Rust layout, `String`, `Vec`, references, data-carrying enums, futures,
and trait-object vtables are not durable foreign contracts. Use explicit
C/system ABIs, opaque handles, fixed-layout fields, or a process protocol.

```
Rust owner --borrow--> native call --returns before borrow ends
Rust owner --transfer-> native handle --matching Rust-exported free/close
native owner --borrow--> Rust wrapper --never outlives native guarantee
```

Never infer ownership from pointer type alone.

## Keep the Public Surface C-Shaped

```rust
#[repr(C)]
pub struct ByteSlice {
    pub ptr: *const u8,
    pub len: usize,
}

// Contract: ptr is null only when len == 0; otherwise it is readable for len
// bytes and retained only for the duration of the call.
```

The struct illustrates why `repr(C)` is necessary but insufficient. It fixes
layout according to the target C ABI; it does not validate the pointer or define
encoding, retention, aliasing, or thread rules. Prefer caller-owned call-scoped
views and opaque handles over exposing allocator capacity or Rust collection
internals. A Rust adapter must special-case `len == 0` before calling
`slice::from_raw_parts`, because that Rust API still requires a non-null,
properly aligned pointer even for a zero-length slice.

## Callbacks, Re-Entrancy, and Unwinding

```
Rust holds invariant temporarily
      |
      +--> calls native
              |
              +--> callback re-enters Rust
                         |
                         +--> observes broken invariant  [BUG]
```

Assume callbacks can re-enter unless the native contract forbids it. Restore
invariants before external calls or use a state that rejects re-entry safely.
Contain Rust panics before non-unwind C boundaries; translate native exceptions
through C-compatible wrappers rather than allowing them to cross an unsupported
ABI.

`catch_unwind` can translate eligible unwinding Rust panics; it cannot catch
`panic = "abort"`, undefined behavior, process termination, or arbitrary foreign
exceptions. Use an unwind-capable ABI such as `"C-unwind"` only for a deliberate
cross-language contract supported by every compiler and target in scope.

## Native Dependency Assurance

Cargo's `-sys` convention and `links` key identify many native integrations, but
the actual library may be bundled, built from source, discovered on the host, or
provided by the OS.

| Source | Assurance concern |
|--------|-------------------|
| Bundled source | upstream identity, patches, compiler flags, update cadence |
| System library | distro/platform patch level and runtime image inventory |
| Prebuilt binary | provenance, signature/digest, target/ABI compatibility |
| Locally compiled C/C++ | compiler/sysroot identity, hardening, sanitizers |
| Kernel API/device | privilege, ioctl/input surface, driver trust |

Static linking simplifies deployment but can hide outdated native code inside
an artifact. Dynamic linking can enable centralized patching but introduces
runtime search-path and compatibility concerns. Choose based on operations and
threat model, not on a blanket security slogan.

## Sandbox by Capability and Consequence

```
untrusted input
      |
      v
low-privilege worker process
  files: explicit roots only
  network: required destinations only
  syscalls/devices: minimal set
  identity: no ambient production admin token
  resources: CPU/memory/process/handle limits
      |
      v
validated result over narrow IPC
```

Platform controls differ:

| Platform family | Examples of mechanisms | Qualification |
|-----------------|------------------------|---------------|
| Linux | user/mount/network namespaces, seccomp, capabilities, cgroups, LSMs | containers share a kernel; policy and kernel version matter |
| Windows | restricted tokens, AppContainer, job objects, process mitigation policies | exact capability depends on application model and OS |
| macOS | sandbox profiles/entitlements, hardened runtime | deployment/signing model constrains availability |
| VM/hypervisor | separate guest kernel and device model | stronger boundary at higher operational cost |
| WebAssembly runtime | capability-oriented host imports, runtime limits | runtime/host implementation becomes TCB |

Validate the deployed policy. Merely running "in a container" is not a
meaningful containment claim.

## Boundary Evidence

| Claim | Evidence |
|-------|----------|
| ABI matches supported targets | generated/header comparison, target builds, integration tests |
| Ownership is single and explicit | API contract, drop/free tests, sanitizer/Miri where applicable |
| Native parser cannot compromise service identity | separate low-privilege process/VM and IPC validation |
| Required libraries are patched | native SBOM/runtime inventory and advisory response |
| Sandbox blocks forbidden capability | negative tests against deployed policy |

## Old World -> New World Bridge

| Established practice | Rust analogue |
|----------------------|---------------|
| P/Invoke/COM interop contract | `extern` declarations plus safe Rust wrapper |
| C++ RAII handle wrapper | Rust newtype with `Drop`, ownership encoded in API |
| Native helper process | sandboxed Rust/native worker over narrow IPC |
| Windows job/restricted token | platform containment below Rust process |
| Linux seccomp/container | syscall/resource boundary; not a language guarantee |

Azure sandboxing, Windows Defender Application Control, Microsoft container
hardening, and confidential-computing offerings can supplement the platform
layer. The universal design remains least privilege, narrow IPC, resource
limits, verified policy, and a documented TCB.

## Common Confusion Points

- **"`repr(C)` makes a type FFI-safe."** Every field, value, ownership rule, and
  target ABI must also be valid.
- **"Rust owns the pointer because it has a Rust type."** Ownership is defined by
  the boundary contract, not syntax.
- **"Static linking removes dependency risk."** It embeds the dependency and its
  patching obligation.
- **"A container is a sandbox."** Isolation depends on privileges, mounts,
  syscalls, devices, network, resources, and shared-kernel threat.
- **"Miri covers native code."** Miri has limited FFI support; sanitizers and
  native tests cover different paths, and neither proves containment.
- **"Safe wrapper means safe platform."** Kernel/driver/runtime compromise can
  violate lower-layer assumptions.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| Designing new FFI | Prefer opaque handles and a small C-shaped API |
| Passing buffers | Specify allocator, ptr/len validity, retention, mutation, and release |
| Native callback | Model re-entry, thread, lifetime, and panic/exception behavior |
| High-risk native parser | Move to least-privilege worker or stronger sandbox |
| Static native library | Include version/provenance in SBOM and patch process |
| Container deployment | Test actual capability/syscall/resource policy |
| Cross-target support | Validate ABI and native toolchain per target |

## Primary Sources

- Rustonomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- Rust Reference, External blocks:
  https://doc.rust-lang.org/reference/items/external-blocks.html
- Rust Reference, Type layout: https://doc.rust-lang.org/reference/type-layout.html
- Cargo `links`: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
- Linux kernel seccomp documentation:
  https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html
- Windows application security documentation:
  https://learn.microsoft.com/windows/security/application-security/

## Related Guides

- Previous: [08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md](08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md)
- Next: [10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md](10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md)
- Unsafe language boundary: [../rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md](../rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md)
