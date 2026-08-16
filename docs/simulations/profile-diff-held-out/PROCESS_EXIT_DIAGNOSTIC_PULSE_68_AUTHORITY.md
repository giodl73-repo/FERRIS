# Process Exit Diagnostic Pulse 68 Authority

Status: historical authority permanently withdrawn before launch
`invalid-prelaunch-predecessor-cleanup-contract`

Historical authority commit:
`afb0d367cc14a750b0afd25a643432c3e91031c1`

Immutable cutoff:
`48c26aff381eb66459bf099559f0d44971d46f97`

Historical declaration identity:
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`

Historical closed-schema mutation controls: `28830`

Monotonic registry total preserved: `319332`

Pulse 68 was the fresh successor authority after the Pulse 67 withdrawal. It
bound exact final Pulse 59 together with a self-excluding cutoff that already
contained the historical Pulse 67 closeout, the separate sealed harmless
`p68_wsl_probe_worker.py`, the matching sealed harmless
`p68_wsl_probe_sealed_dependencies.py`, and the bound
`ferris.pulse-68-wsl-probe-session/v1` schema artifact. No execution occurred:
every authority-callable count, Pulse 59 callable count, seed count,
descriptor count, process count, publication count, and transfer count remains
zero, while all conclusions remain null.

## Historical authority scope

Pulse 68 explicitly separated dynamic route-equivalence plus exact P56 loader
leg only from static exact production binding.

### Dynamic route-equivalence plus exact P56 loader leg only

- Spawn 1 was the exact Pulse 57 `_stage_wsl_bundle` route through
  `subprocess.run`, with exact `SystemRoot` / `SYSTEMROOT` source-precedence
  derivation, exact `System32\cmd.exe` and `System32\wsl.exe` comparison,
  exact stdout payload shape, and the full twelve-file staged bundle.
- Spawn 1 returned the canonical `bundle_root`, and cleanup before spawn 2 was
  forbidden.
- Spawn 2 was the exact Pulse 57 `_NativeWslSession` bootstrap route through
  `subprocess.Popen`, reusing that same `bundle_root` with the exact Pulse 57
  Python flags and exact `-c` descriptor/bootstrap shape.
- The staged worker was the separate sealed harmless `p68_wsl_probe_worker.py`,
  and the staged dependency was the matching sealed harmless
  `p68_wsl_probe_sealed_dependencies.py`.
- Before the harmless probe protocol started, the staged worker derived
  `repo_root = p56_root.parents[3]`, called the exact byte-bound
  `load_exact_p56(repo_root)` implementation/source route from the staged
  probe dependency, and asserted `Path(p56.__file__).parent == p56_root`.
- That exact `load_exact_p56` route validated the exact staged Pulse 56 public
  release tree, the exact callable identities
  `publish_retained_build_and_custody`, `launch_verified`, and
  `close_custody`, and `DEFAULT_LAUNCH_USES == 69`, but it did **not** invoke
  publish/build/launch.
- The bounded protocol was `ready` -> one harmless probe launch -> `close`.

### Static exact production binding only

- The authority separately bound the exact production Pulse 57 worker bytes at
  `pulse-57-capability-bound-diagnostic-executor-release/wsl_session_worker.py`.
- It separately bound the exact production Pulse 57 sealed dependency bytes at
  `pulse-57-capability-bound-diagnostic-executor-release/sealed_dependencies.py`.
- It separately bound the exact production callable route
  `run_capability_bound_diagnostic_executor` together with
  `_stage_wsl_bundle`, `_NativeWslSession`, `_WSL_BUNDLE_BOOTSTRAP`, and
  `_WSL_WORKER_BOOTSTRAP`.

## Permanent predecessor closure

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

Pulse 68 carried those corrections forward but did not cure predecessor
cleanup.

## Independent prelaunch predecessor cleanup review (2026-08-16)

### Exact Pulse 57 leaked the staged `.p57-*` bundle

The exact cutoff Pulse 57 capability executor stages a native `.p57-*` bundle
under the caller-supplied `ubuntu_runtime_parent`, then launches the exact WSL
worker over `staged.root`. The exact `_NativeWslSession.close()` route writes
`close`, closes stdin, waits/terminate/kill, and drains stdout/stderr, but it
never removes `staged.root` and never verifies post-close absence. The exact
startup-failure paths after staging likewise have no native bundle cleanup
route. The exact predecessor therefore leaves caller-parent residue outside
`private_runtime_root`.

### Exact Pulse 58 and Pulse 59 still overclaimed complete cleanup

Pulse 58 claims that all normal and failure paths close the Windows capability
and Ubuntu worker/capability, remove seed/descriptors/P27/runtime private
roots, and verify their absence. Pulse 59 claims it derives terminal custody
only after exact Pulse 58 completes and removes its private runtime root.
Those claims remain incomplete because the exact Pulse 57 staged bundle lives
under caller-native `ubuntu_runtime_parent`, outside `private_runtime_root`,
and no exact predecessor removes it or verifies absence. Pulse 68 therefore
bound a false predecessor-cleanup contract over the exact final Pulse 59 stack.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, probe tree, seed, descriptor, candidate process, publication
root, result tree, witness tree, or transfer artifact was created.

## Permanent closeout

Pulse 68 is now permanently withdrawn before launch under
`P68-P57-STAGED-BUNDLE-CLEANUP`. Every call, seed, descriptor, process,
publication, transfer, result, witness, and conclusion count remains zero or
null. Pulse 68 is non-retryable, non-resumable, and cannot be amended,
reinterpreted, or consumed. Any successor must use a new immutable cutoff
containing this withdrawal and exact final Pulse 59 while moving staged-bundle
ownership into a cleanup-owning successor stack that retains exact native
bundle identity through close, removes only its owned bundle, and verifies
absence.

## Evidence

- [Historical authority declaration](fixtures/process-exit-diagnostic-pulse-68-authority.json)
- [Historical mutation registry](fixtures/process-exit-diagnostic-pulse-68-authority-mutations.json)
- [Historical closed schema](schemas/ferris.process-exit-diagnostic-pulse-68-authority.v1.schema.json)
- [Pulse 68 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-68.md)
- [Historical authority validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_68_authority.rs)
- [Closeout validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_68_closeout.rs)

The historical declaration identity remains
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`.
Its unchanged `28830` deterministic controls preserve the historical
artifact.
