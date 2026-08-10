# Rust Feature and Version Fragmentation

Date: 2026-08-09
Status: Complete
Question: ECOS-Q08
Decision: represent fragmentation as typed, renewable evidence across package
identity, version requirements, requesting edges, public exposure, effective
features, resolver/target/dependency-kind scope, compiler work, artifact and
binary cost, interchange consequences, and remediation ownership. Diagnose
duplicates and feature expansion without assuming they are defects or
rewriting dependency graphs automatically.

## Decision supported

ECOS-Q08 defines the fragmentation evidence required by the OSPREY Ecosystem
adapter and Crate Ecosystem Ledger.

It does not:

- treat every duplicate package version as avoidable;
- treat a zero-duplicate graph as interoperable or efficient;
- infer cost from package count alone;
- infer binary growth from compile-time feature expansion;
- assume `default-features = false` prevents another edge from enabling
  defaults;
- assume Cargo can update across incompatible requirements;
- automatically edit version requirements, lockfiles, features, adapters, or
  public APIs; or
- authorize OSPREY implementation.

## Fragmentation evidence model

| Dimension | Required evidence |
|---|---|
| Package instance | Name, version, source, checksum, package ID, target, profile, and dependency kind |
| Requirement | Declared semver requirement, exact or compatible range, requesting package and edge, lockfile selection, and resolver rationale where observable |
| Multiplicity | Family instance count, duplicate versions, shared and duplicated transitive closure, target/profile reachability, and current `cargo tree -d` evidence |
| Feature demand | Requesting edge, default-feature policy, explicit feature, optional dependency, command-line feature, target and dependency-kind scope |
| Effective features | Unified package feature set, resolver version, target, activated optional dependencies, build scripts, procedural macros, providers, and generated code |
| Exposure | Private implementation use, public type/trait exposure, generic bound, trait object, serialized boundary, adapter, facade, or native/runtime provider |
| Interchange | Exact identity, re-export identity, conversion, wrapper, coherence, semantic loss, runtime behavior, or explicit incompatibility |
| Compiler cost | Clean and incremental command, sample count, wall time, compiler artifacts, build scripts, procedural macros, invalidation scope, and variance |
| Artifact cost | Executable, library, debug data, target-directory footprint, retained symbols or behavior where observed, and target/profile/toolchain |
| Disposition | Required multiplicity, compatible but stale, avoidable constraint, public-contract migration, feature-policy defect, adapter-owned, deferred, or unknown |
| Remediation | Current owner, proposed manifest/API/upstream action, compatibility boundary, validation, rollback, and non-goals |
| Renewal | Package, requirement, lockfile, feature, resolver, target, profile, toolchain, exposure, or measurement change |

No single duplicate count or fragmentation score can represent these
dimensions.

## Measured controls

Commands, exact graphs, five-sample clean timing medians, artifact sizes,
feature-conflict controls, version-identity failures, Cargo update behavior,
sources, and limitations are in
[EXP-01](ecos-q08-feature-version-fragmentation/results/EXP-01-fragmentation-cost-matrix.md).

Observed:

- the combined nineteen-release foundational probe had no duplicate package
  versions on the observed Windows graph;
- an HTTP 0.2.12 + 1.5.0 fixture added two compiler artifacts, increased the
  clean release median 7.5%, increased the executable 12.9%, and increased the
  target footprint 40.4% over the HTTP 1.5-only control;
- a Syn 2.0.119 + 3.0.3 fixture shared Proc Macro2, Quote, and Unicode Ident,
  but the extra Syn instance still increased the clean release median 24.0%,
  executable size 92.6%, and target footprint 57.0%;
- enabling Serde derive expanded the active Windows closure from three to
  eight packages, compiler artifacts from five to twelve, build scripts from
  two to four, and procedural macros from zero to one;
- the Serde derive fixture increased clean check time 119.0%, clean release
  time 91.5%, and target footprint 106.6%, while the observed release
  executable and PDB sizes were unchanged;
- all thirty no-op release builds reported zero nonfresh compiler artifacts,
  showing that clean/rebuild cost and already-fresh graph overhead are
  different measurements;
- HTTP, Rand Core, and Syn duplicate identities reconfirmed expected E0308 or
  E0277 interchange failures;
