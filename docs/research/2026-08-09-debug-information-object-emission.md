# Debug Information and Object Emission

Date: 2026-08-09
Question: PERF-Q28
Status: Complete
Decision: add a read-only debug and emission ledger to the compiler query plan
and labeled Build Forest; preserve crate debug level, target format, CodeView
or DWARF policy, CGUs, object/debug/PDB/archive/incremental bytes, backend
emission regions, linker inputs, and debugger validation separately; treat
line tables as a measured profile candidate, not a default; defer automatic
Cargo profile, split-debug, strip, CGU, linker, source, CI, or editor changes.

## Executive conclusion

Debug information is a material development-build cost, but it is not one
trailing file-write step and it does not justify a universal "disable debug"
rule.

On the pinned x86_64 Windows MSVC nightly, full debug information increased a
five-repetition synthetic object-only median 32.8%, CPU 26.7%, and object bytes
251.1% relative to no crate debuginfo. A separate rustc self-profile showed
`LLVM_module_codegen_emit_obj` rising from 46.3 ms to 102.9 ms. Debug metadata
is created during IR translation, processed through LLVM, lowered into
CodeView, written into COFF objects, and packaged by the native link/PDB
pipeline.

The public METIS-CORE control showed a broader operational effect. Full debug
increased clean Cargo build wall time 21.9%, total target storage 62.8%,
incremental storage 94.2%, and the root Rlib 244.9%. Full CodeView occupied
64.3% of that root archive.

The native linker was not the dominant cause in the synthetic control.
`run_linker` changed from 92.9 ms without crate debug information to 93.5 ms
with full information, while the coarse LLVM region more than doubled.
PERF-Q28 therefore separates debug production and object emission from
PERF-Q29's broader linker problem.

Line tables were a meaningful middle point. They increased synthetic
object-only wall time 16.6% and produced procedure and source-line records
without local-variable records. In the public control they increased target
storage 13.8%, compared with 62.8% for full. The `limited` level produced the
same synthetic CodeView sections and nearly identical public archive evidence
as line tables on this toolchain.

That is not enough to make line tables a FERRIUM default. Full debug emitted
8,626 synthetic local records and 34,062 METIS local records that the reduced
levels omitted, plus far richer type records. Teams relying on local-variable,
type, expression, optimized-frame, panic, or mixed-language debugging may be
giving up essential capability. The host lacked an applicable rust-lldb,
`cdb`, `lldb`, `llvm-pdbutil`, and `dumpbin`, so interactive debugger behavior
remains explicitly unmeasured.

MSVC also changes the artifact model. rustc's stable target policy supports
packed split debuginfo: CodeView records are emitted in objects and the linker
produces a separate PDB. The no-crate-debuginfo control still produced a
baseline PDB from the complete native link context. Total PDB size is
therefore not current-crate debug size; matched deltas and object CodeView
sections are required.

Stripping did not avoid the cost. `-Cstrip=debuginfo` and
`-Cstrip=symbols` produced the same measured full-debug EXE and PDB bytes as
the unstripped packed control. Strip is a final-link retention policy, not a
promise that rustc will skip debug metadata generation and object emission.

More CGUs also exposed a trade. Sixteen full-debug CGUs shortened synthetic
object wall time 15.5% through parallelism, while increasing object bytes 20.2%
and debug-section bytes 33.1%. A profile recommendation must preserve latency,
CPU, memory, disk, linker-input, runtime, debugger, and failure evidence
together.

FERRIUM's opportunity is explanation and evidence. The compiler query plan
should show which workflow reaches debug construction and native emission,
what bytes are produced where, what the profile buys, what debugger capability
is required, and which alternative root was measured. The Build Forest may
retain sibling full, line-table, limited, or no-debug roots, but must not
relabel their artifacts or validation claims as equivalent.

## Decision supported

This research determines:

- how crate debug level enters backend and artifact identity;
- why debug construction, LLVM processing, object emission, linking, PDB
  packaging, and final stripping are separate regions;
- which object, archive, PDB, executable, and incremental bytes must be
  measured;
