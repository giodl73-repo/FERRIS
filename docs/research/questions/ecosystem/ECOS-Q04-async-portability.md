# ECOS-Q04: Async Portability

**Status:** Complete

**Area:** Async runtime and I/O

**Depends on:** ECOS-Q02, ECOS-Q03

## Question

Which runtime, I/O, cancellation, timing, synchronization, and task-local
assumptions leak through library boundaries?

## Starting hypothesis

`Future` is portable at the language boundary, while practical I/O,
cancellation, spawning, timers, and task context remain runtime-sensitive.

## Decision informed

Which async contracts need adapters, compatibility profiles, or upstream
coordination.

## Primary roles

Rust Safety Steward, Interop Boundary Auditor, Ecosystem Strategist.

## Decision

Adopt an operation-level runtime contract covering Future, executor, spawn,
I/O, time, cancellation, blocking work, synchronization, context, and
platform.

Measured fixtures show that pure futures can run under multiple executors,
while Tokio spawn and timer operations panic without Tokio context. Tokio and
futures I/O traits require an explicit adapter, JoinHandle drop detaches rather
than cancels, and runtime coupling cannot be inferred from the package name
alone.

See [Rust async portability](../../2026-08-09-rust-async-portability.md).
