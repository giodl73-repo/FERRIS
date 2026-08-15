# Pulse 56: retained deterministic build and custody release

Status: Complete sealed retained-build/custody infrastructure only

## Goal

Replace the unsound staged retained-identity diagnostic executor at the lower
layer. Pulse 56 does not create or bind a diagnostic executor. It supplies
only a new retained deterministic build, receipt, two-file custody, and
verified-launch-handoff foundation for a separately authorized future layer.

## Released control

The injection-free production API internally creates fresh absent work, stage,
and final roots; materializes two clean `core.autocrlf=false` checkouts at
`29517d732db13cc2ffa304684b344f3538ab587d`; builds `ferris-cli` twice into
distinct target roots; requires byte-identical artifacts; creates a new
semantic `retained:true` receipt; and publishes exactly the binary and receipt
via one rename with zero retries.

The receipt binds artifact identity, cutoff, checkout, command and controls,
Cargo/rustc executable identities and versions, host, rustup toolchain,
target-libdir, actual linker route and identity, controlled environment
identity, and both-build evidence. It does not compare or replay Pulse 33's
non-retaining receipt.

Windows uses toolchain-shipped `rust-lld.exe` under sysroot with
`/Brepro`, `/timestamp:0`, `/debug:none`, and source-path remapping. Ubuntu
WSL records the failed shipped-rust-lld system-library route, then binds the
actual `/usr/bin/cc` and GNU `ld` route, search identity, environment,
`--build-id=sha1`, `debuginfo=0`, and remapping. Real two-build probes passed
on both platforms.

Public `verify_custody` is evidence validation only and cannot produce launch
authority. The future-only `launch_verified` accepts only the original live
in-process `CustodyHandle`, exact platform, and concrete string arguments.
Its private registry owns exact measured bytes and bounded uses; Linux executes
a held verified inode from a native WSL root and Windows holds a
write/delete-denying image handle through process creation. Pulse 56 does not
call that primitive. A later authorized wrapper MUST call
`close_custody(handle)` on every early-stop and still-live `finally` path; it
may never clean a path itself. Active launches are coordinated under the
registry lock, and any uncertain cleanup is fatal rather than a success.

## Non-goals

No diagnostic executor, authority, candidate, process execution, seed,
materialization, publication, witness, product behavior, score,
certification, fix, or PLATFORM-001 conclusion is created. Pulse 55 remains
permanently closed and is neither retried, resumed, nor amended.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/README.md)
- [Receipt](../../../../docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-56-RETAINED-BUILD-CUSTODY-RELEASE-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_56_retained_build_custody_release.rs)
