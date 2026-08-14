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
- [`process-exit-diagnostic-public-adapter.json`](process-exit-diagnostic-public-adapter.json)
  is the harmless Pulse 28 positive declaration. It pins permanent Pulse 22,
  Pulse 24, and Pulse 26 closure, the committed Pulse 27 cutoff, every Pulse
  25 collector binding, the exact Pulse 27 20-file manifest and aggregates,
  20-file-only isolated custody with no private-workspace access, one adapter
  invocation producing two Windows/Ubuntu pairs and two seals, two fresh
  platform verifiers enforcing `2/2/2` whole-store cardinality, wholly new
  custody and generation, and the unchanged Pulse 26 coverage, oracle,
  collection, search, minimization, and publication bounds.
- [`process-exit-diagnostic-public-adapter-mutations.json`](process-exit-diagnostic-public-adapter-mutations.json)
  freezes 263 rejection controls for predecessor reopening, retry, reuse,
  correlation, or inference; authority widening; cutoff drift; Pulse 25 or
  Pulse 27 binding drift; file, aggregate, copy, or collector-identity
  weakening; private-workspace access; adapter, pair, row, seal, verifier,
  cardinality, retry, or residue drift; custody and prohibited-material reuse;
  inherited coverage, oracle, collection, search, minimization, or
  publication weakening; premature activity; unknown fields; and identity.
- [`process-exit-diagnostic-normalized-public-adapter.json`](process-exit-diagnostic-normalized-public-adapter.json)
  is the harmless Pulse 30 positive declaration. It pins permanent Pulse 22,
  Pulse 24, Pulse 26, and Pulse 28 closure; the immutable post-normalization
  cutoff; the exact Pulse 29 receipt; `core.autocrlf=true` materialization;
  `text=set`, `eol=lf`, and LF bytes for all 36 release files; 76/76 normalized
  bindings; every normalized Pulse 25/Pulse 27 file and aggregate; exact
  20-file copying and recomputation; one two-pair adapter invocation; two
  fresh `2/2/2` verifiers; zero retries and residue; post-pass fresh material;
  and unchanged inherited diagnostic bounds.
- [`process-exit-diagnostic-normalized-public-adapter-mutations.json`](process-exit-diagnostic-normalized-public-adapter-mutations.json)
  freezes 322 rejection controls covering all inherited Pulse 28 controls plus
  closed Pulse 28 reopening, authority-at-cutoff drift, normalization receipt
  drift, materialization and per-file attribute weakening, LF and 76-check
  cardinality drift, copy-before-normalization, premature activity,
  post-preflight freshness weakening, unknown fields, and identity.
- [`process-exit-diagnostic-public-input.json`](process-exit-diagnostic-public-input.json)
  is the harmless Pulse 32 positive declaration. It pins permanent Pulse 22,
  Pulse 24, Pulse 26, Pulse 28, and Pulse 30 closure; immutable cutoff
  `29517d732db13cc2ffa304684b344f3538ab587d`; every inherited normalized
  package and adapter-preflight rule; the exact Pulse 31 contract and schema;
  all six positive fixture path, size, and digest bindings; all 33 mutation
  IDs and canonical public digests; public-only generator/classifier scope;
  and 39/39 contract self-validation before any generation.
- [`process-exit-diagnostic-public-input-mutations.json`](process-exit-diagnostic-public-input-mutations.json)
  freezes 538 rejection controls covering every inherited Pulse 30 control
  plus closed Pulse 30 reopening, cutoff drift, contract/schema/fixture/
  mutation binding drift, all 33 per-control digests, public read-scope
  widening, source/test access, self-validation weakening, premature
  generator/classifier state, unknown fields, and identity.
- [`process-exit-diagnostic-public-authority.json`](process-exit-diagnostic-public-authority.json)
  is the harmless Pulse 34 positive declaration. It pins permanent Pulse 22,
  Pulse 24, Pulse 26, Pulse 28, Pulse 30, and Pulse 32 closure; immutable
  cutoff `5df7492fa759c415f6ce540a33a4e89c46714348`; every inherited Pulse 32
  public gate; the exact Pulse 33 37-file manifest, aggregate, seal, build
  adapter, and public receipts; explicit WSL non-login Cargo discovery;
  Cargo `compiler-artifact` JSON discovery; and an exact Windows/Ubuntu
  cutoff binary freeze before preflight.
- [`process-exit-diagnostic-public-authority-mutations.json`](process-exit-diagnostic-public-authority-mutations.json)
  freezes 704 rejection controls covering every inherited Pulse 32 control
  plus closed Pulse 32 reopening, Pulse 33 release drift, Cargo-discovery
  weakening, incomplete platform binary freeze, premature activity, unknown
  fields, and identity.
- [`profile-evidence-v0-positive-scalars.json`](profile-evidence-v0-positive-scalars.json),
  [`profile-evidence-v0-positive-arrays.json`](profile-evidence-v0-positive-arrays.json),
  [`profile-evidence-v0-positive-objects.json`](profile-evidence-v0-positive-objects.json),
  and
  [`profile-evidence-v0-positive-nested-mixed.json`](profile-evidence-v0-positive-nested-mixed.json)
  are complete public inputs spanning scalar, array, object, and recursively
  nested section values.
- [`profile-evidence-v0-positive-boundary-minimum.json`](profile-evidence-v0-positive-boundary-minimum.json)
  and
  [`profile-evidence-v0-positive-boundary-maximum.json`](profile-evidence-v0-positive-boundary-maximum.json)
  exercise one-character and 256-character identity metadata and recursive
  object member names.
- [`profile-evidence-v0-mutations.json`](profile-evidence-v0-mutations.json)
  freezes 33 declared-invalid controls for unavailable/non-file sources,
  inclusive size boundaries, malformed and empty JSON, duplicate members,
  recursive invalid keys, unsupported/missing schema, closed shape, and
  identity metadata. Construction rules and classification precedence are
  normative in
  [`../INPUT_PROFILE_EVIDENCE.md`](../INPUT_PROFILE_EVIDENCE.md).

Repository tests validate 41 core positive schema instances plus the
prospective release, Pulse 22 declaration, Pulse 24 declaration, and Pulse 26
declaration, plus the Pulse 28, Pulse 30, Pulse 32, and Pulse 34 declarations; independently
recompute every published identity and evidence join, including command
surface and change-policy digests; and reject the existing 167 controls plus
the 176 Pulse 26 controls, 263 Pulse 28 controls, and 322 Pulse 30 controls,
plus the 538 Pulse 32 controls, 33 profile-evidence controls, and 704 Pulse 34
controls. The repository therefore has 2203 total declared mutations.
They qualify public infrastructure only and MUST NOT be copied into a sealed
package or treated as a scored case.
