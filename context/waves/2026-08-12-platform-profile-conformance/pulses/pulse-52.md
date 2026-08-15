# Pulse 52: ordered-materialization executor release

Status: Complete public/synthetic infrastructure only

## Goal

Close the final Pulse 51 sequencing gap without reopening or consuming Pulse
50.  The release must make a future authorized caller prove every public gate
before a private P35 seed, descriptor root, or materializer access can exist.

## Authorized scope

- one standard-library Pulse 52 wrapper over exact sealed Pulse 51 and Pulse
  35 source;
- exact source/tree/signature binding, closed public return schema, fixtures,
  fake-only tests, receipt, seal, Rust validator, root-cause record, and
  nine-role review;
- one private launch only after gates 1–6: 32-byte production CSPRNG seed,
  `O_EXCL`/`fsync` seed creation, exact P35 materializer and verifier, bounded
  cleanup, then the existing fixed Pulse 51 Windows/WSL dispatch;
- one fresh Pulse 51 `TerminalPulse47Once` seam after complete gate 8; and
- twenty or more deterministic fake-only qualification cycles.

## Fixed non-goals

Pulse 52 MUST NOT:

- grant, authenticate, consume, revive, or repair Pulse 50 authority;
- execute a real FERRIS binary, candidate, diagnostic, score, search,
  certification, product change, or PLATFORM-001 conclusion;
- accept caller seed, descriptor, generator, materializer, verifier,
  launcher, expectation, fake-binary, callback, grant, trust-mode, prelaunch
  event, or prelaunch summary input;
- expose seed bytes, commitment, seed/descriptor paths, case/order/profile
  tokens, binary bytes, private records, or terminal paths in public events;
- modify frozen Pulse 27, 31, 35, 37, 43, 44, 45, 47, or 51 artifacts; or
- retry P27, P44/P45, materialization, verification, platform dispatch, or
  terminal publication.

## Completion record

The released callable accepts only a declared fresh runtime container, fresh
P27 cycle root, concrete fresh P39 checkout root, fresh absent P41 final-copy
root, and retained P44 custody inputs.  It loads exact Pulse 51, Pulse 39,
Pulse 41, and sealed dependencies; invokes P39 once and P41 once from those
roots; verifies their full summaries and P41 final tree; and only then
constructs the first P43 gate.  It proves the private namespace absent before
every public gate, then begins the single private launch.  It records the P35
seed commitment only in the private record, removes the seed after
verification, removes the descriptor namespace after dispatch, and emits only
P43-shaped events.

Pulse 35's exact verifier first verifies the 70-case corpus, `18/18` domains,
and `8/8` interactions.  Pulse 52 then uses an explicit bounded P35-to-Pulse
51 staging reader because P35's complete exact manifest is larger than Pulse
51's synthetic four-MiB descriptor reader.  The stage retains Pulse 51 path,
role, dispatch, semantic, identity, platform-agreement, 69-per-platform, and
final no-launch checks without changing either sealed predecessor.

Private execution completion is distinct from publication success.  Pulse 52
reports `published` only after the one terminal call returns the exact P47
published witness shape for an exact P43 published result and both final roots
independently verify.  A P43 failure summary, P47 witness failure summary,
malformed terminal return, or incomplete final shape closes
`invalid-publication-integrity`: private execution may be `completed`, but
product/category/fix conclusions are null.  It exposes only bounded P43/P47
publication posture, appends no event, makes no retry or second publication,
and removes/verifies absence of the terminal parent and every P43/P47 stage
residue.  A bounded transient cleanup retry may succeed; a permanent cleanup
or absence-verification failure raises the public-safe unresolved
`terminal-publication-cleanup-indeterminate` fatal state rather than returning
a completed closeout.

P39/P41 custody, published-root verification, terminal invocation, and final
cleanup distinguish exact public/filesystem failures from programmer faults:
only documented predecessor, P43/P47, executor, and filesystem failures are
bounded. `TypeError` and `AssertionError` propagate. Regression tests cover
both propagation and the retained bounded predecessor/publication posture.

Qualification passed 20 cycles: `70/69/1` per cycle, 138 fake dispatches per
cycle, 2,760 total, one P27 invocation, one materializer, one verifier, one
P39 checkout verification, one P41 transactional copy/reverification, one
P43/P47 terminal seam, no early-gate seed, no raw seed or descriptor
disclosure, verified cleanup, and exact terminal success.  P39/P41
summary/root/receipt/path/file/hash/sync/count/retry mutations, P43/P47
failure shapes, same/nested roots, permanent/transient terminal cleanup, and
an exported-production wiring harness are separately covered.  This evidence
is synthetic infrastructure only and establishes no authority.

## Evidence

- [Pulse 52 release](../../../../docs/simulations/profile-diff-held-out/pulse-52-ordered-materialization-executor-release/README.md)
- [Root-cause report](../../../../docs/simulations/profile-diff-held-out/pulse-52-ordered-materialization-executor-release/root-cause-report.md)
- [Public return schema](../../../../docs/simulations/profile-diff-held-out/pulse-52-ordered-materialization-executor-release/schemas/ferris.pulse-52-ordered-materialization-executor.v1.schema.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-52-ORDERED-MATERIALIZATION-EXECUTOR-RELEASE-ROLE-REVIEW.md)
- [Rust release validator](../../../../crates/ferris-cli/tests/pulse_52_ordered_materialization_executor_release.rs)
