# Pulse 63: witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority; no diagnostic execution

## Goal

Authorize one fresh independent single-use diagnostic over the exact final
Pulse 59 executor chain after permanently withdrawing Pulse 62's invalid
synthetic path and missing WSL-route contract.

## Authority

Pulse 63 binds exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact
P27/P31/P35/P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and
only `run_witness_preserving_capability_materialization_executor`. Canonical
identities are derived from immutable cutoff Git blobs at
`5ad78a0623611ad57797ec4e9da34345b40a6e38`, which contains the Pulse 62
withdrawal and predates this authority. Runtime materialization is validated
only against those canonical identities or explicitly declared complete-file
LF/CRLF variants.

Independent future custody must obtain the authority anonymously; validate the
authority checkout and `repo_root` as the same fresh anonymous clean
`core.autocrlf=false` exact-cutoff Windows checkout; validate the caller-
supplied P39 root as a separate fresh anonymous clean `core.autocrlf=false`
exact-cutoff Windows checkout; and complete all mandatory public path and route
qualification before the sole Pulse 59 call and before any seed.

The pre-call contract is authority-bound, public-constant-derived, and
non-consuming. It MUST:

1. use the actual caller-supplied basenames for the exact P41 final/stage/
   rollback names and the exact Pulse 59 terminal sibling
   `<private_runtime_root.name>.pulse59-terminal-publication`;
2. prove the deepest exact Pulse 39 copy path
   `tests/test_checkout_verifier.py` under the actual P41 basename-derived
   topology with same-filesystem rename and rollback cleanup;
3. prove the exact Pulse 58 namespace leaf names plus the exact-length
   conservative Pulse 56 Windows and Ubuntu runtime topologies under the real
   parents, including deepest checkout paths, conservative `work/target-*`
   paths, retained artifact/receipt names, `launches/l-...`, `home`, and
   `tmp`, or fail before call if no exact or explicit bound can be proven;
4. run exactly one harmless WSL route preflight by resolving Windows
   `%SystemRoot%\\System32\\wsl.exe` and invoking exactly
   `--distribution Ubuntu-24.04 --exec /usr/bin/python3 -I -S -B` with an
   isolated bounded script that revalidates native `ubuntu_runtime_parent`,
   rejects `/mnt/*`, verifies platform/Python identity, creates/fsyncs/removes
   the exact `.p57-*` and `.p56-*` probe topologies, and emits only canonical
   bounded JSON; and
5. stop before seed and before the sole Pulse 59 invocation on any path or WSL
   route preflight failure.

Pulse 63 preserves only Pulse 59's exact completed terminal classes:
`published-result`, `published-failure-witness`, and
`invalid-witness-publication`. Published transfer is restricted to the verified
path-free Pulse 59 descriptor plus the known Pulse 63 public custody roots.
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
`P55-P33-RETAINED-IDENTITY-CONTRACT`; Pulse 60 remains permanently withdrawn
`invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`; Pulse 61 remains permanently withdrawn
`invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`; and Pulse 62 remains permanently
withdrawn `invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`, with zero calls, seeds, descriptors,
processes, publications, and transfers plus null conclusions. Pulse 63 does
not revive, reinterpret, or consume any of those historical closures.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-63-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-63-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-63-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_63_AUTHORITY.md)
- [Rust authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_63_authority.rs)

The declaration identity is
`sha256:b8cfea5cc8cb6dc52a7974f4fee35f6351557158943cc92af388c534421915d5`.
Its `23266` deterministic controls raise the monotonic declared-mutation total
to `184635`.
