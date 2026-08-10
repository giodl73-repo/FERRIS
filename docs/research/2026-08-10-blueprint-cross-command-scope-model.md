# Blueprint Cross-Command Scope Model

Date: 2026-08-10
Status: Complete
Decision: define SCOPE-001 separately from IDENTITY-001. Scope is a
multi-dimensional, bounded domain over which ownership, selection,
compilation, execution, validation, capability, lifecycle, or evidence claims
apply. Blueprint must map scopes through typed, directional,
cardinality-aware, conditional edges rather than force them into one tree.

## Decision supported

This research closes
[BLUE-Q03](questions/blueprint/BLUE-Q03-cross-command-scope-mapping.md) and
defines the scope model required by APPLICATION-001, SCOPE-001, FOREST-002,
IDENTITY-001, EVIDENCE-001, PLANNING-001, VALIDATION-001, VIEW-001, and
CONFORMANCE-001.

It answers:

- whether source files, Rust modules, Cargo packages, compiler units, tests,
  contracts, services, artifacts, and deployment targets share one scope;
- how `check`, `build`, `test`, `clippy`, `doc`, doctest, package, run, and
  non-Cargo activities map to work and validation;
- whether package selection and test selection mean the same thing;
- how generated, native, platform-conditional, and runtime-only effects map;
  and
- how unknown mappings affect incremental planning.

Implementation authority remains closed.

## Why the current hierarchy is insufficient

A useful containment path exists:

```text
repository -> workspace -> package -> target -> Cargo unit -> rustc invocation
```

But it is only one projection. Real mappings include:

- one source file containing many items and contributing to multiple targets;
- one package producing many units across commands, features, profiles, and
  host/target contexts;
- one crate containing many compiler owners, queries, mono items, and CGUs;
- many objects and native libraries producing one final link output;
- one compiled test binary containing many runtime test cases;
- one contract operation implemented by multiple packages and validated by
  multiple activities;
- one service assembled from multiple binaries, configuration, contracts,
  native providers, and deployment assets; and
- one Query Forest root spanning many owner scopes without containing them as
  one uniform tree.

Containment, ownership, selection, compilation, generation, execution,
validation, coverage, deployment, and evidence are different relationships.

## Canonical scope definition

A **scope** is:

> The bounded domain over which one ownership, selection, compilation,
> execution, validation, capability, lifecycle, or evidence statement is
> intended to hold.

One scope coordinate records:

```text
owner
+ subject
+ activity
+ configuration
+ platform
+ lifecycle phase
+ evidence time/root
```

Examples:

- package `payments-core`, library target, `check`, features `default`,
  Windows host and target, current source root;
- integration-test target `payments_api`, test compilation, Linux target,
  release candidate root;
- test case `refund_retries`, runtime execution, filtered invocation, staging
  service environment;
- Typebook operation `payments.refund.v2`, compatibility validation, provider
  profile `service.v1`;
- native library `openssl`, discovered version, target ABI, linked artifact,
  deployment image.

A missing coordinate is not automatically global. It is absent, unsupported,
not observed, or unknown and must be handled explicitly.

## Scope dimensions

| Dimension | Representative scopes |
|---|---|
| organization | portfolio, repository, workspace, package |
| application | application, component, service, contract, operation, provider |
| source | revision, change set, file, generated file, module, item, body, input |
| activity | resolve, check, build, clippy, test compilation, test execution, doc, doctest, example, bench, run, package, deploy |
| Cargo | package selection, target, feature set, profile, platform, unit |
| compiler | crate invocation, HIR owner, body, query, mono item, CGU, backend work |
| artifact | metadata, generated output, object, archive, library, link plan, executable, debug package, distribution |
| validation | suite, test binary, test case, lint, doctest case, policy gate, capability coverage |
| native | source mode, build tool, generator, discovered component, ABI, linked component, runtime-loaded component |
| runtime | process, endpoint, service instance, runtime data, environment, deployment target |
| lifecycle | development, candidate, release, support, renewal, deprecation, removal, rollback |
| evidence | observation, prediction, plan, action, outcome, root, ref, expiry |

These dimensions may be serialized together, but none substitutes for another.

## Typed scope mappings

Every mapping records:

- source and target scope;
- mapping type and owner;
- cardinality: one-to-one, one-to-many, many-to-one, or many-to-many;
- condition: command, feature, profile, platform, cfg, policy, runtime, or
  lifecycle state;
