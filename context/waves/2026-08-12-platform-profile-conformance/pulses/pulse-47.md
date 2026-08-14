# Pulse 47: Public publication-outcome witness release

Status: Complete
Implementation authority: Public wrapper, sealed records, documentation, and
test-only validation only

## Goal

Release the smallest complete public witness that permanently records one
closed Pulse 43 publication outcome only after its own independent two-file
transaction succeeds.

## Immutable boundary

Pulse 46 remains permanently `invalid-publication-integrity`, non-retryable,
and null-conclusion. Pulse 47 is not a Pulse 46 retry, resume,
reconstruction, correlation, or inference, and it records no Pulse 46
diagnostic, gate, count, or private material. It creates no diagnostic,
execution, custody, product, category, score, certification, support, fix, or
PLATFORM-001 authority.

## Released control

The standard-library wrapper pins the exact sealed Pulse 43 manifest, receipt,
seal, and source identities before its real callable import. Its callable
boundary accepts only Pulse 43 catalog/events/final-root arguments plus a
separate fresh absent absolute witness root, and invokes the predecessor
exactly once. It validates the complete closed predecessor summary.

Published Pulse 43 summaries yield only public result hashes, final `2/2`,
rename/retry/all-sync posture, and ordered/self-validation aggregate summaries.
Failure summaries yield only failure code, absent/rolled-back/indeterminate
state, final-files flag, rename/retry values, and exact stage/final-parent/
rollback-parent sync postures. No failure witness contains ordered events,
gate counts, self-validation values, paths, private data, or executable
bytes. Malformed, partial, success-shaped incomplete, and thrown predecessor
output fail closed.

The witness canonicalizes and hashes its closed payload, file-fsyncs,
stage-verifies, stage-syncs, renames once, independently final-rehashes, and
parent-syncs exactly `publication-witness.json` and `release-receipt.json`.
It has zero retry/fallback. A failed witness transaction returns only its
bounded absent/rolled-back/indeterminate posture and code, with no captured
Pulse 43 field.

## Evidence

- [Public release](../../../../docs/simulations/profile-diff-held-out/pulse-47-publication-outcome-witness-release/README.md)
- [Sealed manifest](../../../../docs/simulations/profile-diff-held-out/pulse-47-publication-outcome-witness-release/public-manifest.json)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-47-publication-outcome-witness-release/qualification-receipt.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-47-PUBLICATION-OUTCOME-WITNESS-RELEASE-ROLE-REVIEW.md)
- [Rust integration validator](../../../../crates/ferris-cli/tests/pulse_47_publication_outcome_witness_release.rs)

Manifest raw / aggregate:
`sha256:44d5c72b9eb09dc7e24b476a4535fed662eadde3edee6ecbfe1fdfa644082f8b` /
`sha256:5cb97276ee2752888c40d44a50e45079c9e550f7e26398e5aa4841d98083143d`.
Qualification receipt raw / payload:
`sha256:be73ee9a87377e58a87c04308557ef118afbb7ed0fb117b039cc569f9040b265` /
`sha256:dbe44afbb9f0ad43549113028da8dc5d2d0ca5fe9faa15824d7cd80e3edea355`.
Release seal raw / payload:
`sha256:4300f5ba89bdaefb938b91092adf7d1c62dbf11ba6e1a4350c9a34c03cce1a8e` /
`sha256:a00478e73897781ddd88e8e0fcbca2d1453a72758cbbd8ec06ccd9d0c228f681`.

The sealed release has six payload files, nine release-tree files, 64,779
payload bytes, 17 Python test methods, one real Pulse 43 representative
success, all three Pulse 43 failure postures including indeterminate with
failed rollback sync, and zero
retries/fallbacks.
