# EXP-01: Function-Level Machine-Code Cache Matrix

Date: 2026-08-09

Question: can a backend reuse unchanged compiled functions precisely enough to
recover work lost at rustc codegen-unit boundaries, and what identity,
integrity, and operational controls are required?

## Environment

- Windows 11 build 26310 on local NTFS
- Intel Core i7-12800HX, 16 cores and 24 logical processors
- 31.7 GiB RAM
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM 23.1.0
- Target: `x86_64-pc-windows-msvc`
- Backend: distributed `rustc_codegen_cranelift`
- Direct cache library: `cranelift-codegen 0.134.3`
- Direct-cache repetitions: seven per shape
- Public control: METIS-CORE revision
  `78ae34090e043e79a206f2daffaa3889389b4790`

The direct harness constructed deterministic Cranelift functions, compiled
them through `Context::compile_with_cache`, and used an in-memory
`CacheKvStore`. The rustc harness generated 1,000 exported Rust functions in
one source module and compared optimized MIR, requested CGUs, incremental
timings, and self-profile evidence. The METIS control changed one allocation
inside `CoarseningHierarchy::build` from `Vec::new()` to
`Vec::with_capacity(8)` in a disposable copy.

Representative commands:

```powershell
cargo run --release --manifest-path cranelift-cache-lab\Cargo.toml
python measure_rust_function_granularity.py
$env:CARGO_PROFILE_DEV_CODEGEN_BACKEND = "cranelift"
cargo +nightly rustc --manifest-path metis-core\Cargo.toml --lib --offline `
  -Zcodegen-backend -- -Zdump-mir=optimized
