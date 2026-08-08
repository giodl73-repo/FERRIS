# PERF-Q11: Name Resolution and HIR Lowering

**Status:** Complete

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

## Decision

Adopt reduced-graph, import-fixed-point, propagated-binding, effective-
visibility, late-path, AST-owner, HIR-owner, local-node, stable-hash,
incremental-edit, frontend-job, and failure vocabulary.

Prototype a read-only namespace and HIR topology report plus parametric glob,
path, owner, and edit fixtures behind replaceable nightly adapters.

Consider rustc-perf fixtures, structured import/visibility statistics, and
source-attributed resolution or lowering events only after explicit owner
approval. Do not automatically rewrite imports, visibility, modules, or
macros; persist resolver or HIR state; parallelize rustc; or create upstream
activity.

## Results

- [Synthesis](../2026-08-08-name-resolution-hir-lowering.md)
- [EXP-01: namespace topology and owner invalidation](../perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md)

Findings: `FERRIUM-127` through `FERRIUM-136`.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Ecosystem Strategist.
