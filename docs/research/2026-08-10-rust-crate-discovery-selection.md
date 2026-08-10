# Rust Crate Discovery and Selection

Date: 2026-08-10
Status: Complete
Question: ECOS-Q10
Decision: adopt an evidence-backed discovery and selection record that
separates consumer intent, retrieval source, candidate identity, evidence,
eligibility, tradeoffs, decision, and renewal. Search rank, downloads,
curation, repository popularity, and composite scores may generate candidates;
none may approve a crate or stack without an explicit consumer contract.

## Decision supported

ECOS-Q10 determines whether an evidence-backed capability map is a defensible
FERRIUM capability.

It does not:

- recommend one universal Rust crate stack;
- certify any crate as safe, secure, maintained, portable, or correct;
- replace crates.io, Cargo, docs.rs, RustSec, cargo-vet, cargo-deny, lib.rs, or
  community curation;
- turn downloads, stars, activity, age, or one composite score into approval;
- infer an absent MSRV, platform, provider, maintenance, or assurance contract;
  or
- authorize OSPREY implementation.

## Research question

Can evidence improve crate selection beyond keyword search, popularity,
anecdote, and stale recommendation lists?

The decision requires two distinct answers:

1. whether current discovery surfaces retrieve a stable and relevant candidate
   set; and
2. whether exact, consumer-scoped evidence changes which candidates remain
   eligible.

## Evidence model

A renewable selection record needs eight layers:

1. **Consumer intent:** capability, operation, semantics, platform, toolchain,
   runtime, provider, policy, integration, operations, and non-goals.
2. **Retrieval:** exact query, source, source revision or observation time,
   filters, sort, page, rank, and result limit.
3. **Candidate identity:** package, exact release, source, checksum, VCS
   revision, facade/contract family, role, and replacement boundary.
4. **Evidence:** documentation, feature and dependency closure, interchange,
   async, stewardship, assurance, platform, fragmentation, native, license,
   performance, artifact, and operational observations.
5. **Eligibility:** each requirement is pass, fail, unsupported, not observed,
   stale, conflicting, or unknown with a source and expiry.
6. **Tradeoff frontier:** candidates that satisfy mandatory requirements retain
   separate costs and benefits rather than a hidden weighted score.
7. **Decision:** selected, conditionally selected, held for comparison,
   rejected, or deferred with owner, rationale, alternatives, rollback, and
   non-goals.
8. **Renewal:** mutable evidence, source rankings, exact releases, advisories,
   owners, targets, and consumer requirements are re-observed and diffed.

Discovery broadens recall. Evidence narrows eligibility. Human or policy-owned
requirements decide among the remaining tradeoffs.

## Measured controls

The detailed commands and output are recorded in
[EXP-01](ecos-q10-discovery-selection/results/EXP-01-discovery-selection-matrix.md).

### Official search matrix

Eight capability queries were observed through the crates.io API under
relevance, all-time downloads, recent downloads, recent updates, and
alphabetical/name ordering. Each cell retained the first ten results:

```text
8 queries x 5 sorts x 10 results = 400 observations
```

The queries covered JSON, HTTP clients, async runtimes, command-line parsers,
`no_std` logging, TLS, SQLite, and date/time.

| Observation | Result |
|---|---:|
| matching crates reported per query | 3,652 to 38,981 |
| Cargo/API relevance top-ten agreement | 8 of 8 queries; Jaccard 1.000 |
| mean relevance/all-download top-ten Jaccard | 0.030 |
| mean relevance/recent-download top-ten Jaccard | 0.030 |
| mean relevance/recent-update top-ten Jaccard | 0.000 |
| mean relevance/alphabetical top-ten Jaccard | 0.020 |
| relevance results older than two years | 20 of 80 |
| exact query-name packages ranked first | 3 of 8 |

Cargo source documents textual search and a ten-result default. Current
crates.io source bounds relevance to 1,000 candidates, ranks exact-name matches
first, then full-text rank, then recent downloads for ties. The crates.io team
also documents that the relevance candidate set is first bounded to the 1,000
matches with the highest recent downloads.

An expected-failure request for relevance page 101 returned HTTP 400:

```text
Cannot page beyond the first 1000 results when sorting by relevance.
```

The same page under all-time-download sorting returned HTTP 200.

### Known-candidate visibility

The five top-ten lists did not expose several established or curated
alternatives:

- `async-std` and `smol` for `async runtime`;
- `lexopt` and `pico-args` for `command line parser`;
- `defmt` for `no_std logging`;
- `native-tls` and `openssl` for `tls`; and
- `jiff` for `date time`.

