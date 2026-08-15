# Pulse 56 retained-build/custody root cause

## Blocking findings closed

The staged design let public receipt/root evidence flow into a launchable
pathname, treated a frozen dataclass as a capability, reopened a receipt after
hashing, accepted arbitrary lazy launch arguments, and left the final image
open/replacement boundary underbound. Its build binding also hashed rustup
proxies, invoked ambient Git, and named an Ubuntu linker route without binding
its actual system inputs.

Those are infrastructure defects only. They do not alter Pulse 55's permanent
one-call closeout, create authority, or execute FERRIS.

## Replacement

Public receipt/custody verification now proves evidence only and returns no
authorization. A production publication creates a private object-identity
registry record with an unguessable token, exact internally measured artifact
bytes/identity, owned roots/file IDs, and bounded remaining launches. A future
same-process executor can supply only that original live object; copied,
reconstructed, receipt-derived, and root-derived objects fail.

Linux copies exact in-memory bytes to an `O_EXCL` native-WSL root, verifies one
held descriptor, and executes `/proc/self/fd/<fd>`. Windows holds a
write/delete-denying `CreateFileW` image handle from same-handle rehash through
successful `CreateProcess`; it documents the image lock instead of relying on
permissions. Synthetic-only tests mutate the pathname after the Linux
descriptor opens and still observe the original inode.

Tool discovery and both builds share the controlled environment and selected
`RUSTUP_TOOLCHAIN`; direct `rustup which` Cargo/rustc binaries must not be the
proxy files. Bound Git disables user/system/global configuration, filters, and
hooks. Windows binds rust-lld and target libraries. Ubuntu binds cc, collect2,
ld, startup objects, search output, and actual ld-traced startup/system inputs.
