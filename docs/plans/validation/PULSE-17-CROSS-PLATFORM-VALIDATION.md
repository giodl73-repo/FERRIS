# Pulse 17 Cross-Platform Development Validation

Date: 2026-08-12
Implementation cutoff: `f9305bdb5696da4889864b9c885ab4e18a56cdba`
Disposition: Windows and Unix development validation passed
Evidence class: Development, not held-out

## Scope

This receipt validates the existing bounded Ferris workspace and Pulse 15
nine-family `profile-diff` fixture matrix on Windows and Unix from the same
source cutoff.

It does not use a candidate Pulse 16 held-out package, hidden input, privacy
canary, expected record, scorer, or oracle. It does not establish platform
support, owner evidence, profile truth, compatibility, approval, production
readiness, or PLATFORM-001 advancement.

## Windows environment

- OS: Windows 10 Enterprise;
- build: 26310;
- architecture: 64-bit;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- source cutoff:
  `f9305bdb5696da4889864b9c885ab4e18a56cdba`.

Commands:

```console
cargo fmt --all --manifest-path C:\src\FERRIS\Cargo.toml -- --check
cargo test --quiet --locked --workspace --manifest-path C:\src\FERRIS\Cargo.toml
cargo clippy --quiet --locked --workspace --all-targets --manifest-path C:\src\FERRIS\Cargo.toml -- -D warnings
git -C C:\src\FERRIS diff --check
```

Results:

- formatting passed;
- 63 tests passed;
- 2 bounded-command helper tests were ignored;
- 0 tests failed;
- Clippy passed with warnings denied; and
- Windows Git diff validation passed.

## Unix environment

- distribution: Ubuntu 24.04.4 LTS under WSL2;
- kernel:
  `6.6.87.2-microsoft-standard-WSL2`;
- architecture: x86-64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`;
- isolated target directory: `/tmp/ferris-p17-rust195-target`;
- Cargo network mode: offline; and
- source cutoff:
  `f9305bdb5696da4889864b9c885ab4e18a56cdba`.

Commands:

```console
cd /mnt/c/src/FERRIS
export CARGO_TARGET_DIR=/tmp/ferris-p17-rust195-target
export CARGO_NET_OFFLINE=true
cargo fmt --all -- --check
cargo test --quiet --locked --workspace
cargo clippy --quiet --locked --workspace --all-targets -- -D warnings
```

Results:

- formatting passed;
- 63 tests passed;
- 2 bounded-command helper tests were ignored;
- 0 tests failed; and
- Clippy passed with warnings denied.

The CLI integration suite executed all nine Pulse 15 family pairs through the
public `profile-diff` process path.

## Line-ending observation

Exploratory runs in the separate `Ubuntu-24.04` WSL registration are not
counted in this receipt. One combined command appended `git diff --check`.
Linux Git viewed the Windows-mounted checkout through different line-ending
configuration and reported the existing CRLF working-tree representation as
whole-repository whitespace differences after the Rust gates had passed.

The final recorded Unix run was repeated in the `Ubuntu` WSL registration
with the exact Rust/Cargo 1.95.0 toolchain and used Rust-owned gates only.
Repository diff hygiene remained checked by Windows Git in the checkout that
owns the working tree. No file was rewritten to suppress or hide the
observation.

## Claim boundary

This receipt establishes that the current development suite and bounded
profile-diff fixture matrix execute successfully on the recorded Windows and
Unix environments at the recorded cutoff.

It does not replace:

- native Linux hardware evidence;
- additional Unix distributions or architectures;
- real owner-tool, native-library, provider, target, or deployment evidence;
- the independently constructed Pulse 16 held-out package;
- the required 112-process held-out collection; or
- support, compatibility, certification, approval, or release authority.
