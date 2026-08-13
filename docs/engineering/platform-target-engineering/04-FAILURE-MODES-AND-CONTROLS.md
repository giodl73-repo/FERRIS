# Platform Failure Modes and Controls

Status: Guidance
Implementation authority: None

## Purpose

Platform engineering must preserve useful negative evidence. A missing linker,
an unsupported provider, an unrun test, and stale evidence require different
decisions. This guide defines common failure modes, controls, and stop
conditions without authorizing repair or environment mutation.

## Classification rule

Classify the earliest failing boundary and retain later stages as
`not-observed` unless they were independently attempted. Do not relabel an
environment failure as target incompatibility or a product defect as
infrastructure absence.

| State | Control |
|---|---|
| `expected-rejection` | Verify the exact negative predicate and keep support state separate |
| `unsupported` | Preserve owner diagnostic and rejected combination; do not force a fallback |
| `unavailable` | Name missing prerequisite and its owner; propose but do not perform installation |
| `not-observed` | State why it was not run and what evidence would be required |
| `stale` | Prevent promotion; schedule renewal under current identities |
| `unknown` | Widen to the smallest safe owner-native scope or full reference path |
| `fail` | Retain exact evidence, stop unsafe promotion, assign diagnosis owner |

## Resolution and feature failures

### Failure modes

- Cargo cannot parse a manifest under the selected Cargo version.
- Resolution selects a transitive package above the claimed compiler floor.
- Target-specific or build dependencies change the active closure.
- Default features re-enable `std` or a native/provider edge.
- Duplicate public types or traits break interchange.
- Package-root policy fails while a minimal consumer checks, or the reverse.

### Controls

- Record Cargo and rustc as a pair.
- Preserve manifest and lock identity, resolver, complete universe, and active
  target/dependency-kind closure.
- Compare requested and effective features.
- Keep package-self and consumer results separate.
- Treat an MSRV pass as an exact observation, not a proven minimum.
- Do not mutate features or lockfiles automatically.

## Target and library-layer failures

### Failure modes

- Target component is absent.
- The selected target has `core` but not `alloc` or `std`.
- A dependency directly or transitively requires `std`.
- Pointer-width atomics or compare-and-swap are absent.
- Panic unwind, thread-local storage, SIMD, allocator, entropy, clock, or
  threading assumptions are unmet.
- A custom or Tier 3 target requires a separately built sysroot.

### Controls

- Record target tier and component availability separately.
- Inventory exact `core`/`alloc`/`std` and architecture capability needs.
- Treat provider-backed fallbacks as explicit consumer choices with safety
  assumptions and tests.
- Record missing target libraries or sysroot as `unavailable`.
- Do not run `rustup target add` or `-Z build-std` from this guidance.

## Native discovery and generation failures

### Failure modes

- Compiler, assembler, archiver, `pkg-config`, vcpkg, CMake, `protoc`,
  libclang, or another generator is missing.
- Discovery finds host libraries for a cross target.
- A tool is found outside `PATH`, making a passive inventory incomplete.
- Generated binding/source identity changes.
- Vendored tools widen the package/download supply chain.
- System and bundled modes expose different versions, patches, licenses, or
  artifacts.

### Controls

- Record the executable that actually ran, its discovery mechanism and version.
- Require target-specific discovery roots, SDK/sysroot, headers, libraries,
  and cross configuration.
- Preserve host process inputs, Cargo directives, generated output, and native
  artifacts.
- Distinguish system, bundled, prebuilt, generated, pregenerated, vendored-tool,
  and external-artifact modes.
- Stop on unexplained downloads, undeclared input access, or provider changes.
- Never install tools or switch provider/source mode automatically.

## Link failures

### Failure modes

- `cargo check --target` passes but the target linker is absent.
- SDK, sysroot, CRT/libc, startup object, framework, import library, linker
  script, or native library is missing.
- ABI, calling convention, symbol, architecture, link kind, or runtime library
  is incompatible.
- Static/dynamic policy differs from deployment expectations.
- Final artifact lacks required symbols, debug data, or reproducibility.

### Controls

