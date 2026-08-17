# Pulse 80: witness-preserving capability/materialization stage-capture-bootstrap-argv successor

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Preserve exact Pulse 77 / Pulse 59 terminal publication semantics while
rebasing the witness layer over the explicit Pulse 79 / Pulse 78
stage-capture-bootstrap-argv-hardened chain.

## Released control

The production callable remains injection-free and accepts only `repo_root`,
`private_runtime_root`, `p27_cycle_root`, `p39_checkout_root`,
`p41_final_root`, and `ubuntu_runtime_parent`. It byte-binds exact Pulse 79,
Pulse 52, Pulse 78, Pulse 51, Pulse 43, and Pulse 47 modules from verified
sibling bytes on every call. Fresh binder loads are serialized by the
hardened cross-instance kernel-lock model and never rely on ambient import
state.

Pulse 80 preserves exact Pulse 77 / Pulse 59 terminal behavior: publication
remains `not-attempted` until Pulse 79 completes, one fresh sibling terminal
root is derived per cycle, only verified result+witness or verified
failure-witness outputs are published, invalid witness output triggers bounded
cleanup, and transfer descriptors remain path-free. It introduces no authority
and performs no real FERRIS execution.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-80-witness-preserving-capability-materialization-executor-stage-capture-bootstrap-argv-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-80-witness-preserving-capability-materialization-executor-stage-capture-bootstrap-argv-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-80-WITNESS-PRESERVING-CAPABILITY-MATERIALIZATION-EXECUTOR-STAGE-CAPTURE-BOOTSTRAP-ARGV-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_80_witness_preserving_capability_materialization_executor_stage_capture_bootstrap_argv_successor_release.rs)

Qualification is fake-only: 39 receipt-listed behavioral controls, 20 cycles,
2,760 harmless launches, ten published results, ten published failure
witnesses, one terminal invocation per cycle, and no real FERRIS execution.
