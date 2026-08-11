# Ferris Context

Ferris is the cross-workspace enterprise build system for Rust.

This repository is currently a research, specification, and governance
repository. Its 22-specification spine is complete at Draft status. It does
not yet contain an authorized Ferris implementation.

The specification simulation program is complete at Draft after 11 waves and
46 frozen scenarios. It resolved all 25 Simulation Issues through 25 applied
Specification Change Records and froze a separate structural held-out
implementation fixture set.

The only implementation authority is the bounded read-only planning pulse in
`context/waves/2026-08-11-read-only-planning/`. It may implement local
`plan` and `explain` behavior over Cargo metadata and development fixtures. It
does not authorize execution, mutation, connectors, MCP, AI narrowing,
approval, deployment, remote evidence, or held-out oracle access.

## Product boundary

Ferris owns the global application plan, policy, approval, explanation,
evidence, lifecycle, and cross-workspace coordination.

Cargo remains authoritative for:

- package sources and dependency resolution;
- lock state;
- workspace membership;
- targets, features, profiles, and platform conditions;
- build-unit construction and freshness; and
- compiler invocation and local scheduling.

Ferris does not replace Cargo, rustc, linkers, test runners, Typebook/RUNE,
native tools, deployment systems, or their owner-local semantics.

The governing rule is:

> **The plan is global; the work is local.**

## Names

- **Ferris** is the public product.
- `ferris` is the primary command.
- `cargo ferris` is the Cargo external-subcommand entrypoint.
- **Blueprint** is the internal application model and planning engine.
- **Query Forest** is the canonical typed evidence model and immutable-root
  history.
- **Typebook/RUNE** is a separate, product-neutral semantic-contract system.

Public documentation should say Ferris unless it specifically means the
Blueprint Model or Blueprint Plan.

## Program architecture

The work is divided into seven bounded programs:

1. Ferris - public command, governance, approvals, lifecycle, and evidence.
2. Typebook - product-neutral semantic contracts.
3. Profiles - exact, renewable compatibility and support commitments.
4. Blueprint - normalized application model and non-executable planning.
5. Query Forest - scope, evidence, identity, causality, roots, refs, and
   history.
6. Conformance - executable positive, negative, failure, unsupported,
   version-skew, rollback, and removal proof.
7. Ecosystem Bridge - owner-aligned adapters, connectors, MCP, and upstream
   contribution packets.

Microsoft services are replaceable connectors, never canonical dependencies.

## Authority and gates

The canonical specification sequence is in
[`docs/specs/README.md`](docs/specs/README.md). A Draft, Proposed, or Adopted
specification does not by itself authorize implementation.

Implementation requires:

1. completed research dependencies;
2. a complete normative specification;
3. all applicable `.roles` reviews;
4. measurable acceptance and stop criteria;
5. adoption, support, removal, rollback, and maintenance plans; and
6. a separately approved implementation pulse.

Until that gate exists, work is limited to research, plans, specifications,
fixtures, and review records that do not create product code or hidden runtime
commitments.

## Core invariants

- Keep observation, normalization, projection, identity, prediction,
  resolution, execution, validation, outcome, and evidence responsibilities
  distinct.
- Keep Application Definition, Blueprint Model, Blueprint Plan, approved
  Action Plan, and FERRIS Application Contract as separate records.
- Keep Rust source, semantic, ABI, component, wire/data, and projection
  identities separate.
- Scope is multi-dimensional; package, target, activity, compilation, runtime,
  validation, contract, native, platform, lifecycle, and evidence scope must
  not collapse into one hierarchy or Boolean.
- Unknown mappings widen to the smallest safe owner boundary.
- AI may propose plans, mappings, and explanations but cannot establish owner
  truth, remove mandatory work, approve policy, or execute actions.
- Query Forest roots are immutable.
- Branches, tags, channels, aliases, pins, leases, tombstones, and labels have
  distinct semantics.
- Refs never prove compatibility, integrity, trust, validation, availability,
  or reuse.
- Credentials and reusable secrets never enter plans, prompts, roots, refs,
  logs, or durable evidence.
- Ordinary Cargo and owner-system workflows must remain functional after
  Ferris removal.

## Review model

Use the nine repository roles for architecture, specification, and gate
reviews:

- Rust Safety Steward;
- Compiler Performance Engineer;
- Interop Boundary Auditor;
- AI Assurance Skeptic;
- Ecosystem Strategist;
- Rust Maintainer;
- Native Platform Adopter;
- Scope Keeper; and
- Validation Checker.

Role files live under `.roles/`. Reviews record each role's disposition,
required revisions, remaining blockers, and whether implementation is
authorized.

## Key files

- [`README.md`](README.md) - public product and research overview.
- [`docs/plans/FERRIS_PROGRAM.md`](docs/plans/FERRIS_PROGRAM.md) - governing
  product, commands, sequencing, and implementation gates.
- [`docs/plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md`](docs/plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
  - program ownership and cross-program contracts.
- [`docs/plans/BLUEPRINT_PROGRAM.md`](docs/plans/BLUEPRINT_PROGRAM.md) -
  internal model and planning architecture.
- [`docs/specs/README.md`](docs/specs/README.md) - canonical specification
  registry and review status.
- [`docs/specs/FOREST_COMPONENT_MODEL.md`](docs/specs/FOREST_COMPONENT_MODEL.md)
  - Query Forest component boundaries.
- [`docs/simulations/README.md`](docs/simulations/README.md) - no-code
  specification simulation waves, issues, and change records.
- [`docs/research/questions/README.md`](docs/research/questions/README.md) -
  research-question registry.
- `.roles/` - review responsibilities and stakeholder perspectives.

## Repository workflow

- Make Ferris research, specification, fixture, and future implementation
  commits in this repository.
- Keep Ferris commits separate from TRACKER submodule-pointer updates.
- Commit and push Ferris work before an explicit TRACKER portfolio snapshot.
- Do not push unless requested.
- Do not amend commits unless explicitly requested.
- Do not rewrite or remove historical `FERRIUM-*` finding identifiers; new
  findings use `FERRIS-*`.

## Validation

For documentation and specification changes:

- check local Markdown links;
- check balanced code fences;
- run `git diff --check`;
- inspect the specification dependency graph for cycles;
- confirm all nine role dispositions where a review gate is claimed; and
- stage only files belonging to the current logical change.

Implementation validation commands will be added only when an implementation
pulse authorizes product code.
