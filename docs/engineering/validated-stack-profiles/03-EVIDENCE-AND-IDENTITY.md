# Evidence and Identity

Status: Guidance
Implementation authority: None

## Identity rule

A profile name is never sufficient identity. The meaningful identity joins
the exact consumer boundary, selected sources and releases, requested and
effective features, Cargo lock, lock universe, target-active closures,
contracts, toolchain, targets, providers, native environment, stage evidence,
support statement, observation date, and expiry.

Every profile record should be immutable after approval. Corrections,
renewals, substitutions, supersession, and revocation create new revisions
linked to the prior record. Labels and aliases may aid discovery but cannot
prove compatibility, support, integrity, freshness, or approval.

This follows [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
and Query Forest's identity separation in the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Core record identity

At minimum, record:

- stable profile ID and exact revision;
- schema version;
- profile family;
- owner and approval authority;
- consumer, application, repository, and component scope;
- creation, observation, approval, expiry, revocation, and supersession times;
- predecessor and replacement identities;
- support state;
- requirements identity;
- evidence collection identity; and
- references to adoption, exception, renewal, substitution, rollback, and
  removal decisions.

Times should identify the clock basis and timezone. Source observations should
retain the time at which each source was queried rather than inheriting one
undifferentiated profile date.

## Requirements identity

Requirements are versioned inputs, not prose attached after selection. Retain:

- mandatory eligibility requirements;
- preferences and tradeoff criteria;
- representative operations and expected outcomes;
- contract, platform, target, runtime, provider, native, deployment, policy,
  assurance, performance, resource, and support constraints;
- accepted alternatives;
- unsupported combinations;
- non-goals; and
- approval and exception policy.

A changed requirement creates a changed profile evaluation even when packages
and lockfiles remain identical.

## Selection and source identity

Record for every direct package and source:

- package name and exact release;
- registry, Git, path, vendored, bundled, or system source mode;
- source locator and revision where applicable;
- requested features and default-feature policy;
- registry checksum and independently observed archive hash where applicable;
- packaged VCS revision where available;
- publication authority and dated owner snapshot;
- license declarations and consumer policy result; and
- yanked, revoked, unavailable, or replaced state.

Checksums establish bounded source identity. They do not prove source
reproduction, review quality, soundness, future owner continuity, or support.
See the
[profile research](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

## Cargo identity

Cargo remains authoritative for source selection, dependency resolution,
workspace membership, targets, features, lock state, build units, freshness,
and compiler invocation. A profile records Cargo truth; it does not recreate
it.

Retain separately:

1. manifest identity;
2. Cargo lock identity;
3. complete lockfile package universe;
4. target-active normal closure;
5. target-active build closure;
6. target-active development closure where applicable;
7. requested features;
8. effective features per package and target;
9. build scripts and their packages;
10. procedural macros;
11. `links` values and native packages;
12. generated code and generators;
13. unsafe review scope;
14. public dependency exposure; and
15. exact commands and Cargo version used to observe each view.

The lock universe is not the active graph. The measured native profile had 41
lock packages and 26 active Windows packages; the pure-data profile had 46 and
21. Cost, assurance, and platform claims must name the closure they cover.

## Toolchain and environment identity

Record:

- Cargo and rustc versions;
- toolchain channel and exact compiler identity;
- host triple and every target triple;
- installed targets and components;
- linker, archiver, debugger, and runner;
- native compilers, SDKs, sysroots, generators, package managers, and system
  packages;
- runtime and provider;
- container, VM, filesystem, and execution substrate;
- environment variables or configuration that materially affect behavior,
  with secrets excluded;
- deployment target and platform image;
- hardware relevant to performance, embedded, GUI, ML, or GPU claims; and
- observation time.

A declared `rust-version` can select a candidate compiler floor, but an
observed floor requires the exact lock, target, features, command, and
compiler. The profile must expose packages that omit `rust-version`.

## Contract identity

Keep identity domains distinct:

- Rust package and source API identity;
- Typebook/RUNE semantic contract identity;
- Rust compiler metadata identity;
- C ABI identity;
- WIT/component identity;
- wire and persisted-data schema identity;
- adapter identity;
- native library and artifact identity; and
- deployment and operational identity.

Do not hash these into one field and treat equality as universal
compatibility. Each domain has different owners, evolution rules, and
validation. The layered contract model is described in the
[enterprise platform plan](../../plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md).

## Stage evidence matrix

Every stage result should carry:

- stage name and typed state;
- exact command or owner-native action;
- working directory and selected manifest;
- input profile, environment, and contract identities;
- start and completion times;
- exit or owner result;
- bounded stdout, stderr, diagnostics, or external evidence;
- produced artifact identities;
- source, observer, and assertion class;
- limitations and residual unknowns; and
- evidence expiry.

Required stages are resolve, check, lint, build, link, execute, unit test,
integration test, doctest, contract conformance, package, sign or attest,
deploy, operational validation, and rollback. A family may mark a stage
unsupported or not applicable only through an explicit contract rule; it may
not silently omit the stage.

Stage states must distinguish pass, fail, expected rejection, unsupported,
unavailable, not observed, stale, and unknown. Conflicting source evidence
should remain conflicting until an owner resolves it.

## Assurance and stewardship identity

Assurance evidence must state:

- source and registry identity;
- archive checksum and packaged revision;
- advisory database, tool version, query time, and inspected closure;
- license source and policy decision owner;
- unsafe, macro, build-script, generated, and native review scope;
- known incidents, exceptions, and residual unknowns; and
- evidence expiry.

Stewardship evidence must state:

- current declared owner or custodian and source;
- release and repository activity as observations, not rankings;
- security and support channels;
- succession, transfer, seeking-maintainer, or unmaintained declarations;
- attempted contact and response where policy permits;
- known successor candidates with compatibility evidence; and
- observation date and expiry.

Ferris must not infer abandonment from age, activity, downloads, funding,
owner count, or issue totals. See
[ecosystem intervention decisions](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Support identity

Support is its own signed or approved statement, not a deduction from test
results. Identify the support owner, profile revisions, exact supported
combinations, start and end dates, security response, update and emergency
policy, diagnostics, escalation, exception process, training requirements,
and unsupported combinations.

If a provider or enterprise supports a profile, the statement must not be
misattributed to the Rust Project, Rust Foundation, Bytecode Alliance,
crates.io, or individual maintainers.

## Evidence freshness and renewal

The default maximum evidence age should be 90 days. Each evidence item may
have a shorter source-specific expiry. Expired evidence becomes stale; it does
not disappear and must not be refreshed by copying the old result.

Renewal creates a new evidence identity and a reviewed diff. It must compare
identity, closure, features, contracts, environment, validation, assurance,
stewardship, support, limitations, removal, and rollback. Unchanged package
count does not prove unchanged risk or behavior.

## Privacy and redaction

Credentials and reusable secrets must never enter profile records, commands,
diagnostics, logs, roots, refs, fixtures, or generated summaries. Record the
presence and owner of a credential requirement without recording the secret.
Paths, environment details, and owner output should be bounded and redacted
according to declared policy while preserving enough identity to reproduce the
evidence.

## Evidence quality review

The [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md)
requires source revision, action, commands, results, visible failures, and
human approval boundaries. The
[Validation Checker](../../../.roles/editorial/validation-checker.md) requires
reproducible commands, representative fixtures, recorded environments, and
negative cases. Neither role permits generated prose to replace owner evidence.