Conversely, all-time or recent downloads often promoted a neighboring
foundation rather than the requested application role:

- `serde` for JSON;
- `hyper` for an HTTP client;
- `mio` for an async runtime;
- `hashbrown` for `no_std` logging;
- `libsqlite3-sys` for SQLite access; and
- `regex` for date/time.

These packages may be appropriate dependencies or foundations. Their ranks do
not establish that they satisfy the consumer intent.

### Curation and composite ranking

Blessed.rs describes itself as a hand-curated community guide. Its current
command-line section distinguishes:

- `clap` as fully featured; and
- `lexopt` and `pico-args` as minimal alternatives.

The observed data file was commit
`c750a3d44011465b4d4c7a811e7752c4a63f0415`, dated 2026-08-03. It recorded
crate names and rationale but no exact version fields. Git history can version
the curation source when pinned; the live site remains a moving recommendation
surface.

Lib.rs describes an unofficial, semi-curated ranking that combines popularity,
reverse dependencies, trends, documentation, release history, tests,
metadata, authors, dependency weight, maintenance inferences, maintainer
reputation, special cases, and blocklists. It also infers missing keywords,
categories, repository links, MSRV, `no_std`, maintenance, and deprecation
signals. These are useful discovery enrichments, but their composite and
inferred values are not substitutes for the exact evidence they summarize.

### Exact CLI selection control

The three Blessed.rs CLI candidates implemented the same bounded behavior:

```text
--name ferrium --verbose
name=ferrium verbose=true
```

| Candidate | Exact release | Declared Rust | Active packages including fixture | Build-script packages | Proc-macro packages | Release executable |
|---|---:|---:|---:|---:|---:|---:|
| `clap` with `derive` | 4.6.6 | 1.85 | 22 | 2 | 1 | 635,904 bytes |
| `lexopt` | 0.3.2 | not declared | 2 | 0 | 0 | 156,160 bytes |
| `pico-args` | 0.5.0 | not declared | 2 | 0 | 0 | 142,848 bytes |

All three exact archive hashes matched crates.io. All three lockfiles returned
zero RustSec matches under cargo-audit 0.22.2 on 2026-08-10. Those are bounded
identity and dated advisory observations, not quality or safety conclusions.

With a Rust 1.71 consumer constraint:

- `clap 4.6.6` failed because its package uses edition 2024 and declares Rust
  1.85;
- `clap 4.6.6` passed under Rust 1.85; and
- `lexopt 0.3.2` and `pico-args 0.5.0` passed under Rust 1.71 despite not
  declaring `rust-version`.

The exact invalid-option diagnostics also differed:

- Clap generated an exit-2 error, usage, and help hint.
- Lexopt returned an explicit invalid-option error through the fixture.
- Pico-args exposed the leftover argument for fixture-owned handling.

The control does not declare one winner. It establishes that a fully featured
derive profile, a minimal-manual profile, an MSRV-constrained profile, and a
diagnostic-policy profile legitimately retain different candidates.

## Findings

### FERRIUM-640: discovery and selection are different decisions

**Sources:** official search matrix, exact CLI control, and ECOS-Q01 through
ECOS-Q09 evidence models.

**Observed behavior:** Search produced names and summaries. Exact consumer
constraints changed eligibility only after release, feature, closure, compiler,
and behavior evidence was joined.

**Implication:** OSPREY must represent retrieval and approval as separate
records with different owners and failure states.

**Confidence:** High.

### FERRIUM-641: Cargo search is a candidate retrieval surface

**Sources:** Cargo `cargo-search` documentation and the eight Cargo/API
agreement controls.

**Observed behavior:** Cargo displayed package name, latest selected version,
and description. Its top ten matched crates.io relevance for all eight observed
queries.

**Implication:** FERRIUM can ingest Cargo search as a reproducible retrieval
input, but it cannot treat its output as a compatibility or assurance record.

**Confidence:** High for the observed Cargo/crates.io versions.

### FERRIUM-642: relevance is already popularity-gated

**Sources:** crates.io search implementation at revision
`1bb85949b723e3c0f27c730e99c8e31c1b33a5ca`, crates.io's 2026-07 development
update, and the page-101 control.

**Observed behavior:** Relevance is bounded to 1,000 candidates selected by
recent downloads before full-text rank. Results beyond that boundary are
unreachable through relevance pagination.

**Implication:** "Relevance rather than popularity" is a false separation for
current crates.io search. Retrieval provenance must record the ranking policy.

**Confidence:** High.

### FERRIUM-643: sort choice changes the candidate population

**Sources:** 400-row official search matrix.

