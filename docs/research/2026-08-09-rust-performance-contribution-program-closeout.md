# Rust Performance Contribution Program Closeout

Date: 2026-08-09
Status: Complete
Question: PERF-Q36
Decision: close the 36-question Rust performance research sequence and open a
contribution-first Phase 4. Adopt a standard upstream contribution packet,
select a rustc-perf-compatible Relink-Don't-Rebuild body-versus-interface
benchmark as the first target, and require upstream owner alignment before any
issue, comment, benchmark, or patch is submitted. Keep the FERRIUM product
implementation gate closed pending a separately approved held-out maintainer
workflow and cross-platform proof.

## Decision supported

This research decides how FERRIUM should convert its measured Rust performance
corpus into contributions that upstream maintainers can review and sustain.

The program has completed 35 measured questions before this closeout, 41 dated
decision notes, more than 13,000 lines of research, synthetic and public
controls, 483 prior findings, a measurement contract, and nine-role review.
That evidence is large enough to establish a contribution system. It is not a
reason to fork rustc, Cargo, a backend, a linker, or their benchmark
infrastructure.

The correct next unit of work is an issue-specific **upstream performance
contribution packet**:

- one upstream home and maintainer question;
- one minimized, licensed, reproducible fixture;
- exact baseline and comparison revisions;
- named profiles, scenarios, metrics, and controlled edits;
- correctness, negative, and unsupported controls;
- observed versus inferred claims;
- local timing and profiling evidence;
- the requested upstream action;
- maintenance ownership and retirement conditions; and
- explicit approval before external publication.

## Corpus inventory

Local evidence reviewed:

- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
- [Rust latency component roadmap](2026-08-07-rust-latency-component-roadmap.md)
- [Incremental reuse and contribution boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Mid-program role checkpoint](2026-08-08-performance-program-role-checkpoint.md)
- [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [Crate slicing and partial compilation](2026-08-09-crate-slicing-partial-compilation.md)
- [Function-level machine-code caching](2026-08-09-function-level-machine-code-caching.md)
- [Impact-aware validation selection](2026-08-09-impact-aware-validation-selection.md)
- [Contribution readiness matrix](perf-q36-upstream-contribution/results/EXP-01-contribution-readiness-matrix.md)
- [Upstream contribution packet contract](../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md)

Program audit:

```text
PERF question files: 36
Complete before PERF-Q36: 35
Planned before PERF-Q36: 1
Dated research decision notes: 41
Research-note lines before PERF-Q36: 13,409
Prior finding sequence: FERRIUM-01 through FERRIUM-483
```

These counts describe the local corpus, not upstream acceptance or product
readiness.

## Official upstream workflow

### rustc and rustc-perf

The Rust compiler performance suite tracks merged compiler changes and
authorized PR try runs. External contributors can create issues, minimized
cases, local comparisons, profiles, and rustc-perf pull requests, but
authorized Rust team members trigger official perf runs:

- <https://rustc-dev-guide.rust-lang.org/tests/perf.html>
- <https://perf.rust-lang.org/help.html>
- <https://github.com/rust-lang/rustc-perf>

Rustc-perf distinguishes real-world Primary benchmarks from Secondary
benchmarks that isolate one compiler behavior or regression. Its documented
compile-benchmark intake recommends a two-commit benchmark addition and
requires local timing evidence, registration, configuration, lockfile, README,
and licensing updates:

- <https://github.com/rust-lang/rustc-perf/blob/main/collector/compile-benchmarks/README.md>
- <https://github.com/rust-lang/rustc-perf/blob/main/collector/README.md>
- <https://github.com/rust-lang/rustc-perf/blob/main/docs/glossary.md>
- <https://github.com/rust-lang/rustc-perf/blob/main/docs/comparison-analysis.md>

The official metric and scenario vocabulary includes instruction counts,
cycles, wall time, peak RSS, check/debug/optimized profiles, and full,
incremental-full, unchanged, and patched incremental scenarios. Official
tracked compiler performance currently centers on
`x86_64-unknown-linux-gnu`.

Substantial compiler changes should be discussed with the compiler team before
implementation, and large changes may require a Major Change Proposal:

- <https://rustc-dev-guide.rust-lang.org/contributing.html>
- <https://forge.rust-lang.org/compiler/proposals-and-stabilization.html>

### Cargo

Cargo asks contributors to work from accepted issues. Its contributor guide
states that only explicitly accepted issues will be reviewed:

- <https://github.com/rust-lang/cargo/blob/master/CONTRIBUTING.md>
- <https://doc.crates.io/contrib/>

Cargo maintains local Criterion benchmarks and documents open opportunities
for end-to-end Cargo executable, freshness/fingerprinting, and pathological
resolver benchmarks:

- <https://github.com/rust-lang/cargo/blob/master/benches/README.md>

This is a separate ownership path from rustc-perf's compiler benchmark suite.

## Findings

### FERRIUM-484: the research corpus is ready for contribution triage

**Sources:** 36-question registry, 35 completed question files before PERF-Q36,
41 dated decision notes, measurement contract, and role reviews.

**Observed behavior:** The corpus contains repeatable commands, fixture
revisions, controlled edits, positive and negative cases, limitations, public
controls, and explicit upstream boundaries across Cargo, rustc, codegen,
linking, environment, modularization, and validation.

**Implication:** FERRIUM can stop expanding the initial performance backlog and
start converting selected cases into upstream-ready packets.

**Confidence:** High.

### FERRIUM-485: upstream accepts bounded benchmark artifacts, not a research archive

**Sources:** rustc-perf compile-benchmark README, rustc-dev-guide performance
testing, and Cargo contributing guide.

**Observed behavior:** The upstream processes ask for a specific issue or
accepted change, a focused benchmark or reproducer, local measurements,
registration and licensing details, and reviewer interaction. They do not
provide an intake path for a 41-note downstream research corpus as one unit.

**Implication:** Each FERRIUM contribution must reduce to one maintainer
question and one reviewable artifact. The complete corpus remains supporting
provenance, not the submission payload.

**Confidence:** High.

### FERRIUM-486: minimized compiler regressions have a defined rustc-perf home

**Sources:** rustc-perf Primary, Secondary, and Stable benchmark categories.

**Observed behavior:** Secondary compile benchmarks explicitly include
artificial stress tests and minimized examples linked to historical compiler
performance issues.

**Implication:** Narrow FERRIUM compiler cases should target Secondary
benchmarks when they are interesting, stable, licensed, and aligned with an
upstream issue or owner.

**Confidence:** High.

### FERRIUM-487: upstream benchmark scenarios map cleanly to FERRIUM edit matrices

**Sources:** rustc-perf glossary and collector documentation; PERF-Q20 and
PERF-Q35 edit matrices.

**Observed behavior:** rustc-perf distinguishes full, incremental-full,
incremental-unchanged, and incremental-patched scenarios. FERRIUM already
separates clean, warm, no-op, controlled edit, revert, and correctness
controls.

**Implication:** Contribution packets should translate local terminology into
the upstream profile, scenario, and metric vocabulary rather than invent a
parallel benchmark dialect.

**Confidence:** High.

### FERRIUM-488: instruction counts and stable work precede wall-time claims

**Sources:** rustc-perf collector and comparison-analysis documentation;
FERRIUM system-effects decision.

**Observed behavior:** Rustc-perf defaults to instruction counts because they
vary less than wall time and applies significance and relevance analysis.
FERRIUM found that filesystem placement, virtualization, memory, concurrency,
security, and host state can dominate wall measurements.

**Implication:** A compiler contribution packet uses stable upstream metrics
for regression claims and retains wall time as user-impact evidence with a
named environment.

**Confidence:** High.

### FERRIUM-489: contribution throughput depends on reviewer alignment

**Sources:** rustc-dev-guide contribution and perf-run authorization
documentation; Cargo accepted-issue rule.

**Observed behavior:** External contributors cannot independently authorize
official compiler perf runs, merge benchmarks, or bypass Cargo's accepted
issue process. Large compiler changes require early team discussion.

**Implication:** Owner alignment is a pipeline stage, not administrative
cleanup after implementation. FERRIUM should ask whether a case is useful and
where it belongs before polishing a patch.

**Confidence:** High.

### FERRIUM-490: minimization must preserve the distinguishing behavior

**Sources:** FERRIUM positive, negative, oversensitivity, failure, and public
controls across PERF-Q17 through PERF-Q35.

**Observed behavior:** Several plausible optimizations disappeared or reversed
when a generic, downstream, release, debug, public-repository, or hidden-input
control was added. A smaller fixture can cease to represent the mechanism that
made the original case useful.

**Implication:** The contribution packet records minimization provenance and
reruns the distinguishing control after every reduction. Source size alone is
not a minimization success criterion.

**Confidence:** High.

### FERRIUM-491: the first target should be an RDR benchmark, not an RDR implementation

**Sources:** PERF-Q20 cross-crate interface matrix, official Fast Builds/RDR
direction, and rustc-perf Secondary benchmark conventions.

**Observed behavior:** The three-crate `base -> mid -> app` fixture contains 13
controlled source cases, reproducible metadata behavior, incremental-on and
incremental-off controls, positive body-edit candidates, and required inline,
generic, const, macro, layout, item-identity, and interface controls. Every
real edit currently rebuilt all three packages.

**Implication:** The first contribution target is an owner-aligned,
rustc-perf-compatible Secondary benchmark that contrasts non-inline body edits
with interface-sensitive edits. Its purpose is to measure current behavior and
future RDR progress, not to define or implement the interface hash.

**Confidence:** High for target selection; upstream acceptance remains unknown.

### FERRIUM-492: the RDR packet needs one more portability gate

**Sources:** PERF-Q20 limitations and rustc-perf tracked-platform
documentation.

**Observed behavior:** The current RDR evidence was collected on Windows MSVC,
while official rustc-perf tracking centers on `x86_64-unknown-linux-gnu`.

**Implication:** Before submission, reproduce the minimized fixture locally on
the upstream Linux target, translate edits into rustc-perf patch scenarios,
run the documented local comparison, and ask the RDR/rustc-perf owners whether
one benchmark or a smaller control set is preferred.

**Confidence:** High.

### FERRIUM-493: Cargo and compiler contributions require separate queues

**Sources:** Cargo contributing and benchmark documentation; rustc-perf
documentation; PERF-Q32.

**Observed behavior:** Cargo uses accepted issues and its own benchmarks.
Rustc-perf measures compiler behavior. PERF-Q32 already names Cargo issue
`#15644` for positive and negative `hint-mostly-unused` evidence.

**Implication:** FERRIUM must not send Cargo runtime or policy evidence to
rustc-perf merely because both involve Rust builds. The secondary queue starts
with the Cargo `#15644` evidence packet after owner approval.

**Confidence:** High.

### FERRIUM-494: sustained maintenance is part of the contribution

**Sources:** rustc-perf benchmark update policy, collector ownership, and Cargo
accepted-issue process.

**Observed behavior:** Benchmarks require licensing, registration, stable
configuration, later dependency updates, noise investigation, and possible
retirement or replacement. Review and triage remain upstream-owned.

**Implication:** Every packet names a FERRIUM maintainer, expected upstream
owner, update obligation, and exit condition. A drive-by benchmark without a
maintenance plan does not satisfy PERF-Q36.

**Confidence:** High.

### FERRIUM-495: funding and review support can outperform downstream code

**Sources:** compiler performance survey reviewed in the opening research,
rustc team review workflow, and the completed corpus.

**Observed behavior:** Major compiler improvements are constrained by domain
knowledge, implementation time, cross-platform validation, review capacity,
and sustained ownership. FERRIUM's comparative advantage is producing cases,
profiles, negative controls, documentation, and review-ready evidence.

**Implication:** Phase 4 may fund or support upstream owners, maintain
benchmarks, reproduce regressions, and perform local profiling instead of
assuming the valuable output is always a FERRIUM-authored compiler patch.

**Confidence:** High.

### FERRIUM-496: contribution completion and product authorization are separate gates

**Sources:** mid-program role checkpoint, measurement contract, PERF-Q30,
PERF-Q35, and this final role review.

**Observed behavior:** The research sequence has closed its component
questions, but it has not yet shown that one read-only product workflow reduces
maintainer investigation effort across held-out public repositories on
Windows and Unix with an adoption and removal contract.

**Implication:** PERF-Q36 completes the research backlog and opens the upstream
contribution program. It does not open FERRIUM product implementation.

**Confidence:** High.

### FERRIUM-497: the next decision is one bounded proof, not more backlog

**Sources:** Scope Keeper and Rust Maintainer objections from the mid-program
checkpoint; completed PERF-Q01 through PERF-Q36.

**Observed behavior:** The program now has enough vocabulary and candidate
capabilities. Continuing to add research lanes would increase complexity
without testing user value.

**Implication:** After the first contribution packet, the next portfolio
decision should select one removable maintainer workflow for Wave 4's bounded
proof or explicitly defer implementation.

**Confidence:** High.

## Contribution priority

| Priority | Packet | Upstream home | Readiness | Remaining gate |
|---:|---|---|---|---|
| 1 | RDR body-versus-interface incremental benchmark | rustc-perf Secondary benchmark plus RDR owner discussion | Selected | Linux reproduction, upstream scenario translation, owner alignment |
| 2 | `hint-mostly-unused` positive and negative matrix | Cargo issue `#15644` | Evidence ready | Owner approval and concise issue-format packet |
| 3 | Function-cache correctness and economics controls | rustc/rustc_codegen_cranelift goal owners | Research ready | Upstream sponsor and integrated experiment |
| 4 | Stable linker-input identity control | rustc and linker owners | Partial | Linux/macOS and native-link controls |
| 5 | Build-script unchanged-output and input-precision cases | Cargo accepted issue path | Partial | Representative public script and accepted owner question |

The [readiness matrix](perf-q36-upstream-contribution/results/EXP-01-contribution-readiness-matrix.md)
records why the first target wins.

## Decision

### Adopt now

- Freeze the
  [upstream contribution packet contract](../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).
- Create one packet at a time from the completed corpus.
- Start with the RDR body-versus-interface benchmark proposal.
- Translate local metrics and edits into upstream vocabulary.
- Preserve negative and correctness controls during minimization.
- Ask the relevant owner before filing, commenting, or opening a PR.
- Track maintenance, review requests, disposition, and supersession.
- Keep upstream acceptance or a documented external disposition as the Phase 4
  exit gate.

### Prototype behind a compatibility boundary

- A local packet assembler that reads existing FERRIUM evidence and emits a
  reviewable draft without posting externally.
- Disposable rustc-perf benchmark adaptation and `bench_local` runs.
- Local compiler profiling with `x perf`, `profile_local`, self-profile, or
  Cachegrind when the upstream question requires it.
- Cargo Criterion benchmark prototypes only after an accepted issue identifies
  the desired behavior.

### Reject or defer

- automatic issue, comment, or pull-request creation;
- performance claims from wall time alone;
- benchmark minimization that removes the distinguishing control;
- unlicensed or non-redistributable source in public fixtures;
- unaudited private repository content;
- bypassing Cargo accepted-issue or rustc owner-alignment processes;
- a FERRIUM compiler, Cargo fork, perf service, collector machine, backend,
  linker, or benchmark dashboard;
- product implementation before the separate held-out maintainer gate; and
- treating submission as success when upstream declines or cannot maintain the
  artifact.

## Role review

### Rust Safety Steward

Accepts contribution-first work because correctness-sensitive controls remain
attached to each performance case. Requires fail-closed unknowns and rejects
speed claims that remove safety, interface, ABI, release, or validation
coverage.

### Compiler Performance Engineer

Accepts the rustc-perf profile, scenario, stable-metric, local-comparison, and
Secondary-benchmark alignment. Requires the first RDR target to reproduce on
the upstream Linux target before submission.

### Interop Boundary Auditor

Accepts because native, ABI, target, linker, generated, macro, and build-script
cases remain separate packets with explicit owners. No cross-language evidence
is generalized from the initial Rust-only RDR fixture.

### AI Assurance Skeptic

Accepts the packet's observed/inferred/predicted separation, source citations,
negative controls, limitations, and human approval. Rejects automated external
posting and AI-authored confidence without maintainer review.

### Ecosystem Strategist

Accepts rustc-perf, rustc, Cargo, Cranelift, and linker owners as authoritative.
Supports fixtures, profiling, review capacity, and funding over replacement
infrastructure.

### Rust Maintainer

Accepts one issue-specific packet with exact commands and a clear ask. Requires
FERRIUM to maintain or retire accepted fixtures and avoid attaching the entire
research archive to routine upstream review.

### Native Platform Adopter

Accepts the contribution program but not product deployment. Requires
cross-platform evidence, ordinary CI, rollback, and removal before any
FERRIUM workflow becomes operationally required.

### Scope Keeper

Accepts PERF-Q36 as the end of the planned performance research ladder. The
next allowed work is the first contribution packet and a separately approved
bounded proof, not another unbounded research expansion.

### Validation Checker

Accepts the corpus audit, official workflow citations, readiness matrix,
portable first-target gate, packet contract, maintenance obligation, and
explicit separation between research completion and product authorization.

## Program status

```text
PERF-Q01 through PERF-Q36: complete
initial Rust performance research backlog: closed
Phase 4 upstream contribution program: open
first target: RDR body-versus-interface benchmark proposal
external submission: requires owner approval
FERRIUM product implementation gate: closed
TRACKER registration: separately blocked by unrelated TRACKER work
```
