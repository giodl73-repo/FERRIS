# PERF-Q07: rust-analyzer, Cargo, and Concurrent Build Contention

**Status:** Complete

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

## Decision

FERRIUM should model the entire edit-to-diagnostic and foreground-command loop
as a read-only topology diagnostic.

The diagnostic must separate rust-analyzer semantic analysis, Cargo metadata,
build-script and proc-macro build data, flycheck, developer Cargo commands,
lock class, producer and waiter behavior, target-directory topology,
cancellation, diagnostic readiness, total successful work, and foreground
latency.

There is no universal target-directory recommendation. Shared targets minimize
successful duplicate work and disk but can delay incompatible foreground
commands. Isolated editor targets permit overlap but duplicate compiler work,
artifacts, and resource demand. Disabling build scripts or proc macros is not
an accepted optimization because it changes diagnostic correctness.

The implementation gate remains closed. Cargo locking changes, rust-analyzer
scheduling changes, automatic cancellation, and upstream activity remain with
their owners unless a later approved reproduction opens a contribution path.

## Results

- [Research synthesis](../2026-08-08-editor-cargo-contention.md)
- [Editor and Cargo overlap experiment](../perf-q07-ide-cargo-contention/results/EXP-01-editor-cargo-loop.md)

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Native Platform Adopter.
