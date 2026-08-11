# Ferris Typed Doctor Identity Review

Date: 2026-08-11
Scope: Pulse 07 remediation after FHIF-014
Disposition: Validated; new replacement fixture pending
Implementation authority: No expansion

## Result

Doctor record identity is no longer maintained as a parallel manual field
list. Ferris constructs the complete typed record, leaves `report_id` empty,
hashes the canonical record, and then assigns that identity. Invocation
identity binds the resulting report identity.

Canonical Cargo evidence now requires supported lowercase commit lengths and
a valid Gregorian release date. Oversized manifests receive portable
bounded-prefix selection identity without reading beyond the configured
limit.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed formatting, 24 core tests, 12 CLI
tests, lint with warnings denied, and diff checks. The two process-helper tests
remain ignored except when invoked by their timeout/output-bound tests.

## Held-out state

- FHIF-013: failed development evidence; never rescore.
- FHIF-014: failed development evidence; never rescore.
- next score: requires a newly designed and sealed fixture with a new ID.

All nine roles accept the deterministic correction without capability
expansion. Validation Checker withholds a doctor held-out pass.
