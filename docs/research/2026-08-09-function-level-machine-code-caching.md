# Function-Level Machine-Code Caching

Date: 2026-08-09
Question: PERF-Q31
Status: Complete
Decision: adopt function-level cache eligibility, identity, cost, and integrity
vocabulary in the read-only compiler query plan; maintain Cranelift fixtures
that expose precision lost at CGU boundaries; contribute evaluation and
correctness cases upstream; prototype only in disposable, local,
development-only Cranelift experiments behind exact toolchain and capability
boundaries; defer a FERRIUM daemon, external machine-code store, rustc
integration, LLVM and LTO reuse, release use, persistence, transport, and
implementation.

## Executive conclusion

Function-level machine-code caching is not speculative. Cranelift already has
an experimental cache API that hashes a `FunctionStencil` and target ISA
configuration, serializes a compiled stencil, and reapplies function-specific
parameters after retrieval. Wasmtime merged the design in 2022, and Rust's
accepted 2026 Cranelift performance goal explicitly proposes exploring a
persistent daemon that caches individual compiled functions in memory.

The measured opportunity is also real. Exact in-memory cache hits avoided
81.9% to 84.6% of direct compilation time for unique nontrivial functions. A
one-function edit retained every other entry. Five thousand identical empty
functions shared one entry during the first pass, demonstrating compile-time
deduplication before linking.

The rustc precision gap is equally concrete. A generated crate with 1,003 mono
items in one source module remained one actual CGU even when 256 were
requested. One changed optimized MIR body caused Cranelift code generation for
all 1,003 functions. In a public METIS control, 442 of 443 optimized MIR bodies
remained stable after one local allocation edit, but the affected CGU still
contained nine mono items.

Function reuse can therefore recover work that ordinary CGU work-product reuse
cannot isolate. It does not replace CGUs: partitioning still controls
parallelism, optimization scope, object formation, linking, and the number of
functions exposed to a cache miss.

The cache boundary is narrower than Rust correctness identity. rustc must own
the monomorphized instance, type and layout identity, MIR-to-CLIF lowering,
calling convention, target and target features, backend component,
optimization, panic and overflow behavior, instrumentation, dependency
metadata, imported or inlined bodies, symbols, visibility, relocations, unwind,
and debug requirements. Once rustc has produced exact CLIF, Cranelift's
`FunctionStencil` should remain authoritative for backend equivalence rather
than an external tool inventing a second machine-code key.

Cache economics are not uniformly favorable. Cold population added 8% to 19%
for unique functions. A one-operation function used about 3.31 serialized
bytes per emitted code byte, while larger functions approached parity.
The measurements support an admission threshold that avoids caching functions
cheaper to recompile.

Integrity is a hard boundary. Cranelift documents that returned bytes must have
come from the exact computed key, but the key-value trait does not authenticate
stored bytes. In the corruption sweep, many modified blobs still deserialized
as hits and produced output different from a fresh compilation. A trusted
in-process memory store can satisfy the API precondition. Persistence or
transport must add content integrity, provenance, quarantine, and recovery
under PERF-Q30.

The immediate FERRIUM opportunity is upstream-facing evidence:

- classify workloads where CGU reuse loses function precision;
- preserve exact semantic and backend identity requirements;
- measure admission, hit, restoration, and memory economics;
- add corruption, equivalence, debug, unwind, relocation, and broad-invalidation
  fixtures; and
- support the existing `rustc_codegen_cranelift` owners.

FERRIUM should not build a daemon or external cache. Lifecycle, isolation,
memory pressure, eviction, concurrency, versioning, observability, crash
recovery, and compiler integration belong with rustc and the backend. LLVM,
release, LTO, remote reuse, and arbitrary native machine-code artifacts remain
outside this result.

## Decision supported

This research determines:

- whether function-level caching exists in a suitable backend;
- how stencil identity and finalization parameters differ;
- where CGU work products lose one-function precision;
- the direct hit, cold-population, storage, and corruption behavior;
- which Rust identity belongs above the backend key;
- why small-function admission and net benefit matter;
- why persistence requires PERF-Q30 controls;
- which optimization and metadata features can enlarge the cache boundary; and
- why the opportunity belongs upstream in `rustc_codegen_cranelift`.

It does not authorize a daemon, rustc fork, external store, machine-code
restoration, production persistence, remote transport, LLVM caching, LTO,
release use, automatic backend selection, or implementation.

## Evidence reviewed

### Local evidence

