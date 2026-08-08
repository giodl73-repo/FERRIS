# EXP-01: Source Shape, Reparse, and Module Boundaries

Date: 2026-08-08
Question: PERF-Q09
Status: Complete

## Purpose

Measure how rustc parsing changes with:

1. source bytes;
2. line count;
3. token and AST shape;
4. inline and outline module topology;
5. incremental compiler sessions;
6. frontend thread count;
7. parser failure position.

The experiment separates stable end-to-end metadata compilation from nightly
parse and no-analysis boundaries. Nightly diagnostics are not used as stable
workflow claims.

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

All promoted distributions are warm operating-system-cache results. The
compiler executable was the direct rustup toolchain binary.

## Fixtures

| Fixture | Shape | Bytes |
| --- | --- | ---: |
| Tiny | One function | 28 |
| Comment-heavy | One 8 MiB block comment and one function | 8,388,641 |
| Raw-string-heavy | One 8 MiB raw string literal | 8,388,643 |
| Newline-heavy | 2,097,152 newlines and one function | 2,097,180 |
| Many items | 50,000 public constants | 1,727,780 |
| Large expression | One 100,000-element array expression | 200,029 |
| Macro token tree | One unexpanded definition containing 250,000 identifiers | 500,063 |
| Single root | 32,000 constants in the root | 1,097,780 |
| One external module | A 40-byte root and one file containing 32,000 constants | 1,097,820 total |
| 32 external modules | A 1,600-byte root and 32 files of 1,000 constants | 1,099,380 total |
| 32 inline modules | 32 inline modules containing 32,000 constants | 1,098,452 |

The fixtures are synthetic controls. They isolate shapes and are not models of
representative application code.

## Stable primary distribution

Stable direct rustc emitted metadata. Each fixture had three warm-ups and 30
round-robin measured repetitions.

| Fixture | Median | MAD | Relative MAD |
| --- | ---: | ---: | ---: |
| Tiny | 61.07 ms | 2.26 ms | 3.7% |
| Comment-heavy | 75.91 ms | 2.32 ms | 3.1% |
| Raw-string-heavy | 111.88 ms | 3.29 ms | 2.9% |
| Newline-heavy | 100.17 ms | 3.54 ms | 3.5% |
| Many items | 873.63 ms | 9.65 ms | 1.1% |
| Large expression | 210.79 ms | 9.05 ms | 4.3% |
| Macro token tree | 132.50 ms | 3.87 ms | 2.9% |

All final series remained below the measurement contract's 10% relative-MAD
threshold.

Source shape materially changed complete compilation. It did not identify
which component caused the difference. The nightly boundaries below provide
diagnostic decomposition.

## Nightly root-parse and no-analysis boundaries

Nightly used:

```text
-Z parse-crate-root-only
-Z no-analysis
```

Each boundary had three warm-ups and 30 round-robin repetitions.

| Fixture | Root parse median | Root parse MAD | No-analysis median | No-analysis MAD |
| --- | ---: | ---: | ---: | ---: |
| Tiny | 53.63 ms | 2.89 ms | 63.44 ms | 3.13 ms |
| Comment-heavy | 69.84 ms | 2.46 ms | 78.57 ms | 2.64 ms |
| Raw-string-heavy | 78.14 ms | 4.05 ms | 91.58 ms | 5.04 ms |
| Newline-heavy | 67.51 ms | 3.27 ms | 80.44 ms | 4.90 ms |
| Many items | 134.01 ms | 4.24 ms | 271.75 ms | 12.15 ms |
| Large expression | 78.07 ms | 4.26 ms | 100.21 ms | 6.41 ms |
| Macro token tree | 68.37 ms | 2.70 ms | 97.09 ms | 4.97 ms |

Relative to the tiny root boundary:

- scanning an 8 MiB comment added about 16 ms;
- scanning and cooking an 8 MiB raw string added about 25 ms;
- processing more than two million line breaks added about 14 ms;
- parsing 50,000 items added about 80 ms;
- parsing the 100,000-element expression added about 24 ms;
- building the 250,000-identifier token tree added about 15 ms.

