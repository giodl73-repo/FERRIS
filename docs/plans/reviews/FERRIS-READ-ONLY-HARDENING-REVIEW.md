# Ferris Read-Only Hardening Review

Date: 2026-08-11
Scope: Pulse 03 corrective hardening for `plan`, `explain`, and `graph`
Disposition: Complete; independent review findings corrected; applicable scoring passed
Implementation authority: No expansion

Historical note: Pulse 05 later aligned Cargo metadata toolchain selection
with passive doctor and added the same rustup auto-install/update guards. The
held-out result below remains evidence for its frozen cutoff only.

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

## Held-out result

The earlier Pulse 01 and Pulse 02 completion reviews and held-out receipts
remain immutable historical records. They do not establish the corrected
identity or output claims.

The independent custodian verified cutoff
`c3590a39fd053a66996909b87eaf7ca7ac73ded4`, its tag, all 12 sealed
package digests, and the required CLI binding before execution. Classification
occurred before execution:

- FHIF-009 passed `P03-LOCAL-PLAN-HARDENING`;
- FHIF-012 passed `P03-BOUNDED-READONLY-PARITY`;
- no applicable fixture failed, blocked, or was invalid; and
- ten fixtures were outside Pulse 03 and were not executed.

The public-safe receipt exposes only opaque fixture IDs, requirement codes,
result classes, exit codes, and output digests.

## Decision

Pulse 03 is complete on the recorded Windows and Unix environments. It fixes
all reviewed defects without expanding the read-only capability boundary.
