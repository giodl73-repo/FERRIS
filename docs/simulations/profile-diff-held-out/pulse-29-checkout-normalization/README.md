# Pulse 29 checkout-normalization receipt

Disposition: **pass**.

A disposable alternate Git index staged the resulting uncommitted state.
`git checkout-index` then materialized that index on Windows with
`core.autocrlf=true`; it did not copy the current working tree.

The materialized Pulse 25 and Pulse 27 release roots contained 36 files, all
with LF framing and zero CR bytes. Verification passed 76 of 76 checks:
22 Pulse 25 manifest/file/aggregate bindings, 45 Pulse 27 bindings, and nine
byte-identical collector-copy checks.

The machine-readable receipt is
`PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json`. Its raw SHA-256 is
`sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225`;
its sealed payload identity is
`sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40`.

No private data, build, preflight, generation, diagnostic candidate, retry,
production-code change, or new authority occurred. The Pulse 28 result remains
unchanged.
