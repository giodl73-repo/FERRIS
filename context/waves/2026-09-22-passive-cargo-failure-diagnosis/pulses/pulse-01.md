# Pulse 01: Passive Cargo Failure Diagnosis

Status: Complete

## User outcome

Allow a maintainer or automation gate to classify stderr captured from an
ordinary owner-run Cargo command without giving Ferris execution authority or
exposing the diagnostic in Ferris output.

## Control record

- Product outcome: make the existing dependency-failure vocabulary reusable
  for owner-run Cargo while preserving Cargo and repository ownership.
- Maximum effort: one passive command, one bounded report schema, one core
  module, catalog registration, focused fixtures/tests, and docs.
- Completion test: deterministic three-entrypoint output; strict 64 KiB input;
  dependency, lockfile, and offline classifications; a synthetic-scale-style
  assertion remains unclassified; no raw text, paths, or dependency names;
  full tests and lint pass.
- Abandonment condition: stop if useful output requires running Cargo, parsing
  rustc/test semantics, extracting owner identifiers, accepting truncated
  input, or modifying a synthetic repository.
- Product Value Governor: `continue-within-budget`.

## Result

`ferris diagnose-cargo --stderr <FILE>` now reads one complete UTF-8 file up to
64 KiB and emits `ferris.cargo-failure-report/v1`. The record classifies only
dependency, lockfile, and offline-policy shapes; all other input is explicitly
`unclassified`. It binds the input digest and byte count while retaining no raw
text, filename, package name, dependency identity, or source path.

The report is registered as the forty-sixth installed contract and is
byte-identical across `ferris`, `cargo-ferris`, `cargo ferris`, and relocated
equal input. Missing, empty, non-UTF-8, and oversized files fail closed. The
synthetic-scale-style assertion fixture remains unclassified. Validation passed
with 131 core tests and two ignored, 62 legacy CLI tests, four diagnosis tests,
seven contract tests, touched-file rustfmt, parsed JSON Schema, diff checks, and
workspace clippy with warnings denied. One test-only unused import was corrected
after the first focused CLI compile; no product retry was required.
