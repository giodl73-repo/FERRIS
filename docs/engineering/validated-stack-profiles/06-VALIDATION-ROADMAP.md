# Validation Roadmap

Status: Guidance
Implementation authority: None

The active execution map is
[PLATFORM-001 Proposed Program](07-PLATFORM-PROPOSED-PROGRAM.md).

## Roadmap boundary

This roadmap defines evidence gates for validated stack profiles. It does not
authorize code, select packages, create a stable schema, promise support, or
open repository integration.

The only future capability identified by the ecosystem research is local,
read-only profile generation and diffing for one accepted maintainer workflow.
It must remain behind a separately approved pulse and must not install, edit,
update, approve, reject, publish, or deploy. See
[ecosystem intervention decisions](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md)
and [CONTEXT.md](../../../CONTEXT.md).

## Validation goals

Validation must establish that a profile record:

- accurately preserves exact consumer, selection, closure, feature,
  toolchain, target, provider, native, stage, assurance, stewardship, support,
  and lifecycle evidence;
- keeps owner declarations, observations, inferences, decisions, and
  executions distinct;
- improves a representative maintainer decision without replacing owner tools;
- preserves failures, unsupported conditions, unavailable sources, stale
  evidence, version skew, conflicts, and unknowns;
- supports renewal, substitution, emergency response, rollback, and complete
  removal;
- remains useful on Windows and Unix and across distinct profile families; and
- leaves ordinary Cargo behavior unchanged after removal.

## Phase 0: Guidance and specification readiness

Before implementation planning:

1. maintain an explicit profile boundary and non-goals;
2. map the record to PLATFORM-001, CONTRACT-001, EVIDENCE-001,
   VALIDATION-001, TRUST-001, and CONFORMANCE-001;
3. define exact schema identities and typed states;
4. define source ownership and freshness;
5. define adoption, support, emergency, renewal, substitution, rollback, and
   removal contracts;
6. define measurable acceptance and stop criteria;
7. complete all nine role reviews; and
8. obtain a separately approved implementation pulse.

Draft or reviewed guidance is not implementation authority.

## Phase 1: Family fixture design

Create exact, independently scoped fixture contracts for at least six
families before PLATFORM-001 can advance. The fixture program must retain all
nine families required by PLATFORM-001:

1. hosted service;
2. CLI and configuration;
3. pure data;
4. embedded and `no_std`;
5. browser WASM;
6. WebAssembly component;
7. bundled or system-native dependency;
8. identity, credential, TLS, and cryptographic provider; and
9. testing, assurance, packaging, and deployment.

After the required families are represented, separate extension fixtures may
cover desktop and GUI, networking and protocol, and data, ML, and GPU. Those
extensions do not replace a required PLATFORM-001 fixture.

Each fixture needs:

- one named consumer operation;
- exact direct selection and source identity;
- requested and effective features;
- lock universe and target-active closures;
- exact compiler, host, target, provider, native, and deployment assumptions;
- positive, expected-rejection, failure, unsupported, unavailable, stale, and
  unknown cases;
- representative support and expiry records;
- one material change; and
- adoption, renewal, substitution, rollback, and removal expectations.

No fixture may stand in for another family. Shared crates do not erase target,
runtime, native, or operational differences.

## Phase 2: Baseline evidence controls

For every fixture, preserve reproducible commands and environment identities
for:

- resolve;
- check;
- lint;
- build;
- link;
- execute;
- unit and integration test;
- doctest;
- contract conformance;
- package;
- sign or attest where applicable;
- deploy where applicable;
- operational validation; and
- rollback.

Record each stage independently. Include deliberate negative cases that prove
scope boundaries, such as host rejection for a WASM-only crate, unavailable
native cross compiler, unsupported target, invalid provider, incompatible
wire revision, missing device, or revoked evidence.

## Phase 3: Closure and identity conformance

Tests should verify:

- manifest and lock identity are exact;
- lock universe and target-active normal/build/development closures remain
  separately queryable;
- requested and effective features are not collapsed;
- build scripts, procedural macros, `links`, generated code, unsafe, native,
  and public dependencies are visible;
- contract, adapter, provider, source mode, and artifact identities remain
  distinct;
- compiler-floor claims are exact observations, not metadata inference;
- every source carries owner, date, assertion class, limitations, and expiry;
  and
- aliases, labels, branches, or names cannot substitute for exact identity.

Projection tests should prove that human and machine views preserve the same
states, identities, limitations, and owner boundaries.

## Phase 4: Renewal-diff validation

Execute at least one bounded dependency renewal:

1. freeze the approved baseline;
2. propose an exact update;
3. diff direct and transitive identities;
4. diff lock and active closures;
5. diff requested and effective features;
6. diff toolchain, target, provider, native, contract, assurance,
   stewardship, support, and lifecycle evidence;
