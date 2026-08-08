# Cargo Build-Unit Multiplication

Date: 2026-08-08
Question: PERF-Q04
Status: Complete
Decision: define how FERRIUM will distinguish required Cargo unit variants from
suspicious or avoidable repeated work across commands.

## Executive conclusion

Most multiplication observed in this study was required by command semantics,
feature isolation, target role, or validation coverage. Package count and
package-version duplication were poor proxies.

Three public fixtures showed sharply different command expansion:

| Fixture | Check | Build | Test | Bench | All-targets check | Release |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| METIS-CORE | 16 | 16 | 114 | 107 | 114 | 16 |
| RUNE | 34 | 29 | 72 | 58 | 72 | 30 |
| PARLOR | 6 | 6 | 16 | 11 | 12 | 6 |

`cargo tree --duplicates` reported no package-version duplicates for RUNE or
PARLOR even though their test and all-target graphs contained many unit
variants. Conversely, reducing a unit count can be incorrect: resolver 2
compiled a controlled dependency separately for build-time and runtime
features, while resolver 1 used one unit by enabling build, runtime, and
dev-only features everywhere. The ordinary program output changed.

Command names and serialized unit graphs are also insufficient predictors of
artifact reuse:

- a dev `cargo build` allowed five PARLOR library artifacts to remain fresh in
  a following test build because the effective dependency profile was
  compatible;
- `cargo check --all-targets` and `cargo clippy --all-targets` produced
  byte-identical unit-graph JSON, but Clippy rebuilt all 12 observed artifacts;
- `cargo check --all-targets` did not prewarm any of the 11 observed test
  artifacts;
- explicitly selecting the host target triple rebuilt all six PARLOR check
  artifacts in the target-specific namespace.

FERRIUM should therefore build a read-only variant ledger. It should explain
the package, target, mode, effective profile, platform, feature, dependency
role, compiler/wrapper, and coverage differences behind each unit. It should
not automatically unify features, disable targets or doctests, merge profiles,
or reduce validation.

No issue, comment, branch, or pull request was created during this research.

## Decision supported

This research determines:

- the unit-variant vocabulary used by cache and validation questions;
- which multiplication is expected from test, bench, lint, release, and
  explicit-target commands;
- why package-version duplicate reports are incomplete;
- which stable and nightly evidence can classify variants;
- the safe first duplicate-work diagnostic boundary.

It does not authorize feature changes, profile changes, validation reduction,
target removal, shared cache intervention, or upstream filing.

## Research question

How much duplicate work is caused by feature divergence, profile differences,
multiple targets, examples, doctests, benches, and test compilation modes?

## Starting and competing hypotheses

The starting hypothesis was that legitimate identity differences explain some
multiplication, but workspace and CI command composition frequently builds more
variants than maintainers expect.

The investigation tested these competing explanations:

1. Most repeated package names are accidental duplicate work.
2. Feature unification is a safe way to reduce unit count.
3. Profile names directly determine whether artifacts can be reused.
4. Matching unit graphs imply matching artifacts.
5. Checking all targets substantially prewarms test or lint commands.

The evidence rejected all five as general rules. Unit variants need a reason
classification and an observed artifact result.

## Evidence reviewed

### Local evidence

- `docs/research/2026-08-07-rust-latency-telemetry.md`
- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- `docs/research/2026-08-08-cargo-graph-scheduling.md`
- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

### Cargo source

