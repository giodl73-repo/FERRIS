# Rust Ecosystem Intervention Decisions

Date: 2026-08-10
Status: Complete
Question: ECOS-Q12
Decision: close the Crates Series by adopting a product-neutral ecosystem
evidence model, advancing read-only ledger and profile comparison as a bounded
future proof candidate, contributing fixtures and diagnostics through existing
upstream owners, and rejecting a FERRIUM standard library, crate distribution,
certification program, universal score, or automatic dependency authority.
The Crates Series prerequisite for OSPREY specification work is satisfied.
Implementation remains unauthorized until the separate specification,
held-out workflow, validation, adoption, rollback, and implementation-pulse
gates pass.

## Decision supported

ECOS-Q12 decides what FERRIUM should do with the verified ecosystem gaps from
ECOS-Q01 through ECOS-Q11.

It does not:

- select one universal Rust application stack;
- approve, certify, install, upgrade, remove, fork, or replace dependencies;
- create a FERRIUM crate namespace, package registry, distribution, runtime,
  executor, TLS provider, database layer, native toolchain, or standard
  library;
- transfer maintenance or support obligations from current upstream owners;
- turn missing declarations, failed probes, or stale evidence into inferred
  success;
- authorize an upstream issue, comment, pull request, or stewardship takeover;
  or
- authorize OSPREY implementation.

## Research question

For each verified ecosystem gap, should FERRIUM document, adapt, standardize,
contribute upstream, steward, prototype, or defer?

The detailed mapping is recorded in
[EXP-01](ecos-q12-intervention-decision/results/EXP-01-intervention-matrix.md).

## Series synthesis

The Crates Series established that Rust's primary application-platform gap is
not broad absence of usable libraries. It is the absence of one renewable,
consumer-scoped evidence contract that joins:

- capability ownership and coverage class;
- foundational role without popularity-based approval;
- exact type, trait, version, feature, and interchange identity;
- operation-level runtime, cancellation, task, I/O, time, and context
  contracts;
- registry, source, review, succession, lifecycle, and successor evidence;
- archive, revision, publication, advisory, unsafe, build-script, macro,
  native, and license assurance;
- exact compiler, feature closure, host, target, provider, native tool, and
  validation-stage compatibility;
- duplicate/version/feature fragmentation, public exposure, cost, and
  remediation ownership;
- native source mode, discovery, ABI, generation, artifacts, deployment, and
  reproducibility;
- discovery provenance, candidate identity, evidence, eligibility, tradeoffs,
  decision, and renewal; and
- compatibility-profile identity, expiry, renewal, removal, substitution, and
  rollback.

No existing source owns this complete consumer decision. That does not make
FERRIUM the owner of the underlying crates or tools. The missing capability is
coordination and evidence preservation across owner boundaries.

## Intervention policy

FERRIUM uses seven intervention classes:

1. **Document:** explain an existing boundary or owner without inventing a new
   contract.
2. **Adapt:** bridge existing contracts for a named consumer while preserving
   direction, loss, ownership, and removal.
3. **Standardize contract:** define a product-neutral FERRIUM evidence schema
   for later OSPREY specifications; do not claim ecosystem-wide authority.
4. **Contribute upstream:** prepare evidence, fixtures, diagnostics, or a patch
   for the current owner after maintainer alignment.
5. **Stewardship support:** observe ownership and lifecycle changes, produce
   migration or contribution packets, and support current maintainers without
   taking control.
6. **Bounded prototype:** evaluate one read-only, removable capability behind
   a replaceable adapter and separately approved implementation gate.
7. **Reject or defer:** record why an intervention lacks a consumer, owner,
   validation contract, evidence, or acceptable scope.

These classes are dispositions, not action authority. Every contribution,
prototype, or stewardship action still requires a dedicated owner decision and
approved pulse.

## Adopt now

### Evidence contracts

Adopt the ECOS-Q01 through ECOS-Q11 evidence vocabulary as the Crate Ecosystem
Ledger and Ecosystem adapter requirements for later OSPREY specifications:

- capability, foundational role, and owner;
- exact source, package, release, feature, and closure identity;
- interchange, runtime, task, platform, provider, and native boundaries;
- stewardship, assurance, advisory, unsafe, build, macro, license, and
  lifecycle evidence;
- selection, eligibility, recommendation, profile, renewal, removal, and
  rollback records; and
