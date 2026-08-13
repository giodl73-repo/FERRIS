# Failure Modes and Controls

Status: Guidance
Implementation authority: None

## Purpose

Conformance evidence is credible only when non-success behavior is designed,
bounded, observable, and scoreable. This guide defines required failure
families and controls for future reference companions.

## Result vocabulary

Use distinct results:

- `success`;
- `expected-rejection`;
- `unsupported`;
- `unavailable`;
- `permission-denied`;
- `malformed`;
- `conflicting`;
- `stale`;
- `expired`;
- `revoked`;
- `failed`;
- `partial`;
- `cancelled`;
- `timed-out`;
- `resource-limited`;
- `abstained`;
- `blocked`;
- `invalid-fixture`;
- `invalid-harness`;
- `invalid-collection`;
- `invalid-scorer`;
- `not-observed`; and
- `unknown`.

Exact machine vocabulary belongs to a later approved schema. The critical rule
is that these meanings remain separate.

## Case matrix

| Class | Example | Required control |
|---|---|---|
| Positive | Exact supported profile executes its representative operation | Compare with owner-native full reference |
| Negative | Host build of a WASM-only component | Expected rejection with owner and recovery evidence |
| Failure | Owner command exits nonzero | Preserve streams, exit, stage, bounds, and cleanup |
| Unsupported | ABI or target is outside declared support | Return unsupported, never degraded success |
| Unavailable | Required SDK, runner, or evidence service is absent | Identify missing owner prerequisite |
| Stale | Profile or evidence passed expiry | Prevent reuse and require renewal |
| Version skew | Schema, Cargo, compiler, SDK, connector, or protocol mismatch | Reject or select a declared adapter |
| Cross-platform | Path, shell, linker, filesystem, process, or locale differs | Normalize only declared non-semantic differences |
| Rollback | Update passes setup but fails a mandatory gate | Restore exact prior identities and revalidate |
| Removal | Residual hook or metadata remains | Fail removal until ordinary owner workflow is clean |

## Source and dependency failures

Test:

- missing or moved repository;
- mismatched commit or archive digest;
- submodule drift;
- lockfile mismatch;
- offline dependency absence;
- registry or source custody change;
- generated source not reproduced;
- unsupported source type; and
- licensing or redistribution restriction.

Controls:

- stop before execution on identity mismatch;
- retain the observed mismatch;
- do not fetch or rewrite automatically;
- identify the source owner and allowed recovery; and
- require a new binding revision after intentional renewal.

## Toolchain and environment failures

Test:

- absent compiler, target, SDK, linker, runner, or native compiler;
- wrong compiler or Cargo release;
- auto-install attempt;
- environment-variable conflict;
- path encoding, case, separator, or length differences;
- read-only or full filesystem;
- clock skew;
- locale-dependent diagnostics;
- network denial;
- timeout, process-tree cleanup, and output overflow; and
- resource exhaustion.

Controls:

- record exact owner command and working directory;
- disable hidden auto-install and update;
- apply explicit time, output, process, memory, and storage bounds;
- preserve bounded evidence and whether unobserved bytes remain unknown;
- record termination and cleanup completion; and
- never infer platform failure from a missing cross tool.

## Schema, identity, and evidence failures

Test:

- malformed, ambiguous, oversized, secret-bearing, or unsupported records;
- missing required identity;
- source, invocation, or result identity collision;
- replayed, corrupt, expired, revoked, or collected evidence;
- projection inconsistency;
- incompatible viewer or scorer;
- incomplete packet;
- retention and deletion failure; and
- ref or label presented as proof.

Controls:

- fail closed on semantic ambiguity;
- preserve immutable source records;
- distinguish equality, compatibility, trust, validation, and correctness;
- bind projections to snapshots and schema versions;
- qualify two independent viewers where required; and
- prevent labels and refs from conferring authority.

## Planning and validation failures

Test:

- false omission from selected validation;
- unknown mapping;
- mandatory gate suppression;
- stale prediction;
- out-of-population AI proposal;
- selected-only success with full-reference failure;
- resource budget overrun;
- observation barrier changing downstream work; and
- capability loss hidden by a passing build.