7. run all affected positive and negative stages;
8. record consumer approval or rejection;
9. restore the exact prior state; and
10. prove rollback identity and required behavior.

The research Clap 4.6.5 to 4.6.6 control is the baseline example, not evidence
that future updates are safe. See
[compatibility stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

## Phase 5: Substitution and emergency validation

Perform one provider or implementation substitution without silently changing
the consumer contract. Exercise:

- positive and negative semantic compatibility;
- migration and coexistence;
- feature and closure changes;
- runtime, native, data, wire, ABI, deployment, support, cost, and operations
  differences;
- rollback during the declared window; and
- complete removal of the candidate.

Separately simulate an emergency advisory, source revocation, provider outage,
toolchain regression, or support withdrawal. Verify typed stale/revoked/
unavailable states, affected-profile selection, owner routing, human approval,
containment alternatives, and audited response.

## Phase 6: Held-out maintainer workflow

Before any profile automation:

- freeze at least three public Rust repositories;
- include a hosted application, a cross-target or `no_std` case, and a
  native-bound case;
- execute Windows and Unix paths;
- seed exact positive, negative, failure, unsupported, stale, and unknown
  cases;
- compare raw-tool investigation with the proposed evidence record;
- measure collection time, storage, rate limits, cache state, false
  conclusions, omitted scope, maintainer investigation time, renewal cost, and
  operational burden;
- execute one renewal and exact rollback;
- demonstrate complete removal without correctness changes;
- preserve privacy, source attribution, and retention boundaries; and
- complete a fresh nine-role review.

The held-out design must prevent the implementation from reading sealed
expected outcomes. Failed or invalid held-out attempts remain visible and are
not rescored as successes.

## Phase 7: Ordinary Cargo preservation

For every adoption and removal case:

1. run owner-native Cargo commands before profile adoption;
2. introduce the profile without changing Cargo authority;
3. run the same ordinary commands during adoption;
4. remove all profile metadata, projections, services, and automation;
5. run the same commands after removal;
6. compare required behavior and diagnostics; and
7. verify no hidden resolver, lock, registry, feature, source, or environment
   dependency remains.

This is a release-blocking conformance class, not optional usability evidence.

## Future read-only pulse

Only after the preceding gates may a pulse propose bounded implementation. The
pulse should authorize one local maintainer workflow:

> Given two exact profile evidence sets, show what identity, closure, feature,
> toolchain, target, provider, native, stage, assurance, stewardship, support,
> expiry, removal, and rollback evidence changed.

The capability must:

- be local and read-only;
- consume explicit inputs and stable owner sources;
- isolate source-specific adapters;
- expose observed, declared, inferred, stale, conflicting, and unknown states;
- produce complete human and machine-readable diffs;
- avoid credentials and reusable secrets;
- avoid package, manifest, lock, feature, toolchain, provider, source, native,
  CI, or deployment mutation;
- avoid approval, certification, ranking, installation, publication, and
  external posting;
- fail closed with typed diagnostics when evidence is unavailable or invalid;
  and
- be completely removable without changing Cargo behavior.

Profile generation should likewise be read-only: it may propose a record from
explicit evidence, but a human or existing policy owner must approve the
revision and support commitment.

## Role-gated acceptance

Before a pulse, all nine roles must record disposition:

- [Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md):
  safety boundaries and dedicated evidence;
- [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md):
  representative overhead, cache state, variance, and causality;
- [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md):
  ABI, semantic loss, negative tests, migration, and rollback;
- [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md):
  source attribution, assertion class, visible failures, and human authority;
- [Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md):
  current-owner alignment and no competing distribution;
- [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md):
  actionable diagnostics, workflow benefit, simplicity, and removal;
- [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md):
  Windows/Unix, native tooling, operations, recovery, and audit;
- [Scope Keeper](../../../.roles/editorial/scope-keeper.md):
  one bounded read-only capability and visible deferred lanes; and
- [Validation Checker](../../../.roles/editorial/validation-checker.md):
  reproducible commands, representative fixtures, negative cases, and actual
  results.

## Stop criteria

Stop or redesign if the proof:

- needs a parallel resolver, hidden manifest, mandatory registry, or Ferris
  distribution;
- cannot preserve exact owner evidence and typed unknowns;
- promotes build success into broader compatibility or support;
- requires automatic dependency or environment mutation;
- cannot remain consumer-scoped and family-specific;
- creates more maintainer investigation or renewal cost than it removes;
- cannot operate on Windows and Unix as claimed;
- cannot execute rollback or complete removal;
- breaks ordinary Cargo after removal; or
- lacks an explicit consumer, support owner, validation contract, or pulse.

## Documentation validation

Changes to these guides should check local Markdown links, balanced code
fences, ASCII-only content, and `git diff --check`. Any future gate claim must
also inspect specification dependencies and confirm all nine role
dispositions, as required by [AGENTS.md](../../../AGENTS.md).
