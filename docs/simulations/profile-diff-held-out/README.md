# Profile Diff Held-Out Program

Status: Stage A passed; repository selections frozen; execution unperformed
Implementation authority: None
Oracle access: Prohibited

## Purpose

This program defines an independently constructed held-out evaluation for the
bounded Pulse 14 `profile-diff` command. It is separate from the historical
FHIF implementation-fixture series and from the Pulse 15 development fixture
matrix.

The public repository contains:

- the [public scoring contract](PUBLIC_CONTRACT.md);
- the [custody and preflight protocol](CUSTODY_AND_PREFLIGHT.md);
- the [exact identity contract](IDENTITY.md);
- the [three-public-repository workflow](THREE_REPOSITORY_WORKFLOW.md);
- the [public repository-selection binding](REPOSITORY_SELECTION_BINDING.md);
- complete [Draft 2020-12 schemas](schemas/README.md); and
- public [synthetic vectors and preflight fixtures](fixtures/README.md).

It contains no hidden profile inputs, privacy canaries, expected records,
expected digests, scorer predicates, acceptance thresholds derived from a
candidate run, or private fixture identifiers.

## Current state

Independent Stage A passed against immutable cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36`: 789 assertions, 28 LF
artifacts, 11 schemas and 69 closed objects, 41 positive records plus two
nested diffs, 38 mutations, 31 vectors and 161 identities, 59 evidence joins,
10 process archetypes, 112 slots, 26 workflow records, exactly 40 branches,
and 48 links. It reported no public blocker and preserved first-score
integrity.

The three public repository selections are now frozen in
[`repository-selections/`](repository-selections/) by the independent Stage A
result. No hidden change, sealed package, owner-command workflow, scored
process, oracle release, or held-out pass is claimed. A held-out claim still
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
