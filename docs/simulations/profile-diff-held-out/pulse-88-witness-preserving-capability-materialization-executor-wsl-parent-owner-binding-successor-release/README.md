# Pulse 88 witness-preserving wsl-parent-owner-binding successor

Status: sealed infrastructure only; no authority, real diagnostic, or product claim.

## Production boundary

`run_witness_preserving_capability_materialization_executor(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent)` is the only production callable.

Pulse 88 preserves exact Pulse 77/P59 terminal semantics while rebasing them over Pulse 87:

- it loads only its sibling `sealed_dependencies.py` by verified path/hash/compile, never ambient import resolution;
- the hardened binder serializes the full transitive exact-load path with the cross-instance kernel lock model inherited from Pulse 77;
- the binder verifies exact Pulse 87 plus exact Pulse 52, Pulse 86, Pulse 51,
  Pulse 43, and Pulse 47 dependencies on every call; Pulse 87 carries the
  exact Pulse 35 release-tree binding and Pulse 86 carries its exact Pulse 78
  dependency forward unchanged; and
- the terminal publication layer still runs only after exact Pulse 87 completes and still publishes either a verified result+witness pair or a verified failure witness.

Pulse 88 derives one fresh sibling terminal root with suffix `.pulse88-terminal-publication`, keeps transfer descriptors path-free, and treats unresolved terminal cleanup as the public-safe `terminal-publication-cleanup-indeterminate` posture.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification is fake-only. It records 40 behavioral control IDs, including
ordered non-publication of `P86-INDETERMINATE-STAGE-CLEANUP`, alternates ten
published results with ten published failure witnesses, exercises the hardened
loader/lock paths while delegating through exact Pulse 87, and performs 2,760
harmless fake launches with zero real FERRIS execution.
