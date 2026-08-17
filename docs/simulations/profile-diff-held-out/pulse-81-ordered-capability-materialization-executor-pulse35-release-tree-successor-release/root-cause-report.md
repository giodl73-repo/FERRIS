# Pulse 81 ordered pulse35-release-tree successor root cause report

## Defect fixed

Independent review found that the ordered successor chain still underbound
exact Pulse 35. Pulse 79 accepted historical alternate CRLF source digests and
did not fully bind the pinned Pulse 35 manifest, receipt, seal, exact file
set, and every file hash/size before exposing the materializer/verifier.

## Successor approach

Pulse 81 keeps frozen Pulse 70 intact and reuses its exact ordered semantics
while hardening the full sealed dependency path:

- the top-level ordered executor loads only its sibling `sealed_dependencies.py`
  by bounded no-follow path, SHA-256 verification, and fresh module
  compilation;
- that binder verifies exact Pulse 39, Pulse 41, Pulse 52, Pulse 35, and Pulse
  78 release identities before exposing callables;
- Pulse 35 verification now pins the raw manifest, recomputed aggregate,
  receipt raw/payload, seal raw/payload, exact release-tree file set, every
  file size/hash, and the single manifest digests for both source files with no
  alternate whitelist;
- every local and transitive sealed load across that graph is serialized with
  the final Pulse 74 / Pulse 59 kernel-lock discipline, with no ambient import
  substitution; and
- the ordered executor then runs the same Pulse 70 / Pulse 58 public-before-
  private sequence, but the capability executor it calls is exact Pulse 78
  rather than an ambiently resolved predecessor.

## Qualification boundaries

Qualification is fake-only. It proves the local loader ignores ambient
`sealed_dependencies`, proves fresh module loading, proves exact Pulse 78
binding, proves complete-load-graph serialization in 100 concurrent threads and
multi-process kernel-lock stress, proves Pulse 35 tamper rejection for a
historical alternate source digest, receipt tamper, seal tamper, and extra-tree
injection, and re-runs the full Pulse 70 / Pulse 58 behavioral suite over 20
harmless cycles. It executes no authority, no publication, no witness, and no
real FERRIS diagnostic.
