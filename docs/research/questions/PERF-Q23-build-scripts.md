# PERF-Q23: Build-Script Inputs, Reruns, and Outputs

**Status:** Complete

**Area:** Compile-time execution

**Depends on:** PERF-Q01, PERF-Q02

## Question

Which build scripts rerun unexpectedly because of broad or undeclared file and
environment inputs, and what minimal observable contract explains their
reruns, outputs, and ownership?

## Starting hypothesis

Broad default change detection and incomplete `rerun-if-*` declarations cause
avoidable execution and downstream invalidation.

## Investigation focus

- Inventory declared filesystem and environment inputs.
- Test relevant and irrelevant changes, output stability, and downstream fan-out.
- Study deterministic manifests and isolated execution boundaries.

**Model changes if:** most reruns are required by native toolchains or generated
artifacts whose inputs cannot be declared practically.

## Decision informed

Define build-script diagnostics and any safe contract prototype.

## Decision

Adopt the measurement contract's read-only build-script vocabulary for
invocation, declared inputs, rerun cause, output changes, and output ownership.
Use Cargo nightly build analysis only as optional supporting evidence. Defer
caching, output suppression, cleanup automation, sandbox enforcement, source
rewrites, Cargo replacement, and implementation. This diagnostic-only decision
changes no Cargo behavior; rollback is disabling the diagnostic.

## Evidence

- [Build-script input, output, and rerun precision](../2026-08-09-build-script-input-output-precision.md)
- [EXP-01 build-script input, output, and fan-out matrix](../perf-q23-build-scripts/results/EXP-01-build-script-input-output-matrix.md)

## Primary roles

Interop Boundary Auditor, Compiler Performance Engineer, Rust Safety Steward.
