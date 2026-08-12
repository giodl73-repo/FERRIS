---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:scheduled-and-batch-job
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Scheduled and Batch Job Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/05-SCHEDULED-AND-BATCH-JOB.md
canonical_path: rust-application-blueprints/05-SCHEDULED-AND-BATCH-JOB.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:05-scheduled-and-batch-job]
concepts: [scheduled job, batch processing, run ledger, checkpoint, partition, overlap policy, rerun]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Scheduled and Batch Job Blueprint

## The Big Picture

```
+============================================================================+
| scheduler authority: desired time | calendar | trigger identity            |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| job shell: validate config -> claim run -> choose window/partitions        |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| read inputs -> process bounded units -> checkpoint -> publish result       |
+----------------------+----------------------+------------------------------+
                       v                      v
                  source authority       destination authority
                       |                      |
                       +-----------+----------+
                                   v
                     run ledger + metrics + final status
```

A scheduled job is a finite, externally triggered run. Its hard problems are
time-window meaning, overlap, partial progress, rerun identity, and publication
atomicity. Cron syntax is an adapter detail.

## Workspace Layout

```
billing-close/
|-- Cargo.toml
|-- crates/
|   |-- close-domain/
|   |-- close-application/
|   |-- run-ledger/
|   |-- source-adapter/
|   `-- publish-adapter/
|-- apps/
|   `-- billing-close-job/
`-- tests/
    |-- calendar-scenarios/
    `-- restart-scenarios/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

The binary should accept an explicit logical window or run id even when the
scheduler normally supplies it:

```text
billing-close --window-start ... --window-end ... --run-id ...
```

That makes reruns and incident recovery deliberate rather than dependent on the
current wall clock.

## Time, Window, and Overlap Contracts

| Question | Choices |
|----------|---------|
| Window basis | event time, source commit time, or scheduler time |
| Boundaries | usually half-open `[start, end)` |
| Time zone | UTC or named business zone with calendar rules |
| Late input | ignore, next run, correction run, or reopen window |
| Overlap | forbid, queue, replace, or allow disjoint partitions |
| Missed run | catch up all, coalesce, or skip with explicit evidence |

```
trigger T
   |
   v
derive logical window W
   |
   v
claim (job, W) in run ledger
   | already complete -> report existing result
   | active conflict  -> apply overlap policy
   ` claimed          -> execute/checkpoint/publish
```

The scheduler owns trigger delivery. The job owns semantic window derivation,
unless a higher-level orchestration contract supplies the window explicitly.
The run ledger owns uniqueness and recovery state.

Run parameters are privileged input. Authorize who may trigger backfills,
override windows, select destinations, or bypass normal overlap policy. Redact
secrets and sensitive source locations from scheduler arguments and logs, and
bound partitions/concurrency so an operator-triggered catch-up cannot starve
online workloads.

| Asset | Owner |
|-------|-------|
| Trigger delivery and calendar configuration | scheduler/operator |
| Logical window and completion semantics | job/application owner |
| Run claim, partition state, and audit history | run-ledger owner |
| Source snapshot/version | source-system owner |
| Published result and visibility switch | destination/job owner |

## Partitioning, Checkpointing, and Publication

```
run
|-- partition 000: pending -> running -> complete
|-- partition 001: pending -> running -> complete
|-- partition 002: pending -> failed  -> retry
`-- publish manifest only when required set is complete
```

| Pattern | Use when | Risk |
|---------|----------|------|
| One transaction | bounded data in one authority | long locks/resource pressure |
| Checkpoint per partition | restart cost matters | output must hide partial set |
| Append plus manifest | immutable outputs | manifest becomes publication authority |
| Upsert by run/key | reruns replace same logical result | key design must be stable |

Checkpoint progress only after the corresponding effect is durable. A progress
counter in memory is telemetry, not recovery state.

## Testing and Rollback

Required evidence:

- simulated clock/calendar tests, including daylight-saving transitions when a
  named civil zone is part of the contract;
- overlap and duplicate-trigger tests;
- crash between partition effect and checkpoint;
- rerun of a completed window;
- partial publication invisibility;
- old/new code compatibility with active run-ledger records.

```text
cargo test --workspace --all-targets
cargo run -p billing-close-job -- `
  --window-start 2026-08-10T00:00:00Z `
  --window-end 2026-08-11T00:00:00Z `
  --run-id test-001
```

The line continuation above is PowerShell. Use the shell's corresponding
continuation syntax on Linux/macOS.

Rollback separates code from run state:

| Condition | Response |
|-----------|----------|
| No durable effects | stop and run previous artifact |
| Checkpoint-compatible | previous artifact resumes claimed run |
| Output format changed | keep old reader or publish versioned location |
| Partial external effects | reconcile or complete forward |
| Wrong logical window | correction run; do not erase audit history |

Never assume a down migration restores a published batch result. Preserve run
identity, input snapshot/version, code version, and output manifest.

Retire a schedule by disabling new triggers first, proving no claimed or queued
run remains, reconciling the last published window, preserving the run ledger
for its audit/replay horizon, and revoking job credentials after the rollback
window closes.

## Universal Bridge First

The universal bridge is transaction processing over a finite set: define the
set, establish identity, process partitions, and atomically expose completion.
Schedulers initiate; they do not supply correctness.

Supplementally, this maps to SQL Agent, Windows Task Scheduler, Kubernetes-style
jobs, or Azure orchestration triggers. The host differs; the Rust binary should
retain explicit window and run semantics so it can be reproduced outside that
host.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Short stateless periodic action | simple scheduled binary with explicit run id |
| Restartable large finite set | partition ledger and checkpoints |
| Continuous arrivals | worker [04], not a pseudo-infinite batch |
| Dataset transformation/publication | ETL [06] |
| Multi-stage dependencies | external orchestrator plus independently rerunnable stages |
| Strict single active run | durable lease/claim, not process-local mutex |
| Correct late data | correction window or versioned republish policy |

## Common Confusion Points

- **Schedule time is not necessarily data time.** Name the clock behind each
  window.
- **A process lock is not distributed overlap control.** It disappears on
  another host or after a crash.
- **Retrying the whole run may duplicate completed partitions.** Stable
  partition identity is required.
- **Checkpointing too early loses work; too late repeats it.** Align checkpoint
  with the durable effect.
- **"Success" must include publication.** Computing hidden temporary output is
  not a completed batch contract.
- **Backfills are production behavior.** They need the same authority,
  observability, and compatibility controls as scheduled runs.

## Primary Sources

- Rust `SystemTime`: https://doc.rust-lang.org/std/time/struct.SystemTime.html
- Rust `Instant`: https://doc.rust-lang.org/std/time/struct.Instant.html
- Cargo package targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- Cargo testing: https://doc.rust-lang.org/cargo/commands/cargo-test.html
- The Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

## Related Guides

- Worker semantics: [04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md](04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md)
- Data pipelines: [06-DATA-PIPELINE-AND-ETL.md](06-DATA-PIPELINE-AND-ETL.md)
