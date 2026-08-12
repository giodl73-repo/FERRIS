---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:borrow-checking-nll-polonius
kind: guide
module: rust-architecture
section: rust-architecture
title: Borrow Checking - MIR Borrowck, NLL, and Polonius
status: source-custody
source_custody: partial
current_path: rust-architecture/10-BORROW-CHECKING-NLL-AND-POLONIUS.md
canonical_path: rust-architecture/10-BORROW-CHECKING-NLL-AND-POLONIUS.md
backsource_ids: [mdloom-backfill:rust-architecture:10-borrow-checking-nll-polonius]
concepts: [borrow checking, nll, region inference, loans, moves, polonius]
root_concepts: [borrow checking]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Borrow Checking - MIR Borrowck, NLL, and Polonius

## The Big Picture

Borrow checking is the soundness core that lets Rust ship memory safety without
a tracing GC: aliasing XOR mutability, no use-after-move, and references that do
not outlive their referents. In rustc it runs as MIR borrow checking, after type
checking and MIR construction, using the CFG from [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md)
and the type/region obligations from [07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md).

```
+===========================================================================+
|                              MIR BORROWCK                                 |
|                                                                           |
|  typed HIR/typeck [07]        MIR body [09]                               |
|  outlives obligations         locals, places, CFG points                  |
|             |                         |                                   |
|             +-----------+-------------+                                   |
|                         v                                                  |
|                  query: mir_borrowck                                      |
|                         |                                                  |
|     +-------------------+-------------------+                              |
|     | liveness          | region constraints| move/init dataflow           |
|     | CFG point sets    | outlives closure  | maybe init / maybe uninit    |
|     +-------------------+-------------------+                              |
|                         |                                                  |
|                         v                                                  |
|           loan conflict checks + diagnostic construction                   |
|                         |                                                  |
|              accepted MIR or E0382/E0499/E0502/E0505...                   |
+===========================================================================+
```

Authority boundary: the Rust Reference owns the stable borrowing and lifetime
rules; rustc owns the MIR borrowck implementation, NLL representation, loan
dataflow, and diagnostics machinery; Cargo only invokes rustc; rustup selects
the toolchain; std exposes safe APIs that rely on these rules; LLVM/backends do
not enforce borrowing; ecosystem tools may reuse or approximate compiler facts.

---

## What Borrowck Proves

The checks are whole-function static analyses over MIR. The reader already knows
liveness and dataflow; the Rust-specific part is what facts are being made
illegal at each program point.

| Rule enforced | Rust meaning | Typical error |
|---------------|--------------|---------------|
| No dangling references | A reference's region must stay within the referent's valid points | lifetime errors, E0597 family |
| Aliasing XOR mutability | Shared reads and exclusive writes cannot overlap incompatibly | E0502, E0499 |
| No use after move | A moved place is uninitialized until reinitialized | E0382 |
| No move while borrowed | Ownership transfer cannot invalidate an active loan | E0505 |
| Drop safety | Values are dropped only when initialized and not illegally borrowed | interacts with [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) |

```
program point p
   |
   +--> region facts: which references must be valid here?
   +--> loan facts: which borrows are live here?
   +--> move facts: which places may be initialized here?
   +--> access: read / write / move / drop
          |
          v
      accept or reject this access at p
```

This is not a runtime reference-count protocol. It is compile-time proof over a
single MIR body plus type information at its boundaries.

---

## NLL Regions Are CFG Point Sets

Non-Lexical Lifetimes, stabilized after RFC 2094, changed the working model from
"a lifetime is a lexical block" to "a region is a set of MIR program points
where a value must remain valid." That is the decisive shift. Borrows end at
last use, not at the syntactic end of a block.

```
+------------------------------ lexical block ------------------------------+
| let r = &s;                                                               |
| use(r);           last use of r                                           |
| s.push('x');       legal under NLL if no later use of r                   |
+---------------------------------------------------------------------------+
       ^------------- region point-set for r ------------^
                     not the whole lexical block
```

| Model | Region approximation | Result |
|-------|----------------------|--------|
| Pre-NLL lexical | Scope from borrow to block end | Rejected many sound programs |
| NLL | CFG points where the reference can still be used | Accepts borrows ending at last use |
| Polonius direction | More location-sensitive loan reasoning | Intended to accept more sound cases |

```rust
fn nll_ok() {
    let mut s = String::from("rust");
    let r = &s;
    println!("{r}");  // last use of the shared borrow
    s.push('c');      // accepted: the borrow is no longer live
}
```

