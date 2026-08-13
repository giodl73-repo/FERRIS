# Pulse 06 Hosted Service Nine-Role Review

Date: 2026-08-12
Disposition: Approved for bounded implementation
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

## Decision and authority

The nine roles authorize the bounded implementation. Final acceptance requires
measured Windows and Unix evidence and an updated disposition.