**Observed behavior:** Mean top-ten Jaccard overlap between relevance and
all-time or recent downloads was 0.030; relevance and recent updates had zero
overlap for every query.

**Implication:** A recommendation cannot cite only a query. It must retain
source, sort, filters, observation time, rank, page, and limit.

**Confidence:** High for the bounded query set.

### FERRIUM-644: exact-name rank proves identity, not fitness

**Sources:** JSON, TLS, and SQLite exact-name controls.

**Observed behavior:** `json`, `tls`, and `sqlite` ranked first for their exact
query names. `json 0.12.4` was last updated in 2020 and `tls 0.0.3` in 2015,
while other candidates represented different current contracts.

**Implication:** Exact-name preference is useful for navigation but must not
be interpreted as currency, stewardship, compatibility, or capability fit.

**Confidence:** High.

### FERRIUM-645: natural-language queries do not define a capability ontology

**Sources:** known-candidate visibility matrix.

**Observed behavior:** Established alternatives were absent from all five
top-ten lists, while specialized wrappers and neighboring foundations often
ranked above the intended application role.

**Implication:** FERRIUM needs capability, role, contract-family, provider, and
consumer-profile identities outside raw search text.

**Confidence:** High for the selected queries; not a recall estimate for all
crates.

### FERRIUM-646: popularity can rank the wrong layer

**Sources:** download-sorted results for the eight queries and ECOS-Q02's
foundational-role taxonomy.

**Observed behavior:** Download sorting promoted facades, substrates, protocol
foundations, and transitive implementation crates such as Serde, Hyper, Mio,
Hashbrown, and libsqlite3-sys.

**Implication:** Role classification must precede popularity comparison.
Packages at different architectural layers are not interchangeable candidates.

**Confidence:** High.

### FERRIUM-647: recency is not relevance

**Sources:** recent-update search controls.

**Observed behavior:** Recent-update top tens had zero overlap with relevance
top tens across all eight queries and frequently surfaced newly published
applications that merely mentioned the query terms.

**Implication:** Update time is a review trigger and retrieval sort, not a
fitness or maintenance verdict.

**Confidence:** High for the observed snapshot.

### FERRIUM-648: curation adds intent but requires exact renewal

**Sources:** Blessed.rs guide and `data/crates.json` revision
`c750a3d44011465b4d4c7a811e7752c4a63f0415`.

**Observed behavior:** The guide usefully separated fully featured and minimal
CLI choices, but its recommendation records named crates rather than exact
releases or current evidence packets.

**Implication:** Curated sources should be ingested as attributed candidate and
rationale evidence, pinned to a revision, then renewed against exact releases.

**Confidence:** High.

### FERRIUM-649: composite quality scores contain policy

**Sources:** lib.rs About and Data Processing pages.

**Observed behavior:** Lib.rs combines many weighted signals, inferences,
manual overrides, and blocklists. Some fields are inferred from text, features,
dependencies, repository state, and build observations.

**Implication:** Composite rank is a source-owned policy output. FERRIUM may
display it with provenance but must not relabel it as objective crate quality.

**Confidence:** High for the documented methodology; weights and current
internal state were not independently reproduced.

### FERRIUM-650: mandatory requirements filter before preferences rank

**Sources:** Rust 1.71 and 1.85 CLI controls.

**Observed behavior:** Clap's exact release was ineligible for the Rust 1.71
profile before downloads, ergonomics, diagnostics, or artifact size were
considered. Lexopt and pico-args remained eligible for the bounded operation.

**Implication:** Selection engines must evaluate hard requirements before
applying preferences or comparisons.

**Confidence:** High.

### FERRIUM-651: equivalent output can hide materially different closures

**Sources:** exact CLI metadata, build, execution, and invalid-option controls.

**Observed behavior:** The three fixtures printed the same success output, but
the Clap derive closure contained 22 packages, two build-script packages, and a
procedural macro; the manual alternatives contained one dependency each.
Diagnostics and artifacts also differed.

**Implication:** Functional examples are necessary but insufficient. Selection
evidence must retain closure, execution, compile-time code, diagnostics, and
artifact tradeoffs.

**Confidence:** High for the exact fixture.

### FERRIUM-652: release age is a trigger, not a rejection rule

**Sources:** `pico-args 0.5.0` release metadata and Rust 1.71 execution control.

**Observed behavior:** The 2022 release passed the bounded current fixture.
That does not establish responsiveness, succession, broader compatibility, or
future support.

**Implication:** Age may request stewardship review but cannot alone establish
abandonment or fitness.

**Confidence:** High for behavior; no maintenance conclusion.

