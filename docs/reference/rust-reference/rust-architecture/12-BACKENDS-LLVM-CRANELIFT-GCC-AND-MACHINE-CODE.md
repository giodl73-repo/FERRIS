---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:backends-llvm-cranelift-gcc
kind: guide
module: rust-architecture
section: rust-architecture
title: Codegen Backends - LLVM, Cranelift, GCC, and Machine Code
status: source-custody
source_custody: partial
current_path: rust-architecture/12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md
canonical_path: rust-architecture/12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md
backsource_ids: [proof-backfill:rust-architecture:12-backends-llvm-cranelift-gcc]
concepts: [codegen backend, llvm, cranelift, gcc backend, lto, pgo]
root_concepts: [codegen backend]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Codegen Backends - LLVM, Cranelift, GCC, and Machine Code

## The Big Picture

The Rust compiler is not LLVM wearing a crab logo. `rustc` owns the language-facing front end, the query-driven middle end, MIR, monomorphization, codegen-unit partitioning, and the decision to invoke a backend. LLVM, Cranelift, and the GCC backend are code-generation engines that rustc drives through a backend interface. That interface is a rustc implementation detail, not a stable plugin ABI.

```
+===========================================================================+
|                         RUSTC TO MACHINE CODE                             |
|                                                                           |
|  Rust Reference / language authority                                      |
|        |                                                                  |
|        v                                                                  |
|  rustc front + middle end                                                 |
|  parse -> HIR -> typeck -> MIR -> mono MIR -> codegen units        [11]   |
|        |                                                                  |
|        | internal codegen_backend interface                               |
|        v                                                                  |
|  rustc_codegen_llvm     rustc_codegen_cranelift     rustc_codegen_gcc     |
|  default / bundled      fast debug iteration        libgccjit path        |
|          v                       v                         v              |
|  LLVM IR -> LLVM opt -> obj      CLIF -> obj               GCC IR -> obj  |
|           \_______________________|__________________________/            |
|                                   v                                       |
|                         platform linker                  [13]             |
|                    exe / dylib / cdylib / staticlib                       |
+===========================================================================+
```

Read the boundary literally: rustc decides what Rust means; the backend decides how target instructions are selected and optimized; the platform linker produces the final artifact. Cargo schedules the crate graph before any of this happens [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md).

---

## Boundary: What rustc Hands to a Backend

| Layer | Authority | What exists here | Stability |
|-------|-----------|------------------|-----------|
| Language surface | Rust Reference, RFCs, editions | Type system, traits, unsafe rules, ABI names | Contract-ish when documented |
| rustc front/middle | rustc | HIR, MIR, queries, monomorphized items | Internal |
| Codegen-unit partition | rustc | Mono items grouped into CGUs [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) | Internal policy |
| Backend interface | rustc | `codegen_backend` callbacks and data structures | Internal, version-sensitive |
| Backend optimizer | LLVM / Cranelift / GCC projects | IR, instruction selection, register allocation, object emission | Owned by backend project |
| Final link | Platform linker | Relocation, symbol resolution, final image | OS/toolchain contract [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) |

The important term is **monomorphized MIR**. By the time the backend sees a generic `Vec<T>` method, rustc has selected concrete instantiations such as `Vec<u8>` or `Vec<MyType>`. That is why backend choice is not a language choice: borrow checking, trait selection, and generic specialization have already happened.

---

## The LLVM Path: Codegen Unit to Object File

```
+------------------+     +------------------+     +-------------------+
| monomorphized    |     | rustc lowers to  |     | LLVM optimization |
| MIR in one CGU   | --> | LLVM IR + attrs  | --> | pipeline          |
+------------------+     +------------------+     +-------------------+
          |                        |                        |
          |                        | target datalayout      | -C opt-level
          |                        | target-cpu/features    | -C target-cpu
          v                        v                        v
+-----------------------------------------------------------------------+
|                         target object file                            |
|             sections, relocations, symbols, debug info                |
+-----------------------------------------------------------------------+
```

The default backend is `rustc_codegen_llvm`, which links to a bundled LLVM through FFI. The Rust distribution ships a specific LLVM version, usually with Rust-specific integration patches. Generated LLVM IR is therefore useful for inspection, debugging, and interop experiments, but it is not a Rust stability promise.

`-C opt-level` selects rustc codegen settings that map into LLVM optimization pipelines. `-C target-cpu` and `-C target-feature` shape the CPU model, feature set, vector ISA, ABI-relevant target attributes, and target datalayout seen by LLVM. The target specification and backend jointly determine pointer sizes, alignment, relocation model, and available instructions.

