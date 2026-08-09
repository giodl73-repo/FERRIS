# EXP-01: Early-Phase Reuse Granularity Ranking

Date: 2026-08-08
Question: PERF-Q19
Status: Complete

## Purpose

Determine which early rustc results are credible candidates for finer-grained
cross-invocation reuse:

1. source-file tokens or AST fragments;
2. declarative macro expansions;
3. resolver and namespace results;
4. AST-owner indexing;
5. HIR-owner lowering.

This is a cross-experiment synthesis. It reuses the controlled PERF-Q09,
PERF-Q10, PERF-Q11, PERF-Q17, and PERF-Q18 measurements because those
experiments already exercised the required unchanged, rewritten, body, import,
visibility, module, macro, local, and broad edit classes on the same pinned
nightly. No new timing distribution was collected.

## Environment

The reused experiments shared:

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS storage;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- nightly LLVM `23.1.0`;
- host `x86_64-pc-windows-msvc`.

Stable workflow distributions and nightly diagnostic distributions retain the
repetition, variance, and observer-effect qualifications in their source
experiments.

## Current early pipeline

The reviewed compiler source has these boundaries:

```text
root parse
  -> cfg and crate injection
  -> iterative macro expansion and outline-module parsing
  -> import finalization and effective visibility
  -> late crate-wide resolution
  -> resolver output for lowering
  -> crate-wide AST indexing
  -> per-definition lower_to_hir reconstruction
  -> feed stable-hashed hir_owner and hir_attr_map results
  -> downstream incremental queries
```

Root parsing and the expansion/resolution pipeline are driver passes rather
than independently persisted syntax queries. `resolver_for_lowering_raw`,
`index_ast`, and `lower_to_hir` are `eval_always`. `hir_owner` and
`hir_attr_map` are feedable stable boundaries, while `hir_module_items` is
module-keyed and cached on disk.

One narrow expansion precedent already exists:
`derive_macro_expansion`, keyed by expansion identity and input token stream,
is a disk-cached query. This does not make the complete declarative expansion
loop reusable.

## Reused measurements

### Parsing

The 50,000-item parsing control reran `parse_crate` for every state:

| State | Complete wall median | `parse_crate` diagnostic median |
|---|---:|---:|
| Fresh incremental directory | 1,365.04 ms | 64.16 ms |
| Warm unchanged | 1,090.64 ms | 67.82 ms |
| Identical source rewrite | 1,119.72 ms | 101.17 ms |
| One item-value edit | 1,590.81 ms | 99.20 ms |

The diagnostic medians are not used as a claim that rewriting inherently makes
parsing slower. Their relevant result is that no state skipped parsing.

Module topology moved work rather than eliminating it. Moving 32,000 items
among the root, one outline module, 32 outline modules, or 32 inline modules
did not materially reduce the no-analysis boundary. Outline module files were
parsed later inside expansion.

### Declarative macro expansion

The 3,000-invocation control reran expansion for every state:

| State | Complete wall median | `expand_crate` median |
|---|---:|---:|
| Fresh incremental directory | 257.81 ms | 28.45 ms |
| Warm unchanged | 256.33 ms | 30.72 ms |
| Identical source rewrite | 286.42 ms | 29.08 ms |
| One invocation edit | 337.40 ms | 28.47 ms |
| Macro definition edit | 389.71 ms | 28.83 ms |

The definition edit broadened later invalidation, but the expansion pass itself
remained a full repeated crate operation.

### Resolution and lowering edits

The 7,017-owner incremental control produced:

| State | `resolve_crate` median | `lower_to_hir` self time | `lower_to_hir` misses | `hir_owner` total time |
|---|---:|---:|---:|---:|
| Fresh | 6.10 ms | 14.50 ms | 7,017 | 26.60 ms |
| Untouched | 5.98 ms | 14.68 ms | 7,017 | 1.23 ms |
| Identical rewrite | 5.89 ms | 16.11 ms | 7,017 | 1.31 ms |
| Body edit | 5.87 ms | 13.18 ms | 7,017 | 1.38 ms |
| Import edit | 6.08 ms | 14.41 ms | 7,017 | 1.35 ms |
| Visibility edit | 5.90 ms | 13.38 ms | 7,017 | 1.77 ms |
| Module edit | 5.97 ms | 15.08 ms | 7,017 | 1.37 ms |
| Macro edit | 5.96 ms | 13.62 ms | 7,017 | 1.31 ms |

