# Pulse 60: witnessed capability/materialization diagnostic authority

Status: Permanently withdrawn invalid-prelaunch-runtime-root-contract

## Goal

Record the historical Pulse 60 authority and its permanent prelaunch
withdrawal without executing any Pulse 59, Pulse 58, or FERRIS diagnostic
callable.

## Historical authority

Pulse 60 bound exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities were derived from immutable cutoff Git blobs, and runtime
materialization was limited to those identities or explicitly declared
complete-file LF/CRLF variants.

The sealed declaration required anonymous exact-cutoff `core.autocrlf=false`
authority and P39 checkouts, one exact Pulse 59 production call, path-free
terminal custody transfer, permanent null-conclusion `not-attempted` closeout,
and fatal unresolved-custody cleanup posture.

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

## Independent prelaunch contract review (2026-08-16)

The exact cutoff Pulse 58, Pulse 59, Pulse 52, Pulse 41, Pulse 57, and Pulse
56 helpers were re-audited without invoking any authority or diagnostic
callable. That review proved Pulse 60's sealed root contract contradicted the
exact callable stack in three ways:

1. `private_runtime_root` was declared `fresh-absent`, but Pulse 58 first calls
   `p51._safe_runtime_root(...)` and then requires an existing empty safe
   runtime directory;
2. `p27_cycle_root` was declared merely `fresh`, but Pulse 58 requires an
   absent direct child of the runtime root; and
3. Pulse 60 underbound the exact Pulse 41/Pulse 59 final-stage-terminal root
   separation that the one-call route needs to avoid prelaunch or
   post-cleanup `not-attempted` closure.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 call was invoked. No
runtime root, seed, descriptor, candidate process, publication root, result
tree, witness tree, or transfer artifact was created.

## Permanent closeout

Pulse 60 is permanently withdrawn before launch under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`. Every call, seed, descriptor, process,
publication, transfer, result, witness, and conclusion count remains zero or
null. Pulse 60 is non-retryable, non-resumable, and cannot be amended,
reinterpreted, or consumed. Any successor must use a new immutable cutoff that
contains this withdrawal and predates the successor authority.

## Evidence

- [Historical authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-60-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-60-authority.v1.schema.json)
- [Historical mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-60-authority-mutations.json)
- [Authority record and closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_60_AUTHORITY.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_60_authority.rs)
- [Closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_60_closeout.rs)

The historical declaration identity is
`sha256:13ba3aaa5d61c536a9dd22b3a57816b1b7d93c2e11592c87117190709cbfb40c`.
Its unchanged `19085` deterministic controls preserve the monotonic declared
mutation total at `119667`.
