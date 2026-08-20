---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:security-release-gate-and-assurance-case
kind: guide
module: rust-security-assurance
section: security-engineering
title: Security Release Gate and Assurance Case
status: source-custody
source_custody: partial
current_path: rust-security-assurance/15-SECURITY-RELEASE-GATE-AND-ASSURANCE-CASE.md
canonical_path: rust-security-assurance/15-SECURITY-RELEASE-GATE-AND-ASSURANCE-CASE.md
backsource_ids: [proof-backfill:rust-security-assurance:15-security-release-gate-and-assurance-case]
concepts: [security release gate, assurance case, residual risk, release evidence]
root_concepts: [security release gate]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Security Release Gate and Assurance Case

The final gate decides whether a specific artifact may enter a specific
environment under stated residual risk. It does not ask whether "Rust is safe."
It integrates separate claims for language guarantees, unsafe foundations,
logic and input, availability, supply chain, native/platform boundaries,
operations, and response readiness.

## The Big Picture

```
+============================================================================+
|                       SECURITY RELEASE GATE                                |
+============================================================================+
| scope: source + lock + target/features + builder + artifact digest + env   |
+----------------------------------------------------------------------------+
| 1 threat claims [01]       5 availability [08]                             |
| 2 safe/unsafe [02][03]     6 FFI/platform [09]                             |
| 3 supply/build [04][05]    7 challenge evidence [10][11]                   |
| 4 secrets/input [06][07]   8 provenance/response/compliance [12][13][14]   |
+----------------------------------------------------------------------------+
| evidence current? -> blockers? -> exceptions? -> residual risk acceptable? |
+----------------------------------------------------------------------------+
| PASS / PASS-WITH-EXCEPTION / BLOCK / NOT-APPLICABLE-WITH-RATIONALE         |
+============================================================================+
```

Every verdict is bound to an artifact digest and deployment scope. A branch,
tag, or successful pipeline is not the release subject.

## 1. Freeze the Decision Scope

Record:

- source revision and repository identity;
- `Cargo.lock` digest and dependency sources;
- exact Rust/Cargo toolchain, target triples, profiles, and features;
- native libraries, linker/sysroot, runtime/base image;
- builder/workflow identity and configuration;
- output artifact/SBOM/provenance digests;
- intended environment, privileges, data class, and rollout population.

```
same source
  +-- different feature ------> different attack surface
  +-- different target -------> different std/native/platform assumptions
  +-- different builder ------> different provenance
  +-- different config -------> different security behavior
```

## 2. Use a Risk-Class Gate Matrix

| Gate | Required claim | Typical evidence | Block examples |
|------|----------------|------------------|----------------|
| Threat model | material assets/boundaries/abuse cases are current | model, change review, owners | new privileged flow unmodeled |
| Safe/unsafe | unsafe surface has accountable soundness argument | inventory, invariant review, Miri/tests | undocumented unsafe or soundness finding |
| Logic/input | hostile input cannot bypass semantics/authz within scope | validation design, abuse tests, fuzz corpus | known authz bypass or unbounded expansion |
| Availability | resource use and recovery meet budgets | load/failure tests, limits, telemetry | unbounded queue or retry storm |
| Supply/build | resolved sources and host build code meet policy | lock/graph review, isolated build logs | unapproved source/proc macro; mutable build input |
| Secrets/crypto | key/protocol lifecycle meets approved standard | design review, identity/key policy, rotation test | embedded secret; nonce/key reuse risk |
| FFI/platform | ABI/native/containment assumptions are validated | target tests, native SBOM, sandbox tests | ownership ambiguity; excessive privilege |
| Evidence/artifact | deployed bytes trace to reviewed inputs | digest, SBOM, provenance, signature | missing or unverifiable artifact identity |
| Response | owners can detect, patch, rotate, and notify | runbook/exercise, contact, rollback | no security contact or viable remediation path |
| Compliance/safety | required scoped obligations are mapped | approved mapping/case/evidence | mandatory evidence or assessor gate absent |

Not every release needs every expensive tool. Every row needs a status and a
rationale proportionate to risk.
The verdict vocabulary is organizational policy, not a regulatory escape hatch:
some safety, customer, or compliance gates permit no exception for specified
criteria. Map `PASS-WITH-EXCEPTION` to the actual authority and obligations in
scope before using it.

## 3. Run Reproducible Baseline Checks

This conservative workspace baseline assumes a pinned stable toolchain with
`rustfmt` and Clippy installed, a committed lockfile, execution from the
workspace root, and a project intended to support the selected target/feature
combination:

```text
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets --all-features
cargo tree --locked --workspace --duplicates
```

`--all-targets` means Cargo target kinds for the current compilation target; it
does not test every target triple. `--all-features` can create combinations the
product never ships or miss mutually exclusive operational profiles. Replace or
supplement both with the actual release matrix. `cargo fmt` is consistency
evidence, not a security control; Clippy and tests catch selected defects, not
all vulnerabilities.

For crates with material unsafe code, a dated-nightly Miri job with the Miri
component installed and set up as described in [11](11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md)
may add:

```text
cargo +nightly miri test
```

Record the exact nightly and limitations. Advisory, fuzz, sanitizer, Loom, SBOM,
and provenance jobs belong in the matrix only with similarly explicit scope.

