# Pulse 62: witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority; no diagnostic execution

## Goal

Authorize one fresh independent single-use diagnostic over the exact final
Pulse 59 executor chain after permanently withdrawing Pulse 61's inadequate
safe-parent contract and adding mandatory reversible pre-call creatability
qualification.

## Authority

Pulse 62 binds exact final Pulse 59 HEAD `6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact
P27/P31/P35/P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and
only `run_witness_preserving_capability_materialization_executor`. Canonical
identities are derived from immutable cutoff Git blobs at `e38dd20f37923e84ac3a3377892c1a5d0954266a`, which
contains the Pulse 61 withdrawal and predates this authority. Runtime
materialization is validated only against those canonical identities or
explicitly declared complete-file LF/CRLF variants.

Independent future custody must obtain the authority anonymously; validate the
authority checkout and `repo_root` as the same fresh anonymous clean
`core.autocrlf=false` exact-cutoff Windows checkout; validate the caller-
supplied P39 root as a separate fresh anonymous clean `core.autocrlf=false`
exact-cutoff Windows checkout; supply the same exact `private_runtime_root`,
`p27_cycle_root`, `p41_final_root`, and `ubuntu_runtime_parent` surfaces Pulse
61 named; and complete all mandatory reversible creatability probes before the
sole Pulse 59 call and before any seed.

The pre-call probe protocol is authority-bound, public-constant-derived, and
non-consuming. It MUST:

1. use only the public stem `e38dd20f3792-6945f5fc9686` and never derive any probe name from a
   private seed;
2. create restrictive-permission probe directories/files, fsync files and
   directories where supported, verify stable non-symlink/non-reparse identity,
   remove the probes, sync the parent, and verify complete absence;
3. prove `private_runtime_root` can create/remove both a Pulse 58 namespace-like
   child and a Pulse 56 Windows `.p56-*`-like child without colliding with real
   stack names;
4. prove the Pulse 41 final parent can create the exact stage/final probe
   topology, rename stage to final on the same filesystem, preserve sufficient
   path-length headroom, and delete the final probe cleanly;
5. prove the Pulse 59 terminal parent can create/remove the exact sibling-like
   terminal probe; and
6. prove `ubuntu_runtime_parent` can create/remove both a Pulse 57 `.p57-*`-
   like child and a Pulse 56 Ubuntu `.p56-*`-like child while satisfying the
   safe pre-call executable/noexec posture required by the native Linux route.

Any probe failure is a pre-call stop that leaves publication `not-attempted`,
creates no seed, descriptor, process, publication, or transfer artifact, and
must occur before the sole Pulse 59 invocation.

Pulse 62 preserves only Pulse 59's exact completed terminal classes:
`published-result`, `published-failure-witness`, and
`invalid-witness-publication`. Published transfer is restricted to the verified
path-free Pulse 59 descriptor plus the known Pulse 62 public custody roots.
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
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`; and Pulse 61 remains permanently
withdrawn `invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`, with zero calls, seeds,
descriptors, processes, publications, and transfers plus null conclusions.
Pulse 62 does not revive, reinterpret, or consume any of those historical
closures.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-62-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-62-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_62_AUTHORITY.md)
- [Rust authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_62_authority.rs)

The declaration identity is `sha256:f0db3ddf18a796d0ec107d6d73e9a08cf5e59d47cdad880d584ee8c7e8f61c5a`.
Its `21644` deterministic controls raise the monotonic declared-
mutation total to `161369`.
