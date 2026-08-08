# PERF-Q11: Name Resolution and HIR Lowering

**Status:** Planned

**Area:** Frontend

**Depends on:** PERF-Q09, PERF-Q10

## Question

Which parts of name resolution and HIR lowering remain crate-wide, serial, or
insufficiently incremental?

## Starting hypothesis

Global namespace and expansion dependencies cause small edits to repeat broader
work than later body-oriented queries.

## Investigation focus

- Compare body-only, import, module, macro, and visibility edits.
- Measure resolution and lowering categories with optional self-profile data.
- Review parallel and incremental upstream designs and correctness constraints.

**Model changes if:** measured invalidation remains narrow for representative
module graphs.

## Decision informed

Which fixtures and upstream goals FERRIUM should support.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Ecosystem Strategist.
