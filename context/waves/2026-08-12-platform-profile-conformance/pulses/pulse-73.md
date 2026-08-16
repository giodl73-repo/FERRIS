# Pulse 73: ordered capability/materialization stage-identity successor

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Preserve exact Pulse 70 / Pulse 58 ordered execution while rebinding the live
capability layer from ambient predecessor resolution to the explicit Pulse 72
stage-identity successor.

## Released control

The production callable remains injection-free and accepts only `repo_root`,
`private_runtime_root`, `p27_cycle_root`, `p39_checkout_root`,
`p41_final_root`, and `ubuntu_runtime_parent`. It byte-binds exact Pulse 35,
Pulse 39, Pulse 41, Pulse 52, and Pulse 72 releases through a verified
sibling-path sealed loader rather than ambient `sealed_dependencies`
resolution.

Pulse 73 preserves exact Pulse 70 / Pulse 58 ordering: one exact Pulse 39
checkout verification and Pulse 41 transactional copy complete before one
private 32-byte seed, one bounded Pulse 35 materialization, one verification,
descriptor freezing, bounded process search, and one exact Pulse 72 capability
sequence. Topology, directory identity checks, privacy, failure precedence,
and final cleanup remain exact Pulse 70 / Pulse 58 behavior. No publication,
witness, or authority behavior is introduced.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-73-ordered-capability-materialization-executor-stage-identity-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-73-ordered-capability-materialization-executor-stage-identity-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-73-ORDERED-CAPABILITY-MATERIALIZATION-EXECUTOR-STAGE-IDENTITY-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_73_ordered_capability_materialization_executor_stage_identity_successor_release.rs)

Qualification is fake-only: 22 receipt-listed behavioral controls, 20 harmless
cycles, 2,760 total fake launches, one exact public custody and one
seed/materialization/verifier sequence per cycle, zero publication, and no
real FERRIS execution.
