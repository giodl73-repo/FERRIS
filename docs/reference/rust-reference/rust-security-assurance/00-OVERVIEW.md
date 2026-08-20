---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:overview
kind: guide
module: rust-security-assurance
section: security-engineering
title: Rust Security Assurance - Landscape and Reading Paths
status: source-custody
source_custody: partial
current_path: rust-security-assurance/00-OVERVIEW.md
canonical_path: rust-security-assurance/00-OVERVIEW.md
backsource_ids: [proof-backfill:rust-security-assurance:00-overview]
concepts: [rust security, assurance, threat modeling, unsafe rust, supply chain, release gate]
root_concepts: [rust security assurance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Rust Security Assurance - Landscape and Reading Paths

Rust changes the security economics of systems software: under a sound compiler,
standard library, and reachable unsafe foundation, well-formed safe Rust rules
out broad classes of undefined-behavior memory corruption and data races. It
does **not** remove adversaries, logic errors, resource limits, malicious
dependencies, build-time code execution, native interfaces, kernel defects,
credential theft, or operational mistakes.
Assurance therefore asks a larger question than "is this written in Rust?":
**what claims are we making, what assumptions support them, and what evidence
justifies release?**

## The Big Picture

```
+============================================================================+
|                    RUST SECURITY ASSURANCE LANDSCAPE                       |
+============================================================================+
| CLAIM LAYER                                                                |
| assets + adversaries + trust boundaries + acceptable residual risk   [01]  |
+----------------------------------------------------------------------------+
|                                    |                                       |
|                                    v                                       |
+----------------------------------------------------------------------------+
| ENGINEERING RISK CLASSES                                                   |
| language-safe code [02] | unsafe/UB [02][03] | dependencies/build [04][05] |
| secrets/crypto [06]     | input/logic [07]    | availability [08]           |
| FFI/platform [09]       | response/compliance [13][14]                     |
+----------------------------------------------------------------------------+
|                          evidence generation                               |
|                                    |                                       |
|                                    v                                       |
+----------------------------------------------------------------------------+
| EVIDENCE                                                                   |
| tests/fuzz [10] | Miri/sanitizers/Loom/formal [11] | provenance/SBOM [12]  |
+----------------------------------------------------------------------------+
|                                    |                                       |
|                                    v                                       |
+----------------------------------------------------------------------------+
| DECISION                                                                   |
| security release gate + exceptions + residual risk + assurance case  [15]  |
+============================================================================+
```

Read downward as a lifecycle. A tool result has meaning only after the claim and
scope are known. A clean advisory scan cannot establish parser robustness; a
successful fuzz campaign cannot establish dependency provenance; safe Rust
cannot establish authorization correctness.

## The Six Risk Classes That Must Stay Separate

| Risk class | What Rust contributes | What still needs assurance |
|------------|------------------------|----------------------------|
| **Language guarantees** | Safe Rust rules out undefined behavior caused solely by well-formed safe code, subject to sound implementations beneath it; it statically prevents data races | Panic behavior, logic, authorization, confidentiality, protocol correctness |
| **Unsafe foundations** | `unsafe` localizes operations whose validity the compiler cannot prove | Invariants, aliasing, initialization, provenance, FFI contracts, sound dependencies |
| **Logic and input** | Enums, exhaustive matching, types, and explicit errors support robust designs | Business rules, parser limits, canonicalization, state-machine abuse |
| **Availability** | Ownership and `Send`/`Sync` prevent classes of concurrency corruption | Deadlock, starvation, unbounded allocation, algorithmic complexity, queue overload |
| **Supply chain** | Cargo provides explicit manifests, checksums for registry packages, and lockfiles | Publisher compromise, malicious code, build scripts, proc macros, compiler/runner trust |
| **Platform and operations** | Rust can reduce unsafe application code at the boundary | Native libraries, kernel/hypervisor, sandbox policy, credentials, deployment and incident response |

This taxonomy prevents the common but dangerous compression "memory safe =
secure." Memory safety is a major security property. It is neither the only
property nor a substitute for an assurance argument.

## Assurance Is a Chain of Scoped Claims

```
CLAIM
  "Untrusted messages cannot cause memory corruption or unbounded work."
    |
    +--> ASSUMPTIONS
    |      supported target, sound dependencies, configured limits,
    |      allocator/runtime/kernel behave within their contracts
    |
    +--> ARGUMENT
    |      safe parser shell + audited unsafe core + bounded state machine
    |
    +--> EVIDENCE
           review, tests, fuzz corpus, Miri job, dependency inventory,
           release artifact identity, operational limits
```

Evidence is plural because failure modes are heterogeneous. The right unit is a
claim-evidence pair with an owner and expiry condition, not a dashboard with an
undifferentiated green status.

## Reading Paths

| Intent | Read in order |
|--------|---------------|
| Establish a product security baseline | 01 -> 02 -> 04 -> 07 -> 08 -> 15 |
| Own a crate containing `unsafe` | 02 -> 03 -> 09 -> 10 -> 11 -> 15 |
| Secure CI and published artifacts | 04 -> 05 -> 12 -> 13 -> 15 |
| Harden a network parser or service | 01 -> 07 -> 08 -> 10 -> 11 |
| Prepare audit or regulated-release evidence | 01 -> 12 -> 14 -> 15 |
| Respond to a Rust ecosystem advisory | 04 -> 13 -> 12 -> 15 |

The module complements
[`../rust-language/`](../rust-language/00-OVERVIEW.md) for language semantics,
[`../rust-architecture/`](../rust-architecture/00-OVERVIEW.md) for compiler and
Cargo architecture, and
[`../security-engineering/`](../security-engineering/00-OVERVIEW.md) for
general security practice.

## A Minimal Assurance Loop

```
inventory -> model -> design -> implement -> challenge -> package -> decide
    ^                                                            |
    +---------------- incident/advisory feedback -----------------+
```

| Phase | Minimum durable output |
|-------|------------------------|
| Inventory | Products, crates, targets, dependencies, native components, owners |
| Model | Assets, attacker goals, boundaries, abuse cases, risk acceptance criteria |
| Design | Enforced invariants, limits, cryptographic protocols, containment strategy |
| Challenge | Review records, tests, fuzzing, targeted dynamic or formal evidence |
| Package | Lockfile, source identity, SBOM/provenance, signatures, build logs |
| Decide | Release verdict, exceptions, residual risk, response readiness |

## Old World -> New World Bridge

| Established practice | Rust assurance expression |
|----------------------|----------------------------|
| C/C++ secure coding plus memory diagnostics | Safe Rust by default, with explicit unsafe obligations and targeted diagnostics |
| Static-analysis gate | Layered evidence: compiler guarantees, Clippy, tests, fuzzing, Miri/sanitizers where applicable |
| Bill of materials for deployables | Cargo graph plus native/runtime/build inputs, normalized into an SBOM |
| Final Security Review | Claim-oriented release gate with evidence freshness and explicit exceptions |
| Code-signing ceremony | Verification policy linking source, builder, artifact digest, signer, and deployment identity |

The universal bridge is from **control checklists to assurance cases**: controls
say what activities occurred; an assurance case explains why those activities
support a specific claim. For Microsoft-oriented teams, SDL threat modeling,
GitHub Advanced Security, Defender for Cloud, Azure Key Vault, and MSRC-style
response processes can supply pieces of evidence. They remain supplemental; no
vendor product establishes the whole case.

## Common Confusion Points

- **"No `unsafe` in our crate means no unsafe risk."** Transitive dependencies,
  the standard library, native code, and the platform still contain trusted
  unsafe foundations.
- **"Cargo.lock makes the supply chain safe."** It improves reproducibility and
  source selection; it does not prove publisher intent or build-host integrity.
- **"Miri passed, so the unsafe code is sound."** Miri checks executed paths
  under its current model. Soundness is a universal claim.
- **"No advisory means no vulnerability."** Advisories are lagging,
  coverage-dependent signals.
- **"Compliance evidence is security evidence."** It may support a claim, but
  control conformance and product security are not identical.
- **"Microsoft guidance should define the baseline."** Start with universal
  threat, protocol, and evidence principles; add ecosystem integrations only
  where they improve implementation or operations.

## Decision Cheat Sheet

| If the immediate question is... | Start with |
|---------------------------------|------------|
| What can an attacker reach or steal? | [01](01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md) |
| What does safe Rust actually guarantee? | [02](02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md) |
| Could this unsafe operation create UB? | [03](03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md) |
| Can we trust the crate graph and build? | [04](04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md), [05](05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md) |
| Are secrets and cryptographic choices defensible? | [06](06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md) |
| Can hostile inputs or load break the service? | [07](07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md), [08](08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md) |
| What evidence should CI generate? | [10](10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md) through [12](12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md) |
| Should this release ship? | [15](15-SECURITY-RELEASE-GATE-AND-ASSURANCE-CASE.md) |

## Primary Sources

- Rust Security Response Working Group: https://www.rust-lang.org/policies/security
- The Rust Reference: https://doc.rust-lang.org/reference/
- The Rustonomicon: https://doc.rust-lang.org/nomicon/
- Cargo Book: https://doc.rust-lang.org/cargo/
- NIST Secure Software Development Framework: https://csrc.nist.gov/Projects/ssdf
- OpenSSF Best Practices: https://bestpractices.coreinfrastructure.org/
- SLSA specification: https://slsa.dev/spec/

## Related Guides

- Next: [01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md](01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md)
- Language model: [../rust-language/00-OVERVIEW.md](../rust-language/00-OVERVIEW.md)
- Toolchain architecture: [../rust-architecture/00-OVERVIEW.md](../rust-architecture/00-OVERVIEW.md)
- Module status: [STATUS.md](STATUS.md)
