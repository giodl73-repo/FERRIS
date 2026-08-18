# Pulse 87 ordered capability/materialization WSL parent-owner binding successor

Status: complete sealed infrastructure release; no authority, publication, or
real FERRIS execution.

## Production boundary

`run_ordered_capability_materialization_executor(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent)` is the only production callable.

Pulse 87 preserves exact Pulse 70 / Pulse 58 ordered behavior while rebasing
the live capability edge onto Pulse 86 and hardening the complete sealed load
graph:

- the executor loads only its sibling `sealed_dependencies.py` by exact
  path/hash/compile, never ambient import resolution;
- the sealed binder verifies exact Pulse 39, Pulse 41, Pulse 52, Pulse 35, and
  Pulse 86 release trees on every call, and Pulse 35 now requires the pinned
  manifest, receipt, seal, exact file set, and single source digests with no
  alternate-hash whitelist;
- every local and transitive sealed load across that exact graph is serialized
  with the final Pulse 74 / Pulse 59 cross-thread and cross-process kernel-lock
  discipline; and
- ordered execution still performs one exact Pulse 39 / Pulse 41 public custody
  sequence, one private 32-byte seed, one bounded Pulse 35 materialization, one
  verification, descriptor freezing, and one exact Pulse 86 capability
  sequence, with exact Pulse 70 / Pulse 58 cleanup, topology, and privacy
  behavior; and
- Pulse 86's explicit parent-owner WSL binding remains exact, while
  `P86-INDETERMINATE-STAGE-CLEANUP` survives the ordered terminal boundary
  before seed creation.

The topology remains exactly `70/69/1` per platform, 138 launches total, with
no P44/P45 execution, no publication, and no witness or authority creation.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification is fake-only. It records 29 deterministic control IDs, including
100-thread complete-load-graph
serialization, multi-process kernel-lock stress, and adversarial Pulse 35
tamper checks for a historical alternate source digest, receipt tamper, seal
tamper, extra-tree-file injection, and ordered preservation of Pulse 86's
indeterminate staging disposition, then runs 20 harmless cycles for 2,760
total fake launches over a synthetic Pulse 39 checkout. No authority, witness,
publication, or real capability binary is used.
