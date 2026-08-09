# Relink-Don't-Rebuild and Cross-Crate Interfaces

Date: 2026-08-08
Question: PERF-Q20
Status: Complete
Decision: adopt cross-crate interface, implementation, retained-artifact,
early-cutoff, and relink vocabulary now; retain read-only RDR query-plan
explanation as a FERRIUM opportunity; contribute minimized fixtures and
correctness cases upstream; defer an independent interface cache, forced
artifact reuse, compiler fork, or product implementation.

## Executive conclusion

Relink-Don't-Rebuild is one of Rust's highest-value build opportunities, but it
is not a conventional cache optimization.

Today, a real source edit in an upstream crate generally changes its compiler
identity and metadata, which makes Cargo rebuild the downstream graph. In a
controlled `base -> mid -> app` fixture, comments, private bodies, private item
insertion and reordering, public non-inline bodies, inline and generic bodies,
constants, macros, layouts, public additions, and equivalent public type
spellings all rebuilt all three packages. The result was identical with rustc
incremental compilation enabled and disabled.

Cargo checksum freshness correctly skipped an identical-content rewrite. That
is useful, but it answers whether source bytes changed, not whether changed
bytes altered the interface used by downstream compilation.

The leading safe class is an implementation edit whose linkable code changes
without changing the downstream compilation contract:

- a private non-generic function body;
- a public non-inline, non-generic function body;
- source-only edits after proving that diagnostics and emitted link inputs do
  not require additional work.

Those edits can still require rebuilding the edited crate and relinking the
final binary. RDR's benefit is pruning unchanged downstream compilation, not
pretending that implementation code did not change.

The difficult classes are values or representations that cross the crate
boundary: inline and generic MIR, constants, exported macros, layouts, trait
and type contracts, and other optimization or evaluation inputs. False
negatives here are miscompilations.

Two architectural blockers are as important as the interface hash:

1. retained downstream artifacts can contain definition and symbol identities
   invalidated when upstream items are inserted, removed, or reordered;
2. rustc invocations consume transitive metadata, so a file-level build graph
   reruns distant dependents even when an intermediate crate's effective
   interface remains unchanged.

The user-facing model remains a **compiler query plan**. For RDR it must expose
the upstream rebuild, interface decision, retained-artifact compatibility,
downstream pruning, and final link decision as separate nodes.

## Decision supported

This research determines:

- which edit classes are leading RDR candidates;
- which current identities are too broad to serve as the cutoff;
- which cross-crate values must remain correctness-sensitive;
- why stable definition identity and transitive early cutoff are required;
- what FERRIUM may explain externally and what belongs upstream.

It does not authorize a compiler fork, an API-fingerprint implementation, an
independent artifact cache, unsafe downstream artifact reuse, or an upstream
issue or pull request without a minimized regression fixture and maintainer
alignment.

## Evidence reviewed

### Local evidence