- Cargo refused to update exact HTTP 0.2.12 to 1.5.0 because the root
  requirement excluded the requested version and left the lockfile unchanged;
  and
- two branches requesting mutually exclusive features each compiled alone,
  while the resolver-3 application unified both features on one package
  instance and failed its explicit conflict guard.

These fixtures quantify selected mechanisms; they do not rank crates or
recommend dependency changes.

## Findings

### FERRIUM-607: multiplicity and fragmentation are not synonyms

**Sources:** foundational queue `cargo tree -d` and controlled duplicate
fixtures in EXP-01.

**Observed behavior:** The observed foundational graph contained no duplicate
versions, while deliberately composed graphs did. A graph can still have
costly feature expansion or incompatible semantics with one version, and a
duplicate can be required by incompatible contracts.

**Implication:** Record version multiplicity, feature expansion, interchange,
and measured cost as separate evidence.

**Confidence:** High.

### FERRIUM-608: Cargo unifies compatible requirements and permits incompatible versions

**Sources:** Cargo resolver documentation, duplicate fixtures, and failed
cross-major update control.

**Observed behavior:** Shared compatible dependencies such as Bytes, Proc
Macro2, Quote, and Unicode Ident resolved once. Exact HTTP 0.2.12 and 1.5.0
requirements resolved as separate package instances. Cargo rejected replacing
0.2.12 with 1.5.0 because it violated the declared requirement.

**Implication:** A duplicate diagnosis must name every requesting requirement
and whether a common version actually satisfies them.

**Confidence:** High.

### FERRIUM-609: duplicate cost depends on the duplicated crate and retained use

**Sources:** five clean check and release samples for HTTP and Syn pairs.

**Observed behavior:** The HTTP duplicate increased the clean release median
7.5% and executable 12.9%. The Syn duplicate increased the clean release
median 24.0% and executable 92.6%.

**Implication:** Do not estimate compile or binary impact from duplicate count.
Measure the selected target/profile and exercised code.

**Confidence:** High for the fixtures; low for unrelated applications.

### FERRIUM-610: shared transitive dependencies can bound duplicate cost

**Sources:** single and dual Syn active closures.

**Observed behavior:** Adding Syn 2 beside Syn 3 increased the active package
count from five to six, not from five to ten, because both versions shared
Proc Macro2, Quote, and Unicode Ident.

**Implication:** Fragmentation evidence must distinguish duplicated package
instances from shared and duplicated transitive closure.

**Confidence:** High.

### FERRIUM-611: public version exposure turns multiplicity into incompatibility

**Sources:** ECOS-Q03 and reconfirmed HTTP, Rand Core, and Syn compile-fail
controls.

**Observed behavior:** Same-named HTTP requests and Syn AST nodes from
different versions failed E0308. Same-named Rand Core traits failed E0277.

**Implication:** Prioritize duplicates that cross public type, trait, generic,
trait-object, or serialized boundaries. Private multiplicity may be only a
cost or assurance concern.

**Confidence:** High.

### FERRIUM-612: coherence and semver prevent blind deduplication

**Sources:** failed Cargo update control and ECOS-Q03 orphan-rule fixture.

**Observed behavior:** Cargo preserved the exact incompatible requirement, and
a downstream crate could not implement a foreign conversion trait directly
between two foreign HTTP types.

**Implication:** Remediation may require upstream requirement changes, a
facade, a dedicated adapter, a local wrapper, or a public API migration. A
lockfile rewrite alone is not sufficient.

**Confidence:** High.

### FERRIUM-613: feature expansion can cost more than one duplicate version

**Sources:** Serde minimal and derive active graphs and build samples.

**Observed behavior:** Derive added five active packages and seven compiler
artifacts, raising the clean check median 119.0% and release median 91.5%.
The HTTP duplicate added two artifacts and raised those medians 5.5% and 7.5%.

**Implication:** Diagnostics must compare effective feature closures as well
as duplicate versions.

**Confidence:** High for the fixtures.

### FERRIUM-614: compile-time features can expand artifacts without expanding the binary

**Sources:** Serde release artifacts.

**Observed behavior:** Derive added Syn, Quote, Proc Macro2, Unicode Ident, and
Serde Derive, doubled the target footprint, and added a procedural macro. The
observed optimized executable and PDB sizes were unchanged.

