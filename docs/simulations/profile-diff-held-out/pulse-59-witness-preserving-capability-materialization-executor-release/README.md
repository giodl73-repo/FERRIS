# Pulse 59 witness-preserving capability/materialization executor

Status: sealed infrastructure only; no authority, real diagnostic, or product
claim

## Production boundary

`run_witness_preserving_capability_materialization_executor(repo_root,
private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root,
ubuntu_runtime_parent)` is the only production callable. It mirrors Pulse 58's
six concrete inputs and accepts no seed, fake capability, callback,
publication-root, retained custody, trust flag, or other injection surface.

Before any call, Pulse 59 loads its sibling `sealed_dependencies.py` binder by
verified file path rather than ambient import resolution. That binder
byte-binds the complete exact Pulse 58 release at commit
`7c66d70800edd06642274ed4f2e4aee224b7583e`, verifies its manifest, receipt,
seal, source, gate catalog, and production/qualification callable signatures,
then instantiates fresh exact Pulse 52 stage helpers and exact Pulse 57/Pulse
51 terminal dependencies through Pulse 58's own sealed stack on every call.
Pulse 59 does not rebuild P39/P41 ordering, P35 materialization, or P57 launch
semantics; it delegates exact Pulse 58 production or qualification
orchestration.

Pulse 58 removes its private runtime root on every terminal path, so Pulse 59
derives one fresh sibling terminal custody root from `private_runtime_root`
using the fixed suffix `.pulse59-terminal-publication`, rejects any preexisting
path, and creates it only after exact Pulse 58 returns `completed`. The public
event list is therefore exact Pulse 58 output with no post-completion
execution event.

## Terminal classes

After Pulse 58 returns `completed`, Pulse 59 invokes the exact one-use Pulse 51
terminal Pulse 47 seam once and only once over the derived sibling terminal
root. It preserves the exact three completed Pulse 53 classes:

- `published-result`: verified two-file Pulse 43 result root plus verified
  two-file Pulse 47 witness root.
- `published-failure-witness`: verified two-file Pulse 47 witness only when the
  captured Pulse 43 publication is the exact bounded `absent`, `rolled-back`,
  or `indeterminate` failure posture and no Pulse 43 root or stage residue
  remains.
- `invalid-witness-publication`: failed, malformed, mismatched, unverifiable,
  or residue-bearing terminal output. Pulse 59 performs exact bounded verified
  cleanup only; unresolved cleanup raises the non-returning public-safe
  `terminal-publication-cleanup-indeterminate` posture.

Any Pulse 58 preexecution or precompletion failure remains
`publication=not-attempted`. Public transfer descriptors remain path-free and
carry only expected public tree kind, exact file counts, and verified raw and
payload hashes.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Qualification calls exact Pulse 58 fake orchestration, alternates ten
`published-result` and ten `published-failure-witness` closeouts, covers all
three bounded Pulse 43 failure postures, and performs 2,760 harmless fake
launches with zero real FERRIS execution. The release generator rejects Python
cache residue and reseals the complete public tree deterministically.
