# Blueprint Competitive Positioning and CLI Strategy

Date: 2026-08-10
Status: Complete
Naming status: public product and command spelling superseded by BLUE-Q05;
competitive category, one-engine architecture, Cargo boundary, adoption wedge,
and claim limits remain active
Decision: position **FERRIS Blueprint** as a Cargo-native, cross-workspace and
cross-repository orchestration control plane for Rust applications. Use one
engine with two public entrypoints:

```console
ferris blueprint
cargo blueprint
```

`ferris blueprint` is the complete enterprise and multi-repository surface.
`cargo blueprint` is the low-friction current-workspace entrypoint implemented
through Cargo's external-subcommand convention. Do not use an unqualified
standalone `blueprint` binary because existing crates already publish
`blueprint` and `bp` binaries and multiple Blueprint package families.

## Decision supported

This research closes
[BLUE-Q04](questions/blueprint/BLUE-Q04-competitive-positioning-cli.md) and
defines product positioning, adoption, package naming, CLI boundaries, and
competitive claims for APPLICATION-001, PLANNING-001, VIEW-001, and
CONFORMANCE-001.

It answers:

- which tools are direct competitors, adjacent alternatives, and complements;
- whether Blueprint is a build replacement, task runner, CI system, or
  application orchestration layer;
- whether the public command should be standalone, a Cargo subcommand, or both;
- how the CLI should represent cross-workspace and cross-repository scope; and
- which market claims are unsupported until later implementation.

Implementation authority remains closed.

## Competitive categories

### Direct competitors

**Bazel with rules_rust** and **Buck2** are the closest direct competitors when
an organization wants a declarative enterprise build graph, explicit targets,
hermetic-oriented execution, caching, and remote execution.

- Bazel uses `MODULE.bazel`, `BUILD.bazel`, rules, targets, queries, toolchains,
  and a Bazel-owned execution graph. `rules_rust` adapts Rust to that model.
- Buck2 uses `BUCK` files, cells, prelude rules, target graphs, BXL
  introspection, and REAPI-oriented execution. Rust is one language in a
  Buck-owned graph.

Blueprint differs initially by retaining Cargo manifests, lockfiles,
resolution, units, freshness, and ordinary commands. If Blueprint later claims
hermetic execution, remote execution, or complete build-graph ownership, Bazel
and Buck2 become the direct comparison and acceptance bar.

Sources:

