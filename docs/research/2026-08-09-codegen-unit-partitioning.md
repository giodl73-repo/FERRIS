# Codegen-Unit Partitioning

Date: 2026-08-09
Question: PERF-Q25
Status: Complete
Decision: adopt a read-only codegen-unit ledger and Build Forest overlay that
records requested and actual CGUs, initial stable and volatile partitions,
merge lineage, item and inline-copy placement, size estimates, backend
work-product reuse, local and whole-graph LTO, memory, link cost, output size,
runtime controls, and partition stability; prototype repository-specific
what-if comparison behind an exact toolchain boundary; contribute minimized
upstream diagnostics and fixtures; defer automatic profile changes,
partitioning heuristics, compiler forks, and implementation.

## Executive conclusion

A codegen unit is simultaneously:

- an LLVM optimization module;
- a scheduling unit for backend parallelism;
- an incremental backend work product;
- an inlining and internalization boundary;
- an input to local or whole-graph LTO; and
- an object presented to the linker.

Changing the count moves all of those boundaries at once. It cannot be reduced
to “more is faster” or “one is optimized.”

The current compiler first groups mono items by source module. Incremental
compilation splits generic instances into volatile partitions, while
non-incremental compilation does not. Reachable `#[inline]` bodies are copied
into consuming CGUs. The compiler then merges partitions down to a requested
maximum, using inline-item overlap to reduce duplication. Non-incremental
defaults additionally merge estimated units smaller than 1,800.

The requested count is therefore only a maximum. In the PERF-Q25 fixture, the
non-incremental development default of 16 emitted four CGUs; the release
default emitted five; explicit 64 emitted 63; and the incremental default of
256 emitted 68.

Fine granularity worked exactly as intended for one local edit. A four-CGU
build reused three work products, a 16-CGU build reused 15, and a 64-CGU build
reused 63. Unrelated item placements remained stable.

That same granularity carried overhead. The incremental default reused 67 of
68 work products after the local edit but took 1,430.9 ms median, versus
652.1 ms with four CGUs. An unchanged default build reused all 68 work products
but took 1,438.5 ms, versus 584.2 ms with one. Work-product count, proof,
copying, codegen coordination, and linking can outweigh a higher hit rate.

Merged units introduced a different failure mode. Adding generic references
or one module changed every unrelated placement at four and eight CGUs and
about 89% at 16 and 32. Generic growth produced no reusable work products from
four through 32 CGUs. At 64 CGUs, many source partitions remained unmerged and
reuse recovered.

This is the key opportunity: a maintainer cannot see that a one-line edit
changed the partition merge plan and invalidated otherwise unrelated backend
work. Cargo timing and aggregate rustc timing do not expose that causal step.

Cold compile time was not monotonic. Multiple release CGUs shortened the
measured wall path by spending more aggregate CPU. Development rows moved
irregularly, and some were bimodal. Thirty-two release CGUs with local ThinLTO
disabled used 28.2% more peak RSS than one. The correct count therefore depends
on workload, available parallelism, memory budget, linker, and edit topology.

One CGU did not win the runtime control. All ten release configurations had
overlapping distributions and only a 1.7% executable-size span. A newer
upstream report also demonstrates that one CGU plus LTO can over-inline and
regress some runtime workloads. FERRIUM must preserve runtime and size controls
instead of describing one CGU as maximum performance.

Automatic local ThinLTO and explicit ThinLTO are different modes. Explicit
16-CGU ThinLTO took about three times the automatic 16-CGU median in the
fixture and had high
variance, without a measured runtime advantage. PERF-Q26 must study LLVM pass
cost; PERF-Q29 must study linking. PERF-Q25 records their boundaries but does
not optimize either.

Upstream history reinforces the caution. Greedy CGU balancing, shim placement,
and inline-copy deduplication have each improved plausible local metrics while
regressing other compiler workloads or helping only narrow cases. The open
scheduling issue identifies inaccurate size estimation as a prerequisite.

FERRIUM should not choose a universal Cargo profile. It should explain the
current partition and let maintainers compare measured alternatives:

- which initial partitions existed;
- which units were merged and why;
- which inline bodies were duplicated;
- which units were reused or regenerated;
- which edit changed the merge identity;
- how much CPU, memory, link, size, and runtime changed.

