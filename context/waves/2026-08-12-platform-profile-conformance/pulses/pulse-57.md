# Pulse 57: capability-bound diagnostic executor successor

Status: Complete sealed infrastructure release; no diagnostic authority

## Goal

Replace Pulse 51's caller-summary custody route with exact live Pulse 56
capabilities while retaining the complete exact Pulse 51 diagnostic semantics.
Pulse 57 does not authorize, invoke in qualification, resume, or infer any
FERRIS diagnostic result.

## Released control

The production callable is injection-free and does not accept custody roots,
summaries, public receipts, binaries, executable paths, callbacks, process
runners, environment mappings, or arbitrary controls. It verifies complete
exact Pulse 51 and Pulse 56 sealed releases, retains the P27/P31/P35/P37 and
profile-diff controls, and has the fixed 70/69/1 and 140/138/2 accounting.

It creates one Windows and one native-WSL Ubuntu Pulse 56 capability. Windows
uses only `launch_verified`; Ubuntu owns its P56 handle in a fixed
`Ubuntu-24.04` worker session for exactly 69 canonical requests. The catalog
contains only sealed-predecessor binding, Windows/Ubuntu capability
build/custody, exact adapter preflight, P31, P35/P37, descriptor validation,
and bounded process search. It does not contain
`pulse-41-pulse-39-public-custody`: Pulse 39/Pulse 41 execution is neither
invoked nor claimed. A later ordered layer must add and execute P39/P41 before
private materialization; it cannot infer either control from this release.

The worker, its sealed dependency loader, and complete P56 tree are copied
from verified byte buffers into a fresh native WSL directory. A fixed isolated
Python bootstrap opens, hashes, and compiles the worker from a held native
descriptor, so neither a source loader nor a post-verification worker-path
reopen can substitute bytes. The worker compiles staged dependencies from
their verified buffers with bytecode disabled only after complete P56-tree
verification. Every descriptor's semantic contract and input identities freeze
before launch and are rechecked through P51's dispatch mapping immediately
before each target launch. Cleanup completes before the sole terminal event;
cleanup failure after all 138 launches is failed `P57-INDETERMINATE-CLEANUP`.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-57-CAPABILITY-BOUND-DIAGNOSTIC-EXECUTOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_57_capability_bound_diagnostic_executor_release.rs)

Qualification executes 22 negative-control tests plus 20 fake-only cycles
and records their matching counts in the receipt.
