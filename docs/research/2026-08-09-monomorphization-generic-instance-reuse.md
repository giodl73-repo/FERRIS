# Monomorphization and Generic-Instance Reuse

Date: 2026-08-09
Question: PERF-Q24
Status: Complete
Decision: adopt a read-only generic-instance ledger and Build Forest overlay
that attribute instance families, owners, sibling duplication, upstream reuse,
compiler estimates, emitted symbols, and final-link retention; identify
generic-shell/non-generic-core candidates for human review; contribute
minimized upstream cases; defer automatic sharing overrides, API rewriting,
dynamic-dispatch conversion, cross-workspace machine-code caching, compiler
forks, and implementation.

## Executive conclusion

Rust's generic cost is not one number. It is a graph with at least four
separate outcomes:

1. rustc collects a concrete mono item;
2. a crate emits an object symbol for that item;
3. another crate may reuse an exported upstream instance;
4. the linker may retain, discard, inline, or fold emitted copies.

The PERF-Q24 fixture demonstrated all four.

An unused generic parameter still produced 512 instances for 512 concrete
types. The family carried a rustc estimate of 12,288 units and produced an
object approximately the same size as a type-dependent 512-instance family.
The compiler's former polymorphization implementation was removed in December
2024, so current stable behavior does not collapse that family.

A practical source-level pattern already works: move heavy type-independent
logic into one non-generic core and keep the generic shell small. In the same
fixture, that reduced the family estimate by 91.5% and object bytes by 25.3%.
It did not eliminate the 512 shells, and the wall sample must not be generalized
from one synthetic crate.

Current `-Zshare-generics` is real compiler behavior, not a future feature.
It defaults on at optimization levels 0, 1, `s`, and `z`, and off at levels 2
and 3. It can reuse an exported instance from an upstream dependency, but it
cannot merge sibling crates. In the dependency diamond, both siblings emitted
the same instance; the application emitted a third copy only when sharing was
off.

Intermediate duplication did not imply final binary duplication. With sharing
off, the MSVC linker assigned all three identical `shared_kernel` symbols the
same final address through identical-code folding. With sharing on, it folded
the two sibling copies to one address. The final one-CGU executables were both
145,408 bytes.

The cross-workspace control exposed the Build Forest opportunity. A second
workspace using the same disposable target directory reused the shared
dependency artifact but still emitted its own application-level generic
instance. Generic instance identity and ownership are not first-class Cargo
cache records across sibling workspace roots.

FERRIUM should make this topology visible before attempting to change it. The
defensible capability is a monomorphization ledger: which generic definition
created which concrete family, where each instance was collected and emitted,
whether an upstream crate supplied it, how much estimated and emitted work it
carried, and what the final linker retained.

This does not authorize a machine-code cache. Correct reuse would need compiler
integration and identity over definition body, substitutions, target, ABI,
codegen backend, optimization, LTO, codegen units, target features, panic and
overflow behavior, instrumentation, linkage, symbol ownership, and dependency
metadata. That belongs to PERF-Q31 and PERF-Q30, not this decision.

The measured platform scope is x86_64-pc-windows-msvc. Nightly mono-item
diagnostics are optional and version-gated. Ordinary Cargo and rustc behavior
remain authoritative; disabling the diagnostic is the rollback.

## Decision supported

This research determines:

- which monomorphization identities belong in the compiler query plan;
- how collection, emission, upstream reuse, and final-link retention differ;
- where current rustc generic sharing helps and where dependency topology
  prevents it;
- which source-level decomposition is safe to investigate without changing
  dispatch semantics;
- why cross-workspace generic reuse needs a stronger compiler and provenance
  boundary.

It does not authorize automatic `-Zshare-generics`, `#[inline]`, LTO, codegen
unit, trait-object, API, workspace, or cache changes.

## Evidence reviewed

### Local evidence

