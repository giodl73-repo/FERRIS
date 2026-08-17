# Pulse 82 witness-preserving pulse35-release-tree successor root cause report

## Defect fixed

Pulse 77 provided the hardened local sibling loader and terminal witness
custody, but it still sealed the pre-P81 ordered layer. After Pulse 81 closed
exact Pulse 35 release-tree underbinding, the witness surface still needed a
matching successor so terminal publication could delegate only through the new
exact P81/P35/P78 chain.

## Successor approach

Pulse 82 keeps frozen Pulse 77 intact and preserves its exact terminal
semantics while rebasing the predecessor chain:

- the executor still uses the hardened local sibling binder with cross-instance kernel-lock serialization and fresh module loading on every call;
- the binder now verifies exact Pulse 81 instead of the earlier ordered layer;
- Pulse 81 in turn binds exact Pulse 35 manifest/receipt/seal/tree identities
  plus exact Pulse 78, so the full witness chain now terminates in the
  pulse35-release-tree-hardened ordered/capability successor stack; and
- result publication, failure-witness publication, path-free transfer descriptors, and non-returning terminal cleanup posture remain exact Pulse 77/P59 behavior.

## Qualification boundaries

Qualification is fake-only. It re-runs the full Pulse 77 witness suite over the
exact Pulse 81 chain, including binder freshness, kernel-lock discipline,
precompletion publication blocking, alternate failure-witness postures, and the
delegated exact Pulse 35 release-tree requirement enforced by Pulse 81. It
executes no authority and no real FERRIS diagnostic.
