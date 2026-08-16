# Root cause: Pulse 58 ordering still bound the unfixed staged-bundle layer

Pulse 52 correctly placed P39/P41 custody and P35 materialization after public
gates, but dispatch remained bound to its older caller-summary custody route.
Pulse 57 correctly replaced that route with live P56 capabilities, but its
top-level callable accepts an already-materialized descriptor root and so
cannot prove that P39/P41 and all public gates preceded seed creation.

Pulse 70 owns the same complete sequence but binds exact Pulse 69 instead of
the unfixed Pulse 57 layer. It imports exact Pulse 69 internals from verified
bytes rather than rebuilding their normalization, launch, or semantic logic,
and it uses P52's exact P35-to-P51 reader rather than making a second
descriptor interpretation. The only joined boundary remains P35's declared
regular/directory/missing artifact state: Pulse 70 freezes that state locally
with lexical no-follow identities and repeated file-ID checks, without
replacing a Pulse 69 module global. Exact P57-compatible output normalization,
launch, and first-stop behavior remain delegated helpers.

The P39 checkout root remains a future-authority precondition. Pulse 70 invokes
only exact P39 path/attribute/LF semantics and validates the exact P41
transactional copy of the verified root's P39 release; it proves neither
checkout freshness nor anonymity, exact HEAD, clean working tree, or
`core.autocrlf` posture.

This closes an infrastructure ordering gap only. It creates no authority and
runs no real FERRIS process in qualification.
