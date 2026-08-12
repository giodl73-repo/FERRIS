# Microsoft Rust Leadership Package Scorecard

Date: 2026-08-11  
Rubric version: v1.0  
Status: Reviewed; targeted revision required before executive circulation

## Decision supported

This scorecard decides whether the Microsoft Rust leadership package is ready
for:

1. peer discussion;
2. executive sponsor review; or
3. an investment decision.

It evaluates:

- `MICROSOFT_RUST_INVESTMENT_BRIEF.md`;
- `MICROSOFT_RUST_UPSTREAM_OPPORTUNITY_MAP.md`; and
- `MICROSOFT_RUST_INVESTMENT_DECK.pptx`.

The scorecard adapts recurring portfolio review principles:

- TAXLANE: incomplete evidence must remain visibly incomplete;
- VTRACE: claims should trace from need through evidence and verification;
- ROUTE: a null result and bounded scope remain valid;
- portfolio README review: a strong thesis must combine evidence, stakes,
  clarity, truth, and a memorable principle.

## Five-part rubric

Each axis is scored from 1 to 5.

| Score | Meaning |
|---:|---|
| 5 | Decision-ready: specific, evidenced, bounded, and immediately useful |
| 4 | Strong: credible and useful with a small number of explicit gaps |
| 3 | Promising: direction is sound but important proof or specificity is missing |
| 2 | Weak: substantial reframing or evidence work is required |
| 1 | Unsupported: the package cannot responsibly support the claimed decision |

### Axes

| Axis | Question |
|---|---|
| 1. Strategic thesis and stakes | Is there a memorable, consequential decision rather than a technology description? |
| 2. Evidence and truth discipline | Are material claims primary-source-backed, correctly bounded, and explicit about unknowns? |
| 3. Microsoft differentiation and ecosystem fairness | Does the package separate broad upstream benefit from Microsoft-specific product advantage? |
| 4. Executability and traceability | Are ownership, sequence, gates, measures, alternatives, and proof conditions concrete enough to act on? |
| 5. Leadership communication | Can a senior reader understand the opportunity, risk, investment, and ask quickly enough to sponsor it? |

## Promotion bands

| Total | Classification | Promotion rule |
|---:|---|---|
| 23–25 | Sponsor-ready | Circulate for executive sponsorship with ordinary tailoring |
| 20–22 | Strong discussion draft | Peer-circulate; close named gaps before requesting investment |
| 16–19 | Promising concept | Substantive revision before leadership circulation |
| 11–15 | Weak case | Rebuild evidence, decision framing, and proposed action |
| 5–10 | Unsupported | Do not circulate as an investment case |

## Aggregate score

**21 / 25 — Strong discussion draft**

The package is ready for informed peer discussion. It is not yet ready to ask
leadership for funded execution because it does not establish the size and
shape of Microsoft's Rust estate, quantify the cost of fragmentation or the
expected return, name an accountable organizational home, or attach a
claim-level evidence ledger.

## Axis scores

### 1. Strategic thesis and stakes — 5 / 5

**What works**

- The headline is memorable: **govern the conversion while the estate is
  still forming**.
- The package avoids the weaker “Rust is popular” thesis.
- It reframes the opportunity as application-platform coordination across
  workspaces, crates, contracts, native boundaries, AI, security, and
  lifecycle.
- “Interoperability is the new rewrite” prevents a reckless wholesale-rewrite
  interpretation.
- The risk of uncoordinated success is explicit and credible.

**Why this is a 5**

The reader leaves with a portable principle and a time-sensitive decision:
coordinate the Rust estate before local adoption hardens into platform debt.

### 2. Evidence and truth discipline — 4 / 5

**What works**

- Headline adoption and security claims use public first-party or primary
  sources.
- Survey percentages are attributed to respondents rather than generalized to
  all organizations.
- crates.io values are dated observations.
- Microsoft intent, SDK maturity, OpenVMM, and Rust Foundation participation
  are sourced.
- The package explicitly rejects rewrite mandates, universal safety claims,
  approved-crate certification, and lines-of-code vanity measures.

**Gap**

The evidence establishes industry momentum and public Microsoft intent, but
not the internal Microsoft opportunity size. There is no verified baseline
for:

- Microsoft repositories containing Rust;
- active Cargo workspaces and crates;
- Rust/C++ and native boundaries;
- duplicated crate/provider choices;
- build, review, incident, or migration costs;
- security-sensitive new-code exposure; or
- teams prepared to adopt an application blueprint.

The current data supports **why to investigate and coordinate**, not a claim
that a specific Microsoft-wide program will produce a quantified return.

### 3. Microsoft differentiation and ecosystem fairness — 5 / 5

**What works**

- The upstream/public-good and differentiated-product portfolios are visibly
  separate.
- rustc, Cargo, crates.io, Rust Foundation, crate owners, WIT, and other
  upstream authorities retain ownership.
- GitHub, Copilot, Azure, Windows, Entra, Key Vault, policy, provenance, and
  enterprise support are correctly presented as Microsoft's asymmetric
  assets.
- The package rejects a Microsoft Rust fork, private compatibility dialect,
  downstream ownership takeover, and mandatory Microsoft dependencies.
- Success upstream is defined as accepted and maintained community value.

**Why this is a 5**

This is the package's most differentiated and politically credible feature.
It gives Microsoft a way to earn ecosystem credit while preserving a clear
product strategy.

### 4. Executability and traceability — 4 / 5

**What works**

- The strategy has a staged 0–24 month sequence.
- The first proof has entry characteristics and explicit pass conditions.
- The package names application-blueprint contents, measures, guardrails, and
  contribution packets.
