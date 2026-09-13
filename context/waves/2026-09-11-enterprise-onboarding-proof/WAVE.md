# Wave: Enterprise Onboarding Proof

Status: Active; Pulse 03 implementation in progress
Implementation authority: One bounded WSL PowerShell prerequisite and onboarding proof
Successor authority: None

## Systems-development gap

Ferris now has a qualified deterministic regression corpus and bounded adopter
evidence, but it does not yet have a fresh-repository onboarding proof that
starts from ordinary owner workflows, measures the adoption burden, and proves
complete removal. Existing synthetic repositories were designed primarily for
selection and boundary conformance, not first-use product onboarding.

The measurable wave decision is whether fresh enterprise-shape repositories can
adopt a pinned Ferris build, author explicit owner-controlled inputs, exercise
`plan`, `go`, and `verify`, retain their existing owner commands and CI, and
remove Ferris without changing owner correctness.

## Research basis

- The
  [`current strategy`](../../../docs/plans/FERRIS_CURRENT_STRATEGY_AND_FEATURES.md)
  prioritizes installation, Action Plan preparation, repository-owned adapters,
  and removal before affected-only promotion.
- The
  [`post-portability corpus qualification`](../../../docs/research/2026-09-10-post-portability-private-enterprise-corpus-qualification.md)
  establishes deterministic two-platform behavior but explicitly does not
  establish onboarding burden or production value.
- The
  [`real-history shadow`](../../../docs/research/2026-09-09-ferris-real-history-shadow.md)
  widened 37 of 40 revisions, showing that declaration coverage and owner
  command shape matter more than additional narrowing logic.
- Existing PARLOR, RUNE, ICELINES, and BISECT evidence proves bounded slices,
  but each began from an established repository rather than a controlled fresh
  onboarding baseline.

## Candidate consumers

Private identities remain in custody. Public governance uses:

- `EO-01`: one ordinary Cargo workspace with multiple packages and one
  non-Cargo owner domain; and
- `EO-02`: one repository containing three independent Cargo workspaces and one
  explicit application relationship graph.

## Pulse sequence

| Pulse | Title | Status | Decision |
|---:|---|---|---|
| 01 | Owner-native enterprise baselines | Complete; incomplete | Windows owner baselines passed; hosted Actions unavailable; no tags frozen |
| 02 | Pinned Ferris onboarding and removal | Complete; incomplete | WSL had Rust 1.95 and Git but no Linux `pwsh`; stopped before consumer mutation or execution |
| 03 | WSL prerequisite and resumed onboarding | Authorized; in progress | Install one verified removable PowerShell archive, run the unchanged two-platform proof, then remove the environment prerequisite |

## Pulse 02 authority

The owner explicitly approved Windows plus local WSL 2 Ubuntu after hosted
enterprise Actions remained administratively unavailable. Pulse 02 is limited
to public Ferris revision
`b347dc34d62810f043122d0d279def316b890cf0`, Ferris-owned removable files in
`EO-01` and `EO-02`, unchanged owner-command baselines, approved local `plan`,
`go`, and `verify` evidence, complete removal, and unchanged post-removal
validation.

Local WSL evidence is local two-platform evidence. It is not hosted-CI,
clean-runner, support, production, affected-only, performance, or savings
evidence.

The WSL preflight found Cargo and rustc 1.95.0 plus Git, but no Linux `pwsh` in
`PATH`, the standard installation locations, or the package database. The
unchanged owner commands therefore could not run on the authorized Linux
platform. Pulse 02 stopped at its capability gate without translating an owner
command, installing a dependency, changing either consumer, creating a tag,
running a baseline, preparing an Action Plan, or invoking Ferris in a consumer.
No retry or successor authority follows.

## Pulse 03 authority

On 2026-09-13 the owner explicitly directed the onboarding proof to continue.
Pulse 03 authorizes one corrective successor: install the official PowerShell
7.6.6 Linux x64 binary archive in local WSL 2 Ubuntu after verifying its
published SHA-256 digest, expose only the `pwsh` executable, run the unchanged
Pulse 02 Windows/WSL baseline and removable onboarding protocol, and remove the
installed archive and symlink after all post-removal owner validation.

The environment setup is measured separately from owner commands and Ferris
execution. Ubuntu 26.04 is not converted into a Microsoft-supported platform by
the archive installation. No APT repository, Snap daemon, owner-script
translation, consumer dependency, credential, hosted-CI claim, support claim,
or product change is authorized.

## Pulse 01 scope

Affected surfaces:

- two new private repositories and their ordinary Cargo sources;
- repository-owned formatting, lint, test, and deterministic integrity tools;
- hosted Ubuntu owner CI;
- topology and scenario documents that contain no Ferris output or expectation;
- immutable owner-baseline tags; and
- private custody plus public-safe aggregate governance records.

No Ferris crate, schema, fixture, command, adapter, or consumer contract changes
in Pulse 01.

## Completion condition

- both private repositories exist at exact clean default-branch revisions;
- each repository has an explicit owner correctness command independent of
  Ferris;
- `EO-01` proves its ordinary multi-package Cargo baseline;
- `EO-02` proves three independently testable Cargo workspaces and its declared
  application topology without combining Cargo resolution;
- Windows and hosted Ubuntu owner checks pass;
- deterministic generation or integrity checks reproduce checked-in topology;
- baseline scenario and removal invariants are documented;
- immutable owner-baseline tags are frozen; and
- exact private identities, revisions, paths, run IDs, and raw output remain in
  private custody.

## Stop conditions

Stop rather than expand if Pulse 01 requires:

- any Ferris product or public contract change;
- adding Ferris to a consumer repository;
- an Action Plan, owner-entrypoint adapter, or Ferris-generated evidence;
- cross-repository credentials, reusable secrets, or privileged automation;
- workflow parsing, inferred owner commands, or hidden relationships;
- combining Cargo resolution across independent workspaces;
- a third consumer or another topology;
- production representativeness, support, performance, or savings claims; or
- beginning Pulse 02 without separate approval.

## Reviewers

Required reviewers are Rust Safety Steward, Compiler Performance Engineer,
Interop Boundary Auditor, AI Assurance Skeptic, Ecosystem Strategist, Rust
Maintainer, Native Platform Adopter, Scope Keeper, Validation Checker, Product
Value Governor, and Autonomy Supervisor.

## Shared-contract boundary

Any later reusable onboarding record must remain product-neutral where
possible. Pulse 01 creates owner repositories only; it must not make Typebook,
RUNE, Cargo, or owner command semantics depend on Ferris.

## Non-goals

- product implementation or schema promotion;
- affected-only execution or workflow deletion;
- connector, MCP, remote execution, cache, signing, publication, or deployment;
- AI-generated narrowing or command inference;
- production support or service-level commitments; and
- realized time, cost, or prevented-iteration claims.
