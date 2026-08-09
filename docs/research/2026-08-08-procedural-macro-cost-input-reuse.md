# Procedural-Macro Cost, Inputs, and Reuse

Date: 2026-08-08
Question: PERF-Q22
Status: Complete
Decision: adopt read-only procedural-macro invocation, input, output,
generated-shape, rerun-cause, and declared-input observability; preserve
tracked-input correctness controls; contribute minimized fixtures where
useful; reject enabling rustc's current experimental derive cache; defer
general caching, sandbox enforcement, macro consolidation, source rewrites,
compiler forks, and implementation.

## Executive conclusion

Procedural macros are not one cost.

Their contribution to a build separates into:

- compiling and loading the procedural-macro crate;
- crossing the compiler bridge for each invocation;
- executing macro logic;
- reading declared or hidden external inputs;
- producing, parsing, and integrating output tokens;
- resolving, lowering, checking, optimizing, and generating the emitted Rust;
- invalidating downstream work when the macro crate, invocation, or input
  changes.

The synthetic PERF-Q22 fixture found modest overhead for trivial macros rather
than universal macro domination. One thousand no-op derives measured 284.31 ms
median versus 259.25 ms for 1,000 plain structs. One thousand derives that
also emitted associated constants measured 313.66 ms. These results establish
that invocation and generated output are visible costs, not that real
procedural macros are cheap.

The input result is more consequential.

Ordinary environment and file reads were invisible to Cargo and left stale
generated output until another source edit forced recompilation. The unstable
`proc_macro::tracked::env_var` and `proc_macro::tracked::path` APIs correctly
caused Cargo to rebuild and rerun the derive under ordinary compilation.

Rustc also contains a disabled-by-default, disk-cached query for derive macro
output. With `-Zcache-proc-macros`, unchanged derives reused output across
dependency edits, identical rewrites, and unrelated source edits; one changed
derive input reran only that invocation; a macro-crate edit invalidated all
three. Attribute and function-like macros did not receive equivalent reuse.

That narrow success is not a production opportunity. With the experimental
cache enabled, both hidden and declared tracked-input changes loaded stale
derive output. Cargo invoked rustc for tracked changes, but the derive itself
did not execute and the program retained the old value. Rustc labels the flag
potentially unsound, and the experiment reproduced a concrete reason.

FERRIUM should make procedural-macro work and inputs explainable before
attempting to cache them. A safe future cache would require a stronger,
versioned execution contract than the current invocation-and-token query key.

## Decision supported

This research determines:

- which procedural-macro cost dimensions belong in the compiler query plan;
- what the current derive cache does and does not reuse;
- why declared Cargo rebuild inputs are necessary but insufficient for cached
  macro output;
- which observability can be external now;
- which cache and sandbox mechanisms belong upstream or behind an explicit
  compatibility boundary.

It does not authorize enabling `-Zcache-proc-macros`, caching arbitrary native
macro execution, rewriting macro call sites, consolidating invocations,
checking in generated source, restricting existing macros, replacing rustc's
bridge, or opening the implementation gate.

## Evidence reviewed

### Local evidence

