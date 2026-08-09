# EXP-01: Edit Frontiers, Span Dependencies, and Red-Green Containment

Date: 2026-08-08
Question: PERF-Q17
Status: Complete

## Purpose

Measure which controlled Rust edits re-run owner-local or crate-wide query
providers, separate semantic propagation from source-layout sensitivity, test
where the red-green algorithm stops propagation, and minimize trait-impl-set
invalidation without moving downstream source positions.

The experiment answers a narrower question than "how many cache misses
occurred." A provider cache miss means rustc executed that provider in the
current session. It does not prove that the provider's result fingerprint
became red or that every downstream query also executed.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS workspace;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM `23.1.0`;
- Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`;
- host `x86_64-pc-windows-msvc`;
- rustc-perf source review pinned at
  `58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

Self-profile runs were used for provider counts, not primary wall-time claims.
Minimally instrumented timing distributions are labelled separately.

## Compiler model

Rustc records query dependencies and stable result fingerprints. When an input
cannot be proven green, rustc may re-run a provider, compare the new
fingerprint with the prior one, and stop propagation if the result is equal.
The compiler development guide calls this the red-green algorithm.

Stable span hashing is part of that result identity. On the tested compiler,
`stable_hash_span` hashes syntax context and parent information, uses
definition-relative positions when possible, and otherwise hashes the stable
source-file identity, line/column endpoints, and span length.

`-Zincremental-ignore-spans=yes` disables span hashing for testing. The
compiler option itself describes this as "used for testing"; it is not a
production optimization recommendation.

Sources:

