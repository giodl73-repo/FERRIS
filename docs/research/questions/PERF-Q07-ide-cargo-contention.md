# PERF-Q07: rust-analyzer, Cargo, and Concurrent Build Contention

**Status:** Planned

**Area:** IDE and validation loop

**Depends on:** PERF-Q01, PERF-Q02

## Question

Where do rust-analyzer, editor checks, Cargo commands, target directories, and
build locks duplicate work or block one another?

## Starting hypothesis

Separate caches and concurrent commands can repeat analysis, contend on build
locks, and delay feedback even when each tool behaves correctly in isolation.

## Investigation focus

- Trace editor and terminal activity during representative edit loops.
- Separate CPU, lock, filesystem, and artifact duplication.
- Evaluate supported shared-target and scheduling configurations.

**Model changes if:** contention is rare or dominated by project-specific editor
extensions.

## Decision informed

Whether FERRIUM should model the entire edit-to-diagnostic loop.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Native Platform Adopter.
