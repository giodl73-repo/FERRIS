# Pulse 42: Independent transactional-copy diagnostic authority

Status: Permanently invalid; non-retryable; null conclusion
Implementation authority: Closed public authority, documentation, fixtures,
and test-only validation only

## Historical goal

Authorize one new independent `process-exit-agreement` diagnostic authority at
immutable cutoff `2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8`, which contains
the complete Pulse 41 public transactional-copy release and predates this
authority.

## Immutable boundary

Pulse 38 remains permanently
`invalid-before-normalized-checkout-verification`, non-retryable, and
null-conclusion. Pulse 40 remains permanently `invalid` at
`pulse-39-release-custody`, non-retryable, and null-conclusion. Pulse 42 is
not a retry, resume, reseed, reuse, correlation, or inference of Pulse 40 or
Pulse 38. It preserves all predecessor closures, Cargo and owner authority,
the product boundary, and PLATFORM-001 Draft status.

## Ordered authority

Fresh custody first obtains one immutable read-only/public-artifacts cutoff
checkout and verifies exact LF Git-clean Pulse 41 release-tree bindings:
eight files, five manifest payloads, and 49120 bytes. The manifest
raw/aggregate identities are
`sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8` /
`sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755`.
The report raw/payload identities are
`sha256:e16b84700318b3bedb82b283d9af2df8ae963fe63465fd2056a43839dfcfd8ee` /
`sha256:fbcfd656024e3fd6b3e24d18cf8991532fd77284611ab81f578e1985f6a5b4cc`;
receipt raw/payload identities are
`sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c` /
`sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f`;
and seal raw/payload identities are
`sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a` /
`sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf`.

It then directly executes the verified cutoff
`transactional_copy.py` path with `PYTHONDONTWRITEBYTECODE=1`, an exact
absolute cutoff Pulse 39 source root, and a fresh absent absolute custody
final root. It MUST NOT first copy Pulse 41 through another copier. One
invocation requires source/stage/final `8/8`, independent final `8/8`,
eight destination file fsyncs, two honest `synced`/`unsupported` staging sync
attempts, one rename, zero retries, final-parent sync, zero rollback or
indeterminate publication, and zero residue.

Only then does one separate fresh `core.autocrlf=true` cutoff checkout run the
copied Pulse 39 verifier from below the custodial final root: exactly one
check-attr call plus one version probe, two Git processes, zero retry/fallback,
36/36 attributes/LF/zero-CR/safe catalog, and independent 76/76 bindings.
Pulse 39 manifest raw/aggregate identities remain
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`.

Only after these gates do inherited Pulse 25/27 package, Pulse 33 freeze,
adapter preflight, Pulse 31 `39/39`, normalized Pulse 35/Pulse 37, fresh
32-byte seed, 70 descriptor, `18/18`, `8/8`, and one
`<=70`-per-platform/`<=140`-total search gates proceed. Every gate stops at
the first mismatch; the sole search has one launch and zero retries.

## Evidence

- [Normative authority](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_42_AUTHORITY.md)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-42-authority.v1.schema.json)
- [Exact declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-42-authority.json)
- [Comprehensive mutations](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-42-authority-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-42-TRANSACTIONAL-COPY-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_42_authority.rs)

Declaration identity:
`sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc`.
The declaration freezes 9046 comprehensive controls, 29611 total declared
mutations, and zero execution state at every new and inherited gate.

## Public-result publication integrity closure

The sole public custodian summary lacks its claimed committed result:
`result_files_absent=1` and `claimed_result_paths_observed=0`. The committed
order requires the Pulse 33 freeze to precede Pulse 31, normalized Pulse
35/Pulse 37, private materialization, and search. The reported stop and later
reported quantities are therefore order-inconsistent public evidence, not
gate conclusions. Pulse 42 is permanently
`invalid-publication-integrity`, non-retryable, and null-conclusion at
`public-result-publication`.

See the [public-safe result](../../../../docs/simulations/profile-diff-held-out/pulse-42-public-result/README.md).
The claimed `P42-FROZEN-BINARY-UNAVAILABLE` remains only a
`reported_unvalidated` blocker, not a root cause. Pulse 31, Pulse 35/Pulse
37, private materialization, and search are indeterminate; no product,
diagnostic, category, or fix authority follows.
