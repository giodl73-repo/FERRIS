---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:native-dependencies-build-scripts-system-packages
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Native Dependencies, Build Scripts, and System Packages
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md
canonical_path: rust-crate-ecosystem/12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:12-native-dependencies-build-scripts-system-packages]
concepts: [native dependencies, build.rs, system packages, sys crates, cross compilation]
root_concepts: [native dependency integration]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Native Dependencies, Build Scripts, and System Packages

## The Big Picture

A Rust dependency can introduce a second package ecosystem and a second
toolchain. Build scripts run on the host, discover or build native inputs, and
emit instructions for target compilation and linking.

```
+===========================================================================+
|                      RUST + NATIVE BUILD PIPELINE                         |
+===========================================================================+
| HOST                                                                      |
|  build.rs -> host executable -> probe/generate/compile C -> cargo output  |
|                                      |                                    |
|                                      v                                    |
| TARGET                             headers/libs/sysroot                    |
|  Rust crate -> `-sys` bindings -> safe wrapper -> rustc -> native linker  |
|                                                    |                      |
|                                                    v                      |
|                                     executable/shared/static artifact     |
+===========================================================================+
| host != target during cross compilation; discovery must respect TARGET.   |
+===========================================================================+
```

The dependency review must include the native library, its package source,
headers, compiler/linker, ABI, license, advisories, and deployment model.

## `build.rs` as Executed Code

A build script is compiled for the host and runs before the package's target
units. It communicates with Cargo through stdout directives.

```rust
fn main() {
    println!("cargo::rerun-if-env-changed=LIBFOO_DIR");
    println!("cargo::rerun-if-changed=wrapper.h");
    println!("cargo::rustc-check-cfg=cfg(has_libfoo)");

    if let Ok(dir) = std::env::var("LIBFOO_DIR") {
        println!("cargo::rustc-link-search=native={dir}/lib");
        println!("cargo::rustc-link-lib=foo");
        println!("cargo::rustc-cfg=has_libfoo");
    }
}
```

The `cargo::KEY=VALUE` syntax requires Cargo 1.77 or newer; older supported
Cargo versions require the legacy `cargo:KEY=VALUE` form. Individual
directives can have later floors: `cargo::rustc-check-cfg` requires Rust/Cargo
1.80 or newer. This is an example of tool behavior that must be bounded by
MSRV/Cargo version.

| Directive family | Purpose |
|------------------|---------|
| `rerun-if-*` | Declare inputs that invalidate build-script output |
| `rustc-link-*` | Add native link library/search arguments |
| `rustc-cfg` and `rustc-check-cfg` | Define and validate conditional compilation |
| `rustc-env` | Set compile-time environment values |
| `metadata` | Pass data from a `links` crate to direct dependents |
| `warning` | Surface build diagnostics |

If a script does not declare narrow rerun inputs, Cargo may rerun it more often
or miss an external input the script failed to model. Environment, filesystem,
network, clocks, and tools are reproducibility boundaries.

## The `-sys` and Safe-Wrapper Pattern

```text
native libfoo
    |
    v
libfoo-sys: raw extern declarations, constants, build/link discovery
    |
    v
libfoo: safe ownership, lifetime, error, thread, and invariant model
    |
    v
application adapter
```

The `-sys` suffix and `links` field are conventions/mechanisms, not safety
certificates.

```toml
[package]
name = "libfoo-sys"
version = "0.8.0"
links = "foo"

[build-dependencies]
pkg-config = "0.3"
cc = "1"
```

Cargo permits only one package with a given `links` value in a resolved graph,
preventing multiple packages from independently owning link instructions for
the same native library. Arbitrary metadata emitted by that package is exposed
through `DEP_<LINKS>_<KEY>` environment variables only to its direct
dependents, so wrapper layers must forward metadata intentionally when needed.

## System, Vendored, or Prebuilt

| Native supply model | Advantages | Risks/costs |
|---------------------|------------|-------------|
| System package | Security servicing integrates with OS; smaller Rust build | Version drift, distro variance, deployment dependency |
| Vendored source built by crate | Consistent source and easier static packaging | Longer builds, duplicate patch channel, compiler requirements |
| Prebuilt binary | Fast and can simplify proprietary SDK use | Platform matrix, provenance, ABI, signing, redistribution |
| Organization-built artifact | Controlled flags and patching | Internal release/retention/support burden |