- declared, resolved, observed, inferred, unsupported, or unknown status;
- evidence source, confidence, time, and limitations; and
- invalidation and fallback behavior.

Required mapping types include:

| Mapping | Meaning |
|---|---|
| `OWNS_SCOPE` | owner controls the bounded scope |
| `CONTAINS` | structural containment only |
| `MAPS_TO` | generic explicit mapping where no narrower verb applies |
| `COMPILES_AS` | source, target, or generated input realizes as a Cargo/compiler scope |
| `EXPANDS_TO` | macro, generator, feature, or declaration expands into scopes |
| `GENERATES` | execution produces source, metadata, native, or artifact scope |
| `SELECTED_FOR_ACTIVITY` | command selection includes a scope for one activity |
| `EXECUTES_AS` | compiled artifact or test binary becomes runtime execution |
| `VALIDATED_BY` | scope is directly validated by an activity or gate |
| `COVERS_RUNTIME_SCOPE` | validation evidence claims runtime coverage |
| `CONDITIONED_ON` | mapping exists only under named configuration or policy |
| `PART_OF_PROFILE` | scope participates in a compatibility profile |
| `DEPLOYS_AS` | application or artifact scope realizes as a deployment scope |
| `UNKNOWN_EFFECT_ON` | input may affect a scope but the mapping is unresolved |

Generic dependency edges remain useful, but `CONTAINS` and `DEPENDS_ON` alone
cannot express cross-command scope.

## Cross-command mappings

### `cargo check`

- selects packages and targets;
- creates analysis-oriented Cargo units and rustc invocations;
- may cover compiler diagnostics and selected lints;
- does not produce build, link, runtime, test-execution, documentation, or
  deployment equivalence.

### `cargo build`

- selects packages and buildable targets;
- creates code-generation and often link scopes;
- produces final build artifacts;
- does not execute tests or prove runtime, documentation, policy, or
  deployment scope.

### `cargo test`

Separates at least:

1. package and target selection;
2. test compilation activity;
3. compiled test binary or doctest crate;
4. runtime test-case selection and filter;
5. test execution result; and
6. validation coverage claim.

A test-name filter may narrow runtime execution while leaving compilation of
the containing test target unchanged. A passing filtered test claims only the
executed scope.

Unit tests, integration tests, doctests, examples, and benches are distinct
targets or activities with different features, cfgs, dependencies, outputs,
and runtime behavior.

### `cargo clippy`

- creates a separate lint activity over selected targets;
- may share dependency artifacts but retains tool-specific analysis,
  diagnostics, configuration, and failure scope;
- cannot be relabeled as ordinary `check`.

### `cargo doc` and doctest

- rustdoc extraction and rendering are separate activities;
- documentation dependencies and outputs are not ordinary build artifacts;
- doctest generation creates temporary crate and test-execution scopes;
- repeated doctest work may be ephemeral even when package scope is unchanged.

### `cargo run`, package, and deployment

- `run` joins a build scope to a runtime process and runtime-data scope;
- package joins source, license, generated, native, and distribution-artifact
  scopes; and
- deployment joins artifacts, configuration, credentials, environment,
  service, platform, policy, and rollback scopes.

No Cargo compilation result alone proves package or deployment scope.

## Source-to-command example

```text
src/payments/refund.rs
  -> module payments::refund
    -> items RefundService, retry_refund
      -> library target payments_core
        -> check unit
        -> build unit
        -> lib test unit
        -> clippy unit
        -> rustdoc activity
      -> integration target payments_api
      -> doctest crate extracted from public docs
      -> downstream binary link input
      -> Typebook operation payments.refund.v2
      -> service validation and deployment scope
```

Not every edge activates for every edit. A private body edit, public signature
edit, documentation edit, feature change, generated input, or platform change
produces a different closure.

## Change classes and scope fan-out

| Change | Typical initial scope | Potential fan-out |
|---|---|---|
| private body | item/body | owning crate work, codegen, link, selected tests |
| public API signature | item/interface | direct and transitive consumers, contracts, docs, tests |
| inline/generic/const body | item plus exported body | downstream codegen owners and link inputs |
| module or visibility | module/namespace | name resolution, diagnostics, consumers, documentation |
| feature or cfg | configuration | targets, units, dependencies, providers, validation matrix |
| test body | test case/item | containing test binary compilation and selected execution |
| test filter | runtime selection | executed cases only; usually not narrower compilation |
| build-script input | execution cone | generated files, cfg/env, native metadata, consumers |
| proc-macro input | macro invocation/cone | expanded source, items, diagnostics, downstream work |
| native/provider input | native chain | discovery, build, ABI, link, load, package, deployment |
| policy/root configuration | repository/application | mandatory gates or full-reference validation |
| runtime data or deployment config | runtime/deployment | service tests and operations, possibly no Rust compilation |

