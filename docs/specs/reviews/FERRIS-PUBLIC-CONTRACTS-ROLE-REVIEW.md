# Ferris Public Contracts Nine-Role Review

Date: 2026-08-10
Documents:

- Ferris Program;
- PRODUCT-001 Ferris Product Contract;
- VIEW-001 Ferris Command and View Contract; and
- CONFORMANCE-001 Ferris Conformance Contract.

Disposition: Accepted as Draft after required revisions
Implementation authority: None

## Review question

Do the Ferris program and public-boundary specifications define a credible,
bounded cross-workspace enterprise build system for Rust while preserving
Cargo authority, safety, ecosystem ownership, maintainability, operational
trust, removability, and executable proof?

## Executive disposition

All nine roles accept the public boundary as **Draft** after the revisions
recorded below.

The review accepts:

- Ferris as the public product and executable name;
- Blueprint as the internal normalized model and non-executable plan;
- `ferris` and `cargo ferris` as adapters over one semantic engine;
- Cargo and every participating system retaining local semantic authority;
- affected-only multi-workspace checks and tests as the first wedge;
- planning before execution and explicit approval for action;
- conservative widening and full-reference fallback;
- ordinary Cargo preservation and complete removability; and
- strict claim boundaries around safety, correctness, hermeticity, caching,
  remote execution, and official Rust affiliation.

Proposed status is withheld until exact repositories, fixtures, commands,
schemas, exit codes, supported versions, and measurable thresholds are frozen.

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept as Draft after strengthening safety boundaries.

**Required revisions:**

- prohibit build, compiler, or selected-test success from proving safety,
  soundness, security, ABI compatibility, or behavior;
- require ownership, lifetime, aliasing, concurrency, panic, toolchain, and
  unsafe-boundary evidence; and
- require a separate gate for adapters using `unsafe`, FFI, compiler-private
  state, or artifact restoration.

**Result:** Incorporated into PRODUCT-001 and C-SAFETY.

### Compiler Performance Engineer

**Disposition:** Accept the product wedge; withhold performance claims.

**Required revisions:**

- distinguish cold, warm, incremental, check, build, test, and link workflows;
- record hardware, toolchain, filesystem, cache state, target topology,
  commands, repetitions, variance, failures, and limitations; and
- compare selected-only execution with the full reference.

**Result:** Incorporated into C-PERF. Exact fixtures and thresholds remain
open.

### Interop Boundary Auditor

**Disposition:** Accept as Draft after adding an explicit interop suite.

**Required revisions:**

- cover ABI, ownership, lifetime, allocation, panic, exception, unwind,
  threading, layout, bindings, linking, loading, and runtime use;
- require positive, negative, and failure fixtures; and
- preserve incremental migration and removal.

**Result:** Incorporated into C-INTEROP.

### AI Assurance Skeptic

**Disposition:** Accept with evidence and approval constraints.

**Required revisions:**

- retain source revision, model and instruction identity, proposed scope,
  policy or approval, commands, outcomes, rejection, and rollback;
- forbid AI-only semantic similarity from reducing work; and
- require dedicated security, performance, behavior, safety, and soundness
  evidence.

**Result:** Incorporated into C-AI and VIEW-001.

### Ecosystem Strategist

**Disposition:** Accept the category and wedge as Draft.

**Accepted:**

- Bazel and Buck2 remain the replacement-system comparison;
- Nx remains the affected-task UX comparison;
- Cargo, nextest, sccache, Nix, Dagger, task tools, and CI remain owners or
  complements; and
- Blueprint is no longer presented as a second public product.

**Remaining objection:** the three public proof repositories and upstream
collaboration targets are not frozen.

### Rust Maintainer

**Disposition:** Accept after strengthening explanations and removability.

**Required revisions:**

- lead with ordinary Cargo and repository vocabulary;
- link every material explanation to evidence;
- preserve editor and Cargo workflows; and
- prove removal without correctness changes.

**Result:** Incorporated into PRODUCT-001, VIEW-001, and C-REMOVE.

### Native Platform Adopter

**Disposition:** Accept for specification; reject operational adoption today.

**Required revisions:**

- name supported tools, platforms, ABIs, and deployment models;
- record installation, upgrade, training, support, compliance, privacy,
  recovery, rollback, audit, removal, and maintenance cost; and
- make unsupported platforms explicit.

**Result:** Incorporated into PRODUCT-001, C-PLATFORM, and C-OPS. Concrete
support commitments remain open.

### Scope Keeper

**Disposition:** Accept as a bounded public-boundary decision.

**Required revisions:**

- keep reusable schemas and shared libraries product-neutral where practical;
- keep consumer workflows in adapters, application definitions, or policy;
- preserve the affected-work first wedge; and
- retain remote execution, hermeticity, cache restoration, CI replacement, and
  universal-language support as deferred.

**Result:** Incorporated into PRODUCT-001 and the Ferris Program.

### Validation Checker

**Disposition:** Accept as Draft; Proposed status withheld.

**Accepted:**

- positive, negative, failure, unsupported, stale, version-skew, rollback, and
  removal classes are required;
- entrypoint parity and selected/full-reference comparison are explicit; and
- Windows and Unix proof is required.

**Remaining objection:** reproducible commands, exact fixtures, expected
outputs, fixed schemas, numeric exit codes, and measurable thresholds do not
yet exist.

## Required revisions completed

The reviewed documents now include:

- explicit Ferris-versus-Blueprint terminology;
- a one-engine, two-adapter contract;
- owner-local semantic authority;
- safety and unsafe-boundary limits;
- maintainer-facing evidence-linked explanations;
- privacy-aware output rules;
- interop, performance, operations, AI, failure, platform, and removal suites;
- product-neutral shared-contract guidance; and
- visible Proposed-status blockers.

## Remaining gates

The public contracts remain Draft until:

1. three public repositories and exact revisions are selected;
2. held-out changes and raw Cargo/script baselines are frozen;
3. all commands and expected outputs are specified;
4. machine schemas and numeric exit codes are fixed;
5. supported and unsupported tools, versions, platforms, and ABIs are named;
6. measurable success, failure, and stop thresholds are fixed;
7. privacy, retention, support, upgrade, recovery, and removal procedures are
   executable; and
8. the nine roles re-review the completed fixtures.

## Decision

Advance PRODUCT-001 and VIEW-001 to **Draft after nine-role review**.

Advance CONFORMANCE-001 to **Draft framework after nine-role review**.

Do not advance any document to Proposed and do not authorize implementation.
