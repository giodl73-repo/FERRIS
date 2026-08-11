# W05: Governance and Action

Status: Complete after retrace
Claim state: simulated

## Goal

Test exact approval binding, preflight drift, cancellation races, rollback and
cleanup composition, conflicting actions, and audit-visible recovery.

## Locked specification baseline

Baseline commit: `4839acd`

The retrace includes FSIM-SCR-010 and FSIM-SCR-011.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-017](FSIM-017-preflight-drift.md) | Toolchain changes after exact approval | Stale approval and preflight block | Pass without spec change |
| [FSIM-018](FSIM-018-cancellation-race.md) | Cancellation arrives during a non-interruptible owner operation | Requested versus effective stop | Pass after FSIM-SCR-010 |
| [FSIM-019](FSIM-019-rollback-cleanup-failure.md) | Rollback succeeds while cleanup and external recovery fail | Composite terminal outcome | Pass after FSIM-SCR-011 |
| [FSIM-020](FSIM-020-conflicting-action-isolation.md) | Two approved actions target one writable worktree | Conflict detection and isolation | Pass without spec change |

## Wave issues

- FSIM-SI-011: cancellation lacked a stateful protocol separating request,
  acknowledgement, effective stop, and too-late completion; and
- FSIM-SI-012: one final state could lose simultaneous rollback, cleanup, and
  residual-effect failures.

## Role review

- Rust Safety Steward: accepted after cancellation cannot claim an owner
  operation stopped before a safe point and all residual effects remain visible.
- Compiler Performance Engineer: accepted because cancellation and isolation
  preserve owner behavior rather than adding unsafe shared-state shortcuts.
- Interop Boundary Auditor: accepted because external compensation remains
  distinct from local rollback and cleanup.
- AI Assurance Skeptic: accepted because AI cannot approve, silently alter, or
  summarize away partial action outcomes.
- Ecosystem Strategist: accepted because Cargo and external owner operations
  retain their native stop and error semantics.
- Rust Maintainer: accepted because stale preflight, conflicting worktrees,
  and recovery ownership are explicit.
- Native Platform Adopter: accepted for Draft after non-interruptible native
  operations and irreversible effects retained safe-point behavior.
- Scope Keeper: accepted as a bounded controlled-action wave with no execution.
- Validation Checker: accepted after preflight, cancellation, rollback,
  cleanup, conflict, and failure controls were retraced.

## Disposition

Close W05 with no open P0 or P1 issues. Continue to W06 connectors and MCP.
