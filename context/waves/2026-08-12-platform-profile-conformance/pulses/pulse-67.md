# Pulse 67: witnessed capability/materialization diagnostic authority

Status: Permanently withdrawn invalid-prelaunch-cutoff-probe-claim-contract

## Goal

Record the historical Pulse 67 authority and its permanent prelaunch
withdrawal without executing any Pulse 59, Pulse 58, Pulse 57, Pulse 56, or
FERRIS diagnostic callable.

## Historical authority

Pulse 67 bound exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 public release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities derive only from immutable Git blobs at cutoff
`3ec6a36009fd34765508f729e795042fd610e5d4`, which contains the Pulse 66
withdrawal, exact final Pulse 59, and the historical
`p67_wsl_probe_worker.py` / `p67_wsl_probe_sealed_dependencies.py` /
`ferris.pulse-67-wsl-probe-session/v1` probe bundle while predating the
authority. Working-tree bytes are not an identity source except for explicitly
declared complete-file LF/CRLF variants.

The historical declaration required fresh anonymous exact-cutoff
`core.autocrlf=false` authority and Pulse 39 checkouts, separate independent
checkout validation, one exact Pulse 59 production call, actual-caller and
conservative exact-topology public qualification, path-free terminal custody
transfer, permanent null-conclusion `not-attempted` closeout, and fatal
unresolved-custody cleanup posture. It additionally required exactly two
harmless bounded WSL spawns before the sole Pulse 59 call: one exact Pulse 57
stage-bundle `subprocess.run(...)` proof and one exact Pulse 57 worker
bootstrap `subprocess.Popen(...)` proof over a separate harmless probe worker
and dependency bundle.

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
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`; Pulse 65 remains permanently
withdrawn `invalid-prelaunch-wsl-spawn-cardinality-contract` under
`P65-P57-WSL-TWO-SPAWN-CONTRACT`; and Pulse 66 remains permanently withdrawn
`invalid-prelaunch-wsl-probe-bundle-contract` under
`P66-WORKER-HASH-BUNDLE-LIFETIME`.

## Independent prelaunch cutoff/probe review (2026-08-16)

### Authority, P39, and repo current-cutoff fields still pointed at Pulse 66

Pulse 67 declared current-cutoff authority/P39/repo checkout identity, but
its sealed current-cutoff fields still named the prior Pulse 66 cutoff
`3a99e9e0f383a9821297ef47778fd586b447b7ba`. In the historical declaration,
`authority_checkout_root.revision`, `p39_checkout_root.head`,
`p39_checkout_root.revision`, and `repo_root.revision` all remained bound to
that older cutoff instead of the actual Pulse 67 cutoff
`3ec6a36009fd34765508f729e795042fd610e5d4`. The exact current-cutoff
authority/P39/repo checkout contract was therefore false before launch.

### Harmless probe still overclaimed the exact P56 loader leg

Pulse 67 also claimed that its harmless probe dynamically reproduced the exact
production worker's P56 loader leg, including repo-root parent derivation and
expected-root enforcement. But the historical `p67_wsl_probe_worker.py`
checked only direct `p56_root` equality and never derived
`repo_root = p56_root.parents[3]`, never invoked
`load_exact_p56(repo_root)`, and never asserted
`Path(p56.__file__).parent == p56_root`. The historical
`p67_wsl_probe_sealed_dependencies.py` only validated the staged P56 file set
and returned a harmless probe result; it did not import the exact staged P56
module or validate its callable surface. Pulse 67 therefore overclaimed the
production worker's exact P56 loader leg.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, probe tree, seed, descriptor, candidate process, publication
root, result tree, witness tree, or transfer artifact was created.

## Permanent closeout

Pulse 67 is now permanently withdrawn before launch under
`P67-ROOT-CUTOFF-P56-LOADER-CONTRACT`. Every call, seed, descriptor, process,
publication, transfer, result, witness, and conclusion count remains zero or
null. Pulse 67 is non-retryable, non-resumable, and cannot be amended,
reinterpreted, or consumed. Any successor must use a new immutable cutoff
containing this withdrawal and exact final Pulse 59, bind every current
authority/P39/repo cutoff/head/revision field to that new cutoff, and
reproduce the exact `repo_root = p56_root.parents[3]` /
`load_exact_p56(repo_root)` / `Path(p56.__file__).parent == p56_root` worker
leg without calling publish/build/launch.

## Evidence

- [Historical authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json)
- [Historical mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority-mutations.json)
- [Authority record and closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_67_AUTHORITY.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_authority.rs)
- [Closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_closeout.rs)

The historical declaration identity is
`sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05`.
Its unchanged `28196` deterministic controls preserve the historical
artifact.
