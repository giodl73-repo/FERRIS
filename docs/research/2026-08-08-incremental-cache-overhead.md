# Incremental Cache Overhead and Reuse Economics

Date: 2026-08-08
Question: PERF-Q18
Status: Complete
Decision: adopt proof-cost, avoided-work, cache-generation, frontend-result,
backend-work-product, Cargo-freshness, and recovery-boundary vocabulary now;
prototype a read-only incremental economics view behind a nightly
compatibility boundary; defer automatic enablement, cache-format changes,
cross-machine cache transport, compiler forks, and upstream activity.

## Executive conclusion

Rust incremental compilation is neither generally free nor generally slow. It
is an investment: the current compile pays to hash results, maintain a
dependency graph, load selected prior values, promote green cache entries,
serialize the next result cache, and retain recoverable session generations.
The investment wins only when the avoided compiler work is larger.

The controlled break-even depended more on work per reusable owner than on
source size or owner count. Warm unchanged metadata compilation reached
approximate parity around 1,000 trivial owners and was about 16% faster at
5,000 and 10,000. However, one local edit to a trivial owner did not break
even at 5,000 and did not show a benefit at 10,000, where the exact ratio was
unstable. When the same 5,000 owners performed trait or borrow-checking work,
one local edit became 14.3% and 18.5% faster.

Broad edits reversed the economics. Shared type or bound changes invalidated
the reusable frontier, yet rustc still paid graph, hashing, loading, and
persistence costs. The 5,000-owner semantic controls were about 43% to 47%
slower with incremental compilation after broad edits.

Backend work products are a separate and important layer. In a 16-module,
5,000-function rlib fixture, frontend/query reuse alone made an unchanged
compile 16.4% faster. Reusing compiled codegen units increased the improvement
to 38.5%. On a one-module edit, disabling backend work-product caching left
only a 2.3% median benefit; enabling it produced a 21.3% benefit. A broad type
edit invalidated that advantage and made incremental codegen slower.

Cache size followed query topology, not source bytes. A 539 KB,
10,000-owner source created a 31.35 MiB cold cache and approximately 61.15 MiB
of unique warm state. An 80 KB one-body fixture created only 0.17 MiB
cold. The 10,000-owner dependency graph contained approximately 530,388 nodes
and 2,650,627 edges and accounted for about 84% of warm logical cache bytes.

Rustc retained two session generations after warm compilation. This supports
atomic publication and recovery, but approximately doubled unique storage in
the measured metadata fixtures. An unchanged compile still loaded the prior
graph and result cache, deserialized selected values, promoted green entries,
built the current graph, and encoded the next generation.

Cargo provides an earlier boundary. A true Cargo no-op marked the artifact
fresh and skipped rustc entirely, so incremental compiler cache cost was not
paid. FERRIUM must distinguish Cargo freshness from rustc incremental reuse:
the best compiler cache load is still more expensive than not invoking the
compiler.

Failure behavior was conservative. A failed compile did not replace the last
good generation, and restoring the valid source reused it. A damaged
dependency graph triggered full recomputation. Deliberately deleting only the
query cache while retaining a valid graph violated an internal invariant and
caused an ICE on the tested nightly; deleting the whole incremental directory
recovered. Internal cache files are one atomic recovery unit, not independent
user-managed artifacts.

The product implication is configuration and explanation, not a universal
switch. A visual build-control plane can show whether rustc was skipped,
which reuse layer paid off, how much state was loaded and rewritten, which
edit removed the reusable frontier, how much disk two generations consume,
and when a workload should be measured with incremental enabled or disabled.

## Decision supported

This research determines:

- when incremental proof and persistence costs break even;
- whether owner count, source size, or avoided work best predicts value;
- how local and broad edits change the result;
- how frontend query reuse differs from backend codegen-unit reuse;
- when Cargo freshness avoids rustc entirely;
- which cache components dominate CPU, load, serialization, and storage;
- how session generations and failed builds preserve recoverability;
- which operational and upstream opportunities are defensible.

It does not authorize automatic profile changes, internal cache manipulation,
cache-format patches, remote cache transport, a compiler daemon, or upstream
issues and pull requests.

## Evidence reviewed

### Local evidence

