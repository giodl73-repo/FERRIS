# Pulse 71: witness-preserving capability/materialization executor successor

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Preserve Pulse 59's exact terminal publication semantics while rebinding its
live execution layer from exact Pulse 58 to the fixed Pulse 70 ordered
successor.

## Released control

The production callable remains injection-free and accepts only `repo_root`,
`private_runtime_root`, `p27_cycle_root`, `p39_checkout_root`,
`p41_final_root`, and `ubuntu_runtime_parent`. It byte-binds exact Pulse 70,
Pulse 52, Pulse 69, Pulse 51, Pulse 43, and Pulse 47 modules from verified
bytes on every call. Fresh binder loads are serialized by a cross-instance
kernel lock keyed by the verified binder path and digest; Windows uses a named
mutex, and supported Linux/Ubuntu targets use a deterministic abstract
AF_UNIX socket with at-fork cleanup managed by the stable executor module.

Pulse 71 preserves Pulse 59's exact terminal semantics: publication remains
`not-attempted` until Pulse 70 completes and removes its private runtime root,
terminalization runs once over a fresh sibling root, verified result/witness
and failure-witness classes are retained, invalid terminal output triggers
bounded cleanup, and cleanup uncertainty remains fatal with precedence. It
does not create authority or execute real FERRIS.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-71-witness-preserving-capability-materialization-executor-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-71-witness-preserving-capability-materialization-executor-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-71-WITNESS-PRESERVING-CAPABILITY-MATERIALIZATION-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_71_witness_preserving_capability_materialization_executor_successor_release.rs)

Qualification is fake-only: 39 receipt-listed behavioral controls, 20 cycles,
2,760 harmless launches, 10 published results, 10 published failure witnesses,
one terminal invocation per cycle, retained `70/69/1` topology, and zero real
FERRIS execution.
