# Pulse 32 public result

Disposition: `invalid` at `cutoff-build-freeze`.

The public checkout gates passed: 36/36 attribute and LF checks, 76/76
bindings, and exact package verification for 20 files, 20 hashes, four
aggregates and six report/receipt/seal bindings. The Windows cutoff
executable built, but the required Ubuntu executable was unavailable to the
custodian, so no environment freeze completed.

Pulse 33 later diagnosed the Ubuntu blocker as WSL non-login shell
orchestration omitting the ordinary rustup Cargo directory from `PATH`.
Compilation succeeds when Cargo is discovered explicitly; no FERRIS product
change is required.

There were zero preflight operations, zero public-input validation
classifications, zero cases, and zero candidate or minimization
processes. The category conclusion is `null`; further launches are
prohibited.

Raw result SHA-256:
`sha256:27ff0f0c2a4768628fdcdfa7916efa7fe12217faa7bec20f65dbde8e526f88fd`.
Receipt ID:
`sha256:cf48f0ddc7102d29084529b1ffe5b8812acd6b2d5cf75ec544265a1b3c0238cd`.

See `PULSE-32-PUBLIC-RESULT.json` for the complete public-safe counts.
