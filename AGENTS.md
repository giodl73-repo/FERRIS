# Ferris Agent Instructions

Read `CONTEXT.md` first. It is the canonical operating context for this
repository.

## Working rules

- Use `docs/plans/FERRIS_PROGRAM.md` for product authority and sequence.
- Use `docs/specs/README.md` for normative specification status and
  dependencies.
- Read the applicable `.roles` files before claiming a role review.
- Preserve Cargo and every external owner's authority; do not create parallel
  resolvers, hidden manifests, or synthetic owner truth.
- Keep Typebook/RUNE product-neutral and independently usable.
- Keep observation, planning, approval, and execution as separate records and
  authorities.
- Treat unknown, unsupported, unavailable, stale, failed, and not-observed as
  distinct states.
- Never place credentials or reusable secrets in plans, prompts, roots, refs,
  logs, fixtures, or evidence.
- Preserve ordinary Cargo and owner-native workflows and define removal and
  rollback for every adoption.

## Change authority

The closed bounded pulses in
`context/waves/2026-08-11-read-only-planning/pulses/`
authorize only local read-only `plan`, `explain`, declared-workspace `graph`,
passive local `doctor`, and the Pulse 14 two-file experimental
`profile-diff` product code over explicit local inputs and development
fixtures. Pulse 15 authorizes only the nine-family development fixture matrix
and conformance tests for that existing command; it adds no product behavior.
Pulse 16 authorizes only the public held-out contract, custody protocol, and
review. It does not authorize hidden fixture construction by implementation
authors, scoring, or oracle access. Pulse 17 authorizes only Windows and Unix
development validation evidence for the existing implementation and fixtures.
Pulse 18 authorizes only the public-CLI filesystem immutability test and its
bounded evidence. Pulse 19 authorizes only the representative ordinary-Cargo
consumer fixture, before-and-after conformance test, and evidence. Do not
implement profile generation, affected-only scope, `query`, `run`, mutation,
active probes, connectors, MCP, AI narrowing, approval, deployment, remote
evidence, or held-out oracle access. Any later product capability requires
another separately approved pulse.

The active
`context/waves/2026-08-12-platform-profile-conformance/` wave currently
authorizes Pulse 01 documentation and governance plus Pulse 02's frozen
`ferris.platform-profile/v1` schema documents, incomplete exemplar, exact
negative-control mutations, and review. Pulse 03 additionally authorizes one
dependency-free test-only Rust harness that executes those controls. Do not
add production schema types. Pulse 04 authorizes only the pure-data family,
its two zero-dependency consumer revisions, isolated locked/offline owner
commands, source snapshots, and test-only profile materialization. Do not
add external dependencies or generate a product profile. Pulse 05 additionally
authorizes only the CLI/configuration family and reusable integration-test
support for later controlled families. Do not complete another family,
construct hidden held-out material, score an oracle, or change PLATFORM-001
status until the corresponding later pulse and role review grant that exact
authority.

Pulse 06 additionally authorizes only the in-process hosted-service family.
Do not add sockets, network, databases, TLS, credentials, deployment, or
production service behavior.

Pulse 32 is now permanently closed `invalid` at `cutoff-build-freeze`.
Checkout and package gates passed, but custody could not freeze the Ubuntu
executable; zero preflight, public-input validation, cases, or candidates ran,
and the conclusion is null. Pulse 33 authorizes only the public external
build-freeze release, root-cause record, governance, review, and test-only
validation. It records a WSL non-login `PATH` omission, explicit rustup Cargo
discovery, Cargo JSON artifact output, and deterministic Windows/Ubuntu
build hashes without executing a diagnostic or changing product code.

Pulse 34 authorized one independent diagnostic at an immutable cutoff that
contains the complete Pulse 33 release and predates the authority. Public
checkout, binding, package, build-freeze, adapter-preflight, and input gates
passed, but isolated corpus materialization did not complete. Pulse 34 is
permanently closed `invalid` with zero candidate launches, a null conclusion,
and no retry or reuse authority.

Pulses 69 through 82 are sealed successor infrastructure only. The latest
callable chain terminates at Pulse 82 over the Pulse 78 staging/bootstrap
hardening and Pulse 81 exact Pulse 35 release-tree binding. These releases
grant no diagnostic authority and perform no real FERRIS execution. Any
successor authority requires a separate approved pulse with an immutable
pre-authority cutoff.

## Research and specifications

- Inventory local evidence before using external sources.
- Cite actionable research claims with files, line ranges, URLs, or measured
  commands.
- Give findings stable `FERRIS-*` identifiers; retain historical `FERRIUM-*`
  identifiers unchanged.
- Use MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY according to
  `docs/specs/README.md`.
- Keep specification dependencies acyclic and owner-aligned.
- Do not mark a specification Proposed or Adopted without its stated fixtures,
  measurements, and role approvals.
- A review must record all nine role dispositions, completed revisions,
  remaining gates, and implementation authority.

## Validation and commits

- Validate Markdown links and code fences for changed documentation.
- Run `git diff --check`.
- Review staged paths and the specification dependency graph before committing.
- Keep logical changes in focused commits.
- Include the required Copilot co-author trailer when applicable.
- Do not push unless requested.
- Keep Ferris child-repo commits separate from TRACKER submodule-pointer
  updates.
