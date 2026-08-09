# Remote Artifact Provenance and Rust Build Forest Roots

Date: 2026-08-09

Status: Research complete; bounded manifest prototype authorized, remote
artifact service and automatic restoration not authorized

Decision: adopt signed immutable forest roots, policy-checked mutable labels,
explicit producer and consumer expectations, and content-addressed transport as
the control-plane model. Prototype a read-only local manifest and visualization
plus disposable exact-identity transport experiments. Align reusable Cargo
units with Cargo's upstream cross-workspace cache work. Defer production remote
reuse, cross-platform restoration, and execution-cone artifacts.

## Decision supported

PERF-Q30 asks what identity, provenance, trust, transport, platform, and
invalidation model is required to reuse native Rust artifacts or atomic rustc
incremental generations produced elsewhere, and which parts can safely
participate in a labeled Rust Build Forest.

The answer separates four different claims:

1. **Identity:** the consumer requested the same build action and compatibility
   envelope.
2. **Integrity:** downloaded bytes match an immutable digest and signed
   statement.
3. **Provenance and trust:** an accepted builder asserts how the bytes were
   produced.
4. **Correctness:** required validation establishes that the result is suitable
   for use.

No one claim substitutes for the others. A cache hit, valid signature, trusted
label, Cargo-fresh result, or successful compiler invocation is not by itself a
correctness proof.

## Evidence

### Local FERRIUM evidence

- [Cross-workspace artifact reuse](2026-08-08-cross-workspace-artifact-reuse.md)
- [CI cache topology](2026-08-08-ci-cache-topology.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Build-script input, output, and rerun precision](2026-08-09-build-script-input-output-precision.md)
- [Rust Build Forest opportunity](2026-08-08-rust-build-forest-opportunity.md)
- [EXP-01 provenance and transport matrix](perf-q30-remote-artifacts/results/EXP-01-provenance-transport-matrix.md)

### External standards and upstream direction

- Bazel Remote Execution API:
  <https://github.com/bazelbuild/remote-apis/blob/main/build/bazel/remote/execution/v2/remote_execution.proto>
- Bazel remote caching:
  <https://bazel.build/remote/caching>
- SLSA provenance v1.2:
  <https://slsa.dev/spec/v1.2/build-provenance>
- SLSA artifact verification:
  <https://slsa.dev/spec/v1.2/verifying-artifacts>
- Sigstore overview and Cosign verification:
  <https://docs.sigstore.dev/about/overview/> and
  <https://docs.sigstore.dev/cosign/verifying/verify/>
- The Update Framework specification:
  <https://theupdateframework.github.io/specification/latest/>
- Rust 2026 Cargo cross-workspace cache goal:
  <https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/cargo-cross-workspace-cache.md>

## Vocabulary

1. **Action identity:** canonical build type, external parameters, resolved
   dependencies, producer-independent command semantics, and platform
   properties used to decide whether a root is a candidate.
2. **Artifact class:** supported Cargo unit, complete rustc incremental
   generation, final linked output, evidence packet, or validation result.
3. **Compatibility envelope:** exact compiler and sysroot, target and ABI,
   profile, features, flags, source identity, dependency identities, native
   environment, and declared execution inputs required by one artifact class.
4. **Execution cone:** build script, proc macro, native tool, SDK, environment,
   filesystem, clock, network, or other execution that can introduce inputs not
   represented by ordinary package identity.
5. **Content-addressed blob:** immutable bytes named by cryptographic digest and
   size.
6. **Immutable forest root:** canonical signed manifest that identifies action
   expectations, subjects, provenance, lineage, validation, and referenced
   blobs.
7. **Mutable label:** signed, versioned, expiring policy reference from a human
   name to one immutable root.
8. **Producer trust policy:** accepted signer or workload identity, builder ID,
   build type, permissions, and publication scope.
9. **Consumer expectation:** the exact action, platform, source, dependencies,
   validation, and trust requirements checked before installation.
10. **Atomic publication:** stage, verify, and finalize one complete root
    without exposing partial bytes.
11. **Isolated installation:** materialize an immutable root into a private
    consumer directory before Cargo or rustc mutates it.
12. **Revocation policy:** deny future resolution or use of a signer, builder,
    label, or root without pretending historical signatures or CAS bytes ceased
    to exist.

## Findings

### FERRIUM-401: action identity and content identity are different namespaces

**Sources:** Bazel Remote Execution API and Bazel remote caching; PERF-Q05.

**Observed behavior:** Bazel separates an action cache keyed by the encoded
action from a CAS keyed by the bytes themselves. FERRIUM's prior experiments
likewise found that artifact bytes without Cargo fingerprints, dep-info, and
sidecars are not a reusable Cargo entry.

**Implication:** A forest root or transport digest cannot be the lookup key for
semantic compatibility. The consumer first computes or matches an action
identity, then resolves immutable content.

