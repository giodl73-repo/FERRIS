---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:build-scripts-proc-macros-compilers-and-build-trust
kind: guide
module: rust-security-assurance
section: security-engineering
title: Build Scripts, Proc Macros, Compilers, and Build Trust
status: source-custody
source_custody: partial
current_path: rust-security-assurance/05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md
canonical_path: rust-security-assurance/05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md
backsource_ids: [proof-backfill:rust-security-assurance:05-build-scripts-proc-macros-compilers-and-build-trust]
concepts: [build scripts, proc macros, compiler trust, cargo build, build isolation]
root_concepts: [build trust]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Build Scripts, Proc Macros, Compilers, and Build Trust

`build.rs` programs and procedural macros are dependencies that execute on the
build host. They are not inert source libraries. The compiler, linker, native
toolchain, CI runner, and environment can also affect the artifact. Build
assurance therefore treats compilation as privileged code execution with a
traceable input/output boundary.

## The Big Picture

```
+============================================================================+
|                          TRUSTED BUILD ENVELOPE                            |
+============================================================================+
| reviewed source + lockfile + config + toolchain + native inputs            |
|       |              |          |           |                              |
|       v              v          v           v                              |
| Cargo unit graph -> build.rs ----+-> rustc <- proc-macro dylibs            |
|                     host process |    |                                     |
|                                  +--> linker/native tools                   |
|                                         |                                  |
|                                         v                                  |
|                                  artifact + logs + provenance              |
+----------------------------------------------------------------------------+
| runner identity | filesystem | environment/secrets | network | clock/cache |
+============================================================================+
```

The boundary is only as strong as the least controlled input. A reviewed source
tree built on a mutable, secret-rich runner is not a reproducible trust claim.

## Host Code: Build Scripts and Proc Macros

| Unit | How it executes | Typical authority |
|------|-----------------|-------------------|
| `build.rs` | Cargo compiles and runs a host executable | read env/files, run tools, write `OUT_DIR`, emit Cargo directives; OS policy may allow more |
| Proc macro | rustc loads/executes host-side macro code during expansion | inspect/generate tokens; process still inherits host access allowed by OS |
| Build dependency | linked into build script or proc macro | same effective host privilege as its caller |
| Native generator | spawned by build script | arbitrary behavior within runner controls |

Treat all of them as untrusted until policy admits them. Cargo standardizes how
outputs influence compilation; it does not sandbox the process.

## Pin the Toolchain and Record the Target

A repository-level `rust-toolchain.toml` can select a concrete toolchain:

```toml
[toolchain]
channel = "1.85.0"
profile = "minimal"
components = ["clippy", "rustfmt"]
```

The version is an illustrative known release, not a recommendation to remain on
it; security support requires a reviewed update cadence. With rustup installed,
these commands are executable from the workspace root:

```text
rustup toolchain install 1.85.0 --profile minimal --component clippy,rustfmt
cargo +1.85.0 build --locked
rustc +1.85.0 -vV
```

Record `rustc -vV`, Cargo version, host and target triples, linker/native-tool
versions, enabled features, profile, and relevant environment. The
`rust-toolchain.toml` file selects a toolchain but does not authenticate its
download or freeze native tools. A channel label such as `stable` moves over
time; it is useful for compatibility testing but insufficient as sole release
identity.

## Isolate the Build

```
release build job
  |
  +-- ephemeral filesystem
  +-- no production credentials
  +-- read-only reviewed inputs
  +-- explicit dependency/cache population
  +-- network denied or allowlisted after fetch stage
  +-- least-privilege user and constrained process/container/VM
  +-- artifact output directory only
  +-- logs + provenance exported before teardown
```

Isolation limits a malicious macro or build script; it does not prove the
artifact benign. Stronger isolation may require a VM rather than a container
when the threat model includes kernel-level separation.

| Control | Security value | Limit |
|---------|----------------|-------|
| Ephemeral runner | reduces persistence | base image or control plane may be compromised |
| No secrets during compile | prevents simple exfiltration | signing/deploy stage must remain separate |
| Network restriction | limits downloads/exfiltration | preloaded malicious input still executes |
| Read-only source | blocks source rewriting | generated outputs and artifact tampering remain possible |
| Pinned toolchain image | improves repeatability | image provenance and patching still matter |
| Independent rebuild | detects some nondeterminism/tampering | matching compromised builders can collude |

