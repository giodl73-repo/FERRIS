# Pulse 52: ordered-materialization executor release

Status: complete synthetic infrastructure release only.

Pulse 52 closes the final **sequencing** gap in Pulse 51.  Pulse 51 accepted
an already materialized descriptor root, which required a future custodian to
perform the private seed/materialization step before Pulse 51 could establish
its ordered public gates.  This release places that single private launch
strictly after gates 1 through 6.

It grants **no authority**.  It does not execute a real FERRIS binary,
diagnostic, candidate, score, certification, product change, or
PLATFORM-001 conclusion.  A future authority must separately bind the exact
sealed Pulse 51 and Pulse 52 releases and every named predecessor.

## Production surface

`ordered_materialization_executor.run_ordered_materialization_executor` is
the only production callable.  It accepts:

1. an absolute, fresh declared runtime container;
2. a fresh absent P27 cycle-root location;
3. an absolute fresh P39 checkout root containing the fixed Pulse 25, Pulse
   27, and Pulse 39 release roots;
4. a fresh absent P41 final-copy root outside the runtime container; and
5. retained P44 Windows/Ubuntu custody roots and summaries.

It accepts no prelaunch event or summary, descriptor root, seed, seed
generator, materializer, verifier, launcher, process runner, fake binary,
expectations, callback, grant, or trust-mode flag.  The underscore-prefixed
qualification helper is fake-only test infrastructure and is not a runtime
interface.

The declared runtime container must be absolute, regular through every
ancestor, symlink-free, and initially contain only the declared public P44
custody trees.  The P27 root is a direct absent child.  Pulse 52 rejects an
unexpected entry, preexisting private namespace, preexisting terminal
namespace, alias, escape, or symlink before a private launch begins.

## Fixed order

1. Verify the complete exact Pulse 51, Pulse 39, and Pulse 41 release trees,
   source identities, manifests, receipts, seals, and imported callable
   signatures.  Invoke exact P39 `verify` once against the supplied checkout
   and fixed Pulse 25/Pulse 27 roots; validate its complete `36/36` summary.
   Invoke exact P41 `copy_release` once from that checkout's exact P39 release
   root to the fresh supplied P41 final root; validate its complete `8/8`,
   one-rename, zero-retry, sync, and rollback summary, then independently
   reverify the actual P41 final tree.  Only then construct gate 1 internally.
2. Execute gates 1 through 6 exactly once: public custody, both P45 bridges,
   P27 with a fresh absent cycle root, P31 `39/39`, and P35/P37 custody.
   The private launch namespace is proved absent before every gate and again
   after gate 6.
3. Begin the sole private launch only after gate 6: production calls
   `secrets.token_bytes(32)`, creates the seed file through
   `O_CREAT|O_EXCL` with file `fsync`, invokes exact P35
   `materialize` once, invokes exact P35 `verify` once, and records only the
   domain-separated commitment in the private record.
4. Remove the seed immediately after successful P35 verification.  Retain
   only the descriptor tree until the fixed dispatch phase completes, then
   recursively remove and verify absence of the whole private launch
   namespace.  A materializer or verifier failure still consumes the private
   launch and terminalizes gate 7; cleanup is bounded and verified.
5. Mark `bounded-materialization` passed only after exact materialization,
   verification, P35 `70`/`18/18`/`8/8`, and staged P51
   `70/69/1` descriptor confinement checks have passed.
6. Execute gate 8 through Pulse 51's sealed direct Windows and exact
   `Ubuntu-24.04` WSL dispatch/semantic/identity routines.  It executes
   exactly 69 launch-ready dispatches per platform, one final no-launch
   descriptor per platform, and stops on the first cross-platform mismatch.
   No P27, P44/P45, P35 custody, materializer, or verifier is rerun.
