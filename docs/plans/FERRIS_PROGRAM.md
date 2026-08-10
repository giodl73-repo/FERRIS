# Ferris Program

Status: Public-boundary drafts reviewed; implementation not authorized
Public product: **Ferris**
Primary command: `ferris`
Cargo entrypoint: `cargo ferris`
Internal planning model: Blueprint
Repository and research identifier: FERRIS

Program architecture:
[Ferris Seven-Program Architecture](FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)

## Product statement

> **Ferris is the cross-workspace enterprise build system for Rust.**

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

The initial command vocabulary is:

| Command | Purpose |
|---|---|
| `plan` | Produce a non-executable Blueprint Plan and reasons |
| `run` | Execute one explicitly approved plan or named activity |
| `affected` | Calculate changed scope since a revision or root |
| `graph` | Show application, workspace, task, and dependency mappings |
| `query` | Select typed scopes, plans, roots, refs, and evidence |
| `explain` | Explain selection, rebuild, wait, reuse, validation, or fallback |
| `check` | Run the declared affected check activity |
| `test` | Run the declared affected test and validation activity |
| `doctor` | Diagnose tools, configuration, environment, mappings, and evidence |

## Initial adoption wedge

The first proof is affected-only planning, checks, and tests across multiple
existing Cargo workspaces, with:

- no BUILD-file migration;
- no Cargo manifest replacement;
- a plan and explanation before execution;
- explicit omitted and unknown scope;
- full-reference comparison and conservative fallback;
- bounded machine-resource use;
- ordinary Cargo commands preserved; and
- complete removal without correctness changes.

The first proof is local and read-only for planning. Execution remains behind
a separately approved action boundary.

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
Role acceptance MUST NOT authorize implementation. Code requires all
applicable dependencies, frozen held-out fixtures, measurable stop criteria,
and a separately approved implementation pulse.

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

The public boundary is accepted as Draft, not Proposed. Before Proposed status:

- freeze three public repositories and exact revisions;
- define reproducible baseline and Ferris commands;
- fix numeric exit codes and machine schemas;
- define cold, incremental, check, build, test, and link measurements;
- add ABI, unsafe, security, privacy, operational, and removal fixtures;
- freeze governance policy, audit, connector manifest, maturity, and MCP
  schemas;
- prove permission, tenant isolation, revocation, secret non-persistence,
  CLI/MCP parity, prompt-injection, and connector-removal behavior;
- record supported and unsupported platforms and tool versions; and
- resolve the role objections in the public-contract review record.
