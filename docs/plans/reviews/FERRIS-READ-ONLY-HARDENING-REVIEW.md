# Ferris Read-Only Hardening Review

Date: 2026-08-11
Scope: Pulse 03 corrective hardening for `plan`, `explain`, and `graph`
Disposition: Validated; independent review findings corrected; scoring pending
Implementation authority: No expansion

## Corrections

The corrective implementation:

- requires an explicit portable workspace ID and binds it into plan and graph
  identities;
- binds invocation identity to semantic command, workspace, normalized
  manifest selection, metadata format, no-dependency mode, offline mode, and
  locked mode;
- records portable-equivalent command semantics and a Cargo output digest;
- preserves raw Cargo stderr only for internal classification and publishes a
  safe diagnostic with a source digest;
- renders omitted scope, unknowns, evidence, fallback, edge aliases, kinds,
  optional state, conditions, resolutions, and limitations in human output;
  and
- emits a typed invalid JSON envelope for malformed JSON-mode command lines.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

The final development suite contains 14 core tests and 9 CLI tests. It covers
workspace-separated record identity, cross-checkout identity, command-separated
invocation identity, lexical request normalization, complete human graph and
explanation semantics, diagnostic path privacy, source digests, and JSON-mode
parse envelopes.

An independent code review found three remaining issues: error invocation
paths were not lexically normalized, stderr digests were calculated after
lossy trimming, and human graph output omitted node location and evidence.
All three were corrected before the final validation above.

## Held-out status

The earlier Pulse 01 and Pulse 02 completion reviews and held-out receipts
remain immutable historical records. They do not establish the corrected
identity or output claims. This review will record the replacement cutoff and
public-safe held-out results after those gates complete.
