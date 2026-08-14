# Pulse 44: Retained-binary custody release

Status: Complete
Implementation authority: Public build/custody infrastructure, documentation,
and test-only validation only

## Goal

Publish the smallest complete public retained-binary custody adapter that
closes the Pulse 33 retained-executable durability and atomic-pair gap without
reinterpreting the invalid Pulse 42 summary.

## Immutable boundary

Pulse 42 remains permanently invalid, non-retryable, and null-conclusion.
Its summary is not evidence that its claimed Pulse 33 transaction occurred.
Pulse 44 neither resumes nor grants diagnostic authority. It does not execute
a diagnostic, access private data, retain a committed executable, or make a
product, category, fix, score, certification, support, or PLATFORM-001
conclusion.

## Released control

The standard-library adapter pins exact Pulse 33 release identities and cutoff
before invoking its immutable public `build_and_freeze` function once with
retention enabled. It accepts one platform and fresh absent absolute work and
final roots; validates the retained executable and receipt; fsyncs and
verifies both staged files; records honest directory sync; renames exactly
once; reconstructs and verifies the final pair; then synchronizes the final
parent. All failure outputs are terminal and only absent, rolled-back, or
indeterminate. A Pulse-43-compatible ordered event can become completed only
after final pair verification.

## Evidence

- [Public release](../../../../docs/simulations/profile-diff-held-out/pulse-44-retained-binary-custody-release/README.md)
- [Sealed manifest](../../../../docs/simulations/profile-diff-held-out/pulse-44-retained-binary-custody-release/public-manifest.json)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-44-retained-binary-custody-release/qualification-receipt.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-44-RETAINED-BINARY-CUSTODY-RELEASE-ROLE-REVIEW.md)
- [Rust integration validator](../../../../crates/ferris-cli/tests/pulse_44_retained_binary_custody_release.rs)

The release is sealed by its manifest, qualification receipt, and release seal.
Actual Windows qualification first rejected a dirty clone created before
`core.autocrlf=false` was fixed, then independently passed from a clone
normalized before checkout: retained executable/receipt `2/2`, one rename,
zero retries, and exact Pulse 33 artifact identity. All runtime artifacts were
removed after recording; Pulse 44 synthetic controls cover every transaction
failure branch.
