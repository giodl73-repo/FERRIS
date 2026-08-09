# Reuse Across Check, Build, Lint, Test, and Doctest

Date: 2026-08-08
Question: PERF-Q21
Status: Complete
Decision: adopt activity, target-stage, stage-dependency, exact-artifact,
compatible-dependency, tool-specific, coverage-specific, and ephemeral-output
vocabulary now; align future compiler work with Rust's accepted Incremental
Systems Rethought goal; retain read-only cross-command explanation in the
compiler query plan; defer a FERRIUM incremental compiler, artifact aliasing,
command substitution, or validation reduction.

## Executive conclusion

Cargo already reuses artifacts across commands when the artifact is genuinely
the same unit. It does not currently share the common compiler work beneath
different activities.

In a controlled workspace:

- check then build rebuilt all three aligned packages;
- build then check rebuilt all three aligned packages;
- all-target check and Clippy rebuilt the same eight targets through different
  compiler drivers;
- all-target check then test rebuilt all seven test artifacts;
- build then test reused two ordinary library artifacts and compiled five
  test-specific targets;
- test then build reused those two libraries and compiled the normal
  application binary;
- build and documentation reused no artifacts in either direction;
- documentation and doctest reused no artifacts in either direction;
- repeated doctest reused its two library dependencies but reran rustdoc test
  compilation every time;
- repeated test no-run made all seven compiler artifacts fresh.

This resolves the apparent contradiction between "Cargo already reuses work"
and "check and build compile twice."

Cargo's unit and artifact model can reuse whole outputs whose identities match.
Test benefits from this because its normal dependencies can use ordinary build
libraries. Check, build, Clippy, documentation, and doctest have different
activity, output, tool, or coverage identities, so whole-artifact reuse is
usually unavailable.

Much of their compiler pipeline still has a common semantic prefix. Check stops
after metadata analysis. Build performs that work and continues into codegen
and linking. Clippy adds lint-specific analysis. Tests add `cfg(test)`, harness,
dev-dependency, and target work. Rustdoc extracts and renders documentation;
doctest creates temporary test crates.

The missing capability is **target-stage reuse**: preserve a common compiler
base and extend it only through the additional stages required by the next
activity.

That is now an accepted upstream Rust direction. The 2026 Incremental Systems
Rethought goal proposes shared common bases between rustc invocations with
different arguments, explicitly naming check, build, Clippy, and some test
reuse. Its initial target is Build Over Check. The related target-stages RFC
remains open, and the pinned nightly exposes no target-stage option.

FERRIUM should contribute evidence and make the gap visible in the compiler
query plan. It should not create a parallel incremental compiler or pretend
that commands with different semantics are interchangeable.

## Decision supported

This research determines:

- which current command pairs share exact Cargo artifacts;
- which pairs repeat compatible compiler stages under distinct identities;
- which test, lint, documentation, and doctest work is required;
- why reuse can be directional or partial;
- how current Cargo locking and future compiler-stage reuse differ;
- what belongs in FERRIUM's explanation layer versus upstream rustc and Cargo.

It does not authorize command substitution, skipped lints, reduced test
coverage, doctest disabling, artifact filename aliasing, incremental-directory
merging, compiler forks, or product implementation.

## Evidence reviewed

### Local evidence