**Confidence:** High.

### FERRIUM-402: provenance authenticates an assertion, not the build result

**Sources:** SLSA provenance and verification specifications; Sigstore
verification documentation; Rust Safety Steward and AI Assurance Skeptic
roles.

**Observed behavior:** SLSA verification checks the artifact subject, signature,
builder identity, build type, and external parameters against consumer
expectations. It explicitly requires trust in the build platform. Sigstore
binds signatures to identity and transparency evidence, but does not establish
the behavioral correctness of the signed artifact.

**Implication:** Forest roots must keep provenance, trust policy, and validation
evidence distinct. A trusted producer can still be mistaken or compromised.

**Confidence:** High.

### FERRIUM-403: the first reusable Cargo boundary should remain conservative

**Sources:** PERF-Q05, PERF-Q22, PERF-Q23; Rust 2026 Cargo cross-workspace cache
goal.

**Observed behavior:** Cargo's accepted 2026 goal starts with basic crates and
explicitly excludes build scripts and proc macros initially. FERRIUM measured
successful wrong reuse for unrelated path packages in a shared writable target,
stale proc-macro output after hidden input changes, and stale build-script
output after undeclared reads.

**Implication:** FERRIUM should contribute identity, provenance, diagnostics,
and evaluation fixtures to Cargo's cache work rather than build a parallel
general artifact store. Initial remote candidates exclude path packages,
build scripts, proc macros, native tools, and unresolved execution cones.

**Confidence:** High.

### FERRIUM-404: simple artifact identity is bounded; unrestricted execution is not

**Sources:** PERF-Q05, PERF-Q22, PERF-Q23; SLSA build-definition model.

**Observed behavior:** Registry revision, features, profile, target, flags,
compiler, dependency identities, and declared inputs can be represented.
Unrestricted code can additionally observe arbitrary files, environment,
network, clocks, tools, SDKs, and native state.

**Implication:** The compatibility envelope must reject unknown or unrecognized
external parameters and unresolved execution cones. Signing an incomplete
identity only authenticates the incompleteness.

**Confidence:** High.

### FERRIUM-405: a complete rustc generation can be transported as opaque atomic state

**Sources:** PERF-Q18; EXP-01.

**Observed behavior:** A complete 17.9 MB incremental generation was packaged,
verified, restored into an isolated directory, and reused by the exact nightly
compiler and source identity. The restored unchanged compile median was
204.6 ms versus 436.6 ms cold, and the output digest matched.

**Implication:** Whole-generation transport is technically possible without
understanding or composing internal files. It remains an exact-version,
experimental artifact class rather than a stable portable format.

**Confidence:** High for the tested fixture; low for general portability.

### FERRIUM-406: immutable remote roots must be materialized into mutable consumer state

**Sources:** PERF-Q18; EXP-01.

**Observed behavior:** Rustc changed the restored directory after a successful
compile as it advanced and retained compiler-private generations.

**Implication:** Consumers must never compile directly in CAS storage. Verify an
immutable root, then copy, clone, or otherwise materialize it atomically into an
isolated mutable directory owned by one compatible consumer session.

**Confidence:** High.

### FERRIUM-407: source location is part of the tested incremental identity

**Sources:** EXP-01.

**Observed behavior:** Equal source bytes moved to another path were rejected by
the external policy and compiled in 400.0 ms, near the 436.6 ms cold median;
the emitted metadata identity also changed.

**Implication:** Cross-machine reuse cannot assume content equality removes
absolute-path sensitivity. A portable design requires a supported path-remap
and reproducibility contract, tested for diagnostics, debuginfo, macros, native
tools, and outputs.

**Confidence:** High for the fixture; medium for all artifact classes.

### FERRIUM-408: incompatible compiler state should miss before transfer

**Sources:** EXP-01.

**Observed behavior:** A changed rustflag compiled in 478.9 ms and a different
stable compiler and sysroot in 515.8 ms after forced restoration. Rustc safely
recomputed, but the transported bytes provided no benefit.

**Implication:** Compiler version text alone is insufficient. Candidate
selection must include compiler binary or build identity, sysroot, target,
flags, and artifact-class schema before downloading or installing a root.

**Confidence:** High.

### FERRIUM-409: signed content addressing detects transport and composition attacks

**Sources:** Bazel Remote Execution API; SLSA; EXP-01.

**Observed behavior:** EXP-01 rejected a modified signed manifest, a changed ZIP
byte, a changed `query-cache.bin`, and a generation whose file set advanced
after an edit.

**Implication:** Roots bind canonical metadata and transport subjects; clients
verify before publication or installation and fail closed to an ordinary
rebuild. Internal rustc files are never selected or merged independently.

**Confidence:** High.