- explicit fail, expected rejection, unsupported, unavailable,
  not-observed, stale, conflicting, and unknown states.

Owner: FERRIUM specification work.

Expected validation: FOREST-002, EVIDENCE-001, VALIDATION-001, and
CONFORMANCE-001 must encode positive, negative, failure, unsupported,
version-skew, expiry, renewal, removal, rollback, and projection-consistency
cases.

Non-goals: ecosystem-wide standards authority, package approval, or automatic
action.

### Existing-owner routing

Every record must identify the current owner:

- Rust language, standard library, target tiers, Cargo, rustc, rustup, and
  official tools remain Rust Project responsibilities;
- crates.io owns registry and distribution surfaces;
- RustSec, cargo-vet, cargo-deny, OSV, and policy tools retain their criteria;
- crate, runtime, provider, build-helper, native-library, generator, and
  binding maintainers retain their contracts;
- operating systems, SDKs, compilers, package managers, linkers, and deployment
  environments retain platform behavior; and
- consumer repositories own requirements, selection, exceptions, deployment,
  renewal, removal, and rollback.

FERRIUM owns only its evidence schema, research fixtures, source attribution,
and later adapter behavior.

### Documentation and decision packets

Adopt immutable, cited decision packets for:

- capability and foundational-role classification;
- exact-candidate selection;
- compatibility and profile observations;
- assurance and stewardship snapshots;
- dependency graph and native-boundary changes;
- upstream contribution proposals; and
- reject/defer decisions.

Each packet names evidence date, expiry, owner, limitations, alternatives,
validation, non-goals, and rollback where action is proposed.

## Adapt

Adapters remain appropriate only for named consumer boundaries:

- nominal types and traits across exact versions;
- logging, telemetry, service, async-I/O, executor, timer, and transport
  contracts;
- provider selection and native source modes;
- generated code and binding boundaries; and
- ecosystem evidence sources with different identity and freshness models.

Every adapter must record:

- direction;
- owning repository;
- allocation, copying, fallibility, field loss, ordering, backpressure,
  cancellation, panic, and runtime consequences;
- feature, target, provider, native, and compiler assumptions;
- positive and negative semantic tests;
- adoption and removal path; and
- expiry.

No generic FERRIUM adapter crate is authorized. A future adapter is justified
only by a named consumer that cannot solve the boundary through an existing
upstream contract.

## Contribute upstream

FERRIUM should prefer contribution packets over replacement implementations.
Candidate contribution classes are:

1. minimal positive and negative compatibility fixtures;
2. runtime-context, cancellation, task-lifecycle, shutdown, and I/O-adapter
   tests;
3. target/MSRV/feature/provider/native-tool diagnostics;
4. lock-versus-active-closure and graph-change explanations;
5. generated-code, build-script, native-artifact, and prerequisite provenance;
6. lifecycle, succession, seeking-maintainer, and successor documentation;
7. exact search/recommendation provenance and stale-evidence diagnostics; and
8. profile renewal, removal, and rollback examples.

An upstream contribution requires:

- a current owner and contribution path;
- reproduction on the owner's supported toolchain and platform;
- evidence that the owner considers the gap in scope;
- a minimal fixture or patch;
- no private data;
- licensing and maintenance agreement;
- explicit non-goals; and
- owner approval before any external post.

No specific issue or patch is authorized by ECOS-Q12.

## Stewardship support

FERRIUM may support stewardship by:

- preserving crates.io owner, TrustPub, release, yank, repository, archive,
  transfer, security-policy, lifecycle, and successor diffs;
- identifying missing or expired evidence;
- preparing contact or contribution packets;
- recording explicit seeking-maintainer or unmaintained declarations;
- comparing successor compatibility and migration evidence; and
- yielding decisions to current owners and consumers.

FERRIUM must not:

- rank maintainers;
- infer abandonment from age, activity, downloads, funding, owner count, or
  issue totals;
- declare a successor from name or fork similarity;
- publish an unmaintained conclusion without declaration or failed contact;
- assume maintenance of a foundational crate; or
- create a shadow governance authority.

## Bounded future proof candidate

The one capability that advances is a **read-only Crate Ecosystem Ledger and
renewable profile diff** for a named maintainer workflow:

