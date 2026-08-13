# Profile Diff Held-Out Custody and Preflight

Status: Frozen protocol
Contract revision: 1

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
- records exactly one durable row per expected process;
- rejects missing, duplicate, retried, and extra rows;
- parses a complete JSON stream rather than a prefix or selected line;
- accepts the documented terminal newline and rejects trailing content;
- records command, binary, environment, and output digests;
- carries expected process declarations into durable collection records; and
- completes all 112 declared process slots before scoring can begin.

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

It MUST NOT reveal case-to-output mappings, hidden values, canaries, paths,
expected identities, expected digests, or oracle predicates.
