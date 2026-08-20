---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:blueprint-selection-and-evolution
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Blueprint Selection, Evolution, and Exit Criteria
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/15-BLUEPRINT-SELECTION-AND-EVOLUTION.md
canonical_path: rust-application-blueprints/15-BLUEPRINT-SELECTION-AND-EVOLUTION.md
backsource_ids: [proof-backfill:rust-application-blueprints:15-blueprint-selection-and-evolution]
concepts: [architecture selection, blueprint evolution, exit criteria, modular monolith, migration, rollback, decision record]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Blueprint Selection, Evolution, and Exit Criteria

## The Big Picture

```
+============================================================================+
| forces: initiator | state authority | latency | failure | target | owners  |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| choose smallest blueprint that satisfies present evidence                  |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| implement explicit seams + measure pressure                                |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| stay | add adapter/entrypoint | split deployable | merge | retire          |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| exit criteria: contract, ownership, tests, migration, rollback, operations |
+============================================================================+
```

Selection is a constrained optimization, not a taxonomy contest. Begin with the
smallest deployment and authority model that satisfies current requirements.
Preserve semantic seams so measured pressure can drive evolution without
pre-paying every distributed-systems cost.

## Selection Scorecard

Score each candidate 0 (poor) to 3 (strong), then discuss the high-weight rows.
The arithmetic supports judgment; it does not replace it.

| Force | Weight | CLI | HTTP | Worker | Batch | Library | Distributed |
|-------|-------:|----:|-----:|-------:|------:|--------:|------------:|
| Initiator fit | 3 | | | | | | |
| Completion fit | 3 | | | | | | |
| Durable authority | 3 | | | | | | |
| Failure isolation | 2 | | | | | | |
| Independent scaling | 1 | | | | | | |
| Independent release | 2 | | | | | | |
| Target/platform fit | 2 | | | | | | |
| Operability capacity | 3 | | | | | | |
| Rollback feasibility | 3 | | | | | | |

An empty table is intentional: fill it with repository-specific evidence. A
generic score would conceal workload, organization, and platform facts.

## Start from a Reversible Core

```
product/
|-- Cargo.toml
|-- crates/
|   |-- domain/
|   |-- application/
|   `-- adapters/
|-- apps/
|   `-- primary-entry/
|-- tests/
|   `-- scenarios/
`-- ops/
    `-- runbooks/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

This layout permits a second entrypoint without duplicating policy:

```
CLI ----+
        |
HTTP ---+--> application --> domain --> ports --> adapters
        |
worker -+
```

Do not create network boundaries merely because crate seams exist. Crates are
far cheaper than services as authority boundaries, but they still add compile,
dependency, release, and API surface; split them only when the seam earns that
cost.

## Evolution Paths

| From | Pressure | Evolution | Preserve |
|------|----------|-----------|----------|
| CLI | reusable engine | extract library facade | exit/output compatibility |
| HTTP | work exceeds deadline | durable job + worker | request id to job id |
| Worker | many independent facts | owned events/outbox | idempotency and message identity |
| Batch | continuous arrivals | worker/stream pipeline | checkpoint and reconciliation |
| Library | untrusted/independent code | process or Wasm plugin | semantic API and version negotiation |
| Monolith | independent data/team/release | service extraction | source authority and compatibility |
| Services | lockstep/no independence | merge boundary | external contract and data migration |
| Any | platform constraint | Wasm/embedded/Windows adapter | neutral application policy |

```
observe pressure
      |
prove current boundary is insufficient
      |
define target authority and contract
      |
build compatibility bridge
      |
migrate traffic/state
      |
exercise rollback
      |
remove old path after exit criteria
```

## Split and Merge Gates

Split a deployable only when several signals are present:

| Split signal | Evidence |
|--------------|----------|
| Independent data authority | mutation ownership and migration plan |
| Independent lifecycle | separate deployment/rollback is required and testable |
| Distinct failure/scale profile | observed saturation or blast-radius need |
| Stable contract | API/event semantics and compatibility owner |
| Operational owner | on-call, dashboards, runbook, capacity plan |

Merge when the supposed boundary has:

- coordinated releases and rollbacks on nearly every change;
- shared database mutations;
- chatty low-latency calls that reconstruct one in-process operation;
- no independent operator or product authority;
- duplicated compatibility/telemetry work without failure isolation benefit.

