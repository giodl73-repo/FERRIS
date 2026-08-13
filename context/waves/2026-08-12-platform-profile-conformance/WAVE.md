# Wave: Platform Profile Conformance

Status: Active; Pulses 01 through 15 complete

## Goal

Produce the exact controlled evidence, lifecycle controls, independent
held-out results, and nine-role review required to decide whether
PLATFORM-001 can advance from Draft to Proposed.

The wave covers all nine required profile families. It does not authorize
profile generation, affected-only scope, execution, mutation, connectors,
MCP, AI narrowing, approval, deployment control, remote evidence collection,
or any support claim.

## Classification

Specification, fixture, and conformance wave with separately bounded pulses.

## Owner actions

| Owner | Action |
|---|---|
| FERRIS | Define contracts, controlled fixtures, test-only harnesses, measured evidence, lifecycle records, and role reviews |
| Cargo and external systems | Retain authority; validation harnesses invoke only exact documented owner commands |
| Independent validation custodian | Select sealed changes, construct hidden material, execute held-out packages, and score once |
| Three public repositories | Remain unchanged by implementation authors until publicly selected and frozen for the held-out program |
| TRACKER | Defer until child commits are complete and a separate portfolio snapshot is requested |

## Pulse table

| Pulse | Title | Status | Authority |
|---:|---|---|---|
| 01 | Program foundation | Complete | Wave boundary, requirement map, sequencing, stop criteria, and role review only |
| 02 | Canonical profile fixture contract | Complete | Versioned schema, projection boundary, malformed controls, and schema review |
| 03 | Shared evidence harness | Complete | Test-only frozen-control schema validation; owner-command collection remains deferred |
| 04 | Pure data family | Complete | Exact controlled `r1` and `r2` family evidence |
| 05 | CLI and configuration family | Complete | Exact controlled `r1` and `r2` family evidence plus reusable test-only family support |
| 06 | Hosted service family | Complete | Exact controlled in-process `r1` and `r2` service evidence |
| 07 | Embedded and `no_std` family | Complete | Exact controlled `r1` and `r2` family evidence |
| 08 | Browser WASM family | Complete | Exact controlled `r1` and `r2` family evidence |
| 09 | WebAssembly component family | Complete | Exact controlled `r1` and `r2` family evidence |
| 10 | Native dependency family | Complete | Exact controlled `r1` and `r2` family evidence |
| 11 | Identity and provider family | Complete | Exact controlled `r1` and `r2` family evidence |
| 12 | Assurance, packaging, and deployment family | Complete | Exact controlled `r1` and `r2` family evidence |
| 13 | Cross-family closure and identity conformance | Complete | Exact closure, identity, evidence-class, projection, and relocation proof |
| 14 | Renewal and exact rollback | Complete | One bounded real renewal and exact restoration |
| 15 | Substitution and emergency response | Complete | Provider substitution, emergency state, containment, and rollback |
| 16 | Adoption, ordinary Cargo, and removal | Authorized | Per-family owner workflows and one canonical Removal Record |
| 17 | Independent held-out program | Planned | External 56-case score and three-public-repository workflow |
| 18 | PLATFORM-001 Proposed review | Planned | Acceptance matrix, dependency reconciliation, and nine-role disposition |

No planned pulse has implementation authority until its pulse document and
applicable role review are complete.

## Required evidence order

1. freeze the canonical fixture contract;
2. build only the test support needed to validate it;
3. complete each family independently;
4. verify closure, identity, evidence, and projection invariants across all
   families;
5. execute renewal, substitution, emergency, rollback, and removal;
6. finish independent held-out collection and scoring; and
7. review PLATFORM-001 against measured results.

Family fixtures may share test support. One family must not stand in for
another, and shared dependencies must not erase target, runtime, provider,
native, contract, operational, or lifecycle differences.

## Common validation

Every code-bearing pulse must run:

```console
cargo fmt --all -- --check
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
git diff --check
```

Applicable owner commands must also run on Windows and Ubuntu 24.04.4 WSL2
with exact toolchain, environment, target, cache, network, and target-directory
state recorded. Native Linux, browser, device, component-runtime, signing, or
deployment claims require their actual owner environment rather than WSL or
compilation as a substitute.

Documentation-only pulses validate changed links, code fences, schema
references, ASCII policy, staged paths, and Git diff hygiene.

## Commit boundaries

- Keep pulse authority and review records separate from later product code.
- Keep exact implementation cutoffs separate from validation receipts when a
  receipt cites an immutable commit.
- Keep each family and each lifecycle control independently revertible.
- Never combine independent held-out material with implementation-author
  commits.
- Do not mix FERRIS commits with TRACKER submodule-pointer updates.

## Stop conditions

Stop or redesign if work requires:

- a parallel resolver, hidden manifest, mandatory registry, or Ferris-owned
  source of Cargo truth;
- changing owner manifests, locks, features, targets, providers, credentials,
  deployment, or support without separate authority;
- presenting controlled fixtures as ecosystem support;
- converting failed, stale, revoked, unsupported, unavailable, not-observed,
  or unknown evidence into success;
- treating compilation, tests, signatures, or profiles as safety, security,
  semantic compatibility, correctness, approval, or support;
- implementation-author access to hidden held-out inputs, canaries, oracle
  predicates, or expected records;
- inability to execute exact rollback or complete removal;
- ordinary Cargo failure after removal; or
- advancing PLATFORM-001 with an unmet acceptance or dependency gate.

## Completion gate

- all nine exact family revisions are frozen;
- lock universe and target-active closures are independently verified;
- compiler, host, target, native, runtime, provider, contract, and deployment
  states remain explicit;
- required positive and non-success stage states are executable;
- assurance, stewardship, support, expiry, renewal, substitution, emergency,
  rollback, and removal evidence is complete;
- ordinary Cargo and non-Ferris consumers remain functional;
- both independent held-out packages are valid;
- all nine roles review measured results; and
- PLATFORM-001 either advances to Proposed or remains Draft with explicit
  blockers.