### FERRIUM-410: one signed transport digest is the normal integrity path

**Sources:** Bazel CAS compressed-blob requirements; EXP-01.

**Observed behavior:** SHA-256 verification, Ed25519 verification, and
extraction cost 87.5 ms locally. Rehashing every extracted file added another
97.1 ms. The signed transport-digest path broke even near 351 Mbps before
network latency; the full-tree audit required about 1,071 Mbps.

**Implication:** A monolithic archive root should sign its compressed blob
digest and size. Full extracted-tree hashing is an audit, diagnosis, or
Merkle-node mode, not mandatory duplicate hashing after every trusted blob
verification.

**Confidence:** High for the measured payload.

### FERRIUM-411: remote benefit is a net-benefit decision, not a hit-rate claim

**Sources:** PERF-Q06; Bazel remote caching; EXP-01.

**Observed behavior:** The 6.3 MB payload was 362.4 ms slower than cold
compilation at an idealized 100 Mbps, 93.7 ms faster at 1 Gbps, and 139.3 ms
faster at 10 Gbps under transport-digest verification. These calculations omit
latency and service overhead.

**Implication:** Publication and restore policy must compare expected avoided
work with hashing, compression, transfer, extraction, verification, contention,
and miss probability. Small roots may be local-only even when technically
reusable.

**Confidence:** High for the fixture; medium for workload extrapolation.

### FERRIUM-412: labels are policy references, never cache keys or correctness evidence

**Sources:** Rust Build Forest architecture; TUF specification; EXP-01.

**Observed behavior:** Signed sequence-1 and sequence-2 labels could advance one
human name while preserving immutable roots. Replaying sequence 1 after 2,
using an expired label, or resolving a revoked root was rejected.

**Implication:** Labels include sequence or version, expiration, target root,
and signature. Clients retain anti-rollback state. Action identity finds
content; labels support navigation, release policy, pinning, and audit.

**Confidence:** High.

### FERRIUM-413: revocation controls future trust, not historical existence

**Sources:** TUF and Sigstore documentation.

**Observed behavior:** TUF rotates and revokes trusted roles through newer
metadata. Sigstore transparency records are intentionally durable. Neither
model makes an old signature or content digest cease to have existed.

**Implication:** Forest policy can tombstone or deny a root, signer, or builder
for future use while retaining audit history. Physical CAS deletion is a
retention action and cannot be the only revocation mechanism.

**Confidence:** High.

### FERRIUM-414: publication requires a successful finalized producer state

**Sources:** PERF-Q06, PERF-Q18, SLSA provenance model.

**Observed behavior:** Failed rustc compilations preserve the last good
generation, and cache services can expose immutable first-writer entries.
Publishing before producer success or from an in-place directory can therefore
name stale or partial state.

**Implication:** A producer stages an isolated build, completes required
validation, snapshots finalized bytes, creates provenance, signs the root, and
publishes with compare-and-set or idempotent content addressing. Failed,
cancelled, or interrupted producers publish nothing.

**Confidence:** High.

### FERRIUM-415: native and cross-platform artifacts need an explicit ABI envelope

**Sources:** PERF-Q23; link capability contract from PERF-Q29; Native Platform
Adopter and Interop Boundary Auditor roles.

**Observed behavior:** Build scripts can emit native link search paths,
libraries, and environment-dependent outputs. Link results depend on target,
ABI, SDK, linker, native objects, debug packaging, signing, and deployment
policy.

**Implication:** No root is called cross-platform merely because Rust source is
portable. Native artifact reuse requires a declared and validated platform,
ABI, tool, SDK, library, symbol, and deployment envelope. Unknown axes miss.

**Confidence:** High.

### FERRIUM-416: retention follows root reachability and policy

**Sources:** Rust Build Forest architecture; Bazel remote caching; TUF.

**Observed behavior:** Content-addressed blobs may be shared by many roots,
while labels and pins identify currently valuable histories. Cache backends
still require quotas, lifecycle management, and poisoned-entry recovery.

**Implication:** Garbage collection starts from pinned roots, accepted labels,
active leases, and retention policy, then traverses referenced blobs. Deleting
an unreachable blob must not mutate another root, and cache absence must remain
a correctness-neutral rebuild.

**Confidence:** High.

### FERRIUM-417: PERF-Q30 closes only the read-only forest provenance gate

**Sources:** all evidence above and the nine-role review.

**Observed behavior:** The manifest, label, trust, compatibility, integrity,
publication, revocation, retention, and recovery rules are bounded enough for a
read-only control plane. General Cargo artifact reuse, compiler-private format
stability, execution-cone identity, cross-platform portability, and broad
remote economics remain unresolved.

**Implication:** FERRIUM may prototype local immutable manifests, labels,
lineage, policy evaluation, visualization, and disposable exact-identity
transport. It may not deploy an artifact service, automatically restore state,
or promise portable remote builds.

