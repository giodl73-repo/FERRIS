# Parsing and Tokenization

Date: 2026-08-08
Question: PERF-Q09
Status: Complete
Decision: add source-shape, root-versus-outline-module, reparse, and parser
failure evidence to FERRIUM; pursue fixtures and finer telemetry before parser
architecture changes.

## Executive conclusion

Parsing can be material, especially for generated source, but source bytes do
not predict it and parsing is often not the dominant frontend cost.

On the stable metadata control:

- a tiny crate took 61.07 ms;
- an 8 MiB comment took 75.91 ms;
- an 8 MiB raw string took 111.88 ms;
- 50,000 constants in 1.73 MB took 873.63 ms;
- a 100,000-element expression in 200 KB took 210.79 ms.

Nightly root boundaries separated some of that difference. The 50,000-item
root added about 80 ms over the tiny parse boundary, while its no-analysis
boundary added another 138 ms and stable metadata compilation added much more.
The opportunity is therefore not “make the lexer faster” in isolation. It is
to identify whether a slow source shape is dominated by source loading,
tokenization, AST construction, external-module parsing, expansion,
resolution, or later metadata work.

The current `parse_crate` event has a non-obvious scope. It parses the root and
inline modules. An outline declaration such as `mod foo;` becomes
`ModKind::Unloaded`; `foo.rs` is opened, lexed, and parsed later during
expansion. A tiny root plus one 32,000-item external module therefore reported
about 0.7 ms in `parse_crate` and about 101 ms of `expand_crate` self time in a
diagnostic profile.

Moving 32,000 items among a root, one outline module, 32 outline modules, and
32 inline modules did not materially reduce the no-analysis boundary. It
moved attribution and introduced file topology.

rustc also does not persist tokens or ASTs in its on-disk incremental cache.
Once rustc is invoked, root parsing repeats. Cargo can avoid the process
entirely for a fresh no-op unit, and later rustc queries can reuse work, but
the parser itself did not become a cache hit in the controlled compiler
session.

The parallel frontend does not currently parallelize root parsing or the
outline-module expansion walk. Eight frontend jobs did not improve either
control.

rust-analyzer demonstrates a different model: a lossless immutable syntax tree
can replace one token or reparse a containing block and structurally share the
rest. That tree cannot currently be passed to rustc. The two parsers have
different token, tree, mutation, lifetime, diagnostic, and macro-expansion
contracts.

FERRIUM should build a read-only parse-topology diagnostic and a portable
source-shape fixture. Credible upstream paths are rustc-perf coverage and
finer timers. Parser replacement, a rust-analyzer tree bridge, automatic
module splitting, and a parse daemon remain deferred.

No upstream activity was created.

## Decision supported

This research determines:

- when source bytes, lines, tokens, items, expressions, and module files affect
  parsing;
- what `parse_crate` includes and omits;
- whether rustc incremental compilation reuses parsing;
- whether frontend threads reduce parsing;
- whether rust-analyzer's incremental tree can be reused by rustc;
- whether parser fixtures or architectural intervention are justified.

It does not authorize compiler changes, parser replacement, source rewriting,
module splitting, a persistent parser, rust-analyzer integration, or upstream
filing.

## Evidence reviewed

### Local evidence

