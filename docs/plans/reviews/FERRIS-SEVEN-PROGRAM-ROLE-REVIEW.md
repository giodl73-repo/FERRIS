# Ferris Seven-Program Nine-Role Review

Date: 2026-08-10
Plan: Ferris Seven-Program Architecture
Research: Ferris Seven-Program Synthesis and EXP-01 closure matrix
Disposition: Accepted as Draft after required revisions
Implementation authority: None

## Review question

Does the seven-program architecture preserve the complete research corpus
while keeping product, contract, profile, planning, evidence, conformance, and
external-owner responsibilities sufficiently clear, bounded, reviewable, and
removable?

## Executive disposition

All nine roles accept the architecture as Draft.

The review accepts:

- one public Ferris product and CLI;
- Typebook as a separate product-neutral contract program;
- Profiles as renewable consumer support records;
- Blueprint as the planning program;
- Query Forest as the evidence and history program;
- Conformance as a continuous gate;
- Ecosystem Bridge as adapter and upstream-owner routing; and
- explicit cross-program contracts rather than implicit shared truth.

The review does not advance any implementation or unresolved specification to
Proposed.

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept after adding a cross-program assurance rule.

**Required revision:** no program may derive behavioral, safety, soundness,
security, ABI, ownership, lifetime, aliasing, concurrency, panic, or unwind
claims from build success or another program's record alone.

**Result:** Cross-program contracts require owner, evidence, scope, lifecycle,
and conformance. Typebook and Conformance retain semantic and negative-test
responsibility.

### Compiler Performance Engineer

**Disposition:** Accept if the architecture retains one measured developer
workflow.

**Required revision:** program decomposition must not replace representative
cold, warm, incremental, check, build, test, link, resource, and
selected/full-reference measurements with program-local metrics.

**Result:** Conformance owns the shared measurement contract and Blueprint
retains global resource and workflow planning.

### Interop Boundary Auditor

**Disposition:** Accept after clarifying Typebook, Profiles, and Ecosystem
Bridge boundaries.

**Required revision:** semantic shape, ABI mechanics, operational support, and
adapter implementation must remain distinct. Every projection records loss and
negative tests.

**Result:** Typebook owns meaning, Profiles own support claims, and Ecosystem
Bridge owns versioned adapters without collapsing their contracts.

### AI Assurance Skeptic

**Disposition:** Accept with authority and provenance controls.

**Required revision:** AI may propose across programs but may not silently
create contract truth, narrow scope, approve action, erase failures, or infer
success. Model, instruction, evidence, proposal, approval, result, rejection,
and rollback remain recorded.

**Result:** Blueprint, Query Forest, Conformance, and Ferris authority
boundaries retain these requirements.

### Ecosystem Strategist

**Disposition:** Accept as a defensible portfolio rather than seven products.

**Required revision:** current upstream tools and maintainers retain their
homes; Typebook remains independent; Profiles avoid distribution and
certification; Ecosystem Bridge prefers contributions over replacements.

**Result:** Incorporated in each program boundary and non-goal.

### Rust Maintainer

**Disposition:** Accept if users experience one coherent tool.

**Required revision:** maintainers must not learn seven CLIs or internal graph
vocabulary to diagnose ordinary work. Explanations lead with Cargo,
repository, test, contract, native, and platform terms and link to evidence.

**Result:** Ferris remains the single public build-system command; other
programs expose records through Ferris views or independent standards APIs.

### Native Platform Adopter

**Disposition:** Accept for specification planning.

**Required revision:** Profiles and Ecosystem Bridge must expose supported and
unsupported platforms, ABIs, native tools, deployment assumptions, compliance,
training, support, recovery, rollback, audit, and removal.

**Result:** Profiles and Conformance own renewable operational proof; concrete
support commitments remain a Proposed-status blocker.

### Scope Keeper

**Disposition:** Accept after making the one-product rule normative.

**Required revision:** the seven programs must not become seven brands,
repositories, mandatory services, or implementation projects. Consumer
workflows remain in application definitions and adapters.

**Result:** The plan explicitly rejects seven public products and permits
further splits only when evidence establishes independent lifecycle.

### Validation Checker

**Disposition:** Accept the closure model; withhold Proposed status.

**Required revision:** every PERF, ECOS, and BLUE question must map to a
program, every program to specifications, every edge to a contract, and every
claim to conformance.

**Result:** EXP-01 covers all 53 completed questions. Exact fixtures, commands,
schemas, outputs, platforms, and thresholds remain open.

## Required revisions completed

The reviewed architecture now includes:

- a one-product, seven-program rule;
- explicit mission, ownership, inputs, outputs, specifications, and non-goals
  for every program;
- typed cross-program contracts;
- a complete research closure matrix;
- continuous conformance;
- owner-local execution and upstream routing;
- role review at each specification stage; and
- visible implementation and Proposed-status blockers.

## Remaining gates

Before Proposed status:

1. freeze the first Typebook contract, profiles, applications, repositories,
   and revisions;
2. define exact adapter versions and owner support;
3. complete CONTRACT-001 through FERRIS-001 as applicable;
4. freeze commands, schemas, exit codes, expected outputs, and thresholds;
5. execute positive, negative, failure, unsupported, stale, version-skew,
   cross-platform, rollback, and removal fixtures; and
6. repeat the nine-role review over measured evidence.

## Decision

Adopt the seven-program architecture as the Draft organizational spine for
remaining Ferris specification work.

Do not authorize implementation.

