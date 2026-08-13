# Pulse 17 Independent Validation Handoff

Date: 2026-08-13
Contract revision: 3
Disposition: Stage B/C complete; valid implementation failure; program closed

## Independent Stage A result

Stage A passed against immutable cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36` with 789 assertions, 28
LF artifacts, 11 schemas and 69 closed objects, 41 positive records plus two
nested diffs, 38 mutations, 31 vectors and 161 identities, 59 evidence joins,
10 process archetypes, 112 slots, 26 workflow records, exactly 40 branches,
and 48 links. It reported zero public blockers and preserved first-score
integrity.

The resulting exact public repository selection is frozen in
[`REPOSITORY_SELECTION_BINDING.md`](../../simulations/profile-diff-held-out/REPOSITORY_SELECTION_BINDING.md).

## Frozen inputs available to the custodian

- public 56-case matrix and process cardinality;
- custody, preflight, privacy, quarantine, and first-score rules;
- exact public identity derivations and synthetic digest vectors;
- complete Draft 2020-12 output, collection, environment, owner, comparison,
  public repository profile, lifecycle, and immutability schemas;
- exact three-public-repository slot, eligibility, owner-command, sealed
  change, projection, comparison, rollback, removal, cleanup, and threshold
  contract;
- LF-only normative JSON and exact human-output byte fixtures;
- public synthetic preflight fixtures and repository tests covering 10
  collection archetypes, 38 mutations, 41 positive schema instances, and all
  40 mandatory repository disposition/cardinality branches;
- immutable repository history containing all nine controlled families and
  lifecycle evidence;
- Windows and Unix Rust/Cargo 1.95.0 development receipts; and
- three immutable public repository-selection records with full commits,
  license and eligibility evidence, exact command maps, workflow bounds,
  execution policy, comparison policy, and recomputable identities.

## Inputs intentionally unbound

- hidden before/after values and privacy canaries;
- oracle predicates and expected identities;
- executable sealed package;
- independent custodian identity;
- hidden repository changes and changed paths;
- collection environment identities; and
- per-case score evidence.

These fields are unbound because the implementation author is not permitted to
choose or observe them. They remain sealed after program completion.

The first independent Stage B attempt qualified sealed materials but was
invalidated before execution because the curve25519-dalek license digest had
been computed from CRLF-transformed bytes instead of the frozen LF Git blob.
The public binding now records the verified raw-byte digest
`sha256:403c53069750101aeb9df7e15f127056ceaf7e4e92d0b919a1f4c084afd5f1d4`.
The custodian reported zero of 112 scored processes, preserving first-score
integrity.

## Completed Stage B/C result

Independent custody later completed Stage B/C against cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5` with fixture
`P17-R3-D6B553CBC3B1240B673B8190`. Exactly 112 processes were collected on
Windows and Ubuntu 24.04 with zero missing, duplicate, retried, extra, launch
failure, abnormal termination, stream failure, or privacy-hit records.

All three repository workflow slots passed with 28 owner commands and zero
owner, comparison, or lifecycle failures. Cleanup completed. First-score
attempt 1 and scorer attempt 1 produced a valid implementation failure with
the sole public-safe category `process-exit-agreement`.

The immutable
[public-safe result](../../simulations/profile-diff-held-out/PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
contains the permitted aggregate counts and seals. This is not invalid
custody and not a held-out pass. The one-score program is closed; the fixture
and custody artifacts are quarantined and MUST NOT be retried, rescored, or
reused. No hidden material is disclosed.
