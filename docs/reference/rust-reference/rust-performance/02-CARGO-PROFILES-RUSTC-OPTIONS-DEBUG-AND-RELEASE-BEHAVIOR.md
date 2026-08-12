---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:cargo-profiles-rustc-options-debug-and-release-behavior
kind: guide
module: rust-performance
section: rust-performance
title: Cargo Profiles, rustc Options, Debug, and Release Behavior
status: source-custody
source_custody: partial
current_path: rust-performance/02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md
canonical_path: rust-performance/02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md
backsource_ids: [mdloom-backfill:rust-performance:02-cargo-profiles-rustc-options-debug-and-release-behavior]
concepts: [cargo profiles, rustc options, release builds, debug builds, optimization levels, codegen units]
root_concepts: [cargo profiles]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Cargo Profiles, rustc Options, Debug, and Release Behavior

## The Big Picture

`cargo build` and `cargo build --release` select profiles; profiles become
per-crate `rustc` options; the target and linker complete the artifact contract.
The profile name is not itself a performance guarantee.

```
+=============================================================================+
|                         BUILD CONFIGURATION FLOW                            |
|                                                                             |
| Cargo.toml profiles + Cargo config + env + CLI + target                     |
|                  |                                                          |
|                  v                                                          |
| Cargo unit graph: host crates, target crates, dependencies, build scripts   |
|                  |                                                          |
|                  v                                                          |
| rustc per crate: opt-level, debuginfo, CGUs, incremental, panic, overflow   |
|                  |                                                          |
|                  v                                                          |
| backend + linker: LTO, target CPU/features, symbols, final binary           |
+=============================================================================+
```

## Dev and Release Are Policy Bundles

Cargo's built-in profiles choose different trade-offs. Exact defaults are
documented by Cargo and can evolve, so inspect effective policy rather than
memorizing folklore.

| Dimension | Dev intent | Release intent | Why it matters |
|-----------|------------|----------------|----------------|
| Optimization | Fast compilation, low/no optimization | Runtime optimization | Changes code shape dramatically |
| Debug assertions | Usually enabled | Usually disabled | May retain validation branches |
| Overflow checks | Usually enabled | Usually disabled unless configured | Integer behavior differs where overflow is possible |
| Debug info | Useful for debugging | Often reduced/absent unless requested | Profiles need symbols for good stacks |
| Incremental | Commonly enabled | Commonly disabled | Build time and CGU reuse trade-offs |
| Codegen units | More parallelism | Fewer units than dev | Compile time vs optimization scope |

Never benchmark unoptimized dev code and generalize to release. Conversely,
release-only behavior can hide diagnostics or make debugging harder; create a
profiling profile rather than repeatedly editing release policy.

## A Practical Profile Set

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"

[profile.profiling]
inherits = "release"
debug = 1
strip = "none"

[profile.release.package."*"]
opt-level = 3
```

Use these as starting points, not universal recommendations:

- `panic = "abort"` can reduce unwind machinery and size, but changes panic
  behavior and may be incompatible with requirements to catch unwinds.
- `codegen-units = 1` can improve optimization opportunities while lengthening
  builds; measure both runtime and delivery cost.
- `strip = "symbols"` improves shipped size but removes symbol-table data.
  Preserve separate symbols/PDBs in your release pipeline before stripping.
- dependency profile overrides are useful when dev builds spend most runtime in
  unoptimized dependencies; they can increase incremental build time.

Named custom profiles are stable Cargo functionality. Invocation is:

```
cargo build --profile profiling
```

The artifact directory is normally `target/profiling`, not
`target/release`.

## Stable rustc Codegen Options

Most user-facing codegen controls use stable `-C` options. Prefer Cargo profile
keys when Cargo exposes the setting; they are easier to review and reproduce.

| Goal | Cargo/profile control | Direct rustc equivalent or note |
|------|-----------------------|---------------------------------|
| Optimization level | `opt-level` | `-C opt-level=...` |
| Debug information | `debug` | `-C debuginfo=...` |
| Codegen partitioning | `codegen-units` | `-C codegen-units=N` |
| LTO | `lto` | `-C lto=...` |
| Panic strategy | `panic` | `-C panic=abort|unwind` |
| Overflow checks | `overflow-checks` | `-C overflow-checks=yes|no` |
| Target tuning | Cargo config or `RUSTFLAGS` | `-C target-cpu=...`, `-C target-feature=...` |
| Frame pointers | Cargo config or `RUSTFLAGS` | `-C force-frame-pointers=yes` |

```
# Stable, one-off local experiment.
RUSTFLAGS="-C target-cpu=native -C force-frame-pointers=yes" \
  cargo build --profile profiling

