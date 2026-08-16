# Process Exit Diagnostic Pulse 67 Authority

Status: current `authorized-unexecuted` authority at immutable cutoff
`3ec6a36009fd34765508f729e795042fd610e5d4`

Declaration identity:
`sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05`

Closed-schema mutation controls: `28196`

Monotonic registry total: `290502`

Pulse 67 is the fresh successor authority after the Pulse 66 withdrawal. It
binds exact final Pulse 59 together with a new self-excluding cutoff that
already contains the historical Pulse 66 closeout, the separate sealed
harmless `p67_wsl_probe_worker.py`, the matching sealed harmless
`p67_wsl_probe_sealed_dependencies.py`, and the bound
`ferris.pulse-67-wsl-probe-session/v1` schema artifact. No execution has
occurred: every authority-callable count, Pulse 59 callable count, seed count,
descriptor count, process count, publication count, and transfer count remains
zero, while all conclusions remain null.

## Truthful preflight scope

Pulse 67 explicitly separates **dynamic route-equivalence only** from **static
exact production binding**.

### Dynamic route-equivalence only

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
  `p67_wsl_probe_worker.py`, and the staged dependency is the matching sealed
  harmless `p67_wsl_probe_sealed_dependencies.py`.
- The bounded protocol is `ready` -> one harmless probe launch -> `close`,
  followed by wait / terminate / kill handling and one final cleanup of the
  staged bundle after both spawns complete.
- The dynamic preflight proves only the exact WSL route/bootstrap/protocol/
  lifecycle shape. It does **not** claim execution of the exact production
  Pulse 57 worker bytes and does **not** claim any real Pulse 56 capability or
  FERRIS work.

### Static exact production binding only

- The authority separately binds the exact production Pulse 57 worker bytes at
  `pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py`.
- It separately binds the exact production Pulse 57 sealed dependency bytes at
  `pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py`.
- It separately binds the exact production callable route
  `run_capability_bound_diagnostic_executor` together with
  `_stage_wsl_bundle`, `_NativeWslSession`, `_WSL_BUNDLE_BOOTSTRAP`, and
  `_WSL_WORKER_BOOTSTRAP`.

## Pulse 66 permanent withdrawal carried forward

Pulse 66 remains permanently withdrawn as
`invalid-prelaunch-wsl-probe-bundle-contract` under
`P66-WORKER-HASH-BUNDLE-LIFETIME`. Independent prelaunch review proved two
non-retryable blockers before launch:

1. The exact production worker validates `worker/sealed_dependencies.py`
   against the production sealed-dependency hash before it emits `ready`, so
   Pulse 66's fake dependency could never honestly witness exact worker-byte
   bootstrap.
2. Pulse 66 required spawn 1 cleanup plus verified absence before spawn 2,
   even though spawn 2 also required the same staged `bundle_root` to still
   exist. The bundle-lifetime contract was self-contradictory.

Pulse 67 replaces those overclaims with a truthful probe-worker contract while
keeping exact production byte identity in static form only.

## Evidence

- [Pulse 67 authority declaration](fixtures/process-exit-diagnostic-pulse-67-authority.json)
- [Pulse 67 mutation registry](fixtures/process-exit-diagnostic-pulse-67-authority-mutations.json)
- [Pulse 67 closed schema](schemas/ferris.process-exit-diagnostic-pulse-67-authority.v1.schema.json)
- [Pulse 67 wave record](../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-67.md)
- [Pulse 67 harmless probe worker](fixtures/p67_wsl_probe_worker.py)
- [Pulse 67 harmless probe dependency](fixtures/p67_wsl_probe_sealed_dependencies.py)
- [Pulse 67 probe session schema](schemas/ferris.pulse-67-wsl-probe-session.v1.schema.json)
- [Pulse 67 validator test](../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_67_authority.rs)
