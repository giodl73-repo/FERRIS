# Process Exit Diagnostic Pulse 68 Authority

Status: current `authorized-unexecuted` authority at immutable cutoff
`48c26aff381eb66459bf099559f0d44971d46f97`

Declaration identity:
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`

Closed-schema mutation controls: `28830`

Monotonic registry total: `319332`

Pulse 68 is the fresh successor authority after the Pulse 67 withdrawal. It
binds exact final Pulse 59 together with a new self-excluding cutoff that
already contains the historical Pulse 67 closeout, the separate sealed
harmless `p68_wsl_probe_worker.py`, the matching sealed harmless
`p68_wsl_probe_sealed_dependencies.py`, and the bound
`ferris.pulse-68-wsl-probe-session/v1` schema artifact. No execution has
occurred: every authority-callable count, Pulse 59 callable count, seed count,
descriptor count, process count, publication count, and transfer count remains
zero, while all conclusions remain null.

## Truthful preflight scope

Pulse 68 explicitly separates dynamic route-equivalence plus exact P56 loader leg only from static exact production binding.

### Dynamic route-equivalence plus exact P56 loader leg only

- Spawn 1 is the exact Pulse 57 `_stage_wsl_bundle` route through
  `subprocess.run`, with exact `SystemRoot` / `SYSTEMROOT` source-precedence
  derivation, exact `System32\cmd.exe` and `System32\wsl.exe` comparison,
  exact stdout payload shape, and the full twelve-file staged bundle.
- Spawn 1 returns the canonical `bundle_root`, and cleanup before spawn 2 is
  forbidden.
- Spawn 2 is the exact Pulse 57 `_NativeWslSession` bootstrap route through
  `subprocess.Popen`, reusing that same `bundle_root` with the exact Pulse 57
  Python flags and exact `-c` descriptor/bootstrap shape.
- The staged worker is the separate sealed harmless
  `p68_wsl_probe_worker.py`, and the staged dependency is the matching sealed
  harmless `p68_wsl_probe_sealed_dependencies.py`.
- Before the harmless probe protocol starts, the staged worker now derives
  `repo_root = p56_root.parents[3]`, calls the exact byte-bound
  `load_exact_p56(repo_root)` implementation/source route from the staged
  probe dependency, and asserts `Path(p56.__file__).parent == p56_root`.
- That exact `load_exact_p56` route validates the exact staged Pulse 56 public
  release tree, the exact callable identities
  `publish_retained_build_and_custody`, `launch_verified`, and
  `close_custody`, and `DEFAULT_LAUNCH_USES == 69`, but it does **not** invoke
  publish/build/launch.
- The bounded protocol is `ready` -> one harmless probe launch -> `close`,
  followed by wait / terminate / kill handling and one final cleanup of the
  staged bundle after both spawns complete.
- The dynamic preflight proves only the exact WSL route/bootstrap/protocol/
  lifecycle shape plus the exact P56 loader leg. It does **not** claim
  execution of the exact production Pulse 57 worker bytes and does **not**
  claim any real Pulse 56 capability or FERRIS work.

### Static exact production binding only

- The authority separately binds the exact production Pulse 57 worker bytes at
  `pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py`.
- It separately binds the exact production Pulse 57 sealed dependency bytes at
  `pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py`.
- It separately binds the exact production callable route
  `run_capability_bound_diagnostic_executor` together with
  `_stage_wsl_bundle`, `_NativeWslSession`, `_WSL_BUNDLE_BOOTSTRAP`, and
  `_WSL_WORKER_BOOTSTRAP`.

## Pulse 67 permanent withdrawal carried forward

Pulse 67 remains permanently withdrawn as
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

Pulse 68 replaces those overclaims with a truthful exact-loader probe contract
while keeping exact production byte identity in static form only.

## Evidence

- [Pulse 68 authority declaration](fixtures/process-exit-diagnostic-pulse-68-authority.json)
- [Pulse 68 mutation registry](fixtures/process-exit-diagnostic-pulse-68-authority-mutations.json)
- [Pulse 68 closed schema](schemas/ferris.process-exit-diagnostic-pulse-68-authority.v1.schema.json)
- [Pulse 68 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-68.md)
- [Pulse 68 harmless probe worker](fixtures/p68_wsl_probe_worker.py)
- [Pulse 68 harmless probe dependency](fixtures/p68_wsl_probe_sealed_dependencies.py)
- [Pulse 68 probe session schema](schemas/ferris.pulse-68-wsl-probe-session.v1.schema.json)
- [Pulse 68 validator test](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_68_authority.rs)
