# Public Synthetic Qualification Fixtures

Status: Contract revision 3 candidate public vectors
Hidden material: None

- [`identity-vectors.json`](identity-vectors.json) contains source values,
  canonical bytes, normalized paths, every profile-diff selection branch,
  invocation, CLI fallback, diff, result, and aggregate expected digest.
- [`command-result-success.json`](command-result-success.json),
  [`command-result-difference.json`](command-result-difference.json), and
  [`command-result-incomplete.json`](command-result-incomplete.json) are
  current Rust serialization exemplars.
- [`preflight-vectors.json`](preflight-vectors.json) contains ten synthetic
  exactly-once rows covering launch failure, timeout, both output bounds, both
  stream-reader failures, stdout-only, stderr-only, both-stream, empty-stream,
  zero-exit, and nonzero-exit branches; two environment receipts; and the exact
  56-case by two-platform 112-row declaration cardinality.
- [`repository-evidence-vectors.json`](repository-evidence-vectors.json)
  contains selections for all three slots, six owner-command outcomes, all
  three check inventories, nine public profile projections, pass/fail
  comparisons, pass/fail lifecycle receipts, and an immutability receipt.
- [`schema-mutations.json`](schema-mutations.json) freezes 38 missing, extra,
  cardinality, nullability, exit, bound, and identity rejection controls.
- [`repository-disposition-vectors.json`](repository-disposition-vectors.json)
  freezes exact set equality for all 40 mandatory pass, fail, invalid,
  unsupported, and blocked branches, including dirtiness, rollback, cleanup,
  cardinality, omission, promotion, privacy, prohibited-conclusion, and bound
  failures.
- [`post-score-diagnostic-release.json`](post-score-diagnostic-release.json)
  is a harmless positive prospective release receipt.
- [`post-score-diagnostic-release-mutations.json`](post-score-diagnostic-release-mutations.json)
  freezes 12 rejection controls for tier escalation, score mutation, hidden
  overlap, reuse, platform/category/exit mismatch, bounds, cleanup, retirement,
  unknown fields, and identity.
- [`process-exit-diagnostic-replication.json`](process-exit-diagnostic-replication.json)
  is the harmless Pulse 22 positive declaration. It contains public coverage
  categories and frozen bounds only; its disposition is
  `authorized-unexecuted`, and it contains no seed, case, input, process, or
  result.
- [`process-exit-diagnostic-replication-mutations.json`](process-exit-diagnostic-replication-mutations.json)
  freezes 35 rejection controls for authority widening, old-fixture access,
  case reuse or premature construction, platform and process bounds, retries,
  seed disclosure, coverage erosion, oracle inference, minimization widening,
  certification eligibility, custody ownership, unknown fields, and identity.
- [`process-exit-diagnostic-replacement.json`](process-exit-diagnostic-replacement.json)
  is the harmless Pulse 24 positive declaration. It pins permanent Pulse 22
  closure, the immutable Ferris cutoff, all five collector qualification
  digests, new custody and corpus requirements, harmless preflight,
  transactional collection, unchanged public coverage and oracle, and an
  `authorized-unexecuted` result with zero preflight and candidate activity.
- [`process-exit-diagnostic-replacement-mutations.json`](process-exit-diagnostic-replacement-mutations.json)
  freezes 82 rejection controls for predecessor reopening, authority widening,
  cutoff drift, collector digest or repair drift, preflight weakening, custody
  and generation reuse, bound and retry widening, coverage or oracle erosion,
  non-transactional collection, minimization or publication weakening,
  premature conclusions, custody ownership, unknown fields, and identity.
- [`process-exit-diagnostic-public-bundle.json`](process-exit-diagnostic-public-bundle.json)
  is the harmless Pulse 26 positive declaration. It pins permanent Pulse 22
  and Pulse 24 closure, the immutable later cutoff, the exact public Pulse 25
  source directory, manifest, receipt, seal, all nine file hashes and three
  aggregates, a nine-file-only isolated copy, independent recomputation,
  exactly two zero-retry harmless atomic preflight pairs, new custody and
  generation, unchanged public coverage and oracle, and an
  `authorized-unexecuted` result with zero custody, preflight, or candidate
  activity.
- [`process-exit-diagnostic-public-bundle-mutations.json`](process-exit-diagnostic-public-bundle-mutations.json)
  freezes 176 rejection controls for predecessor reopening or inference,
  authority widening, cutoff drift, public directory or release-binding
  drift, file or aggregate drift, copy or recomputation weakening, preflight
  count or retry changes, custody and prohibited-material reuse, search and
  minimization widening, coverage or oracle erosion, non-transactional
  collection, publication weakening, premature execution or conclusions,
  custody ownership, unknown fields, and identity.

Repository tests validate 41 core positive schema instances plus the
prospective release, Pulse 22 declaration, Pulse 24 declaration, and Pulse 26
declaration; independently recompute every published identity and evidence
join, including command surface and change-policy digests; and reject the
existing 167 controls plus the 176 Pulse 26 controls, for 343 total declared
mutations. They qualify public infrastructure only and MUST NOT be copied into
a sealed package or treated as a scored case.