- [EXP-01 function-cache matrix](perf-q31-function-cache/results/EXP-01-function-cache-matrix.md)
- [Monomorphization and generic-instance reuse](2026-08-09-monomorphization-generic-instance-reuse.md)
- [Codegen-unit partitioning](2026-08-09-codegen-unit-partitioning.md)
- [Development codegen backends](2026-08-09-development-codegen-backends.md)
- [Remote artifact provenance](2026-08-09-remote-artifact-provenance.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Upstream sources

- [Rust 2026 Cranelift performance goal](https://github.com/rust-lang/goals/blob/main/src/2026/improve-cg_clif-performance.md)
- [Cranelift incremental-cache source](https://docs.wasmtime.dev/api/src/cranelift_codegen/incremental_cache.rs.html)
- [Cranelift incremental-cache API](https://docs.wasmtime.dev/api/cranelift_codegen/incremental_cache/index.html)
- [Original cache design issue](https://github.com/bytecodealliance/wasmtime/issues/4155)
- [Merged Cranelift cache implementation](https://github.com/bytecodealliance/wasmtime/pull/4551)

## Current cache model

### rustc owns semantic identity

Before backend compilation, rustc determines the concrete mono item, type
layouts, calling convention, MIR, dependency and inline-body state, target,
panic behavior, instrumentation, symbol requirements, and selected backend.
Those values decide whether generating the same CLIF is correct.

An external source hash cannot substitute for that compiler state. Equal source
text can lower differently under another substitution, layout, dependency,
target feature, compiler revision, or instrumentation mode.

### Cranelift owns stencil equivalence

Cranelift splits a function into:

- `FunctionStencil`, the compilation input hashed for reuse; and
- function parameters applied after retrieval for final names, external
  identities, relocations, and related fixups.

The key includes the stencil, ISA name, target triple, shared flags, and
ISA-specific flags. The serialized value includes a Cranelift version marker.
Function names and selected external identifiers can change without changing
the stencil key.

This is the correct ownership split: rustc decides that exact CLIF is valid for
the Rust program, and Cranelift decides whether it has compiled that exact
backend input before.

### A cache hit is not free

The hit path still computes a key, retrieves bytes, deserializes a compiled
stencil, applies parameters, and finalizes the result. Persistent storage adds
serialization, I/O, integrity, locking, cleanup, and recovery. A daemon removes
some disk cost but adds process lifecycle, memory, isolation, protocol, and
version coordination.

The decision metric is avoided compile work minus all lookup, restoration,
storage, and lifecycle costs. Hit rate by itself is not an outcome.

### Optimization can enlarge the unit

The measured lane compiles independent Cranelift functions without LLVM or
LTO. If output depends on imported bodies, whole-program state, cross-function
optimization, instrumentation summaries, or profile data, those inputs become
part of identity or move the reusable unit above one function.

Debug, unwind, symbols, visibility, relocations, and object integration also
require explicit equality and debugger controls. Code bytes alone do not
establish a complete Rust object.

## Findings

### FERRIUM-418: Cranelift already proves function-cache feasibility

**Sources:** Cranelift cache API and source; merged PR `#4551`; EXP-01.

**Observed behavior:** `Context::compile_with_cache` computes a function key,
retrieves or compiles one stencil, serializes misses, and returns whether the
result was a hit. The API is feature-gated but implemented and used by
Wasmtime-facing code.

**Implication:** FERRIUM does not need to invent a backend cache design to
establish feasibility. It should evaluate and contribute to the existing one.

**Confidence:** High.

### FERRIUM-419: stencil identity and finalization parameters are separate

**Sources:** PR `#4551`; current incremental-cache source.

**Observed behavior:** the compiled stencil is keyed independently of function
names and selected external-name mappings, which are applied as parameters
after retrieval.

**Implication:** A useful key should ignore harmless renumbering while
preserving every compilation-relevant field. External tools should not hash
final symbol names as a proxy for backend equivalence.

**Confidence:** High.

### FERRIUM-420: exact hits avoid most direct Cranelift compilation work

**Sources:** EXP-01 direct-cache matrix.

**Observed behavior:** exact hits for unique nontrivial functions were 81.9% to
84.6% faster than uncached compilation. A one-function edit retained all other
entries.

**Implication:** The mechanism has enough headroom to matter in
backend-dominant development workflows, subject to rustc integration and
end-to-end controls.

**Confidence:** High for the harness; medium for complete Rust builds.

### FERRIUM-421: equivalent stencils can deduplicate during the first compile

**Sources:** EXP-01 zero-operation shape; PR `#4551`.

**Observed behavior:** 5,000 functions with different final names but identical
stencils produced one miss, 4,999 hits, and one 72-byte entry during cold
population.

**Implication:** Function caching can remove duplicate compilation within one
build or across related builds before linker identical-code folding.

**Confidence:** High for equivalent Cranelift input.

### FERRIUM-422: cold population and small functions need an economic gate

**Sources:** EXP-01.

**Observed behavior:** cold population added 8% to 19% for unique functions.
The one-operation shape stored 3.31 serialized bytes per emitted code byte;
larger shapes approached 1.0.

**Implication:** Admission should consider estimated compile cost, serialized
size, restoration cost, expected reuse, memory pressure, and eviction. Caching
every function can regress the workload.

**Confidence:** High for the measured shapes.

### FERRIUM-423: backend configuration partitions the key space

**Sources:** current cache-key implementation; EXP-01 flag mismatch.

**Observed behavior:** the key hashes the stencil, ISA name, target triple,
shared flags, and ISA-specific flags. Changing optimization caused unique
functions to miss and created a second entry set.

**Implication:** Target, backend version, and effective flags are identity, not
labels. Cross-configuration hit claims are invalid unless the backend key
proves equivalence.

**Confidence:** High.

### FERRIUM-424: the cache-store precondition requires authenticated integrity

**Sources:** current `CacheKvStore` contract; EXP-01 corruption sweep; PERF-Q30.

**Observed behavior:** corrupted values sometimes failed to deserialize and
recompiled, but 10 of 32, 28 of 32, and 31 of 32 sampled flips for the
nontrivial shapes were accepted as hits with output different from fresh
compilation.

**Implication:** In-memory compiler-owned storage may rely on process
isolation. Persistent, shared, or remote storage must bind key, bytes, size,
producer, and policy with cryptographic integrity, quarantine, and ordinary
rebuild recovery.

**Confidence:** High.

### FERRIUM-425: CGU work products can lose one-function precision

**Sources:** PERF-Q25; EXP-01 one-module rustc fixture.

**Observed behavior:** all 1,003 mono items remained in one actual CGU at
requested counts 1, 16, and 256. One changed optimized MIR body caused 1,003
Cranelift function compilations.

**Implication:** Function caching can recover backend work when source-module
partitioning and CGU merging cannot isolate a local edit.

**Confidence:** High for the mechanism and fixture.

### FERRIUM-426: better CGU partitioning narrows but does not remove the gap

**Sources:** EXP-01 METIS control.

**Observed behavior:** the public library spread 2,371 mono items across 140
CGU labels. A one-function edit preserved 442 of 443 optimized MIR bodies, but
the affected CGU contained nine mono items.

**Implication:** Preserve CGUs for parallelism and object-level reuse while
measuring function-cache opportunity inside invalidated units. The two
mechanisms are complementary.

**Confidence:** High for the public control.

### FERRIUM-427: rustc must own the identity above the backend stencil

**Sources:** PERF-Q24, PERF-Q25, PERF-Q27; current cache source.

**Observed behavior:** the Cranelift key begins after rustc has selected and
lowered one mono item. It does not independently encode Rust substitutions,
layout provenance, dependency metadata, MIR lowering rules, or all
instrumentation policy.

**Implication:** rustc integration must decide when exact CLIF is semantically
valid. The backend stencil remains authoritative only after that decision.

**Confidence:** High.

### FERRIUM-428: broad semantic changes correctly destroy hit potential

**Sources:** EXP-01 optimized-MIR comparison.

**Observed behavior:** one local body edit changed one of 1,001 optimized MIR
bodies; changing the shared scalar type changed all 1,001.

**Implication:** High local-edit hit rates must not become a promise for type,
layout, ABI, target, instrumentation, or imported-body changes. The compiler
must miss broadly when semantics broaden.

**Confidence:** High for the fixture.

### FERRIUM-429: debug, unwind, symbols, and relocations are separate controls

**Sources:** PERF-Q27, PERF-Q28, PERF-Q29; Cranelift stencil/parameter split.

**Observed behavior:** a compiled function participates in object layout,
relocation, symbol, unwind, debug, linker, and debugger behavior beyond its
instruction bytes.

**Implication:** Integration tests must compare complete compiled results and
consumer capabilities, not only code bytes or successful execution.

**Confidence:** High for the boundary; unmeasured for integrated caching.

### FERRIUM-430: cross-function optimization changes the cache unit

**Sources:** PERF-Q25 and PERF-Q26; EXP-01 limitations.

**Observed behavior:** the measured direct cache compiled independent
Cranelift stencils. LLVM, LTO, imported bodies, and profile-guided decisions can
make one function's output depend on another function or whole-program state.

**Implication:** Initial research remains Cranelift, development, non-LTO, and
function-local. LLVM and release reuse require a separate identity and
economics program.

**Confidence:** High.

### FERRIUM-431: a daemon is an operational system, not a cache map

**Sources:** Rust 2026 Cranelift goal; PERF-Q18 and PERF-Q30.

**Observed behavior:** the upstream goal proposes in-memory caching to remove
disk serialization. Persistence still introduces lifecycle, protocol,
multi-user isolation, memory accounting, eviction, concurrency, crash
recovery, version skew, observability, and rollback concerns.

**Implication:** Daemon ownership belongs with rustc and
`rustc_codegen_cranelift`. FERRIUM may supply requirements and fixtures but
should not create a parallel service.

**Confidence:** High for the operational boundary.

### FERRIUM-432: PERF-Q31 authorizes upstream fixtures, not implementation

**Sources:** all evidence above and the nine-role review.

**Observed behavior:** feasibility, precision opportunity, economic risks, and
integrity requirements are clear. Rust integration, capability equality,
multi-platform behavior, long-running economics, and ownership remain
unproven.

**Implication:** Add the model to FERRIUM's read-only plans and support
upstream evaluation. Keep the implementation gate closed.

**Confidence:** High.

## Recommendations

### Adopt now

- Add function stencil, Rust semantic envelope, admission, hit, miss,
  restoration, integrity, and capability dispositions to the compiler query
  plan.
- Classify invalidated CGUs by stable and changed function bodies.
- Maintain synthetic large-module and public-repository fixtures.
- Treat Cranelift's cache key as authoritative after exact rustc lowering.
- Apply PERF-Q30 integrity and provenance rules to any persisted experiment.
- Contribute minimized correctness, corruption, and economics cases upstream.

### Prototype behind a compatibility boundary

- Direct in-memory Cranelift cache experiments in disposable fixtures.
- Upstream-owned `rustc_codegen_cranelift` integration experiments under an
  exact nightly and component identity.
- Admission-policy comparisons by function compile cost and serialized size.
- Debug, unwind, relocation, symbol, instrumentation, panic, and debugger
  equality controls.
- Long-running daemon memory, eviction, concurrency, restart, and fallback
  measurements only with the upstream owner.

### Reject or defer

- A FERRIUM function-cache daemon or external machine-code store.
- Independent source-, MIR-, object-, or symbol-derived machine-code keys.
- Automatic machine-code restoration in ordinary repositories.
- Disk persistence or transport without authenticated integrity.
- LLVM, LTO, release, remote, cross-toolchain, cross-target, or cross-platform
  function reuse.
- Treating a hit, matching code bytes, compilation, or a happy-path run as
  behavioral equivalence.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted with constraint: rustc retains semantic authority, corrupted or mismatched entries fail closed, and cache hits are not correctness proof. |
| Compiler Performance Engineer | Accepted: direct hit, population, storage, CGU, public-control, and observer-effect evidence are separated; end-to-end Rust gains remain unclaimed. |
| Interop Boundary Auditor | Accepted with constraint: ABI, target features, panic, unwind, relocations, symbols, native links, and debug capabilities remain explicit identity or validation inputs. |
| AI Assurance Skeptic | Accepted: exact evidence, limitations, corruption failures, and unmeasured integration claims remain visible. |
| Ecosystem Strategist | Accepted: FERRIUM supports the existing Cranelift and rustc owners instead of duplicating their backend or daemon. |
| Rust Maintainer | Accepted: ordinary Cargo and LLVM validation remain available, diagnostics must explain misses, and experimental integration must be removable. |
| Native Platform Adopter | Accepted with constraint: target support, multi-user isolation, memory, rollback, audit, crash recovery, and debugger behavior gate adoption. |
| Scope Keeper | Accepted: PERF-Q31 changes research vocabulary and upstream fixture authority only; product implementation remains closed. |
| Validation Checker | Accepted: the evidence records toolchain, commands, repeated timings, local and broad edits, key changes, corruption, equality checks, CGU behavior, and a public control. |

## Prototype gate

Any later integrated prototype requires:

1. ownership and review from `rustc_codegen_cranelift` maintainers;
2. exact Rust semantic and backend identity documented in code;
3. one synthetic large-module fixture and at least two held-out public repos;
4. Windows, Linux, and macOS evidence for claimed targets;
5. debug, unwind, panic, ABI, relocation, symbol, instrumentation, sanitizer,
   coverage, PGO, and debugger dispositions;
6. corruption, stale-entry, version-skew, crash, restart, eviction, memory,
   concurrency, and ordinary-rebuild recovery tests;
7. net wall, CPU, memory, storage, and latency benefit over compiler-native CGU
   reuse;
8. a small-function admission policy and observable miss reasons; and
9. a removable local-development-only adoption and rollback contract.

## Non-goals

- Designing a stable cross-version machine-code format.
- Replacing rustc incremental compilation or CGU partitioning.
- Creating another codegen backend.
- Shipping a cache daemon during the current research pulse.
- Claiming the direct Cranelift harness predicts universal Rust build speedup.
