# rust-application-blueprints/ - Status

**17 files (STATUS.md + 16 guides) | Complete | Source-first, partial source custody**

This module is a peer-level reference for choosing and evolving neutral Rust
application shapes. It begins with authority, completion, operational, testing,
and rollback contracts rather than framework selection. The blueprints cover
local tools, services, workers, jobs, pipelines, messaging, libraries, plugins,
Wasm, embedded/edge, Windows integration, distributed systems, and repository
governance.

## Guides

| File | Topic | Status |
|------|-------|--------|
| `00-OVERVIEW.md` | Landscape, selection axes, shared workspace baseline, cross-cutting concerns, and reading paths | done |
| `01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md` | Contract layers, ports/adapters, configuration, identity, telemetry, shutdown, ownership, and rollback | done |
| `02-CLI-AND-DEVELOPER-TOOL.md` | Process protocol, arguments/streams/status, automation, packaging, and mutation recovery | done |
| `03-HTTP-AND-API-SERVICE.md` | Request lifecycle, protocol/application authority, backpressure, graceful shutdown, and schema rollout | done |
| `04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md` | Delivery, acknowledgement, idempotency, bounded concurrency, retry, quarantine, and drain | done |
| `05-SCHEDULED-AND-BATCH-JOB.md` | Logical windows, run identity, overlap, partition checkpoints, publication, and reruns | done |
| `06-DATA-PIPELINE-AND-ETL.md` | Source identity, canonical transforms, quality/rejection, lineage, publication, and reconciliation | done |
| `07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md` | Commands/events, outbox/inbox, ordering, projections, coordination, replay, and evolution | done |
| `08-REUSABLE-LIBRARY-AND-SDK.md` | Public API, SemVer/MSRV, features, dependencies, runtime ownership, release, and recovery | done |
| `09-PLUGIN-AND-EXTENSION-HOST.md` | Extension discovery, ABI/process/Wasm boundaries, capabilities, lifecycle, compatibility, and quarantine | done |
| `10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md` | Browser/WASI/component hosts, interfaces, capabilities, runtime matrices, state, and artifact rollback | done |
| `11-EMBEDDED-AND-EDGE-DEVICE.md` | `no_std` layering, HAL/board/driver ownership, interrupts, resource bounds, HIL, and firmware rollback | done |
| `12-WINDOWS-SERVICE-AND-DESKTOP-NATIVE-INTEGRATION.md` | SCM, desktop dispatch, COM/Win32 boundaries, native ownership, installers, and mixed-version recovery | done |
| `13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md` | Service/data authority, partial failure, consistency, contracts, independent release, and merge signals | done |
| `14-MONOREPO-AND-MULTI-WORKSPACE-APPLICATION.md` | Repository versus workspace boundaries, graph policy, affected testing, release units, and ownership | done |
| `15-BLUEPRINT-SELECTION-AND-EVOLUTION.md` | Weighted selection, reversible cores, split/merge gates, migration, retirement, and exit criteria | done |

## Coverage Notes

Every guide uses `maxim.frontmatter.v1` with module
`rust-application-blueprints`, `status: source-custody`,
`source_custody: partial`, canonical/current paths, a unique guide id, concepts,
root concepts, and a module-consistent `proof-backfill` backsource id.

The editorial spine is:

1. **Authority before mechanism** - each durable record, effect, protocol, and
   recovery action has a named owner.
2. **Completion before concurrency** - a blueprint states what successful work
   means before selecting sync, async, threads, or processes.
3. **Operational boundaries are contracts** - admission, deadlines, retry,
   idempotency, telemetry, shutdown, schema evolution, and recovery are part of
   the architecture.
4. **Evidence follows the boundary** - pure logic, adapters, entrypoints,
   external systems, hosts, and devices receive progressively stronger tests.
5. **Rollback includes state** - binary downgrade is separated from data,
   message, plugin, firmware, installer, and external-effect recovery.
6. **Removal is an architecture operation** - admission, callers, state,
   credentials, routes/subscriptions, artifacts, and retention all have exit
   evidence.

All guides include a Big Picture ASCII diagram first, layered drill-down,
additional useful ASCII structures, decision/comparison tables, a
universal-first bridge with Microsoft/.NET context only where supplemental, a
Decision Cheat Sheet, and Common Confusion Points. Each includes a concrete
Cargo/package/workspace layout and explicit testing and rollback implications.
Diagrams use ASCII-only box characters for PROOF-safe rendering.

No product-specific commitment is made. Ferris is neither an implicit dependency
nor a promised consumer; the module remains a neutral reference for any
repository that independently adopts and validates a blueprint.

## Source and Version Posture

Primary sources are official Rust/Cargo documentation, official project
specifications and documentation for relevant boundaries (HTTP, CloudEvents,
AsyncAPI, WebAssembly, WASI, embedded Rust), and Microsoft platform
documentation where the guide is specifically about Windows.

Claims are stable-contract-first and bounded. Ecosystem/runtime/target support
that changes quickly is described as version-sensitive and requires each
consuming repository to pin and test its exact toolchain, target, host, and
runtime matrix. Target/MSRV example values are bounded to named environments
and must be replaced by repository-tested policy rather than treated as
universal defaults.

Workspace examples use Cargo dependency resolver 3 with edition-2024-era
toolchains. Virtual-workspace and member-manifest responsibilities are kept
separate: binary targets and target-specific dependencies live in member package
manifests, while shared workspace policy remains at the root.

## Review Status

- **Independent cross-review (2026-08-11):** corrected Cargo resolver/layout
  semantics, one mismatched backsource id, PROOF-invalid diagrams, CLI
  atomicity/path/secret boundaries, HTTP proxy/body/migration authority, worker
  lease/quarantine security, privileged batch reruns, ETL/event retention,
  library build-time/global-policy risks, enforceable plugin/Wasm isolation,
  embedded anti-rollback, Windows service/IPC/install security, distributed
  service extraction, monorepo CI trust, and blueprint retirement gates.
- **Reader Path Editor:** overview supplies intent-based routes; every guide
  links to adjacent choices and opens with the authority/execution map.
- **Reference Integrity Auditor:** strong claims are scoped to named boundaries;
  Rust ABI, exactly-once effects, Wasm/WASI support, target support, and rollback
  limits carry explicit caveats.
- **Executable Evidence Auditor:** commands are limited to stable Cargo
  baselines or visibly marked repository-specific placeholders; external-system,
  Windows, runtime, and hardware evidence is named rather than implied.
- **Learner Advocate:** prose is peer-level; concrete trees, ledgers, lifecycle
  flows, failure tables, and decision surfaces anchor the abstractions.

Validation after correction:

- PROOF from the repository manifest, with all 16 numbered guides passed
  explicitly by path and no wildcard or config override:
  **16 files checked, 0 errors, 0 warnings**.
- Frontmatter: 16 unique ids and 16 derived/matching backsource ids; canonical
  and current paths resolve to this module.
- All 20 TOML fences parse. A scratch Cargo metadata check validated resolver 3,
  the CLI member `[[bin]]`, and member-owned target-specific dependencies; the
  scratch tree was removed.
- Relative Markdown links, fence balance, ASCII-only fenced content, final
  newlines, and whitespace checks pass.

No unresolved editorial tags are present. This module is source-first and has
**not** been
run through source-backfill; no navigation, generated artifacts, `REVIEW.md`, or
repository-external paths were changed. PROOF's transient `last-check.json`
cache was removed after validation. It is not claimed as Certified Gold.
