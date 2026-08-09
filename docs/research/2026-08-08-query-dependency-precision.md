# Query Dependency Precision and False Invalidation

Date: 2026-08-08
Question: PERF-Q17
Status: Complete
Decision: adopt edit-frontier, provider-execution, result-color, source-layout,
and diagnostic-dependency vocabulary now; prototype read-only invalidation and
source-layout visualization behind a nightly compatibility boundary; defer
span elision, automatic source layout, solver changes, compiler forks, and
upstream activity.

## Executive conclusion

Rustc's incremental query system is substantially more precise than a raw
cache-miss count suggests. Local body, binding, helper, visibility, equivalent
signature, and shared-constant edits stayed local in the tested fixtures. A
shared alias type change correctly invalidated approximately 1,000 callers.
The red-green algorithm also stopped many broad provider re-executions before
borrow checking, optimized MIR, and non-debug codegen.

The clearest broad frontier was source layout. Inserting one ordinary comment
before a helper and 1,000 callers changed no Rust semantics but caused 1,002
`mir_built` providers to execute. The same edit produced no MIR misses with
the testing-only span-ignore control. Across seven minimally instrumented
metadata repetitions, normal span hashing measured 279.57 ms versus
215.62 ms for the control.

Documentation, helper attributes, and unused-item insertion initially showed
the same linear fan-out. Fixed-width substitutions that preserved every later
owner's byte offset collapsed those cases to one local MIR and borrow-check
frontier. A trait-heavy documentation case fell from 1,000 type-check and
1,001 MIR misses to 0 and 1 respectively.

This does not make span hashing a bug. MIR carries source locations used by
diagnostics, debuginfo, coverage, and other observable outputs. A MIR dump
showed semantically identical operations changing from line 6 to line 7 after
one inserted documentation line. Full-debuginfo builds were also much more
sensitive than metadata or debuginfo-disabled builds.

The architectural opportunity is narrower and more useful: separate semantic
work from source-location maintenance wherever correctness and net performance
permit, and make the distinction visible before changing rustc. FERRIUM can
show which owners moved, which providers ran, where fingerprints became equal,
which downstream stages stayed reusable, and whether the trigger was semantic,
diagnostic, layout, or non-persisted session work.

Crate-level lint changes are a counterexample to indiscriminate narrowing. An
equal-width crate lint substitution still invalidated 1,001 type-check roots.
A 20-owner failure control changed an equal-width `allow(unused_mut)` into
`deny(unused_mut)`, turning a successful compile into 20 structured errors.
That dependency is correctness-relevant diagnostic work.

The other confirmed broad case was solver-specific. Replacing reserved comment
bytes with a same-trait impl left all caller offsets unchanged. The old solver
reused the 1,000 callers; the global next solver re-ran type checking, MIR, and
borrow checking for all 1,000. An unrelated-trait impl stayed local in both
modes. This is a targeted dependency-precision candidate, not a general verdict
on either solver.

## Decision supported

This research determines:

- how to distinguish provider execution, changed fingerprints, and downstream
  invalidation;
- which tested semantic edits stayed local or propagated broadly;
- whether documentation, attributes, item insertion, and reordering were
  semantic or source-layout effects;
- where red-green marking contained broad provider re-execution;
- why lint-level dependencies can be correctness-required;
- whether trait impl-set breadth remained after source positions were fixed;
- which visualization and upstream fixture opportunities are defensible.

It does not authorize ignoring spans, rewriting source layout, changing solver
mode, patching rustc, building an incremental compiler replacement, or opening
upstream issues.

## Evidence reviewed

### Local evidence

