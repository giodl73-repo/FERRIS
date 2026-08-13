# Ferris Rust Engineering Library Nine-Role Review

Date: 2026-08-12
Disposition: Accepted as Guidance
Implementation authority: None
Base revision: `c401d35dae45e3b6cfbd5b41e4bbe67beac03977`
Reviewed corpus: 39 files
Corpus SHA-256: `28f29836dea5c06303c0fcef43589cbd74ffc6166eb5860e65cd7cde9eba8c9a`

The corpus digest is calculated from raw file bytes. For each path sorted by
case-insensitive repository-relative POSIX path, the digest input contains the
repository-relative path, one NUL byte, the lowercase SHA-256 of the raw file
bytes, and one newline. This review file is excluded so recording the digest
does not change the reviewed corpus. Recomputing that exact algorithm returns
`28f29836dea5c06303c0fcef43589cbd74ffc6166eb5860e65cd7cde9eba8c9a`.

## Complete corpus reviewed

- [Ferris Rust Engineering Library](README.md);
- all seven guides in
  [`ai-assisted-rust/`](ai-assisted-rust/00-OVERVIEW.md);
- all seven guides in
  [`platform-target-engineering/`](platform-target-engineering/00-OVERVIEW.md);
- all seven guides in
  [`validated-stack-profiles/`](validated-stack-profiles/00-OVERVIEW.md);
- all seven guides in
  [`maintainer-upstream/`](maintainer-upstream/00-OVERVIEW.md);
- all seven guides in
  [`reference-implementations/`](reference-implementations/00-OVERVIEW.md);
- [Ferris Rust engineering gap closure](../research/2026-08-12-ferris-rust-engineering-gaps.md);
- repository [`README.md`](../../README.md); and
- [`PRODUCT_PLAN.md`](../../PRODUCT_PLAN.md).

Each series was reviewed across the complete shared file set:

1. `00-OVERVIEW.md`;
2. `01-BOUNDARY-AND-OWNERSHIP.md`;
3. `02-OPERATING-WORKFLOW.md`;
4. `03-EVIDENCE-AND-IDENTITY.md`;
5. `04-FAILURE-MODES-AND-CONTROLS.md`;
6. `05-ADOPTION-ROLLBACK-AND-REMOVAL.md`; and
7. `06-VALIDATION-ROADMAP.md`.

## Review authorities

The review applied:

- [`CONTEXT.md`](../../CONTEXT.md) and [`AGENTS.md`](../../AGENTS.md);
- all nine files under [`.roles/`](../../.roles/ROLE.md);
- the [specification registry](../specs/README.md);
- [PRODUCT-001](../specs/FERRIS_PRODUCT_CONTRACT.md);
- [PLATFORM-001](../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md);
- [CONFORMANCE-001](../specs/FERRIS_CONFORMANCE_CONTRACT.md);
- the [Ferris program](../plans/FERRIS_PROGRAM.md);
- the [seven-program architecture](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md);
  and
- the source research cited by each series.

## Review question

Do the five series close the requested applied Rust engineering gaps while
keeping the work in FERRIS, preserving MAXIM as the canonical general Rust
reference, respecting external owner authority, and withholding unauthorized
product implementation?

## Executive disposition

All nine roles accept the library as **Guidance**.

The review accepts:

- FERRIS ownership of applied AI, platform, profile, upstream, and conformance
  workflows;
- MAXIM ownership of the canonical general Rust reference;
- one consistent seven-guide lifecycle across all five series;
- Cargo, rustc, platform systems, consumers, and upstream maintainers retaining
  their authority;
- exact identity, typed negative states, evidence expiry, renewal,
  substitution, rollback, removal, and retirement;
- AI as a proposal and explanation surface rather than an authority;
- profiles as renewable consumer records rather than a distribution or
  certification;
- upstream packets as local evidence until approved external action; and
- reference companions as future Conformance evidence rather than example
  code or current implementation.

No series advances a Draft specification, creates a stable schema, authorizes
runtime work, or expands the current implementation boundary.

## Findings and closure

The first complete role pass found six substantive issues. Acceptance was
withheld until each correction was applied.

