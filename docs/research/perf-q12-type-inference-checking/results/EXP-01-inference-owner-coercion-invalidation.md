# EXP-01: Inference, Owner Topology, Coercion, and Invalidation

Date: 2026-08-08
Question: PERF-Q12
Status: Complete

## Purpose

Measure trait-light type-checking cost across:

1. explicit and inferred local equalities;
2. generic identity calls;
3. function-item coercions;
4. tuple patterns;
5. expected-type-guided generic constructors;
6. one large body versus many body owners;
7. frontend job count;
8. untouched, rewritten, localized, helper-body, and shared-type edits;
9. inference and coercion failures.

The experiment separates stable complete metadata compilation from nightly
no-analysis, time-pass, self-profile, input-stat, incremental, and failure
diagnostics.

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

## Compiler boundaries

The pinned compiler defines:

- `check_type_wf(())` for crate-wide type well-formedness;
- `check_well_formed(LocalDefId)` for item well-formedness;
- `typeck_root(LocalDefId) -> TypeckResults` for root body inference;
- `cache_on_disk` for `typeck_root`;
- parallel traversal of HIR body owners in `check_crate`;
- one inference and fulfillment context shared by a root and nested closures.

Within `typeck_root`, expression checking, coercion, fallback, deferred checks,
obligation selection, closure analysis, and writeback are not separate
queries.

## Fixture families

| Fixture | Shape | Source bytes | HIR records |
| --- | --- | ---: | ---: |
| Tiny | One annotated function | 28 | 18 |
| Annotated bindings | One owner, 10,000 typed bindings | 357,816 | 90,018 |
| Inferred bindings | One owner, 10,000 equality bindings | 307,818 | 60,018 |
| Generic identity | One owner, 9,999 generic calls | 407,873 | 100,033 |
| Coercion body | One owner, 10,000 branch/function-item coercions | 927,915 | 300,057 |
| Coercion owners | 10,000 owners, one coercion each | 918,965 | 320,042 |
| Tuple patterns | One owner, 10,000 inferred tuple patterns | 805,624 | 220,024 |
| Inferred owners | 10,000 owners, one inferred binding each | 538,890 | 160,008 |
| Generic owners | 10,000 owners, one generic call each | 557,837 | 140,027 |
| Expected types | One owner, 10,000 typed `Option::None` values | 637,823 | 190,018 |

The coercion, tuple, and expected-type bodies immediately consumed each local;
the binding and generic chains kept earlier values transitively live.
Dead-local liveness therefore did not dominate. Generic identity has no
user-written trait bound. Primitive function pointers and tuple patterns avoid
method and operator lookup.

## Primary fixture matrix

Stable rustc emitted metadata. Each fixture had three warm-ups and 30
round-robin repetitions.

| Fixture | Stable median | MAD | Relative MAD | Nightly no-analysis |
| --- | ---: | ---: | ---: | ---: |
| Tiny | 61.64 ms | 2.20 ms | 3.6% | 53.33 ms |
| Annotated bindings | 209.90 ms | 8.49 ms | 4.0% | 79.53 ms |
| Inferred bindings | 172.49 ms | 5.51 ms | 3.2% | 70.12 ms |
| Generic identity | 294.12 ms | 10.91 ms | 3.7% | 83.69 ms |
| Coercion body | 698.33 ms | 12.89 ms | 1.8% | 172.64 ms |
| Coercion owners | 707.79 ms | 8.72 ms | 1.2% | 196.69 ms |
| Tuple patterns | 560.48 ms | 12.95 ms | 2.3% | 126.06 ms |
| Inferred owners | 476.29 ms | 11.88 ms | 2.5% | 117.15 ms |
| Generic owners | 457.84 ms | 9.67 ms | 2.1% | 112.90 ms |
| Expected types | 349.95 ms | 13.98 ms | 4.0% | 110.84 ms |

All stable and no-analysis series remained below the contract's 10% relative
MAD threshold.

