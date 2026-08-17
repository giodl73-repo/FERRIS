# Pulse 78: stage-capture/bootstrap-argv capability executor successor

Status: complete sealed infrastructure release; no authority or real diagnostic
execution

## Goal

Close the remaining native WSL mkdir→open ownership-capture and worker
bootstrap argv/dependency-loader binding gaps in frozen Pulse 75 without
changing any frozen predecessor.

## Released control

The production callable remains
`run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 78 byte-binds exact Pulse 75 through a verified sibling sealed loader,
captures bundle ownership only through the verified parent descriptor,
classifies any pre-capture reopen mismatch or failure as
`P78-INDETERMINATE-STAGE-CLEANUP`, keeps post-capture staging fd-relative or
identity-revalidated, and closes the bootstrap argv gap by passing expected
parent/root identity plus worker/dependency-loader path/hash bindings into the
exact WSL bootstrap process. That process consumes the dependency-loader
binding internally, re-validates parent/root identity, opens and hashes both
dependency loader and worker no-follow, executes only from verified bytes, and
forwards only the exact named worker flags into the real worker `argparse`.

Cleanup still removes only the original owned tree. Replacement deletion is
forbidden; pre-capture ownership uncertainty is fatal
`P78-INDETERMINATE-STAGE-CLEANUP`, and post-capture cleanup uncertainty
remains fatal `P57-INDETERMINATE-CLEANUP`.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-78-STAGE-CAPTURE-BOOTSTRAP-ARGV-CAPABILITY-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_78_capability_bound_diagnostic_executor_stage_capture_bootstrap_argv_successor_release.rs)

Qualification is fake-only: 18 deterministic control tests, 20 cycles, 2,760
harmless launches, one staged-identity revalidation and one owned-bundle
cleanup per cycle, explicit create/open substitution and bootstrap ready/close
coverage, and no real FERRIS execution.