7. Create fresh fixed P43/P47 output roots and invoke Pulse 51's one-use
   `TerminalPulse47Once` seam once.  The returned object contains only the
   exact P43 catalog/events, a closed terminal-publication disposition, and a
   caller-private in-memory record.  `published` requires the complete exact
   P43 result **and** P47 witness success shapes plus independent final-root
   verification.  A P43 failure summary, a P47 witness failure summary, a
   malformed terminal value, or a missing final shape is instead
   `invalid-publication-integrity`: private execution may remain
   `completed`, but product, category, and fix conclusions are null.
   Pulse 52 does not append an event, retry, or republish after this sole
   terminal call.  It bounded-retries only terminal cleanup.  If cleanup or
   absence verification remains unsuccessful after that bound,
   `TerminalPublicationCleanupIndeterminate` is raised instead of returning
   a completed closeout.  Its sole public field is the
   `terminal-publication-cleanup-indeterminate` cleanup-owner/posture object;
   it has no events, roots, private record, or publication-success substitute.

P39/P41 custody converts only their exact `PublicFailure` values and expected
filesystem failures to bounded prelaunch.  Terminal root verification converts
only exact P43 `PublicFailure`, P47 `WitnessFailure`, and filesystem failures
to invalid publication; the sealed terminal invocation similarly preserves
only documented P43/P47 failure posture.  `TypeError`, `AssertionError`, and
other programmer faults propagate.  Terminal cleanup retries and its
indeterminate conversion likewise admit only the bounded executor/filesystem
failure types.

Pulse 35's sealed private verifier remains the authority for its complete
coverage manifest and aggregate.  Pulse 52 stages its verified descriptors
through Pulse 51's fixed path, dispatch, semantic, and identity checks with
an explicit bounded eight-MiB manifest reader; it does not modify Pulse 51 or
replace either predecessor's sealed source.

Public events are P43-shaped only.  They contain no seed bytes, seed
commitment, seed or descriptor path, descriptor token, case identifier,
binary bytes, private record, or terminal-root path.  A public-gate failure
returns a P43 terminal event with `private_launch_started:false` in the
private record.  After the private boundary, detailed failure data remains
private and the public event stays a bounded gate aggregate.  The outer
terminal-publication disposition exposes only P43/P47's allowed bounded
publication/witness posture; it never uses completed execution as a
publication-success substitute.  A copied fixture event cannot satisfy gate
1 because no caller event is accepted and P39/P41 roots are reverified.

## Qualification

Run from this release directory:

```powershell
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20
```

The tests and receipt use only deterministic fake seed bytes and a harmless
in-process fake final process boundary.  A separate test-only production
wiring harness calls the exported injection-free callable, wraps the real
`secrets.token_bytes(32)` boundary, and mocks only sealed external operations;
it proves fixed Pulse 51 P33 expectations, the fixed production process
runner, exact P35 materializer/verifier selection, and one real sealed
P39/P41 custody sequence without running a FERRIS binary.  It proves early
fixed-binary and P39/P41 failures never call the CSPRNG, and mutates P39/P41
summary, root, receipt, path, file/hash, sync, count, and retry evidence.
P43 failure, P47 witness failure, same/nested terminal roots, exact
published-result/witness shape, no retry, bounded transient cleanup, and a
permanent cleanup lock that raises the unresolved fatal state are covered.
P39/P41, terminal-verifier, terminal-invocation, and terminal-cleanup
`TypeError`/`AssertionError` regressions prove that programmer faults
propagate, while exact predecessor and publication failures remain bounded.
Twenty cycles record one P39 verification, one P41 copy/reverification,
`70/69/1`, `138` fake dispatches each, `2,760` total, one materializer and
verifier invocation each, no raw seed or descriptor disclosure, and
successful terminal publication.  No FERRIS binary runs.

`public-manifest.json`, `qualification-receipt.json`, and `release-seal.json`
seal the release.  Removal deletes only this infrastructure, tests, and
governance record; it cannot alter Pulse 50's permanent withdrawal or create
authority.
