# Platform Profile Schema Nine-Role Review

Date: 2026-08-12
Pulse: Canonical Platform Profile Fixture Contract
Disposition: Accepted as the frozen controlled-fixture contract
Implementation authority: Schema documents and controls only

## Review question

Does `ferris.platform-profile/v1` preserve the identity, evidence, stage,
platform, support, lifecycle, privacy, and owner boundaries required to build
all nine controlled PLATFORM-001 families without turning an experimental
fixture contract into support, approval, or product authority?

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

The schema retains toolchain, target, contract, native, provider, stage, and
limitation evidence. It does not let compilation, tests, profiles, or
signatures establish safety or soundness.

### Compiler Performance Engineer

**Disposition:** Accept with harness requirement.

Commands retain working directory, environment, network, and target-directory
topology. Pulse 03 must add cache and timing evidence only where a measured
claim requires it; the schema itself makes no performance claim.

### Interop Boundary Auditor

**Disposition:** Accept.

Rust source, semantic, native ABI, WIT component, wire/data, and projection
contracts are distinct. Native tools, providers, runtimes, stages, migration,
substitution, rollback, and removal remain separately addressable.

### AI Assurance Skeptic

**Disposition:** Accept.

Claim class, owner, source, command, dates, expiry, state, diagnostic, and
limitations are mandatory where evidence is recorded. Unknown and inferred
claims cannot be relabeled as direct observation.

### Ecosystem Strategist

**Disposition:** Accept with dependency caveat.

Cargo and RUNE remain external owners. FERRIS references an exact RUNE
revision and neutral profile instead of copying its model. The revision is
not presented as a RUNE v1 release.

### Rust Maintainer

**Disposition:** Accept.

The schema uses recognizable package, lock, feature, target, command,
diagnostic, support, rollback, and removal vocabulary. The experimental diff
projection is explicitly lossy and removable.

### Native Platform Adopter

**Disposition:** Accept.

Host, targets, native tools, providers, runtimes, filesystem, network,
packaging, deployment, support, emergency response, rollback, and removal are
explicit. Empty or unsupported native fields cannot become support.

### Scope Keeper

**Disposition:** Accept.

The pulse freezes documents and controls only. It does not add a parser,
harness, owner execution, family completion, generation, mutation, approval,
support, or specification advancement.

### Validation Checker

**Disposition:** Accept.

The schema has one valid exemplar and exact mutation controls for unsupported,
unknown-member, ambiguous-source, unknown-state, unsafe-metadata, duplicate,
malformed, and oversized cases. The schema document and base exemplar must
pass an independent JSON Schema validator at an immutable cutoff before this
pulse is complete. That validation passed at
`9ab231b9a347885e873de0cd76de8d2e2fa0fa7f`; Pulse 03 must execute every
mutation control.

## Remaining gates

- The nine mutation controls are not executed by repository-owned code.
- The valid exemplar is not a completed family and contains placeholder
  digests and unobserved lifecycle controls.
- No owner command or Windows/Unix schema matrix has run.
- No exact family, renewal, substitution, removal, or held-out result exists.
- At this schema review, RUNE v1 had not yet been reconciled; Pulse 21 later
  accepted the same exact revision as the v1 contract and release-readiness
  baseline without changing Cargo `0.1.0`, collection `v0`, profile `v0`, or
  fixture evidence.
- PLATFORM-001 remains Draft.

## Decision and authority

All nine roles accept `ferris.platform-profile/v1` as the frozen contract for
the controlled conformance program. Immutable-cutoff schema validation passed.
This review grants no parser, product, family, owner-execution, support,
held-out, or Proposed-status authority.
