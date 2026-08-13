# Validation Roadmap

> Status: Guidance only.
> Implementation authority: None. This guide does not authorize product code,
> execution, mutation, approval, or changes to Ferris canonical records.

## Roadmap purpose

This roadmap turns AI-assisted Rust claims into staged, falsifiable evidence.
It does not define a new Ferris record schema. Canonical validation,
conformance, identity, evidence, and Query Forest authority remain in the
[specification registry](../../specs/README.md).

## Claim-to-evidence rule

| Claim | Required evidence | Evidence that is insufficient alone |
|---|---|---|
| "The patch compiles" | Exact Cargo/rustc activity, toolchain, target, features, profile, source, command, and result | Model explanation or syntax appearance |
| "The behavior is correct for X" | Named behavior, positive and negative tests, relevant boundary cases, owner review, and limitations | Compilation or one happy-path test |
| "The change is safe" | Defined safety claim, invariant/boundary evidence, qualified review, dedicated tools/tests, and platform scope | Safe-Rust syntax, compiler acceptance, or absence of a crash |
| "The FFI is compatible" | ABI and semantic contract, generated-binding provenance, link/load/runtime and negative version tests | Matching type names or successful linking |
| "The dependency is acceptable" | Consumer requirements, exact identity and closure, policy/assurance evidence, alternatives, owner decision, renewal, and rollback | Search rank, downloads, one scan, or passing build |
| "The selected tests are sufficient" | Deterministic scope, mandatory gates, full-reference comparison, held-out false-omission controls, and approval | Direct-package tests or AI confidence |
| "The patch is faster" | Representative baseline/candidate measurements, repetitions, variance, components, correctness controls, and limitations | One wall-time sample |
| "The patch is approved" | Exact reviewed patch/plan, authorized human or policy principal, scope, commands, limits, expiry, and revocation state | Authentication, signature, CI green, or model confidence |

## Phase 0: Freeze the validation contract

Before evaluating AI assistance:

- name consumers and representative workflows;
- define source fixture classes and privacy tiers;
- define positive, negative, failure, unsupported, stale, and unknown cases;
- freeze baseline commands and environment identity;
- define risk-band-specific mandatory gates;
- define selected-versus-full comparison;
- define false-omission, regression, privacy, cost, and stop thresholds;
- define held-out evidence separation; and
- define rollback and removal exercises.

Do not collect favorable examples first and design thresholds afterward. Apply
[FP-12](../../governance/ENGINEERING_PRINCIPLES.md) and the
[Build Latency Measurement Contract](../../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md).

## Phase 1: Synthetic controls

Use small fixtures where the expected result is known:

- ordinary safe-Rust behavior edit;
- borrow/type error repaired without behavior loss;
- public API change with downstream compilation;
- generated source drift;
- proc-macro compile-pass and compile-fail change;
- build-script declared and hidden input change;
- dependency feature or lock change;
- unsafe invariant violation or rejected operation;
- FFI layout/version/ownership mismatch;
- flaky, unavailable, unsupported, and timed-out validation;
- prompt-injection text attempting scope expansion or secret access; and
- performance case with known noise or shifted work.

Synthetic controls verify that the process can detect known conditions. They
do not establish real-world value.

## Phase 2: Representative repository workflows

Select public or authorized private fixtures by build and maintenance shape,
not popularity:

- small ordinary crate;
- multi-package workspace;
- deep or broad dependency graph;
- proc-macro-heavy repository;
- build-script/native dependency repository;
- public API or generic fan-out;
- cross-platform or cross-target repository;
- FFI or mixed-language boundary; and
- dependency-heavy application with material link/runtime validation.

Freeze revision, manifest, lock hash, features, profiles, targets, toolchain,
commands, environment, data policy, and licensing. Private results must follow
the disclosure limits in the
[measurement contract](../../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md).

## Phase 3: Behavioral and negative validation

For every generated patch class, measure:

- requested behavior achieved;
- malformed, boundary, and adversarial input;
- error and diagnostic quality;
- panic, cancellation, timeout, retry, and cleanup where applicable;
- concurrency and ordering where applicable;
- feature, profile, target, and platform variants;
- generated-code consistency;
- compile-fail or rejection behavior;
- rollback behavior; and
- maintainer ability to identify owner, impact, safe next action, prohibited
  shortcut, and needed evidence.

The
[Validation Checker](../../../.roles/editorial/validation-checker.md) requires
commands, representative fixtures, environment assumptions, negative cases,
and accurate status.

## Phase 4: Scope-selection validation

Evaluate direct-only, owner reverse-closure, conservative, and full-reference
plans over:

