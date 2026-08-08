# Build Intelligence Research Program

Status: Research plan

## Opportunity thesis

FERRIUM's leading build opportunity is not another timing dashboard. It is an
evidence-backed build and validation planner that can:

1. forecast the build and test blast radius of a proposed Rust change;
2. explain what rebuilt after the change and why;
3. recommend the smallest sufficient validation plan without hiding risk;
4. diagnose cache identity, workspace, feature, macro, codegen, and linker
   causes; and
5. emit an auditable evidence packet for maintainers and AI-generated changes.

This joins the Hammer build-latency lane with the Temper assurance lane. It
remains a research direction, not an implementation commitment, until the
[measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md) passes.

## Why Rust latency needs continued decomposition

"Rust is slow" is not a useful engineering diagnosis. A developer-visible delay
can come from several independently owned systems:

| Research area | Why it can be costly | How FERRIUM may help |
|---|---|---|
| Cargo graph and scheduling | Duplicate build units, feature divergence, target multiplication, and critical-path serialization | Map graph causes and identify repeated or avoidable units |
| Frontend and semantic analysis | Parsing, expansion, resolution, type checking, trait solving, and borrow checking perform substantial correctness work | Attribute frontend-dominant cases and contribute minimized fixtures |
| Incremental queries | Hashing, dependency tracking, persistence, and conservative invalidation can repeat more work than an edit appears to require | Compare edit intent with observed invalidation and isolate broad query dependencies |
| Procedural macros and build scripts | Arbitrary execution and incomplete input declarations make reuse and invalidation difficult | Inventory inputs, reruns, fan-out, and negative cases |
| Generics and monomorphization | Concrete downstream instantiations repeat code generation and interact with inlining and LTO | Measure duplicate instances and identify safe research boundaries |
| Codegen and optimization | LLVM optimization, codegen-unit choices, and release profiles trade iteration time for runtime quality | Distinguish development and release needs and evaluate supported backend/profile choices |
| Debug information and object emission | Large native objects and debug records add CPU, memory, and filesystem work | Separate emission cost from semantic compilation |
| Linking | Binary size, native dependencies, debug data, and linker behavior may dominate the final wait | Measure relink-only paths and evaluate existing incremental linkers |
| Cache topology | Workspaces, CI jobs, profiles, targets, and repositories may rebuild compatible work under different identities | Define cache identity and explain misses before distributing artifacts |
| IDE and validation loop | Check, test compilation, doctests, examples, and editor actions may duplicate nearby work | Model time from edit to trustworthy feedback rather than one Cargo command |

FERRIUM should help first by making these causes observable and comparable. It
should modify rustc only after a representative fixture isolates a specific
upstream problem.

## Capability sequence

### BI-01: Build-shape census

Classify representative repositories by crate graph, target types, features,
macros, build scripts, generics, native dependencies, profiles, and linking.

**Research output:** frozen fixture revisions and baseline evidence.

### BI-02: Post-build causality

Explain direct package work, downstream metadata invalidation, downstream
codegen, macro or build-script reruns, cache misses, and relinking.

**Research output:** a causal vocabulary validated against controlled edits.

### BI-03: Pre-change blast-radius forecast

Given a proposed file or API change, predict which packages, targets, tests, and
link steps are likely to be affected.

**Research output:** predictions compared with held-out observed builds.

### BI-04: Evidence-backed validation plan

Recommend the checks, builds, and tests justified by the change and dependency
graph. The plan may reduce redundant work, but it must expose uncertainty and
must never claim that a targeted run proves global correctness.

**Research output:** validation recommendations with explicit coverage and
human approval boundaries.

### BI-05: Cache and workspace diagnosis

Explain artifact incompatibility, feature/profile divergence, workspace
topology, and build-script or macro inputs that prevent reuse.

**Research output:** measured experiments and reversible recommendations, not
automatic rewrites.

### BI-06: Ferris build evidence packet

Combine revision, environment, commands, causal observations, prediction
accuracy, validation results, failures, and limitations into a reviewable
record.

**Research output:** the Hammer-specific input to Pulse 03's broader Ferris
evidence contract.

## Research questions

The next latency work should answer these questions in order:

1. **RQ-01:** Which component dominates each representative edit-to-feedback
   workflow?
2. **RQ-02:** Which controlled edits trigger unexpectedly broad package or
   target work?
3. **RQ-03:** What causality can stable Cargo metadata, JSON messages, and
   timing artifacts establish without nightly rustc?
4. **RQ-04:** Where does optional self-profile evidence materially improve the
   explanation?
5. **RQ-05:** Which duplicate build units arise from legitimate identity
   differences versus avoidable feature, profile, target, or workspace splits?
6. **RQ-06:** When do procedural macros and build scripts dominate or broaden
   invalidation?
7. **RQ-07:** When do monomorphization, codegen, debug information, or linking
   dominate over semantic analysis?
8. **RQ-08:** Can a pre-change forecast predict observed package and target work
   accurately enough to help a maintainer?
9. **RQ-09:** Can a validation plan reduce feedback latency while preserving
   explicit uncertainty, negative tests, and ordinary Cargo workflows?

## Contribution paths

Each measured problem has one of four dispositions:

| Disposition | Use when |
|---|---|
| Explain externally | Existing behavior is valid but difficult to interpret |
| Configure or wrap | Supported Cargo, rustc, backend, or linker choices solve the measured problem |
| Contribute upstream | A minimized fixture demonstrates an upstream profiler, diagnostic, cache, invalidation, or performance gap |
| Research behind a boundary | The opportunity is real but depends on unstable interfaces or an unresolved correctness model |

A compiler fork, new backend, custom linker, or independent package manager is
not an initial disposition.

## Stage gates

### Stage A: Observe

- Freeze public and synthetic fixture revisions.
- Execute the workload and edit matrices.
- Preserve unknown and failed cases.

**Gate:** stable, reproducible evidence on at least three public repositories.

### Stage B: Explain

- Classify observed work using the causal vocabulary.
- Validate explanations against synthetic controls.
- Minimize surprising real-world cases.

**Gate:** explanations outperform total wall time as a maintainer diagnostic.

### Stage C: Predict

- Hold back edit scenarios from explanation development.
- Forecast package, target, test, and link impact.
- Compare predictions with observed work and publish errors.

**Gate:** useful accuracy without pretending certainty.

### Stage D: Plan validation

- Map predicted impact to explicit Cargo checks and tests.
- Preserve mandatory project release or safety gates.
- Require human approval for reduced validation on high-risk changes.

**Gate:** faster trustworthy feedback on fixtures without hidden coverage loss.

### Stage E: Prototype or contribute

- Propose the smallest external compatibility boundary that passed the gates.
- Send minimized compiler/Cargo cases upstream where appropriate.
- Defer capabilities whose correctness or maintenance model remains unclear.

**Gate:** full `.roles` review and an approved bounded implementation pulse.

## Role review

| Role | Program disposition |
|---|---|
| Rust Safety Steward | Accepted with the requirement that validation reduction never becomes an implied safety proof. |
| Compiler Performance Engineer | Accepted because latency remains decomposed by workflow and compiler/build component. |
| Interop Boundary Auditor | Accepted because native dependencies and mixed-language effects remain explicit measurement dimensions. |
| AI Assurance Skeptic | Accepted because prediction errors, unknown causes, failed commands, and human approval boundaries remain visible. |
| Ecosystem Strategist | Accepted because external explanation and upstream contribution precede replacement tooling. |
| Rust Maintainer | Accepted because the primary outputs are explanation, prediction, and review evidence rather than source churn. |
| Native Platform Adopter | Accepted because private measurement remains local and recommendations must be reversible. |
| Scope Keeper | Accepted because the stages isolate observation, explanation, prediction, validation planning, and implementation. |
| Validation Checker | Accepted because every gate depends on fixtures, held-out edits, commands, negative cases, and measured outcomes. |

## Immediate next work

1. Select exact Tier 0 and Tier 1 fixture revisions.
2. Define deterministic edits for at least six scenarios.
3. Record baseline Cargo metadata and environment identities.
4. Run the first project-cold, warm-no-op, warm-edit, and warm-revert matrix.
5. Score which latency component and causal gap appears repeatedly.
6. Decide whether BI-02 has enough evidence to propose a prototype.

No product code is authorized by this plan.
