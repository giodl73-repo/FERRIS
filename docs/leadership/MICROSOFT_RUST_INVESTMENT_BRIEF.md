# Microsoft Rust: Govern the Conversion While the Estate Is Still Forming

Date: 2026-09-21
Audience: Microsoft engineering leaders, developer-platform leaders, security
leaders, and prospective sponsors  
Status: Leadership discussion draft; refreshed for enterprise value validation

## Executive recommendation

Microsoft should treat Rust adoption as a coordinated application-platform
transition, not as a collection of unrelated language migrations.

The immediate opportunity is to:

1. establish a cross-company Rust portfolio view across repositories,
   workspaces, crates, native dependencies, platforms, and application
   boundaries;
2. publish renewable application blueprints that describe supported stacks,
   contracts, validation, security evidence, lifecycle, and rollback;
3. make GitHub, Copilot, Azure, Windows, and Microsoft security systems the
   best environment in which to adopt and govern Rust; and
4. contribute shared improvements through the Rust Foundation and existing
   upstream owners without turning community infrastructure into a
   Microsoft-controlled dependency.

This is a time-sensitive coordination opportunity. Rust is already moving
from admired language to production systems platform. Microsoft has already
made public investments in Azure, virtualization, SDKs, open source, and the
Rust Foundation. The strategic question is no longer whether isolated teams
will use Rust. It is whether those teams will converge on reusable contracts,
supported crate profiles, safe native boundaries, common evidence, and a
governed migration strategy before fragmentation becomes expensive.

## The evidence for action

| Claim | Signal | Public evidence | Strategic meaning |
|---|---|---|---|
| P-01 | Developer pull | Rust was the most admired programming language in Stack Overflow's 2024 survey, at approximately 83%. | Recruiting and internal advocacy start with unusually strong developer preference. |
| P-02 | Organizational use | The 2025 State of Rust survey reports 48.8% of responding organizations making non-trivial use of Rust, continuing a multi-year rise. | Rust is becoming an organizational capability, not only an individual experiment. |
| P-03, P-04 | Ecosystem scale | The crates.io public API reported 314,949 crates and 395.5 billion cumulative downloads on 2026-08-11. | Package selection, provenance, maintenance, and compatibility are now portfolio-scale concerns. |
| P-05, P-06 | Security outcome | Google reported Android memory-safety vulnerabilities falling from 76% to 24% over six years as new development shifted toward memory-safe languages. Rust changes also had less than half the rollback rate of C++ changes. | Incremental safe-language adoption can improve a large existing estate without rewriting it all. |
| M-01 | Microsoft intent | Azure states that Rust has already been adopted in critical infrastructure components and that adoption is expected to expand substantially. | The internal coordination need is already present. |
| M-02 | Microsoft product maturity | The Azure SDK for Rust is stable, with stable core, identity, Key Vault, and Storage libraries and SemVer guarantees. | Microsoft is creating a credible first-party Rust application surface. |
| M-03 | Microsoft systems proof | OpenVMM is a Microsoft open-source, modular, cross-platform VMM written in Rust. | Rust is already viable in demanding Microsoft systems contexts. |
| M-04 | Ecosystem stewardship | Microsoft joined the Rust Foundation as a founding Platinum member in January 2021. | Microsoft has an established fair-governance channel for community investment. |

These signals should not be read as a mandate to rewrite mature code.
Google's published conclusion is more useful: **interoperability is the new
rewrite**. New and actively changing security-sensitive code should move
toward memory-safe implementation, while existing assets cross explicit,
tested boundaries and age safely.

## The risk of uncoordinated success

Rust can spread successfully inside Microsoft and still produce a weak
enterprise outcome.

Without a portfolio strategy, teams will independently choose:

- different crate stacks for the same capability;
- incompatible async, TLS, cryptography, telemetry, and serialization
  providers;
- different approaches to C++, C ABI, WIT, generated bindings, and wire
  contracts;
- different minimum Rust versions, targets, feature closures, and native
  toolchains;
- incomplete or inconsistent supply-chain evidence;
- local CI rules that cannot explain application-level affected work;
- AI-generated changes without durable provenance or validation scope; and
- migration plans that do not include renewal, substitution, removal, or
  rollback.

Cargo correctly owns package resolution and workspace execution. It does not
own Microsoft's application portfolio, service boundaries, support promises,
security policy, deployment evidence, or cross-repository change planning.
Those are the missing layers.

## The proposed Microsoft Rust application strategy

### 1. Discover and map the Rust estate

Create an inventory of:

- repositories, Cargo workspaces, packages, targets, features, and toolchains;
- applications, services, deployable units, and owning organizations;
- C/C++, .NET, Java, Python, WebAssembly, firmware, and service boundaries;
- native libraries, build scripts, procedural macros, code generators, and
  ambient tools;
- crate versions, publishers, provenance, advisories, licenses, maintenance,
  and support status; and
- validation, deployment, incident, renewal, and rollback paths.

The output is not a central replacement graph. It is a typed portfolio map
that preserves each owner's authority.

