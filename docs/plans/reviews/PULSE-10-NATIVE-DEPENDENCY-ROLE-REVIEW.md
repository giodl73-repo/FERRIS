# Pulse 10 Native Dependency Nine-Role Review

Date: 2026-08-13
Disposition: Accepted on measured Windows and Unix evidence
Implementation authority: Exact Windows kernel32 and Unix libc process APIs

## Review question

May FERRIS complete one system-native family through minimal conditional FFI
while recording the actual linker/provider boundary and avoiding a claim that
the Cargo graph owns ambient native installation, patching, or ABI?

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Approve with unsafe boundary | Exact declarations, no pointers or caller memory, safe wrappers |
| Compiler Performance Engineer | Approve | No latency, size, or build-speed claim |
| Interop Boundary Auditor | Approve | OS-specific ABI and linker names remain explicit |
| AI Assurance Skeptic | Approve | Native identity, ownership, commands, and limitations are retained |
| Ecosystem Strategist | Approve | Ordinary OS APIs; no FFI framework or resolver replacement |
| Rust Maintainer | Approve | Tiny conditional modules and deterministic tests |
| Native Platform Adopter | Approve | Actual Windows and Unix execution required |
| Scope Keeper | Approve | Process-identity native family only |
| Validation Checker | Approve | Compile, link, execute, immutability, digest, and cross-platform gates |

## Measured disposition

Cutoff `41b7086cb43bc6b9a37b7ba5920cfdec39950f4a` passed exact declaration,
link, execution, immutability, digest, and repository gates on both required
development hosts. The roles accept the bounded family without widening FFI,
native ownership, deployment, support, held-out, or PLATFORM-001 status.
