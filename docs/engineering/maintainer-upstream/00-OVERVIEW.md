# Maintainer and Upstream Engineering Guide

Status: Guidance
Implementation authority: None

## Purpose

This guide set defines how Ferris prepares useful work for Rust ecosystem
owners without replacing their authority. It covers discovery, diagnosis,
evidence, contribution packets, stewardship support, adoption, rollback,
removal, and validation. It is guidance only: it does not authorize external
posting, implementation, funding, mutation, or an upstream commitment.

Ferris coordinates evidence across owner boundaries. Cargo, rustc, rustc-perf,
crate maintainers, standards bodies, native tool owners, and consumer
repositories retain their own truth, workflows, review, and release decisions.
The governing rules are:

> The plan is global; the work is local.

> Ferris coordinates owner truth; it does not replace it.

These rules follow the repository boundary in
[CONTEXT.md](../../../CONTEXT.md), the working constraints in
[AGENTS.md](../../../AGENTS.md), and the Ecosystem Bridge boundary in the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Intended readers

- Ferris engineers preparing a diagnostic, fixture, documentation change, or
  patch for an external owner.
- Rust maintainers deciding whether Ferris evidence is useful and sufficiently
  bounded.
- Program owners funding review, testing, release engineering, or succession
  without taking over a project.
- Reviewers applying the repository's nine roles before a packet advances.

## Guide map

1. [Boundary and ownership](01-BOUNDARY-AND-OWNERSHIP.md) explains owner
   discovery, routing, authority, and stewardship without takeover.
2. [Operating workflow](02-OPERATING-WORKFLOW.md) defines the ordinary path
   from a maintainer question to an upstream-first contribution.
3. [Evidence and identity](03-EVIDENCE-AND-IDENTITY.md) defines contribution
   packet identity, statuses, provenance, licensing, and public-safe evidence.
4. [Failure modes and controls](04-FAILURE-MODES-AND-CONTROLS.md) identifies
   recurrent boundary and process failures and the controls that stop them.
5. [Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md)
   defines reversible use, renewal, retirement, forks, and adapters.
6. [Validation roadmap](06-VALIDATION-ROADMAP.md) supplies measurable
   acceptance and a staged proof plan.

## Upstream-first intervention order

Use the least authority-expanding intervention that can answer the named
maintainer question:

1. route to the current owner;
2. document an existing contract or limitation;
3. provide a minimal reproducer;
4. improve a diagnostic;
5. contribute a positive and negative fixture;
6. propose an owner-native documentation change;
7. prepare a focused patch after owner alignment;
8. support maintenance, testing, release, or succession under current
   governance;
9. use a removable consumer adapter only when an upstream contract cannot
   serve the named consumer;
10. consider a fork only under the exceptional rules in the lifecycle guide.

The intervention classes and their non-authorizing character come from
[ECOS-Q12](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).
The public-good portfolio and first packet candidates are summarized in the
[leadership opportunity map](../../leadership/MICROSOFT_RUST_UPSTREAM_OPPORTUNITY_MAP.md).

## Required operating principles

Every effort must:

- start with one owner-answerable question, not a request to review Ferris as
  a whole;
- preserve ordinary Cargo, repository, editor, CI, and owner-native workflows;
- minimize before proposing and rerun positive and negative controls after
  each reduction;
- distinguish observed, inferred, predicted, approved, executed, failed,
  unsupported, stale, unavailable, not-observed, and unknown states;
- use public-safe, licensed inputs for any material that might leave the
  repository;
- obtain explicit approval before creating an external issue, comment, branch,
  benchmark run request, pull request, or public artifact;
- include the expected maintenance and review burden in the proposal;
- define response, renewal, supersession, retirement, rollback, and removal;
- measure success by owner usefulness and maintained outcomes, not patch size,
  lines changed, or merge alone; and
- leave the current owner free to redirect, decline, defer, or keep the case
  external.

## Role review lens

The nine role files are mandatory source material, not ceremonial labels:

- [Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md):
  state where Rust guarantees stop and what evidence supports safety claims.
- [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md):
  require representative workflows, causal evidence, variance, and limits.
- [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md):
  preserve ABI, ownership, lifetime, panic, threading, and allocation facts.
- [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md):
  separate evidence from assertion and retain failures.
- [Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md):
  test whether contribution beats duplication.
- [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md): reduce
  review cost and preserve ordinary workflows.
- [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md):
  expose platform, operations, training, audit, and recovery burden.
- [Scope Keeper](../../../.roles/editorial/scope-keeper.md): keep the effort
  bounded and non-goals visible.
- [Validation Checker](../../../.roles/editorial/validation-checker.md):
  require reproducible commands, representative fixtures, and negative cases.

No guide in this directory records role approval or opens implementation.

## Definition of useful upstream support

An effort is useful when the current owner can reproduce the case, understand
the requested decision, review a small evidence surface, preserve their local
architecture, and either accept, redirect, decline, or retire the work without
depending on Ferris. A durable result may be a maintained test, benchmark,
diagnostic, document, decision, or supported maintainer process. Merged code is
only one possible result.

The normative packet fields and status vocabulary remain in the
[Rust Performance Contribution Packet](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).
This guide set explains how to operate that contract; it does not replace it.
