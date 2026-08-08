# PERF-Q20: Relink-Don't-Rebuild and Cross-Crate Interfaces

**Status:** Planned

**Area:** Cross-crate reuse

**Depends on:** PERF-Q02, PERF-Q17

## Question

Which upstream edits can safely reuse downstream compilation artifacts and
require only relinking?

## Starting hypothesis

Private non-generic body edits are the strongest initial class, while inline
MIR, generics, constants, macros, layouts, and optimization facts constrain
eligibility.

## Investigation focus

- Build a body/API/generic/inline/constant/layout/macro edit matrix.
- Test false reuse and unnecessary rebuild cases.
- Track the official RDR design and contribute regression fixtures.

**Model changes if:** the effective cross-crate interface is too broad or
unstable for useful eligibility.

## Decision informed

Define FERRIUM's RDR fixture and upstream contribution program.

## Primary roles

Rust Safety Steward, Compiler Performance Engineer, Rust Maintainer.
