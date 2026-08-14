# Pulse 22: Independent Process-Exit Diagnostic Replication

Status: Complete; invalid diagnostic run; no category conclusion
Implementation authority: Public contract, machine-readable declaration,
nine-role review, independent custody execution, and public-safe closeout only

## Goal

Authorize one fresh independent diagnostic replication program for the
released category `process-exit-agreement`.

This is not certification, a score, a Pulse 17 retry, or product-fix
authority. The valid Pulse 17 failure remains immutable regardless of the
Pulse 22 outcome.

## Bounded authority

This pulse authorizes only:

- the normative
  [diagnostic replication contract](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md);
- one closed Draft 2020-12 declaration schema, one public
  `authorized-unexecuted` fixture, and public mutation controls;
- one nine-role pre-execution review;
- test-only validation of the public declaration; and
- a later independent custody execution within the frozen contract.

The independent custodian, not an implementation author, constructed the
fresh corpus from public rules, privately committed the deterministic seed,
froze the independent reference classifier, and began the bounded search.

The run became invalid after one retained Windows process and before the
required Ubuntu partner launch. A collector durability failure prevented the
atomic cross-platform pair from completing. The contract prohibits retrying
or converting that incomplete pair into a category conclusion.

## Fixed search and minimization bounds

- maximum 512 unique fresh cases per platform;
- exactly two platforms: Windows x86-64 and Ubuntu 24.04 WSL2 x86-64;
- maximum 1,024 search process launches total;
- one launch per reached case per platform and zero candidate retries;
- one search execution, stopped after the first completed cross-platform case
  pair containing a target mismatch;
- a separate deterministic minimization phase only after reproduction;
- maximum 128 recorded transformations and 256 minimization process launches;
  and
- no Pulse 17 access, modification, retry, rescore, reuse, reconstruction,
  correlation, or inference.

The search is diagnostic, not scored. Minimization may execute derived fresh
public candidates because it begins only after the immutable search result is
preserved and remains outside Pulse 17.

## Frozen public boundary

The custodian must derive expected behavior only from the public
`profile-diff` rules, schemas, identity rules, and result-class map. The
independent classifier must be frozen before candidate generation and must
not call Ferris production or test code.

Each reached case compares:

- expected result class and public mapped exit;
- emitted result class and `process_exit_code`;
- actual operating-system exit;
- diagnostic class;
- record nullability; and
- stdout/stderr route.

The target category is reproduced when any frozen expected/emitted/actual
exit-agreement predicate fails. Adjacent diagnostic, nullability, framing, or
route failures are preserved but do not get relabeled as the target category.

## Result boundary

If the target category is reproduced, the custodian stops the search,
preserves the first reproducer, and may minimize only through the separate
bounded lineage phase. A completed reproduction disposition requires a fresh
sanitized public directory and a
`ferris.post-score-diagnostic-release/v1` receipt with all zero-overlap gates
equal to zero. The private search package and public reproducer are both
permanently ineligible for certification.

If the complete bounded search finds no target predicate failure, the
custodian publishes exact coverage, cardinality, platform counts, and
aggregate digests with the bounded disposition `no-reproduction`. That result
does not authorize a fix or alter Pulse 17.

## Execution outcome

- cutoff: `94890e53631d9110128bb420bf0cbbb074187e7c`;
- generated and independently selected cases: 188;
- search processes: 1 Windows, 0 Ubuntu, 1 total;
- completed cross-platform pairs: 0;
- candidate retries: 0;
- minimization transformations and processes: 0;
- disposition: `invalid`;
- blocker: `collector-durability-fsync-invalid-descriptor`; and
- target-category conclusion: none.

The frozen search cannot resume or be retried. No reproducer or diagnostic
release receipt exists. Pulse 17 remains immutable, PLATFORM-001 remains
Draft, and no product-fix authority exists.

## Evidence

- [Normative contract](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md)
- [Declaration schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-replication.v1.schema.json)
- [Authorized/unexecuted declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-replication.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-replication-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-22-PROCESS-EXIT-DIAGNOSTIC-REPLICATION-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_replication.rs)
- [Public-safe invalid result](../../../../docs/simulations/profile-diff-held-out/pulse-22-public-result/README.md)

## Stop conditions

Stop rather than widen this pulse if work would require:

- Ferris production code, behavior, dependency, API, schema, or output
  changes;
- implementation-author construction or selection of candidate cases;
- hidden material, an old fixture, or a Pulse 17 oracle;
- more than one search execution, a candidate retry, favorable-result
  selection, or scoring;
- more than 512 cases on either platform or 1,024 search launches total;
- more than 128 minimization transformations;
- publication without complete lineage and zero-overlap gates;
- a certification, support, correctness, compatibility, or platform claim; or
- changing PLATFORM-001 from Draft.