No-analysis excludes type checking but includes parsing, expansion,
resolution, and HIR lowering. Stable minus no-analysis is not a direct
type-check timer because later matching, liveness, MIR, borrow checking,
lints, metadata, and toolchain differences remain.

Explicit annotations did not make the equality chain faster. They added
50,000 source bytes and 30,000 HIR records, produced nearly equal type-check
self time, and coincided with a larger stable total.

## Self-profile attribution

Thirty time-pass runs and five full-event self-profiles produced:

| Fixture | `type_check_crate` pass | `typeck_root` self | Misses | Match self | Liveness self | Borrow-check self |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Tiny | 1.05 ms | 0.15 ms | 1 | 0.01 ms | 0.03 ms | 0.11 ms |
| Annotated bindings | 16.26 ms | 25.19 ms | 1 | 8.58 ms | 6.64 ms | 0.12 ms |
| Inferred bindings | 15.96 ms | 26.32 ms | 1 | 8.32 ms | 5.98 ms | 0.13 ms |
| Generic identity | 54.92 ms | 104.84 ms | 2 | 8.97 ms | 18.02 ms | 0.15 ms |
| Coercion body | 75.05 ms | 163.98 ms | 3 | 10.88 ms | 57.14 ms | 0.16 ms |
| Coercion owners | 104.63 ms | 142.68 ms | 10,002 | 16.83 ms | 21.34 ms | 13.99 ms |
| Tuple patterns | 88.78 ms | 176.95 ms | 1 | 34.48 ms | 43.95 ms | 0.18 ms |
| Inferred owners | 70.05 ms | 79.64 ms | 10,000 | 21.89 ms | 21.80 ms | 14.88 ms |
| Generic owners | 89.75 ms | 121.12 ms | 10,001 | 6.56 ms | 14.55 ms | 14.37 ms |
| Expected types | 60.32 ms | 117.90 ms | 1 | 33.03 ms | 9.26 ms | 0.16 ms |

Time passes and self-profile alter execution and wall time. Their values are
diagnostic and are not expected to match each other.

The separately reported `evaluate_obligation` self time remained at or below
0.11 ms in every fixture. This does not prove that trait solving was absent:
obligation registration and selection also occur inside `typeck_root`. It
does show that the promoted controls did not expose a large standalone
obligation-query event.

Pattern, coercion, expected-type, and generic-call shapes produced different
type-check costs at similar operation counts. HIR records alone did not rank
them reliably.

## Owner topology and frontend jobs

Nightly metadata runs compared the default frontend setting with eight jobs.
Each series had three warm-ups and 30 repetitions.

| Fixture | Jobs | Median | MAD | Relative MAD |
| --- | ---: | ---: | ---: | ---: |
| Generic identity body | Default | 358.52 ms | 44.55 ms | 12.4% |
| Generic identity body | 8 | 365.31 ms | 30.33 ms | 8.3% |
| Generic identity owners | Default | 527.71 ms | 35.45 ms | 6.7% |
| Generic identity owners | 8 | 352.85 ms | 29.69 ms | 8.4% |
| Coercion owners | Default | 788.19 ms | 86.10 ms | 10.9% |
| Coercion owners | 8 | 539.90 ms | 44.66 ms | 8.3% |

The one-body default series was noisy and supports only “no observed
acceleration.” The many-owner generic comparison was robust and improved by
33.1% with eight jobs. The coercion-owner result pointed in the same direction
but its default series exceeded 10% relative MAD.

Owner width therefore exposes compiler scheduling opportunity, while one
large body remains one type-check root. This result does not justify source
splitting: many owners also added query, HIR, MIR, borrow-check, metadata, and
maintenance overhead.

## Incremental edit matrix

The incremental fixture contained:

- one shared `Scalar` type alias;
- one generic identity helper;
- 2,000 caller body owners;
- one generic call and one cast per caller.

