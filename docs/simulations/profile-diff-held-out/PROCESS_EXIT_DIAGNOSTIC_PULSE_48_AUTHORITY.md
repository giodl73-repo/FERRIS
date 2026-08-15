# Pulse 48 witnessed-publication authority closeout

Status: Permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion. Blocker: `P48-P43-CATALOG-PRIVACY-IDENTIFIER` at
`public-result-publication`.

## Sole-launch public record

Authority commit `5a8d92d211806d0f2940016af6c317878c5fdfc1`, immutable cutoff
`70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d`, and declaration identity
`sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e`
bind Pulse 48's sole launch. The original declaration, schema, and mutation
controls remain the committed, exact pre-launch authority; this closeout adds
no authority.

The public Pulse 43 result root is absent. The exact Pulse 47 witness root
[`pulse-48-publication-witness`](pulse-48-publication-witness/) exists with
exactly `publication-witness.json` and `release-receipt.json`. Its witnessed
Pulse 43 failure is `P43-PRIVACY-BEARING-IDENTIFIER`: publication is absent,
rename attempts and retries are `0`, and stage, final-parent, and
rollback-parent sync are all `not-attempted`.

The witness raw/payload identities are
`sha256:65183b80fba13f27a6680e2d0f99f0410e40c659446e39716856cb8aed63c6f1`
and
`sha256:5c547fb2c482f1879bd18bc17d8e574dd7e2cc676f3e51e9d5d7ea8f1dfca35c`.
The receipt raw/payload identities are
`sha256:e2f7e44e89731e4ac2bccae1c2f9312832cee33368aa368696ff218a0e6e9c01`
and
`sha256:07607f76c9cc548655ba298c3d9f9f2e62efa643f009de48f3391816029fe265`.

## Public incompatibility only

An independent public reproduction against the exact Pulse 43 catalog
validator rejects the committed Pulse 48 catalog because gate identifier
`private-materialization` contains the forbidden identifier part `private`.
This establishes only a public catalog/publisher incompatibility. It does not
state whether, or how far, private execution progressed; it makes no private
data, gate, or search inference and performs no rerun.

Category, diagnostic, and product conclusions are null, and no fix authority
exists. No retry, resume, reconstruction, reseed, reuse, correlation, or
inference is permitted. A future redesign may use neutral
`bounded-materialization`, but no new authority is created here.

## Evidence

- [Exact pre-launch declaration](fixtures/process-exit-diagnostic-pulse-48-authority.json)
- [Closed pre-launch schema](schemas/ferris.process-exit-diagnostic-pulse-48-authority.v1.schema.json)
- [Pre-launch mutation controls](fixtures/process-exit-diagnostic-pulse-48-authority-mutations.json)
- [Nine-role authority review](../../plans/reviews/PULSE-48-WITNESSED-PUBLICATION-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Rust authority validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_48_authority.rs)
- [Rust result/witness validator](../../../crates/ferris-cli/tests/pulse_48_publication_witness.rs)
