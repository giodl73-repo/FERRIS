# Ferris Program

Status: Draft architecture with an implemented, adopter-tested bounded product subset
Public product: **Ferris**
Primary command: `ferris`
Cargo entrypoint: `cargo ferris`
Internal planning model: Blueprint
Repository and research identifier: FERRIS

Program architecture:
[Ferris Seven-Program Architecture](FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)

## Product statement

> **Ferris is the cross-workspace enterprise build system for Rust.**

> **Go fast by proving what can be skipped.**

Ferris keeps Cargo as the authoritative Rust package, resolution, unit,
freshness, and compilation engine. It adds the application-level planning and
control layer needed to coordinate commands, workspaces, repositories,
contracts, validation, native tools, resources, evidence, packaging, and
deployment.

Ferris is therefore a build system in the enterprise sense: it owns the global
plan, policy, explanation, approval, and evidence contract. It does not
reimplement the local semantics of Cargo, rustc, linkers, test runners,
contract systems, native tools, or deployment systems.

The governing rule is:

> **The plan is global; the work is local.**

## Name and architecture

**Ferris** is the public product and executable, comparable in naming shape to
Clippy. **Blueprint** is the internal model and planning engine:

```text
Ferris product
  -> Application Definition
  -> Blueprint Model
  -> Blueprint Plan
  -> approved Action Plan
  -> owner-local execution
  -> Query Forest Root
  -> FERRIS Application Contract and evidence
```

Public documentation SHOULD say “Ferris” unless it specifically refers to the
Blueprint Model or Blueprint Plan. The previously proposed public commands
`ferris blueprint` and `cargo blueprint` are retired before implementation.

The existing `ferris` crates.io package is an unrelated timer-wheel library and
does not publish a binary. Ferris may still use the `ferris` executable name,
while published packages use qualified names such as `ferris-cli`,
`ferris-build-*`, and `cargo-ferris`. Availability observations are not
reservations.

See the
[Ferris product naming decision](../research/2026-08-10-ferris-product-naming.md).

The authoritative summary of that implemented subset is
[`Ferris Current Strategy and Feature Set`](FERRIS_CURRENT_STRATEGY_AND_FEATURES.md).

## Command surfaces

One semantic engine serves two adapters:

```console
ferris
cargo ferris
```

- `ferris` exposes application, repository, multi-workspace, contract,
  profile, policy, CI, packaging, deployment, root, and ref scope.
- `cargo ferris` is the Cargo-native current-workspace entrypoint implemented
  by a `cargo-ferris` executable.

The two entrypoints share command IDs, configuration, schemas, plans, policy,
output formats, exit semantics, evidence, and conformance. They differ only in
discovery defaults and available scope.

The current implemented command vocabulary is:

| Command | Purpose |
|---|---|
| `plan` | Produce a non-executable Blueprint Plan and reasons |
| `validation-plan` | Select conservative Cargo and owner-declared validation scope from explicit inputs or bound revisions |
| `federated-plan` | Collate independent plans for 2-16 explicitly declared Cargo workspaces |
| `federated-validation-plan` | Propagate validation through explicit consumer-owned application relationships |
| `revision-skew` | Report bounded local producer-consumer revision topology |
| `profile-diff` | Compare two explicit experimental profile records |
| `go` | Execute one explicitly approved Action Plan through its owner-native commands |
| `verify` | Verify deterministic execution-receipt integrity |
| `replay` | Compare local execution evidence with remote failure evidence |
| `schedule` | Replay one observed owner topology under bounded counterfactual profiles |
| `artifacts` | Report artifact compatibility, fan-in, and optional measured local-file qualification |
| `graph` | Show one Cargo workspace's package and dependency graph |
| `explain` | Explain one Cargo workspace plan |
| `doctor` | Diagnose tools, configuration, environment, mappings, and evidence |

`go` is the canonical public spelling for approved local execution.
Historical research and simulation records may retain the earlier working name
`run`; those records remain unchanged. A successful interactive `go` may end
with the human-facing line **"It's over. Go home."** Machine output and evidence
receipts MUST NOT include decorative text.

## Demonstrated adoption wedge

The demonstrated wedge is owner-first planning, validation, execution, and
evidence across existing repositories, with:

- no BUILD-file migration;
- no Cargo manifest replacement;
- a plan and explanation before any approved execution;
- owner-declared non-Cargo validation domains;
- local base/head/tested revision binding;
- explicit omitted and unknown scope;
- full-reference comparison and conservative fallback;
- bounded machine-resource use;
- ordinary Cargo commands preserved; and
- deterministic receipts and independent verification;
- artifact qualification and counterfactual replay; and
- complete removal without correctness changes.

PARLOR, RUNE, ICELINES, and BISECT exercise distinct parts of this wedge.
Execution remains behind an explicit approved Action Plan, and every adopter
retains its owner commands, required workflows, and success policy.

## Final specification set

Ferris specification work is grouped into four gates.

### Gate A: Product, governance, and application contract

1. PRODUCT-001 - public identity, category, namespace, authority, entrypoints,
   compatibility, and removability.
2. GOVERNANCE-001 - principals, authorization, policy, approval, tenancy,
   data, secrets, audit, budgets, and revocation.
3. CONTRACT-001 - Rust API, semantic, ABI, component, and wire contracts.
4. PLATFORM-001 - supported profiles and lifecycle.
5. APPLICATION-001 - Application Definition, Blueprint Model, roots, and
   FERRIS Application Contract.

### Gate B: Planning truth

6. SCOPE-001 - multi-dimensional scope and mappings.
7. FOREST-001 through FOREST-003 - component model, canonical schema, maps,
   ledgers, projections, and consistency.