### FERRIUM-653: absent declarations remain unknown after a passing check

**Sources:** Lexopt and pico-args manifests and Rust 1.71 controls.

**Observed behavior:** Neither exact release declared `rust-version`; both
compiled in one selected consumer under Rust 1.71.

**Implication:** FERRIUM must record "observed on 1.71" separately from
"declared MSRV." It must not infer a maintained lower-bound policy.

**Confidence:** High.

### FERRIUM-654: no scalar ordering preserves consumer tradeoffs

**Sources:** full versus minimal CLI curation and exact control.

**Observed behavior:** Generated usage, derive ergonomics, MSRV, dependency
closure, compile-time execution, diagnostics, and artifact size favored
different candidates or required consumer-owned weighting.

**Implication:** Eligible candidates should form a visible tradeoff frontier.
Any scoring policy must be named, versioned, explainable, and consumer-owned.

**Confidence:** High.

### FERRIUM-655: recommendations are scoped claims

**Sources:** dynamic search rankings, exact release observations, and prior
Crates Series renewal models.

**Observed behavior:** Retrieval order, current releases, owner sets,
advisories, compiler requirements, and curation changed independently.

**Implication:** A recommendation must name consumer profile, exact release,
feature closure, evidence snapshot, source policies, owner, expiry, and
rollback. "Use crate X" is not a durable decision record.

**Confidence:** High.

### FERRIUM-656: AI must preserve evidence classes

**Sources:** search descriptions, lib.rs inference policy, curated rationale,
and measured controls.

**Observed behavior:** Discovery sources mix publisher claims, algorithmic
ranking, inferred metadata, curator judgment, and executed observations.

**Implication:** AI-generated selection assistance must label retrieved,
declared, inferred, measured, recommended, and unknown assertions separately
and cite the exact source for each.

**Confidence:** High.

### FERRIUM-657: the defensible wedge is an evidence filter and decision record

**Sources:** ECOS-Q01 through ECOS-Q10.

**Observed behavior:** Existing services already own registry search,
documentation, curation, security advisories, policy linting, and audits. The
remaining gap is joining those sources to consumer requirements and measured
compatibility without collapsing their meanings.

**Implication:** FERRIUM should specify a capability map, candidate retrieval
record, typed eligibility matrix, tradeoff frontier, and renewable selection
decision. It should not build another registry or universal crate score.

**Confidence:** High.

## Decision

### Adopt now

- Add consumer intent, retrieval provenance, candidate role, exact identity,
  evidence coverage, eligibility, tradeoff, decision, and renewal records to
  the OSPREY Ecosystem adapter and Crate Ecosystem Ledger.
- Treat search, downloads, categories, keywords, curation, reputation, and
  composite scores as attributed candidate-generation signals.
- Evaluate mandatory requirements before preferences.
- Preserve declared, inferred, observed, unsupported, stale, conflicting, and
  unknown states.
- Present eligible candidates as a tradeoff frontier rather than one rank.
- Require every recommendation to name an exact release, feature closure,
  consumer profile, evidence snapshot, owner, expiry, alternatives, rollback,
  and non-goals.
- Preserve AI assertion class and source for every synthesized claim.

Owner: FERRIUM.

Expected validation: ECOS-Q11 renewable stack profiles, ECOS-Q12 intervention
decisions, and later CONFORMANCE-001 tests over held-out candidate sets.

Non-goals: universal rankings, package installation, dependency rewrites,
automatic approval, maintainer scoring, security certification, replacing
existing discovery services, or OSPREY implementation.

### Prototype behind a compatibility boundary

- read-only ingestion of Cargo/crates.io queries with exact source, sort,
  filter, page, rank, limit, and observation time;
- attributed import of categories, curated lists, RustSec suggestions,
  std-replacement data, cargo-vet audits, and policy-tool results;
- capability and architectural-role classification with user correction;
- exact release evidence joins across ECOS-Q03 through ECOS-Q09 dimensions;
- mandatory requirement filters with typed unknown and conflict states;
- explainable, consumer-owned preference policies over the eligible frontier;
- immutable selection packets and renewal diffs; and
- held-out evaluation measuring candidate recall, evidence coverage, false
  exclusion, stale decisions, and explanation accuracy.

### Reject or defer

- one FERRIUM crate-quality, safety, maintenance, portability, or popularity
  score;
- approval from search rank, downloads, stars, age, owner count, release
  frequency, or one advisory result;
- silently treating inferred metadata as declared or measured;
- recommendations without exact release and consumer scope;
- automatic dependency edits, provider switches, feature changes, upgrades,
  forks, or package installation;
