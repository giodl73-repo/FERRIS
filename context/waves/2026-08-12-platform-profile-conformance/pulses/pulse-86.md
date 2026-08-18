# Pulse 86: WSL parent-owner binding capability executor successor

Status: Complete sealed infrastructure release; no diagnostic authority or real
FERRIS execution

## Goal

Remove the prospective Ubuntu custody blocker exposed by Pulse 85 without
retrying Pulse 84, suppressing WSL stderr, or changing frozen Pulse 78.

## Released control

The production callable remains
`run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 86 byte-binds exact Pulse 78 and its exact Pulse 75 stack. Before staging,
one explicit-root read-only WSL bootstrap validates the native runtime parent,
reads its no-follow owner UID, and resolves the corresponding distribution
username. Staging, revalidation, worker, and cleanup spawns then include
`--user <resolved-owner>` before `--exec`.

The owner is filesystem-derived, not caller supplied. Nonzero owner lookup,
any owner-lookup stderr, malformed protocol, invalid username, or missing
account mapping fails closed as `P86-WSL-OWNER`. Pulse 78's worker stderr,
identity, exact-tree cleanup, and protocol failures remain fatal.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-86-wsl-parent-owner-binding-capability-executor-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-86-wsl-parent-owner-binding-capability-executor-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-86-WSL-PARENT-OWNER-BINDING-CAPABILITY-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_86_wsl_parent_owner_binding_capability_executor_successor_release.rs)

Qualification includes one harmless real WSL parent-owner lookup, 25 controls,
20 fake-only capability cycles, 2,760 harmless fake launches, zero candidates,
and zero real FERRIS execution.
