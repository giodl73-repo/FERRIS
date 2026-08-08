# PERF-Q09: Parsing and Tokenization

**Status:** Complete

**Area:** Frontend

**Depends on:** PERF-Q01, PERF-Q08

## Question

When do lexing and parsing dominate frontend latency, and can unchanged source
regions be reused or processed more efficiently?

## Starting hypothesis

Parsing is usually modest but becomes material in very large generated files,
macro-heavy crates, and repeated whole-crate frontend work.

## Investigation focus

- Select large-source and generated-source fixtures.
- Separate lexer, parser, source-map, and diagnostic costs.
- Review incremental parsing and parallel parsing constraints.

**Model changes if:** expansion or resolution consistently dwarfs parsing in the
same fixtures.

## Decision informed

Whether to contribute parser fixtures or defer to broader frontend work.

## Decision

Adopt source-shape, root-versus-outline-module, incremental-reparse, frontend
thread, and parser-failure vocabulary. Prototype a portable parametric fixture
and read-only parse-topology report behind nightly compatibility boundaries.

Consider rustc-perf fixture coverage and finer source-load, lexer, token-tree,
root-parse, and outline-module-parse timers only after explicit owner approval.
Do not build a parser replacement, rust-analyzer tree bridge, persistent
parser, automatic module splitter, or parallel module loader from this
evidence.

## Results

- [Synthesis](../2026-08-08-parsing-tokenization.md)
- [EXP-01: source shape, reparse, and module boundaries](../perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

Findings: `FERRIUM-108` through `FERRIUM-116`.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.
