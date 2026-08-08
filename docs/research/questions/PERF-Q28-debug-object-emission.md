# PERF-Q28: Debug Information and Object Emission

**Status:** Planned

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

## Decision informed

Define development-profile guidance and upstream emission cases.

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Rust Maintainer.
