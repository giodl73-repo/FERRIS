# Pulse 73 ordered capability/materialization stage-identity successor

Status: sealed infrastructure only; no authority, publication, or FERRIS run.

## Production boundary

`run_ordered_capability_materialization_executor(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent)` is the only production callable.

Pulse 73 preserves exact Pulse 70/P58 ordered behavior, but it rebases that ordering over Pulse 72 explicitly:

- the executor loads its sibling `sealed_dependencies.py` by exact path/hash/compile rather than ambient import resolution;
- the sealed binder verifies exact P35, P39, P41, P52, and Pulse 72 release trees;
- ordered execution still performs one exact P39/P41 sequence, one bounded seed, one P35 materialization, one verification, and one exact Pulse 72 capability sequence; and
- all terminal cleanup, topology, and privacy behavior remains exact Pulse 70/P58 behavior.

The topology remains exactly `70/69/1` per platform, 138 launches total, with no P44/P45 execution, no publication, and no real FERRIS execution.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification is fake-only. It records 22 unittest control IDs, including the explicit local loader / exact Pulse 72 binding controls, then runs 20 harmless cycles for 2,760 total fake launches over a synthetic P39 checkout. No authority, witness, publication, or real capability binary is used.
