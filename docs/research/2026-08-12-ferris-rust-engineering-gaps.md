# Ferris Rust Engineering Gap Closure

Date: 2026-08-12
Status: Complete
Decision: place the five applied Rust engineering series in FERRIS, not in the
canonical MAXIM Rust reference.

## Decision supported

This research decides where the following requested material belongs and what
authority it carries:

1. AI-assisted Rust engineering;
2. platform and target engineering;
3. validated stack profiles;
4. maintainer and upstream contribution; and
5. executable reference implementations.

The decision is to publish all five as FERRIS engineering guidance. They
translate existing Ferris research, specifications, and program boundaries
into repeatable adoption workflows. They do not extend Rust language reference
coverage and therefore do not belong in MAXIM's canonical reference modules.

The series do not authorize new Ferris product code. Current implementation
authority remains limited by
[`CONTEXT.md`](../../CONTEXT.md) and the separately approved read-only pulses.

## Local evidence inventory

| Evidence | Relevance |
|---|---|
| [`PRODUCT_PLAN.md`](../../PRODUCT_PLAN.md) | Names AI-generated code assurance, platform support, supported profiles, upstream contribution, and executable evidence as Ferris concerns. |
| [Ferris seven-program architecture](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md) | Assigns the work to Profiles, Blueprint, Query Forest, Conformance, and Ecosystem Bridge. |
| [Renewable platform profile contract](../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md) | Defines exact closure, environment, stage, support, renewal, substitution, removal, and rollback requirements. |
| [Rust platform compatibility](2026-08-09-rust-platform-compatibility.md) | Demonstrates why target tiers and `cargo check --target` do not establish link, execution, test, or deployment support. |
| [Compatibility-tested stack profiles](2026-08-10-rust-compatibility-stack-profiles.md) | Demonstrates renewable consumer profiles without creating a Ferris distribution. |
| [Ecosystem intervention decisions](2026-08-10-rust-ecosystem-intervention-decisions.md) | Establishes owner-aligned adoption, adaptation, contribution, stewardship, and deferral choices. |
| [Rust performance contribution packet](../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md) | Provides a bounded upstream packet contract and explicit external-posting gate. |
| [Cross-command scope model](2026-08-10-blueprint-cross-command-scope-model.md) | Separates deterministic scope authority from AI proposals and explanations. |
| [Conformance contract](../specs/FERRIS_CONFORMANCE_CONTRACT.md) | Requires positive, negative, failure, unsupported, version-skew, rollback, removal, and cross-platform proof. |
| [Ferris engineering principles](../governance/ENGINEERING_PRINCIPLES.md) | Rejects confidence-shaped assurance, opaque recommendations, hidden failure, and unbounded prototypes. |

## Findings

### FERRIS-GAP-01: these gaps are Ferris adoption capabilities

