# Crate Slicing and Partial Dependency Compilation

Date: 2026-08-09
Question: PERF-Q32
Status: Complete
Decision: adopt a read-only dependency-surface and partial-compilation
eligibility ledger; treat Cargo's nightly `hint-mostly-unused` support as a
real but selective codegen-slicing mechanism; maintain sparse, dense, generic,
private, multi-consumer, and whole-crate correctness fixtures; contribute
measured evaluation to Cargo issue `#15644`; defer full stub-rlib crate
slicing, source-level slicing, compiler or Cargo forks, automatic profile
rewrites, and implementation.

## Executive conclusion

Rust does not currently have full crate slicing, but it does have a narrower
partial-compilation mechanism. Cargo's unstable `hint-mostly-unused` profile
setting and rustc's `-Zhint-mostly-unused` move eligible dependency codegen into
the consumer. The dependency still parses, expands, resolves, type-checks,
borrow-checks, and builds MIR for its bodies. Only machine-code ownership moves.

That distinction is decisive. In a generated development fixture with 1,200
public non-generic functions, sparse use reduced median clean wall time 35.5%,
CPU time 49.3%, peak RSS from 600.3 MiB to 221.8 MiB, and the dependency rlib
43.2%. Mono-item evidence moved one used function into the consumer instead of
compiling all 1,200 in the dependency. When the consumer used all 1,200
functions, the same hint increased wall time 4.8% and CPU time 13.4% because
all codegen moved downstream.

A public METIS-CORE control confirmed that this is not only a synthetic
effect. A small binary using one METIS entry point reduced its five-run median
clean development build from 14.59 seconds to 10.00 seconds, a 31.4% decrease.
The METIS rlib fell from 11.31 MiB to 0.63 MiB while the executable and output
remained equal. Mono-item output changed from 2,365 items before the consumer
compile marker and 20 after it to zero before and 2,200 after it.

The opportunity is sharply bounded:

- public, non-generic, wide APIs with sparse use can avoid substantial
  dependency-owned codegen;
- generic definitions already instantiate on demand in consumers;
- private unreachable functions already avoid ordinary codegen;
- release optimization may already make eligible bodies cross-crate
  inlinable, making the explicit hint redundant;
- dense use moves rather than removes work and can regress;
- several consumers can duplicate deferred codegen;
- explicit inline policy affects eligibility; and
- type errors and other whole-crate correctness work still apply to unused
  bodies.

The existing hint is therefore partial codegen slicing, not full crate slicing.
The unaccepted Rust 2026 crate-slicing proposal is more ambitious: dependency
crates would emit stub rlibs after HIR, while type checking, borrow checking,
MIR, and codegen for reachable items would be completed later and scheduled
with the root crate. That model could attack frontend critical paths and use
otherwise idle cores, but it crosses coherence, macro expansion, generated
code, dynamic dispatch, diagnostics, incremental identity, and compiler
ownership boundaries that the current hint deliberately avoids.

FERRIUM should classify and measure candidates rather than build a slicer. The
read-only compiler query plan should distinguish declared surface, frontend
correctness work, deferred MIR availability, dependency-owned codegen,
consumer-owned codegen, duplicated consumer work, and final retained code.
Candidate advice remains experimental, explicit, reversible, and tied to
measured sparse-use evidence.

## Decision supported

This research determines:

- what current Rust partial-compilation support actually skips;
- which dependency surfaces are already lazy;
- where public non-generic APIs create avoidable codegen;
- when moving codegen into consumers helps or regresses;
- why dense and multi-consumer graphs require negative controls;
- which whole-crate semantic checks remain mandatory;
- how full crate slicing differs from `hint-mostly-unused`;
- where Cargo, rustc, library authors, and FERRIUM own the boundary; and
- whether FERRIUM should prototype a slicer.

It does not authorize source transformation, stub rlibs, compiler-private
metadata consumption, a rustc or Cargo fork, automatic manifest changes,
production nightly dependence, or implementation.

## Evidence reviewed

### Local evidence