### 2. Publish application blueprints

A Microsoft Rust application blueprint should bind:

- business application and component identities;
- exact Cargo workspaces and active dependency closures;
- supported crate and provider profiles;
- Rust, C ABI, CXX, WIT, or wire-schema contracts;
- Windows, Linux, Azure, edge, firmware, and WebAssembly targets;
- security, provenance, native-boundary, and policy evidence;
- required build, test, fuzz, compatibility, and deployment validation;
- affected-work and change-impact rules;
- support period, renewal date, owner, successor, removal, and rollback.

Blueprints should be renewable evidence records, not permanent approved-crate
lists. They provide a stable way for teams and AI agents to answer:

> What is this application, what Rust is supported here, what changes now,
> what must be validated, and who has authority to proceed?

### 3. Make conversion incremental and boundary-led

Prioritize new code and high-change, security-sensitive components. Retain
mature code when replacement economics are weak. Invest heavily in:

- safe and explicit Rust/C++ integration;
- opaque C ABI boundaries for independently versioned native components;
- WIT/component boundaries for sandboxed and polyglot extensions;
- wire contracts for services and durable messages;
- generated binding provenance and conformance;
- side-by-side migration, fallback, and rollback; and
- measurable reduction in memory-unsafe change exposure.

### 4. Make GitHub and Copilot the control surface

GitHub and Copilot can turn Rust's compiler feedback into a differentiated
assurance loop:

- discover the application and affected work before generation;
- generate only inside an approved blueprint and scope;
- explain selected packages, features, contracts, native effects, and tests;
- validate against owner tools and full-reference controls;
- attach provenance, policy, and test evidence to the change;
- require human approval for narrowing, exceptions, and mutation; and
- learn from accepted diagnostics without treating model confidence as proof.

This is a target workflow, not a current Ferris capability. Its proposed value
is governed native change grounded in compiler and application evidence. Every
use must record the source revision, model action, exact commands, results, and
human approval. Ferris has no GitHub, Azure, Copilot, or MCP integration today.

## Two investment portfolios

### Fair upstream public-good investment

Microsoft should earn community trust by contributing through existing owners:

- fund Rust compiler, Cargo, rust-analyzer, debugging, Windows target, and
  infrastructure maintainers;
- contribute minimized performance benchmarks and regression evidence;
- improve Windows/MSVC linking, debugging, PDB, diagnostics, and target
  reliability;
- improve safe interop patterns and documentation;
- strengthen crates.io, trusted publishing, provenance, advisories, and
  maintainer sustainability;
- sponsor compatibility and conformance work for critical crates;
- improve async/concurrency diagnostics and observability;
- support WIT/component-model and cross-language contract work; and
- publish reusable evaluation fixtures and evidence rather than private
  downstream forks.

These are candidate contribution areas, not committed projects. Select an area
only after a maintainer identifies where the work belongs, a named consumer
confirms the need, and a build-versus-contribute review favors investment.

Success is upstream acceptance, sustained maintenance, and broad ecosystem
benefit—not Microsoft ownership.

### Differentiated Microsoft value

Microsoft should build product value where it has asymmetric assets:

- GitHub-native Rust estate discovery and application blueprints;
- Copilot planning, generation, review, and validation grounded in Cargo and
  compiler evidence;
- Azure-hosted Rust build intelligence, provenance, and policy integration;
- Windows as the best-supported enterprise Rust native platform;
- Entra, Key Vault, Azure Artifacts, Azure Monitor, attestations, and policy
  connectors;
- enterprise support profiles for common Rust application families;
- cross-repository affected-work planning across Rust and polyglot systems;
- governed agent protocols through MCP; and
- lifecycle evidence for migration, renewal, substitution, removal, and
  rollback.

These capabilities should consume portable contracts and remain removable.
Microsoft services are premium integrations, not the canonical definition of
Rust application correctness.

## Ferris as the governed proving vehicle

Ferris is positioned as a bounded research and engineering vehicle for this
strategy. It is not a Cargo or rustc replacement.

Ferris now demonstrates a public, bounded product foundation:

- `cargo ferris` asks Cargo to locate the current workspace;
- `validation-plan` turns explicit changed paths and packages into
  conservative validation scope;
- `federated-plan` links two to sixteen independent Cargo workspace plans
  without inventing one shared resolver, dependency graph, or lock;
- approved Action Plans execute exact owner commands and produce deterministic
  receipts that preserve direct and transitive blockers for verification and
  replay;
- replay, scheduling analysis, artifact qualification, workspace readiness,
  and application readiness preserve explicit owner boundaries; and
- repeatable enterprise onboarding, execution, receipt verification, removal,
  and unchanged post-removal owner commands passed on Windows and native Linux.

This proof is intentionally smaller than the proposed platform. Ferris does
not yet demonstrate positive selected-versus-full value, justify narrowing a
required CI gate, define a production support boundary, integrate GitHub or
Azure evidence, expose MCP, or own remote execution, cache, publication, or
deployment.

