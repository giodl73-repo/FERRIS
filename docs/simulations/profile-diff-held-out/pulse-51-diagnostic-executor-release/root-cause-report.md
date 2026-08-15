# P51-RC-001: executor boundary gaps

The initial staged public executor was not releasable. Its default launcher
discarded the platform and therefore did not run Ubuntu through a declared WSL
distribution or prove Windows-to-WSL translation. Its synthetic launcher also
discarded the executable/path boundary. It accepted caller-created P45 gate
events and a caller-selected synthetic versus custodian grant, so runtime
infrastructure could be made to self-authenticate a bypass. It did not bind
the complete P44/P45 bridge identity or custody roots.

The same version treated P27 failures inconsistently, left partial P27 roots
without a stated cleanup/retention policy, did not bound several filesystem,
Git, sealed-import, P31, WSL, and P47 precondition failures, and allowed a
terminal helper to reuse or overlap roots. Its JSON checker accepted a partial
envelope and its human checker accepted any nonempty line; it compared raw
normalized output rather than the public path-free process-exit semantic
projection.

Pulse 51 now has fixed native Windows and exact `Ubuntu-24.04` WSL dispatch,
private-root confinement, sealed P44-to-P45 bridging, one-call P27 handling,
full frozen output validation, independently recomputed profile semantics and
all four output identities, semantic projection comparison, and a one-use
Pulse 47 terminal object. Synthetic-only qualification uses the same dispatch
constructor and full fake outputs while putting all runtime scratch below
`target/` and leaving no ignored release-tree residue. These controls make no
claim that an authority exists, has been verified, or has been exercised. No
private descriptor, seed, candidate, diagnostic, result, or witness was
created.

The documented-order audit found no retained Pulse 51 child process after the
standalone unit suite, and every `os.open` path has deterministic close
handling. All runtime and synthetic `scandir` iterators are scoped context
managers; completed subprocesses use `run`, while the one concurrent
synthetic lock holder is context-scoped and closed. The remaining failure mode
is therefore treated as a transient Windows scanner/sharing race only for
synthetic/test roots: four fixed delays (0.02, 0.05, 0.10, and 0.20 seconds)
retry `PermissionError`/`WinError 32`, require absence, and otherwise fail
explicitly. Production P27 cleanup remains unchanged and fail-closed.