These are prediction starting points, not universal guarantees.

## Scope closure composition

For one proposed change, Blueprint computes separate closures:

```text
source ownership closure
Cargo package/target/unit closure
compiler semantic and codegen closure
generated and native execution-cone closure
artifact and link closure
contract and compatibility closure
validation activity and runtime-coverage closure
application and deployment closure
```

The Blueprint Plan joins them through typed mappings and conditions:

```text
required plan scope =
  union(activated owner-specific scope closures)
  + mandatory policy scopes
  + explicit finalization scopes
  + conservative widening for unknown mappings
```

Package scope and activity scope are independent dimensions. An empty package
selection does not mean no validation. A selected activity pass does not mean
the full repository or application scope passed.

## Unknown and fallback

Unknown or unmapped changes never produce an empty success-shaped plan.

- unknown repository or root-policy input widens to the full repository
  reference plan;
- unknown generated, macro, build-script, or native input widens to the full
  owning execution cone and its consumers;
- unknown feature, profile, provider, or platform mapping widens to the
  affected configuration matrix;
- unknown runtime or deployment mapping retains the required operational gates;
  and
- unsupported compiler detail falls back to Cargo/package/target granularity.

The plan records why it widened and which evidence would permit future
narrowing.

## AI-efficient scope design

AI works best over scopes that are stable, typed, observable, owner-aligned,
and cheap to recompute. It should not invent free-form semantic regions and
then treat them as correctness boundaries.

### Preferred anchor scopes

Use stable owner-native scopes first:

1. repository and application policy;
2. workspace, package, Cargo target, and command activity;
3. feature, profile, host/target, provider, and platform;
4. contract, operation, component, service, and capability;
5. validation activity, test binary, repository gate, and deployment gate;
6. artifact, link output, package, and deployment target; and
7. environment, execution session, evidence root, and lifecycle state.

Module, item, body, compiler query, mono-item, CGU, and test-case scopes are
valuable refinements only when supported by tool evidence. They must not be
required for the safe baseline.

### Scope slices

AI plans over a **scope slice**, not one label:

```text
subject: package/payments-core
activity: test-execution
configuration: features/default + profile/test
platform: host/windows-x64 + target/windows-x64
runtime: test-binary/payments_core
selection: test/refund_retries
evidence: root/<digest>
```

This representation prevents “run tests for package X” from silently
conflating compilation, filtering, execution, platform, and coverage.

### Coarse-to-fine planning

1. Begin at stable repository, package, target, activity, and policy anchors.
2. Apply deterministic declared and observed mappings.
3. Narrow to module, item, test case, contract operation, or runtime scope only
   when evidence supports the mapping.
4. Ask AI to rank alternatives, explain closures, propose missing mappings, or
   classify ambiguous changes.
5. Require a policy engine or human approval before an AI-proposed mapping can
   remove work or validation.
6. Widen automatically when evidence becomes stale, conflicting, unsupported,
   or unknown.

Narrowing is evidence-gated; widening is safety-preserving.

### Deterministic core, AI-assisted edge

The deterministic scope engine owns:

- schema validation and canonical IDs;
- Cargo metadata and repository-declared mappings;
- set operations, closure traversal, conditions, and cardinality;
- mandatory gates and fallback;
- freshness, expiry, confidence thresholds, and policy;
- plan reproducibility and audit; and
- selected-versus-full coverage comparison.

AI may:

- interpret a semantic change;
- propose a module, capability, contract, service, or test mapping;
- identify likely missing inputs;
- compare candidate closure explanations;
- suggest a narrower held-out experiment; and
- generate the human-readable causal narrative.

AI may not silently create an authoritative mapping, suppress a mandatory
scope, or convert low-confidence inference into observation.

### Scope budgets

Fine-grained scope can cost more than the work it avoids. Every plan therefore
sets budgets for:

- mapping nodes and edges loaded;
- closure traversal time and memory;
- compiler- or analyzer-detail collection;
- AI context and inference cost;
- stale mapping renewal;
- explanation size; and
- expected avoided work.

