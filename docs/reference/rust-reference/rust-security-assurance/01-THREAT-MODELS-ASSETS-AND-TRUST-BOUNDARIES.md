---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:threat-models-assets-and-trust-boundaries
kind: guide
module: rust-security-assurance
section: security-engineering
title: Threat Models, Assets, and Trust Boundaries
status: source-custody
source_custody: partial
current_path: rust-security-assurance/01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md
canonical_path: rust-security-assurance/01-THREAT-MODELS-ASSETS-AND-TRUST-BOUNDARIES.md
backsource_ids: [proof-backfill:rust-security-assurance:01-threat-models-assets-and-trust-boundaries]
concepts: [threat modeling, assets, trust boundaries, abuse cases, attack surface]
root_concepts: [threat modeling]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Threat Models, Assets, and Trust Boundaries

A Rust threat model begins with product behavior, not with the borrow checker.
The useful output names valuable assets, attacker capabilities, boundary
crossings, unacceptable outcomes, and the owners of mitigations. Language
choice affects some attack paths; it does not define the adversary.

## The Big Picture

```
+============================================================================+
|                         THREAT-MODEL DATA FLOW                             |
+============================================================================+
| untrusted client -> parser -> domain logic -> privileged action            |
|       |              |           |                 |                       |
|       |              |           |                 +--> keys / money / ACL |
|       |              |           +--> authorization + state invariants     |
|       |              +--> CPU, memory, recursion, unsafe/native codecs     |
|       +--> spoofing, replay, malformed input, load, protocol sequencing    |
+----------------------------------------------------------------------------+
| BUILD boundary: registry -> Cargo -> build.rs/proc macro -> compiler       |
| HOST boundary: process -> libc/native library -> kernel/hypervisor         |
| OPS boundary: artifact -> signer -> registry -> deployer -> runtime config |
+============================================================================+
```

Every arrow is a place where identity, representation, authority, or resource
ownership changes. Mark those transitions before selecting a taxonomy.

## 1. Inventory Assets and Security Properties

An asset is not merely stored data. Include authority, continuity, and evidence.

| Asset | Required property | Example failure |
|-------|-------------------|-----------------|
| Signing key | confidentiality, controlled use, rotation | attacker signs a trojan release |
| Authorization decision | integrity, freshness, context binding | confused deputy grants another tenant's access |
| Service capacity | availability, fair allocation | one request monopolizes workers or memory |
| Audit record | integrity, ordering, retention | attacker erases evidence or injects false events |
| Build recipe and source identity | integrity, traceability | artifact cannot be tied to reviewed source |
| Unsafe invariant | validity across all safe call paths | safe caller triggers UB through a soundness hole |

Write unacceptable outcomes in product language: "read another tenant's
records," "forge an update," "make recovery exceed four hours." "Buffer
overflow" is a technique, not a business consequence.

## 2. State the Adversary and Assumptions

```
ATTACKER CAPABILITY LADDER

remote unauthenticated
        |
        v
authenticated low privilege
        |
        v
malicious tenant / protocol peer
        |
        v
dependency publisher or build-input attacker
        |
        v
local process / container escape attempt
        |
        v
administrator, CI runner, or signing-system compromise
```

Do not silently jump between rungs. A sandbox may be a useful mitigation against
a malicious parser input but irrelevant if the attacker controls the signer.
Record assumptions such as "kernel trusted," "registry index available over
authenticated transport," or "operator can rotate keys within one hour."

## 3. Draw Trust Boundaries, Including Build Time

| Boundary | Questions |
|----------|-----------|
| Network -> parser | Is length bounded before allocation? Are framing and timeouts explicit? |
| Parser -> domain object | Does syntactic validity imply semantic authorization? Usually not. |
| Safe API -> unsafe implementation | Can any safe input violate the documented invariant? |
| Crate -> dependency | Who can publish, what features activate, what code runs at build time? |
| Rust -> native ABI | Who owns memory, which layout/ABI applies, may unwinding cross? |
| Process -> OS service | Which syscalls, files, devices, tokens, and namespaces are reachable? |
| CI -> signing | Can a build job request a production signature without independent policy? |

From a workspace root with a supported Cargo installation, these read-only
commands help inventory the Rust graph:

```text
cargo metadata --locked --format-version 1
cargo tree --locked --workspace --all-features
cargo tree --locked --workspace --target all
```

`--locked` makes the command fail rather than update `Cargo.lock`. For
`cargo tree`, `--target all` is a conservative display mode and may show
dependencies not built by a particular release target; it does not compile or
test every target. Preserve the actual target/feature matrix separately.

