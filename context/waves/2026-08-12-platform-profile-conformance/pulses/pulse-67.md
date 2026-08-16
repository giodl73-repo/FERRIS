# Pulse 67 - Witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority at immutable cutoff
`3ec6a36009fd34765508f729e795042fd610e5d4`

Declaration identity:
`sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05`

Closed-schema mutation controls: `28196`

Monotonic registry total: `290502`

Pulse 67 is the current authority for the Platform Profile Conformance wave.
It uses a fresh self-excluding cutoff that already contains the Pulse 66
withdrawal, exact final Pulse 59, the separate sealed harmless P67 WSL probe
worker, the matching sealed harmless P67 probe dependency, and the bound
probe-session schema while predating this authority declaration. No authority
callable or diagnostic has run, so calls, seeds, descriptors, processes,
publications, and transfers remain zero while every conclusion remains null.

## What Pulse 67 now proves

Pulse 67 makes the WSL preflight truthful and implementable by separating two
claims that Pulse 66 had conflated:

1. **Dynamic route-equivalence only.** Spawn 1 uses the exact Pulse 57
   `subprocess.run` stage-bundle route, stages the full twelve-file P57-shaped
   bundle, and returns the canonical `bundle_root` without removing it. Spawn 2
   then reuses that same staged `bundle_root` and launches a separate sealed
   harmless `p67_wsl_probe_worker.py` with the exact Pulse 57 Python flags,
   exact `-c` descriptor/bootstrap shape, bounded line protocol, one harmless
   probe launch, one close request, and bounded wait/terminate/kill cleanup.
   The single bundle cleanup occurs only once after both spawns complete, and
   absence is verified after that final cleanup.
2. **Static exact production binding only.** A separate byte-identity binding
   proves the exact production Pulse 57 worker bytes, exact production sealed
   dependency bytes, and the exact production callable route
   `run_capability_bound_diagnostic_executor` together with the internal
   `_stage_wsl_bundle`, `_NativeWslSession`, `_WSL_BUNDLE_BOOTSTRAP`, and
   `_WSL_WORKER_BOOTSTRAP` route symbols.

Pulse 67 therefore proves exact WSL OS-route/bootstrap/protocol/lifecycle
shape without overclaiming exact production worker-byte execution or any real
Pulse 56 / FERRIS work during qualification.

## Permanent predecessor closeout carried forward

Pulse 66 is permanently withdrawn as
`invalid-prelaunch-wsl-probe-bundle-contract` under
`P66-WORKER-HASH-BUNDLE-LIFETIME`. Independent prelaunch review proved that
its fake probe dependency bytes could never satisfy the exact production Pulse
57 worker hash check before `ready`, and that its declared spawn 1 cleanup /
absence proof contradicted the required spawn 2 reuse of the same staged
`bundle_root`. Retry and resume remain prohibited.

## Evidence

- [Pulse 67 authority declaration](../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority.json)
- [Pulse 67 mutation registry](../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-67-authority-mutations.json)
- [Pulse 67 closed schema](../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json)
- [Pulse 67 authority record](../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_67_AUTHORITY.md)
- [Pulse 67 harmless probe worker](../../../docs/simulations/profile-diff-held-out/fixtures/p67_wsl_probe_worker.py)
- [Pulse 67 harmless probe dependency](../../../docs/simulations/profile-diff-held-out/fixtures/p67_wsl_probe_sealed_dependencies.py)
- [Pulse 67 probe protocol schema](../../../docs/simulations/profile-diff-held-out/schemas/ferris.pulse-67-wsl-probe-session.v1.schema.json)
- [Pulse 67 validator test](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_authority.rs)
