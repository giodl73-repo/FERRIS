---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:containers-kubernetes-systemd-windows-services
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Containers, Kubernetes, systemd, and Windows Services
status: source-custody
source_custody: partial
current_path: rust-production-engineering/10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md
canonical_path: rust-production-engineering/10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md
backsource_ids: [mdloom-backfill:rust-production-engineering:10-containers-kubernetes-systemd-windows-services]
concepts: [containers, kubernetes, systemd, windows services, process supervision, resource limits, deployment platforms]
root_concepts: [deployment platforms]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Containers, Kubernetes, systemd, and Windows Services

## The Big Picture

Every production platform must arrange roughly the same capabilities:
installation, identity, configuration, networking, resource control,
supervision, health, logs, upgrades, and rollback. Containers, Kubernetes,
systemd, and Windows services compose those capabilities differently.

```
+============================================================================+
|                          HOSTING CONTRACT                                  |
|                                                                            |
|  artifact + config + identity                                              |
|                |                                                           |
|                v                                                           |
|  supervisor --> process --> sockets/files/dependencies                     |
|      |             |                                                       |
|      |             +--> stdout/stderr or platform event channel            |
|      +--> start/stop/restart/health/resource limits                        |
|                |                                                           |
|                v                                                           |
|          rollout + rollback + operator access                              |
|                                                                            |
|  implementations: process manager | container | orchestrator | SCM         |
+============================================================================+
```

Kubernetes is valuable when its scheduling and control-plane capabilities solve
real fleet problems. It is not a prerequisite for a reliable Rust service.

## Platform Comparison

| Surface | Container runtime | Kubernetes | systemd | Windows SCM |
|---|---|---|---|---|
| Unit | container/process | pod/workload | service unit | service |
| Supervision | restart policy | controllers + kubelet | service manager | SCM recovery |
| Stop | signal then force | pod termination flow | configured signal/timeout | service control |
| Health | runtime-specific | startup/liveness/readiness | watchdog/exit policy | service status |
| Config/secrets | mounts/env | objects + external stores | files/credentials/env | registry/files/secret stores |
| Resources | cgroups/job objects | requests/limits | cgroups directives | job objects/policy |
| Rollout | external tool | Deployment/StatefulSet/etc. | package + orchestration | installer + orchestration |

The table compares common capabilities, not exact equivalence. Windows service
controls are not Unix signals; Kubernetes readiness affects routing; systemd
readiness may use `sd_notify`; each requires an adapter.

## Executable Container Example

Scope: a Linux-targeted Cargo binary named `orders`, dynamically linked against
glibc, with no runtime shell requirement.

```dockerfile
# Illustrative known toolchain, not a current-version recommendation.
# Release pipelines should replace this tag with an approved image digest.
FROM rust:1.85-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN useradd --uid 10001 --no-create-home --shell /usr/sbin/nologin app
COPY --from=build /src/target/release/orders /usr/local/bin/orders
USER 10001
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/orders"]
```

Build and inspect:

```bash
docker build --pull -t orders:local .
docker run --rm --read-only --cap-drop=ALL -p 127.0.0.1:8080:8080 orders:local
```

Pin base images by digest in a release pipeline. If the process launches child
processes, it must reap them or use a suitable init. Ensure the Rust process
receives the stop signal directly; shell-form entrypoints can interfere.

## Kubernetes Is a Control-Plane Choice

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: orders
spec:
  replicas: 3
  selector:
    matchLabels: { app: orders }
  template:
    metadata:
      labels: { app: orders }
    spec:
      terminationGracePeriodSeconds: 30
      containers:
        - name: orders
          image: registry.example/orders@sha256:REPLACE_WITH_DIGEST
          ports:
            - { name: http, containerPort: 8080 }
          readinessProbe:
            httpGet: { path: /ready, port: http }
          startupProbe:
            httpGet: { path: /live, port: http }
            periodSeconds: 2
            failureThreshold: 30
          livenessProbe:
            httpGet: { path: /live, port: http }
          resources:
            requests: { cpu: "250m", memory: "128Mi" }
            limits: { memory: "256Mi" }
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
            runAsNonRoot: true
            capabilities: { drop: ["ALL"] }
