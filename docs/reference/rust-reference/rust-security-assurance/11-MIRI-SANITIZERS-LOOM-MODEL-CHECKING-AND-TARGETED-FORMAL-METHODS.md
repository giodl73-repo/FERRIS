---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:miri-sanitizers-loom-model-checking-and-targeted-formal-methods
kind: guide
module: rust-security-assurance
section: security-engineering
title: Miri, Sanitizers, Loom, Model Checking, and Targeted Formal Methods
status: source-custody
source_custody: partial
current_path: rust-security-assurance/11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md
canonical_path: rust-security-assurance/11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md
backsource_ids: [mdloom-backfill:rust-security-assurance:11-miri-sanitizers-loom-model-checking-and-targeted-formal-methods]
concepts: [Miri, sanitizers, Loom, model checking, formal verification, unsafe rust]
root_concepts: [program analysis]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Miri, Sanitizers, Loom, Model Checking, and Targeted Formal Methods

No single analysis closes Rust assurance. Miri checks executed Rust paths
against an interpreter and current undefined-behavior models. Sanitizers
instrument native execution. Loom explores bounded concurrency schedules for
code written against its model types. Formal tools prove or refute specific
properties under explicit models. Choose the tool by claim.

## The Big Picture

```
+============================================================================+
|                         TARGETED EVIDENCE LADDER                           |
+============================================================================+
| source/property specification                                              |
|      |                                                                     |
|      +--> Miri: Rust UB on interpreted paths                               |
|      +--> ASan/MSan/TSan/etc.: native instrumented paths                   |
|      +--> Loom: bounded thread/atomic schedules                            |
|      +--> model checker: bounded states/traces                             |
|      +--> deductive/bounded verifier: stated assertions under model        |
|                                                                            |
| result + scope + assumptions + counterexample/proof obligation -> case     |
+============================================================================+
```

Green results are evidence for the exercised model and configuration. Red
results are valuable counterexamples. Neither should be generalized silently.

## Evidence Matrix

| Tool class | Strong signal for | Important blind spots |
|------------|-------------------|-----------------------|
| Miri | many Rust UB classes, aliasing/model violations, invalid accesses on run paths | unsupported FFI/syscalls, unexecuted paths, model evolution, performance |
| AddressSanitizer | native out-of-bounds/use-after-free on run paths | logical bugs, many uninitialized reads, unexecuted paths |
| MemorySanitizer | uninitialized-memory use on supported native stack | requires instrumented dependencies; target limits |
| ThreadSanitizer | native data races on run schedules | Rust-model aliasing, logical races, schedule coverage |
| Loom | bounded interleavings and memory-order cases using Loom primitives | production-only code not abstracted, state explosion, model mismatch |
| Bounded model checker | counterexamples/proofs within bounds/model | properties outside harness, unsupported features, bound completeness |
| Deductive verification | mathematical argument for specified contracts | specification correctness, trusted solver/toolchain/axioms |

## Miri: Dynamic Rust Semantics

Pin a nightly because Miri is distributed with nightly toolchains. From a crate
or workspace root:

```text
rustup +nightly component add miri
cargo +nightly miri setup
cargo +nightly miri test
```

For durable CI, replace moving `nightly` with an approved dated nightly and
record `rustc +<toolchain> -vV`. Run the smallest crate/tests that exercise the
unsafe boundary; unsupported external interactions may need a model-friendly
test seam rather than blanket skipping.

Miri executes the schedules reached by the interpreted program; it is not a
systematic scheduler like Loom. A Miri-clean concurrent test therefore does not
establish that all interleavings or weak-memory behaviors were explored.

| Miri finding | Typical question |
|--------------|------------------|
| invalid alignment/dereference | Was pointer arithmetic/layout justified? |
| use after free | Who owned the allocation and callback/handle lifetime? |
| invalid value | Was `transmute`/`assume_init` valid for the target type? |
| aliasing violation | Did a raw-pointer/reference conversion overpromise exclusivity? |
| data race | Was `Send`/`Sync` or shared mutation implemented incorrectly? |

## Sanitizers: Native Execution

Sanitizer support is nightly, target-dependent, and documented by rustc. A
representative **Linux x86-64** AddressSanitizer command is:

```text
rustup component add rust-src --toolchain nightly
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

That shell syntax is POSIX, not PowerShell. Pin the nightly and consult the
current rustc sanitizer documentation before copying to another target. Native
dependencies may also need matching instrumentation; otherwise coverage is
partial.

```
Miri: Rust MIR interpreter --------> Rust-specific semantic checks
Sanitizer: compiled machine code ---> allocator/native/FFI execution checks
                         overlap, neither contains the other
