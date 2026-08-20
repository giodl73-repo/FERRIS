---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:metrics-health-telemetry-design
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Metrics, Health, and Telemetry Design
status: source-custody
source_custody: partial
current_path: rust-production-engineering/03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md
canonical_path: rust-production-engineering/03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md
backsource_ids: [proof-backfill:rust-production-engineering:03-metrics-health-telemetry-design]
concepts: [metrics, health checks, telemetry, cardinality, histograms, readiness, liveness, saturation]
root_concepts: [observability]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Metrics, Health, and Telemetry Design

## The Big Picture

Metrics compress many executions into time series. Health endpoints answer
narrow routing or supervision questions. Traces and logs explain individual
paths. Production telemetry works when these signals share vocabulary without
pretending they are interchangeable.

```
+============================================================================+
|                         TELEMETRY CONTRACT                                 |
|                                                                            |
|  work --> counters + histograms + gauges --> aggregation --> SLI/alerts    |
|    |                                                                       |
|    +----> health state --> live / ready / startup --> supervisor/router    |
|    |                                                                       |
|    +----> spans/events -----------------------> investigation              |
|                                                                            |
|  shared resource fields: service, release, region, instance, role          |
|  controlled dimensions: operation, outcome, dependency, status_class       |
+============================================================================+
```

Start with user-visible questions: Is useful work succeeding? How long does it
take? Is demand approaching a hard resource limit? Instrument implementation
details only when they help answer or explain those questions.

## Metric Types and Semantics

| Type | Good use | Frequent mistake |
|---|---|---|
| Counter | requests, failures, bytes, retries | resetting or treating as current state |
| Gauge | queue depth, in-flight work, open connections | recording monotonic totals |
| Histogram | latency, payload size, wait time | choosing buckets after the incident |
| Up/down counter | active sessions or workers | non-atomic updates that drift |

Name the unit and aggregation meaning. `request_duration` without seconds,
scope, or operation is not a contract. Prefer stable dimensions such as route
templates, not raw paths or user identifiers.

## Cardinality Is a Capacity Budget

If a metric has 20 operations, 8 status values, 10 regions, and 50 releases, it
can produce 80,000 series before instance labels. Every unbounded label
(`user_id`, URL, SQL text, error message) turns telemetry into an uncontrolled
database.

```
series count ~= product of label cardinalities

operation(20) * outcome(4) * region(5) * release(3) = 1,200
user_id(1,000,000) added                              = disaster
```

Resource attributes also multiply series. Instance, pod, build, or commit
identity is not free merely because the telemetry SDK attaches it
automatically; retain it on service-level metrics only when the backend and
operator task justify that per-instance split.

Record high-cardinality identity in sampled traces or bounded logs, not in
metric labels.

## Executable Minimal Metric

This standard-library example demonstrates thread-safe monotonic accounting and
Prometheus-compatible text without choosing a metrics crate or HTTP framework.
It is suitable for learning and tests; production services normally use a
maintained recorder/exporter library.

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static REQUESTS: AtomicU64 = AtomicU64::new(0);
static FAILURES: AtomicU64 = AtomicU64::new(0);

fn record_request(ok: bool) {
    REQUESTS.fetch_add(1, Ordering::Relaxed);
    if !ok {
        FAILURES.fetch_add(1, Ordering::Relaxed);
    }
}

fn render() -> String {
    format!(
        "# TYPE app_requests_total counter\n\
         app_requests_total {}\n\
         # TYPE app_failures_total counter\n\
         app_failures_total {}\n",
        REQUESTS.load(Ordering::Relaxed),
        FAILURES.load(Ordering::Relaxed)
    )
}

fn main() {
    record_request(true);
    record_request(false);
    print!("{}", render());
}
```

Run with `rustc metrics.rs && ./metrics` (PowerShell:
`rustc metrics.rs; .\metrics.exe`). `Relaxed` is sufficient because these
counters do not synchronize other memory; exact cross-counter snapshots are not
guaranteed. A real exporter also handles metadata, label encoding, concurrent
collection, and transport.

## Health Contracts

```
startup: has initialization completed enough to evaluate readiness?
liveness: is restart likely to improve this stuck process?
readiness: should new traffic or work be assigned now?
```

| Probe | Failure action | Include dependency checks? |
|---|---|---|
| Startup | continue waiting or fail deployment | only initialization-critical dependencies |
| Liveness | restart process | rarely; dependency outage should not restart every client |
| Readiness | remove from routing/admission | dependencies required to serve this workload |

Health should be cheap, bounded, and explicit. A deep query against every
dependency can become an outage amplifier. When one capability can degrade
independently, expose component state or route-level admission rather than
turning the whole process unready.

## Histograms and Tail Behavior

Averages hide saturation. Use distributions for request latency, queue wait,
dependency latency, and payload size. Choose bucket boundaries from service
objectives and physical limits before rollout. Quantiles calculated locally
cannot generally be averaged across instances; aggregate histograms or use a
backend with defined distribution semantics.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | `metrics`, OpenTelemetry metrics, framework-specific instruments |
| Runtime | exporter task scheduling, scrape endpoint, shutdown flush |
| Platform | pull vs push collection, retention, alert evaluation, dashboards |

Prometheus exposition is a transport/ecosystem choice, not the definition of a
metric. Kubernetes probes are one consumer of health contracts; systemd,
load balancers, and custom supervisors can consume equivalent state.

## Old World -> New World Bridge

The universal bridge is from **performance counters** to **dimensional time
series**. The advantage is sliceable aggregation; the price is a cardinality
budget and carefully defined labels.

Windows Performance Counters, EventCounters, and Azure Monitor metrics map
naturally to counters, gauges, and distributions. Those products are useful
bridges, but the service-level names and units should remain portable.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Counter | event total only increases |
| Gauge | current bounded state can rise or fall |
| Histogram | distribution and tail behavior matter |
| Liveness check | restart can repair a stuck process |
| Readiness check | new work should be admitted or withheld |
| Startup check | initialization may legitimately exceed liveness delay |
| Trace/log field | value is high-cardinality or request-specific |
| Metric label | value set is small, stable, and operationally useful |

## Common Confusion Points

- **Readiness is not dependency uptime.** It is a local admission decision.
- **Liveness failure is an automated destructive action.** False positives
  create restart storms.
- **A gauge is not a sampled event count.** Define whether it is current state,
  last value, or an asynchronously observed value.
- **Percentiles do not compose by averaging.** Preserve distributions.
- **More labels do not mean better observability.** They can make the system
  unaffordable or unusable.

## Primary Sources

- OpenTelemetry metrics specification: https://opentelemetry.io/docs/specs/otel/metrics/
- Prometheus metric types: https://prometheus.io/docs/concepts/metric_types/
- Prometheus naming guidance: https://prometheus.io/docs/practices/naming/
- Kubernetes probe semantics: https://kubernetes.io/docs/concepts/workloads/pods/probes/
- Google SRE monitoring distributed systems: https://sre.google/sre-book/monitoring-distributed-systems/

## Related Guides

- Previous: [02-STRUCTURED-LOGGING-AND-TRACING.md](02-STRUCTURED-LOGGING-AND-TRACING.md)
- Next: [04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md](04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md)
- Capacity signals: [06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md](06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md)
- SLO use: [14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md](14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md)
