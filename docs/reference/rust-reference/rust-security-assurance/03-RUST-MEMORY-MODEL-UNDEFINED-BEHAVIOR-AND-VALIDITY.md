---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:rust-memory-model-undefined-behavior-and-validity
kind: guide
module: rust-security-assurance
section: security-engineering
title: Rust Memory Model, Undefined Behavior, and Validity
status: source-custody
source_custody: partial
current_path: rust-security-assurance/03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md
canonical_path: rust-security-assurance/03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md
backsource_ids: [proof-backfill:rust-security-assurance:03-rust-memory-model-undefined-behavior-and-validity]
concepts: [rust memory model, undefined behavior, validity, provenance, aliasing, initialization]
root_concepts: [undefined behavior]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Rust Memory Model, Undefined Behavior, and Validity

Unsafe Rust is reviewed against the Rust abstract machine, not against "what
worked in C" or "what this CPU happens to tolerate." The Reference defines
important undefined-behavior and validity rules, while parts of aliasing and
pointer provenance continue to be refined. Miri operationalizes a current model
for testing; its models are high-value diagnostics, not the entire normative
language definition.

## The Big Picture

```
+============================================================================+
|                       TYPED MEMORY ACCESS CONTRACT                         |
+============================================================================+
| allocation identity + provenance                                           |
|        |                                                                   |
|        v                                                                   |
| pointer/reference: live? in bounds? aligned? permitted to access?          |
|        |                                                                   |
|        v                                                                   |
| bytes initialized for this operation?                                      |
|        |                                                                   |
|        v                                                                   |
| bit pattern valid for T?  aliasing/exclusivity respected?                  |
|        |                                                                   |
|        v                                                                   |
| operation and concurrency ordering satisfy their contracts?                |
+----------------------------------------------------------------------------+
| Violate a required condition in an executed path -> undefined behavior     |
+============================================================================+
```

The optimizer may assume UB never occurs. A defect can therefore manifest far
from the unsafe line, vary by optimization level, or disappear under logging.

## Validity Is Type-Specific

| Type/value | Validity condition |
|------------|--------------------|
| `bool` | only the defined `false`/`true` representations |
| `char` | valid Unicode scalar value |
| reference `&T` / `&mut T` | non-null, aligned, dereferenceable for required extent, valid pointee, correct aliasing/lifetime |
| `NonZeroU32` | not zero |
| enum | a valid discriminant and valid active payload |
| function pointer | valid function address with compatible ABI/signature |
| integer | every bit pattern is generally a valid integer value of that width |

Do not turn arbitrary bytes into `T` with `transmute`, pointer casts, or
`assume_init` unless every bit pattern and invariant is established. Padding can
exist without being semantically initialized as a field; copying bytes and
reading them as typed values are different operations.

## Initialization: Prefer Construction Over Repair

Use safe constructors where possible:

```rust
let squares: [u64; 8] = std::array::from_fn(|i| (i * i) as u64);
assert_eq!(squares[3], 9);
```

`MaybeUninit<T>` is the tool for deliberately uninitialized storage. Its name
does not relax `T`'s validity rules: calling `assume_init` transfers the proof
obligation to you.

```
MaybeUninit<T>
   |
   +-- write a valid T to every required element/field
   +-- track partial initialization for every exit path
   +-- drop only initialized values
   +-- call assume_init exactly when the whole T is valid
```

For arrays and collections, prefer stable safe APIs such as
`std::array::from_fn`, iterators, `Vec::with_capacity` plus safe `push`, or a
well-reviewed abstraction rather than reproducing partial-initialization logic.

## References Are Stronger Than Raw Pointers

Creating `&T` or `&mut T` asserts more than non-nullness. The referent must be
valid and the access/aliasing contract must hold for the reference's use. A raw
pointer carries fewer compiler-enforced guarantees, but dereferencing it or
turning it into a reference still requires all relevant conditions.

```
integer address          raw pointer                 reference
weak identity info  ->   provenance + address   ->   validity + alias contract
                         (model-sensitive)            (strong language promise)
```

Strict-provenance APIs and current Unsafe Code Guidelines work help express
intent, but pointer-provenance details are an evolving area. Avoid integer
round-trips and fabricated references unless an official API explicitly
supports the operation and your supported compiler/target matrix validates it.
The Unsafe Code Guidelines material is design guidance rather than a complete
normative specification; distinguish it from guarantees stated by the Rust
Reference and stable API documentation.

## Aliasing and Interior Mutability

The practical rule remains "shared or mutable": an `&mut T` carries an
exclusivity promise for its relevant use; mutation reachable through `&T`
generally requires `UnsafeCell`. The exact aliasing model continues to evolve,
so audit against current Reference, standard-library, and supported-toolchain
contracts rather than treating a slogan as the full rule. Types such as `Cell`,
`RefCell`, `Mutex`, and atomics build sound interfaces over `UnsafeCell`.

