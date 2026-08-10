# EXP-01: Crates Series Intervention Matrix

Date: 2026-08-10
Question: ECOS-Q12
Decision input: assign an owner, intervention class, validation gate, and
non-goals to the verified ecosystem boundaries from ECOS-Q01 through
ECOS-Q11.

## Method

The matrix reviewed:

- decision, adopt-now, prototype, reject/defer, findings, limitations, and role
  sections from all eleven completed ECOS notes;
- findings FERRIUM-498 through FERRIUM-677;
- the Crates Series completion gate;
- FERRIUM engineering principles FP-01 through FP-12;
- the OSPREY phase sequence;
- FOREST-001 ownership, evidence, action, removal, and conformance boundaries;
  and
- the FERRIUM specification registry.

Each intervention records:

```text
intervention ID
source questions and finding range
verified gap or boundary
current owner
FERRIUM disposition
required validation
non-goals
```

Disposition vocabulary:

```text
document
adapt
standardize contract
contribute upstream
stewardship support
bounded prototype
reject or defer
```

## Decision matrix

### Adopted evidence and contract interventions

| ID | Sources | Verified boundary | Current owner | Disposition | Required validation | Non-goals |
|---|---|---|---|---|---|---|
| INT-01 | Q01; FERRIUM-498–512 | capability is split among language/`std`, official tools, ecosystem, fragmented contracts, and material gaps | Rust Project, crate owners, consumers | standardize contract | canonical capability/owner/provider/data/platform fields and typed unknown cases | no larger FERRIUM standard library |
| INT-02 | Q02; FERRIUM-513–526 | foundational status is a structural role and replacement consequence, not popularity or approval | crate families and consumers | document | retain exact release, role, closure, owner, license, MSRV, and verification disposition | no adoption from census rank |
| INT-03 | Q03; FERRIUM-527–544 | nominal types/traits, versions, coherence, features, serialization, and runtime semantics form layered interchange | crate contract owners | standardize contract | positive/negative exact-version fixtures, public exposure, adapter direction/loss, semantic tests | no structural typing or silent serialization bridge |
| INT-04 | Q04; FERRIUM-545–560 | async portability is operation-level across executor, spawn, I/O, time, cancellation, blocking, context, lifecycle, and platform | runtime, I/O, application-library, and embedded owners | standardize contract | runtime-context failures, cancellation/shutdown, task ownership, I/O behavior, embedded/host cases | no FERRIUM runtime or universal async abstraction |
| INT-05 | Q05; FERRIUM-561–577 | stewardship needs renewable registry, source, work, response, succession, lifecycle, and successor evidence | publishers, repository maintainers, security contacts, consumers | stewardship support | temporal diffs, path-scoped work, contact evidence, explicit lifecycle and successor state | no maintainer score or abandonment inference |
| INT-06 | Q06; FERRIUM-578–591 | assurance joins archive, source, publication, advisory, closure, build, macro, unsafe, native, audit, and license evidence | crates.io, RustSec, policy tools, maintainers, consumers | standardize contract | exact identity joins, target-active closure, dated advisories, host execution inventory, license states | no safety, soundness, trust, or legal certificate |
| INT-07 | Q07; FERRIUM-592–606 | compatibility belongs to exact feature closure, compiler, host/target, provider, tools, and observed validation stage | Rust target/tool owners, crate maintainers, consumers | standardize contract | package-root and consumer cases, check/link/run/test/deploy split, negative/unknown states, expiry | no portable/MSRV/WASM/embedded score |
| INT-08 | Q08; FERRIUM-607–622 | duplicate versions, feature expansion, public exposure, cost, and remediation ownership are distinct | Cargo resolver, crate authors, dependency owners | standardize contract | target/dependency-kind graphs, public contract exposure, compile/artifact cost, before/after semantics | no automatic deduplication or feature rewriting |
| INT-09 | Q09; FERRIUM-623–639 | native integration is an execution/artifact chain across source mode, tools, discovery, ABI, generation, providers, artifacts, assurance, and deployment | crate, native-library, toolchain, OS, package, and deployment owners | standardize contract | multi-platform positive/negative tool, link, run, generated, artifact, deployment, and rollback cases | no installation, provider switching, ABI/FIPS certification, or hidden fallback |
| INT-10 | Q10; FERRIUM-640–657 | discovery generates candidates; exact consumer evidence determines eligibility and tradeoffs | Cargo, crates.io, curation services, evidence owners, consumers | standardize contract | held-out queries/candidates, evidence coverage, false exclusion, stale decisions, explanation accuracy | no universal rank or automatic approval |
| INT-11 | Q11; FERRIUM-658–677 | profiles are expiring consumer contracts over exact selection, closure, compiler, targets, stages, assurance, renewal, removal, and rollback | consumer repos with FERRIUM evidence methodology | standardize contract | six profile shapes, renewal/rollback, held-out adoption/removal, cross-platform execution | no distribution, global lockfile, or certificate |