**Sources:** [`PRODUCT_PLAN.md`](../../PRODUCT_PLAN.md), [enterprise Rust
application platform](../plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md), and
the [seven-program architecture](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

**Observed behavior:** The existing product model already assigns profiles,
application intent, evidence, conformance, owner routing, AI boundaries,
platform state, and upstream packets to Ferris. MAXIM supplies language and
engineering reference material but does not own consumer support commitments
or Ferris product workflows.

**Implication:** The five series should be maintained under
`docs/engineering/` in FERRIS and may cite the generated MAXIM mirror as
background. They must not be synchronized back into MAXIM.

**Confidence:** High.

### FERRIS-GAP-02: guidance can close the operating-model gap without opening code

**Sources:** [`CONTEXT.md`](../../CONTEXT.md), [specification
registry](../specs/README.md), and [Conformance
program](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md#program-6-conformance).

**Observed behavior:** Ferris has a complete Draft specification spine and a
bounded implementation. The missing material is the connective operating
guidance that tells adopters how to use the research and contracts without
mistaking a draft specification for an executable capability.

**Implication:** Every guide must state `Implementation authority: None`,
identify the owning program and specifications, and distinguish current manual
practice from future Ferris automation.

**Confidence:** High.

### FERRIS-GAP-03: AI-assisted Rust needs an evidence workflow, not prompting advice

**Sources:** [cross-command scope
model](2026-08-10-blueprint-cross-command-scope-model.md), [causality
contract](../specs/FERRIS_CAUSALITY_CONTRACT.md), [prediction
contract](../specs/FERRIS_PREDICTION_CONTRACT.md), and [engineering
principles](../governance/ENGINEERING_PRINCIPLES.md).

**Observed behavior:** Existing Ferris authority permits AI to propose mappings,
plans, and explanations while deterministic policy, owner tools, and human
approval retain authority. Generated-code risk also crosses unsafe, FFI,
dependency, build-script, procedural-macro, performance, privacy, and
operational boundaries.

**Implication:** The AI series must center on patch identity, provenance,
compiler feedback, behavioral and negative evidence, risk-tiered approval,
failure visibility, and removal. It must not present model confidence as proof.

**Confidence:** High.

### FERRIS-GAP-04: platform support is a renewable stage matrix

**Sources:** [Rust platform
compatibility](2026-08-09-rust-platform-compatibility.md), [native dependency
boundary](2026-08-10-rust-native-dependency-boundary.md), and
[PLATFORM-001](../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md).

**Observed behavior:** The measured platform work separated resolution,
checking, linking, execution, testing, native prerequisites, providers, and
deployment. Target tiers, installed standard libraries, and successful
cross-checks did not establish complete cross-platform support.

**Implication:** The platform series must retain host and target identity,
toolchains, SDKs, linkers, sysroots, runners, providers, packaging, servicing,
and typed negative or unknown states across independently qualified stages.

**Confidence:** High.

### FERRIS-GAP-05: tested stacks are profiles, not a Rust distribution

**Sources:** [compatibility-tested stack
profiles](2026-08-10-rust-compatibility-stack-profiles.md),
[PLATFORM-001](../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md), and [ecosystem
intervention decisions](2026-08-10-rust-ecosystem-intervention-decisions.md).

**Observed behavior:** Six measured profile shapes demonstrated that exact
closure, compiler floor, target, runtime, provider, advisory, support, and
lifecycle evidence differ by consumer. Renewal and rollback can be bounded,
while one universal stack would merge incompatible assumptions.

**Implication:** The profile series must teach exact, expiring,
consumer-scoped support records. Candidate domain profiles remain independent
and must preserve ordinary Cargo use, substitution, removal, and rollback.

**Confidence:** High.

### FERRIS-GAP-06: upstream work requires owner alignment before external action

**Sources:** [ecosystem intervention
decisions](2026-08-10-rust-ecosystem-intervention-decisions.md), [maintenance
and stewardship](2026-08-09-rust-maintenance-stewardship.md), and [Rust
performance contribution packet](../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

**Observed behavior:** Research can identify an owner, reproduce a problem,
minimize a fixture, and prepare a packet without having authority to create an
external issue, comment, branch, pull request, or support obligation.

**Implication:** The maintainer series must make owner discovery, public-safe
evidence, licensing, bounded asks, maintainer burden, approval, response
ownership, supersession, and retirement explicit.

**Confidence:** High.

### FERRIS-GAP-07: reference implementations belong to Conformance

**Sources:** [Conformance
contract](../specs/FERRIS_CONFORMANCE_CONTRACT.md), [simulation
method](2026-08-10-ferris-specification-simulation-method.md), and [held-out
manifest](../simulations/held-out/MANIFEST.md).

**Observed behavior:** A useful reference implementation is not merely sample
code. It carries exact source and environment identity, expected results,
positive and negative cases, platform scope, failure and unsupported behavior,
anti-leak boundaries, maintenance, and retirement.

**Implication:** Reference companions should be governed as versioned
Conformance fixtures or repositories. The initial series specifies their
contract and promotion gate; executable companions require later bounded
pulses.

**Confidence:** High.

### FERRIS-GAP-08: all five series need one shared lifecycle

**Sources:** [seven-program
architecture](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md),
[PLATFORM-001](../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md), and
[CONFORMANCE-001](../specs/FERRIS_CONFORMANCE_CONTRACT.md).

**Observed behavior:** AI proposals, platform observations, profiles, upstream
packets, and fixtures all become misleading when identity, evidence, owner,
expiry, replacement, or removal is omitted.

**Implication:** The engineering library should use one recurring structure:
boundary and ownership, workflow, evidence and identity, failure controls,
adoption and removal, and validation roadmap.

**Confidence:** High.

## Recommendations

### Adopt now

- Publish the five FERRIS engineering series under `docs/engineering/`.
- Keep each guide explicitly non-normative and non-authorizing.
- Cite the exact Ferris research, specification, and owner boundary behind
  every operational recommendation.
- Use the same lifecycle vocabulary across the series: identity, owner,
  evidence, state, expiry, renewal, substitution, rollback, removal, and
  retirement.
- Preserve MAXIM as the canonical general Rust reference and use its generated
  mirror only for background language and engineering explanations.

### Prototype behind a compatibility boundary

- read-only generated-patch evidence assembly;
- read-only platform prerequisite and stage-matrix reporting;
- read-only profile generation, renewal diffing, and expiry reporting;
- local upstream contribution packet generation; and
- executable reference companions with frozen expected results.

Each prototype requires a separately approved pulse, exact fixtures, all nine
role dispositions, measurable thresholds, ordinary Cargo fallback, and
removal proof.

### Reject or defer

- another broad MAXIM Rust module for these Ferris-specific workflows;
- a universal recommended Rust stack or permanent Ferris distribution;
- AI authority to establish scope, safety, correctness, performance, policy,
  or approval;
- automatic SDK, linker, provider, dependency, profile, or environment
  mutation;
- external posting or stewardship commitments without owner approval; and
- reference implementations presented as conformance before their positive,
  negative, failure, platform, rollback, and removal gates pass.

## Ownership and validation

| Series | Ferris owner programs | Expected validation | Non-goal |
|---|---|---|---|
| AI-assisted Rust | Blueprint, Query Forest, Conformance | provenance, deterministic scope comparison, behavioral and negative tests, risk approval | autonomous correctness or approval |
| Platform and target engineering | Profiles, Ecosystem Bridge, Conformance | exact host/target stage matrix and prerequisite evidence | universal portability or environment repair |
| Validated stack profiles | Profiles, Query Forest, Conformance | exact closure, renewal diff, expiry, substitution, rollback, removal | distribution or certification |
| Maintainer and upstream | Ecosystem Bridge, Query Forest, Conformance | minimized public-safe reproducer, bounded owner ask, lifecycle | owner replacement or unauthorized posting |
| Reference implementations | Conformance with all producing programs | frozen positive, negative, failure, unsupported, skew, platform, rollback, removal cases | example code as proof |

## Research limitations

- This pass synthesized the existing Ferris corpus; it did not rerun the prior
  platform or profile experiments.
- No external owner accepted a new support or maintenance obligation during
  this work.
- No new product command, schema, executable fixture, connector, or automated
  workflow was implemented.
- Candidate domain profiles remain examples until exact consumers, fixtures,
  owners, and renewal evidence exist.