Bytes alone did not predict the result. The 1.73 MB many-item fixture was
slower to parse than either 8 MiB single-token fixture.

The no-analysis boundary also showed that parsing was not the complete
frontend cost. The many-item fixture added about 138 ms between root parsing
and the no-analysis boundary, and stable metadata compilation added far more
resolution, validation, lowering, and metadata work.

## Diagnostic self-profiles

Separate one-run profiles used:

```text
-Z parse-crate-root-only
-Z self-profile=<directory>
-Z self-profile-events=default
summarize summarize <profile-prefix>
```

| Fixture | `parse_crate` self time | Total profiled CPU |
| --- | ---: | ---: |
| Tiny | 0.64 ms | 1.11 ms |
| Comment-heavy | 35.36 ms | 36.86 ms |
| Raw-string-heavy | 28.41 ms | 28.87 ms |
| Newline-heavy | 14.68 ms | 15.43 ms |
| Many items | 81.45 ms | 97.88 ms |
| Large expression | 18.67 ms | 22.25 ms |
| Macro token tree | 20.41 ms | 22.20 ms |

The profiles confirm that `parse_crate` responds to the selected shapes. They
are diagnostic single runs and do not replace the external distributions. In
particular, single-run comment and raw-string ordering differed from the
external medians.

## Module topology

The 32,000-item module fixtures had three warm-ups and 30 repetitions.

| Topology | Root parse median | No-analysis median |
| --- | ---: | ---: |
| All items in root | 112.96 ms | 215.12 ms |
| One external module | 55.82 ms | 224.11 ms |
| 32 external modules | 58.48 ms | 225.72 ms |
| 32 inline modules | 113.31 ms | 226.81 ms |

The result exposed an important profiling boundary:

- `parse_crate` parses the crate-root file and inline modules;
- `mod foo;` becomes an unloaded AST module at that boundary;
- external module files are located, loaded, lexed, and parsed during
  expansion.

Separate no-analysis self-profiles showed:

| Topology | `parse_crate` | `expand_crate` self time | Total profiled CPU |
| --- | ---: | ---: | ---: |
| All items in root | 90.08 ms | 37.33 ms | 204.87 ms |
| One external module | 0.72 ms | 100.69 ms | 186.49 ms |
| 32 external modules | 0.66 ms | 88.88 ms | 174.92 ms |
| 32 inline modules | 50.00 ms | 38.66 ms | 164.17 ms |

These one-run profiles are not comparative distributions. They demonstrate
timer placement: external module parsing is visible under expansion rather
than the root `parse_crate` event.

The no-analysis medians were within about 12 ms across all four layouts.
Moving declarations into outline modules moved attribution and added file
topology; it did not remove the need to parse them.

## Frontend thread control

The parser and external-module expansion controls compared the default with:

```text
-Z unstable-options --jobs-frontend 8
```

Each had three warm-ups and 30 measured repetitions.

| Scenario | Default | Eight frontend jobs |
| --- | ---: | ---: |
| Many-item root parse | 119.18 ms | 124.49 ms |
| 32 external modules through no-analysis | 186.49 ms | 192.91 ms |

The eight-job control did not improve either boundary on this fixture. The
small slowdown must not be generalized as a parallel-frontend regression; the
source architecture shows that root parsing and the external-module expansion
walk are synchronous today.

## Incremental reparse control

Direct nightly rustc emitted metadata for the 50,000-item fixture with a real
incremental directory. This intentionally bypassed Cargo freshness so the
experiment could observe what rustc does once invoked.

Primary external wall time used 15 repetitions. Separate `-Z time-passes`
diagnostics used 10 repetitions.