When detail exceeds its budget, Blueprint summarizes under the nearest stable
anchor scope and records the loss of precision. It does not drop the affected
scope.

### Best-scope test

A useful scope is:

- owned by a named system or repository;
- stable enough to compare across runs;
- observable or explicitly declared;
- specific enough to avoid material work;
- broad enough to compute and explain economically;
- capability-bearing where validation is concerned;
- composable through typed mappings; and
- equipped with a conservative fallback.

Avoid scopes based only on file names, free-form labels, model embeddings,
cache keys, unstable compiler IDs, or popularity.

## Scope anti-patterns

The worst scope designs either over-select forever or narrow without evidence.

| Anti-pattern | Why it fails |
|---|---|
| universal workspace scope | safe only as a fallback; destroys useful incrementality when used as the normal model |
| changed-files-only scope | paths do not reveal generated, semantic, native, runtime, policy, or downstream effects |
| package-only scope | one package produces many targets, units, activities, artifacts, tests, and platform variants |
| dependency-graph-only scope | dependency edges do not express activity, capability, runtime coverage, hidden inputs, or finalization |
| command-name scope | two invocations of `cargo test` may select different packages, targets, features, platforms, filters, and gates |
| test-filter-as-build scope | runtime filtering usually does not identify the compiled test target or its dependencies |
| cache-key or artifact-name scope | lookup and filenames do not prove source, action, compatibility, freshness, integrity, or trust |
| branch, tag, channel, or label scope | human navigation does not establish the root's compatibility or covered work |
| AI-only semantic scope | embeddings or model judgment are not reproducible ownership, dependency, or validation evidence |
| compiler-private-ID scope | unstable query or cache identifiers cannot be a durable application contract |
| flattened “green” scope | compilation, selected tests, full validation, security, contracts, runtime, and deployment are different claims |
| configuration-free scope | omitting feature, profile, host/target, provider, ABI, or environment makes unrelated work appear equivalent |
| timeless scope | stale mappings silently survive source, toolchain, policy, platform, and lifecycle change |
| ownerless scope | no system can validate, renew, remove, or roll back the mapping |
| fallback-free scope | uncertainty becomes silent omission instead of conservative widening |
| maximally fine scope | collection and reasoning cost exceed avoided work and overwhelm AI context |

Broad scopes remain necessary as explicit fallbacks. The anti-pattern is
pretending a broad fallback is the only meaningful model, or pretending an
unsupported narrow scope is authoritative.

## Blueprint views

The scope model enables:

- “what does this file participate in?”;
- “which command activities compile this package differently?”;
- “which test binaries were compiled, and which test cases actually ran?”;
- “which contract, service, native, and deployment scopes depend on this
  item?”;
- “why did this feature or platform activate another provider?”;
- “which scopes were selected, omitted, unknown, or covered only indirectly?”;
  and
- “what is the smallest full-fallback boundary for this unknown input?”.

## Recommendations

### Adopt now

- Create SCOPE-001 separately from IDENTITY-001.
- Replace the single scope hierarchy with a canonical multi-dimensional scope
  taxonomy and typed mapping contract.
- Require every command record to separate package, target, activity,
  configuration, platform, compilation, execution, and validation scope.
- Make mapping cardinality, conditions, source, confidence, and unknown state
  explicit.
- Keep selected-plan and full-reference scope separate.
- Require owner-native anchor scopes, deterministic mapping and policy,
  evidence-gated AI narrowing, automatic conservative widening, and scope
  complexity budgets.

### Prototype behind a compatibility boundary

- Read-only source-file to package/target/activity scope map using Cargo
  metadata and repository validation declarations.
- Per-command scope comparison for `check`, `build`, `test`, `clippy`, `doc`,
  and doctest.
- Test compilation versus runtime-filter and executed-case view.
- Generated/native unknown mapping and conservative widening fixture.
- Feature/platform conditional scope fixture.
- Predicted versus observed mapping accuracy on held-out edits.

### Propose upstream

- stable Cargo unit-plan and source-to-target mapping evidence;
- machine-readable activity, test-target, doctest, and generated-output scope;
- rustc owner/body/interface and downstream-cutoff summaries;
- build-script, proc-macro, generator, native, and linker input/output mapping;
  and
- test-runner machine-readable compiled-binary, filter, execution, and coverage
  scope.

### Reject or defer