| Flag | Scope | Typical effect |
|------|-------|----------------|
| `-C opt-level=0..3,s,z` | LLVM optimization pipeline | Compile time vs runtime/code size trade-off |
| `-C target-cpu=native` | CPU model | Enables instructions available on the build host |
| `-C target-feature=+avx2` | ISA features | Enables or disables specific backend features |
| `--emit=llvm-ir` | Diagnostic/interop output | Writes LLVM IR for inspection |
| `--emit=asm,obj` | Backend output kind | Assembly or object files, target-owned format |

---

## Alternative Backends: Cranelift, GCC, and gccrs

| Backend | What it is | Primary reason to care | Status |
|---------|------------|------------------------|--------|
| LLVM | External optimizer/codegen project, bundled and driven by rustc | Best-supported targets and production performance | Default |
| Cranelift | Rust-native code generator through `rustc_codegen_cranelift` | Much faster debug-build iteration, especially when runtime speed is secondary | Nightly preview component on supported hosts; otherwise build from source |
| GCC backend | `rustc_codegen_gcc`, using libgccjit | Potential GCC target coverage and GCC optimization stack | Experimental, not default |
| gccrs | A separate GCC front-end reimplementation of Rust | Alternative compiler project, not rustc | Separate from rustc backends |

On hosts for which the Rust distribution publishes it, install Cranelift into
the matching nightly toolchain with:

```text
rustup component add rustc-codegen-cranelift --toolchain nightly
```

Component availability is host- and nightly-specific. If it is absent from
`rustup component list --toolchain nightly`, follow the backend repository's
source-build instructions rather than assuming a generic plugin ABI.

Cranelift is the iteration backend: lower latency, fewer optimizer ambitions. It is attractive for debug builds, test loops, and compiler-development scenarios where waiting for LLVM dominates. The GCC backend is a different experiment: keep rustc's front/middle end, but target GCC infrastructure through libgccjit. `gccrs` is not that; it is a separate Rust front end inside GCC and should not be confused with `rustc_codegen_gcc`.

```
Same rustc front/middle end
        |
        +--> LLVM backend       production default
        +--> Cranelift backend  fast debug-loop backend
        +--> GCC backend        experimental GCC/libgccjit path

Different project:
        gccrs = GCC reimplements a Rust front end
```

---

## Optimization Context: LTO, PGO, and BOLT

```
NORMAL:       crate -> CGUs -> objects -> linker

THIN LTO:     crate -> LLVM summaries --------+
              crate -> LLVM summaries ----+   |
                                           v   v
                         cross-CGU/cross-crate LLVM import + inline
                                           |
                                           v
                                      final objects

PGO:          instrumented build -> run workload -> profile data
                         |                         |
                         +------ profile-use <----+

BOLT:         final linked binary -> post-link layout optimizer
```

| Mechanism | Rust knob | Owner of most semantics | Notes |
|-----------|-----------|-------------------------|-------|
| Thin/fat LTO | `-C lto=thin`, `-C lto=fat`, Cargo profile `lto` | LLVM plus linker integration | Cross-CGU/cross-crate inlining; interacts with `codegen-units` [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) |
| PGO | `-C profile-generate`, `-C profile-use` | LLVM PGO | Profile-guided block layout, inlining, branch weights |
| BOLT | Not a normal user `-C` codegen flag | LLVM BOLT project | Used in rustc's own build/distribution pipeline [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) |

LTO is where the local codegen-unit decision meets whole-program optimization. More CGUs improve parallelism and incremental reuse; fewer CGUs and LTO give LLVM broader visibility. PGO is workload truth injected into the optimizer. BOLT is later still: it rewrites a linked binary using profile data. It matters for understanding rustc's performance engineering, but it is not how ordinary Rust crates request backend codegen.

---

## Commands and Traces

| Question | Command |
|----------|---------|
| Which target CPUs does this backend know? | `rustc --print target-cpus` |
| What LLVM IR did rustc emit? | `rustc --crate-type=lib --emit=llvm-ir -C opt-level=2 src/lib.rs` |
| What assembly or object came out? | `rustc --crate-type=lib --emit=asm,obj -C opt-level=2 src/lib.rs` |
| What happens with host CPU tuning? | `RUSTFLAGS="-C target-cpu=native" cargo build --release` |
| How do I request ThinLTO? | Set Cargo profile `lto = "thin"`, then `cargo build --release` |

```text
# Inspect the LLVM handoff for one file.
rustc --crate-type=lib --emit=llvm-ir,obj -C opt-level=2 src/lib.rs

# Ask rustc which CPU names the selected target/backend understands.
rustc --print target-cpus
rustc --print target-features

# Use production-ish CPU tuning through Cargo.
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

Coordinate LTO and codegen units through one Cargo profile:

```toml
# Cargo.toml
[profile.release]
lto = "thin"
codegen-units = 1
```

```text
cargo build --release

