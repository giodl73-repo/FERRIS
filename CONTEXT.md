# Ferris Context

Ferris is the cross-workspace enterprise build system for Rust.

This repository contains a bounded experimental Ferris implementation in
addition to its research, specification, and governance records. Its
22-specification spine is complete at Draft status.

The specification simulation program is complete at Draft after 11 waves and
46 frozen scenarios. It resolved all 25 Simulation Issues through 25 applied
Specification Change Records and froze a separate structural held-out
implementation fixture set.

The only implementation authority is the bounded read-only wave in
`context/waves/2026-08-11-read-only-planning/`. It may implement local
`plan`, `explain`, bounded declared-workspace `graph`, and passive local
`doctor` behavior over explicit local inputs and development fixtures. It does
not authorize affected-only scope, query, execution, mutation, active probes,
connectors, MCP, AI narrowing, approval, deployment, remote evidence, or
held-out oracle access.

Pulses 01 and 02 established local `plan`, `explain`, and declared `graph`.
Pulse 03 hardens their explicit portable workspace identity, invocation
identity, evidence representation, human output completeness, diagnostic
redaction, and JSON-mode CLI parse failures. Its applicable held-out fixtures
passed. Earlier held-out cutoffs remain historical evidence for their frozen
commits.

Pulse 04 authorizes only a passive `doctor` command that validates a portable
workspace identity, reads the explicitly selected manifest, and invokes
`cargo --version`. It does not invoke Cargo metadata or owner work. Windows
and Unix development gates passed; all 12 existing held-out fixtures were
independently classified out of scope and were not executed.

Pulse 05 corrects the Pulse 04 review findings. Cargo metadata and passive
doctor now use the same selected-manifest directory and inherited owner
toolchain context with offline, no-update, and no-auto-install guards. Doctor
adds a 1 MiB manifest bound, five-second process bound, 64 KiB per-stream
output bounds, owner-output-bound identities, and manifest-digest failure
identity after the manifest is read. Both applicable replacement held-out
owner-context fixtures passed. No dedicated passive-doctor fixture exists, so
no held-out doctor claim is made.

An independently designed blind doctor fixture, FHIF-013, then found a strict
Cargo-evidence and post-read identity gap at the Pulse 05 cutoff. It is now
development evidence and cannot be rescored. Pulse 06 tightens the canonical
Cargo version grammar, exposes safe commit/date evidence, and binds command,
working-directory, every resource bound, framing, and owner evidence into
doctor identity. A separately sealed replacement fixture is required.

The first replacement, FHIF-014, also failed its blind score and is now
development evidence. Pulse 07 removes manual doctor report identity field
lists by hashing the complete typed record, tightens canonical Cargo commit
and Gregorian release-date validation, and gives oversized manifests a
portable bounded-prefix selection identity. A new replacement ID is required.

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
