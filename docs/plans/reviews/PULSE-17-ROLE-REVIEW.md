# Pulse 17 Nine-Role Validation Review

Date: 2026-08-12
Pulse: Cross-Platform Profile Diff Development Validation
Disposition: Accepted as development evidence
Implementation authority: Validation evidence only

## Review question

Do the Windows and Unix runs establish honest cross-platform development
conformance for the existing Ferris workspace and nine-family profile-diff
matrix without becoming support, owner, held-out, or PLATFORM-001 evidence?

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

No Rust or `unsafe` code changed. Compiler and test acceptance remains
development evidence rather than a behavioral safety proof.

### Compiler Performance Engineer

**Disposition:** Accept.

The receipt records environments and isolated Unix build output but makes no
timing, cache, throughput, or performance comparison.

### Interop Boundary Auditor

**Disposition:** Accept.

The same JSON and CLI boundaries execute on Windows and Unix. WSL execution
does not claim ABI, WIT, wire, native-library, provider, or deployment
conformance.

### AI Assurance Skeptic

**Disposition:** Accept.

Exact cutoff, commands, environments, test counts, initial line-ending
observation, and limitations are retained. The evidence is not promoted into
support, compatibility, or held-out success.

### Ecosystem Strategist

**Disposition:** Accept.

The runs exercise Ferris's local development suite only. They introduce no
resolver, profile source, owner adapter, registry, distribution, ranking, or
parallel support authority.

### Rust Maintainer

**Disposition:** Accept.

The commands are reproducible, use the repository's recorded toolchain, and
preserve the Windows checkout. Unix artifacts use an isolated target
directory, so cross-platform validation does not contaminate ordinary local
build output.

### Native Platform Adopter

**Disposition:** Accept with explicit limitation.

Ubuntu 24.04.4 WSL2 is valid Unix development evidence but not universal or
native Linux support. Real native tools, targets, deployment, recovery, and
operations remain untested.

### Scope Keeper

**Disposition:** Accept.

The pulse records validation only. Product behavior, fixtures, hidden
evaluation material, profile generation, owner execution, and specification
status remain unchanged.

### Validation Checker

**Disposition:** Accept.

Both platforms use one source cutoff and Rust/Cargo 1.95.0. Each executed 63
passing tests with 2 ignored helpers and no failures; formatting and Clippy
passed on both, and checkout diff hygiene passed under Windows Git.

## Remaining gates

- This evidence is development, not held-out.
- WSL2 is not native Linux hardware or broad Unix support.
- The independent Pulse 16 package, preflight, cutoff, 112-process
  collection, scorer conformance, and first score do not exist.
- No compatibility, support, security, freshness, approval, certification,
  production, or PLATFORM-001 claim is authorized.

## Decision and authority

All nine roles accept Pulse 17 as cross-platform development evidence only.
It grants no new product or evaluation authority.
