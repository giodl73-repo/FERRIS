---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:structured-logging-tracing
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Structured Logging and Tracing
status: source-custody
source_custody: partial
current_path: rust-production-engineering/02-STRUCTURED-LOGGING-AND-TRACING.md
canonical_path: rust-production-engineering/02-STRUCTURED-LOGGING-AND-TRACING.md
backsource_ids: [mdloom-backfill:rust-production-engineering:02-structured-logging-tracing]
concepts: [structured logging, distributed tracing, spans, correlation, context propagation, sampling, redaction]
root_concepts: [observability]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Structured Logging and Tracing

## The Big Picture

Logs record discrete facts. Traces connect work into causal paths. Both should
be structured events with stable fields, not prose that an operator must parse
during an incident. In concurrent Rust, context propagation deserves deliberate
design because work may move between tasks, threads, queues, and processes.

```
+============================================================================+
|                        REQUEST EVIDENCE FLOW                               |
|                                                                            |
| inbound context                                                            |
| trace_id, parent_id, request_id                                            |
|          |                                                                 |
|          v                                                                 |
|  SERVER SPAN ---------------------------------------------------+          |
|    fields: route, method, release, tenant_class                 |          |
|      |                                                         |           |
|      +--> event: request.validated                              |          |
|      +--> DB SPAN: operation, system, outcome                   |          |
|      +--> RPC SPAN: peer, operation, retry_attempt              |          |
|                                                                |           |
| outbound context <----------------------------------------------+          |
|          |                                                                 |
|          v                                                                 |
| logs + spans --> processor/exporter --> storage/query --> alert/debug      |
+============================================================================+
```

A trace ID is a join key, not a substitute for meaningful fields. An operator
should be able to query failures by release, operation, dependency, and outcome
without text search.

## Event and Span Design

| Surface | Use it for | Avoid |
|---|---|---|
| Event | state transition, decision, failure, notable fact | per-item noise in hot loops |
| Span | duration and context of one operation | treating every function call as a span |
| Field | stable query dimension or measured value | unbounded payloads and secrets |
| Message | concise human explanation | encoding the only copy of key data |

Prefer low-cardinality names (`http.server.request`) plus fields over dynamic
names (`request_for_customer_318742`). Record errors on the span that owns the
failed operation and set a machine-queryable outcome.

## Executable `tracing` Example

```toml
# Cargo.toml
[package]
name = "tracing-example"
version = "0.1.0"
edition = "2021"

[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

```rust
use tracing::{info, info_span, instrument};
use tracing_subscriber::EnvFilter;

#[instrument(skip(card_token), fields(payment.system = "example"))]
fn authorize(order_id: u64, card_token: &str) -> Result<(), &'static str> {
    let _ = card_token; // deliberately never recorded
    info!(outcome = "approved", "authorization completed");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .try_init()?;

    let request = info_span!(
        "http.server.request",
        http.method = "POST",
        http.route = "/orders/{id}",
        request.id = "req-42",
        release = env!("CARGO_PKG_VERSION")
    );

    let _guard = request.enter();
    authorize(42, "not-for-telemetry")?;
    info!(outcome = "success", "request completed");
    Ok(())
}
```

The example is synchronous and runs without an async executor. For asynchronous
work, instrument the future with `tracing::Instrument::instrument` and let the
selected runtime poll it; do not hold an entered-span guard across `.await`.

Run the initialization and synchronous function path with:

```bash
cargo generate-lockfile
RUST_LOG=info cargo run --locked
```

On PowerShell, run `cargo generate-lockfile`, then
`$env:RUST_LOG = "info"; cargo run --locked`.

## Context Propagation

```
same function stack     span guard / instrumentation
async task              instrument the future; preserve task context
new OS thread           explicitly enter or attach the parent span
queue/message           serialize approved trace context in metadata
outbound RPC            inject standard propagation headers
```

Do not serialize an in-process span object. Across process boundaries, use a
standard wire format such as W3C Trace Context. Treat inbound baggage as
untrusted, size-limited input; copying arbitrary user strings into every
downstream request creates cost and disclosure risk.

## Sampling and Volume

Head sampling decides before the trace completes; tail sampling decides after
observing outcomes but requires buffering and a collector. Neither changes the
need for metrics: sampled traces cannot reliably count all events.

| Signal need | Better mechanism |
|---|---|
| exact request/error rate | metric counter |
| one failing request path | trace |
| detailed state transition | structured event |
| security audit record | dedicated durable audit stream |

Logs are often billed by bytes. Bound event size, suppress routine success
noise, and make debug levels dynamically controllable with an expiry and audit
trail.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | `tracing`, `log`, OpenTelemetry SDKs, framework adapters |
| Runtime | span/task propagation, async exporter scheduling, blocking policy |
| Platform | collector, storage, retention, sampling, access control |

`tracing` is an application instrumentation facade. OpenTelemetry defines a
cross-language telemetry model and protocols. A vendor backend may ingest that
protocol, but vendor-specific fields should not become the only application
semantics.

## Old World -> New World Bridge

The universal bridge is from **printf debugging** to **event-sourced
diagnostics**: preserve stable dimensions and causal identity at emission time
rather than reverse-parsing strings later. Distributed tracing extends the call
stack across asynchronous and network boundaries.

.NET `Activity`/`ILogger` scopes and Java logging MDC solve similar propagation
problems. Azure Monitor is one possible backend; the application-level contract
should remain useful with another OpenTelemetry-compatible collector or local
JSON output.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Structured event | one decision or state transition matters |
| Span | an operation has duration, children, or causal context |
| Metric instead | you need complete aggregation or alerting |
| Head sampling | low overhead and uniform early decision are more important |
| Tail sampling | rare errors/latency outliers must be retained |
| Local JSON | supervisor/agent owns transport and rotation |
| Direct exporter | process can tolerate exporter dependencies and backpressure policy is explicit |
| Dedicated audit stream | records have legal/security retention and integrity requirements |

## Common Confusion Points

- **A trace is not "all logs with the same request ID."** Parent-child timing
  and propagation semantics are the point.
- **Entering a span across `.await` can be wrong.** Instrument the future; a
  guard held across suspension may associate unrelated work on the thread.
- **Error text is not an error taxonomy.** Record stable kind/outcome fields.
- **Debug logging is not free when disabled incorrectly.** Avoid constructing
  expensive values before the level check.
- **Telemetry is an exfiltration path.** Redact before emission and control who
  can raise verbosity.

## Primary Sources

- `tracing`: https://docs.rs/tracing/
- `tracing-subscriber`: https://docs.rs/tracing-subscriber/
- OpenTelemetry tracing specification: https://opentelemetry.io/docs/specs/otel/trace/
- W3C Trace Context: https://www.w3.org/TR/trace-context/
- OWASP Logging Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html

## Related Guides

- Previous: [01-CONFIGURATION-ENVIRONMENTS-AND-SECRETS.md](01-CONFIGURATION-ENVIRONMENTS-AND-SECRETS.md)
- Next: [03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md](03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md)
- Error evidence: [04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md](04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md)
- Incident use: [13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md](13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md)
