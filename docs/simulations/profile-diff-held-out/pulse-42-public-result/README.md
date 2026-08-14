# Pulse 42 Public Result: Publication-Integrity Closure

Disposition: `invalid-publication-integrity` (permanently invalid,
non-retryable, null category conclusion).

The public closeout is at `public-result-publication` with
`P42-PUBLIC-RESULT-UNAVAILABLE-ORDER-INCONSISTENT`. It grants no product,
diagnostic, category, or fix authority.

## Public evidence boundary

At closeout, the expected custodian result files were absent (`1`) and claimed
result paths observed were `0`. The committed authority orders Pulse 41, Pulse
39, package, Pulse 33 freeze, preflight, Pulse 31, normalized Pulse 35/Pulse
37, private materialization, then search with stop-on-failure. The public
summary cannot reconcile its claimed Pulse 33 stop with its later ordered
counts. This determines publication/order integrity only; it does not
determine whether any ordered gate ran.

The sole public custodian summary is preserved in `reported_unvalidated`.
Its claimed `P42-FROZEN-BINARY-UNAVAILABLE` blocker is not an established
gate result or root cause. Its reported `39/39`, Pulse 35, materialization,
and search-adjacent quantities are not execution conclusions. The reported
`0/140` search count is unvalidated and not certified.

All privacy disclosure fields are false. No private seed, corpus, descriptor,
workspace path, discarded stream, unreleased candidate, or prior custody is
published.

## Canonical machine artifacts

- Result envelope: `public-result.json`
  - Raw identity:
    `sha256:ff0ba22671e9e08f1234db1b6a4949bf0d0f7345b975028ef19d9c3f0741e433`
  - Canonical payload identity:
    `sha256:5fb7cda080f5f4ec5287da1902f937325bdbeacf0f3020d1d8d4923f23e6a46b`
- Receipt envelope: `release-receipt.json`
  - Raw identity:
    `sha256:3313775ddbc126133b414daf279f7ab4ebf1882363b9a0c252ba23f39a05eb65`
  - Canonical payload identity / receipt ID:
    `sha256:44b87f0643dc082a9ad9166873aa12e4cc7d062d6cf9bbfaa995d83122ef11b2`

The integration test
`crates/ferris-cli/tests/pulse_42_public_result.rs` verifies the envelopes,
privacy, null conclusions, and the authority-order mismatch.
