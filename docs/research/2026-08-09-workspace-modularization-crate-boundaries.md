# Workspace Modularization and Crate Boundaries

Date: 2026-08-09
Question: PERF-Q34
Status: Complete
Decision: adopt a read-only, workload-weighted crate-boundary ledger and
counterfactual advisor model; report clean parallelism, serial depth, edit
containment, downstream fan-out, rustc invocation and metadata multiplication,
generic ownership, test and link multiplication, storage, and non-performance
boundary reasons; prototype recommendations only against reversible synthetic
counterfactuals and held-out repositories; reject automatic crate splitting,
combining, source movement, manifest rewrites, public-API redesign, and
universal crate-count guidance.

## Executive conclusion

Crate boundaries are graph cuts, not a generic compile-time optimization.

A matched fixture placed the same 1,280 functions into one library crate, two,
four, eight, or sixteen sibling crates, or an eight-crate dependency chain.
Eight sibling crates produced the fastest clean build at 954.6 milliseconds,
15.3% below the one-crate median. The serial chain took 1,293.4 milliseconds,
14.7% above the one-crate median. Parallel width helped; serial depth hurt.
Neither result identifies an ideal crate count.

The clean-build result concealed substantial added work. Sixteen sibling
crates used 1.60 median CPU seconds versus 0.57 seconds for the one-crate
control. Cargo overlapped the extra rustc invocations, so wall time still fell
9.8%. Additional boundaries traded machine work for elapsed parallelism.

Local edit latency did not improve materially. A private helper edit in the
one-crate control rebuilt its library and application in about 0.93 seconds.
Sibling layouts also rebuilt one library and the application in roughly
0.90-0.94 seconds. Rustc's existing module-oriented incremental codegen
contained most unaffected work inside the larger crate. A crate split did not
replace an absent incremental boundary; it duplicated a boundary rustc already
used effectively.

Dependency position mattered more than crate count. Editing the final crate in
the serial chain rebuilt two artifacts. Editing its foundation crate rebuilt
all eight libraries and the application and took 27.9% longer than the flat
edit. In the public PARLOR workspace, a private unused item in `parlor-go`
rebuilt that leaf and the CLI, while the same edit shape in `parlor-core`
rebuilt all six packages and used 2.17x the CPU.

Tests exposed the largest fragmentation cost. The flat workspace compiled
three artifacts for `cargo test --workspace --no-run`. Sixteen sibling
libraries compiled 33 and used 9.40 CPU seconds, about nine times the flat
control. Its test wall time was 61.3% slower despite broad parallelism.

Generic code can also multiply across boundaries. Eight calls to one generic
kernel produced one `shared_kernel::<u64>` mono item when the callers shared a
crate and eight items when each caller lived in a sibling crate. The split
control used 222.2% more CPU and 143.5% more target bytes. Its wall regression
was only 5.3% and remained within noisy short-run evidence; the ownership,
CPU, and bytes establish the duplication.

FERRIUM should therefore explain boundaries rather than prescribe them. A
candidate split must show that clean-build or edit-path savings exceed startup,
metadata, generic, linking, testing, storage, and maintenance costs for the
repository's actual workload mix. A candidate merge must preserve independent
reuse, semver, feature, ownership, platform, safety, and operational reasons
that may outweigh build latency.

The advisor remains read-only. It may construct synthetic counterfactuals,
rank measured opportunities, and name confidence and non-performance
constraints. It must not move source, rewrite manifests, change APIs, merge
ownership domains, or claim that a crate count is intrinsically good.

## Decision supported

This research determines:

- when sibling crates expose useful clean-build parallelism;
- why serial crate chains can lengthen the critical path;
- whether crate boundaries improve local incremental edits beyond rustc's
  existing module and codegen-unit reuse;
- how dependency position changes the rebuild cone;
- how package boundaries multiply test and link targets;
- how generic instances duplicate across sibling consumers;
- which non-performance reasons remain part of a crate boundary;
- what evidence a measured modularization advisor must require; and
- whether FERRIUM should restructure repositories automatically.

It does not authorize source movement, manifest edits, API redesign, package
publication changes, feature changes, test selection, compiler changes, or
implementation.

## Evidence reviewed

### Local evidence

