# EXP-01: Matcher, Transcription, and Invalidation Shapes

Date: 2026-08-08
Question: PERF-Q10
Status: Complete

## Purpose

Measure declarative macro cost across:

1. input token volume;
2. generated output volume;
3. invocation count;
4. matcher arm count and prefix overlap;
5. direct repetition versus recursive TT munching;
6. Rust nonterminal fragment parsing;
7. frontend job count;
8. incremental compiler sessions and edit fanout;
9. failed matching and diagnostics.

Procedural macro execution is outside this experiment.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS workspace;
- stable rustc `1.95.0 (59807616e 2026-04-14)`;
- stable LLVM `22.1.2`;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- nightly LLVM `23.1.0`;
- host `x86_64-pc-windows-msvc`.

All promoted distributions are warm operating-system-cache results using the
direct toolchain compiler executable.

## Fixture families

| Family | Controlled dimension |
| --- | --- |
| Capture | Match 10,000 identifiers and emit nothing |
| Emit | Emit 10,000 constants through 1, 100, or 1,000 invocations |
| Distinct arms | Match the last of 1, 100, or 1,000 arms with different first tokens |
| Overlapping arms | Match the last of 100 or 1,000 arms sharing a ten-token prefix |
| Repetition | Consume 250, 500, 1,000, or 1,500 identifiers in one repetition |
| TT muncher | Recursively consume the same identifier counts one token at a time |
| Fragment | Match 1,000 identical inputs as token trees or expressions |
| Incremental | Expand 3,000 invocations after no change, rewrite, invocation edit, or definition edit |
| Failure | Fail after trying 100 or 1,000 arms |

The fixtures are synthetic controls. They isolate dimensions and are not
representative macro APIs.

## Stable primary distribution

Stable direct rustc emitted metadata. Each scenario had three warm-ups and 30
round-robin repetitions.

| Scenario | Source bytes | Median | MAD | Relative MAD |
| --- | ---: | ---: | ---: | ---: |
| Tiny control | 28 | 92.57 ms | 7.76 ms | 8.4% |
| Capture 10,000 identifiers, no output | 88,952 | 94.14 ms | 7.11 ms | 7.6% |
| Emit 10,000 items in 1 invocation | 98,977 | 346.32 ms | 32.26 ms | 9.3% |
| Emit 10,000 items in 100 invocations | 68,879 | 341.13 ms | 19.56 ms | 5.7% |
| Emit 10,000 items in 1,000 invocations | 76,979 | 341.12 ms | 19.75 ms | 5.8% |
| 1 arm, 1,000 invocations | 25,970 | 141.87 ms | 8.30 ms | 5.8% |
| 100 distinct-prefix arms, 1,000 invocations | 31,594 | 145.75 ms | 6.79 ms | 4.7% |
| One repetition over 1,000 identifiers | 7,951 | 92.55 ms | 8.42 ms | 9.1% |
| Recursive TT muncher over 1,000 identifiers | 8,019 | 145.65 ms | 6.80 ms | 4.7% |

All stable series remained below the contract's 10% relative-MAD threshold.

Matching 10,000 identifiers and emitting nothing remained within about 2 ms of
the tiny control. Emitting 10,000 items added about 249 ms. Splitting that
output among 1, 100, or 1,000 invocations did not materially change the stable
median in this simple fixture.

## Nightly parse and no-analysis boundaries

Nightly boundaries used 60 repetitions after three warm-ups.

| Scenario | Root parse median | No-analysis median |
| --- | ---: | ---: |
| Tiny control | 63.45 ms | 72.21 ms |
| Capture 10,000 identifiers | 66.64 ms | 79.07 ms |
| Emit 10,000 items in 1 invocation | 69.04 ms | 117.63 ms |
| Emit 10,000 items in 100 invocations | 67.30 ms | 120.30 ms |
| Emit 10,000 items in 1,000 invocations | 68.50 ms | 121.49 ms |
| 1 arm, 1,000 invocations | 65.92 ms | 85.41 ms |
| 100 distinct-prefix arms, 1,000 invocations | 66.64 ms | 83.43 ms |
| One repetition over 1,000 identifiers | 65.36 ms | 77.25 ms |
| Recursive TT muncher over 1,000 identifiers | 66.95 ms | 125.39 ms |

The matching and recursive fixture pairs had similar root boundaries. Their
differences appeared after expansion began.

The no-analysis boundary includes expansion, macro name resolution, output
integration, AST validation, and other early frontend work. It is not an
exclusive macro matcher timer.

## Generated output and invocation count

Separate `-Z macro-stats` diagnostics reported:

