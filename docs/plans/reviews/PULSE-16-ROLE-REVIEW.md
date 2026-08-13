# Pulse 16 Nine-Role Design Review

Date: 2026-08-12
Pulse: Profile Diff Held-Out Design
Disposition: Accepted as a public evaluation contract
Implementation authority: Documentation and independent evaluation design only

## Review question

Does the Pulse 16 design enable an independent, first-run held-out evaluation
of `profile-diff` without leaking hidden material, repeating prior harness and
scorer failures, widening product authority, or overstating what a pass would
establish?

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

The contract scores command behavior and explicitly rejects safety,
correctness, and compatibility promotion. It adds no Rust or `unsafe` code.

### Compiler Performance Engineer

**Disposition:** Accept.

No performance claim is made. Environment and process evidence are recorded,
while the score remains conformance-focused rather than a timing benchmark.

### Interop Boundary Auditor

**Disposition:** Accept.

Array, empty-container, pointer-escaping, native-family, and cross-platform
boundaries are explicit without claiming ABI, WIT, wire, or provider semantic
equivalence.

### AI Assurance Skeptic

**Disposition:** Accept.

Independent construction, sealed canaries, no source/test access, one first
score, anti-tuning rules, and public-safe release preserve the distinction
between observed evidence and implementation-team assertion.

### Ecosystem Strategist

**Disposition:** Accept.

The evaluation concerns Ferris's bounded local comparison behavior only. It
creates no profile registry, resolver, distribution, ranking, owner adapter,
or competing support authority.

### Rust Maintainer

**Disposition:** Accept.

The command and expected result classes are public and actionable. Collection,
scorer, and fixture failures remain distinct, and failed packages cannot be
recycled into favorable scores.

### Native Platform Adopter

**Disposition:** Accept.

Every case runs on Windows and Unix from the same source cutoff. Environment,
filesystem, command, binary, and output evidence are required without
claiming broader target or native-tool support.

### Scope Keeper

**Disposition:** Accept.

The pulse is documentation-only. It does not create hidden fixtures, inspect
an oracle, run a score, change product behavior, generate profiles, or advance
PLATFORM-001.

### Validation Checker

**Disposition:** Accept.

Exact 56-case and 112-process cardinality, preflight, scorer qualification,
stream capture, result classes, bounds, privacy canaries, identity checks,
cutoff, quarantine, and public-safe reporting are all explicit.

## Corrections incorporated from historical evidence

The protocol directly prevents previously observed invalidation classes:

- missing process cardinality;
- expected process declarations absent from durable records;
- prefix or selected-line JSON parsing;
- scorer parsers tied to one incidental evidence layout;
- incomplete oracle branch qualification;
- Ferris not being launched by the frozen harness; and
- rerunning or rescoring failed or invalid packages.

No private historical fixture material or oracle predicate was used.

## Remaining gates

- No executable profile-diff held-out package exists.
- No opaque fixture ID, private manifest revision, package digest, cutoff, or
  score is claimed.
- Independent construction and both platform runs remain outstanding.
- A pass would establish only experimental command conformance at one cutoff.
- Profile generation, owner evidence collection, support, compatibility,
  approval, deployment, and PLATFORM-001 advancement remain unauthorized.

## Decision and authority

All nine roles accept the public contract and custody protocol. This review
authorizes independent future evaluation design only and grants no product or
oracle authority.
