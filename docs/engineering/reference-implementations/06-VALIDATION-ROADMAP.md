# Validation Roadmap

Status: Guidance
Implementation authority: None

## Purpose

This roadmap identifies the evidence and review gates required before future
executable companions could support the Conformance capability. It does not
authorize implementation, create a Proposed specification, or claim a
repository is ready.

## Current state

The current state is guidance only:

- the 22-specification spine is Draft;
- specification simulations are complete at Draft but are not runtime proof;
- held-out fixture classes and custody controls exist;
- exact future companion repositories, commands, schemas, expected outputs,
  environments, and thresholds remain blockers;
- current implementation authority remains read-only `plan`, `explain`,
  declared-workspace `graph`, and passive local `doctor`; and
- no new product code or executable companion is authorized here.

See the [specification registry](../../specs/README.md),
[simulation registry](../../simulations/README.md), and
[held-out fixture registry](../../simulations/held-out/README.md).

## Gate 0: guidance completeness

Exit criteria:

- taxonomy and ownership are explicit;
- hermetic and owner-native lanes are separated;
- identity, evidence, privacy, anti-leak, portability, failure, adoption,
  rollback, removal, maintenance, expiry, and retirement are covered;
- candidate companion families are bounded;
- all links and ASCII constraints validate; and
- no implementation or conformance claim is made.

This guide series targets Gate 0 only.

## Gate 1: candidate charter

For one candidate family, produce a reviewed charter with:

- named consumer and owner;
- bounded claim and non-goals;
- producing program and specification dependencies;
- owner-native full reference;
- applicable Conformance suites;
- positive, negative, failure, unsupported, stale, skew, platform, rollback,
  and removal cases;
- exact privacy and publication boundary;
- maintenance, cost, expiry, replacement, and retirement plan; and
- measurable pass, fail, stop, and disable thresholds.

All nine roles must record dispositions. Implementation authority remains none.

## Gate 2: public contract and repository selection

Bind:

- repository custody model;
- public or private source policy;
- immutable revisions;
- license and redistribution rights;
- fixture layout and population separation;
- owner-native commands;
- host and target matrix;
- toolchain and environment acquisition;
- expected output and diagnostic contract;
- viewer, collector, harness, and scorer responsibilities; and
- cleanup and ordinary-operation proof.

Selection should prefer current owner-maintained public repositories where
they are representative and stable, with sealed edit packs when anti-leak
separation is needed. Forking is justified only by a named fixture or custody
requirement.

## Gate 3: separately approved implementation pulse

A pulse must bound:

- files and repositories that may change;
- product surfaces, if any;
- exact development fixtures;
- prohibited held-out access;
- commands, schemas, and outputs;
- resource and privacy constraints;
- stop conditions;
- targeted tests;
- rollback and removal; and
- review and completion evidence.

No pulse may inherit implementation authority from this roadmap or from Draft
CONFORMANCE-001.

## Gate 4: development fixture qualification

Required evidence:

- owner-native baseline on each claimed platform;
- deterministic setup and cleanup;
- exact identity capture;
- positive and negative behavior;
- bounded failures and diagnostics;
- selected-only versus full-reference comparison where applicable;
- privacy and redaction tests;
- rollback and complete removal;
- maintenance-cost measurement; and
- all nine role dispositions on observed results.

Development fixtures may be debugged. They cannot support a held-out claim.

## Gate 5: harness and scorer qualification

Before held-out scoring:

- qualify success and every non-success class;
- qualify expected process and record cardinality;
- qualify stream framing, truncation, encoding, and storage;
- qualify all accepted semantic layouts;
- qualify every oracle branch;
- prove scorer determinism;
- prove raw evidence is sealed before oracle release;
- test invalid fixture, harness, collection, and scorer classification; and
- freeze collector, harness, viewer, and scorer identities.

## Gate 6: blind held-out score

Required sequence:

1. freeze implementation, configuration, and evidence cutoff;
2. verify sealed input digests;
3. execute Ferris and owner-native full reference independently;
4. capture and seal complete evidence;
5. release the oracle;
6. score without implementation or oracle changes;
7. classify result and infrastructure validity;
8. quarantine the fixture after scoring; and
9. publish only public-safe evidence.

A failure becomes development evidence. A leaked or invalid fixture is
replaced, not rerun.

## Gate 7: promotion

Promotion to a supported companion requires:

- applicable held-out pass;
- Windows and Unix evidence where required;
- fixed thresholds met;
- no unresolved mandatory-gate omission;
- ordinary Cargo and owner workflow preserved;
- privacy, retention, and public-safe publication approved;
- rollback, cleanup, and complete removal passed;
- support owner and renewal budget accepted;
- exact limitations and unsupported states published; and
- all nine roles approve the exact frozen scope.

