# Environment Readiness and Requirement Sources

Date: 2026-09-16
Status: Complete; supports a bounded contract-design wave

## Decision supported

Ferris should add an owner-declared, read-only environment-readiness boundary.
It should normalize requirements and observations without installing tools,
interpreting owner commands, or replacing Cargo, rustup, environment managers,
containers, CI workflows, or operating-system package managers.

The first product slice should validate explicit executable availability,
environment-name presence, repository-relative file or directory presence, and
platform applicability. Version execution, package installation, shell
evaluation, service probes, network access, secret values, resource mutation,
and inferred requirements should remain closed.

## Research question

Can Ferris make cross-machine dependency failures clear before owner validation
runs while preserving the authority of the layer that actually owns each
requirement?

## Starting hypothesis

No single ecosystem format describes all Rust toolchain, native executable,
environment, filesystem, service, and resource prerequisites. Ferris can still
provide value by accepting an explicit product-neutral requirement record,
pointing to known owner-native sources, and comparing those declarations with
bounded local observations.

Competing hypotheses were:

1. Cargo metadata already provides enough information;
2. one existing environment format should become Ferris's canonical input; or
3. ordinary owner-command failure is sufficient diagnosis.

## Local evidence

Ferris's passive `doctor` currently validates one `Cargo.toml`, runs a bounded
`cargo --version`, and records Cargo identity, bounded output, unknowns, and
limitations. It does not accept owner-declared external prerequisites
([`ferris-core/src/lib.rs`](../../crates/ferris-core/src/lib.rs), lines
6283-6405).

The enterprise validation history encountered distinct missing-readiness
classes:

- absent Linux PowerShell;
- Cargo omitted from process `PATH`;
- an unavailable ancestor-configured `rustc-wrapper`;
- missing Windows environment names needed by PowerShell, Cargo, and the linker;
- missing `pytest`;
- insufficient target-storage capacity; and
- missing FFmpeg for non-ignored REEL tests.

Every case stopped safely, but most were discovered sequentially by attempting
owner work rather than by one readiness report
([`CONTEXT.md`](../../CONTEXT.md), lines 205-252, 331-400, and 433-517).

## External evidence

