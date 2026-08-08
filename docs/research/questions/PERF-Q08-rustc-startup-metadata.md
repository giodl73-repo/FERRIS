# PERF-Q08: rustc Startup and Metadata Loading

**Status:** Planned

**Area:** rustc startup

**Depends on:** PERF-Q01

## Question

How much compiler-invocation latency comes from process startup, crate metadata,
dependency loading, option processing, and initialization before useful queries?

## Starting hypothesis

Startup is material for many small crates and repeated short invocations but is
secondary for large semantic or backend-heavy crates.

## Investigation focus

- Compare tiny-crate and large-crate invocation profiles.
- Attribute metadata decoding and initialization costs.
- Investigate batching, daemon, and selective-loading proposals upstream.

**Model changes if:** startup is negligible across representative short builds.

## Decision informed

Whether startup deserves fixtures, upstream profiling improvements, or daemon
research.

## Primary roles

Compiler Performance Engineer, Ecosystem Strategist, Validation Checker.
