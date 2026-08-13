# Boundary and Ownership

Status: Guidance
Implementation authority: None

## Purpose

This guide assigns responsibility for future reference companions without
moving authority away from Cargo, rustc, consumer repositories, platform
owners, native systems, Typebook/RUNE, deployment systems, or upstream
maintainers.

## Conformance ownership

The Conformance Program owns:

- fixture and suite contracts;
- case classification and coverage dimensions;
- exact binding and renewal rules;
- expected-output and scorer contracts;
- held-out custody and anti-leak controls;
- selected-only versus full-reference comparison;
- pass, fail, stop, disable, and promotion thresholds; and
- fixture expiry, quarantine, replacement, and retirement records.

Conformance does not own the semantics exercised by a fixture. Producing
programs define those contracts, and current owners remain authoritative.
Program boundaries are defined in the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

## Responsibility map

| Concern | Primary owner | Companion obligation |
|---|---|---|
| Packages, sources, resolution, lock state, features, targets, profiles, build units | Cargo | Invoke Cargo through recorded owner-native commands |
| Compiler acceptance and compiler-derived evidence | rustc | Record exact toolchain and avoid behavioral overclaim |
| Application intent and support commitments | Consumer | Supply requirements, approval, validation, exceptions, and lifecycle |
| Blueprint Model and Plan | Blueprint | Keep plans non-executable and preserve uncertainty |
| Identity, evidence, roots, lineage, and projections | Query Forest | Preserve typed domains and immutable evidence history |
| Semantic contracts | Typebook/RUNE or current standard owner | Test exact contract identity and projection loss |
| Native compilation, ABI, linking, loading, packaging | Native and platform owners | Exercise native tools and expose boundary failures |
| Deployment and operations | Provider and consumer operators | Own rollout, recovery, and operational truth |
| Public contribution process | Current upstream owner | Define acceptable packet and submission workflow |
| Fixture custody and scoring | Validation owner | Separate authors, inputs, oracle, collection, and score release |

## Boundary records

Every companion declares:

1. capability under test;
2. producing program and governing specification versions;
3. owner whose semantics are exercised;
4. owner-native command or procedure;
5. hermetic controls applied around that procedure;
6. inputs Ferris may observe;
7. outputs Ferris may normalize;
8. states Ferris may infer or predict;
9. actions that remain prohibited;
10. support, renewal, rollback, removal, and retirement owners.

Missing ownership is a blocking unknown, not permission for Ferris to fill the
gap.

## Hermetic boundary

A hermetic fixture may control:

- repository revision and input archive;
- dependency and toolchain acquisition policy;
- environment-variable allowlist;
- network access;
- filesystem roots and target-directory isolation;
- locale, time zone, clock source, and randomness;
- CPU, memory, time, output, process, and storage bounds;
- command framing and output collection; and
- cleanup and evidence sealing.

It must not replace:

- Cargo resolution with a Ferris resolver;
- rustc acceptance with a synthetic compiler result;
- repository tests with an invented validation set;
- native discovery with hidden Ferris metadata;
- platform execution with cross-compilation alone;
- upstream review with packet shape matching; or
- consumer approval with a fixture result.

Hermetic controls are part of environment identity and may affect results.
They are never silently omitted from publication.

## Owner-native full reference

The full reference is the complete owner-approved workflow against which a
selected or optimized path is compared. It must:

- use the same source, lock, toolchain, platform, and relevant environment;
- run independently from the Ferris-selected lane;
- include repository-mandatory and consumer-mandatory gates;
- preserve owner exit status and diagnostics;
- record any non-equivalence or unavailable step; and
- remain executable after Ferris metadata and integration are removed.

A full reference need not be identical across platforms. Platform-specific
commands and expected results are valid when their ownership and support scope
are explicit.

## Companion family boundaries

### Blueprint applications

Companions may exercise normalized application intent, declared workspaces,
typed scope mappings, plans, explanations, and expected evidence. They must not
authorize affected-only selection, action, or execution under current
authority. Cargo and repository scripts remain the full reference.

### Renewable profiles

Profile companions may bind exact releases, features, source, lock identity,
active target closure, compiler and target, native prerequisites, validation
stages, expiry, renewal, substitution, rollback, and removal. They must not
become a Ferris distribution or universal supported stack. The evidence model
follows the
[compatibility stack profile research](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

### AI-generated patches

Patch companions may record model, instructions, context cutoff, proposed
patch, deterministic scope, compiler evidence, behavioral tests, rejection,
approval, and rollback. AI cannot establish owner truth, remove mandatory
validation, approve the change, or transform confidence into proof.

### Native boundaries

Native companions must keep ABI, calling convention, ownership, lifetime,
allocation, panic or exception, threading, layout, generation, linking,
loading, runtime, packaging, and removal contracts explicit. Rust compilation
alone is not native-boundary proof.

### Platform targets

Platform companions qualify named stages for exact host, target, toolchain,
SDK, linker, runner, provider, filesystem, and deployment assumptions.
Target availability or `cargo check` cannot imply link, run, test, package, or
deploy support.

### Upstream packets

Packet companions may produce a public-safe reproducer and bounded owner ask.
They do not authorize issue creation, comments, branches, pull requests,
external upload, or support commitments.

## Segregation of duties

For a held-out score:

- implementation authors may receive the public contract and input only;
- fixture authors must not tune the implementation;
- the Validation Checker controls oracle release;
- the Scope Keeper confirms the fixture was not used for development;
- the AI Assurance Skeptic reviews model and prompt exposure;
- collection preserves raw owner and Ferris results before scoring; and
- scorer defects, harness defects, and implementation failures remain distinct.

The repository's held-out history demonstrates why invalid harnesses and
scorers cannot be reported as product failures; see the
[held-out manifest](../../simulations/held-out/MANIFEST.md) and
[public-safe result FHIF-030](../../simulations/held-out/PUBLIC_SAFE_DOCTOR_RESULT_022.md).

## Nine-role minimum concerns

| Role | Blocking question |
|---|---|
| Rust Safety Steward | Are safety boundaries and compiler-evidence limits explicit? |
| Compiler Performance Engineer | Are representative workflows, baselines, cache state, variance, and limitations recorded? |
| Interop Boundary Auditor | Are lost semantics and negative native-boundary cases exercised? |
| AI Assurance Skeptic | Are model assertions separated from deterministic and owner evidence? |
| Ecosystem Strategist | Is this a missing proof capability rather than a weaker duplicate? |
| Rust Maintainer | Are diagnostics, maintenance, ordinary Cargo use, and removal understandable? |
| Native Platform Adopter | Are platform, ABI, deployment, training, support, rollback, and audit costs explicit? |
| Scope Keeper | Is the companion bounded and are deferred capabilities visible? |
| Validation Checker | Are claims tied to exact commands, environments, cases, and observed results? |

## Prohibited ownership transfers

A companion must not imply that Ferris:

- maintains a third-party crate or tool;
- supports a platform the platform or consumer has not qualified;
- approves a generated patch;
- certifies an ABI or deployment;
- owns an upstream issue after packet preparation;
- can publish private evidence because a public fixture exists; or
- can recreate hidden owner truth after removal.

These boundaries implement the governing rule: the plan is global; the work is
local.
