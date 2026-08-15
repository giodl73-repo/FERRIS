# Pulse 56 retained deterministic build and custody

Status: sealed retained-build/custody infrastructure only. It creates no
diagnostic authority or executor and never executes FERRIS.

## Capability and threat model

The trusted local custodian OS and process build and retain the artifact. The
source cutoff and public receipt are anonymous, auditable evidence; the receipt
is **not** a signature and is never launch authorization. `verify_custody`
therefore accepts a public root only to validate evidence and explicitly
returns `receipt_authorizes_launch:false`.

Only `publish_retained_build_and_custody(platform, runtime_parent)` creates an
opaque in-process `CustodyHandle`. A private identity-keyed registry retains
the random 32-byte token, exact bytes measured from both artifacts, artifact
identity, owned runtime/custody/launch roots and file IDs, and 69 uses. A
copied, reconstructed, or dataclass-forged handle, a receipt, and a custody
root are not registry identities and fail. Each valid launch atomically
decrements the use count and increments an active-launch count before any
launch root exists. The final active last-use completion atomically retires the
handle and exact-cleans its recorded roots exactly once. Cleanup accepts no
caller path and removes only exact registry-created runtime, custody, and
launch roots whose file identities still match. `close_custody(handle)` is the
production early-stop API: it atomically retires the exact live handle and
cleans its roots, rejects expired handles, and refuses while a launch is
active. Any cleanup uncertainty is fatal `P56-INDETERMINATE-CLEANUP`, never a
success-shaped completion.

## Build and custody

The production API makes two fresh clean `core.autocrlf=false` detached
checkouts at `29517d732db13cc2ffa304684b344f3538ab587d`, builds distinct
targets, requires equal measured bytes, and writes a fresh semantic
`retained:true` receipt. It creates the public evidence pair from those
in-memory bytes through an `O_EXCL`, fsynced stage and a single rename. Receipt
hashing and JSON parsing use one opened descriptor.

One controlled allowlisted environment serves tool discovery and both builds.
It sets `RUSTUP_TOOLCHAIN` to the selected exact toolchain and resolves
`rustup which --toolchain <name> cargo` and `rustc`; direct executable
identities are recorded separately and must differ from rustup proxy files.
Git is a bound direct executable too. Clone and checkout use explicit
no-system/no-global configuration, disabled filters and hooks, `core.eol=lf`,
and a recorded Git binary/version identity. Toolchain and target-libdir trees
are remeasured for both builds and must match.

Windows uses the direct sysroot `rust-lld.exe`, binding the binary/help
identity, target libraries, and SDK-facing allowlisted environment with
`/Brepro`, `/timestamp:0`, `/debug:none`, and source remapping. Ubuntu-24.04
WSL binds the effective `/usr/bin/cc` → `collect2` → GNU `ld` route, startup
objects, GNU-ld search output, and every regular system/startup input selected
by the actual GNU-ld `-t` trace. The receipt records the effective route, not
a route label. Current platform artifact identities are in
[qualification-receipt.json](qualification-receipt.json).

## Future launch primitive

`launch_verified(handle, platform, arguments)` accepts only the exact live
handle, its platform, and a concrete `tuple` or `list` of plain strings. It
materializes and rejects invalid/lazy arguments before creating any launch
file; it accepts no root, receipt, executable, environment, cwd, callback, or
runner. The child receives a newly computed fixed allowlisted environment with
deterministic locale and handle-owned home/temp roots. It inherits neither
loader nor Python/Rust injection variables.

On Ubuntu, launch roots must be native Linux filesystems (not `/mnt/c`). The
exact in-memory bytes are written with `O_EXCL`, reopened and rehashed through
one descriptor, and executed only through that held descriptor's
`/proc/self/fd/<fd>` path with `pass_fds`. On Windows, `CreateFileW` opens the
fresh `O_EXCL` image with sharing that denies write/delete; the same handle is
rehashes before `CreateProcess`, and remains held until process creation has
opened the image. This is the Windows image lock; no `chmod` claim is used.

Qualification launches only harmless synthetic `/bin/sh` or `cmd.exe` copies
to test the primitive. The native-WSL control passes `profile-diff`-shaped
arguments and captures the exact output bytes. It never launches a FERRIS
artifact.

## Later-wrapper lifecycle contract

A later authorized wrapper MUST retain only its original `CustodyHandle`; it
MUST NOT retain or accept a custody path, runtime path, token, receipt, or
launch root. It MUST call `close_custody(handle)` on every early diagnostic
stop and from every `finally` path that still owns a live handle. It MUST
surface a close failure, including `P56-INDETERMINATE-CLEANUP`; it MUST NOT
report a completed diagnostic after cleanup fails. Concurrent wrappers MUST
treat `P56-HANDLE-ACTIVE` as a coordination failure and wait for or otherwise
coordinate the owning launch rather than racing root cleanup. A last-use
launch may already have auto-retired the handle before its wrapper's `finally`;
that `P56-HANDLE-EXPIRED` result is not a cleanup success claim.

## Qualification

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --actual --write-receipt
python -B generate_release.py
```

The adversarial suite covers forged public evidence, copied/forged handles,
single-descriptor receipt parsing, lazy argument rejection before side effects,
environment injection absence, early close, atomic use exhaustion and
last-use cleanup, active-close coordination, cleanup-failure non-success,
substituted-root refusal, and native-Linux descriptor execution after pathname
mutation.
