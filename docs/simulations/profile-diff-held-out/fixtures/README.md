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
- [`process-exit-diagnostic-pulse-36-authority.json`](process-exit-diagnostic-pulse-36-authority.json) is the harmless Pulse 36 declaration. It pins permanent Pulse 22/24/26/28/30/32/34 closure, cutoff `48697c8da0e93b92fa633e353925ca05707bf9ed`, every Pulse 34 gate, and exact Pulse 35 manifest, eight files, aggregate, receipt, seal, machine schema, seed/HMAC, verification, tuple, and publication requirements. All execution/result fields are zero, false, or null.
- [`process-exit-diagnostic-pulse-36-authority-mutations.json`](process-exit-diagnostic-pulse-36-authority-mutations.json) freezes 1998 rejection controls for cutoff/release/gate/cardinality/tuple/seed/privacy/premature-activity drift, every scalar field, required-record removal, unknown members, and identity recomputation.
- [`process-exit-diagnostic-pulse-38-authority.json`](process-exit-diagnostic-pulse-38-authority.json)
  is the harmless Pulse 38 declaration. It pins permanent Pulse 36 closure,
  cutoff `6807bd68aa01cbf0c819198765b7d6b5aa443328`, every Pulse 36/Pulse 34
  gate, normalized Pulse 35 eight-file/manifest/aggregate/seal bindings,
  exact Pulse 37 receipt proof, private seed/HMAC materialization and fresh
  verification, and one <=70/platform, <=140-process transactional search.
  Every activity/result field is zero, false, or null.
- [`process-exit-diagnostic-pulse-38-authority-mutations.json`](process-exit-diagnostic-pulse-38-authority-mutations.json)
  freezes 7288 rejection controls for every scalar, required-member removal,
  unknown member, identity recomputation, Pulse 36 closure, cutoff, normalized
  binding, Pulse 37 proof, inherited gate, seed, tuple, search-bound, and
  premature-activity drift.
- [`process-exit-diagnostic-pulse-40-authority.json`](process-exit-diagnostic-pulse-40-authority.json)
  is the harmless Pulse 40 declaration. It pins the immutable
  `65d1eec688f53bf7263ecfc8094ac849f9d3be4c` cutoff, permanent Pulse 38
  closure, every inherited gate, exact eight-file release tree, its five-payload-file/26455-byte manifest, and all raw
  release identities, LF Git-clean custody before package copy, one
  below-root `core.autocrlf=true` checkout, one check-attr plus one version
  probe, 36/36 zero-CR safe paths, 76/76 normalized bindings, and the later
  private-seed/70-descriptor/<=70-platform/<=140-total bounds.
- [`process-exit-diagnostic-pulse-40-authority-mutations.json`](process-exit-diagnostic-pulse-40-authority-mutations.json)
  freezes 9076 rejection controls for every inherited and new scalar,
  required-member removal, closed-shape addition, identity recomputation,
  Pulse 38 closure, cutoff, every Pulse 39 manifest/report/receipt/seal/file
  binding, checkout/process/count/path control, retained 76/76 proof, and
  premature activity drift.
- [`process-exit-diagnostic-pulse-42-authority.json`](process-exit-diagnostic-pulse-42-authority.json) is the harmless Pulse 42 declaration. It pins immutable cutoff `2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8`, permanent Pulse 38 and Pulse 40 closure, exact Pulse 41 eight-file/five-payload/49120-byte release identities (manifest `sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8`), direct cutoff execution of `transactional_copy.py` with `PYTHONDONTWRITEBYTECODE=1`, exact absolute source/fresh absent final roots, 8/8/8 copy verification, eight fsyncs, two honest staging sync attempts, one rename, zero retry/rollback/residue, then the copied Pulse 39 verifier (manifest `sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c`) in one separate fresh `core.autocrlf=true` checkout before every inherited gate. Every execution field is zero, false, or null.
- [`process-exit-diagnostic-pulse-42-authority-mutations.json`](process-exit-diagnostic-pulse-42-authority-mutations.json) freezes 9046 comprehensive rejection controls for every required member, closed object, scalar, and array cardinality; direct adapter execution and no-alternate-copier rules; every Pulse 41/Pulse 39 binding and zero execution count; predecessor closure; inherited gates; identity recomputation; and one-launch search bounds.
- [`process-exit-diagnostic-pulse-46-authority.json`](process-exit-diagnostic-pulse-46-authority.json)
  is the harmless Pulse 46 declaration. It pins cutoff
  `22ea38e274b882d6e607810382f842b76e483f10`, preserves Pulse 42's
  invalid-publication/null closure and Pulse 17's valid baseline, requires
  complete exact current-cutoff Pulse 41/Pulse 39/Pulse 43/Pulse 44/Pulse 45
  release custody, fixes the eight stable ordered gates, keeps public
  self-validation nonadvancing, and requires terminal Pulse 43 publication.
  Every execution/result field is zero, false, or null.
- [`process-exit-diagnostic-pulse-46-authority-mutations.json`](process-exit-diagnostic-pulse-46-authority-mutations.json)
  freezes 9208 comprehensive controls: every scalar replacement, every
  required-member removal, an unknown-member addition to every closed object,
  every array-member removal, and declaration-identity recomputation.
- [`../pulse-44-retained-binary-custody-release/fixtures/synthetic-build-receipt.json`](../pulse-44-retained-binary-custody-release/fixtures/synthetic-build-receipt.json)
  is a bounded public retained-build receipt shape used only by the Pulse 44
  custody release. It contains no executable bytes, local paths, private data,
  diagnostic result, or authority.
- [`../pulse-45-binary-custody-event-bridge-release/fixtures/pulse-44-published-summary.json`](../pulse-45-binary-custody-event-bridge-release/fixtures/pulse-44-published-summary.json)
  and
  [`../pulse-45-binary-custody-event-bridge-release/fixtures/pulse-44-failed-summary.json`](../pulse-45-binary-custody-event-bridge-release/fixtures/pulse-44-failed-summary.json)
  are bounded public Pulse 44 source-summary shapes used only to qualify the
  Pulse 45 event bridge. They contain no path, executable byte, private data,
  diagnostic result, or authority.
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
plus the 538 Pulse 32 controls, 33 profile-evidence controls, 704 Pulse 34
controls, 1998 Pulse 36 controls, 7288 Pulse 38 controls, 9076 Pulse 40
controls, 9046 Pulse 42 comprehensive controls, and 9208 Pulse 46
comprehensive controls. The repository therefore has 38819 total declared
mutations. Pulse 35
adds no fixture or mutation control: its public release binds the existing
Pulse 31 fixtures and Pulse 34 authority/result only, and its own rejection
controls remain executable unit-test coverage rather than scored inputs. They
qualify public infrastructure only and MUST NOT be copied into a sealed package
or treated as a scored case.
