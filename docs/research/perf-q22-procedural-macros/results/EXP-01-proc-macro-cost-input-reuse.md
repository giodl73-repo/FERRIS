# EXP-01: Procedural-Macro Cost, Inputs, and Reuse

Date: 2026-08-08
Question: PERF-Q22
Environment: Windows 11 Enterprise, x86_64-pc-windows-msvc
Rust: rustc 1.99.0-nightly (1a98b1e13 2026-08-07)
Cargo: cargo 1.99.0-nightly (c79e8f894 2026-08-04)

## Purpose

This experiment isolates four procedural-macro questions:

1. How do invocation count, token input, token output, and generated item shape
   relate to warm edit latency?
2. When rustc is forced to run, which derive, attribute, and function-like
   invocations execute again?
3. Do declared environment and file inputs cause Cargo to rebuild correctly?
4. What does rustc's experimental `-Zcache-proc-macros` reuse, and is that
   reuse safe in the presence of external inputs?

The fixture is synthetic. It does not estimate the cost of `serde_derive`,
`syn`, `quote`, database schema macros, UI frameworks, or other real macro
ecosystems.

## Fixture

Each disposable workspace contained:

- one `proc-macro` crate;
- one application crate;
- a third ordinary dependency for the cache invalidation control;
- debug information disabled;
- offline and locked Cargo execution;
- incremental compilation disabled for the primary cost matrix;
- incremental compilation enabled for reuse and input matrices.

The macro crate exposed:

- `Noop`, a derive emitting no tokens;
- `One`, a derive emitting one associated constant;
- `pass`, a pass-through attribute;
- `identity_value!`, an identity function-like macro;
- `emit_items!`, one function-like macro emitting 1,000 constants;
- tracked and untracked environment-reading derives;
- tracked and untracked file-reading derives.

Every primary timing was an uninstrumented warm `cargo +nightly check` after a
one-line application edit. Five repetitions were recorded. A separate
instrumented run counted invocations and stringified token streams. A separate
single rustc self-profile run recorded compiler event boundaries.

## Primary cost matrix

| Scenario | Warm samples, ms | Median, ms | MAD, ms | Invocations | Diagnostic input chars | Diagnostic output chars |
|---|---:|---:|---:|---:|---:|---:|
| 1,000 plain structs | 288.38, 257.16, 273.61, 232.17, 259.25 | 259.25 | 14.36 | 0 | 0 | 0 |
| 1,000 plain constants | 279.94, 267.72, 290.08, 504.69, 274.88 | 279.94 | 10.14 | 0 | 0 | 0 |
| 1,000 no-op derives | 285.45, 284.31, 269.83, 282.40, 304.02 | 284.31 | 1.90 | 1,000 | 14,890 | 0 |
| 1,000 derives emitting one associated const | 546.47, 313.66, 298.93, 312.35, 327.82 | 313.66 | 14.16 | 1,000 | 14,890 | 41,890 |
| 1,000 pass-through attributes | 235.24, 242.66, 228.59, 234.66, 222.84 | 234.66 | 6.07 | 1,000 | 14,890 | 14,890 |
| 1,000 identity function-like macros | 252.82, 266.91, 263.69, 320.94, 298.38 | 266.91 | 14.09 | 1,000 | 2,893 | 2,893 |
| One invocation emitting 1,000 constants | 202.42, 222.64, 220.54, 213.49, 447.28 | 220.54 | 7.06 | 1 | 4 | 34,780 |

The outlying 504.69, 546.47, and 447.28 ms samples remain visible. Medians and
MADs are descriptive controls, not confidence intervals.

The matched struct comparison is the least ambiguous:

- 1,000 plain structs: 259.25 ms median;
- 1,000 no-op derives: 284.31 ms median;
- 1,000 derives emitting associated constants: 313.66 ms median.

The fixture therefore detected procedural-macro work, but 1,000 trivial
invocations did not dominate the complete warm check. Emitted items added both
macro work and later compiler work.

The attribute and function-like scenarios are not valid subtraction
benchmarks against the plain controls. Source parsing, expansion topology,
AST integration, and final item shape differ. Their ordering demonstrates
that invocation count alone does not predict total latency.

## Diagnostic timing

The instrumented macro run measured time before log-file open and write, but it
did include token-stream stringification used to record input and output
lengths. It is therefore an observer-affected diagnostic, not pure macro-body
time.

| Scenario | Instrumented macro diagnostic, ms |
|---|---:|
| 1,000 no-op derives | 6.14 |
| 1,000 derives emitting associated constants | 23.70 |
| 1,000 pass-through attributes | 6.96 |
| 1,000 identity function-like macros | 6.19 |
| One invocation emitting 1,000 constants | 1.33 |

The separate self-profile run observed:

| Scenario | `expand_proc_macro` invocations | `expand_proc_macro`, ms |
|---|---:|---:|
| 1,000 no-op derives | 1,000 | 1.88 |
| 1,000 derives emitting associated constants | 1,000 | 10.81 |
| 1,000 pass-through attributes | 1,000 | 2.41 |
| 1,000 identity function-like macros | 1,000 | 1.51 |
| One invocation emitting 1,000 constants | 1 | 1.31 |

