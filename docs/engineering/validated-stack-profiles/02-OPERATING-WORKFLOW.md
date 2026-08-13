# Operating Workflow

Status: Guidance
Implementation authority: None

## Workflow principles

The profile workflow separates consumer intent, owner observation, evidence
normalization, review, approval, operation, renewal, substitution, and
retirement. No earlier record grants authority to create a later one.

The workflow uses Cargo and each external system as the source of owner truth.
It does not create a parallel resolver, hidden manifest, synthetic target
graph, automatic installer, or success-shaped fallback. See
[AGENTS.md](../../../AGENTS.md) and the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Step 1: Define the consumer contract

Record:

- consumer, application, repository, component, and approval authority;
- required operations and semantic outcomes;
- mandatory compiler, target, platform, provider, native, deployment,
  assurance, policy, performance, and support constraints;
- preferences that may influence comparison but cannot override eligibility;
- accepted alternatives;
- explicit non-goals and unsupported combinations; and
- required adoption, renewal, substitution, rollback, and removal outcomes.

Choose one independently scoped family: hosted service, CLI/config, pure data,
embedded/`no_std`, browser WASM, native dependency, desktop/GUI,
networking/protocol, or data/ML/GPU. Split mixed applications into multiple
profiles when their contracts or evidence differ.

## Step 2: Identify candidates without approving them

Candidate discovery records source, query, date, candidate version, current
owner, claimed capabilities, known limitations, and why it might satisfy the
consumer contract. Popularity, downloads, recency, package count, or a
composite score cannot establish eligibility.

The result is a candidate set, not a recommendation or adopted profile.
Existing owner contracts and contribution paths should be preferred over new
Ferris wrappers or standards. See
[ecosystem intervention decisions](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Step 3: Freeze exact selection identity

For each candidate revision, capture:

- exact direct releases, source modes, checksums, and requested features;
- manifest identity and Cargo lock identity;
- complete lockfile package universe;
- target-active normal, build, and development closures as applicable;
- effective features and public dependency exposure;
- build scripts, procedural macros, `links`, generated code, unsafe and native
  boundaries;
- contract, adapter, provider, and accepted alternative identities; and
- the commands and source dates used to collect the record.

Lock universe and active closure must remain separate. The measured profiles
showed substantial differences, such as 41 lock packages versus 26 active
Windows packages for the native fixture. See
[compatibility stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

## Step 4: Freeze the environment

Record the exact Cargo and rustc versions, toolchain channel, host and target
triples, installed components, linker, archiver, debugger, runner, native
compiler, SDK, generator, package manager, system packages, runtime, provider,
container or VM, filesystem, execution substrate, deployment target, and
evidence date.

The highest declared `rust-version` may select a compiler-floor experiment,
but only execution with the exact closure and command establishes an observed
floor. Packages without `rust-version` and future resolution prevent a
metadata-only promise.

## Step 5: Execute the stage matrix

Record each applicable stage independently:

| Stage | Minimum question |
|---|---|
| Resolve | Did Cargo produce the exact intended selection under recorded inputs? |
| Check | Did type checking complete for this target and feature set? |
| Lint | Did the declared lint policy execute and what scope did it inspect? |
| Build | Were expected compilation units and artifacts produced? |
| Link | Did final native or target linking complete? |
| Execute | Did the representative operation run in the intended runtime? |
| Unit test | Did package-local behavior pass under the exact profile? |
| Integration test | Did component interaction pass at the selected boundary? |
| Doctest | Did documented examples execute where applicable? |
| Contract conformance | Did positive and negative semantic fixtures pass? |
| Package | Was the deployable or distributable form produced? |
| Sign or attest | Was the required owner process completed? |
| Deploy | Did the exact artifact reach the declared environment? |
| Operational validation | Did health, telemetry, recovery, and service checks pass? |
| Rollback | Was the prior approved state restored and revalidated? |

Use only pass, fail, expected rejection, unsupported, unavailable, not
observed, stale, unknown, or another explicitly specified typed state. Never
promote one stage into another.

## Step 6: Collect assurance and stewardship evidence

Capture source and registry identity, archive checksum, packaged revision,
publication authority, license and policy result, advisory database and tool,
unsafe/build-script/macro/native review scope, ownership and succession
evidence, support statements, incidents, exceptions, residual unknowns, and
expiry.

Zero advisory matches are dated query evidence, not security certification.
Recent repository activity is not maintenance proof. Artifact hashes identify
observed outputs but do not prove reproducibility. Apply the
[Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md) and
[Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md)
lenses.

## Step 7: Review the candidate packet

Review must compare requirements to evidence and identify:

- satisfied mandatory requirements;
- failed, expected-rejection, unsupported, unavailable, stale, and unknown
  conditions;
- evidence gaps and collection limitations;
- exact support commitments and exclusions;
- alternatives and tradeoffs;
- operational, performance, native, interoperability, and maintenance cost;
- renewal triggers and default expiry;
- adoption, substitution, rollback, and removal work; and
- the authority responsible for each decision.

All nine repository roles should record dispositions before a gate is claimed.
Role review is not implementation authority.

## Step 8: Approve or reject adoption

The consumer approval authority chooses one of:

- approve the exact profile revision;
- approve with recorded exception and expiry;
- request more evidence;
- reject for a named reason; or
- defer until an owner, environment, validation, or support dependency is
  available.

Approval records the exact evidence identity and cannot float to a newer lock,
feature set, toolchain, provider, target, or support statement.

## Step 9: Operate and monitor expiry

During the support period:

- preserve the approved profile and historical evidence;
- execute only the declared support and emergency procedures;
- record incidents, revocations, unavailable sources, and environment drift;
- keep ordinary Cargo commands available; and
- avoid silent package, feature, provider, native, or deployment changes.

The default maximum evidence age is 90 days unless a shorter consumer or risk
policy applies.

## Step 10: Renew by reviewed diff

Renew earlier when dependencies, features, contracts, adapters, advisories,
owners, licensing, compiler, Cargo, target tier, SDK, native tools, providers,
deployment, required stages, policy, requirements, or evidence validity
changes.

The renewal diff must compare:

1. profile and source identities;
2. manifests, lock identities, and direct releases;
3. lock universe and every target-active closure;
4. requested and effective features;
5. contracts, adapters, providers, unsafe, generated, and native boundaries;
6. toolchains, targets, tools, environments, and deployment;
7. every stage result and artifact;
8. provenance, advisories, licensing, stewardship, and support;
9. limitations, exceptions, unknowns, and expiry; and
10. substitution, removal, and rollback procedures.

A green scheduled run may refresh evidence. It cannot approve adoption or
merge an update.

## Step 11: Substitute, roll back, or remove

Substitution requires explicit contract comparison and positive, negative,
migration, operational, and rollback fixtures. Rollback restores more than a
lockfile whenever data, wire, ABI, native artifact, deployment, or external
state changed. Removal deletes profile-specific metadata and automation while
preserving ordinary Cargo behavior and historical evidence.

Detailed procedures are in
[Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md).

