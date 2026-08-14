# Profile Diff Public Schemas

Status: Contract revision 3 candidate public schemas
Dialect: JSON Schema Draft 2020-12

These schemas describe public scorer and owner-harness records. Every object
rejects unknown members with `additionalProperties:false`.

## CLI records

- [`ferris.command-result.v2.schema.json`](ferris.command-result.v2.schema.json)
  is the complete `profile-diff` specialization of the generic
  `ferris.command-result/v2` Rust envelope.
- [`ferris.profile-diff.v0.schema.json`](ferris.profile-diff.v0.schema.json)
  is the exact non-null command record.

The command-result specialization intentionally binds
`semantic_command_id:"profile-diff"` and the currently reachable result
classes. Other Ferris commands use the same generic Rust envelope with
different command-specific record types and are outside Pulse 17 scoring.

## Public profile input

- [`ferris.profile-evidence.v0.schema.json`](ferris.profile-evidence.v0.schema.json)
  is the complete recursive parsed-value schema for explicit
  `ferris.profile-evidence/v0` inputs. Its root and twelve-member `sections`
  objects are closed. Section values may be any recursive JSON value, while
  every object member name at every depth uses 1 through 256 visible ASCII
  characters. Raw size, regular-file state, malformed JSON, and
  duplicate-member rejection are normative companion rules in
  [`../INPUT_PROFILE_EVIDENCE.md`](../INPUT_PROFILE_EVIDENCE.md).

## Collection records

- [`ferris.profile-diff-collection-row.v1.schema.json`](ferris.profile-diff-collection-row.v1.schema.json)
- [`ferris.profile-diff-environment-receipt.v1.schema.json`](ferris.profile-diff-environment-receipt.v1.schema.json)

## Three-repository records

- [`ferris.repository-selection.v1.schema.json`](ferris.repository-selection.v1.schema.json)
- [`ferris.public-repository-profile.v1.schema.json`](ferris.public-repository-profile.v1.schema.json)
- [`ferris.owner-command-receipt.v1.schema.json`](ferris.owner-command-receipt.v1.schema.json)
- [`ferris.owner-check-inventory.v1.schema.json`](ferris.owner-check-inventory.v1.schema.json)
- [`ferris.profile-comparison.v1.schema.json`](ferris.profile-comparison.v1.schema.json)
- [`ferris.repository-lifecycle-receipt.v1.schema.json`](ferris.repository-lifecycle-receipt.v1.schema.json)
- [`ferris.repository-immutability-receipt.v1.schema.json`](ferris.repository-immutability-receipt.v1.schema.json)

## Prospective diagnostic release

- [`ferris.post-score-diagnostic-release.v1.schema.json`](ferris.post-score-diagnostic-release.v1.schema.json)
  describes a future opt-in sanitized-reproducer receipt. It does not apply
  retroactively to Pulse 17.

## Diagnostic replication declaration

- [`ferris.process-exit-diagnostic-replication.v1.schema.json`](ferris.process-exit-diagnostic-replication.v1.schema.json)
  freezes the Pulse 22 public authority, coverage, oracle, search,
  minimization, publication, result-disposition, and custody-handoff bounds.
  The current positive declaration is `authorized-unexecuted`.

## Independent diagnostic replacement declaration

- [`ferris.process-exit-diagnostic-replacement.v1.schema.json`](ferris.process-exit-diagnostic-replacement.v1.schema.json)
  freezes Pulse 24's permanent Pulse 22 closure, immutable Ferris cutoff,
  exact collector qualification digests, harmless preflight gate, new custody
  and corpus requirements, unchanged public coverage and oracle, transactional
  pair collection, result dispositions, and publication bounds. The positive
  declaration is `authorized-unexecuted`.

## Independent public-bundle diagnostic declaration

- [`ferris.process-exit-diagnostic-public-bundle.v1.schema.json`](ferris.process-exit-diagnostic-public-bundle.v1.schema.json)
  freezes Pulse 26's permanent Pulse 22/Pulse 24 closure, immutable later
  cutoff, exact public Pulse 25 directory, manifest, nine files, source, test,
  and bundle aggregates, receipt and seal, nine-file-only isolated copy,
  independent recomputation, exactly two zero-retry harmless preflight pairs,
  wholly new custody and generation, unchanged public coverage and oracle,
  transactional collection, result dispositions, and publication bounds. The
  positive declaration is `authorized-unexecuted`.

## Independent public-adapter diagnostic declaration

- [`ferris.process-exit-diagnostic-public-adapter.v1.schema.json`](ferris.process-exit-diagnostic-public-adapter.v1.schema.json)
  freezes Pulse 28's permanent Pulse 22/Pulse 24/Pulse 26 closure, immutable
  committed Pulse 27 cutoff, every Pulse 25 collector binding, the exact
  Pulse 27 directory and 20-file manifest, independent file and aggregate
  recomputation, one exact two-pair adapter invocation, two fresh platform
  verifiers, whole-store cardinality `2/2/2`, wholly new custody and
  generation, and the unchanged Pulse 26 coverage, oracle, collection,
  search, minimization, and publication bounds. The positive declaration is
  `authorized-unexecuted`.

## Final normalized public-adapter diagnostic declaration

- [`ferris.process-exit-diagnostic-normalized-public-adapter.v1.schema.json`](ferris.process-exit-diagnostic-normalized-public-adapter.v1.schema.json)
  freezes Pulse 30's permanent Pulse 22/Pulse 24/Pulse 26/Pulse 28 closure,
  immutable post-normalization cutoff, exact Pulse 29 receipt, per-file
  `text=set`/`eol=lf` checks, 36/36 LF and 76/76 binding gates, every
  normalized Pulse 25/Pulse 27 binding, exact 20-file copy and recomputation,
  one exact two-pair adapter invocation, two fresh verifiers, `2/2/2`
  cardinality, post-pass fresh material, and unchanged Pulse 26 coverage,
  oracle, collection, search, minimization, and publication bounds. The
  positive declaration is `authorized-unexecuted`.

The three Stage A selection instances are published in
[`../repository-selections/`](../repository-selections/) and are bound by
[`../REPOSITORY_SELECTION_BINDING.md`](../REPOSITORY_SELECTION_BINDING.md).

Receipt schemas describe private durable custody artifacts. Publication is
still restricted by `CUSTODY_AND_PREFLIGHT.md`.

All 18 schemas use Draft 2020-12. Typed contract objects reject unknown
members; the profile-evidence schema intentionally permits recursive section
objects only while constraining every member name and recursive value.
Nullable process exits, digests, targets, license fields, wrappers, and
lifecycle joins are explicit rather than inferred. The public vectors include
41 core scorer instances and 38 core mutations. Dedicated tests additionally
validate the prospective release receipt with 12 mutations and the Pulse 22
authorized/unexecuted declaration with 35 mutations, plus the Pulse 24
authorized/unexecuted replacement declaration with 82 mutations and the
Pulse 26 authorized/unexecuted public-bundle declaration with 176 mutations,
plus the Pulse 28 authorized/unexecuted public-adapter declaration with 263
mutations, plus the Pulse 30 authorized/unexecuted normalized public-adapter
declaration with 322 mutations, plus the public profile-evidence input with
six positive fixtures and 33 negative controls.