8. IDENTITY-001 and EVIDENCE-001 - identity, lineage, adapters, and ownership.
9. CAUSALITY-001 and PREDICTION-001 - explanation, uncertainty, and held-out
   evaluation.
10. VALIDATION-001 and PLANNING-001 - coverage, owner closures, Cargo plans,
   resources, fallback, and adaptive replanning.

### Gate C: Controlled action and trust

11. RESOLUTION-001 - plan selection and human decision.
12. TRUST-001 - provenance, privacy, security, ref authority, retention,
    revocation, and deletion.
13. EXECUTION-001 - approved action projection, rollback, cleanup, and audit.
14. CONNECTOR-001 - replaceable connector manifests, Microsoft profiles,
    owner semantics, failure, lifecycle, and governed MCP.
15. FERRIS-001 - evidence and upstream contribution packets.

### Gate D: Public contract and proof

16. VIEW-001 - `ferris`, `cargo ferris`, and governed MCP commands, scope defaults, outputs,
    explanations, and exit semantics.
17. CONFORMANCE-001 - CLI/MCP parity, positive and negative fixtures,
    fallback, removal, cross-platform behavior, and acceptance thresholds.

PRODUCT-001, GOVERNANCE-001, CONNECTOR-001, VIEW-001, and CONFORMANCE-001
define the final public and enterprise boundary. The internal specifications
define the evidence required to implement it correctly.

## Program ownership

The specification set is organized through seven bounded programs:

1. Ferris - public build-system product and command;
2. Typebook - product-neutral semantic contracts;
3. Profiles - renewable support and compatibility records;
4. Blueprint - application model and federated planning;
5. Query Forest - evidence, causality, identity, roots, and history;
6. Conformance - executable proof, validation, and removal; and
7. Ecosystem Bridge - adapters, owner routing, and upstream contributions.

Ferris is the only build-system product. Typebook remains independently useful.
The other five are programs and replaceable capability boundaries, not
mandatory public products or executables.

Ferris includes a portable Enterprise Governance Plane. Ecosystem Bridge
includes a governed Connector and MCP Framework. The Microsoft connector pack
is the first named enterprise integration profile, not a required dependency
or eighth program. See
[Ferris Microsoft Enterprise Integration](FERRIS_MICROSOFT_ENTERPRISE_INTEGRATION.md).

## Review result and implementation gates

The nine-role review of the Ferris program and three public-boundary
specifications explicitly tested:

- safety claims and validation preservation;
- measured value rather than build-system branding alone;
- native and cross-language boundary fidelity;
- AI narrowing, authority, provenance, and failure behavior;
- ecosystem duplication and upstream ownership;
- maintainer simplicity and ordinary Cargo preservation;
- platform adoption, support, rollback, and removal;
- bounded scope and deferred capabilities; and
- executable conformance and held-out controls.

The review accepted the documents as Draft and withheld Proposed status. See
the
[Ferris public-contract review](../specs/reviews/FERRIS-PUBLIC-CONTRACTS-ROLE-REVIEW.md).
The broader program architecture is accepted as Draft in the
[Ferris seven-program review](reviews/FERRIS-SEVEN-PROGRAM-ROLE-REVIEW.md).
The governance and connector additions are accepted as Draft in the
[Ferris Microsoft integration review](reviews/FERRIS-MICROSOFT-INTEGRATION-ROLE-REVIEW.md).
The complete 22-specification spine is accepted as coherent Draft architecture
in the
[Ferris specification convergence review](../specs/reviews/FERRIS-SPECIFICATION-CONVERGENCE-ROLE-REVIEW.md).
Role acceptance does not authorize implementation by itself. Separately
reviewed bounded pulses now authorize the implemented command subset listed
above, including owner-domain and revision-bound validation, approved local
Action Plan execution, receipt verification, replay, scheduling analysis, and
artifact qualification. Their individual contracts and evidence define the
exact authority.

The broader Draft architecture still does not authorize connector or MCP
execution, AI narrowing, remote execution, live scheduling, credential
handling, workflow mutation, publication, or deployment.

## Specification simulation gate

Before any specification advances to Proposed, Ferris MUST run no-code
scenario waves under the
[Ferris Specification Simulation Program](../simulations/FERRIS_SPECIFICATION_SIMULATION_PROGRAM.md).

Each wave MUST:

- freeze fixtures and specification versions;
- hand-derive expected records, states, failures, and public views;
- classify outputs as simulated rather than observed;
- record `FSIM-SI-*` Simulation Issues;
- apply normative fixes through `FSIM-SCR-*` Specification Change Records;
- retrace every affected scenario;
- include applicable nine-role dispositions; and
- preserve implementation authority as None.

Simulation validates Draft consistency. It does not satisfy executable
CONFORMANCE-001 fixtures or authorize code.

## Claim boundaries

Ferris MUST NOT claim:

- replacement of Cargo or rustc;
- hermeticity without complete declared-input isolation;
- Bazel- or Buck2-equivalent remote execution;
- deterministic incremental correctness without validated invalidation;
- cache correctness from matching keys alone;
- validation coverage from package selection alone;
- universal language support without implemented adapters; or
- official Rust Project or Rust Foundation affiliation.

## Remaining blockers

The public boundary is accepted as Draft, not Proposed. Current product
priorities are:

- simplify installation, Action Plan preparation, and owner adapter authoring;
- define compatibility, versioning, and support policy for the smallest useful
  public records;
- add materially different adopter and failure evidence;
- remove the Windows long-path constraint in the retained fixture corpus; and
- preserve every existing owner workflow until a separate repository-specific
  reconciliation proves safe narrowing or deletion.

The broader enterprise architecture additionally requires frozen governance,
connector, tenancy, credential, MCP, publication, deployment, and support
contracts with executable conformance. Those are not implied by the current
bounded implementation.