- [Experiment](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Frontend parallelism](2026-08-08-frontend-parallelism.md)
- [Incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [dep-graph loading](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/load.rs#L44-L118)
- [query-result-cache loading](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/load.rs#L121-L150)
- [cache promotion and persistence](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/save.rs#L20-L89)
- [memory-mapped file format](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/file_format.rs#L83-L150)
- [query cache promotion](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/dep_graph/graph.rs#L1062-L1091)
- [session finalization](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/fs.rs#L293-L340)
- [generation garbage collection](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/fs.rs#L589-L835)
- [green-node cache invariant](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_query_impl/src/plumbing.rs#L155-L178)

### Cargo and performance framework

- [Cargo incremental profile setting](https://doc.rust-lang.org/cargo/reference/profiles.html#incremental)
- [Cargo default profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles)
- [`CARGO_INCREMENTAL`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads)
- [`build.incremental`](https://doc.rust-lang.org/cargo/reference/config.html#buildincremental)
- [rustc-perf scenario model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/scenario.rs)
- [rustc-perf scenario execution](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs)
- [rustc-perf detailed query data](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/site/src/self_profile.rs)

## Findings

### FERRIUM-203: incremental compilation has a fixed proof and persistence floor

**Sources:** [trivial-owner break-even](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#trivial-owner-break-even) and [self-profile attribution](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#self-profile-attribution).

**Observed behavior:** Cold incremental metadata compilation added measurable
latency because it hashed results, constructed a dependency graph, and
persisted cache state without receiving prior reuse. Tiny warm crates also
regressed because loading and rewriting state exceeded the work avoided.

**Implication:** Incremental is an investment with a minimum viable avoided-
work threshold. Reports must include disabled, cold, and warm controls.

**Confidence:** High for the tested fixture classes.

### FERRIUM-204: avoided work per reusable owner predicts value better than owner count

**Sources:** [semantic owner scaling](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#reusable-work-per-owner-determines-value).

**Observed behavior:** A one-owner edit remained slower across 5,000 trivial
owners, but became 14.3% faster across 5,000 trait owners and 18.5% faster
across 5,000 borrow owners.

**Implication:** A cache advisor needs reusable owner cost and edit frontier,
not a crate-size or item-count heuristic.

**Confidence:** High.

### FERRIUM-205: broad edits can make incremental compilation materially slower

**Sources:** [trivial-owner matrix](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#trivial-owner-break-even) and [semantic matrix](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#reusable-work-per-owner-determines-value).

**Observed behavior:** Shared type and bound edits reran most semantic work and
still paid cache overhead. The 5,000-owner trait and borrow controls were
46.6% and 42.9% slower.

**Implication:** Incremental benefit must be conditioned on edit class.
"Warm target directory" is not sufficient evidence of likely reuse.

**Confidence:** High for the controlled broad edits.

### FERRIUM-206: backend work products are distinct from frontend query reuse

**Sources:** [backend work-product control](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#backend-work-product-reuse-is-a-separate-layer).

**Observed behavior:** Disabling backend caching preserved frontend/query
reuse but removed most of the one-module edit benefit. Enabling backend
caching improved the local median from 2.3% to 21.3% relative to disabled
compilation.

**Implication:** Reports must separate frontend result reuse, optimized MIR,
codegen-unit work products, and final link work. One "incremental hit rate"
cannot explain codegen latency.

**Confidence:** High for the 16-module rlib control.

### FERRIUM-207: Cargo freshness is cheaper than rustc incremental reuse

**Sources:** [Cargo boundary](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#cargo-freshness-boundary).

**Observed behavior:** Every true no-op Cargo run reported a fresh artifact and
zero rebuilt artifacts. Rustc did not run, so graph and query-cache costs were
not paid.

**Implication:** The build-control plane must show Cargo freshness before
analyzing compiler cache behavior. Optimizing an avoided rustc invocation is
the wrong layer.

**Confidence:** High.

### FERRIUM-208: unchanged compilation still loads and rewrites material state

**Sources:** [self-profile attribution](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#self-profile-attribution), [load source](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/load.rs#L44-L150), and [save source](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/save.rs#L20-L89).

**Observed behavior:** The profiled 10,000-owner unchanged compile spent
material time loading the graph and query cache, deserializing selected
results, promoting green entries, and encoding the next graph.

**Implication:** "All owners green" is not zero-cost. FERRIUM should expose
load, proof, promotion, encode, and persist cost separately.

**Confidence:** High for attribution; self-profile wall values are diagnostic.

### FERRIUM-209: query topology predicts cache storage better than source bytes

**Sources:** [cache topology](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#cache-topology-and-generations).

**Observed behavior:** The 10,000-owner source was 6.7 times larger than the
one-body source, but its cold cache was about 184 times larger. The dependency
graph contained about 530,000 nodes and 2.65 million edges.

**Implication:** Storage estimates should use owner, query, edge, and cacheable-
result topology rather than lines of code or source bytes.

**Confidence:** High for the synthetic topology controls.

### FERRIUM-210: two session generations approximately double warm storage

**Sources:** [generation measurement](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#cache-topology-and-generations) and [generation garbage collection](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/fs.rs#L589-L835).

**Observed behavior:** After warm compilation, rustc retained the most recent
prior finalized session and the new one. Unique 10,000-owner cache storage
grew from 31.35 MiB cold to 61.15 MiB warm.

**Implication:** Disk forecasts and cleanup diagnostics must model generation
retention, hard links, and unique bytes rather than summing directory entries
naively.

**Confidence:** High on NTFS with the tested nightly.

### FERRIUM-211: cache promotion is an explicit upstream optimization candidate

**Sources:** [persistence source](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/save.rs#L65-L85) and [query promotion implementation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/dep_graph/graph.rs#L1062-L1091).

**Observed behavior:** Rustc loads green disk-cached values that would
otherwise disappear from the newly serialized result cache. The source FIXME
asks whether values can be identified or promoted without decoding them into
memory.

**Implication:** A minimized rustc-perf fixture for unchanged, query-cache-
heavy workloads is defensible. FERRIUM should not prescribe a format or patch
before Linux upstream measurement and maintainer review.

**Confidence:** High that the work exists; low on the best implementation.

### FERRIUM-212: the incremental directory is one atomic recovery unit

**Sources:** [recovery controls](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#failure-and-recovery-controls) and [green-node invariant](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_query_impl/src/plumbing.rs#L155-L178).

**Observed behavior:** Truncating the dependency graph caused safe
recomputation. Removing only the query cache while preserving a graph that
marked values green violated the paired-state invariant and caused an ICE.
Deleting the whole cache recovered.

**Implication:** Tools must never prune, copy, repair, or transport individual
internal files. Supported recovery removes the whole isolated incremental
cache.

**Confidence:** High for deliberate corruption on the tested revision.

### FERRIUM-213: failed compilations preserve the last good generation

**Sources:** [failed-edit control](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#failed-compilation), [save guard](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/save.rs#L20-L35), and [successful finalization](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/fs.rs#L293-L340).

**Observed behavior:** A type-check failure did not publish a replacement.
Restoring valid source reused the prior cache with no body query misses.

**Implication:** Recovery and visualization should distinguish working,
failed, and finalized generations rather than treating the cache as one
mutable snapshot.

**Confidence:** High.

### FERRIUM-214: incremental mode is part of artifact identity

**Sources:** [artifact identity control](perf-q18-incremental-cache-overhead/results/EXP-01-proof-persistence-break-even.md#artifact-identity-control) and [Cargo profile setting](https://doc.rust-lang.org/cargo/reference/profiles.html#incremental).

**Observed behavior:** Incremental and disabled metadata bytes differed for
equivalent source, while hashes remained stable within each mode.

**Implication:** Cross-mode output hashes are not a correctness oracle or
shared artifact key. Configuration, flags, compiler, target, and mode remain
part of provenance.

**Confidence:** High.

### FERRIUM-215: rustc-perf covers scenarios better than cache-cost attribution

**Sources:** [rustc-perf scenario execution](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs) and [detailed query data](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/site/src/self_profile.rs).

**Observed behavior:** Rustc-perf correctly measures full, cold incremental,
unchanged, and patched scenarios and retains cache hits, misses, and
incremental load time. At the pinned revision, the reviewed detailed-query
schema did not retain stable-result hashing time as a first-class metric.

**Implication:** Upstream regression work belongs in rustc-perf, while a
proposal to expose hashing or persistence attribution needs a separate,
maintainer-reviewed telemetry case.

**Confidence:** Medium because this was source review, not a collector run.

### FERRIUM-216: workload-aware cache economics belongs in the visual control plane

**Sources:** findings FERRIUM-203 through FERRIUM-215, [query invalidation](2026-08-08-query-dependency-precision.md), and [frontend sessions](2026-08-08-frontend-parallelism.md).

**Observed behavior:** Incremental value depended on Cargo freshness, edit
class, reusable owner cost, frontend and backend reuse, graph topology,
session generation, storage, and concurrent machine load.

**Implication:** FERRIUM can prototype a read-only view that explains:

- whether Cargo skipped rustc;
- cold, unchanged, local, broad, and failed states;
- frontend query and backend work-product reuse;
- avoided work versus proof, load, promotion, encode, and persist cost;
- graph and query topology;
- logical versus unique cache bytes and generation count;
- supported whole-cache recovery;
- measured profile guidance without automatic toggling.

**Confidence:** High for the explanation need; medium for product viability.

## Recommendations

### Adopt now

- Record incremental mode, Cargo freshness, cache state, edit class, owner
  frontier, output mode, and backend-work-product policy in every benchmark.
- Compare disabled, cold incremental, warm unchanged, local-edit, and broad-
  edit states using the same fixture and command shape.
- Estimate cache economics from avoided work per owner, not source size alone.
- Record logical and unique bytes, session generation count, component sizes,
  and hard-link identity separately.
- Treat a failed compile as a non-published working session and preserve the
  prior finalized generation.
- Recover from suspected incremental-cache damage by removing the whole
  isolated cache directory.
- Preserve Cargo's current development and release defaults unless a
  repository-specific repeated measurement justifies an override.
- Treat one-shot CI jobs with no reusable incremental state as candidates for
  measurement, not automatic global disablement.

### Prototype behind a compatibility boundary

- A read-only incremental economics panel joining Cargo freshness, rustc
  invocation, edit frontier, provider reuse, cache activities, codegen-unit
  reuse, generations, and storage.
- A profile comparison that estimates saved work versus hashing, loading,
  promotion, encoding, persistence, and backend object cost.
- A cache topology view showing graph nodes, edges, cacheable results,
  component bytes, hard links, and unique bytes.
- A recovery diagnostic that detects unsupported partial cache state and
  recommends whole-directory removal without reading internal values.
- Orthogonal rustc-perf candidates for query-cache-heavy unchanged builds,
  expensive local semantic reuse, broad invalidation, and backend work-product
  reuse.
- A maintainer-reviewed telemetry proposal for stable-result hashing and
  persistence attribution if rustc-perf cannot already express the needed
  regression.

### Reject or defer

- Globally enabling or disabling incremental compilation from these fixtures.
- Editing Cargo profiles automatically.
- Treating source bytes, crate size, or item count as a benefit predictor.
- Pruning or transporting individual `dep-graph.bin`, `query-cache.bin`,
  metadata, or work-product files.
- Depending on the incremental file format as a stable API.
- Whole-file compression, differential persistence, content-addressed
  generations, or cache-promotion patches without upstream prototypes and
  rustc-perf evidence.
- Sharing writable rustc incremental directories across unrelated builds.
- Remote incremental cache transport; provenance and trust remain PERF-Q30.
- Compiler forks, daemons, shared semantic caches, or upstream issues and pull
  requests in this question.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: cache optimization never weakens semantic, diagnostic, metadata, or backend correctness; damaged internal state is discarded as a unit. |
| Compiler Performance Engineer | Accepted: disabled, cold, unchanged, local, broad, frontend, backend, storage, variance, and observer-effect boundaries are separated. |
| Interop Boundary Auditor | Accepted: no ABI, linker, FFI, remote-artifact, or cross-language conclusion is inferred from metadata and rlib controls. |
| AI Assurance Skeptic | Accepted: unstable distributions, deliberate corruption, cross-mode hash differences, and synthetic-fixture limits remain explicit. |
| Ecosystem Strategist | Accepted: Cargo defaults and rustc-perf remain authoritative; FERRIUM supplies explanation, workload classification, and minimized candidates. |
| Rust Maintainer | Accepted: no internal file ritual, automatic source or profile change, compiler fork, or upstream filing is proposed. |
| Native Platform Adopter | Accepted: disk growth, generation retention, failed builds, CI reuse, whole-cache recovery, and reversible configuration are represented. |
| Scope Keeper | Accepted: the work answers cache overhead and leaves early-phase incrementality, cross-crate RDR, remote reuse, codegen partitioning, and linking to later questions. |
| Validation Checker | Accepted: commands, revisions, repetitions, medians, MAD, CPU, memory, storage, freshness, hashes, failures, recovery, and negative conclusions are recorded. |

## Non-goals

- Predicting every Rust crate from synthetic fixtures.
- Claiming incremental compilation should always be enabled or disabled.
- Treating self-profile wall time as the primary benchmark.
- Defining a stable rustc incremental cache format.
- Designing remote incremental cache transport.
- Modifying Cargo defaults or project manifests.
- Repairing individual cache files.
- Publishing raw profiles, temporary cache directories, or private paths.
- Creating an upstream issue, branch, comment, or pull request.
