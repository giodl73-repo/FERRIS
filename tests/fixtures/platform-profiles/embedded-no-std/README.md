# Embedded and `no_std` Profile Family

Status: Controlled family under Pulse 07

Both revisions are safe-Rust, zero-dependency `#![no_std]` libraries that
write one deterministic sensor frame into caller-provided storage.

Revision `r1` emits a four-byte reading frame. Revision `r2` adds explicit
status flags and a checksum in a six-byte frame. Host tests execute behavior;
`thumbv7em-none-eabi` compilation proves only the exact target build, not a
board, runner, firmware, timing, or hardware-support claim.
