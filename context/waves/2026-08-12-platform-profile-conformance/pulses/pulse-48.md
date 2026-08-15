# Pulse 48: witnessed-publication process-exit diagnostic closeout

Status: Permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion. Blocker: `P48-P43-CATALOG-PRIVACY-IDENTIFIER` at
`public-result-publication`.

## Sole-launch public record

Authority commit `5a8d92d211806d0f2940016af6c317878c5fdfc1`, cutoff
`70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d`, and declaration identity
`sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e`
bind the sole launch. The committed declaration remains the exact pre-launch
authority and is not amended by this closeout.

The public Pulse 43 result root is absent. The Pulse 47 witness root contains
exactly two files, `publication-witness.json` and `release-receipt.json`.
It witnesses `P43-PRIVACY-BEARING-IDENTIFIER`, absent publication, zero rename
attempts, zero retries, and stage/final-parent/rollback-parent sync all
`not-attempted`.

## Bounded conclusion

Public reproduction against exact Pulse 43 rejects the committed catalog:
`private-materialization` contains forbidden identifier part `private`. This
establishes only a public catalog/publisher incompatibility, not whether or
how far private execution progressed. No private data, gate, or search
inference and no rerun occurs.

Category, diagnostic, and product conclusions are null; no fix authority
exists. Further launch is prohibited. A future redesign may use neutral
`bounded-materialization`, but this record creates no new authority.

## Evidence

- [Normative authority](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_48_AUTHORITY.md)
- [Exact declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-48-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-48-authority.v1.schema.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-48-authority-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-48-WITNESSED-PUBLICATION-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_48_authority.rs)
- [Result/witness validator](../../../../crates/ferris-cli/tests/pulse_48_publication_witness.rs)

Declaration identity:
`sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e`.
The exact pre-launch declaration has 9,498 mutation controls, 48,317 declared
registry controls total, and zero execution state.
