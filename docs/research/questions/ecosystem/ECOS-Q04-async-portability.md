# ECOS-Q04: Async Portability

**Status:** Planned

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
