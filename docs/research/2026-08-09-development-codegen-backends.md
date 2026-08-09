# Development Codegen Backends

Date: 2026-08-09
Question: PERF-Q27
Status: Complete
Decision: treat Cranelift as a nightly, target- and workflow-specific
development backend candidate; add a read-only backend eligibility and outcome
ledger to the compiler query plan and labeled Build Forest; require separate
target roots, exact component identity, clean and incremental build evidence,
runtime and failure-path controls, and ordinary LLVM validation; prototype
advisory comparison only; defer automatic profile changes, repository
configuration, CI defaults, mixed-backend artifact reuse, release use, and a
FERRIUM backend.

## Executive conclusion

Cranelift is a real development-build accelerator, but "use Cranelift" is not a
safe repository-wide default.

On the pinned x86_64 Windows nightly, Cranelift shortened a five-repetition
METIS-CORE clean debug build by 21.8% and clean test compilation by 8.2%. It
also reduced CPU substantially. The same backend shortened the smaller
synthetic clean build by only 2.9%, did not improve warm no-op builds, and did
not establish an incremental body-edit win.

The reason is architectural. Cranelift replaces backend code generation.
Parsing, expansion, type checking, borrow checking, MIR, monomorphization,
metadata, procedural macros, build scripts, Cargo scheduling, freshness, and
linking still exist. A repository benefits in proportion to the time and CPU
that actually reach replaceable codegen.

Cargo check was effectively unchanged because it does not perform ordinary
machine-code generation. Warm no-op builds were unchanged because Cargo did
not invoke codegen. A broad test graph benefited less than the public library
build because frontend, macro, dependency, test-target, and linker work
diluted the backend difference.

Runtime quality also changed. Both synthetic executables produced the same
checksum, but Cranelift output ran 18.2% slower. That result is acceptable only
for consumers whose development runtime remains inside a declared feedback
budget. It cannot justify release use.

The decisive Windows limitation was failure behavior. Passing unit tests
worked. An intentionally failing LLVM test produced an ordinary named failure
and exited. The Cranelift version printed `running 1 test` and did not complete
within 15 seconds. A direct `catch_unwind` probe also failed to complete after
panic. The public METIS suite showed the same distinction when an unavailable
external executable caused assertions: LLVM returned ordinary failed-test
output, while Cranelift terminated abnormally.

That behavior agrees with the current upstream statement that panic unwinding
is experimental and unsupported on Windows and macOS. A backend that speeds
successful compilation but can conceal or distort a failing test is not yet a
trustworthy default for `cargo test` on this platform.

Cranelift also rejected ThinLTO, requires nightly Cargo's unstable
`codegen-backend` feature, has narrower target coverage than LLVM, and retains
partial SIMD and `std::arch` support. Backend identity changes target features,
panic strategy, machine code, artifacts, runtime behavior, debugging, and
failure semantics. It belongs in the build identity, not in an invisible local
speed switch.

The upstream direction is strong. Cranelift is distributed through rustup for
nightly Linux, macOS, and x86_64 Windows, uses rustc's shared codegen
abstraction, and has an explicit Rust project goal to become suitable for
local development. That same goal described roughly 20% lower codegen time and
about 5% clean-build improvement on several large projects while naming
unwinding, ABI, SIMD, Windows, and debuginfo work as remaining gaps.

FERRIUM should support that direction through evidence, not competition. The
compiler query plan should report whether a workflow is backend-eligible, how
much of its critical path reaches codegen, what the alternative changed, and
which validation still requires LLVM. The labeled Build Forest may retain
separate immutable LLVM and Cranelift roots, but it must never merge or restore
their artifacts as if they shared one identity.

The immediate product boundary is a read-only comparison and eligibility
ledger. It does not edit `Cargo.toml`, create `.cargo/config.toml`, set
environment variables, switch CI, suppress LLVM builds, or recommend Cranelift
when failure behavior, runtime, debugging, target, intrinsic, ABI, sanitizer,
or release requirements are unverified.

## Decision supported

This research determines:

- which parts of edit-to-feedback latency a backend can change;
- why check, warm, clean, incremental, test, and run workflows need separate
  evidence;
- how backend selection changes artifact and validation identity;
- which target, panic, SIMD, LTO, runtime, debugger, and failure-path controls
  gate Cranelift adoption;
- why LLVM remains the release and compatibility authority;
- how FERRIUM can explain backend eligibility without owning a backend.

It does not authorize automatic backend selection, repository configuration,
CI changes, release use, mixed-backend artifact reuse, skipped LLVM validation,
or implementation.

## Evidence reviewed

### Local evidence

