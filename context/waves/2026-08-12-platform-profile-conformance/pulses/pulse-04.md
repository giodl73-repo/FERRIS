# Pulse 04: Pure Data Profile Family

Status: Complete; Windows and Unix development gates passed
Implementation authority: Bounded to this document

## Goal and authority

Complete the first exact PLATFORM-001 family using two controlled,
zero-dependency pure-data consumers and owner-native Cargo evidence.

The consumer operation is:

> Normalize one ASCII record key or return a typed owner error.

Revision `r1` trims leading and trailing ASCII whitespace, lowercases ASCII
letters, and rejects empty, non-ASCII, or internally spaced keys. Revision
`r2` is a material contract change that accepts internal ASCII whitespace and
collapses each run to `-`.

This pulse authorizes:

- `r1` and `r2` local Rust library fixtures with exact lockfiles;
- deterministic positive and expected-rejection owner tests;
- a compact pure-data family manifest;
- test-only materialization of complete v1 profile values from the frozen
  schema exemplar and family manifest;
- exact canonical profile digests asserted in tests;
- isolated locked/offline Cargo metadata, check, build, Clippy, unit-test,
  doctest, and package commands;
- explicit unsupported or not-observed states for inapplicable stages;
- source-tree immutability around every owner command;
- Windows and Unix development validation; and
- one family-specific nine-role review.

The test-only materializer is fixture construction, not a product profile
generator. It emits no durable profile during normal tests.

## Required profile evidence

Each revision must retain:

- consumer, operation, owner, profile, source, manifest, lock, and package
  identity;
- lock universe and target-active normal closure;
- requested and effective features;
- exact Cargo, rustc, toolchain, host, and target observations;
- all required stage kinds with pass, expected-rejection, unsupported, or
  not-observed state;
- assurance, stewardship, support, expiry, limitations, and source
  attribution;
- planned adoption, renewal, substitution, emergency, rollback, and removal
  controls; and
- no production, security, support, certification, or approval claim.

## Acceptance

- both consumer lockfiles are exact and have no external dependency;
- owner metadata identifies exactly one package and the expected revision;
- owner check, build, Clippy, unit test, doctest, and package stages pass;
- `r1` rejects internal whitespace and `r2` accepts and normalizes it;
- invalid empty and non-ASCII inputs remain expected rejections;
- source and lock trees remain byte-identical after every owner command;
- materialized profiles satisfy the bounded v1 harness policy;
- materialized canonical digests are stable and distinct;
- lock and active closures remain separate;
- Windows and Unix use Rust/Cargo 1.95.0;
- full repository gates pass; and
- all nine roles accept the measured family boundary.

## Stop conditions

Stop rather than widening this pulse if work requires:

- an external crate, registry update, build script, macro, unsafe, native,
  provider, runtime, target, service, deployment, credential, or network;
- production profile parsing or generation;
- changing the existing profile-diff command;
- consumer approval, support, compatibility, security, or correctness claims;
- treating the pure-data family as another required family; or
- accessing hidden held-out material.

## Evidence

- [Validation receipt](../../../../docs/plans/validation/PULSE-04-PURE-DATA-FAMILY.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-04-PURE-DATA-ROLE-REVIEW.md)