Controls:

- widen unknown mappings to the smallest safe owner boundary;
- compare every selected result with the full reference;
- keep original predictions immutable;
- report false omissions, over-selection, abstention, and fallback;
- require deterministic policy or human approval for work reduction; and
- stop on missing mandatory gates.

These controls align with C-SCOPE, C-PREDICT, and C-VALIDATE in
[CONFORMANCE-001](../../specs/FERRIS_CONFORMANCE_CONTRACT.md).

## AI failures

Test:

- prompt injection through source, diagnostics, connector output, or packet;
- hidden oracle exposure;
- model or instruction drift;
- unsupported tool use;
- fabricated owner evidence;
- patch larger than the approved scope;
- unsafe, FFI, dependency, build-script, or macro changes not escalated;
- confidence presented as correctness;
- rejected proposal reintroduced; and
- failed patch rollback.

Controls:

- bind model, instructions, context cutoff, tools, and proposed patch;
- isolate untrusted content from instructions;
- use deterministic scope and policy checks;
- require behavioral, negative, safety, security, and performance evidence as
  applicable;
- preserve rejection and fallback;
- prohibit model authority over approval or execution; and
- quarantine leaked held-out cases.

## Native and interop failures

Test applicable:

- ABI and calling-convention mismatch;
- ownership or lifetime transfer error;
- allocator mismatch;
- panic, exception, or unwind crossing;
- thread-affinity or synchronization violation;
- layout or generated-binding skew;
- missing native library;
- link or load failure;
- runtime capability mismatch;
- cleanup and uninstall failure; and
- partial migration rollback.

Controls:

- name boundary owner and exact contract;
- test positive and negative calls;
- run native link, load, and execution stages;
- reject unreviewed `unsafe` or compiler-private integration;
- preserve native diagnostics;
- define incremental migration and removal; and
- do not claim behavior from compilation.

## Platform and profile failures

The profile research records useful controls:

- hosted networking failed for browser WASM rather than becoming a browser
  profile;
- CLI and data stacks that checked for WASM did not claim browser usability;
- embedded builds did not claim hardware execution;
- a WASM build did not claim JavaScript execution; and
- a Linux cross-build missing `x86_64-linux-gnu-gcc` did not prove the crate
  failed on Linux.

See
[Rust compatibility-tested stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md).
Future companions should preserve these distinctions.

## Harness and scorer failures

Test:

- fewer processes than expected;
- missing durable records;
- stream truncation or split records;
- parser rejection of contract-equivalent layouts;
- oracle branch not prequalified;
- nondeterministic scorer;
- scorer reading prohibited material; and
- collection after cleanup deletes evidence.

Controls:

- freeze expected cardinality;
- preflight mixed result classes;
- seal raw records before scoring;
- qualify semantic layouts and every oracle branch;
- bind collector, parser, viewer, and scorer versions;
- classify infrastructure defects separately; and
- never repair and rescore the same held-out fixture.

The held-out fixture history in
[MANIFEST.md](../../simulations/held-out/MANIFEST.md) is evidence that
collection and scorer validity are part of the proof.

## Diagnostic quality

Every actionable failure should identify:

- result class;
- owner;
- failing stage;
- impact;
- violated contract or missing prerequisite;
- safe next action;
- prohibited shortcut;
- evidence needed to recover;
- retry, rollback, cleanup, and removal state; and
- whether detail was truncated or redacted.

Meaning must survive plain text, localization, accessibility use, pagination,
and output bounds.

## Stop and disable rules

Disable promotion or current use when:

- false omissions exceed the fixed threshold;
- a mandatory gate is missing;
- evidence or identity cannot be trusted;
- a secret is captured;
- rollback or cleanup fails;
- ordinary Cargo operation is broken;
- a platform claim exceeds observed stages;
- a fixture or oracle leaks;
- scorer validity is uncertain; or
- maintenance or privacy cost exceeds the approved envelope.

The disabled state remains visible until a new reviewed revision supplies
replacement evidence.