> For this exact dependency or stack change, what capability, identity,
> feature, runtime, platform, assurance, stewardship, native, artifact,
> recommendation, and lifecycle evidence changed; what remains unknown; and
> what validation, removal, and rollback are required?

The first proof candidate must be:

- local and read-only;
- removable without changing Cargo behavior;
- based first on stable Cargo, registry, source, advisory, and repository
  evidence;
- isolated behind source-specific adapters;
- explicit about observed, declared, inferred, stale, and unknown assertions;
- unable to install, edit, update, approve, reject, publish, or deploy;
- limited to one accepted maintainer workflow;
- evaluated on held-out repositories and exact dependency changes; and
- created only after the remaining specifications and an approved
  implementation pulse.

### Required held-out gate

Before code:

1. freeze at least three public Rust repositories representing a host
   application, a cross-target or `no_std` case, and a native-bound case;
2. define Windows and Unix execution;
3. seed exact positive, negative, failure, unsupported, stale, and unknown
   cases;
4. compare raw-tool investigation with the proposed evidence record;
5. measure evidence collection time, storage, rate limits, false conclusions,
   omitted scope, maintainer investigation time, and renewal cost;
6. perform one dependency renewal and exact rollback;
7. demonstrate complete removal without correctness changes;
8. preserve privacy, source attribution, and retention boundaries; and
9. complete a new nine-role review.

This candidate advances to specification and held-out design, not
implementation.

## Reject or defer

### Permanent platform or distribution

Reject:

- a larger FERRIUM standard library;
- reimplementation of available ecosystem capabilities under a FERRIUM
  namespace;
- a global lockfile or curated crate distribution;
- one universal server, async, TLS, crypto, database, GUI, embedded, WASM, or
  observability stack; and
- a permanent FERRIUM-owned compatibility layer where upstream coordination is
  viable.

The evidence found mature implementations and fragmented ownership, not a
general absence that justifies a replacement platform.

### Scores, ranks, and certificates

Reject one:

- crate quality score;
- maintenance or bus-factor score;
- safety, soundness, trust, provenance, or compliance score;
- portability, MSRV, WASM, embedded, native-risk, fragmentation, or
  reproducibility score;
- universal candidate rank; or
- blessed, standard, certified, secure, safe, portable, or maintained label.

These dimensions have different owners, expiry, evidence, and consumer policy.

### Automatic dependency and environment authority

Reject automatic:

- manifest, lockfile, feature, provider, runtime, or source rewrites;
- dependency upgrades, downgrades, deduplication, replacement, or fork
  selection;
- compiler, linker, SDK, sysroot, package-manager, native-library, generator,
  Clang, libclang, protoc, runtime, or tool installation;
- advisory ignores, policy exceptions, legal conclusions, or certification;
- generated-binding refresh;
- system/bundled, static/dynamic, TLS, crypto, database, FIPS, or backend
  changes;
- validation deletion or success-shaped fallback; and
- upstream posting.

Observation and recommendation do not imply mutation authority.

### Premature crates and standards

Defer:

- a universal async abstraction layer;
- new shared traits without converged upstream ownership and representative
  consumers;
- FERRIUM executors, runtimes, providers, adapters, facades, package managers,
  registries, or assurance databases;
- stewardship takeover of existing crates; and
- ecosystem-wide metadata standards proposed without repeated held-out need
  and owner alignment.

## OSPREY disposition

The Crates Series Phase 0 gate is complete. It supplies the ecosystem concepts
required by:

- FOREST-002 canonical schema;
- EVIDENCE-001 evidence adapters and ownership;
- FOREST-003 maps, ledgers, and projection consistency;
- VALIDATION-001 validation and capability preservation;
- TRUST-001 provenance, privacy, retention, and deletion;
- FERRIS-001 decision and contribution packets; and
- CONFORMANCE-001 held-out and executable acceptance tests.

The next authorized work is architecture specification and planning-reference
examples. FOREST-001 remains Draft. No package, executable, service, repository
adapter, or production integration is authorized.

## Findings

### FERRIUM-678: the primary gap is evidence coordination, not library scarcity

**Sources:** ECOS-Q01 through ECOS-Q11 decision notes.

**Observed behavior:** Common capabilities usually had credible
implementations, while selection, composition, platform, assurance,
stewardship, native, and renewal evidence remained split across owners.

**Implication:** FERRIUM should coordinate evidence rather than recreate the
application platform.