Source revision:
[`21c2a90636b4a1991eacd14eca439e7e308c1af4`](https://github.com/rust-lang/cargo/commit/21c2a90636b4a1991eacd14eca439e7e308c1af4)

- [test dependency construction](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_dependencies.rs#L226-L264)
- [check versus build dependency modes](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_dependencies.rs#L837-L852)
- [unit-graph serialized fields and omitted `unit_for`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_graph.rs#L63-L84)
- [unit-graph serialization](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_graph.rs#L123-L143)
- [timing labels distinguish check and check-test](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/timings/mod.rs#L124-L139)

### Cargo documentation

- [dependency resolver and feature unification](https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2)
- [Rust 2021 feature resolver guide](https://doc.rust-lang.org/edition-guide/rust-2021/default-cargo-resolver.html)
- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [`cargo check`](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
- [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html)
- [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
- [`cargo tree --duplicates`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html#tree-options)

## Variant model

FERRIUM classifies repeated package-target work by these axes:

| Axis | Examples | Default classification |
| --- | --- | --- |
| Target | library, binary, integration test, example, bench, doctest | Required when selected coverage differs |
| Compile mode | check, build, test harness, doctest, build-script run | Required unless compatible artifacts are observed |
| Effective profile | optimization, debug info, panic, incremental, LTO, codegen units | Required when effective fields differ |
| Platform | implicit host, explicit target, build host, target runtime | Required correctness boundary |
| Features | normal, build, dev/test, target-specific activation | Required when isolation prevents semantic leakage |
| Dependency role | normal, build, dev, proc macro, artifact dependency | Required unless Cargo proves one compatible unit |
| Compiler invocation | rustc, rustdoc, Clippy driver, wrapper, flags | Required artifact identity even when graph shape matches |
| Coverage intent | default targets, all targets, tests, benches, docs, lint | Required until a validation decision explicitly changes coverage |
| Package version | two resolved versions | Potential dependency consolidation opportunity |

Each variant receives one disposition:

- **Required:** a named semantic, platform, profile, tool, or coverage boundary
  differs.
- **Compatible and reused:** Cargo observed one existing artifact as fresh.
- **Suspicious:** fields appear compatible but repeated compilation is
  observed.
- **Unknown:** available stable evidence cannot expose the differing identity
  field.

## Findings

### FERRIUM-59: package-version duplication is not build-unit multiplication

**Sources**

- [`cargo tree --duplicates`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html#tree-options)
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observation**

`cargo tree --duplicates` reports packages with multiple resolved versions.
RUNE and PARLOR returned no such packages.

Their unit graphs still expanded:

- RUNE: 34 check units to 72 test units;
- PARLOR: 6 check units to 16 test units;
- PARLOR: 6 check units to 12 all-target check units.

One package version can form several target, mode, profile, platform, feature,
or role units. Different package versions are only one multiplication source.

**Implication**

FERRIUM must not present `cargo tree --duplicates` as a duplicate-build report.
It is one input to a broader unit-variant ledger.

**Confidence:** high.

### FERRIUM-60: test and bench commands intentionally multiply target and mode
coverage

**Sources**

- [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
- [Cargo test dependency construction](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_dependencies.rs#L226-L264)
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

The public fixture graphs contained:

| Fixture | Check units/roots | Test units/roots | Bench units/roots |
| --- | ---: | ---: | ---: |
| METIS-CORE | 16 / 1 | 114 / 9 | 107 / 2 |
| RUNE | 34 / 6 | 72 / 28 | 58 / 6 |
| PARLOR | 6 / 6 | 16 / 11 | 11 / 6 |

PARLOR's five library packages each formed build, test-harness, and doctest
units in the test graph. RUNE added 18 named integration-test targets plus
library, binary, proc-macro, and doctest variants. METIS-CORE's dev dependencies
expanded the graph from 10 packages in check to 75 packages in test.

**Implication**

Test and bench multiplication is not labeled waste merely because a package or
target name repeats. FERRIUM should explain the selected coverage and mode for
each root before proposing any reduction.

**Confidence:** high for the measured fixtures.

### FERRIUM-61: feature isolation can require duplicate compilation

**Sources**

- [Cargo feature resolver version 2](https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2)
- [Rust 2021 feature resolver guide](https://doc.rust-lang.org/edition-guide/rust-2021/default-cargo-resolver.html)
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

The controlled application used one dependency in three roles:

- normal dependency with feature `runtime`;
- build dependency with feature `buildtime`;
- dev dependency with feature `devsupport`.

Resolver 2 ordinary build produced two dependency units:

```text
build role: buildtime
runtime role: runtime
```

Resolver 1 produced one dependency unit with all three features. Running the
ordinary program changed from:

```text
runtime=runtime build=buildtime
```

to:

```text
runtime=devsupport build=devsupport
```

Resolver 2 test correctly used separate `buildtime` and
`runtime+devsupport` variants. Resolver 1 reduced the unit count by enabling
all features in both roles.

**Implication**

Feature unification is a semantic change, not a generic optimization. FERRIUM
may expose why variants exist and where features were activated, but must not
recommend unification without consumer-specific compatibility evidence.

**Confidence:** high.

### FERRIUM-62: profile labels are not compatibility keys

**Sources**

- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

PARLOR's test unit graph labeled all units with profile `test`. After a dev
`cargo build`, the following `cargo test --no-run` reported five ordinary
library artifacts as fresh and compiled six test artifacts.

The test profile inherits from dev by default. Cargo reused the compatible
ordinary libraries despite the different profile label. By contrast, a
release build after the dev build compiled all six artifacts under different
effective settings.

**Implication**

FERRIUM compares effective profile fields and observed artifact freshness, not
profile names. A profile-label difference alone is neither proof of duplicate
work nor proof of incompatibility.

**Confidence:** high for the observation and identity boundary.

### FERRIUM-63: check-test variants can look identical in the unstable unit
graph

**Sources**

- [unit-graph omitted `unit_for`](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/unit_graph.rs#L63-L84)
- [timing labels distinguish check-test](https://github.com/rust-lang/cargo/blob/21c2a90636b4a1991eacd14eca439e7e308c1af4/src/compiler/timings/mod.rs#L124-L139)
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

PARLOR's all-target check graph contained two units for every workspace target.
For several packages the serialized package, target, profile, platform, mode,
features, and dependency indexes were identical.

Cargo's internal compile mode distinguishes `Check { test: true }` from
`Check { test: false }`, and timing labels expose `check-test` separately from
`check`. The unit-graph JSON serialized both as `check`; it also intentionally
omits internal `unit_for`.

**Implication**

Even the unstable unit graph is not a complete external identity key. FERRIUM
must preserve `unknown internal role` rather than collapse visibly identical
units or call them accidental duplicates.

**Confidence:** high.

### FERRIUM-64: matching unit graphs do not imply reusable compiler artifacts

**Sources**

- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

For METIS-CORE, RUNE, and PARLOR, the JSON output of:

```text
cargo check --all-targets --unit-graph
```

was byte-identical to:

```text
cargo clippy --all-targets --unit-graph
```

On PARLOR, a stable all-target check compiled 12 artifacts. Running Clippy
immediately afterward compiled all 12 again; none was reported fresh.

Clippy changes the compiler driver/invocation identity, which the serialized
unit graph does not represent.

**Implication**

Graph equivalence and artifact compatibility are separate questions. FERRIUM
needs observed Cargo JSON or build-analysis evidence before declaring
cross-command reuse.

**Confidence:** high.

### FERRIUM-65: check and all-targets check do not prewarm test compilation

**Source**

- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

On fresh PARLOR target directories:

| Sequence | First command | Following test artifacts |
| --- | --- | --- |
| Test alone | None | 11 dirty |
| Check, then test | 6 dirty check artifacts | 11 dirty, 0 fresh |
| All-targets check, then test | 12 dirty check artifacts | 11 dirty, 0 fresh |
| Build, then test | 6 dirty build artifacts | 6 dirty, 5 fresh |

`cargo check` produces metadata-oriented artifacts under a different mode.
Checking test targets still did not produce the build and test-harness artifacts
required by `cargo test --no-run`.

**Implication**

FERRIUM should not advertise all-target checking as a test-compilation cache
warm-up. It may still be valuable for earlier diagnostics; that is a workflow
and coverage decision, not artifact reuse.

**Confidence:** high for the fixture.

### FERRIUM-66: explicit target selection is an artifact namespace boundary

**Sources**

- [`cargo build` target selection](https://doc.rust-lang.org/cargo/commands/cargo-build.html#target-selection)
- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`

**Observed behavior**

Selecting `--target x86_64-pc-windows-msvc` on the same host:

- kept the unit count unchanged for all three public fixtures;
- split host and target platform roles for METIS-CORE and RUNE;
- placed PARLOR outputs in the target-specific namespace;
- caused all six PARLOR check artifacts to compile after an implicit-host check.

RUNE showed four packages with both host and explicit-target units because proc
macros and their dependencies remain host work.

**Implication**

Same-triple explicit target selection is not treated as a no-op. FERRIUM should
explain the target namespace and host/target split before calling the work
duplicated.

**Confidence:** high.

### FERRIUM-67: explanation is ready; coverage and identity changes are not

**Sources**

- Experiment:
  `docs/research/perf-q04-build-unit-multiplication/results/EXP-01-command-variants.md`
- `docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md`

**Observation**

The corpus contains explainable multiplication from target coverage, test
harnesses, doctests, dev dependencies, feature isolation, profiles, explicit
targets, and compiler drivers. It also exposes incomplete external identity
surfaces.

No measured case justified changing features, profiles, targets, or validation
coverage. Some sequences reused artifacts, but running an extra command solely
to prewarm another command would add work and needs its own wall-clock
evaluation.

**Implication**

The immediate FERRIUM opportunity is a read-only command and unit-variant diff:

- show how each command expands package targets into units;
- classify the differing identity or coverage axis;
- distinguish planned units from observed dirty artifacts;
- show compatible artifacts Cargo actually reused;
- retain unknown when internal role or invocation fields are absent;
- hand validation-coverage decisions to PERF-Q35.

**Confidence:** high.

## Model evolution

The starting hypothesis was refined:

1. Command composition does repeat work, but most observed variants had a
   semantic or coverage reason.
2. The dangerous optimization is minimizing unit count without preserving role
   and feature semantics.
3. Effective profile compatibility and observed freshness matter more than
   profile labels.
4. Unit graphs describe planned shape but cannot establish artifact reuse by
   themselves.
5. Validation coverage and build reuse must remain separate decisions.

## Recommendations

### Adopt now

- Add required, compatible-and-reused, suspicious, and unknown dispositions.
- Inventory planned units and observed dirty artifacts separately.
- Compare target, mode, effective profile, platform, features, dependency role,
  compiler invocation, and coverage intent.
- Show feature activation by dependency kind.
- Preserve target and validation coverage in every recommendation.

Owner: FERRIUM.

Validation: PERF-Q05, PERF-Q06, PERF-Q21, and PERF-Q35 inherit this variant
model.

### Prototype behind a compatibility boundary

- A stable command diff using Cargo metadata and JSON artifacts.
- A nightly unit-graph adapter for planned unit shape.
- An optional build-analysis adapter for missing identity and dirty causes.
- A report that joins command intent, unit variants, observed freshness, and
  explicit coverage changes.

Owner: FERRIUM.

Validation:

- resolver 1/2 feature-isolation controls;
- check/build/test/bench/lint/release/target fixtures;
- schema-version tests;
- stable-only unknown results;
- no manifest or command mutation;
- no reduced validation recommendation.

### Reject or defer

- Automatic feature unification.
- Disabling doctests, integration tests, examples, benches, or targets because
  they add units.
- Merging dev, test, release, or bench profiles from labels alone.
- Treating package-version duplicates as the complete problem.
- Treating identical unit graphs as artifact compatibility.
- Adding prewarm commands without whole-workflow timing.
- Filing Cargo or Clippy work without explicit owner approval.

## Contribution path

1. **Explain externally now:** provide command and unit-variant diffs.
2. **Configure or wrap later:** help maintainers select explicit commands while
   showing coverage and artifact consequences.
3. **Research further:** PERF-Q21 studies whether command artifacts can share
   more work safely.
4. **Contribute upstream only with approval:** minimize an identity or
   observability gap after several fixtures show material repeated work.

## Non-goals

- Reducing test or lint coverage.
- Changing feature semantics.
- Replacing Cargo's resolver or profiles.
- Equating fewer units with a faster trustworthy workflow.
- Predicting rustc query reuse from Cargo units.
- Publishing private command or dependency graphs.

## Open questions

- Which repeated dirty artifacts are materially expensive on larger public
  workspaces?
- Can stable Cargo expose check-test and dependency-role distinctions more
  precisely?
- Which effective profile fields permit build-to-test reuse?
- How much check, build, Clippy, test, and doctest work can share safely?
- Which CI command matrices repeat equivalent artifacts across jobs?

## Role review

| Role | Verdict | Required discipline |
| --- | --- | --- |
| Rust Safety Steward | Approve | Feature, platform, profile, and validation boundaries are not weakened to reduce unit count. |
| Compiler Performance Engineer | Approve | Planned units, dirty artifacts, reuse, and coverage are measured separately; no count-only optimization is claimed. |
| Interop Boundary Auditor | Approve | Host/target, build-script, proc-macro, and explicit-target roles remain visible. |
| AI Assurance Skeptic | Approve | Visibly identical units and unexplained artifact differences remain unknown rather than guessed. |
| Ecosystem Strategist | Approve | The capability complements Cargo metadata, tree, unit graph, JSON, and build analysis. |
| Rust Maintainer | Approve | Reports explain ordinary commands and do not prescribe feature or validation churn. |
| Native Platform Adopter | Approve with restriction | Target, profile, CI, and removal consequences must accompany any future configuration advice. |
| Scope Keeper | Approve | Q04 classifies multiplication; artifact-sharing implementation and validation planning remain Q21 and Q35. |
| Validation Checker | Approve | Three public fixtures, one semantic control, negative reuse sequences, revisions, commands, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

