# Capacity-Bound ICELINES Value Cohort

Date: 2026-09-15
Status: Complete; invalid before measurement

## Frozen profile

Pulse 20 retained Pulse 19's exact Ferris and ICELINES revisions, owner
commands, owner domains, semantic command checks, eight-pair design, threshold,
and exclusions. It added only process-local `CARGO_INCREMENTAL=0` for every
owner invocation and a mandatory 10 GiB free-space reserve after independent
selected and full target trees coexisted.

The machine exposed one local 1.86 TiB Windows volume with 39.49 GiB free during
selection. The shared kache measured 19.44 GiB and was explicitly excluded from
cleanup or reconfiguration.

## Findings

### FERRIS-VALUE-030: The selected nonincremental target was runnable

The selected checkout ran the exact `icelines-core --lib` and
`icelines-fetch` owner commands with semantic test evidence. Both passed. Its
checkout-local target tree measured 7.757 GiB.

### FERRIS-VALUE-031: The independent full target did not complete

The full checkout ran the exact workspace owner command but exited `101` during
linking. The retained log records multiple linker/PDB resource failures. Its
partial target tree had already reached 16.548 GiB.

### FERRIS-VALUE-032: The capacity reserve correctly blocked measurement

The two target trees already occupied 24.305 GiB while the incomplete full
build left 3.544 GiB free. This failed the frozen 10 GiB reserve. No Ferris
measurement root, selected/full pair, timing, median, or value conclusion
followed.

## Decision

The real-adopter value gate remains `No`. Both preflight roots were removed and
30.798 GiB free was observed afterward. The shared compiler cache, BISECT,
ICELINES, and REEL remained unchanged. Pulse 20 is the final capacity-profile
attempt in this wave; no retry, advisory CI reconciliation,
compatibility/support work, performance claim, or savings claim is authorized.
