# Pulse 38: Independent Normalized Public Diagnostic Authority

Status: Complete: permanently invalid before normalized checkout verification
Implementation authority: Exhausted; no retry, resume, replacement launch,
diagnostic conclusion, or product-fix authority

## Goal

Authorize one new independent `process-exit-agreement` diagnostic search only
under immutable cutoff `6807bd68aa01cbf0c819198765b7d6b5aa443328`. The cutoff
contains the complete Pulse 37 normalization and predates the authority.
Pulse 38 binds the normalized Pulse 35 release to exact cutoff Git blobs. It
creates no custody or diagnostic activity in this repository change.

## Immutable historical boundary

Pulses 22, 24, 26, 28, 30, 32, 34, and 36 remain permanently invalid,
non-retryable, and null-conclusion. Pulse 36 remains closed at
`pulse35-release-copy-verification`; it is not retried, resumed, reseeded,
reused, correlated, or used for inference. Pulse 37 preserves that history and
only supplies the normalized public-artifact proof.

## Ordered authority

Pulse 38 inherits every Pulse 36/Pulse 34 gate and bound. Custody must pass
normalized checkout, exact Pulse 25/Pulse 27 package custody, Pulse 33 build
freezes, adapter preflight, and Pulse 31 validation. It must then exactly copy
the normalized eight-file Pulse 35 release and prove Pulse 37 clean filtering
against cutoff Git blobs. Only then may it make a new private 32-byte CSPRNG
seed, materialize 70 descriptors, and run fresh private verification. Only
then may one transactional cross-platform search run under 70 cases/processes
per platform and 140 processes total, with zero retries and stop at the first
target mismatch. Inherited minimization and publication rules apply.

## Bound identities

- manifest/aggregate/total:
  `sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
  `sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
  and `403316`;
- seal raw/payload:
  `sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23` /
  `sha256:834781867ea008dc14a54d7b811002ee1b8fa759c0b1d7f32432ea6c0d5c5375`;
- Pulse 37 receipt raw/identity:
  `sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6` /
  `sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae`; and
- declaration identity:
  `sha256:a3317422e8c34d4e08d7c5e577e3539820f1376d7fba2ef38d262d1f967031b4`.

The six text files are exact LF bindings, the two JSON files are unchanged,
and the clean-filter proof is 8/8 with zero CR. Pulse 35's qualification
payload, machine schema, 70-case, tuple, seed/HMAC, request-resolution,
change-count, sync, one-rename, and zero-retry terms remain binding.

## Evidence

- [Normative authority](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_38_AUTHORITY.md)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-38-authority.v1.schema.json)
- [Exact declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-38-authority.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-38-authority-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-38-NORMALIZED-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_38_authority.rs)

## Decision

The sole independent attempt bound the cutoff and completed one cutoff
materialization plus the release-tree cardinality check. Its first required
attribute check did not complete, so custody stopped at
`normalized-checkout-verification` under the no-retry rule. No package, build,
preflight, input, seed, materialization, candidate, search, or minimization
activity occurred. The category conclusion remains null.

The immutable [public result](../../../../docs/simulations/profile-diff-held-out/pulse-38-public-result/README.md)
has raw digest
`sha256:d3e74d220a9de9da4f2fff72812443de42272c9a8f78b0efad37573ab33b1c9c`
and receipt
`sha256:56ddacc0e3043b327b8ce2d6ce869e9662a564faee9ce4f9a2c3d783a390bdad`.
Pulse 38 is permanently invalid and non-retryable. It changes no product
behavior, does not alter Pulse 17, and grants no fix or PLATFORM-001 authority.