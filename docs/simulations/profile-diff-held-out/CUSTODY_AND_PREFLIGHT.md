# Profile Diff Held-Out Custody and Preflight

Status: Closed after valid Stage B/C implementation failure
Contract revision: 3

## Stage A disposition

Independent Stage A passed against cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36` with zero public blockers and
first-score integrity preserved. The resulting public repository selection
is frozen in
[`REPOSITORY_SELECTION_BINDING.md`](REPOSITORY_SELECTION_BINDING.md).

This disposition qualifies only the public contract and selection binding.
It does not construct hidden material, qualify the sealed scorer, execute an
owner workflow, open an oracle, or produce a held-out result.

## Stage B/C final disposition

Independent Stage B/C completed against cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5` with fixture
`P17-R3-D6B553CBC3B1240B673B8190`. Exactly 112 processes were collected on
Windows and Ubuntu 24.04 without collection-integrity or privacy failure. The
three repository workflow slots passed. The valid first score failed only in
the public-safe category `process-exit-agreement`.

This is a valid implementation failure, not invalid custody. The immutable
[public-safe result](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
is the only released score record. Quarantine is sealed, cleanup is complete,
and retry, rescore, and reuse are prohibited.

## Independent construction

The validation owner constructing the candidate package MUST NOT:

- read Ferris implementation source or tests;
- use the Pulse 15 development fixture values as hidden inputs;
- receive prior private profile-diff fixture attempts;
- run Ferris while selecting, tuning, or debugging hidden cases;
- disclose hidden inputs or oracle predicates to implementation authors; or
- change a case after observing a Ferris result.

The owner MAY read public product documentation, Pulse 14, the experimental
input schema, the public held-out contract, and ordinary JSON and RFC 6901
standards.

The owner MUST create three separately digested artifacts:

1. input and harness package;
2. expected-record and predicate oracle; and
3. scorer implementation and qualification evidence.

The opaque fixture ID is assigned only after all three artifacts are frozen.

## Preflight before freeze

The harness MUST pass a public synthetic preflight that is not part of the
held-out package. Preflight must prove that it:

- launches the exact binary supplied at runtime;
- preserves nonzero process exits without aborting collection;
- captures complete stdout and stderr separately as bytes;
- durably distinguishes launch failure, timeout, stdout/stderr output bounds,
  each stream-reader failure, zero exit, and nonzero exit;
- records exactly one durable row per expected process;
- rejects missing, duplicate, retried, and extra rows;
- parses a complete JSON stream rather than a prefix or selected line;
- accepts the documented terminal newline and rejects trailing content;
- records command, binary, environment, and output digests;
- carries expected process declarations into durable collection records; and
- completes all 112 declared process slots before scoring can begin.

The preflight MUST validate receipt instances against the published Draft
2020-12 schemas, reject every public mutation vector, recompute every public
identity vector from source values, and compare selected-versus-full
repository workflow vectors. A schema parser that ignores
`additionalProperties:false`, nullability, enum values, `required`, complete
stream parsing, or conditional exit relationships is not qualified.

Preflight output and inputs are development infrastructure evidence. They MUST
NOT be reused as scored cases.

## Scorer qualification

Before fixture freeze, the scorer MUST be tested against synthetic records
covering:

- every required result class and exit;
- stdout-only, stderr-only, both-stream, and empty-stream branches;
- malformed, truncated, and trailing-content JSON;
- missing, duplicate, and unexpected fields;
- allowed record ordering versus required sorted arrays;
- all three change kinds;
- arrays, empty containers, and RFC 6901 escaping;
- exact-bound and overflow behavior;
- path relocation and object-key reordering identities;
- pre-read and second-input failure identity distinctions;
- canary detection in every output location;
- absent and extra process rows; and
- pass, fail, invalid, unsupported, and blocked final dispositions.

Qualification MUST use contract-equivalent field layouts. A parser or scorer
that recognizes only one incidental serialization layout is invalid.

The public qualification set is in [`fixtures/`](fixtures/). It includes no
hidden input, oracle predicate, canary, selected repository, or source change.

## Collection acceptance

Before oracle release, the custodian MUST:

1. validate every row and environment receipt against the published schemas;
2. require exactly one row for every `(case, platform)` declaration;
3. require `attempt: 1` for all rows and reject retries even if an earlier row
   is discarded;
4. reject a duplicate row identity, command identity, or declaration key;
5. recompute executable, command, environment, stdout, stderr, row, and
   aggregate digests;
6. verify complete retained byte streams and byte counts;
7. verify JSON stream selection, terminal LF, schema, typed exit, and actual
   exit agreement;
8. verify human stream selection and exact public grammar;
9. verify zero hidden canary occurrences over both complete streams; and
10. seal the collection and environment receipts before opening the oracle.

Launch failure, timeout, and output-bound termination have a null process exit.
Ordinary process completion records `process_result:success` only with exit 0
and `process_result:failure` only with exit 1 through 255. Each stream records
`read_status` independently with exact nullable read-error fields. Launch
failure marks both readers `not-attempted`; an output-bound row identifies the
bounded stream. An empty stream has byte count zero and the SHA-256 digest of
zero bytes; it is not represented by a null digest.

## Repository workflow preflight

The scorer MUST validate all six repository receipt layouts plus the public
profile layout before hidden change construction: selection, owner command,
check inventory, selected-versus-full comparison, lifecycle, immutability,
and `ferris.public-repository-profile/v1`.

- one selection and one check inventory for each of the three distinct slots;
- exact owner-row counts for all seven phases;
- one sealed source-file change per slot;
- owner nonzero exits retained without aborting later row collection;
- dirtiness, unexpected mutation, rollback mismatch, removal residue, cache
  sharing, network attempts, output overflow, timeout, omission, promotion,
  privacy leakage, and prohibited-conclusion classification;
- pass, fail, invalid, unsupported, and blocked final dispositions; and
- public-safe aggregation without row or hidden-change disclosure.

The synthetic repository vectors name all 40 mandatory pass, infrastructure,
environment, prerequisite, owner-result, cardinality, comparison, privacy,
bound, rollback, removal, and cleanup branches enumerated in
`THREE_REPOSITORY_WORKFLOW.md`. Qualification MUST compare the branch-name set
for exact equality; a subset or minimum-count assertion is not qualified.

Patch application, owner command execution, rollback, removal, and cleanup are
harness-owner responsibilities. Ferris product code MUST NOT implement them.

## Immutable cutoff

After independent package construction and qualification:

1. record the Ferris commit SHA and clean worktree;
2. create an immutable cutoff tag;
3. record binary and source-tree digests;
4. publish only the opaque fixture ID, private manifest revision, package
   digests, expected process count, platforms, and cutoff;
5. confirm that implementation authors have not received hidden material; and
6. seal further implementation changes until the first score is complete.

The Windows and Unix runs MUST use binaries built from the same source cutoff.
Environment-specific binary digests are recorded separately.

## First execution and oracle release

For each platform:

1. validation supplies the sealed input and harness package;
2. the harness executes all 56 cases exactly once;
3. the complete 56-record collection and environment receipt are sealed;
4. both platform collections are checked for 112 total records;
5. scorer conformance runs before the hidden oracle is released;
6. the oracle is released to the qualified scorer;
7. one final disposition is produced without implementation changes; and
8. all packages and receipts remain immutable.

Implementation authors may receive only the public-safe result after scoring.

## Leakage, failure, and quarantine

Leakage includes revealing a hidden input, key, value, canary, path, expected
digest, expected record, predicate, or scorer branch before the score.

If the implementation fails a valid score, the package becomes development
evidence and MUST NOT be rerun or rescored. A correction requires a new
implementation cutoff and an independently constructed replacement package
with a new opaque ID.

If harness, collection, custody, environment, or scorer qualification fails,
the attempt is `invalid`. Its artifacts remain quarantined and MUST NOT be
reused as the replacement.

## Public-safe result

The public result MUST include only:

- public contract revision;
- opaque fixture ID and private manifest revision;
- sealed package digests;
- immutable cutoff commit and tag;
- platform and process cardinality;
- scorer-conformance disposition;
- aggregate result-class counts;
- aggregate public-output digest;
- final disposition; and
- a statement that no hidden material is disclosed.

For contract revision 3 it also includes the three slot identifiers, sealed
repository-selection receipt digests, aggregate repository-workflow
disposition, and aggregate repository-output digest. It MUST NOT include a
repository-to-hidden-change mapping, changed path, patch content, before/after
source value, per-repository canary, or per-row output.

It MUST NOT reveal case-to-output mappings, hidden values, canaries, paths,
expected identities, expected digests, or oracle predicates.