| Need | Correct mechanism | Wrong shortcut |
|------|-------------------|----------------|
| Single-thread checked interior mutation | `Cell` / `RefCell` | cast `&T` to `&mut T` |
| Cross-thread mutation | `Mutex`, `RwLock`, atomics | unsynchronized raw writes |
| Shared immutable view | `&T` | mutate behind it without `UnsafeCell` |
| Unique mutation | `&mut T` | keep hidden aliases that are later used |

`UnsafeCell` permits interior mutation; it does not waive data-race, lifetime,
or validity rules.

## Concurrency and Atomics

Safe Rust prevents data races, not all races. Unsafe shared-memory code must
choose atomic operations and orderings that establish the intended
happens-before relations. A data race is UB; a logically stale or reordered
protocol can be incorrect without causing UB.

```
Thread A: initialize data -> Release store flag
                                      |
                                      v synchronizes-with
Thread B: Acquire load flag -> read initialized data
```

The diagram is a pattern, not a drop-in algorithm. Prove object lifetime,
single/multiple-writer rules, ABA behavior, and failure orderings for the actual
data structure. Use Loom or a similar exploration tool for bounded schedules,
then retain the human argument.

## An Unsafe Review Worksheet

| Question | Evidence |
|----------|----------|
| Is each pointer derived from a live allocation with permitted provenance? | construction trace and API contract |
| Is every access in bounds and aligned for its type? | arithmetic proof, checked lengths, tests |
| Is the pointee initialized and valid for `T`? | constructor/state invariant |
| Are aliasing and interior mutation legal? | ownership diagram, `UnsafeCell` placement |
| Can panic, drop, cancellation, or re-entry break the invariant? | failure-path review and tests |
| Is cross-thread access synchronized? | memory-order argument and Loom/TSan where supported |
| Are ABI/layout assumptions target-specific and tested? | `repr` contract and target matrix |

## Old World -> New World Bridge

| C/C++ habit | Rust requirement |
|-------------|------------------|
| "Non-null pointer is usable" | provenance, bounds, alignment, lifetime, validity, and alias permission |
| Placement construction | `MaybeUninit` with explicit partial-init/drop accounting |
| Type punning through casts/unions | only operations permitted by Rust validity and layout rules |
| `volatile` for concurrency | atomics and synchronization; volatile is for externally observable I/O semantics |
| Sanitizer-clean means defined | dynamic evidence covers executed paths, not universal validity |

For .NET readers, the nearest analogy is the difference between verifiable IL
and unverifiable/native code: once outside the checked subset, runtime success
does not prove the metadata, lifetime, or interop contract was valid.

## Common Confusion Points

- **"The hardware allows unaligned access."** Rust still requires alignment for
  ordinary typed dereference; use APIs designed for unaligned reads/writes.
- **"A zeroed value is valid for every type."** False for references,
  `NonZero*`, many enums, and other constrained types.
- **"Raw pointers have no rules."** They have fewer static checks; operations on
  them remain governed by the abstract machine.
- **"Miri defines Rust."** Miri implements diagnostic models and catches many
  violations on covered paths; the Reference and evolving language work remain
  the authority.
- **"`UnsafeCell` makes races safe."** It permits interior mutation, not
  unsynchronized cross-thread access.
- **"Debug tests passed."** UB often changes under optimization and target
  differences; test the supported matrix and retain the invariant proof.

## Decision Cheat Sheet

| Situation | Preferred action |
|-----------|------------------|
| Need initialized fixed array | `std::array::from_fn` or safe construction |
| Need partial initialization internally | `MaybeUninit` with explicit initialized-count cleanup |
| Need read of packed/unaligned bytes | byte parsing or `read_unaligned` with all other obligations met |
| Need shared mutation | safe `Cell`/lock/atomic abstraction |
| Need pointer-to-integer round trip | avoid; use supported strict-provenance APIs and target-specific validation |
| Need lock-free structure | established crate or expert review plus Loom/sanitizer evidence |
| Unsure whether a bit pattern is valid | do not construct `T`; consult official type and Reference contracts |

## Primary Sources

- Rust Reference, Behavior considered undefined:
  https://doc.rust-lang.org/reference/behavior-considered-undefined.html
- Rust Reference, Type layout: https://doc.rust-lang.org/reference/type-layout.html
- Rustonomicon: https://doc.rust-lang.org/nomicon/
- `MaybeUninit` documentation: https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
- Rust Unsafe Code Guidelines: https://rust-lang.github.io/unsafe-code-guidelines/
- Strict provenance documentation:
  https://doc.rust-lang.org/std/ptr/index.html#strict-provenance
- Miri: https://github.com/rust-lang/miri

## Related Guides

- Previous: [02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md](02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md)
- Next: [04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md](04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md)
- Targeted evidence: [11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md](11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md)