```

## Loom: Bounded Concurrency Exploration

Loom replaces synchronization primitives with model versions and repeatedly
runs a test under possible schedules/memory reorderings within configured
bounds.

```rust
#[cfg(loom)]
use loom::sync::{Arc, Mutex};

// Production builds should select std equivalents behind the same small
// abstraction; model tests must exercise the same algorithm, not a rewrite.
```

The crucial design task is sharing one algorithm between production and model
configurations. If the Loom test simplifies away the risky behavior, schedule
coverage is irrelevant.

| State-explosion control | Risk |
|-------------------------|------|
| Fewer threads/operations | may omit required production scenario |
| Preemption bound | may miss bug requiring more switches |
| Abstract data payload | sound if payload is irrelevant; dangerous if not |
| Smaller capacities | often exposes boundaries, but verify generalization |

## Model Checking and Formal Methods

Use formalization where consequence or concurrency complexity justifies its
cost:

```
product claim
   |
   +--> protocol/state property ----> TLA+/PlusCal or similar model
   +--> unsafe function contract ---> bounded verifier / deductive tool
   +--> arithmetic/memory invariant -> proof-oriented Rust tool
   +--> crypto construction --------> specialized protocol/crypto analysis
```

Kani, Verus, Creusot, Prusti, and related projects have different supported
Rust subsets and assurance models. Their maturity and compatibility evolve.
Pin the exact tool, document unsupported constructs and assumptions, and keep a
conventional test/reference implementation where practical. A proof of the
wrong specification is a precise failure.

## Build an Evidence Package

For every advanced-analysis result, retain:

- claim/property and code revision;
- tool/version/toolchain/target/features;
- harness/model and bounds;
- command/configuration;
- result and machine-readable logs where available;
- excluded code, stubs, assumptions, and unsupported operations;
- owner and rerun trigger.

## Old World -> New World Bridge

| Established tool/practice | Rust assurance role |
|---------------------------|---------------------|
| Valgrind / native ASan | Miri plus sanitizer matrix, with different semantics |
| CHESS-style schedule exploration | Loom model tests |
| TLA+ service protocol | same abstract model, connected to Rust state-machine tests |
| Code Contracts/formal annotations | verifier-specific Rust contracts and harnesses |
| Static analyzer "clean" report | dated scoped evidence, never a universal proof |

Microsoft and other large engineering organizations have used TLA+-style
protocol review and bounded/concurrency analysis within similar evidence
ladders. Tool pedigree is less important than explicit semantics, bounds, and
reproducibility.

## Common Confusion Points

- **"Miri proves unsafe soundness."** It finds violations on executed paths
  under its model.
- **"Sanitizers are redundant in Rust."** They add native/FFI/allocator evidence
  and different runtime observations.
- **"Loom tests production scheduling automatically."** Only code expressed
  through the modeled primitives and explored bounds is covered.
- **"Formal verification proves the product secure."** It proves stated
  properties under a model and trusted assumptions.
- **"A tool that cannot model FFI may ignore it safely."** The exclusion is a
  case assumption and often the highest-risk boundary.
- **"Nightly tool output is stable."** Pin the exact nightly/tool versions.

## Decision Cheat Sheet

| Claim/risk | Use |
|------------|-----|
| Unsafe Rust validity/aliasing | Miri plus invariant review |
| Native/FFI memory defect | ASan/MSan as target support permits |
| Native data race | TSan plus schedule-focused tests |
| Custom atomic/lock-free algorithm | Loom and written memory-order argument |
| Protocol safety/liveness | abstract model checker plus implementation tests |
| Narrow critical function invariant | bounded/deductive Rust verifier after capability review |
| Tool unsupported path | Record exclusion; add another evidence source or reduce claim |

## Primary Sources

- Miri: https://github.com/rust-lang/miri
- rustc Sanitizer documentation:
  https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
- Loom: https://github.com/tokio-rs/loom
- Kani Rust Verifier: https://model-checking.github.io/kani/
- Verus: https://verus-lang.github.io/verus/
- Creusot: https://creusot-rs.github.io/creusot/
- Prusti: https://www.pm.inf.ethz.ch/research/prusti.html

## Related Guides

- Previous: [10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md](10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md)
- Next: [12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md](12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md)
- Memory-model obligations: [03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md](03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md)
