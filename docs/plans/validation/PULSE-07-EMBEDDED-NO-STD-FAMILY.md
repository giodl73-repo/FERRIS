# Pulse 07 Embedded and `no_std` Family Validation

Date: 2026-08-12
Implementation cutoff: `ed214488aa19d025a9c9565dbe6db828b43582ac`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope

Two exact safe-Rust, zero-dependency `#![no_std]` libraries encode one sensor
reading into caller-provided storage:

- `r1` emits a four-byte frame containing a 12-bit reading; and
- `r2` emits a six-byte frame adding four status bits and an XOR checksum.

Both reject invalid input or insufficient storage before output mutation.

## Exact identities

| Revision | Package | Source-tree digest | Canonical profile digest |
|---|---|---|---|
| `r1` | `ferris-profile-embedded-no-std@0.1.0` | `sha256:0f0491f59612c5c038e04b9d0e02dd7984540ad63d9297786061f696d150b3e3` | `sha256:e71420eb5e6ae42b42025ec73de2e535c15df909696fd84a8396f2d6695ed1c5` |
| `r2` | `ferris-profile-embedded-no-std@0.2.0` | `sha256:fa17335af28fd43a2fdc2a7d27277f64091038e5716fbd1ff7f67a7a7cbefa11` | `sha256:3e4aa47ed3c825eccfac5ab601c409ee50037a77a180a7d2ac2d42d50ca22eeb` |

## Owner evidence

Each revision passes locked/offline metadata, host unit tests and doctests,
package construction, and target check, build, and Clippy with warnings denied
for `thumbv7em-none-eabi` in separate external target directories. Every
command leaves the complete source and lock tree byte-identical.

The canonical profiles retain target execution and operational validation as
unavailable because no board, emulator, runner, or device is configured.

## Platform evidence

Windows build 26310 and Ubuntu 24.04.4 WSL2 both used Rust/Cargo 1.95.0 with
the exact `thumbv7em-none-eabi` Rust standard-library target. Each full
workspace run reported 71 passing tests, 2 ignored bounded-command helpers,
and 0 failures. Formatting and Clippy passed on both; Windows Git diff
validation passed.

## Claim boundary

This completes only the controlled embedded/`no_std` family. Target
compilation and host tests do not establish a board, runner, firmware image,
linker script, device I/O, timing, memory use, energy use, hardware safety,
deployment, support, approval, held-out evidence, or PLATFORM-001 Proposed
status.
