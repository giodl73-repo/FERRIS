---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:artifact-provenance-sboms-signing-and-reproducible-evidence
kind: guide
module: rust-security-assurance
section: security-engineering
title: Artifact Provenance, SBOMs, Signing, and Reproducible Evidence
status: source-custody
source_custody: partial
current_path: rust-security-assurance/12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md
canonical_path: rust-security-assurance/12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md
backsource_ids: [mdloom-backfill:rust-security-assurance:12-artifact-provenance-sboms-signing-and-reproducible-evidence]
concepts: [provenance, SBOM, signing, reproducible builds, artifact evidence]
root_concepts: [artifact provenance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Artifact Provenance, SBOMs, Signing, and Reproducible Evidence

Release assurance must identify the exact artifact, not merely the source
repository or tag. Provenance says how and where the artifact was built; an SBOM
describes components; a signature binds an identity to bytes or an attestation;
reproducibility compares independent outputs. These controls answer different
questions and should not be collapsed into "signed build."

## The Big Picture

```
+============================================================================+
|                      ARTIFACT EVIDENCE CHAIN                               |
+============================================================================+
| source digest + lockfile + build recipe + toolchain/native inputs          |
|        |                                                                   |
|        v                                                                   |
| isolated builder identity ----> artifact bytes ----> SHA-256 digest         |
|        |                            |                 |                     |
|        +--> provenance attestation  +--> SBOM         +--> signature        |
|                                                   \                         |
| independent rebuild ------------------------------> compare/verify          |
+----------------------------------------------------------------------------+
| deployment policy verifies identity, digest, predicates, and environment   |
+============================================================================+
```

Signing unknown or weakly described bytes only makes their origin easier to
attribute.

## Distinguish the Evidence Types

| Artifact | Answers | Does not answer |
|----------|---------|-----------------|
| `Cargo.lock` | selected Cargo package sources/versions for a resolution | native/runtime components, builder integrity, publisher intent |
| SBOM | declared/discovered components and relationships in a stated scope | whether components are safe or artifact was built correctly |
| Provenance | builder, inputs, recipe, subject digest, process metadata | semantic correctness of source |
| Signature | specified identity authorized the exact bytes/statement | whether signer should have authorized them |
| Reproducible build | independent build produced equivalent bytes/result under defined process | whether source is benign |
| Advisory report | known records matched inventory at a time | absence of unknown or inapplicable defects |

## Establish Artifact Identity First

From a PowerShell release workspace after a build, hash the exact artifact:

```powershell
cargo build --locked --release
Get-FileHash .\target\release\myapp.exe -Algorithm SHA256
```

The path assumes a Windows binary package named `myapp`; use the actual target
artifact for other packages/platforms. Preserve the hash with target triple,
profile, features, `rustc -vV`, Cargo version, linker/native inputs, and source
revision.

```
release name "v1.4.0"  [human label]
git commit              [source identity]
artifact SHA-256        [byte identity]
provenance subject      [builder assertion about byte identity]
signature               [identity binding to byte/assertion]
```

Never substitute the human label for the artifact digest in a verification
policy.

## Build a Complete-enough SBOM

Cargo metadata is a strong starting point:

```text
cargo metadata --locked --format-version 1
```

An operational SBOM may also need:

- build dependencies and proc macros;
- native libraries, bundled source, system packages, and runtime image;
- statically linked components;
- compiler/linker/tool versions where the use case requires them;
- license and supplier fields with stated confidence;
- target/profile/features and relationship type.

SPDX and CycloneDX are exchange formats, not completeness guarantees.
`cargo-cyclonedx`, `cargo-sbom`, `cargo-auditable`, and similar tools evolve;
pin and validate whichever generator your organization selects. Compare output
against Cargo metadata and native/runtime inventories before relying on it.
Keep scope and relationship types explicit: a build dependency or compiler is a
build material, not necessarily a component shipped in the runtime artifact.
Some programs need separate build-material and deployable-component inventories
rather than one ambiguous list.

## Provenance and Signing Policy

```
builder emits:
  subject: artifact digest
  materials: source/lock/toolchain/base-image digests
  recipe: release command/config
  identity: workload/builder
  environment: relevant parameters

verifier checks:
  trusted builder identity
  expected repository/ref
  approved recipe/parameters
  no forbidden mutable inputs
  subject digest equals bytes being deployed
```

Keyless signing can replace long-lived key custody with short-lived identity
credentials and transparency evidence, but verification must constrain issuer,
subject/workflow identity, repository, and predicate. "Signature valid" without
identity policy is weak. Verification also needs certificate/time validity,
revocation or transparency policy as applicable, and trusted-root update
governance.

## Reproducibility and Equivalence

Rust does not promise bit-for-bit reproducible builds for every project.
Potential variability includes absolute paths, native tools, linker behavior,
timestamps/build IDs, environment, proc macros/build scripts, target features,
and embedded metadata.

| Rebuild result | Interpretation |
|----------------|----------------|
| Exact byte match | strong evidence of deterministic equivalence for those environments |
| Normalized match | useful only if normalization is specified and cannot hide meaningful differences |
| Mismatch explained | record known nondeterministic field and risk |
| Unexplained mismatch | block high-assurance release until understood |

Use independent builders or trust domains for stronger evidence. Two executions
inside the same compromised image are repetition, not independence. Compare
pre-signing artifacts or specify how signatures, timestamps, build IDs, and
other intentionally variable envelopes are separated; otherwise the signing
step itself can prevent a byte-for-byte match.

## Evidence Retention

Retain a release bundle containing:

1. artifact and digest;
2. source and lockfile identities;
3. SBOM and generator identity;
4. provenance and verification result;
5. signatures/certificates/transparency references;
6. test/security gate outputs and exceptions;
7. deployment manifest linking deployed bytes to the release;
8. retention/immutability policy and access logs.

## Old World -> New World Bridge

| Established practice | Modern/Rust evidence |
|----------------------|----------------------|
| Authenticode/package signature | artifact/attestation signature plus identity policy |
| NuGet package manifest | Cargo graph normalized into SPDX/CycloneDX |
| Build number and logs | provenance subject/materials/recipe with immutable digest |
| Rebuild from source | independent reproducibility comparison |
| CMDB/runtime inventory | SBOM plus deployed artifact and image identity |

GitHub artifact attestations and `gh attestation verify`, Azure workload
identity, Key Vault/Managed HSM, and container-registry policy can implement
parts of the chain. Verification rules must still bind the expected repository,
workflow/builder, predicate, and artifact digest.

## Common Confusion Points

- **"SBOM equals dependency lockfile."** SBOM scope can include native/runtime
  components and relationships; a lockfile is resolver input/output.
- **"Signed means safe."** It means an identity signed bytes or a statement,
  subject to key and verification policy.
- **"Provenance means reproducible."** Provenance describes a build;
  reproducibility compares builds.
- **"Same version means same artifact."** Rebuilds and targets can differ; use a
  digest.
- **"SPDX/CycloneDX guarantees completeness."** They standardize format, not
  discovery quality.
- **"Reproducible build proves source security."** It detects some build
  tampering/nondeterminism, not malicious reviewed source.

## Decision Cheat Sheet

| Need | Evidence/control |
|------|------------------|
| Identify deployed bytes | cryptographic artifact digest |
| List components | validated SBOM spanning Cargo, native, and runtime scope |
| Show how artifact was built | provenance with materials, recipe, builder, subject |
| Authorize release identity | signature/attestation plus strict verification policy |
| Detect hidden build variation | independent reproducibility comparison |
| Respond to advisory | query SBOM and deployed artifact inventory, then verify applicability |
| Audit later | immutable release bundle and retention/access policy |

## Primary Sources

- SLSA specification: https://slsa.dev/spec/
- in-toto Attestation Framework:
  https://github.com/in-toto/attestation
- SPDX specification: https://spdx.github.io/spdx-spec/
- CycloneDX specification: https://cyclonedx.org/specification/overview/
- Sigstore documentation: https://docs.sigstore.dev/
- Cargo Metadata: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- GitHub Artifact Attestations:
  https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations

## Related Guides

- Previous: [11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md](11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md)
- Next: [13-ADVISORIES-VULNERABILITY-RESPONSE-PATCHING-AND-DISCLOSURE.md](13-ADVISORIES-VULNERABILITY-RESPONSE-PATCHING-AND-DISCLOSURE.md)
- Build trust: [05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md](05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md)
