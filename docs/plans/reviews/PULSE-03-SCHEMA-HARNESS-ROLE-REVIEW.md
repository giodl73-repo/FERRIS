# Pulse 03 Schema Harness Nine-Role Review

Date: 2026-08-12
Pulse: Platform Profile Schema Harness
Disposition: Approved for bounded implementation
Implementation authority: Test-only frozen-control harness

## Review question

May FERRIS add one dependency-free Rust integration test that executes the
frozen schema controls without creating a production parser, owner adapter,
family result, or semantic decision?

## Role dispositions

### Rust Safety Steward

**Disposition:** Approve.

The test parses inert JSON bytes, uses safe Rust, and makes no safety or
soundness claim from acceptance.

### Compiler Performance Engineer

**Disposition:** Approve.

The 4 MiB bound is a resource control, not a benchmark. No latency or
throughput claim is authorized.

### Interop Boundary Auditor

**Disposition:** Approve.

The harness checks source-location ambiguity and typed contract fields only.
It does not execute ABI, WIT, wire, native, or runtime boundaries.

### AI Assurance Skeptic

**Disposition:** Approve.

Expected classes come from the frozen public control manifest. Failures remain
visible and are not rewritten into success.

The first execution exposed a malformed-control construction defect: removing
one or two bytes may remove only trailing line-ending bytes on a CRLF
checkout. The approved corrective boundary is changing that control to remove
three bytes so the closing object delimiter is absent on LF and CRLF
checkouts.

### Ecosystem Strategist

**Disposition:** Approve.

No dependency, resolver, registry, external schema service, or owner behavior
is duplicated.

### Rust Maintainer

**Disposition:** Approve.

The change is one removable integration test over repository-local fixtures
and does not widen the CLI or library API.

### Native Platform Adopter

**Disposition:** Approve with cross-platform gate.

The same test must pass on Windows and Unix. It makes no native-tool, target,
runtime, packaging, deployment, or support claim.

### Scope Keeper

**Disposition:** Approve.

The pulse is limited to the nine controls and explicitly excludes family,
owner-command, product, generation, and held-out work.

### Validation Checker

**Disposition:** Approve.

The base case and exact expected class for every mutation are frozen. Full
workspace formatting, tests, Clippy, and Git diff validation are required.

## Decision and authority

All nine roles approve the bounded implementation. Final acceptance requires
the measured Windows and Unix results and an updated disposition.