## Separate Fetch, Build, Sign, and Deploy

```
FETCH            BUILD              VERIFY             SIGN/DEPLOY
networked   ->   isolated/no key -> policy checks  ->  independent authority
sources          artifact+digest    digest/evidence    signs exact digest
```

The signing service should receive an artifact digest and verified evidence, not
arbitrary source code to compile. A build job should not automatically inherit
production signing or deployment privilege.

If sources and indexes are prepared locally, test the isolation claim with:

```text
cargo build --locked --offline
```

This command proves only that the configured Cargo sources/cache suffice for
that build. It does not prevent build scripts from attempting other network
access; OS-level network policy must enforce that.

## Compiler Trust and Diverse Evidence

Compiler bootstrap attacks are a classic supply-chain concern. Practical
controls include trusted distribution channels, recorded checksums/digests,
minimal builder images, updates through reviewed policy, and selected
independent or diverse rebuilds for high-consequence artifacts.

| Claim | Useful evidence |
|-------|-----------------|
| Approved Rust toolchain was selected | rustup/toolchain file, `rustc -vV`, image digest |
| No undeclared source fetch occurred during build | offline success plus enforced network telemetry/policy |
| Build-time code set is known | Cargo metadata, proc-macro/custom-build inventory |
| Artifact came from this build | cryptographic digest and provenance statement |
| Builder could not sign arbitrary output | separate signing policy and identity logs |

## Old World -> New World Bridge

| Old world | Rust build equivalent |
|-----------|-----------------------|
| MSBuild custom task / pre-build event | `build.rs` host executable |
| Roslyn Source Generator | procedural macro loaded during compilation |
| NuGet install script risk | build dependency and macro execution risk |
| Hermetic C/C++ toolchain image | pinned rustup/rustc/Cargo plus native linker/sysroot image |
| Release-signing pipeline | separate digest-verifying signer after isolated build |

GitHub Actions environments, artifact attestations, OIDC federation, Azure
Key Vault/Managed HSM, and Defender monitoring can implement parts of this
design. Use them as concrete controls under the same separation-of-duties model,
not as a vendor-specific definition of build trust.

## Common Confusion Points

- **"Proc macros only transform tokens."** Their logical interface is tokens;
  the host process can exercise any OS access not otherwise restricted.
- **"`cargo build --offline` is a sandbox."** It restricts Cargo source access,
  not arbitrary child-process network operations.
- **"A container makes the build hermetic."** Only if inputs, network, clock,
  caches, kernel assumptions, and outputs are deliberately controlled.
- **"Pinned Rust means reproducible artifact."** Linkers, native libraries,
  paths, timestamps, environment, and build scripts also influence output.
- **"The build can hold the signing key because CI is trusted."** Separate
  compromise domains and authorize signing only after evidence verification.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| New proc macro or build script | Review as host-executed code and update threat model |
| Release build | Use ephemeral least-privilege runner with no production secret |
| Toolchain selection | Pin an approved exact version/image and record `rustc -vV` |
| Restricted-network claim | Enforce at OS/runtime layer; use `--offline` as supporting evidence |
| Signing | Sign the verified digest in a separate authority boundary |
| High-consequence artifact | Add independent rebuild or diverse verification where practical |
| Build needs native compiler | Pin sysroot/linker/library identities and target matrix |

## Primary Sources

- Cargo Build Scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html
- Rust Reference, Procedural Macros:
  https://doc.rust-lang.org/reference/procedural-macros.html
- rustup Overrides: https://rust-lang.github.io/rustup/overrides.html
- Rust Platform Support: https://doc.rust-lang.org/rustc/platform-support.html
- SLSA Build Track: https://slsa.dev/spec/
- NIST SSDF: https://csrc.nist.gov/Projects/ssdf

## Related Guides

- Previous: [04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md](04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md)
- Next: [06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md](06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md)
- Cargo internals: [../rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md](../rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md)
