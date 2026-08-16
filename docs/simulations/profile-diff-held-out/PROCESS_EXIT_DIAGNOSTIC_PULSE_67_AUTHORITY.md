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
`p67_wsl_probe_worker.py`, `p67_wsl_probe_sealed_dependencies.py`, and
`ferris.pulse-67-wsl-probe-session/v1` schema artifact while predating the
authority. Working-tree bytes are not an identity source except for explicitly
declared complete-file LF/CRLF variants.

The historical declaration separated dynamic WSL route-equivalence from static
exact production binding, but it still claimed that the harmless probe covered
the exact current-cutoff authority/P39/repo roots and the production worker's
exact P56 loader leg.

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

### Current-cutoff authority/P39/repo fields were stale

Pulse 67 declared current-cutoff authority/P39/repo checkout identity, but the
historical declaration still pointed its current-cutoff fields at the older
Pulse 66 cutoff `3a99e9e0f383a9821297ef47778fd586b447b7ba`. Specifically,
`authority_checkout_root.revision`, `p39_checkout_root.head`,
`p39_checkout_root.revision`, and `repo_root.revision` all remained stale
instead of naming the actual Pulse 67 cutoff
`3ec6a36009fd34765508f729e795042fd610e5d4`.

### The harmless probe did not execute the claimed exact P56 loader path

The exact production Pulse 57 worker derives
`repo_root = p56_root.parents[3]`, calls `load_exact_p56(repo_root)`, and then
asserts `Path(p56.__file__).parent == p56_root` before any capability handle is
created. The historical `p67_wsl_probe_worker.py` did not do that: it checked
only direct `p56_root` equality and then immediately started the harmless
probe protocol. The historical `p67_wsl_probe_sealed_dependencies.py` likewise
validated only the staged P56 file set and returned a harmless probe result.
It never imported the exact staged P56 module and never validated the exact
callable identities without launch. Pulse 67 therefore overclaimed the exact
production worker's P56 loader leg.

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
containing this withdrawal and exact final Pulse 59 while binding every
current authority/P39/repo cutoff/head/revision field to that new cutoff and
reproducing the exact `repo_root = p56_root.parents[3]` /
`load_exact_p56(repo_root)` / `Path(p56.__file__).parent == p56_root` leg
without calling publish/build/launch.

## Evidence

- [Historical authority declaration](docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json)
- [Historical closed schema](docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json)
- [Historical mutation registry](docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority-mutations.json)
- [Authority record and closeout](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_67_AUTHORITY.md)
- [Historical authority validator](crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_authority.rs)
- [Closeout validator](crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_closeout.rs)

The historical declaration identity remains
`sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05`.
Its unchanged `28196` deterministic controls preserve the historical
artifact.
