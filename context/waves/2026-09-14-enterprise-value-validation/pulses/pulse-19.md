# Pulse 19: Corrected ICELINES Pure-Cargo Value Cohort

Status: Complete; incomplete at resource exhaustion
Implementation authority: Exhausted

## Authority

The user's explicit 2026-09-15 direction to continue authorizes one correction
of Pulse 18's measurement wrapper:

- the command-array parameter MUST be named `commandArgs`, not PowerShell's
  reserved automatic variable `$args`; and
- every successful owner invocation MUST emit test-run evidence and MUST NOT
  emit bare Cargo usage.

The pulse otherwise inherits Pulse 18's exact Ferris and ICELINES revisions,
owner commands, disposable owner-domain declaration, separate preflight,
eight-pair order, checkout-local cold/warm states, selected planning charge,
10% threshold, cleanup, and exclusions. Pulse 18 timings are never reused.

Any preflight, planning, owner-command, semantic-log, cleanup, or threshold
failure ends the pulse. No further correction or later gate follows
automatically.

Product Value Governor disposition: `continue-within-budget`.

## Result

The corrected helper bound `commandArgs` and its semantic checks rejected bare
Cargo usage. A fresh disposable preflight ran both selected commands and the
full workspace command successfully. The fresh planning preflight and pair 1
plan also succeeded without fallback over all five inputs and four focused
owner entrypoints.

Pair 1's selected lane ran both real owner commands and passed. The full lane
then failed during compilation with `rustc-LLVM ERROR: IO failure on output
stream: no space on device` and Cargo exit `101`. No complete pair or
admissible timing resulted, and pairs 2 through 8 did not start.

The pulse did not delete or alter the shared compiler cache. All disposable
checkouts were removed, recovering local capacity, and the BISECT, ICELINES,
and REEL research clones remained clean. Pulse 19 is exhausted and grants no
retry or later-gate authority.
