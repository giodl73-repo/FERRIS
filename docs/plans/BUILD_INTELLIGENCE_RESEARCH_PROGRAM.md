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
| Procedural macros and build scripts | Arbitrary execution and incomplete input declarations make reuse and invalidation difficult | Inventory inputs, reruns, fan-out, generated work, and negative cases before caching |
| Generics and monomorphization | Concrete downstream instantiations repeat code generation and interact with inlining and LTO | Measure duplicate instances and identify safe research boundaries |
| Codegen and optimization | LLVM optimization, codegen-unit choices, and release profiles trade iteration time for runtime quality | Distinguish development and release needs and evaluate supported backend/profile choices |
| Debug information and object emission | Large native objects and debug records add CPU, memory, and filesystem work | Separate emission cost from semantic compilation |
| Linking | Binary size, native dependencies, debug data, and linker behavior may dominate the final wait | Measure relink-only paths and evaluate existing incremental linkers |
| Cache topology | Workspaces, CI jobs, profiles, targets, and repositories may rebuild compatible work under different identities | Define cache identity and explain misses before distributing artifacts |
| System environment | Filesystem crossings, VM limits, job policy, memory pressure, security scanning, indexing, power, and thermal state can dominate or destabilize wall time | Fingerprint the environment, guard comparisons, and use supported traces before attribution |
| IDE and validation loop | Check, test compilation, doctests, examples, and editor actions may duplicate nearby work | Model time from edit to trustworthy feedback rather than one Cargo command |

FERRIUM should help first by making these causes observable and comparable. It
should modify rustc only after a representative fixture isolates a specific
upstream problem. The compiler, crate, query, generic, codegen, link, cache, and
validation boundaries are mapped in
[Rust incremental reuse scopes and contribution boundaries](../research/2026-08-07-rust-incremental-reuse-boundaries.md).

## Capability sequence

### BI-01: Build-shape census

Classify representative repositories by crate graph, target types, features,
macros, build scripts, generics, native dependencies, profiles, and linking.

**Research output:** frozen fixture revisions and baseline evidence.

### BI-02: Post-build causality

Explain direct package work, downstream metadata invalidation, downstream
codegen, macro or build-script reruns, cache misses, and relinking.

