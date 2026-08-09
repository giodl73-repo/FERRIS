# PERF-Q22: Procedural-Macro Cost, Inputs, and Reuse

**Status:** Complete

**Area:** Compile-time execution

**Depends on:** PERF-Q01, PERF-Q10

## Question

How much time and invalidation comes from procedural macros, and what deterministic
input/output model would permit safer reuse?

## Starting hypothesis

A small number of derive and attribute macros can dominate frontend time, but
arbitrary execution and hidden environment inputs block general caching.

## Investigation focus

- Attribute execution time, invocation count, input size, and output size.
- Test repeated identical invocations and changed token inputs.
- Study sandboxing, deterministic contracts, server reuse, and cache identity.

**Model changes if:** macro-associated cost is mostly downstream type checking
of generated code rather than macro execution.

## Decision informed

Define observability now and the minimum contract for later caching research.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, AI Assurance Skeptic.

## Decision

Adopt read-only procedural-macro invocation, input, output, generated-shape,
rerun-cause, and declared-input observability. Preserve tracked and untracked
input controls. Reject enabling the current experimental derive cache because
it reused stale output across tracked-input changes. Defer general caching,
sandbox enforcement, macro consolidation, source rewrites, compiler forks, and
implementation.

## Results

- [Procedural-macro cost, inputs, and reuse](../2026-08-08-procedural-macro-cost-input-reuse.md)
- [EXP-01: procedural-macro cost, input, and reuse](../perf-q22-procedural-macros/results/EXP-01-proc-macro-cost-input-reuse.md)
