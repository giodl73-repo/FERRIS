# Cargo Build-Unit Identity

Date: 2026-08-07
Question: PERF-Q02
Status: Complete
Decision: define the identity model FERRIUM will use to explain duplicate work,
missed reuse, rebuilds, and future cache experiments.

## Executive conclusion

"The Cargo build-unit identity" is not one key.

Cargo uses related but deliberately different identities:

1. a graph `Unit` determines whether two nodes in one invocation are the same
   package-target compilation;
2. `Metadata::unit_id` determines the artifact namespace and output filename;
3. `Metadata::c_metadata` determines symbol disambiguation;
4. a `Fingerprint` determines whether an artifact in that namespace is fresh
   or must be overwritten.

This separation is essential. A source edit or declared build-script input
change should normally rebuild the same artifact identity. A feature, profile,
mode, target, toolchain, compiler-flag, or dependency-identity change may
require a distinct artifact identity so both results can coexist.

FERRIUM should build an identity-diff and rebuild-explanation layer. It should
not build a shared artifact cache yet. Cargo intentionally makes local path
artifacts relocatable, and an open Cargo bug documents that unrelated
workspaces with equivalent relative path-package identities can collide when
they share `CARGO_TARGET_DIR`.

No issue, comment, branch, or pull request was created during this research.

## Decision supported

This research defines:

- the cache-identity vocabulary inherited by PERF-Q03 through PERF-Q07;
- the boundary between artifact identity and freshness;
- the minimum fields needed for an identity comparison;
- the safe first FERRIUM tooling opportunity;
- the blockers that must be resolved before cross-workspace cache experiments.

It does not authorize a remote cache, shared writable target directory, Cargo
fork, or upstream contribution.

## Evidence reviewed

### Local evidence

- `docs/research/2026-08-07-rust-latency-telemetry.md`
- `docs/research/2026-08-07-rust-incremental-reuse-boundaries.md`
- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/research/questions/PERF-Q02-cargo-build-unit-identity.md`

### Cargo source

Source revision:
[`21c2a90636b4a1991eacd14eca439e7e308c1af4`](https://github.com/rust-lang/cargo/commit/21c2a90636b4a1991eacd14eca439e7e308c1af4)

- [`UnitInner`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit.rs#L48-L138)
- [`Metadata` and `UnitHash`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/build_runner/compilation_files.rs#L19-L111)
- [`compute_metadata`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/build_runner/compilation_files.rs#L738-L857)
- [fingerprint identity comparison](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/fingerprint/mod.rs#L46-L111)
- [`calculate_normal`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/fingerprint/mod.rs#L1564-L1667)
- [unit-graph serialization](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_graph.rs#L39-L134)

### Cargo interfaces and issue state

- [`--unit-graph` tracking issue](https://github.com/rust-lang/cargo/issues/8002)
- [`-Zbuild-analysis` tracking issue](https://github.com/rust-lang/cargo/issues/15844)
- [shared target directory path-package collision](https://github.com/rust-lang/cargo/issues/12516)
- [source-path canonicalization](https://github.com/rust-lang/cargo/issues/7078)

### Experiment

- `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

## Identity model

| Layer | Purpose | Representative inputs | Expected behavior |
| --- | --- | --- | --- |
| Graph unit | Deduplicate and connect work inside one Cargo invocation | package, target, profile, host/target kind, compile mode, features, rustflags, rustdocflags, links overrides, artifact role, standard-library role, dependency hash | Equal units become one graph node |
| Artifact namespace | Keep simultaneously useful outputs separate | package ID, features, profile, mode, LTO, host/target kind, target name/kind, rustc version, selected wrapper/channel state, dependency unit IDs, most compiler arguments | Different identity normally gets a different output hash/path |
| Symbol metadata | Disambiguate symbols across crate versions/configurations | artifact identity subset plus dependency symbol identities | Compatible symbols remain stable where reproducibility requires it |
| Freshness fingerprint | Decide whether the artifact at an existing identity must rebuild | rustc, target details, profile and extra args, enabled and declared features, dependency fingerprints, source/dep-info state, linker/config/lints, rustflags, outputs | Changed freshness overwrites the existing artifact identity |