**Confidence:** High.

## Root and label model

```text
consumer expectation
  -> action identity
  -> immutable signed root
       +-- artifact class and schema
       +-- subject digests and sizes
       +-- builder identity and build type
       +-- external parameters and resolved dependencies
       +-- toolchain, target, ABI, profile, features, flags
       +-- execution-cone disposition
       +-- parent root or roots
       +-- validation and failure evidence
       +-- transport blob references

signed mutable label
  -> name + sequence + expiry + root digest
```

The normal consumer sequence is:

1. Compute the expected action and compatibility envelope.
2. Resolve an action entry or policy label to an immutable root.
3. Check anti-rollback, expiration, revocation, signer, builder, build type,
   parameters, dependencies, platform, and validation expectations.
4. Fetch blobs by digest and size.
5. Verify transport integrity before exposing bytes.
6. Stage and atomically install into an isolated mutable consumer directory.
7. Let Cargo or rustc perform its own freshness and compiler-private checks.
8. On any miss or verification failure, quarantine the entry and rebuild
   normally; never reinterpret failure as a hit.

## Recommendations

### Adopt now

- Add immutable root, mutable label, lineage, trust, compatibility, validation,
  retention, and revocation vocabulary to the Build Forest.
- Authorize a read-only local manifest, policy evaluator, and visualization
  prototype.
- Keep action identity separate from content identity and human labels.
- Preserve Cargo artifact sets and rustc incremental generations as distinct
  artifact classes.
- Require isolated staged publication and installation.
- Record net benefit rather than cache-hit rate alone.

### Prototype behind a compatibility boundary

- Exact-identity, same-platform transport of complete rustc generations in
  disposable fixtures.
- Signed archive roots with optional full-tree audit mode.
- Label sequencing, expiry, revocation, pinning, lineage, and reachability GC.
- Cargo shared-cache observability and evaluation fixtures in collaboration
  with upstream Cargo.
- Path-remapping and reproducibility experiments before any cross-machine
  portability claim.

### Reject or defer

- A FERRIUM-hosted general remote cache or remote execution service.
- Automatic artifact restoration in ordinary repositories.
- Raw compiler outputs treated as complete Cargo entries.
- Partial composition of rustc incremental generations.
- Shared writable target or incremental directories.
- Build-script, proc-macro, path-package, native-tool, or unknown
  execution-cone publication.
- Cross-toolchain, cross-target, cross-ABI, or cross-platform reuse by default.
- Labels, signatures, cache hits, or compiler success presented as correctness.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: compiler-private generations remain opaque and atomic; signatures and compilation are not called correctness proof. |
| Compiler Performance Engineer | Accepted with constraint: restore is authorized only when measured verification and transfer preserve net iteration benefit. |
| Interop Boundary Auditor | Accepted with constraint: native, SDK, linker, ABI, debug, signing, and deployment identity must be explicit or the root misses. |
| AI Assurance Skeptic | Accepted: producer assertions, consumer expectations, integrity, trust, and validation remain separate evidence classes with visible failures. |
| Ecosystem Strategist | Accepted: FERRIUM aligns Cargo-unit reuse with Cargo's 2026 cache work and does not duplicate Bazel, Sigstore, TUF, or remote-execution services. |
| Rust Maintainer | Accepted: the first surface is removable read-only explanation; ordinary Cargo and rustc remain the final freshness and compilation authorities. |
| Native Platform Adopter | Accepted with constraint: rollback, revocation, audit, quarantine, retention, and ordinary rebuild recovery are mandatory. |
| Scope Keeper | Accepted: only the PERF-Q30 provenance gate and bounded prototype authority change; production storage and automation remain non-goals. |
| Validation Checker | Accepted: the experiment records environment, commands, repeated timings, exact and mismatch controls, corruption, mix, rollback, expiry, and revocation cases. |

## Prototype gate

A later artifact-bearing prototype requires:

1. one supported Cargo artifact class with complete upstream-compatible
   sidecars and identity;
2. one real public repository and one held-out repository;
3. Linux, Windows, and macOS evidence where the artifact class claims support;
4. path-remap, debug, native, build-script, proc-macro, and environment
   disposition tests;
5. authenticated concurrent publication, interruption, quarantine, revocation,
   retention, and recovery tests;
6. measured service latency and bandwidth, not idealized transfer alone;
7. a removable adoption and rollback contract;
8. upstream Cargo review before any competing artifact layout or protocol.

## Non-goals

- Defining a stable rustc incremental-cache format.
- Claiming signed provenance makes a build reproducible or correct.
- Building remote execution.
- Replacing Cargo, rustc, Bazel REAPI, SLSA, Sigstore, or TUF.
- Shipping production code during the current research pulse.

