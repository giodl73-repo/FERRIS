# Pulse 68 - Witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority at immutable cutoff
`48c26aff381eb66459bf099559f0d44971d46f97`

Declaration identity:
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`

Closed-schema mutation controls: `28830`

Monotonic registry total: `319332`

Pulse 68 is the current authority for the Platform Profile Conformance wave.
It uses a fresh self-excluding cutoff that already contains the Pulse 67
withdrawal, exact final Pulse 59, the separate sealed harmless P68 WSL probe
worker, the matching sealed harmless P68 probe dependency, and the bound
probe-session schema while predating this authority declaration. No authority
callable or diagnostic has run, so calls, seeds, descriptors, processes,
publications, and transfers remain zero while every conclusion remains null.

## What Pulse 68 now proves

Pulse 68 makes the current-cutoff and WSL probe contract truthful by binding
two claims separately.

1. **Dynamic route-equivalence plus exact P56 loader leg only.** Spawn 1 uses
   the exact Pulse 57 `subprocess.run` stage-bundle route, stages the full
   twelve-file P57-shaped bundle, and returns the canonical `bundle_root`
   without removing it. Spawn 2 then reuses that same staged `bundle_root` and
   launches a separate sealed harmless `p68_wsl_probe_worker.py` with the
   exact Pulse 57 Python flags, exact `-c` descriptor/bootstrap shape, and the
   bound ready/one-probe/close protocol. Before the harmless probe lifecycle
   begins, that staged worker now derives `repo_root = p56_root.parents[3]`,
   calls the exact byte-bound `load_exact_p56(repo_root)` implementation/source
   route from the staged probe dependency, and asserts
   `Path(p56.__file__).parent == p56_root`; the exact P56 callable identities
   are validated without invoking publish/build/launch. The single bundle
   cleanup still occurs only once after both spawns complete, and absence is
   verified after that final cleanup.
2. **Static exact production binding only.** A separate byte-identity binding
   proves the exact production Pulse 57 worker bytes, exact production sealed
   dependency bytes, and the exact production callable route
   `run_capability_bound_diagnostic_executor` together with the internal
   `_stage_wsl_bundle`, `_NativeWslSession`, `_WSL_BUNDLE_BOOTSTRAP`, and
   `_WSL_WORKER_BOOTSTRAP` route symbols.

Pulse 68 therefore proves exact WSL OS-route/bootstrap/protocol/lifecycle
shape plus the exact production worker's P56 loader leg without overclaiming
exact production worker-byte execution or any real Pulse 56 / FERRIS work
during qualification.

## Permanent predecessor closeout carried forward

Pulse 67 is permanently withdrawn as
`invalid-prelaunch-cutoff-probe-claim-contract` under
`P67-ROOT-CUTOFF-P56-LOADER-CONTRACT`. Independent prelaunch review proved two
non-retryable blockers before launch:

1. The historical declaration's current-cutoff authority/P39/repo
   cutoff/head/revision fields still pointed at the prior Pulse 66 cutoff
   instead of the actual Pulse 67 cutoff.
2. The historical harmless probe claimed the exact `repo_root` /
   `load_exact_p56` / `Path(p56.__file__).parent` worker leg without actually
   deriving `repo_root`, importing the exact staged P56 module, or validating
   its callable identities.

Retry and resume remain prohibited.

## Evidence

- [Pulse 68 authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-68-authority.json)
- [Pulse 68 mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-68-authority-mutations.json)
- [Pulse 68 closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-68-authority.v1.schema.json)
- [Pulse 68 authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_68_AUTHORITY.md)
- [Pulse 68 harmless probe worker](../../../../docs/simulations/profile-diff-held-out/fixtures/p68_wsl_probe_worker.py)
- [Pulse 68 harmless probe dependency](../../../../docs/simulations/profile-diff-held-out/fixtures/p68_wsl_probe_sealed_dependencies.py)
- [Pulse 68 probe protocol schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.pulse-68-wsl-probe-session.v1.schema.json)
- [Pulse 68 validator test](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_68_authority.rs)
