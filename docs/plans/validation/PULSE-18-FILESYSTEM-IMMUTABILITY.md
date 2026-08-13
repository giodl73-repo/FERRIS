# Pulse 18 Filesystem Immutability Validation

Date: 2026-08-12
Implementation cutoff: `ecb10e7ed82009e1a7cf46eb585f97e3769102b8`
Disposition: Windows and Unix development validation passed
Evidence class: Development, not sandbox or held-out evidence

## Scope

This receipt validates a new public-CLI integration test:

```text
profile_diff_does_not_mutate_inputs_or_working_directory
```

For each of the nine Pulse 15 profile families, the test:

1. reads the exact before and after fixture bytes;
2. records file length and modification time;
3. records the complete input-directory entry set;
4. creates a unique empty temporary working directory;
5. runs the public `ferris profile-diff` process from that directory;
6. requires typed difference exit 1 with stderr empty;
7. rereads and compares both input files byte for byte;
8. compares file lengths and modification times;
9. compares the input-directory entry set; and
10. requires the working directory to remain empty.

The helper requests temporary-directory cleanup after the test; the scored
assertion is that the directory is empty before cleanup.

## Windows evidence

- OS: Windows 10 Enterprise build 26310, 64-bit;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- source cutoff:
  `ecb10e7ed82009e1a7cf46eb585f97e3769102b8`.

Commands:

```console
cargo fmt --all --manifest-path C:\src\FERRIS\Cargo.toml -- --check
cargo test --quiet --locked --workspace --manifest-path C:\src\FERRIS\Cargo.toml
cargo clippy --quiet --locked --workspace --all-targets --manifest-path C:\src\FERRIS\Cargo.toml -- -D warnings
git -C C:\src\FERRIS diff --check
```

Result: 64 tests passed, 2 bounded-command helper tests were ignored, 0
failed, and formatting, Clippy, and diff checks passed.

## Unix evidence

- distribution: Ubuntu 24.04.4 LTS under WSL2;
- kernel:
  `6.6.87.2-microsoft-standard-WSL2`;
- architecture: x86-64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`;
- isolated target directory: `/tmp/ferris-p18-rust195-target`;
- Cargo network mode: offline; and
- source cutoff:
  `ecb10e7ed82009e1a7cf46eb585f97e3769102b8`.

Commands:

```console
cd /mnt/c/src/FERRIS
export CARGO_TARGET_DIR=/tmp/ferris-p18-rust195-target
export CARGO_NET_OFFLINE=true
cargo fmt --all -- --check
cargo test --quiet --locked --workspace
cargo clippy --quiet --locked --workspace --all-targets -- -D warnings
```

Result: 64 tests passed, 2 bounded-command helper tests were ignored, 0
failed, and formatting and Clippy passed.

## What the evidence establishes

At the recorded cutoff and environments, all nine development fixture pairs
remain byte-identical after `profile-diff`; their lengths, modification times,
and directory membership remain unchanged; and the isolated process working
directory remains empty.

This supports the Pulse 14 local read-only and removal boundary: using the
command creates no fixture-local or current-directory state that must later be
cleaned up.

## What the evidence does not establish

The test is not an operating-system sandbox or syscall audit. It does not
observe:

- filesystem locations outside the two input directories and isolated working
  directory;
- access-time changes;
- registry, process-global, kernel, or service state;
- network traffic;
- external owner tools;
- hidden held-out inputs; or
- ordinary Cargo behavior in a consumer repository.

The implementation contract and source review continue to prohibit those
effects, but this test alone does not prove their absence. It also does not
complete the PRODUCT-001 Removal Record or PLATFORM-001 removal and rollback
gates.
