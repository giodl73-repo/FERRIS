# Rust Build-State References

Date: 2026-08-10
Status: Complete
Decision: Blueprint should represent immutable Query Forest roots through
typed human references. Branches move with development lineage, tags are
write-once, channels move through policy-controlled promotion, aliases are
local conveniences, and pins retain roots. Leases and tombstones are policy
records; labels are searchable metadata only. No reference is a compatibility
key, integrity proof, or build accelerator by itself.

## Decision supported

This research closes
[BLUE-Q01](questions/blueprint/BLUE-Q01-forest-root-references.md) and defines
the reference and lineage input to APPLICATION-001, FOREST-002, IDENTITY-001,
TRUST-001, and CONFORMANCE-001.

The decision supports these user workflows:

- return to a useful state after branch, worktree, or agent-session changes;
- compare current Cargo and application state with a known validated root;
- promote or roll back validated evidence without rebuilding or copying it;
- retain release and regulated evidence independently of ordinary history;
- revoke future resolution without erasing audit history; and
- fall back to ordinary Cargo operation on every missing, incompatible,
  corrupt, untrusted, stale, or uneconomic reuse candidate.

Implementation authority remains closed.

## Evidence

### Local FERRIS evidence

- [Rust Build Forest opportunity](2026-08-08-rust-build-forest-opportunity.md)
  established immutable roots, lineage, human navigation, and read-only
  visualization above Cargo and rustc.
- [Cross-workspace artifact reuse](2026-08-08-cross-workspace-artifact-reuse.md)
  demonstrated exact registry dependency reuse, incomplete copied artifacts,
  corruption surviving Cargo freshness, and successful wrong reuse from
  unrelated path packages sharing a target directory.
- [CI cache topology](2026-08-08-ci-cache-topology.md) established branch
  scope, writer trust, immutable cache-entry behavior, retention, and
  restore-cost boundaries.
- [Remote artifact provenance](2026-08-09-remote-artifact-provenance.md)
  established separate action and content identities, immutable signed roots,
  sequenced and expiring policy references, anti-rollback, revocation,
  reachability retention, isolated materialization, and correctness-neutral
  rebuild fallback.
- [Impact-aware validation selection](2026-08-09-impact-aware-validation-selection.md)
  keeps validation coverage explicit and prevents promotion names from
  replacing evidence.
- [Query Forest component model](../specs/FOREST_COMPONENT_MODEL.md) defines
  the Forest as a typed evidence model and immutable-root history rather than
  a cache or monolithic service.

### External reference systems

| System | Immutable identity | Human reference | Relevant lesson |
|---|---|---|---|
| Git | commit object ID | branch, tag, worktree-local `HEAD` | shared refs and local heads are distinct; compare-and-set and reflogs prevent silent lost updates |
| OCI | manifest or blob digest | registry tag | promotion can move a tag without changing content; digest verification remains authoritative |
| Nix and Guix | store path | profile generation, channel | generations support rollback; GC roots retain content independently of moving names |
| OSTree | content-addressed commit | ref | atomic deployment and rollback keep immutable commits separate from tracked refs |
| Bazel | action and CAS digest | target label is an identifier, not an output pointer | semantic target naming, action lookup, and content integrity require different namespaces |
| rustup | exact or dated toolchain | stable, beta, nightly channel | channels are intentionally moving streams; repository files provide exact local pinning |
| npm | immutable published package version | distribution tag | `latest`, `beta`, and `canary` are mutable release aliases, not version identity |
| GitHub Actions cache | immutable cache entry after creation | key and restore-prefix lookup | branch scope and lookup hints affect availability but do not prove compatibility or integrity |

Primary references:

- [Git branches](https://git-scm.com/docs/git-branch),
  [tags](https://git-scm.com/docs/git-tag),
  [reflogs](https://git-scm.com/docs/git-reflog),
  [worktrees](https://git-scm.com/docs/git-worktree), and
  [atomic ref updates](https://git-scm.com/docs/git-update-ref);
- [Cargo build cache](https://doc.rust-lang.org/cargo/reference/build-cache.html),
  [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html), and
  [unstable build directory](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-dir);
- [rustup channels](https://rust-lang.github.io/rustup/concepts/channels.html)
  and [overrides](https://rust-lang.github.io/rustup/overrides.html);
- [OCI distribution](https://github.com/opencontainers/distribution-spec/blob/main/spec.md)
  and [descriptor](https://github.com/opencontainers/image-spec/blob/main/descriptor.md);
- [Nix profiles](https://github.com/NixOS/nix/blob/master/doc/manual/source/package-management/profiles.md)
  and
  [garbage-collector roots](https://github.com/NixOS/nix/blob/master/doc/manual/source/package-management/garbage-collector-roots.md);
- [Guix package generations](https://guix.gnu.org/manual/en/html_node/Invoking-guix-package.html)
  and [garbage collection](https://guix.gnu.org/manual/en/html_node/Invoking-guix-gc.html);
- [OSTree introduction](https://github.com/ostreedev/ostree/blob/main/docs/introduction.md)
  and
  [atomic upgrades](https://github.com/ostreedev/ostree/blob/main/docs/atomic-upgrades.md);
- [Bazel labels](https://bazel.build/concepts/labels) and
  [remote caching](https://bazel.build/remote/caching);
- [GitHub Actions dependency caching](https://docs.github.com/actions/reference/workflows-and-actions/dependency-caching);
  and
- [npm distribution tags](https://docs.npmjs.com/cli/latest/commands/npm-dist-tag).

Current Cargo reports reinforce the local controls:

- [cargo#12516](https://github.com/rust-lang/cargo/issues/12516) documents
  cross-workspace target-directory cache poisoning;
- [cargo#14053](https://github.com/rust-lang/cargo/issues/14053) reports
  concurrent shared-target artifact races;
- [cargo#16804](https://github.com/rust-lang/cargo/issues/16804) states the
  user tradeoff between shared-cache races and isolated-target storage growth;
- [cargo#16642](https://github.com/rust-lang/cargo/issues/16642) and
  [cargo#16911](https://github.com/rust-lang/cargo/issues/16911) show continuing
  worktree, path, and stale-state edge cases.

The detailed comparison is retained in
[EXP-01 reference-model comparison](blue-q01-forest-refs/results/EXP-01-reference-model-comparison.md).

## Selected model

### Identity and name kinds

| Term | Normative meaning |
|---|---|
| root | immutable canonical Query Forest manifest identified by digest |
| ref | umbrella term for a typed name that resolves to one root |
| branch | moving development ref constrained by recorded lineage |
| tag | write-once published ref to one root |
| channel | policy-controlled moving promotion ref |
| alias | local convenience ref with no support or trust meaning |
| pin | retention-only ref that prevents collection |
| lease | expiring active-use retention claim, not a selection ref |
| tombstone | policy record denying future resolution while preserving history |
| label | non-dereferenceable searchable metadata |
| generation | monotonically ordered ref-update event, not a separate ref kind |

Only roots establish immutable object identity. Ref resolution yields a root
candidate; compatibility, trust, validation, integrity, availability, and
economics are checked separately.

### Root contents

A root manifest binds or references:

- application-definition and normalized Blueprint-model identity;
- source snapshot and Cargo lock identity;
- Cargo packages, units, targets, features, profiles, and dependency closure;
- toolchain, target, platform, ABI, native, environment, and execution inputs;
- artifact and complete opaque incremental-generation descriptors where
  separately eligible;
- validation plans, outcomes, confidence, failures, and unknowns;
- provenance, producer, signatures, policy, expiry, and limitations; and
- parent root or roots.

The live Cargo `target` or experimental build directory is mutable scratch and
is not the canonical named root.

### Update rules

1. Every mutable ref update supplies the expected prior root and fails on a
   mismatch.
2. Coupled ref moves use one atomic transaction.
3. Every successful create, move, rollback, expiry, deletion, and tombstone
   appends a durable generation event.
4. Tags are write-once. Correction creates a new tag and may tombstone the old
   one; it never silently retargets it.
5. Branches may move or be force-updated with explicit authority and history.
   They do not assert release support.
6. Channels move only when named policy, validation, provenance, trust, and
   expiry gates accept the target root.
7. Aliases are local and best-effort. They cannot satisfy support, promotion,
   or retention requirements.
8. Pins and leases affect retention only. They cannot make a root compatible,
   trusted, validated, or selected.
9. Tombstones deny future resolution or reuse of a root, ref, signer, or
   producer while retaining prior events and evidence.
10. Labels never dereference and never participate in compatibility or
    correctness decisions.

A disposable Git control verified the core update behavior:

```console
git update-ref refs/blueprint/channels/stable <new> <expected-old>
```

The expected-value update succeeded, a stale expected value failed with exit
128, the reflog retained both generations, and an expected-value rollback
restored the prior root. This validates the primitive, not a Blueprint storage
format.

## Git and worktree relationship

Blueprint should derive source context from Git rather than require users to
administer duplicate source branches.

- A Blueprint branch may record its associated Git ref and source commit, but
  its identity remains the Query Forest root.
- Worktree-local current state is a session binding, analogous to Git's
  worktree-local `HEAD`; it is not a shared Blueprint branch.
- Multiple worktrees use isolated writable Cargo build state.
- A Git branch move does not move a Blueprint ref automatically.
- Observation may propose a new branch root. Mutation requires an explicit
  approved ref-update action.
- Detached heads, dirty trees, generated inputs, and untracked inputs remain
  explicit source states rather than being collapsed into a branch name.

## Promotion and rollback

Promotion changes a ref, not the root:

```text
validated immutable root R
  -> create tag/validated/<policy>/<time> -> R
  -> compare-and-set channel/candidate   -> R
  -> compare-and-set channel/stable      -> R
```

Rollback moves a channel to an earlier accepted root through the same checks
and records a new generation. It does not erase the bad promotion or declare
the older artifacts usable on a changed consumer without compatibility
verification.

## Rebuild and reuse behavior

References improve discovery, comparison, planning, promotion, retention, and
audit. They do not make compilation faster alone.

Avoided work is available only when a separately eligible root contains or
references compatible material and:

```text
expected rebuild cost
  > lookup + verification + transfer + extraction + materialization
    + contention + miss risk
```

Any absent, stale, expired, replayed, revoked, corrupt, incompatible, unknown,
or uneconomic candidate falls back to ordinary isolated Cargo operation.
Cargo and rustc retain their own freshness and compiler-private checks.

## Candidate command surface

These examples are inputs to specification work, not an implemented CLI:

```console
cargo blueprint roots list
cargo blueprint roots show <digest>
cargo blueprint compare --to channel/main-green
cargo blueprint refs list
cargo blueprint refs history channel/stable
cargo blueprint branch set feature/payments --root <digest> --expect <old>
cargo blueprint tag create release/2.1 --root <digest>
cargo blueprint channel promote stable --root <digest> --expect <old>
cargo blueprint channel rollback stable --to <prior-generation>
cargo blueprint pin create audit/2026-q3 --root <digest>
cargo blueprint pin release audit/2026-q3
cargo blueprint lease renew session/<id> --ttl 8h
cargo blueprint policy tombstone root <digest> --reason <record>
```

Read-only inspection and comparison remain the first prototype boundary.
Metadata mutation requires EXECUTION-001 action approval. Artifact
materialization remains separately gated.

## Recommendations

### Adopt now

- Standardize the selected vocabulary and type distinctions.
- Make immutable roots, typed refs, lineage, update generations, expected-value
  updates, and reachability part of the Blueprint specifications.
- Keep Git source refs, Blueprint build-state refs, action identities, content
  digests, validation evidence, and human labels separate.
- Define ordinary isolated Cargo rebuild as the universal safe fallback.
- Use tags for fixed publication and channels for moving promotion.

### Prototype behind a compatibility boundary

- Local read-only root and ref manifests.
- Root/ref/history/lineage visualization.
- Current-state comparison with a branch, tag, or channel.
- Retention and collection simulation from pins, leases, accepted refs, and
  policy.
- Update transaction fixtures without production artifact restoration.

### Propose upstream

- Stable Cargo-owned artifact manifests and self-contained artifact
  directories.
- Supported cross-workspace cache identity and integrity contracts.
- Better branch, worktree, cache-miss, and invalidation diagnostics.
- Generic atomic storage, GC, and provenance primitives where Cargo or shared
  standards are the proper owner.

### Reject or defer

- shared writable target directories across unrelated workspaces or worktrees;
- a Blueprint branch that duplicates Git source-control administration;
- mutable tags;
- refs, labels, cache keys, signatures, or producer identity as correctness;
- pinning as selection or trust;
- retention by age alone;
- deletion as the only revocation mechanism;
- mutation of rustc-private cache files;
- automatic remote restoration; and
- implementation before specifications, held-out benefit, and conformance.

## Findings

### FERRIS-705: immutable roots and mutable names are separate namespaces

**Sources:** Git, OCI, Nix, Guix, OSTree, Bazel, rustup, npm, and local
FERRIS PERF-Q05, PERF-Q06, and PERF-Q30 evidence.

**Observed behavior:** Mature systems name exact content independently from
human-facing development, release, promotion, or retention references.

**Implication:** Blueprint roots are immutable identities; refs only resolve
names to root candidates.

**Confidence:** High.

### FERRIS-706: Cargo build state is scratch, not a named-root contract

**Sources:** Cargo build-cache and build-directory documentation; PERF-Q05 and
PERF-Q18.

**Observed behavior:** Cargo freshness and compiler state use mutable internal
layouts, fingerprints, timestamps, and compiler-private generations.

**Implication:** Blueprint must model roots above Cargo without naming a live
target directory as the canonical object.

**Confidence:** High.

### FERRIS-707: shared writable targets cannot implement branch reuse safely

**Sources:** PERF-Q05; cargo#12516, cargo#14053, cargo#16804.

**Observed behavior:** unrelated path packages can produce successful wrong
reuse, while concurrent producers can collide or overwrite state.

**Implication:** worktrees and branches require isolated writable state;
cross-workspace reuse needs an explicit immutable compatibility boundary.

**Confidence:** High.

### FERRIS-708: one free-form label cannot represent all policies

**Sources:** Git tags and branches; rustup channels; npm dist-tags; Nix GC
roots; Bazel labels.

**Observed behavior:** moving development, fixed publication, promotion,
convenience, retention, and metadata have different mutation and authority
rules.

**Implication:** Blueprint needs typed refs and must reserve `label` for
non-dereferenceable metadata.

**Confidence:** High.

### FERRIS-709: Blueprint should derive Git context without duplicating Git

**Sources:** Git worktree and ref documentation; Cargo worktree issue evidence.

**Observed behavior:** Git shares ordinary refs while keeping current heads and
selected pseudo-refs worktree-local.

**Implication:** Blueprint records Git association and source identity but
maintains its own explicit root refs only for build and evidence state.

**Confidence:** High.

### FERRIS-710: moving refs require expected-value updates and durable history

**Sources:** Git update-ref and reflog documentation; disposable compare-and-set
control; PERF-Q30 anti-rollback experiment.

**Observed behavior:** expected-value updates reject stale writers, atomic
transactions coordinate coupled moves, and history enables audit and rollback.

**Implication:** every Blueprint ref mutation needs compare-and-set,
authorization, generation ordering, and an append-only event.

**Confidence:** High.

### FERRIS-711: promotion and rollback move refs, not roots

**Sources:** OCI tags, rustup channels, npm dist-tags, Nix generations, OSTree
deployments, and PERF-Q30.

**Observed behavior:** release streams and deployments advance or roll back by
changing a pointer to immutable content.

**Implication:** Blueprint promotion must not rebuild, copy, or mutate the
selected root.

**Confidence:** High.

### FERRIS-712: retention follows reachability and explicit policy

**Sources:** Nix and Guix GC roots; PERF-Q06 and PERF-Q30.

**Observed behavior:** moving references, fixed pins, active use, quotas, and
policy determine what remains reachable; age alone cannot preserve required
evidence.

**Implication:** collection begins from accepted refs, pins, active leases, and
policy, then traverses immutable root content.

**Confidence:** High.

### FERRIS-713: revocation is a future-resolution decision

**Sources:** PERF-Q30; The Update Framework and Sigstore models cited there.

**Observed behavior:** revoked trust does not erase historical signatures,
events, or content identity.

**Implication:** tombstones deny future resolution or use while audit history
remains; physical deletion is a separate retention action.

**Confidence:** High.

### FERRIS-714: refs accelerate decisions before they accelerate builds

**Sources:** PERF-Q05, PERF-Q06, PERF-Q30, and the reference-system comparison.

**Observed behavior:** names make useful states discoverable and promotable,
but avoided compilation requires separate compatible materialization whose
cost is lower than rebuilding.

**Implication:** the first product value is compare, explain, navigate,
promote, retain, and roll back; artifact restoration remains separately gated.

**Confidence:** High.

### FERRIS-715: the read-only reference model is ready for specification

**Sources:** FERRIS-705 through FERRIS-714 and the role review below.

**Observed behavior:** vocabulary, authority, failure fallback, and unsafe
boundaries converge without requiring a cache service or compiler mutation.

**Implication:** APPLICATION-001, FOREST-002, IDENTITY-001, TRUST-001, and
CONFORMANCE-001 may adopt the model; implementation authority remains closed.

**Confidence:** High.

## Nine-role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: no ref, signature, or cache hit proves correctness; ordinary isolated rebuild remains mandatory fallback. |
| Compiler Performance Engineer | Accepted: refs expose lineage and avoided-work candidates but do not claim speed without measured materialization economics. |
| Interop Boundary Auditor | Accepted: source, Cargo unit, toolchain, ABI, native, artifact, validation, action, content, and ref identities remain separate. |
| AI Assurance Skeptic | Accepted: stale, replayed, revoked, corrupt, missing, unknown, and uneconomic controls remain visible. |
| Ecosystem Strategist | Accepted: Blueprint uses established primitives and routes Cargo-owned cache and artifact contracts upstream. |
| Rust Maintainer | Accepted: Git and Cargo stay authoritative; no duplicate source branch administration or required workflow change is introduced. |
| Native Platform Adopter | Accepted: platform and ABI envelopes remain explicit, with isolated state and rollback on Windows and Unix. |
| Scope Keeper | Accepted: read-only manifests and comparison are in scope; cache service, deployment, and automatic restoration remain closed. |
| Validation Checker | Accepted: expected-value conflicts, fixed-tag mutation, rollback, revocation, retention, missing content, and removal require conformance fixtures. |

## Limitations

- No production Blueprint schema or command surface exists.
- The disposable ref control validates Git's atomic primitive, not a complete
  authorization, signing, storage, or distributed-consensus design.
- User benefit still needs held-out workflow timing and comprehension tests.
- Cargo layout, cache, checksum-freshness, and cross-workspace work continue to
  evolve.
- General artifact restoration, path-independent identity, execution-cone
  capture, and cross-platform reuse remain unresolved.
