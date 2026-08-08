# Declarative Macro Expansion

Date: 2026-08-08
Question: PERF-Q10
Status: Complete
Decision: add macro matcher, transcription, expansion ratio, recursion,
invalidation, and diagnostics evidence to FERRIUM; prioritize fixtures and
telemetry over macro rewriting or expansion caches.

## Executive conclusion

Declarative macro cost is not one “macro count” number.

The controlled evidence separated five dimensions:

1. input matching;
2. matcher search shape;
3. cumulative transcription;
4. generated output integrated into the AST;
5. later work invalidated by that output.

Matching 10,000 identifiers and emitting nothing remained near the tiny stable
control: 94.14 versus 92.57 ms. Emitting 10,000 constants increased stable
metadata compilation to about 341–346 ms. Splitting those items among 1, 100,
or 1,000 invocations changed little in the simple fixture. Generated output
and later item processing dominated invocation setup.

Recursive TT munching produced a different result. A direct repetition over
1,500 identifiers had a 75.00 ms no-analysis median. A recursive one-token
muncher took 178.54 ms. Although the final expansion emitted no items,
`macro-stats` recorded 1,501 expansions and 14.1 MB of cumulative intermediate
output because every recursion transcribed the remaining tail.

Matcher arm shape also mattered more than arm count alone. One thousand
invocations against 1,000 distinct-prefix arms had a 96.63 ms no-analysis
median. Giving those arms a shared ten-token prefix increased the median to
295.07 ms with similar final output. The matcher had to progress further
before rejecting each unsuccessful arm.

The rustc source explains these results. Macro-by-example uses an NFA-like
matcher, calls the Rust parser for named nonterminals, transcribes the selected
right-hand side, applies hygiene, reparses output into the requested AST
fragment, and integrates it into an iterative crate-level expansion queue.
The matcher source explicitly acknowledges worse pathological complexity than
traditional NFA or Earley parsing because named matches are constructed
eagerly.

Macro expansion is not persisted in rustc's incremental query cache. Once the
compiler runs, expansion repeats. In the 3,000-invocation control,
`expand_crate` remained about 28–31 ms for fresh, unchanged, rewritten,
invocation-edit, and definition-edit sessions. The definition edit was still
the slowest complete compile because it changed all generated values and
broadened later work.

Eight frontend jobs did not accelerate either recursive or high-invocation
expansion. Current project goals still describe parallel macro expansion as
future work.

FERRIUM should provide a read-only macro-cost diagnostic that joins stable
wall time, `macro-stats`, expansion profiles, matcher shape, generated output,
and edit fanout. Credible upstream paths are orthogonal rustc-perf fixtures,
structured macro statistics, and finer matcher/transcriber timers. Automatic
macro rewrites, persistent expansion caches, parallel expansion, and parser or
daemon replacement remain deferred.

No upstream activity was created.

## Decision supported

This research determines:

- which declarative macro shapes create disproportionate matching or
  transcription;
- whether input, invocations, arms, output, or recursion predict cost;
- what existing expansion diagnostics reveal and omit;
- whether incremental or parallel compiler settings reuse expansion;
- which external diagnostics and upstream fixtures are defensible.

It does not authorize source rewrites, macro API changes, compiler changes,
expansion caches, parallel expansion, recursion-limit changes, procedural
macro intervention, or upstream filing.

## Evidence reviewed

### Local evidence

