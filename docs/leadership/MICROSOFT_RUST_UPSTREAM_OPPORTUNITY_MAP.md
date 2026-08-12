# Microsoft Rust Investment Map

Date: 2026-08-11  
Status: Leadership discussion draft

## Investment rule

Use one question to separate the portfolios:

> Does this capability make Rust broadly better under its existing community
> owners, or does it create differentiated Microsoft value from GitHub,
> Copilot, Azure, Windows, and enterprise governance?

Shared language, package, standards, and maintainer infrastructure belongs
upstream. Application governance and Microsoft service integration can be
differentiated, provided the underlying contracts remain portable.

## Upstream public-good portfolio

| Opportunity | Microsoft contribution | Fair-governance path | Proof of success |
|---|---|---|---|
| rustc performance and incrementality | Minimized regression fixtures, rustc-perf benchmarks, profiles, review capacity, and owner-aligned patches | rustc and rustc-perf teams | Accepted and maintained benchmarks or changes; no downstream fork |
| Cargo performance and workspace behavior | End-to-end benchmarks, freshness/fingerprinting cases, resolver diagnostics, and accepted-issue implementation | Cargo team | Accepted issue disposition and maintained tests |
| Windows/MSVC excellence | Linking, PDB/debugging, symbol, target, filesystem, process, and tool-discovery improvements | Rust compiler, Cargo, rustup, rust-analyzer, LLVM, and relevant tool owners | Reduced Windows-specific failures and supported upstream tests |
| Rust/C++ and native interoperability | Safe patterns, conformance fixtures, CXX/C ABI guidance, binding provenance, and migration examples | Existing interop projects and standards owners | Reusable upstream guidance and test suites |
| Supply-chain integrity | Trusted publishing, package provenance, advisory workflows, reproducibility evidence, and maintainer tooling | crates.io, RustSec, OpenSSF, Sigstore, and Rust Foundation | Broader ecosystem adoption and renewable evidence |
| Critical crate stewardship | Paid maintainer time, audits, release engineering, compatibility testing, and succession planning | Current crate owners and Rust Foundation funds | Maintained releases, lower owner risk, explicit succession |
| Async and concurrency diagnostics | Runtime-neutral tracing contracts, deadlock/task diagnostics, debugger support, and reproducible cases | Tokio, tracing, async ecosystem, debugger owners | Portable diagnostics with accepted owner support |
| WIT and component contracts | Native async, tooling, conformance, language projection, and secure host integration | Bytecode Alliance and WASI owners | Interoperable components and passing conformance |
| AI-assisted Rust assurance | Public fixtures for compiler-grounded generation, repair, privacy, and evaluation | Rust Project, research community, benchmark owners | Reproducible evaluations and accepted tools or guidance |
| Documentation and learning | Enterprise migration patterns, unsafe-boundary review, Windows guidance, and curriculum support | Rust Project and Foundation | Upstream-owned, maintained documentation |

## Differentiated Microsoft portfolio

| Product opportunity | Microsoft advantage | Initial product wedge | Portability requirement |
|---|---|---|---|
| Rust estate intelligence | GitHub repository graph and enterprise footprint | Discover repositories, workspaces, applications, owners, contracts, and support state | Exportable typed records; Cargo remains package authority |
| Application blueprints | GitHub, Azure, Windows, security, and deployment context | Renewable blueprint for one multi-workspace application | Provider-neutral schema and removal path |
| Copilot native-code assurance | Model capability plus GitHub change workflow | Plan, generate, explain, and validate inside approved affected scope | Compiler evidence outranks model confidence |
| Cross-repository affected work | GitHub graph, Actions, Azure DevOps, BuildXL experience | Explain global impact and emit owner-local work plans | Do not replace Cargo, CI, or local schedulers |
| Azure Rust build intelligence | Elastic compute, identity, storage, telemetry, policy | Remote analysis, evidence retention, and policy-attested validation | Local mode and direct owner-tool fallback |
| Windows Rust developer platform | OS, MSVC, debugger, SDK, driver, and tooling ownership | Supported native profile, diagnostics, interop, and deployment evidence | Upstream target improvements first where possible |
| Enterprise crate profiles | Microsoft support and security operations | Exact renewable profiles for services, CLI, identity, TLS, telemetry, and native Windows/Linux | Profiles are evidence, not certification or exclusive distribution |
| Governed MCP surface | Visual Studio, GitHub Copilot, Azure AI, and MCP investment | Read-only plan, graph, explain, query, and doctor tools | MCP is an adapter, not the trust boundary |
| Provenance and policy | GitHub attestations, Entra, Key Vault, Azure Artifacts, SFI, Defender | Trace change, dependency, build, validation, approval, and artifact evidence | Secrets excluded; connectors replaceable |
| Migration lifecycle | Large Microsoft C/C++ and polyglot estate | Boundary-led migration plans with renewal, fallback, rollback, and retirement | No forced rewrite; preserve owner authority |

