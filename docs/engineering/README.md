# Ferris Rust Engineering Library

Status: Guidance
Implementation authority: None

This library translates Ferris research and Draft specifications into
repeatable engineering workflows. It is owned by FERRIS because it concerns
application intent, support commitments, evidence, validation, upstream
ownership, and lifecycle. It is not another general Rust language reference.

MAXIM remains the canonical source for the broad Rust reference mirrored under
[`docs/reference/rust-reference/`](../reference/rust-reference/README.md).

## Series

| Series | Purpose | Primary Ferris programs |
|---|---|---|
| [AI-assisted Rust engineering](ai-assisted-rust/00-OVERVIEW.md) | Govern generated Rust changes through compiler-grounded, behavioral, provenance, risk, and approval evidence. | Blueprint, Query Forest, Conformance |
| [Platform and target engineering](platform-target-engineering/00-OVERVIEW.md) | Qualify exact host, target, toolchain, native, runtime, packaging, and support stages without collapsing them into portability labels. | Profiles, Ecosystem Bridge, Conformance |
| [Validated stack profiles](validated-stack-profiles/00-OVERVIEW.md) | Maintain exact, renewable, consumer-scoped stack and support records without creating a Ferris distribution. | Profiles, Query Forest, Conformance |
| [Maintainer and upstream contribution](maintainer-upstream/00-OVERVIEW.md) | Route evidence, reproducers, diagnostics, documentation, and patches through current owners with bounded lifecycle obligations. | Ecosystem Bridge, Query Forest, Conformance |
| [Reference implementations](reference-implementations/00-OVERVIEW.md) | Define future executable companions as versioned conformance evidence with positive, negative, failure, platform, rollback, and removal cases. | Conformance |

The placement decision and evidence are recorded in
[Ferris Rust engineering gap closure](../research/2026-08-12-ferris-rust-engineering-gaps.md).
The completed documentation review is recorded in the
[nine-role review](FERRIS-RUST-ENGINEERING-LIBRARY-ROLE-REVIEW.md).

## Shared guide structure

Each series contains:

1. an overview and reader path;
2. boundary and ownership;
3. operating workflow;
4. evidence and identity;
5. failure modes and controls;
6. adoption, rollback, and removal; and
7. a validation roadmap.

## Shared operating rules

- Cargo, rustc, platform tools, upstream owners, and deployment systems retain
  their own authority.
- Observation, inference, prediction, proposal, approval, execution, result,
  and evidence remain distinct.
- Unsupported, unavailable, failed, not observed, stale, and unknown remain
  distinct states.
- AI may propose work and explanations but cannot establish owner truth,
  suppress required work, approve policy, or execute merely because a proposal
  exists.
- Every adoption names support, renewal, substitution, rollback, removal, and
  ordinary non-Ferris operation.
- A guide, Draft specification, example, or successful demonstration does not
  authorize implementation or establish conformance.

## Governing authorities

- [Ferris context](../../CONTEXT.md)
- [Ferris program](../plans/FERRIS_PROGRAM.md)
- [Seven-program architecture](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
- [Specification registry](../specs/README.md)
- [Ferris engineering principles](../governance/ENGINEERING_PRINCIPLES.md)
