# Early-Phase Incrementality

Date: 2026-08-08
Question: PERF-Q19
Status: Complete
Decision: adopt early-phase reuse-unit, reconstruction, stable-output,
namespace-coupling, and proof-cost vocabulary now; prioritize HIR-owner
reconstruction and per-file parsing fixtures for upstream investigation;
prototype only read-only opportunity visualization; defer general persistent
macro expansion, name resolution, AST caches, compiler forks, and upstream
activity.

## Executive conclusion

Rustc's incremental system protects much more downstream work than it avoids
in the early frontend.

Once rustc is invoked, root parsing, declarative macro expansion, outline-module
parsing, name resolution, AST indexing, and HIR lowering run again. Stable HIR
owner hashes then prevent unchanged results from invalidating type checking,
borrow checking, MIR, and code generation.

That reconstruct-and-compare strategy is effective for correctness and
downstream containment, but it leaves a measurable early floor:

- a 50,000-item root spent about 81 ms in diagnostic parse self time;
- 3,000 declarative macro invocations repeated about 29 ms of expansion;
- 10,000 flat structs spent about 32 ms in HIR lowering;
- a pathological 100-layer glob topology spent about 1,665 ms in resolution.

The largest cost is not automatically the best cache target. Resolution and
macro expansion are coupled to namespace fixed points, effective visibility,
hygiene, cfg, expansion order, privacy, ambiguity, spans, and diagnostics.
Persisting those results safely requires a better decomposition before it
requires a cache.

HIR lowering is the strongest near-term seam. It is already owner-shaped:
`lower_to_hir` reconstructs each definition, then feeds stable `hir_owner` and
`hir_attr_map` results into incremental queries. In the 7,017-owner incremental
control, every edit state reran all 7,017 lowering providers, while unchanged
`hir_owner` total time fell from 26.60 ms fresh to 1.23 ms warm. Later work was
protected, but reconstruction remained.

Per-file or structural parsing ranks second. Rust-analyzer demonstrates token
replacement and containing-block reparsing with structural sharing. That is
evidence that finer syntax reuse is possible, not that rust-analyzer's tree can
be passed to rustc. The compiler requires different tokens, AST mutation,
source maps, attributes, macro integration, diagnostics, and lifetimes.

There are already narrow precedents. Rustc disk-caches derive procedural-macro
expansion by invocation and input token stream, and it disk-caches module item
collections. These examples support selective query boundaries. They do not
justify serializing the whole early pipeline.

FERRIUM should make the opportunity visible first: which early work repeated,
which stable results protected later phases, how much time a finer reuse unit
could theoretically avoid, and which correctness dependencies make it unsafe.
Compiler work begins only with a minimized rustc-perf fixture and a
maintainer-reviewed boundary.

The user-facing abstraction is a **compiler query plan**. Like a database query
plan, it presents nodes, dependencies, expected and observed cost, cacheability,
invalidation, serial regions, parallel capacity, and selected reuse. Unlike a
database plan, it spans Cargo freshness, rustc driver passes, compiler queries,
backend work products, linking, validation, and concurrent build sessions.

## Decision supported

This research determines:

- which early phases currently rerun across compiler invocations;
- which existing query boundaries already provide narrow persistence;
- which candidate reuse units have credible value;
- which namespace, hygiene, source, identity, and diagnostic constraints limit
  reuse;
- which candidates deserve fixtures, design work, or deferral.

It does not authorize a syntax daemon, rust-analyzer/rustc tree bridge,
persistent resolver, expansion cache, AST serialization format, compiler fork,
or upstream issue and pull request.

## Evidence reviewed

### Local evidence

