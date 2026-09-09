# Pulse 01: Strict Owner-Domain Selection

Status: Approved retroactively; implemented for closeout review

Implementation authority: Bounded to this document

Approval: Explicit repository-owner authorization on 2026-08-30

Budget: Production implementation consumed; review-and-fix closeout only

Closeout ceiling: One clean full role review, one clean Codex autoreview, and
one targeted-test pass. Any further defect requiring more than a localized fix
inside the authorized files ends this pulse; revert or defer that work to a
separately approved pulse rather than starting another review cycle.

## Goal

Extend the read-only `validation-plan` command so explicit changed paths can
select opaque repository-owned validation entrypoint IDs without transferring
command, workflow, environment, artifact, policy, or success authority to
Ferris.

## Authorized files

- `crates/ferris-core/`;
- `crates/ferris-cli/`;
- `AGENTS.md`, `README.md`, and `docs/plans/FERRIS_PROGRAM.md`;
- validation-plan schemas and directly related documentation;
- bounded development fixtures under `tests/fixtures/`;
- this wave and its review record; and
- `.gitattributes` only to preserve the existing CRLF contract of the edited
  owner-domain plan without a whole-file rewrite.

## Required behavior

- accept an optional closed `ferris.owner-validation-domains/v1` contract;
- require exact workspace identity and normalized, disjoint
  Cargo-workspace-root-relative prefixes;
- select stable owner-domain IDs and opaque entrypoint IDs without interpreting
  or executing them;
- compose filesystem-backed Cargo package selection with owner-domain
  selection;
- accept missing paths only through explicit `--deleted-path` lexical evidence;
- allow missing paths to select declared owner domains but never narrow Cargo
  package scope without filesystem evidence;
- retain visible full-workspace fallback for unknown or ambiguous Cargo effects;
- preserve no-contract output and the pinned baseline validation-plan identity;
  and
- keep structural schema validation separate from runtime semantic
  conformance.

## Prohibited behavior

- workflow YAML parsing or inference of npm, Python, browser, provider, or other
  owner command semantics;
- execution of owner entrypoints or Cargo validation activities;
- Git diff, rename, base/head, sibling, network, or hosted-provider discovery;
- caller-asserted narrowing of Cargo package scope for missing paths;
- external adopter workflow changes or savings claims; and
- release, platform, support, correctness, or CI-equivalence claims.

## Acceptance

- `ferris-core --lib` owner-domain and deleted-path behavior tests;
- focused `ferris-cli` validation-plan tests;
- published validation-plan schema and semantic tests with positive and
  negative controls;
- a pinned origin-main no-contract validation-plan identity;
- clean Rust Maintainer, Native Platform Adopter, Scope Keeper, AI Assurance
  Skeptic, and Product Value Governor dispositions;
- clean Codex autoreview; and
- `git diff --check`.

The no-contract identity constants were derived from `origin/main` commit
`c88b34ad9fa03fcc499cc35ef388a57d6ba1363b` with:

```console
cargo run -p ferris-cli --bin ferris -- validation-plan --workspace-id ferris.test/simple --manifest-path tests/fixtures/simple-workspace/Cargo.toml --changed-path tests/fixtures/simple-workspace/alpha/src/lib.rs --format json
cargo run -p ferris-cli --bin ferris -- validation-plan --workspace-id ferris.test/simple --manifest-path tests/fixtures/simple-workspace/Cargo.toml --changed-path tests/fixtures/simple-workspace/alpha/src/lib.rs --changed-package fixture-alpha --format json
```

The resulting plan identities are respectively
`validation-plan:e1b07ca49796836bc2b54b646fcbc391abb52b0384fd3472108a24642d8a8d7c`
and
`validation-plan:fb73ae3c12fcad53993e2b058038ed97519f00821764d195f80ab45976a3bc3e`.

## Stop conditions

Stop rather than widening this pulse if a fix requires owner command semantics,
workflow parsing, Git discovery, execution, unverifiable Cargo narrowing,
another architectural layer, or external adopter coordination.

## Removal

Remove the optional contract and deleted-path arguments, their additive record
fields and schemas, and their direct fixtures. Ordinary Cargo workflows and the
pre-existing no-contract `validation-plan` behavior remain unchanged.