## 4. Write the Assurance Case

```
TOP CLAIM
"Artifact H may be released to Environment E with residual risk R."
  |
  +-- C1 memory/data-race claim
  |     safe-language argument + unsafe/dependency/platform assumptions
  |
  +-- C2 logic/confidentiality/integrity claim
  |     protocol, validation, authorization, crypto evidence
  |
  +-- C3 availability claim
  |     resource budgets, load/failure evidence, recovery
  |
  +-- C4 supply/artifact claim
  |     source/build/provenance/SBOM/signature evidence
  |
  +-- C5 operational claim
        monitoring, rollback, patching, disclosure, compliance/safety duties
```

This separation is mandatory. Safe Rust materially strengthens C1; it does not
automatically satisfy C2 through C5.

## 5. Handle Exceptions as Expiring Risk Decisions

| Exception field | Required content |
|-----------------|------------------|
| Failed/missing criterion | exact gate and evidence gap |
| Exposure | affected artifacts/environments/users |
| Compensating control | implemented and validated mitigation |
| Residual risk | consequence and likelihood under current assumptions |
| Risk acceptor | named authority independent enough for consequence |
| Expiry | date/event and automatic re-block behavior |
| Remediation | owner, work item, target release |
| Monitoring | signal that detects assumption failure |

Do not let the engineer who introduced a high-consequence risk be the sole
acceptor. Exceptions are not green checks; surface them in the release record.

## 6. Stage Rollout and Verify Reality

```
signed digest -> canary -> telemetry/security checks -> staged population
                    |                 |
                    +-- rollback -----+
```

Verify that the deployed digest equals the gated digest, runtime policy matches
the reviewed environment, secrets/identities are the intended ones, and
telemetry can identify regressions or exploitation. A perfect pre-release packet
cannot compensate for deploying different bytes.

## Microsoft Ecosystem Supplement

Teams may implement the gate with GitHub protected environments and artifact
attestations, Azure DevOps approvals, Defender/GitHub security findings, Azure
Key Vault/Managed HSM, Azure Policy, and an SDL Final Security Review or
MSRC-aligned response process. Keep the assurance matrix tool-neutral so the
same claims survive platform migration and independent audit.

## Old World -> New World Bridge

| Established release practice | Assurance-case form |
|------------------------------|---------------------|
| Ship-room checklist | claim/evidence matrix tied to artifact digest |
| Final Security Review | risk-class gate with explicit residual-risk owner |
| Code-signing approval | signer verifies provenance and exact subject digest |
| Go/no-go meeting | durable PASS/EXCEPTION/BLOCK verdict with rationale |
| Post-deploy validation | digest/policy/telemetry verification and staged rollout |

The conceptual shift is from **activity completion** to **claim sufficiency**.
The checklist still matters, but every item must say what claim it supports and
when that evidence expires.

## Common Confusion Points

- **"All CI checks passed, so ship."** CI may omit threat, artifact, operational,
  or environment claims.
- **"Safe Rust is the memory-safety evidence."** It is a central argument whose
  assumptions include the compiler and sound reachable unsafe, native, and
  platform foundations.
- **"Not applicable means skip."** It requires a reviewed rationale bound to
  scope.
- **"A waiver is a pass."** It is an explicit residual-risk acceptance with
  expiry.
- **"Signed tag identifies deployed bytes."** Verify the artifact digest and
  provenance subject.
- **"Compliance approval is the security verdict."** It is one scoped input to
  the release decision.
- **"Release is the end of assurance."** Monitoring, advisory response,
  rollback, and evidence renewal continue.

## Decision Cheat Sheet

| Gate condition | Verdict |
|----------------|---------|
| Required claim has current sufficient evidence; no blocker | PASS |
| Evidence gap has bounded exposure, validated control, owner, acceptor, expiry | PASS-WITH-EXCEPTION |
| Known exploitable defect, soundness issue, untrusted artifact/build, or unacceptable residual risk | BLOCK |
| Criterion truly outside release scope with documented rationale | NOT-APPLICABLE-WITH-RATIONALE |
| Deployed digest differs from gated subject | BLOCK/ROLLBACK |
| Assumption expires after release | reopen gate and regenerate affected evidence |

## Primary Sources

- NIST Secure Software Development Framework: https://csrc.nist.gov/Projects/ssdf
- SLSA specification: https://slsa.dev/spec/
- NIST SP 800-160 Vol. 1:
  https://csrc.nist.gov/publications/detail/sp/800-160/vol-1/rev-1/final
- Rust Security Policy: https://www.rust-lang.org/policies/security
- Cargo Book: https://doc.rust-lang.org/cargo/
- GitHub Artifact Attestations:
  https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations
- Microsoft Security Development Lifecycle:
  https://www.microsoft.com/sdl

## Related Guides

- Previous: [14-COMPLIANCE-MAPPINGS-SAFETY-CASES-AND-AUDIT-EVIDENCE.md](14-COMPLIANCE-MAPPINGS-SAFETY-CASES-AND-AUDIT-EVIDENCE.md)
- Start of module: [00-OVERVIEW.md](00-OVERVIEW.md)
- Threat model: [01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md](01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md)
- Module status: [STATUS.md](STATUS.md)
