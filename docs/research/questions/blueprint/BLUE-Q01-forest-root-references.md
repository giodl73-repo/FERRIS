# BLUE-Q01: Query Forest Root References

**Status:** Complete

**Decision:** Use immutable Query Forest roots with typed branches, write-once
tags, policy-controlled channels, local aliases, and retention pins. Leases
and tombstones are policy records; labels are metadata only. Ref updates use
expected prior values and durable generations. Refs do not establish
compatibility, integrity, trust, validation, or build reuse.

**Outputs:**

- [Rust build-state references](../../2026-08-10-rust-build-state-references.md)
- [EXP-01 reference-model comparison](../../blue-q01-forest-refs/results/EXP-01-reference-model-comparison.md)

## Research question

How should Blueprint name, resolve, update, retain, compare, promote, share, and
revoke human-readable references over immutable Query Forest roots so Rust
users can resume work, switch branches and worktrees, reuse compatible build
state, promote validated results, and roll back quickly without treating a
name or cache hit as correctness authority?

## Decision informed

Define the reference and lineage portion of APPLICATION-001, IDENTITY-001,
FOREST-002, TRUST-001, and CONFORMANCE-001.

The decision must determine:

- whether Blueprint needs roots, refs, branches, tags, channels, aliases,
  leases, pins, generations, or a smaller vocabulary;
- which references may move and under what compare-and-set policy;
- how Git source refs relate to Blueprint refs without creating duplicate
  branch administration;
- how refs affect navigation, comparison, reuse eligibility, validation,
  promotion, retention, garbage collection, revocation, and rollback;
- which operations are read-only, which mutate only Blueprint metadata, and
  which remain prohibited;
- what evidence would justify future artifact restoration; and
- which generic primitives should be proposed upstream rather than owned by
  Blueprint.

## Dependencies

- PERF-Q05 cross-workspace artifact reuse;
- PERF-Q06 CI cache topology;
- PERF-Q07 editor and Cargo contention;
- PERF-Q18 incremental-cache behavior and recovery;
- PERF-Q30 remote artifact provenance;
- PERF-Q35 impact-aware validation;
- ECOS-Q11 compatibility-profile renewal and rollback;
- FOREST-001 Query Forest component model; and
- the Blueprint Cargo Application Model.

## User workflows to test

1. Switch from `main` to a feature branch and back without paying avoidable
   dependency or validation cost.
2. Use two Git worktrees concurrently without sharing unsafe writable target
   state.
3. Resume the last validated local state after an interrupted edit or failed
   build.
4. Compare the current application definition and Cargo graph with
   `main-green`.
5. Promote one validated root from candidate to stable without copying or
   rewriting the root.
6. Pin a release or regulated evidence state while allowing ordinary branch
   histories to advance.
7. Roll back a moving reference after a bad promotion without deleting audit
   history.
8. Revoke future trust in a root, producer, or signer while retaining historical
   evidence.
9. Expire and collect unreachable build state without deleting a root retained
   by another tag, channel, lease, or policy.
10. Rebuild normally when a referenced root is absent, incompatible, stale,
    corrupt, untrusted, or uneconomic to restore.

## Starting hypotheses

1. Rust users need Git-like branches and tags for build state.
2. A generic free-form label is sufficient for every workflow.
3. Git branches can be reused directly as Blueprint branches.
4. A human-readable ref can also serve as a cache lookup key.
5. Moving `main-green` should make its target root trusted automatically.
6. Refs can accelerate builds without artifact restoration.
7. Retention can be implemented by age alone.
8. Ref updates do not require sequence, expiry, signature, or compare-and-set
   controls.
9. Branch switching should share complete target directories.
10. A remote hit is beneficial whenever compatible content exists.

The research must preserve negative results rather than forcing this
vocabulary to survive.

## Evidence and comparisons

Review and compare:

- Git refs, branches, tags, reflogs, worktrees, and atomic updates;
- Cargo target directories, fingerprints, branch/worktree behavior, locking,
  cleaning, and upstream cross-workspace cache work;
- CI cache keys, branch scopes, restore keys, immutability, and retention;
- OCI tags and digests;
- Nix and Guix profiles, generations, store paths, and garbage-collection
  roots;
- OSTree refs;
- Bazel labels, action identities, CAS digests, and remote cache;
- rustup channels;
- package-manager distribution tags;
- deployment and model-registry promotion aliases; and
- TUF/Sigstore/SLSA trust, expiration, rollback, and revocation controls.

## Required controls

- immutable roots with equal and unequal action identities;
- mutable ref update with expected prior value;
- concurrent writers;
- fixed tag mutation attempt;
- channel promotion and rollback;
- expired, replayed, revoked, corrupt, and missing refs;
- two Git branches and two worktrees;
- exact, compatible, incompatible, and unknown build states;
- retained and unreachable roots;
- restore-benefit positive and negative economics; and
- complete removal followed by ordinary Cargo operation.

## Candidate vocabulary

The starting vocabulary is:

| Term | Candidate meaning |
|---|---|
| root | immutable canonical Query Forest result |
| ref | umbrella name-to-root pointer |
| branch | moving ref constrained to an evolving lineage |
| tag | fixed human-readable ref |
| channel | policy-controlled promotion ref |
| alias | local convenience ref without support meaning |
| pin | retention decision preserving a root |
| lease | expiring active-use retention claim |
| tombstone | future-resolution denial retaining audit history |
| label | searchable metadata only, not a pointer |

The research may remove, rename, or split these terms.

## Measurements

- time to identify and return to the last useful state;
- clean, warm, restored, and rebuild-after-ref timings;
- dependency, workspace, link, and validation work avoided;
- bytes retained, restored, copied, and collected;
- lookup, verification, materialization, and fallback cost;
- false reuse, stale state, hidden-input, and compatibility failures;
- concurrent update conflicts and lost-update prevention;
- operator steps for compare, promote, pin, roll back, revoke, and remove; and
- maintainer comprehension without internal compiler vocabulary.

## Adopt-now boundary

The question may standardize product-neutral reference vocabulary, immutable
root identity, lineage, read-only comparison, retention reachability, and
policy evaluation.

## Prototype boundary

A future prototype may create a local read-only root/ref manifest and
visualization. Metadata-only ref updates require a separate approved action
contract. Artifact restoration remains separately gated.

## Reject or defer

- shared writable target directories across unrelated workspaces;
- refs as semantic compatibility or cache keys;
- labels, signatures, or trusted producers as correctness proof;
- direct mutation or composition of rustc-private cache files;
- automatic remote restoration;
- hidden branch or channel updates;
- retention by age without reachability, leases, pins, and policy;
- deletion as the only revocation mechanism;
- replacing Git source control, Cargo freshness, or upstream cache work; and
- implementation before held-out user benefit and nine-role review.

## Role gates

All nine FERRIS roles must review:

- safety and cache-poisoning boundaries;
- performance and net-benefit evidence;
- source, build, artifact, platform, and Typebook identity separation;
- trust, replay, revocation, privacy, and authority;
- Cargo and upstream ownership;
- branch/worktree usability and removal;
- cross-platform behavior;
- scope and naming convergence; and
- executable positive, negative, failure, stale, rollback, and removal tests.

## Expected outputs

- cited decision note;
- reference-model comparison matrix;
- selected vocabulary and state machine;
- user-workflow and command examples;
- retention, promotion, rollback, and revocation rules;
- measured benefit and failure controls;
- specification consequences; and
- adopt, prototype, upstream, reject, and defer dispositions.
