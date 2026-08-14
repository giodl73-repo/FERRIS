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

The three Stage A selection instances are published in
[`../repository-selections/`](../repository-selections/) and are bound by
[`../REPOSITORY_SELECTION_BINDING.md`](../REPOSITORY_SELECTION_BINDING.md).

Receipt schemas describe private durable custody artifacts. Publication is
still restricted by `CUSTODY_AND_PREFLIGHT.md`.

All 13 schemas use Draft 2020-12. Typed objects reject unknown members.
Nullable process exits, digests, targets, license fields, wrappers, and
lifecycle joins are explicit rather than inferred. The public vectors include
41 core scorer instances and 38 core mutations. Dedicated tests additionally
validate the prospective release receipt with 12 mutations and the Pulse 22
authorized/unexecuted declaration with 35 mutations.