- [Cross-crate interface matrix](perf-q20-relink-dont-rebuild/results/EXP-01-cross-crate-interface-matrix.md)
- [Cargo build-unit identity](2026-08-07-cargo-build-unit-identity.md)
- [Incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Early-phase incrementality](2026-08-08-early-phase-incrementality.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Official direction and issue evidence

- [2026 Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/master/src/2026/roadmap-fast-builds.md)
- [2025H2 Relink-Don't-Rebuild goal](https://github.com/rust-lang/rust-project-goals/blob/master/src/2025h2/relink-dont-rebuild.md)
- [Compiler MCP 790](https://github.com/rust-lang/compiler-team/issues/790)
- [Cargo issue 14604](https://github.com/rust-lang/cargo/issues/14604)
- [Cargo checksum-freshness tracking issue 14136](https://github.com/rust-lang/cargo/issues/14136)

The 2026 roadmap targets avoiding downstream rebuilds when only function
bodies change and describes a 5-10x common-change opportunity. The earlier
goal remains marked Proposed, and the Cargo RDR and checksum-freshness tracking
issues remain open. The installed nightly exposed checksum freshness but no
RDR or API-fingerprint option.

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [Current crate hash](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/map.rs)
- [Metadata crate header and dependency records](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/encoder.rs)
- [Cross-crate MIR encoding policy](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/encoder.rs)

The current crate hash covers all local HIR owners, upstream crate hashes,
source identities, debugger visualizers, compiler options, stable crate
identity, visibility information, and incremental owner spans. Metadata records
that crate hash and dependency crate hashes. This is a compiler and artifact
identity, not a minimal public-interface digest.

Rustc encodes cross-crate MIR where consumers can require it, including
const-evaluation MIR and optimized MIR for reachable generic or
cross-crate-inlinable code. That source boundary supports the edit
classification below.

## RDR decision model

```text
upstream source changed?
  -> Cargo content freshness
  -> rebuild edited upstream crate
  -> compute conservative cross-crate interface identity
  -> verify retained definition and symbol compatibility
  -> prune or execute each downstream compile
  -> compare required link inputs
  -> relink only when linkable inputs changed
```

Three identities must remain distinct:

1. **Compiler artifact identity:** the broad identity rustc and Cargo require
   for correctness, compatibility, diagnostics, and incremental state.
2. **Cross-crate interface identity:** the conservative semantic input that can
   affect compilation of an unchanged dependent.
3. **Link-input identity:** the code, data, native objects, metadata, and linker
   options that determine whether the final link must run.

Reusing a downstream artifact requires all three relevant checks. Equality of
current `.rmeta` is a useful observation, not the definition of the future
interface contract.

## Edit classification

| Edit class | Downstream compile disposition | Link disposition | Reason |
|---|---|---|---|
| Identical-content rewrite | Skip upstream and downstream work when checksum-fresh | Skip | Source content is unchanged |
| Comment or formatting | Candidate to prune | Skip if link inputs are unchanged | Current metadata is oversensitive; source observables still require validation |
| Private non-generic body | Strong candidate to prune | Relink when emitted code changes | Body is not a downstream compile input |
| Public non-inline, non-generic body | Strong candidate to prune | Relink when emitted code changes | Signature crosses the boundary; ordinary body does not |
| Inline body | Rebuild consumers | Relink | Body can be imported and optimized downstream |
| Generic body or bound | Rebuild affected consumers | Relink | Consumers monomorphize and optimize the body |
| Constant or const-evaluable body | Rebuild affected consumers | Relink | Value or MIR can be evaluated downstream |
| Exported macro | Rebuild affected consumers | Relink | Expansion tokens generate downstream source |
| Public or externally observable layout | Rebuild affected consumers | Relink | Size, alignment, ABI, niches, drop, and codegen can change |
| Add or reorder private item | Candidate only after identity stability | Depends on code | Semantic interface may be stable while retained references move |
| Add public item | Conservatively rebuild | Depends on use and identity | Public interface and definition numbering change |
| Equivalent public type spelling | Canonicalization research | Usually skip if semantic identity is equal | Syntax can differ while the resolved type is equivalent |

This table is an investigation boundary, not a reusable-artifact proof.
Traits, associated items, reexports, auto traits, opaque types, specialization,
proc macros, build scripts, native dependencies, LTO, dynamic linking, and
target-specific ABI require additional cases.

## Findings

### FERRIUM-235: current nightly has no RDR cutoff for real content edits

**Sources:** EXP-01 and Cargo issue 14604.

**Observed behavior:** Every actual edit to `base` rebuilt `base`, `mid`, and
`app` in all repetitions, with incremental compilation both disabled and
enabled.

**Implication:** Rustc incremental reuse can reduce work inside each invocation
but does not currently prevent Cargo from invoking unchanged downstream crates.

**Confidence:** High for the fixture and pinned nightly.

### FERRIUM-236: checksum freshness and RDR solve different invalidation layers

**Sources:** EXP-01 and Cargo issue 14136.

**Observed behavior:** An identical-content rewrite rebuilt nothing under
`-Z checksum-freshness`, while every actual source-content edit rebuilt the
complete chain.

**Implication:** Content checksums remove timestamp oversensitivity. RDR still
needs a semantic interface decision after changed content is compiled.

**Confidence:** High.

### FERRIUM-237: current crate and metadata identities are intentionally broader than an RDR interface

**Sources:** EXP-01, rustc crate-hash source, and metadata encoder.

**Observed behavior:** Comments, private bodies, private item insertion and
reordering, and equivalent public type spelling changed upstream and
downstream `.rmeta`. Rustc's crate hash includes all HIR owners and other
compiler-wide state.

**Implication:** Do not weaken the existing correctness identity or use raw
`.rmeta` equality as the design. Add a separate conservative cross-crate
interface identity.

**Confidence:** High on current behavior; medium on final upstream design.

### FERRIUM-238: private non-generic bodies are the leading eligibility class

**Sources:** EXP-01, Compiler MCP 790, and the official RDR goal.

**Observed behavior:** A private body edit changed runtime output and upstream
linkable artifacts but does not provide source or MIR for ordinary downstream
compilation.

**Implication:** Rebuild the owner crate, retain compatible downstream
artifacts, and relink. This is the first minimized correctness fixture.

**Confidence:** High as a candidate; implementation still requires identity
proof.

### FERRIUM-239: public visibility does not make every function body a downstream input

**Sources:** rustc MIR encoding policy, Compiler MCP 790, and EXP-01.

**Observed behavior:** The body of a public non-inline, non-generic function
changed implementation behavior, while its signature remained the cross-crate
compile contract.

**Implication:** RDR classification must distinguish signature, visibility,
inline eligibility, genericity, const evaluation, and other body export paths.

**Confidence:** High.

### FERRIUM-240: inline, generic, constant, macro, and layout edits are correctness-sensitive

**Sources:** rustc MIR encoding policy, Compiler MCP 790, and EXP-01.

**Observed behavior:** These edits changed runtime output and represent values,
tokens, MIR, representation, or optimization inputs that can cross crate
boundaries.

**Implication:** Visibility-only or text-only heuristics are unsafe. False
negatives can retain miscompiled downstream code.

**Confidence:** High.

### FERRIUM-241: stable definition and symbol identity is an independent prerequisite

**Sources:** Compiler MCP 790, official RDR goal, and EXP-01 private insertion
and reorder cases.

**Observed behavior:** Adding or reordering items changes current metadata even
when the fixture's observed behavior is unchanged. The official proposal
reports retained downstream references becoming invalid when `DefId`s move.

**Implication:** Interface equality cannot authorize reuse until all retained
cross-crate references resolve to stable equivalent definitions and symbols.

**Confidence:** High.

### FERRIUM-242: transitive early cutoff requires a semantic build graph

**Sources:** official RDR goal and Cargo issue 14604.

**Observed behavior:** Rustc invocations consume metadata from transitive
dependencies. A changed transitive `.rmeta` therefore dirties a crate even when
its direct dependency exposes no changed interface from that transitive crate.

**Implication:** Cargo needs more than direct API hashing. The plan must support
early cutoff after rebuilding an intermediate crate and proving its effective
interface unchanged.

**Confidence:** High on the blocker; medium on final graph design.

### FERRIUM-243: an RDR query plan has five separately observable decisions

**Sources:** findings FERRIUM-235 through FERRIUM-242 and PERF-Q19.

**Observed behavior:** Upstream freshness, upstream compilation, interface
comparison, retained-artifact compatibility, downstream pruning, and linking
can have different outcomes.

**Implication:** A report saying "cache hit" or "rebuild" is insufficient. The
compiler query plan must show why each downstream node was retained or
executed and why the linker did or did not run.

**Confidence:** High.

### FERRIUM-244: deterministic metadata inequality is evidence, not the future contract

**Sources:** EXP-01 reproducibility control.

**Observed behavior:** Rebuilding identical edited source reproduced the same
upstream `.rmeta` hash, while each edit changed the hash relative to baseline.

**Implication:** The matrix measures real deterministic sensitivity. It does
not prove that byte equality is necessary or sufficient for safe RDR.

**Confidence:** High.

### FERRIUM-245: source-equivalent public spelling exposes canonicalization work

**Sources:** EXP-01 equivalent-type case and Compiler MCP 790.

**Observed behavior:** A semantically equivalent public type spelling changed
metadata and rebuilt the chain without changing runtime output.

**Implication:** A useful interface identity should prefer resolved semantic
types over irrelevant source position or spelling, while preserving downstream
diagnostic and documentation requirements explicitly.

**Confidence:** Medium.

### FERRIUM-246: relink eligibility and downstream compile eligibility are not the same

**Sources:** EXP-01 private and public non-inline body cases.

**Observed behavior:** Implementation edits changed executable behavior and
linkable artifacts even though they are leading candidates for retaining
downstream compilation.

**Implication:** RDR must never report "no work." It should report "upstream
rebuilt, downstream compile pruned, final relink required."

**Confidence:** High.

### FERRIUM-247: FERRIUM's near-term role is explanation and upstream fixture quality

**Sources:** official RDR status, findings FERRIUM-235 through FERRIUM-246, and
the FERRIUM prototype gate.

**Observed behavior:** The opportunity is active upstream and depends on
compiler metadata, stable identities, Cargo scheduling, and correctness review.

**Implication:** FERRIUM should visualize candidate versus observed cutoff,
maintain edit matrices, and contribute minimized cases. It should not create a
parallel artifact format or silently force reuse.

**Confidence:** High.

## Recommendations

### Adopt now

- Add RDR interface, implementation, retained-artifact, early-cutoff, and
  relink states to the measurement contract.
- Show upstream rebuild, interface result, downstream compile result, and link
  result separately in the compiler query plan.
- Preserve checksum freshness as a source-content optimization, not an RDR
  claim.
- Use private and public non-inline body edits as the first positive fixtures.
- Use inline, generic, const, macro, layout, item-insertion, reorder, and
  equivalent-type cases as required safety and oversensitivity controls.

### Prototype behind an upstream compatibility boundary

- A rustc-perf-compatible body-versus-interface fixture.
- Read-only visualization of expected and observed RDR eligibility.
- Canonical semantic interface experiments only with rustc maintainer review.
- Stable-definition and transitive-cutoff experiments in upstream compiler and
  Cargo branches, not a FERRIUM compiler fork.

### Reject or defer

- Comparing source text or visibility alone.
- Treating current `.rmeta`, SVH, crate hash, or artifact filename as the
  public-interface hash.
- Reusing downstream artifacts after a mismatched definition or symbol map.
- Skipping relinking when linkable implementation code changed.
- Publishing a proprietary cache or metadata format.
- Opening product implementation while the research gate remains closed.

## Role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted: false-negative interface classification is treated as a miscompilation risk; inline, generic, const, macro, layout, and identity cases remain mandatory. |
| Compiler Performance Engineer | Accepted: incremental work reduction, downstream invocation pruning, and linking are measured separately. |
| Interop Boundary Auditor | Accepted: ABI, native objects, dynamic linking, and target-specific layout remain explicit follow-on cases. |
| AI Assurance Skeptic | Accepted: runtime equality and metadata hashes are evidence, not proof; unknown and unsupported classes remain visible. |
| Ecosystem Strategist | Accepted: active upstream Rust and Cargo work is preferred over a parallel FERRIUM implementation. |
| Rust Maintainer | Accepted: the recommendation begins with minimized fixtures and does not weaken current compiler identities. |
| Native Platform Adopter | Accepted: final-link work, profile behavior, resource pressure, and rollback remain visible operational outcomes. |
| Scope Keeper | Accepted: Q20 closes with a fixture and contribution program, not a compiler or cache implementation. |
| Validation Checker | Accepted: positive, negative, oversensitivity, reproducibility, and both incremental-mode controls are recorded. |

## Non-goals

- Claim that RDR is implemented in the pinned nightly.
- Claim every public body is interface-sensitive.
- Claim every private item or field is interface-insensitive.
- Define the complete future API hash.
- Force reuse to demonstrate speed at the expense of correctness.
- Replace Cargo, rustc incremental compilation, or the linker.
