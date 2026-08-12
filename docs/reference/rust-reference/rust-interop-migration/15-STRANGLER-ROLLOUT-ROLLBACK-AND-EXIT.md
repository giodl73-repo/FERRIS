---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:strangler-rollout-rollback-exit
kind: guide
module: rust-interop-migration
section: computing-software
title: Strangler Rollout, Rollback, Observability, and Exit
status: source-custody
source_custody: partial
current_path: rust-interop-migration/15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md
canonical_path: rust-interop-migration/15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md
backsource_ids: [mdloom-backfill:rust-interop-migration:15-strangler-rollout-rollback-exit]
concepts: [strangler migration, progressive rollout, rollback, observability, shadow traffic, semantic diff, exit criteria]
root_concepts: [strangler migration]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Strangler Rollout, Rollback, Observability, and Exit

A migration is complete when the old implementation and temporary bridge can be
deleted without losing a supported consumer, rollback path, diagnostic signal,
or state repair capability. "Rust handles production traffic" is a midpoint,
not an exit criterion.

## The Big Picture

```
+============================================================================+
|                   STRANGLER CONTROL AND EVIDENCE LOOP                      |
+============================================================================+
|  BASELINE -> RECORD/REPLAY -> SHADOW -> CANARY -> PROGRESSIVE -> DEFAULT   |
|                                    |                                       |
|                                    v                                       |
|  EVIDENCE                                                                  |
|  correctness diff | latency/resources | errors | crashes | data | support  |
|                                    |                                       |
|  GATE RESULT: pass -> advance traffic; fail -> rollback/reroute            |
|                                                    |                       |
|                                                    v                       |
|                                             diagnose and repair            |
|                                                                            |
|  EXIT: 100% stable + rollback drill + no old callers + compatible state    |
|        + support ready + bridge deletion PR + retained recovery evidence   |
+============================================================================+
```

## Phase Gates

| Phase | Rust authority | Required gate |
|-------|----------------|---------------|
| Baseline | None | Existing SLOs, defect signatures, corpus, and cost known |
| Record/replay | Offline | Representative inputs replay with normalized comparison |
| Shadow | None | No unintended side effects; divergence classified |
| Canary | Small deterministic cohort | Error/correctness/resource gates and rapid rollback |
| Progressive | Growing cohorts | Stable trends across time, tenants, targets, and load |
| Default | Most/all eligible traffic | Support readiness and old-path fallback policy |
| Exit | Rust only | Old consumers zero, rollback state resolved, bridge/old code removed |

Route by a stable key such as tenant, account, document, or region so repeated
operations do not randomly alternate implementations. Keep cohort identity in
telemetry.

## Compare Semantics, Not Bytes

```
  old output ---- normalize ----+
                                +--> semantic comparator --> divergence class
  Rust output --- normalize ----+

  classes:
    equivalent
    allowed improvement/change
    legacy defect intentionally preserved for compatibility
    Rust defect
    test/input ambiguity
```

Normalization can remove timestamps, ordering where semantically irrelevant,
generated IDs, floating tolerance, or formatting. The comparator itself is a
versioned test asset. Never dismiss divergence until its class and customer
effect are known.

## Observability Contract

| Signal | Migration dimensions |
|--------|----------------------|
| Request metrics | Implementation, contract version, target, cohort, result class |
| Latency | Boundary time, core time, queue time, serialization/copy time |
| Resources | CPU, memory, allocation, handles/fds, threads/tasks, native load failures |
| Correctness | Match/divergence counts by normalized category |
| Failure | Stable foreign status, Rust error chain internally, panic/crash signature |
| Data | Write version, lag, reconciliation/backfill checkpoints |
| Deployment | Package/native build ID, toolchain, symbol/source correlation |

Keep label cardinality bounded. Put request/tenant identifiers in traces/logs,
not metric labels. Preserve the ability to correlate a host exception with a
native build and Rust panic/crash report.

## Rollback Levels

| Level | Mechanism | Precondition |
|-------|-----------|--------------|
| Traffic rollback | Router/feature switch sends work to old path | Old path still deployed and data-compatible |
| Package rollback | Restore prior host/native package | Loader/cache and schema remain compatible |
| Process rollback | Start old service/image | Protocol and state compatibility |
| Data rollback | Reverse migration/restore/replay | Tested recovery, acceptable loss window |
| Forward fix | Patch Rust while keeping traffic bounded | Safer than reverting incompatible state |

Rollback is not always the right action. If new writes cannot be read by the old
binary, a package revert can deepen the incident. The release gate must identify
the last reversible point and the forward-repair plan beyond it.

