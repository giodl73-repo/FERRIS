# Adoption, Rollback, and Removal

> Status: Guidance only.
> Implementation authority: None. This guide does not authorize product code,
> execution, mutation, approval, or changes to Ferris canonical records.

## Adoption goal

Adopt AI assistance incrementally so a team can stop using it without changing
Rust source semantics, losing ordinary Cargo access, or depending on hidden
correctness state. Reversibility is a promotion criterion, not cleanup to
design later.

The normative adoption and removal obligations are in
[PRODUCT-001](../../specs/FERRIS_PRODUCT_CONTRACT.md). The product strategy
also requires bounded, research-backed prototypes and ordinary workflow
preservation in the [Ferris product plan](../../../PRODUCT_PLAN.md).

## Preconditions

Before a repository adopts patch generation:

- name the consumer and bounded use case;
- inventory existing editor, Cargo, CI, review, security, and release
  workflows;
- identify data classifications and approved model/connector boundaries;
- define risk bands and prohibited change classes;
- define exact owner-native baseline and full-reference commands;
- name support, evidence, incident, rollback, and removal owners;
- define measurable benefit, cost, false-omission, privacy, and stop
  thresholds;
- test operation without the assistant; and
- obtain accountable approval for the adoption experiment.

Do not make adoption depend on a new manifest language, alternate package
resolver, hidden build graph, shared writable artifact state, or mandatory
remote service.

## Maturity progression