```

The digest placeholder must be replaced by CI. The startup probe suppresses
readiness and liveness probes until it succeeds; use it only when `/live`
accurately distinguishes startup progress from a wedged process. CPU limits can
create throttling and tail latency; memory limits can lead to termination.
Requests and limits must come from measurement, not copied templates.

On Pod deletion, EndpointSlice readiness transitions and routing convergence are
not instantaneous for every client or existing connection. Any `preStop` hook
and application drain share the Pod's termination grace budget; the process must
still handle the stop signal and remain correct when the grace period ends in
forced termination.

## systemd Service

```ini
[Unit]
Description=Orders service
After=network-online.target
Wants=network-online.target
StartLimitIntervalSec=60s
StartLimitBurst=3

[Service]
Type=exec
ExecStart=/opt/orders/orders --config /etc/orders/config.toml
User=orders
Group=orders
Restart=on-failure
RestartSec=5s
TimeoutStopSec=30s
NoNewPrivileges=yes
ProtectSystem=strict
ReadWritePaths=/var/lib/orders

[Install]
WantedBy=multi-user.target
```

`network-online.target` does not prove a remote dependency is healthy. The
application still needs bounded connection and readiness policy. `Type=exec`
reports process-execution failures more accurately than `Type=simple`, but it
does not report application readiness. Use `Type=notify` only when the process
implements the `sd_notify` protocol and sends `READY=1` after its own startup
contract is satisfied. Restart rate limits prevent a tight crash loop but do
not replace alerting or fault repair.

## Windows Service Contract

A Windows service process must connect to the Service Control Manager,
register a control handler, report status transitions, and handle stop/shutdown
controls. Merely running a console binary through `sc.exe create` does not add
that protocol. Use a maintained Windows service crate or a small service host
that launches the core application logic.

```
SCM Start --> service_main --> StartPending --> Running
SCM Stop  --> control handler --> StopPending --> drain --> Stopped
```

The control handler should report `StopPending`, hand lengthy drain work to
normal execution or a worker thread, and return promptly; blocking the control
dispatcher can delay controls for other services in the process. Continue
reporting valid status/checkpoint progress as required by the host contract and
finish with `Stopped`.

Keep the business/service core independent of SCM calls so it can run in a
console harness for tests and diagnostics.

## Resource and Filesystem Assumptions

| Assumption | Make explicit |
|---|---|
| Writable paths | exact directories and durability |
| User identity | UID/GID or service account and permissions |
| Network | bind address, outbound destinations, DNS behavior |
| Clock | monotonic vs wall clock; synchronization expectations |
| Limits | CPU, memory, descriptors, process/thread count |
| Logs | stdout/stderr, journal, Event Log, or file ownership |

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | signal/service adapters, health server, platform API wrappers |
| Runtime | no inherent requirement; sync and async services fit all four |
| Platform | container runtime, Kubernetes, systemd, Windows SCM, VM tooling |

Do not let platform adapters infect domain code. A small lifecycle interface
can translate platform start/stop/readiness into application state.

## Old World -> New World Bridge

The universal bridge is from **application host** to **composed process
contract**. IIS application pools, Windows services, Unix daemons, containers,
and pods all supervise a process, but differ in routing and rollout ownership.

Azure Kubernetes Service and Azure Container Apps are supplemental platform
implementations. They do not change the Rust binary's need for correct signal,
health, identity, and resource behavior.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Plain supervised process | host count and rollout needs are modest |
| systemd | Linux host integration, hardening, and journaling are sufficient |
| Windows service | native SCM lifecycle and Windows operations are required |
| Container | immutable filesystem/process packaging improves delivery |
| Kubernetes | fleet scheduling, declarative rollout, and control-plane APIs justify complexity |
| Sidecar/proxy | cross-cutting capability needs independent lifecycle and cost is accepted |
| Platform identity | short-lived workload credentials are available |

## Common Confusion Points

- **A container is not a VM.** It shares the host kernel.
- **Kubernetes liveness is not an alert.** It triggers restart.
- **`latest` is not a release identity.** Deploy immutable digests.
- **A read-only root still needs explicit writable state paths.**
- **Windows services do not use Unix signal semantics.**
- **Resource limits can change latency and failure mode.** Test under them.

## Primary Sources

- OCI runtime specification: https://github.com/opencontainers/runtime-spec
- Dockerfile reference: https://docs.docker.com/reference/dockerfile/
- Kubernetes workload resources: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
- systemd service manual: https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html
- Windows service programs: https://learn.microsoft.com/windows/win32/services/service-programs
- Windows Job Objects: https://learn.microsoft.com/windows/win32/procthread/job-objects

## Related Guides

- Previous: [09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md](09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md)
- Next: [11-CI-CD-AND-PROMOTION.md](11-CI-CD-AND-PROMOTION.md)
- Lifecycle: [05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md](05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md)
- Readiness gates: [15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md](15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md)