| Severity | Finding | Roles | Closure |
|---|---|---|---|
| High | Platform guidance collapsed `expected-rejection` into failure detail or support state | Validation Checker, Native Platform Adopter, AI Assurance Skeptic | Added `expected-rejection` as an independent top-level stage result and kept `unsupported` as a separate owner/profile support state. |
| High | Validated profile families replaced three required PLATFORM-001 families with later domain extensions | Ecosystem Strategist, Interop Boundary Auditor, Native Platform Adopter, Scope Keeper, Validation Checker | Restored all nine PLATFORM-001 families and classified desktop/GUI, networking/protocol, and data/ML/GPU as later independent extensions. |
| Medium | All five lifecycle guides omitted PRODUCT-001's canonical Removal Record | Rust Maintainer, Native Platform Adopter, Scope Keeper, Validation Checker | Every `05-ADOPTION-ROLLBACK-AND-REMOVAL.md` now distinguishes capability cleanup from partial or complete Ferris removal and requires the canonical versioned Removal Record. |
| Medium | Repository foundation status stopped at Pulse 02 and omitted passive `doctor` and Pulse 13 | Rust Maintainer, Scope Keeper, Validation Checker | Updated `README.md` through Pulse 13, added `doctor`, and linked Pulse 04, Pulse 13, and the public-safe FHIF-030 result. |
| Medium | The review asserted exhaustive coverage without binding the full corpus or evidence | AI Assurance Skeptic, Scope Keeper, Validation Checker | Bound this record to the base revision and deterministic 39-file corpus digest; enumerated guides, authorities, validation, and findings closure. |
| Medium | Non-normative reference guidance used specification-level capitalized requirements | Scope Keeper, Validation Checker | Replaced the two uncited `MUST` forms with ordinary guidance language. |

No substantive finding remains open.

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept as Guidance.

The series consistently reject compiler or build success as proof of
behavior, safety, soundness, security, ABI compatibility, or platform support.
Unsafe Rust, FFI, panic, allocation, ownership, concurrency, provider, and
architecture assumptions require dedicated evidence and specialist review.

**Remaining gate:** Any executable unsafe, FFI, native, artifact, or generated
code capability requires an exact safety contract, positive and negative
fixtures, observed evidence, and a separately approved pulse.

### Compiler Performance Engineer

**Disposition:** Accept as Guidance.

The series distinguish cold, warm, incremental, check, build, test, link,
runtime, platform-matrix, maintainer, storage, and operational costs. They
require representative baselines, variance, causality, and correctness
controls rather than isolated timing claims.

**Remaining gate:** Future automation must freeze representative repositories,
environments, commands, repetitions, thresholds, and selected-versus-full
reference comparisons.

### Interop Boundary Auditor

**Disposition:** Accept as Guidance.

Platform, profile, upstream, AI, and companion guidance preserve semantic
contract, ABI, ownership, lifetime, allocation, panic, exception, threading,
linking, loading, generated binding, provider, and migration boundaries.

**Remaining gate:** Every claimed boundary requires exact positive, negative,
version-skew, migration, rollback, and removal fixtures on the supported
platforms.

### AI Assurance Skeptic

**Disposition:** Accept as Guidance.

The AI series records generated work as a proposal, separates model assertions
from owner evidence, uses deterministic authority for scope and mandatory
gates, keeps failures visible, and requires human approval proportional to
risk. The other series also prohibit AI-generated support, compatibility,
stewardship, and conformance claims.

**Remaining gate:** No AI capability may advance without frozen model and
instruction identity, provenance, privacy controls, adversarial fixtures,
false-omission thresholds, held-out evaluation, rollback, and removal proof.

### Ecosystem Strategist

**Disposition:** Accept as Guidance.

The library fills an operating-model gap instead of duplicating the Rust
language reference, Cargo, rustc, platform tools, package registries, standards
bodies, or current maintainers. Profiles avoid a Ferris distribution, and the
upstream series uses contribution, stewardship support, adapters, forks, and
deferral in increasing order of authority.

**Remaining gate:** Every future intervention must name a current owner,
consumer, compatibility boundary, maintenance commitment, expiry, and
contribute-versus-duplicate decision.

### Rust Maintainer

**Disposition:** Accept as Guidance.

The guides lead with ordinary Cargo, repository, platform, and upstream
vocabulary. Generated patches remain ordinary diffs; profiles and packets are
removable evidence; diagnostics identify owner and next action; and reference
companions retain owner-native full references.