- [red-green algorithm](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html#improving-accuracy-the-red-green-algorithm)
- [pinned stable span hashing](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ich.rs#L78-L154)
- [pinned `try_mark_green`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/dep_graph/graph.rs#L894-L1018)
- [testing-only span-ignore option](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/options.rs#L2542-L2544)
- [`rustc_clean` fingerprint checks](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/clean.rs#L1-L23)

## Primary fixture and protocol

The first fixture generated:

- one shared scalar alias;
- two traits;
- two concrete types and initial impls;
- one shared constant;
- one private helper;
- 1,000 public caller bodies.

Each scenario received a fresh baseline incremental cache. The edited compile
reused that cache:

```powershell
rustc fixture.rs --crate-name consumer --crate-type lib --edition 2024 `
  --emit metadata -o output.rmeta -Cincremental=<scenario-cache>
```

Five minimally instrumented edited compiles supplied wall distributions.
Separately labelled runs added:

```powershell
-Z self-profile=<profile-dir> -Z self-profile-events=all
```

The profile summary retained provider cache misses for type checking, MIR,
borrow checking, optimized MIR, and related queries. Queries that also execute
in an untouched compile were not attributed to the edit merely because their
provider count was nonzero.

## Primary edit matrix

Representative edited-compile provider misses:

| Edit | `typeck_root` | `mir_built` | `mir_borrowck` | Interpretation |
|---|---:|---:|---:|---|
| Untouched | 0 | 0 | 0 | Reuse control |
| Identical rewrite | 0 | 0 | 0 | File rewrite alone was reusable |
| Trailing ordinary comment | 0 | 0 | 0 | Existing owner spans did not move |
| One caller body | 1 | 1 | 1 | Required local semantic frontier |
| Local binding rename | 1 | 1 | 1 | Local owner frontier |
| Helper body | 1 | 1 | 1 | Default policy kept callers reusable |
| Helper visibility | 1 | 1 | 1 | Local in this crate |
| Equivalent signature spelling | 1 | 1 | 1 | Provider ran locally; no caller fan-out |
| Shared const value | 1 | 1 | 0 | Reference identity remained reusable |
| Shared alias `u32` to `u64` | 1,004 | 1,004 | 1,003 | Required semantic propagation |
| Helper documentation, initial fixture | 1,000 | 1,001 | 1 | Suspicious until position was controlled |
| Unused item inserted before callers | 1,001 | 1,001 | 1 | Confounded by item and span movement |
| Crate lint attribute | 1,004 | 1,003 | 0 | Confounded by lint semantics and movement |
| Reorder two items | 2 | 1,002 | 2 | Broad MIR count, narrow semantic frontier |

The matrix disproves a simple "all shared edits rebuild all callers" model.
Shared type shape propagated broadly; a shared constant value and private
helper body did not. It also shows why broad provider counts require
minimization before being called false invalidation.

## Owner-count minimization

Documentation, helper attributes, and insertion position were scaled over 1,
10, 100, and 1,000 callers.

At 1,000 callers:

| Edit | `typeck_root` | `mir_built` | `mir_borrowck` |
|---|---:|---:|---:|
| Referenced helper documentation | 0 | 1,002 | 1 |
| Unused helper documentation | 0 | 1,001 | 1 |
| Documentation on first caller | 0 | 1,000 | 1 |
| Helper-level allow attribute | 0 | 1,002 | 1 |
| Unused item before owners | 1 | 1,003 | 1 |
| Same unused item after owners | 1 | 1 | 1 |
| Reorder first two equal-width callers | 0 | 2 | 0 |
| Crate-level allow attribute | 1,002 | 1,002 | 0 |

The broad MIR counts scaled with the number of later owners, not with semantic
call relationships. Moving an insertion after the owners reduced the MIR
frontier from 1,003 to 1. Reordering two equal-width callers rebuilt those two
bodies rather than every following body.

## Layout-only source shift

A stronger control inserted an ordinary non-documentation comment before the
helper and 1,000 callers. The comment changed no tokens, attributes, item
identity, types, bodies, or lint levels. It only moved later source
coordinates.

| Mode | `typeck_root` | `mir_built` | `mir_borrowck` | Edited wall median | MAD |
|---|---:|---:|---:|---:|---:|
| Normal incremental hashing | 0 | 1,002 | 0 | 279.57 ms | 8.04 ms |
| Ignore spans, testing control | 0 | 0 | 0 | 215.62 ms | 6.56 ms |

The normal edit was 29.7% slower than the testing control. This is not a
production speedup claim for ignoring spans. It is evidence that source-layout
movement alone can create a measurable provider frontier.

## Span-ignore control

At 1,000 callers:

| Scenario | Normal `mir_built` | Ignore-spans `mir_built` |
|---|---:|---:|
| Referenced helper documentation | 1,002 | 1 |
| Unused helper documentation | 1,001 | 1 |
| One caller documentation | 1,000 | 1 |
| Helper allow attribute | 1,002 | 1 |
| Unused item before callers | 1,003 | 1 |
| Unused item after callers | 1 | 1 |
| Reorder two callers | 2 | 0 |
| Crate allow attribute | 1,002 | 0 |

The crate allow edit still produced 1,002 `typeck_root` misses with spans
ignored. Span movement therefore explained the broad MIR frontier, but not
the crate-wide type-check frontier caused by changing lint levels.

## Equal-offset substitutions

The next controls changed meaning without changing total source length or any
following owner's byte offset.

| Fixed-width substitution | `typeck_root` | `mir_built` | `mir_borrowck` |
|---|---:|---:|---:|
| Ordinary comment to helper doc comment | 0 | 1 | 1 |
| Helper lint attribute to another same-width lint | 0 | 1 | 1 |
| Crate lint attribute to another same-width lint | 1,001 | 0 | 0 |
| Reserved comment bytes to an unused item | 1 | 1 | 1 |

The trait-heavy primary fixture was repeated with a same-width ordinary
comment changed into a helper doc comment. Its initial 1,000 type-check and
1,001 MIR misses collapsed to 0 type-check, 1 MIR, and 1 borrow-check miss.

These controls isolate the cause:

- helper documentation and helper attributes were owner-local when offsets
  remained fixed;
- adding an unused item did not inherently renumber or invalidate later
  owners because stable item identities survived the reserved-slot change;
- crate-level lint semantics still reached every body even when no source
  coordinate moved.

## Dependency-graph edge probe

A 10-owner helper-documentation edit was compiled with:

```powershell
rustc fixture.rs --crate-type lib --emit metadata `
  -Cincremental=<cache> -Zquery-dep-graph -Zdump-dep-graph=yes
```

The emitted query dependency listing uses `query -> dependency` direction and
included:

```text
def_span -> source_span
thir_body -> source_span
thir_body -> hir_owner
thir_body -> typeck_root
check_match -> thir_body
check_match -> hir_owner
mir_built -> check_match
mir_built -> thir_body
mir_built -> def_span
mir_built -> hir_owner
mir_built -> hir_attr_map
mir_built -> source_span
typeck_root -> def_span
typeck_root -> hir_owner
typeck_root -> hir_attr_map
```

This identifies direct and short-path edges from span-bearing owner state into
THIR, match checking, and MIR construction. The graph labels do not expose
per-owner red/green colors, so the edge probe is combined with provider counts
and fixed-offset controls rather than used alone.

## MIR carries the changed source coordinates

The same one-caller fixture was dumped with:

```powershell
rustc fixture.rs --crate-type lib --emit metadata `
  -Zdump-mir=all -Zdump-mir-exclude-pass-number `
  -Zmir-include-spans=yes -Zdump-mir-dir=<dir>
```

Adding one helper documentation line left every MIR operation textually equal,
but changed all source annotations in `caller_0` from line 6 to line 7,
including:

```text
debug value => _1
StorageLive(_2)
_0 = helper(move _2)
StorageDead(_2)
return
resume
```

The observable MIR value therefore includes source information even when its
semantic control-flow and operations are equal. Correct diagnostics,
debuginfo, coverage, and other source-facing outputs need current locations.
The experiment does not establish that recomputing every semantic MIR
operation is the only possible architecture for updating those locations.

## Red-green containment and codegen

The helper-documentation source shift was repeated for metadata, an rlib with
debuginfo disabled, and an rlib with full debuginfo.

Provider misses:

| Output mode | Span mode | `mir_built` | `optimized_mir` | `codegen_unit` |
|---|---|---:|---:|---:|
| Metadata | Normal | 1,002 | 0 | 0 |
| Metadata | Ignore spans | 1 | 0 | 0 |
| Rlib, debuginfo 0 | Normal | 1,002 | 0 | 0 |
| Rlib, debuginfo 0 | Ignore spans | 1 | 0 | 0 |
| Rlib, debuginfo 2 | Normal | 1,002 | 0 | 1 |
| Rlib, debuginfo 2 | Ignore spans | 1 | 0 | 0 |

In the normal runs, `check_match` also ran for approximately every shifted
body, while `mir_borrowck` remained at one provider miss. The red-green
algorithm therefore contained most propagation before borrow checking,
optimized MIR, and non-debug codegen.

Seven minimally instrumented edited-build repetitions produced:

| Output mode | Normal | Ignore-spans control | Normal overhead |
|---|---:|---:|---:|
| Metadata | 246.98 ms | 230.60 ms | 7.1% |
| Rlib, debuginfo 0 | 309.72 ms | 292.31 ms | 6.0% |
| Rlib, debuginfo 2 | 389.35 ms | 286.74 ms | 35.8% |

The full-debuginfo difference is expected to be more source-location
sensitive. The span-ignore comparison remains diagnostic evidence only; it
does not preserve the production contract for source-facing outputs.

## Crate lint dependency and failure control

The fixed-width crate lint substitution invalidated 1,001 type-check roots
without moving source positions. A separate 20-owner failure fixture changed:

```rust
#![allow(unused_mut)]
```

to the equal-width:

```rust
#![deny( unused_mut)]
```

The allow revision exited successfully with no `unused_mut` errors. The deny
revision exited with status 1 and emitted 20 structured `unused_mut` errors.

The crate lint dependency is therefore correctness-relevant diagnostic work,
not a span false positive. A future optimization would need an equivalent
proof or a persisted lint result; it cannot simply remove the dependency.

## Trait impl-set control without source movement

The first impl-set matrix inserted new items and moved every caller. The
minimized fixture instead reserved fixed-width comment slots and replaced one
slot with an impl, leaving every caller at the same byte offset.

| Solver | Added impl | `typeck_root` | `mir_built` | `mir_borrowck` |
|---|---|---:|---:|---:|
| Old solver | Same trait, different type | 1 | 2 | 1 |
| Old solver | Unrelated trait | 1 | 1 | 1 |
| Global next solver | Same trait, different type | 1,001 | 1,001 | 1,001 |
| Global next solver | Unrelated trait | 1 | 1 | 1 |

The old solver reused the 1,000 callers for the non-overlapping same-trait
impl. The global next solver invalidated them all. The unrelated-trait edit
remained local in both modes.

This is a real solver dependency-precision difference on the tested nightly,
not a source-position artifact. It does not establish overall solver
superiority or authorize changing solver mode in production.

## Always-executed and non-persisted work

Untouched self-profiles still included provider activity such as
`hir_crate_items`, `effective_visibilities`, HIR lowering support, dep-graph
encoding, and metadata decoding. Some query results are deliberately not
persisted, and some session work must execute even when semantic body results
are reusable.

The reporting rule is:

1. compare against an untouched run using the same mode;
2. identify the first edit-sensitive provider frontier;
3. distinguish provider execution from a red result;
4. follow whether downstream persisted work also executes;
5. classify source-facing output dependencies separately from semantic
   dependencies.

## rustc-perf and upstream boundary

Rustc-perf supports `Full`, `IncrFull`, `IncrUnchanged`, and benchmark-defined
`IncrPatched` scenarios, including multiple numbered patch files. This is the
correct upstream performance framework.

The gap exposed here is not a missing incremental mode. It is a missing
standardized edit-geometry matrix that holds semantics constant while varying:

- insertion before versus after owners;
- source-position movement versus fixed-width substitution;
- ordinary comment versus doc or attribute semantics;
- same-trait versus unrelated-trait impl-set changes;
- check, debug, and debuginfo-sensitive output modes.

Sources:

- [rustc-perf scenario model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/scenario.rs)
- [rustc-perf patch guidance](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [rustc-perf patch expansion](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs)

No rustc-perf collector run or upstream issue was created.

## Classification

| Case | Classification |
|---|---|
| One body or local binding edit | Required local semantic propagation |
| Shared alias type change | Required broad semantic propagation |
| Crate lint level change | Required or conservative diagnostic dependency |
| Ordinary comment inserted before owners | Layout-driven provider re-execution |
| Doc, helper attribute, or item insertion with moved offsets | Mixed semantic and layout dependency until minimized |
| Equal-offset helper doc, attribute, or unused item | Owner-local result |
| Shifted MIR with reusable borrow/optimized/codegen results | Red-green containment after provider re-execution |
| Same-trait impl under global next solver | Broad solver dependency on tested revision |
| Untouched nonzero provider counts | Session or non-persisted work, not edit fan-out |

## Limitations

- Fixtures are generated and trait-light compared with large ecosystem crates.
- Windows timing and self-profile observer effects limit tiny wall
  comparisons.
- Provider counts do not expose every internal red/green transition directly.
- `-Zincremental-ignore-spans` is a diagnostic control, not a valid production
  configuration.
- Full debuginfo was measured through rustc provider counts and wall time, not
  debugger behavior or line-table inspection.
- No claim is made that splitting semantic and source-location payloads is
  easy, safe, or net beneficial.
- No compiler patch, rustc-perf run, upstream issue, or pull request was
  created.

## Reproducibility record

The retained local harnesses generated:

```text
edit-invalidation-matrix.json
candidate-minimization.json
span-sensitivity-control.json
equal-offset-controls.json
trait-doc-equal-offset.json
layout-only-shift.json
codegen-span-control.json
codegen-span-timing.json
impl-set-equal-offset-controls.json
lint-diagnostic-control.json
```

Representative commands are recorded above. Regenerable incremental caches,
self-profile binaries, MIR dump directories, compiler outputs, and temporary
artifact-consistency probes are not repository deliverables. The small
dependency-graph text and DOT dumps were retained in session evidence until
the final synthesis identified the relevant edges, then removed with other
regenerable compiler dumps.
