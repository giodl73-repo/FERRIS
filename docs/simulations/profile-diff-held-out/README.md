# Profile Diff Held-Out Program

Status: Contract revision 3 candidate-ready for independent re-preflight
Implementation authority: None
Oracle access: Prohibited

## Purpose

This program defines an independently constructed held-out evaluation for the
bounded Pulse 14 `profile-diff` command. It is separate from the historical
FHIF implementation-fixture series and from the Pulse 15 development fixture
matrix.

The public repository contains:

- the [public scoring contract](PUBLIC_CONTRACT.md); and
- the [custody and preflight protocol](CUSTODY_AND_PREFLIGHT.md);
- the [exact identity contract](IDENTITY.md);
- the [three-public-repository workflow](THREE_REPOSITORY_WORKFLOW.md);
- complete [Draft 2020-12 schemas](schemas/README.md); and
- public [synthetic vectors and preflight fixtures](fixtures/README.md).

It contains no hidden profile inputs, privacy canaries, expected records,
expected digests, scorer predicates, acceptance thresholds derived from a
candidate run, or private fixture identifiers.

## Current state

Contract revision 3 repairs the public Stage A byte, human-output, collection,
repository-evidence, and lifecycle-branch blockers reported against revision
2. An independent validation owner may now repeat public scorer preflight
before selecting or sealing any repository package. No independent
re-preflight, repository selection, or pass is claimed. A held-out claim
requires all of the following later records:

1. opaque fixture ID and private manifest revision;
2. sealed input, harness, and oracle package digests;
3. qualified preflight receipt;
4. immutable Ferris commit and tag cutoff;
5. Windows and Unix environment receipts;
6. complete 112-process collection receipt;
7. scorer-conformance receipt;
8. one irreversible first score; and
9. a complete three-public-repository workflow receipt; and
10. a public-safe result that exposes no hidden material.

Until those records exist, Ferris makes no held-out `profile-diff` claim.
