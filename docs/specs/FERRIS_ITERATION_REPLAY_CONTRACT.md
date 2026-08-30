# Ferris Iteration Replay Contract

Status: Implemented for GO-WP-004

## User outcome

Answer one practical question with evidence:

> Did an exact owner check fail locally on the same source before the matching
> remote CI failure, making that remote iteration preventable?

The owner CI system remains authoritative for its gate identity, result,
timing, and failure classification. Ferris contributes deterministic
comparison and cohort accounting. It does not infer equivalence from similar
command names or error text.

## Interface

`ferris replay --request <JSON>` reads
`ferris.iteration-replay-request/v1` and emits
`ferris.iteration-replay-report/v1`.

Each case references one verified `ferris.execution-receipt/v1`, names one
receipt lane and its owner-classified stable failure fingerprint, and supplies
one strict `ferris.remote-iteration-evidence/v1` record. Input files are bounded
regular files. Unknown fields, duplicate case IDs, duplicate remote iteration
identities, incomplete evidence, and malformed identities fail the whole
request.

Receipt paths use portable `/` separators and are resolved relative to the
canonical directory containing the request file. Absolute paths, `.` or `..`
components, backslashes, unavailable files, non-files, and paths whose
canonical target escapes that directory are rejected.

The request carries a provenance level and a bounded non-empty case list. Each
case binds one receipt path and local receipt lane and one embedded remote
evidence record. Remote evidence includes a structured repository/PR/provider/
pipeline/run/attempt identity, source revision, owner gate, lane, entrypoint ID
and digest, operating system and architecture, non-secret environment identity,
terminal status, typed owner failure category, stable failure fingerprint,
complete/truncated flags and raw digests for both output streams, duration, and
an explicit whole-record completeness assertion.

Classification precedence is deterministic. A remote success or cancellation
is classified first, followed by the authoritative non-owner-actionable remote
failure categories, a local non-failure, and then repository, source, platform,
gate, lane, entrypoint, environment, and failure-evidence mismatches in that
order.

## Required equivalence

A case supports a prevented iteration only when all of these are exact:

- repository identity;
- PR identity;
- source revision;
- owner gate identity;
- lane identity;
- owner entrypoint identity;
- operating system and architecture;
- non-secret inherited environment identity;
- local and remote terminal failure;
- owner failure classification; and
- the owner-defined stable failure fingerprint.

The remote evidence must be complete, terminal, and classified
`owner_actionable`. Infrastructure-only, secret-only, unavailable, cancelled,
unknown, or truncated remote failures never count as prevented.

Raw stdout and stderr digests remain in the evidence for traceability but are
not compared for equality. Real local and hosted-CI streams routinely differ in
absolute paths, timing, terminal formatting, redaction, and harmless progress
output. Requiring byte equality would reject genuine reproductions. The stable
failure fingerprint is owner-defined, must be content-addressed, and must be
derived by the same owner procedure on both sides.

Ferris compares identities and classifications only. It does not use fuzzy text
matching, an AI judgment, or a generic Cargo command as a substitute for an
owner entrypoint. Replay validates evidence integrity and consistency; it does
not authenticate the remote provider or the person who produced the evidence.

## Result classes

Every case receives exactly one classification:

- `prevented_iteration_supported`;
- `local_did_not_fail`;
- `remote_did_not_fail`;
- `repository_mismatch`;
- `source_mismatch`;
- `platform_mismatch`;
- `gate_mismatch`;
- `lane_mismatch`;
- `entrypoint_mismatch`;
- `environment_mismatch`;
- `failure_evidence_mismatch`;
- `infrastructure_only`;
- `secret_only`;
- `unavailable`;
- `cancelled`; or
- `unknown`.

Mismatch and ineligible cases remain visible; they are never silently removed.

## Cohort accounting

The report includes:

- distinct failed PRs observed;
- distinct failed PRs with at least one eligible owner-actionable case;
- supported prevented iterations;
- average supported prevented iterations per eligible failed PR;
- eligible failed PRs with at least one supported iteration and their coverage
  ratio;
- the fixed average target of at least one supported prevented iteration per
  eligible failed PR; and
- exact revision identities excluded from all later tail-savings analysis.

One remote iteration may contribute at most once; repeating its identity makes
the request invalid rather than silently choosing a lane. Avoided revisions are
exported as an explicit disjoint set.
Results are labeled `fixture`, `historical_replay`, or `shadow_observation`.
Fixture results cannot satisfy or reject the product target.

## Boundaries

GO-WP-004 does not add changed-path selection, scheduling profiles,
cancellation policy, provider APIs, automatic CI collection, publication,
branch-policy changes, or consumer-repository modifications. GO-WP-005 and
later remain separately controlled.