Each reused scenario had an independent incremental directory. Primary wall
time used 15 repetitions; query summaries used five profiles.

| Scenario | Wall median | MAD | `typeck_root` self | Hits | Misses |
| --- | ---: | ---: | ---: | ---: | ---: |
| Fresh directory | 233.17 ms | 12.73 ms | 32.23 ms | 18,009 | 2,001 |
| Untouched source | 186.07 ms | 8.47 ms | 0.00 ms | 0 | 0 |
| Identical rewrite | 216.49 ms | 4.68 ms | 0.00 ms | 0 | 0 |
| One body literal | 274.08 ms | 13.35 ms | 2.31 ms | 8,006 | 1 |
| Helper body only | 267.12 ms | 12.27 ms | 2.08 ms | 8,009 | 1 |
| Shared type alias | 362.20 ms | 34.03 ms | 32.09 ms | 18,009 | 2,001 |

All wall series remained below 10% relative MAD.

The absence of a `typeck_root` row for untouched and identical-rewrite
profiles is recorded as zero provider execution, not as zero cache-load cost
for the complete compiler.

One body edit invalidated one body result. Changing the helper implementation
without changing its signature also invalidated only the helper body; callers
continued to reuse their type results. Changing the shared alias target from
`u32` to `i32` invalidated every body owner.

The identical rewrite was slower than untouched despite no type-check
execution. Parsing, lowering, stable comparison, incremental loading, and
later work remain separate costs.

## Expected failures

Nightly metadata compilation retained exit status 1 and complete stderr:

| Failure | Median | MAD | Stderr |
| --- | ---: | ---: | ---: |
| Unconstrained `Option::None` | 67.58 ms | 4.94 ms | 652 bytes |
| Unconstrained closure parameter | 61.05 ms | 2.94 ms | 616 bytes |
| Incompatible function coercion | 70.02 ms | 4.41 ms | 555 bytes |

These are correctness and diagnostic controls, not fast paths. Adding
annotations or changing branch types can alter public APIs, fallback,
coercion, and diagnostics.

## Diagnostics assessment

Current nightly diagnostics expose:

- `type_check_crate` as a crate-level time-pass region;
- `typeck_root` as a per-owner self-profile query with hits and misses;
- `check_type_wf` and `check_well_formed` item-WF queries;
- obligation, match, liveness, MIR, borrow-check, and metadata events;
- AST and HIR record counts through `input-stats`;
- debug logs for type checking, fallback, coercion, and method probing.

They do not provide a stable machine-readable per-owner split of:

- inference-variable creation and unification;
- expected-type propagation;
- generic argument inference;
- branch and return LUB work;
- coercions, deferred casts, and fallback;
- pattern checking and closure analysis;
- writeback and result hashing;
- nested trait-obligation work.

FERRIUM can join owner profiles to source and static shape externally. Any
compiler event contribution requires maintainer guidance and explicit
approval.

## Limitations

- Synthetic bodies do not represent complete application APIs.
- Trait solving is architecturally interleaved with body inference.
- Windows scheduling, filesystem, antivirus, and indexing remain host-local
  influences.
- Linux and macOS have not been measured.
- Stable complete and nightly diagnostic runs use different compiler
  revisions.
- Time passes and self-profile materially affect wall time.
- Five-profile medians are diagnostic, not primary distributions.
- Input stats count AST/HIR records, not inference variables or constraints.
- The many-owner job result does not justify source refactoring.
- Alias invalidation depends on this exact shared-type dependency graph.
- Failed compilations can stop before complete successful work.

## Reproduction evidence

The retained session evidence includes:

- deterministic fixture generators;
- stable and no-analysis distributions;
- 30-sample time-pass distributions;
- five-run self-profile summaries;
- AST and HIR input statistics;
- frontend-job controls;
- incremental edit distributions and query summaries;
- expected failure output;
- exact source sizes, toolchains, commands, and compiler revisions.
