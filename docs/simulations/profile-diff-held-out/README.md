# Profile Diff Held-Out Program

Status: Public contract complete; no executable fixture bound
Implementation authority: None
Oracle access: Prohibited

## Purpose

This program defines an independently constructed held-out evaluation for the
bounded Pulse 14 `profile-diff` command. It is separate from the historical
FHIF implementation-fixture series and from the Pulse 15 development fixture
matrix.

The public repository contains only:

- the [public scoring contract](PUBLIC_CONTRACT.md); and
- the [custody and preflight protocol](CUSTODY_AND_PREFLIGHT.md).

It contains no hidden profile inputs, privacy canaries, expected records,
expected digests, scorer predicates, acceptance thresholds derived from a
candidate run, or private fixture identifiers.

## Current state

An independent validation owner may now construct and seal a candidate
fixture package from this public contract. A held-out claim requires all of
the following later records:

1. opaque fixture ID and private manifest revision;
2. sealed input, harness, and oracle package digests;
3. qualified preflight receipt;
4. immutable Ferris commit and tag cutoff;
5. Windows and Unix environment receipts;
6. complete 112-process collection receipt;
7. scorer-conformance receipt;
8. one irreversible first score; and
9. a public-safe result that exposes no hidden material.

Until those records exist, Ferris makes no held-out `profile-diff` claim.