- Adoption, renewal, fallback, rollback, removal, and null outcomes remain
  valid.
- Ferris is bounded as a proving vehicle rather than presented as an
  authorized enterprise platform.

**Gap**

The program lacks:

- an accountable executive sponsor and organizational home;
- a responsible product or engineering lead;
- an indicative team shape, funding range, and opportunity cost;
- a candidate application and participating organizations;
- a decision between build, partner, upstream-only, and do-nothing options;
- a privacy and legal approval path for the estate census;
- baseline values and target thresholds for the proposed measures; and
- an explicit stop rule if the census finds insufficient coordination value.

The package explains what a good pilot looks like, but not yet who can start it
on Monday.

### 5. Leadership communication — 3 / 5

**What works**

- The deck has a coherent 12-slide narrative.
- It uses a distinctive visual system and varied layouts.
- The brief, map, and deck reinforce the same thesis.
- The leadership asks are visible and bounded.
- The package contains several memorable lines suitable for advocacy.

**Gap**

For a senior Microsoft audience, the package remains too concept-dense and
under-specified at the investment point:

- no single slide quantifies the Microsoft-sized problem;
- no slide gives a team, funding, or organizational-home option;
- no explicit alternatives slide shows why this program beats upstream-only,
  decentralized adoption, or existing platform processes;
- citations are source-level rather than claim-level;
- the deck lacks a one-page appendix mapping every headline number to date,
  scope, caveat, and URL; and
- Ferris appears before a Microsoft design partner or internal proof has been
  named, which may make the proposal feel tool-led rather than
  customer-led.

The communication is strong enough to create interest, but not yet optimized
to secure budget.

## Artifact scores

| Artifact | Thesis | Evidence | Differentiation | Execution | Communication | Total |
|---|---:|---:|---:|---:|---:|---:|
| Executive brief | 5 | 4 | 5 | 4 | 4 | **22 / 25** |
| Opportunity map | 4 | 4 | 5 | 5 | 3 | **21 / 25** |
| Leadership deck | 5 | 4 | 4 | 3 | 4 | **20 / 25** |

The package aggregate is the rounded mean: **21 / 25**.

## Blocking gaps before an investment ask

### G1. Microsoft Rust estate baseline

Produce a privacy-safe, source-owned census with counts and distributions for
repositories, workspaces, crates, toolchains, native boundaries, providers,
owners, and application mappings.

**Promotion evidence:** dated census methodology, coverage statement, unknown
classification, owner review, and reproducible aggregate.

### G2. Economic and risk model

Quantify the cost of uncoordinated adoption and the expected value of one
blueprint proof. Include ranges rather than false precision.

At minimum model:

- duplicated platform engineering;
- CI and build investigation time;
- dependency and native-boundary review;
- migration and rollback effort;
- security and provenance coverage;
- maintainer funding; and
- pilot team cost.

**Promotion evidence:** assumptions ledger, low/base/high scenarios, named
unknowns, and a do-nothing comparison.

### G3. Named design partners and ownership

Identify:

- one Windows or Azure systems application;
- one GitHub/Copilot workflow;
- one accountable executive sponsor;
- one engineering lead;
- one upstream liaison; and
- one privacy/security reviewer.

**Promotion evidence:** written participation, bounded scope, owner-approved
success measures, and an exit path.

### G4. Alternatives and null case

Compare:

1. decentralized team adoption;
2. upstream funding only;
3. existing GitHub/Azure/BuildXL processes;
4. a bounded Ferris blueprint proof; and
5. no new program.

State what evidence would cause the blueprint strategy to be rejected.

**Promotion evidence:** decision matrix with benefits, costs, risks,
dependencies, reversibility, and stop conditions.

### G5. Claim ledger and executive appendix

Create a compact evidence appendix containing:

- claim ID;
- exact wording;
- source owner;
- publication date;
- observed date;
- population or scope;
- caveat;
- URL; and
- slide and brief references.

**Promotion evidence:** every quantitative or Microsoft-adoption claim maps to
one reviewed ledger row.

## Fastest path to 24 / 25

1. Add a Microsoft-estate baseline or explicitly label it as the funded
   discovery deliverable with a pre-approved method.
2. Add one investment-options slide with team shape, six-month cost range, and
   organizational-home alternatives.
3. Add a do-nothing/upstream-only/decentralized/Ferris comparison.
4. Add a claim ledger and source appendix.
5. Lead the pilot slide with named design-partner needs; position Ferris as the
   removable proving mechanism beneath them.

Expected revised scores:

| Axis | Current | Target |
|---|---:|---:|
| Strategic thesis and stakes | 5 | 5 |
| Evidence and truth discipline | 4 | 5 |
| Microsoft differentiation and ecosystem fairness | 5 | 5 |
| Executability and traceability | 4 | 5 |
| Leadership communication | 3 | 4 |
| **Total** | **21** | **24** |

## Current promotion decision

**Peer circulation: approved.**  
**Executive sponsor discussion: approved as a discovery proposal.**  
**Funded platform investment decision: blocked pending G1–G5.**

The package must remain labeled as a discussion draft. It must not be used to
claim that Microsoft has already validated the Ferris strategy, measured the
internal Rust estate, selected a product home, or committed funding.

## Trace and validation

Expected trace links:

- leadership claims to public evidence;
- scores to specific package sections or slides;
- blocking gaps to promotion evidence;
- future revisions to the rubric version and prior frozen score.

Evidence produced:

- five-axis package score;
- artifact-level scores;
- blocking-gap register;
- promotion decision; and
- target score for the next revision.

Validation command:

```powershell
git diff --check
```

