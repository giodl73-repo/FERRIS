# Pulse 43: Ordered public-result publisher release

Status: Complete
Implementation authority: Public infrastructure, documentation, and test-only
validation only

## Goal

Release the smallest complete public ordered-result publisher that prevents a
terminal public summary from existing without its claimed result directory and
keeps public artifact/self-validation counts distinct from ordered execution.

## Immutable boundary

Pulse 42 remains permanently `invalid-publication-integrity`, non-retryable,
and null-conclusion at `public-result-publication`. Pulse 43 neither retries,
reconstructs, nor reinterprets Pulse 42 or an earlier diagnostic. It creates
no diagnostic, custody, product, category, fix, score, certification,
support, or PLATFORM-001 authority and accesses no private data.

## Released control

The standard-library publisher accepts only a bounded, predeclared public gate
catalog and closed explicit event records. Stable event classifications are
`public-artifact-self-validation` and `ordered-execution`. Ordered records
consume catalog gates once and in sequence and have one `terminal-stop`;
earlier missing gates, duplicate gates, unknown gates, incomplete completion,
or execution after a failure/stop are rejected. Self-validation uses separate
derived counters and cannot advance execution.

The publisher canonicalizes and hashes result and receipt payloads, writes
both staged files with file fsync, verifies them, synchronizes the stage,
renames exactly once, independently verifies the final directory and hashes,
then synchronizes the final parent before a `published` summary. It has zero
retries and no fallback. Failure reports are exclusively `absent`,
`rolled-back`, or `indeterminate`; an unproven rollback is never success.
Windows directory sync is documented as explicitly `unsupported`, not
durable.

## Evidence

- [Public release](../../../../docs/simulations/profile-diff-held-out/pulse-43-ordered-result-publisher-release/README.md)
- [Sealed manifest](../../../../docs/simulations/profile-diff-held-out/pulse-43-ordered-result-publisher-release/public-manifest.json)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-43-ordered-result-publisher-release/qualification-receipt.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-43-ORDERED-RESULT-PUBLISHER-RELEASE-ROLE-REVIEW.md)
- [Rust integration validator](../../../../crates/ferris-cli/tests/pulse_43_ordered_result_publisher_release.rs)

The release identities and qualification counts are sealed in its manifest,
receipt, seal, and Rust validator.

Manifest raw / aggregate:
`sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4` /
`sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346`.
Qualification receipt raw / payload:
`sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c` /
`sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2`.
Release seal raw / payload:
`sha256:4445a0b181419b303c28f1d91e1700a594d2a040fbae1cd0dc526fa7fc8e8f05` /
`sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1`.