- [Mono-item growth, sharing, and link-retention matrix](perf-q24-monomorphization/results/EXP-01-mono-item-sharing-matrix.md)
- [Rust incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Type inference and checking](2026-08-08-type-inference-checking.md)
- [Rust Build Forest opportunity](2026-08-08-rust-build-forest-opportunity.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources and documentation

- [rustc dev guide: monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
- [Current mono-item collector](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_monomorphize/src/collector.rs)
- [Current codegen-unit partitioning](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_monomorphize/src/partitioning.rs)
- [Current upstream-monomorphization lookup](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ty/instance.rs)
- [Current generic-sharing defaults](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/config.rs)
- [Original upstream generic-sharing implementation](https://github.com/rust-lang/rust/pull/48779)
- [Generic-sharing tracking issue](https://github.com/rust-lang/rust/issues/47317)
- [Experimental optimized-build sharing comparison](https://github.com/rust-lang/rust/pull/50861)
- [Removed polymorphization tracking issue](https://github.com/rust-lang/rust/issues/124962)
- [Accepted compiler proposal to remove old polymorphization](https://github.com/rust-lang/compiler-team/issues/810)
- [Polymorphization removal](https://github.com/rust-lang/rust/pull/133883)
- [Current WIP polymorphization redesign](https://github.com/rust-lang/rust/pull/160300)
- [Core formatting generic-core extraction](https://github.com/rust-lang/rust/pull/157714)
- [Rust Performance Book compile-time guidance](https://nnethercote.github.io/perf-book/compile-times.html)

## Current compiler model

### Collection creates a directed instance graph

The rustc collector discovers functions, methods, closures, statics, and drop
glue reachable from roots. A generic definition can contribute zero to many
concrete mono items. Generic definitions from one crate can be instantiated in
another crate.

Constants, vtables, and some shims are created on demand and are not all
represented as collected mono items. A mono-item count is therefore useful but
not a complete machine-code inventory.

Collection can be lazy or eager. Incremental compilation uses a more stable
eager strategy that may include additional drop glue and default trait methods.
Reports must preserve collection mode.

### Sharing follows dependency direction

When sharing is enabled and metadata advertises an exact upstream instance,
`Instance::upstream_monomorphization` lets the downstream crate link to it
instead of collecting a local body.

That lookup is directional. Two sibling crates cannot consume each other's
instances while they are being built. A later dependent can consume one of
their exported instances.

Even with sharing disabled, current rustc may reuse an upstream generic marked
`#[inline(never)]`. Attribute semantics must therefore be recorded rather than
assuming the global flag is the only reuse control.

### Sharing trades internalization for reuse

Exported instances cannot be treated exactly like crate-private internal
instances. This can reduce repeated translation and LLVM work while limiting
some local optimization. Whole-graph LTO may change the trade again.

The experiment's alternating runtime control found no meaningful difference:
935.920 ms median with sharing on versus 939.310 ms with sharing off, with
overlapping dispersion. This is a negative control, not proof that sharing is
runtime-neutral.

### Linkers perform another equivalence decision

Object files can contain duplicate or differently named bodies that a linker
later folds. Linker behavior depends on target, linkage, section layout, flags,
and toolchain. Intermediate symbol count and final binary duplication are not
interchangeable.

## Findings

### FERRIUM-301: monomorphization is an instance graph, not one backend total

**Sources:** rustc dev guide, collector source, and EXP-01.

**Observed behavior:** one definition produced one item per concrete type, with
additional methods and support items reachable from the concrete body.

**Implication:** Attribute cost by generic family, substitutions, owner crate,
and use edges rather than reporting only codegen or LLVM time.

**Confidence:** High.

### FERRIUM-302: collection, emission, and final retention are separate states

**Sources:** partitioning source, `llvm-nm`, linker maps, and EXP-01.

**Observed behavior:** sharing-off collected and emitted three copies while
the linker assigned all three symbols one final address.

**Implication:** A duplicate claim must name whether it concerns collected
items, emitted symbols, archive bytes, selected objects, or retained addresses.

**Confidence:** High for the measured target; medium across linkers.

### FERRIUM-303: unused generic parameters still multiply instances

**Sources:** EXP-01, issue 124962, MCP 810, and PR 133883.

**Observed behavior:** 512 concrete types produced 512 `heavy_unused`
instances with a family estimate of 12,288.

**Implication:** Detect high-count families whose estimated body does not vary
with a generic parameter; do not claim current rustc will polymorphize them.

**Confidence:** High.

### FERRIUM-304: type-dependent generic work can amplify later support work

**Sources:** EXP-01 scale matrix.

**Observed behavior:** 512 type-dependent instances produced 1,045 total mono
items, compared with 533 for the unused-parameter family and 11 for control.

**Implication:** Family count alone is incomplete; include reachable trait
methods, shims, drop glue, and other collected support items.

**Confidence:** High for the fixture.

### FERRIUM-305: a non-generic core can reduce repeated heavy IR now

**Sources:** EXP-01, Rust Performance Book, and PR 157714.

**Observed behavior:** a thin two-unit generic shell plus one 24-unit core
reduced the family estimate by 91.5% and object bytes by 25.3% versus the
24-unit unused generic body.

**Implication:** Surface generic-shell/non-generic-core candidates for human
review, preserving API, diagnostics, inlining, and runtime controls.

**Confidence:** High for the mechanism; medium for consumer benefit.

### FERRIUM-306: removing generic dispatch changes a different contract

**Sources:** EXP-01 erased control and role review.

**Observed behavior:** one direct non-generic core reduced the family to one
instance and the object to 33,853 bytes, but it no longer represented a
generic API boundary.

**Implication:** Trait objects, function pointers, erasure, and non-generic
APIs are semantic and runtime choices, not automatic compile-time fixes.

**Confidence:** High.

### FERRIUM-307: generic sharing is implemented and profile-sensitive

**Sources:** current config and instance source, PR 48779, and EXP-01 defaults.

**Observed behavior:** opt-level 0 defaulted to sharing behavior and opt-level
3 defaulted to non-sharing behavior; explicit unstable flags reversed either.

**Implication:** Reports must record effective sharing mode and optimization
level. Do not present sharing as an unimplemented proposal.

**Confidence:** High.

### FERRIUM-308: current sharing reuses upstream instances but not siblings

**Sources:** PR 48779, current upstream lookup, and EXP-01 diamond.

**Observed behavior:** both siblings emitted each tested instance with sharing
on; the application reused one and emitted no third copy.

**Implication:** Visualize ownership along dependency direction and classify
sibling duplication separately from downstream reuse.

**Confidence:** High.

### FERRIUM-309: sharing can remove compiler work without changing final size

**Sources:** EXP-01 mono items, symbols, maps, and executable bytes.

**Observed behavior:** sharing changed three emitted copies to two, while the
one-CGU release executable remained 145,408 bytes because the linker folded
both shapes to one address.

**Implication:** Evaluate compile work, archive bytes, link work, and final
binary size independently.

**Confidence:** High for the fixture and linker.

### FERRIUM-310: LTO can dominate the sharing decision

**Sources:** EXP-01 time-pass and wall evidence.

**Observed behavior:** explicit ThinLTO builds took about 2.7 to 2.9 seconds
median, versus about 0.9 to 1.1 seconds for comparable non-whole-graph rows.
ThinLTO itself dominated compiler time.

**Implication:** Preserve LTO and local-ThinLTO mode in instance evidence.
Never recommend sharing or LTO from mono-item count alone.

**Confidence:** High for the synthetic diagnostic; low for ecosystem ratios.

### FERRIUM-311: runtime optimization loss must be measured, not assumed

**Sources:** EXP-01 alternating runtime control.

**Observed behavior:** sharing-on and sharing-off medians differed by 0.36%,
below observed dispersion.

**Implication:** A proposed sharing, inlining, erasure, or core-extraction
change needs a representative runtime and size control.

**Confidence:** High for the negative fixture result.

### FERRIUM-312: sharing does not eliminate crate invalidation

**Sources:** EXP-01 incremental edit.

**Observed behavior:** changing one sibling rebuilt that sibling and the
application in both modes. Sharing reduced recollected tested instances from
two to one but did not keep the application fresh.

**Implication:** Separate crate dirtiness, instance reuse, codegen, and relink
decisions in the compiler query plan.

**Confidence:** High.

### FERRIUM-313: Cargo artifact reuse is not generic-instance reuse

**Sources:** EXP-01 shared-target forest control.

**Observed behavior:** the second workspace reused the common dependency
artifact but emitted its own application instance and added 2,917,795 target
bytes.

**Implication:** Add cross-root instance families to the Build Forest ledger;
do not equate a fresh dependency artifact with shared downstream codegen.

**Confidence:** High for behavior; medium for storage generalization.

### FERRIUM-314: current mono diagnostics are useful but unstable

**Sources:** rustc `-Zhelp`, `-Zprint-mono-items`, JSON mono stats, and EXP-01.

**Observed behavior:** nightly diagnostics exposed item names, counts,
estimates, CGUs, and linkage, but no stable Cargo surface carried the complete
model.

**Implication:** Consume them behind an exact toolchain/schema adapter while
keeping stable Cargo and ordinary builds independent.

**Confidence:** High.

### FERRIUM-315: old compiler polymorphization is gone; redesign remains research

**Sources:** issue 124962, MCP 810, PR 133883, and PR 160300.

**Observed behavior:** the prior implementation was removed for limited value
and compiler complexity; a broad redesign is open and WIP.

**Implication:** Contribute minimized cases and evidence upstream. Do not build
a FERRIUM compiler fork around an unstable redesign.

**Confidence:** High.

### FERRIUM-316: a monomorphization ledger is the immediate product wedge

**Sources:** findings FERRIUM-301 through FERRIUM-315 and the role review.

**Observed behavior:** existing tools expose pieces of collection, IR, symbols,
and linking, but do not present one dependency- and forest-aware ownership
record.

**Implication:** Adopt read-only family, owner, duplicate-class, cost, and
retention diagnostics before attempting optimization or caching.

**Confidence:** High on the diagnostic need; medium on adoption until public
fixtures are evaluated.

### FERRIUM-317: generic machine-code caching remains a compiler contract

**Sources:** current instance and partitioning source, PERF-Q05, PERF-Q18,
PERF-Q30, and role review.

**Observed behavior:** instance availability and linkage depend on compiler
metadata, target, flags, ownership, optimization, and dependency direction.

**Implication:** Defer cross-workspace or function-level machine-code reuse to
PERF-Q31 and provenance work to PERF-Q30.

**Confidence:** High.

## Recommendations

### Adopt now

- Add generic definition, substitution, owner crate, collection mode, CGU,
  linkage, effective sharing mode, upstream provider, compiler estimate,
  emitted symbol, and final retained-address vocabulary to the measurement
  contract.
- Add a read-only monomorphization ledger to the compiler query-plan model.
- Overlay repeated instance families on labeled Build Forest roots without
  restoring or publishing machine code.
- Preserve sibling duplication, downstream reuse, cross-workspace duplication,
  and linker folding as separate dispositions.
- Rank high-count families by total estimate and emitted bytes, not count alone.
- Surface generic-shell/non-generic-core candidates as review prompts with
  explicit runtime, size, API, diagnostic, and maintenance controls.
- Use exact nightly adapters for mono diagnostics; stable Cargo remains the
  baseline and fallback.
- Send minimized, representative unused-parameter or sharing cases upstream
  with rustc-perf-compatible evidence.

### Prototype behind a compatibility boundary

- A local report joining Cargo dependency edges, mono stats, printed items,
  rlib symbols, and final linker maps.
- A source-to-link family view showing collection, owner, emitted copies,
  selected objects, folded aliases, and retained addresses.
- Public-fixture evaluation of high-count generic families and core-extraction
  opportunities.
- WIP polymorphization experiments only on pinned nightlies with compiler-team
  alignment and no product dependency.

### Reject or defer

- Automatically forcing `-Zshare-generics` on optimized builds.
- Automatically changing LTO, codegen units, `#[inline]`, or visibility.
- Automatically converting generics to trait objects, function pointers, or
  erased adapters.
- Automatically rewriting generic APIs or extracting helper functions.
- Treating mono-item count as binary size or runtime cost.
- Treating duplicate rlib symbols as final binary duplication.
- Sharing writable target directories across unrelated repositories.
- Publishing or restoring cross-workspace generic machine code.
- A FERRIUM compiler fork or private generic metadata format.
- Opening the FERRIUM implementation gate.

## Role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted: the decision changes no dispatch, ABI, ownership, aliasing, or unsafe boundary; erasure and cache reuse remain explicit semantic decisions. |
| Compiler Performance Engineer | Accepted after separating collection, emission, linking, size, and runtime; unstable wall rows are not promoted, and public-fixture plus rustc-perf evidence remains required. |
| Interop Boundary Auditor | Accepted with deferral: target, ABI, linker, LTO, native symbol, and dynamic-library behavior are recorded, while only x86_64 Windows MSVC was measured. |
| AI Assurance Skeptic | Accepted: every structural claim is tied to compiler diagnostics or linker evidence; negative runtime evidence, diagnostic observer effect, and single-fixture limits remain visible. |
| Ecosystem Strategist | Accepted: FERRIUM complements rustc diagnostics, cargo-llvm-lines, linkers, and upstream polymorphization work rather than duplicating a compiler or package manager. |
| Rust Maintainer | Accepted for read-only diagnosis: ordinary Cargo remains unchanged, recommendations are review prompts, and rollback is disabling the report. |
| Native Platform Adopter | Accepted for research only: no automatic profile or API change is made, the unsupported platform scope is explicit, and operational adoption awaits public and cross-platform fixtures. |
| Scope Keeper | Accepted: PERF-Q24 ends at instance diagnosis and upstream cases; CGU partitioning, LLVM passes, linking, remote provenance, and function caching remain Q25, Q26, Q29, Q30, and Q31. |
| Validation Checker | Accepted after preserving commands, exact toolchain, 1/32/128/512 controls, sharing defaults and overrides, incremental edit, linker map, alternating runtime, forest case, and limitations. |

## Non-goals

- Claim every generic family is a performance problem.
- Claim every unused parameter is safely removable from a public API.
- Claim the 512-type synthetic result represents ecosystem code.
- Claim generic sharing always improves compile time, binary size, or runtime.
- Claim linker identical-code folding is portable across targets.
- Claim a fresh Cargo dependency means downstream generic code was reused.
- Design a production generic-instance cache key.
- Collapse PERF-Q25 codegen-unit partitioning into this decision.
- Collapse PERF-Q31 function-level caching into this decision.
- Open the FERRIUM implementation gate.
