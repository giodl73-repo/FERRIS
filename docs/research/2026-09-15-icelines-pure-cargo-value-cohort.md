# ICELINES Pure-Cargo Value Cohort

Date: 2026-09-15
Status: Complete; invalid measurement

## Frozen case

- ICELINES head:
  `935136020140bd5b408d26cbb0777dd6f0fb5ef9`;
- ICELINES base:
  `41fec3dab0dd0d28e55a3b5d5f98c2ac650f108f`;
- Ferris revision: `1baa6a8`;
- platform: local Windows x86_64;
- selected owner commands:
  - `cargo test -p icelines-core --lib`;
  - `cargo test -p icelines-fetch`; and
- full owner command: `cargo test --workspace`.

## Findings

### FERRIS-VALUE-024: The frozen ICELINES owner environment was runnable

A separate disposable checkout ran both selected commands and the full command
successfully before measurement. This distinguishes Pulse 18 from the missing
BISECT Python prerequisite, but preflight is behavioral eligibility rather than
performance evidence.

### FERRIS-VALUE-025: Revision-bound focused planning succeeded

Exact Ferris produced eight fresh successful plans without fallback. Every plan
classified the frozen revision's five committed paths into four declared
focused owner entrypoints spanning `icelines-core`, `icelines-fetch`, and the
change's retained design and role evidence.

### FERRIS-VALUE-026: The measurement wrapper did not invoke owner commands

The PowerShell helper declared a parameter named `$args`, colliding with
PowerShell's automatic argument variable. The command arrays were empty inside
the helper. Each of 24 intended selected or full invocations therefore ran bare
Cargo, printed its help text, and exited `0`.

The initial exit-code-only ledger misclassified those calls as successful.
Retained command output exposed the mismatch before publication. Zero owner
lanes ran, so all recorded durations and every derived median are invalid and
inadmissible.

## Decision

The real-adopter value gate remains `No`. No ICELINES or REEL file, dependency,
workflow, command, or environment was changed. All disposable checkouts were
removed, and the BISECT, ICELINES, and REEL research clones remained clean.
Pulse 18 is exhausted; no retry, advisory CI reconciliation, support-readiness
work, performance claim, or savings claim follows.