The important bridge is to ordinary liveness: a lifetime is no longer a source
span painted over text; it is a solved set of CFG points.

---

## The MIR Borrowck Pipeline

The implementation pipeline is internal, but its conceptual stages are stable
enough to reason about. Query names, data structures, and ordering details can
change between rustc releases.

```
+------------------+     +------------------+     +------------------+
| liveness         | --> | constraints      | --> | region inference |
| uses at points   |     | 'a: 'b, p in 'r  |     | point-set solve  |
+------------------+     +------------------+     +------------------+
                                                          |
                                                          v
+------------------+     +------------------+     +------------------+
| move/init flow   | --> | loan conflicts   | --> | diagnostics      |
| MaybeInit facts  |     | access vs loan   |     | spans/errors     |
+------------------+     +------------------+     +------------------+
```

| Stage | What rustc computes | Why it matters |
|-------|---------------------|----------------|
| Liveness | Where values and references are still used | Limits region point-sets to useful points |
| Constraint generation | Outlives constraints from typeck and MIR operations | Connects types, borrows, and control flow |
| Region inference | Propagates constraints until each region has a point-set | Decides how long references must be valid |
| Loan checking | Tests each read/write/move/drop against live loans | Enforces aliasing XOR mutability |
| Move tracking | Dataflow over initialized/uninitialized places | Catches use-after-move and informs drops |
| Diagnostics | Explains the failed proof in source terms | Feeds [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md) |

Subtyping matters because it generates outlives constraints. A coercion that
requires `'a: 'b` becomes part of the same propagation problem as a borrow in a
basic block.

---

## Loans, Moves, and Two-Phase Borrows

A borrow creates a loan; the checker determines where that loan is live and what
accesses conflict with it. Moves are a parallel dataflow story over places. The
two meet at drops: a moved-out value should not be dropped, but a borrowed value
also cannot be moved out from under the borrower.

```
ordinary mutable borrow:       reserve + activate immediately
   &mut x -------------------> exclusive loan live

two-phase borrow:
   reservation --------------> shared-like window ----> activation
   v.push(        v.len()     )                         mutation occurs here
```

| Mechanism | Purpose | Example |
|-----------|---------|---------|
| Shared loan | Allows reads, blocks conflicting mutation | `let r = &v;` |
| Mutable loan | Exclusive access once active | `let r = &mut v;` |
| Two-phase borrow | Permits reservation before evaluating arguments | `v.push(v.len())` |
| Move path | Tracks ownership transfer at place granularity | `let y = x; use(x)` is E0382 |
| Drop interaction | Keeps destructor paths sound after moves | drop elaboration in [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) |

Two-phase borrows are not a general relaxation of exclusivity. They are a
carefully delimited reservation/activation model for method-call patterns that
would otherwise be rejected despite being sound.

---

## Diagnostics and Concrete Traces

Borrowck is also where Rust's signature error quality is earned. The compiler
has region and loan facts, so it can say where a borrow occurs, where the
conflicting access occurs, and where the earlier borrow is later used.

| Diagnostic | Meaning | Useful command |
|------------|---------|----------------|
| E0502 | cannot borrow as mutable because also borrowed as immutable | `rustc --explain E0502` |
| E0499 | cannot borrow as mutable more than once | `rustc --explain E0499` |
| E0382 | use of moved value | `rustc --explain E0382` |
| E0505 | cannot move out because it is borrowed | `rustc --explain E0505` |

```rust
fn bad() {
    let mut s = String::from("rust");
    let r = &s;
    s.push('c');      // conflicting mutable borrow
    println!("{r}");  // shared borrow later used here
}
```

```text
$ rustc src\main.rs
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src\main.rs:4:5
  |
3 |     let r = &s;
  |             -- immutable borrow occurs here
4 |     s.push('c');
  |     ^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{r}");
  |               --- immutable borrow later used here

$ rustc --explain E0502
$ rustc +nightly -Z dump-mir=all src\main.rs     # UNSTABLE internal MIR
```

Exact wording and spans evolve, but the error's meaning follows the stable
borrowing rules.

---

## Polonius

Polonius is a next-generation formulation of borrow checking: more explicitly
location-sensitive, Datalog-shaped, and centered on loan liveness facts. It is
not the default borrow checker as of this writing. It is experimental, available
only through unstable nightly options, and its status is version-sensitive.

