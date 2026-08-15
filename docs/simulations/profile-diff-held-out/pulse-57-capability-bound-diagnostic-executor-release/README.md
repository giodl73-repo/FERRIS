# Pulse 57 capability-bound diagnostic executor

Status: sealed successor infrastructure; no authority and no qualification
execution of a FERRIS binary.

## Production boundary

`run_capability_bound_diagnostic_executor(repo_root, descriptor_root,
private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)` is the sole
production callable. It accepts neither a receipt, custody root, binary,
executable path, process runner, callback, environment, capability, nor
arbitrary control. It first verifies every file, manifest, receipt, seal, and
required API of the exact sealed Pulse 51 and Pulse 56 releases.

It retains Pulse 51's P27/P31/P35/P37 behavior, exact 70/69/1 per-platform
topology, 140/138/2 total accounting, descriptor/path confinement, bounded
result parsing, full profile-diff semantic projection, and selection,
invocation, result, and diff identity recomputation. It freezes every
descriptor's expected semantics and each input's existence, file identity,
size, and digest before any launch; immediately before each Windows and Ubuntu
launch it rechecks the frozen identities and performs the exact P51 dispatch
mapping. Ubuntu arguments come only from that frozen validated mapping. The
first substitution or cross-platform semantic mismatch stops the run.

The Pulse 44/Pulse 45 caller-summary bridge is deliberately absent. The
privacy-safe Pulse 43 catalog contains only `sealed-predecessor-binding`,
Windows/Ubuntu capability-build/custody, exact adapter preflight, P31,
P35/P37 normalization, descriptor validation, and bounded process exit search.
It does **not** contain or imply `pulse-41-pulse-39-public-custody`: P57 does
not execute P39 or P41. A later ordered layer must execute and record P39/P41
before any private materialization; it MUST NOT infer either from P57.

## Capability route

Before descriptor validation or any possible later materialization wrapper,
the executor calls the exact Pulse 56
`publish_retained_build_and_custody` once for Windows and once inside the
native Ubuntu session. Public receipts and roots remain evidence only; none is
ever converted to a capability.

Windows calls only Pulse 56 `launch_verified(handle, "windows-x86_64",
concrete_args)`. Before Ubuntu starts, the parent copies the worker, sealed
dependency loader, and complete P56 tree from verified byte buffers into a
fresh native Linux directory. Isolated Python starts a fixed bootstrap which
opens, hashes, and compiles the worker from its held native descriptor; no
source loader or worker-path reopen occurs. The worker compiles every staged
dependency from its verified byte buffer with bytecode disabled only after
complete P56-tree verification. It receives no ambient `PYTHONPATH`, site
customization, `WSLENV`, or mounted-path import route. It then publishes one
live 69-use capability, consumes exactly 69 canonical ordered requests, and
returns only bounded canonical result envelopes.

Every I/O, protocol, startup, timeout, and launch failure owns the worker
process: close, bounded wait, terminate, and kill are attempted in that order.
An orphan or live capability is fatal cleanup failure. Cleanup completes before
the sole terminal event is emitted. A close failure, including after all 138
launches, is `P57-INDETERMINATE-CLEANUP` with exactly one failed terminal
event—never completed-plus-failed and never rewritten as a P43 catalog error.
Explicit exact predecessor failure classes retain their safe codes; unknown
programmer faults re-raise only after successful cleanup, otherwise fatal
cleanup takes precedence.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification first executes the 22 named negative-control tests, then binds
the real sealed predecessor sources but replaces only the
unexported, source-bound Pulse 56 capability/session implementation with a
harmless fake. It alternates two valid fake artifact variants across 20 cycles
and 2,760 fake launches, creating fresh fake capabilities each cycle. Receipt
counts record all 22 executed controls: final cleanup after 138 launches,
source/helper substitution and bytecode-residue rejection, lazy/path and
prelaunch-semantics substitution, profile/result mismatch, capability
exhaustion, P51/P31/P35/P37/terminal bounded-failure classification, worker
injection/replay/order/extra output, startup I/O orphan termination, close
timeout, and successful-versus-fatal cleanup precedence for parent and worker
unknown faults. It creates no seed,
materialization, authority, result root, or FERRIS execution.
