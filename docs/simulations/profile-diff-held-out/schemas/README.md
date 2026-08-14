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

## Independent public-input diagnostic declaration

- [`ferris.process-exit-diagnostic-public-input.v1.schema.json`](ferris.process-exit-diagnostic-public-input.v1.schema.json)
  freezes Pulse 32's permanent Pulse 22/Pulse 24/Pulse 26/Pulse 28/Pulse 30
  closure, immutable Pulse 31 cutoff, every normalized Pulse 25/Pulse 27
  infrastructure gate, the exact public input contract and schema, all six
  positive fixture path/size/digest bindings, all 33 mutation IDs and
  per-control public digests, public-only generator/classifier scope, 39/39
  contract self-validation before generation, and unchanged coverage, oracle,
  collection, search, minimization, and publication bounds. The positive
  declaration is `authorized-unexecuted`.

## Independent public-authority diagnostic declaration

- [`ferris.process-exit-diagnostic-public-authority.v1.schema.json`](ferris.process-exit-diagnostic-public-authority.v1.schema.json)
  freezes Pulse 34's permanent Pulse 22/Pulse 24/Pulse 26/Pulse 28/Pulse
  30/Pulse 32 closure, immutable Pulse 33 cutoff, every inherited Pulse 32
  public gate, the exact Pulse 33 manifest, aggregate, seal, build adapter,
  and public receipts, explicit WSL non-login Cargo discovery, Cargo
  `compiler-artifact` JSON discovery, and the mandatory exact Windows/Ubuntu
  Pulse 34 binary freeze. The positive declaration is
  `authorized-unexecuted`.

## Independent materialized public diagnostic declaration

- [`ferris.process-exit-diagnostic-pulse-36-authority.v1.schema.json`](ferris.process-exit-diagnostic-pulse-36-authority.v1.schema.json) freezes a later Pulse 36 authority at the complete Pulse 35 cutoff. It retains every Pulse 34 gate and closed invalid predecessor, exact eight-file release/qualification binding, private 32-byte seed/HMAC materialization and verification, 70/`18/18`/`8/8` tuple closure, transactional publication, and zero current execution fields.

## Independent normalized public diagnostic declaration

- [`ferris.process-exit-diagnostic-pulse-38-authority.v1.schema.json`](ferris.process-exit-diagnostic-pulse-38-authority.v1.schema.json) freezes a new authority at the complete Pulse 37 cutoff. It preserves Pulse 36's permanent invalid closure, retains every Pulse 36/Pulse 34 gate, binds the exact normalized eight-file Pulse 35 Git blobs and Pulse 37 proof, requires fresh private seed/materialization/verification, caps the sole transactional search at 70 cases/processes per platform and 140 processes total, and fixes every current activity/result field at zero, false, or null.

## Independent verifier-custody diagnostic declaration

- [`ferris.process-exit-diagnostic-pulse-40-authority.v1.schema.json`](ferris.process-exit-diagnostic-pulse-40-authority.v1.schema.json) freezes a new authority at immutable cutoff `65d1eec688f53bf7263ecfc8094ac849f9d3be4c`. It preserves Pulse 38's permanent invalid closure and every inherited gate while requiring exact LF Git-clean Pulse 39 verifier custody before any package copy: an eight-file release tree, a five-file manifest payload totaling 26455 bytes, one below-root `core.autocrlf=true` checkout, one NUL-framed root-anchored check-attr call, one version probe, 36/36 attributes/LF files, zero CR bytes, safe paths, and retained 76/76 bindings. Its later private-seed, 70-descriptor, <=70/platform, <=140-total search and publication bounds remain exact.

## Independent transactional-copy diagnostic declaration

- [`ferris.process-exit-diagnostic-pulse-42-authority.v1.schema.json`](ferris.process-exit-diagnostic-pulse-42-authority.v1.schema.json) freezes one new independent authority at immutable cutoff `2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8`. It preserves Pulse 38 and Pulse 40 as permanently invalid/null/non-retryable, verifies the complete Pulse 41 release before directly executing its cutoff adapter, then verifies the copied Pulse 39 checker in a separate `core.autocrlf=true` checkout before every inherited gate. Pulse 41 manifest raw identity `sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8` and Pulse 39 manifest raw identity `sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` are exact. Every current Pulse 41/Pulse 39 and later-gate execution field is zero, false, or null.

## Independent publication-order diagnostic declaration

- [`ferris.process-exit-diagnostic-pulse-46-authority.v1.schema.json`](ferris.process-exit-diagnostic-pulse-46-authority.v1.schema.json)
  freezes one new independent authority at immutable cutoff
  `22ea38e274b882d6e607810382f842b76e483f10`. It preserves Pulse 42's
  permanent invalid-publication/null closure and the valid Pulse 17 scoring
  baseline, binds complete exact current-cutoff Pulse 41/Pulse 39/Pulse
  43/Pulse 44/Pulse 45 release trees, fixes the eight-gate ordered catalog,
  separates public self-validation, requires one Pulse 45/Pulse 44 operation
  per platform, and permits one 70-per-platform/140-total zero-retry search
  only after inherited gates. Exact terminal Pulse 43 `2/2` publication is
  required before any public summary.