The value result remains unresolved. In the first 40-revision history shadow,
37 plans widened to full validation and the only admissible narrowed comparison
was 7.7% slower. Later cohorts did not produce admissible timing because owner
prerequisites, measurement correctness, or local capacity failed preflight.

The next Ferris phase should progress through explicit gates. This package
proposes no implementation authority by itself:

1. authorize at most one new selected-versus-full cohort after a named owner
   freezes passing selected and full commands, prerequisites, hardware,
   toolchain, cache state, capacity, alternating lane order, and cold/warm
   repetitions;
2. require a repeatable positive result with zero selected-pass/full-fail
   divergence; stop affected-only value work if preflight is invalid, any
   divergence occurs, or the result is nonpositive;
3. run one owner-approved advisory CI shadow while retaining required checks;
4. define the smallest exercised installation, schema, platform, ABI,
   ownership, panic/error, threading, allocation, debugging, deployment,
   rollback, audit, and support boundary;
5. complete a privacy-safe Microsoft Rust estate baseline and economic model;
6. name the accountable sponsor, application owner, engineering lead, and
   review partners; and
7. submit independently useful upstream evidence and contributions where the
   shared language ecosystem owns the problem.

## A phased investment

| Phase | Duration | Outcome |
|---|---:|---|
| 0. Product foundation | Complete | Cargo-native planning, approved owner execution, deterministic evidence, Windows/Linux onboarding, and complete removal proof |
| 1. Sponsor and validate | 0-6 months | Named owner, estate and economic baseline, and at most one preflight-qualified value cohort; stop if invalid, divergent, or nonpositive |
| 2. Advisory pilot | Gate 2 required | Owner-approved CI shadow with required checks retained and divergence measured |
| 3. Productize selectively | Gates 1-4 required | Smallest supported capability set, named service ownership, renewable profiles, and measured outcomes |

## Measures that matter

- percentage of Rust repositories mapped to an owned application blueprint;
- percentage of active dependency closures covered by renewable profiles;
- time to explain affected work and required validation;
- build and review time avoided without reduced validation coverage;
- memory-unsafe new-code exposure in selected portfolios;
- critical native boundaries with explicit contracts and conformance tests;
- provenance and attestation coverage;
- stale, unsupported, or ownerless dependencies found before release;
- rollback and removal success;
- upstream contributions accepted and maintained;
- funded maintainer capacity; and
- developer adoption and satisfaction.

Avoid vanity measures such as total Rust lines, number of crates approved,
number of AI-generated changes, or downstream patches produced.

## Leadership asks

1. Sponsor bounded discovery and value validation rather than a platform
   funding decision.
2. Name one accountable sponsor, one application owner, and one
   GitHub/Copilot workflow partner.
3. Authorize at most one preflight-qualified selected-versus-full value cohort.
   Stop affected-only value work if it is invalid, divergent, or nonpositive;
   the next priority is environment readiness and owner-visible dependency
   failure handling without CI narrowing.
4. Authorize a privacy-safe Rust estate census and economic baseline.
5. Open an advisory CI shadow only after the measured-value gate passes.
6. Define the smallest support and interop boundary, including platforms, ABI,
   ownership, panic/error, threading, allocation, debugging, deployment,
   rollback, audit, and an explicit exit path before any production commitment.
7. Fund upstream liaison and maintainer capacity for work that belongs in the
   shared Rust ecosystem.

## Public evidence

The claim-level source, scope, caveat, and presentation mapping are maintained
in [`MICROSOFT_RUST_CLAIM_LEDGER.md`](MICROSOFT_RUST_CLAIM_LEDGER.md).
The repository role dispositions and experiment stop contract are recorded in
[`MICROSOFT_RUST_LEADERSHIP_ROLE_REVIEW.md`](MICROSOFT_RUST_LEADERSHIP_ROLE_REVIEW.md).

- Stack Overflow, 2024 Developer Survey:
  <https://survey.stackoverflow.co/2024/technology#admired-and-desired>
- Rust Project, 2025 State of Rust Survey:
  <https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/>
- crates.io public summary API, observed 2026-08-11:
  <https://crates.io/api/v1/summary>
- Google Security Blog, “Eliminating Memory Safety Vulnerabilities at the
  Source,” 2024-09-25:
  <https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html>
- Microsoft Azure, “Rust as the path forward over C/C++”:
  <https://azure.microsoft.com/en-us/blog/microsoft-azure-security-evolution-embrace-secure-multitenancy-confidential-compute-and-rust/>
- Microsoft Azure SDK, stable Azure SDK for Rust:
  <https://devblogs.microsoft.com/azure-sdk/from-beta-to-stable-announcing-the-azure-sdk-for-rust-ga/>
- Microsoft OpenVMM:
  <https://github.com/microsoft/openvmm>
- Rust Foundation members:
  <https://rustfoundation.org/members/>
- GitHub Octoverse 2024:
  <https://github.blog/news-insights/octoverse/octoverse-2024/>