The measured platform is x86_64-pc-windows-msvc. Nightly mono-item and CGU-name
diagnostics are optional, unstable, and version-gated. Ordinary Cargo and rustc
behavior remain authoritative; disabling the diagnostic is the rollback.

## Decision supported

This research determines:

- which CGU identities belong in the compiler query plan;
- how requested maximum, initial partitions, merging, and actual CGUs differ;
- how local body edits and partition-shape edits affect backend reuse;
- how inline copies, internalization, local ThinLTO, and explicit ThinLTO cross
  the partition boundary;
- why hit rate alone cannot choose a CGU count;
- why profile guidance must be repository-specific and retain runtime, size,
  memory, and link controls;
- which upstream diagnostics and fixtures are defensible.

It does not authorize automatic `codegen-units`, incremental, LTO, inlining,
profile, source-module, crate, linker, cache, or compiler-fork changes.

## Evidence reviewed

### Local evidence

- [Codegen-unit partition and reuse matrix](perf-q25-codegen-units/results/EXP-01-codegen-unit-partition-matrix.md)
- [Monomorphization and generic-instance reuse](2026-08-09-monomorphization-generic-instance-reuse.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Rust incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources and documentation

- [Current CGU partitioning](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_monomorphize/src/partitioning.rs)
- [Current CGU defaults](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/session.rs)
- [Current CGU output override](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/config.rs)
- [rustc codegen-unit documentation](https://doc.rust-lang.org/rustc/codegen-options/index.html#codegen-units)
- [Cargo profile documentation](https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units)
- [Partitioning test contract](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/codegen-units/partitioning/README.md)
- [Regular module placement test](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/codegen-units/partitioning/regular-modules.rs)
- [Incremental merge-name test](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/codegen-units/partitioning/incremental-merging.rs)
- [`#[inline(always)]` placement test](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/codegen-units/partitioning/inline-always.rs)
- [Independent incremental ThinLTO CGUs](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/incremental/thinlto/independent_cgus_dont_affect_each_other.rs)
- [ThinLTO import invalidation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/incremental/thinlto/cgu_invalidated_via_import.rs)
- [Inaccurate post-ThinLTO reuse reporting](https://github.com/rust-lang/rust/issues/119076)
- [Open CGU scheduling roadmap](https://github.com/rust-lang/rust/issues/89281)
- [ThinLTO quality gap](https://github.com/rust-lang/rust/issues/47745)
- [One-CGU LTO runtime regression](https://github.com/rust-lang/rust/issues/148670)
- [Greedy partitioning experiment](https://github.com/rust-lang/rust/pull/112766)
- [Shim-placement experiment](https://github.com/rust-lang/rust/pull/141560)
- [Inline-copy deduplication experiment](https://github.com/rust-lang/rust/pull/154345)
- [MIR-based CGU size estimate](https://github.com/rust-lang/rust/pull/47415)

## Current compiler model

### Initial placement follows module and volatility

Non-generic roots are assigned to the innermost source module. Generic
functions use a volatile companion partition only when incremental compilation
is enabled. Items without a characteristic module enter a fallback partition.

Methods prefer the module of their characteristic self type. This connects
PERF-Q24 generic ownership to PERF-Q25 partition ownership.

### Inline availability creates multiple placements

The partitioner recursively collects reachable local-copy items. A
`#[inline]` or required glue body can therefore exist in every CGU that needs
it. That duplication improves local optimization but increases LLVM work and
means one body change can invalidate several units.

The merge algorithm counts overlap in duplicated inline items and prefers
merges that remove duplicate copies. Inline overlap is therefore both a cost
and a current optimization signal.

### Merge identity is incremental state

Incremental CGU names encode the sorted names of the initial partitions merged
into them. This makes unchanged compositions stable, but a new or resized
partition can change which units merge and therefore change work-product
identity for unrelated items.

Non-incremental names are numeric and sorted by estimated size. They are useful
for backend scheduling but not stable source ownership identifiers.

### Actual count is a compiler decision

The configured count is a maximum. The compiler cannot emit more CGUs than
initial partitions, and non-incremental defaults continue merging tiny units.
Reports must preserve requested maximum, default versus explicit origin,
initial count, final count, and merge reason.

### LTO adds another reuse boundary

Local ThinLTO optimizes across CGUs in one crate. Explicit ThinLTO operates
across a larger crate graph. Imports can cause a consumer CGU's post-LTO result
to depend on another CGU even when its pre-LTO body did not change.

Current incremental tests distinguish no reuse, pre-LTO reuse, and post-LTO
reuse, but issue `#119076` records inaccurate reporting after a backend change.
FERRIUM must not infer post-LTO reuse from one unstable diagnostic.

## Findings

### FERRIUM-318: a CGU is an optimization, parallelism, and reuse boundary

**Sources:** current partitioning source, partitioning test README, and EXP-01.

**Observed behavior:** one item assignment controlled LLVM module membership,
backend scheduling, incremental work-product identity, inline availability,
internalization, and link input.

**Implication:** A CGU report must preserve all of those roles. “Backend time”
and “cache hit” are insufficient abstractions.

**Confidence:** High.

### FERRIUM-319: requested CGUs are a maximum, not the emitted count

**Sources:** current defaults, minimum-size merge source, and EXP-01.

**Observed behavior:** defaults of 16 and 256 emitted 4, 5, and 68 CGUs in
different modes; explicit 64 emitted 63 non-incremental CGUs.

**Implication:** Record requested, default or explicit origin, initial
partitions, actual CGUs, and merge reasons separately.

**Confidence:** High.

### FERRIUM-320: incremental and non-incremental builds partition different graphs

**Sources:** `place_mono_items`, current defaults, and EXP-01.

**Observed behavior:** generic instances received volatile companion
partitions only with incremental compilation. Non-incremental default merging
also applied a minimum estimated size that incremental merging did not.

**Implication:** A CGU count from one cache mode cannot predict the other.
Incremental state is part of partition identity.

**Confidence:** High.

### FERRIUM-321: inline-copy cost grows with surviving CGUs

**Sources:** inline placement test, overlap merge source, and EXP-01.

**Observed behavior:** the shared inline helper occupied one CGU at count one,
16 at count 16, and 33 initial consuming partitions at high counts. Development
placement count grew 6.8% and saved object bytes 5.8% from one to 64.

**Implication:** Report duplicated item identities, placement multiplicity,
estimated size, and consuming units rather than only the number of duplicated
functions.

**Confidence:** High for the mechanism and fixture.

### FERRIUM-322: overlap-aware merging trades duplication for identity coupling

**Sources:** current merge algorithm and EXP-01.

**Observed behavior:** merging uses inline overlap to reduce duplicate copies,
but merged incremental names include every consumed initial partition.

**Implication:** Merge quality needs at least duplicate bytes, estimated and
observed backend cost, work-product stability, and final output controls.

**Confidence:** High.

### FERRIUM-323: local body edits can achieve precise backend reuse

**Sources:** EXP-01 local edit and PERF-Q18 backend cache control.

**Observed behavior:** unrelated placements were unchanged. Explicit 4, 8, 16,
32, and 64 CGUs regenerated exactly one work product.

**Implication:** Preserve source-module and CGU lineage in the compiler query
plan. Local edit precision is a real capability, not only a theoretical goal.

**Confidence:** High.

### FERRIUM-324: generic references can invalidate unrelated merged units

**Sources:** partitioning source invalidation rationale and EXP-01 generic edit.

**Observed behavior:** adding 64 generic references changed every unrelated
placement at 4 and 8 CGUs and 89.2% at 16 and 32. No prior work product was
reused from 4 through 32.

**Implication:** A generic edit explanation must show both new mono items and
the resulting merge-plan change. Mono-item delta alone misses the collateral
backend invalidation.

**Confidence:** High for the fixture; medium for prevalence.

### FERRIUM-325: adding a module can shift most merged work-product identities

**Sources:** EXP-01 module-add edit.

**Observed behavior:** one new module changed 100% of unrelated placements at
4 and 8 CGUs and 89.6% at 16 and 32. At 64, where most source partitions
survived unmerged, 57 of 64 prior work products remained reusable.

**Implication:** Module topology is a partition input. Automatic module
splitting or consolidation cannot be a generic latency recommendation.

**Confidence:** High for the fixture.

### FERRIUM-326: work-product hit rate does not determine incremental latency

**Sources:** EXP-01 unchanged and local matrices plus PERF-Q18.

**Observed behavior:** the default local edit reused 67 of 68 work products
but took 2.19 times the four-CGU median. The unchanged default reused all 68
but took 2.46 times the one-CGU median.

**Implication:** A planner must include proof, load, copy, coordination, LLVM,
link, and storage costs beside reuse count.

**Confidence:** High for the fixture.

### FERRIUM-327: more CGUs do not produce monotonic cold speedups

**Sources:** EXP-01 cold matrix and current default rationale.

**Observed behavior:** CPU consumption generally rose with more CGUs while
development wall time moved irregularly and some rows were bimodal. Release
16 and 32 CGUs shortened wall time relative to the unstable one-CGU row.

**Implication:** Recommend measurement ranges, not a universal count.
Available cores do not equal useful partition count.

**Confidence:** High that the relationship is non-monotonic; low on exact
portable optima.

### FERRIUM-328: CGU parallelism has a memory budget

**Sources:** EXP-01 peak RSS and issue `#82685`.

**Observed behavior:** with local ThinLTO disabled, 32 CGUs used 28.2% more
peak RSS than one. Explicit ThinLTO also increased peak RSS.

**Implication:** Profile comparison must record memory and concurrent crate
pressure, not only one rustc process's wall time.

**Confidence:** Medium because RSS was sampled on one platform.

### FERRIUM-329: one CGU is not a runtime guarantee

**Sources:** EXP-01 runtime control, issue `#47745`, and issue `#148670`.

**Observed behavior:** all measured runtime distributions overlapped. Upstream
has both quality-gap reports for multiple CGUs and a recent over-inlining
regression for one CGU plus LTO.

**Implication:** “Maximum performance” requires application benchmarks.
FERRIUM must not equate fewer CGUs with faster code.

**Confidence:** High that direction is workload-specific.

### FERRIUM-330: local and whole-graph ThinLTO are different workloads

**Sources:** rustc LTO model, EXP-01, and ThinLTO incremental tests.

**Observed behavior:** explicit 16-CGU ThinLTO took about three times the local
ThinLTO median with high variance and no measured runtime gain.

**Implication:** Preserve LTO scope, import topology, bitcode mode, and
pre/post-LTO reuse separately. PERF-Q26 owns pass cost; PERF-Q29 owns link cost.

**Confidence:** High for the distinction; medium for the fixture ratio.

### FERRIUM-331: current size estimates are not reliable scheduling cost

**Sources:** issue `#89281`, PR `#47415`, PR `#112766`, and PR `#141560`.

**Observed behavior:** more even estimated partitions regressed compiler
instructions, and characteristic shim placement produced broad mixed results.
Maintainer analysis attributes some failures to inaccurate estimates and long
LLVM poles.

**Implication:** A what-if planner must learn from observed per-CGU backend
duration and preserve estimate error. It must not optimize only MIR size.

**Confidence:** High.

### FERRIUM-332: inline deduplication is promising but not general

**Sources:** PR `#154345` and EXP-01 duplication evidence.

**Observed behavior:** the prototype improved a synthetic stress test and
`syn`, but most crates did not change, so the PR was closed for more research.

**Implication:** Keep large repeated inline items as diagnostics and upstream
fixtures. Do not introduce a FERRIUM partitioner or hidden linkage changes.

**Confidence:** High.

### FERRIUM-333: current CGU diagnostics need a versioned adapter

**Sources:** rustc output override, `-Zprint-mono-items`, issue `#119076`, and
EXP-01 pilot correction.

**Observed behavior:** output selection can force one CGU, human-readable names
are unstable, and post-ThinLTO reuse reporting is known inaccurate.

**Implication:** Bind evidence to exact rustc revision, output mode, flags, and
schema. Stable Cargo and ordinary compilation remain the fallback.

**Confidence:** High.

### FERRIUM-334: the immediate opportunity is a read-only CGU ledger

**Sources:** findings 318 through 333 and all role reviews.

**Observed behavior:** no existing stable surface joins requested count,
initial partitions, merge lineage, duplicated inline items, actual CGUs,
work-product reuse, memory, LTO, link, size, and runtime.

**Implication:** Add this topology to the compiler query plan and labeled Build
Forest. Prototype only after representative public-repository evaluation.

**Confidence:** High for the observability gap; medium for product adoption.

### FERRIUM-335: automatic profile guidance remains gated

**Sources:** measurement contract, upstream mixed results, and role review.

**Observed behavior:** one synthetic fixture produced different best rows for
unchanged, local, topology-changing, cold release, memory, and runtime goals.

**Implication:** Do not rewrite Cargo profiles. Require held-out edits, at
least three public repositories, cross-platform controls, rollback, and
maintainer approval before any advisory profile recommendation.

**Confidence:** High.

## Recommendations

### Adopt now

- Add requested maximum, actual CGU count, incremental state, and LTO scope to
  every backend measurement.
- Add initial partition, stable or volatile class, merge lineage, item
  placements, inline multiplicity, linkage, and work-product disposition to
  the compiler query-plan vocabulary.
- Show one-function, generic-reference, module-topology, unchanged, and cold
  controls separately.
- Preserve CPU, peak RSS, object, executable, runtime, and link evidence.
- Contribute minimized fixtures and diagnostic gaps upstream when a public
  repository reproduces a costly case.

### Prototype behind a compatibility boundary

- An exact-nightly adapter for mono-item placement and human-readable CGU names.
- A read-only CGU ledger and Build Forest overlay.
- A measured what-if comparison that compiles supported alternative profiles
  in isolated targets and reports tradeoffs without modifying manifests.
- Per-CGU observed duration capture if a stable or exact-version diagnostic can
  be established.

### Reject or defer

- A universal development or release CGU count.
- Automatic Cargo profile rewriting.
- Automatic module or crate splitting to improve partitions.
- A FERRIUM partitioning algorithm, compiler fork, custom backend, or linker.
- Hidden inlining, linkage, internalization, or LTO changes.
- Function-level machine-code reuse before PERF-Q31 and provenance before
  PERF-Q30.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because the decision changes no unsafe, ABI, ownership, panic, or runtime semantics; any future profile advice must preserve behavioral tests and cannot describe compiler success as safety proof. |
| Compiler Performance Engineer | Accepted with the requirement to keep cold, unchanged, local, topology-changing, CPU, RSS, link, size, and runtime evidence separate; the synthetic matrix cannot select a general default. |
| Interop Boundary Auditor | Accepted provisionally because no FFI or ABI is changed; mixed native linkers, C/C++ objects, exceptions, allocation, and target ABI remain required controls before adopter guidance. |
| AI Assurance Skeptic | Accepted because raw variance, unstable rows, negative runtime results, diagnostic pitfalls, and upstream regressions remain visible; no model-selected profile is applied automatically. |
| Ecosystem Strategist | Accepted because the capability explains existing Cargo and rustc behavior and prioritizes upstream fixtures over replacement tooling; compiler-fork and custom-partitioner paths remain rejected. |
| Rust Maintainer | Accepted because the output is a removable diagnostic that preserves ordinary Cargo and editor use; recommendations require actionable causal evidence and do not create source or manifest churn. |
| Native Platform Adopter | Accepted provisionally for local read-only use; cross-platform linker, memory, CI concurrency, support, rollback, and operational-cost evidence are still required. |
| Scope Keeper | Accepted because PERF-Q25 remains bounded to partition formation and reuse; LLVM pass choice, backends, debug emission, linking, remote reuse, and function caching remain in their named questions. |
| Validation Checker | Accepted because commands, toolchain, fixture shape, edits, repetitions, MAD, runtime checksums, negative results, limitations, and upstream sources are recorded; Tier 1 and cross-platform validation remain open gates. |

No role authorizes implementation or automatic profile changes.

## Product implication

The compiler query plan should add a backend partition region:

```text
mono items
  -> initial stable / volatile / fallback partitions
  -> inline local-copy expansion
  -> estimated size and overlap
  -> merge lineage
  -> actual CGUs
  -> pre-LTO work-product decision
  -> local or whole-graph LTO imports
  -> post-LTO work-product decision
  -> object and link inputs
  -> final size and runtime controls
```

The Build Forest may label the same edit under alternative measured profiles,
but each root remains immutable and records toolchain, target, profile,
incremental generation, LTO, linker, environment, and validation evidence.

## Prototype gate

No CGU implementation or profile advisor may begin until:

1. at least three Tier 1 repositories reproduce the ledger end to end;
2. held-out local, generic, inline, module, public-interface, and test edits are
   evaluated;
3. Linux and Windows evidence exists, with macOS or a documented deferral;
4. runtime, binary size, memory, and link controls are consumer-representative;
5. diagnostic disablement and manifest rollback are explicit;
6. the exact-nightly adapter fails closed on schema drift;
7. an upstream maintainer reviews any proposed compiler-facing fixture;
8. PERF-Q30 and PERF-Q31 remain respected for artifact and function reuse.

The implementation gate remains closed.
