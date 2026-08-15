# Root cause: caller-held custody was not a capability boundary

Pulse 51 correctly froze descriptor, profile-diff, and bounded dispatch
semantics, but its P44/P45 route accepted caller-held final roots and summaries
at the executor boundary. A valid public receipt or root is audit evidence,
not proof that the executing process retains the byte-verified object.

Pulse 57 removes that route. Exact Pulse 56 is verified as a complete sealed
release and is the only component that creates a live object-identity
capability. Windows keeps that object in the executor process. Ubuntu keeps
its distinct object in the native-WSL worker process for all 69 requests.

This is an infrastructure correction only. It creates no diagnostic authority,
does not revive a withdrawn pulse, and has no product, category, score,
support, certification, or PLATFORM-001 conclusion.