Use the [maturity model](00-OVERVIEW.md#adoption-maturity-model) as a reversible
sequence.

### M0: Observe

AI may explain existing code, compiler output, or possible tests without
changing the repository.

Exit evidence:

- maintainers can distinguish assertion from owner evidence;
- privacy and prompt-injection controls work;
- explanations reduce investigation effort; and
- disabling the assistant has no repository effect.

### M1: Propose

AI may create a bounded local patch for human review.

Exit evidence:

- provenance is consistently retained;
- ordinary Cargo commands reproduce the result;
- behavioral and negative tests cover material changes;
- R2/R3 changes reliably escalate;
- failure and revert drills succeed; and
- no required build, test, or editor path depends on the assistant.

### M2: Evidence-governed

Teams use deterministic owner scope, mandatory gates, selected-versus-full
comparison, false-omission review, and explicit approval.

Exit evidence:

- full-reference controls detect seeded or held-out omissions;
- unknowns widen or block;
- privacy, unsupported, unavailable, and failure states remain visible;
- performance benefit is measured on representative workflows;
- incidents disable narrowing quickly; and
- removal verification succeeds.

### M3: Controlled scale

Multiple repositories or teams use governed assistance with defined support,
renewal, cross-platform, incident, and conformance expectations.

Exit evidence:

- fixed acceptance and stop thresholds pass on held-out workflows;
- all applicable nine-role concerns have recorded dispositions;
- support and maintenance costs are acceptable;
- rollback and removal work across repository, workstation, and CI
  integrations; and
- no claim exceeds the conformance evidence.

Maturity may regress. A serious false omission, privacy event, unreviewed
high-risk patch, removal failure, or unexplained behavior should suspend the
affected capability.

## Adoption controls by surface

### Repository

- Keep generated changes as ordinary diffs.
- Preserve Cargo.toml, Cargo.lock, standard configuration, and documented
  scripts as owner-readable sources.
- Mark generated files and regeneration ownership where the repository already
  has such conventions.
- Do not hide required context in a model session.

### Developer workstation

- Make the assistant optional.
- Document allowed data, commands, network access, and local storage.
- Keep credentials outside prompts and durable evidence.
- Provide a disable path that does not break Cargo, rust-analyzer, or editors.

### CI

- Start read-only or proposal-only.
- Do not let an AI identity imply approval.
- Preserve required owner-native gates.
- Keep failure output and unavailable checks visible.
- Ensure disabling the integration restores the previous workflow.

### Dependencies and generated tools

- Pin and renew exact tool/dependency identity according to owner policy.
- Inventory build scripts, proc macros, native components, and generators.
- Avoid requiring the assistant to regenerate or validate source.
- Retain a documented manual or owner-native path.

## Rollback design

Define rollback before accepting a generated change:

- trigger: which test, incident, metric, boundary failure, or owner decision
  initiates rollback;
- scope: source, lockfile, generated files, configuration, data, artifacts,
  deployment, and external actions;
- authority: who may order and perform it;
- method: revert, feature disable, configuration restore, data compensation,
  or release rollback;
- validation: owner-native commands and behavior that establish recovery;
- cleanup: credentials, sessions, hooks, caches, generated outputs, temporary
  resources, and external state;
- failure path: what happens when rollback is partial or impossible; and
- evidence: times, commands, results, remaining unknowns, and owner.

Rollback failure is a visible failure, not "mostly restored." Where execution
is involved, [EXECUTION-001](../../specs/FERRIS_EXECUTION_CONTRACT.md) keeps
cancellation, compensation, rollback, cleanup, and partial results distinct.

## Capability disablement

The fastest safe response may be to disable only:

- patch generation;
- dependency suggestions;
- scope narrowing;
- automatic command execution;
- access to private context;
- a model or connector;
- a repository integration; or
- publication.

Define these switches before scale. Disabling narrowing must restore
full-reference owner-native validation, not skip validation.

## Removal procedure

Disabling or deleting AI-assistance components is capability cleanup. Every
partial or complete removal of Ferris integration must also produce the
canonical, versioned
[PRODUCT-001 Removal Record](../../specs/FERRIS_PRODUCT_CONTRACT.md#removal-record).
The procedure below supplies that record with the exact integration inventory,
authority, freeze and drain state, evidence export, cleanup, owner-native
verification, retained evidence, residual effects, recovery owner, and final
disposition.

Plan removal as:

1. **Freeze** new generated or mutating actions.
2. **Inventory** versions, integrations, hooks, agents, connectors,
   configuration, credentials, caches, generated files, evidence, and active
   work.
3. **Drain** or explicitly cancel active sessions and external actions.
4. **Export** required review, audit, and historical evidence under policy.
5. **Disable** workstation, CI, repository, connector, network, publication,
   and credential paths.
6. **Clean** optional configuration, caches, generated artifacts, services,
   and hooks without deleting owner data.
7. **Verify** ordinary Cargo, editor, repository, CI, and release commands.
8. **Record** retained evidence, deletion requests, unknown residual effects,
   recovery owner, deadline, and reinstall boundary.

Do not call removal complete while a hidden correctness dependency, active
mutation, reusable credential, unresolved hook, unavailable required evidence,
failed owner-native verification, or unknown residual effect remains. This is
the removal rule in
[PRODUCT-001](../../specs/FERRIS_PRODUCT_CONTRACT.md).

## Ordinary Cargo verification

At minimum, use the repository's documented equivalents of:

- metadata or workspace discovery, when already part of the workflow;
- check/build for required targets and features;
- relevant unit, integration, documentation, and compile-fail tests;
- lints and formatting;
- native/link/package/release gates where applicable; and
- clean-environment or full-reference validation required by owner policy.

The exact commands are repository-owned. This guide does not invent a universal
Cargo command set.

## Adoption scorecard

Promote only when evidence answers:

- Does the workflow reduce maintainer effort on a named task?
- What false omissions or misleading explanations occurred?
- Are R2/R3 boundaries consistently escalated?
- Are behavior and negative tests improved rather than weakened?
- Are privacy and prompt-injection controls effective?
- Are performance gains representative and greater than overhead?
- Can teams diagnose failures without a model session?
- Can the capability be disabled quickly?
- Can the repository build, test, and recover normally after removal?
- Who renews dependencies, policies, models, and evidence?

Usage volume, generated lines, acceptance rate, and model confidence are not
standalone maturity evidence.
