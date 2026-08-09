# Impact-Aware Validation Selection

Date: 2026-08-09
Status: Complete
Question: PERF-Q35
Decision: adopt a read-only validation-plan and coverage ledger. Prototype
package selection only behind conservative file-to-input mappings, complete
activity coverage, mandatory repository gates, full fallback for uncertainty,
periodic full reference runs, and held-out mutation testing. Do not represent a
selected pass as a full-suite pass or automatically delete required gates.

## Decision supported

This research decides whether FERRIUM build-impact evidence can reduce Rust
validation latency without hiding material coverage loss.

The answer is qualified yes. A planner can select affected packages while
retaining required validation activities, features, targets, profiles,
doctests, and repository gates. Cargo package ownership and reverse dependency
closure are useful inputs, but they are not a complete impact model. Shared
runtime data and repository policy can live outside package roots and outside
Cargo metadata.

The safe product boundary is therefore a reviewable plan, not autonomous test
deletion:

- show changed inputs and their declared owners;
- show the reverse dependency cone;
- show every retained validation dimension and mandatory gate;
- show why a full fallback was or was not selected;
- distinguish selected evidence from full-reference evidence; and
- keep ordinary repository CI authoritative.

## Evidence reviewed

### Local FERRIUM evidence

- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
- [Cargo graph scheduling](2026-08-08-cargo-graph-scheduling.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [Command artifact reuse](2026-08-08-command-artifact-reuse.md)
- [Workspace modularization and crate boundaries](2026-08-09-workspace-modularization-crate-boundaries.md)
- [Experiment record](perf-q35-validation-selection/results/EXP-01-validation-selection-matrix.md)

### Upstream behavior

- Cargo metadata exposes workspace packages and resolved dependency nodes, but
  repository-specific runtime inputs and policy gates require repository
  declarations:
  <https://doc.rust-lang.org/cargo/commands/cargo-metadata.html>
- Cargo package selection changes which packages are operated on:
  <https://doc.rust-lang.org/cargo/reference/workspaces.html>
- `cargo test` has distinct package, target, feature, profile, execution, and
  doctest behavior:
  <https://doc.rust-lang.org/cargo/commands/cargo-test.html>
- `cargo check` and Clippy preserve validation dimensions that package tests
  alone do not cover:
  <https://doc.rust-lang.org/cargo/commands/cargo-check.html> and
  <https://doc.rust-lang.org/clippy/usage.html>

## Terms

**Package selection** chooses Cargo packages to validate.

**Activity selection** chooses checks, lints, tests, test compilation,
doctests, release checks, formatting, policy scripts, and other gates.

**Validation dimension** is an activity, feature set, target set, profile,
toolchain, platform, or execution mode whose coverage can differ.

**Declared input mapping** links files or external inputs outside a package
root to the packages, activities, or gates they can affect.

**Unknown change** is any changed input without a complete, current, and
reviewed mapping.

**Conservative fallback** expands an unknown or ambiguous change to the full
reference plan.

**Selected pass** means the emitted selected plan passed. It does not mean the
full repository contract passed.

## Findings

### FERRIUM-472: validation selection has package and activity dimensions

**Source:** synthetic command matrix and Cargo command documentation.

**Observed behavior:** Selecting `leaf_00` and `app_00` did not determine
whether all features, all targets, Clippy, doctests, release compilation, or a
repository policy gate ran. Those dimensions were controlled independently.

**Implication:** FERRIUM must report selected packages separately from retained
activities, features, targets, profiles, platforms, and gates.

**Confidence:** High.

### FERRIUM-473: changed-package tests are not a sufficient policy

**Source:** eight seeded failures in the synthetic 17-package workspace.

**Observed behavior:** Changed-package tests caught two of eight failures. They
missed a shared-core downstream regression, all-features compile failure,
Clippy-only failure, release-only failure, shared runtime-data failure, and
repository-specific policy failure.

**Implication:** FERRIUM rejects "test the changed package" as a safe default.

**Confidence:** High for represented failure classes.

### FERRIUM-474: reverse dependency closure protects downstream package effects

**Source:** synthetic shared-core and leaf mutations.

**Observed behavior:** A changed core package required all 17 packages. A
changed leaf required that leaf and its application consumer. The reverse cone
with the complete command matrix caught the downstream regression that
changed-package tests missed.

**Implication:** Reverse dependency closure is the minimum package-level impact
model for owned Rust source changes.

**Confidence:** High for explicit Cargo dependencies.

### FERRIUM-475: Cargo graph closure does not cover undeclared runtime inputs

**Source:** `shared/leaf5.expected` mutation outside package roots.

**Observed behavior:** The reverse-cone policy selected no package and missed
the failing runtime-data test. Conservative treatment of the unowned file
expanded to the full plan and caught it.

**Implication:** Shared files, generated inputs, build-script inputs, macro
inputs, native inputs, environment dependencies, and other non-package effects
need declared mappings or full fallback.

**Confidence:** High for the demonstrated class; completeness across real
repositories remains repository-specific.

### FERRIUM-476: validation activities are not interchangeable

**Source:** feature-only, Clippy-only, release-only, and doctest-only mutations.

**Observed behavior:** Ordinary package tests did not catch the all-features,
Clippy, or release failure. Doctest execution caught a documentation example
whose library tests remained green.

**Implication:** Package selection must not silently narrow the repository's
feature, lint, target, profile, doctest, or execution contract.

**Confidence:** High.

### FERRIUM-477: mandatory repository gates exist outside Cargo metadata

**Source:** synthetic `policy.txt` mutation and `validation_gate.py`.

**Observed behavior:** Cargo activities were green while the mandatory
repository gate failed.

**Implication:** Repository-owned formatting, generated-file, policy,
licensing, schema, security, packaging, and compliance gates remain explicit
inputs to every plan in which their declared scope applies.

**Confidence:** High.

### FERRIUM-478: unknown changes require a full fallback

**Source:** shared-data and repository-gate controls.

**Observed behavior:** Treating unowned files as unaffected created a false
negative. Treating them as uncertain and selecting the full reference plan
preserved all seeded failures.

**Implication:** Unknown must mean broader validation, not no validation.
Mappings can narrow work only after review and continuing audit.

**Confidence:** High.

### FERRIUM-479: conservative selection preserved the seeded failure set

**Source:** synthetic detection matrix.

**Observed behavior:** Conservative graph selection with full fallback caught
eight of eight seeded failures. Full reference also caught eight of eight.
Reverse closure with the complete command matrix caught seven of eight.

**Implication:** BI-04 may advance to a bounded prototype, provided the
prototype is judged by false negatives rather than speed alone.

**Confidence:** Medium. Eight designed classes establish feasibility, not a
universal safety proof.

### FERRIUM-480: useful savings came from package scope, not coverage deletion

**Source:** five warm passing-edit repetitions.

**Observed behavior:** Conservative selection retained the complete activity
matrix and policy gate while selecting two of 17 packages. Its 1,096.8 ms
median was 57.1% below the 2,553.8 ms full-reference median. Changed-package
tests were 911.5 ms, only 16.9% faster than conservative selection, while
missing six of eight failure classes.

**Implication:** The defensible optimization is package selection with coverage
retention. Deleting validation dimensions produced little additional benefit
relative to its measured safety loss.

**Confidence:** High for the fixture; absolute timing is not portable.

### FERRIUM-481: topology and mandatory gates bound the gain

**Source:** public PARLOR control at
`0975fad880cb3bda0b911cd8eb4fc58edbbfaf29`.

**Observed behavior:** Selecting `parlor-go` and its reverse-dependent
`parlor-cli` while retaining release tests, Clippy, and workspace formatting
reduced the warm median from 1,344.7 ms to 1,217.6 ms, a 9.4% gain. The CLI's
broad dependencies and mandatory workspace formatting limited the saving.

**Implication:** FERRIUM must forecast expected savings from actual topology
and gate scope instead of promising a universal percentage.

**Confidence:** Medium-high for transfer of the mechanism; one public
repository is not a market-wide benchmark.

### FERRIUM-482: historical mappings require continuous adversarial audit

**Source:** the deliberate unmapped shared-data failure.

**Observed behavior:** A plausible package-only model appeared precise until a
held-out input class violated its ownership assumption.

**Implication:** A validation map must be versioned, reviewable, tested with
held-out mutations, compared periodically with full reference runs, and
governed by a false-negative budget. Historical green runs alone cannot train
or prove a safe selector.

**Confidence:** High for the assurance requirement.

### FERRIUM-483: selected success is not global correctness

**Source:** policy comparison and role review.

**Observed behavior:** Different policies produced green results over
materially different evidence. Even the conservative selected plan did not run
the full reference for an owned leaf edit.

**Implication:** User interfaces, logs, and evidence packets must say
"selected plan passed," list omitted package scope, and name the next full
reference obligation. They must not use full-suite confidence language.

**Confidence:** High.

## Decision

### Adopt now

- Add a read-only validation-plan and coverage ledger to BI-04.
- Separate package scope from activity, feature, target, profile, platform,
  doctest, execution, and mandatory-gate scope.
- Treat unknown, unowned, generated, root-policy, environment, build-script,
  macro, native, and cross-target changes conservatively.
- Record selected-plan evidence separately from full-reference evidence.
- Maintain the synthetic failure classes and public controls as regression
  fixtures.

### Prototype behind a compatibility boundary

A bounded prototype may:

1. read Cargo metadata and repository-owned validation declarations;
2. map owned Rust changes to reverse dependency cones;
3. preserve declared required activities and mandatory gates;
4. expand uncertain changes to the full reference plan;
5. emit an explainable plan for human approval;
6. run only in disposable or owner-approved workflows;
7. audit selected outcomes against periodic full reference runs; and
8. use held-out mutations and a zero-tolerance false-negative promotion gate.

### Reject or defer

- automatic deletion of repository gates;
- default skipping of unknown or unowned files;
- reducing features, targets, profiles, doctests, lints, platforms, or release
  checks solely because a package cone is narrow;
- representing a selected pass as a full-suite pass;
- self-training solely from historical passing runs;
- replacing required CI;
- writing validation mappings or workflow files without owner approval; and
- production autonomy before multi-repository held-out evidence exists.

## Role review

### Rust Safety Steward

Accepts package narrowing only when unsafe, FFI, build-script, macro, generated,
feature, platform, and release effects remain explicit and uncertain changes
fall back to full validation.

### Compiler Performance Engineer

Accepts the matched policy comparison, warm-up, five-run medians and MADs,
failure matrix, and public control. Requires larger repositories and cold,
cross-platform, and CI measurements before performance claims broaden.

### Interop Boundary Auditor

Accepts because Cargo closure is not treated as complete for native, generated,
runtime, environment, packaging, or cross-language inputs. Those boundaries
require mappings or fallback.

### AI Assurance Skeptic

Accepts the held-out false-negative test and explicit confidence language.
Requires continuing mutation testing, full-reference audits, visible omitted
scope, and no learning solely from historical green runs.

### Ecosystem Strategist

Accepts a Cargo-compatible planner that preserves repository contracts.
Recommends upstream or ecosystem contribution when stable metadata cannot
express a recurring impact relationship.

### Rust Maintainer

Accepts a removable, read-only recommendation with exact commands and reasons.
Rejects workflow rewrites, surprise gate deletion, and unexplained confidence
scores.

### Native Platform Adopter

Accepts mandatory policy, packaging, compliance, platform, and release gates
as first-class plan inputs. Requires ordinary full CI to remain available and
authoritative.

### Scope Keeper

Accepts PERF-Q35 as a validation-planning decision. Autonomous CI replacement,
workflow editing, and production deployment remain closed.

### Validation Checker

Accepts the eight failure classes, four policy controls, complete command
matrix, five-run timing comparison, public transfer control, retained raw
evidence, conservative fallback, and explicit limitations.

## Validation and retained evidence

Primary commands:

```text
wsl.exe -d Ubuntu-24.04 -- bash -lc "python3 measure_validation_selection.py"
wsl.exe -d Ubuntu-24.04 -- bash -lc "python3 measure_parlor_validation.py"
```

Retained session evidence:

- `perf-q35/measure_validation_selection.py`
- `perf-q35/results/validation-selection.json`
- `perf-q35/measure_parlor_validation.py`
- `perf-q35/results/parlor-validation.json`

Public reproducibility details, exact commands, revisions, timing summaries,
limitations, and failure outcomes are recorded in the
[experiment record](perf-q35-validation-selection/results/EXP-01-validation-selection-matrix.md).
