# Pulse 06: Hosted Service Profile Family

Status: Complete
Implementation authority: Bounded to this document

## Goal and authority

Complete one zero-dependency hosted-service family with an in-process request
handler rather than opening a network listener.

Revision `r1` serves one health operation with a fixed successful response.
Revision `r2` adds an explicit readiness state and returns a typed unavailable
response until the owner marks the service ready.

This pulse authorizes:

- two exact local service-library revisions and lockfiles;
- positive, malformed-request, unavailable, cancellation, and state-transition
  tests;
- reusable test-only v1 profile materialization and exact digests;
- locked/offline owner Cargo stages in isolated target directories;
- explicit runtime, service, data, deployment, and operational states;
- source-tree immutability;
- Windows and Unix development validation; and
- the bounded nine-role authorization review.

It does not authorize sockets, remote clients, credentials, TLS, databases,
deployment, production operation, product profile generation, another family,
support, approval, or held-out access.

## Acceptance

- `r1` and `r2` retain exact and distinct service contracts;
- readiness unavailable is never promoted to pass;
- malformed requests and cancellation are deterministic;
- no network access or durable service state occurs;
- owner metadata, check, build, Clippy, tests, doctest, and package pass;
- consumer trees remain unchanged;
- profiles contain all 15 stage states and stable distinct digests;
- Windows and Unix use Rust/Cargo 1.95.0; and
- all nine roles accept the measured result.

## Stop conditions

Stop if work requires a listener, external runtime, database, TLS, credential,
deployment system, network, unsafe code, production command, or broader claim.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-06-HOSTED-SERVICE-ROLE-REVIEW.md)
- [Windows and Unix validation](../../../../docs/plans/validation/PULSE-06-HOSTED-SERVICE-FAMILY.md)

Implementation cutoff:
`de5b5242a26ed5ce15d1dae2d3ec333a3a7663d2`.

The two exact source trees and canonical profiles retain distinct digests.
Windows build 26310 and Ubuntu 24.04.4 WSL2 passed the full Rust/Cargo 1.95.0
gates with 70 passing tests, 2 ignored bounded-command helpers, and no
failures. Readiness unavailable remained non-success, consumer trees remained
unchanged, and no listener, network, durable state, or deployment system was
introduced.