- [Procedural-macro cost, input, and reuse experiment](perf-q22-procedural-macros/results/EXP-01-proc-macro-cost-input-reuse.md)
- [Declarative macro expansion](2026-08-08-declarative-macro-expansion.md)
- [Incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Early-phase incrementality](2026-08-08-early-phase-incrementality.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Rust compiler and language sources

- [Rust Reference: procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [rustc procedural-macro bridge and derive expansion](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/proc_macro.rs)
- [rustc derive macro query declaration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs)
- [rustc derive cache incremental test](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/tests/incremental/derive_macro_expansion/proc_macro_unchanged.rs)
- [rustc unstable option declaration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/options.rs)
- [tracked environment and path tracking issue 99515](https://github.com/rust-lang/rust/issues/99515)

### Process boundary and sandbox prior art

- [rust-analyzer procedural-macro response protocol](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/tools/rust-analyzer/crates/proc-macro-api/src/bidirectional_protocol/msg.rs)
- [rust-analyzer procedural-macro server loop](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/tools/rust-analyzer/crates/proc-macro-srv-cli/src/main_loop.rs)
- [Watt WebAssembly procedural-macro runtime](https://docs.rs/watt/latest/watt/)

## Current execution model

Rust supports function-like, derive, and attribute procedural macros. Each is
native code executed during compilation over token streams.

In the pinned rustc source:

- `BangProcMacro` records `expand_proc_macro` and calls the bridge with one
  input stream;
- `AttrProcMacro` records the same event and calls the bridge with attribute
  and annotated-item streams;
- `DeriveProcMacro` converts the input item to tokens and either executes the
  derive directly or asks `derive_macro_expansion`;
- execution strategy is same-thread or cross-thread through
  `MaybeCrossThread`;
- the bridge provides compiler services such as spans and tracked inputs.

Same-thread versus cross-thread is an execution choice, not a security
sandbox. The Rust Reference states that procedural macros have the same
resources and file access as the compiler and carry the same security concerns
as Cargo build scripts.

## Cost model

### Invocation count is not total cost

The fixture's 1,000 no-op derives added a measurable but modest median delta
over plain structs. The derive emitting associated constants cost more because
it also:

- constructed output tokens;
- parsed output items;
- integrated additional AST and HIR owners;
- checked the generated associated constants.

The pass-through attribute, identity function-like macro, and one bulk emitter
did not order by invocation count. Source topology and output shape differed.
One invocation can reduce bridge crossings while still generating substantial
downstream work.

This rejects both simple claims:

- "procedural macros dominate because there are many invocations";
- "combine invocations and the build will be faster."

A useful report must preserve invocation topology, macro kind, input size,
output size, generated shape, macro crate identity, and later compiler work.

### Instrumentation must remain separate

The primary wall timings ran without macro logging. The diagnostic run
stringified token streams and wrote an invocation record. The rustc
self-profile was another separate run.

Neither diagnostic is substituted for minimally instrumented Cargo wall time.
The self-profile event is useful for locating bridge execution, while complete
macro-associated cost also appears in expansion, parsing, integration,
lowering, semantic analysis, codegen, and dependency compilation.

## Input model

### Hidden inputs produce stale builds

The untracked environment and file derives used ordinary `std` APIs.

Changing the input did not make Cargo rebuild the application. The old program
output remained in place. A later source edit caused the macro to run and
observe the new input.

The build system cannot safely reuse or invalidate work whose inputs it cannot
name.

### Tracked inputs repair Cargo rerun causality

The tracked variants declared the environment variable or path through
`proc_macro::tracked`.

Changing either input rebuilt the app and reran the derive under ordinary
compilation. Output changed from `1` to `2`.

The APIs remain unstable under issue 99515, whose public API design and
stabilization work are incomplete. They are evidence that input declaration is
possible, not a stable FERRIUM dependency.

### A rebuild dependency is not automatically a macro-cache dependency

The experimental derive cache demonstrated the distinction.

Tracked input changes made Cargo invoke rustc, but the disk-cached derive query
still loaded the previous token output. The macro did not execute, and output
remained stale.

A crate-level rerun edge and a cached-expansion invalidation edge are different
parts of the query plan. A future design must connect them explicitly.

## Current derive cache

Rustc declares:

```text
derive_macro_expansion(
    LocalExpnId,
    input TokenStream
) -> output TokenStream
```

The query is `cache_on_disk`. Its provider also reads the defining procedural
macro crate hash so a macro-crate change invalidates the result.

The pinned compiler only enters this query when:

- incremental compilation is active; and
- `-Zcache-proc-macros` is enabled.

The unstable option defaults to false and is described as potentially
unsound. Rustc has an incremental test proving that an unchanged derive can be
loaded from disk when the flag is enabled.

The PERF-Q22 matrix confirmed that implementation:

- exact unchanged derives reused output;
- one token-input change reran one derive;
- a macro-crate change reran all derives;
- attribute and function-like macros continued to execute.

It also confirmed that the current key and dependencies do not form a complete
external-input contract.

## Minimum future cache identity

A safe cache cannot be keyed only by a rendered token string. At minimum, the
identity and validation contract must account for:

1. procedural-macro artifact and dependency identity;
2. macro entry point and kind;
3. input token trees;
4. spans, source mapping, and hygiene where observable;
5. declared environment variables and values;
6. declared file paths, content identity, and path semantics;
7. compiler, bridge protocol, edition, target, cfg, and relevant options;
8. working directory and other explicitly allowed execution context;
9. diagnostics, panics, tracked reads, and emitted token output;
10. output parsing and integration compatibility.

Undeclared filesystem, environment, process, time, randomness, network, and
native-library inputs prevent deterministic reuse. A system can reject them,
record them through a controlled capability boundary, or decline to cache the
invocation. It cannot safely ignore them.

## Process isolation and sandboxing

Rust-analyzer runs procedural macros through a separate server process and its
protocol returns:

- expanded token trees;
- span data;
- tracked environment variables;
- tracked paths.

This is valuable process and protocol separation. It can support crash
containment, lifecycle control, and observability. It is not by itself a
security sandbox: a separate native process can retain the user's filesystem,
environment, network, and operating-system privileges unless those
capabilities are explicitly restricted.

Watt demonstrates a stricter prior-art boundary. It executes procedural macros
compiled to WebAssembly and limits their interaction to consuming and
producing tokens. That creates a deterministic, isolatable class of macro, but
it requires a different build and publication workflow, has runtime and
tooling tradeoffs, and is not a transparent compatibility solution for the
existing native ecosystem.

FERRIUM should distinguish:

- process boundary;
- capability sandbox;
- deterministic input contract;
- cache identity;
- ecosystem compatibility.

They are related but not interchangeable.

## Findings

### FERRIUM-270: procedural-macro cost has separate execution and generated-code dimensions

**Sources:** EXP-01 cost matrix, rustc procedural-macro source, and PERF-Q10.

**Observed behavior:** Invocation crossings, macro logic, token output,
output parsing, AST integration, and later generated-code work appeared as
separate costs. One scalar "macro time" did not represent the complete effect.

**Implication:** The compiler query plan must model macro execution and emitted
Rust work separately.

**Confidence:** High on the model; medium on representative magnitudes.

### FERRIUM-271: trivial procedural macros did not dominate the synthetic warm check

**Sources:** EXP-01 primary cost matrix.

**Observed behavior:** One thousand no-op derives measured 284.31 ms median
against 259.25 ms for 1,000 plain structs. Derives emitting associated
constants measured 313.66 ms.

**Implication:** Reject the starting assumption that a small number of macros
necessarily dominates. Inventory real macro logic and generated work before
prioritizing an optimization.

**Confidence:** High for this fixture; low for ecosystem generalization.

### FERRIUM-272: invocation consolidation is not an automatic source recommendation

**Sources:** EXP-01 cost matrix.

**Observed behavior:** One bulk emitter had fewer bridge crossings, but the
scenarios differed in source parsing, expansion topology, and generated shape.
Invocation count did not order complete latency.

**Implication:** Do not combine macros solely to reduce invocation count.
Require API, diagnostics, hygiene, maintenance, and end-to-end validation.

**Confidence:** High.

### FERRIUM-273: tracked inputs restore Cargo-visible rerun causality

**Sources:** EXP-01 ordinary input matrix and issue 99515.

**Observed behavior:** Tracked environment and path changes rebuilt the app,
reran the derive, and updated output. Ordinary `std` reads did not.

**Implication:** Declared-input reporting is a defensible observability and
correctness surface.

**Confidence:** High on the nightly fixture; medium on stabilization direction.

### FERRIUM-274: hidden procedural-macro inputs can leave stale artifacts

**Sources:** EXP-01 ordinary input matrix and Rust Reference security model.

**Observed behavior:** Changing an untracked environment variable or file did
not rebuild the app. Program output remained stale until a later source edit.

**Implication:** Hidden inputs are a correctness boundary, not only a cache
efficiency problem.

**Confidence:** High.

### FERRIUM-275: rustc has a narrow, disabled derive-output disk cache

**Sources:** rustc query declaration, derive provider, option declaration,
incremental test, and EXP-01 cache matrix.

**Observed behavior:** With incremental compilation and
`-Zcache-proc-macros`, unchanged derive output loaded across unrelated
rebuilds. The option defaults off and is marked potentially unsound.

**Implication:** Describe the cache as compiler precedent, not supported
production behavior.

**Confidence:** High.

### FERRIUM-276: the derive cache invalidates token-local and macro-crate changes precisely

**Sources:** EXP-01 cache matrix.

**Observed behavior:** Editing one derive input reran one of three derives.
Editing the defining macro crate reran all three. Unrelated edits reran none.

**Implication:** Invocation-level reuse is technically possible when its
identity is complete.

**Confidence:** High for the fixture.

### FERRIUM-277: the current derive cache can reuse stale tracked-input output

**Sources:** EXP-01 cached-input matrix and rustc option warning.

**Observed behavior:** Tracked environment and path changes caused Cargo to
rebuild, but the cached derive did not execute and output remained stale.

**Implication:** Reject enabling `-Zcache-proc-macros`. Any future cache must
bind declared inputs into invocation-level invalidation.

**Confidence:** High.

### FERRIUM-278: attribute and function-like macros have no equivalent disk-cached query

**Sources:** rustc procedural-macro source, query declaration, and EXP-01 cache
matrix.

**Observed behavior:** Attribute and function-like macros executed on every
forced app compilation while unchanged derives loaded from disk.

**Implication:** Do not generalize derive cache behavior to all procedural
macro kinds.

**Confidence:** High.

### FERRIUM-279: safe cache identity requires an execution contract

**Sources:** findings FERRIUM-270 through FERRIUM-278, Rust Reference, rustc
source, and rust-analyzer protocol.

**Observed behavior:** Token input and macro crate identity explained local
reuse but not environment, file, span, diagnostic, protocol, or execution
context.

**Implication:** Define allowed capabilities, declared inputs, output evidence,
and invalidation semantics before defining a cache store.

**Confidence:** High on the requirement; medium on final key design.

### FERRIUM-280: procedural macros execute with compiler-level resources

**Sources:** Rust Reference and rustc bridge source.

**Observed behavior:** Procedural macros can access files and other resources
available to the compiler. Same-thread and cross-thread execution do not
restrict those capabilities.

**Implication:** Supply-chain and sandbox claims require explicit operating
system or runtime capability controls.

**Confidence:** High.

### FERRIUM-281: rust-analyzer provides a useful server protocol, not a security verdict

**Sources:** rust-analyzer response protocol and server loop.

**Observed behavior:** Expansion occurs across a process protocol that returns
tokens, spans, tracked environment variables, and tracked paths.

**Implication:** Reuse the process/protocol idea for observability research,
but do not call process separation a sandbox.

**Confidence:** High.

### FERRIUM-282: Watt proves a deterministic token-only macro class is possible

**Sources:** Watt documentation.

**Observed behavior:** Watt executes WebAssembly procedural macros behind a
token-only boundary and explicitly targets isolation and determinism, while
documenting performance and tooling tradeoffs.

**Implication:** Treat deterministic sandboxed macros as an opt-in compatibility
class and prior art, not as a transparent migration for native macros.

**Confidence:** High on prior art; medium on current ecosystem applicability.

### FERRIUM-283: observability should precede procedural-macro caching

**Sources:** findings FERRIUM-270 through FERRIUM-282 and the role review.

**Observed behavior:** Cost attribution, hidden inputs, unstable declared
inputs, incomplete cache identity, arbitrary native capability, and generated
downstream work remain distinct.

**Implication:** Adopt read-only telemetry and correctness controls now. Defer
cache activation or enforcement until representative evidence and an upstream
contract exist.

**Confidence:** High.

## Recommendations

### Adopt now

- Add procedural-macro crate, entry point, kind, invocation count, input size,
  output size, generated shape, rerun cause, declared inputs, and later
  generated-code work to the measurement contract.
- Preserve tracked and untracked environment/file fixtures as correctness
  controls.
- Report Cargo skipping rustc, rustc invoking a macro, and rustc loading a
  cached derive as different events.
- Use minimally instrumented Cargo wall time as primary evidence and keep macro
  logs and self-profile separate.
- Contribute minimized rustc-perf or compiler tests when a real macro exposes
  a reproducible compiler regression or invalidation gap.

### Prototype behind a compatibility boundary

- A read-only procedural-macro inventory and execution report.
- Declared-input and hidden-input diagnostics that do not claim completeness.
- Versioned protocol research for deterministic opt-in macros.
- Sandbox experiments only with explicit capability, compatibility, and
  rollback contracts.

### Reject or defer

- Enabling `-Zcache-proc-macros`.
- General caching of arbitrary native procedural macros.
- Treating Cargo rerun edges as sufficient cache identity.
- Calling a thread or process boundary a sandbox.
- Combining or rewriting macros solely for fewer invocations.
- Checking in generated Rust as a general optimization.
- Replacing rustc's bridge or creating a FERRIUM compiler fork.
- Folding build-script work into this result; PERF-Q23 remains separate.

## Role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted with a hard objection to the current derive cache: tracked changes produced stale output, so no cache activation or correctness claim is allowed. |
| Compiler Performance Engineer | Accepted: primary wall time, variance, invocation diagnostics, compiler events, generated work, and synthetic limitations remain separate. |
| Interop Boundary Auditor | Accepted with deferral: native libraries, target-specific loading, host/target differences, ABI effects, and cross-platform sandbox controls remain unmeasured. |
| AI Assurance Skeptic | Accepted: the corrected cache control, stale-output failure, outliers, observer effects, and single-host limits remain visible. |
| Ecosystem Strategist | Accepted: rustc, tracked APIs, rust-analyzer, and Watt are treated as existing mechanisms and prior art; FERRIUM retains an explanation wedge. |
| Rust Maintainer | Accepted: ordinary Cargo remains authoritative, no source rewrite is proposed, and diagnostics are intended to be removable and read-only. |
| Native Platform Adopter | Accepted for research only: adoption needs stable APIs, platform coverage, capability policy, rollback, audit, and support evidence. |
| Scope Keeper | Accepted: Q22 closes with one synthetic matrix and a bounded observability decision; build scripts remain in Q23. |
| Validation Checker | Accepted: commands, toolchain, matched controls, cache-on/off states, tracked/untracked failures, variance, and limitations are recorded. |

## Non-goals

- Claim procedural macros generally dominate Rust compilation.
- Claim trivial synthetic macros represent common ecosystem derives.
- Recommend bulk generation or macro consolidation from this matrix.
- Treat tracked input APIs as stable.
- Treat `-Zcache-proc-macros` as safe.
- Treat rust-analyzer's server as a sandbox.
- Design a production cache key or sandbox protocol in this question.
- Open the FERRIUM implementation gate.