- [Cross-command reuse matrix](perf-q21-command-artifact-reuse/results/EXP-01-command-reuse-matrix.md)
- [Cargo build-unit identity](2026-08-07-cargo-build-unit-identity.md)
- [Cargo build-unit multiplication](2026-08-08-cargo-build-unit-multiplication.md)
- [Editor and Cargo contention](2026-08-08-editor-cargo-contention.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Performance program role checkpoint](2026-08-08-performance-program-role-checkpoint.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Upstream direction

- [2026 Incremental Systems Rethought goal](https://github.com/rust-lang/rust-project-goals/blob/master/src/2026/incremental-system-rethought.md)
- [goal tracking issue 641](https://github.com/rust-lang/goals/issues/641)
- [target-stages RFC 3881](https://github.com/rust-lang/rfcs/pull/3881)
- [Cargo check/build metadata reuse issue 3501](https://github.com/rust-lang/cargo/issues/3501)
- [Cargo fine-grained locking issue 4282](https://github.com/rust-lang/cargo/issues/4282)
- [Cargo check/build concurrency issue 5169](https://github.com/rust-lang/cargo/issues/5169)

The project goal is Accepted. Its July 2026 update describes active research and
an intended Build Over Check experiment. The RFC remains open. Cargo's
fine-grained locking work permits more concurrent activity but does not make
different command artifacts or incremental bases equivalent.

### Cargo source

Installed Cargo source revision:
[`c79e8f89441b3e73d6d65d125c0c745792808c74`](https://github.com/rust-lang/cargo/commit/c79e8f89441b3e73d6d65d125c0c745792808c74).

- [check emits `.rmeta`](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/build_context/target_info.rs)
- [compile mode is part of artifact metadata identity](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/build_runner/compilation_files.rs)
- [check and doc dependencies select check mode](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/unit_dependencies.rs)
- [doctest outputs are temporary and deleted](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/build_runner/compilation_files.rs)

Cargo hashes compile mode into artifact metadata identity. Check emits only an
`.rmeta`. Check and documentation select metadata-only mode for ordinary target
dependencies, while build and test select build mode. Cargo records no
persistent doctest output because rustdoc builds doctest crates in temporary
directories and deletes them.

## Cross-command reuse model

```text
developer command
  -> selected coverage and targets
  -> Cargo graph units and exact artifact identities
  -> compiler activity and target stage
  -> stage-sensitive flags and dependencies
  -> tool-specific work
  -> emitted persistent or ephemeral outputs
  -> optional execution
```

Reuse can occur at several levels:

1. **Exact artifact reuse:** the same Cargo unit and output is fresh.
2. **Compatible dependency reuse:** a different root activity consumes an
   already built dependency artifact with matching identity.
3. **Common-stage reuse:** different activities share earlier rustc work but
   require different later stages. This is the primary upstream gap.
4. **Tool-specific reuse:** rustc, Clippy, or rustdoc shares a base while still
   running required tool-specific analysis.
5. **Ephemeral-output reuse:** generated temporary work would need an explicit
   persistence and identity contract before it can be reused.

Whole-artifact freshness proves only the first two.

## Activity matrix

| From | To | Current exact reuse | Required distinct work | Future common-stage opportunity |
|---|---|---|---|---|
| Check | Build | None in fixture | Codegen and linking | Analysis and earlier stages |
| Build | Check | None in fixture | Check activity identity | Build's completed analysis should satisfy compatible check |
| Check | Clippy | None in fixture | Clippy lint analysis and diagnostics | Parsing, expansion, lowering, and compatible semantic work |
| Build | Test | Ordinary libraries | Test cfg, harnesses, integration targets, examples, dev dependencies | Shared library frontend and backend work already exists; more stage reuse may remain |
| Check all targets | Test | None in fixture | Linkable libraries and test outputs | Compatible frontend work before test-specific divergence |
| Build | Documentation | None in fixture | Metadata-mode dependencies and rustdoc rendering | Compatible compiler frontend |
| Documentation | Doctest | None in fixture | Linkable libraries and rustdoc test compilation | Documentation extraction and dependency analysis where identities permit |
| Doctest | Doctest | Library dependencies only | Temporary rustdoc test crates every run | Persisted doctest-unit design, if net beneficial and correct |

Direction matters. A higher-stage result may contain enough common work for a
lower-stage activity, while a lower-stage result requires extension. Neither
direction means that activity-specific diagnostics or outputs can be skipped.

## Findings

### FERRIUM-257: aligned check and build currently share neither artifacts nor incremental bases

**Sources:** EXP-01, Cargo issue 3501, and the Incremental Systems Rethought
goal.

**Observed behavior:** Check then build and build then check each ran three
rustc compilations with zero fresh artifacts. Each package retained distinct
check and build incremental namespaces.

**Implication:** A shared target directory does not create cross-activity
incrementality. The gap exists below current whole-artifact identity.

**Confidence:** High for the fixture and pinned nightly.

### FERRIUM-258: check and build metadata are distinct by current design

**Sources:** EXP-01 and Cargo artifact-identity source.

**Observed behavior:** Check and build produced different `.rmeta` sizes,
hashes, filenames, and unit directories. Cargo hashes compile mode into
artifact metadata identity.

**Implication:** Do not alias files or declare build `.rmeta` and check
`.rmeta` interchangeable externally. Upstream must define which compiler state
is common and which outputs remain activity-specific.

**Confidence:** High.

### FERRIUM-259: target-stage reuse is more precise than command or artifact reuse

**Sources:** Incremental Systems Rethought goal, RFC 3881, and EXP-01.

**Observed behavior:** Check and build repeat a common semantic prefix but have
different stopping points and outputs. The accepted goal proposes target stages
and stage-sensitive dependencies rather than command equivalence.

**Implication:** FERRIUM reports reusable stages, required extensions, and
activity-specific outputs separately.

**Confidence:** High on the model; medium on the final upstream design.

### FERRIUM-260: check and Clippy need shared bases without shared verdicts

**Sources:** EXP-01, PERF-Q04, and the accepted project goal.

**Observed behavior:** Matching all-target coverage produced eight rustc check
artifacts and eight distinct Clippy artifacts. The failed pilot also showed
Clippy rejecting code that rustc check accepted.

**Implication:** Common frontend and semantic work is a strong reuse candidate,
but Clippy-specific lints, diagnostics, configuration, and failure remain
mandatory.

**Confidence:** High.

### FERRIUM-261: build and test already demonstrate safe partial cross-command reuse

**Sources:** EXP-01 and Cargo dependency-mode source.

**Observed behavior:** Build then test and test then build reused the ordinary
`dep` and `corelib` libraries. Test-specific harnesses and the ordinary
application executable remained distinct.

**Implication:** Cross-command reuse is not all-or-nothing. The query plan
should show compatible dependency hits alongside required root variants.

**Confidence:** High.

### FERRIUM-262: check coverage does not substitute for test artifacts

**Sources:** EXP-01 and PERF-Q04.

**Observed behavior:** All-target check then test produced zero fresh artifacts
and seven rustc invocations.

**Implication:** Selecting the same source targets does not erase differences
in linkability, `cfg(test)`, harnesses, dev dependencies, execution intent, or
compiler activity.

**Confidence:** High.

### FERRIUM-263: documentation uses a separate metadata and rustdoc activity

**Sources:** EXP-01 and Cargo dependency-mode source.

**Observed behavior:** Build and documentation reused no artifacts in either
direction. Documentation compiled two metadata-only dependencies and ran three
rustdoc units.

**Implication:** Documentation is not a free view over build artifacts today.
Future stage sharing must preserve rustdoc flags, cfg, diagnostics, rendered
outputs, and documentation-specific semantics.

**Confidence:** High.

### FERRIUM-264: doctest dependency reuse stops at temporary test crates

**Sources:** EXP-01 and Cargo doctest output source.

**Observed behavior:** Repeated doctest made both library dependencies fresh
but reran two rustdoc test compilations. Cargo records no persistent doctest
outputs because the generated crates are temporary and deleted.

**Implication:** Doctest persistence is a separate identity, storage, cleanup,
diagnostic, and economics problem. It is not solved by sharing ordinary
libraries.

**Confidence:** High.

### FERRIUM-265: repeated command freshness and repeated validation execution are different

**Sources:** EXP-01.

**Observed behavior:** Repeated test no-run launched no compiler because all
seven artifacts were fresh. Repeated doctest still compiled temporary tests.
Ordinary `cargo test` would additionally execute test binaries even when
compilation is fresh.

**Implication:** The plan distinguishes compilation reuse, generated-test
reuse, and validation execution.

**Confidence:** High.

### FERRIUM-266: activity identity needs stage-sensitive configuration

**Sources:** RFC 3881, Cargo mode hashing, and PERF-Q02.

**Observed behavior:** Current mode-level hashing safely separates outputs but
also makes flags that matter only to later stages part of an early reuse
decision. The RFC proposes stage dependencies so options such as stripping or
linking do not invalidate a check stage unnecessarily, while cfg-sensitive
options still do.

**Implication:** A useful explanation names the earliest stage affected by a
flag instead of saying only that command identities differ.

**Confidence:** High on the need; medium on the proposed mechanism.

### FERRIUM-267: concurrency and cross-activity reuse are independent improvements

**Sources:** Cargo issues 4282 and 5169, PERF-Q07, and the accepted project
goal.

**Observed behavior:** Fine-grained locking can allow more commands to proceed
concurrently. Current check and build still compile separate artifacts and
incremental state.

**Implication:** Parallel progress can reduce waiting while increasing total
machine work. Shared stages can reduce work while preserving or changing
concurrency. The query plan reports both.

**Confidence:** High.

### FERRIUM-268: the compiler query plan needs activity and target-stage nodes

**Sources:** findings FERRIUM-257 through FERRIUM-267 and PERF-Q19.

**Observed behavior:** One command pair can contain exact hits, compatible
dependency hits, repeated common stages, required tool-specific stages,
ephemeral outputs, and optional execution.

**Implication:** A command-level "cache hit" is too coarse. Planned and observed
graphs should expose activity, stage boundary, stage-sensitive inputs, emitted
output, persistence, and execution.

**Confidence:** High.

### FERRIUM-269: FERRIUM should align with the accepted upstream incremental redesign

**Sources:** Incremental Systems Rethought goal, RFC 3881, findings
FERRIUM-257 through FERRIUM-268, and the role checkpoint.

**Observed behavior:** Rust has an accepted, funded, active goal covering the
compiler mechanism. FERRIUM's differentiated opportunity is cross-tool
explanation, evidence, history, and visualization.

**Implication:** Contribute fixtures, measurements, and query-plan vocabulary.
Do not build a parallel incremental compiler or patch artifact directories
externally.

**Confidence:** High.

## Recommendations

### Adopt now

- Add compiler activity, target stage, stage dependency, exact artifact,
  compatible dependency, tool-specific work, coverage-specific work, and
  ephemeral output to the measurement contract.
- Show cross-command reuse directionally in the compiler query plan.
- Preserve build/test library reuse as a positive control.
- Preserve check/build, check/Clippy, check/test, build/doc, and
  doc/doctest separation as current negative controls.
- Treat repeated doctest compilation as a distinct persistent-output research
  question, not as generic Cargo cache failure.

### Prototype behind an upstream compatibility boundary

- Target-stage-compatible rustc-perf fixtures for check/build and check/Clippy.
- Read-only comparison of command graphs, artifact identities, and repeated
  compiler stages.
- Optional nightly target-stage diagnostics only after an upstream unstable
  interface exists.
- Doctest persistence experiments only with rustdoc and Cargo owner alignment.

### Reject or defer

- Replacing one command with another.
- Claiming check covers Clippy or tests.
- Aliasing check and build `.rmeta` files.
- Merging incremental directories from distinct activities.
- Disabling doctests because their outputs are temporary.
- Treating fine-grained locking as reduced total work.
- Building a FERRIUM compiler fork or independent target-stage protocol.

## Role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted: command-specific cfg, lint, test, documentation, linking, and diagnostic work remains mandatory; no artifact aliasing is proposed. |
| Compiler Performance Engineer | Accepted: exact reuse, common-stage opportunity, required distinct work, ephemeral output, and exploratory timing are separated. |
| Interop Boundary Auditor | Accepted with deferral: native libraries, ABI, build scripts, dynamic linking, and target-specific documentation remain explicit gaps. |
| AI Assurance Skeptic | Accepted: a shared compiler base is not presented as a shared verdict; the failed Clippy pilot and unsupported target-stage state remain visible. |
| Ecosystem Strategist | Accepted: the compiler mechanism is deferred to the accepted upstream goal; FERRIUM retains explanation and fixture work. |
| Rust Maintainer | Accepted: ordinary Cargo commands remain authoritative and no workflow substitution or compiler fork is introduced. |
| Native Platform Adopter | Accepted for research: the Windows result is useful, while Unix, large-workspace, storage, and operational evidence remain required before adoption. |
| Scope Keeper | Accepted: Q21 closes with one matrix and upstream contribution boundary, not a command cache implementation. |
| Validation Checker | Accepted: aligned command pairs, positive reuse, negative reuse, repeat controls, failed lint behavior, environment, and limitations are recorded. |

## Non-goals

- Claim target stages are implemented in the pinned nightly.
- Claim different Cargo commands should produce identical artifacts.
- Skip activity-specific diagnostics, lints, tests, docs, codegen, or linking.
- Persist doctest outputs without a supported identity and cleanup contract.
- Reduce validation coverage.
- Open an implementation gate.