## 4. Turn Diagrams into Abuse Cases

```
GOAL: execute an unauthorized privileged operation
  |
  +-- forge or replay identity evidence
  +-- exploit parser disagreement / canonicalization gap
  +-- confuse tenant or object identity after validation
  +-- induce stale authorization-cache use
  +-- compromise dependency or build-time code
  +-- exploit unsafe/native boundary to alter control flow
```

Use STRIDE, attack trees, misuse cases, or another taxonomy as a completeness
prompt. The deliverable is not the acronym; it is a prioritized set of abuse
cases tied to design decisions and tests.

| Abuse case | Prevent | Detect | Recover |
|------------|---------|--------|---------|
| Oversized frame allocates unbounded memory | pre-allocation length cap | rejected-size metric | shed load; restart within budget |
| Replayed signed request | nonce/timestamp and audience binding | replay counter | revoke credential; tighten window |
| Malicious proc macro exfiltrates CI secret | isolated build and no secret in build job | egress/audit alert | rotate secret; rebuild from known inputs |
| Safe API reaches invalid native pointer | ownership/lifetime design and review | Miri/sanitizer on supported paths | treat as security incident; patch dependents |

## 5. Define Reviewable Outputs

A useful model has versioned artifacts:

1. system/context diagram and data flows;
2. asset and attacker table;
3. trust assumptions and explicitly untrusted components;
4. prioritized abuse cases with security properties;
5. mitigations, owners, verification method, and residual risk;
6. change triggers: new target, parser, privilege, dependency source, or
   deployment boundary.

## Old World -> New World Bridge

| Established practice | Rust-specific extension |
|----------------------|-------------------------|
| Data-flow diagram | Add Cargo/build-time and unsafe/native flows |
| STRIDE worksheet | Pair each category with concrete Rust entry points and abuse tests |
| Attack-surface inventory | Include features, targets, proc macros, `build.rs`, FFI, and syscalls |
| Security requirement | Express as invariant plus measurable resource/response bound |
| Final threat-model review | Re-open on lockfile, target, feature, privilege, or protocol changes |

Microsoft's STRIDE and SDL threat-modeling practices are useful supplemental
workflows, especially for teams already using them. They are not the only valid
taxonomy, and completing a template is not evidence that a mitigation works.

## Common Confusion Points

- **"Rust removes memory threats from the diagram."** Safe Rust narrows them;
  unsafe code, native libraries, kernels, and soundness defects remain.
- **"The dependency graph is not part of runtime threat modeling."** Build-time
  compromise changes runtime artifacts and must be modeled.
- **"Authentication is the trust boundary."** Identity establishment,
  authorization, tenant binding, and object selection are separate transitions.
- **"STRIDE scores risk."** STRIDE prompts categories; prioritize with stated
  likelihood/impact criteria suited to the product.
- **"A DFD is the model."** The model also needs adversaries, outcomes,
  assumptions, mitigations, owners, and validation evidence.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| New service or protocol | Draw end-to-end data and authority flows before coding |
| Existing crate gains `unsafe` | Add a safe/unsafe boundary and invariant-abuse cases |
| New dependency or feature | Recompute build/runtime boundaries and publisher trust |
| Cross-compilation or new target | Revisit native libraries, linker, runner, and platform assumptions |
| Multi-tenant authorization change | Model tenant/object identity from parse through storage |
| CI gains signing access | Separate build identity from signing authorization and model compromise |
| Model has no owners or tests | Treat it as unfinished |

## Primary Sources

- NIST SP 800-154, Guide to Data-Centric System Threat Modeling:
  https://csrc.nist.gov/publications/detail/sp/800-154/draft
- NIST Secure Software Development Framework: https://csrc.nist.gov/Projects/ssdf
- Microsoft Threat Modeling documentation:
  https://learn.microsoft.com/azure/security/develop/threat-modeling-tool
- OWASP Threat Modeling: https://owasp.org/www-community/Threat_Modeling
- Cargo Metadata: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- Cargo Tree: https://doc.rust-lang.org/cargo/commands/cargo-tree.html

## Related Guides

- Previous: [00-OVERVIEW.md](00-OVERVIEW.md)
- Next: [02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md](02-SAFE-RUST-UNSAFE-OBLIGATIONS-AND-ASSURANCE-OWNERSHIP.md)
- General threat modeling: [../security-engineering/01-THREAT-MODELING.md](../security-engineering/01-THREAT-MODELING.md)
