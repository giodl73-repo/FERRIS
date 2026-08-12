# rust-security-assurance/ - Status

**17 files (STATUS.md + 16 canonical guides) | Complete | Source-first**

This module treats Rust security as an assurance problem, not a language slogan.
Safe Rust provides strong memory-safety and data-race guarantees within the
language model, but a product can still fail through unsound dependencies,
incorrect `unsafe`, authorization or business-logic flaws, resource exhaustion,
malicious build inputs, vulnerable native code, compromised platforms, or weak
operations. The guides keep those risk classes separate and join them only in
the final release assurance case.

## The Big Picture

```
+============================================================================+
|                    RUST SECURITY ASSURANCE STATUS                          |
+============================================================================+
| 16 canonical guides                                                       |
|        |                                                                   |
|        +--> claims and threat boundaries                         [00-03]    |
|        +--> supply, build, secrets, input, availability          [04-08]    |
|        +--> FFI, adversarial evidence, provenance                [09-12]    |
|        +--> response, compliance, integrated release gate        [13-15]    |
|                                                                            |
| custody: canonical source / partial backsource                             |
| maturity: independently corrected; no Certified Gold claim                 |
+============================================================================+
```

## Guides

| File | Topic | Status |
|------|-------|--------|
| `00-OVERVIEW.md` | Security landscape, risk classes, assurance lifecycle, and reading paths | done |
| `01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md` | Threat-model inputs, asset inventories, boundaries, abuse cases, and review outputs | done |
| `02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md` | Language guarantees, unsafe proof obligations, dependency trust, and accountable ownership | done |
| `03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md` | Abstract-machine rules, validity, provenance, aliasing, initialization, and UB review | done |
| `04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md` | Cargo resolution, lockfiles, registries, source identity, policy, and dependency review | done |
| `05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md` | Host-side build code, compiler/toolchain trust, isolation, and build-policy evidence | done |
| `06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md` | Secret lifecycle, established protocols, CSPRNG use, key custody, and rotation | done |
| `07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md` | Untrusted bytes, parser limits, semantic validation, canonicalization, and protocol state | done |
| `08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md` | Race freedom versus liveness, bounded work, backpressure, cancellation, and DoS budgets | done |
| `09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md` | ABI contracts, ownership, native dependencies, syscall exposure, and containment | done |
| `10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md` | Target design, properties, fuzz campaigns, corpus governance, and reproducible triage | done |
| `11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md` | Evidence from dynamic semantics, sanitizers, concurrency exploration, and bounded proofs | done |
| `12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md` | Build identity, attestations, SBOM scope, signing, verification, and reproducibility | done |
| `13-ADVISORIES-VULNERABILITY-RESPONSE-PATCHING-AND-DISCLOSURE.md` | Intake, applicability, severity, remediation, coordinated disclosure, and learning | done |
| `14-COMPLIANCE-MAPPINGS-SAFETY-CASES-AND-AUDIT-EVIDENCE.md` | Bounded control mappings, claims-arguments-evidence, safety interfaces, and audit records | done |
| `15-SECURITY-RELEASE-GATE-AND-ASSURANCE-CASE.md` | Risk-based release gate, exceptions, residual risk, and integrated assurance case | done |

## Quality and Custody Notes

- Every canonical guide uses `maxim.frontmatter.v1`, module
  `rust-security-assurance`, `status: source-custody`, and
  `source_custody: partial`.
- Every guide implements the seven MAXIM surfaces: a Big Picture ASCII map,
  layered drill-down, additional ASCII structure, comparison/decision tables,
  an old-world/new-world bridge, Common Confusion Points, and a Decision Cheat
  Sheet.
- Universal engineering guidance is primary. Microsoft SDL, MSRC, Azure,
  Defender, and GitHub security features appear only as supplemental examples.
- Commands state their workspace, toolchain, target, or operating assumptions.
  Nightly, target-dependent, and third-party tooling is labeled rather than
  presented as a stable language guarantee.
- Legal, regulatory, and compliance material is framed as engineering support,
  not legal advice or automatic certification.
- Primary sources favor official Rust, Cargo, NIST, CISA, OpenSSF, SLSA, SPDX,
  CycloneDX, IETF, and vendor documentation. No third-party tutorial is
  load-bearing.
- Source custody is intentionally partial: no source-backfill, generated
  artifacts, navigation, registry, or TRACKER files were changed.

## Independent Cross-Review

The 2026-08-11 independent pass corrected conditional safe-Rust guarantees,
memory-model authority language, Cargo workspace and vendoring commands,
tool-install pinning, CSPRNG/password/side-channel guidance, parser overflow and
slow-input caveats, collision/queue/OOM availability claims, FFI layout and
unwind examples, fuzzing scope, Miri scheduling limits, SBOM scope, signing and
reproducibility qualifications, advisory handling, and bounded release/
compliance language.

MDLOOM check result: `17 files checked, 0 errors, 0 warnings`.

## Four-Role Review

| Role | Resolved quality question |
|------|---------------------------|
| Reader Path Editor | Overview supplies task-oriented reading paths; each guide links to prerequisites and next evidence. |
| Reference Integrity Auditor | Claims distinguish stable guarantees from tool evidence, platform behavior, implementation models, and policy choices. |
| Executable Evidence Auditor | Commands are scoped, copyable, and accompanied by limits; passing a tool never becomes a proof claim. |
| Learner Advocate | Openings explain operational value, jargon is contextual, and decision sheets support real engineering choices. |

No inline editorial tags or unresolved review findings remain. The module is
complete as canonical source content, but it is not claimed as Certified Gold
and has not been run through source-backfill validation.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Enter the module by task | Reading paths in `00-OVERVIEW.md` |
| Review unsafe or UB claims | Guides 02, 03, 09, and 11 |
| Review dependency/build trust | Guides 04, 05, and 12 |
| Review hostile input and availability | Guides 07, 08, and 10 |
| Prepare response or audit evidence | Guides 13 and 14 |
| Make a release decision | Guide 15 with the exact artifact and environment |
| Regenerate derived artifacts | Use a later explicitly scoped backfill task |
