# Linking and Incremental Linking

Date: 2026-08-09
Question: PERF-Q29
Status: Complete
Decision: add a read-only linker plan and state ledger to the compiler query
plan and labeled Build Forest; distinguish complete linking, incremental
preparation, reusable linker state, object identity, optimization policy,
debug packaging, output bytes, fallback, and release finalization; prototype
stable Rust linker-input identity only as an upstream compatibility
investigation; defer automatic linker selection, profile changes, persistent
ILK management, source changes, CI/editor changes, or a FERRIUM linker.

## Executive conclusion

Native linking is a real Rust iteration region, but a faster unchanged relink
is not the same as a faster Rust edit loop.

On the dependency-heavy public FLETCH executable, the ordinary MSVC complete
link took a 749.1 ms median. Replaying the same inputs with `rust-lld` took
716.2 ms, a 4.4% improvement, while reducing peak process-tree RSS 29.6%.
Faster complete-link engines remain useful, but this case did not expose an
order-of-magnitude gap.

MSVC incremental linking did expose one. An unchanged prepared relink took
183.7 ms, 75.5% less wall time than the ordinary full link, and reported zero
changed modules. The result required changing rustc's ordinary
`/OPT:REF,NOICF` policy to `/OPT:NOREF,NOICF`, retaining a 53.2 MB ILK, and
accepting an 82.5% larger executable and a 33.7% larger PDB. Microsoft
explicitly recommends a non-incremental final release link.

The gain did not survive the measured Rust edit. One body change regenerated
181 old root object paths as 182 entirely new paths. MSVC reported 182 new and
181 changed modules, then performed a full link in 910.4 ms. Current Rust CGU
filename and crate identity behavior defeated the native incremental linker's
reuse key before linker algorithms could help.

This changes the opportunity. FERRIUM should not build another complete linker
or automatically enable MSVC incremental linking. It should explain the link
plan and state, measure whether input identity survives the edit, expose
fallback and artifact cost, and contribute minimized upstream fixtures for a
stable linker-input identity or explicit incremental-link handoff. Wild is the
strongest ecosystem collaboration path for future Rust-native incremental
linking, but it does not yet implement incrementality or Windows support.
`lld-link` and mold remain fast complete-link approaches rather than stateful
incremental solutions.

The immediate user-facing abstraction is a **link capability contract**:

```text
target and ABI compatibility
  + debug and symbol packaging
  + edit-to-runnable latency
  + release optimization and finalization
  + reproducibility
  + native library and mixed-language support
  + signing, deployment, and rollback
```

A workflow declares the capabilities it requires. Measurement determines
whether the supported platform linker, `rust-lld`, Wild, mold, or another
engine satisfies them. Linker names and flags are implementation choices, not
the requirement.

## Decision supported

This research determines:

- how to isolate native linking from rustc object emission;
- when a complete-link replacement materially helps;
- when MSVC incremental state can reduce repeated link latency;
- which `/OPT`, PDB, ILK, padding, thunk, and release policies accompany it;
- why stable object identity is a prerequisite for Rust incremental linking;
- how debug level and CGU count affect link inputs and outputs;
- which behavior belongs in FERRIUM explanation, supported configuration,
  upstream rustc fixtures, or linker-project collaboration.

It does not authorize automatic linker configuration, `/OPT` changes,
persistent ILK lifecycle management, source or CGU changes, CI/editor defaults,
release policy changes, or implementation.

## Evidence reviewed

### Local evidence

