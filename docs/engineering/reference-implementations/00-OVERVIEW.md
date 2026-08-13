# Ferris Reference Implementations

Status: Guidance
Implementation authority: None

## Purpose

This series defines how future executable companion fixtures and reference
repositories could provide Conformance evidence for Ferris. It does not create
an executable fixture, authorize a new command, bind a conformance oracle, or
claim that any implementation conforms.

Current product authority remains limited to local read-only `plan`, `explain`,
declared-workspace `graph`, and passive local `doctor` behavior. The authority
boundary is recorded in [Ferris context](../../../CONTEXT.md) and
[agent instructions](../../../AGENTS.md). Executable companions require a
separately approved implementation pulse.

The series applies the Conformance mission in the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md#program-6-conformance)
and the executable-proof requirements in
[CONFORMANCE-001](../../specs/FERRIS_CONFORMANCE_CONTRACT.md). It also preserves
the separation between simulated outcomes and observed implementation behavior
defined by the [simulation registry](../../simulations/README.md).

## Governing rule

> A reference implementation is a versioned evidence producer, not example
> code and not a correctness certificate.

A useful companion fixes source, toolchain, environment, command, expected
result, failure controls, owner-native comparison, maintenance, expiry, and
removal. A repository containing only a successful example is not a
Conformance fixture.

## Taxonomy

| Term | Meaning | May support a conformance claim? |
|---|---|---|
| Specification simulation | A no-code hand trace used to test a Draft contract | No |
| Example | Explanatory source or configuration without a frozen proof contract | No |
| Development fixture | An executable case visible to implementation authors and usable for debugging | No held-out claim |
| Calibration fixture | A case used to tune thresholds, mappings, prompts, or scorers | No held-out claim |
| Held-out fixture | A sealed input and oracle unavailable to implementation authors before scoring | Yes, within its named scope |
| Reference companion | A maintained repository or fixture family with public contracts, exact identities, expected outputs, and lifecycle controls | Only after applicable gates pass |
| Owner-native full reference | The authoritative Cargo, repository, platform, or upstream workflow used as the comparison baseline | It supplies owner evidence, not Ferris conformance by itself |
| Viewer or scorer | An independently versioned consumer of fixture output | Only for the schemas and predicates it validates |

Development, calibration, and held-out populations must remain distinct. A
held-out case disclosed for debugging becomes development evidence and needs a
new ID, sealed input, and independently frozen oracle, as required by the
[oracle custody protocol](../../simulations/held-out/ORACLE_CUSTODY.md).

## Required case families

Every promoted companion suite defines applicable:

- positive cases;
- negative and expected-rejection cases;
- bounded failure and partial-result cases;
- unsupported and unavailable cases;
- stale, expired, revoked, and corrupt evidence cases;
- schema, protocol, dependency, compiler, and tool version-skew cases;
- Windows, Unix, target, native-tool, and runtime cases;
- adoption and partial-install cases;
- rollback and cleanup cases; and
- complete integration and metadata removal cases.

`Unsupported`, `unavailable`, `failed`, `not observed`, `stale`, and `unknown`
are separate results. None may be rewritten as a degraded success.

## Candidate companion repository families

The following are candidates for later bounded design. Names are descriptive,
not repository commitments.

| Candidate family | Intended proof boundary | Owner-native reference |
|---|---|---|
| Blueprint applications | Multi-workspace application definitions, owner closures, non-executable plans, explanations, and full-reference comparison | Cargo metadata plus repository scripts |
| Renewable profiles | Exact releases, features, lock and active-target closure, compiler floor, stages, expiry, renewal, rollback, and removal | Cargo and consumer-owned validation |
| AI-generated patches | Patch provenance, deterministic scope comparison, behavioral and negative validation, rejection, fallback, and rollback | Human-reviewed patch plus full repository gates |
| Native boundaries | ABI, ownership, allocation, panic, threading, linking, loading, failure diagnosis, and incremental removal | Native build, test, debugger, and packaging systems |
| Platform targets | Exact host/target/toolchain/SDK/linker/runner identity and independent resolve/check/build/link/run/test/package stages | Platform-owner and repository commands |
| Upstream packets | Public-safe reproducer, owner routing, bounded ask, review state, supersession, and retirement without automatic posting | Current upstream contribution process |

These families must remain separate where their owners, toolchains, privacy
rules, or support commitments differ. The measured stack-profile research
demonstrates why one universal repository would collapse incompatible host,
WASM, embedded, and native assumptions; see
[Rust compatibility-tested stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

## Hermetic and owner-native lanes

A suite normally needs two complementary lanes:

1. **Hermetic fixture lane:** freezes inputs, dependencies, environment
   allowlists, network policy, clocks, resource limits, output framing, and
   expected records so results can be reproduced.
2. **Owner-native lane:** invokes ordinary Cargo, repository scripts, native
   tools, platform runners, or upstream processes without replacing their
   semantics.

Hermeticity controls the evidence boundary. It must not manufacture owner
truth. Owner-native behavior supplies the full reference. Differences between
the lanes are explicit evidence, not noise to discard.

## Reader path

1. [Boundary and ownership](01-BOUNDARY-AND-OWNERSHIP.md)
2. [Operating workflow](02-OPERATING-WORKFLOW.md)
3. [Evidence and identity](03-EVIDENCE-AND-IDENTITY.md)
4. [Failure modes and controls](04-FAILURE-MODES-AND-CONTROLS.md)
5. [Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md)
6. [Validation roadmap](06-VALIDATION-ROADMAP.md)

## Role review lens

Promotion requires dispositions from all nine repository roles:

- [Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md);
- [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md);
- [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md);
- [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md);
- [Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md);
- [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md);
- [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md);
- [Scope Keeper](../../../.roles/editorial/scope-keeper.md); and
- [Validation Checker](../../../.roles/editorial/validation-checker.md).

A role list is not a review claim. A gate record must contain each
disposition, required revisions, remaining blockers, and explicit
implementation authority.

## Claim boundary

Publication of these guides establishes guidance only. It does not:

- bind repositories, revisions, commands, schemas, thresholds, or oracles;
- authorize execution, mutation, connectors, MCP, AI narrowing, approval,
  deployment, or external posting;
- turn a simulation, demonstration, or fixture pass into product conformance;
- transfer support duties from consumers or upstream owners to Ferris; or
- certify safety, correctness, security, performance, portability, or
  compatibility.

The specification registry remains the authority for contract status:
[Ferris specification registry](../../specs/README.md).
