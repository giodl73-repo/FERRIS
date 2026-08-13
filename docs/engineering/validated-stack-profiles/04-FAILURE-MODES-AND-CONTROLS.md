# Failure Modes and Controls

Status: Guidance
Implementation authority: None

## Control objective

Profile controls prevent exact evidence from turning into broad authority.
They preserve failures, unsupported conditions, unavailable tools, stale
observations, unknowns, owner boundaries, ordinary Cargo behavior, and
reversibility.

The controls below apply independently to hosted service, CLI/config, pure
data, embedded/`no_std`, browser WASM, native dependency, desktop/GUI,
networking/protocol, and data/ML/GPU profiles.

## Failure-mode catalog

| Failure mode | Harm | Required control |
|---|---|---|
| Profile name used as identity | A moving stack appears stable | Require exact revision and evidence identity |
| Universal-stack promotion | Incompatible target and runtime assumptions merge | Keep families and consumer contracts independent |
| Compilation promoted to correctness | Behavioral, deployment, or safety gaps disappear | Record each stage independently |
| Lock universe treated as active closure | Cost and assurance scope are misstated | Retain both identities per target |
| Declared MSRV treated as tested floor | Missing declarations and exact-lock behavior vanish | Execute the exact compiler and closure |
| Zero advisories treated as secure | Dated database coverage becomes certification | Record database, tool, scope, time, limitations, expiry |
| Recent activity treated as maintained | Stewardship is inferred without owner evidence | Preserve observations and explicit declarations only |
| Artifact hash treated as reproducible | Output identity becomes a build reproducibility claim | Separate identity from reproduction evidence |
| Bundled native source treated as pure Rust | Compiler, ABI, SDK, and deployment boundaries vanish | Record source mode and complete native chain |
| `.wasm` artifact treated as browser support | JS glue and runtime execution are skipped | Require browser-specific package and execute stages |
| Cross-build treated as target support | Native execution and deployment remain unobserved | State check/build/link/execute outcomes separately |
| Host tests treated as embedded execution | Board, timing, panic, memory, and transport are untested | Require architecture and runner-specific evidence |
| AI summary treated as owner truth | Unsupported inference gains authority | Record assertion class, sources, and human decision |
| Scheduled green run treated as approval | Updates bypass consumer policy | Separate refresh from approval and merge authority |
| Automatic fallback hides failure | Unsupported or unavailable states look successful | Emit typed failure and preserve original diagnostics |
| Provider swap treated as equivalent | Semantic, native, cost, and operations changes disappear | Require substitution diff and migration fixtures |
| Lockfile restore treated as complete rollback | Data, ABI, deployment, or external state remains changed | Define full-system rollback |
| Profile metadata becomes mandatory | Ferris creates lock-in | Preserve ordinary Cargo and complete removal |
| Expired evidence remains selectable | Old observations appear current | Enforce expiry and stale state |
| Support inferred from local success | An owner is assigned obligations it never accepted | Require explicit support statement |

## Stage non-promotion

The following implications are forbidden:

- resolve implies check;
- check implies build;
- build implies link;
- link implies execute;
- execute implies semantic conformance;
- one test implies full test coverage;
- package implies deploy;
- deploy implies operational validation;
- current success implies future support;
- one platform implies another platform; or
- one target, feature, provider, or profile revision implies another.

Expected rejection is a successful negative control only for the exact
rejection contract. It must not be displayed as a general pass. Unsupported,
unavailable, not observed, stale, and unknown remain distinct.

## Exactness controls

Before relying on evidence, verify:

1. profile revision and schema version;
2. consumer and requirements identity;
3. manifest and Cargo lock identity;
4. direct releases and source modes;
5. requested and effective features;
6. lock universe and target-active closures;
7. toolchain, host, target, provider, native, and deployment identity;
8. command and working directory;
9. stage result and artifact identity;
10. evidence source, observer, date, and expiry; and
11. support, exception, and approval identity.

Any material mismatch requires a new observation or renewal. It must not be
papered over with an alias.

## Native and interoperability controls

Native and cross-language profiles must record ABI, ownership, lifetime,
allocation, exception, panic/unwind, threading, calling convention, symbol,
compiler, runtime library, generation, packaging, deployment, and cleanup
rules. Generated bindings require positive and negative compatibility tests.

