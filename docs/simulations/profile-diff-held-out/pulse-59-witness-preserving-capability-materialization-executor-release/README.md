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
verified file path rather than ambient import resolution. On every production
or qualification call it freshly reads the sibling binder bytes, SHA-256
verifies them against the sealed executor constant, compiles them into a new
private module object, and immediately calls that fresh binder. It does not
reuse a cached binder, registry module, or predecessor module object. The
fresh binder byte-binds the complete exact Pulse 58 release at commit
`7c66d70800edd06642274ed4f2e4aee224b7583e`, verifies its manifest, receipt,
seal, source, gate catalog, and production/qualification callable signatures,
then instantiates fresh exact Pulse 52 stage helpers and exact Pulse 57/Pulse
51 terminal dependencies through Pulse 58's own sealed stack on every call.
Because the exact predecessor stack still uses bare `sealed_dependencies`
imports internally, the binder serializes the full exact-load path with a
cross-instance kernel lock keyed by a SHA-256 over the resolved sibling binder
path plus its exact source digest. On Windows it uses a named mutex through
`CreateMutexW`/`WaitForSingleObject`; on supported Linux/Ubuntu POSIX targets
it uses a deterministic abstract AF_UNIX socket address
`\0ferris-p59-<sha256>` whose bound socket is held for the full critical
section. Bind is atomic, `EADDRINUSE` retries are bounded and fail closed, a
process crash closes the kernel socket automatically, and no filesystem lock
path, lock file, unlink step, or mutable Python registry is used. The
reentrancy state tracks the owning kernel object, PID, native thread identity,
and a mutable live token shared across copied `ContextVar` states so a replayed
or cross-thread copied context cannot bypass the kernel lock after unwind. The
stable Pulse 59 executor module owns one Linux `os.register_at_fork(after_in_child=...)`
cleanup hook per loaded executor instance, and each fresh exact binder
registers only its currently held abstract socket state with that stable
manager through an internal byte-bound API. In a forked child, the stable
manager closes inherited abstract socket descriptors and invalidates the copied
live token before user code resumes; later child reentry reacquires explicitly
rather than skipping or releasing the parent's acquisition. Non-Linux POSIX
platforms fail closed rather than weakening the lock. The lock is released on
every path and held across the entire exact Pulse 58 transitive
verify/import/binding sequence through exact Pulse 52, Pulse 57, Pulse 51, and
terminal Pulse 43/Pulse 47 dependency loading until all returned modules are
fully loaded and detached from temporary generic bindings. It restores any
preexisting generic module slot only if that slot still holds the exact module
it installed. Pulse 59 does not rebuild P39/P41 ordering, P35 materialization,
or P57 launch semantics; it delegates exact Pulse 58 production or
qualification orchestration.

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

Realistic integrity boundary: Pulse 59 rejects ambient import resolution,
preseeded generic slots, reused private keys, stale registry artifacts,
replaceable pathname locks, copied cross-thread owner state, stale inherited
fork state, replayed invalidated lock contexts, and mutated cached binder
state before a call begins. Arbitrary mutation of live private Python objects
during an active call remains outside this release's process-integrity
boundary.
