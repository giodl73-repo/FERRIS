---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:linking-lto-pgo-bolt-and-binary-size
kind: guide
module: rust-performance
section: rust-performance
title: Linking, LTO, PGO, BOLT, and Binary Size
status: source-custody
source_custody: partial
current_path: rust-performance/11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md
canonical_path: rust-performance/11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md
backsource_ids: [proof-backfill:rust-performance:11-linking-lto-pgo-bolt-and-binary-size]
concepts: [linking, link time optimization, profile guided optimization, bolt, binary size, symbols]
root_concepts: [link optimization]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Linking, LTO, PGO, BOLT, and Binary Size

## The Big Picture

Late-stage optimization trades build complexity and time for cross-unit
visibility, workload-informed layout, and smaller or faster artifacts.

```
+=============================================================================+
|                         FINAL ARTIFACT PIPELINE                             |
|                                                                             |
| rustc mono items -> codegen units -> object/bitcode -> linker -> binary     |
|        |                |                 |             |                   |
|        |                +-> Thin/Fat LTO -+             +-> strip/symbols   |
|        |                                                     |              |
|        +-> instrumentation build -> representative run -> profile data      |
|                                                    |                        |
|                                                    +-> PGO rebuild          |
|                                                                             |
| linked binary + execution profile -> BOLT post-link layout (target-specific)|
+=============================================================================+
```

## Linkers and Link Time

Cargo/rustc drive the platform linker. Choice depends on target:

| Target family | Common linkers/tools | Notes |
|---------------|----------------------|-------|
| Linux GNU | GNU ld, gold, lld, mold | Availability and flags vary by distribution |
| Windows MSVC | `link.exe` or lld-link-compatible paths | PDB generation and Visual C++ environment matter |
| macOS | Apple `ld` via toolchain | dSYM and platform SDK contracts |
| musl/cross targets | target-specific linker | Static linking and native dependencies change size |

A faster linker can materially improve builds without changing runtime. It can
also differ in identical-code folding, debug support, and flag compatibility.
Validate correctness and artifact policy before standardizing.

```
# See the exact linker invocation.
cargo build --release -vv

# Override examples belong in target-specific Cargo config, not universal docs.
# Verify linker installation and target compatibility before use.
```

## LTO: Cross-Unit Visibility

| Setting | Optimization scope | Build cost | Typical use |
|---------|--------------------|------------|-------------|
| `lto = "off"` | No LTO | Lowest link cost | Explicitly disable even local Thin LTO |
| `lto = false` | Cargo/rustc may perform thin local LTO across CGUs within a crate when applicable | Low | Cargo default policy; not identical to `"off"` |
| Thin LTO | Scalable cross-unit summaries and selected importing | Moderate | Strong release default candidate |
| Fat LTO | Broad whole-program LLVM optimization | Highest CPU/memory/link time | Maximum-performance experiment |

```toml
[profile.release]
lto = "thin"
codegen-units = 1
```

One codegen unit and fat LTO are not universally best. Thin LTO can deliver much
of the measured win for some workloads with less build cost; multiple CGUs may
compile faster. Benchmark runtime, binary size, peak link memory, and release
critical-path time. Preserve the exact Cargo version because profile semantics
and backend implementation are part of the observation.

## PGO: Optimize for an Observed Workload

Profile-guided optimization has three stages:

```
instrumented build -> representative executions -> merge profiles -> rebuild
```

```powershell
# Windows PowerShell example for a binary target named `my_app`, using stable
# rustc codegen flags and LLVM tools from `llvm-tools-preview`.
rustup component add llvm-tools-preview
$root = (Resolve-Path .).Path
$profiles = Join-Path $root "pgo-data"
$profdata = Join-Path $profiles "merged.profdata"
$hostTriple = ((rustc -vV | Select-String "^host:").Line -split "\s+", 2)[1]
if (Test-Path $profiles) { throw "Archive or remove stale pgo-data first." }
New-Item -ItemType Directory $profiles | Out-Null

$env:RUSTFLAGS="-Cprofile-generate=$profiles"
$env:CARGO_TARGET_DIR=(Join-Path $root "target-pgo-generate")
cargo build --release --target $hostTriple
& "$env:CARGO_TARGET_DIR\$hostTriple\release\my_app.exe" workload.json

$sysroot = rustc --print sysroot
$llvmProfdata = Get-ChildItem "$sysroot\lib\rustlib" -Recurse `
  -Filter llvm-profdata.exe | Select-Object -First 1 -ExpandProperty FullName
if (-not $llvmProfdata) { throw "llvm-profdata.exe not found in selected sysroot" }
& $llvmProfdata merge -o $profdata $profiles

$env:RUSTFLAGS="-Cprofile-use=$profdata"
$env:CARGO_TARGET_DIR=(Join-Path $root "target-pgo-use")
cargo build --release --target $hostTriple