- [EXP-01 partial-compilation matrix](perf-q32-crate-slicing/results/EXP-01-partial-compilation-matrix.md)
- [rustc startup and metadata loading](2026-08-08-rustc-startup-metadata.md)
- [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [monomorphization and generic-instance reuse](2026-08-09-monomorphization-generic-instance-reuse.md)
- [function-level machine-code caching](2026-08-09-function-level-machine-code-caching.md)
- [build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Upstream sources

- [Rust 2026 crate-slicing goal](https://github.com/rust-lang/goals/blob/main/src/2026/crate-slicing.md)
- [Rust 2026 Fast Builds roadmap](https://github.com/rust-lang/goals/blob/main/src/2026/roadmap-fast-builds.md)
- [Call for testing `hint-mostly-unused`](https://blog.rust-lang.org/inside-rust/2025/07/15/call-for-testing-hint-mostly-unused/)
- [Cargo unstable profile hint documentation](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#profile-hint-mostly-unused)
- [Cargo tracking issue `#15644`](https://github.com/rust-lang/cargo/issues/15644)
- [rustc monomorphization collector](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_monomorphize/collector/index.html)

## Current partial-compilation model

### Metadata demand is already narrower than crate size

PERF-Q08 found that merely passing a large `.rmeta` dependency to rustc cost
little until names, reexports, or items were actually referenced. Metadata
decoding is already demand-shaped. Crate slicing should not be justified by
treating all dependency metadata as eagerly decoded.

### Generic and private code are already comparatively lazy

PERF-Q24 established that generic definitions are instantiated for concrete
types in downstream crates. The PERF-Q32 generic control likewise produced one
used external instance without the explicit hint. Unused private functions in
the private-wide control were not ordinary codegen roots.

The primary current waste is therefore a dependency's eligible public
non-generic surface: rustc normally emits machine code for those reachable
exports even when a particular consumer uses very few of them.

### `hint-mostly-unused` changes codegen ownership

The hint tells rustc that a dependency's exported functions are expected to be
used sparsely. Eligible bodies are made available downstream rather than
emitted into the dependency's ordinary codegen set.

The generated sparse control showed:

- baseline dependency: 1,200 generated functions;
- hinted dependency: zero generated functions;
- baseline consumer: zero deferred external generated functions; and
- hinted consumer: one used external function.

The dense control moved all 1,200 functions into the consumer. The mechanism
does not make dense demand disappear.

### Frontend correctness remains whole-crate

Self-profile evidence for the generated dependency retained 1,200 invocations
of `typeck_root`, `mir_borrowck`, and `mir_built` with and without the hint.
The dependency's broad LLVM emission disappeared, but frontend semantic work
did not.

An invalid assignment inserted into unused function `f_1199` still failed a
hinted build whose consumer called only `f_0`. The current mechanism does not
defer type errors in unused bodies.

### Full crate slicing is a different compiler architecture

The 2026 crate-slicing proposal would emit dependency stubs after HIR and defer
later work to root compilation. Reachability would select which bodies proceed
through type checking, borrow checking, MIR, and codegen, and deferred work
could fill otherwise idle graph capacity.

That proposal is marked **Not accepted**. It has no implementation commitment,
and its unresolved boundaries are substantially broader than the existing
hint. Source-level slicers cannot safely substitute for compiler-owned
reachability because Rust correctness depends on impl sets, coherence,
blanket implementations, macro and generated-code expansion, generic and
dynamic dispatch, language items, exported symbols, diagnostics, and
incremental identities that are not equivalent to textual call-graph reach.

## Findings

### FERRIUM-433: dependency metadata consumption is already demand-shaped

**Sources:** PERF-Q08; rustc metadata architecture.

**Observed behavior:** a large passed-but-unused metadata file added little
cost, while referenced items and glob reexports increased decoding demand.

**Implication:** a partial-compilation model must separate namespace and
signature metadata from body analysis and codegen. "Large rlib" and "large
metadata demand" are not interchangeable claims.

**Confidence:** High.

### FERRIUM-434: public non-generic APIs are the primary current codegen gap

**Sources:** EXP-01 sparse, generic, and private controls; PERF-Q24.

**Observed behavior:** the ordinary development build emitted 1,200 public
non-generic functions in the dependency. Generic code instantiated only for
used types, and unused private functions did not become ordinary codegen roots.

**Implication:** candidate classification should focus on wide public
non-generic surfaces rather than crate size alone.

**Confidence:** High for the controlled shapes.

### FERRIUM-435: `hint-mostly-unused` performs selective codegen slicing

**Sources:** EXP-01 mono-item and self-profile evidence; Cargo and rustc
documentation.

**Observed behavior:** the hint removed the dependency's 1,200-function
codegen set and moved selected functions into the consumer, while type
checking, borrow checking, and MIR construction remained whole-crate.

**Implication:** the correct label is partial codegen slicing. It should not be
described as lazy parsing, lazy type checking, or full crate slicing.

**Confidence:** High.

### FERRIUM-436: sparse demand can materially reduce clean development cost

**Sources:** EXP-01 generated sparse control.

**Observed behavior:** one used function reduced clean development wall time
35.5%, CPU 49.3%, peak RSS 63.1%, and dependency rlib bytes 43.2%. Output
digests matched.

**Implication:** the mechanism has enough headroom to justify candidate
measurement and upstream evaluation.

**Confidence:** High for the generated fixture; medium for ecosystem-wide
effect.

### FERRIUM-437: the public METIS control confirms real-repository value

**Sources:** EXP-01 METIS-CORE control.

**Observed behavior:** five clean development builds reduced median wall time
31.4%, from 14.59 seconds to 10.00 seconds. The METIS rlib shrank 94.4%, the
consumer executable stayed at 694 KiB, and output remained
`[0, 1, 0, 0]`.

**Implication:** sparse public dependency surfaces exist in an ordinary Rust
library and are not limited to generated function farms.

**Confidence:** High for the pinned public control.

### FERRIUM-438: dense demand can regress by moving all work downstream

**Sources:** EXP-01 dense development control.

**Observed behavior:** when the consumer called all 1,200 functions, the hint
increased wall time 4.8% and CPU 13.4%. Mono-item evidence moved all 1,200
functions from the dependency into the consumer.

**Implication:** the hint must never be recommended solely from API width or
rlib shrinkage. Observed use density and end-to-end cost are mandatory.

**Confidence:** High for the controlled shape.

### FERRIUM-439: multiple consumers can duplicate deferred codegen

**Sources:** EXP-01 four-consumer control; rustc monomorphization ownership.

**Observed behavior:** four binaries each using the same 600 eligible
functions produced 2,400 downstream external mono items. In the release
fixture, those functions were already deferred through cross-crate
inlinability, so the explicit hint was redundant.

**Implication:** a workspace-level advisor must estimate repeated consumer
ownership, not only dependency savings. Shared dependency codegen can be
cheaper than repeated downstream codegen.

**Confidence:** High for the fixture.

### FERRIUM-440: optimization and inline policy define eligibility

**Sources:** EXP-01 release and `#[inline(never)]` controls.

**Observed behavior:** release optimization already deferred the generated
functions, leaving rlib contents and mono-item placement unchanged by the
explicit hint. Adding `#[inline(never)]` prevented the development fixture from
exposing the intended deferral behavior.

**Implication:** effective profile, optimization, and inline attributes are
eligibility inputs. A package-name allowlist is not a sufficient model.

**Confidence:** High.

### FERRIUM-441: whole-crate errors remain visible under the current hint

**Sources:** EXP-01 unused-body negative case.

**Observed behavior:** a type error in unused `f_1199` failed a hinted build
whose consumer referenced only `f_0`.

**Implication:** current partial codegen preserves whole-crate frontend
correctness. Full slicing would need explicit diagnostic, coherence, and
deferred-error semantics rather than silently ignoring unused bodies.

**Confidence:** High.

### FERRIUM-442: full crate slicing targets a larger critical-path gap

**Sources:** Rust 2026 crate-slicing goal; Fast Builds roadmap; EXP-01
self-profile.

**Observed behavior:** current hinting leaves parse-through-MIR work in the
dependency. The proposal would defer type checking, borrow checking, MIR, and
codegen and schedule reachable work with the root.

**Implication:** full slicing could reduce frontend critical paths and increase
parallel graph utilization beyond current codegen-only gains. Existing hint
results do not prove that architecture correct or beneficial.

**Confidence:** High on the architectural difference; low on unimplemented
end-to-end benefit.

### FERRIUM-443: source-level slicing cannot own Rust semantic reachability

**Sources:** crate-slicing goal; PERF-Q10, Q13, Q17, Q20, Q22, Q23, and Q24.

**Observed behavior:** coherence and impl sets, macros, generated code,
generic instances, dynamic dispatch, inline bodies, layouts, and incremental
identities cross textual function boundaries.

**Implication:** FERRIUM should not build a source transformer or call-graph
slicer. Reachability and deferred correctness work must remain compiler-owned.

**Confidence:** High.

### FERRIUM-444: separate-compilation gap must exceed analysis and duplication cost

**Sources:** crate-slicing goal's cargo-slicer and PRECC-Rust evidence;
EXP-01 sparse, dense, generic, private, and multi-consumer controls.

**Observed behavior:** prior prototypes report gains on some large workloads
and regressions when the removable gap is too small. Local controls reproduce
the same sign change: sparse public use wins, dense and redundant cases do not.

**Implication:** candidate prediction must estimate avoidable dependency work,
consumer demand, repeated consumers, existing laziness, and hint overhead.
Selective measurement is mandatory.

**Confidence:** High on selectivity; medium on a future prediction model.

### FERRIUM-445: upstream evaluation is ready; a FERRIUM slicer is not

**Sources:** Cargo issue `#15644`; unaccepted crate-slicing goal; EXP-01.

**Observed behavior:** an upstream nightly mechanism exists with a public call
for testing, while the broader compiler goal is not accepted and lacks an
implementation owner.

**Implication:** FERRIUM should preserve fixtures, report results and negative
cases upstream, and add read-only eligibility evidence. It should not create a
compiler fork, wrapper, source transformer, or independent partial-compilation
format.

**Confidence:** High.

## Dependency-surface ledger

The read-only compiler query plan and Build Forest should be able to record:

| Field | Meaning |
|---|---|
| Declared surface | Public non-generic, generic, private, macro, trait, impl, static, and generated items |
| Consumer demand | Direct references, instantiated generics, dynamic or vtable reachability, and final retained symbols |
| Frontend work | Expansion, resolution, type checking, trait solving, borrow checking, and MIR that still ran |
| Eligibility | Toolchain, profile, optimization, inline policy, crate type, and current hint support |
| Dependency codegen | Mono items and bytes emitted in the dependency |
| Consumer codegen | Deferred external items emitted by each consumer |
| Duplication | Same deferred body emitted in multiple consumers or targets |
| Final retention | Objects and symbols retained after archive selection and linking |
| Outcome | Wall, CPU, memory, artifact bytes, runtime, and output equivalence |
| Limitation | Nightly status, unsupported shape, whole-crate constraint, or inconclusive measurement |

This is explanatory evidence, not an automatic manifest edit.

## Recommendations

### Adopt now

- Add dependency-surface, hint eligibility, codegen ownership, consumer demand,
  duplication, and outcome vocabulary to the read-only compiler query plan.
- Maintain generated sparse, dense, generic, private, multi-consumer, inline,
  release, and unused-error controls.
- Preserve the METIS-CORE sparse-use control as a Tier 1 example.
- Distinguish current partial codegen slicing from proposed full crate slicing.
- Report measured positive and negative cases to Cargo issue `#15644`.

### Prototype behind a compatibility boundary

- A read-only nightly comparison that measures baseline and hinted builds for
  explicitly selected dependencies in disposable target directories.
- Candidate scoring based on public non-generic surface, measured consumer
  demand, profile, inline policy, consumer count, codegen ownership, and net
  wall/CPU/memory outcome.
- Build Forest visualization of dependency-owned versus consumer-owned
  codegen and repeated downstream emission.

The prototype must not modify manifests automatically, depend on compiler-
private metadata as a stable interface, or influence release settings.

### Reject or defer

- A FERRIUM source-level crate slicer.
- Stub rlib or deferred type-checking implementation.
- rustc, Cargo, or linker forks.
- Automatic `hint-mostly-unused` adoption.
- Recommendations based only on crate size, API count, rlib shrinkage, or hit
  rate.
- Production dependence on nightly profile syntax.
- Skipping whole-crate errors, coherence, macro expansion, generated code, or
  dynamic-dispatch requirements.
- Generalizing development results to release, LTO, cross-platform, runtime,
  or final binary-size claims.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because current advice preserves whole-crate frontend correctness and rejects source-level semantic slicing. |
| Compiler Performance Engineer | Accepted because frontend work, dependency codegen, consumer codegen, duplication, memory, and end-to-end cost remain separate. |
| Interop Boundary Auditor | Accepted because build scripts, generated code, native dependencies, symbols, target, and ABI remain explicit eligibility boundaries. |
| AI Assurance Skeptic | Accepted because sparse wins, dense regressions, redundant generic/private cases, and unused-body failures remain visible. |
| Ecosystem Strategist | Accepted because FERRIUM evaluates Cargo and rustc mechanisms and contributes upstream rather than creating a competing compiler path. |
| Rust Maintainer | Accepted because the immediate output is a fixture and read-only explanation model, not manifest or source churn. |
| Native Platform Adopter | Accepted because candidate use is reversible, nightly-only, isolated, measured, and excluded from production defaults. |
| Scope Keeper | Accepted because the result authorizes classification and upstream evidence while deferring full slicing and implementation. |
| Validation Checker | Accepted because output identity, wall, CPU, RSS, artifacts, mono items, self-profile, public controls, and negative cases are all required. |

## Limitations

- The generated quantitative matrix used three exploratory repetitions except
  for the five-run METIS control.
- The local environment was Windows x86-64 MSVC on one nightly toolchain.
- The generated release bodies were already cross-crate-inlinable, so local
  release timing differences are noisy and not promoted.
- The METIS control used one small consumer and one dependency entry point.
- No proc-macro, build-script-generated, dynamic-dispatch, coherence-conflict,
  mixed-language, LTO, debugger, sanitizer, coverage, PGO, or multi-platform
  fixture was measured end to end.
- No full crate-slicing implementation exists in this work.
