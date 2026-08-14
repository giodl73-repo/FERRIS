# Pulse 30: Final Independent Normalized Public-Adapter Authority

Status: Complete; authority authorized and unexecuted
Implementation authority: Governance, public fixtures, review, and test-only
validation only

## Goal

Authorize one final new independent `process-exit-agreement` diagnostic
program using the normalized Pulse 25 collector and Pulse 27 exact-two-pair
adapter after Pulse 29, without executing any program component.

Pulses 22, 24, 26, and 28 remain permanently invalid, non-retryable, and
unable to produce category conclusions. Pulse 30 is not their retry, resume,
reseed, rescore, reuse, continuation, correlation, or inference.

## Immutable cutoff

Future custody is bound to
`cf6b3309c31e5da37d4a8e6655a781f4e92ef603`.

That cutoff contains `.gitattributes` and the Pulse 29 normalization receipt,
but does not contain this authority. The authority is later.

## Mandatory pre-copy normalization gate

Custody must materialize the exact cutoff in a new isolated Git checkout with
`core.autocrlf=true`. Before copying any package file it must:

- run `git check-attr text eol -- <path>` for every byte-bound file under the
  Pulse 25 and Pulse 27 release roots;
- require `text=set` and `eol=lf` for all 36 files;
- verify 36/36 LF files and zero CR bytes;
- verify the exact Pulse 29 receipt raw digest
  `sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225`;
  and
- independently pass all 76/76 Pulse 29 bindings: 22 Pulse 25, 45 Pulse 27,
  and nine collector-identity checks.

Any failure closes the new package `invalid-before-candidates` before copy.

## Exact normalized public package

The declaration pins every normalized Pulse 25 and Pulse 27 binding, all nine
collector manifest entries, all 20 adapter manifest entries, every per-file
size and digest, all source/test/collector/release aggregates, reports,
receipts, and seals.

Only after the normalization gate passes may custody copy exactly the 20
Pulse 27 manifest-listed files into a new isolated package workspace and
independently recompute every file and aggregate digest. The nine collector
files must remain byte-identical to Pulse 25.

## Exact preflight and later generation

The package must run exactly one adapter invocation producing two
Windows/Ubuntu pairs, four rows, and two seals. Exactly two fresh platform
verifiers must enforce whole-store cardinality `2/2/2`. Retries and residue
are zero.

Only after the complete preflight passes may the program freeze a new custody
identity, seed, classifier, generator, case manifest, coverage manifest, and
fresh corpus.

Pulse 30 inherits without change the Pulse 26 coverage, oracle, search,
collection, minimization, and publication objects: eight interactions, eight
oracle fields, six target predicates, 512 cases per platform, 1,024 search
processes, one search execution, zero retries, 128 transformations, 256
minimization processes, and sanitized-reproducer or bounded
no-reproduction publication.

## Evidence

- [Normative contract](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_NORMALIZED_PUBLIC_ADAPTER.md)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-normalized-public-adapter.v1.schema.json)
- [Authorized fixture](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-normalized-public-adapter.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-normalized-public-adapter-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-30-NORMALIZED-PUBLIC-ADAPTER-AUTHORITY-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_normalized_public_adapter.rs)

## Decision

The public authority is frozen and unexecuted. No adapter, verifier, preflight,
candidate, build, generation, minimization, or result activity occurred. No
production code or dependency changed, and PLATFORM-001 remains Draft solely
because of the valid Pulse 17 `process-exit-agreement` failure.

## Stop conditions

Stop rather than widen if work would execute the authority, use another
cutoff, bypass checkout normalization, copy another package, weaken exact
preflight or inherited bounds, access prior custody, reopen a closed program,
or change PLATFORM-001 status.
