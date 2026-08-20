---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:production-profiling-telemetry-capacity-and-cost
kind: guide
module: rust-performance
section: rust-performance
title: Production Profiling, Telemetry, Capacity, and Cost
status: source-custody
source_custody: partial
current_path: rust-performance/14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md
canonical_path: rust-performance/14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md
backsource_ids: [proof-backfill:rust-performance:14-production-profiling-telemetry-capacity-and-cost]
concepts: [production profiling, telemetry, capacity planning, cost efficiency, observability, canary]
root_concepts: [production performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Production Profiling, Telemetry, Capacity, and Cost

## The Big Picture

Lab evidence becomes operational truth only after deployment. Production
performance engineering connects user outcomes to queues, resource saturation,
code paths, capacity curves, and cost while keeping observability overhead and
data exposure bounded.

```
+=============================================================================+
|                         PRODUCTION EVIDENCE                                 |
|                                                                             |
| users -> requests/jobs -> queues -> Rust process -> dependencies            |
|          |                |          |               |                      |
|          v                v          v               v                      |
|    traces/logs       depth/waits  profiles       downstream SLOs            |
|                              |                                              |
|                              v                                              |
|                    capacity and cost model                                  |
|                req/core-hour, bytes/req, $/unit                             |
|                              |                                              |
|                              v                                              |
|                  canary -> gate -> rollout/rollback                         |
+=============================================================================+
```

## Telemetry Layers

| Layer | Core signals |
|-------|--------------|
| User/SLO | success, latency distribution, freshness, deadline misses |
| Request/job | route/type, size class, result, queue time, service time |
| Runtime | task/worker queues, thread utilization, lock waits, allocations |
| Process | CPU, RSS/working set, faults, handles/FDs, network/disk bytes |
| Host/container | quota, throttling, steal time, NUMA, disk/network limits |
| Dependency | downstream latency, retries, errors, pool saturation |
| Cost | instance-hours, egress/storage, requests or records per resource unit |

Use RED (rate, errors, duration) for request-oriented services and USE
(utilization, saturation, errors) for resources. Neither replaces profiles:
metrics say when and how much; profiles say where.

## Instrumentation Without Cardinality Collapse

High-cardinality labels such as user ID, URL, file name, or arbitrary error text
can overwhelm telemetry systems and leak data.

| Good metric dimension | Risky metric dimension |
|-----------------------|------------------------|
| route template | raw URL |
| status class | full error message |
| bounded size bucket | exact object ID |
| deployment version | request correlation ID |
| operation name | arbitrary SQL/query text |

Put per-request detail in sampled traces/logs with access controls, not metric
labels. Measure telemetry CPU, allocations, bytes, and lock contention under
load. Disabled log levels may still evaluate expensive arguments if code is
structured poorly; inspect the logging facade's behavior.

## Traces and Queue Time

```
request span
  +-- admission queue: 12 ms
  +-- handler CPU:       3 ms
  +-- downstream wait:  40 ms
  +-- encode/write:      2 ms
total:                  57 ms
```

Without queue-time spans, a 3 ms handler looks innocent while the service misses
its SLO. Propagate deadlines and trace context across async tasks, channels, and
outbound calls. Sampling must retain enough slow/error traces to explain tails;
uniform low-rate sampling can discard the very events being investigated.

OpenTelemetry is a common vendor-neutral ecosystem, but crate APIs and exporters
evolve independently of stable Rust. Pin compatible versions and isolate
telemetry behind a small internal facade.

## Production Profiling

Prefer low-overhead sampling with a defined budget and symbol custody.

```
# Linux template: replace <PID> with one approved numeric process ID.
perf record -F 99 -g -p <PID> -- sleep 30
perf report
```

Attaching requires permissions and can expose code paths or tenant-sensitive
metadata in stacks. Follow incident/change policy. Continuous profilers and eBPF
tools have their own kernel, container, and privilege requirements. Measure
profiler CPU, memory, network, and tail-latency overhead before continuous use.

On Windows, WPR/WPA, PerfView, Visual Studio tooling, or an approved continuous
profiler can collect native stacks and ETW events. Preserve matching PDBs. In
Azure, Application Insights/Azure Monitor can provide request/dependency metrics
and traces, while VM/container profiling still needs the appropriate native
tooling and permissions. Azure is a telemetry backend, not a substitute for a
performance model.

## Symbol and Build Custody

```
deployment version -> exact binary -> exact symbols -> exact source/toolchain
```

Keep:

- build/commit ID embedded or emitted at startup;
- `rustc -Vv`, target, lockfile, features, profile, linker policy;
- PDB/DWARF/dSYM indexed by artifact identity;
- mapping for post-link rewritten/stripped binaries;
- retention long enough for delayed incidents.

An unsymbolized production profile is often expensive evidence with little
actionability.

## Capacity Curves

Run load sweeps, not a single load point:

```
latency
   ^
   |                         /
SLO|------------------------X  saturation knee
   |                    ___/
   |              _____/
   +---------------------------------> offered load
                     sustainable capacity
```

At each point collect throughput, latency distribution, CPU, memory, queues,
downstream saturation, and errors. Capacity is the maximum sustainable load
within the SLO and stability constraints, not the highest momentary throughput.

Use an arrival model that matches the claim. A closed-loop load generator can
back off when responses slow and hide coordinated omission; an open-loop
schedule can expose queueing but must have an explicit overload/stop policy.
Preserve histogram counts and resolution rather than averaging percentiles
across instances.

For bursty systems, include recovery: after overload stops, do queues drain and
memory return to policy limits?

## Cost Efficiency

| Metric | Formula |
|--------|---------|
| Requests per core-hour | completed requests / allocated core-hours |
| CPU time per record | process CPU seconds / records |
| Memory density | concurrent useful work / GiB |
| Network efficiency | payload bytes / transferred bytes |
| Cost per million operations | total attributable cost / operations * 1,000,000 |

Cost must include side effects: a CPU optimization that increases egress,
storage, reliability risk, or operational complexity may not reduce total cost.
Use successful useful operations in the denominator, and include required
redundancy, headroom, idle reserve, retries, observability, and data-transfer
cost. Azure VM SKU comparisons need identical workload/SLO, current pricing
context, and enough duration to include throttling/boost behavior.

## Canary and Rollout

```
offline evidence -> small canary -> guarded expansion -> full rollout
                          |                  |
                          +-> auto rollback  +-> compare by version
```

Canary metrics must be comparable. Random routing can still skew by geography,
tenant, or cache state. Use stratification or paired routing where necessary.
Define rollback thresholds before deployment, including correctness and resource
signals, not only latency. Compare absolute outcomes and cohort composition;
aggregate ratios can reverse when the canary receives a different workload mix.

## Old World -> New World Bridge

| Prior art | Rust production practice |
|-----------|--------------------------|
| ETW + PerfView | Same native stack/wait analysis with Rust PDBs |
| Application Insights | Rust OpenTelemetry/exporter or SDK integration |
| CLR counters | Process/runtime-specific metrics; no default GC/JIT counters |
| Capacity lab | Load sweep to SLO saturation knee |
| Azure flighting | Version-labeled canary with guardrails |
| Cost per transaction | Requests/records per core-hour and total cloud cost |

The universal bridge is the control loop: observe outcome, localize limiting
resource, change, canary, and retain rollback.

## Common Confusion Points

- **Metrics without queue time misattribute latency.**
- **Average CPU can hide one saturated worker/core.**
- **Uniform trace sampling can miss rare slow requests.**
- **Closed-loop load can hide coordinated omission.**
- **Telemetry can become a performance problem.**
- **Hosted dashboards do not replace exact artifact/symbol custody.**
- **Maximum throughput is not sustainable capacity.**
- **Lower process CPU does not automatically lower total cloud cost.**
- **Canary cohorts can be workload-skewed.**

## Decision Cheat Sheet

| Need | Evidence |
|------|----------|
| Detect user-visible regression | SLO latency/error distributions by deployment version |
| Explain latency | Trace queue, service, downstream, and write phases |
| Explain CPU | Low-overhead sampled profile with exact symbols |
| Explain memory | RSS plus allocation/live/retention evidence |
| Set capacity | Load sweep to SLO knee with recovery behavior |
| Set autoscaling | Saturation/queue signal with known lead time, not CPU alone |
| Prove cost win | Same SLO/workload, total resource and cloud cost per unit |
| Roll out optimization | Canary with predeclared guardrails and rollback |

## Primary Sources

- OpenTelemetry Rust: https://opentelemetry.io/docs/languages/rust/
- Windows Performance Toolkit: https://learn.microsoft.com/windows-hardware/test/wpt/
- Linux perf: https://perf.wiki.kernel.org/
- Azure Monitor overview: https://learn.microsoft.com/azure/azure-monitor/overview
- Brendan Gregg, USE method: https://www.brendangregg.com/usemethod.html

## Related Guides

- CPU profiles: [03-CPU-PROFILING-AND-FLAME-GRAPHS.md](03-CPU-PROFILING-AND-FLAME-GRAPHS.md)
- Benchmark gates: [13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md](13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md)
- Release decision: [15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md)