- [EXP-01 linker matrix](perf-q29-linking/results/EXP-01-linker-matrix.md)
- [Debug information and object emission](2026-08-09-debug-information-object-emission.md)
- [Codegen-unit partitioning](2026-08-09-codegen-unit-partitioning.md)
- [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Rust and linker sources

- [Pinned rustc native link path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_ssa/src/back/link.rs)
- [Microsoft `/INCREMENTAL` reference](https://learn.microsoft.com/en-us/cpp/build/reference/incremental-link-incrementally?view=msvc-170)
- [Microsoft `/OPT` reference](https://learn.microsoft.com/en-us/cpp/build/reference/opt-optimizations?view=msvc-170)
- [LLVM Windows linker support](https://lld.llvm.org/windows_support.html)
- [LLVM incremental-linking issue](https://github.com/llvm/llvm-project/issues/35265)
- [Wild linker](https://github.com/wild-linker/wild)
- [mold linker](https://github.com/rui314/mold)

## Current linking model

```text
Rust edit
  -> Cargo freshness and rustc invalidation
  -> mono-item collection and CGU partition
  -> native object and archive emission
  -> link-plan construction
       -> object and library identities
       -> target, ABI, subsystem, exports, and native search paths
       -> optimization and dead-code policy
       -> debug, PDB, symbols, NATVIS, and unwind policy
  -> linker engine
       -> complete link
       -> or prepared incremental state lookup
  -> executable, PDB/debug package, ILK/state, maps, and diagnostics
  -> smoke, debugger, ABI, runtime, signing, and release validation
```

An incremental linker can reuse only what arrives under compatible identities.
Compiler invalidation, CGU partition churn, object naming, changed link options,
added or removed objects, missing state, and final-release policy can each turn
an incremental request into a complete link.

## Findings

### FERRIUM-387: native linking is a distinct measurable region

**Sources:** EXP-01 link-only proxy and direct linker replays.

**Observed behavior:** nightly no-link output allowed native object generation
and final linking to be separated. The direct linker process tree could be
measured without attributing LLVM emission to linking.

**Implication:** The compiler query plan must represent object emission,
link-plan construction, native linker execution, and output validation
separately.

**Confidence:** High.

### FERRIUM-388: `.rlink` is an unstable isolation aid, not a complete contract

**Sources:** EXP-01 custom-linker and custom-link-argument probes.

**Observed behavior:** link-only consumed transient objects and did not preserve
the tested earlier custom linker or custom link argument. The linker choice had
to be supplied again and inputs captured before execution.

**Implication:** FERRIUM may use no-link/link-only in pinned experiments but
must not make `.rlink` a durable product or cache interface.

**Confidence:** High for the pinned toolchain; low for future format behavior.

### FERRIUM-389: `rust-lld` was a modest complete-link improvement

**Sources:** EXP-01 FLETCH complete-link matrix.

**Observed behavior:** `rust-lld` reduced the 749.1 ms MSVC median to 716.2 ms,
or 4.4%, and reduced peak RSS 29.6%.

**Implication:** Supported complete-linker comparison belongs in diagnosis and
configuration guidance, but this fixture does not justify an automatic switch.

**Confidence:** High for the fixture; medium for prevalence.

### FERRIUM-390: PDB work remains material under both complete linkers

**Sources:** EXP-01 FLETCH outputs and PERF-Q28.

**Observed behavior:** both complete linkers produced approximately 85 MB PDBs
for a 10.8 MB executable. The engine change did not remove debug packaging.

**Implication:** Linker selection cannot substitute for the debug capability
contract. Object debug production, PDB merging, debugger quality, and retention
remain separate decisions.

**Confidence:** High for artifact observation.

### FERRIUM-391: unchanged MSVC incremental linking was much faster

**Sources:** EXP-01 prepared and unchanged FLETCH rows.

**Observed behavior:** the unchanged prepared relink took 183.7 ms and reported
zero changed modules, 75.5% faster than the ordinary full link.

**Implication:** Stateful incremental linking is a real latency mechanism worth
upstream and ecosystem collaboration when compiler inputs remain stable.

**Confidence:** High.

### FERRIUM-392: incremental preparation has substantial artifact cost

**Sources:** EXP-01 executable, PDB, and ILK inventories.

**Observed behavior:** the prepared executable was 82.5% larger, the unchanged
PDB 33.7% larger, and the ILK 53.2 MB.

**Implication:** Record storage, transport, padding, thunks, image, PDB, and ILK
retention beside latency. Do not describe incremental linking as a free switch.

**Confidence:** High.

### FERRIUM-393: rustc's ordinary `/OPT:REF` policy conflicts with MSVC incrementality

**Sources:** captured rustc link plan; Microsoft `/OPT` and `/INCREMENTAL`
references; EXP-01 negative control.

**Observed behavior:** rustc requested `/OPT:REF,NOICF`. MSVC warned that
`/OPT:REF` disables `/INCREMENTAL` and performed a full link. The prepared
experiment required `/OPT:NOREF,NOICF`.

**Implication:** Incremental linking changes dead-code retention and output
policy. It requires an explicit development identity and non-incremental
release finalization.

**Confidence:** High.

### FERRIUM-394: linker state has explicit invalidation and fallback rules

**Sources:** Microsoft `/INCREMENTAL` reference; EXP-01 missing-ILK control.

**Observed behavior:** deleting the ILK recreated it with full-link-scale
latency. Microsoft also names missing output, changed timestamps, changed
options, and added or omitted objects as full-link causes.

**Implication:** A linker ledger must expose state identity, disposition, and
fallback reason rather than reporting only that `/INCREMENTAL` was requested.

**Confidence:** High.

### FERRIUM-395: one Rust body edit defeated MSVC incremental reuse

**Sources:** EXP-01 FLETCH source-edit object-set comparison and verbose
incremental diagnostics.

**Observed behavior:** 181 old root object paths became 182 entirely new paths.
MSVC reported 182 new and 181 changed modules and performed a full link in
910.4 ms.

**Implication:** The relevant optimization target is not only linker speed. Rust
needs stable enough object or linker-module identity across eligible edits for
stateful native incrementality to apply.

**Confidence:** High for the fixture and edit; medium for prevalence.

### FERRIUM-396: CGU partition identity crosses the compiler-linker boundary

**Sources:** PERF-Q25 partition findings; EXP-01 source edit and CGU controls.

**Observed behavior:** compiler CGU naming and partition output formed the
linker's module keys. One body edit changed the complete root-object set.

**Implication:** A stable linker-input design must join rustc incremental
identity, CGU partitioning, object naming, libraries, and linker-state
compatibility. A linker-only patch cannot solve the whole path.

**Confidence:** High for the observed mechanism.

### FERRIUM-397: small link controls did not predict the public result

**Sources:** EXP-01 synthetic and FLETCH matrices.

**Observed behavior:** synthetic complete links clustered near 83 to 101 ms and
showed startup-scale variance. FLETCH exposed 716 to 749 ms complete links,
large PDBs, and useful incremental-state economics.

**Implication:** Keep synthetic controls for causality but require
dependency-heavy public executable fixtures for product claims.

**Confidence:** High.

### FERRIUM-398: current linker projects occupy different opportunity layers

**Sources:** LLVM, Wild, mold, and Microsoft sources.

**Observed behavior:** MSVC supplies stateful Windows incrementality;
`lld-link` supplies fast complete COFF/PDB links without MSVC-style
incrementality; mold supplies fast Unix complete links; Wild targets future
incrementality but currently supports Linux complete links and not Windows.

**Implication:** Collaborate rather than duplicate. Use platform-supported
configuration now, contribute Rust input-identity fixtures upstream, and track
Wild for the cross-platform incremental design lane.

**Confidence:** High for documented current support.

### FERRIUM-399: linker choice is a capability and lifecycle decision

**Sources:** findings 389 through 398 and the debug capability contract.

**Observed behavior:** linkers differed in wall time, memory, PDB bytes,
incremental state, optimization compatibility, platform support, and final
artifact lifecycle.

**Implication:** Express needs through target, ABI, debug, iteration, release,
reproducibility, native-library, signing, deployment, and rollback
capabilities. Do not expose one universal "fast linker" label.

**Confidence:** High.

### FERRIUM-400: the immediate opportunity is a linker plan and state ledger

**Sources:** findings 387 through 399 and role review.

**Observed behavior:** existing tools perform the links, but maintainers lack
one joined explanation of input identity, engine, policy, state reuse, fallback,
artifact cost, and validation.

**Implication:** Add a read-only ledger and comparison overlay. Keep changes
advisory, reversible, and human-approved. File upstream fixtures only after
cross-platform minimization.

**Confidence:** High for the evidence gap; medium for adoption.

## Recommendations

### Adopt now

- Measure native linker execution separately from LLVM and object emission.
- Record linker engine and version, target, ABI, subsystem, object/library
  identities, debug policy, `/OPT`, incremental request, state disposition,
  fallback reason, output bytes, PDB/debug bytes, and validation outcome.
- Preserve verbose incremental diagnostics in diagnostic runs.
- Use `rust-lld` or platform alternatives only through supported,
  repository-owned configuration and measured rollback.
- Produce final release artifacts with the platform's required non-incremental
  optimization, signing, and validation policy.
- Contribute minimized rustc fixtures for unstable object identity across
  otherwise eligible body edits.

### Prototype behind a compatibility boundary

- A read-only linker plan and state ledger in the compiler query plan.
- Build Forest sibling roots for complete and incrementally prepared
  development outputs with separate artifact and validation identities.
- An upstream experiment for stable linker-module identity across eligible Rust
  edits.
- A linker capability probe covering target format, PDB/DWARF/dSYM, native
  libraries, LTO, exports, incremental support, response files, maps, and
  diagnostics.
- Collaboration fixtures for Wild and LLVM rather than a FERRIUM linker.

### Reject or defer

- Universal `rust-lld`, mold, Wild, or MSVC incremental defaults.
- Automatic `.cargo/config.toml`, profile, `/OPT`, CGU, source, CI, editor, or
  release changes.
- Treating an incremental request as proof that an incremental link occurred.
- Persisting or transporting ILK state without provenance, locking, cleanup,
  integrity, and failure contracts.
- Reusing validation claims across complete, prepared, debug, release, linker,
  or state identities.
- A FERRIUM linker, PDB implementation, object format, compiler fork, or
  package manager.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because linker success is not treated as behavioral or safety proof; panic, unwind, runtime, ABI, and unsafe evidence remain separate. |
| Compiler Performance Engineer | Accepted because emission, complete link, prepared link, unchanged replay, source edit, fallback, CPU, memory, storage, variance, and public-fixture evidence remain distinct. |
| Interop Boundary Auditor | Accepted provisionally because ABI, exports, native libraries, exceptions, unwind, allocation, mixed-language debugging, signing, and non-MSVC formats remain open gates. |
| AI Assurance Skeptic | Accepted because failed passive capture, ignored incrementality, missing-state fallback, source-edit full link, unstable `.rlink`, and unmeasured debugger behavior remain visible. |
| Ecosystem Strategist | Accepted because MSVC, LLVM, mold, Wild, rustc, and Cargo remain the implementation owners; FERRIUM contributes evidence and avoids duplicating a linker. |
| Rust Maintainer | Accepted because ordinary Cargo remains valid, configuration stays repository-owned and removable, and diagnostics explain why a link was complete or incremental. |
| Native Platform Adopter | Accepted provisionally because platform support, native libraries, debug tooling, signing, deployment, release finalization, operations, audit, and rollback require consumer validation. |
| Scope Keeper | Accepted because PERF-Q29 owns final linking and linker state while PERF-Q25 owns partitioning, PERF-Q28 owns debug emission, and PERF-Q30 owns remote state provenance. |
| Validation Checker | Accepted because the work records toolchains, commands, repeated distributions, a public fixture, negative controls, output behavior, artifact bytes, fallback diagnostics, and limitations. |

No role authorizes implementation or automatic linker selection.

## Product implication

The compiler query plan should add:

```text
workflow and edit class
  -> required link capability contract
  -> emitted object and library identities
  -> link plan
       -> linker engine and version
       -> target, ABI, subsystem, exports, and native inputs
       -> debug, symbols, unwind, maps, and packaging
       -> dead-code, folding, LTO, and release policy
  -> incremental state
       -> requested, eligible, reused, invalidated, missing, or rejected
       -> ILK or engine-specific identity and bytes
       -> changed, added, and removed module set
       -> fallback reason
  -> wall, CPU, memory, executable, PDB/debug, and state bytes
  -> smoke, debugger, ABI, runtime, signing, deployment, and rollback validation
```

The Build Forest may retain sibling complete, prepared-development, and
final-release roots. It must preserve their object sets, linker state, outputs,
debug packages, validation, and rollback dispositions separately.

## Prototype gate

No linker advisor or configuration implementation may begin until:

1. at least three Tier 1 executable repositories reproduce clean and edit
   measurements;
2. Linux ELF with `lld`, mold, and Wild, macOS Mach-O, and Windows COFF/PDB
   evidence exists;
3. object identity is tested across private body, public API, generic, inline,
   macro, build-script, binary-only, and revert edits;
4. native libraries, exports, dynamic libraries, tests, proc macros, LTO, panic,
   unwind, and mixed-language boundaries are covered;
5. debugger, crash, symbol, ABI, runtime, reproducibility, signing, and release
   finalization controls pass;
6. state provenance, locking, corruption, cleanup, disk pressure, and recovery
   are defined;
7. ordinary Cargo, editor, CI, opt-out, rollback, and support workflows remain
   valid;
8. held-out maintainers find fallback explanations actionable;
9. `.roles` approves one bounded implementation pulse.

Until then, the output is research, read-only modeling, supported manual
configuration experiments, and upstream fixtures.

## Limitations and next research

- Repeat on Linux and macOS with their native object and debug formats.
- Sweep Rust edit classes and CGU/object identity stability.
- Measure native static and dynamic libraries, exports, mixed Rust/C++, LTO,
  tests, and large generated binaries.
- Validate interactive debugging, crash dumps, panic/unwind, signing, runtime,
  deterministic output, and release behavior.
- Minimize the source-edit object-renaming case for upstream rustc.
- Track and contribute compatible fixtures to Wild and LLVM.
- Keep remote linker state and Build Forest transport behind PERF-Q30.

## Final decision

Record linking as a first-class compiler query-plan region. Add a read-only
linker plan and state ledger. Treat faster complete linking and stateful
incremental linking as different mechanisms. Prioritize stable Rust
linker-input identity and upstream collaboration, keep final release linking
non-incremental where required, and defer automatic configuration or a new
FERRIUM linker.