- [rustc startup and metadata loading](2026-08-08-rustc-startup-metadata.md)
- [Rust latency telemetry](2026-08-07-rust-latency-telemetry.md)
- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### rustc sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [`rustc_interface::passes::parse`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs)
- [`rustc_parse`](https://github.com/rust-lang/rust/tree/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse)
- [lexer cooking](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/lexer/mod.rs)
- [token-tree construction](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/lexer/tokentrees.rs)
- [item and module parsing](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/parser/item.rs)
- [external module expansion](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/expand.rs)
- [external module parser](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/module.rs)
- [rustc incremental compilation](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)

### Incremental parsing and performance direction

- [rust-analyzer incremental reparse](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/syntax/src/parsing/reparsing.rs)
- [rust-analyzer architecture](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/tools/rust-analyzer/docs/book/src/contributing/architecture.md)
- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [parallel rustc announcement](https://blog.rust-lang.org/2023/11/09/parallel-rustc/)
- [parallel compiler tracking issue](https://github.com/rust-lang/rust/issues/113349)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

## Parsing model

```text
root source file
  -> file load and source-map registration
  -> raw lexer tokens
  -> cooked rustc tokens and symbol/literal handling
  -> delimiter-aware token trees
  -> recursive-descent root AST and inline modules
  -> outline modules remain unloaded
  -> expansion locates and parses outline module files
  -> macro expansion and cfg processing
  -> name resolution, validation, HIR, queries, and metadata
```

The `parse_crate` event combines the first five root-file steps. It is not a
lexer-only event and is not a whole-crate timer when outline modules exist.

## Findings

### FERRIUM-108: source shape predicts parsing better than bytes alone

**Sources**

- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)
- [rustc lexer](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/lexer/mod.rs)

**Observed behavior**

The nightly tiny root boundary was 53.63 ms.

- An 8 MiB block comment measured 69.84 ms.
- An 8 MiB raw string measured 78.14 ms.
- More than two million newlines measured 67.51 ms.
- 50,000 constants in 1.73 MB measured 134.01 ms.
- A 100,000-element expression in 200 KB measured 78.07 ms.
- A 250,000-identifier token tree in 500 KB measured 68.37 ms.

rustc first produces raw token kinds and lengths, then cooks identifiers,
literals, comments, whitespace, and delimiters into compiler tokens and token
trees.

**Implication**

FERRIUM must report bytes, line count, token shape, delimiter shape, item
count, expression shape, and generated-source origin separately. Source bytes
or lines alone are not parser cost models.

**Confidence:** high for the fixture and lexer architecture; low for
generalizing the exact ratios.

### FERRIUM-109: parsing is material but often not the dominant frontend work

**Source**

- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

**Observed behavior**

The many-item root added about 80 ms over the tiny parse boundary, but its
no-analysis median was 271.75 ms and its stable metadata median was 873.63 ms.

The 100,000-element expression had a 78.07 ms parse boundary and a 210.79 ms
stable metadata median. The macro token tree had a 68.37 ms parse boundary and
a 132.50 ms stable metadata median.

**Implication**

Generated source can make parsing visible, but a parser-only recommendation
requires component evidence. Expansion, resolution, validation, lowering,
query work, and emitted metadata can dwarf root parsing on the same source.

**Confidence:** high for the controls.

### FERRIUM-110: `parse_crate` is a root-file event, not a whole-crate parser

**Sources**

- [`parse_item_mod`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_parse/src/parser/item.rs)
- [`parse_external_mod`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/module.rs)
- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

**Observed behavior**

An outline module declaration becomes `ModKind::Unloaded` during root parsing.
The expansion walk later calls `parse_external_mod`, which opens and parses the
module file.

A 40-byte root plus one external 32,000-item module measured 55.82 ms at the
root boundary and 224.11 ms through no-analysis. Its diagnostic profile placed
about 0.72 ms in `parse_crate` and 100.69 ms of self time in `expand_crate`.

**Implication**

Self-profile reports must not equate `parse_crate` with whole-crate parse cost.
Outline-module source loading and parsing must be recovered from expansion or
finer future events.

**Confidence:** high.

### FERRIUM-111: module sharding moves parse attribution rather than removing
work

**Source**

- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

**Observed behavior**

The same 32,000 declarations measured:

- 215.12 ms through no-analysis in one root;
- 224.11 ms in one external module;
- 225.72 ms in 32 external modules;
- 226.81 ms in 32 inline modules.

The root-only boundary changed sharply because outline modules were deferred,
but the later boundary remained close.

**Implication**

FERRIUM must not recommend file or module splitting from a lower
`parse_crate` number. Module topology affects maintainability, incrementality,
resolution, metadata, and per-crate process count beyond this control.

**Confidence:** high for the control; low for larger module-depth or filesystem
topologies.

### FERRIUM-112: current root and outline-module parsing remain serial

**Sources**

- [`rustc_interface::passes::parse`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs)
- [external module expansion](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_expand/src/expand.rs)
- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

**Observed behavior**

Root parsing is one synchronous parser call. External modules are loaded in a
single expansion visitor. The 2026 goal lists parallel name resolution and
macro expansion as future work while prioritizing correctness, testing,
Cargo, and rustc-perf support.

Eight frontend jobs measured 124.49 ms versus 119.18 ms for the many-item root
and 192.91 ms versus 186.49 ms for the external-module boundary. There was no
speedup.

**Implication**

Frontend thread count is not a parser tuning knob today. Parallel module
loading or parsing is upstream compiler architecture work with diagnostic,
ordering, module-discovery, and incremental-correctness constraints.

**Confidence:** high for current source and controls.

### FERRIUM-113: rustc incremental compilation does not persist the root parse

**Sources**

- [rustc incremental compilation](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)
- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

**Observed behavior**

With a real incremental directory, the 50,000-item fixture had
`parse_crate` time-passes medians of:

- 64.16 ms with a fresh directory;
- 67.82 ms with the directory reused and source untouched;
- 101.17 ms when identical source bytes were rewritten;
- 99.20 ms after alternating one final literal.

The incremental cache reduced later end-to-end work in the untouched case, but
root parsing remained.

Rewriting identical bytes and changing one token produced the same parse-pass
range. The extra time belongs to source mutation and loading conditions, not
proven semantic parser work.

**Implication**

Cargo freshness is the current whole-invocation parse reuse boundary. Once
rustc runs, FERRIUM should assume source is loaded and parsed unless a future
compiler surface proves otherwise. Filesystem and antivirus effects remain
separate from token or AST cost.

**Confidence:** high that parsing repeats; medium on the local rewrite-cost
composition.

### FERRIUM-114: parser failures can reduce measured work by aborting early

**Source**

- [Experiment](perf-q09-parsing-tokenization/results/EXP-01-source-shape-reparse-and-modules.md)

**Observed behavior**

The valid 50,000-item root measured 125.69 ms. A malformed constant before the
items failed in 79.52 ms; the same error after the items failed in 125.88 ms.

**Implication**

Failed parses are not throughput samples. Reports retain exit status, failure
position, recovery or fatal behavior, and stderr. Faster invalid input cannot
support an optimization claim.

**Confidence:** high for this fatal parse error; low for broader diagnostic
recovery patterns.

### FERRIUM-115: rust-analyzer's incremental tree is not a rustc cache

**Sources**

- [rust-analyzer incremental reparse](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/syntax/src/parsing/reparsing.rs)
- [rust-analyzer architecture](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/tools/rust-analyzer/docs/book/src/contributing/architecture.md)

**Observed constraint**

rust-analyzer can replace one changed whitespace, comment, identifier, or
string token, or reparse a containing block and structurally share the
unchanged lossless green tree.

rustc constructs different cooked tokens, token trees, and a mutable,
non-lossless AST for expansion and compilation. No supported bridge passes a
rust-analyzer tree into rustc.

**Implication**

Incremental reparse is proven useful for IDE semantics, but sharing it with
rustc is not an external FERRIUM integration. Any common parser or tree
proposal would require compiler and rust-analyzer ownership, compatibility,
diagnostic-equivalence, macro, edition, cfg, and lifetime design.

**Confidence:** high for current architecture.

### FERRIUM-116: the contribution wedge is fixture and timer precision

**Sources**

- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- Findings FERRIUM-108 through FERRIUM-115

**Observed constraint**

rustc-perf already documents parsing-adjacent workloads:

- `libc` stresses the parser;
- `html5ever` stresses macro parsing;
- `include-blob` stresses large included data;
- `coercions` contains a large string-literal array;
- `token-stream-stress` and `tt-muncher` target token streams and expansion.

Those fixtures serve real regression coverage. The documented inventory does
not independently vary source bytes, line count, token density, item count,
expression shape, and module-file count.

The current root `parse_crate` event also combines source loading, raw lexing,
token cooking, token-tree construction, and AST parsing, while outline module
parsing appears under expansion.

**Implication**

FERRIUM can supply a parametric fixture and read-only parse-topology report.
Potential upstream work should begin with benchmark coverage or finer timing
events after owner approval, not parser replacement.

**Confidence:** high for the measurement gap; medium for upstream acceptance
until maintainers are consulted.

## Recommendations

### Adopt now

- Record source bytes, lines, generated origin, token shape, delimiter shape,
  item count, expression shape, root and outline module topology.
- Separate stable end-to-end compilation from nightly root parsing,
  expansion-attributed outline parsing, and later phases.
- Treat `parse_crate` as root and inline-module evidence only.
- Record exit status, error position, recovery behavior, and stderr for parser
  failures.
- Preserve Cargo freshness and rustc incremental reuse as different layers.
- Report file rewrite, filesystem, page-cache, antivirus, and source-loader
  effects separately from syntax complexity.

### Prototype behind a compatibility boundary

- A portable parametric parse fixture varying one source dimension at a time.
- A read-only parser topology report joined to Cargo packages and generated
  files.
- Compiler-internal diagnostic adapters for root parse, outline module parse,
  source load, lexer, token cooking, token trees, AST items, and diagnostics.
- A minimized rustc-perf fixture or timer proposal after explicit owner
  approval.

The implementation gate remains closed.

### Reject or defer

- Reject source bytes, lines, or `parse_crate` alone as whole-parser estimates.
- Reject file or module splitting based on a lower root parse timer.
- Reject frontend job count as a current parser optimization.
- Reject failed-fast parser runs as performance wins.
- Reject automatic generated-source rewrites without consumer validation.
- Defer shared rustc/rust-analyzer syntax trees.
- Defer persistent parser or compiler daemons.
- Defer parallel module parsing and parser replacement to upstream compiler
  research.
- Defer macro expansion optimization to PERF-Q10.
- Defer upstream activity until explicit owner approval.

## Potential contribution paths

Without creating upstream activity, Q09 identifies:

1. a rustc-perf fixture that varies bytes, lines, tokens, items, expressions,
   and module files orthogonally;
2. separate root source-load, raw-lex, token-cooking, token-tree, AST-parse,
   outline-module-parse, and diagnostic-recovery timers;
3. parser-heavy incremental patches showing full reparse after small edits;
4. parallel-frontend coverage proving parser and module-expansion boundaries
   remain serial;
5. minimized regressions where generated source creates disproportionate
   parser allocations or recovery work.

Each path needs Linux and macOS reproduction where relevant, current rustc-perf
comparison, and project-specific maintainer guidance.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: no parser shortcut, token reuse, module rewrite, cfg change, or diagnostic suppression is presented as semantics-preserving without compiler validation. |
| Compiler Performance Engineer | Accepted: stable totals, nightly boundaries, source shape, module topology, incremental state, frontend jobs, failures, variance, and observer effects remain separate. |
| Interop Boundary Auditor | Accepted: source files, generated inputs, filesystem, proc-macro, expansion, platform loader, and compiler-internal boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: an incorrect whole-crate interpretation of `parse_crate` was rejected after the module control; failed-fast and identical-rewrite results remain visible. |
| Ecosystem Strategist | Accepted: rustc, rust-analyzer, rustc-perf, Cargo, and the parallel frontend remain owners; FERRIUM supplies decomposition and evidence. |
| Rust Maintainer | Accepted: ordinary Cargo, modules, generated source, and editor workflows remain unchanged; diagnostics are explanatory and removable. |
| Native Platform Adopter | Accepted: Windows, NTFS, warm-cache, antivirus uncertainty, toolchain revision, and missing Linux/macOS coverage are explicit. |
| Scope Keeper | Accepted: Q09 stops at parsing and tokenization; expansion, resolution, HIR, incrementality, and modularization remain later questions. |
| Validation Checker | Accepted: exact generators, sizes, toolchains, commands, 30-sample distributions, incremental controls, expected failures, profiles, source revisions, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q09 is complete.

FERRIUM should preserve parsing as a source-shape and topology component, not a
file-size counter or one self-profile event. The next question is PERF-Q10:
determine when declarative macro expansion dominates and which token or
invalidation shapes make it disproportionate.