The original Polonius work used an external Datalog engine; the current in-tree
`next` mode is an evolving rustc implementation and should not be assumed to
match that engine one-for-one. On rustc 1.99.0-nightly,
`-Zpolonius=legacy|next|off` selects a mode. Ordinary compilation defaults to no
Polonius; bare `-Zpolonius` selects `legacy`, while the newer path is requested
explicitly with `-Zpolonius=next`.

```
+----------------------+      +----------------------+      +----------------+
| MIR facts            | ---> | Datalog-style rules  | ---> | invalid loans  |
| origins, loans, cfg  |      | reachability/liveness|      | diagnostics    |
+----------------------+      +----------------------+      +----------------+
```

| Point | Current guidance |
|-------|------------------|
| Goal | Accept more sound programs the current NLL analysis rejects |
| Interface | `rustc +nightly --crate-type=lib -Z polonius=next src\lib.rs` - EXPERIMENTAL/UNSTABLE |
| Default? | No; do not claim it ships as the default checker |
| Contract? | None for internal facts or output |
| Strategic meaning | A research-to-production path for more precise borrow reasoning |

Read Polonius as a direction of travel, not as today's stable semantics.

---

## Old World -> New World Bridge

| Old world | Rust borrowck analogue | Difference that matters |
|-----------|------------------------|-------------------------|
| Dataflow liveness | NLL region point-sets | The liveness fact is a memory-safety obligation, not just an optimization input |
| Advanced static analyzer | MIR borrowck | It is part of the compiler's acceptance rule, not an advisory warning |
| Cyclone/region systems | Rust lifetimes and regions | Rust integrates ownership, moves, traits, and RAII into mainstream code |
| .NET/JVM GC safety | Rust static proof | No GC is needed to prevent use-after-free in safe Rust |
| Race detection tools | `Send`/`Sync` plus borrowing discipline | Many data races are ruled out statically before codegen |

The closest compiler-theory phrase is "a flow-sensitive, type-informed,
whole-body static analysis enforcing an ownership discipline." That is more
accurate than calling borrowck a fancy scope checker.

---

## Stability Boundary

| Stable contract | Internal / experimental |
|-----------------|-------------------------|
| Borrowing and lifetime rules in the language | Region inference data structures |
| Move semantics and drop semantics | Loan graph/dataflow representation |
| The accepted meaning of E0382/E0499/E0502/E0505 | Exact diagnostics wording and span selection |
| Non-lexical lifetime behavior as language reality | MIR point numbering and dumps |
| Future compilers may accept more sound programs | Polonius flags, facts, and algorithms |

The implementation may get more precise. It must not make previously accepted
safe code invalid except through normal compatibility channels.

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|----------|--------|-----------|
| Why did a borrow end before the block ended? | NLL: the region is a CFG point-set ending at last use | language + rustc implementation |
| Why does `v.push(v.len())` work? | Two-phase mutable borrow reservation before activation | rustc implementation of stable rules |
| Where do use-after-move errors come from? | MIR move/init dataflow over places | rustc |
| Is Polonius production behavior? | No; it is experimental/nightly and version-sensitive | rustc research/implementation |
| Where should I read the formal user contract? | Rust Reference: references, lifetimes, destructors | language |
| Where should I debug compiler internals? | rustc-dev-guide and nightly MIR dumps | rustc, unstable |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "A lifetime is a lexical scope." | In NLL borrowck, a region is a set of MIR CFG points. |
| "Borrowck runs on source text." | It runs on MIR using typeck results and source spans for diagnostics. |
| "Polonius is the borrow checker now." | No. It remains experimental and not the default. |
| "A diagnostic's exact text is stable." | The rule is stable; wording and span presentation can change. |
| "Rust uses hidden runtime checks for this." | Safe Rust's core aliasing/move/reference rules are checked statically. |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| rustc-dev-guide: "MIR borrowck" | The compiler pipeline and `mir_borrowck` framing |
| rustc-dev-guide: "Region inference (NLL)" | Regions as point-sets and constraint propagation |
| rustc-dev-guide: "Two-phase borrows" | Reservation/activation model |
| rustc-dev-guide: "Polonius" | Experimental alternative formulation |
| RFC 2094 | Non-Lexical Lifetimes design rationale |
| Polonius book and `rust-lang/polonius` | Datalog/location-sensitive loan model |
| The Rust Reference: references and lifetimes | Stable language-facing rules |

*Cross-links:* [00](00-OVERVIEW.md) for the full pipeline; [07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md) for constraints flowing in; [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) for MIR and drops; [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md) for error reporting; [16](16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md) for panic/drop runtime edges.