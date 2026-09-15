# Corrected ICELINES Pure-Cargo Value Cohort

Date: 2026-09-15
Status: Complete; incomplete measurement

## Frozen case

Pulse 19 retained Pulse 18's exact Ferris revision, ICELINES head and base,
selected owner commands, full owner command, eight-pair order, checkout-local
cold/warm states, planning charge, threshold, and exclusions. Its only harness
changes were renaming the PowerShell command-array parameter to `commandArgs`
and requiring semantic test-run evidence in each successful log.

## Findings

### FERRIS-VALUE-027: Semantic command validation prevented another false pass

The corrected helper passed command arguments to Cargo. Fresh preflight logs
contained `test result: ok` and did not contain bare Cargo usage for both
selected commands and the full workspace command.

### FERRIS-VALUE-028: Focused planning and the first selected lane passed

The planning preflight and pair 1 plan both classified all five committed paths
into four focused owner entrypoints without fallback. Pair 1's selected lane
then ran the real `icelines-core --lib` and `icelines-fetch` commands, and both
passed their semantic and exit-code gates.

### FERRIS-VALUE-029: Local disk exhaustion prevented a complete pair

Pair 1's full `cargo test --workspace` reached real compilation and stopped
with Cargo exit `101`. Rustc reported `IO failure on output stream: no space on
device` while producing test artifacts. The pulse stopped immediately; pairs 2
through 8 did not start.

No complete selected/full pair exists. The selected lane duration cannot be
compared to a successful full reference, so zero timings are admissible and no
median or value conclusion follows.

## Decision

The real-adopter value gate remains `No`. The shared compiler cache was not
altered. All disposable roots were removed, local capacity was recovered, and
the BISECT, ICELINES, and REEL research clones remained clean. No retry,
advisory CI reconciliation, compatibility/support work, performance claim, or
savings claim is authorized.