**Implication:** Separate compiler, cache/storage, executable, runtime, and
distribution cost. Binary size is not a proxy for build cost.

**Confidence:** High for this fixture.

### FERRIUM-615: `default-features = false` is edge-local

**Sources:** ECOS-Q03 feature-unification fixture and Cargo feature
documentation.

**Observed behavior:** The low branch requested Serde `alloc` without
defaults. When composed with a derive/default branch, the shared Serde package
also enabled default, `std`, derive, and Serde Derive features.

**Implication:** Absence-sensitive policies such as no-`std`, provider
exclusion, or reduced compile surface require resolved-graph assertions.

**Confidence:** High.

### FERRIUM-616: mutually exclusive features are a composition defect

**Sources:** resolver-3 backend feature-conflict fixture.

**Observed behavior:** Backend A and Backend B branches compiled independently.
The combined application enabled both features on the same package and failed
an explicit mutual-exclusion guard.

**Implication:** Prefer additive features. Incompatible backends may require
runtime selection, separate packages, target-specific resolution, or an
application-owned provider boundary.

**Confidence:** High.

### FERRIUM-617: some multiplicity is required and should be retained

**Sources:** Cargo resolver rules, exact cross-major fixtures, ECOS-Q03
interchange evidence, and ECOS-Q07 MSRV/platform evidence.

**Observed behavior:** Requirements can be genuinely incompatible because of
semver, public contracts, MSRV, target, provider, or migration sequencing.
Forcing one version can break compilation or behavior.

**Implication:** Disposition duplicates as required, removable, migratory,
adapter-owned, or unknown before proposing change.

**Confidence:** High.

### FERRIUM-618: `cargo tree -d` is a locator, not a verdict

**Sources:** Cargo tree documentation and measured fixtures.

**Observed behavior:** The command identified duplicate HTTP and Syn versions
and showed their reverse edges. It did not report public exposure, conversion,
semantic loss, compile time, binary retention, target reachability, or
remediation ownership.

**Implication:** Reuse Cargo's graph evidence and join it with interchange,
cost, target, assurance, and ownership records.

**Confidence:** High.

### FERRIUM-619: already-fresh builds do not reveal rebuild cost

**Sources:** thirty no-op release samples.

**Observed behavior:** Every no-op sample reported zero nonfresh compiler
artifacts, even for the larger duplicate and derive graphs.

**Implication:** Measure clean builds and representative invalidation paths
separately from no-op graph resolution. Do not claim a duplicate is free
because a fresh build reused it.

**Confidence:** High.

### FERRIUM-620: artifact footprint and shipped footprint are different

**Sources:** release target-directory, executable, and PDB measurements.

**Observed behavior:** Serde derive added 24,054,367 target bytes with no
observed executable or PDB increase. HTTP and Syn duplicates increased both
build footprint and shipped executable size by different proportions.

**Implication:** Record target/cache storage, debug data, executable, library,
and distribution artifacts independently.

**Confidence:** High for the selected Windows release profile.

### FERRIUM-621: fragmentation remediation is an owned compatibility change

**Sources:** failed update, coherence, adapter, feature-conflict, and
measurement controls.

**Observed behavior:** Potential remediations affect declared requirements,
public types/traits, generated code, provider selection, MSRV, targets, and
lockfiles.

**Implication:** Every proposal needs the current owner, consumer impact,
upstream path, validation, rollout, rollback, and removal plan. OSPREY should
diagnose and model options, not edit automatically.

**Confidence:** High.

### FERRIUM-622: fragmentation evidence expires with the resolved graph

**Sources:** exact lockfiles, feature trees, resolver scope, target/profile
measurements, and prior ECOS decisions.

**Observed behavior:** Version and feature multiplicity, active packages,
compiler work, and binary retention depend on the lockfile, resolver, target,
profile, toolchain, feature requests, and code reachability.

**Implication:** Renew after any of those inputs or public exposure changes.

**Confidence:** High.

## Recommendations

### Adopt now

- Add the fragmentation evidence model to the OSPREY Ecosystem adapter and
  Crate Ecosystem Ledger.
- Reuse Cargo package IDs, metadata, feature trees, inverse trees, and
  duplicate reports rather than inventing a resolver.