- Treat check, codegen, archive, and final link as separate stages.
- Capture actual linker and link input identity.
- Record requested native name separately from discovered/linked artifact.
- Execute platform artifact inspection under owner-native tools.
- Require representative ABI calls for FFI support.
- Do not treat Cargo `links` as proof of native linkage.

## Execution and test failures

### Failure modes

- Runner, emulator, simulator, browser, WASI runtime, device, probe, or target
  host is missing.
- The artifact executes but provider, clock, entropy, filesystem, networking,
  threads, dynamic loading, or permissions fail.
- Tests compile but do not run.
- Cross-test coverage silently differs from native coverage.
- Device tests are flaky, destructive, or dependent on unrecorded state.

### Controls

- Record runner/runtime/device identity and exact test scope.
- Distinguish harness compilation, execution, integration, and operational
  validation.
- Bound retries and preserve first-failure evidence.
- Isolate device and deployment tests with stop/recovery procedures.
- Keep unexecuted tests `not-observed`.
- Fall back to the required full owner-native matrix when mapping is unknown.

## Packaging, signing, deployment, and servicing failures

### Failure modes

- Package omits native libraries, assets, debug symbols, licenses, or metadata.
- Windows, Apple, Android, or iOS signing/provisioning is unavailable.
- macOS notarization or platform policy rejects the artifact.
- Installer, container, package manager, bootloader, browser host, or store
  packaging changes runtime behavior.
- Deployment succeeds but health, rollback, crash symbols, or update ownership
  is absent.
- A system native dependency is outside the Rust update channel.

### Controls

- Qualify package contents and installation independently from compilation.
- Keep private keys, tokens, and provisioning material out of Ferris evidence.
- Record signing service identity and policy, not reusable credentials.
- Require deployment health, rollback marker, support and servicing owner.
- Join Rust and native advisory/license/update channels.
- Stop publication or deployment when required policy evidence is absent.

## Debugging failure modes

Support may require more than executable behavior:

- PDB, DWARF, dSYM, source maps, split debug, or firmware symbols are absent;
- optimized code prevents required locals or stack diagnosis;
- panic/unwind and mixed-language frames are not understandable;
- symbols cannot be matched to the deployed artifact;
- mobile or production crash symbols are not retained; or
- probe/device debug cannot recover safely.

Record debugging as a capability contract with a reproducible scenario. A
binary with `debuginfo` enabled does not by itself prove the required debugging
workflow.

## Platform-specific traps

| Family | Typical trap | Required classification |
|---|---|---|
| Linux | Cross-check passes but GNU linker/libc/sysroot absent | `unavailable` at link |
| Windows | Visual Studio tools found through vendor discovery, not `PATH` | Record actual discovered tools |
| macOS | Target installed but Apple SDK/codesigning absent | `unavailable`, not target unsupported |
| Android/iOS | Simulator passes, device or signing untested | Device/sign/deploy `not-observed` |
| Browser WASM | Module builds but provider/glue/browser behavior differs | Runtime/provider stage separate |
| WASI | One runtime/version passes | Do not generalize to browser or other WASI hosts |
| Embedded/RTOS | Library checks but final image/linker script/board not tested | Link/run/device stages `not-observed` |

## Safety stop conditions

Stop and require explicit owner review when a proposal would:

- introduce or change `unsafe`, FFI, ABI, allocator, critical-section, panic,
  interrupt, concurrency, crypto, entropy, or privilege assumptions;
- choose a provider or fallback not approved by the consumer;
- accept a license, certificate, provisioning profile, entitlement, or vendor
  agreement;
- install or mutate toolchains, SDKs, system packages, CI, devices, signing,
  deployment, or production state;
- hide a negative result, remove a mandatory gate, or claim support from
  incomplete evidence; or
- make rollback or ordinary Cargo operation unavailable.

## Sources

- [Ferris agent instructions](../../../AGENTS.md)
- [Platform compatibility research](../../research/2026-08-09-rust-platform-compatibility.md)
- [Native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Mirrored target compatibility guide](../../reference/rust-reference/rust-crate-ecosystem/11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)
- [Mirrored native dependency guide](../../reference/rust-reference/rust-crate-ecosystem/12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md)