Remove-Item Env:RUSTFLAGS
Remove-Item Env:CARGO_TARGET_DIR
```

The script assumes the selected Windows target emits `my_app.exe`; replace the
name with the repository's actual binary. On Linux/macOS, use shell environment
syntax, find `llvm-profdata` under the
sysroot's `lib/rustlib/<host>/bin`, and use the platform binary path. A helper
such as `cargo-binutils` can also locate LLVM tools. Keep generation and use
target directories separate so instrumented objects are never mistaken for
final ones. The explicit `--target` keeps host build scripts from being
instrumented under Cargo's target/host flag separation; verify with `-vv`.
Absolute profile paths avoid Cargo's varying rustc working directories. Start
with an empty profile-data directory, keep all other rustc flags identical
between generation and use, and adjust the executable suffix on non-Windows
hosts.

The workload is the policy. A profile dominated by startup may hurt steady-state
throughput; one dominated by the common request may neglect rare but
latency-critical paths. Build a weighted training suite, version it, and
evaluate the optimized artifact on separate holdout workloads so the gate does
not merely reward overfitting to the training run.

PGO codegen options are stable user-facing rustc controls; profile format remains
tied to the compiler's LLVM toolchain. Generate and consume with compatible
tool versions.

## BOLT: Post-Link Optimization

BOLT uses execution profiles to reorder functions and basic blocks in an already
linked binary. It can improve instruction-cache and branch behavior after the
linker has fixed most layout.

```
linked binary -> collect BOLT-compatible profile -> llvm-bolt -> reordered binary
```

Current upstream BOLT operates on **x86-64 and AArch64 ELF binaries**. It is not
a Windows/MSVC or macOS/Mach-O release step. Input needs an unstripped symbol
table, and upstream recommends emitted relocations for maximum optimization
opportunity. PIE and shared-object support exist but should be validated against
the pinned LLVM version. Test unwind, debug, startup, correctness, and signing
behavior on the rewritten artifact.

```
llvm-bolt --help
perf2bolt --help

# Exact collection/rewrite flags vary by LLVM/BOLT version and target.
# Pin the LLVM toolchain in release automation.
```

Treat BOLT as an optional final stage after ordinary profiling, LTO, and PGO.
It adds profile custody, tooling, artifact validation, and rollback requirements.

## Binary Size

File size is not one number:

| Metric | Why track it |
|--------|--------------|
| Unstripped binary | Build/debug artifact cost |
| Stripped binary | Deployment/download footprint |
| Compressed package | Distribution bandwidth |
| Mapped code/data pages | Startup and steady working set |
| Symbols/PDB/dSYM | Diagnostic storage, not shipped process pages |

```
# External tools.
cargo install cargo-bloat --locked
cargo bloat --release --crates
cargo bloat --release -n 30

# LLVM/platform tools may also inspect sections.
llvm-size target/release/my_app
```

On Windows, use `dumpbin /headers`, `llvm-size`, and PDB tooling. Preserve PDBs
in a symbol store before stripping or packaging. On macOS, preserve dSYMs.

Size controls include:

- `opt-level = "s"` or `"z"` (measure runtime);
- Thin/fat LTO;
- `panic = "abort"` where semantics allow;
- stripping with separate symbol custody;
- removing unused features/dependencies;
- reducing monomorphization and proc-macro/runtime baggage;
- target-specific static vs dynamic linking choices.

## Artifact Verification

Every late optimization needs:

1. unit/integration tests on the final artifact;
2. smoke test on every supported target;
3. symbol/unwind verification for crash diagnosis;
4. startup and steady-state benchmark;
5. reproducible mapping from profile data to source/toolchain;
6. rollback to the unoptimized artifact.

Signed binaries may need signing after rewriting. Rewriting a signed executable
invalidates its signature.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| LTCG / whole-program optimization | Cargo/rustc LTO |
| Visual C++/.NET Native PGO | rustc LLVM instrumentation PGO |
| Profile-based code layout | BOLT where target/toolchain supports it |
| `/DEBUG` plus symbol server | DWARF/PDB/dSYM custody separate from stripped ship artifact |
| IL trimming / native size analysis | Cargo feature/dependency review plus native symbol/section analysis |
| Faster MSVC/link.exe alternatives | Target-compatible lld/mold experiments |

Rust adds monomorphization as a major source of both optimization opportunity and
artifact growth.

## Common Confusion Points

- **LTO, PGO, and BOLT are separate stages.**
- **PGO is only as representative as its training workload.**
- **A smaller binary is not always a faster binary.**
- **Stripping without symbol custody weakens production diagnosis.**
- **BOLT is not supported uniformly across targets.**
- **Changing linker or LTO can change build time more than runtime.**
- **Profile data and LLVM tools must be compatible.**
- **Final rewritten binaries need re-testing and re-signing.**

## Decision Cheat Sheet

| Goal | First experiment |
|------|------------------|
| Better release runtime with moderate build cost | Thin LTO |
| Maximum single-binary runtime | Compare fat LTO and CGU settings under release budget |
| Stable hot production workload | PGO with a versioned representative training suite |
| Remaining front-end/layout stalls on supported ELF target | Evaluate BOLT after PGO/LTO |
| Faster links | Compare target-compatible linker without changing runtime claims |
| Smaller deployment | Size profile, dependency/features audit, size opt-level, LTO, abort/strip policy |
| Preserve diagnosis | Optimized symbolized build, archive symbols, then package/strip |
| Ship decision | Apply [15](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md) to final artifact |

## Primary Sources

- Cargo profiles and LTO: https://doc.rust-lang.org/cargo/reference/profiles.html
- rustc PGO: https://doc.rust-lang.org/rustc/profile-guided-optimization.html
- LLVM BOLT README: https://github.com/llvm/llvm-project/blob/main/bolt/README.md
- rustc linker configuration: https://doc.rust-lang.org/rustc/codegen-options/index.html#linker
- cargo-bloat: https://github.com/RazrFalcon/cargo-bloat

## Related Guides

- Build configuration: [02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md](02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md)
- Monomorphization/codegen: [06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md](06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md)
- Compile-time cost: [12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md](12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md)
