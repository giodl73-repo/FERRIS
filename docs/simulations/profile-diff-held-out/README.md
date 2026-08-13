# Profile Diff Held-Out Program

Status: Closed; Stage B/C completed with a valid implementation failure
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
- the [immutable public-safe Stage B/C result](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
  and its
  [machine-readable companion](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.json);
- complete [Draft 2020-12 schemas](schemas/README.md); and
- public [synthetic vectors and preflight fixtures](fixtures/README.md).

Prospective programs may also opt into the
[post-score diagnostic release protocol](POST_SCORE_DIAGNOSTIC_RELEASE.md).
[Why quarantine exists](WHY_QUARANTINE.md) explains the certification and
debugging tradeoff. Neither document changes this closed program.

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
[`repository-selections/`](repository-selections/) by that result.

Independent Stage B/C then completed against immutable cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5` with opaque fixture
`P17-R3-D6B553CBC3B1240B673B8190`. Exactly 112 processes were collected
without collection-integrity or privacy failure, and all three repository
workflow slots passed. The valid first score failed only in the public-safe
category `process-exit-agreement`. This is a valid implementation failure,
not invalid custody, and it is not a held-out pass.

The [public-safe result](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
contains only permitted aggregates. The fixture is sealed in quarantine. The
one-score program is closed: retry, rescore, and reuse are prohibited.