- [Parsing and tokenization](2026-08-08-parsing-tokenization.md)
- [rustc startup and metadata loading](2026-08-08-rustc-startup-metadata.md)
- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler and language sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [`rustc_interface::passes`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs)
- [macro expansion loop](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/expand.rs)
- [macro-by-example rules](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_rules.rs)
- [NFA matcher](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_parser.rs)
- [transcriber](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/transcribe.rs)
- [macro statistics](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/stats.rs)
- [rustc-dev-guide macro expansion](https://github.com/rust-lang/rustc-dev-guide/blob/0e48eac6e3fb4b92ad46495325d6237a7b3ed989/src/macro-expansion.md)
- [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
- [`recursion_limit`](https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute)

### Performance direction

- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [parallel compiler tracking issue](https://github.com/rust-lang/rust/issues/113349)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [`tt-muncher` fixture](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/tt-muncher)

## Expansion model

```text
parsed macro definition and invocation token trees
  -> resolve macro name and choose candidate definition
  -> try matcher arms in declaration order
  -> NFA-like token matching and named nonterminal parsing
  -> construct named matches
  -> transcribe selected right-hand-side token trees
  -> apply expansion IDs and hygiene contexts
  -> parse output into the required AST fragment
  -> assign node and definition identities
  -> integrate output and collect newly introduced invocations
  -> repeat until the crate has no unresolved invocations
  -> continue validation, resolution, lowering, queries, and metadata
```

`macro_expand_crate` and `expand_crate` are timing regions, not persistent
incremental queries.

## Findings

### FERRIUM-117: declarative expansion is iterative AST construction, not text
substitution

**Sources**

- [rustc-dev-guide macro expansion](https://github.com/rust-lang/rustc-dev-guide/blob/0e48eac6e3fb4b92ad46495325d6237a7b3ed989/src/macro-expansion.md)
- [macro expansion loop](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/expand.rs)
- [macro-by-example rules](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_rules.rs)

**Observed constraint**

rustc repeatedly resolves invocations, matches token trees, transcribes
output, applies hygiene, parses AST fragments, assigns identities, integrates
definitions, and collects new invocations.

Macro name and import resolution are interleaved with this loop.

**Implication**

A macro-cost report needs matcher, transcriber, output, hygiene, integration,
resolution, and later generated-item work. Invocation count or expanded text
alone is incomplete.

**Confidence:** high.

### FERRIUM-118: generated output can dominate large input matching

**Sources**

- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)
- [transcriber](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/transcribe.rs)

**Observed behavior**

Matching 10,000 identifiers and emitting nothing measured 94.14 ms stable,
near the 92.57 ms tiny control. Emitting 10,000 constants from the same
identifier shape measured 346.32 ms.

The no-analysis medians were 79.07 and 117.63 ms. `macro-stats` reported zero
output bytes for capture and 298,889 cumulative bytes for emission.

**Implication**

FERRIUM must distinguish input tokens from cumulative expansion output, final
AST items, and later item cost. Large invocation input is not sufficient
evidence of an expensive macro.

**Confidence:** high for the fixture.

### FERRIUM-119: simple output volume outweighed invocation count

**Source**

- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

Emitting the same 10,000 simple constants through 1, 100, and 1,000
invocations produced stable medians of 346.32, 341.13, and 341.12 ms.
No-analysis medians were 117.63, 120.30, and 121.49 ms.

**Implication**

Invocation count is a required topology dimension, but it is not a standalone
cost estimate. Output shape, matcher work, expansion depth, hygiene contexts,
and downstream item work can dominate.

**Confidence:** high for simple local item emission; low for complex hygiene,
resolution, or nested expansion.

### FERRIUM-120: recursive TT munching amplifies cumulative transcription

**Sources**

- [NFA matcher](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_parser.rs)
- [rustc-perf `tt-muncher`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/tt-muncher)
- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

Direct repetition remained near 74–76 ms through no-analysis from 250 to
1,500 identifiers.

The recursive muncher increased from 77.74 to 178.54 ms. Cumulative output
increased from 339 KB to 14.1 MB because each invocation transcribed its
remaining tail.

The matcher source warns that pathological complexity can be worse than
traditional NFA or Earley parsing because named matches are constructed
eagerly.

**Implication**

FERRIUM should detect recursive tail forwarding, expansion depth, cumulative
output, and recursion limit. Raising `recursion_limit` can permit more work; it
is not a performance fix.

**Confidence:** high for this classic TT-muncher shape; medium for describing
other recursive macros.

### FERRIUM-121: matcher prefix overlap matters more than arm count alone

**Sources**

- [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
- [NFA matcher](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_parser.rs)
- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

One thousand invocations against 1,000 distinct-prefix arms measured 96.63 ms
through no-analysis. The same invocation and output count with a shared
ten-token prefix measured 295.07 ms.

Stable metadata medians were 169.07 and 364.83 ms. Root parsing differed by
only about 4 ms and macro output remained approximately 30 KB.

**Implication**

Arm count reports also need candidate ordering, prefix overlap, token depth,
fragment kinds, and success position. Mechanical arm reordering can change
diagnostics or accepted syntax and is not automatically safe.

**Confidence:** high for literal-prefix matching.

### FERRIUM-122: named nonterminal cost needs representative fragments

**Source**

- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

One thousand invocations matching `1 + 2 * 3` as token trees or as
`$value:expr` measured 75.46 and 76.36 ms through no-analysis. Stable medians
were 81.87 and 82.35 ms.

**Implication**

Calling the Rust parser for a named nonterminal is a real architecture
boundary, but this simple expression showed no material penalty. FERRIUM
should not generalize fragment cost without complex expression, type, pattern,
path, and item fixtures.

**Confidence:** high for the negative control.

### FERRIUM-123: declarative macro expansion remains serial

**Sources**

- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

Eight frontend jobs measured 188.98 ms versus 179.63 ms for the 1,500-token
muncher and 127.17 ms versus 119.00 ms for 1,000 emit invocations.

The project goal lists parallel macro expansion as future work while
prioritizing correctness, testing, Cargo support, and reduced contention.

**Implication**

Frontend job count is not a declarative macro optimization today. Parallel
expansion requires upstream ownership of resolution ordering, hygiene,
diagnostics, definition identity, and incremental correctness.

**Confidence:** high for current source direction and controls.

### FERRIUM-124: expansion repeats; edit breadth appears after expansion

**Sources**

- [`rustc_interface::passes`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs)
- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

With a real incremental directory, `expand_crate` remained about 28–31 ms for
fresh, untouched, identical-rewrite, invocation-edit, and definition-edit
sessions.

Full wall medians were 257.81 ms fresh, 256.33 ms untouched, 337.40 ms after
one invocation changed, and 389.71 ms after the macro definition changed all
3,000 outputs.

**Implication**

Cargo freshness can skip rustc, but rustc does not persist the expanded AST
once invoked. Macro definition fanout and invocation edits need later
invalidation evidence in addition to expansion time.

**Confidence:** high that expansion repeats; medium on the exact later-work
composition.

### FERRIUM-125: current diagnostics expose volume but not causal subphases

**Sources**

- [macro statistics](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/stats.rs)
- [Experiment](perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

**Observed behavior**

`-Z macro-stats` reports macro name, uses, lines, cumulative output bytes, and
averages. Self-profile exposes `expand_crate` inside
`macro_expand_crate`.

The 1,500-token muncher had 117.02 ms of `expand_crate` self time versus
3.25 ms for direct repetition.

Neither surface separates arm search, named nonterminal parsing,
transcription, output reparsing, hygiene, resolution, collection, and AST
integration. `macro-stats` is human-readable and adds observer effect.

Expected no-match failures also completed near the process floor and cannot be
treated as successful throughput.

**Implication**

FERRIUM should join, not conflate, stable wall time, macro statistics,
self-profile, failures, and expanded-output topology. A structured,
versioned statistics surface and finer events are plausible upstream work.

**Confidence:** high.

### FERRIUM-126: rustc-perf covers TT munching but not the full orthogonal
matrix

**Sources**

- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- Findings FERRIUM-117 through FERRIUM-125

**Observed constraint**

rustc-perf includes:

- `tt-muncher`, a quadratic declarative macro stress test;
- `html5ever` and `serde_derive`, which exercise macro parsing or expansion in
  real crates;
- `deep-vector`, which stresses expansion and inference;
- `token-stream-stress`, which is specifically a procedural macro token-stream
  construction benchmark and belongs to PERF-Q22.

The documented inventory does not independently vary input tokens, output
items, invocation count, distinct versus overlapping arms, fragment kind,
recursion depth, definition fanout, and diagnostics.

**Implication**

FERRIUM can contribute a parametric fixture and causal report without
duplicating rustc-perf. Upstream work should begin with benchmark or telemetry
evidence after owner approval.

**Confidence:** high for the documented coverage gap; medium for upstream
acceptance.

## Recommendations

### Adopt now

- Record macro origin, definition, export status, invocation count, nesting
  depth, recursion limit, matcher arms, prefix overlap, fragment kinds, input
  tokens, cumulative output bytes, final items, and edit fanout.
- Keep declarative, built-in, attribute, derive, and procedural macro work
  separate.
- Use stable repeated wall time as primary evidence.
- Use `macro-stats`, self-profile, time-passes, expanded output, and failure
  diagnostics as separate nightly evidence.
- Treat generated output and downstream item work separately from matching.
- Preserve expected failures, recursion-limit errors, local ambiguity, and
  recovery behavior.

### Prototype behind a compatibility boundary

- A read-only declarative macro census joined to Cargo units and source edits.
- A parametric fixture for matching, transcription, arms, recursion, fragments,
  output, invocation count, and edit fanout.
- A versioned adapter for `macro-stats` plus self-profile expansion regions.
- Finer compiler events for arm matching, nonterminal parsing, transcription,
  output reparsing, hygiene, collection, and integration.
- A rustc-perf fixture or structured macro-statistics proposal after explicit
  owner approval.

The implementation gate remains closed.

### Reject or defer

- Reject macro invocation count, input tokens, output bytes, or arm count as
  standalone cost estimates.
- Reject raising `recursion_limit` as a speed improvement.
- Reject automatic TT-muncher, arm-order, fragment, or output rewrites.
- Reject checking expanded source into repositories as a general remedy.
- Reject caching expansion by macro name and token input; scope, imports,
  editions, cfg, hygiene, diagnostics, and definition identity are required.
- Defer parallel macro expansion to upstream compiler work.
- Defer persistent expansion caches, parser replacement, and compiler daemons.
- Defer procedural macro execution and caching to PERF-Q22.
- Defer downstream query invalidation precision to PERF-Q17 and PERF-Q20.
- Defer upstream activity until explicit owner approval.

## Potential contribution paths

Without creating upstream activity, Q10 identifies:

1. a rustc-perf matrix for input, output, invocation, arm-prefix, recursion,
   fragment, and edit-fanout shapes;
2. structured `macro-stats` output with explicit schema and cumulative-output
   semantics;
3. matcher, named-nonterminal, transcriber, output-parser, hygiene,
   invocation-collector, and integration events;
4. minimized pathological NFA or recursive-transcription regressions;
5. incremental patches that distinguish macro definition edits from one
   invocation edit;
6. parallel-frontend controls for expansion ordering and diagnostics.

Each path needs current rustc-perf reproduction, Linux and macOS coverage where
relevant, and project-specific maintainer guidance.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: no matcher rewrite, expansion cache, generated-source substitution, or recursion-limit change is presented as semantics-preserving without compiler validation. |
| Compiler Performance Engineer | Accepted: stable totals, parse and no-analysis boundaries, input, output, invocation, arm, recursion, fragment, incremental, thread, failure, variance, and observer effects remain separate. |
| Interop Boundary Auditor | Accepted: source tokens, macro definitions, generated AST, hygiene, cfg, resolver, filesystem, compiler, and later native boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: a PowerShell smoke outlier was not promoted, negative fragment and arm-count controls remain visible, and unstable internal pass deltas are not optimization claims. |
| Ecosystem Strategist | Accepted: rustc, Cargo, rustc-perf, the language reference, and parallel-frontend owners remain authoritative; FERRIUM supplies decomposition and fixtures. |
| Rust Maintainer | Accepted: ordinary macro APIs remain unchanged; diagnostics explain costs without prescribing obscure rewrites or expanded-source check-ins. |
| Native Platform Adopter | Accepted: Windows, NTFS, warm-cache scope, nightly diagnostics, toolchain revisions, and missing Linux/macOS evidence are explicit. |
| Scope Keeper | Accepted: Q10 covers declarative expansion only; procedural macros, general query invalidation, and crate modularization remain later questions. |
| Validation Checker | Accepted: exact generators, source sizes, toolchains, commands, 30- and 60-sample distributions, expected failures, macro statistics, profiles, source revisions, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q10 is complete.

FERRIUM should preserve declarative macro cost as a matcher, transcription,
output, recursion, integration, and invalidation topology rather than one
macro count. The next question is PERF-Q11: determine where name resolution
and HIR lowering dominate and whether their crate-wide work can become more
incremental or parallel.
