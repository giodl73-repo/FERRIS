# Pulse 58: ordered capability/materialization executor successor

Status: Complete sealed infrastructure release; no authority or diagnostic execution

## Goal

Join Pulse 52's public-before-private ordering with Pulse 57's exact live
Pulse 56 capability boundary without modifying any committed predecessor. The
release closes only orchestration infrastructure; it does not consume, revive,
or create authority.

## Released control

The only production callable accepts exactly a repository root, a fresh private
runtime root, a fresh P27 cycle root, a caller-supplied P39 checkout root, a
fresh P41 final root, and a native Ubuntu runtime parent. It accepts no
descriptor root, seed, capability, custody root/receipt, process runner,
callback, environment, or arbitrary controls. A future authority, not P58,
must prepare the P39 root as a fresh anonymous exact-cutoff checkout with the
required HEAD, clean-tree, and `core.autocrlf` posture.

It byte-binds complete exact P35/P39/P41/P52/P56/P57 releases and APIs from
held buffers. Pulse 57 supplies its own exact P51/P56 byte-bound stack. P58
runs exact P39 path/attribute/LF semantics only, then exact P41 transactional
copy and full final custody validation. It does not claim P39 root freshness,
anonymity, HEAD, cleanliness, or `core.autocrlf` validation. It completes every
public-only gate before private material: sealed predecessor binding;
Windows/Ubuntu capability build/custody; P27; P31; and P35/P37. The remaining
ordered gates are bounded materialization, descriptor validation, and bounded
process search. The catalog does not invoke or claim P44/P45 or any
publication path.

Only after every public gate passes, P58 calls `secrets.token_bytes(32)` once,
writes one bounded `O_EXCL`/`fsync` seed, materializes/verifies exact P35
70-descriptor material once, and delegates frozen input checks, dispatch,
normalization, semantic comparison, and first-stop to exact P57 helpers over
the already-live capabilities. The P52 exact reader supplies the P35-to-P51
projection. P35's declared directory state is checked in P58 through lexical
no-follow directory identity and repeated file-ID rechecks; P57 module globals
remain unchanged.

The fixed topology is 70 descriptors, 69 launches plus ordinal-70 no-launch
per platform, and 138 launches total. All normal/failure paths close the
Windows capability and Ubuntu worker/capability, remove seed/descriptors/P27
and runtime private roots, and verify their absence. Uncertain cleanup is fatal
`P58-INDETERMINATE-CLEANUP`; unknown programmer faults reraise only after
successful cleanup. The result has private execution accounting and public
ordered events only; it has no P43/P47/publication operation.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-58-ORDERED-CAPABILITY-MATERIALIZATION-EXECUTOR-RELEASE-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_58_ordered_capability_materialization_executor_release.rs)

Qualification executes 20 receipt-listed behavioral controls and 20 fake-only
cycles, with 2,760 fake launches, exact P39 semantics/P41 copy/P27/
materializer/verifier once per cycle, zero P44/P45 or publication calls, and
no real FERRIS execution.
