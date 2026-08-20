---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:production-debugging-incident-response
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Production Debugging and Incident Response
status: source-custody
source_custody: partial
current_path: rust-production-engineering/13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md
canonical_path: rust-production-engineering/13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md
backsource_ids: [proof-backfill:rust-production-engineering:13-production-debugging-incident-response]
concepts: [production debugging, incident response, triage, profiling, crash dumps, mitigation, postmortem]
root_concepts: [incident response]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Production Debugging and Incident Response

## The Big Picture

Incident response is a control loop under uncertainty. Stabilize user impact,
preserve evidence, form discriminating hypotheses, apply the least risky
mitigation, verify recovery, and only then pursue complete explanation.

```
+============================================================================+
|                         INCIDENT CONTROL LOOP                              |
|                                                                            |
| detect --> declare/own --> bound impact --> preserve evidence              |
|                              |                    |                        |
|                              v                    v                        |
|                         mitigate now        investigate hypotheses         |
|                              |                    |                        |
|                              +-------- verify recovery                     |
|                                           |                                |
|                                           v                                |
|                              reconcile state --> learn/fix                 |
|                                                                            |
| tracks: command | communications | operations | investigation              |
+============================================================================+
```

Debugging is one incident track, not the whole response. A perfect root cause
found after avoidable user harm is a failed incident process.

## First Questions

| Question | Evidence |
|---|---|
| What is the user-visible symptom? | SLI, support signal, failed operation |
| What changed? | release, config, dependency, traffic, platform events |
| How broad is impact? | region, tenant class, route, release, dependency |
| Is capacity failing? | queues, pools, CPU, memory, descriptors, throttling |
| Is state at risk? | error taxonomy, transaction/outbox lag, corruption signal |
| What action is safest now? | rollback, shed, disable feature, add capacity, fail over |

Anchor the timeline in UTC and artifact identity. Do not assume temporal
correlation proves causation, but use changes to prioritize reversible tests.

## Scoped Linux Triage Commands

Scope: a systemd-managed Linux process named `orders`; commands require
appropriate host access and may expose sensitive data.

```bash
systemctl status orders --no-pager
journalctl -u orders --since "-15 min" --no-pager
pid="$(systemctl show -p MainPID --value orders)"
test "$pid" -gt 0
ps -o pid,ppid,stat,etimes,%cpu,%mem,rss,vsz,nlwp,cmd -p "$pid"
cat "/proc/$pid/limits"
ls -1 "/proc/$pid/fd" | wc -l
```

If core dumps are configured:

```bash
coredumpctl list orders --no-pager
coredumpctl info orders --no-pager
```

Do not attach a debugger, dump memory, or collect profiles before considering
latency, pause, data exposure, and chain-of-custody effects.

## Scoped Windows Triage Commands

Scope: a Windows service process; run from PowerShell with authorized access.

```powershell
Get-Service -Name orders
$p = Get-Process -Name orders
$p | Select-Object Id, StartTime, CPU, WorkingSet64, PrivateMemorySize64,
  @{Name = "ThreadCount"; Expression = { $_.Threads.Count }}
Get-WinEvent -FilterHashtable @{
  LogName = "Application"
  StartTime = (Get-Date).AddMinutes(-15)
} | Select-Object -First 100 TimeCreated, Id, LevelDisplayName, Message
```

Windows Error Reporting or an approved dump tool can capture a dump. Preserve
the exact executable and PDB mapping. Dumps can contain credentials and customer
data; apply incident-data controls.

## Rust-Specific Diagnostic Surfaces

| Symptom | Useful evidence |
|---|---|
| Panic | hook output, backtrace, task/thread identity, release |
| Deadlock/stall | thread stacks, task dump if runtime supports it, lock/queue metrics |
| CPU hot loop | sampled profile with symbols |
| Allocation growth | allocator/process metrics, heap profiler where supported |
| Async starvation | runtime/task metrics, blocking duration, poll hotspots |
| Native crash | core/minidump, symbols, FFI boundary, unsafe-code inventory |

