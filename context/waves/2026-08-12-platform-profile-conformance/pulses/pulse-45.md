# Pulse 45: Binary-custody event bridge release

Status: Complete
Implementation authority: Public composition adapter, records, documentation,
and test-only validation only

## Goal

Publish the smallest complete public bridge that permits independently
completed Pulse 44 Windows and Ubuntu retained-binary results to act as
intermediate Pulse 43 ledger gates without changing the sealed Pulse 44
terminal result.

## Immutable boundary

Pulse 44 remains sealed and unchanged. Pulse 45 validates its complete public
summary after exactly one adapter invocation per platform and maps only its
fully published `2/2` result to a platform-specific Pulse 43
`gate-complete/passed`. It neither executes a diagnostic nor grants custody,
private-data, product, category, fix, support, certification, or PLATFORM-001
authority.

Any closed Pulse 44 failure with `absent`, `rolled-back`, or `indeterminate`
posture is preserved as a platform-specific Pulse 43
`terminal-stop/failed`. Malformed, partial, or thrown results cannot become a
pass. There are zero retries, fallbacks, path-bearing output fields, or
executable bytes in the public bridge result.

## Evidence

- [Public release](../../../../docs/simulations/profile-diff-held-out/pulse-45-binary-custody-event-bridge-release/README.md)
- [Sealed manifest](../../../../docs/simulations/profile-diff-held-out/pulse-45-binary-custody-event-bridge-release/public-manifest.json)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-45-binary-custody-event-bridge-release/qualification-receipt.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-45-BINARY-CUSTODY-EVENT-BRIDGE-RELEASE-ROLE-REVIEW.md)
- [Rust integration validator](../../../../crates/ferris-cli/tests/pulse_45_binary_custody_event_bridge_release.rs)

The release is sealed by its manifest, qualification receipt, and release
seal. Its callable test boundary injects Pulse 44; its CLI verifies and imports
the exact sealed Pulse 44 release before forwarding the unchanged
`repo`, `cutoff`, `platform`, `work-root`, and `final-root` arguments once.
