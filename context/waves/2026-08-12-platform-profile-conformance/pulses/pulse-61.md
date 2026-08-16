# Pulse 61: witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority; no diagnostic execution

## Goal

Authorize one fresh independent single-use diagnostic over the exact final
Pulse 59 executor chain after permanently withdrawing Pulse 60's contradictory
prelaunch root contract.

## Authority

Pulse 61 binds exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities are derived from immutable cutoff Git blobs at
`70ed752359c04e4aac77a49280c37f2cf6b8d012`, which contains the Pulse 60
withdrawal and predates this authority. Runtime materialization is validated
only against those canonical identities or explicitly declared complete-file
LF/CRLF variants.

Independent future custody must obtain the authority anonymously; validate the
authority checkout and `repo_root` as the same fresh anonymous clean
`core.autocrlf=false` exact-cutoff Windows checkout; validate the caller-
supplied P39 root as a separate fresh anonymous clean `core.autocrlf=false`
exact-cutoff Windows checkout; supply `private_runtime_root` as an absolute
existing empty safe directory; supply `p27_cycle_root` as an absent direct
child of that runtime root and not `.pulse58-private-launch`; supply
`p41_final_root` as an absent absolute Windows path whose derived exact stage
root `.<final-root-name>.pulse-41-stage` and rollback/final publication path
remain absent and non-overlapping with `repo_root`, the runtime root, the P27
cycle root, the P39 checkout, and the derived Pulse 59 terminal sibling; and
supply `ubuntu_runtime_parent` as an absolute native Linux safe directory that
is not under `/mnt/*` so Pulse 56 can create its fresh child safely. The
authority is consumed on attempt. There is no retry, resume, alternate
executor, or republication route.

Pulse 61 preserves only Pulse 59's exact completed terminal classes:
`published-result`, `published-failure-witness`, and
`invalid-witness-publication`. Published transfer is restricted to the verified
path-free Pulse 59 descriptor plus the known Pulse 61 public custody roots.
Prelaunch or runtime `publication=not-attempted` closes permanently with null
conclusions and no transfer. `terminal-publication-cleanup-indeterminate` is a
fatal unresolved-custody posture.

## Permanent predecessor closure

Pulse 46 and Pulse 48 remain permanently `invalid-publication-integrity`.
Pulse 49 remains permanently withdrawn `invalid-prelaunch-authority-integrity`;
Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; and Pulse 60 remains permanently
withdrawn `invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`, with zero calls, seeds, descriptors,
processes, publications, and transfers plus null conclusions. Pulse 61 does
not revive, reinterpret, or consume any of those historical closures.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-61-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-61-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-61-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_61_AUTHORITY.md)
- [Rust authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_61_authority.rs)

The declaration identity is
`sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d`.
Its `20058` deterministic controls raise the monotonic declared-mutation total
to `139725`.
