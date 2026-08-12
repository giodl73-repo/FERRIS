---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:safe-rust-unsafe-obligations-and-assurance-ownership
kind: guide
module: rust-security-assurance
section: security-engineering
title: Safe Rust, Unsafe Obligations, and Assurance Ownership
status: source-custody
source_custody: partial
current_path: rust-security-assurance/02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md
canonical_path: rust-security-assurance/02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md
backsource_ids: [mdloom-backfill:rust-security-assurance:02-safe-rust-unsafe-obligations-and-assurance-ownership]
concepts: [safe rust, unsafe rust, soundness, invariants, assurance ownership]
root_concepts: [safe rust]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Safe Rust, Unsafe Obligations, and Assurance Ownership

Safe Rust is a strong language-level assurance boundary, not a process
containment boundary. Well-formed safe code cannot directly perform operations
the language classifies as unsafe, and the compiler enforces ownership,
borrowing, and data-race rules. The claim depends on the compiler and every
reachable unsafe implementation upholding their contracts. Assurance work
assigns those obligations instead of hiding them behind "written in Rust."

## The Big Picture

```
+============================================================================+
|                        ASSURANCE OWNERSHIP STACK                           |
+============================================================================+
| PRODUCT OWNER: assets, authorization, abuse resistance, availability       |
+----------------------------------------------------------------------------+
| SAFE-API OWNER: API makes invalid states/operations hard or impossible     |
+----------------------------------------------------------------------------+
| UNSAFE OWNER: proves validity, aliasing, lifetime, concurrency invariants  |
+----------------------------------------------------------------------------+
| DEPENDENCY OWNER: selects versions/features; monitors soundness/advisories |
+----------------------------------------------------------------------------+
| BUILD/PLATFORM OWNER: toolchain, native code, runner, OS, deployment       |
+============================================================================+
```

No layer can delegate its claim to the layer below. A sound allocator does not
make an authorization check correct; a perfect authorization check does not
make a dangling FFI pointer valid.

## What Safe Rust Does and Does Not Establish

| Property | Safe Rust contribution | Remaining qualification |
|----------|------------------------|-------------------------|
| Use-after-free and invalid references | Safe code cannot construct these through sound safe APIs | Unsound `unsafe`, FFI, compiler defects, or platform faults can violate the premise |
| Data races | Safe Rust prevents data races through `Send`/`Sync` and borrowing | Deadlock, starvation, races at the application/protocol level remain |
| Bounds | Standard indexing checks bounds; many APIs return `Option` | Allocation size, integer logic, and algorithmic complexity remain |
| Type validity | Safe constructors preserve type invariants | Deserialization semantics and unsafe construction still require review |
| Confidentiality/integrity | Types can encode authority and secret handling | No automatic authorization, encryption, side-channel, or logging guarantee |
| Availability | A panic follows the configured unwind/abort runtime behavior rather than itself being UB | Panic-driven outage, abort, OOM, unbounded queues, and slow operations remain |

The precise claim is conditional: safe code enjoys the language guarantees **if
the compiler, standard library, dependencies, and platform uphold the contracts
on which that safe code relies**.

## Unsafe Is a Universal Obligation

An `unsafe` block says the compiler cannot prove some required fact. The author
must make the missing facts explicit:

```
+============================================================================+
|                      UNSAFE PROOF OBLIGATION                               |
+============================================================================+
| safe caller inputs                                                         |
|        |                                                                   |
|        v                                                                   |
| establish preconditions: length, alignment, provenance, initialization,    |
| aliasing, lifetime, thread access, ABI, and panic/cancellation behavior    |
|        |                                                                   |
|        v                                                                   |
| perform the smallest required unsafe operation                             |
|        |                                                                   |
|        v                                                                   |
| restore postconditions: valid safe values, unambiguous ownership, and      |
| invariants preserved on success, error, panic, cancellation, and drop      |
+============================================================================+
```

For each unsafe region, record:

- the exact operation requiring `unsafe`;
- every precondition, including target/ABI and concurrency conditions;
- why surrounding safe code establishes them;
- what remains true on error, panic, cancellation, and drop;
- how tests and analysis exercise the boundary;
- the accountable reviewer and re-review triggers.

Use a `// SAFETY:` comment adjacent to the operation. It should contain the
argument, not "this is safe because we need it."

## Minimize and Inventory the Boundary

For a crate that should contain no unsafe code, make the policy executable at
the crate root:

```rust
#![forbid(unsafe_code)]
```

This applies to that crate, not to transitive dependencies. From a repository
root with `rg` installed, a first-pass source inventory is:

```text
rg -n "\bunsafe\b" --glob "*.rs" .
```

