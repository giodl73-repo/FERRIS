# Pulse 70: ordered capability/materialization executor successor

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Preserve Pulse 58's exact public-before-private ordering while rebinding its
live execution layer from the unfixed Pulse 57 route to the cleanup-owning
Pulse 69 successor.

## Released control

The production callable remains injection-free and accepts only `repo_root`,
`private_runtime_root`, `p27_cycle_root`, `p39_checkout_root`,
`p41_final_root`, and `ubuntu_runtime_parent`. It byte-binds exact Pulse 35,
Pulse 39, Pulse 41, Pulse 52, and Pulse 69 releases from verified bytes. Pulse
69 in turn binds the exact Pulse 57/Pulse 51/Pulse 56 live-capability stack.

Pulse 70 preserves the exact Pulse 58 gate order: P39/P41 custody and all
public gates complete before one private 32-byte seed, one exact P35
materialization, one exact P35 verification, descriptor freezing, and bounded
process search. It keeps Pulse 58's lexical no-follow directory identity and
first-stop accounting while delegating the fixed Pulse 69 worker/capability
close and owned native staged-bundle cleanup semantics. It does not create
publication, witness, or authority behavior.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-70-ordered-capability-materialization-executor-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-70-ordered-capability-materialization-executor-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-70-ORDERED-CAPABILITY-MATERIALIZATION-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_70_ordered_capability_materialization_executor_successor_release.rs)

Qualification is fake-only: 20 receipt-listed behavioral controls, 20 fake
cycles, 2,760 harmless launches, one exact P39 semantics/P41 copy sequence and
one seed/materialization/verifier sequence per cycle, zero publication calls,
and no real FERRIS execution.