- local implementation changes;
- public API and generic changes;
- features, profiles, targets, and toolchain changes;
- tests and documentation;
- generated inputs;
- proc macros and build scripts;
- dependencies and native/ABI inputs;
- runtime data and environment;
- policy and repository gates; and
- unknown and stale mappings.

Every narrower result is compared with full reference. Track:

- work selected and omitted;
- mandatory gates retained;
- false omissions;
- over-selection;
- abstention/blocking;
- widening reason;
- capability preserved, unverified, reduced, or lost;
- cost; and
- evidence expiry.

Disable or widen a policy when its fixed threshold is crossed. A later
full-reference pass does not retroactively make a missed selected run
sufficient. See
[VALIDATION-001](../../specs/FERRIS_VALIDATION_COVERAGE_CONTRACT.md).

## Phase 5: Performance validation

Measure both benefit and overhead:

- maintainer investigation and review time;
- model/tool latency and cost;
- compiler and validation latency;
- CPU, memory, I/O, storage, network, and retained evidence;
- cold, warm, incremental, check, build, test, codegen, and link workflows;
- target-directory and cache topology;
- patch size and revision count;
- defect, false-omission, revert, and incident rates; and
- adoption, support, and removal effort.

Report distributions and variance. Attribute causes before recommending
workspace, dependency, compiler, cache, linker, or source changes. A faster
workflow that misses a material failure does not pass.

## Phase 6: Privacy and adversarial validation

Exercise:

- credentials and reusable secrets in source or logs;
- private repository names, paths, dependencies, and timing data;
- personal or regulated test data;
- tenant and audience separation;
- redaction that changes interpretation;
- deletion and retention requests;
- injected instructions in source, issues, diagnostics, generated files,
  dependencies, and web content;
- requests for network, publication, mutation, or broader source access; and
- unsupported model or connector versions.

Expected behavior is minimum disclosure, visible omission, bounded output, no
secret retention, no authority granted to embedded instructions, and fail
closed when safe handling is unavailable. Use
[TRUST-001](../../specs/FERRIS_TRUST_CONTRACT.md) and
[GOVERNANCE-001](../../specs/FERRIS_ENTERPRISE_GOVERNANCE_CONTRACT.md).

## Phase 7: Held-out conformance

Separate development, calibration, and held-out fixtures. Freeze evidence
cutoffs before prediction or generation. Include:

- Windows and Unix where supported;
- local, API, generic, proc-macro, build-script, dependency, native, FFI,
  configuration, and policy changes;
- positive, negative, failure, unsupported, and version-skew cases;
- selected-only and full-reference runs;
- privacy, retention, rollback, and removal; and
- ordinary Cargo operation after disabling assistance.

Report known failures and limitations. Passing one suite must not imply
complete conformance. The controlling requirements are in
[CONFORMANCE-001](../../specs/FERRIS_CONFORMANCE_CONTRACT.md).

## Phase 8: Adoption and removal proof

For each maturity promotion, exercise:

- clean installation or enablement;
- partial or interrupted setup;
- support and diagnostic workflow;
- capability-specific disablement;
- patch revert;
- scope-narrowing disablement and full-reference restoration;
- credential, connector, hook, cache, configuration, and generated-artifact
  cleanup;
- evidence export and policy-driven deletion;
- ordinary Cargo/editor/CI verification; and
- reinstall boundary.

Use
[Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md).

## Promotion gates

### M0 to M1

- Bounded tasks and data policy are defined.
- Model assertions are distinguishable from evidence.
- Prompt-injection controls hold.
- Disabling the assistant has no repository effect.

### M1 to M2

- Provenance is reproducible.
- Behavioral and negative tests cover material patch classes.
- R2/R3 escalation is reliable.
- Selected and full-reference commands are known.
- Revert and removal drills pass.

### M2 to M3

- Held-out false-omission and failure thresholds pass.
- Representative performance benefit exceeds measured overhead.
- Privacy, cross-platform, unsupported, and version-skew controls pass.
- Support ownership, renewal, rollback, and removal are sustainable.
- Applicable nine-role dispositions are recorded.

## Stop and regression criteria

Stop promotion or regress maturity when:

- a material failure is omitted by narrowing;
- generated work introduces an unreviewed R3 boundary;
- tests are weakened to obtain a pass;
- a secret or private input crosses policy;
- prompt injection changes authority or scope;
- performance claims are not reproducible;
- ordinary Cargo or editor workflows become dependent on the assistant;
- rollback or removal is partial or unknown; or
- evidence is presented as proving owner truth, correctness, safety,
  performance, or approval.

The valid outcome may be to improve the fixture, contribute evidence upstream,
return to a wider owner-native workflow, or defer the capability.

