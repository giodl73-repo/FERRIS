# Pulse 19: Corrected ICELINES Pure-Cargo Value Cohort

Status: Authorized
Implementation authority: One bounded local measurement only

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