- one universal scope tree;
- package selection as the complete validation scope;
- test filters as compilation scope;
- file paths as semantic or artifact identity;
- lockfile membership as active target scope;
- compilation success as runtime, contract, package, or deployment coverage;
- unknown mappings treated as no effect;
- automatic mutation or validation deletion; and
- implementation before SCOPE-001 and held-out mapping conformance.

## Findings

### FERRIS-727: scope is multi-dimensional, not one hierarchy

**Sources:** PERF-Q01 through PERF-Q36, ECOS-Q01 through ECOS-Q12, FOREST-001.

**Observed behavior:** ownership, source, activity, configuration, platform,
compilation, validation, runtime, lifecycle, and evidence boundaries overlap
with different cardinalities.

**Implication:** Blueprint uses scope coordinates and typed mappings rather
than one containment tree.

**Confidence:** High.

### FERRIS-728: source scope and build scope are different

**Sources:** PERF-Q01, PERF-Q02, PERF-Q08 through PERF-Q15.

**Observed behavior:** files contain modules, items, and bodies; packages
produce multiple targets and units; compiler work occurs at owner, query, mono
item, and CGU scopes.

**Implication:** a changed path is the start of mapping, not the unit of build
or validation.

**Confidence:** High.

### FERRIS-729: command activity is an independent scope dimension

**Sources:** PERF-Q21.

**Observed behavior:** `check`, `build`, `clippy`, `test`, `doc`, and doctest
create different units, cfgs, stages, tool work, outputs, and execution.

**Implication:** every Cargo invocation records activity separately from
package, target, feature, profile, and platform.

**Confidence:** High.

### FERRIS-730: test compilation, filtering, execution, and coverage differ

**Sources:** PERF-Q21 and PERF-Q35.

**Observed behavior:** one compiled test target may contain many test cases;
runtime filters select executed cases without necessarily narrowing target
compilation.

**Implication:** Blueprint must not infer compiled, executed, passed, and
covered scopes from one test command or boolean.

**Confidence:** High.

### FERRIS-731: configuration and platform make mappings conditional

**Sources:** PERF-Q03, ECOS-Q04, ECOS-Q08, ECOS-Q11.

**Observed behavior:** features, dependency kind, cfg, profile, host/target,
provider, runtime, native tools, and platform capability activate different
packages, units, contracts, and validation.

**Implication:** scope edges carry explicit conditions and cannot be reused
outside them.

**Confidence:** High.

### FERRIS-732: generated and native scopes are dynamic execution cones

**Sources:** PERF-Q22, PERF-Q23, ECOS-Q05, ECOS-Q06, ECOS-Q09.

**Observed behavior:** macros, build scripts, generators, native discovery,
filesystem, environment, process, and network inputs can create or alter
source, configuration, link, and runtime scopes.

**Implication:** declared and observed mappings remain separate; hidden inputs
produce `UNKNOWN_EFFECT_ON` edges and conservative widening.

**Confidence:** High.

### FERRIS-733: application and contract scope are many-to-many

**Sources:** ECOS-Q03, ECOS-Q04, ECOS-Q11, CONTRACT-001 and APPLICATION-001
research.

**Observed behavior:** one semantic contract may have multiple Rust, C ABI,
WIT, wire, service, and test projections, while one application component may
implement or consume many contracts.

**Implication:** contract, package, component, service, operation, validation,
and deployment scopes require typed mappings rather than containment.

**Confidence:** High.

### FERRIS-734: a Query Forest root spans scopes

**Sources:** FOREST-001, BLUE-Q01, BLUE-Q02.

**Observed behavior:** one root binds evidence from multiple owners,
activities, platforms, validations, artifacts, and lifecycle states.

**Implication:** a root is an immutable cross-scope evidence projection, not
the parent scope of every referenced node.

**Confidence:** High.

### FERRIS-735: unknown mappings widen rather than disappear

**Sources:** PERF-Q22, PERF-Q23, PERF-Q30, PERF-Q35, ECOS-Q09.

**Observed behavior:** hidden, generated, native, runtime, policy, and
unsupported inputs can escape declared package mappings.

**Implication:** the plan selects the smallest safe owner boundary and records
the reason for full or partial fallback.

**Confidence:** High.

### FERRIS-736: SCOPE-001 must precede identity and planning

**Sources:** FERRIS-727 through FERRIS-735.

**Observed behavior:** identity compatibility and closure planning require a
defined subject, activity, condition, platform, lifecycle, and evidence domain.

**Implication:** create SCOPE-001 as a prerequisite to FOREST-002,
IDENTITY-001, EVIDENCE-001, PLANNING-001, and CONFORMANCE-001.

