# Pulse 53: witness-preserving ordered executor release

Status: Complete synthetic infrastructure only.

Pulse 53 is a sealed replacement terminal branch for exact Pulse 52.  It fixes
one narrow closeout error: a successfully published and independently verified
Pulse 47 witness of a bounded Pulse 43 publication failure is a permanent
public artifact, not failed terminal residue.  It grants no authority and does
not execute a real FERRIS binary, diagnostic, candidate, score, certification,
product change, or PLATFORM-001 conclusion.

## Exact binding and production surface

`witness_preserving_ordered_executor.run_witness_preserving_ordered_executor`
is the sole production callable.  It accepts only the same six concrete inputs
as Pulse 52: repository root, fresh declared runtime root, fresh P27 cycle
root, fresh P39 checkout root, fresh P41 final-copy root, and retained P44
custodies.  It accepts no seed, descriptor, materializer, verifier, launcher,
terminal callback, output root, expectation, fake binary, trust flag, or
authority input.

Before any phase runs, `sealed_dependencies.py` verifies the complete exact
Pulse 52 release tree, manifest, receipt, seal, source and bounded phase
callables at commit `e4ef9617f227670f3911be42ca63df4b2e66d24f`; it then invokes
Pulse 52's own exact Pulse 51 loader for commit
`d09c923c1e2cd2be003026597f4ad2a0e2d3764f`.  The copied code is limited to
Pulse 52's orchestration and terminal branch.  P39/P41 custody, public gates,
CSPRNG seed timing, P35 materialization/verification, private cleanup,
Windows/WSL dispatch, search topology, and error boundaries reuse exact Pulse
52 internals without production monkeypatching.

## Terminal classes

After exactly one `TerminalPulse47Once` call and no post-publication event,
Pulse 53 returns one of these public-safe classes:

- `published-result` retains a verified two-file P43 result root and verified
two-file P47 witness root.  Its transfer descriptor has kind
  `result-and-witness`, exact `2 + 2 = 4` file counts, and only verified raw
  and payload hashes.
- `published-failure-witness` retains only a verified two-file P47 witness
  root when the captured P43 outcome is an exact bounded failed posture
  (`absent`, `rolled-back`, or `indeterminate`).  Pulse 53 verifies the P43
  root and its stage are absent, verifies the witness root has exactly two
  files, and retains no P43 result file.  Product, category, and fix
  conclusions are null.  Its descriptor kind is `failure-witness-only` with
  exactly two files and witness raw/payload hashes.
- `invalid-witness-publication` covers failed, malformed, mismatched, or
  unverifiable witness publication and any missing required final shape.  It
  makes no retry or republication and uses Pulse 52's bounded verified terminal
  cleanup.  Unresolved cleanup raises the non-returning public-safe
  `terminal-publication-cleanup-indeterminate` posture.

A transfer descriptor has no terminal paths, root names, case IDs, or private
record.  A custodian already authorized for the known roots performs transfer.

## Qualification

Run from this directory:

```powershell
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20
```

The fake-only suite covers P43 published result retention; every bounded P43
failure posture with a successful P47 witness; P47 failure, malformed summary,
hash mismatch, and P43-residue cleanup; no retry; cleanup fatal behavior;
programmer fault propagation; public descriptor privacy; exact Pulse 51/Pulse
52 binding; and the injection-free production signature.  Twenty alternating
cycles retain ten `published-result` and ten `published-failure-witness`
closeouts, exercise absent/rolled-back/indeterminate P43 postures, perform
2,760 fake dispatches, and execute no FERRIS binary.

Removal deletes only this Pulse 53 release, review, validator, schema, and
wave record.  It cannot modify Pulse 51/Pulse 52, revive withdrawn Pulse 50
authority, or alter custody-owned public roots.