**Research output:** a causal vocabulary validated against controlled edits
and a planned-versus-observed compiler query-plan model. Cross-crate plans
separate upstream compilation, interface comparison, retained-artifact
compatibility, downstream pruning, and final linking according to the
[Relink-Don't-Rebuild decision](../research/2026-08-08-relink-dont-rebuild.md).
It also separates exact artifacts from shared compiler stages across check,
build, Clippy, test, documentation, and doctest according to the
[cross-command reuse decision](../research/2026-08-08-command-artifact-reuse.md).
Procedural-macro plans separately expose invocation, native execution,
declared and hidden inputs, cached derive output, generated Rust, and later
compiler work according to the
[procedural-macro decision](../research/2026-08-08-procedural-macro-cost-input-reuse.md).
Build-script plans separately expose host compilation, run identity, default
or declared detection, hidden inputs, replayed instructions, effective
outputs, persistent output ownership, native metadata, and downstream fan-out
according to the
[build-script decision](../research/2026-08-09-build-script-input-output-precision.md).
Generic plans separately expose definition families, concrete substitutions,
collection, owner crate, upstream reuse, sibling duplication, emitted symbols,
linker folding, final retention, and cross-workspace repetition according to
the
[monomorphization decision](../research/2026-08-09-monomorphization-generic-instance-reuse.md).
Backend partition plans separately expose requested maximum, initial stable
and volatile units, inline copies, estimated size and overlap, merge lineage,
actual CGUs, pre- and post-LTO work products, memory, link cost, output size,
runtime controls, and partition stability according to the
[codegen-unit decision](../research/2026-08-09-codegen-unit-partitioning.md).
LLVM plans separately expose IR translation, pre-link optimization, LTO/import
optimization, nested module/SCC/function/loop pass events, machine instruction
selection and register allocation, emission, observer effect, and final
runtime and size controls according to the
[LLVM optimization decision](../research/2026-08-09-llvm-optimization-cost.md).
Development-backend plans separately expose shared frontend work, replaceable
codegen share, exact backend component, target and capability eligibility,
isolated artifact identity, clean and incremental outcomes, panic and failure
behavior, test execution, runtime, and mandatory LLVM validation according to
the
[development backend decision](../research/2026-08-09-development-codegen-backends.md).
Function-cache plans separately expose the Rust semantic envelope,
monomorphized instance, backend stencil, finalization parameters, admission
decision, population cost, hit or miss, restoration cost, integrity
disposition, invalidated CGU, stable neighboring functions, and daemon
lifecycle according to the
[function-level cache decision](../research/2026-08-09-function-level-machine-code-caching.md).
Partial-dependency plans separately expose declared public non-generic,
generic, private, trait, impl, macro, static, and generated surfaces;
whole-crate frontend work; hint eligibility; dependency-owned codegen;
consumer-owned codegen; repeated consumer emission; final retention; and
end-to-end outcome according to the
[crate-slicing decision](../research/2026-08-09-crate-slicing-partial-compilation.md).
System-environment plans separately expose host identity, execution substrate,
source, target, Cargo home, temp, cache and linker-state placement, cache-layer
assumptions, Cargo and rustc jobs, jobserver domains, memory limits and reserve,
concurrent sessions, security and indexing state, power and thermal evidence,
background pressure, and attribution confidence according to the
[system-effects decision](../research/2026-08-09-system-effects-build-latency.md).
Workspace-modularization plans separately expose logical, compilation, package,
and workspace boundaries; sibling width and serial depth; edit containment;
downstream fan-out; rustc invocation and metadata multiplication; generic
ownership; test, link, and target multiplication; boundary stability;
non-performance constraints; and disposable counterfactual outcomes according
to the
[crate-boundary decision](../research/2026-08-09-workspace-modularization-crate-boundaries.md).
Debug-emission plans separately expose effective debug level and origin,
source, line, procedure, local, and type capability, LLVM processing, object
and archive bytes, incremental storage, linker input, split-debug packaging,
PDB/DWARF/dSYM output, stripping, and interactive debugger validation
according to the
[debug emission decision](../research/2026-08-09-debug-information-object-emission.md).
Link plans separately expose object and library identity, linker engine and
version, target and ABI policy, debug packaging, dead-code and folding policy,
complete-link cost, incremental-state request and eligibility, changed/added/
removed modules, state reuse or invalidation, fallback reason, output and state
bytes, release finalization, and validation according to the
[linking decision](../research/2026-08-09-linking-incremental-linking.md).

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

PERF-Q35 authorizes a read-only validation-plan and coverage ledger and a
bounded package-selection prototype. Package scope must be reported separately
from activity, feature, target, profile, platform, doctest, execution, and
repository-gate scope. Owned Rust changes use reverse dependency closure;
unknown, unowned, generated, root-policy, environment, build-script, macro,
native, or otherwise ambiguous inputs require a declared mapping or full
fallback. Selected-plan evidence is not full-reference evidence. Periodic full
runs, held-out mutation tests, false-negative review, exact commands, and human
approval remain required. Automatic gate deletion and required-CI replacement
remain outside BI-04.

### BI-05: Cache and workspace diagnosis

Explain artifact incompatibility, feature/profile divergence, workspace
topology, and build-script or macro inputs that prevent reuse.

**Research output:** measured experiments and reversible recommendations, not
automatic rewrites. Procedural-macro diagnosis must preserve tracked and
untracked input failures and must not enable rustc's experimental derive cache.
Build-script diagnosis must preserve hidden-input failures, distinguish saved
output replay from execution, and must not suppress compilation or clean
`OUT_DIR` without an explicit output and ownership contract.
Generic diagnosis must distinguish collected items, emitted symbols, archive
bytes, selected objects, folded aliases, and retained code. It must not
automatically force sharing, LTO, inlining, erasure, API changes, or writable
cross-workspace targets.
Codegen diagnosis must distinguish a requested maximum from actual CGUs and
must not optimize for hit rate, estimated size, wall time, runtime, memory, or
binary size alone. It must not automatically rewrite Cargo profiles, source
modules, crate boundaries, inlining, LTO, or linker settings.
LLVM diagnosis must distinguish requested rustc labels from the selected LLVM
pipeline and function attributes, preserve nested pass scope, and calibrate
observer effect. It must not automatically inject LLVM arguments, rewrite
profiles or source, disable vectorization, change target features, enable LTO,
or select another backend.
Development-backend diagnosis must distinguish check, build, test compilation,
test execution, run, clean, warm, and incremental workflows; preserve target,
panic, runtime, failure, ABI, debug, and artifact differences; and keep LLVM
validation visible. It must not automatically select a backend, modify Cargo
profiles or configuration, switch CI or editor commands, share target roots,
or use Cranelift for release or unsupported capabilities.
Function-cache diagnosis must distinguish changed Rust semantic identity from
unchanged Cranelift stencils, separate CGU work-product reuse from per-function
reuse, report population and restoration economics, and preserve integrity,
debug, unwind, ABI, symbol, relocation, instrumentation, and optimization
boundaries. It must not invent an external key, restore machine code, launch a
daemon, persist unverified bytes, or generalize Cranelift evidence to LLVM,
LTO, release, remote, or cross-platform reuse.
Partial-dependency diagnosis must distinguish already-lazy metadata, generic
instantiation, and private unreachable code from avoidable public non-generic
dependency codegen. It must report whole-crate frontend work, profile and inline
eligibility, each consumer's demand, repeated downstream emission, final
retention, and net wall, CPU, memory, artifact, runtime, and output evidence.
It must not recommend from crate size or rlib shrinkage alone, silently edit
Cargo profiles, skip whole-crate errors, generalize development results to
release, or implement source-level or stub-rlib slicing.
System-environment diagnosis must distinguish project-cold from process,
page-cache, VM-cache, and physical-cold state; separate source from target
placement; and preserve VM, CPU, memory, security, indexing, power, thermal,
and background-pressure unknowns. It must not infer antivirus, indexing, or
thermal blame from wall time; apply exclusions; disable services or protection;
change power, affinity, priority, VM, swap, or job settings; force memory
pressure; migrate repositories; or call an environment gain a compiler
optimization.
Linker diagnosis must distinguish complete linking from prepared incremental
state, preserve object and library identity, `/OPT`, debug packaging, output,
ILK or engine state, fallback, release, ABI, native-library, runtime, signing,
and rollback evidence, and must not infer reuse from a requested flag. It must
not automatically select a linker, rewrite profiles or configuration, change
CGUs or source, retain or transport linker state, or alter CI, editor, signing,
deployment, or release policy.

### BI-06: Ferris build evidence packet

Combine revision, environment, commands, causal observations, prediction
accuracy, validation results, failures, and limitations into a reviewable
record.

**Research output:** the Hammer-specific input to Pulse 03's broader Ferris
evidence contract.

PERF-Q36 adds an upstream adaptation of this record. One issue-specific packet
names the upstream home, maintainer question, licensed reproducer, minimization
provenance, upstream profile/scenario/metric vocabulary, commands, correctness
controls, requested action, maintenance owner, and lifecycle. The packet is a
local draft until explicit approval permits external submission. The
[packet contract](../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md) is the
Phase 4 handoff.

### BI-07: Labeled Rust Build Forest

Represent immutable workspace build roots, mutable human labels, parent
lineage, Cargo unit edges, artifact references, atomic rustc cache-generation
references, generic-instance family and ownership summaries, validation
evidence, CGU partition and merge summaries, backend work-product dispositions,
LLVM optimization-stage and pass-cost summaries, machine-pass and emission
dispositions, development-backend eligibility and outcome summaries,
function-cache opportunity, stencil, admission, hit, miss, restoration,
integrity, and capability dispositions,
dependency-surface, hint-eligibility, frontend-work, dependency-codegen,
consumer-codegen, duplication, final-retention, and partial-compilation outcome
summaries,
host, substrate, placement, cache-layer, CPU-policy, memory, security,
indexing, power, thermal, background-pressure, and attribution summaries,
debug-level capability, object, archive, incremental, and packaged-symbol
summaries, linker-plan and state dispositions, changed/added/removed link
modules, fallback reasons, complete/prepared/final output identities, and
concurrent-session pressure without depending on rustc's
internal cache format or treating machine code as a portable cache entry.

**Research output:** the
[Rust Build Forest architecture decision](../research/2026-08-08-rust-build-forest-opportunity.md),
the PERF-Q30
[remote artifact provenance decision](../research/2026-08-09-remote-artifact-provenance.md),
the PERF-Q31
[function-level cache decision](../research/2026-08-09-function-level-machine-code-caching.md),
the PERF-Q32
[crate-slicing decision](../research/2026-08-09-crate-slicing-partial-compilation.md),
the PERF-Q33
[system-effects decision](../research/2026-08-09-system-effects-build-latency.md),
the PERF-Q34
[crate-boundary decision](../research/2026-08-09-workspace-modularization-crate-boundaries.md),
the PERF-Q35
[validation-selection decision](../research/2026-08-09-impact-aware-validation-selection.md),
and a bounded read-only local manifest, policy, and visualization prototype.
Signed roots, labels, and disposable exact-identity transport may be explored;
a production remote service, automatic restore, portable compiler-private
state, machine-code restoration, and automatic partial-compilation changes
remain outside this phase. Host configuration changes and environment tuning
also remain outside this phase. Automatic crate splitting, combining, source
movement, manifest rewrites, API redesign, and package-boundary changes remain
outside this phase.
Automatic validation-gate deletion, unknown-input skipping, full-suite
confidence claims for selected plans, and replacement of required repository
CI also remain outside this phase.

## Research questions

The next latency work should answer these questions in order:

These program-level questions are expanded into component-specific,
independently executable investigations in the
[Rust performance research-question registry](../research/questions/README.md).

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
- Use the PERF-Q36 packet contract and one owner-aligned case at a time.
- Start with a rustc-perf-compatible RDR body-versus-interface Secondary
  benchmark after Linux reproduction and owner confirmation.

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

## Closeout disposition

PERF-Q01 through PERF-Q36 are complete. The initial component research backlog
is closed.

The next approved work is:

1. align the selected RDR benchmark shape with rustc-perf and RDR owners;
2. reproduce it on `x86_64-unknown-linux-gnu`;
3. prepare the local contribution packet without external posting;
4. pursue upstream acceptance or record why the case remains external; and
5. separately decide whether one removable maintainer workflow has enough
   held-out, cross-platform, adoption, and rollback evidence for a bounded
   implementation pulse.

No additional performance backlog or FERRIUM product implementation is implied
by this closeout.

No product code is authorized by this plan.
