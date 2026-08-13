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

Repository tests validate 41 positive schema instances, independently
recompute every published identity and evidence join, including command
surface and change-policy digests, and reject every mutation. They qualify
public infrastructure only and MUST NOT be copied into a sealed package or
treated as a scored case.
