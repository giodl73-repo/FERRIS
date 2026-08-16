# Pulse 73 ordered stage-identity successor root cause report

## Defect fixed

Pulse 70 preserved exact ordered P58 semantics over Pulse 69, but it still reached its sealed predecessor chain through an ambient `from sealed_dependencies import ...` resolution path. That left the ordered layer weaker than Pulse 72's explicit sibling-path binding: the capability executor successor could be replaced in ambient import state even though its sealed release bytes were otherwise fixed.

## Successor approach

Pulse 73 keeps frozen Pulse 70 intact and reuses its exact ordered semantics while rebinding the capability layer explicitly:

- the top-level ordered executor loads only its sibling `sealed_dependencies.py` by bounded no-follow path, SHA-256 verification, and fresh module compilation;
- that binder verifies exact P35, P39, P41, P52, and Pulse 72 release identities before exposing callables;
- the ordered executor then runs the same Pulse 70/P58 sequence, but the capability executor it calls is exact Pulse 72 rather than ambiently imported Pulse 69; and
- private materialization, topology accounting, directory identity checks, cleanup precedence, and privacy-safe event behavior remain exact Pulse 70/P58 behavior.

## Qualification boundaries

Qualification is fake-only. It proves the local loader ignores ambient `sealed_dependencies`, proves fresh module loading, proves exact Pulse 72 binding, and re-runs the full Pulse 70/P58 behavioral suite over 20 harmless cycles. It executes no authority, no publication, and no real FERRIS diagnostic.
