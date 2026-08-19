# Ferris Shared-Substrate Diamond

Date: 2026-08-20
Status: Measured local public-repository evidence
Product behavior changed: No

## Purpose

This record tests the graph axis that crosses repository silos: two shared
producers consumed by three independently owned applications. It also tests
whether Ferris can distinguish conservative impact propagation from Cargo's
actual resolved revision identity.

The complete execution record is
[Pulse 01](../../../context/waves/2026-08-20-shared-substrate-diamond/pulses/pulse-01.md).
The structured result is in the
[diamond receipt](FERRIS-SHARED-SUBSTRATE-DIAMOND-RECEIPT.json).

## Diamond

```text
FLETCH -----+----> BISECT
            +----> ROUTE
            +----> ICELINES

METIS-CORE -+----> BISECT
            +----> ROUTE
```

These are explicit review relationships. Each consumer's Cargo manifest and
lockfile remain authoritative for its actual dependency source and revision.

## Main result

Ferris selected 4 of 5 workspace scopes for a FLETCH change, 3 of 5 for a
METIS-CORE change, and all 5 when both producers changed. Consumer-local
changes remained isolated to one workspace.

The same evidence exposed a separate compatibility axis: every consumer locks
an older FLETCH revision than the producer snapshot, and both applicable
consumers lock an older METIS-CORE revision. Current Ferris does not report
this skew automatically.

## Decision

Adopt the result as evidence for explicit cross-repository impact planning and
as a bounded requirement for a future read-only revision-skew report. Do not
convert it into automatic dependency discovery, semantic compatibility,
manifest rewriting, validation execution, or build-time savings claims.
