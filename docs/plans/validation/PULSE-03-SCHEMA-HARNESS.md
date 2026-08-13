# Pulse 03 Platform Profile Schema Harness Validation

Date: 2026-08-12
Implementation cutoff: `80ce90b332ca8e649d1b5bfd013da272934e9089`
Disposition: Windows and Unix development validation passed
Evidence class: Test-only schema-control conformance

## Scope

This receipt validates the dependency-free Rust integration harness that
executes the nine public platform-profile schema controls. The harness is test
support and does not add a production parser, CLI behavior, library API,
profile generation, owner adapter, completed family, or semantic decision.

## Control correction

The first run found that the Pulse 02 malformed mutation removed only the
fixture's final newline and therefore remained valid JSON. A two-byte
truncation also removed only the CRLF line ending on Windows.

The control was corrected to remove three bytes. That removes the closing
object delimiter on both LF and CRLF checkouts and preserves the intended
single malformed-input defect. The original Pulse 02 control-manifest digest
remains historical evidence; it is not rewritten.

Current frozen digests:

| File | SHA-256 |
|---|---|
| `tests/fixtures/platform-profiles/schema/controls.json` | `1fdb11ba452937402e71c404cf515bdc66e22d5880d9e2c2e57c37354a3d2352` |
| `crates/ferris-core/tests/platform_profile_schema.rs` | `384ef9bf62b6aad775ce116af37bb3ab0f48144cb916b74ff5c75e7a2f74a6ce` |

## Windows evidence

- operating system: Microsoft Windows 11 Enterprise Insider Preview;
- build: 26310, x64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`; and
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.

The full workspace reported 67 passing tests, 2 ignored bounded-command
helpers, and 0 failures. Formatting, Clippy with warnings denied, and Git diff
validation passed.

## Unix evidence

- distribution: Ubuntu 24.04.4 LTS under WSL2;
- kernel: `6.6.87.2-microsoft-standard-WSL2`;
- architecture: x86-64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- isolated target directory: `/tmp/ferris-p03-rust195-target`.

The Rust 1.95 validation components were restored after the first command
reported that `cargo-fmt` was absent. The rerun reported 67 passing tests,
2 ignored bounded-command helpers, and 0 failures. Formatting and Clippy with
warnings denied passed.

## Exact control results

| Control | Expected and observed |
|---|---|
| base exemplar | `valid` |
| unsupported schema version | `unsupported` |
| unknown top-level member | `invalid` |
| ambiguous path-plus-URI source | `invalid` |
| unknown success-shaped state | `invalid` |
| unsafe output-visible identifier | `invalid` |
| duplicate object member | `invalid` |
| malformed JSON | `invalid` |
| record above 4 MiB | `blocked` |

An additional direct test rejects a duplicate member nested below the top
level.

## Claim boundary

The evidence establishes deterministic execution of the frozen schema
controls at the recorded cutoff. It does not establish full JSON Schema
coverage, owner-observed profile evidence, a completed family, native Linux
support, profile compatibility, support, safety, trust, approval, held-out
conformance, or PLATFORM-001 Proposed status.
