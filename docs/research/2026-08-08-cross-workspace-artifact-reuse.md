# Cross-Workspace Cargo Artifact Reuse

Date: 2026-08-08
Question: PERF-Q05
Status: Complete
Decision: retain cross-workspace reuse eligibility and provenance diagnosis as
a FERRIUM opportunity, but defer a cache implementation to Cargo's active
upstream design.

## Executive conclusion

Cargo can reuse an ordinary immutable registry dependency across unrelated
workspaces today when both workspaces write into one target directory and the
complete build identity matches. A controlled fixture reused
`itoa 1.0.15` while compiling only the second workspace root. Version and
rustflag changes produced intentional misses, and returning to the original
flags recovered the earlier artifact.

That mechanism is not a safe general cache.

In a second controlled fixture, two unrelated workspaces had path packages
with equal names, versions, relative locations, file sizes, and modification
times but different source behavior. After building workspace A, Cargo marked
both workspace B artifacts fresh and successfully ran A's binary. Cleaning and
rebuilding B changed the output to B's expected value. The experiment
reproduced Cargo issue
[#12516](https://github.com/rust-lang/cargo/issues/12516) as successful wrong
artifact reuse.

Copying compiler output files alone did not create reuse; Cargo required its
fingerprints and associated internal state. Deleting an output caused a safe
rebuild, but replacing a metadata artifact with corrupt bytes was not detected
by Cargo freshness. The dependency was reported fresh and rustc rejected it
only when compiling the consumer.

Upstream Cargo's current cross-workspace-cache roadmap has converged on the
same first boundary:

- immutable registry and Git packages, not path packages;
- idempotent units, initially excluding build-script runs and their dependents
  and proc-macro dependents;
- package-and-hash-scoped build directories;
- granular locking;
- garbage collection;
- explicit poisoning recovery.

The prerequisite build-directory layout was stabilized and then reverted after
regressions. As of 2026-08-08 it remains an open tracking issue.

FERRIUM should not create a competing artifact store or recommend a shared
writable target directory. Its defensible opportunity is a read-only reuse
eligibility and provenance ledger: identify candidate immutable units, explain
identity mismatches, estimate actual overlap, identify excluded build-script
and proc-macro cones, and produce minimized evidence for Cargo's upstream
experiments.

No issue, comment, branch, or pull request was created during this research.

## Decision supported

This research determines:

- which local Cargo artifact class is the safest first reuse candidate;
- why a shared writable target directory is not that cache;
- which provenance, integrity, isolation, locking, and cleanup boundaries are
  required;
- whether FERRIUM should build an adapter, contribute evidence, or defer;
- which evidence PERF-Q06 and PERF-Q30 inherit.

It does not authorize a cache service, target-directory rewrite, remote
transport, source substitution, or upstream filing.

## Research question

Which ordinary Rust artifacts can be safely reused across workspaces, and what
prevents that reuse today?

## Starting and competing hypotheses

The starting hypothesis was that ordinary non-workspace crates were the safest
first reuse level once Cargo had a stable build-unit identity and cleanup
model.

The investigation tested these competing explanations:

1. A shared target directory is already a sufficient user-wide cache.
2. Compiler output files are self-contained cache entries.
3. Cargo freshness also verifies artifact integrity.
4. Exact dependency overlap is common enough to guarantee large local wins.
5. FERRIUM should build an artifact adapter before Cargo's upstream design
   stabilizes.

The evidence rejected all five as general rules while preserving immutable
registry units as the correct first eligibility class.

## Evidence reviewed

### Local evidence

- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- `docs/research/2026-08-08-cargo-build-unit-multiplication.md`
- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

### Cargo documentation and upstream state

- [Cargo build cache](https://doc.rust-lang.org/cargo/reference/build-cache.html)
- [Cross-workspace build cache tracking issue](https://github.com/rust-lang/cargo/issues/5931)
- [Shared target path-package collision](https://github.com/rust-lang/cargo/issues/12516)
- [Build-directory package-and-hash layout](https://github.com/rust-lang/cargo/issues/15010)
- [Build-directory layout call for testing](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/)
- [Build-directory cleanup after stabilization](https://github.com/rust-lang/cargo/issues/17182)
- [Granular Cargo locking](https://github.com/rust-lang/cargo/issues/4282)
- [Target-directory garbage collection](https://github.com/rust-lang/cargo/issues/5026)
- [sccache Rust support and limitations](https://github.com/mozilla/sccache/blob/main/docs/Rust.md)

Issue state and maintainer comments were reviewed on 2026-08-08. Cargo issue
#5931 currently defines the minimum viable cache as immutable registry or Git
items plus idempotent units, excluding path packages, build-script execution
and dependents, and proc-macro dependents.

## Reuse eligibility model

A cross-workspace candidate is not just an artifact filename. The consumer
must establish:

| Boundary | Required evidence |
| --- | --- |
| Source provenance | Immutable registry checksum or immutable Git revision |
| Cargo unit identity | Package, target, mode, effective profile, platform, features, role, dependency identities |
| Compiler identity | rustc version, wrapper, target specification, relevant flags and configuration |
| Execution purity | No excluded build-script or proc-macro execution cone |
| Artifact set | All outputs and Cargo metadata required for the unit |
| Integrity | Entry manifest and cryptographic digest verified before use |
| Isolation | Imported read-only or atomically installed without unrelated writable path packages |
| Concurrency | Per-entry publication and read/prune locking |
| Retention | Owned access tracking, garbage collection, and bounded cleanup |
| Recovery | Targeted invalidation and a documented poisoned-entry escape hatch |

Matching only package name and version is insufficient. Matching only Cargo's
artifact filename is also insufficient because freshness inputs such as
rustflags can differ from symbol metadata and because Cargo's internal layout
is still changing.

## Findings

### FERRIUM-68: Cargo's upstream cache begins with immutable, idempotent units

**Sources**

- [Cargo issue #5931](https://github.com/rust-lang/cargo/issues/5931)
- [Build-directory layout rationale](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/)

**Observed constraint**

Cargo's current roadmap limits the first cache to immutable registry and Git
packages. It excludes path packages. It also excludes build-script executions
and their dependent units and proc-macro dependents because their undeclared
inputs can make apparently equal units non-idempotent.

The planned storage work first reorganizes intermediate artifacts into
self-contained package-and-hash directories, then adds locking, experiments,
and garbage collection.

**Implication**

FERRIUM's first eligibility report should mirror this boundary rather than
invent a broader cacheability claim. Build scripts and proc macros remain
separate research questions PERF-Q23 and PERF-Q22.

**Confidence:** high.

### FERRIUM-69: an exact immutable registry unit can be reused across unrelated
workspaces

**Source**

- Experiment:
  `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

**Observed behavior**

Workspace A and workspace B were unrelated local packages that both depended
on `itoa 1.0.15`. After A compiled into a shared disposable target, B's Cargo
JSON reported `itoa` fresh and compiled only B's root.

Changing the dependency to `itoa 1.0.14` compiled a distinct unit. Applying
`RUSTFLAGS=-Cdebuginfo=1` compiled new filenames for both the dependency and
root. Removing the flag made the original artifacts fresh again.

**Implication**

The reuse opportunity is real but narrower than package overlap. Eligibility
requires an equal effective unit and compiler identity. Intentional misses must
remain visible rather than being treated as cache failure.

**Confidence:** high for the controlled fixture.

### FERRIUM-70: compiler outputs alone are not Cargo cache entries

**Source**

- Experiment:
  `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

**Observed behavior**

Copying only the `itoa` compiler outputs into an otherwise empty target did not
produce reuse. Cargo rebuilt the dependency. Copying the complete target
snapshot from workspace A allowed B to reuse `itoa`, because the snapshot also
included Cargo fingerprints and related state.

**Implication**

A FERRIUM adapter must not scrape and copy `rlib` or `rmeta` files. The
cacheable unit needs an explicit Cargo-owned artifact manifest or a stabilized
self-contained directory contract. Whole-target snapshots are too broad.

**Confidence:** high.

### FERRIUM-71: freshness and artifact integrity are separate checks

**Source**

- Experiment:
  `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

**Observed behavior**

Deleting the cached dependency output while retaining fingerprints caused
Cargo to rebuild it. Replacing the dependency metadata output with corrupt
bytes did not make Cargo's freshness check dirty. Cargo reported the
dependency fresh; rustc then failed the consumer with invalid metadata.

**Implication**

An import boundary needs cryptographic integrity verification before an entry
becomes visible. Cargo freshness is a rebuild decision over expected local
state, not a trust or corruption-verification protocol.

**Confidence:** high for the tested metadata artifact; behavior of other
corruption classes remains untested.

### FERRIUM-72: unrelated path packages can produce successful wrong reuse

**Sources**

- [Cargo issue #12516](https://github.com/rust-lang/cargo/issues/12516)
- Experiment:
  `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

**Observed behavior**

Two unrelated workspaces used equal root and dependency names, versions,
relative paths, file sizes, and controlled modification times. Their dependency
behavior differed.

After building A, Cargo reported both B units fresh. Running the result from
B printed A's value. After cleaning and rebuilding B, the result printed B's
value.

Cargo maintainers preserve relative path identity to support relocation. A
2026 maintainer comment also states that sharing target directories across
workspaces is not endorsed because of entry conflicts, lock contention, and
cache poisoning.

**Implication**

One shared writable target directory across unrelated repositories is
prohibited as a FERRIUM recommendation. Path packages require workspace-scoped
freshness or a future content/provenance design that does not collapse distinct
sources.

**Confidence:** high.

### FERRIUM-73: whole-target cleanup has cross-workspace blast radius

**Sources**

- [Cargo issue #5026](https://github.com/rust-lang/cargo/issues/5026)
- [Cargo issue #5931](https://github.com/rust-lang/cargo/issues/5931)
- Experiment:
  `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

**Observed behavior**

`cargo clean` issued from workspace B reduced the collision fixture's shared
target from 3,093,660 bytes to zero, removing A's state as well. In the registry
fixture, ordinary version and rustflag variation accumulated side-by-side
artifacts.

Cargo's upstream design expects ordinary `cargo clean` to remain
workspace-target cleanup while a separate cache owns access tracking, pruning,
and poisoned-entry recovery.

**Implication**

Cache ownership, retention, and recovery cannot be delegated to an unrelated
workspace's `cargo clean`. FERRIUM diagnostics should identify storage growth
and cleanup ownership separately from compile reuse.

**Confidence:** high for the fixture and upstream design direction.

### FERRIUM-74: shared target locking is a correctness and latency boundary

**Sources**

- [Cargo issue #4282](https://github.com/rust-lang/cargo/issues/4282)
- [Cargo issue #5931](https://github.com/rust-lang/cargo/issues/5931)

**Observed constraint**

Cargo historically locks a broad target/build directory for a command. Fine
grained locking work now permits some non-artifact-producing commands to run
concurrently, but unresolved cases still block at broader scope.

The upstream cache design proposes building outside the cache, publishing by
atomic rename, and coordinating reads with pruning.

**Implication**

Cross-workspace reuse cannot be evaluated only by hit rate. A cache can lose
wall time through lock contention or duplicate concurrent production even when
its identity is correct. PERF-Q06 must record restoration and contention cost.

**Confidence:** high for current tracking state.

### FERRIUM-75: exact real-workspace overlap can be sparse

**Source**

- Experiment:
  `docs/research/perf-q05-cross-workspace-cache/results/EXP-01-local-artifact-reuse.md`

**Observed behavior**

The pinned PERF-Q04 check graphs contained:

- nine registry package IDs in METIS-CORE;
- eleven in RUNE;
- none in PARLOR.

METIS-CORE and RUNE shared one exact visible unit signature,
`unicode-ident 1.0.24`. No registry package appeared in all three.

Cargo maintainers independently identify dependency-version churn and
dependency identity propagation as threats to local cache value.

**Implication**

FERRIUM should measure an overlap opportunity before promising a speedup. A
reuse ledger needs exact unit signatures and dependency cones, not a count of
similar crate names.

**Confidence:** medium. The sample is exact but small and limited to check
graphs.

### FERRIUM-76: the prerequisite self-contained build layout is not stable yet

**Sources**

- [Cargo issue #15010](https://github.com/rust-lang/cargo/issues/15010)
- [Cargo issue #17182](https://github.com/rust-lang/cargo/issues/17182)
- [2026 call for testing](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/)

**Observed constraint**

Cargo's proposed v2 build-directory layout groups fingerprints and outputs
under package and build-unit hashes so cacheable units can be managed
independently. It was stabilized in 2026 and then reverted after regressions,
including command-line scaling problems in large workspaces. The tracking issue
was reopened.

**Implication**

FERRIUM may analyze nightly layout experiments but must not depend on the
current internal directory shape. Stable operation remains read-only and based
on Cargo-supported metadata and JSON surfaces.

**Confidence:** high as of 2026-08-08.

### FERRIUM-77: the defensible FERRIUM wedge is eligibility and evidence, not
storage

**Sources**

- Findings FERRIUM-68 through FERRIUM-76
- [sccache Rust limitations](https://github.com/mozilla/sccache/blob/main/docs/Rust.md)

**Observed constraint**

sccache already caches supported rustc invocations, but disables incremental
compilation and cannot cache crates that invoke the system linker; proc macros
and filesystem-reading macros also retain caveats. Cargo is actively building
the higher-level package/unit cache with information unavailable to a compiler
wrapper.

FERRIUM already has identity, unit-variant, scheduling, and stable artifact
freshness models.

**Implication**

The differentiated opportunity is a reuse eligibility and provenance ledger:

- classify immutable versus local source;
- expose exact identity differences and dependency-cone exclusions;
- estimate overlap from observed commands;
- identify build-script, proc-macro, native, and flag blockers;
- produce minimized fixtures and evidence packets for upstream work.

A cache store, transport protocol, and compiler wrapper are deferred.

**Confidence:** high for the opportunity boundary; consumer value still needs
PERF-Q06 portfolio measurement.

## Recommendations

### Adopt now

- Add the cross-workspace provenance, integrity, isolation, locking, retention,
  and recovery vocabulary to the measurement contract.
- Treat immutable registry or Git units without excluded execution cones as
  candidate reuse, not guaranteed reuse.
- Preserve the path-package collision and corruption fixtures as controlled
  negative evidence.
- Carry exact unit overlap and restoration cost into PERF-Q06.

### Prototype behind a compatibility boundary

- A read-only reuse eligibility ledger using stable Cargo metadata and JSON.
- Optional nightly unit-graph and build-layout analysis, versioned by Cargo
  revision and never required for stable operation.
- A portfolio overlap report that estimates reusable units without importing
  or mutating artifacts.

The implementation gate remains closed. These are candidate capabilities for a
later wave, not authorized product work.

### Reject or defer

- Reject one shared writable `CARGO_TARGET_DIR` across unrelated repositories.
- Reject copying individual target files as a cache adapter.
- Defer a FERRIUM artifact store while Cargo issue #5931 is active.
- Defer build-script and proc-macro cacheability to PERF-Q23 and PERF-Q22.
- Defer CI transport and cache keys to PERF-Q06.
- Defer signing, remote producer trust, revocation, and transfer economics to
  PERF-Q30.
- Defer upstream issue, comment, or pull-request activity until the owner
  explicitly approves it.

## Potential upstream contribution paths

Without creating upstream activity, this research identifies bounded future
contributions:

1. publishable workload-overlap methodology for Cargo issue #5931;
2. minimized Windows fixtures for path collision, missing output, and corrupt
   metadata behavior;
3. build-directory v2 compatibility testing on deep portfolio workspaces;
4. documentation clarifying that shared target directories are not a
   provenance-safe user-wide cache.

The existing path-collision issue already has a reproduction, so any
contribution must add new evidence rather than duplicate the report.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: successful wrong-artifact reuse is treated as a correctness failure; freshness is not presented as integrity or proof. |
| Compiler Performance Engineer | Accepted: diagnostic timings are not promoted as benchmarks, misses remain intentional where identity differs, and overlap value remains unproven beyond the sampled corpus. |
| Interop Boundary Auditor | Accepted: build scripts, proc macros, native dependencies, target platforms, and remote transfer remain explicit excluded boundaries. |
| AI Assurance Skeptic | Accepted: corruption, collision, sparse overlap, reverted stabilization, and failed builds remain visible; no cacheability inference exceeds the evidence. |
| Ecosystem Strategist | Accepted: Cargo and sccache are treated as existing owners; FERRIUM differentiates through eligibility, explanation, fixtures, and evidence. |
| Rust Maintainer | Accepted: ordinary Cargo remains the authority, no target internals become a stable dependency, and no upstream activity occurred. |
| Native Platform Adopter | Accepted: the recommendation is reversible, read-only, cleanup ownership is explicit, and Windows behavior was tested. |
| Scope Keeper | Accepted: Q05 remains local cross-workspace reuse; CI topology and remote provenance stay in Q06 and Q30. |
| Validation Checker | Accepted: commands, environment, positive reuse, intentional misses, missing output, corruption, collision, cleanup, and corpus overlap are recorded. |

## Decision

PERF-Q05 is complete.

FERRIUM should contribute diagnosis and evidence to the cross-workspace-cache
problem, not build a competing cache or recommend shared writable targets. The
next cache question is PERF-Q06: measure real CI cache topology, exact overlap,
restore cost, contention, retention, and duplicate work using the provenance
model established here.