## Independent witnessed-publication diagnostic declaration

- [`ferris.process-exit-diagnostic-pulse-48-authority.v1.schema.json`](ferris.process-exit-diagnostic-pulse-48-authority.v1.schema.json)
  freezes one new independent authority at immutable cutoff
  `70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d`. It preserves Pulse 42 and
  Pulse 46 permanent invalid/non-retryable/null closure; binds complete exact
  current-cutoff Pulse 41/Pulse 39/Pulse 43/Pulse 44/Pulse 45/Pulse 47
  release trees; retains the eight-gate catalog and one
  70-per-platform/140-total zero-retry search; and makes the exact Pulse 47
  `witness_pulse_43` call the only terminal publication route. It fixes one
  witness invocation, one Pulse 43 invocation only through the witness,
  separate fresh absent absolute non-overlapping roots, actual-main-workspace
  root/path-set observation before a claim, and bounded witnessed disclosure.

## Public corpus-materializer records

- [`ferris.pulse-35-corpus-materializer.v1.schema.json`](ferris.pulse-35-corpus-materializer.v1.schema.json)
  closes Pulse 35 concrete case and independently derived coverage manifests.
  It fixes the exact 70-descriptor complete corpus, mandatory private
  verification seed/commitment, HMAC-SHA256 case/order/profile tokens,
  host-independent request resolution and custody namespaces, UNC-preserving
  lexical witnesses, explicit per-pair change counts/boundaries, directory-sync
  statuses, all 18 named domain records, and eight exact tuple catalogs. It
  fixes diagnostic execution, product modification, and logical retries at
  `false`, `false`, and `0`.

## Retained-binary custody records

- [`../pulse-44-retained-binary-custody-release/schemas/ferris.pulse-44-retained-binary-custody.v1.schema.json`](../pulse-44-retained-binary-custody-release/schemas/ferris.pulse-44-retained-binary-custody.v1.schema.json)
  closes the public success/failure summary shapes for a retained executable
  and receipt pair. It fixes the `2/2` final verification and one-rename/zero
  retry success posture, closed Pulse-43-compatible terminal events, and only
  absent/rolled-back/indeterminate failure posture without a local path or
  executable byte field.
- [`../pulse-45-binary-custody-event-bridge-release/schemas/ferris.pulse-45-binary-custody-event-bridge.v1.schema.json`](../pulse-45-binary-custody-event-bridge-release/schemas/ferris.pulse-45-binary-custody-event-bridge.v1.schema.json)
  defines the closed Pulse 45 bridge output and documents the complete closed
  Pulse 44 source summary that must be validated before a platform-specific
  Pulse-43 gate event can be emitted. It fixes both supported platforms,
  predecessor release identities, one invocation, zero retries, and terminal
  failure preservation without a path, private-data, or executable-byte field.
- [`../pulse-47-publication-outcome-witness-release/schemas/ferris.pulse-47-publication-outcome-witness.v1.schema.json`](../pulse-47-publication-outcome-witness-release/schemas/ferris.pulse-47-publication-outcome-witness.v1.schema.json)
  closes the persisted public Pulse 43 publication-outcome witness. It fixes
  sealed predecessor identities, one invocation, public `2/2` result hashes
  and aggregate-only success evidence, and bounded failure posture without
  ordered events/counts, local paths, private data, or executable bytes.

The three Stage A selection instances are published in
[`../repository-selections/`](../repository-selections/) and are bound by
[`../REPOSITORY_SELECTION_BINDING.md`](../REPOSITORY_SELECTION_BINDING.md).

Receipt schemas describe private durable custody artifacts. Publication is
still restricted by `CUSTODY_AND_PREFLIGHT.md`.

All 30 schemas use Draft 2020-12. Typed contract objects reject unknown
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
declaration with 322 mutations, plus the Pulse 32 authorized/unexecuted
public-input declaration with 538 mutations, plus the public profile-evidence
input with six positive fixtures and 33 negative controls, plus the Pulse 34
authorized/unexecuted public-authority declaration with 704 mutations, the
Pulse 36 materialized authority with 1998 mutations, the Pulse 38 normalized
authority with 7288 mutations, the Pulse 40 verifier-custody authority with
9076 mutations, the Pulse 42 transactional-copy authority with 9046 mutations,
the Pulse 46 publication-order authority with 9208 mutations, the Pulse 48
witnessed-publication authority with 9498 mutations, and the Pulse 35 public
corpus-materializer release records. The repository has 48317 total declared
mutations.
