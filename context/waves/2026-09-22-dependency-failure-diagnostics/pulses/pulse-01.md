# Pulse 01: First-Class Cargo Metadata Failure Classes

Status: Complete

## User outcome

Allow a Rust maintainer or release gate to distinguish an observed dependency
resolution/source failure from lockfile, offline-access, and unknown Cargo
metadata blockage without exposing repository paths or Cargo output.

## Control record

- Product outcome: replace one generic blocked result with stable actionable
  failure classes at Ferris's existing Cargo metadata boundary.
- Maximum effort: one core classifier, the existing metadata call sites,
  focused tests and docs, and no new command or schema.
- Completion test: real missing-path dependency output classifies as dependency
  blocked; lockfile and offline shapes remain distinct; malformed manifests
  remain invalid; unknown failures remain blocked; private output is absent;
  full tests and lint pass.
- Abandonment condition: stop if the work requires arbitrary rustc parsing,
  raw owner-output retention, another owner command, or a change to
  `ferris.command-result/v2`.
- Product Value Governor: `continue-within-budget`.

## Result

Ferris now emits `FERRIS-CARGO-DEPENDENCY-BLOCKED`,
`FERRIS-CARGO-LOCK-BLOCKED`, or `FERRIS-CARGO-OFFLINE-BLOCKED` when the
retained Cargo metadata diagnostic contains the corresponding stable observed
shape. Unknown failures remain `FERRIS-CARGO-METADATA-BLOCKED`, and malformed
manifests remain `FERRIS-MANIFEST-INVALID`.

The existing `ferris.command-result/v2` shape is unchanged. Output contains a
safe class message, actionable owner step, and digest evidence but no raw Cargo
diagnostic, dependency name, or repository path. The real missing-path fixture,
seven focused classification controls, 131 core tests with two ignored, 62 CLI
tests, and workspace clippy with warnings denied passed on Windows with the
installed Cargo toolchain.