- [Cross-experiment synthesis](perf-q19-early-phase-incrementality/results/EXP-01-reuse-granularity-ranking.md)
- [Parsing and tokenization](2026-08-08-parsing-tokenization.md)
- [Declarative macro expansion](2026-08-08-declarative-macro-expansion.md)
- [Name resolution and HIR lowering](2026-08-08-name-resolution-hir-lowering.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler and tool sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [rustc early driver passes](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs#L64-L164)
- [early and HIR query definitions](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L130-L247)
- [HIR owner hashing and providers](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/mod.rs#L173-L199)
- [rust-analyzer incremental reparse](https://github.com/rust-lang/rust-analyzer/blob/478b8936bb221e84718ba2aa90906c3b32dfd3c8/crates/syntax/src/parsing/reparsing.rs)
- [rustc incremental compilation](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)
- [rustc macro expansion](https://rustc-dev-guide.rust-lang.org/macro-expansion.html)
- [rustc name resolution](https://rustc-dev-guide.rust-lang.org/name-resolution.html)
- [rustc HIR](https://rustc-dev-guide.rust-lang.org/hir.html)

### Performance direction

- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [parallel compiler tracking issue](https://github.com/rust-lang/rust/issues/113349)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

The parallel frontend goal mentions finer-grained parallelism, name resolution,
and macro expansion. It does not establish persistent early-phase reuse.
Parallelism and incrementality remain separate interventions.

## Early reuse model

```text
source identity
  -> parse unit
  -> expansion unit
  -> namespace component
  -> AST owner
  -> HIR owner
  -> downstream semantic queries

candidate benefit
  = repeated work avoided
  - identity, validation, loading, hashing, and persistence cost
  - invalidation and diagnostic risk
```

A candidate's **reuse unit** is the smallest result proposed for persistence.
Its **reconstruction cost** is the work rustc currently repeats before stable
comparison. Its **stable output boundary** is the result whose fingerprint
protects downstream work. Its **coupling frontier** is the namespace, hygiene,
source, configuration, or diagnostic state that can invalidate the unit.

## Findings

### FERRIUM-222: early incrementality is a sequence of different reuse problems

**Sources:** rustc early driver passes, early query definitions, and PERF-Q09
through PERF-Q11.

**Observed behavior:** Parsing, expansion, resolution, indexing, and lowering
have different inputs, output identities, granularity, diagnostics, and
coupling. Several are driver passes; others are `eval_always` queries; stable
owner queries begin after lowering.

**Implication:** "Cache the frontend" is not an actionable design. Each
candidate requires its own unit, identity, invalidation, persistence, and
failure contract.

**Confidence:** High.

### FERRIUM-223: root and outline-module parsing currently repeat

**Sources:** PERF-Q09 and rustc early driver passes.

**Observed behavior:** `parse_crate` ran for fresh, unchanged, rewritten, and
edited sessions. Outline module files were parsed later during expansion, and
changing module count moved attribution rather than creating persistence.

**Implication:** Per-file parsing is a credible research unit for generated or
many-item crates, but it must include source-map, edition, cfg, attribute,
diagnostic, and module-loading identity.

**Confidence:** High that parsing repeats; medium on achievable net benefit.

### FERRIUM-224: rust-analyzer proves structural reparsing, not compiler-tree reuse

**Sources:** rust-analyzer incremental reparse and PERF-Q09.

**Observed behavior:** Rust-analyzer can replace one token or reparse a
containing block and structurally share the remaining immutable green tree.
Rustc uses different parser, AST mutation, expansion, source, and diagnostic
contracts.

**Implication:** Structural syntax reuse is plausible. A direct tree bridge or
shared cache is not justified without an explicit common contract.

**Confidence:** High.

### FERRIUM-225: general declarative expansion repeats despite downstream reuse

**Sources:** PERF-Q10 and rustc early driver passes.

**Observed behavior:** The 3,000-invocation fixture repeated approximately
29 ms of expansion for unchanged, rewritten, invocation-edit, and
definition-edit sessions. Definition edits broadened later work but did not
make the expansion pass itself incremental.

**Implication:** Expansion opportunity must distinguish invocation matching,
transcription, hygiene, integration, generated definitions, and later
invalidation.

**Confidence:** High for declarative macros in the tested fixture.

### FERRIUM-226: disk-cached derive expansion is a narrow precedent

**Sources:** rustc early query definitions.

**Observed behavior:** `derive_macro_expansion` is keyed by local expansion
identity and input token stream and marked `cache_on_disk`.

**Implication:** Selective expansion persistence is possible where rustc can
define a complete query boundary. It does not generalize to the iterative
declarative expansion and resolver loop.

**Confidence:** High.

### FERRIUM-227: resolution offers high value and the hardest correctness boundary

**Sources:** PERF-Q11 and rustc name-resolution architecture.

**Observed behavior:** Ordinary resolution controls cost milliseconds, while
the 100-layer glob fixture spent about 1,665 ms resolving imports and effective
visibility. Resolution combines expansion-time graph construction, import
fixed points, visibility, ambiguity, privacy, macro scopes, and a late crate
walk.

**Implication:** Expensive namespace fixtures deserve upstream coverage, but a
persistent resolver should be deferred until outputs and invalidation can be
decomposed below the crate-wide boundary.

**Confidence:** High on cost and coupling; low on the best future design.

### FERRIUM-228: crate-wide resolver ownership constrains AST-owner persistence

**Sources:** rustc query definitions.

**Observed behavior:** `resolver_for_lowering_raw`, `index_ast`, and
`lower_to_hir` are `eval_always`. The AST index explicitly carries one shared
`ResolverAstLowering` for all owners.

**Implication:** Persisting one AST or HIR owner is not only a serialization
task. The compiler first needs stable ownership of the resolver inputs consumed
by that owner.

**Confidence:** High.

### FERRIUM-229: HIR-owner lowering is the strongest near-term seam

**Sources:** PERF-Q11, HIR owner hashing, and HIR providers.

**Observed behavior:** Ten thousand flat structs spent 32.47 ms in lowering.
In the 7,017-owner edit matrix, every state reran all lowering providers, but
stable `hir_owner` results sharply reduced downstream work.

**Implication:** A rustc-perf fixture should ask whether stable per-owner
AST/resolver inputs can avoid lowering reconstruction at net benefit while
preserving spans, attributes, desugaring, identities, and diagnostics.

**Confidence:** High on the repeated work; medium on implementation viability.

### FERRIUM-230: reconstruct-and-compare is already a successful containment strategy

**Sources:** PERF-Q11 and PERF-Q17.

**Observed behavior:** Early providers reran, but equal stable HIR and MIR
results frequently stopped invalidation before expensive later semantic or
backend work.

**Implication:** An early cache proposal must outperform a system that already
contains downstream work well. Avoided reconstruction, not downstream hit
count, is the relevant benefit.

**Confidence:** High.

### FERRIUM-231: proof and persistence cost can erase small early savings

**Sources:** PERF-Q18 and the cross-experiment synthesis.

**Observed behavior:** Incremental graph, hashing, loading, promotion, and
persistence have a fixed cost. Ordinary parse, expansion, resolution, and
lowering regions were often tens rather than hundreds of milliseconds.

**Implication:** Early persistence should target large generated source,
expensive expansions, pathological namespaces, or very high owner counts and
must compare disabled, cold, unchanged, local, and broad states.

**Confidence:** High.

### FERRIUM-232: parallelism and persistence are independent interventions

**Sources:** PERF-Q16 and the 2026 parallel frontend goal.

**Observed behavior:** Frontend jobs can schedule independent query work but
did not skip root parsing, expansion, resolution, indexing, or lowering.
Upstream plans to parallelize more early work do not define cross-invocation
reuse.

**Implication:** FERRIUM reports whether work was avoided and whether remaining
work overlapped as separate dimensions.

**Confidence:** High.

### FERRIUM-233: early-phase opportunity belongs in the visual control plane

**Sources:** findings FERRIUM-222 through FERRIUM-232 and the Rust Build Forest
opportunity.

**Observed behavior:** Current evidence can show repeated early regions, owner
counts, stable downstream containment, edit classes, and cache economics even
without changing rustc.

**Implication:** FERRIUM can visualize candidate reuse units, repeated work,
coupling frontiers, stable outputs, theoretical saved time, and upstream
fixture priority before implementing a compiler cache.

**Confidence:** High for explanation; medium for product viability.

### FERRIUM-234: the visual control plane is a compiler query plan

**Sources:** findings FERRIUM-222 through FERRIUM-233, PERF-Q03 scheduling,
PERF-Q17 invalidation, PERF-Q18 cache economics, and the Rust Build Forest
opportunity.

**Observed behavior:** Existing evidence already describes work nodes,
dependencies, execution order, provider state, stable outputs, cache layers,
cost, invalidation, and concurrency. Those are the core elements of a query
plan, but they are currently split across Cargo and rustc diagnostics.

**Implication:** FERRIUM should present a planned-versus-observed compiler query
plan whose execution history can be retained as a labeled Build Forest root.

**Confidence:** High for the model; medium for the product boundary.

## Priority ranking

1. **HIR-owner reconstruction fixture:** highest-priority upstream research
   candidate because stable owner outputs already exist.
2. **Per-file or structural parsing fixture:** worthwhile for large generated
   and many-item source, with rust-analyzer as architectural evidence rather
   than an integration contract.
3. **Narrow expansion queries:** study existing derive persistence and future
   independently identifiable macro operations.
4. **Namespace decomposition:** retain pathological import and visibility
   fixtures, but solve granularity before persistence.
5. **Whole early-pipeline cache:** reject because identity, invalidation,
   memory, diagnostics, and format risk are too broad.

## Recommendations

### Adopt now

- Record parse, expansion, resolution, indexing, lowering, stable-output, and
  downstream query time separately.
- Record candidate reuse unit, reconstruction cost, stable output boundary,
  coupling frontier, and proof/persistence cost.
- Present planned and observed work as a compiler query plan with dependencies,
  cacheability, invalidation, cost, concurrency, and selected reuse.
- Preserve body, import, visibility, module, macro, source-layout, failure, and
  broad-edit controls.
- Treat derive expansion and module-item persistence as narrow precedents.

### Prototype behind a compatibility boundary

- A read-only early-phase opportunity panel joining time-pass, self-profile,
  owner, module, macro, namespace, invalidation, and cache-economics evidence.
- A rustc-perf candidate for high-owner HIR reconstruction after maintainer
  review.
- A per-file parse fixture comparing full parse with a hypothetical structural
  reuse boundary without asserting a shared rust-analyzer/rustc tree.
- Structured diagnostics for resolver subphases and owner reconstruction if
  existing evidence cannot attribute regression.

### Reject or defer

- A direct rust-analyzer syntax-tree bridge.
- Serializing the complete rustc AST or resolver as a FERRIUM-owned format.
- General persistent declarative macro expansion.
- Persistent crate-wide name resolution.
- Automatic source or module restructuring.
- Compiler forks, daemons, or upstream filing from this question alone.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: reuse must preserve accepted programs, diagnostics, spans, hygiene, privacy, cfg, and owner identity. |
| Compiler Performance Engineer | Accepted: opportunity size, reconstruction, proof cost, persistence cost, and downstream containment are separated. |
| Interop Boundary Auditor | Accepted: target, native dependency, build-script, and external macro inputs remain explicit identity axes. |
| AI Assurance Skeptic | Accepted: theoretical saved time and architectural precedents are not treated as proof of a safe cache. |
| Ecosystem Strategist | Accepted: rustc-perf fixtures and upstream boundaries precede replacement infrastructure. |
| Rust Maintainer | Accepted: stable outputs and existing query architecture are preserved; no fork or filing is proposed. |
| Native Platform Adopter | Accepted: cold cost, memory, disk, failure, rollback, and unsupported-format risk remain visible. |
| Scope Keeper | Accepted: Q19 ranks early phases and leaves procedural macros, build scripts, remote artifacts, and implementation to later gates. |
| Validation Checker | Accepted: reused experiments retain commands, repetitions, variance, source revisions, edit classes, failures, and limitations. |

## Non-goals

- Predicting benefit for every Rust crate.
- Defining a stable syntax, AST, resolver, or HIR serialization format.
- Claiming rust-analyzer and rustc can share trees.
- Persisting general macro or namespace state.
- Modifying rustc, Cargo, rust-analyzer, or source code.
- Creating an upstream issue, branch, comment, or pull request.