### Adaptation and upstream contribution interventions

| ID | Sources | Verified boundary | Current owner | Disposition | Required validation | Non-goals |
|---|---|---|---|---|---|---|
| INT-12 | Q03–Q04 | adapters can bridge nominal I/O, logging, telemetry, service, executor, timer, or transport contracts but may lose semantics | existing adapter and contract owners | adapt | direction, field/behavior loss, readiness, backpressure, cancellation, panic, allocation, performance, removal | no generic FERRIUM adapter crate |
| INT-13 | Q04 | task drop, cancellation, blocking work, shutdown, and context failures need explicit diagnostics and fixtures | runtime/application-library owners | contribute upstream | reproduce on owner-supported versions; minimal positive/negative fixture; maintainer alignment | no external post or universal runtime policy from Q12 |
| INT-14 | Q07–Q09 | target, provider, native prerequisite, generator, and binding failures need actionable source-owned diagnostics | Cargo, build-helper, provider, generator, and native-tool owners | contribute upstream | current release reproduction on Windows and Unix; exact missing prerequisite and owner path | no auto-install or silent fallback |
| INT-15 | Q05–Q06 | lifecycle, succession, TrustPub, source identity, advisory, and policy evidence is fragmented | crates.io, RustSec, repository and policy-tool owners | contribute upstream | repeated held-out need, source/API stability, privacy/rate-limit review, owner acceptance | no unilateral ecosystem metadata standard |
| INT-16 | Q08–Q11 | exact graph, selection, and profile diffs can support maintainer decisions | Cargo and consumer repositories | contribute upstream | owner-specific diagnostic gap, minimal fixture, measured benefit, rollback, no hidden policy | no automatic graph mutation |

### Bounded prototype and specification interventions

| ID | Sources | Verified boundary | Current owner | Disposition | Required validation | Non-goals |
|---|---|---|---|---|---|---|
| INT-17 | Q08–Q11 | maintainers repeatedly need one joined view of graph, platform, native, assurance, selection, and profile changes | FERRIUM schema; source systems retain data | bounded prototype | FOREST-002, EVIDENCE-001, VALIDATION-001, CONFORMANCE-001, three held-out repos, Windows/Unix, renewal/removal/rollback | no code before approved pulse; read-only only |
| INT-18 | all questions | the Crates Series vocabulary must become canonical schema and conformance examples before implementation | FERRIUM specification process | standardize contract | schema serialization, projection consistency, source attribution, version skew, expiry, unknown, action separation | no monolithic service or implementation-by-specification |

### Rejected or deferred interventions

| ID | Sources | Rejected/deferred proposal | Reason | Reconsideration gate |
|---|---|---|---|---|
| INT-19 | Q01–Q02 | larger FERRIUM standard library or foundational crate namespace | mature implementations exist; verified gap is coordination and evidence | named consumer proves a material missing capability that upstream/adaptation cannot close |
| INT-20 | Q01, Q10, Q11 | curated distribution, global lockfile, recommended stack, or certification | collapses consumer requirements and creates renewal/lock-in ownership | published renewable criteria, owner, removal, rollback, and external governance review; still not implied |
| INT-21 | Q05–Q10 | universal health, safety, quality, trust, maintenance, portability, fragmentation, native-risk, reproducibility, or popularity score | dimensions have different criteria, owners, evidence, and expiry | no current reconsideration; retain typed records and tradeoffs |
| INT-22 | Q03–Q04 | universal async abstraction, trait convergence, or runtime-neutral facade | contracts differ by operation and consumer; adapters can be lossy | representative consumers, owner convergence, semantic/performance controls |
| INT-23 | Q08–Q11 | automatic manifest/lock/feature/provider/runtime/dependency rewrites | graph changes can alter identity, semantics, safety, platform, and operations | independently approved action contract with exact validation and rollback; outside first proof |
| INT-24 | Q07–Q09 | automatic compiler, SDK, sysroot, native package, generator, runtime, Clang/libclang, or protoc installation | mutates host trust and deployment state; prerequisites are owner-specific | explicit consumer-approved execution plan; outside first proof |
| INT-25 | Q05 | stewardship takeover or automatic successor promotion | publication, maintenance, succession, and compatibility require owner/contact evidence | explicit upstream request, governance, funding, maintenance, migration, and exit plan |
| INT-26 | all questions | OSPREY implementation immediately after Q12 | Phase 0 is only one prerequisite; schema, conformance, held-out workflow, adoption, rollback, and approved pulse remain | complete Phases 1–3 and approve Phase 4 pulse |

