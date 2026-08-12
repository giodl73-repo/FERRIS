---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:http-and-api-service
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: HTTP and API Service Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/03-HTTP-AND-API-SERVICE.md
canonical_path: rust-application-blueprints/03-HTTP-AND-API-SERVICE.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:03-http-and-api-service]
concepts: [http service, api contract, request lifecycle, backpressure, graceful shutdown, database migration]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# HTTP and API Service Blueprint

## The Big Picture

```
+============================================================================+
| client -> proxy/load balancer -> connection and request admission          |
+---------------------------------------+------------------------------------+
                                        v
+----------------------------------------------------------------------------+
| protocol shell                                                             |
| route | decode | authenticate | deadline | correlation | response mapping  |
+---------------------------------------+------------------------------------+
                                        v
+----------------------------------------------------------------------------+
| application use case -> domain policy -> ports                             |
+----------------------+----------------------+------------------------------+
                       v                      v
                 data adapter          remote-service adapter
                       |                      |
                       +-----------+----------+
                                   v
                    response + telemetry + durable effects
```

An HTTP service owns a synchronous protocol boundary. That does not imply that
all business work completes synchronously; it means the response must precisely
state whether work completed, was rejected, or was accepted under a separately
observable identity.

## Workspace Layout

```
catalog-service/
|-- Cargo.toml
|-- crates/
|   |-- catalog-domain/
|   |-- catalog-application/
|   |-- catalog-http/           # route and representation contract
|   |-- catalog-store/
|   `-- catalog-client/         # optional generated/handwritten client
|-- apps/
|   `-- catalog-server/
|       `-- src/main.rs
|-- migrations/
`-- tests/
    |-- api-contract/
    `-- service-integration/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

Framework handlers should translate protocol types into application commands.
Keep framework extractors, response builders, and middleware errors out of the
domain and application crates.

## Request Lifecycle and Authority

```
accept connection/request
        |
        v
establish request deadline and identity
        |
        v
authenticate ---> authorize in application context
        |
        v
decode + validate representation
        |
        v
execute use case ---> commit effects
        |
        v
map semantic outcome to protocol response
```

| Concern | Owning layer |
|---------|--------------|
| TLS termination and edge limits | proxy/platform or server entrypoint |
| Authentication evidence | protocol/security adapter |
| Authorization decision | application/domain policy |
| HTTP status and headers | HTTP representation layer |
| Transaction and concurrency rule | application plus store port |
| Database schema | service data owner |
| Deadline propagation | entrypoint establishes; adapters respect |

Only a configured trusted proxy may supply client identity, scheme, host, or
forwarded-address evidence. Bound both encoded and decoded body size, header
count/size, parse time, decompression ratio, and streaming buffers. Transport
authentication, CORS, and CSRF controls are protocol/security concerns;
authorization and tenant/data scope remain application authority.

Return errors by semantics, not by internal exception shape. A conflict, absent
resource, invalid precondition, dependency outage, and defect need distinct
application categories before they become status codes.

## Load, Backpressure, and Shutdown

| Pressure point | Required bound |
|----------------|----------------|
| Connections | listener/platform limit |
| In-flight requests | concurrency budget |
| Request body | byte and time limit |
| Database work | pool and query deadline |
| Remote calls | deadline, concurrency, retry budget |
| Response buffering | stream or bounded body |

```
shutdown requested
      |
      +--> stop new admission
      +--> mark unready at routing boundary
      +--> drain in-flight work until deadline
      +--> cancel work that preserves invariants
      `--> close adapters and exit observably
```

Retries are dangerous inside request paths. Retry only operations that are
idempotent under the same deadline and attempt budget; never allow nested client,
service, and database retries to multiply invisibly.

## Data Evolution, Testing, and Rollback

```
contract tests -> handler tests -> real-store integration
       -> packaged server smoke -> rollout/drain exercise
```

Minimum executable evidence:

```text
cargo test --workspace --all-targets
cargo run -p catalog-server
# probe health/readiness and one representative API scenario
```

The exact probe tool and address belong to repository documentation. Avoid
claiming a fixed framework default.

Use expand/contract for durable schemas:

```
release A: old reader/writer
release B: add new shape; read old+new; write compatible form
release C: backfill/reconcile
release D: stop old writes
release E: remove old shape after rollback window
```

| Change | Rollback rule |
|--------|---------------|
| Handler only | old binary accepts current config and data |
| API additive | old clients ignore or tolerate new fields per format contract |
| API breaking | new version/route and explicit retirement window |
| Database | old code works throughout rollback window |
| Accepted async work | job identity remains observable across deployment |

Run schema migration under one named release authority rather than letting every
replica race startup migration. Retire an endpoint only after supported clients
have migrated, observed traffic is below the declared threshold, credentials
and routes are revoked, and rollback no longer requires the old representation.

## Universal Bridge First

The primary bridge is a protocol adapter around use cases: HTTP is one
representation and transport, not the domain model. This is the same separation
used in RPC systems, message handlers, and GUI controllers.

Supplementally, ASP.NET middleware/controller/application-service layering maps
closely. Rust frameworks differ in types and runtime integration, but the stable
decision is still where authentication, authorization, transaction, and
representation ownership sit.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Simple synchronous CRUD/use case | one service binary with HTTP adapter and owned store |
| Work exceeds request deadline | return accepted job identity; process via worker [04] |
| Multiple independent data owners | distributed application [13] |
| Streaming response/request | explicit flow control and cancellation contract |
| Public client ecosystem | versioned representation plus SDK [08] if useful |
| Internal admin operation | consider CLI [02] rather than an undocumented endpoint |
| Zero-downtime data change | expand/contract plus compatibility tests |

## Common Confusion Points

- **Health and readiness are different.** Liveness asks whether restart may
  help; readiness asks whether new traffic should be admitted.
- **HTTP success is not necessarily business completion.** `202`-style
  acceptance needs a durable job identity and observation path.
- **Framework state is not domain state.** Connection pools and routers belong
  to process composition.
- **A request timeout does not automatically cancel external effects.** Define
  cancellation and idempotency across each adapter.
- **OpenAPI or another schema is not the whole contract.** Authorization,
  idempotency, ordering, rate limits, and compatibility policy remain semantic.
- **Forwarded headers are not self-authenticating.** Trust them only from the
  configured edge and reject ambiguous authority/host information.
- **Splitting handlers into services is not decomposition.** Data authority and
  independent lifecycle are the stronger boundaries.

## Primary Sources

- Rust asynchronous programming book: https://rust-lang.github.io/async-book/
- Rust `Future`: https://doc.rust-lang.org/std/future/trait.Future.html
- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- HTTP Semantics (IETF): https://www.rfc-editor.org/rfc/rfc9110

## Related Guides

- Worker handoff: [04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md](04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md)
- Distributed boundary: [13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md](13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md)