- [Bazel](https://bazel.build/);
- [rules_rust](https://bazelbuild.github.io/rules_rust/);
- [Buck2](https://github.com/facebook/buck2);
- [Buck2 Rust rules](https://buck2.build/docs/prelude/rules/rust/rust_binary/);
  and
- [Buck2 remote execution](https://buck2.build/docs/users/remote_execution/).

### Orchestration and affected-task competitors

**Nx** is the strongest adjacent comparison for affected-only task selection,
project graphs, local and remote task caching, plugin-inferred tasks, and
explainable monorepo execution. It is not Rust-native, but its user experience
is close to Blueprint's planning wedge.

**Turborepo** provides a useful task-pipeline and cache UX comparison but is
explicitly focused on JavaScript and TypeScript codebases.

Sources:

- [Nx](https://github.com/nrwl/nx);
- [Nx task caching](https://nx.dev/docs/features/cache-task-results);
- [Turborepo](https://turborepo.dev/docs); and
- [Turborepo remote caching](https://turborepo.dev/docs/core-concepts/remote-caching).

### Rust-native task and workspace tools

These are lower-friction competitors or adoption patterns rather than complete
enterprise peers:

- `just` is explicitly a command runner, not a build system;
- `cargo-make` supplies TOML tasks, dependencies, execution plans, workspace
  support, `cargo make`, and a standalone `makers` command;
- `xtask` is a convention for repository-owned Rust automation;
- `cargo-workspaces` manages workspace-oriented version, publish, execute, and
  changed-package workflows;
- `cargo-nextest` improves Rust test execution and scheduling;
- guppy, hakari, and Cargo metadata provide package-graph analysis.

Blueprint must offer more than another task file: application definition,
cross-workspace scope mappings, affected closures, validation coverage,
resource coordination, explanation, contracts, lifecycle, and evidence roots.

Sources:

- [just](https://just.systems/man/en/);
- [cargo-make](https://sagiegurari.github.io/cargo-make/);
- [Cargo xtask pattern](https://github.com/matklad/cargo-xtask);
- [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces);
- [cargo-nextest](https://nexte.st/);
- [guppy](https://github.com/facebookincubator/cargo-guppy); and
- [cargo-hakari](https://docs.rs/cargo-hakari/latest/cargo_hakari/).

### Pipeline, environment, and cache complements

- Dagger supplies programmable container workflows and strong OpenTelemetry
  visibility.
- Earthly supplies cross-repository container build targets and caching, but
  its current repository states that it is no longer actively maintained.
- Nix and Guix supply reproducible environments, derivations, profiles, and
  binary caches.
- sccache supplies compiler-result caching and optional distributed
  compilation through Cargo's compiler-wrapper boundary.
- Make and Ninja remain useful recipe and low-level execution backends.

Blueprint should consume or coordinate these systems rather than duplicate
their proven boundaries.

Sources:

- [Dagger](https://docs.dagger.io/);
- [Earthly](https://github.com/earthly/earthly);
- [Nix](https://github.com/NixOS/nix);
- [Guix shell](https://guix.gnu.org/manual/en/html_node/Invoking-guix-shell.html);
- [sccache](https://github.com/mozilla/sccache);
- [GNU Make](https://www.gnu.org/software/make/manual/); and
- [Ninja](https://ninja-build.org/manual.html).

The detailed comparison is retained in
[EXP-01 competitor matrix](blue-q04-competitive-positioning/results/EXP-01-competitor-matrix.md).

## Positioning

Recommended category:

> **Cargo-native orchestration and application control plane for
> cross-workspace Rust estates.**

Recommended short statement:

> **Keep Cargo. Gain cross-workspace planning, affected-only execution,
> contracts, policy, resource coordination, and explainability.**

Avoid leading with:

- “Rust Bazel,” because Blueprint initially does not own hermetic build
  semantics or remote execution;
- “task runner,” because that understates application, validation, contract,
  resource, lifecycle, and evidence scope;
- “monorepo tool,” because multiple repositories and independent workspaces are
  first-class;
- “CI system,” because local developer planning is equally important;
- “package manager,” because Cargo remains authoritative; or
- “analytics platform,” because the product decision is an executable planning
  and coordination system, not greatness inferred from dashboards.

## Command decision

### Why `cargo blueprint` remains important

Cargo officially dispatches an external subcommand by looking for
`cargo-<name>` on `PATH`. A package named `cargo-blueprint` can therefore
provide:

```console
cargo blueprint plan
cargo blueprint affected
cargo blueprint check
cargo blueprint test
cargo blueprint explain
```

This is the best zero- or low-configuration entrypoint for one current Cargo
workspace.

Source:
[Cargo external tools](https://doc.rust-lang.org/cargo/reference/external-tools.html).

### Why Cargo cannot be the only command surface

The complete product must address:

- multiple Cargo workspaces and lockfiles;
- multiple repositories;
- Typebook contracts and generated projections;
- non-Cargo native, packaging, policy, validation, and deployment activities;
- enterprise repository discovery and selection; and
- application-level roots, channels, profiles, and lifecycle.

Putting every operation under `cargo` would imply that one Cargo workspace is
the product boundary and would undersell the application control plane.

### Why the standalone command is `ferris blueprint`

As observed on 2026-08-10:

- `blueprint` is an occupied crates.io package;
- `blueprint-cli` is an occupied package that already publishes binaries named
  `blueprint` and `bp`;
- `blueprint-core` is an occupied package in another Blueprint SDK family;
- `cargo-blueprint` returned no crates.io package;
- `ferris-cli`, `ferris-blueprint`, and `ferrisctl` returned no crates.io
  package.

Availability observations are not reservations. They do show that an
unqualified `blueprint` binary and `blueprint-*` internal package family would
create avoidable Rust ecosystem collisions.

The product should therefore use:

```console
ferris blueprint plan
ferris blueprint run
ferris blueprint affected
ferris blueprint graph
ferris blueprint explain
ferris blueprint query
ferris blueprint doctor
```

This preserves **FERRIS Blueprint** as the brand while leaving room for one
future FERRIS platform CLI.

## One engine, two adapters

```text
ferris blueprint ...
        |
        +-- enterprise/repository/application scope adapter
        |
shared Blueprint command, model, planning, policy, and execution engine
        |
        +-- current Cargo workspace adapter
        |
cargo blueprint ...
```

The two entrypoints must share:

- command parser and semantic command IDs;
- configuration and application discovery;
- scope selectors and Query Forest schema;
- plan generation and explanation;
- policy, permissions, fallback, and evidence;
- output formats, exit codes, and conformance tests; and
- version and compatibility policy.

They may differ only in defaults and available scope:

- `cargo blueprint` defaults to the current Cargo workspace and Rust-native
  commands;
- `ferris blueprint` exposes repository, application, multi-workspace,
  contract, policy, profile, CI, and deployment selection.

## Initial command hierarchy

### Shared commands

| Command | Purpose |
|---|---|
| `plan` | produce the non-executable Blueprint Plan and reasons |
| `run` | execute one approved named activity or plan |
| `affected` | calculate changed scope since a revision or root |
| `graph` | show discovered application, workspace, task, and dependency mappings |
| `query` | select typed scopes and evidence |
| `explain` | explain selection, rebuild, wait, cache, validation, or fallback |
| `check` | run the declared affected check activity |
| `test` | run the declared affected test and validation activity |
| `doctor` | diagnose environment, configuration, tools, mappings, and evidence |

### Enterprise-only or later commands

| Command | Purpose |
|---|---|
| `repo` | discover, register, and select repositories |
| `workspace` | inspect and select Cargo workspaces |
| `contract` | coordinate Typebook and boundary-contract activities |
| `profile` | select and inspect compatibility profiles |
| `policy` | inspect mandatory gates, authority, and fallback |
| `root` / `ref` | inspect immutable roots and typed references |
| `cache` | inspect eligibility, economics, storage, and quarantine |
| `ci` | render or execute approved CI projections |

Examples:

```console
cargo blueprint plan test --affected --since origin/main
cargo blueprint explain rebuild --package payments-core
ferris blueprint plan --application payments
ferris blueprint run test --repo payments --workspace api
ferris blueprint query "activity:test and affected()"
```

These commands are candidate specification inputs, not implemented syntax.

## Package architecture

Do not use the occupied generic `blueprint-*` package family. Candidate package
names are:

- `ferris-blueprint-model`;
- `ferris-blueprint-cargo`;
- `ferris-blueprint-plan`;
- `ferris-blueprint-exec`;
- `ferris-blueprint-observe`;
- `ferris-blueprint-config`;
- `ferris-cli`, providing the `ferris` binary; and
- `cargo-blueprint`, providing the `cargo-blueprint` binary.

One workspace may hold these packages initially. Package boundaries remain
implementation work and require fresh availability, ownership, licensing, and
maintenance review before publication.

## Adoption wedge

The strongest first use case is:

> **Affected-only checks and tests across several Cargo workspaces, with a
> plan and explanation before execution.**

This wedge:

- preserves existing Cargo manifests and commands;
- avoids immediate BUILD-file migration;
- addresses real local and CI fan-out;
- demonstrates cross-workspace scope mapping;
- exercises resource budgets and validation fallback;
- complements nextest, sccache, Nix, Dagger, and existing CI;
- can be removed without correctness changes; and
- provides measurable comparison with ad hoc scripts and full-workspace runs.

The first proof remains local and read-only for planning. Execution requires a
separately approved action boundary.

## Competitive claim boundaries

Do not claim:

- hermetic builds without complete declared-input isolation and enforcement;
- Bazel- or Buck2-equivalent remote execution;
- Cargo replacement;
- deterministic incremental correctness without validated invalidation;
- universal language support without implemented adapters;
- remote cache correctness from transport or key matches;
- test coverage from package selection or filtered passes;
- complete observability without trace-grade evidence;
- automatic enterprise support from one successful plan; or
- that cross-workspace orchestration implies one global dependency resolution.

## Recommendations

### Adopt now

- Position FERRIS Blueprint as Cargo-native cross-workspace orchestration and
  application control.
- Standardize the dual entrypoints `ferris blueprint` and `cargo blueprint`.
- Keep one semantic engine and command model.
- Use `cargo blueprint` as the adoption wedge and `ferris blueprint` as the
  complete product surface.
- Define plan, affected, graph, explain, query, doctor, check, and test first.
- Use FERRIS-prefixed internal package names.

### Prototype behind a compatibility boundary

- One command parser invoked through both entrypoints.
- Workspace-local planning through `cargo blueprint plan`.
- Multi-workspace discovery and affected-only dry-run through
  `ferris blueprint plan`.
- Explain output compared with Cargo, scripts, and full reference commands.
- Removal and fallback to ordinary Cargo and existing CI.

### Propose upstream

- stable Cargo plan, scope, artifact, freshness, test, and build-analysis
  plumbing;
- external-subcommand interoperability and machine-readable output conventions;
  and
- generic coordination primitives proven useful without embedding FERRIS
  policy in Cargo.

### Reject or defer

- only `cargo blueprint`;
- only an unqualified `blueprint` binary;
- BUILD-file migration in the first adoption path;
- claiming hermeticity, remote execution, or full build replacement;
- production cache or remote execution before separate evidence;
- a new configuration language before Cargo metadata and existing declarations
  prove insufficient; and
- implementation before CLI, scope, planning, removal, and conformance specs.

## Findings

### FERRIS-743: Bazel and Buck2 are the replacement-system comparison

**Sources:** Bazel, rules_rust, Buck2, Buck2 Rust rule, and remote-execution
documentation.

**Observed behavior:** both systems own declarative target graphs, rule
semantics, execution, caching, and remote-execution integration rather than
delegating Rust builds to ordinary Cargo workflows.

**Implication:** Blueprint competes directly only when it claims those
capabilities; initial positioning must emphasize Cargo-native federation.

**Confidence:** High.

### FERRIS-744: Nx is the closest affected-task user-experience comparison

**Sources:** Nx repository and task-cache documentation.

**Observed behavior:** Nx discovers project tasks, computes affected work,
caches task outputs, and exposes graph and task information across a
polyglot-oriented monorepo model.

**Implication:** Blueprint should match the clarity of affected, graph, plan,
and explain experiences while supplying Rust-specific correctness boundaries.

**Confidence:** High.

### FERRIS-745: Rust-native tools are complements unless Blueprint remains a
task runner

**Sources:** just, cargo-make, xtask, cargo-workspaces, nextest, guppy, and
hakari documentation.

**Observed behavior:** existing tools cover recipes, repository automation,
workspace release workflows, test execution, and package-graph analysis but do
not join the complete application, scope, policy, resource, lifecycle, and
evidence model.

**Implication:** Blueprint must demonstrate cross-workspace affected planning
and explanation rather than compete on task syntax alone.

**Confidence:** High.

### FERRIS-746: pipeline, environment, and cache systems should remain adapters

**Sources:** Dagger, Earthly, Nix, Guix, sccache, Make, and Ninja documentation.

**Observed behavior:** these systems already provide container workflows,
environments, caches, recipes, or low-level execution with independent
strengths and limitations.

**Implication:** Blueprint coordinates them through typed plans and evidence
instead of duplicating every execution substrate.

**Confidence:** High.

### FERRIS-747: one Cargo subcommand cannot express the complete product scope

**Sources:** Cargo external tools documentation; BLUE-Q02 and BLUE-Q03.

**Observed behavior:** `cargo blueprint` is a valid native extension mechanism,
but Blueprint plans span independent workspaces, repositories, contracts,
policies, deployment, profiles, and non-Cargo activities.

**Implication:** retain the Cargo entrypoint but provide a FERRIS enterprise
entrypoint.

**Confidence:** High.

### FERRIS-748: the unqualified Blueprint command namespace is occupied

**Sources:** crates.io API observations for `blueprint`, `blueprint-cli`,
`blueprint-core`, and `cargo-blueprint` on 2026-08-10.

**Observed behavior:** existing packages use the Blueprint name and publish
`blueprint` and `bp` binaries, while `cargo-blueprint` was unclaimed.

**Implication:** use the qualified product name FERRIS Blueprint,
`ferris blueprint`, `cargo blueprint`, and FERRIS-prefixed internal packages.

**Confidence:** High for the dated observation.

### FERRIS-749: one engine should serve both CLI entrypoints

**Sources:** cargo-make's dual executable pattern; Cargo external subcommands;
Blueprint scope and planning requirements.

**Observed behavior:** separate command implementations would create semantic,
configuration, output, and compatibility drift.

**Implication:** entrypoints differ only in discovery defaults and available
scope; command IDs, schemas, plans, policy, and conformance remain shared.

**Confidence:** High.

### FERRIS-750: affected checks and tests are the strongest adoption wedge

**Sources:** Nx affected model; PERF-Q35; BLUE-Q02; BLUE-Q03.

**Observed behavior:** organizations already have Cargo commands and scripts,
while unnecessary cross-workspace checking and testing produce visible local
and CI cost.

**Implication:** prove planning and explanation on existing commands before
introducing build replacement, remote execution, or a new configuration
language.

**Confidence:** High.

### FERRIS-751: competitive claims must follow implemented authority

**Sources:** all competitor evidence and FERRIS role boundaries.

**Observed behavior:** “build system,” “hermetic,” “remote execution,”
“deterministic,” and “multi-language” imply mature capabilities with strict
correctness and operational contracts.

**Implication:** product language must distinguish current Cargo-native
orchestration from separately gated future execution and cache capabilities.

**Confidence:** High.

### FERRIS-752: dual-entry competitive positioning is ready for specification

**Sources:** FERRIS-743 through FERRIS-751.

**Observed behavior:** product category, competitor boundaries, command
surfaces, naming conflicts, adoption wedge, and unsupported claims converge.

**Implication:** VIEW-001 and CONFORMANCE-001 may standardize the dual-entry
CLI without authorizing implementation.

**Confidence:** High.

## Nine-role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: positioning does not claim hermeticity, cache correctness, or build replacement without evidence. |
| Compiler Performance Engineer | Accepted: the first wedge measures affected work and machine cost while leaving Cargo and rustc authoritative. |
| Interop Boundary Auditor | Accepted: standalone scope can include contracts, native, packaging, and deployment without pretending Cargo owns them. |
| AI Assurance Skeptic | Accepted: analytics, plans, execution, outcomes, and competitive claims remain separate evidence classes. |
| Ecosystem Strategist | Accepted: Blueprint complements Cargo-native tools and existing execution substrates before competing with replacement systems. |
| Rust Maintainer | Accepted: `cargo blueprint` uses the official external-subcommand boundary and ordinary Cargo remains removable. |
| Native Platform Adopter | Accepted: the enterprise CLI can expose platform, provider, resource, and deployment scope explicitly. |
| Scope Keeper | Accepted: one engine and two adapters avoid duplicate products; remote execution and BUILD-file migration remain deferred. |
| Validation Checker | Accepted: both entrypoints require identical plan, output, fallback, removal, and conformance fixtures. |

## Limitations

- Product and package names are not reserved.
- Competitor capabilities, governance, maintenance, and pricing continue to
  evolve.
- Pants Rust support was not strong enough in current official evidence to
  classify as a primary Rust competitor.
- No user study has compared the proposed command vocabulary with existing
  Rust or enterprise-build users.
- The first CLI hierarchy remains a specification input, not implemented
  syntax.
