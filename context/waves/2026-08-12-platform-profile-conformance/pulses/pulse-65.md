# Pulse 65: witnessed capability/materialization diagnostic authority

Status: Permanently withdrawn invalid-prelaunch-wsl-spawn-cardinality-contract

## Goal

Record the historical Pulse 65 authority and its permanent prelaunch
withdrawal without executing any Pulse 59, Pulse 58, Pulse 57, Pulse 56, or
FERRIS diagnostic callable.

## Historical authority

Pulse 65 bound exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities were derived from immutable cutoff Git blobs at self-excluding
cutoff `e3b0b62f6dd62b5071886d32a9eedca85c76b4ae`, which contains the Pulse 64
withdrawal and predates the authority. Runtime materialization was limited to
those identities or explicitly declared complete-file LF/CRLF variants.

The sealed declaration required anonymous exact-cutoff `core.autocrlf=false`
authority and Pulse 39 checkouts, one exact Pulse 59 production call, actual
caller-basename and conservative exact-topology public qualification for the
exact Pulse 41/P56/P57/P58/P59 stack, one exact WSL qualification before any
possible sole Pulse 59 call, path-free terminal custody transfer,
null-conclusion `not-attempted` closeout, and fatal unresolved-custody
cleanup posture.

## Permanent predecessor closure

Pulse 46 and Pulse 48 remain permanently `invalid-publication-integrity`.
Pulse 49 remains permanently withdrawn `invalid-prelaunch-authority-integrity`;
Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; Pulse 60 remains permanently withdrawn
`invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`; Pulse 61 remains permanently withdrawn
`invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`; Pulse 62 remains permanently
withdrawn `invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`; Pulse 63 remains permanently withdrawn
`invalid-prelaunch-wsl-bootstrap-contract` under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`; and Pulse 64 remains permanently
withdrawn `invalid-prelaunch-unbound-wsl-qualification-contract` under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`. Pulse 65 does not revive,
reinterpret, or consume any of those historical closures.

## Independent prelaunch exact WSL spawn-cardinality review (2026-08-16)

The exact cutoff Pulse 59, Pulse 58, Pulse 57, Pulse 56, Pulse 52, Pulse 41,
Pulse 39, Pulse 51, Pulse 57 `wsl_session_worker.py`, and all exact cutoff
API bindings were re-audited without invoking any authority or diagnostic
callable. That review proved Pulse 65's sealed WSL qualification still failed
to bind the exact Pulse 57 implementation path in one dispositive way.

### Exact WSL preflight cardinality remained underbound

Pulse 65 declared one harmless bounded proof could establish the exact Pulse
57 stage-bundle and worker-bootstrap route, and its declaration fixed
`single_wsl_process_spawn` at true. The exact cutoff Pulse 57 source instead
uses two separate WSL processes with distinct contracts:

1. `_stage_wsl_bundle` calls `subprocess.run(...)` once to send the exact
   bundle payload over stdin, require empty stderr, bound stdout to one
   canonical JSON line, and prove the exact staged `bundle_root`;
2. `_NativeWslSession` then calls `subprocess.Popen(...)` to start the worker,
   requires a separate exact ready message, and later exercises distinct
   bounded `close`, `wait`, `terminate`, `kill`, stdout-drain, and stderr-drain
   cleanup behavior.

One spawn therefore cannot simultaneously prove both exact routes, both
separate process handles, the exact ready/close lifecycle, or the absence of a
hidden second or third WSL spawn. Pulse 65 did not bind a distinct second
harmless worker-bootstrap proof and thus remained underbound before the sole
Pulse 59 call.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, probe tree, seed, descriptor, candidate process, publication
root, result tree, witness tree, or transfer artifact was created.

## Permanent closeout

Pulse 65 is permanently withdrawn before launch under
`P65-P57-WSL-TWO-SPAWN-CONTRACT`. Every call, seed, descriptor, process,
publication, transfer, result, witness, and conclusion count remains zero or
null. Pulse 65 is non-retryable, non-resumable, and cannot be amended,
reinterpreted, or consumed. Any successor must use a new immutable cutoff
containing this withdrawal and exact final Pulse 59 while separately binding
the exact stage-bundle and worker-bootstrap WSL spawns.

## Evidence

- [Historical authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-65-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-65-authority.v1.schema.json)
- [Historical mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-65-authority-mutations.json)
- [Authority record and closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_65_AUTHORITY.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_65_authority.rs)
- [Closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_65_closeout.rs)

The historical declaration identity is
`sha256:5bd7c876180a3bfb9f0bcb1518ef68921d1b28210d1f717c904753508e28abb0`.
Its unchanged `25815` deterministic controls preserve the historical
artifact.
