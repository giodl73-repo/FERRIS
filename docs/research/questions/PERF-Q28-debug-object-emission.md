# PERF-Q28: Debug Information and Object Emission

**Status:** Complete

**Area:** Backend and emission

**Depends on:** PERF-Q01, PERF-Q25

## Question

How much latency, memory, disk use, and linker input comes from debug information
and native object emission?

## Starting hypothesis

Large applications and generic-heavy crates can spend substantial time and I/O
emitting debug records even after semantic work is complete.

## Investigation focus

- Compare debuginfo levels and split-debuginfo options where supported.
- Measure object size, emission time, memory, and debugger usability.
- Separate emission from LLVM optimization and linking.

**Model changes if:** debug information is minor outside release or specialized
workloads.

## Result

Debug information was material in both the shaped synthetic fixture and public
METIS-CORE control. Full debug increased synthetic object-only wall time 32.8%
and more than doubled the self-profiled LLVM object-emission event. In METIS it
increased clean-build wall time 21.9%, target storage 62.8%, incremental bytes
94.2%, and the root Rlib 244.9%.

Line tables preserved source-line records at lower measured cost, but omitted
local-variable records. Interactive debugger adequacy was not measurable on
the host and remains a consumer gate. MSVC packed PDB, unstable split modes,
stripping, CGU, object, archive, incremental, and linker regions were kept
separate.

Decision:
[Debug information and object emission](../2026-08-09-debug-information-object-emission.md).

Experiment:
[EXP-01 debug emission matrix](../perf-q28-debug-object-emission/results/EXP-01-debug-emission-matrix.md).

## Decision informed

Define development-profile guidance and upstream emission cases.

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Rust Maintainer.