The layers overlap but are not interchangeable. FERRIUM must report which layer
changed instead of saying only "the cache missed."

## Findings

### FERRIUM-42: Cargo identity is layered by design

**Sources**

- [`Metadata` documentation](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/build_runner/compilation_files.rs#L38-L111)
- [fingerprint comparison table](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/fingerprint/mod.rs#L46-L111)

**Observed constraint**

Cargo separates the key that keeps outputs from colliding from the fingerprint
that decides whether an existing output is stale. It also uses a distinct
metadata hash for symbol mangling.

Cargo's source explains the tradeoff directly: placing too little in artifact
metadata can overwrite simultaneously useful results; placing too much there
retains unnecessary artifacts. Inputs such as source modification state belong
in the freshness fingerprint because the old artifact no longer needs to
coexist.

**Implication**

FERRIUM's evidence model needs separate `unit_identity`,
`artifact_identity`, and `freshness_cause` concepts. A single opaque cache key
would hide correctness and storage tradeoffs.

**Confidence:** high.

### FERRIUM-43: the graph unit includes more than package, target, and features

**Sources**

- [`UnitInner`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit.rs#L48-L138)
- [unit generation](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/ops/cargo_compile/unit_generator.rs#L53-L146)

**Observed constraint**

The internal unit contains package, manifest target, resolved profile,
compile kind, compile mode, sorted enabled features, rustflags, rustdocflags,
native links overrides, artifact-dependency role, standard-library role,
dependency hash, artifact target for feature resolution, and
compile-time-dependency state.

The dependency hash exists because otherwise identical units may link to
different dependency identities.

**Implication**

An external identity explanation that shows only package and features is
incomplete. FERRIUM should progressively disclose command, role, target,
profile, flags, and dependency identity.

**Confidence:** high.

### FERRIUM-44: features, profiles, modes, targets, flags, and toolchains can
create distinct artifact namespaces

**Sources**

- [`compute_metadata`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/build_runner/compilation_files.rs#L738-L857)
- Experiment:
  `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

**Observed behavior**

The synthetic fixture produced a different root artifact filename when the
`alpha` feature was enabled. Applying `RUSTFLAGS=-Copt-level=1` produced new
artifact filenames for all four compiler artifacts. Returning to the original
flags reused all original artifacts.

Cargo source also mixes the package identity, profile, mode, LTO, compile kind,
target name/kind, rustc version, dependency identities, and most applicable
compiler arguments into `unit_id`.

Compiler flags containing remap-path-prefix receive special handling because
absolute paths would harm reproducibility. `c_metadata` deliberately excludes
ordinary rustflags so symbol names can remain usable in workflows such as
profile-guided optimization.

**Implication**

FERRIUM can explain why two commands cannot share an artifact without
interpreting compiler internals. It must preserve Cargo's reproducibility
exceptions rather than inventing a simpler hash.

**Confidence:** high.

### FERRIUM-45: one package can correctly form several units in one command

**Source**

- Experiment:
  `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

**Observed behavior**

The five-unit synthetic `cargo check` graph contained two units for the same
dependency:

- a build-mode host dependency with `build-time` and `default` features;
- a check-mode runtime dependency with `runtime` and `default` features.

The application itself formed separate build-script compilation,
build-script execution, and root check units. An explicit target triple kept
the build script and its dependency on the host while the runtime units moved
to the selected target.

**Implication**

Package count is not a build-work count. FERRIUM should identify multiplication
by dependency role, compile mode, profile, target side, and feature set before
labeling work duplicated or avoidable.

**Confidence:** high.

### FERRIUM-46: command intent can multiply units dramatically

**Source**

- Experiment:
  `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

**Observed behavior**

On the public METIS-CORE fixture:

| Command shape | Units | Roots |
| --- | ---: | ---: |
| Check | 16 | 1 |
| Build | 16 | 1 |
| Release build | 16 | 1 |
| Test without running | 114 | 9 |
| Check with explicit host target | 16 | 1 |

The explicit target graph contained both host and target platforms. The test
graph multiplied units and roots even though the package graph did not change.

**Implication**

PERF-Q03 and PERF-Q04 should compare unit graphs, not package graphs or manifest
counts. Validation commands are a first-class source of build multiplication.

**Confidence:** high for the measured fixtures.

### FERRIUM-47: source and declared build-script input changes normally affect
freshness, not artifact identity

**Source**

- Experiment:
  `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

**Observed behavior**

A root source-body edit rebuilt one compiler artifact but retained the same
artifact filename. Reverting the edit rebuilt the same identity again because
Cargo's ordinary local freshness check uses source and dep-info modification
state rather than a content-addressed artifact history.

Changing the build script's declared `IDENTITY_INPUT` environment input reran
the build script and rebuilt the dependent application. The compiled build
script and both dependency artifacts stayed fresh. Cargo's structured analysis
reported the environment variable change as the root rebuild and the
application as one cascading rebuild.

**Implication**

FERRIUM should distinguish:

- identity change: a different artifact namespace;
- freshness change: overwrite work inside the same namespace;
- causal propagation: a dependency freshness change makes another unit dirty.

Warm revert does not imply artifact rollback reuse.

**Confidence:** high.

### FERRIUM-48: relocation reuse is intentional, but unrelated shared writable
target directories are unsafe

**Sources**

- [Cargo fingerprint path guidance](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/fingerprint/mod.rs#L252-L278)
- [shared target collision issue](https://github.com/rust-lang/cargo/issues/12516)
- [source-path canonicalization issue](https://github.com/rust-lang/cargo/issues/7078)
- Experiment:
  `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

**Observed behavior**

Copying the complete synthetic workspace to another absolute location and
using the original target directory reused all four compiler artifacts. This
matches Cargo's stated goal of avoiding absolute paths and preserving artifacts
when a project directory moves.

The same property creates ambiguity when different workspaces contain distinct
path packages with equivalent relative identity. Cargo issue #12516 documents
incorrect artifact reuse in a shared `CARGO_TARGET_DIR`; a Cargo maintainer
also notes lock contention, cache poisoning, and cache-entry conflicts in that
workflow. Symlink canonicalization remains a separate needs-design issue.

**Implication**

FERRIUM may explain relocation-compatible identity, but must not recommend one
shared writable target directory across unrelated repositories. Cross-workspace
reuse requires a stronger provenance and isolation model in PERF-Q05 and
PERF-Q30.

**Confidence:** high.

### FERRIUM-49: stable Cargo surfaces expose outcomes, not the complete identity
or reason model

**Sources**

- [`--unit-graph` tracking issue](https://github.com/rust-lang/cargo/issues/8002)
- [`-Zbuild-analysis` tracking issue](https://github.com/rust-lang/cargo/issues/15844)
- Experiment:
  `docs/research/perf-q02-cargo-identity/results/EXP-01-identity-dimensions.md`

**Observed constraint**

Cargo metadata exposes the package graph, not the invocation's unit graph.
Stable JSON messages expose artifacts and freshness, but not the full graph
identity or structured dirty cause.

The unstable unit graph exposes package, target, profile, platform, mode,
features, roots, and dependencies, but omits fields such as rustflags and some
internal dependency-role information. Its tracking issue remains
needs-design.

Current nightly build analysis produced a JSONL stream with invocation
identity, unit registration, dependency indexes, fingerprint status and cause,
start/finish timing, and unblocked units. Cargo documents its schema and
evolution as unresolved before stabilization.

**Implication**

FERRIUM should use stable Cargo metadata and JSON as the required baseline and
place build analysis behind a versioned nightly adapter. It must retain
`unknown` when stable evidence cannot identify the differing field.

**Confidence:** high.

### FERRIUM-50: explanation is ready; cache intervention and upstream filing
are not

**Sources**

- [`-Zbuild-analysis` tracking issue](https://github.com/rust-lang/cargo/issues/15844)
- [shared target collision issue](https://github.com/rust-lang/cargo/issues/12516)
- [source-path canonicalization issue](https://github.com/rust-lang/cargo/issues/7078)

**Observed constraint**

Cargo's build-analysis work already owns structured rebuild reporting. Shared
target and path canonicalization concerns remain needs-design and involve
correctness, reproducibility, cleanup, locking, and poisoning tradeoffs.

**Implication**

The immediate FERRIUM opportunity is an evidence consumer:

- compare two commands or sessions;
- explain which identity layer changed;
- show required versus suspicious multiplication;
- preserve stable-only operation;
- generate minimized evidence for later upstream discussion.

Any upstream issue, comment, or PR requires explicit owner approval and prior
coordination with Cargo maintainers. No filing was made by this research.

**Confidence:** high.

## Minimum identity comparison

When comparing two observed units, FERRIUM should record:

1. package ID and source class;
2. target name, target kind, and source path relative to the workspace;
3. compile mode and requested root status;
4. profile fields, including optimization, debug information, panic strategy,
   incremental mode, LTO, and codegen units where known;
5. host or target compile kind and selected target triple;
6. enabled feature set and feature role;
7. toolchain and relevant wrapper;
8. rustflags, rustdocflags, and command extra arguments where observable;
9. build-script or native-links override identity;
10. artifact-dependency and standard-library role;
11. dependency identities and edge roles;
12. artifact filename or namespace identity;
13. freshness status and structured dirty cause;
14. evidence source and missing fields.

The comparison result is one of:

- identical unit and fresh;
- identical artifact identity but dirty;
- distinct artifact identity for a required reason;
- distinct artifact identity for a suspicious or unknown reason;
- unsafe to compare because provenance is incomplete.

## Recommendations

### Adopt now

- Add the layered identity vocabulary to all later PERF research.
- Treat package count and manifest count as insufficient work metrics.
- Classify rebuilds as identity, freshness, or propagation changes.
- Treat unrelated shared writable target directories as unsupported.
- Use immutable toolchain and Cargo source revisions in identity evidence.

Owner: FERRIUM.

Validation: PERF-Q03 and PERF-Q04 must use unit-level rather than package-level
comparisons.

### Prototype behind a compatibility boundary

- A read-only identity/session diff over stable Cargo metadata and JSON.
- A nightly adapter for `-Zbuild-analysis` logs.
- A diagnostic-only adapter for `--unit-graph`.
- Evidence export that removes private paths and source content.

Owner: FERRIUM.

Validation:

- fixture-based schema-version tests;
- stable-only fallback;
- no artifact writes or cache substitution;
- correct classification of feature, profile, mode, target, flag, source, and
  build-script-input changes;
- privacy review.

### Reject or defer

- Shared writable `CARGO_TARGET_DIR` across unrelated repositories.
- Remote or prewarmed artifact distribution before provenance research.
- Treating Cargo internal fingerprint files as a stable API.
- Guessing unobservable rustflags or dependency roles from filenames.
- Filing upstream work without explicit owner approval.

## Role review

| Role | Verdict | Required discipline |
| --- | --- | --- |
| Rust Safety Steward | Approve | Cache identity and freshness remain correctness boundaries; no artifact substitution is authorized. |
| Compiler Performance Engineer | Approve | Unit multiplication is measured by command, role, mode, profile, target, and features. |
| Interop Boundary Auditor | Approve | Host/target, build-script, native-links, and artifact-dependency roles remain explicit. |
| AI Assurance Skeptic | Approve | Missing identity fields produce unknown, not an invented explanation. |
| Ecosystem Strategist | Approve | FERRIUM complements Cargo build analysis instead of competing with it. |
| Rust Maintainer | Approve | The first tool is read-only and preserves ordinary Cargo behavior. |
| Native Platform Adopter | Approve with restriction | Shared writable target directories are rejected until isolation and provenance are proven. |
| Scope Keeper | Approve | Identity explanation is separated from cache implementation and upstream filing. |
| Validation Checker | Approve | Source fields and controlled changes were cross-checked against artifact and freshness behavior. |

No role raised a blocking objection.

## Open follow-ups

- PERF-Q03: quantify scheduling and critical paths over the resulting unit
  graph.
- PERF-Q04: measure multiplication across features, profiles, targets, and test
  modes on larger workspaces.
- PERF-Q05: research safe cross-workspace artifact reuse without assuming a
  shared writable target directory.
- Track Cargo build-analysis schema evolution before defining any adapter.
