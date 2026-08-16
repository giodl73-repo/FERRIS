# Pulse 66: witnessed capability/materialization diagnostic authority

Status: Permanently withdrawn invalid-prelaunch-wsl-probe-bundle-contract

## Goal

Record the historical Pulse 66 authority and its permanent prelaunch
withdrawal without executing any Pulse 59, Pulse 58, Pulse 57, Pulse 56, or
FERRIS diagnostic callable.

## Historical authority

Pulse 66 bound exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities were derived only from immutable cutoff Git blobs at self-excluding
cutoff `3a99e9e0f383a9821297ef47778fd586b447b7ba`, which contains the Pulse 65
withdrawal, exact final Pulse 59, and the historical
`p66_wsl_probe_sealed_dependencies.py` helper while predating the authority.
Runtime materialization was limited to those identities or explicitly declared
complete-file LF/CRLF variants.

The sealed declaration required fresh anonymous exact-cutoff
`core.autocrlf=false` authority and Pulse 39 checkouts, separate independent
checkout validation, one exact Pulse 59 production call, actual-caller and
conservative exact-topology public qualification, path-free terminal custody
transfer, permanent null-conclusion `not-attempted` closeout, and fatal
unresolved-custody cleanup posture. It additionally required exactly two
harmless bounded WSL spawns before the sole Pulse 59 call: one exact Pulse 57
stage-bundle `subprocess.run(...)` proof and one exact Pulse 57 worker
bootstrap `subprocess.Popen(...)` proof over a fake-dependency probe bundle.

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
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`; Pulse 64 remains permanently
withdrawn `invalid-prelaunch-unbound-wsl-qualification-contract` under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`; and Pulse 65 remains permanently
withdrawn `invalid-prelaunch-wsl-spawn-cardinality-contract` under
`P65-P57-WSL-TWO-SPAWN-CONTRACT`. Pulse 66 does not revive, reinterpret, or
consume any of those historical closures.

## Independent prelaunch exact WSL probe-bundle review (2026-08-16)

The exact cutoff Pulse 59, Pulse 58, Pulse 57, Pulse 56, Pulse 52, Pulse 41,
Pulse 39, Pulse 51, Pulse 57 `wsl_session_worker.py`, and all exact cutoff API
bindings were re-audited without invoking any authority or diagnostic
callable. That review proved Pulse 66's sealed WSL preflight still failed to
bind the exact Pulse 57 implementation path in two dispositive ways.

### Exact worker hash rejected the fake dependency bundle

Pulse 66 declared that spawn 2 would launch the exact cutoff Pulse 57
`wsl_session_worker.py` while staging
`docs/simulations/profile-diff-held-out/fixtures/p66_wsl_probe_sealed_dependencies.py`
as `worker/sealed_dependencies.py`. The exact worker hard-binds
`SEALED_DEPENDENCIES_SHA256` to the production
`pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py`
bytes, and `_load_sealed_dependencies(...)` rejects any other file before the
ready message is published. The declared fake dependency therefore cannot reach
the claimed ready/close lifecycle, cannot prove exact worker-bootstrap or
source-loader semantics, and cannot truthfully satisfy the historical spawn-2
contract.

### Spawn 1 cleanup contradicted the required bundle handoff

Pulse 66 also declared that spawn 1 would remove the staged bundle in the same
invocation with verified absence, while spawn 2 simultaneously required that
same `bundle_root` to remain available as the pre-staged worker probe bundle.
The declaration therefore contradicted bundle lifetime: it cannot both verify
absence before spawn 2 and hand the same bundle to spawn 2. A truthful
successor must preserve the staged bundle through ready/probe/close and
wait/terminate/kill handling, then perform one cleanup and absence
verification only after both WSL spawns finish.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, probe tree, seed, descriptor, candidate process, publication
root, result tree, witness tree, or transfer artifact was created.

## Permanent closeout

Pulse 66 is permanently withdrawn before launch under
`P66-WORKER-HASH-BUNDLE-LIFETIME`. Every call, seed, descriptor, process,
publication, transfer, result, witness, and conclusion count remains zero or
null. Pulse 66 is non-retryable, non-resumable, and cannot be amended,
reinterpreted, or consumed. Any successor must use a new immutable cutoff
containing this withdrawal and exact final Pulse 59 while separately binding
(a) exact production Pulse 57 worker/dependency identities and callable route
by cutoff bytes and (b) a separate harmless sealed probe worker/dependency
bundle whose lifetime spans both WSL spawns and whose cleanup occurs once
after spawn 2.

## Evidence

- [Historical authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-66-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-66-authority.v1.schema.json)
- [Historical mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-66-authority-mutations.json)
- [Authority record and closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_66_AUTHORITY.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_66_authority.rs)
- [Closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_66_closeout.rs)

The historical declaration identity is
`sha256:2cf44e16b0c61d79ed5ac889ab6fbfe46ee693ce6d9ccf2b4528bb877db45034`.
Its unchanged `27156` deterministic controls preserve the historical
artifact.