## Automated Gates

Example policy:

```text
advance only if, for 60 minutes:
  correctness divergence classified-as-defect == 0
  error rate delta <= 0.1 percentage point
  p99 latency <= old p99 * 1.05
  memory per operation <= agreed budget
  panic/crash/native-load failures == 0
  data reconciliation lag <= 5 minutes

rollback automatically if:
  panic/crash/native-load failure > 0
  correctness defect > 0 for protected operation
  SLO burn exceeds canary budget
```

Numbers are product-specific; the structure is not. Include hold periods long
enough to cover background work, cache cycles, target diversity, and rare
inputs.

## Exit Criteria

```
  [ ] all supported targets and host versions at intended traffic
  [ ] rollback drill observed, including native package selection
  [ ] old reader/writer compatibility window closed deliberately
  [ ] no runtime calls, dynamic loads, imports, registrations, or operators
      depend on old implementation
  [ ] support runbook, symbols, dashboards, ownership, and patch path active
  [ ] bridge deletion and old-code deletion reviewed together
  [ ] retained corpus/replay tests prevent semantic regression
  [ ] licenses, packages, config, schemas, and deployment assets removed
```

Remove the bridge when it has no remaining consumer. A permanent adapter may be
a legitimate product API, but then rename and support it as such rather than
calling it migration scaffolding.

## Boundary Hazard Register

| Hazard | Rollout/exit evidence |
|--------|-----------------------|
| ABI | Compatibility tests cover old/new host and native versions; no consumer relies on Rust ABI or trait objects. |
| Allocator | Leak/double-free/handle telemetry and stress tests are clean through rollback/unload paths. |
| Panic/unwind | Panic, foreign exception, callback, and process-crash paths produce bounded failure and actionable correlation. |
| Lifetime | Long-lived handles, buffers, streams, subscriptions, and callbacks drain correctly before switch/unload. |
| Threading | Race, reentrancy, cancellation, shutdown, and load tests cover every supported host/runtime model. |
| Target | Canary includes the actual OS/arch/libc/CRT/runtime/package matrix, not one representative target. |
| Packaging | Upgrade and rollback from clean hosts prove artifact selection, loader dependencies, signing, symbols, and retained prior versions. |

## Old World -> New World Bridge

| Established rollout practice | Rust migration use |
|------------------------------|--------------------|
| Blue/green | Old and Rust implementations remain routable |
| Dark launch | Shadow Rust without authority |
| Golden master testing | Record/replay plus semantic normalization |
| Database expand/contract | Preserve binary rollback while state evolves |
| VSTS/Azure DevOps release gates | Automated correctness/SLO/package gates |
| Feature flag retirement | Delete migration routing and old path after evidence |

## Common Confusion Points

- **"Shadow success proves production readiness."** It does not prove side
  effects, backpressure, authoritative writes, or rollback.
- **"No errors means equivalent."** Silent semantic divergence needs explicit
  comparison.
- **"Rollback is a feature flag."** It also requires old artifacts, compatible
  data, loader behavior, and practiced operations.
- **"Canary one platform, then ship all."** Native ABI and packaging failures
  are target-specific.
- **"Keep the old path forever for safety."** An untested fallback decays and
  doubles support burden. Either exercise it or delete it.
- **"Exit means deleting source files."** Remove packages, registrations,
  schemas, flags, dashboards, runbooks, and ownership ambiguity too.

## Decision Cheat Sheet

| Situation | Action |
|-----------|--------|
| Output is deterministic/comparable | Record/replay then shadow with semantic diff |
| Side effects cannot be safely shadowed | Use dry-run validation or canary directly with bounded cohort |
| State changes remain old-readable | Traffic/package rollback is viable |
| State is no longer old-readable | Prefer forward repair or tested reverse migration |
| Native load failure appears | Stop rollout; validate package/target closure before code diagnosis |
| 100 percent traffic but old callers remain | Do not exit; inventory and migrate consumers |
| Bridge has become public API | Declare support/version policy; stop calling it temporary |

## Primary Sources

- Martin Fowler, Strangler Fig Application: https://martinfowler.com/bliki/StranglerFigApplication.html
- Google SRE Workbook, canarying releases: https://sre.google/workbook/canarying-releases/
- OpenTelemetry semantic conventions: https://opentelemetry.io/docs/specs/semconv/
- Microsoft safe deployment practices: https://azure.microsoft.com/resources/cloud-computing-dictionary/what-is-safe-deployment

## Related Guides

- Previous: [14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md](14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md)
- Start again at seam selection: [01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md](01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md)
- Data rollback: [10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md](10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md)
