# Root cause: Pulse 58 orders execution but intentionally does not publish

Pulse 53 preserved a valid Pulse 47 witness of bounded Pulse 43 publication
failure, but it bound exact Pulse 52 ordering and retained-binary custody
rather than the later exact Pulse 58 live-capability executor. Pulse 58 closed
the public-before-private ordering gap for live Pulse 56 capabilities, yet it
returns only privacy-safe ordered events and private execution accounting. It
does not own terminal publication or witness retention.

Pulse 59 closes only that terminal gap. It delegates exact Pulse 58 production
and fake qualification orchestration unchanged, waits until Pulse 58 has
completed and removed its private runtime root, then derives one fresh sibling
terminal custody location with no caller injection. Exact Pulse 51/Pulse 47
publication semantics run once over that sibling root, preserving Pulse 53's
three completed terminal classes without adding a new execution event. The
wrapper now loads its sibling binder by verified file path, freshly SHA-256
verifies its bytes on every call, execs those bytes into a new private module
object, and immediately invokes that fresh binder with no reusable private
module key or Python registry. The binder reinstantiates exact verified Pulse
58/P52/P57/P51/P43/P47 modules on every call. Because the exact predecessor
stack still relies on bare `sealed_dependencies` imports, the binder
serializes the entire exact-load sequence with a cross-instance OS-backed lock
keyed by the resolved sibling binder path, rejects symlink and Windows reparse
ancestors across the lexical repo `target` lock chain before and immediately
after open, uses exclusive safe creation for the lock file, closes descriptors
on acquisition failure, and restores any generic module slot only if the exact
installed module remains in place. Neither ambient import resolution, stale
mutable module objects, forged old private keys, forged registry artifacts,
nor concurrent slot interleaving can steer execution before a call begins.
Arbitrary mutation of live Python objects during an active call remains
outside process integrity and is explicitly not claimed.

This is infrastructure only. It creates no authority, performs no real FERRIS
diagnostic, and does not alter any historical pulse disposition.