```

## Direct Cranelift cache

Median wall time:

| Shape | No cache | Cold population | Exact hit | One local edit | Local hits |
|---|---:|---:|---:|---:|---:|
| 5,000 x 0 operations | 102.6 ms | 18.7 ms | 19.0 ms | 19.3 ms | 5,000 |
| 2,000 x 1 operation | 53.9 ms | 58.3 ms | 9.8 ms | 9.7 ms | 1,999 |
| 1,000 x 25 operations | 136.5 ms | 155.8 ms | 21.8 ms | 21.5 ms | 999 |
| 500 x 100 operations | 216.3 ms | 256.5 ms | 33.2 ms | 33.7 ms | 499 |

For unique nontrivial functions, exact hits avoided 81.9% to 84.6% of the
uncached compile median. Cold population cost 8.2% to 18.6% more than compiling
without the cache. A one-function body edit missed only that function.

The zero-operation functions intentionally had identical stencils. The first
function missed and the remaining 4,999 hit during the initial pass. One
72-byte entry represented all 5,000 function identities. This is compile-time
deduplication of equivalent backend input, independent of linker identical-code
folding.

## Key and storage behavior

| Shape | Entries | Serialized bytes | Emitted code bytes | Blob/code ratio |
|---|---:|---:|---:|---:|
| 5,000 x 0 operations | 1 | 72 | 60,000 | 0.0012 |
| 2,000 x 1 operation | 2,000 | 171,994 | 51,994 | 3.31 |
| 1,000 x 25 operations | 1,000 | 495,994 | 433,994 | 1.14 |
| 500 x 100 operations | 500 | 885,494 | 854,494 | 1.04 |

Renaming a function did not change its cache key. Editing a non-empty body did.
Changing the Cranelift optimization flag caused the unique functions to miss
and created a second key set. Small functions therefore need an admission
policy: their serialized representation and lookup cost can exceed their
machine-code size and recompilation cost.

## Corruption controls

The harness flipped one byte at 32 evenly distributed positions in a cached
blob, then compared the returned result with a fresh compilation.

| Shape | Accepted different output | Safe miss and recompile |
|---|---:|---:|
| 5,000 x 0 operations | 9 of 32 | 23 of 32 |
| 2,000 x 1 operation | 10 of 32 | 22 of 32 |
| 1,000 x 25 operations | 28 of 32 | 4 of 32 |
| 500 x 100 operations | 31 of 32 | 1 of 32 |

The API assumes that bytes returned for a key were inserted for that exact key.
Deserialization and the internal version marker are not content
authentication. A trusted in-process memory store can satisfy that precondition
operationally. Disk persistence, sharing, or transport requires authenticated
content integrity and the PERF-Q30 provenance boundary.

Cranelift's
`enable_incremental_compilation_cache_checks` control recompiled and asserted
equality for all 16 functions in the validation probe.

## rustc and CGU granularity

All 1,003 mono items in the generated one-module crate were assigned to one
actual CGU when 1, 16, or 256 CGUs were requested. Optimized MIR comparison
showed:

| Edit | Changed optimized MIR bodies | Stable bodies |
|---|---:|---:|
| One local body | 1 of 1,001 | 1,000 |
| Shared helper body at opt-level 0 | 1 of 1,001 | 1,000 |
| Broad scalar type change | 1,001 of 1,001 | 0 |

Minimally instrumented medians:

| Requested CGUs | Cold | Unchanged | Local edit | Helper edit | Broad type |
|---:|---:|---:|---:|---:|---:|
| 1 | 424.0 ms | 316.2 ms | 437.9 ms | 502.8 ms | 538.8 ms |
| 16 | 354.0 ms | 336.3 ms | 466.8 ms | 476.7 ms | 537.1 ms |
| 256 | 369.5 ms | 294.1 ms | 445.1 ms | 498.1 ms | 531.1 ms |

The local edit was slower than cold in two of three requested-count rows even
though only one optimized MIR body changed. Self-profile evidence showed the
reason: the one invalidated CGU caused 1,003 `codegen fn`, 1,003
`codegen clif ir`, and 1,003 `compile function` invocations. The unchanged
build loaded incremental state and reused the CGU work product.

The self-profile mode materially increased wall time and remains causal
evidence, not the primary timing source.

## Public METIS control

The METIS library produced 443 optimized MIR files. After the local allocation
edit:

- 442 of 443 optimized MIR bodies were byte-stable, or 99.77%;
- only `CoarseningHierarchy::build` changed;
- the incremental compiler emitted eight MIR dump files for recomputed bodies;
- `-Zprint-mono-items=yes` reported 2,371 mono items across 140 CGU labels; and
- the CGU containing `CoarseningHierarchy::build` contained nine mono items.
- the complete library test control passed all 180 tests.

METIS therefore demonstrates both sides of the boundary. Source-module
partitioning can already isolate much better than the synthetic one-module
case, but a one-function edit still invalidates neighboring mono items in the
same CGU. Function caching is an incremental precision layer, not a replacement
for good partitioning.

## Interpretation

- Function-level caching is technically real in Cranelift.
- A backend stencil is a precise machine-code reuse unit after rustc has
  produced exact CLIF.
- Rust semantic identity, monomorphization, layout, lowering, ABI, and
  dependency identity remain rustc responsibilities above that stencil.
- Hit rate alone is insufficient; population, lookup, serialization,
  restoration, memory, integrity, and eviction costs determine net benefit.
- Cross-function optimization, LTO, imported bodies, debug and unwind metadata,
  symbols, relocations, instrumentation, and target capabilities can enlarge
  the reuse unit or require separate validation.
- The viable first lane is local, development-only, compiler-managed Cranelift
  research.

## Limitations

- The direct harness bypassed rustc lowering and used an in-memory store.
- The generated Rust fixture put all owners in one source module.
- The public control used one local allocation-policy edit whose ordinary test
  outputs were unchanged; out-of-memory timing was not an equivalence claim.
- No rustc integration of Cranelift's function cache was implemented.
- No LLVM, release, LTO, remote, debugger, unwind, sanitizer, coverage, PGO,
  multi-platform, daemon-lifecycle, or long-running memory-pressure claim was
  tested.
