---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:slos-runbooks-ownership-cost
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: SLOs, Runbooks, Ownership, and Cost
status: source-custody
source_custody: partial
current_path: rust-production-engineering/14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md
canonical_path: rust-production-engineering/14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md
backsource_ids: [proof-backfill:rust-production-engineering:14-slos-runbooks-ownership-cost]
concepts: [slos, slis, error budgets, runbooks, ownership, on-call, cost, unit economics]
root_concepts: [service reliability]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# SLOs, Runbooks, Ownership, and Cost

## The Big Picture

Operational maturity connects a user promise to measured behavior, an owner,
an action policy, and an economic boundary. SLOs without ownership are reports;
alerts without runbooks are interruptions; reliability without cost visibility
can consume unlimited resources.

```
+============================================================================+
|                       SERVICE OPERATING MODEL                              |
|                                                                            |
| user journey --> SLI --> SLO --> error budget --> action policy            |
|      |            |       |          |                |                    |
|      |            |       |          v                v                    |
|      |            |       |      release pace     alert/runbook            |
|      |            |       |                                                |
|      +------------+-------+--> named owner + dependency contracts          |
|                                   |                                        |
|                                   v                                        |
|                    capacity + telemetry + support + cost                   |
+============================================================================+
```

Choose a small number of indicators that represent successful user work. Host
CPU and memory are diagnostic signals, not usually user-facing SLIs.

## SLI and SLO Design

| Element | Example |
|---|---|
| Event | an eligible order submission |
| Good event | accepted durably within 750 ms |
| SLI | good eligible events / all eligible events |
| SLO | 99.9% over rolling 28 days |
| Exclusions | explicitly enumerated invalid requests |
| Error budget | 0.1% bad eligible events |

Define eligibility and measurement point precisely. Measuring at the server
after the load balancer can miss requests that never reached the process.
Latency objectives often use multiple thresholds so a small number of very slow
requests are not hidden.

## Executable Error-Budget Calculation

```rust
fn main() {
    let total = 1_000_000_u64;
    let good = 999_200_u64;
    let objective = 0.999_f64;

    let allowed_bad = ((1.0 - objective) * total as f64).round() as u64;
    let observed_bad = total - good;
    let remaining = allowed_bad.saturating_sub(observed_bad);
    let attainment = good as f64 / total as f64;

    println!("attainment={:.4}%", attainment * 100.0);
    println!("allowed_bad={allowed_bad}");
    println!("observed_bad={observed_bad}");
    println!("remaining_budget={remaining}");
}
```

Run with `rustc budget.rs && ./budget` (PowerShell:
`rustc budget.rs; .\budget.exe`). Scope: one simple availability SLI. Real
systems need windowing, late data, low-volume statistics, multiple thresholds,
and policy for missing telemetry.

## Burn-Rate Alerts

An error budget converts objective breach into a rate. Fast-burn alerts detect
severe failures over short and longer confirmation windows; slow-burn alerts
detect persistent degradation. Page only when timely human action can improve
the outcome.

```
budget burn
  fast + high impact  --> page
  slow + actionable   --> ticket/work queue
  diagnostic only     --> dashboard, no alert
```

Alerts should include affected SLO, scope, current burn, release/change context,
and the first runbook action. Avoid one alert per instance for a service-level
failure.

## Runbooks

| Runbook section | Required content |
|---|---|
| Trigger | alert and symptom this handles |
| User impact | what is failing and for whom |
| Safety | actions that can lose data or widen impact |
| First checks | bounded queries/commands and expected results |
| Mitigations | reversible steps, permissions, verification |
| Escalation | owner, dependency contact, decision thresholds |
| Recovery | health confirmation and state reconciliation |
| Follow-up | temporary changes and evidence to preserve |

Test runbooks during game days and after platform changes. A command copied from
an old incident is not reliable until its permissions, output, and side effects
are revalidated.

## Ownership and Dependency Contracts

Every service needs a named team, escalation path, repository, artifact
registry, dashboards, SLOs, runbooks, data classification, and lifecycle state.
Shared libraries also need ownership because a vulnerability or runtime defect
can affect many services at once.

For dependencies, record:

- provider and escalation path;
- client timeout/retry/admission policy;
- dependency objective or expectation;
- failure/degraded behavior;
- data and identity exchanged;
- exit/migration plan.

## Cost as a Reliability Dimension

| Cost driver | Useful denominator |
|---|---|
| CPU/memory | per successful request or job |
| Database | per durable operation or tenant |
| Telemetry | per million requests and per retained day |
| Network | per useful byte/workflow |
| On-call toil | interventions per service/month |
| Idle capacity | resilience margin by failure domain |

Optimize cost per successful unit of work, not host utilization in isolation.
High utilization can destroy latency and recovery margin; low utilization can be
appropriate insurance. Make the chosen margin visible.

Rust can reduce compute and memory cost, but engineering time, compile time,
operational tooling, and specialized knowledge are also costs. Measure the
whole service, not a language benchmark.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | SLI instrumentation and cost attribution fields |
| Runtime | runtime metrics that explain saturation, not the objective itself |
| Platform | SLO evaluator, paging, runbook system, cost allocation |

Azure Monitor, Prometheus, Grafana, Datadog, and other systems can evaluate
signals. The SLI definition and action policy should outlive one backend.

## Old World -> New World Bridge

The universal bridge is from **availability reporting** to **reliability
governance**. An SLO becomes useful when its error budget changes release and
investment decisions.

Traditional operations manuals, Windows performance dashboards, and VSTS
service ownership records are direct prior art. Modern runbooks add executable
links, telemetry context, and continuous validation; Azure-specific tooling is
one implementation.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Availability SLI | success/failure of eligible work is primary |
| Latency SLI | delayed success loses user value |
| Freshness SLI | data age matters more than request latency |
| Error budget | reliability and release pace need one policy variable |
| Page | prompt human action can materially reduce user harm |
| Ticket | issue is actionable but not urgent |
| Runbook | response requires repeatable diagnosis/mitigation |
| Unit cost | architecture or scaling decision needs economic comparison |

## Common Confusion Points

- **An SLA is not an SLO.** A contract/penalty surface and an engineering target
  serve different purposes.
- **100% objectives usually destroy useful prioritization.**
- **Infrastructure uptime can be green while user work fails.**
- **An alert that has no action should not page.**
- **Cost optimization that removes failure margin can increase total cost.**
- **Ownership by a distribution list is not decision ownership.**

## Primary Sources

- Google SRE service level objectives: https://sre.google/workbook/implementing-slos/
- Google SRE alerting on SLOs: https://sre.google/workbook/alerting-on-slos/
- OpenSLO specification: https://openslo.com/
- FinOps Framework: https://www.finops.org/framework/
- OpenTelemetry semantic conventions: https://opentelemetry.io/docs/specs/semconv/

## Related Guides

- Previous: [13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md](13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md)
- Next: [15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md](15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md)
- Metric design: [03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md](03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md)
- Capacity/cost: [06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md](06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md)