**Confidence:** High.

### FERRIUM-679: intervention must preserve current owner boundaries

**Sources:** ECOS-Q02, ECOS-Q05, ECOS-Q09, and ECOS-Q11 ownership evidence.

**Observed behavior:** Registry authority, source custody, runtime contracts,
native tools, platform services, and consumer deployment had different owners.

**Implication:** Every record and proposal must name the current owner; FERRIUM
must not silently become the maintainer or authority.

**Confidence:** High.

### FERRIUM-680: the eleven evidence contracts should be adopted now

**Sources:** findings FERRIUM-498 through FERRIUM-677.

**Observed behavior:** Each question produced a distinct, tested identity or
lifecycle boundary needed by later decisions.

**Implication:** The complete vocabulary should constrain OSPREY schemas and
conformance tests.

**Confidence:** High.

### FERRIUM-681: FERRIUM standardizes its schema, not the ecosystem

**Sources:** engineering principles FP-05, FP-06, and FOREST-001.

**Observed behavior:** Existing projects already own language, registry,
runtime, crate, provider, advisory, and native-tool contracts.

**Implication:** Product-neutral canonical evidence is appropriate; unilateral
ecosystem standards are not.

**Confidence:** High.

### FERRIUM-682: adapters are directional consumer interventions

**Sources:** ECOS-Q03 and ECOS-Q04 adapter controls.

**Observed behavior:** Wrappers and adapters could satisfy nominal contracts
while losing fields, behavior, readiness, cancellation, or runtime semantics.

**Implication:** Adapter publication requires a named consumer, semantic tests,
owner, removal, and expiry.

**Confidence:** High.

### FERRIUM-683: upstream contribution requires owner alignment

**Sources:** FP-06, ECOS-Q03 through ECOS-Q11 recommendations, and the Rust
performance contribution precedent.

**Observed behavior:** Fixtures and diagnostics can reveal owner-specific gaps,
but no series result established authority to post or patch upstream.

**Implication:** FERRIUM should prepare contribution packets and wait for
maintainer alignment.

**Confidence:** High.

### FERRIUM-684: stewardship support is observation, not takeover

**Sources:** ECOS-Q05 findings FERRIUM-561 through FERRIUM-577.

**Observed behavior:** Ownership, activity, response, succession, lifecycle,
and successors required temporal evidence and contact.

**Implication:** FERRIUM may preserve diffs and migration evidence but must not
rank, declare, or assume maintenance.

**Confidence:** High.

### FERRIUM-685: the bounded candidate is a read-only ecosystem ledger

**Sources:** ECOS-Q08 through ECOS-Q11 prototype recommendations.

**Observed behavior:** Repeated work involved collecting and diffing exact
graph, platform, native, assurance, selection, and profile evidence.

**Implication:** One read-only ledger/profile-diff workflow merits held-out
design.

**Confidence:** Medium-high pending held-out consumer evaluation.

### FERRIUM-686: automation authority must remain narrower than evidence

**Sources:** FP-11, FOREST-001, and ECOS-Q06 through ECOS-Q11.

**Observed behavior:** Passing checks, audits, searches, and profiles did not
establish approval, safety, or mutation authority.

**Implication:** The first proof may observe and explain but not edit, install,
approve, publish, or deploy.

**Confidence:** High.

### FERRIUM-687: a distribution would create the lock-in Q11 prevented

**Sources:** ECOS-Q01 FERRIUM-510 through FERRIUM-512 and ECOS-Q11
FERRIUM-658 through FERRIUM-677.

**Observed behavior:** Exact profiles were useful only because they were
consumer-scoped, expiring, removable, and reversible.

**Implication:** Reject a permanent FERRIUM package set, global lockfile, or
recommended stack.

**Confidence:** High.

### FERRIUM-688: universal scores erase incompatible evidence classes

**Sources:** ECOS-Q05 through ECOS-Q10.

**Observed behavior:** Maintenance, security, platform, fragmentation, native,
and selection evidence used different sources, criteria, and expiry.

**Implication:** Preserve a tradeoff frontier and typed states rather than a
scalar score.

**Confidence:** High.

### FERRIUM-689: graph diagnostics do not authorize graph mutation

**Sources:** ECOS-Q08 FERRIUM-607 through FERRIUM-622.

**Observed behavior:** Some duplicate versions were necessary, feature
expansion could cost more than duplication, and remediation changed public
compatibility.

