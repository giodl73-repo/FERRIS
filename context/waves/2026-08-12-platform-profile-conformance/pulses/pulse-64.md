# Pulse 64: witnessed capability/materialization diagnostic authority

Status: Permanently withdrawn invalid-prelaunch-unbound-wsl-qualification-contract

## Goal

Record the historical Pulse 64 authority and its permanent prelaunch
withdrawal without executing any Pulse 59, Pulse 58, Pulse 57, or FERRIS
diagnostic callable.

## Historical authority

Pulse 64 bound exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities were derived from immutable cutoff Git blobs, and runtime
materialization was limited to those identities or explicitly declared
complete-file LF/CRLF variants.

The sealed declaration required anonymous exact-cutoff `core.autocrlf=false`
authority and P39 checkouts, one exact Pulse 59 production call, path-free
terminal custody transfer, permanent null-conclusion `not-attempted`
closeout, fatal unresolved-custody cleanup posture, actual caller-basename
plus conservative exact-topology qualification for the exact Pulse 41/P56/P57/
P58/P59 stack, and one exact Pulse 57 WSL qualification before any possible
sole Pulse 59 call.

## Permanent predecessor closure

Pulse 46 and Pulse 48 remain permanently `invalid-publication-integrity`.
Pulse 49 remains permanently withdrawn `invalid-prelaunch-authority-integrity`;
Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; Pulse 60 remains permanently
withdrawn `invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`; Pulse 61 remains permanently
withdrawn `invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`; Pulse 62 remains permanently
withdrawn `invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`; and Pulse 63 remains permanently
withdrawn `invalid-prelaunch-wsl-bootstrap-contract` under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`. Pulse 64 does not revive,
reinterpret, or consume any of those historical closures.

## Independent prelaunch exact WSL qualification review (2026-08-16)

The exact cutoff Pulse 59, Pulse 58, Pulse 57, Pulse 56, Pulse 52, Pulse 41,
Pulse 39, Pulse 51, Pulse 57 `wsl_session_worker.py`, and all exact cutoff
API bindings were re-audited without invoking any authority or diagnostic
callable. That review proved Pulse 64's sealed WSL qualification still failed
to bind the exact Pulse 57 route in two ways:

1. The declared optional public qualification callable remained nonexistent and
   unbound. Pulse 64 permitted the same exact stage-bundle route or the public
   callable `qualify_exact_p57_wsl_bootstrap_contract`, but no bound Pulse 27/
   P31/P35/P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 source at cutoff
   defines that callable. The only exact implemented path to a native Ubuntu
   worker remains Pulse 57's `_stage_wsl_bundle` plus `_NativeWslSession`
   construction inside `run_capability_bound_diagnostic_executor`, later
   consumed by the exact Pulse 58/Pulse 59 stack. The optional authority branch
   therefore could not be selected, byte-bound, or independently verified.
2. The declared Windows path values remained literal `%SystemRoot%`
   placeholders rather than exact source-precedence derivations. Pulse 57's
   exact cutoff source reads
   `os.environ.get("SystemRoot") or os.environ.get("SYSTEMROOT")`, derives
   `system32 = os.fspath(Path(system_root) / "System32")`, derives `ComSpec`
   from `Path(system32) / "cmd.exe"`, and derives `_wsl_executable()` from
   `Path(system_root) / "System32" / "wsl.exe"`. Pulse 64 instead published
   literal `%SystemRoot%\\System32\\wsl.exe`,
   `%SystemRoot%\\System32\\cmd.exe`, `%SystemRoot%\\System32`, and
   `%SystemRoot%` strings rather than an exact concrete-value derivation and
   comparison contract, so it did not prove safe absolute Windows-directory
   validation, concrete path equality, or exact regular non-reparse identities
   before the sole Pulse 59 call.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, probe tree, seed, descriptor, candidate process, publication
root, result tree, witness tree, or transfer artifact was created.

## Permanent closeout

Pulse 64 is permanently withdrawn before launch under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`. Every call, seed, descriptor,
process, publication, transfer, result, witness, and conclusion count remains
zero or null. Pulse 64 is non-retryable, non-resumable, and cannot be
amended, reinterpreted, or consumed. Any successor must use a new immutable
cutoff that contains this withdrawal and predates the successor authority
while preserving exact final Pulse 59.

## Evidence

- [Historical authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-64-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-64-authority.v1.schema.json)
- [Historical mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-64-authority-mutations.json)
- [Authority record and closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_64_AUTHORITY.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_64_authority.rs)
- [Closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_64_closeout.rs)

The historical declaration identity is
`sha256:634e7b3197f5d550c6f3816dbf13770d44738c4f05de6956aa07966548a0be23`.
Its unchanged `24700` deterministic controls preserve the historical artifact;
Pulses 65 and 66 later raised the monotonic declared mutation total to
`262306`.