Release builds need enough debug information for symbolication. Frame pointers
can improve profiling on some targets at a performance/code-size cost. Verify
the chosen profiler and unwind method on the actual target.

## Hypothesis-Driven Investigation

```
hypothesis: DB pool starvation drives timeouts

predict:
  pool wait rises before request latency
  active connections remain at max
  DB execution time may remain normal

query/measure:
  pool_wait histogram, in_use gauge, query histogram

discriminate:
  if query time rises too, dependency slowdown is more likely
```

Write predictions before querying. Otherwise dashboards encourage a story for
whatever line moved. Change one reversible variable at a time when possible and
record commands/results in the timeline.

## Mitigation Hierarchy

| Mitigation | Benefit | Risk |
|---|---|---|
| Shed optional work | immediate capacity protection | reduced functionality |
| Disable feature | isolates recent/high-cost path | config propagation |
| Roll back | fast if compatibility remains | state/schema incompatibility |
| Roll forward | preserves one-way state transition | new code under pressure |
| Add capacity | helps true capacity shortage | may overload dependency |
| Fail over | escapes failed domain | stale state, cold caches, reduced margin |
| Restart | clears some local faults | destroys evidence; restart storm |

Restarting is a mitigation, not a diagnosis. Capture enough evidence first when
impact and safety permit.

## Communications and Handoffs

Maintain one incident commander and explicit operations/investigation owners.
Updates should state impact, current hypothesis, action taken, observed result,
next decision time, and unknowns. Avoid optimistic ETAs unsupported by
evidence.

After recovery, reconcile uncertain operations, delayed messages, partial
rollouts, and temporary policy changes. Incident closure before state
reconciliation creates the next incident.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | panic hooks, tracing, metrics, diagnostic endpoints |
| Runtime | task dumps/metrics, scheduler behavior, runtime console |
| Platform | journal/Event Log, profiler, dump service, orchestration events |

Tokio Console is useful for instrumented Tokio applications, not a universal
Rust debugger. `perf`, eBPF tools, ETW, Windows Performance Recorder, and cloud
profilers are platform-specific mechanisms.

## Old World -> New World Bridge

The universal bridge is from **debugging a program** to **controlling a
socio-technical system**. The code, deployment controller, operator action,
dependency, and data state all belong in the hypothesis graph.

Visual Studio dump debugging, WinDbg, ETW, and Azure Monitor are strong
supplemental tools for Windows/Azure estates. The incident sequence remains the
same on Linux, bare metal, or another cloud.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Rollback | recent artifact is suspect and compatibility is reversible |
| Feature disable | one path can be isolated without broad rollback |
| Load shedding | saturation threatens all work |
| Sampled CPU profile | CPU is high and stack attribution is needed |
| Core/minidump | crash/native state requires postmortem |
| Live debugger | lower-impact evidence is insufficient and pause risk is accepted |
| Restart | local state reset is safer than continued impact |
| Failover | failure domain is isolated and target state/capacity are verified |

## Common Confusion Points

- **The loudest log is not necessarily the cause.**
- **A restart destroys volatile evidence and may amplify load.**
- **Async stalls may occur with low CPU.**
- **Symbols must match the exact binary, not merely the version label.**
- **Profiling and dump capture have overhead and privacy consequences.**
- **Root cause is often a contributing-factor graph, not one defective line.**

## Primary Sources

- Rust backtraces: https://doc.rust-lang.org/std/backtrace/
- Linux proc filesystem: https://docs.kernel.org/filesystems/proc.html
- systemd coredump: https://www.freedesktop.org/software/systemd/man/latest/coredumpctl.html
- Windows Error Reporting: https://learn.microsoft.com/windows/win32/wer/windows-error-reporting
- Google SRE incident response: https://sre.google/sre-book/managing-incidents/
- FIRST incident response teams: https://www.first.org/standards/frameworks/csirts/

## Related Guides

- Previous: [12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md](12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md)
- Next: [14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md](14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md)
- Telemetry: [02-STRUCTURED-LOGGING-AND-TRACING.md](02-STRUCTURED-LOGGING-AND-TRACING.md), [03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md](03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md)
- Crash policy: [04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md](04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md)
