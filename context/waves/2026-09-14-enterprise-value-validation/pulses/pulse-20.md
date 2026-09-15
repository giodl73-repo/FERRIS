# Pulse 20: Capacity-Bound ICELINES Value Cohort

Status: Authorized
Implementation authority: One bounded local measurement only

## Authority

The user's fresh 2026-09-15 `continue` direction authorizes one process-local
capacity correction after Pulse 19:

- set `CARGO_INCREMENTAL=0` identically for every selected and full owner
  invocation; and
- retain at least 10 GiB free after independently built selected and full
  preflight target trees coexist.

No shared compiler cache, Cargo configuration, toolchain, adopter, dependency,
workflow, or persistent environment may change. The shared kache remains
ordinary ambient owner infrastructure and MUST NOT be cleaned, reconfigured,
or claimed as Ferris behavior.

The capacity preflight uses separate selected and full disposable checkouts,
runs the exact owner commands with semantic test evidence, measures both target
trees, and checks free capacity while both trees still exist. Failure ends the
pulse before measurement.

If capacity passes, the preflight roots are removed and the pulse reruns Pulse
19's exact Ferris and ICELINES revisions, owner-domain declaration, owner
commands, eight-pair order, checkout-local cold/warm states, planning charge,
semantic-log gates, 10% threshold, cleanup, and exclusions under the same
process-local nonincremental profile.

This is the final capacity-profile attempt in this wave. Any capacity,
planning, owner-command, semantic-log, cleanup, or threshold failure ends the
pulse. No further correction follows automatically.

Product Value Governor disposition: `continue-within-budget`.
