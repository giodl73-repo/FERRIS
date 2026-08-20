---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:packaging-deployment-versioning-support
kind: guide
module: rust-interop-migration
section: computing-software
title: Packaging, Deployment, Versioning, and Support
status: source-custody
source_custody: partial
current_path: rust-interop-migration/14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md
canonical_path: rust-interop-migration/14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md
backsource_ids: [proof-backfill:rust-interop-migration:14-packaging-deployment-versioning-support]
concepts: [Rust packaging, deployment, versioning, target matrix, cdylib, staticlib, support matrix, symbol versioning, native dependencies]
root_concepts: [interop packaging]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Packaging, Deployment, Versioning, and Support

Interop succeeds only when the right artifact is selected, loaded, diagnosed,
upgraded, and rolled back on every supported target. The package is part of the
boundary: a correct function signature in an unloadable DLL is not a working
contract.

## The Big Picture

```
+============================================================================+
|                       INTEROP DELIVERY PIPELINE                            |
+============================================================================+
|  SOURCE + LOCKFILE + TOOLCHAIN POLICY                                      |
|      |                                                                     |
|      v                                                                     |
|  BUILD MATRIX                                                              |
|  target triple x profile x features x runtime/host adapter                 |
|      |                                                                     |
|      v                                                                     |
|  ARTIFACT CONTRACT                                                         |
|  cdylib/staticlib/bin/wasm + exports + protocol/schema + version query     |
|      |                                                                     |
|      v                                                                     |
|  HOST PACKAGE                                                              |
|  NuGet RID | wheel | npm prebuild | JAR native | installer/image/MSIX      |
|      |                                                                     |
|      v                                                                     |
|  VERIFY                                                                    |
|  clean install -> load -> contract tests -> upgrade -> rollback -> symbols |
|      |                                                                     |
|      v                                                                     |
|  SUPPORT                                                                   |
|  compatibility matrix + SBOM/provenance + servicing owner + retirement     |
+============================================================================+
```

## Artifact Types

| Rust crate type | Consumer | Interop posture |
|-----------------|----------|-----------------|
| `cdylib` | Foreign dynamic loader | Export explicit C/system ABI only |
| `staticlib` | Foreign final linker | Include native dependency/link requirements |
| `bin` | Process/service/supervisor | Protocol is the contract |
| `rlib` | Matching rustc build | Internal Rust compilation artifact; not durable foreign package |
| `dylib` | Rust dynamic linking | Rust ABI/toolchain coupled; not a foreign stability contract |
| `wasm` component/module | Compatible runtime/host | WIT/module imports/exports plus runtime feature matrix |

Rust ABI, compiler metadata, symbol mangling, generic instantiations, and trait
object vtables are not stable packaging contracts. If two Rust crates must share
those, compile them together under one pinned toolchain or expose a different
boundary.

## Target Matrix

```
  target triple
  x86_64-pc-windows-msvc
  aarch64-pc-windows-msvc
  x86_64-unknown-linux-gnu
  x86_64-unknown-linux-musl
  aarch64-apple-darwin
          |
          +-> host runtime/version
          +-> CPU baseline
          +-> libc/CRT and dynamic deps
          +-> package identifier/tag/RID
```

Build commands are scoped evidence:

