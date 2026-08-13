# Pulse 18 Nine-Role Immutability Review

Date: 2026-08-12
Pulse: Profile Diff Filesystem Immutability
Disposition: Accepted as bounded development evidence
Implementation authority: Conformance test and validation evidence only

## Review question

Does the new public-CLI test establish a precise, cross-platform
non-mutation claim for the explicit profile inputs and isolated working
directory without overstating sandboxing, removal, support, or held-out
evidence?

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

No product Rust or `unsafe` code changed. The test checks observable file
effects and does not promote compiler acceptance into behavioral safety.

### Compiler Performance Engineer

**Disposition:** Accept.

The test adds no performance claim. Its temporary directory and Unix target
directory are isolated, and timing is not compared or interpreted.

### Interop Boundary Auditor

**Disposition:** Accept.

The same process and filesystem assertions pass on Windows and Unix. The
result does not claim ABI, native-library, WIT, wire, provider, or deployment
immutability.

### AI Assurance Skeptic

**Disposition:** Accept.

Exact cutoff, commands, environments, assertions, results, and unobserved
side-effect domains are explicit. The claim is narrower than the product
contract and does not become support or held-out evidence.

### Ecosystem Strategist

**Disposition:** Accept.

The test concerns Ferris's local read-only behavior only. It creates no owner
adapter, resolver, registry, profile source, distribution, ranking, or
parallel support authority.

### Rust Maintainer

**Disposition:** Accept.

The test uses the public binary, identifies a failing family, leaves its
unique temporary directory empty before cleanup, and verifies that fixture
files and directories remain unchanged.

### Native Platform Adopter

**Disposition:** Accept with limitation.

Windows and Ubuntu 24.04.4 WSL2 both pass. This is useful development evidence
but not a complete filesystem, native Linux, installation, deployment, or
operations audit.

### Scope Keeper

**Disposition:** Accept.

The pulse adds one test and evidence only. Product authority, development
fixtures, hidden evaluation, profile generation, owner execution, and
specification status remain unchanged.

### Validation Checker

**Disposition:** Accept.

The test covers all nine family pairs, byte equality, length, modification
time, directory membership, empty current directory, process exit, and stream
placement. Both platform suites executed 64 passing tests with 2 ignored
helpers and no failures.

## Remaining gates

- The test is not a syscall, sandbox, registry, network, or whole-machine
  audit.
- Access times and locations outside the bounded directories are not observed.
- Ordinary Cargo preservation in a consumer repository is not exercised.
- No PRODUCT-001 Removal Record or PLATFORM-001 lifecycle gate is complete.
- The independent Pulse 16 held-out package and score remain outstanding.
- No support, compatibility, security, freshness, approval, certification, or
  production claim is authorized.

## Decision and authority

All nine roles accept the bounded filesystem immutability evidence. The review
grants no new product, removal, support, or held-out authority.
