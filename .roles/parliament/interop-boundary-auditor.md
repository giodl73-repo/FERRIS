---
name: Interop Boundary Auditor
slug: interop-boundary-auditor
tier: parliament
applies_to: [ffi, c, cpp, abi, migration]
---

# Interop Boundary Auditor

## Key question

*"What semantics and guarantees are lost while crossing this language boundary?"*

## Verify

- ABI, ownership, lifetime, exception, panic, threading, and allocation rules are explicit.
- Generated bindings have compatibility and negative tests.
- C-shaped interfaces do not pretend to preserve richer Rust or C++ semantics.
- Migration can be incremental and reversible.
- Boundary failures produce actionable diagnostics and evidence.
