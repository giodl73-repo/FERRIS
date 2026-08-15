# P52-RC-001: pre-gate private materialization

Pulse 51 correctly sealed public P27, P31, P33, P35/P37, P43, P44, P45, and
P47 execution controls, but its production runner required a completed
descriptor root as an input.  A future caller would therefore have to create
a private 32-byte seed and invoke P35 before the Pulse 51 public gate order
began.  That ordering could not prove that public preflight failure consumed
no private launch.

Pulse 52 is a narrow staging wrapper, not a modification of Pulse 51.  It
seals the exact Pulse 51, Pulse 39, and Pulse 41 trees, manifests, receipts,
seals, sources, and callable signatures.  It no longer accepts a caller-made
P43-shaped prelaunch event.  Gate 1 invokes exact P39 once on a supplied
fresh checkout root, invokes exact P41 once to copy that checkout's sealed P39
root to a supplied fresh final root, validates the complete P39/P41 summaries,
and independently rechecks the copied P41 tree before constructing its event.
Thus a copied fixture dictionary has no authority without matching roots.

After that custody sequence, Pulse 52 runs gates 2–6 once, proves the private
namespace absent, then creates one `O_EXCL`/`fsync` seed file and calls exact
P35 materialization and verification once.  Only after both exact P35 checks
and cleanup policy pass does it construct Pulse 51's fixed dispatches.

P35's complete verified descriptor manifest is larger than Pulse 51's
synthetic four-MiB reader and uses its own sealed aggregate framing.  Pulse 52
therefore has a bounded explicit staged reader: P35 verifies the full
manifest/coverage/seed first; Pulse 52 then reuses Pulse 51 role confinement,
input validation, descriptor types, direct Windows/WSL dispatch, semantic and
identity validation.  This avoids copying or changing Pulse 51 and does not
weaken the P35 verifier.

The result remains infrastructure only.  No authority, diagnostic conclusion,
private material, candidate, product behavior, or PLATFORM-001 claim follows.

Terminal publication is a separate closeout from private execution.  A
completed private dispatch becomes `published` only when the one-use P47 seam
returns its complete published witness for a complete P43 result and both
final roots verify.  A bounded P43/P47 failure posture instead closes
`invalid-publication-integrity` with null product/category/fix conclusions.
Custody converts only exact P39/P41 public failures and filesystem errors;
terminal verification converts only exact P43/P47 public failures and
filesystem errors.  Programmer faults, including `TypeError` and
`AssertionError`, are never rewritten into a successful, bounded, or
publication-shaped result.
There is no added event, retry, or republish: Pulse 52 removes and verifies
absence of the terminal parent and its P43/P47 stage residue.  Cleanup retries
are bounded and never invoke P43/P47 again.  If cleanup or absence verification
remains indeterminate, the callable raises the public-safe unresolved
`terminal-publication-cleanup-indeterminate` fatal posture instead of returning
any completed closeout.