Merging is not architectural failure. It can restore the truthful authority
boundary.

## Exit Criteria for Any Blueprint Change

```
[ ] semantic contract documented
[ ] data/effect authority singular
[ ] old/new compatibility tested
[ ] migration resumable and observable
[ ] rollback or forward-repair exercised
[ ] trust/capability changes threat-modeled and least-privilege
[ ] deployment artifact and config identified
[ ] operator ownership and runbook accepted
[ ] old path retirement condition measured
```

| Gate | Minimum evidence |
|------|------------------|
| Contract | fixtures/scenarios for supported old and new forms |
| Implementation | targeted plus workspace tests |
| Operations | health, admission, saturation, and recovery signals |
| Security | caller/capability/data-scope matrix plus revocation evidence |
| Migration | dry run or rehearsal on representative state |
| Rollback | timed exercise before irreversible step |
| Retirement | no supported reader/writer/traffic depends on old path |

```text
cargo test --workspace --all-targets
cargo metadata --format-version 1
# plus blueprint-specific integration and rollback exercises
```

## Ownership, Decision Records, and Time

Record:

| Field | Why |
|-------|-----|
| Decision and rejected alternatives | prevents mythology |
| Current facts/metrics | bounds the claim in time |
| Authority map | identifies who may mutate and recover |
| Compatibility horizon | makes old/new overlap explicit |
| Exit criteria | prevents permanent transitional architecture |
| Revisit trigger/date | allows evidence to invalidate the choice |

For retirement, record the last supported caller/reader/writer, state export or
deletion owner, credential/route/schedule/subscription revocation, artifact
retention window, and the observation proving the old path is unused. "Code
deleted" is not an exit criterion.

Current library/runtime capabilities evolve. Pin concrete versions in each
repository and revisit claims when toolchain, target, host, or workload changes.
The blueprint remains stable at the semantic level while adapter feasibility can
change.

## Universal Bridge First

The universal bridge is evolutionary architecture: keep options where change is
plausible, but require fitness evidence before adding a boundary. The closest
analogues are schema expand/contract, compiler IR pass separation, and staged
distributed protocol upgrades.

Supplementally, solution/project decomposition and hosted-service extraction in
.NET follow the same path. The lesson is not to translate project types
one-for-one; it is to preserve authority and compatibility while changing the
execution host.

## Decision Cheat Sheet

| Situation | Decision |
|-----------|----------|
| Unsure between one app and services | start modular in one deployable |
| Human invokes finite work | CLI [02] |
| Request needs immediate answer | HTTP [03] |
| Durable delivery owns retry | worker [04] |
| Clock owns finite run | batch [05] |
| Dataset custody/publication dominates | ETL [06] |
| Facts fan out across owners | event-driven [07] |
| Consumer process owns lifecycle | library [08] |
| Independent extension lifecycle | plugin [09] |
| Portable capability host | Wasm [10] |
| Hardware/resource bounds dominate | embedded [11] |
| Windows host rules dominate | Windows [12] |
| Independent data/release authority exists | distributed [13] |
| Repository graphs need governance | monorepo [14] |

## Common Confusion Points

- **Future possibility is not present evidence.** Design seams; do not deploy
  speculative services.
- **Reversibility is not abstraction everywhere.** Preserve the few contracts
  likely to move; keep ordinary implementation concrete.
- **A migration is not complete when new code runs.** It completes when old
  paths can be retired under measured exit criteria.
- **Rollback may become unsafe after an irreversible step.** Mark that point and
  switch to forward-repair procedures.
- **Team boundaries alone do not guarantee service boundaries.** Teams still
  need singular data and operational authority.
- **Architecture scorecards can manufacture precision.** Keep assumptions and
  evidence beside the numbers.

## Primary Sources

- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Cargo `metadata`: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- Cargo dependency resolver: https://doc.rust-lang.org/cargo/reference/resolver.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Cargo SemVer compatibility: https://doc.rust-lang.org/cargo/reference/semver.html
- The Rust Reference: https://doc.rust-lang.org/reference/

## Related Guides

- Landscape: [00-OVERVIEW.md](00-OVERVIEW.md)
- Contract anatomy: [01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md](01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md)