- claims that a live curated list is current without a pinned revision and
  renewal check;
- AI-generated rationales without source and assertion class; and
- OSPREY implementation before the Crates Series gate.

## Role review

### Rust Safety Steward

Accepts exact closure, compile-time execution, unsafe/native joins, audit
criteria, and typed unknowns as selection evidence. Rejects popularity,
publisher descriptions, or zero advisory matches as safety proof.

### Compiler Performance Engineer

Accepts package count, build-script and procedural-macro presence, artifact
size, and prior measured compiler evidence as tradeoffs. Rejects ranking the
CLI candidates by one concurrent cold-build observation.

### Interop Boundary Auditor

Accepts capability role, public contract family, feature closure, provider,
adapter, and runtime behavior as eligibility inputs. Requires ECOS-Q11 to test
selected candidates in complete stack boundaries.

### AI Assurance Skeptic

Accepts source revisions, exact API queries, expected failure, archive hashes,
compiler controls, and labeled curated/inferred evidence. Requires AI output to
separate retrieval, declaration, inference, measurement, and recommendation.

### Ecosystem Strategist

Accepts the capability map and evidence filter as a defensible wedge because
existing services remain retrieval and evidence owners. Rejects a competing
registry, recommendation brand, or universal score.

### Rust Maintainer

Accepts consumer-scoped selection records with reasons, alternatives, expiry,
and rollback. Requires diagnostics to identify which requirement excluded a
candidate and which upstream owner can resolve missing evidence.

### Native Platform Adopter

Accepts target, provider, native prerequisite, deployment, support, and
operations evidence as mandatory-profile inputs. Rejects rankings that erase
system integration or rollout cost.

### Scope Keeper

Accepts Q10 as discovery and decision-model research only. Dependency changes,
stack approval, automated actions, production integration, and OSPREY
implementation remain closed.

### Validation Checker

Accepts eight queries, five sorts, 400 official results, Cargo/API agreement,
pagination failure, source revisions, curated-data inspection, three exact
release identities, archive checks, closure/build/run/error/MSRV controls,
three dated audits, sources, and limitations. Requires held-out and
multi-profile renewal in ECOS-Q11.

## Limitations

- Search results are one 2026-08-10 snapshot and will change.
- Eight English queries do not estimate total ecosystem search quality or
  recall.
- Only the first ten results of each observed sort were compared.
- crates.io relevance internals may change after the cited source revision.
- No repository-star ranking or social recommendation corpus was measured.
- Lib.rs weights and live internal data were not independently reproduced.
- Blessed.rs was inspected as one current revision, not evaluated for
  historical recommendation accuracy.
- The CLI control covered one small parsing operation on Windows x86-64.
- Artifact sizes include fixture code and one release profile; they are not
  general performance rankings.
- The concurrent first-build elapsed times were affected by Cargo package-cache
  locking and are not used as comparative evidence.
- Rust 1.71 checks do not establish the minimum supported Rust version for
  Lexopt or pico-args.
- Zero RustSec matches do not establish absence of vulnerabilities.
- No usability study, API-learning study, support-response test, or production
  deployment was performed.
- No crate or stack was approved for adoption.

## Primary sources

- Cargo search:
  <https://doc.rust-lang.org/cargo/commands/cargo-search.html>
- Cargo info:
  <https://doc.rust-lang.org/cargo/commands/cargo-info.html>
- crates.io search source:
  <https://github.com/rust-lang/crates.io/blob/main/src/controllers/krate/search.rs>
- crates.io development update:
  <https://blog.rust-lang.org/2026/07/13/crates-io-development-update/>
- crates.io data access:
  <https://crates.io/data-access>
- crates.io default-ranking RFC:
  <https://github.com/rust-lang/rfcs/blob/master/text/1824-crates.io-default-ranking.md>
- Blessed.rs crate directory:
  <https://blessed.rs/crates>
- Blessed.rs data:
  <https://github.com/nicoburns/blessed-rs/blob/main/data/crates.json>
- Lib.rs About:
  <https://lib.rs/about>
- Lib.rs Data Processing:
  <https://lib.rs/data-processing>
- Cargo Vet:
  <https://mozilla.github.io/cargo-vet/>
- cargo-deny:
  <https://embarkstudios.github.io/cargo-deny/>
- RustSec:
  <https://rustsec.org/>
- Clap:
  <https://docs.rs/clap/4.6.6/clap/>
- Lexopt:
  <https://docs.rs/lexopt/0.3.2/lexopt/>
- pico-args:
  <https://docs.rs/pico-args/0.5.0/pico_args/>