Promotion is scoped to named suite, revision, specifications, commands,
platforms, tools, environments, and date. It is not complete Ferris
conformance.

## Candidate repository roadmap

### Blueprint application companions

First proof:

- several declared Cargo workspaces;
- exact application-definition identity;
- local read-only plan, explanation, and graph;
- owner-native Cargo metadata and repository baselines;
- unknown and stale input controls; and
- removal with ordinary Cargo operation.

Deferred until separately authorized:

- affected-only selection;
- query, check, test, run, approval, mutation, deployment, or remote evidence.

### Profile companions

First proof:

- independent hosted, CLI, data, embedded, browser WASM, and native lanes;
- exact lock and target-active closure;
- compiler-floor observations;
- positive, expected-rejection, unsupported, and missing-prerequisite cases;
- renewal diff, expiry, rollback, and removal.

Do not combine the lanes into a universal Ferris stack. Use the measured shapes
in
[Rust compatibility-tested stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md)
as research evidence, not frozen future repository selections.

### AI-generated patch companions

First proof:

- frozen model and instruction identity;
- public-safe source and patch provenance;
- deterministic selected and full-reference scope;
- compiler, behavioral, and negative evidence;
- unsafe, FFI, dependency, macro, and build-script escalation;
- rejection, abstention, fallback, rollback, and removal;
- one seeded false omission in a held-out population.

No model may approve or execute the patch.

### Native-boundary companions

First proof:

- one C ABI or equivalent owner boundary;
- explicit ownership, allocation, panic, threading, layout, and version rules;
- generated-binding positive and negative tests;
- Windows and Unix native link, load, execute, and package stages;
- unsupported ABI and missing-tool cases;
- incremental migration, rollback, and uninstall.

### Platform-target companions

First proof:

- exact Windows, Unix, WASM, embedded, and native-tool lanes where
  representative;
- independent resolve, check, build, link, run, test, package, and deploy
  states;
- target, SDK, linker, runner, filesystem, and provider identity;
- native execution before support claims;
- unavailable and unsupported controls;
- servicing, expiry, rollback, and removal.

### Upstream-packet companions

First proof:

- owner discovery and current upstream home;
- minimized public-safe reproducer;
- exact source, toolchain, command, and output;
- bounded maintainer ask and expected response ownership;
- licensing and disclosure review;
- packet supersession and retirement;
- proof that submission-ready state does not post externally.

## Portability minimum

A promoted portfolio should include:

| Dimension | Minimum evidence |
|---|---|
| Host OS | Windows and at least one Unix platform |
| Paths/filesystems | separators, case, symlink, permissions, long path, read-only and full-disk controls as applicable |
| Processes | exit, timeout, cancellation, child cleanup, stream bounds |
| Toolchains | exact current binding plus supported skew and rejection cases |
| Targets | native execution for claimed support; cross results kept separate |
| Native tools | present, absent, wrong version, and unsupported cases |
| Output | machine, plain human, accessible, localized, paginated, and bounded behavior where applicable |
| Lifecycle | setup, interrupted setup, renewal, rollback, cleanup, complete removal |

## Evidence publication minimum

Public-safe promotion records should include:

- suite, case class, revision, and cutoff;
- governing specifications;
- source and toolchain identities safe to disclose;
- environment and platform scope;
- command and expected-output contract;
- record cardinality and aggregate result;
- public-output digests;
- limitations, unsupported states, expiry, and next renewal;
- role dispositions; and
- explicit statement that the result is scoped and not complete Ferris
  conformance.

## Final gate checklist

- [ ] Exact repositories and revisions are frozen.
- [ ] Development, calibration, and held-out populations are separate.
- [ ] Owner-native full-reference commands are executable.
- [ ] Positive and every required non-success family exist.
- [ ] Exact toolchain and environment identity is captured.
- [ ] Windows and Unix evidence is complete where applicable.
- [ ] Expected semantic outputs and allowed variance are reviewable.
- [ ] Harnesses, collectors, viewers, and scorers are qualified.
- [ ] Privacy, redaction, retention, deletion, and public-safe publication pass.
- [ ] Rollback, cleanup, ordinary operation, and complete removal pass.
- [ ] Maintenance, expiry, replacement, and retirement are funded.
- [ ] All nine roles approve the exact frozen boundary.
- [ ] A separately approved pulse supplies implementation authority.

Until every applicable item is complete, the companion remains guidance,
development evidence, calibration evidence, or a blocked candidate. It must
not claim conformance.
