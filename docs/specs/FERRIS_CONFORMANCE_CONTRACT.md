# CONFORMANCE-001: Ferris Conformance Contract

Status: Draft framework after nine-role review
Implementation authority: None
Depends on: All preceding Ferris specifications

## Purpose

This specification defines the executable proof required before any
implementation may claim conformance with Ferris product, planning, command,
evidence, action, or removal contracts.

## Conformance suites

### C-PRODUCT: identity and namespace

- public output identifies the product as Ferris;
- Blueprint appears only as the internal model or plan;
- `ferris` and `cargo ferris` invoke the same semantic engine;
- package names do not alter product or evidence identity; and
- no output implies official Rust Project or Foundation status.

### C-ENTRY: entrypoint equivalence

For identical explicit inputs, both entrypoints MUST match on:

- semantic command ID;
- normalized configuration;
- selected scope;
- Blueprint Plan identity;
- diagnostics and unknowns;
- output schema;
- exit class; and
- yielded evidence.

Negative fixtures MUST prove that adapter defaults do not silently cross
workspace, repository, or application boundaries.

### C-SCOPE: affected selection and widening

Fixtures MUST include:

- source-only local changes;
- public API and generic changes;
- macro and build-script changes;
- feature, profile, target, and toolchain changes;
- test-only and contract changes;
- native, ABI, linker, packaging, and deployment changes;
- unknown and stale mappings; and
- multiple independent workspaces.

Every selected-only result MUST be compared with a full-reference run.
Unknown mappings MUST widen to the specified boundary.

### C-PLAN: planning and explanation

Fixtures MUST verify:

- owner-specific closures remain distinct;
- each Cargo activity has its own invocation plan;
- mandatory validation cannot disappear;
- resource limits trigger a recorded replan or defer result;
- observation barriers alter downstream work correctly;
- selected, omitted, reused, rebuilt, waiting, failed, and unknown states are
  explained from evidence; and
- a Blueprint Plan cannot execute without an approved Action Plan.

### C-FAIL: failure and unsupported behavior

Positive, negative, failure, unsupported, stale, corrupt, revoked,
version-skew, and uneconomic cases are mandatory for every adapter.

No case may:

- silently return success;
- suppress mandatory validation;
- accept incompatible machine output;
- reuse evidence outside its identity or scope;
- treat a ref or label as compatibility proof; or
- continue mutation after approval, isolation, or rollback requirements fail.

### C-REMOVE: adoption, rollback, and removal

Fixtures MUST prove:

- ordinary Cargo commands work before, during, and after Ferris adoption;
- Ferris metadata can be ignored without changing source correctness;
- failed or partial setup has a documented recovery path;
- repository integration can be removed;
- no shared writable target directory is required; and
- removal does not require recreating hidden Ferris-owned build truth.

### C-PLATFORM: operational portability

The held-out proof MUST run on Windows and at least one Unix platform.
Differences in paths, processes, filesystems, linkers, native discovery, and
resource accounting MUST be normalized or explicitly explained.

Unsupported platforms MUST produce an unsupported result rather than a
degraded success.

### C-AI: model-assisted decisions

Fixtures MUST record:

- model and instruction identity;
- proposed scope or action;
- deterministic policy or human approval;
- evidence used;
- full-reference comparison;
- rejection and fallback; and
- result and rollback.

AI-only semantic similarity MUST NOT authorize work-reducing scope.

Security, performance, behavioral-correctness, safety, and soundness claims
MUST each cite dedicated evidence. A build or selected test pass is not a
substitute.

### C-SAFETY: Rust guarantees and unsafe boundaries

Fixtures MUST verify:

- safe interfaces do not conceal unreviewed invariants;
- ownership, lifetime, aliasing, concurrency, and panic assumptions are
  recorded where applicable;
- toolchain identity is attached to compiler-derived evidence;
- compiler acceptance is not presented as behavioral proof; and
- an adapter requiring `unsafe` or compiler-private integration is rejected
  unless its separately approved contract is present.

### C-INTEROP: native and language boundaries

Fixtures MUST include positive, negative, and failure cases for applicable:

- ABI and calling convention;
- ownership and lifetime transfer;
- allocation and deallocation;
- panic, exception, and unwind behavior;
- threading and synchronization;
- layout and generated bindings;
- native discovery, linking, loading, and runtime use; and
- incremental migration and removal.

Boundary failures MUST identify the owner, stage, violated contract, and
evidence needed for recovery.

### C-PERF: representative build-system value

Measurements MUST distinguish:

- cold check, build, test, and link;
- warm no-change work;
- incremental local implementation changes;
- public API, generic, macro, build-script, feature, profile, target, and
  contract changes; and
- selected-only versus full-reference execution.

Every result MUST record hardware, operating system, filesystem, toolchain,
commands, cache state, target-directory topology, repetitions, variance,
failures, and limitations. Convenient microbenchmarks MUST NOT stand in for
the held-out developer workflow.

### C-OPS: adoption and operational trust

Fixtures and records MUST cover:

- supported and unsupported tools, platforms, ABIs, and deployment models;
- installation and upgrade;
- training and support ownership;
- compliance, privacy, retention, and audit effects;
- actionable failure diagnosis;
- partial-install and interrupted-action recovery;
- rollback and removal; and
- operational and maintenance cost.

## Held-out workflow

The initial acceptance workflow is:

> Given a proposed change across several existing Cargo workspaces, determine
> what must be checked and tested, explain why, execute only after approval,
> and compare the result with the full reference.

The proof freezes:

- three public repositories;
- multiple Cargo workspaces;
- Windows and Unix environments;
- representative local, API, generic, macro, build-script, contract, native,
  and configuration edits;
- raw Cargo and repository-script baselines;
- correctness, unknown, failure, and unsupported controls;
- resource and investigation-time measures; and
- adoption, rollback, removal, privacy, and retention procedures.

Exact repositories, revisions, commands, fixtures, output schemas, and
thresholds remain blockers for Proposed status.

## Claim rule

An implementation MUST name:

- the specification versions it implements;
- the conformance suites and fixtures passed;
- optional capabilities not implemented;
- unsupported tools, versions, and platforms;
- known failures and limitations; and
- the date and environment of the result.

Passing one suite MUST NOT imply complete Ferris conformance.

## Acceptance criteria

CONFORMANCE-001 may advance to Proposed only when:

1. fixture repositories and revisions are frozen;
2. commands and expected outputs are executable and reviewable;
3. selected-only and full-reference comparisons are defined;
4. positive, negative, failure, unsupported, and version-skew cases exist for
   every adapter;
5. Windows and Unix execution is specified;
6. adoption, rollback, and removal are exercised;
7. measurable pass, fail, and stop thresholds are fixed; and
8. all nine roles record a disposition.