Adapters are directional. They may lose fields, ordering, readiness,
backpressure, cancellation, error detail, or runtime semantics while still
compiling. The
[Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md)
requires these losses and migration paths to remain explicit.

## Performance controls

Do not convert one-machine artifact sizes, target-directory sizes, or elapsed
times into comparative performance claims without a dedicated measurement
contract. Record hardware, toolchain, command, cache state, isolation,
variance, confounders, and representative workflow.

The measured profile research explicitly rejected elapsed-time comparison
because concurrent registry-cache waiting contaminated observations. The
[Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md)
requires check, build, test, link, cold, and incremental workflows to remain
separate.

## Assurance controls

Assurance evidence must retain scope and expiry:

- checksum matches do not prove reviewed or reproducible source;
- compiler acceptance does not prove soundness or behavior;
- lint success does not prove absence of defects;
- advisory absence does not prove security;
- license metadata does not itself establish a legal decision;
- unsafe-code counts do not establish review quality;
- owner snapshots do not guarantee succession or response; and
- profile branding does not establish certification.

Apply the
[Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md) rule:
name where Rust's guarantees stop and what dedicated evidence supports any
stronger claim.

## Automation controls

Any future read-only profile capability must be unable to:

- edit manifests or lockfiles;
- change requested or effective features;
- update, downgrade, deduplicate, replace, or fork packages;
- switch providers, runtimes, source modes, or native dependencies;
- install toolchains, targets, SDKs, compilers, generators, or packages;
- add advisory ignores or policy exceptions;
- approve, reject, sign, attest, publish, deploy, or post upstream;
- delete validation; or
- convert failures into a success-shaped fallback.

Observation and explanation are narrower than recommendation; recommendation
is narrower than approval; approval is narrower than execution.

## Emergency controls

An emergency may be triggered by a critical advisory, revoked source,
compromised owner, severe incident, provider outage, toolchain regression,
target withdrawal, licensing change, support withdrawal, or invalid evidence.

The response record must:

1. identify the affected exact profile revisions and consumers;
2. preserve the source alert and time;
3. mark evidence revoked or stale without rewriting history;
4. distinguish confirmed impact from precautionary scope;
5. identify the current decision and support owners;
6. name immediate containment options and their limitations;
7. compare update, substitution, rollback, shutdown, and exception paths;
8. require explicit authority for mutation or deployment;
9. run the relevant positive, negative, migration, operational, and rollback
   validation; and
10. publish a replacement, supersession, or retirement record.

Emergency speed does not authorize silent dependency or environment mutation.
If evidence is incomplete, the state remains unknown or unavailable and
policy decides the safe action.

## Support failure controls

If a support owner, contact path, environment, validation service, advisory
source, registry, provider, or deployment target becomes unavailable:

- mark the affected evidence unavailable;
- do not infer continued support;
- identify the contractual grace or exception policy;
- evaluate earlier renewal, substitution, rollback, or retirement;
- retain diagnostics and escalation attempts; and
- avoid attributing the failure to an upstream owner without evidence.

## Diagnostic requirements

Diagnostics should identify:

- failed profile revision and consumer requirement;
- exact stage and typed state;
- changed package, feature, target, provider, native tool, contract, or
  environment element;
- owner-native command and bounded output;
- current source or owner;
- what remains supported, unsupported, unavailable, stale, or unknown;
- next evidence or decision owner; and
- rollback or ordinary Cargo path.

Maintainers should not need Ferris internals to understand the issue. This is
the [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md) control.

## Audit checklist

Before accepting a profile claim, ask:

- Is the claim scoped to an exact consumer, revision, environment, and stage?
- Are negative, unsupported, unavailable, stale, and unknown cases visible?
- Does the claim cite reproducible commands and dated owner evidence?
- Are safety, interoperability, performance, support, and operations claims
  supported by their own evidence?
- Can the consumer renew, substitute, roll back, and remove the profile?
- Does ordinary Cargo still work?
- Is implementation or mutation authority being implied where none exists?

The [Validation Checker](../../../.roles/editorial/validation-checker.md) and
[Scope Keeper](../../../.roles/editorial/scope-keeper.md) remain blocking
review lenses for these questions.

