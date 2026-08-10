# EXP-01: Blueprint Competitor Matrix

Date: 2026-08-10
Question: BLUE-Q04
Result: FERRIS Blueprint should occupy the Cargo-native orchestration layer,
with Bazel/Buck2 as replacement-system competitors, Nx as the affected-task UX
comparison, and existing Rust/pipeline/cache/environment tools as complements.

| System | Owns build graph? | Cargo-native? | Cross-workspace/repo | Cache / remote execution | Best comparison |
|---|---:|---:|---:|---|---|
| Bazel + rules_rust | yes | no; Rust adapted to Bazel rules | strong | mature cache and remote-execution ecosystem | direct replacement competitor |
| Buck2 | yes | no; Rust adapted to Buck rules | strong | REAPI-oriented execution and caching | direct replacement competitor |
| Pants | generally | Rust maturity not established by current evidence | monorepo-oriented | general Pants capabilities | monitor; do not overstate |
| Nx | task graph | no | strong monorepo | local/remote task cache and distribution | affected-task UX competitor |
| Turborepo | task graph | JS/TS-focused | monorepo | local/remote task cache | UX comparator only |
| cargo-make | task graph | yes | workspace-oriented | no enterprise cache/RE | low-end task competitor |
| just / Make | recipes | shells out | repository-oriented | no native enterprise cache/RE | incumbent script baseline |
| xtask | arbitrary repository code | yes | repository-oriented | custom | adoption convention |
| cargo-workspaces | workspace operations | yes | workspace | none | narrow workflow complement |
| cargo-nextest | test execution | yes | workspace | not primary | test complement |
| Dagger | workflow graph | wraps Cargo | cross-repo capable | content-addressed pipeline cache | pipeline/observability complement |
| Earthly | container target DAG | wraps Cargo | cross-repo refs | cache and parallelism | weakening complement |
| Nix / Guix | derivation/environment graph | complements Cargo | broad | binary caches | environment complement |
| sccache | compiler cache | yes | cache may be shared | local/cloud/distributed compile | cache complement |
| Cargo | package/unit graph per invocation | authoritative | one workspace/resolution context | local build state | required owner system |

## Positioning tests

| If Blueprint claims... | Customer comparison | Required evidence |
|---|---|---|
| task runner | just, cargo-make, xtask | materially better cross-workspace planning and explanation |
| affected execution | Nx | correct scope mapping, cache inputs, graph UX, selected/full validation |
| build system | Bazel, Buck2 | declarative graph, hermeticity, artifact semantics, scheduling, reproducibility |
| remote execution | Bazel/Buck2 REAPI ecosystems | complete input model, isolation, trust, retries, logs, platform selection |
| CI platform | Dagger and hosted CI | local/remote parity, secrets, tracing, concurrency, failure recovery |
| reproducible environment | Nix/Guix | exact environment and dependency closure |
| Rust compilation cache | sccache/Cargo cache work | compatible identity, integrity, materialization, economics |

## CLI decision

| Surface | Default scope | Purpose |
|---|---|---|
| `cargo blueprint` | current Cargo workspace | Rust-native adoption and local command coordination |
| `ferris blueprint` | selected application/repositories/workspaces | complete cross-workspace enterprise orchestration |

Both use one engine, schemas, output formats, policy, and conformance suite.

## Initial wedge

```text
git change
  -> discover affected packages and workspaces
  -> show Blueprint Plan
  -> explain selected and omitted checks/tests
  -> execute only after approval
  -> compare with full reference
```

The first proof does not require BUILD-file migration, hermetic execution,
remote execution, or artifact restoration.
