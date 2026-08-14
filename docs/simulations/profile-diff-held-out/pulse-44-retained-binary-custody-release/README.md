# Pulse 44 retained-binary custody release

This public, standard-library-only adapter makes one retained Pulse 33 binary
and its exact build receipt an atomic two-file custody tree. It is public
release infrastructure only. It creates no diagnostic authority, does not
execute a diagnostic or candidate, accesses no private data, and makes no
product, category, fix, support, certification, or PLATFORM-001 conclusion.

## Fixed inputs and boundary

One invocation accepts exactly one of `windows-x86_64` or
`ubuntu-24.04-x86_64`, immutable Pulse 33 cutoff
`29517d732db13cc2ffa304684b344f3538ab587d`, a fresh absent absolute work
root, and a fresh absent absolute final custody root. The roots may not
overlap, have symlinked/non-directory parents, or reuse a stale stage.

Before it imports anything, the adapter recomputes and pins the exact Pulse 33
public manifest raw identity
`sha256:9082bb18ab72e5e5ced2ec43811ecc5ce14ac43c9cd2878e4db0f10bf7a741fd`,
aggregate
`sha256:07df7bc02cab288adccbdc0f87e45f3fd52939ebc9d22c8d064f773843e861a4`,
37-file/59895-byte control total, and immutable `build_freeze.py` identity
`sha256:43bb31210175ceacba2431a238608d9973672a08de57572543ad0f9dae41cbe6`.
It then calls that public `build_and_freeze` function exactly once with
`retain_executable=True`.

The work tree must contain exactly the expected logical executable and
`<logical-name>.receipt.json`. The adapter rejects symlinks, non-regular
files, additions, omissions, duplicate JSON members, a false retention field,
and any filename, platform, cutoff, size, SHA-256, clean-checkout,
locked-release-build, or no-diagnostic-execution disagreement.

## Transaction

The adapter copies both files into an absent sibling stage, fsyncs each
destination before close, verifies the staged `2/2` raw size/hash and receipt
recomputation, removes the work tree, and honestly synchronizes the stage.
It makes one `os.replace` and no retry or fallback. It reconstructs the final
root from the original absolute request, independently recomputes final
`2/2` size/hash/receipt bindings, then synchronizes the final parent.

Only after final `2/2` verification does a success summary contain the
Pulse-43-compatible ordered-execution terminal event. Every failure instead
contains a terminal `failed` event and an `absent`, `rolled-back`, or
`indeterminate` custody posture. A rollback is `rolled-back` only after
removal and an honest synced or explicitly unsupported parent-sync posture.
The public summaries contain no executable bytes, local path, or private data.
`unsupported` directory sync is a portability result under
`os.open+os.fsync-directory-v1`, never a durability assertion.

Run from any working directory:

```console
python retained_binary_custody.py --repo C:\public\cutoff-checkout --cutoff 29517d732db13cc2ffa304684b344f3538ab587d --platform windows-x86_64 --work-root C:\public\work --final-root C:\public\custody
```

The final custody directory is controlled non-public runtime state and MUST
NOT be committed.

## Qualification

The Python qualification has 29 deterministic methods using bounded synthetic
executable and receipt bytes. It covers retention/receipt and
filename/platform/cutoff/hash/size rejection; absent/stale/overlap roots;
symlink and non-regular rejection; copy, file-sync, stage verification, stage
sync, rename, final verification, final sync, rollback removal, rollback sync,
and event-emission failures; one rename/zero retries; and path-free terminal
summaries. The Rust integration validator recomputes this release's sealed
identities and invokes both synthetic success and failure mutations.

Actual Windows qualification used two independent zero-retry invocations. The
first correctly rejected a cutoff clone that had been checked out under
`core.autocrlf=true` before being switched to `false`; its tracked tree was
dirty, so it returned `P44-BUILD-FREEZE-FAILURE` before custody and cleaned
both runtime roots. A newly created clone fixed `core.autocrlf=false` before
checkout. That clean invocation built once with retention enabled and
published an exact final `2/2` executable/receipt pair with one rename, zero
retries, artifact size `1436672`, and artifact SHA-256
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
The runtime checkout, work root, and final custody root were removed after
recording the public-safe result. This is infrastructure evidence only, not
diagnostic execution or any product/category/fix conclusion.

The manifest, qualification receipt, and release seal bind this release's
file set, control totals, qualification posture, and identities.
