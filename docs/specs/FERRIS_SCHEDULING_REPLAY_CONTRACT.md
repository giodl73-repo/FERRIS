# Ferris Scheduling Replay Contract

Status: GO-WP-005 implementation target

## User outcome

Answer one bounded question before changing live execution:

> Given one observed failed owner topology, which work could each scheduling
> profile have avoided without hiding required scope or useful diagnostics?

Owner systems retain gate semantics, dependency truth, failure decisiveness,
cancellability, diagnostic value, and final success policy. Ferris contributes
deterministic replay and typed counterfactual accounting. It never invents an
owner label.

## Interface

`ferris schedule --request <JSON>` reads
`ferris.schedule-replay-request/v1` and emits
`ferris.schedule-replay-report/v1`.

The request binds one repository, pull request, exact source revision, topology
identity, and bounded non-empty observed node graph. Every node retains:

- stable node identity and requiredness;
- owner dependencies;
- observed start and finish offsets;
- observed terminal outcome;
- whether its failure is owner-decisive; and
- optional owner-provided cancellability and diagnostic-value labels.

Unknown fields, duplicate nodes, unknown dependencies, cycles, impossible
timing, malformed identities, and oversized inputs fail the whole request.

## Profiles

Every accepted request emits the same required graph under four profiles:

- `conservative_owner_order` changes no independent work;
- `fail_fast` may stop owner-cancellable independent work after the first
  owner-decisive failure;
- `flush_out` continues independent diagnostics and blocks only work whose
  prerequisites can no longer succeed; and
- `balanced` stops owner-cancellable low-value independent work while retaining
  high-value, standard, and unclassified diagnostics.

Missing `cancellable` or `diagnostic_value` labels always produce conservative
continuation. A model, heuristic, duration, or command name cannot supply the
missing owner decision.

## Invariants

Across all profiles these values are identical:

- repository, pull request, source, and topology identity;
- complete node inventory and required-node set;
- dependency graph;
- observed outcomes;
- final success predicate; and
- aggregate success result.

Only typed start, continuation, dependency-block, and owner-authorized
cancellation decisions may differ. A required node never disappears, and a
failed, blocked, cancelled, unavailable, or missing required result never
becomes success.

## Timing projection

The trigger is the earliest completed failed required node marked
`owner_decisive_failure`; ties resolve by node identity. Optional failures
cannot be decisive. A dependency descendant that had not started before its
non-successful prerequisite completed is `blocked_by_dependency` under every
profile.

For owner-authorized independent cancellation:

- work not started at the trigger may project its full observed duration as
  saved; and
- work still running at the trigger may project only its observed remaining
  duration as saved.

Completed work contributes zero. The report retains the trigger and reason for
every non-continued node.

These are counterfactual projections over observed timing, not proof that a
provider would reproduce the schedule.

## Product target

One revision cannot establish the 22-36 minute per-failed-PR target. A report is:

- `insufficient_evidence` when owner policy labels are incomplete; or
- `observation_only` when labels are complete.

Cohort validation remains separate and must exclude every revision already
counted as a prevented iteration by GO-WP-004.

## Boundaries

GO-WP-005 does not add live parallel execution, provider APIs, resource
scheduling, retries, credentials, publication, branch-policy changes, or
consumer-repository modifications. Live coordination requires a later explicit
decision after replay demonstrates value and safety.