# UNSTABLE/nightly: requires the matching Cranelift component above.
rustc +nightly -Z codegen-backend=cranelift src/main.rs
```

`-Z codegen-backend=...` is deliberately marked unstable. Some workflows configure `codegen-backend` through nightly configuration files or wrapper tools, but the compatibility story is still compiler-internal rather than ecosystem-stable.

---

## Old world -> New World Bridge

| Old world | Rust mapping | Difference that matters |
|-----------|--------------|-------------------------|
| Roslyn emits IL, CLR/RyuJIT lowers at runtime | rustc emits native objects ahead of time | Rust has no runtime JIT in normal execution |
| clang front end using LLVM | rustc front/middle end using LLVM | `rustc:LLVM :: clang:LLVM`; LLVM is not the language compiler |
| MSVC LTCG | `-C lto=thin/fat` | Same whole-program optimization idea, different ownership and object formats |
| .NET tiered PGO / ReadyToRun PGO | LLVM PGO via `-C profile-generate/use` | Profile data feeds AOT native optimization |
| Choosing RyuJIT vs alternative JIT/AOT backend | Choosing LLVM vs Cranelift/GCC backend | Backend choice changes codegen trade-offs, not Rust semantics |

If your mental model is MSBuild plus Roslyn plus CLR, remove the managed runtime box. Cargo schedules, rustc type-checks and monomorphizes, the backend emits native objects, and the platform linker finishes. The closest .NET analogy to Cranelift is not another language; it is a different code generator behind the same semantic front end.

---

## Stability and Ownership Boundaries

| Thing | Treat as |
|-------|----------|
| `codegen_backend` interface | Internal rustc implementation detail |
| MIR-to-backend lowering | Internal rustc implementation detail |
| LLVM IR emitted by rustc | Version-sensitive diagnostic/interop artifact |
| Bundled LLVM version and patches | Toolchain-version detail |
| Cranelift/GCC backend support | Evolving/experimental unless documented otherwise |
| Documented `-C` flags | Stable knobs when not `-Z` |
| `--emit=llvm-ir,asm,obj` output kinds | Stable set of requested outputs; file format owned by LLVM/target |
| Object file ABI | Platform/toolchain contract, not rustc's private promise |

The observable contract is that stable Rust compiles correctly for supported targets. The backend interface, pass ordering, IR shape, and alternate backend behavior are not a plugin ecosystem contract.

---

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Ship production Rust today | Default LLVM backend through stable rustc |
| Inspect what rustc handed LLVM | `--emit=llvm-ir` on a small crate or file |
| Tune for the deployment CPU | `-C target-cpu=<cpu>` or `target-cpu=native` when deployment matches build |
| Enable cross-crate optimization | Cargo profile `lto = "thin"` or `-C lto=thin` |
| Trade compile time for debug iteration | Evaluate Cranelift on nightly, behind tooling guardrails |
| Experiment with GCC infrastructure | Track `rustc_codegen_gcc`; expect churn |
| Debug final symbol/link behavior | Move to [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) |
| Understand CGU partitioning first | Read [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "LLVM is the Rust compiler." | LLVM is the default backend. rustc owns Rust semantics and the middle end. |
| "LLVM IR is a stable Rust IR." | It is a version-sensitive backend handoff/debug artifact. |
| "Cranelift changes the language." | It changes code generation trade-offs after type checking and MIR. |
| "The GCC backend is gccrs." | No. `rustc_codegen_gcc` is a rustc backend; gccrs is a separate GCC front end. |
| "LTO is just a linker flag." | It coordinates rustc CGUs, LLVM summaries/bitcode, and linker behavior. |
| "BOLT is another `-C` backend option." | BOLT is post-link binary optimization, used notably in rustc's own build pipeline. |
| "`target-cpu=native` is portable." | It bakes in host CPU assumptions; use only when deployment hardware matches. |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| rustc-dev-guide: Code generation | rustc's codegen architecture and backend handoff |
| rustc-dev-guide: Backend agnostic codegen | The internal backend abstraction and what it deliberately does not promise |
| rustc-dev-guide: Debugging LLVM | Reading LLVM IR and diagnosing backend issues |
| rustc-dev-guide: LTO | Thin/fat LTO behavior and codegen-unit interaction |
| The rustc book: codegen options, `--emit`, `--print` | Stable command-line surface for backend-related flags |
| `rust-lang/rustc_codegen_cranelift` | Cranelift backend status and usage notes |
| `rust-lang/rustc_codegen_gcc` | GCC backend status and experimental limitations |
| LLVM documentation | LLVM IR, optimization pipelines, target features, BOLT |
| gccrs project documentation | Separate GCC Rust front-end effort, not a rustc backend |

*Cross-links:* start with the landscape in [00](00-OVERVIEW.md), then read monomorphization and CGUs in [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md), artifact/link behavior in [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md), incremental reuse in [14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md), and rustc distribution/perf in [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md).