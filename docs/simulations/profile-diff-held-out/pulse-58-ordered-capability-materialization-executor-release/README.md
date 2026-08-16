# Pulse 58 ordered capability/materialization executor

Status: sealed infrastructure only; no authority, publication, or FERRIS run.

## Production boundary

`run_ordered_capability_materialization_executor(repo_root,
private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root,
ubuntu_runtime_parent)` is the only production callable. It accepts no
Descriptor root, seed, capability, custody root or receipt, process runner,
callback, environment mapping, or arbitrary control. It does not call a P57
top-level executor, P44/P45, P47, or P43 publication route.

The callable first byte-binds complete P35, P39, P41, P52, P56, and P57 sealed
release trees and APIs. P57 itself binds exact P51/P56 from held source bytes.
The caller-supplied P39 checkout root is an external future-authority
precondition: that authority must prepare a fresh anonymous exact-cutoff root
with its required HEAD, clean-working-tree, and `core.autocrlf` posture. P58
does not establish or claim any of those properties. It invokes only exact P39
path/attribute/LF semantics, then exact P41 transactional copy and final-tree
binding of the verified root's P39 release. The public-only gates then run in
this exact order: sealed predecessor binding, Windows capability build/custody,
native Ubuntu WSL capability/worker build/custody, P27, P31, and P35/P37.
After private materialization, the remaining ordered gates validate descriptors
and run the bounded process search. All catalog IDs are prevalidated before any
seed and do not claim P44/P45 execution or public publication.

After all public gates pass, the executor calls `secrets.token_bytes(32)` once,
writes one `O_EXCL`/`fsync` bounded seed, calls the exact P35 materializer and
verifier once, and removes the seed. The byte-bound P52 reader projects the
exact 70 P35 descriptors into a P58-local frozen descriptor representation.
P58 delegates P57 normalization, semantic projection, worker,
capability-close, and first-stop semantics while retaining lexical no-follow
identity itself. Directory identities use `lstat`, available
`O_DIRECTORY|O_NOFOLLOW` descriptor checks, file IDs, and repeated prelaunch
rechecks without resolving a link; P57 module globals remain unmodified.

The topology is exactly `70/69/1` per platform: 69 launches and one no-launch
ordinal 70 per platform, 138 launches total. Windows launch accounting mutates
immediately after every successful Windows launch, so ordinal-69 Ubuntu
failure or mismatch never re-closes the expired Windows handle. On every
terminal path P58 closes both live capabilities/worker, removes the seed,
descriptors, P27/runtime private roots, and verifies absence. Cleanup uncertainty is fatal
`P58-INDETERMINATE-CLEANUP`; an unknown programmer fault reraises only after
successful cleanup. The returned object has a private execution record and
privacy-safe ordered events; it never publishes a public result.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20 --write-receipt
python -B generate_release.py
```

Twenty harmless fake-only cycles perform 2,760 fake launches, one exact-P39
semantic/P41-copy sequence over a dedicated synthetic P39 root, one
seed/materializer/verifier, two fake capabilities, and one P27 preflight each.
The receipt lists exactly the behavioral unittest control IDs that ran,
including P39/P41 early failures, zero seed, unknown-fault cleanup precedence,
ordinal-69 cleanup, no-follow directory substitution, WSL no-follow, and
final cleanup. No FERRIS binary, authority, candidate, score, result, witness,
or publication is used.
