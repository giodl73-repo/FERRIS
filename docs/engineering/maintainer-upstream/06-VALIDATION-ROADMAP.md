# Validation Roadmap

Status: Guidance
Implementation authority: None

## Goal

Validation must prove that a maintainer or consumer can use Ferris-prepared
evidence with less investigation cost and no authority takeover, hidden
workflow dependency, private-data leak, correctness regression, or removal
trap. This roadmap does not authorize a product implementation or external
submission.

The roadmap applies the measurable gates in
[CONTEXT.md](../../../CONTEXT.md), the conformance responsibilities in the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md),
the packet promotion gate in
[RUST_PERFORMANCE_CONTRIBUTION_PACKET.md](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md),
and the held-out direction in
[ECOS-Q12](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Stage 0: Documentation integrity

Acceptance:

- all seven guides contain `Status: Guidance` and
  `Implementation authority: None`;
- all repository-relative links resolve;
- code fences, tables, and headings are structurally valid;
- files contain ASCII only;
- no file outside `docs/engineering/maintainer-upstream/` changes;
- `git diff --check` passes.

## Stage 1: Packet completeness fixtures

Create local synthetic packet fixtures for:

- rustc diagnostic or regression classification;
- Cargo accepted-issue test or benchmark;
- rustc-perf compile benchmark;
- crate positive and negative semantic fixture;
- standards documentation or proposal mapping;
- stewardship support request;
- adapter reject/defer decision.

Each fixture must exercise every identity, evidence, approval, burden,
lifecycle, rollback, and removal field. Include deliberately incomplete,
conflicting, stale, private, unlicensed, unsupported, and wrong-owner cases.

Acceptance:

- 100 percent of required packet fields are machine- or reviewer-detectable;
- every invalid fixture fails for the intended reason;
- no invalid fixture can reach Submission-ready;
- status transitions are auditable and non-skipping;
- failures remain visible.

## Stage 2: Minimal reproducer challenge

For each owner class, start with a larger public or synthetic case and have an
independent engineer minimize it.

Measure:

- elapsed minimization time;
- source, dependency, and command reduction;
- positive-control retention;
- negative and correctness-control retention;
- rejected reductions;
- clean-environment rerun success;
- reviewer setup and comprehension time.

Acceptance:

- every minimized fixture preserves its distinguishing mechanism;
- every reduction reruns required controls;
- a reviewer can reproduce from packet commands without Ferris-specific
  knowledge;
- no private or unlicensed input remains.

## Stage 3: Owner-specific conformance

### rustc

- exact toolchain and source identity;
- diagnostic, behavior, incremental, unsafe, ABI, macro, and target controls
  as applicable;
- owner-directed test placement and patch shape.

### Cargo

- accepted issue or explicit owner-interest evidence;
- Cargo-native test or Criterion form;
- filesystem, resolver, fingerprint, cache, and platform controls;
- separation of Cargo, rustc, link, and harness cost.

### rustc-perf

- upstream profile, scenario, and metric vocabulary;
- benchmark manifest, `perf-config.json`, patch files, registration, README,
  `REUSE.toml`, and `Cargo.lock` as applicable;
- documented local timing and `collector bench_local` comparison;
- no unauthorized official perf run.

### Crates

- repository-native contribution form;
- MSRV, feature, target, runtime, unsafe, native, and release-policy evidence;
- positive and negative semantic tests;
- explicit long-term owner.

### Standards

- repeated product-neutral need;
- representative consumers;
- descriptive versus normative distinction;
- projection loss and compatibility limits;
- current standards process.

Acceptance:

- all owner-required local checks pass;
- unsupported cases are explicit;
- packet vocabulary maps to owner vocabulary;
- ordinary owner commands remain primary.

## Stage 4: Public-safety and approval exercise

Seed fixtures with internal paths, usernames, tenant identifiers, private
source fragments, tokens, ambiguous licenses, and third-party notices.

Acceptance:

- all reusable secrets and prohibited private data are detected or blocked;
- license and provenance are complete;
- redaction preserves the distinguishing behavior;
- Submission-ready requires recorded organizational approval;
- no external artifact can be created from a non-approved fixture.

The exercise validates process controls only; it must not actually post
externally.

## Stage 5: Maintainer burden study

Use at least one current-owner collaborator for a bounded, approved local
review. Compare raw evidence with the packet.

Measure:

- setup and reproduction time;
- time to identify the owner decision;
- number of clarification turns;
- patch or fixture review size;
- CI and platform minutes;
- flake and noise rate;
- recurring renewal and triage estimate;
- owner-rated usefulness and burden;
- redirect, decline, external, accepted, or retired disposition.

Acceptance thresholds must be frozen before the study. At minimum, the packet
must not increase median investigation time, must expose all known limitations,
and must produce a clear decision without requiring Ferris internals.

## Stage 6: Stewardship support exercise

Test a renewable support record without changing governance.

Acceptance:

- registry, publication, source, work, responsiveness, concentration,
  succession, lifecycle, replacement, and support evidence remain separate;
- owner and policy changes appear as dated diffs;
- quiet or concentrated activity does not become an abandonment verdict;
- support terms name decision rights, response, funding, renewal, succession,
  and exit;
- current maintainers retain roadmap, review, release, and publication
  authority.

## Stage 7: Adoption, rollback, and removal

Freeze at least three public or synthetic repositories representing:

- a host application;
- a cross-target or `no_std` case;
- a native-bound case.

Run on Windows and Unix. Seed positive, negative, failure, unsupported, stale,
unknown, version-skew, privacy, and owner-routing cases.

Measure:

- baseline raw-tool investigation;
- adoption time and files changed;
- evidence collection time, storage, and rate limits;
- false conclusions and omitted scope;
- renewal cost;
- exact rollback time and result;
- complete removal time and residual state;
- correctness and workflow equivalence after removal.

Acceptance:

- one dependency or evidence renewal succeeds;
- exact rollback succeeds without the adopted capability;
- removal leaves ordinary Cargo and owner workflows functional;
- no hidden service, manifest, resolver, credential, or governance dependency
  remains;
- consumer correctness does not change after removal.

## Stage 8: Fork and adapter decision tests

Exercise decision records, not implementations.

Acceptance:

- common weak signals never justify a fork;
- adapter proposals fail without a named consumer, semantic tests, expiry, and
  removal;
- fork proposals fail without contact evidence or lifecycle declaration,
  material need, license and governance plans, compatibility, migration,
  funding, succession, rollback, and retirement;
- network lineage alone never produces successor status;
- direct upstream contribution and stewardship support are evaluated first.

## Stage 9: Nine-role review and stop gate

Record a disposition from every repository role:

- safety guarantees and limits;
- representative performance and burden;
- interop and native semantics;
- evidence versus model assertion;
- contribute-versus-duplicate decision;
- maintainer simplicity and ordinary workflow;
- platform operations, rollback, and audit;
- bounded scope and visible non-goals;
- reproducible commands, fixtures, failures, and actual results.

Implementation remains blocked unless a later record also provides complete
specifications, measurable acceptance and stop criteria, adoption and support
plans, rollback and removal proof, and a separately approved implementation
pulse.

## Program-level success measures

Track:

- percentage of packets correctly routed on first contact;
- reproduction and minimization success rate;
- median maintainer setup and investigation time;
- clarification turns and review burden;
- public-safety and license gate failures caught before approval;
- accepted tests, benchmarks, diagnostics, documents, decisions, and patches;
- artifacts still maintained after renewal;
- response, supersession, external, decline, and retirement rates;
- funded maintainer and review time;
- rollback and removal success;
- zero unauthorized posts, leaked secrets, shadow ownership claims, or
  permanent downstream forks.

Success is maintained owner value with preserved authority, not contribution
volume.
