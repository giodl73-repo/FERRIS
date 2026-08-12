---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:compliance-mappings-safety-cases-and-audit-evidence
kind: guide
module: rust-security-assurance
section: security-engineering
title: Compliance Mappings, Safety Cases, and Audit Evidence
status: source-custody
source_custody: partial
current_path: rust-security-assurance/14-COMPLIANCE-MAPPINGS-SAFETY-CASES-AND-AUDIT-EVIDENCE.md
canonical_path: rust-security-assurance/14-COMPLIANCE-MAPPINGS-SAFETY-CASES-AND-AUDIT-EVIDENCE.md
backsource_ids: [mdloom-backfill:rust-security-assurance:14-compliance-mappings-safety-cases-and-audit-evidence]
concepts: [compliance, control mapping, safety case, assurance case, audit evidence]
root_concepts: [assurance case]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Compliance Mappings, Safety Cases, and Audit Evidence

Compliance frameworks specify controls and evidence expectations; they do not
automatically prove a Rust product secure or safe. An assurance case connects
product claims to arguments, assumptions, and evidence. A control mapping then
shows where those artifacts support a framework, without pretending one test or
language feature satisfies an entire requirement.

## The Big Picture

```
+============================================================================+
|                    CLAIMS, CONTROLS, AND EVIDENCE                          |
+============================================================================+
| product/security/safety claim                                              |
|        |                                                                   |
|        v                                                                   |
| structured argument -> subclaims -> assumptions -> evidence                 |
|        |                                  |                                |
|        +---------------- control mapping --+--> auditor/reviewer            |
+----------------------------------------------------------------------------+
| governance: owner | version | scope | retention | independence | expiry    |
+============================================================================+
```

The assurance case is about why the product claim is justified. The mapping is
about how organizational obligations are met. Keep both directions traceable.

## Claims-Arguments-Evidence

```
CLAIM C1
"For supported inputs and targets, the parser cannot corrupt memory and
 rejects work above the documented resource budget."
  |
  +-- ARGUMENT A1: public parser path is safe Rust
  |      +-- EVIDENCE: unsafe inventory and dependency boundary review
  |
  +-- ARGUMENT A2: unsafe/native decoder preserves validity
  |      +-- EVIDENCE: invariant review, Miri/sanitizer/fuzz results
  |
  +-- ARGUMENT A3: resource amplification is bounded
         +-- EVIDENCE: limits, load tests, corpus, operational metrics

ASSUMPTIONS: listed targets, sound toolchain/dependencies, kernel/sandbox policy
```

Use a notation such as Goal Structuring Notation if it improves review, but the
logic matters more than the diagramming syntax.

## Map Rust Evidence to Control Intent

| Engineering artifact | Can support control intent for | Must not be claimed as |
|----------------------|-------------------------------|------------------------|
| Threat model | risk assessment, secure design, change review | complete vulnerability inventory |
| `forbid(unsafe_code)`/unsafe review | secure coding, high-risk code review | proof of whole-product memory safety |
| Lockfile/SBOM/provenance | configuration, supplier, build integrity records | proof dependencies are non-malicious |
| Tests/fuzz/Miri/sanitizers | verification and defect detection | exhaustive correctness proof |
| Branch/review policy | change control and separation of duties | semantic correctness of approved change |
| Signed artifact | release integrity and authorization | product safety/security by itself |
| Incident/advisory records | response, remediation, lessons learned | prevention of all future incidents |

Framework wording and audit expectations vary by version, organization,
jurisdiction, system boundary, and assessor. Maintain mappings against the exact
edition/profile/contract in scope.

## Safety and Security Cases Interact

Safety focuses on unacceptable harm, including accidental faults; security adds
intentional adversaries who may violate environmental assumptions.

```
hazard: actuator command exceeds safe envelope
   |
   +-- accidental cause: arithmetic/sensor/state fault
   +-- adversarial cause: forged command, replay, compromised dependency
   +-- shared controls: typed state, range/interlock, independent monitor,
                       authentication, fail-safe response, audit
```

| Rust contribution | Safety/security qualification |
|-------------------|-------------------------------|
| Memory-safe subset | under sound compiler/unsafe foundations, removes classes of undefined-behavior corruption paths |
| Exhaustive enums/match | supports explicit state handling |
| Strong types/newtypes | encode units, identities, and states |
| Deterministic destruction | supports resource/lock cleanup arguments |
| No GC requirement | can improve timing predictability in some designs |
| Toolchain/ecosystem | qualification, target, compiler, unsafe/native, and timing evidence still required |

Do not claim a language or compiler is certified for a safety standard unless
the exact toolchain, usage, evidence, and assessor decision support that claim.