| Fixture | Uses | Output lines | Cumulative output bytes |
| --- | ---: | ---: | ---: |
| Capture 10,000 identifiers | 1 | 1 | 0 |
| Emit 10,000 items once | 1 | 10,000 | 298,889 |
| Emit 10 items 1,000 times | 1,000 | 10,000 | 267,900 |

The simple emit fixtures had equivalent item output and similar stable and
no-analysis medians despite a 1,000-fold invocation-count difference.

This does not prove invocation count is free. Every invocation still needs
resolution, an expansion ID, hygiene, matching, transcription, integration,
and diagnostics. It shows that generated output and later item work dominated
this fixture before invocation setup did.

## Recursive TT muncher scaling

Each scaling scenario had three warm-ups and 30 repetitions.

| Input identifiers | Repetition no-analysis | Muncher no-analysis | Repetition stable | Muncher stable | Muncher cumulative output |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 250 | 75.23 ms | 77.74 ms | 86.40 ms | 94.80 ms | 339,381 bytes |
| 500 | 73.56 ms | 87.59 ms | 85.87 ms | 102.89 ms | 1,371,256 bytes |
| 1,000 | 76.43 ms | 121.82 ms | 87.60 ms | 151.66 ms | 5,997,006 bytes |
| 1,500 | 75.00 ms | 178.54 ms | 89.13 ms | 215.58 ms | 14,122,756 bytes |

Root parse medians remained within about 3 ms across every repetition and
muncher pair.

The direct repetition stayed flat. The recursive muncher repeatedly
transcribed its remaining tail into the next invocation. `macro-stats`
therefore recorded about 14.1 MB of cumulative expansion output for an 18.5 KB
source whose final expansion emitted no items.

The observed growth is consistent with the known quadratic TT-muncher pattern.
It is not a general complexity theorem for all recursive macros.

## Matcher arm and prefix shape

The arm-shape controls had equivalent 1,000-item output and 30 repetitions.

| Matcher | Stable median | Root parse median | No-analysis median |
| --- | ---: | ---: | ---: |
| 1 arm | 146.06 ms | 63.28 ms | 82.58 ms |
| 100 distinct first tokens | 146.99 ms | 64.59 ms | 81.99 ms |
| 1,000 distinct first tokens | 169.07 ms | 66.81 ms | 96.63 ms |
| 100 arms with a shared ten-token prefix | 173.25 ms | 64.71 ms | 103.00 ms |
| 1,000 arms with a shared ten-token prefix | 364.83 ms | 71.13 ms | 295.07 ms |

`macro-stats` reported approximately 27–33 KB of output for the three
1,000-arm comparison variants. Output volume therefore did not explain the
roughly 198 ms no-analysis difference between distinct and overlapping
1,000-arm matchers.

Arm count alone was a weak estimate. Rustc could reject distinct leading
tokens quickly. Shared prefixes forced more matcher progress before an arm
failed.

## Fragment-kind negative control

One thousand no-output invocations matched the same `1 + 2 * 3` input either
as token trees or as an expression fragment.

| Matcher fragment | Stable metadata | No-analysis |
| --- | ---: | ---: |
| Token tree | 81.87 ms | 75.46 ms |
| Expression | 82.35 ms | 76.36 ms |

This simple expression did not produce a material fragment-parser cost.
Complex types, patterns, paths, and expressions remain unmeasured.

## Frontend thread control

Thirty repetitions compared the default with:

```text
-Z unstable-options --jobs-frontend 8
```

| Scenario | Default | Eight frontend jobs |
| --- | ---: | ---: |
| 1,500-token TT muncher | 179.63 ms | 188.98 ms |
| 1,000 invocations emitting 10,000 items | 119.00 ms | 127.17 ms |

Eight jobs did not improve either expansion control. The small slowdown is not
generalized beyond this fixture.

## Incremental expansion and edit fanout

Direct nightly rustc emitted metadata for 3,000 single-item invocations with a
real incremental directory. This intentionally bypassed Cargo freshness to
observe rustc after process launch.

Primary wall time used 15 repetitions. Separate `-Z time-passes` diagnostics
used 30 repetitions.

| Scenario | Wall median | Wall MAD | `expand_crate` median |
| --- | ---: | ---: | ---: |
| Fresh incremental directory | 257.81 ms | 16.97 ms | 28.45 ms |
| Reused directory, source untouched | 256.33 ms | 13.67 ms | 30.72 ms |
| Reused directory, identical bytes rewritten | 286.42 ms | 11.15 ms | 29.08 ms |
| One invocation name alternated | 337.40 ms | 12.20 ms | 28.47 ms |
| Macro definition value alternated | 389.71 ms | 14.16 ms | 28.83 ms |

The expansion pass remained around 28–31 ms in every case. It did not become
zero when the incremental directory and source were unchanged.

