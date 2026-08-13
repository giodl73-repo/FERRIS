# Public Synthetic Qualification Fixtures

Status: Frozen for contract revision 2
Hidden material: None

- [`identity-vectors.json`](identity-vectors.json) contains source values,
  canonical bytes, normalized paths, every profile-diff selection branch,
  invocation, CLI fallback, diff, result, and aggregate expected digest.
- [`command-result-success.json`](command-result-success.json),
  [`command-result-difference.json`](command-result-difference.json), and
  [`command-result-incomplete.json`](command-result-incomplete.json) are
  current Rust serialization exemplars.
- [`preflight-vectors.json`](preflight-vectors.json) contains four synthetic
  exactly-once rows covering stdout-only, stderr-only, both-stream, empty
  stream, zero exit, and nonzero exits; two environment receipts; and the
  exact 56-case by two-platform 112-row declaration cardinality.
- [`schema-mutations.json`](schema-mutations.json) freezes missing, extra,
  nullability, exit, and change-digest rejection controls.
- [`repository-disposition-vectors.json`](repository-disposition-vectors.json)
  freezes pass, fail, invalid, unsupported, and blocked classification,
  including dirtiness, rollback, cleanup, omission, promotion, privacy,
  prohibited-conclusion, and bound failures.

Repository tests independently recompute these vectors. They qualify public
infrastructure only and MUST NOT be copied into a sealed package or treated as
a scored case.