- [EXP-01 development backend matrix](perf-q27-development-backends/results/EXP-01-development-backend-matrix.md)
- [LLVM optimization cost](2026-08-09-llvm-optimization-cost.md)
- [Codegen-unit partitioning](2026-08-09-codegen-unit-partitioning.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Rust and backend sources

- [Pinned rustc codegen-backend trait](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_ssa/src/traits/backend.rs)
- [Pinned rustc backend loader](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/util.rs)
- [rustc_codegen_cranelift README](https://github.com/rust-lang/rustc_codegen_cranelift)
- [Cranelift usage](https://github.com/rust-lang/rustc_codegen_cranelift/blob/master/docs/usage.md)
- [Production-ready Cranelift project goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2025h2/production-ready-cranelift.md)
- [Rust 2025H2 project goals](https://blog.rust-lang.org/2025/10/28/project-goals-2025h2/)
- [rustc_codegen_gcc](https://github.com/rust-lang/rustc_codegen_gcc)

## Current backend model

### The backend boundary is explicit but broad

rustc's `CodegenBackend` trait owns target configuration, codegen, joined
compiled modules, work products, linking hooks, supported crate types, and
backend capabilities such as ThinLTO. The frontend and much of
`rustc_codegen_ssa` are shared.

This makes an alternative backend less than a compiler replacement but more
than a local optimizer toggle. It can affect:

- recognized and enabled target features;
- intrinsic lowering and fallback behavior;
- machine code and object format details;
- work-product identity;
- crate-type support;
- LTO support;
- debuginfo and unwind metadata;
- linking inputs and runtime behavior.

### Cargo selection is profile-level and unstable

The distributed Cranelift backend can be selected through an unstable Cargo
profile feature or environment profile override. Stable Cargo rejects the
required `-Zcodegen-backend` flag.

A backend recommendation therefore also recommends:

- a nightly toolchain;
- an exact backend component;
- a profile and command scope;
- isolated target and incremental roots;
- a rollback to ordinary LLVM Cargo.

### Clean codegen share predicts opportunity

A backend can only improve work that reaches its boundary. A useful model is:

```text
Cargo and build-time execution
  -> rustc frontend and MIR
  -> mono-item collection and CGU partitioning
  -> selected codegen backend
  -> object and debug emission
  -> linker
  -> executable or test behavior
```

The replaceable codegen share must be measured on the consumer workflow.
Aggregate clean-build size, crate count, or source lines are not sufficient.

### Backend success is not behavioral equivalence

Compilation and happy-path output are necessary controls. They do not establish
equivalent:

- panic and test-failure behavior;
- runtime performance;
- debugger behavior;
- stack unwinding and FFI unwinding;
- SIMD, inline assembly, sanitizer, coverage, or profiler behavior;
- ABI and dynamic-library compatibility;
- release optimization.

## Findings

### FERRIUM-354: a development backend replaces codegen, not the build

**Sources:** pinned `CodegenBackend`, backend loader, EXP-01.

**Observed behavior:** Cargo check and warm no-op builds were effectively
unchanged, while codegen-heavy clean builds changed more.

**Implication:** Attribute the critical path before recommending a backend.
Frontend-, macro-, build-script-, linker-, execution-, or freshness-bound
workflows cannot inherit a clean codegen result.

**Confidence:** High.

### FERRIUM-355: Cranelift's clean-build gain is repository-specific

**Sources:** EXP-01 synthetic and METIS clean builds.

**Observed behavior:** Cranelift shortened the synthetic clean build 2.9% but
the public METIS build 21.8%.

**Implication:** No universal percentage is defensible. Measure the exact
repository, profile, target, and command.

**Confidence:** High for the fixtures; medium for prevalence.

### FERRIUM-356: backend CPU savings may exceed wall savings

**Sources:** EXP-01 clean build and test compilation.

**Observed behavior:** METIS test compilation used 35.4% less CPU but shortened
wall time 8.2%. Shared frontend, dependency, scheduling, and linker work
remained.

**Implication:** Preserve wall, CPU, memory, critical path, and concurrent
workload effects. CPU reduction can matter in CI even when foreground latency
changes less.

**Confidence:** High for the fixture.

### FERRIUM-357: Cargo check is not a backend-selection workload

**Sources:** EXP-01 synthetic check matrix.

**Observed behavior:** LLVM and Cranelift check medians overlapped.

**Implication:** Do not promise faster check or rust-analyzer feedback merely
because a codegen backend improved build.

**Confidence:** High.

### FERRIUM-358: fresh builds do not benefit from a faster backend

**Sources:** EXP-01 synthetic and METIS warm builds.

**Observed behavior:** warm no-op wall medians differed by less than 1%.

**Implication:** Backend guidance needs the workflow's freshness and edit
distribution. Improve avoidable rebuilds before optimizing codegen that does
not run.

**Confidence:** High.

### FERRIUM-359: the measured incremental edit did not establish a win

**Sources:** EXP-01 equivalent body edit.

**Observed behavior:** Cranelift's median was 2.4% slower with high variance.

**Implication:** Clean-build wins cannot stand in for incremental iteration.
Use multiple edit classes and report inconclusive rows.

**Confidence:** Medium for the negative conclusion; low for other edits.

### FERRIUM-360: development runtime can regress

**Sources:** EXP-01 synthetic runtime control.

**Observed behavior:** checksums matched, but Cranelift output ran 18.2% slower.

**Implication:** `cargo run`, test execution, examples, simulations, and
developer servers need a representative runtime budget. Faster compilation is
not automatically faster feedback.

**Confidence:** High for the fixture; low for portable ratios.

### FERRIUM-361: passing tests do not validate failure semantics

**Sources:** EXP-01 smoke, METIS, isolated failing-test, and unwind controls.

**Observed behavior:** positive tests passed, but a one-test panic hung under
Cranelift on Windows while LLVM produced ordinary failure output.

**Implication:** Backend qualification must include intentional failure,
panic, timeout, cancellation, and diagnostic controls. A distorted failure is
a validation defect.

**Confidence:** High for the pinned Windows component.

### FERRIUM-362: panic strategy belongs in backend eligibility

**Sources:** Cranelift README and EXP-01.

**Observed behavior:** upstream marks panic unwinding experimental and
unsupported on Windows and macOS. The local unwind probes failed operationally.

**Implication:** Record requested and effective panic strategy, target, test
harness behavior, FFI unwind requirements, and failure diagnostics.

**Confidence:** High.

### FERRIUM-363: Cranelift is not an LTO or release substitute

**Sources:** EXP-01 ThinLTO negative control and upstream purpose.

**Observed behavior:** the backend rejected ThinLTO, and development output ran
slower in the runtime control.

**Implication:** LLVM remains authoritative for release, LTO, and performance
qualification unless a separate future decision establishes otherwise.

**Confidence:** High.

### FERRIUM-364: target and intrinsic coverage remain narrower than LLVM

**Sources:** Cranelift support matrix and unsupported list.

**Observed behavior:** Windows AArch64 is unsupported, several targets are
untested or not rustup-distributed, and SIMD/`std::arch` coverage is partial.

**Implication:** Eligibility is target-, feature-, intrinsic-, ABI-, and
distribution-specific.

**Confidence:** High.

### FERRIUM-365: Cargo integration is usable but unstable

**Sources:** Cranelift README and stable-Cargo negative control.

**Observed behavior:** nightly rustup distribution and profile selection worked;
stable Cargo rejected the feature.

**Implication:** Keep the adapter exact-nightly, removable, and opt-in. Do not
write unstable profile configuration automatically.

**Confidence:** High.

### FERRIUM-366: backend identity partitions artifacts and incremental state

**Sources:** pinned backend interface, PERF-Q02, PERF-Q18, and EXP-01.

**Observed behavior:** backends produced different machine code, artifact
sizes, runtime, capabilities, and failure behavior.

**Implication:** Include backend component hash and capability policy in build
identity. Keep target and incremental roots isolated.

**Confidence:** High.

### FERRIUM-367: intermediate backend bytes are not code quality

**Sources:** EXP-01 Rlib and executable controls.

**Observed behavior:** Cranelift produced much smaller intermediate METIS Rlibs
and a smaller synthetic executable while the synthetic runtime was slower.

**Implication:** Preserve archive, object, debug, executable, runtime, and
deployment controls separately.

**Confidence:** High for the fixtures.

### FERRIUM-368: Cranelift has an active upstream adoption path

**Sources:** rustup distribution and production-ready Cranelift project goal.

**Observed behavior:** the Rust project explicitly targets local development
use and names the remaining ABI, unwind, SIMD, Windows, and debuginfo work.

**Implication:** FERRIUM should contribute fixtures and evidence rather than
forking or wrapping the backend as its own implementation.

**Confidence:** High.

### FERRIUM-369: GCC codegen addresses a different primary gap

**Sources:** `rustc_codegen_gcc` README.

**Observed behavior:** the GCC backend is work in progress, requires patched
libgccjit, and prioritizes non-LLVM target support with runtime optimization as
a secondary goal.

**Implication:** Do not group every alternative backend into one development
speed recommendation.

**Confidence:** High.

### FERRIUM-370: the immediate opportunity is a backend eligibility ledger

**Sources:** findings 354 through 369 and role review.

**Observed behavior:** selection requires joined workflow, target, component,
capability, performance, runtime, artifact, and failure evidence that Cargo's
backend switch alone does not present.

**Implication:** Add a read-only eligibility and comparison overlay to the
compiler query plan and labeled Build Forest. Keep selection advisory and
human-approved.

**Confidence:** High for the evidence gap; medium for product adoption.

## Recommendations

### Adopt now

- Record backend name, rustc revision, component hash, target, panic strategy,
  profile, command, target root, incremental root, target features, LTO,
  debuginfo, and relevant instrumentation.
- Measure clean, warm, incremental, check, build, test compilation, test
  execution, and run workflows separately.
- Pair compile evidence with behavior, intentional failure, runtime, memory,
  artifact, debugger, ABI, and platform controls required by the consumer.
- Keep LLVM and Cranelift roots separate and retain ordinary LLVM validation.
- Contribute minimized backend incompatibilities upstream.

### Prototype behind a compatibility boundary

- An exact-nightly Cranelift availability and capability probe.
- A read-only backend eligibility and outcome ledger.
- Isolated LLVM-versus-Cranelift Build Forest roots for representative
  commands.
- An advisory that says why a workflow is eligible, ineligible, or
  inconclusive without changing repository configuration.

### Reject or defer

- Automatic `Cargo.toml`, `.cargo/config.toml`, environment, CI, or editor
  changes.
- Cranelift as a Windows test default while panic failure behavior differs.
- Release, benchmark, LTO, sanitizer, coverage, profiler, debugger, FFI unwind,
  or unsupported-target guidance without dedicated evidence.
- Sharing artifacts or incremental state across backends.
- A FERRIUM codegen backend, compiler fork, or generic backend abstraction
  layer.
- Treating clean-build speed, CPU, archive size, or happy-path tests as the
  complete adoption decision.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because backend success is not treated as proof; panic, failure, ABI, target-feature, unsafe, runtime, and ordinary LLVM controls remain explicit. |
| Compiler Performance Engineer | Accepted because clean, check, warm, incremental, test compilation, execution, CPU, memory, variance, and runtime remain separate; inconclusive rows are not promoted. |
| Interop Boundary Auditor | Accepted provisionally because ABI, unwind, native libraries, target features, inline assembly, dynamic linking, allocation, and cross-language failure behavior remain open gates. |
| AI Assurance Skeptic | Accepted because exact component identity, commands, failed tests, timeout behavior, unsupported LTO, variance, and negative conclusions remain visible; no backend is selected automatically. |
| Ecosystem Strategist | Accepted because Cranelift and rustc remain upstream owners, GCC is distinguished by purpose, and FERRIUM contributes evidence rather than duplicating a backend. |
| Rust Maintainer | Accepted because ordinary Cargo and LLVM remain available, target roots are isolated, the feature is removable, and recommendations explain workflow-specific benefit and incompatibility. |
| Native Platform Adopter | Accepted provisionally because Windows failure behavior blocks default adoption and cross-platform, debugger, ABI, support, rollout, audit, and rollback evidence remain required. |
| Scope Keeper | Accepted because PERF-Q27 owns development backend selection while PERF-Q28 and PERF-Q29 retain debug emission and linking; release backend work remains deferred. |
| Validation Checker | Accepted because the experiment preserves five-repetition primary matrices, checksums, public tests, intentional failure, timeouts, unsupported modes, exact revisions, and limitations. |

No role authorizes implementation or an automatic backend default.

## Product implication

The compiler query plan should add:

```text
workflow and edit class
  -> Cargo freshness and unit graph
  -> shared rustc frontend and MIR
  -> mono-item and CGU work
  -> backend eligibility
       -> component and target support
       -> panic, intrinsic, ABI, debug, sanitizer, and LTO capabilities
  -> isolated backend root
  -> codegen, object, debug, and link outcome
  -> behavior, failure, runtime, and validation controls
```

The Build Forest may retain sibling LLVM and Cranelift roots with a common
source parent. Their labels and evidence can be compared, but their artifacts,
incremental caches, and validation claims are not interchangeable.

## Prototype gate

No backend advisor or automatic selection implementation may begin until:

1. at least three Tier 1 repositories reproduce clean and incremental matrices;
2. Linux, macOS, and Windows evidence exists;
3. x86_64 and AArch64 are covered where upstream support permits;
4. check, build, test compile, passing tests, failing tests, and run workflows
   are represented;
5. panic, timeout, cancellation, runtime, debugger, sanitizer, coverage,
   profiler, SIMD, inline assembly, ABI, FFI, dynamic-library, and native-link
   requirements are classified;
6. backend artifacts and incremental roots remain isolated;
7. exact-nightly capability probes fail closed on missing or changed behavior;
8. LLVM release and repository-required validation remain mandatory;
9. rollback removes only optional local configuration and target roots;
10. upstream issues receive minimized fixtures for every blocking mismatch.

The implementation gate remains closed.