Do not assume a crate feature named `vendored` has identical meaning across
crates. It may build upstream source, download an archive, or select a bundled
copy. Inspect source and documentation.

## Discovery Tools

Common build dependencies include `cc`, `pkg-config`, and `vcpkg`.

```rust
fn main() {
    pkg_config::Config::new()
        .atleast_version("1.2")
        .probe("libfoo")
        .expect("libfoo 1.2+ is required");
}
```

This is valid only where `pkg-config` metadata and target-aware configuration
exist. In cross compilation, a host `pkg-config` result can incorrectly locate
host libraries. Configure target sysroots and tool-specific cross settings, or
use an explicit artifact path.

| Variable/input | Role |
|----------------|------|
| `HOST` | Triple where build script runs |
| `TARGET` | Triple being produced |
| `OUT_DIR` | Cargo-managed generated output directory |
| `CC_<target>` / `AR_<target>` | Target C compiler/archive selection used by common tooling |
| target linker config | Linker Cargo/rustc uses for target |
| sysroot/package roots | Headers and libraries for target |

## Cargo Target Configuration

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[env]
PKG_CONFIG_SYSROOT_DIR = { value = "/opt/aarch64-sysroot", force = true }
```

Cargo's `[env]` table is not target-scoped, so place target-specific values in a
dedicated target CI job/configuration or wrapper rather than reusing this file
for unrelated targets. Environment configuration support and relative-path
interpretation should be validated against the installed Cargo version. For
hermetic builds, prefer workspace-relative or image-owned paths over
developer-machine assumptions.

## Native Dependency Review

```
1. identify native library and ABI
2. identify source: OS, vendor, downloaded, bundled, internal
3. inspect build.rs and all downloads/probes
4. define host/target toolchain matrix
5. scan license/advisory channels for Rust and native components
6. test clean, offline, cross, release, and deployment builds
7. assign servicing and rollback owner
```

| Evidence | Example |
|----------|---------|
| Link result | Verbose release build and artifact inspection |
| Runtime dependency | `ldd`, `otool -L`, or Windows dependency tooling, platform-specific |
| Source custody | Native source/archive hash and origin |
| ABI compatibility | Supported native version range and integration tests |
| Reproducibility | Clean image build with declared toolchain/system packages |

## Old World -> New World Bridge

| Familiar native integration | Cargo ecosystem form |
|-----------------------------|----------------------|
| MSBuild custom task | `build.rs` host executable |
| C/C++ import library and headers | `-sys` crate plus link directives |
| NuGet package with native assets | Crate feature selecting bundled/prebuilt native artifacts |
| vcpkg/pkg-config discovery | Same tools called from build scripts |
| P/Invoke wrapper | Raw FFI crate plus safe Rust wrapper |

The universal model remains compiler, linker, ABI, and deployment custody.
Cargo makes the orchestration convenient; it does not erase native boundaries.

## Common Confusion Points

- **"Pure Rust application means no native dependencies."** Transitive crates
  can add C/C++, assembly, system libraries, or platform SDKs.
- **"Build scripts run for the target."** They run on the host and configure
  target units.
- **"`vendored` is automatically hermetic."** Inspect downloads, tools, flags,
  timestamps, and generated outputs.
- **"`links` guarantees one native version."** It guarantees one package owner
  per links value, not ABI compatibility with the installed library.
- **"`cargo check` proves linking."** It often avoids the final link; use a real
  build and execute where support claims require it.

## Decision Cheat Sheet

| Need | Prefer |
|------|--------|
| Distro-integrated security servicing | System package with explicit version matrix |
| Portable static appliance | Reviewed vendored source or organization-built artifact |
| Raw FFI | Established `-sys` crate with narrow responsibility |
| Safe product API | Safe wrapper behind internal adapter |
| Cross compilation | Explicit target compiler, sysroot, discovery roots, and runner |
| Reproducible build | No network in build script, declared inputs, pinned native toolchain |

## Primary Sources

- Cargo build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html
- Cargo `links`: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
- Cargo configuration: https://doc.rust-lang.org/cargo/reference/config.html
- Rust FFI guidance: https://doc.rust-lang.org/nomicon/ffi.html
- Cargo target configuration: https://doc.rust-lang.org/cargo/reference/config.html#target

## Related Guides

- Previous: [11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md](11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)
- Next: [13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md](13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md)
- Compiler-side host/target context: [../rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md](../rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md)