The compiler reconstructed all owners, then stable owner results protected
later work. The sharp reduction in `hir_owner` total time shows successful
downstream reuse; it does not remove parsing, resolution, indexing, or lowering
cost.

### Topology-dependent opportunity size

Cold fixture profiles showed that opportunity size varies by topology:

| Fixture | Dominant early shape | Relevant diagnostic median |
|---|---|---:|
| 50,000 flat constants | Root parsing | 81.45 ms `parse_crate` self time |
| 3,000 macro invocations | Declarative expansion | about 29 ms `expand_crate` |
| 10,000 flat structs | HIR owner count | 32.47 ms `lower_to_hir` self time |
| 100 glob layers x 1,000 names | Namespace propagation | 1,665.31 ms `resolve_crate` |

The largest number is not automatically the safest cache target. The glob
case is expensive because imports and effective visibility propagate through a
crate-wide namespace topology; that same coupling makes invalidation and
diagnostic equivalence difficult.

## Candidate ranking

| Candidate | Potential value | Correctness boundary | Current disposition |
|---|---|---|---|
| Per-file or structural parsing | Medium to high for generated and many-item source | Token identity, editions, cfg, source maps, diagnostics, outline modules | Upstream fixture and design research |
| Individual declarative expansion | Medium for repeated expensive invocations | Resolver ordering, hygiene, expansion IDs, cfg, diagnostics, generated definitions | Defer general cache; study narrow query precedents |
| Namespace component or module resolution | Potentially very high for pathological topology | Imports, glob fixed points, visibility, ambiguity, macros, privacy, diagnostics | Defer persistence; first decompose and measure |
| AST-owner indexing | Medium at high owner count | One crate-wide resolver-lowering object currently supplies all owners | Prototype only with an upstream-compatible decomposition |
| HIR-owner lowering | Medium and already owner-shaped | Requires stable AST owner, resolver input, spans, attributes, desugaring, diagnostics | Highest-priority upstream fixture candidate |
| Module item collection | Narrow but proven | Module identity and HIR owner results | Existing disk-cached precedent |

## Safety criteria

An early reuse proposal is not safe merely because later HIR hashes match. It
must preserve:

- accepted and rejected programs;
- diagnostics, lints, spans, suggestions, and source ownership;
- edition, cfg, feature, target, toolchain, and command identity;
- macro hygiene, expansion ordering, recursion, and generated definitions;
- import ambiguity, privacy, effective visibility, and namespace fixed points;
- stable definition and owner identity;
- memory and serialization cost below the work avoided.

The comparison must include cold persistence, warm unchanged, local edit,
namespace edit, macro-definition edit, source-layout edit, failure, and broad
invalidation states.

## Result

The strongest near-term compiler research seam is HIR-owner reconstruction.
Rustc already has owner-level stable outputs and downstream containment; the
remaining question is whether stable AST/resolver inputs can be represented
without rebuilding every owner and without making proof or persistence cost
larger than the avoided lowering.

Per-file or structural parsing ranks second for large generated or many-item
sources. Rust-analyzer proves that token and block reparsing with structural
sharing is practical for an IDE syntax contract, but its tree is not a
drop-in rustc AST and does not establish cross-process compiler correctness.

General persistent macro expansion and name resolution remain lower priority.
They can dominate selected fixtures, but expansion, hygiene, imports,
visibility, and diagnostics are mutually coupled. The existing disk-cached
derive expansion and module-item queries are useful narrow precedents, not
evidence that the crate-wide early pipeline should be serialized wholesale.

The immediate FERRIUM product boundary remains read-only: expose repeated early
work, candidate granularity, invalidation risk, and expected saved work. Any
compiler implementation requires an upstream fixture and maintainer-reviewed
design. The natural presentation is a compiler query plan joining Cargo units,
driver passes, compiler queries, cache decisions, cost, and concurrency.
