# Pulse 87: ordered WSL parent-owner binding successor

Status: Complete sealed infrastructure release; no authority, publication,
witness, or real FERRIS execution

## Value control

- **Ferris user outcome:** a maintainer can run the existing ordered capability
  executor without regressing Ubuntu work to the wrong WSL account or losing
  the exact indeterminate-stage cleanup disposition before private seed
  creation.
- **Maximum effort:** one ordered successor over exact merged Pulse 86, one
  implementation attempt, one fake-only qualification, and one closeout
  review. No witness layer or further successor is included.
- **Completion test:** bind the complete exact Pulse 86 release and merged
  implementation commit, preserve Pulse 81 ordering and Pulse 35 release-tree
  behavior, preserve `P86-INDETERMINATE-STAGE-CLEANUP` at the ordered terminal,
  and pass the sealed 29-control, 20-cycle, 2,760-launch qualification plus the
  static Rust validator.
- **Abandonment condition:** stop `stop-value-exhausted` if the replacement
  requires changed ordered semantics, another custody or authority layer, real
  FERRIS execution, or a second corrective successor.

## Goal

Preserve exact Pulse 70 / Pulse 58 public-before-private ordering while
rebinding the live capability layer from Pulse 78 to exact merged Pulse 86.

## Released control

The production callable remains
`run_ordered_capability_materialization_executor(repo_root, private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root, ubuntu_runtime_parent)`.

Pulse 87 retains Pulse 81's exact Pulse 35 manifest, receipt, seal, file-set,
size, hash, and sole source-digest binding. It also retains exact Pulse 39,
Pulse 41, and Pulse 52 ordering plus the final cross-thread and cross-process
sealed-load discipline.

The only live capability change is the explicit exact Pulse 86 edge. Public
custody completes before one private seed, one materialization, one
verification, descriptor freezing, and the bounded capability sequence.
Pulse 86's filesystem-derived WSL `--user` binding remains exact, and
`P86-INDETERMINATE-STAGE-CLEANUP` remains visible at the ordered terminal
boundary before seed creation.

No publication, witness, retry, authority, score, support claim, or
PLATFORM-001 change is introduced.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-87-ordered-capability-materialization-executor-wsl-parent-owner-binding-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-87-ordered-capability-materialization-executor-wsl-parent-owner-binding-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-87-ORDERED-CAPABILITY-MATERIALIZATION-EXECUTOR-WSL-PARENT-OWNER-BINDING-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_87_ordered_capability_materialization_executor_wsl_parent_owner_binding_successor_release.rs)

Qualification is fake-only: 29 deterministic controls, 20 cycles, 2,760 fake
launches, one exact public custody and seed/materialization/verifier sequence
per cycle, zero publication, and no real FERRIS execution.