| Scenario | Wall median | Wall MAD | `parse_crate` median | Parse MAD |
| --- | ---: | ---: | ---: | ---: |
| Fresh incremental directory | 1,365.04 ms | 21.99 ms | 64.16 ms | 3.27 ms |
| Reused directory, source untouched | 1,090.64 ms | 12.87 ms | 67.82 ms | 4.97 ms |
| Reused directory, identical bytes rewritten | 1,119.72 ms | 13.95 ms | 101.17 ms | 2.31 ms |
| Reused directory, final literal alternated | 1,590.81 ms | 33.96 ms | 99.20 ms | 2.48 ms |

Incremental compilation reduced later work in the untouched control, but the
root still parsed. The fresh and untouched parse medians were close.

Rewriting identical bytes and changing one literal produced nearly identical
parse-pass medians. The extra time therefore cannot be assigned to semantic
reparse work. The `parse_crate` pass includes opening and loading source; an
immediately rewritten file can change filesystem, page-cache, antivirus, and
source-loading behavior.

This control does not imply that a Cargo no-op parses source. Cargo freshness
can skip rustc entirely. It establishes that rustc's on-disk incremental
cache does not provide a persistent root token or AST cache once the compiler
process runs.

## Parser failure-position control

Expected failure was retained rather than converted into success. The valid
50,000-item file was compared with one malformed constant before or after the
same item set. Each had three warm-ups and 30 repetitions.

| Scenario | Expected exit | Median | MAD | Stderr |
| --- | ---: | ---: | ---: | ---: |
| Valid | 0 | 125.69 ms | 5.64 ms | 0 bytes |
| Parse error before the items | 1 | 79.52 ms | 2.06 ms | 298 bytes |
| Parse error after the items | 1 | 125.88 ms | 4.94 ms | 316 bytes |

The early error aborted before the complete valid suffix was parsed. Its lower
time is failure-short-circuit evidence, not a parser speedup. The end error had
the same median as the valid control.

## Source correspondence

The nightly source revision
`1a98b1e135b254f209c67d447b6d8bcd56a859e0` shows:

- `rustc_interface::passes::parse` wraps root source loading, lexing,
  token-tree construction, and AST parsing in `parse_crate`;
- `parse_item_mod` records outline modules as `ModKind::Unloaded`;
- expansion's invocation collector calls `parse_external_mod`;
- `parse_external_mod` opens the outline module file and invokes a parser;
- root parsing and the outline-module expansion walk are synchronous;
- rustc's persistent query reuse does not cache the root tokens or AST; later
  query work begins from a newly constructed root AST.

Sources:

- [`rustc_interface::passes::parse`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs)
- [`parse_item_mod`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/parser/item.rs)
- [rustc lexer](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/lexer/mod.rs)
- [token-tree construction](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/lexer/tokentrees.rs)
- [external module expansion](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/expand.rs)
- [`parse_external_mod`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/module.rs)
- [rustc incremental compilation](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)

## Limitations

- One Windows host and warm operating-system caches.
- Synthetic generated source, not a representative real crate.
- Stable totals include work after parsing and cannot attribute components.
- Nightly stop boundaries and self-profile are compiler-internal diagnostics.
- Root `parse_crate` is not a whole-crate parse timer for outline modules.
- No lexer-only supported boundary exists, so source loading, raw lexing,
  token cooking, token-tree construction, and AST parsing remain combined.
- Token counts are generator-defined approximations rather than compiler
  counters.
- The incremental control deliberately invokes rustc when Cargo would skip a
  no-op unit.
- Immediate file rewriting can interact with NTFS, page cache, antivirus, and
  indexing.
- Expected parser failures can abort at different points and are not
  throughput comparisons.
- Frontend jobs affect other compiler work not exercised by the stop
  boundaries.
- No cold-boot, Linux, macOS, native profiler, allocation, hardware-counter,
  or energy evidence was collected.
- No upstream activity was created.

## Retained evidence

The private experiment record retains:

- deterministic fixture generators;
- stable 30-sample metadata distributions;
- nightly 30-sample root and no-analysis boundary distributions;
- module-topology and frontend-job distributions;
- incremental primary and time-passes distributions;
- expected parser failure output;
- time-passes JSON;
- raw measureme profiles and summaries;
- exact toolchains, source sizes, and commands.
