# Operating Workflow

Status: Guidance
Implementation authority: None

## Workflow outcome

The output is a locally reviewable contribution packet or a recorded decision
to document, adapt, support, defer, or retire. It is not an external post. The
packet contract is defined in
[RUST_PERFORMANCE_CONTRIBUTION_PACKET.md](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

## Step 1: State the maintainer question

Write one question that the likely owner can answer, such as:

- Is this a useful Secondary rustc-perf benchmark?
- Does this reproduce the accepted Cargo issue?
- Is this an expected behavior, diagnostic gap, or regression?
- Which compiler component should own this invalidation?
- Would this positive and negative fixture fit the crate's test suite?
- Is a documentation clarification preferable to a code change?

Reject questions that ask the owner to review the entire Ferris research
program, accept a broad architecture, or assume a patch is wanted.

## Step 2: Discover and renew routing

Apply the routing process in
[Boundary and ownership](01-BOUNDARY-AND-OWNERSHIP.md). Confirm the canonical
home, contribution instructions, supported toolchain and platform, likely
reviewers or team, and evidence that the question is in scope. If the routing
record is stale, renew it before further work.

## Step 3: Reproduce in an owner-native workflow

Start with ordinary commands and the owner's supported environment:

- name the working directory and exact source revision;
- record rustc, Cargo, rustup, backend, linker, LLVM, host, and target details
  as relevant;
- record lockfile, profile, features, targets, configuration, cache state,
  target directory, and environment assumptions;
- preserve expected exit status and bounded stdout and stderr;
- separate Cargo runtime, rustc compilation, linking, test, and application
  behavior; and
- rerun from a clean declared state.

Ferris-specific tooling may organize the record, but the core case must remain
understandable and runnable without Ferris.

## Step 4: Minimize without changing the mechanism

The minimal reproducer is the smallest current fixture that preserves the
distinguishing behavior and its correctness frontier.

Use this reduction loop:

1. Freeze the original observation and commands.
2. Remove one source, dependency, feature, target, environment input, or step.
3. Rerun the positive distinguishing case.
4. Rerun the negative and correctness controls.
5. Compare the observed mechanism and owner-relevant outcome.
6. Keep the reduction only if all required distinctions remain.
7. Record the minimization history and rejected reductions.

Do not minimize away build scripts, procedural macros, native inputs, feature
unification, ABI effects, generated files, platform behavior, or runtime
context when they are part of the mechanism.

## Step 5: Classify the smallest useful intervention

Choose exactly one primary request:

- confirm expected behavior;
- identify the correct owner;
- classify or prioritize an issue;
- accept a test or benchmark;
- review profiling evidence;
- advise on fixture or benchmark shape;
- accept a diagnostic or documentation improvement;
- review a focused patch; or
- decline or keep the case external.

Prefer diagnostics, fixtures, and documentation when they resolve the
maintainer question with lower maintenance burden than code. Prefer a focused
patch only after the owner confirms scope and direction.

## Step 6: Adapt to the owner

### rustc

- provide the smallest compiler case and exact toolchain;
- identify the suspected phase only as inference unless measured;
- include compile-success and behavior or diagnostic controls;
- preserve unsafe, ABI, macro, incremental, and cross-target boundaries; and
- use owner guidance for test placement and patch structure.

### Cargo

- begin from an accepted issue or explicit invitation;
- use Cargo's existing test and Criterion conventions;
- preserve filesystem, platform, resolver, fingerprint, and cache controls;
- show whether time is spent in Cargo, rustc, linking, or the test harness;
- do not rewrite manifests, lockfiles, or resolver semantics downstream.

### rustc-perf

- map local language to Check, Debug, Opt, Doc, DocJson, or Clippy;
- map scenarios to Full, IncrFull, IncrUnchanged, or IncrPatched;
- report instructions, cycles, wall time, or peak RSS as appropriate;
- include distributions and stable relevance interpretation;
- preserve correctness cases that must rebuild or relink; and
- follow the upstream benchmark registration and local collector checklist.

### Crates

- follow repository templates and MSRV, feature, platform, and release policy;
- minimize dependency churn and avoid opaque abstractions;
- add positive and negative semantic tests;
- document public API, runtime, cancellation, panic, unsafe, native, or
  generated-boundary consequences;
- include who will maintain the new test, fixture, document, or code.

### Standards

- demonstrate repeated product-neutral need and representative consumers;
- distinguish a descriptive mapping from a normative change;
- identify projection loss and compatibility limits;
- use the current RFC, proposal, or working-group process;
- do not present a Ferris adapter as ecosystem consensus.

## Step 7: Assemble the packet

Complete identity, maintainer question, reproducer, environment, vocabulary,
commands, evidence, correctness controls, requested action, and lifecycle.
Use the status transitions in
[Evidence and identity](03-EVIDENCE-AND-IDENTITY.md). The requested action
must be smaller than the evidence archive.

## Step 8: Review burden before external approval

Estimate:

- reviewer setup and reproduction time;
- patch, fixture, or benchmark review size;
- CI and platform cost;
- flake and regression triage cost;
- dependency and license review;
- documentation and release-note work;
- long-term ownership, renewal, and retirement cost; and
- response expected from the upstream owner.

Reduce or split the proposal when the burden is disproportionate to the
question. Funding review or maintenance is preferable to exporting an
unfunded obligation.

## Step 9: Obtain approval before posting

Before any external issue, comment, branch, benchmark request, pull request,
mail, chat message, or public artifact:

1. verify public-safe evidence and licenses;
2. verify current owner and intake route;
3. record organizational approval;
4. record the upstream contact or alignment, when required;
5. verify that no credential, private source, tenant data, internal path, or
   reusable secret is present;
6. freeze the submission-ready packet version.

Without approval, stop at a local packet.

## Step 10: Respond, renew, and retire

- Acknowledge owner response and route corrections.
- Update the packet rather than arguing from stale evidence.
- Keep upstream decisions separate from Ferris predictions.
- Renew environment, ownership, reproduction, license, and burden facts before
  resubmission or after material delay.
- Mark superseded work when a newer issue, benchmark, mechanism, or packet
  replaces it.
- Retire work that no longer reproduces, is declined without a consumer need,
  lacks a maintainer, or exceeds its support commitment.

The status and lifecycle record must remain useful even when no code merges.