**Implication:** Reject automatic deduplication, feature removal, semver
widening, and lockfile rewriting.

**Confidence:** High.

### FERRIUM-690: native observations do not authorize environment mutation

**Sources:** ECOS-Q09 FERRIUM-623 through FERRIUM-639.

**Observed behavior:** Native success depended on exact compilers, discovery,
providers, generators, system state, and deployment assumptions.

**Implication:** Reject automatic installation, provider switching, binding
generation, or fallback.

**Confidence:** High.

### FERRIUM-691: new foundational crates require a material missing capability

**Sources:** ECOS-Q01 and ECOS-Q02.

**Observed behavior:** The census found mature implementations for most common
capabilities and classified the queue for verification, not replacement.

**Implication:** Defer new FERRIUM crates until a named consumer proves a gap
that documentation, adaptation, profile evidence, or upstream contribution
cannot close.

**Confidence:** High.

### FERRIUM-692: Crates Series completion satisfies one prerequisite only

**Sources:** OSPREY program, specification registry, and FOREST-001.

**Observed behavior:** Phase 0 required all twelve questions and final role
review, while implementation also requires specifications, conformance,
held-out workflows, adoption, rollback, and an approved pulse.

**Implication:** Close Phase 0 and open specification work without opening
implementation.

**Confidence:** High.

### FERRIUM-693: the next authorized work is specification and examples

**Sources:** OSPREY Phases 1 and 2 and the specification sequence.

**Observed behavior:** FOREST-002, EVIDENCE-001, VALIDATION-001, and
CONFORMANCE-001 directly depend on the completed Crates Series.

**Implication:** Translate the evidence model into canonical schema,
ownership, validation, and conformance examples before code.

**Confidence:** High.

### FERRIUM-694: held-out evaluation must test usefulness and removal

**Sources:** Q10 and Q11 prototype gates, FP-07, FP-12, and OSPREY Phase 3.

**Observed behavior:** Synthetic fixtures established boundaries but did not
measure maintainer investigation benefit, operational adoption, or held-out
removal.

**Implication:** A future proof requires public repositories, Windows and Unix,
seeded failure states, raw-tool baseline, renewal, rollback, and complete
removal.

**Confidence:** High.

### FERRIUM-695: all nine roles accept Phase 0 closeout with code withheld

**Sources:** the role review below.

**Observed behavior:** Every role accepted the ecosystem model, owner routing,
and read-only candidate while retaining implementation, validation, platform,
assurance, usability, and scope gates.

**Implication:** ECOS-Q12 and the Crates Series may move to Complete; OSPREY
remains architecture planning only.

**Confidence:** High.

## Final Crates Series role review

### Rust Safety Steward

**Disposition:** Accept Phase 0 closeout; implementation withheld.

Accepts explicit unsafe, generated, native, provider, runtime, cancellation,
advisory, and unknown boundaries. Rejects safe, sound, secure, or certified
labels from compilation, one audit, one profile, or one score. Requires
dedicated safety and semantic evidence in later conformance cases.

### Compiler Performance Engineer

**Disposition:** Accept the read-only candidate for held-out design.

Accepts separate lock and active closures, compile-time execution, artifact,
storage, and renewal costs. Requires evidence-ingestion overhead, cache state,
rate limits, representative graph changes, and maintainer workflow benefit to
be measured before implementation claims.

### Interop Boundary Auditor

**Disposition:** Accept the layered ecosystem model.

Accepts nominal identity, feature, runtime, provider, ABI, generated-code, and
adapter direction as first-class evidence. Requires positive and negative
semantic tests plus removal and rollback for every future adapter.

### AI Assurance Skeptic

**Disposition:** Accept observation and explanation only.

Accepts source attribution, assertion class, typed unknowns, immutable packets,
and human-owned decisions. Rejects inferred success, hidden fallback, automatic
approval, dependency mutation, installation, or external posting.

### Ecosystem Strategist

**Disposition:** Accept the coordination wedge; reject a competing platform.

Accepts an evidence model that composes Cargo, crates.io, RustSec, policy tools,
crate maintainers, native tools, and consumers without taking ownership.
Rejects a distribution, registry, certification brand, universal score, or
replacement foundational stack.

### Rust Maintainer

**Disposition:** Accept with simplicity and removability gates.

