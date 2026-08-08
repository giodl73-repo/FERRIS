# PERF-Q09: Parsing and Tokenization

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.
