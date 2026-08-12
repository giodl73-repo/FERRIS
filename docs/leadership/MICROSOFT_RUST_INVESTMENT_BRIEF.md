# Microsoft Rust: Govern the Conversion While the Estate Is Still Forming

Date: 2026-08-11  
Audience: Microsoft engineering leaders, developer-platform leaders, security
leaders, and prospective sponsors  
Status: Leadership discussion draft

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

| Signal | Public evidence | Strategic meaning |
|---|---|---|
| Developer pull | Rust was the most admired programming language in Stack Overflow's 2024 survey, at approximately 83%. | Recruiting and internal advocacy start with unusually strong developer preference. |
| Organizational use | The 2025 State of Rust survey reports 48.8% of responding organizations making non-trivial use of Rust, continuing a multi-year rise. | Rust is becoming an organizational capability, not only an individual experiment. |
| Ecosystem scale | The crates.io public API reported 314,949 crates and 395.5 billion cumulative downloads on 2026-08-11. | Package selection, provenance, maintenance, and compatibility are now portfolio-scale concerns. |
| Security outcome | Google reported Android memory-safety vulnerabilities falling from 76% to 24% over six years as new development shifted toward memory-safe languages. Rust changes also had less than half the rollback rate of C++ changes. | Incremental safe-language adoption can improve a large existing estate without rewriting it all. |
| Microsoft intent | Azure states that Rust has already been adopted in critical infrastructure components and that adoption is expected to expand substantially. | The internal coordination need is already present. |
| Microsoft product maturity | The Azure SDK for Rust is stable, with stable core, identity, Key Vault, and Storage libraries and SemVer guarantees. | Microsoft is creating a credible first-party Rust application surface. |
| Microsoft systems proof | OpenVMM is a Microsoft open-source, modular, cross-platform VMM written in Rust. | Rust is already viable in demanding Microsoft systems contexts. |
| Ecosystem stewardship | Microsoft joined the Rust Foundation as a founding Platinum member in January 2021. | Microsoft has an established fair-governance channel for community investment. |

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

The value is not “Copilot writes Rust.” The value is **Copilot changes native
systems with compiler-grounded, application-aware evidence**.

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

Ferris already demonstrates a disciplined pattern:

- use Cargo as the authority for package graph truth;
- add application and cross-workspace planning above Cargo;
- preserve separate identities for selection, invocation, evidence, and
  result;
- emit explainable, machine-readable outcomes;
- keep observation separate from execution authority;
- validate on Windows and Linux;
- retain privacy-safe, bounded evidence; and
- require specification, simulation, review, and held-out evaluation before
  expanding authority.

The next Ferris phase should prove one portfolio slice:

1. inventory a bounded Microsoft-like multi-repository Rust application;
2. publish one renewable application blueprint;
3. generate one cross-workspace affected plan;
4. expose the same read-only semantics through CLI and MCP;
5. attach provenance and validation evidence in GitHub;
6. demonstrate adoption, renewal, rollback, and removal; and
7. submit one independently useful upstream contribution packet.

## A phased investment

| Phase | Duration | Outcome |
|---|---:|---|
| 0. Sponsor and discover | 0–90 days | Executive sponsor, cross-company working group, Rust estate census, candidate application, upstream liaison, and baseline measures |
| 1. Blueprint proof | 3–6 months | One application blueprint, supported profile, contract map, affected-work plan, GitHub evidence flow, Windows/Linux proof, and removal exercise |
| 2. Platform pilot | 6–12 months | Multiple organizations, read-only MCP, Azure/GitHub connectors, Copilot assurance workflow, portfolio dashboard, and first accepted upstream contributions |
| 3. Productize selectively | 12–24 months | Supported GitHub/Azure/Windows capabilities, renewable profiles, service ownership, SLAs, and measurable migration/security outcomes |

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

1. Sponsor a six-month, cross-company Microsoft Rust application-platform
   proof rather than another isolated tooling experiment.
2. Name one Windows/Azure systems application and one GitHub/Copilot workflow
   as bounded design partners.
3. Fund dedicated upstream liaison and maintainer capacity from the start.
4. Authorize a privacy-safe Rust estate census and application-boundary map.
5. Require the proof to demonstrate interoperability, support renewal,
   rollback, removal, Windows/Linux operation, and measurable user value.
6. Keep product differentiation in GitHub, Copilot, Azure, Windows, security,
   and application governance while keeping shared language infrastructure
   upstream and foundation-governed.

## Public evidence

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

