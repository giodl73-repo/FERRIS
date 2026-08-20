---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:processes-services-ipc-networking
kind: guide
module: rust-interop-migration
section: computing-software
title: Processes, Services, IPC, and Networking
status: source-custody
source_custody: partial
current_path: rust-interop-migration/11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md
canonical_path: rust-interop-migration/11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md
backsource_ids: [proof-backfill:rust-interop-migration:11-processes-services-ipc-networking]
concepts: [process boundary, service migration, IPC, networking, sidecar, protocol, backpressure, idempotency]
root_concepts: [process interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Processes, Services, IPC, and Networking

A process boundary is the default migration seam when rollback, allocator
isolation, runtime isolation, or blast-radius control matters more than the last
microsecond. The hard work shifts from pointer safety to protocol and
distributed-systems correctness.

## The Big Picture

```
+============================================================================+
|                     PROCESS-BOUNDARY MIGRATION                             |
+============================================================================+
|  CALLER / OLD SYSTEM                                                       |
|      | request: version, deadline, idempotency key, trace context          |
|      v                                                                     |
|  ROUTER / ADAPTER                                                          |
|  old implementation: authoritative/fallback                               |
|  Rust implementation: shadow/canary/primary                               |
|                    | response/status/telemetry                             |
|                    v                                                       |
|  CONTRACT                                                                  |
|  schema | framing | ordering | auth | backpressure | timeout | retry       |
|                    |                                                       |
|                    v                                                       |
|  DEPLOYMENT                                                                |
|  subprocess | sidecar | local daemon | remote service | queue consumer     |
+============================================================================+
```

## Select the Process Shape

| Shape | Good fit | Watch |
|-------|----------|-------|
| Child process | Desktop/CLI host, simple lifecycle ownership | Stdout/stderr framing, process death, upgrades |
| Sidecar | Same deployment unit, separate runtime/heap | Resource duplication, startup ordering |
| Local daemon | Multiple local clients, shared expensive state | Security boundary, version skew, service manager |
| Remote service | Independent scale/deploy/failure domain | Network partitions, auth, latency, operations |
| Queue consumer | Async workloads, replay, gradual traffic | Idempotency, ordering, poison messages |

Local IPC is still a protocol: named pipes, Unix-domain sockets, loopback TCP,
shared memory, or platform RPC all need framing, access control, versioning, and
lifecycle.

## Protocol Contract

```
  frame =
    magic/version
    message type
    correlation id
    payload length
    payload bytes
    integrity/auth metadata as required

  receiver:
    bound length -> allocate/read -> decode -> validate semantics -> dispatch
```

Bound every length and queue. A length-prefixed protocol without a maximum is an
allocation attack. A request protocol without deadlines and backpressure
transforms overload into cascading failure.

| Contract area | Required decision |
|---------------|-------------------|
| Compatibility | Supported client/server version combinations |
| Timeout | End-to-end deadline, not independent expanding timeouts |
| Retry | Which errors are retryable; idempotency key semantics |
| Ordering | Per-key, per-connection, global, or none |
| Backpressure | Queue bounds, rejection status, load shedding |
| Authentication | Peer identity, transport security, local ACLs |
| Observability | Trace propagation, stable metrics, structured error codes |
| Shutdown | Drain, cancel, checkpoint, and forced termination behavior |

## Child Process Example

```rust
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
};

fn health() -> io::Result<String> {
    let mut child = Command::new("rust-worker")
        .arg("--oneshot")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take()
        .ok_or_else(|| io::Error::other("worker stdin unavailable"))?;
    writeln!(stdin, "{{\"version\":1,\"op\":\"health\"}}")?;
    drop(stdin);

    let stdout = child.stdout.take()
        .ok_or_else(|| io::Error::other("worker stdout unavailable"))?;
    let mut reader = BufReader::new(stdout);
    let mut reply = String::new();
    let bytes = reader.by_ref().take(8_193).read_line(&mut reply)?;
    if bytes == 0 || bytes > 8_192 || !reply.ends_with('\n') {
        let _ = child.kill();
        let _ = child.wait();
        return Err(io::Error::other("invalid or oversized worker reply"));
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::other("worker failed"));
    }
    Ok(reply)
}
```

This bounded newline-framed JSON sketch assumes a one-shot worker and a
serializer that emits one compact JSON value per line; embedded newlines must
remain escaped. Production code also needs a startup/read deadline (the blocking
standard-library calls above have none), stderr policy, process supervision,
authentication if the channel is exposed, and a protocol version handshake.

## Shared Memory Is Not a Free Process Boundary

Shared memory removes copies by reintroducing memory-layout, allocator, atomic,
and lifetime coupling:

- define a stable C-like or serialized layout, never Rust layout;
- establish ownership and reclamation for slots/buffers;
- use cross-process synchronization supported on every target;
- include crash recovery for an owner dying mid-update;
- version the region and reject incompatible readers;
- validate cache-line alignment and atomic support.

Use shared memory only after a measured transport bottleneck and with a protocol
as rigorous as a storage engine.

## Rollout Mechanics

| Mode | Purpose |
|------|---------|
| Record/replay | Establish compatibility on representative corpus |
| Shadow | Send a copy to Rust; old path remains authoritative |
| Compare | Normalize and diff outputs/side effects |
| Canary | Rust authoritative for bounded traffic |
| Progressive | Increase by tenant/key/region with automatic gates |
| Fallback | Route eligible failures to old path only when semantics are safe |

Retries and fallback can duplicate side effects. An operation must be idempotent
or carry a deduplication key before automatic replay to another implementation.

## Boundary Hazard Register

| Hazard | Process boundary rule |
|--------|-----------------------|
| ABI | Machine ABI is replaced by a specified protocol/schema; shared memory still must not persist Rust ABI or trait objects. |
| Allocator | Heaps are isolated; bound message sizes; shared buffers require explicit owner/reclaimer. |
| Panic/unwind | Panic becomes process failure or protocol error; supervisor handles restart/quarantine, not cross-frame unwind. |
| Lifetime | Requests, streams, leases, shared regions, and subprocess handles have explicit close/expiry semantics. |
| Threading | Define concurrency, ordering, backpressure, worker pools, callback/stream delivery, and shutdown races. |
| Target | Validate transports, filesystem/socket semantics, TLS stack, service manager, containers, and architectures. |
| Packaging | Ship executable/image, config/schema, service definitions, certificates, health checks, symbols, and upgrade/rollback logic. |

## Old World -> New World Bridge

| Established model | Rust migration use |
|-------------------|--------------------|
| Out-of-proc COM / RPC | Runtime and allocator isolation behind an interface contract |
| Sidecar proxy | Co-deployed Rust capability with independent process failure |
| Batch job | Queue-driven Rust worker with checkpoint/replay |
| Blue/green service | Old/Rust implementations behind routable traffic |
| Circuit breaker | Bound failure propagation during canary |
| Transactional outbox | Reliable event handoff without dual-commit fantasy |

## Common Confusion Points

- **"Local IPC cannot fail like a network."** Processes crash, pipes fill,
  permissions change, peers hang, and versions skew.
- **"Retry fixes transient errors."** It can amplify overload and duplicate
  non-idempotent effects.
- **"Shadow traffic is harmless."** It consumes resources and can cause side
  effects unless explicitly suppressed.
- **"Shared memory is zero-copy RPC."** It is a concurrent storage protocol.
- **"Health means process is alive."** Readiness must prove dependencies,
  protocol compatibility, and capacity for useful work.
- **"A panic is contained because it is another process."** The crash is
  contained, but restart storms, corrupt external state, and repeated poison
  inputs still need policy.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Strong rollback and crash isolation | Sidecar/service/process boundary |
| One host owns lifecycle | Child process with version handshake and supervision |
| Multiple local clients | Authenticated local daemon |
| Async replayable migration | Queue consumer with idempotency |
| Very low latency after measurement | Local socket/pipe first; shared memory only with full protocol |
| Safe canary | Deterministic routing key, bounded traffic, automated gates |
| Automatic fallback | Only for idempotent or deduplicated operations |

## Primary Sources

- gRPC concepts: https://grpc.io/docs/what-is-grpc/core-concepts/
- Protocol Buffers updating guidance: https://protobuf.dev/programming-guides/proto3/#updating
- OpenTelemetry context propagation: https://opentelemetry.io/docs/concepts/context-propagation/
- Microsoft REST API Guidelines: https://github.com/microsoft/api-guidelines
- The Twelve-Factor App, processes: https://12factor.net/processes

## Related Guides

- Previous: [10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md](10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md)
- Next: [12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md](12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md)
- Boundary comparison: [02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md](02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md)