| Source | What it owns | Useful declared evidence | Boundary |
|---|---|---|---|
| [Cargo package metadata](https://docs.rs/cargo_metadata/latest/cargo_metadata/struct.Package.html) | Rust packages and Cargo dependency declarations | `rust_version`, targets, dependencies, `links`, and free-form `package.metadata` | It does not standardize arbitrary native tools, environment names, services, or machine capacity |
| [rustup toolchain files](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) | Rust toolchain selection | Channel or path, profile, components, and targets | rustup may install missing components; Ferris must observe without triggering installation |
| [asdf `.tool-versions`](https://github.com/asdf-vm/asdf/blob/master/docs/manage/configuration.md#tool-versions) | Multi-tool version selection | Tool names and version requests | Plugin meaning and installation remain owned by asdf |
| [mise configuration](https://mise.jdx.dev/configuration.html) | Tools, environment, and tasks | Version requests, platform conditions, environment names, and task relationships | Configuration is hierarchical and may include installation or executable task behavior |
| [Development Containers](https://containers.dev/implementors/json_reference/) | Containerized development environment construction | Features, environment, users, mounts, ports, and lifecycle commands | Materialization and lifecycle commands are active operations, not passive observations |
| [Development Container Features](https://containers.dev/implementors/features/) | Reusable container tooling installation | Feature identity, version, options, environment, capabilities, and install entrypoint | Features install and may require image-specific behavior |
| [Devfile](https://devfile.io/docs/2.3.0/what-is-a-devfile) | IDE and cloud-native development environment | Components and instructions to configure, build, run, and deploy | Its lifecycle semantics exceed a local readiness check |

## Findings

### FERRIS-787: Real adopter failures span several dependency owners

**Sources**

- [`CONTEXT.md`](../../CONTEXT.md), lines 205-252, 331-400, and 433-517
- [`2026-09-15-reel-scene-delivery-value-cohort.md`](2026-09-15-reel-scene-delivery-value-cohort.md)

**Observation**

The observed failures included tool installation, path visibility, inherited
configuration, environment completeness, language-package availability,
native executable availability, and storage capacity. Cargo owned only part of
that truth.

**Implication**

Ferris should model readiness dimensions and their owners explicitly. A single
generic “dependency missing” state would lose the information users need.

**Confidence:** High; the cases are retained execution evidence.

### FERRIS-788: Existing formats are complementary, not interchangeable

**Sources**

- [rustup toolchain-file documentation](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
- [asdf `.tool-versions` documentation](https://github.com/asdf-vm/asdf/blob/master/docs/manage/configuration.md#tool-versions)
- [mise configuration documentation](https://mise.jdx.dev/configuration.html)
- [Development Container JSON reference](https://containers.dev/implementors/json_reference/)
- [Devfile overview](https://devfile.io/docs/2.3.0/what-is-a-devfile)

**Observation**

The formats overlap on tool or environment setup but have different discovery,
precedence, installation, container, command, and lifecycle semantics. None is
a passive, universal readiness vocabulary.

**Implication**

Ferris should not designate one format as global truth or partially emulate its
resolver. It should preserve source kind, path, digest, authority, and
interpretation status for every adapter observation.

**Confidence:** High; the formats' primary documentation defines materially
different ownership.

### FERRIS-789: A normalized comparison is the missing product capability

**Sources**

- [`FERRIS_PROGRAM.md`](../plans/FERRIS_PROGRAM.md), lines 19-32 and 84-102
- [`FERRIS_VALIDATION_COVERAGE_CONTRACT.md`](../specs/FERRIS_VALIDATION_COVERAGE_CONTRACT.md),
  lines 103-165
- [`ferris-core/src/lib.rs`](../../crates/ferris-core/src/lib.rs), lines
  6283-6405

**Observation**

Ferris already owns cross-workspace planning, diagnostics, typed unavailable
states, and evidence while leaving local work to owner tools. The current
doctor record has bounded observations but only a Cargo-specific requirement.

**Implication**

The defensible extension is a product-neutral requirement-to-observation
report. It should distinguish `satisfied`, `missing`, `mismatched`,
`unsupported`, `unavailable`, and `not_observed`; retain provenance and
limitations; and fail closed only for requirements the owner marks required.

**Confidence:** High; this extends an implemented boundary without changing
Cargo or owner-command authority.

### FERRIS-790: Readiness must be useful without becoming environment automation

**Sources**

- [Engineering principles FP-05, FP-06, FP-07, and FP-10](../governance/ENGINEERING_PRINCIPLES.md)
- [Development Container Features](https://containers.dev/implementors/features/)
- [mise configuration](https://mise.jdx.dev/configuration.html)

**Observation**

Several source formats can install tools or execute lifecycle commands. Running
those behaviors would mutate the environment and import another tool's policy.
Presence checks and declarative parsing can remain read-only.

**Implication**

Initial Ferris adapters must never install, update, activate, evaluate shell,
run lifecycle tasks, access networks, or expose environment values. Unsupported
constructs remain explicit rather than being guessed.

**Confidence:** High; the mutation boundary is both technically observable and
required by Ferris governance.

## Model evolution

The starting hypothesis was confirmed, with one refinement: pointing at known
formats is not enough unless Ferris records whether each format was merely
identified, safely interpreted, unsupported, conflicting, or not observed.
Source provenance is part of the result, not incidental metadata.

## Adopt now

- Treat environment readiness as a first-class Ferris product problem.
- Design one strict, versioned, product-neutral requirement and report contract.
- Extend the existing `doctor` concept rather than add an unrelated command.
- Keep requirement declaration separate from local observation.
- Make required-versus-advisory policy explicit and owner-controlled.
- Preserve exact source provenance and conflicts across machines.

## Prototype behind a compatibility boundary

The smallest future implementation should accept one explicit local
requirements file and perform only:

1. platform and architecture comparison;
2. executable-name resolution without shell evaluation;
3. environment-name presence without retaining values;
4. bounded repository-relative file and directory checks; and
5. deterministic typed output with a fail-closed option.

Known-format adapters should follow only after frozen fixtures prove their
read-only subsets. An adapter result must never outrank an explicit owner
declaration.

## Reject or defer

- automatic installation or repair;
- command, script, workflow, or lifecycle execution;
- shell evaluation;
- inferred requirements from failed log text;
- secret values or reusable credentials;
- OS package-manager resolution;
- service, daemon, port, network, or credential probes;
- free-space enforcement;
- hidden parent or user configuration discovery;
- CI equivalence or support certification; and
- treating a present executable as proof that owner validation will pass.

## Contribution path

Adopt rustup, Cargo, asdf, mise, Development Container, and Devfile semantics
through bounded adapters where their documented contracts are sufficient.
Contribute adapter defects upstream when the owning format is ambiguous.
Maintain only Ferris's normalized readiness and evidence contract.

## Open questions

1. Should the first contract accept only an explicit Ferris file, or also
   owner-supplied pre-normalized adapter observations?
2. Which version comparison forms can remain product-neutral without invoking
   tools?
3. Should fail-closed policy be per requirement, per validation entrypoint, or
   both?
4. How should conflicting owner-native sources be presented without selecting
   a winner?

## Pulse 02 resolution

READINESS-001 resolves the V1 questions as follows:

- V1 accepts only one explicit Ferris requirements file;
- V1 performs no version-command execution or comparison;
- required/advisory policy is per requirement;
- V1 interprets no owner-native source and therefore has no source precedence
  or conflict observation;
- future adapter conflicts require a later schema and explicit VIEW-001
  incomplete-versus-blocked semantics; and
- the readiness report is the record payload of the existing
  `ferris.command-result/v2` envelope.

## Role review

The associated
[eleven-role review](../plans/reviews/FERRIS_ENVIRONMENT_READINESS_RESEARCH_ROLE_REVIEW.md)
accepts this as a research and contract-design direction only. No product code,
schema status, adapter, probe, installation, or execution authority follows.
