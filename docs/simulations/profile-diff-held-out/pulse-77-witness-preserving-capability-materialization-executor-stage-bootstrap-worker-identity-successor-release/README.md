# Pulse 77 witness-preserving stage-bootstrap-worker-identity successor

Status: sealed infrastructure only; no authority, real diagnostic, or product claim.

## Production boundary

`run_witness_preserving_capability_materialization_executor(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent)` is the only production callable.

Pulse 77 preserves exact Pulse 71/P59 terminal semantics while rebasing them over Pulse 76:

- it loads only its sibling `sealed_dependencies.py` by verified path/hash/compile, never ambient import resolution;
- the hardened binder serializes the full transitive exact-load path with the cross-instance kernel lock model inherited from Pulse 71;
- the binder verifies exact Pulse 76 plus exact Pulse 52, Pulse 75, Pulse 51, Pulse 43, and Pulse 47 dependencies on every call; and
- the terminal publication layer still runs only after exact Pulse 76 completes and still publishes either a verified result+witness pair or a verified failure witness.

Pulse 77 derives one fresh sibling terminal root with suffix `.pulse77-terminal-publication`, keeps transfer descriptors path-free, and treats unresolved terminal cleanup as the public-safe `terminal-publication-cleanup-indeterminate` posture.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification is fake-only. It records 39 behavioral control IDs, alternates ten published results with ten published failure witnesses, exercises the hardened loader/lock paths, and performs 2,760 harmless fake launches with zero real FERRIS execution.
