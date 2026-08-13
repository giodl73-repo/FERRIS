# Pulse 06 Hosted Service Nine-Role Review

Date: 2026-08-12
Disposition: Accepted on measured Windows and Unix evidence
Implementation authority: Controlled in-process hosted-service fixtures

## Review question

May FERRIS complete one hosted-service family with an in-process owner
boundary, explicit readiness and unavailable behavior, complete profile
states, and no network or deployment authority?

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Approve | Safe Rust and typed states; tests are not general safety proof |
| Compiler Performance Engineer | Approve | Isolated targets; no latency or throughput claim |
| Interop Boundary Auditor | Approve | Request, response, cancellation, and state are explicit; no ABI or wire claim |
| AI Assurance Skeptic | Approve | Exact commands, failures, sources, digests, and limitations |
| Ecosystem Strategist | Approve | Standard library and Cargo only; no service framework replacement |
| Rust Maintainer | Approve | Small owner API, deterministic diagnostics, removable fixtures |
| Native Platform Adopter | Approve with platform gate | Windows/Unix required; no deployment or operations claim |
| Scope Keeper | Approve | In-process hosted family only |
| Validation Checker | Approve | Positive, malformed, unavailable, cancellation, immutability, and cross-platform gates |

## Measured disposition

The exact implementation cutoff
`de5b5242a26ed5ce15d1dae2d3ec333a3a7663d2` passed the full workspace,
owner-command, immutability, and profile-digest gates on Windows build 26310
and Ubuntu 24.04.4 WSL2 with Rust/Cargo 1.95.0. The nine roles accept the
controlled family within the original boundary. The result does not widen
into network, deployment, performance, security, support, approval, another
family, held-out evidence, or PLATFORM-001 status.
