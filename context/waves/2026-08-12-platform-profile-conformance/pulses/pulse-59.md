# Pulse 59: witness-preserving capability/materialization executor release

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Bind exact Pulse 58 ordered capability/materialization execution while
preserving the exact witness-preserving terminal closeouts first defined by
Pulse 53.

## Released control

- one standard-library sealed wrapper callable that binds exact Pulse 58 commit
  `7c66d70800edd06642274ed4f2e4aee224b7583e`, verifies its manifest/receipt/
  seal/source/callable identities, and invokes exact Pulse 58 production or
  fake qualification orchestration rather than reimplementing ordering;
- exact Pulse 52 stage helpers plus exact Pulse 57/Pulse 51/Pulse 43/Pulse 47
  terminal dependencies loaded only through Pulse 58's sealed stack;
- a fixed fresh sibling terminal custody root derived from
  `private_runtime_root`, rejected when preexisting, created only after exact
  Pulse 58 returns `completed`, and never caller-injected; and
- Pulse 53's exact completed terminal classes
  (`published-result`, `published-failure-witness`,
  `invalid-witness-publication`) with path-free transfer descriptors, bounded
  verified cleanup, fake-only qualification, role review, and Rust validator.

## Fixed non-goals

Pulse 59 MUST NOT create or consume authority, execute a real FERRIS
diagnostic, claim P44/P45 execution, accept production seed/fake-capability/
callback/publication-root/trust injection, retry terminal publication, expose
terminal paths or private material publicly, or modify frozen predecessor
releases.

## Completion record

The sole production callable mirrors Pulse 58's six concrete inputs:
`repo_root`, `private_runtime_root`, `p27_cycle_root`, `p39_checkout_root`,
`p41_final_root`, and `ubuntu_runtime_parent`. Pulse 59 delegates exact Pulse
58 ordering and receives the exact Pulse 58 ordered event list with no added
post-completion execution event. Only after Pulse 58 has completed and removed
its private runtime root does Pulse 59 derive one fresh sibling terminal
custody root and invoke the exact one-use Pulse 51/Pulse 47 terminal route.

The completed closeouts are unchanged from Pulse 53: verified P43 result plus
P47 witness as `published-result`; verified P47 witness only for exact bounded
P43 `absent` / `rolled-back` / `indeterminate` failure as
`published-failure-witness`; all malformed, mismatched, unverifiable, or
residue-bearing output as `invalid-witness-publication` with exact bounded
verified cleanup and the non-returning public-safe
`terminal-publication-cleanup-indeterminate` fatal posture. Any pre-execution
or precompletion Pulse 58 failure remains publication `not-attempted`.

Qualification is fake-only: 20 cycles, 2,760 harmless launches, alternating
ten result and ten failure-witness closeouts, all three bounded P43 failure
postures, 14 behavioral controls, exact Pulse 58 fake orchestration, and zero
real FERRIS execution or authority.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-59-WITNESS-PRESERVING-CAPABILITY-MATERIALIZATION-EXECUTOR-RELEASE-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_59_witness_preserving_capability_materialization_executor_release.rs)