**Confidence:** High.

### FERRIS-737: owner-native anchors are the best baseline scopes for AI

**Sources:** PERF-Q02, PERF-Q21, PERF-Q35, BLUE-Q02, FOREST-001.

**Observed behavior:** repository, package, target, activity, platform,
validation gate, artifact, and root scopes are more stable and observable than
free-form semantic regions or compiler-private details.

**Implication:** AI planning starts from tool-native anchors and adds
fine-grained scope only through supported evidence.

**Confidence:** High.

### FERRIS-738: AI scope narrowing must be monotonic and evidence-gated

**Sources:** PERF-Q22, PERF-Q23, PERF-Q30, PERF-Q35.

**Observed behavior:** hidden inputs, stale mappings, and missed validation can
make an apparently precise scope unsound.

**Implication:** AI may propose narrowing, but deterministic policy or human
approval must establish it; stale or unknown evidence widens automatically.

**Confidence:** High.

### FERRIS-739: deterministic scope algebra must mediate AI reasoning

**Sources:** FOREST-001, BLUE-Q02, assurance and validation requirements.

**Observed behavior:** reproducible closure traversal, mandatory gates,
selected/full comparison, and fallback are rule-governed operations, while
semantic interpretation and explanation benefit from AI.

**Implication:** Blueprint keeps schema, mappings, set operations, policy, and
audit deterministic; AI operates at the proposal and explanation boundary.

**Confidence:** High.

### FERRIS-740: scope precision requires an economics budget

**Sources:** PERF-Q05, PERF-Q06, PERF-Q17, PERF-Q30, BLUE-Q02.

**Observed behavior:** collecting, storing, renewing, traversing, and
explaining fine-grained evidence can exceed the work it avoids.

**Implication:** every plan budgets scope detail and falls back to the nearest
safe anchor rather than pursuing maximal granularity.

**Confidence:** High.

### FERRIS-741: scopes that flatten claim domains are unsafe

**Sources:** PERF-Q05, PERF-Q21, PERF-Q30, PERF-Q35, BLUE-Q01.

**Observed behavior:** package names, command names, cache keys, refs, and one
passing result each omit identities or activities required for broader claims.

**Implication:** Blueprint rejects scopes that conflate selection,
compilation, execution, validation, trust, runtime, and deployment.

**Confidence:** High.

### FERRIS-742: ownerless, timeless, and fallback-free scopes cannot govern work

**Sources:** PERF-Q35, ECOS-Q05 through ECOS-Q12, BLUE-Q02.

**Observed behavior:** mappings decay as source, tools, platforms, policy, and
lifecycle change; unsupported mappings require an accountable renewal and
fallback boundary.

**Implication:** every authoritative scope needs an owner, evidence time,
renewal condition, and conservative fallback.

**Confidence:** High.

## Nine-role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: scope narrowing cannot remove safety, semantic, or mandatory validation obligations; unknowns widen. |
| Compiler Performance Engineer | Accepted: file, package, unit, query, mono-item, CGU, artifact, and link scopes remain distinct. |
| Interop Boundary Auditor | Accepted: contract, ABI, generated binding, native, runtime, and deployment mappings remain explicit. |
| AI Assurance Skeptic | Accepted: declared, resolved, observed, inferred, unsupported, and unknown mappings remain distinct evidence states. |
| Ecosystem Strategist | Accepted: Cargo, rustc, test tools, Typebook/RUNE, linkers, and platforms retain ownership of their local scopes. |
| Rust Maintainer | Accepted: stable Cargo metadata is the first boundary; no requirement depends on compiler-private scope identifiers. |
| Native Platform Adopter | Accepted: feature, provider, host/target, SDK, ABI, loader, runtime, and deployment conditions are first-class. |
| Scope Keeper | Accepted: SCOPE-001 defines vocabulary and mappings only; automatic selection, mutation, and restoration remain separately gated. |
| Validation Checker | Accepted: package/activity separation, filtered tests, generated/native unknowns, conditional mappings, full fallback, and removal require conformance fixtures. |

## Limitations

- Rust module and item mapping may require compiler or analyzer evidence that
  is unstable, incomplete, or unavailable.
- Test runners differ in filter, sharding, discovery, execution, and coverage
  reporting.
- Runtime service and deployment mappings are repository-specific.
- Generated and native effects cannot always be observed completely.
- No held-out study has yet measured scope-map accuracy or user comprehension.
