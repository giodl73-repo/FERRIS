# Pulse 01: Cargo Failure Standard Input

Status: Complete

## Control record

- User outcome: CI can pipe one already-captured Cargo stderr diagnostic into
  Ferris without creating a temporary file.
- Maximum effort: one shared bounded reader, one CLI convention, focused tests,
  and documentation; no schema change.
- Completion test: file and stdin reports are byte-identical across all three
  entrypoint forms, while empty, non-UTF-8, and oversized stdin fail closed.
- Abandonment condition: stop if stdin requires asynchronous streaming,
  unbounded buffering, report-schema changes, Cargo execution, or batch policy.
- Product Value Governor: `continue-within-budget`.

## Claim boundary

EOF marks the complete caller-supplied input. Ferris does not prove Cargo
provenance, execute a command, retain raw stderr, or interpret rustc and test
failures.

## Result

`ferris diagnose-cargo --stderr -` now reads one bounded stdin stream through
EOF using the same core reader as file input. Equal bytes produce the same
report and identities across `ferris`, `cargo-ferris`, and `cargo ferris`.
Empty, non-UTF-8, and oversized stdin fail closed with the existing typed
diagnostics. No report schema changed.

Validation passed with five focused diagnosis tests, 131 core unit tests and
two ignored, 62 legacy CLI tests, touched-file rustfmt, workspace clippy with
warnings denied, and diff checks. The broader core package run additionally
passed eight platform-profile tests; three target-specific tests could not run
because `thumbv7em-none-eabi`, `wasm32-wasip2`, and `wasm32-unknown-unknown`
are not installed in this environment. Disposition: `stop-complete`.