This misses generated code, expanded macros, and dependencies. An optional
ecosystem audit can use `cargo-geiger`; pin and validate the tool in your own CI
before treating its output as evidence. Inventory proc macros, build scripts,
native code, and activated features separately.

| Pattern | Assurance posture |
|---------|-------------------|
| Pure safe application crate | `forbid(unsafe_code)` where practical; focus on logic, input, and availability |
| Safe wrapper over unsafe core | Tiny internal boundary, invariant-focused API, independent review |
| FFI crate | Explicit ABI/ownership contract, platform matrix, native tests and containment |
| Unsafe trait implementation | Document why every implementer/user invariant holds across threads and drop |
| Performance optimization | Require measured need and a safe reference implementation for differential tests |

## Assurance Ownership Is More Than Code Ownership

```
CHANGE                         REQUIRED APPROVER
new unsafe block ------------> unsafe-invariant owner
new dependency/features -----> dependency/supply-chain owner
new parser/protocol ----------> product security + domain owner
new native target ------------> platform/FFI owner
release exception -----------> named risk acceptor, not the implementer alone
```

Use `CODEOWNERS`, review policy, or an equivalent mechanism to route changes,
but keep the assurance record in a durable, reviewable form. A team alias is
useful only if someone is accountable for response and expiry.

## Failure and Panic Are Part of Soundness

Unsafe code must remain correct when constructors return early, destructors run
during unwinding, futures are cancelled, callbacks re-enter, or allocation
fails. Rust's safe type system does not automatically make an unsafe operation
exception-safe or cancellation-safe.

| Edge | Review question |
|------|-----------------|
| Partial initialization | Can drop observe an uninitialized field? |
| Panic/unwind | Is an invariant temporarily broken across a possible panic? |
| Cancellation | Can a future be dropped after external state changed but before local commit? |
| Re-entrancy | Can a callback alias or mutate state assumed exclusive? |
| OOM/abort | Does the product claim recovery that the process model cannot provide? |

## Old World -> New World Bridge

| Old world | Rust assurance |
|-----------|----------------|
| C/C++ coding standard bans dangerous APIs | Safe subset enforced by the language, plus reviewed unsafe islands |
| `reinterpret_cast` review | `unsafe` operation with explicit invariant and safe wrapper |
| Managed-code P/Invoke boundary | Rust FFI boundary: native ownership and ABI still manual |
| Static analyzer suppressions | `// SAFETY:` argument plus scoped evidence and owner |
| Security champion sign-off | Named owner per claim class, with re-review triggers |

For Microsoft-oriented teams, SDL-required security review and GitHub
`CODEOWNERS`/branch protection can enforce routing. They do not establish
soundness; the invariant argument and evidence do.

## Common Confusion Points

- **"`unsafe` means the block itself may do anything."** The same language
  validity rules still apply; `unsafe` only permits specific unchecked
  operations.
- **"No unsafe keyword means no unsafe dependency."** Safe APIs frequently wrap
  unsafe internals, including the standard library.
- **"A SAFETY comment is proof."** It is a reviewable argument; evidence and
  independent challenge are still needed.
- **"Memory safe means secure."** Logic, availability, supply chain, secrets,
  side channels, and platform risks are separate.
- **"The compiler approved it."** The compiler checks that unsafe syntax is
  properly marked, not that its semantic preconditions are true.

## Decision Cheat Sheet

| Situation | Required action |
|-----------|-----------------|
| Crate needs no unsafe | Add `#![forbid(unsafe_code)]` and keep dependency review |
| Unsafe proposed for speed | Require benchmark, safe baseline, invariant document, targeted tests |
| Safe wrapper exposes raw ownership | Redesign until safe callers cannot violate lifetime/free rules |
| Dependency contains unsafe | Identify critical surface, advisories, maintainer posture, and evidence |
| Unsafe trait implementation | Review global thread/aliasing obligations, not only local syntax |
| Owner is unavailable | Block high-risk change or transfer ownership explicitly |
| Claim spans code and platform | Split responsibilities and record shared assumptions |

## Primary Sources

- Rust Reference, Unsafety: https://doc.rust-lang.org/reference/unsafe-keyword.html
- Rustonomicon: https://doc.rust-lang.org/nomicon/
- Rust API Guidelines, Dependability:
  https://rust-lang.github.io/api-guidelines/dependability.html
- Clippy lint documentation: https://rust-lang.github.io/rust-clippy/master/
- Cargo Geiger repository: https://github.com/geiger-rs/cargo-geiger

## Related Guides

- Previous: [01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md](01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md)
- Next: [03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md](03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md)
- Language mechanics: [../rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md](../rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md)
