# Pulse 19 Process-Exit Diagnostic Nine-Role Pre-Implementation Review

Date: 2026-08-13
Pulse: Platform Profile Conformance Pulse 19
Disposition: Approved for the bounded diagnostic harness only; unimplemented
Implementation authority: New public/development fixtures and test-only
diagnosis only

## Review question

May FERRIS add one test-only harness over newly authored public fixtures to
localize the valid `process-exit-agreement` failure to core classification,
envelope construction, CLI emission/`ExitCode`, format parity, or no
reproduction, without accessing hidden material, retrying the closed fixture,
fixing product behavior, or advancing PLATFORM-001?

## Role dispositions

### Rust Safety Steward

**Disposition:** Approve the bounded diagnostic harness only.

The authorized work is safe-Rust test code over explicit local inputs.
Compiler or test success creates no safety, soundness, or correctness claim.
No production or `unsafe` change is approved.

### Compiler Performance Engineer

**Disposition:** Approve the bounded diagnostic harness only.

Input, change, process, and stream bounds are conformance controls, not
benchmarks. No timing, throughput, cache, or performance claim is approved.

### Interop Boundary Auditor

**Disposition:** Approve the bounded diagnostic harness only.

The only boundary under diagnosis is the existing process exit and
stdout/stderr contract. No ABI, WIT, native-library, network, provider, or
deployment boundary is approved.

### AI Assurance Skeptic

**Disposition:** Approve the bounded diagnostic harness only.

The valid Pulse 17 failure remains visible. The harness uses declared public
expectations, records mismatches without normalization, and prohibits hidden
case inference, quarantine access, retry, rescore, or conversion to a pass.

### Ecosystem Strategist

**Disposition:** Approve the bounded diagnostic harness only.

The work adds no resolver, registry, owner adapter, repository modification,
support statement, or alternative source of Cargo truth. It diagnoses the
existing local command contract only.

### Rust Maintainer

**Disposition:** Approve the bounded diagnostic harness only.

One removable integration harness and adjacent public fixtures are
maintainable and reviewable. No CLI behavior, output contract, dependency,
visibility, or public API change is approved.

### Native Platform Adopter

**Disposition:** Approve the bounded diagnostic harness only.

Windows x86-64 and Ubuntu 24.04.4 WSL2 must run the same cutoff and fixture
revision. The results remain development evidence, not native Linux support,
compatibility, certification, operations, or deployment evidence.

### Scope Keeper

**Disposition:** Approve the bounded diagnostic harness only.

Authority ends at one of five localization outcomes. A fix, product behavior
change, hidden-material action, or PLATFORM-001 advancement requires separate
authority; any fix requires an approved Pulse 20.

### Validation Checker

**Disposition:** Approve the bounded diagnostic harness only.

The review requires 23 unique public input branches, 26 exactly-once CLI
processes per platform, all six public result classes, diagnostic-class
agreement, JSON routing, three human/JSON pairs, exact evidence fields, and
zero missing, duplicate, retried, or extra rows.

## Remaining gates

- The diagnostic harness, public fixtures, and validation receipt do not yet
  exist.
- Pulse 19 implementation must pass Windows and Ubuntu development validation
  at one immutable cutoff.
- A localized mismatch must stop without repair; `no reproduction` must not be
  promoted into a held-out pass.
- Fixture `P17-R3-D6B553CBC3B1240B673B8190` remains permanently closed.
- Any fix requires a separately reviewed and approved Pulse 20.
- The independent RUNE v1 blocker remains open and PLATFORM-001 remains Draft.

## Decision and authority

All nine roles approve only the bounded public/development diagnostic harness
defined by
[Pulse 19](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-19.md).
No role approves a fix, product behavior change, hidden-material access,
retry, rescore, support claim, held-out claim, or specification advancement.