## OSPREY schema consequences

The completed series requires later specifications to represent at least:

### Identity

- capability and coverage class;
- package, source, release, checksum, revision, feature, lock, and target-active
  closure;
- facade, core, construction, platform, build, and substrate roles;
- public type, trait, serialized, wire, provider, native, generated, artifact,
  and deployment identity; and
- profile, evidence snapshot, expiry, renewal, removal, and rollback identity.

### Ownership

- language/tool, registry, package, repository, publisher, reviewer, security
  contact, runtime, provider, native library, compiler, generator, platform,
  deployment, consumer, evidence, and action owners;
- transfer, succession, fork, successor, and supersession lineage; and
- missing owner or conflicting authority.

### State

- declared, inferred, observed, predicted, recommended, approved, executed, and
  yielded assertion classes;
- pass, fail, expected rejection, unsupported, unavailable infrastructure,
  not observed, stale, conflicting, unknown, deferred, and rejected;
- required, removable, migratory, adapter-owned, provider-owned, upstream,
  consumer-owned, and no-action dispositions; and
- evidence and decision expiry.

### Validation

- resolve, metadata, check, build, link, execute, test, package, deploy,
  operate, renew, remove, and rollback stages;
- positive, negative, failure, unsupported, version-skew, stale, and unknown
  cases;
- semantic, cancellation, shutdown, ABI, generated-code, native, artifact,
  security, license, and operational evidence; and
- omitted scope and fallback.

### Action separation

- observation and source acquisition;
- candidate generation;
- evidence join;
- eligibility filter;
- tradeoff presentation;
- consumer decision;
- approved action;
- validation and rollback; and
- external contribution or stewardship handoff.

No stage inherits authority from an earlier stage.

## Held-out acceptance outline

The read-only candidate remains blocked until a future plan freezes:

| Dimension | Minimum |
|---|---|
| repositories | three public repos: host app, cross-target/`no_std`, native-bound |
| platforms | Windows and Unix execution |
| changes | direct update, transitive update, feature change, duplicate/version change, provider/native change |
| failure controls | unsupported target, missing tool, stale advisory/owner evidence, adapter semantic loss, runtime-context failure |
| baseline | raw Cargo/registry/source/advisory investigation |
| measures | correct conclusion, unknown preservation, investigation time, collection time, storage, rate limits, renewal cost |
| lifecycle | adoption, exact rollback, complete removal, evidence expiry |
| governance | privacy, retention, current owner, contribution route, nine-role review |

## Final disposition

| Category | Decision |
|---|---|
| document | capability, foundational role, owner, boundary, unsupported state, and reject/defer explanations |
| adapt | only named directional consumer boundaries with semantic and removal tests |
| standardize contract | the product-neutral Crate Ecosystem Ledger and adapter evidence vocabulary |
| contribute upstream | owner-aligned fixtures, diagnostics, provenance, lifecycle, and compatibility evidence |
| stewardship support | renewable diffs, contact/contribution packets, successor/migration evidence |
| bounded prototype | read-only ecosystem ledger/profile diff after specifications and held-out design |
| reject/defer | distribution, certification, universal scores, automatic mutation/installation, new foundational crates without a proven material gap |

The intervention matrix covers findings FERRIUM-498 through FERRIUM-677 and
supports the Q12 closeout findings FERRIUM-678 through FERRIUM-695.