- why total PDB size is not current-crate debug attribution;
- how CGU parallelism trades latency against object and debug bytes;
- what evidence is required before recommending a reduced development debug
  profile;
- which cases belong in a minimized upstream emission fixture.

It does not authorize automatic profile edits, reduced debugger capability,
strip or split-debug changes, CGU changes, linker selection, source rewrites,
CI or editor defaults, or implementation.

## Evidence reviewed

### Local evidence

- [EXP-01 debug emission matrix](perf-q28-debug-object-emission/results/EXP-01-debug-emission-matrix.md)
- [LLVM optimization cost](2026-08-09-llvm-optimization-cost.md)
- [Codegen-unit partitioning](2026-08-09-codegen-unit-partitioning.md)
- [Development codegen backends](2026-08-09-development-codegen-backends.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Rust, Cargo, LLVM, and platform sources

- [Pinned rustc codegen options](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/doc/rustc/src/codegen-options/index.md)
- [Pinned rustc LLVM debuginfo implementation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs)
- [Pinned rustc backend write path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_ssa/src/back/write.rs)
- [Pinned rustc native link path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_codegen_ssa/src/back/link.rs)
- [Pinned MSVC target debuginfo policy](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_target/src/spec/base/msvc.rs)
- [Cargo profile debug reference](https://doc.rust-lang.org/cargo/reference/profiles.html#debug)
- [Microsoft linker PDB reference](https://learn.microsoft.com/en-us/cpp/build/reference/pdb-use-program-database?view=msvc-170)
- [LLVM PDB format documentation](https://llvm.org/docs/PDB/index.html)

## Current debug and emission model

### Debug level changes backend work before the linker

rustc's debug level controls which source locations, procedures, variables,
types, and scopes are represented in LLVM debug metadata. Full function
signatures and template parameters are only created for full debuginfo in the
pinned LLVM backend.

The pipeline is:

```text
Rust source and spans
  -> rustc debug metadata during IR translation
  -> LLVM optimization and machine-code pipeline
  -> CodeView or DWARF records in native objects
  -> archive or native linker input
  -> PDB, dSYM, DWP, executable sections, or retained objects
  -> debugger, profiler, backtrace, crash, and diagnostic behavior
```

No one size or timer represents this pipeline.

### Windows MSVC uses CodeView objects and packed PDB output

The pinned target selects COFF, `DebuginfoKind::Pdb`, packed split debuginfo,
and only packed as the stable supported split mode. LLVM module flags request
CodeView. The linker creates the PDB and writes a CodeView debug-directory
entry into the executable with PDB name, GUID, and age.

PDB bytes are distinct from executable bytes, but they are not isolated
current-crate bytes. Precompiled Rust dependencies and native linker inputs
participate in the final program database.

### Debugger capability is the benefit side of the trade

Line records can support file-and-line backtraces or source stepping. Full
local and type records support richer variable and type inspection. Exact
quality depends on target format, linker, debugger, optimization, inlining,
panic and unwind metadata, native dependencies, source mapping, and tool
versions.

Reducing debug data without measuring the consumer's diagnostic workflow is a
coverage reduction, not a free optimization.

### CGU policy and incremental storage amplify emission choices

Every CGU can carry repeated file, type, line, symbol, and relocation context.
More CGUs can emit in parallel and improve wall time while increasing object
count and repeated debug bytes.

Cargo incremental compilation retains backend work products and related state.
A debug policy can therefore affect both current command output and the
persistent storage used by future edits.

## Findings

### FERRIUM-371: debug information is a pipeline, not a trailing file

**Sources:** pinned rustc debuginfo, write, and link paths; EXP-01.

**Observed behavior:** full debug changed IR translation, LLVM processing,
object emission, COFF content, PDB size, and persistent Cargo bytes.

**Implication:** Plans must separate debug construction, backend processing,
native object emission, link/PDB packaging, final retention, and debugger use.

**Confidence:** High.

### FERRIUM-372: Cargo development defaults to the most expensive debug level

**Sources:** rustc codegen option reference; Cargo profile reference.

**Observed behavior:** Cargo's development profile defaults to full debug
information, including types and variables.

**Implication:** The default has real diagnostic value and real build/storage
cost. Neither side can be omitted from a profile recommendation.

**Confidence:** High.

### FERRIUM-373: MSVC packed PDB is a target-specific artifact model

**Sources:** pinned MSVC target policy; LLVM debuginfo path; Microsoft and LLVM
PDB references; EXP-01.

**Observed behavior:** COFF objects carried CodeView sections and the linker
created a separate PDB named by the executable debug directory.

**Implication:** Windows results cannot be copied to ELF/DWARF, split DWARF,
macOS dSYM, or Windows GNU without direct evidence.

**Confidence:** High.

### FERRIUM-374: full debug materially increased object-emission latency

**Sources:** EXP-01 primary and self-profile matrices.

**Observed behavior:** full debug increased synthetic object-only wall time
32.8%. In the separate diagnostic,
`LLVM_module_codegen_emit_obj` increased from 46.3 ms to 102.9 ms.

**Implication:** Debug-heavy development builds need an explicit emission
region in the compiler query plan and upstream performance fixtures.

**Confidence:** High for this fixture and toolchain.

### FERRIUM-375: line tables were a measured middle point

**Sources:** EXP-01 synthetic and METIS matrices.

**Observed behavior:** line tables retained procedures and source lines without
local records. They added 16.6% synthetic object wall time and 13.8% public
target storage, versus 32.8% and 62.8% for full.

**Implication:** Line tables are a candidate for consumer-specific prototype
comparison where source locations matter more than interactive local/type
inspection.

**Confidence:** High for the measured fixtures; low for debugger sufficiency.

### FERRIUM-376: limited and line-tables-only collapsed on this MSVC case

**Sources:** EXP-01 CodeView and archive inspection.

**Observed behavior:** the levels produced identical synthetic object sizes,
debug-section sizes, procedure, local, line, type, and checksum counts, and
nearly identical public archive evidence.

**Implication:** Do not assume every documented debug label maps to a distinct
backend result on every target. Record effective emitted evidence.

**Confidence:** High for this case; low for other targets.

### FERRIUM-377: debug-section bytes are not total debug-induced bytes

**Sources:** EXP-01 COFF section and total-object inspection.

**Observed behavior:** full debug added 445,324 named debug-section bytes but
increased the object by 614,758 bytes. The remaining delta appeared in
non-debug sections and object-container, relocation, and symbol overhead that
the experiment did not fully partition.

**Implication:** Record total object and archive bytes alongside named debug
sections. Do not equate `.debug$S` plus `.debug$T` with total emission cost.

**Confidence:** High.

### FERRIUM-378: the linker was not the dominant full-debug cost

**Sources:** EXP-01 complete-link time-pass control.

**Observed behavior:** `run_linker` changed by less than one millisecond while
the LLVM region increased by about 66 ms.

**Implication:** Do not classify every PDB-associated delay as linking.
PERF-Q29 must start after backend emission has been separated.

**Confidence:** High for the synthetic control.

### FERRIUM-379: total PDB bytes are not current-crate attribution

**Sources:** EXP-01 executable debug directory and PDB sizes.

**Observed behavior:** the no-crate-debuginfo link still produced a 1,372,160
byte PDB. Line tables and full added 188,416 and 450,560 bytes over that matched
baseline.

**Implication:** Attribute current changes through matched deltas and object
records, not one final PDB size.

**Confidence:** High for observation; medium for the exact baseline contents.

### FERRIUM-380: CGU parallelism traded wall time for debug bytes

**Sources:** EXP-01 one- and sixteen-CGU controls.

**Observed behavior:** sixteen full-debug CGUs shortened object wall time 15.5%
but increased object bytes 20.2% and debug-section bytes 33.1%.

**Implication:** Debug and CGU recommendations require joined latency,
resource, storage, linker-input, and runtime controls.

**Confidence:** High for the fixture.

### FERRIUM-381: full debug amplified public Cargo storage

**Sources:** EXP-01 METIS clean builds.

**Observed behavior:** full debug increased total target bytes 62.8%,
incremental bytes 94.2%, and the root Rlib 244.9%.

**Implication:** Build intelligence should expose debug cost in archives and
incremental roots, not only final executables.

**Confidence:** High for METIS; medium for prevalence.

### FERRIUM-382: stripping did not avoid debug generation or PDB output

**Sources:** rustc strip reference; EXP-01 strip controls.

**Observed behavior:** `strip=debuginfo` and `strip=symbols` produced the same
measured full-debug EXE and PDB bytes as the unstripped packed control.

**Implication:** A strip recommendation cannot stand in for measuring a lower
debug level. Generation, link retention, packaging, and deployment are
different decisions.

**Confidence:** High for this target and artifacts.

### FERRIUM-383: non-packed MSVC split modes are unstable and non-portable

**Sources:** rustc split-debuginfo reference; pinned MSVC target policy;
EXP-01 negative controls.

**Observed behavior:** stable use of `off` and `unpacked` failed. With nightly
unstable options, `off` retained the packed PDB outcome and `unpacked` added a
retained object without replacing the PDB.

**Implication:** FERRIUM must probe target support and observed artifacts
rather than recommend split-debug labels generically.

**Confidence:** High.

### FERRIUM-384: emitted records do not prove debugger usability

**Sources:** EXP-01 tool and CodeView controls.

**Observed behavior:** CodeView records and PDB identity were inspectable, but
no applicable interactive debugger or PDB stream tool was available.

**Implication:** Interactive source, local, type, optimized-frame, panic, and
mixed-language debugging remain mandatory consumer validation.

**Confidence:** High.

### FERRIUM-385: debug policy is part of build and validation identity

**Sources:** findings 371 through 384.

**Observed behavior:** debug level changed latency, CPU, memory, object layout,
archive bytes, incremental storage, PDB bytes, and available records.

**Implication:** Alternative debug roots are not interchangeable artifacts or
equivalent validation. Record profile origin, target format, debugger
requirements, and rollback.

**Confidence:** High.

### FERRIUM-386: the immediate opportunity is a debug-emission ledger

**Sources:** findings 371 through 385 and role review.

**Observed behavior:** rustc and Cargo expose the controls, but maintainers do
not receive one joined explanation of backend cost, artifact bytes, effective
records, target policy, and diagnostic capability.

**Implication:** Add a read-only ledger and comparison overlay. Contribute
minimized upstream cases when emission dominates; keep profile changes advisory
and human-approved.

**Confidence:** High for the evidence gap; medium for product adoption.

## Recommendations

### Adopt now

- Record effective debug level and origin, target debuginfo kind, split mode,
  CGUs, incremental policy, backend, linker, strip, save-temps, and exact
  toolchain.
- Separate object-only, archive, complete-link, PDB, executable, incremental,
  and debugger evidence.
- Inspect named debug sections and total artifact bytes.
- Use self-profile or time-pass diagnostics only as observer-affected causal
  evidence beside minimally instrumented primary measurements.
- Preserve full-debug validation for workflows that require local, type,
  panic, optimized-frame, or mixed-language debugging.
- Contribute minimized upstream fixtures where debug emission dominates.

### Prototype behind a compatibility boundary

- A read-only debug and native-emission ledger in the compiler query plan.
- Sibling Build Forest roots for full and line-table development policies with
  explicit debugger-capability labels.
- An exact-target support probe for debug level, split mode, object format,
  linker, PDB/DWARF/dSYM output, and inspection tools.
- A maintainer advisory that explains measured latency and storage tradeoffs
  without editing repository configuration.

### Reject or defer

- Universal `debug=0`, `debug=1`, or line-table guidance.
- Automatic `Cargo.toml`, environment, `.cargo/config.toml`, CI, editor, strip,
  split-debug, CGU, linker, or source changes.
- Treating PDB, object, archive, debug-section, or target-directory bytes as
  interchangeable size measures.
- Claiming debugger adequacy from CodeView or DWARF record presence.
- Reusing artifacts or validation claims across debug identities.
- A FERRIUM object writer, PDB implementation, linker, compiler fork, or debug
  format.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because reduced debug data is not treated as a safety proof and panic, unwind, unsafe, crash, and behavior evidence remain separate. |
| Compiler Performance Engineer | Accepted because object-only, complete-link, public Cargo, CGU, CPU, memory, storage, variance, and observer-affected diagnostic regions remain distinct. |
| Interop Boundary Auditor | Accepted provisionally because PDB, unwind, ABI, native symbols, mixed-language stepping, allocation, exceptions, and non-MSVC formats remain open consumer gates. |
| AI Assurance Skeptic | Accepted because exact commands, revisions, failed split modes, unavailable debugger tools, variance, and unmeasured usability remain visible. |
| Ecosystem Strategist | Accepted because rustc, Cargo, LLVM, Microsoft PDB tooling, and upstream fixtures remain the owners; FERRIUM does not duplicate a format or linker. |
| Rust Maintainer | Accepted because ordinary Cargo full debug remains available, alternative roots are removable, and the advisory explains tradeoffs without source or manifest churn. |
| Native Platform Adopter | Accepted provisionally because debugger installation, support, mixed native stacks, CI storage, rollout, audit, and rollback require platform-specific validation. |
| Scope Keeper | Accepted because PERF-Q28 owns debug construction and native emission while PERF-Q29 retains broader linker and incremental-link decisions. |
| Validation Checker | Accepted because primary matrices use five repetitions, identities, negative controls, object/PDB inspection, a public fixture, and explicit limitations. |

No role authorizes implementation or an automatic debug-profile default.

## Product implication

The compiler query plan should add:

```text
workflow and edit class
  -> Cargo freshness and effective profile
  -> rustc frontend, mono items, and CGUs
  -> debug construction
       -> level and origin
       -> source, line, procedure, local, and type capability
  -> LLVM and native object emission
       -> object count and section bytes
       -> CPU, wall, memory, and observer-affected regions
  -> archive and incremental storage
  -> linker input and split-debug packaging
  -> PDB, dSYM, DWP, object, or executable debug output
  -> interactive debugger and diagnostic validation
```

The Build Forest may retain sibling roots with different debug identities. It
must preserve their objects, archives, incremental generations, PDBs, debugger
capabilities, and validation dispositions separately.

## Prototype gate

No debug-profile advisor or automatic configuration implementation may begin
until:

1. at least three Tier 1 repositories reproduce clean and incremental
   measurements;
2. Linux DWARF, macOS dSYM, and Windows PDB evidence exists;
3. representative executable, library, test, and mixed native-link workflows
   are covered;
4. interactive source, local, type, optimized-frame, panic, and crash
   diagnostics are tested;
5. CI storage, transport, retention, and clean-versus-edit economics are
   measured;
6. runtime, size, ABI, unwind, reproducibility, and failure controls pass;
7. rollout, audit, opt-out, rollback, and support ownership are defined;
8. held-out maintainers find the explanation useful;
9. `.roles` approves a bounded implementation pulse.

Until then, the immediate output is research, upstream fixtures, and a
read-only model.

## Limitations and next research

- Repeat on Linux DWARF/split DWARF and macOS dSYM.
- Add an installed Windows debugger and PDB stream inspection.
- Measure incremental edit classes rather than clean target creation alone.
- Add executable and test graphs with native dependencies.
- Separate linker input reading, PDB merging, incremental linking, and final
  image production in PERF-Q29.
- Test macro-heavy, generic-heavy, generated-code, and very large workspaces.
- Preserve unsupported, unstable, and unknown target behavior.

## Final decision

Record debug information and native emission as first-class compiler query-plan
regions. Add a read-only ledger and Build Forest overlay. Treat line tables as
a measured compatibility candidate, not a default. Keep full debug available,
require consumer debugger validation, contribute minimized upstream emission
cases, and defer automatic profile or tooling changes.
