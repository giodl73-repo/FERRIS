# Pulse 68 - Witnessed capability/materialization diagnostic authority

Status: Permanently withdrawn invalid-prelaunch-predecessor-cleanup-contract

Historical authority commit:
`afb0d367cc14a750b0afd25a643432c3e91031c1`

Immutable cutoff:
`48c26aff381eb66459bf099559f0d44971d46f97`

Historical declaration identity:
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`

Historical closed-schema mutation controls: `28830`

Monotonic registry total preserved: `319332`

## Goal

Record the historical Pulse 68 authority and its permanent prelaunch
withdrawal without executing any Pulse 59, Pulse 58, Pulse 57, Pulse 56, or
FERRIS diagnostic callable.

## Historical authority

Pulse 68 bound exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 public release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities derive only from immutable Git blobs at cutoff
`48c26aff381eb66459bf099559f0d44971d46f97`, which contains the Pulse 67
withdrawal, exact final Pulse 59, and the historical
`p68_wsl_probe_worker.py`, `p68_wsl_probe_sealed_dependencies.py`, and
`ferris.pulse-68-wsl-probe-session/v1` schema artifact while predating the
authority. Working-tree bytes are not an identity source except for explicitly
declared complete-file LF/CRLF variants.

The historical declaration truthfully separated dynamic WSL route-equivalence
plus the exact P56 loader leg from static exact production binding, but it
still bound exact final Pulse 59 cleanup semantics over a predecessor stack
that leaked native staged-bundle residue outside `private_runtime_root`.

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
`P65-P57-WSL-TWO-SPAWN-CONTRACT`; Pulse 66 remains permanently withdrawn
`invalid-prelaunch-wsl-probe-bundle-contract` under
`P66-WORKER-HASH-BUNDLE-LIFETIME`; and Pulse 67 remains permanently withdrawn
`invalid-prelaunch-cutoff-probe-claim-contract` under
`P67-ROOT-CUTOFF-P56-LOADER-CONTRACT`.

## Independent prelaunch predecessor cleanup review (2026-08-16)

### Exact Pulse 57 closed the worker but leaked the staged native bundle

The exact cutoff Pulse 57 `_NativeWslSession.__init__` stages one `.p57-*`
bundle under caller-supplied native `ubuntu_runtime_parent` and passes
`staged.root` into the worker bootstrap. The exact
`_NativeWslSession.close()` route then writes the `close` request, closes
stdin, waits/terminate/kill, and drains stdout/stderr, but it never retains
bundle ownership after construction, never removes `staged.root`, and never
proves post-close absence. A startup failure after staging likewise had no
native bundle removal route. Pulse 68 therefore still bound a predecessor
stack with caller-parent residue outside `private_runtime_root`.

### Exact Pulse 58 and Pulse 59 overclaimed stack cleanup and zero residue

Pulse 58 claimed that every terminal path closes the Windows capability and
Ubuntu worker/capability, removes seed/descriptors/P27/runtime private roots,
and verifies absence. Pulse 59 claimed it invokes terminal publication only
after exact Pulse 58 completes and removes its private runtime root. Those
claims do not cover the staged `.p57-*` native bundle because that bundle
lives under caller-supplied `ubuntu_runtime_parent`, outside
`private_runtime_root`, and exact Pulse 57 never deleted it. Pulse 68
therefore overclaimed exact predecessor cleanup over the final Pulse 59
callable stack.

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
containing this withdrawal, preserve the historical Pulse 68 authority artifact
unchanged, and bind a cleanup-owning successor stack that retains exact native
staged-bundle identity through worker close and removes only its owned bundle
with verified absence.

## Evidence

- [Historical authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-68-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-68-authority.v1.schema.json)
- [Historical mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-68-authority-mutations.json)
- [Authority record and closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_68_AUTHORITY.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_68_authority.rs)
- [Closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_68_closeout.rs)

The historical declaration identity remains
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`.
Its unchanged `28830` deterministic controls preserve the historical
artifact.