- [EXP-01 crate boundary response matrix](perf-q34-workspace-modularization/results/EXP-01-crate-boundary-matrix.md)
- [Cargo graph scheduling and critical paths](2026-08-08-cargo-graph-scheduling.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [Monomorphization and generic-instance reuse](2026-08-09-monomorphization-generic-instance-reuse.md)
- [Crate slicing and partial dependency compilation](2026-08-09-crate-slicing-partial-compilation.md)
- [System effects on build latency](2026-08-09-system-effects-build-latency.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Upstream and ecosystem sources

- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo timings](https://doc.rust-lang.org/cargo/reference/timings.html)
- [Rust compiler monomorphization model](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
- [Rust Performance Book compile-time guidance](https://nnethercote.github.io/perf-book/compile-times.html)
- [Rust 2026 Fast Builds roadmap](https://github.com/rust-lang/goals/blob/main/src/2026/roadmap-fast-builds.md)

Cargo workspaces coordinate related packages through a shared lockfile, output
directory, and workspace commands. They do not make every member one compiler
unit. The Rust Performance Book recommends Cargo timings to identify large
crates that serialize compilation and may be split, not to infer a split from
size alone. The Fast Builds roadmap likewise treats better parallelization and
Relink-Don't-Rebuild as separate end-to-end opportunities.

## Crate-boundary model

FERRIUM distinguishes:

1. **Logical boundary:** modules, visibility, ownership, and concepts within
   one crate.
2. **Compilation boundary:** one rustc crate invocation and its metadata,
   archive, incremental state, and codegen ownership.
3. **Package boundary:** one Cargo package with manifest, features, targets,
   tests, publication, and semver identity.
4. **Workspace boundary:** packages coordinated by one workspace root,
   lockfile, target directory, and command selection.
5. **Parallel-width effect:** independent ready crates Cargo can compile
   concurrently.
6. **Serial-depth effect:** dependency edges that force one crate to wait for
   another.
7. **Edit-containment effect:** work avoided because an edit remains inside one
   crate or one rustc incremental reuse frontier.
8. **Downstream fan-out:** dependent crates that Cargo invokes after the edited
   crate changes.
9. **Invocation multiplication:** additional rustc startup, sysroot,
   metadata, hashing, archive, and process cost.
10. **Generic ownership:** the crate that collects and emits each concrete
    generic instance.
11. **Target multiplication:** additional library, binary, test, example,
    benchmark, build-script, and documentation targets.
12. **Link multiplication:** additional executables, test harnesses, archives,
    and final links.
13. **Boundary stability:** how often code on each side changes together and
    whether the public interface is narrower than the implementation.
14. **Non-performance boundary:** independent reuse, publication, semver,
    features, platform, capability, unsafe review, ownership, security, or
    operational reasons for separation.
15. **Counterfactual topology:** a reversible synthetic or disposable
    alternative used for measurement, not a proposed patch.

## Findings

### FERRIUM-460: crate boundaries change several cost systems at once

**Sources:** EXP-01; Cargo workspace documentation; PERF-Q03, Q20, and Q24.

**Observed behavior:** one boundary changed Cargo graph width and depth, rustc
invocation count, metadata edges, incremental ownership, generic emission,
test targets, linking, CPU, wall time, and target bytes.

**Implication:** crate count and source lines per crate are not sufficient
advisor inputs. Recommendations require a workload-weighted graph and artifact
model.

**Confidence:** High.

### FERRIUM-461: sibling width can shorten clean wall time by spending more CPU

**Sources:** EXP-01 matched topology clean builds.

**Observed behavior:** eight sibling libraries completed 15.3% faster than the
one-library control. Sixteen siblings completed 9.8% faster but used 1.60 CPU
seconds versus 0.57 seconds.

**Implication:** a clean-build gain may be a parallelism trade, not avoided
work. Reports show wall, CPU, memory, variance, and ready width together.

**Confidence:** High for the fixture; low for a universal crate count.

### FERRIUM-462: serial crate depth can retain overhead without exposing width

**Sources:** EXP-01 `chain-8`; PERF-Q03.

**Observed behavior:** the eight-crate chain was 14.7% slower than the flat
clean build and 35.5% slower than the eight-sibling clean build.

**Implication:** splitting a serial subsystem can worsen the critical path.
The advisor should distinguish independent siblings from dependency chains.

**Confidence:** High for the controlled topology.

### FERRIUM-463: intra-crate incremental reuse can erase the expected edit gain

**Sources:** EXP-01 private-helper edits; PERF-Q17 and Q25.

**Observed behavior:** flat and sibling edit medians stayed near 0.9 seconds.
The flat crate rebuilt as one Cargo artifact, but rustc reused unaffected
module-oriented incremental work products.

**Implication:** a proposed split must measure what rustc already reuses inside
the crate. Large source size alone does not prove avoidable incremental work.

**Confidence:** High for the generated module shape.

### FERRIUM-464: dependency position determines the rebuild cone

**Sources:** EXP-01 serial-chain edits and PARLOR control.

**Observed behavior:** a chain leaf edit rebuilt two artifacts; a foundation
edit rebuilt nine. PARLOR's game-leaf edit rebuilt two packages; its shared-core
edit rebuilt all six.

**Implication:** change frequency must be joined to downstream fan-out. Stable
foundations and volatile leaves have different boundary economics.

**Confidence:** High.

### FERRIUM-465: current downstream invalidation limits boundary isolation

**Sources:** EXP-01; PERF-Q20; Rust Fast Builds roadmap.

**Observed behavior:** behavior-neutral private-item edits still rebuilt every
Cargo dependent. A crate boundary isolated unaffected siblings but did not
prune dependents.

**Implication:** modularization advice must model current behavior and an
explicit future RDR scenario separately. Do not sell crate splitting as a
substitute for Relink-Don't-Rebuild.

**Confidence:** High.

### FERRIUM-466: package tests can dominate fragmentation cost

**Sources:** EXP-01 clean `cargo test --workspace --no-run`.

**Observed behavior:** the flat topology compiled three artifacts and used
1.04 CPU seconds. Sixteen sibling libraries compiled 33 artifacts and used
9.40 seconds, with 61.3% higher wall time.

**Implication:** every modularization comparison includes test-harness count,
link count, selected validation command, and test execution policy. Build-only
advice is incomplete.

**Confidence:** High for the fixture.

### FERRIUM-467: no-op freshness did not depend materially on crate count

**Sources:** EXP-01 warm no-op rows.

**Observed behavior:** all no-op medians were 29.4-36.1 milliseconds and
reported zero compiled artifacts.

**Implication:** the measured boundary was compilation and target
multiplication, not a large Cargo freshness regression at sixteen local
packages.

**Confidence:** High for this small dependency-free workspace.

### FERRIUM-468: sibling generic consumers duplicate concrete instances

**Sources:** EXP-01 generic control; PERF-Q24; rustc monomorphization guide.

**Observed behavior:** one consumer emitted one `shared_kernel::<u64>` mono
item. Eight sibling consumers emitted eight, used 222.2% more CPU, and wrote
143.5% more target bytes.

**Implication:** a boundary advisor includes concrete generic families and
owner crates. Moving generic-heavy callers across sibling boundaries can
trade isolation for repeated backend work.

**Confidence:** High for ownership, CPU, and bytes; inconclusive for the 5.3%
wall difference.

### FERRIUM-469: parallel wall gains can conceal invocation and metadata cost

**Sources:** EXP-01 clean CPU, artifact, and wall evidence; PERF-Q08.

**Observed behavior:** sibling topologies often completed sooner while
compiling more artifacts and consuming more CPU. The chain exposed the same
overhead without parallel overlap.

**Implication:** wall time remains primary for users, but the advisor must
surface machine cost, CI capacity, energy, memory, and contention implications.

**Confidence:** High.

### FERRIUM-470: non-performance reasons can outweigh measured latency

**Sources:** Cargo workspace model; FERRIUM engineering principles.

**Observed behavior:** the experiment held package semantics artificial and
did not model publication, semver, features, platform, ownership, security, or
unsafe review boundaries.

**Implication:** a slower crate boundary can still be correct architecture.
The advisor records these constraints and never reduces them to a timing score.

**Confidence:** High on the boundary; repository-specific weight is unknown.

### FERRIUM-471: recommendations require reversible counterfactual evidence

**Sources:** EXP-01 limitations; all nine FERRIUM role reviews.

**Observed behavior:** clean, edit, test, generic, chain, and public controls
produced different winners. No topology won every workload.

**Implication:** FERRIUM may rank candidates only after measuring a disposable
counterfactual against the repository's workload mix. It must preserve
uncertainty and reject automatic restructuring.

**Confidence:** High.

## Advisor decision model

```text
candidate boundary
  -> name non-performance reason
  -> classify sibling width and serial depth
  -> join historical edit frequency to dependent fan-out
  -> measure current intra-crate incremental reuse
  -> attribute generic and inline ownership
  -> count rustc, metadata, archive, link, and test multiplication
  -> run clean, no-op, local edit, foundation edit, revert, and validation
  -> compare wall, CPU, memory, bytes, variance, and behavior
  -> label confidence and portability
  -> human decision; no automatic rewrite
```

Candidate signals for a split:

- one crate is a measured serial bottleneck;
- independent modules can become genuinely ready siblings;
- volatile code has a narrow dependent cone;
- the new interface is stable and materially narrower;
- generic and inline work will not multiply disproportionately; and
- test and link multiplication remains acceptable.

Candidate signals for a merge:

- a chain contains many tiny crates that always change and validate together;
- package boundaries provide no independent reuse, publication, feature,
  ownership, platform, or assurance value;
- repeated generic instances or metadata/startup costs dominate;
- test harnesses and links multiply validation cost; and
- a disposable merged counterfactual improves the representative workload.

These are measurement prompts, not source-change instructions.

## Recommendations

### Adopt now

1. Add crate-boundary, graph-position, edit-frequency, generic-ownership, and
   target-multiplication nodes to the compiler query plan.
2. Record clean width, serial depth, dependent fan-out, rustc invocation count,
   CPU, wall, memory, bytes, and test/link targets for every modularization
   claim.
3. Require clean, no-op, leaf edit, foundation edit, revert, and test
   compilation workloads.
4. Preserve synthetic flat, sibling, chain, generic, and test controls.
5. Treat non-performance boundary reasons as first-class constraints.

### Prototype behind a compatibility boundary

1. Build a read-only boundary ledger from stable Cargo metadata, compiler
   artifact messages, timings, repository history, and optional versioned
   rustc diagnostics.
2. Generate disposable counterfactual fixtures without changing the owner
   worktree.
3. Evaluate candidate ranking on held-out public workspaces with distinct
   topologies.
4. Model current behavior and future RDR assumptions separately.
5. Require human review before any repository architecture proposal.

### Reject or defer

- automatic crate splitting or combining;
- source movement or module rewriting;
- automatic `Cargo.toml`, feature, workspace, or package changes;
- public API, semver, visibility, ownership, or unsafe-boundary changes;
- a universal crate-size or crate-count threshold;
- advice based only on source lines, compile duration, or fan-out;
- counting reduced validation as a modularization gain;
- compiler, Cargo, linker, or scheduler replacement; and
- implementation before held-out repository and role gates pass.

## Role review

### Rust Safety Steward

Accepts a read-only advisor. Crate boundaries can encode unsafe review and
capability isolation, so performance evidence cannot merge them automatically.

### Compiler Performance Engineer

Accepts the matched clean, incremental, no-op, chain, generic, test, CPU,
memory, bytes, variance, and public controls. Requires larger and
cross-platform held-out fixtures before product claims.

### Interop Boundary Auditor

Accepts because no ABI or package boundary is changed. Any future merge or
split involving FFI, native libraries, panic, allocator, or threading
contracts requires a separate boundary review.

### AI Assurance Skeptic

Accepts the explicit negative result that splitting did not improve local edit
latency. Requires generated counterfactuals, commands, behavior checks,
failures, and human approval to remain visible.

### Ecosystem Strategist

Accepts a Cargo-compatible diagnostic rather than a workspace manager.
Recommends contributing minimized compiler or Cargo cases upstream when the
advisor finds mechanism gaps.

### Rust Maintainer

Accepts actionable explanations over automatic patches. Diagnostics must say
which workload, edit cone, and artifact cost produced the recommendation and
must remain removable.

### Native Platform Adopter

Accepts the rollback and ordinary-Cargo boundary. Requires CI capacity,
debugging, packaging, compliance, ownership, and training costs in any future
architecture proposal.

### Scope Keeper

Accepts PERF-Q34 as one bounded research decision. Implementation, repository
rewrites, and PERF-Q35 validation selection remain closed.

### Validation Checker

Accepts the pinned environment, warm-up, five-run medians and MADs, negative
controls, artifact counts, public revision, limitations, and retained
reproduction bundle.

## Validation and retained evidence

Primary commands:

```text
wsl.exe -d Ubuntu-24.04 -- bash -lc "python3 measure_modularization.py"
wsl.exe -d Ubuntu-24.04 -- bash -lc "python3 measure_controls.py"
```

Retained evidence:

- generated-fixture and public-control harnesses;
- raw per-run JSON;
- medians and MADs;
- Cargo compiler-artifact package sets;
- `/usr/bin/time -v` resource fields; and
- mono-item diagnostic lines.

Generated source and target trees were removed.

## Follow-on

PERF-Q35 can use the crate-boundary ledger when evaluating validation impact,
but it must not treat package boundaries as proof that tests outside the
dependent cone are unnecessary.
