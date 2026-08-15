# Pulse 55: immutable-blob witness-preserving diagnostic authority

Status: Permanently closed terminal-prerequisite-identity-failure

## Goal

Authorize one fresh independent single-use route over the sealed Pulse 53
executor without retrying, amending, or repairing permanently withdrawn Pulse
54. The immutable self-excluding cutoff is
`47113e444ef3309afec9a844f0cba62775f19f6f`.

## Authority

Pulse 55 binds the same exact P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/
P52/P53 executable chain and only
`run_witness_preserving_ordered_executor` as Pulse 54. It has the same
public-before-private ordering, one-call consumption, no direct P51/P52/P47/
P43 call, no retry/fallback/republication, `70/69/1` per-platform topology,
`140/138/2` total, and null-only transfer conclusions.

Every current release-tree and callable canonical identity is read from a Git
blob at the cutoff. The authority validator validates a checkout independently
against those canonical identities and only explicitly declared sealed
full-file CRLF/LF variants. For P35 it binds Pulse 37-normalized canonical LF
bytes and the exact Pulse 51 custody variant records with size and newline
framing. A new anonymous `core.autocrlf=false` checkout is the required runtime
posture.

## Permanent predecessor closure

Pulse 46 and Pulse 48 remain permanently `invalid-publication-integrity`.
Pulse 49 remains permanently withdrawn
`invalid-prelaunch-authority-integrity`; Pulse 50 remains permanently
withdrawn `invalid-prelaunch-infrastructure-integrity`. Pulse 54 is
permanently withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`. Pulse 54 is not retried or amended:
it made zero P53 or authority calls, produced no runtime, seed, result, or
witness artifact, and all conclusions remain null.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-55-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-55-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-55-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_55_AUTHORITY.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-55-WITNESS-PRESERVING-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Rust authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_55_authority.rs)

## Single-use execution and permanent closeout

At authority commit `36b3ac6b9692924af57c7c98b0a976835fe778f6`,
independent custody completed one fresh Pulse 44 operation per platform and
then invoked the sole Pulse 53 callable exactly once. It returned
`not-attempted` with one terminal stop at
`pulse-41-pulse-39-public-custody`, zero completed gates, zero seed,
descriptors, processes, no-launch dispositions, downstream calls, transfers,
results, or witnesses, and null conclusions.

The fresh Windows retained binary did not match Pulse 51's exact historic
artifact identity. Ubuntu matched its expected artifact identity, but both
fresh retained receipts necessarily differed from the published non-retaining
receipt identities because `retained_in_public_bundle` changed from `false`
to `true`. Windows `/Brepro` plus Rust/Cargo versions also underbound the
linker and SDK environment.

Pulse 55 is permanently closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`, non-retryable and non-resumable. A
successor requires a corrected retained-build/custody contract and a new
sealed diagnostic, ordered, and witness executor chain before any new
authority.

See the [execution record](../../../../docs/simulations/profile-diff-held-out/PULSE_55_EXECUTION_RECORD.md).