The definition edit changed all 3,000 expanded constant values and had the
largest full wall time. The invocation edit changed one generated item and
was cheaper. That difference occurs across post-expansion invalidation and
metadata work; the experiment does not assign it to one query family.

Immediately rewriting identical bytes increased root parsing, as Q09
observed, but did not increase the expansion pass.

Some internal pass series had relative MAD just above 10%; their exact small
differences are diagnostic only. The repeated nonzero expansion and source
architecture support the reuse conclusion.

Cargo can skip rustc for a fresh no-op unit. This experiment establishes that
rustc does not persist declarative expansion results once invoked.

## Failed matcher control

Expected failures were retained.

| Scenario | Expected exit | Median | MAD | Stderr |
| --- | ---: | ---: | ---: | ---: |
| 100 arms, no match | 1 | 68.78 ms | 2.55 ms | 852 bytes |
| 1,000 arms, no match | 1 | 73.50 ms | 3.16 ms | 854 bytes |

One failed invocation remained near the process floor. It cannot estimate a
crate with many successful or failed invocations and is not a speed result.

## Diagnostic self-profiles

Separate one-run default-event profiles reported:

| Fixture | `expand_crate` self time | Inclusive expansion time | Total profiled CPU |
| --- | ---: | ---: | ---: |
| Capture 10,000 identifiers | 4.64 ms | 13.48 ms | 22.82 ms |
| Emit 10,000 items once | 28.15 ms | 34.48 ms | 66.47 ms |
| Emit 10 items 1,000 times | 30.24 ms | 38.72 ms | 67.83 ms |
| 1,000 distinct arms × 1,000 invocations | 26.46 ms | 31.99 ms | 47.24 ms |
| Repetition over 1,500 identifiers | 3.25 ms | 9.66 ms | 14.96 ms |
| TT muncher over 1,500 identifiers | 117.02 ms | 122.96 ms | 128.06 ms |

`macro_expand_crate` wraps `expand_crate`, so its self time is small while its
inclusive time contains the child event.

The current summaries do not separate arm matching, named nonterminal parsing,
transcription, output reparsing, hygiene, invocation collection, macro
resolution, and AST integration.

## Source correspondence

The nightly source revision
`1a98b1e135b254f209c67d447b6d8bcd56a859e0` shows:

- crate expansion iteratively resolves, expands, and integrates invocations;
- macro-by-example matching uses an NFA-like set of matcher positions;
- arms are considered in declaration order until one succeeds;
- named nonterminals call the Rust parser;
- transcription substitutes bindings, builds token streams, and applies
  hygiene marks;
- expanded token output is parsed into the requested AST fragment;
- expansion is timed as imperative compiler work rather than an incremental
  query.

Sources:

- [`rustc_interface::passes`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs)
- [macro expansion loop](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/expand.rs)
- [macro-by-example rules](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_rules.rs)
- [NFA matcher](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/macro_parser.rs)
- [transcriber](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/mbe/transcribe.rs)
- [macro statistics](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/stats.rs)
- [rustc-dev-guide macro expansion](https://github.com/rust-lang/rustc-dev-guide/blob/0e48eac6e3fb4b92ad46495325d6237a7b3ed989/src/macro-expansion.md)

## Limitations

- One Windows host and warm operating-system caches.
- Synthetic macros designed to isolate dimensions.
- `-Z no-analysis` includes more than macro matching and transcription.
- Stable metadata includes validation, resolution, lowering, incremental, and
  metadata work after expansion.
- `macro-stats` is a human-readable diagnostic with observer effect, not a
  durable machine interface.
- Cumulative output bytes count every successful expansion result, including
  intermediate recursive invocations; they are not final crate size.
- Matcher inputs used identifiers and literal prefixes; other token trees,
  separators, nesting, and fragment kinds can behave differently.
- The expression-fragment control used one small expression.
- The incremental control invokes rustc where Cargo can skip a no-op unit.
- Definition and invocation edits changed different expanded outputs and later
  query invalidation.
- Some diagnostic pass distributions remained slightly above 10% relative
  MAD and are not optimization comparisons.
- Expected match failures do not model recovery-heavy error storms.
- Frontend jobs affect later compiler work not exercised by no-analysis.
- No procedural macro execution, built-in macro-specific behavior, Linux,
  macOS, cold boot, allocation profiler, hardware counter, or energy evidence.
- No upstream activity was created.

## Retained evidence

The private experiment record retains:

- deterministic fixture generators;
- stable 30-sample metadata distributions;
- nightly 60-sample root and no-analysis distributions;
- 30-sample recursion, arm, fragment, thread, and failure controls;
- incremental wall and pass distributions;
- `macro-stats` output;
- time-passes data;
- raw measureme profiles and summaries;
- exact source sizes, toolchains, commands, and expected failures.