## Evidence Quality

| Attribute | Question |
|-----------|----------|
| Relevance | Does evidence directly support this subclaim? |
| Scope | Which code, target, feature, input, and environment? |
| Reproducibility | Can an authorized reviewer rerun or independently verify it? |
| Integrity | Is it tied to source/artifact digest and protected from alteration? |
| Independence | Was the highest-risk claim challenged by someone other than its author? |
| Freshness | What changes invalidate it? |
| Completeness | Are exclusions and negative results retained? |

Screenshots are weak primary evidence when machine-readable reports, signed
attestations, logs, configuration, and exact commands are available.

## Audit Evidence Package

```
control/claim ID
  +-- owner and system boundary
  +-- policy/procedure version
  +-- implementation/configuration
  +-- sampled execution records
  +-- exceptions and approvals
  +-- artifact/source identities
  +-- retention location and access history
```

Evidence collection should be a by-product of engineering workflows, not a
manual reconstruction before audit. Protect evidence stores from both deletion
and indiscriminate access; security artifacts can reveal architecture,
vulnerabilities, and keys or secrets if poorly sanitized.

## Bounded Compliance Guidance

Common frameworks may include NIST SSDF/CSF, ISO/IEC 27001, SOC 2 criteria,
FedRAMP/NIST SP 800-53 profiles, PCI DSS, automotive/industrial/medical safety
standards, or contractual customer controls. Applicability and interpretation
are fact-specific. Use qualified compliance, safety, privacy, and legal experts;
this guide does not provide legal advice, certification, or an authoritative
crosswalk.

## Old World -> New World Bridge

| Established artifact | Assurance-oriented form |
|----------------------|---------------------------|
| Compliance checklist | bidirectional control-to-evidence mapping with scope |
| Final Security Review packet | release assurance case tied to artifact digest |
| Safety case | security-informed hazard argument and adversarial assumptions |
| Audit screenshot | machine-readable, immutable, reproducible evidence |
| Control owner | claim owner plus evidence owner and risk acceptor |

Microsoft SDL practices, Azure Policy/Defender evidence, GitHub security
settings, and Microsoft compliance offerings can supply implementation records.
They remain evidence sources under the chosen framework and system boundary;
they do not grant automatic compliance.

## Common Confusion Points

- **"Compliant means secure."** Compliance is scoped control conformance, not a
  universal product-security verdict.
- **"Rust satisfies secure-coding controls."** It supports them; unsafe,
  dependencies, logic, build, and operations still need evidence.
- **"One artifact maps to one control."** Evidence often supports several
  controls, and one control usually needs several artifacts.
- **"Safety and security are separate."** Adversaries can trigger hazards and
  invalidate safety assumptions.
- **"A passing report remains valid."** Toolchain, target, dependency, feature,
  architecture, and policy changes can expire it.
- **"The auditor defines engineering truth."** Audit and assurance have
  different purposes; preserve technical precision while meeting evidence needs.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| New compliance framework | identify exact version, scope, owner, and authoritative interpretation |
| Existing Rust evidence | map to control intent with limits; do not relabel it as certification |
| Safety-relevant Rust component | add adversarial causes and toolchain/unsafe/platform assumptions |
| Audit request | provide digest-bound reproducible artifacts, not screenshots alone |
| Evidence reused across releases | define invalidation/freshness triggers |
| Control exception | record compensating controls, risk acceptor, expiry, and remediation |
| Ambiguous legal/regulatory duty | involve qualified specialists |

## Primary Sources

- NIST Secure Software Development Framework: https://csrc.nist.gov/Projects/ssdf
- NIST Cybersecurity Framework: https://www.nist.gov/cyberframework
- NIST SP 800-53 Rev. 5:
  https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final
- ISO/IEC 27001 overview: https://www.iso.org/standard/27001
- Goal Structuring Notation Community Standard:
  https://scsc.uk/gsn
- NIST SP 800-160 Vol. 1:
  https://csrc.nist.gov/publications/detail/sp/800-160/vol-1/rev-1/final

## Related Guides

- Previous: [13-ADVISORIES-VULNERABILITY-RESPONSE-PATCHING-AND-DISCLOSURE.md](13-ADVISORIES-VULNERABILITY-RESPONSE-PATCHING-AND-DISCLOSURE.md)
- Next: [15-SECURITY-RELEASE-GATE-AND-ASSURANCE-CASE.md](15-SECURITY-RELEASE-GATE-AND-ASSURANCE-CASE.md)
- General compliance: [../security-engineering/09-COMPLIANCE.md](../security-engineering/09-COMPLIANCE.md)