- Record requesting requirements and effective target/dependency-kind feature
  sets.
- Join duplicate and feature evidence with public exposure, interchange,
  compiler work, artifact size, platform, assurance, stewardship, and owner.
- Preserve required, removable, migratory, adapter-owned, deferred, and
  unknown dispositions.

### Prototype behind a compatibility boundary

- read-only duplicate and feature-expansion diagnostics;
- clean, no-op, and representative invalidation cost comparisons;
- public type/trait exposure and adapter-path identification;
- lockfile and feature-closure diffs with build-script, macro, provider, and
  native-boundary changes;
- maintainer-reviewed remediation options with commands and rollback; and
- FERRIS evidence packets for exact before/after graph observations.

### Reject or defer

- one fragmentation, dependency-health, or deduplication score;
- automatic semver widening, lockfile rewriting, feature removal, or provider
  selection;
- treating every duplicate as a defect;
- treating no duplicates as proof of compatibility or efficiency;
- binary-size claims inferred from package count;
- build-cost claims inferred from executable size;
- FERRIUM-owned replacement facades where upstream coordination is viable; and
- OSPREY implementation before the Crates Series gate.

## Role review

### Rust Safety Steward

Accepts explicit provider and feature-conflict states and rejects forced
unification that could change safety assumptions. Requires semantic and unsafe
boundaries to be revalidated after graph changes.

### Compiler Performance Engineer

Accepts five-sample clean medians, compiler-artifact counts, no-op controls,
and separate target/executable footprints. Requires future work to add
representative incremental invalidation before generalizing costs.

### Interop Boundary Auditor

Accepts public exposure, nominal identity, trait, coherence, adapter, and
semantic disposition as first-class fragmentation evidence. Requires adapters
to retain direction, loss, allocation, and ownership.

### AI Assurance Skeptic

Accepts raw commands, exact versions, negative controls, scope, and unknowns.
Rejects recommendations inferred from duplicate count or modeled savings
without measured before/after behavior.

### Ecosystem Strategist

Accepts reuse of Cargo diagnostics and upstream feature/version mechanisms.
Requires contribution to current crate owners rather than a FERRIUM package
resolver or replacement registry.

### Rust Maintainer

Accepts read-only, actionable diagnostics that identify requesting edges,
effective features, cost, exposure, and options. Requires ordinary Cargo use,
reviewable changes, and a clear removal path.

### Native Platform Adopter

Accepts target/profile-specific cost and provider evidence. Requires proposed
graph changes to preserve supported platforms, native dependencies, deployment,
offline operation, diagnostics, and rollback.

### Scope Keeper

Accepts Q08 as evidence and diagnostics only. Automatic dependency editing,
stack approval, package replacement, migration execution, and implementation
remain closed.

### Validation Checker

Accepts six passing comparison binaries, sixty clean build samples, thirty
no-op samples, active graphs, feature trees, three version-identity failures,
one failed update control, one feature-conflict control, sources, and
limitations. Requires raw before/after identities and fixture equivalence in
future renewals.

## Limitations

- Measurements used one Windows host, one current toolchain, and one release
  profile.
- Clean samples used fresh target directories with warm registry and source
  caches.
- Five samples characterize these fixtures, not ecosystem-wide cost.
- User code was controlled for equivalent output but not instruction-level
  equivalence.
- Executable size does not establish runtime memory or performance.
- Target-directory size includes intermediate and debug artifacts specific to
  Cargo and rustc 1.95.0.
- No representative incremental source or dependency edit was measured.
- Feature expansion was measured with Serde derive; other features can have
  different costs and runtime effects.
- Duplicate compatibility controls covered HTTP, Rand Core, and Syn only.
- The foundational queue's zero-duplicate result applies only to the observed
  lockfile and Windows graph.
- No automatic remediation was attempted.

## Primary sources

- Cargo features:
  <https://doc.rust-lang.org/cargo/reference/features.html>
- Cargo dependency resolver:
  <https://doc.rust-lang.org/cargo/reference/resolver.html>
- Cargo tree:
  <https://doc.rust-lang.org/cargo/commands/cargo-tree.html>
- Cargo update:
  <https://doc.rust-lang.org/cargo/commands/cargo-update.html>
- Rust implementations and orphan rules:
  <https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules>
