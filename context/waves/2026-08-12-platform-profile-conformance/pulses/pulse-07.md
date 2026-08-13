# Pulse 07: Embedded and `no_std` Profile Family

Status: Complete
Implementation authority: Bounded to this document

## Goal and authority

Complete one zero-dependency embedded family as a safe-Rust `#![no_std]`
library compiled for `thumbv7em-none-eabi` and tested on the owner host.

Revision `r1` encodes one bounded sensor reading into a caller-provided fixed
frame. Revision `r2` adds explicit status flags and a deterministic checksum,
changing the exact frame contract without adding allocation or a runtime.

This pulse authorizes:

- two exact local `#![no_std]` library revisions and lockfiles;
- positive, out-of-range, and undersized-buffer owner tests;
- host tests plus target check, build, and Clippy for
  `thumbv7em-none-eabi`;
- explicit target, allocator, runner, runtime, memory, and unavailable states;
- reusable test-only v1 profile materialization and exact digests;
- locked/offline owner Cargo stages in isolated target directories;
- source-tree immutability;
- Windows and Unix development validation; and
- the bounded nine-role authorization review.

It does not authorize a board, emulator, runner, device I/O, interrupt,
register, linker script, allocator, unsafe code, firmware image, flashing,
hardware support, production profile generation, another family, support,
approval, or held-out access.

## Acceptance

- both revisions retain `#![no_std]`, zero dependencies, safe Rust, and no
  allocator;
- `r1` and `r2` preserve exact and distinct frame contracts;
- invalid readings and short buffers fail before output mutation;
- target check, build, and Clippy pass for `thumbv7em-none-eabi`;
- host tests, doctests, metadata, and package pass;
- execute remains unavailable because no runner or device is configured;
- consumer trees remain unchanged;
- profiles contain all 15 stage states and stable distinct digests;
- Windows and Unix use Rust/Cargo 1.95.0; and
- all nine roles accept the measured result.

## Stop conditions

Stop if work requires hardware, a target runner, unsafe code, a linker script,
device-specific crates, external dependencies, allocation, network, flashing,
deployment, production behavior, support, or a broader platform claim.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-07-EMBEDDED-NO-STD-ROLE-REVIEW.md)
- [Windows and Unix validation](../../../../docs/plans/validation/PULSE-07-EMBEDDED-NO-STD-FAMILY.md)

Implementation cutoff:
`ed214488aa19d025a9c9565dbe6db828b43582ac`.

Both exact revisions passed host behavior and `thumbv7em-none-eabi` target
gates on Windows build 26310 and Ubuntu 24.04.4 WSL2 with Rust/Cargo 1.95.0.
The workspace reported 71 passing tests, 2 ignored bounded-command helpers,
and no failures. Target execution and operational validation remain
unavailable because no board, emulator, runner, or device is configured.