# PowerShell syntax:
$env:RUSTFLAGS="-C target-cpu=native -C force-frame-pointers=yes"
cargo build --profile profiling
Remove-Item Env:RUSTFLAGS
```

`target-cpu=native` makes the binary specific to the build host's available CPU
features. It is appropriate for a controlled deployment fleet or local
experiment, not for a generic downloadable binary. Environment `RUSTFLAGS`
changes Cargo's fingerprint/cache identity and can affect dependencies and
host-side units as Cargo applies flags to rustc invocations. For cross builds or
release policy, prefer target-scoped Cargo configuration and verify the verbose
rustc command lines. Record every flag explicitly.

## Stable vs Nightly Diagnostics

```
stable public controls:  Cargo profile keys, rustc -C options, --emit kinds
nightly internals:       rustc -Z ..., many MIR/LLVM diagnostic dumps
```

`-Z` options require a nightly compiler and are intentionally unstable. They are
valuable for investigation, but pin the nightly date and never imply that output
format or option name is a compatibility promise.

```
# Nightly-only diagnostic example; exact option/output is version-sensitive.
rustup toolchain install nightly-2026-08-01
cargo +nightly-2026-08-01 rustc --release --lib -- -Z print-mono-items=lazy
```

The command selects a library target; use `--bin <name>` for a binary package.
Use stable tools such as `cargo llvm-lines`, disassemblers, profilers, and
artifact-size tools when they answer the question without compiler internals.

## Target, Features, and Build Identity

The same source and profile can produce materially different code:

| Input | Example consequence |
|-------|---------------------|
| Target triple | ABI, object format, unwind model, available instructions |
| Cargo features | Different dependency/code paths; features are part of workload identity |
| `target-cpu`/features | Vector width and instruction selection |
| Linker | Link time, identical-code folding, debug format, binary layout |
| Panic strategy | Unwind tables and cleanup behavior |
| LTO | Cross-unit optimization and link cost |
| Toolchain version | LLVM and optimizer changes |

Record `rustc -Vv`, the target triple, `Cargo.lock`, `cargo tree -e features`,
profile definitions, config files, and relevant environment variables.

## Inspect What Cargo Actually Runs

```
# Stable: show rustc command lines and freshness decisions.
cargo build --profile profiling -vv

# Stable: timings report for the build graph.
cargo build --timings

# Stable: print target-specific cfg values.
rustc --print cfg --target x86_64-unknown-linux-gnu

# Stable: list codegen options and descriptions.
rustc -C help
```

`cargo --timings` HTML support is stable on current Cargo; machine-readable
timing formats may have different stability, so check the Cargo version before
automating them. MSVC builds require the Visual C++ linker environment; GNU and
musl targets use different linkers and libraries.

## Old World -> New World Bridge

| Old world | Rust |
|-----------|------|
| Debug/Release solution configurations | Cargo `dev`/`release` profiles |
| Custom MSBuild configuration | Named profile with `inherits` |
| `/O2`, `/DEBUG`, `/GL`, LTCG | `opt-level`, `debug`, `lto`, codegen-unit controls |
| RyuJIT tiering knobs | No default JIT; AOT codegen and link policy decide final code |
| ReadyToRun/native AOT target matrix | Rust target triples plus CPU feature policy |
| Symbol server/PDB retention | Preserve PDB/DWARF/dSYM separately from stripped binaries |

The universal model is configuration layering. Cargo owns build orchestration;
rustc owns compilation; the backend and platform linker own the final machine
artifact.

## Common Confusion Points

- **`--release` does not mean "maximum performance."** It selects a policy
  bundle that may not fit latency, size, or build-time goals.
- **`opt-level = 3` is not always faster than `2`.** Larger code can hurt
  instruction-cache behavior.
- **`target-cpu=native` harms portability.**
- **Debug symbols do not usually mean unoptimized code.** A profiling profile
  can be optimized and symbolized.
- **Stripping without archiving symbols damages production diagnosis.**
- **`-Z` options are nightly and version-sensitive.**
- **Changing flags invalidates caches.** Include compile-time effects in the
  decision.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Fast edit-build loop | `dev` profile; tune dependencies selectively |
| Representative runtime benchmark | `release` or a named release-derived profile |
| Profile optimized code | release-derived `profiling` with `debug = 1`, symbols retained |
| Small portable binary | Measure `opt-level = "s"`/`"z"`, LTO, panic abort, and stripping |
| Fleet-specific maximum throughput | Evaluate pinned target CPU/features on the oldest supported fleet CPU |
| Investigate codegen internals | Pinned nightly `-Z` only for diagnostics; do not ship the dependency casually |
| Reproduce a result | Save profile, lockfile, features, target, flags, linker, and `rustc -Vv` |

## Primary Sources

- Cargo profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- Cargo configuration: https://doc.rust-lang.org/cargo/reference/config.html
- rustc codegen options: https://doc.rust-lang.org/rustc/codegen-options/
- rustc platform support: https://doc.rust-lang.org/rustc/platform-support.html

## Related Guides

- Measurement contract: [01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md)
- Final artifact tuning: [11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md](11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md)
- Build-time trade-offs: [12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md](12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md)
