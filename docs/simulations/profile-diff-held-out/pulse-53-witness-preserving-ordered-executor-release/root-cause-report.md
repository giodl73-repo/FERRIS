# P53-RC-001: witnessed P43 failure was discarded

Pulse 52 correctly required exact P43 result and P47 witness success before it
classified terminal output as `published`.  Its fallback treated every other
terminal summary as `invalid-publication-integrity` and removed the terminal
parent.  That rule also removed a valid P47 two-file witness whose purpose is
to retain the exact bounded failure posture of a Pulse 43 publication failure.

Pulse 53 does not alter Pulse 52 ordering, P39/P41 custody, public gates,
private seed/materialization, dispatch, or the one-use P47 route.  It imports
and verifies exact Pulse 52, then copies only the bounded orchestration and
terminal classification necessary to distinguish three outcomes.  A P43
`absent`, `rolled-back`, or `indeterminate` failure is valid only when the
P47 summary and actual two-file witness root independently verify, the P43
root and stage are absent, and the terminal parent contains only that witness.
That state is retained as `published-failure-witness`; it has null conclusions
and a path-free hash-only transfer descriptor.

Anything else remains invalid: witness failure, malformed summary, hash
mismatch, unsafe/missing root, unexpected terminal shape, or retained P43
result residue.  Pulse 53 invokes P47 once, never retries or republishes, and
uses exact Pulse 52 bounded verified cleanup only for that failed terminal
residue.  Filesystem and exact P43/P47 failure boundaries remain bounded;
`TypeError` and `AssertionError` propagate.  The change is synthetic
infrastructure only and establishes no diagnostic or authority outcome.
