# PERF-Q23: Build-Script Inputs, Reruns, and Outputs

**Status:** Planned

**Area:** Compile-time execution

**Depends on:** PERF-Q01, PERF-Q02

## Question

Which build scripts rerun unnecessarily, hide inputs, or prevent artifact reuse,
and how can their contracts become more precise?

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

## Primary roles

Interop Boundary Auditor, Compiler Performance Engineer, Rust Safety Steward.