**Remaining gate:** Representative maintainers must demonstrate reduced or
non-increased investigation cost, understandable output, preserved editor and
Cargo workflows, sustainable renewal, and complete removal.

### Native Platform Adopter

**Disposition:** Accept as Guidance.

The platform and profile series explicitly cover Linux, Windows, macOS,
mobile, WASM/WASI/browser, embedded, bare metal, RTOS, native dependencies,
SDKs, linkers, sysroots, runners, packaging, signing, deployment, debugging,
servicing, support, recovery, rollback, audit, training, and removal.

**Remaining gate:** Concrete support commitments require exact consumer
profiles, native execution, package and deployment evidence, unsupported
states, operational ownership, renewal, rollback, and removal on every claimed
platform.

### Scope Keeper

**Disposition:** Accept as a bounded documentation capability.

The work stays in FERRIS, leaves MAXIM unchanged, creates no new public product
or executable, and repeatedly marks automation, mutation, approval,
deployment, external posting, and companion code as deferred. The five series
map onto existing Ferris programs instead of creating new authorities.

**Remaining gate:** Future work must select one bounded consumer workflow and
must not infer implementation authority from this library.

### Validation Checker

**Disposition:** Accept as Guidance.

All five series define positive, negative, failure, expected-rejection,
unsupported, unavailable, not-observed, stale, unknown, version-skew,
cross-platform, rollback, removal, and held-out evidence where applicable.
The documentation set has complete guide structure, valid local links,
balanced code fences, ASCII-only new library and research files, no newly
introduced non-ASCII integration text, and clean whitespace. `PRODUCT_PLAN.md`
contains one pre-existing em dash outside the changed lines; it is not part of
the new-file ASCII gate.

**Remaining gate:** Proposed or implementation status still requires exact
fixtures, revisions, commands, schemas, semantic outputs, environments,
thresholds, observed results, and all nine role dispositions over the frozen
evidence.

## Required revisions completed

The reviewed library:

- places all five applied series in FERRIS and explicitly keeps MAXIM
  canonical for general Rust reference material;
- gives every series the same boundary, workflow, evidence, failure,
  lifecycle, and validation structure;
- identifies existing Ferris program and specification owners;
- states `Implementation authority: None` throughout;
- avoids a universal stack, certification, hidden resolver, environment
  mutation, unauthorized external action, or example-shaped conformance;
- includes ordinary Cargo fallback and complete removal;
- records typed negative, unavailable, stale, and unknown states;
- includes adoption, support, renewal, substitution, rollback, retirement, and
  stop conditions; and
- integrates the library into the repository README and product plan.

## Validation evidence

The corrected corpus produced:

| Check | Result |
|---|---|
| Series file contract | Five series, exactly seven guides each |
| Reviewed corpus binding | 39 files; SHA-256 `28f29836dea5c06303c0fcef43589cbd74ffc6166eb5860e65cd7cde9eba8c9a` |
| Repository-relative Markdown links | 0 broken |
| Balanced fenced code blocks | 0 unbalanced |
| Trailing whitespace | 0 findings |
| New engineering and placement files | ASCII only |
| README and product-plan additions | No newly introduced non-ASCII text |
| Reference-overview normative language scan | 0 uncited capitalized RFC 2119 terms |
| PRODUCT-001 Removal Record coverage | Present in all five lifecycle guides |
| Platform state coverage | `expected-rejection` present as a top-level state |
| `git diff --check` | Passed |

The corpus binding and structural checks were executed from
`C:\src\FERRIS` over `README.md`, `PRODUCT_PLAN.md`, the engineering index,
all 35 series guides, and the placement research note. The role-review record
was then updated with the resulting identity and is not part of its own
digest.

## Remaining gates

Before any series advances beyond guidance:

1. select one exact consumer workflow;
2. freeze repositories, revisions, owner-native commands, environments, and
   expected semantic outputs;
3. complete the applicable normative specification work;
4. define measurable pass, fail, stop, support, expiry, and removal criteria;
5. qualify development, calibration, harness, scorer, and held-out evidence as
   applicable;
6. execute Windows and Unix or other claimed platform evidence;
7. demonstrate ordinary Cargo preservation, rollback, cleanup, and complete
   removal;
8. repeat all nine role reviews over observed results; and
9. obtain a separately approved bounded implementation pulse.

## Decision

Adopt the five-series Ferris Rust engineering library as complete Guidance.

Do not authorize implementation.
