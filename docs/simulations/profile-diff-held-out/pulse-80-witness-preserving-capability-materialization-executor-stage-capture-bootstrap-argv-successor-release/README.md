# Pulse 80 witness-preserving stage-capture-bootstrap-argv successor

Status: sealed infrastructure only; no authority, real diagnostic, or product claim.

## Production boundary

`run_witness_preserving_capability_materialization_executor(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent)` is the only production callable.

Pulse 80 preserves exact Pulse 77/P59 terminal semantics while rebasing them over Pulse 79:

- it loads only its sibling `sealed_dependencies.py` by verified path/hash/compile, never ambient import resolution;
- the hardened binder serializes the full transitive exact-load path with the cross-instance kernel lock model inherited from Pulse 77;
- the binder verifies exact Pulse 79 plus exact Pulse 52, Pulse 78, Pulse 51, Pulse 43, and Pulse 47 dependencies on every call; and
- the terminal publication layer still runs only after exact Pulse 79 completes and still publishes either a verified result+witness pair or a verified failure witness.

Pulse 80 derives one fresh sibling terminal root with suffix `.pulse80-terminal-publication`, keeps transfer descriptors path-free, and treats unresolved terminal cleanup as the public-safe `terminal-publication-cleanup-indeterminate` posture.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification is fake-only. It records 39 behavioral control IDs, alternates ten published results with ten published failure witnesses, exercises the hardened loader/lock paths, and performs 2,760 harmless fake launches with zero real FERRIS execution.