Accepts one bounded question, source-linked explanations, exact change records,
ordinary Cargo preservation, and consumer-owned approval. Requires concise
diagnostics, incremental adoption, complete removal, and no mandatory service
or workflow rewrite.

### Native Platform Adopter

**Disposition:** Accept specification work; reject operational adoption.

Accepts explicit host/target, compiler, SDK, provider, native source, generated
code, deployment, support, expiry, removal, and rollback evidence. Requires
Windows and Unix held-out execution plus native installation and recovery cost
before a proof advances.

### Scope Keeper

**Disposition:** Accept Crates Series completion.

Accepts the eleven evidence contracts, one read-only candidate, owner routing,
and explicit rejected interventions. Requires Phases 1 through 3 and a separate
implementation pulse; no code, package, service, repository adapter, or
production integration is opened by Q12.

### Validation Checker

**Disposition:** Accept Phase 0; conformance gate remains blocking.

Accepts twelve cited decisions, findings FERRIUM-498 through FERRIUM-695,
measured positive and negative fixtures, exact releases, profiles, renewal,
rollback, limitations, and final role dispositions. Requires FOREST-002,
EVIDENCE-001, VALIDATION-001, and CONFORMANCE-001 to define executable positive,
negative, failure, unsupported, version-skew, stale, renewal, removal,
rollback, and packet-completeness tests before implementation.

## Series completion

The Crates Series completion gate is satisfied:

1. ECOS-Q01 through ECOS-Q12 have cited decision notes.
2. Exact release, revision, feature, closure, owner, license, maintenance, and
   provenance evidence was recorded where crates were evaluated.
3. Server, CLI, data, embedded `no_std`, WASM, and native-integration profiles
   were tested with explicit stage limitations.
4. Interchange, async, security, platform, feature, version, and native
   boundaries have positive and negative controls.
5. The intervention matrix names owner, disposition, validation, and non-goals.
6. Profiles have renewal, removal, substitution, expiry, and rollback rules.
7. No crate or stack is declared standard, blessed, certified, safe, secure,
   maintained, or portable.
8. The final synthesis defines the OSPREY ecosystem model.
9. All nine roles accept Phase 0 closeout with implementation withheld.

## Limitations

- The series evaluated representative capabilities and exact releases, not the
  complete crates.io ecosystem.
- Most execution evidence came from one Windows x86-64 host; Linux, browser,
  embedded hardware, and native deployment evidence remains bounded.
- No held-out maintainer workflow measured the proposed ledger's usefulness.
- No upstream maintainer has accepted a Q12 contribution proposal.
- No ecosystem schema has been reviewed outside FERRIUM.
- Stewardship, advisory, search, owner, release, and compatibility evidence
  expires.
- No implementation cost, service topology, UI, storage format, or operating
  model has been selected.

## Sources

Crates Series decisions:

- [ECOS-Q01 capability coverage](2026-08-09-rust-capability-coverage.md)
- [ECOS-Q02 foundational crate census](2026-08-09-rust-foundational-crate-census.md)
- [ECOS-Q03 interchange contracts](2026-08-09-rust-interchange-contracts.md)
- [ECOS-Q04 async portability](2026-08-09-rust-async-portability.md)
- [ECOS-Q05 maintenance and stewardship](2026-08-09-rust-maintenance-stewardship.md)
- [ECOS-Q06 security and provenance](2026-08-09-rust-security-provenance.md)
- [ECOS-Q07 platform compatibility](2026-08-09-rust-platform-compatibility.md)
- [ECOS-Q08 feature and version fragmentation](2026-08-09-rust-feature-version-fragmentation.md)
- [ECOS-Q09 native dependency boundary](2026-08-10-rust-native-dependency-boundary.md)
- [ECOS-Q10 crate discovery and selection](2026-08-10-rust-crate-discovery-selection.md)
- [ECOS-Q11 compatibility-tested stack profiles](2026-08-10-rust-compatibility-stack-profiles.md)

Program and governance:

- [FERRIUM engineering principles](../governance/ENGINEERING_PRINCIPLES.md)
- [Crates Series research program](../plans/ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md)
- [OSPREY program](../plans/OSPREY_PROGRAM.md)
- [Query Forest component model](../specs/FOREST_COMPONENT_MODEL.md)
- [FERRIUM specification registry](../specs/README.md)
