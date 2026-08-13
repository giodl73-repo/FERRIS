# Platform Evidence and Identity

Status: Guidance
Implementation authority: None

## Purpose

Platform claims are trustworthy only when a reviewer can identify exactly what
was selected, what ran, where it ran, what it produced, and what was not
observed. This guide defines the evidence joins required by Ferris platform
profiles.

## Identity graph

Do not create one "platform hash." Retain typed identities and joins:

```text
consumer requirement
  -> profile revision
  -> source/manifest/lock identity
  -> Cargo resolution and active closure
  -> host toolchain and host-executed units
  -> target specification and target capabilities
  -> native/provider/tool/SDK/sysroot identities
  -> invocation identity
  -> artifact identities
  -> validation result
  -> package/sign/deploy/operation outcome
```

Query Forest may preserve these records and immutable roots, but a root or ref
does not prove compatibility, validation, trust, availability, or support. See
the [seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Required identity classes

### Consumer and profile

- application, repository, workspace, component, and owner;
- profile ID, revision, schema version, support state, approval and expiry;
- requirements, non-goals, exceptions, unsupported combinations;
- predecessor, superseding profile, rollback target, and removal owner.

### Rust source and resolution

- registry/source kind, package name/version, archive checksum, source revision;
- manifest and lockfile digests;
- Cargo resolver and exact Cargo/rustc versions;
- lockfile package universe;
- target-active normal, build, and development closures;
- requested/effective features and target `cfg`;
- build scripts, procedural macros, generated code, `links`, unsafe and FFI
  edges.

### Host

- OS edition/release, architecture, host triple, container/VM/image when used;
- selected toolchain and selection mechanism;
- Cargo/rustc executable identity and reported version;
- filesystem/path and environment policy;
- host build-script, proc-macro, generator, discovery, and wrapper executables;
- actual compiler/tool discovery path, not only `PATH` inventory.

### Target

- target triple and target specification identity;
- Rust target tier plus observation date;
- installed target component or custom/sysroot source;
- pointer width, endianness, alignment, atomics/CAS, SIMD, unwind, TLS,
  thread, allocator, panic, and ABI capabilities;
- required `core`, `alloc`, or `std` layer;
- OS/runtime, minimum version, libc/CRT, loader, board/MCU, RTOS, browser,
  WASI/component host, simulator/emulator, or device class.

### Native and provider

- capability and selected provider;
- system, bundled source, prebuilt object/binary, generated, pregenerated, or
  externally supplied source mode;
- native component version/revision/hash and patch set;
- compiler, assembler, archiver, linker, SDK, sysroot, headers, libraries,
  package database, generator, binding tool, and discovery inputs;
- Cargo directives and generated output;
- ABI, calling convention, link kind, runtime library, load result, and
  deployment identity.

### Invocation

An invocation identity includes:

- command and arguments;
- working directory and selected manifest;
- environment allowlist or digest with secrets excluded;
- host/target pair and profile;
- network/offline policy;
- resource, output, and time bounds;
- input identities;
- runner/device/deployment selection; and
- approval record when the action is mutating or privileged.

Reusable secrets, private keys, tokens, provisioning material, and raw
credentials must not enter identities, logs, roots, prompts, or durable
evidence.

### Artifacts and outcomes

Track independently:

- `.rmeta`, `rlib`, object, archive, static/shared library, executable, WASM
  module/component, firmware image, debug data, generated source, package, SBOM,
  signature/attestation, and deployment artifact;
- content hash, size, format, producer invocation, target, and retention;
- native and Rust artifact joins;
- installed/loaded/executed identity;
- test and operational result;
- rollback and removal result.

Rust metadata is compiler-version-sensitive; platform object, link, and debug
formats remain owned by their toolchain ecosystems. See the
[mirrored artifact guide](../../reference/rust-reference/rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md).

## Evidence by stage

| Stage | Minimum evidence |
|---|---|
| Resolve | Manifests, lock, Cargo/rustc, source identities, complete universe, active closures, features |
| Check | Invocation, selected units, target, diagnostics, package-root versus consumer scope |
| Build | Codegen/profile inputs, produced Rust/native artifacts, build-script and macro outcomes |
| Link | Actual linker, arguments or plan identity, SDK/sysroot, native inputs, final image |
| Execute | Runner/runtime/device, deployed image, configuration, stdout/stderr/exit or device outcome |
| Test | Harness/binary/cases, execution substrate, filters, retries, result and coverage limits |
| Package | Package format, contents, metadata, symbols, dependencies, reproducibility observation |
| Sign/attest | Signer/service identity, policy, timestamp, artifact digest; never private key material |
| Deploy | Destination/ring, package identity, configuration, installation and health result |
| Debug | Symbol/source identity, debugger/tool, scenario, observable capability and limitations |
| Service | Update channel, patch owner, incident path, support dates, telemetry and recovery evidence |
| Rollback/remove | Prior/replacement identity, procedure, cleanup inventory, validation and retained history |

## State model

Record state per identity and stage:

| State | Use |
|---|---|
| `pass` | Acceptance criteria met for the exact observation |
| `fail` | The stage executed and did not meet criteria |
| `expected-rejection` | The stage produced the required rejection for the exact negative case |
| `unsupported` | Owner/profile explicitly excludes the combination |
| `unavailable` | A required component, service, tool, SDK, runner, device, or permission was absent |
| `not-observed` | No attempt or result exists for the stage |
| `stale` | Previously valid evidence is outside age or invalidated by change |
| `unknown` | Evidence exists but cannot establish a more precise result |

Recommended failure detail includes `compile-failed`, `link-failed`,
`missing-tool`, `missing-system-package`, `discovery-failed`,
`generator-failed`, `runner-unavailable`, `runtime-failed`,
`package-failed`, `signing-unavailable`, `deploy-failed`, and
`rollback-failed`. These details do not replace the top-level state.

## Evidence freshness and invalidation

Evidence expires at the profile date and sooner when any material identity
changes:

- source, direct or transitive package, lockfile, feature, or dependency kind;
- Cargo, rustc, toolchain channel, component, target specification, or tier;
- host OS/image, architecture, environment policy, or filesystem behavior;
- architecture capability, `core`/`alloc`/`std`, allocator, panic, or provider;
- native component, build script, generator, generated output, compiler,
  linker, SDK, sysroot, package database, runtime, runner, or device;
- packaging, signing, deployment, servicing, or policy configuration;
- owner, approval, support term, advisory, license, or evidence revocation.

Do not refresh an observation timestamp without rerunning or revalidating the
evidence under its stated method.

## Platform-specific evidence additions

| Family | Additional identities |
|---|---|
| Linux | libc/loader/distro or image, system packages, kernel assumptions, ELF/debug package |
| Windows | SDK/toolset/CRT, MSVC or GNU ABI, import libraries, PDB, signer/installer/minimum OS |
| macOS | Apple SDK/deployment target, architecture slices, frameworks, entitlements, signature/notarization, dSYM |
| Android | NDK/API level/ABI, emulator or device, APK/AAB, keystore service identity, native symbols |
| iOS | Xcode SDK/deployment target, simulator/device, provisioning/entitlements, app/framework package, dSYM |
| WASM/WASI/browser | target, runtime class/version, JS/WASI provider, glue/bundle, browser matrix, source map |
| Embedded/RTOS | MCU/board, memory map/linker script, HAL/PAC/RTOS, probe, bootloader, image, flashing and recovery |

## Evidence quality checks

Before accepting a record, ask:

1. Can another authorized owner reproduce the observation?
2. Are command, environment, tool, source, closure, and target identities exact?
3. Are package-root, consumer, host, target, and deployment scopes distinct?
4. Are negative and missing states visible?
5. Are logs bounded and secrets excluded?
6. Are artifacts joined to the invocation that produced them?
7. Are limitations and untested stages explicit?
8. Is the evidence date and invalidation policy recorded?
9. Does the claim stay within what the evidence establishes?

## Sources

- [Ferris context](../../../CONTEXT.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Platform compatibility research](../../research/2026-08-09-rust-platform-compatibility.md)
- [Native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [Mirrored artifact and debug guide](../../reference/rust-reference/rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md)
- [Mirrored target compatibility guide](../../reference/rust-reference/rust-crate-ecosystem/11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)