The derive runs also exposed `expand_derive_proc_macro_outer`, and all
scenarios exposed broader `expand_crate`, lowering, HIR, and later compiler
work. Self-profile total times varied substantially and are not compared
across these single diagnostic runs.

## Experimental derive cache matrix

Rustc's derive cache is disabled by default. The cache matrix explicitly set:

```text
RUSTFLAGS=-Zcache-proc-macros
```

The fixture contained three identical `One` derives, one pass-through
attribute, and one identity function-like macro.

| Step | Dirty packages | Derives executed | Attribute executed | Function-like executed |
|---|---|---:|---:|---:|
| Initial compile | dependency, macro crate, app | 3 | 1 | 1 |
| Ordinary dependency body edit | dependency, app | 0 | 1 | 1 |
| Same-content app rewrite | app | 0 | 1 | 1 |
| Unrelated app constant edit | app | 0 | 1 | 1 |
| One derive input edit | app | 1 | 1 | 1 |
| Second unrelated app edit | app | 0 | 1 | 1 |
| Procedural-macro crate edit | macro crate, app | 3 | 1 | 1 |

This confirms the narrow cache behavior:

- unchanged derive invocations loaded cached output across unrelated rebuilds;
- changing one derive input reran only that derive;
- changing the defining macro crate invalidated every derive result;
- attribute and function-like macros still executed whenever the app was
  compiled.

Without `-Zcache-proc-macros`, all five macro invocations executed on every
forced app rebuild. The cache result is therefore not ordinary Cargo behavior.

## External input matrix without derive caching

Each derive emitted an associated constant read from either an environment
variable or a file.

| Input API | Input changed without source edit | Cargo rebuilt app | Macro executed | Program output |
|---|---|---:|---:|---:|
| `std::env::var` | `1` to `2` | No | No | stale `1` |
| `proc_macro::tracked::env_var` | `1` to `2` | Yes | Yes | `2` |
| `std::fs::read_to_string` | file `1` to `2` | No | No | stale `1` |
| `proc_macro::tracked::path` plus file read | file `1` to `2` | Yes | Yes | `2` |

For each untracked case, a later source edit forced compilation and the macro
then observed `2`. The initial no-op was therefore a missed dependency, not a
macro implementation error.

The tracked APIs correctly added Cargo-visible rebuild inputs in this fixture.
They remain unstable.

## External input matrix with derive caching

The same four cases were repeated with `-Zcache-proc-macros`.

| Input API | Input change rebuilt app | Derive executed after change | Output after change | Output after later source edit |
|---|---:|---:|---:|---:|
| Untracked environment | No | No | stale `1` | stale `1` |
| Tracked environment | Yes | No | stale `1` | stale `1` |
| Untracked file | No | No | stale `1` | stale `1` |
| Tracked file | Yes | No | stale `1` | stale `1` |

The tracked environment and file changes caused Cargo to invoke rustc, but the
derive query loaded its old output from disk and did not execute the macro.
The resulting program remained stale.

This is a correctness failure in the experimental cache boundary. It is
consistent with rustc describing `-Zcache-proc-macros` as potentially
unsound. The flag must not be recommended for production use.

## Reproduction command shape

The harness generated each workspace from scratch and ran:

```powershell
cargo +nightly generate-lockfile --offline
cargo +nightly check -p app --locked --offline `
  -Z checksum-freshness --message-format=json-render-diagnostics
```

Cost runs disabled incremental compilation in `[profile.dev]`. Cache and input
runs enabled it. Cache-specific runs added:

```powershell
$env:RUSTFLAGS = "-Zcache-proc-macros"
```

Self-profile diagnostics used:

```powershell
cargo +nightly rustc -p app --locked --offline -- `
  -Zself-profile=<output-directory>
summarize summarize <profile-prefix> --json
```

## Limitations

- One Windows host and one nightly toolchain.
- Synthetic macros with no `syn`, `quote`, schema parsing, network access, or
  large dependency graph.
- Five primary timing repetitions.
- No Linux, macOS, CI, cold boot, energy, allocation, or hardware-counter
  evidence.
- Diagnostic token lengths use `TokenStream::to_string()` characters, not
  canonical token counts or serialized byte identity.
- Self-profile runs are single observer-affected diagnostics.
- The cache is an unstable, disabled-by-default compiler option.
- No general cache key, sandbox, daemon, source rewrite, or macro consolidation
  was implemented.

## Experimental conclusion

Procedural-macro cost has at least four separable dimensions:

1. invocation and bridge crossings;
2. macro logic and token processing;
3. output parsing and AST integration;
4. downstream checking and code generation of emitted Rust.

The current compiler contains a narrow derive-output cache, but enabling it is
not a safe optimization boundary. Exact token reuse worked in the controlled
matrix, while external inputs produced stale output even when the tracked APIs
correctly caused Cargo to rebuild.

The defensible next step is observability and declared-input reporting, not
general cache activation.
