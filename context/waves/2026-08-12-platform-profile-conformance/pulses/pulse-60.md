# Pulse 60: witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority; no diagnostic execution

## Goal

Authorize one fresh independent single-use diagnostic over the exact final
Pulse 59 executor chain without retrying, resuming, or amending permanently
closed Pulse 55.

## Authority

Pulse 60 binds exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities are derived from immutable cutoff Git blobs; runtime materialization
is validated only against those canonical identities or explicitly declared
complete-file LF/CRLF variants.

Independent future custody must obtain the authority anonymously; validate the
authority checkout and caller-supplied P39 checkout as fresh anonymous clean
`core.autocrlf=false` exact-cutoff roots; prepare fresh absent runtime/P27/P41
roots; supply a native Ubuntu runtime parent; and then call the exact Pulse 59
production API once. The authority is consumed on attempt. There is no retry,
resume, alternate executor, or republication route.

Pulse 60 preserves only Pulse 59's exact completed terminal classes:
`published-result`, `published-failure-witness`, and
`invalid-witness-publication`. Published transfer is restricted to the verified
path-free Pulse 59 descriptor plus the known Pulse 60 public custody roots.
Prelaunch or runtime `publication=not-attempted` closes permanently with null
conclusions and no transfer. `terminal-publication-cleanup-indeterminate` is a
fatal unresolved-custody posture.

## Permanent predecessor closure

Pulse 46 and Pulse 48 remain permanently `invalid-publication-integrity`.
Pulse 49 remains permanently withdrawn `invalid-prelaunch-authority-integrity`;
Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; and Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`. Pulse 60 does not revive, reinterpret,
or consume any of those historical closures.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-60-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-60-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-60-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_60_AUTHORITY.md)
- [Rust authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_60_authority.rs)

The declaration identity is
`sha256:13ba3aaa5d61c536a9dd22b3a57816b1b7d93c2e11592c87117190709cbfb40c`.
Its `19085` deterministic controls raise the monotonic declared-mutation total
to `119667`.
