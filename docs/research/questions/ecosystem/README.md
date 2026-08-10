# Crates Series Research Questions

## Purpose

The Crates Series decomposes Rust ecosystem and application-platform research
into twelve independently reviewable questions. It follows PERF-Q01 through
PERF-Q36 and precedes OSPREY implementation.

Findings continue the global `FERRIUM-XX` sequence. `ECOS-Qxx` identifies a
question, not a finding.

## Registry

| ID | Area | Question | Depends on | Primary path |
|---|---|---|---|---|
| ECOS-Q01 | Coverage | Which common capabilities are in Rust `std`, the official toolchain, external crates, or materially absent? | None | Map |
| ECOS-Q02 | Foundations | Which crates function as ecosystem infrastructure? | Q01 | Map/verify |
| ECOS-Q03 | Interchange | Where do types, traits, errors, and features prevent composition? | Q01, Q02 | Verify |
| ECOS-Q04 | Async | Which runtime and cancellation assumptions leak across boundaries? | Q02, Q03 | Verify |
| ECOS-Q05 | Stewardship | How can users detect concentration, transfer, forks, and abandonment? | Q02 | Verify |
| ECOS-Q06 | Security | What evidence covers advisories, integrity, unsafe, macros, build scripts, native code, and licenses? | Q02, Q05 | Verify |
| ECOS-Q07 | Platforms | How consistently do foundational crates support MSRV, OSes, architectures, `no_std`, WASM, embedded, and cross-compilation? | Q02, Q06 | Verify |
| ECOS-Q08 | Fragmentation | What cost and incompatibility comes from feature policies and simultaneous versions? | Q02, Q03, Q07 | Compare |
| ECOS-Q09 | Native boundary | Where do system libraries, providers, code generation, and build scripts reduce portability? | Q03, Q06, Q07 | Verify |
| ECOS-Q10 | Discovery | Can evidence improve selection beyond search and popularity? | Q01 through Q09 | Compare |
| ECOS-Q11 | Stack profiles | Can renewable application-stack profiles be compatibility tested without lock-in? | Q03 through Q10 | Compare/prototype |
| ECOS-Q12 | Intervention | Which verified gaps should be documented, adapted, standardized, contributed, stewarded, prototyped, or deferred? | Q01 through Q11 | Decide |

## Execution order

1. Map capability coverage and candidate foundational crates.
2. Verify interchange, async, stewardship, security, platform, and native
   boundaries.
3. Measure feature and version fragmentation.
4. evaluate evidence-backed discovery and compatibility-tested profiles.
5. Make one intervention decision for every verified gap.

Questions may overlap in evidence collection, but each receives a separate
decision and nine-role review.

## Shared requirements

Every question must:

1. follow the FERRIUM research skill;
2. record exact crate versions, revisions, owners, licenses, and release dates
   whenever evaluating an individual crate;
3. inspect complete dependency, feature, macro, build-script, native, and
   unsafe closures when making assurance claims;
4. distinguish documented support from measured support;
5. treat popularity as a signal rather than proof;
6. name current owners and upstream paths;
7. preserve unsupported and unknown cases;
8. identify implications for the OSPREY Ecosystem adapter and Crate Ecosystem
   Ledger; and
9. complete all nine role reviews before moving to Complete.

## Status

ECOS-Q01 through ECOS-Q09 are complete.

- Q01 established the five-class capability taxonomy.
- Q02 selected nineteen exact releases as a verification queue using contract,
  construction, platform, build, and implementation-substrate roles.
- Q03 established layered interchange evidence and measured identity,
  coherence, error, adapter, and feature boundaries.
- Q04 established operation-level runtime contracts and measured executor,
  context, I/O, spawn, task-lifecycle, and Send boundaries.
- Q05 established renewable stewardship evidence across registry, release,
  source, work, response, succession, lifecycle, and successor dimensions.
- Q06 established joined assurance evidence across archive, source,
  publication, advisory, closure, build, macro, unsafe, native, audit, and
  license boundaries without a universal safety score.
- Q07 established renewable compatibility evidence across exact feature
  closure, Cargo/rustc pair, host/target pair, target tier, library and
  architecture capability, provider, native prerequisites, and independently
  observed check, link, execution, and test stages.
- Q08 established typed fragmentation evidence across requesting requirements,
  duplicate and shared closure, public exposure, effective features, resolver
  scope, compiler and artifact cost, interchange consequences, remediation
  ownership, and renewal without automatic graph rewrites.
- Q09 established renewable native-boundary evidence across source mode,
  provider, host and target tools, discovery, ABI, generated code and bindings,
  Cargo directives, native component identity, artifacts, assurance,
  reproducibility, and deployment without automatic installation or provider
  switching.

ECOS-Q10 through ECOS-Q12 are planned.
