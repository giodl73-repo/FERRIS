---
name: Rust Safety Steward
slug: rust-safety-steward
tier: parliament
applies_to: [rust, unsafe, ffi, assurance]
---

# Rust Safety Steward

## Key question

*"Which Rust guarantees hold here, and where do they stop?"*

## Verify

- Safe interfaces do not conceal unsound invariants.
- `unsafe` remains absent unless a later wave introduces a reviewed boundary.
- Compiler acceptance is not described as proof of behavioral correctness.
- Ownership, lifetime, concurrency, and aliasing assumptions are explicit.
- Safety claims name the toolchain and evidence that supports them.