```powershell
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

```text
cargo build --release --target x86_64-unknown-linux-gnu
```

Cross-compilation also needs a target linker, sysroot/SDK, native dependencies,
and tests on the real target. A successful link on the build host is not a load
test.

## Contract Versioning

Separate four version axes:

| Version | Meaning |
|---------|---------|
| Rust crate SemVer | Source dependency compatibility for Rust builds |
| Foreign ABI/protocol version | What non-Rust consumer can call/read |
| Host package version | NuGet/wheel/npm/JAR/installer resolution |
| Deployment/schema version | What mixed old/new processes and data support |

They may advance together, but they are not interchangeable. Export a cheap
version/capability query or handshake and fail early with an actionable message.
For C records, use size/version fields when extension is required:

```c
typedef struct rim_options_v1 {
    uint32_t size;
    uint32_t version;
    uint32_t flags;
    uint32_t reserved;
} rim_options_v1;
```

The callee validates `size` before reading fields and requires reserved fields
to be zero. A new incompatible contract gets a new function/interface/version.

## Compatibility and Version Skew

Assume the host package and native artifact can become mismatched through loader
search paths, stale extraction caches, side-by-side installs, partial upgrades,
or rollback.

| Host/native combination | Policy |
|-------------------------|--------|
| Host N with native N | Primary tested pair |
| Host N with native N-1 | Support only if the ABI capability range says so |
| Host N-1 with native N | Usually reject unless forward compatibility is explicit |
| Host N with unknown native | Fail before creating handles or registering callbacks |
| Two native versions loaded in one process | Avoid unless symbols, global state, allocators, and callbacks are isolated by design |

Probe the foreign contract version immediately after loading and before passing
state. Prefer package-private absolute loading or unique side-by-side artifact
names over global search-path luck. Retain the prior host/native pair as one
rollback unit; restoring only one half can create a new incompatibility.

## Loader and Dependency Closure

| Platform/host | Typical load problem |
|---------------|----------------------|
| Windows | Wrong architecture, Safe DLL search, missing VC runtime/dependent DLL, package identity |
| Linux | glibc baseline, `rpath`/`RUNPATH`, SONAME, missing `.so`, symbol version |
| macOS | install names, `@rpath`, code signing/notarization, universal slices |
| Python | Incorrect wheel tag or unbundled native dependency |
| Node | Wrong OS/arch/libc prebuild or package `exports` |
| JVM | Extraction collision, classloader, `java.library.path`, dependent library |
| .NET | RID resolution, publish mode, trimming/NativeAOT, dependent native library |

Inspect artifacts with platform tools (`dumpbin`, `link /dump`, `readelf`,
`objdump`, `otool`, symbol scanners) in CI where available. More importantly,
load them from a clean consumer test package.

## Support Contract

Publish a matrix with:

- supported host/runtime versions;
- OS, architecture, libc/CRT, and minimum platform;
- package versions and compatible ABI/protocol ranges;
- security update and end-of-support policy;
- debug symbol/source correlation;
- crash dump and telemetry collection path;
- native dependency and license inventory;
- rollback artifact retention;
- owner for bridge and core.

Do not claim "supports Linux" if only one glibc container was tested. Name the
actual target and baseline.

## Boundary Hazard Register

| Hazard | Packaging/support rule |
|--------|------------------------|
| ABI | Export/test only declared C/system/protocol/WIT contracts; scan for accidental symbols; never support Rust ABI or trait objects externally. |
| Allocator | Package the matching release APIs and compatible runtime dependencies; static/dynamic CRT policy is recorded. |
| Panic/unwind | Build panic policy is known (`unwind`/`abort`); foreign entry points still contain/translate as designed. |
| Lifetime | Unload policy states whether live handles/callbacks/threads prevent unload; process packages define drain/stop. |
| Threading | Runtime initialization and shutdown integrate with host unload/service lifecycle; no orphan callbacks after package update. |
| Target | Build and test every claimed OS/arch/libc/CRT/runtime tuple on real or faithful target environments. |
| Packaging | Package selection, loader path, signing, dependencies, symbols, SBOM/provenance, upgrade, and rollback are release gates. |

## Old World -> New World Bridge

| Prior practice | Rust interop equivalent |
|----------------|-------------------------|
| DLL import library + redist | `cdylib` plus host package and dependency closure |
| NuGet/RID or Maven classifier | Select exact native target artifact |
| COM interface version | Stable foreign contract version independent of Rust crate |
| MSI/MSIX rollback | Retain previous native artifact and compatible state |
| PDB/source indexing | Native symbols and source commit/toolchain correlation |
| Servicing baseline | Explicit support matrix and security patch process |

## Common Confusion Points

- **"`cargo build --release` produces a portable binary."** It produces for one
  target with specific runtime dependencies and CPU assumptions.
- **"Static linking removes all dependencies."** System libraries, licenses,
  kernel interfaces, and host runtime contracts remain.
- **"SemVer protects the C ABI."** Only if the project defines and tests ABI
  compatibility; Cargo SemVer is a source ecosystem convention.
- **"Package managers solve native discovery."** They select files; loader and
  transitive dependency rules still apply.
- **"The managed package version proves the loaded native version."** Loader
  search, caches, and partial upgrades can select a different artifact; query
  the native contract before use.
- **"Unload is just dropping the library."** Live threads, callbacks, TLS,
  handles, and host references can make unload unsafe.
- **"Stripping symbols is harmless."** It can remove accidental exports or
  destroy production diagnosability if symbol artifacts are not retained.

## Decision Cheat Sheet

| Need | Do |
|------|----|
| Foreign dynamic library | `cdylib` with controlled export surface |
| Foreign static link | `staticlib` plus documented native link dependencies |
| Independent rollout | `bin`/service with versioned protocol |
| Host package | Publish per exact target identity, not generic OS label |
| ABI evolution | Query/version/capability plus additive structures or new entry point |
| Host/native skew | Early version probe, explicit compatibility range, paired rollback |
| Production diagnosis | Retain symbols, source/toolchain metadata, crash correlation |
| Support claim | Publish and continuously test the full compatibility matrix |

## Primary Sources

- Cargo build command and target selection: https://doc.rust-lang.org/cargo/commands/cargo-build.html
- Rust Reference, linkage: https://doc.rust-lang.org/reference/linkage.html
- Cargo profiles and panic setting: https://doc.rust-lang.org/cargo/reference/profiles.html
- Rust platform support: https://doc.rust-lang.org/rustc/platform-support.html
- Reproducible Builds: https://reproducible-builds.org/docs/

## Related Guides

- Previous: [13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md](13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md)
- Next: [15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md](15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md)
- Rust artifact internals: [../rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md](../rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md)