- [Experiment](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md)
- [Incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Type inference and type checking](2026-08-08-type-inference-checking.md)
- [Trait-solving cost and reuse](2026-08-08-trait-solving-cost-reuse.md)
- [Borrow-checking cost and incrementality](2026-08-08-borrow-checking-cost-incrementality.md)
- [MIR construction and optimization](2026-08-08-mir-construction-optimization.md)
- [Frontend parallelism](2026-08-08-frontend-parallelism.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [stable span hashing](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ich.rs#L78-L154)
- [red-green `try_mark_green`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/dep_graph/graph.rs#L894-L1018)
- [testing-only span-ignore option](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/options.rs#L2542-L2544)
- [`rustc_clean` fingerprint checking](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/clean.rs#L1-L23)
- [incremental compilation in detail](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)

### Performance framework

rustc-perf revision:
`58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

- [scenario model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/scenario.rs)
- [patched-scenario guidance](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [patch expansion](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs)

## Findings

### FERRIUM-189: provider execution is not equivalent to downstream invalidation

**Sources:** [red-green algorithm](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html#improving-accuracy-the-red-green-algorithm) and [containment control](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#red-green-containment-and-codegen).

**Observed behavior:** A shifted helper-documentation edit caused 1,002
`mir_built` misses, but one `mir_borrowck` miss, no `optimized_mir` misses, and
no non-debug codegen-unit misses.

**Implication:** FERRIUM reports at least four states: provider skipped,
provider executed, result proved equal or green, and downstream provider
executed. A cache-miss heatmap alone overstates rebuild impact.

**Confidence:** High for the tested query frontiers.

### FERRIUM-190: semantic edit breadth depends on the changed contract

**Sources:** [primary matrix](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#primary-edit-matrix) and PERF-Q12 through PERF-Q15.

**Observed behavior:** One caller, local binding, helper body, helper
visibility, equivalent signature spelling, and shared constant value stayed
local. Changing the shared scalar alias from `u32` to `u64` invalidated about
1,000 type-check, MIR, and borrow roots.

**Implication:** Rebuild explanations should name the changed semantic
contract—body, type shape, trait candidate set, constant identity, or
optimization input—rather than label every shared item as broad.

**Confidence:** High for the generated fixture.

### FERRIUM-191: source-layout movement alone can re-run every later MIR provider

**Sources:** [layout-only control](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#layout-only-source-shift) and [stable span hashing](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ich.rs#L78-L154).

**Observed behavior:** One ordinary comment inserted before 1,002 existing
owners produced 1,002 `mir_built` misses and no type-check or borrow-check
misses. The testing span-ignore control produced no body misses.

**Implication:** Edit geometry—what moved in the file—is a first-class
incremental input distinct from Rust semantics.

**Confidence:** High.

### FERRIUM-192: fixed-width substitutions separate meaning from movement

**Sources:** [equal-offset controls](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#equal-offset-substitutions).

**Observed behavior:** Fixed-width comment-to-doc, helper-attribute, and
reserved-comment-to-item substitutions each stayed at one local body frontier.
The trait-heavy helper-doc case collapsed from 1,000 type-check and 1,001 MIR
misses to 0 and 1.

**Implication:** Incremental benchmarks should pair ordinary edits with
equal-offset controls before attributing broad work to attributes, docs, item
identity, or query dependencies.

**Confidence:** High.

### FERRIUM-193: insertion position predicts span-driven fan-out

**Sources:** [owner-count minimization](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#owner-count-minimization).

**Observed behavior:** Adding an unused item before 1,000 callers caused 1,003
MIR misses; adding the same item after them caused one. Reordering two
equal-width callers caused two rather than 1,000.

**Implication:** A visual invalidation tool can estimate a layout-risk frontier
from source ranges and edit position without claiming semantic dependency.
This is an explanation aid, not a source-order recommendation.

**Confidence:** High for the minimized fixture.

### FERRIUM-194: MIR construction directly depends on span-bearing owner state

**Sources:** [dependency-graph probe](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#dependency-graph-edge-probe), [MIR span dump](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#mir-carries-the-changed-source-coordinates), and [rustc span model](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ich.rs#L78-L154).

**Observed behavior:** The emitted query graph contained direct
`mir_built -> source_span`, `mir_built -> def_span`, `mir_built -> hir_owner`,
and `mir_built -> hir_attr_map` edges, plus span-bearing paths through THIR and
match checking. An inserted documentation line left MIR operations equal but
changed every recorded caller location from line 6 to line 7.

**Implication:** Updated source locations are observable compiler output.
Potential optimization must separate or patch source information correctly,
not erase it from correctness identity.

**Confidence:** High for the dump; low on the feasibility of a split payload.

### FERRIUM-195: red-green containment limits most non-debug downstream work

**Sources:** [output-mode control](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#red-green-containment-and-codegen).

**Observed behavior:** Metadata and debuginfo-disabled rlib builds re-ran broad
MIR construction but reused optimized MIR and codegen. Full debuginfo caused
one codegen-unit provider to run and had the largest wall difference.

**Implication:** Source-layout work should be reported by stage. "All MIR
rebuilt" does not mean "all machine code rebuilt," while debug builds can
legitimately be more location-sensitive.

**Confidence:** High for providers; medium for wall ratios.

### FERRIUM-196: span-ignore is a diagnostic control, not an optimization flag

**Sources:** [testing-only option](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/options.rs#L2542-L2544) and [span controls](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#span-ignore-control).

**Observed behavior:** Ignoring spans removed almost all layout-driven MIR
misses and reduced measured latency, especially with full debuginfo.

**Implication:** FERRIUM uses the flag only to identify causality. Production
recommendations must preserve diagnostics, debuginfo, coverage, metadata, and
other source-facing behavior.

**Confidence:** High.

### FERRIUM-197: crate lint changes are correctness-relevant broad dependencies

**Sources:** [lint dependency control](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#crate-lint-dependency-and-failure-control).

**Observed behavior:** An equal-width crate lint substitution re-ran 1,001
type-check roots. Changing an equal-width allow into deny turned one successful
20-owner compile into 20 structured `unused_mut` errors.

**Implication:** Broad work is not false merely because types and MIR stay
equal. Diagnostic policy is observable behavior and must remain in the
dependency model unless replaced by an equivalent persisted result.

**Confidence:** High.

### FERRIUM-198: stable owner identity survives reserved-slot item insertion

**Sources:** [equal-offset item control](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#equal-offset-substitutions) and [stable identifiers](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html#a-question-of-stability-bridging-the-gap-between-compilation-sessions).

**Observed behavior:** Replacing fixed-width comment bytes with an unused item
produced one new type-check, MIR, and borrow root rather than invalidating
1,000 later owners.

**Implication:** Sequential in-memory IDs are not the primary problem in this
case. Stable owner keys work; the broad ordinary insertion result came from
source-position movement.

**Confidence:** High.

### FERRIUM-199: global next-solver impl-set dependencies can be much broader

**Sources:** [equal-offset impl-set control](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#trait-impl-set-control-without-source-movement) and [trait-solving research](2026-08-08-trait-solving-cost-reuse.md).

**Observed behavior:** A same-trait impl for a different concrete type left
caller offsets fixed. The old solver reused 1,000 callers; the global next
solver re-ran 1,001 type-check, MIR, and borrow roots. An unrelated-trait impl
stayed local in both.

**Implication:** Same-trait impl-set precision is a defensible minimized
upstream benchmark candidate. It is not a basis for selecting a solver
globally.

**Confidence:** High for the tested revision and fixture.

### FERRIUM-200: untouched provider work must be subtracted before attribution

**Sources:** [always-executed work](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#always-executed-and-non-persisted-work) and [persistence model](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html#the-real-world-how-persistence-makes-everything-complicated).

**Observed behavior:** Untouched profiles still ran crate-item, visibility,
HIR support, metadata, and dep-graph work. Not every query result is persisted
between compiler sessions.

**Implication:** Reports need an untouched baseline and a persisted-versus-
session-work classification. Nonzero provider count is not automatically edit
fan-out.

**Confidence:** High.

### FERRIUM-201: rustc-perf has patched scenarios but no standard edit-geometry axis

**Sources:** [rustc-perf boundary](perf-q17-query-invalidation/results/EXP-01-edit-frontier-span-dependency.md#rustc-perf-and-upstream-boundary), [scenario model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/scenario.rs), and [patch guidance](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md).

**Observed behavior:** Rustc-perf correctly supports benchmark-defined
incremental patches, but this research found no standardized paired controls
for moved versus fixed offsets, before versus after insertion, or same-trait
versus unrelated-trait impl changes.

**Implication:** Defensible contributions begin with small orthogonal patches
and maintainer review, not a FERRIUM benchmark replacement.

**Confidence:** Medium because the review was source and inventory based, not
a collector run.

### FERRIUM-202: invalidation visualization is a credible build-control-plane wedge

**Sources:** findings FERRIUM-189 through FERRIUM-201, [frontend session topology](2026-08-08-frontend-parallelism.md), and [reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md).

**Observed behavior:** The useful explanation spans files, owners, queries,
source coordinates, diagnostic policy, solver mode, cache persistence, output
mode, Cargo sessions, and machine resources.

**Implication:** FERRIUM can prototype a visual layer above Cargo and rustc
that shows:

- changed source ranges and owners;
- semantic versus layout movement;
- provider-execution and downstream-containment frontiers;
- stable identity and artifact-reuse boundaries;
- lint, debug, coverage, and solver reasons;
- concurrent Cargo/rustc sessions and machine pressure.

Rustc remains the compiler and Cargo remains the build graph. The prototype
must consume supported or explicitly compatibility-bound evidence and remain
read-only before any scheduling or compiler intervention.

**Confidence:** High for the explanation need; medium for product viability.

## Recommendations

### Adopt now

- Add edit position, moved owner count, fixed-offset control, provider
  execution, result color or containment, persistence status, output mode,
  lint policy, and solver mode to incremental evidence records.
- Compare every edit against an untouched run using the same command and cache
  mode.
- Distinguish semantic propagation, diagnostic dependency, layout-driven work,
  and non-persisted session work.
- Report the furthest downstream stage that actually executed.
- Preserve ordinary Cargo and rustc behavior as the primary workflow.
- Treat debug, coverage, diagnostics, metadata, and docs as observable outputs,
  not disposable noise.

### Prototype behind a compatibility boundary

- A read-only invalidation viewer joining source ranges, stable owners, query
  providers, red-green containment, and output stages.
- A source-layout overlay showing owners moved by an edit separately from
  owners semantically affected.
- A paired-edit benchmark generator that creates moved-offset and fixed-offset
  variants without recommending source padding.
- A query-delta classifier that subtracts untouched provider work and labels
  persisted versus session-only results.
- A nightly adapter for self-profile and MIR span evidence with explicit
  version checks and graceful unsupported-state reporting.
- rustc-perf candidate patches for layout-only shifts, fixed-width docs and
  attributes, insertion before/after owners, and same-trait impl-set changes.
- Integration with the PERF-Q16 machine-session view so concurrent builds show
  both invalidation breadth and resource pressure.

### Reject or defer

- Enabling `-Zincremental-ignore-spans` in production.
- Padding, reordering, or rewriting source automatically to preserve offsets.
- Treating all broad work as false invalidation.
- Removing lint or source-location dependencies without correctness proofs.
- Selecting old or next solver from this one fixture.
- Reading rustc incremental cache internals as a stable public API.
- Building a compiler fork, query daemon, shared semantic cache, or automatic
  scheduler in this question.
- Opening upstream issues or pull requests without maintainer-approved
  reproduction on current Linux rustc-perf infrastructure.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: span and lint dependencies remain correctness-relevant; the testing-only control is not promoted to production. |
| Compiler Performance Engineer | Accepted: untouched baselines, fixed-offset controls, provider versus downstream work, output modes, distributions, and limitations are separated. |
| Interop Boundary Auditor | Accepted: no ABI, linker, FFI, or cross-language conclusion is inferred from compiler-query evidence. |
| AI Assurance Skeptic | Accepted: suspicious cases were minimized, confounders are named, and product opportunity is separated from compiler feasibility. |
| Ecosystem Strategist | Accepted: rustc and rustc-perf remain authoritative; FERRIUM contributes explanation and orthogonal fixtures. |
| Rust Maintainer | Accepted: ordinary Cargo remains primary, diagnostics stay actionable, and no source-padding ritual or compiler fork is proposed. |
| Native Platform Adopter | Accepted: nightly evidence is compatibility-bound and debug, diagnostic, reproducibility, and removal concerns remain explicit. |
| Scope Keeper | Accepted: the work answers query dependency precision and leaves fingerprint cost, early-phase incrementality, cross-crate reuse, and implementation to later questions. |
| Validation Checker | Accepted: commands, revisions, semantic controls, layout controls, failures, timing distributions, provider counts, and negative conclusions are recorded. |

## Non-goals

- Proving that all Rust crates have the same invalidation frontiers.
- Claiming every provider re-execution is avoidable.
- Designing a replacement for rustc's query engine.
- Changing diagnostic, debuginfo, coverage, metadata, or documentation
  behavior.
- Selecting one trait solver.
- Recommending source padding or generated reserved slots.
- Publishing raw session paths, caches, profiles, or temporary compiler
  artifacts.
- Creating an upstream issue, branch, comment, or pull request.
