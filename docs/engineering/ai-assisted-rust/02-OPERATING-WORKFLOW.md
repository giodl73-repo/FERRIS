# Operating Workflow

> Status: Guidance only.
> Implementation authority: None. This guide does not authorize product code,
> execution, mutation, approval, or changes to Ferris canonical records.

## Workflow objective

Use AI to reduce investigation and drafting effort while keeping source
ownership, deterministic scope, command execution, evidence interpretation,
and approval with their accountable owners. The output is an ordinary,
reviewable Rust patch and evidence set, not an autonomous correctness claim.

## Step 1: Frame the request

Write a bounded change brief:

- requested behavior and named consumer;
- source repository and baseline revision;
- files or owner-native scopes initially in view;
- explicit non-goals;
- applicable risk band from [the overview](00-OVERVIEW.md#review-risk-bands);
- prohibited actions, especially mutation, network, publication, secrets, and
  deployment;
- required repository gates; and
- stop, rollback, and cleanup conditions.

If the task cannot be bounded or ownership is unclear, use AI only for
investigation and questions. Do not proceed to a patch by guessing.

## Step 2: Establish the owner baseline

Before generation, capture or verify:

- Git status, revision, and existing local changes;
- selected Cargo manifest and lock state;
- toolchain and relevant components;
- host and target;
- features, profile, and environment needed by the repository;
- documented format, lint, test, build, and policy commands;
- current behavior or failing reproducer; and
- native tools, build scripts, procedural macros, generated artifacts, and
  dependency changes likely to participate.

Prefer existing repository commands and stable Cargo surfaces. Do not ask the
model to recreate resolution or infer all build units from source layout. The
authority boundary is in
[PRODUCT-001](../../specs/FERRIS_PRODUCT_CONTRACT.md).

## Step 3: Classify data before model exposure

Treat source, issue text, logs, compiler diagnostics, dependency metadata, and
test data as potentially private and as untrusted instruction-bearing input.

- Remove credentials, tokens, private keys, connection strings, personal data,
  private paths, and unrestricted proprietary context.
- Share the minimum context needed for the bounded request.
- Keep redaction visible and record how it limits interpretation.
- Do not paste raw secrets into a prompt for convenience.
- Do not allow source comments, generated files, issue text, or diagnostics to
  override the change brief or authorize commands.

Fail closed when safe handling cannot be established. See
[TRUST-001](../../specs/FERRIS_TRUST_CONTRACT.md) and
[GOVERNANCE-001](../../specs/FERRIS_ENTERPRISE_GOVERNANCE_CONTRACT.md).

## Step 4: Request a proposal, not a verdict

Ask for:

- assumptions and unknowns;
- a small proposed diff;
- expected owner scopes affected;
- tests that demonstrate intended behavior and rejected behavior;
- dependency, unsafe, FFI, macro, build-script, generated-code, and platform
  implications;
- commands to run, without claiming they have run;
- possible failure modes; and
- rollback.

Reject a proposal that relies on uncited invented APIs, hidden files, broad
rewrites, unexplained dependency churn, disabled checks, or statements such as
"the compiler proves this is correct."

## Step 5: Review the patch before execution

Inspect the diff as maintainer-owned code:

- Is the change minimal and understandable?
- Are public API, error, panic, cancellation, concurrency, and resource
  semantics intentional?
- Did Cargo.toml, Cargo.lock, configuration, build.rs, proc-macro code,
  generated source, or native bindings change?
- Was an `unsafe` block added, removed, or made dependent on a new invariant?
- Are tests behavior-oriented rather than snapshots of implementation trivia?
- Can ordinary tooling still be used?

If the patch crosses a higher-risk boundary, stop the ordinary loop and apply
the specialist controls in
[Failure modes and controls](04-FAILURE-MODES-AND-CONTROLS.md).

## Step 6: Run a staged compiler feedback loop

Use the repository's own commands. A typical progression is:

1. formatter or generated-source consistency check;
2. the smallest Cargo check that exercises the changed owner scope;
3. focused behavioral and negative tests;
4. relevant lints and compile-fail checks;
5. reverse-dependency or affected owner closure;
6. broader repository gates; and
7. full-reference validation when required by risk, policy, uncertainty, or
   the selection policy.

At each failure:

- retain the exact command, exit status, diagnostics, toolchain, and scope;
- classify the failure as source, configuration, dependency, environment,
  unsupported, unavailable, stale, permission, resource, or unknown where the
  evidence permits;
- revise only after understanding the owner-reported failure;
- keep prior failed attempts visible in the review evidence; and
- stop after the declared bound rather than looping until output looks green.

Compiler feedback can improve a patch, but repeated compile-fix cycles can also
overfit diagnostics, remove intended behavior, or weaken tests. The
[AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md)
requires behavior evidence and visible failures.

## Step 7: Validate behavior and rejection

For each material behavior, include:

- a positive case showing the requested outcome;
- a negative or rejection case;
- error propagation and cleanup where applicable;
- boundary values and malformed input;
- concurrency, cancellation, timeout, or partial failure where relevant;
- platform and feature conditions that alter behavior; and
- regression coverage for the original defect.

Compilation-only evidence is insufficient. Direct-package tests may also be
insufficient. Preserve validation dimensions and full-reference obligations
under [VALIDATION-001](../../specs/FERRIS_VALIDATION_COVERAGE_CONTRACT.md).

## Step 8: Evaluate performance only when claimed

Do not add benchmarks merely to decorate the patch. When performance is a
goal:

- state the representative user workflow;
- freeze source, commands, features, profile, target, toolchain, environment,
  cache state, and target topology;
- separate cold, warm, incremental, check, build, test, codegen, and link;
- run repetitions and report distribution or variance;
- include correctness and output controls;
- inspect regressions in CPU, memory, I/O, binary size, runtime, and validation
  where applicable; and
- report inconclusive or negative results.

Use the
[Build Latency Measurement Contract](../../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
and [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md)
lens. AI explanation is a hypothesis until measurements support it.

## Step 9: Assemble review evidence

The review package should identify:

- baseline and final source revisions or diff;
- model/tool involvement and the permitted context class;
- human edits made after generation;
- exact commands and results, including failures;
- toolchain, platform, target, features, profile, and relevant environment;
- selected scope, omitted scope, unknowns, and full-reference comparison;
- unsafe, FFI, dependencies, build scripts, proc macros, generated source,
  privacy, and performance effects;
- limitations and unsupported cases; and
- rollback/removal instructions.

This is a human-readable application of
[Evidence and identity](03-EVIDENCE-AND-IDENTITY.md), not a replacement schema
for Query Forest or [FERRIS-001](../../specs/FERRIS_EVIDENCE_PACKET_CONTRACT.md).

## Step 10: Human decision

The reviewer decides to:

- accept;
- request revision;
- require specialist review;
- require wider or full-reference validation;
- defer because evidence is unavailable;
- reject; or
- revert.

Approval must bind to the reviewed patch and evidence. A material change after
approval requires renewed review. Model confidence, passing selected tests, or
a signed evidence artifact cannot grant approval.

## Step 11: Preserve rollback

Before merge or rollout:

- verify the patch can be reverted cleanly;
- identify data, generated files, lock state, configuration, and external
  effects that need restoration;
- preserve the last known owner-native commands;
- avoid making the AI tool required for build or recovery; and
- define the signal that triggers rollback.

For broader adoption, use
[Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md).

