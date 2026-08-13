# Pulse 03 Schema Harness Nine-Role Review

Date: 2026-08-12
Pulse: Platform Profile Schema Harness
Disposition: Accepted after Windows and Unix validation
Implementation authority: Test-only frozen-control harness

## Review question

May FERRIS add one dependency-free Rust integration test that executes the
frozen schema controls without creating a production parser, owner adapter,
family result, or semantic decision?

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

The test parses inert JSON bytes, uses safe Rust, and makes no safety or
soundness claim from acceptance.

### Compiler Performance Engineer

**Disposition:** Accept.

The 4 MiB bound is a resource control, not a benchmark. No latency or
throughput claim is authorized.

### Interop Boundary Auditor

**Disposition:** Accept.

The harness checks source-location ambiguity and typed contract fields only.
It does not execute ABI, WIT, wire, native, or runtime boundaries.

### AI Assurance Skeptic

**Disposition:** Accept.

Expected classes come from the frozen public control manifest. Failures remain
visible and are not rewritten into success.

The first execution exposed a malformed-control construction defect: removing
one or two bytes may remove only trailing line-ending bytes on a CRLF
checkout. The approved corrective boundary is changing that control to remove
three bytes so the closing object delimiter is absent on LF and CRLF
checkouts.

### Ecosystem Strategist

**Disposition:** Accept.

No dependency, resolver, registry, external schema service, or owner behavior
is duplicated.

### Rust Maintainer

**Disposition:** Accept.

The change is one removable integration test over repository-local fixtures
and does not widen the CLI or library API.

### Native Platform Adopter

**Disposition:** Accept.

The same test passes on Windows and Ubuntu 24.04.4 WSL2 with Rust 1.95. It
makes no native-tool, target, runtime, packaging, deployment, or support
claim.

### Scope Keeper

**Disposition:** Accept.

The pulse is limited to the nine controls and explicitly excludes family,
owner-command, product, generation, and held-out work.

### Validation Checker

**Disposition:** Accept.

The base case and exact expected class for every mutation are frozen. Full
workspace formatting, tests, Clippy, and Git diff validation are required.

## Measured result

At cutoff `80ce90b332ca8e649d1b5bfd013da272934e9089`, Windows and Unix each
reported 67 passing tests, 2 ignored bounded-command helpers, and 0 failures.
All nine schema controls produced their exact expected class. Formatting and
Clippy with warnings denied passed on both platforms; Windows Git diff
validation passed.

## Decision and authority

All nine roles accept the measured test-only harness. The review grants no
production parser, owner adapter, completed family, support, held-out, or
PLATFORM-001 status authority.