## Microsoft application-blueprint minimum

Every blueprint should include:

1. application, component, service, repository, workspace, and owner identity;
2. Cargo package, target, feature, source, and toolchain truth;
3. exact supported crate/provider profiles and renewal dates;
4. Rust API, C ABI, CXX, WIT, generated binding, and wire-schema boundaries;
5. native compilers, linkers, SDKs, libraries, generators, and deployment
   prerequisites;
6. provenance, advisory, license, unsafe, FFI, build-script, macro, and
   attestation evidence;
7. build, test, fuzz, compatibility, security, and deployment validation;
8. affected-work selection with unknowns and full-reference fallback;
9. GitHub/Azure policy, approval, budget, audit, and data controls; and
10. adoption, renewal, substitution, rollback, removal, and incident paths.

## First six contribution packets

1. **Relink-Don't-Rebuild benchmark:** owner-aligned rustc-perf benchmark
   distinguishing body-only edits from interface-sensitive changes.
2. **Cargo unused-dependency matrix:** positive and negative evidence for the
   accepted Cargo `hint-mostly-unused` direction.
3. **Windows native build reliability:** reproducible cases covering MSVC
   discovery, link inputs, PDB/debug information, and filesystem behavior.
4. **Generated-boundary provenance:** reusable fixture linking schema,
   generator executable, arguments, generated Rust, native artifacts, and
   final package identity.
5. **Critical crate renewal profile:** one current-owner collaboration that
   funds maintenance and publishes version/feature/target compatibility
   evidence without creating a Microsoft fork.
6. **Compiler-grounded AI benchmark:** public tasks that measure whether an
   agent selects the correct Rust scope, preserves contracts, and produces the
   required validation and provenance.

## First Microsoft product proof

Select one bounded application with:

- at least two Cargo workspaces;
- one C/C++ or native-library boundary;
- one service or WIT contract;
- Windows and Linux targets;
- GitHub pull-request and CI workflows;
- at least one Azure integration; and
- a real owner willing to test adoption and removal.

The proof passes only if it:

- produces the same semantic plan through CLI and MCP;
- explains affected work across repositories;
- keeps Cargo and owner tools authoritative;
- attaches complete, privacy-safe evidence to a GitHub change;
- detects an unsupported or stale dependency condition;
- demonstrates a safe full-reference fallback;
- survives a profile renewal;
- rolls back and removes cleanly; and
- creates at least one useful upstream contribution packet.

## Governance guardrails

- Do not create a Microsoft Rust fork or private compatibility dialect.
- Do not equate an approved crate list with support or security.
- Do not let an AI agent narrow validation without deterministic policy.
- Do not retain credentials in plans, prompts, logs, or evidence.
- Do not claim memory safety across unsafe, native, generated, or external
  boundaries without evidence.
- Do not auto-migrate, auto-publish, or auto-deploy in the initial program.
- Do not measure success by lines rewritten.
- Do fund maintenance, review, cross-platform testing, and retirement.

