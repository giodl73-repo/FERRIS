---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:ci-cd-promotion
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: CI/CD and Promotion
status: source-custody
source_custody: partial
current_path: rust-production-engineering/11-CI-CD-AND-PROMOTION.md
canonical_path: rust-production-engineering/11-CI-CD-AND-PROMOTION.md
backsource_ids: [mdloom-backfill:rust-production-engineering:11-ci-cd-promotion]
concepts: [continuous integration, continuous delivery, promotion, build once, pipeline security, deployment, progressive delivery]
root_concepts: [software delivery]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# CI/CD and Promotion

## The Big Picture

Continuous integration produces evidence about a source revision. Continuous
delivery turns one verified artifact into a sequence of controlled exposure
decisions. The strongest production pipeline builds once, identifies the
artifact immutably, and promotes those same bytes.

```
+============================================================================+
|                           DELIVERY PIPELINE                                |
|                                                                            |
| change --> review --> clean build --> test/analyze --> package/attest      |
|                                      |                                     |
|                                      v                                     |
|                              immutable artifact                            |
|                                      |                                     |
|                +---------------------+---------------------+               |
|                v                     v                     v               |
|             test env             staging/canary          production        |
|          verify contract      compare live signals     expand exposure     |
|                |                     |                     |               |
|                +-------- evidence + approval policy ------+                |
+============================================================================+
```

An environment should supply configuration and identity, not rebuild source.
Promotion is a metadata and policy transition around a digest.

## Integration Gates

| Gate | Question |
|---|---|
| Format/lint | Does source satisfy mechanical policy? |
| Build matrix | Do supported targets/features compile? |
| Unit/property tests | Do local invariants hold? |
| Integration/contract tests | Do real boundaries agree? |
| Dependency/license checks | Is the resolved graph acceptable? |
| Artifact inspection | Is the package complete and expected? |
| Provenance/signature | Can deployers verify origin and integrity? |

Use `--locked` in CI for applications. Test intentional feature combinations;
`--all-features` can be invalid when features are mutually exclusive, while
testing only defaults can miss supported surfaces.

## Executable Local CI Baseline

Scope: a Rust application whose supported surface is the default feature set
and all Cargo target kinds in the workspace for the CI host target. In Cargo,
`--all-targets` means libraries, binaries, examples, tests, and benches; target
triples require an explicit `--target <triple>` matrix.

```bash
set -eu
rustc --version --verbose
cargo --version
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo doc --workspace --no-deps --locked
cargo build --workspace --release --locked
```

PowerShell equivalent:

```powershell
$ErrorActionPreference = "Stop"
rustc --version --verbose
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo --version
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo fmt --all --check
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo clippy --workspace --all-targets --locked -- -D warnings
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --locked
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo doc --workspace --no-deps --locked
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo build --workspace --release --locked
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
```

`rustfmt` and Clippy must be installed toolchain components. Security,
license, SBOM, and provenance tools are additional explicit dependencies; do
not hide them in an unversioned runner image. Run only supported feature
combinations and target triples; neither `--all-targets` nor `--all-features`
means "all supported platforms." If library doctests are a supported surface,
run `cargo test --workspace --doc --locked` in a job whose selected packages
actually contain library targets; Cargo reports an error for a binary-only
package selected with `--doc`.

## Pipeline Trust Boundaries

```
untrusted change
   |
   +--> build/test with read-only source and no production secrets
   |
trusted protected revision/tag
   |
   +--> sign/publish using short-lived scoped identity
   |
deployment controller
   |
   +--> verify policy, then change environment reference to digest
```

Pull-request code can change build scripts, proc macros, tests, and CI commands;
therefore it can execute arbitrary code on the runner. Do not expose release or
production credentials to untrusted change builds. Prefer workload federation
or another short-lived identity over stored long-lived secrets.

## Caching Without Corrupting Evidence

Cargo registry/source caches are generally safer to share than compiled target
directories. Compiler outputs depend on toolchain, target, flags, environment,
features, native libraries, and paths. Cache keys must encode relevant inputs,
and a cache hit must never substitute for artifact provenance.

| Cache | Benefit | Main risk |
|---|---|---|
| Registry/git source | download reduction | stale policy metadata |
| `target/` | compile reduction | incorrect reuse across inputs |
| Compiler cache | object reuse | key/configuration mismatch |
| Container layers | packaging speed | hidden mutable base/tag |

## Promotion and Progressive Exposure

Separate deployment from release when useful: deployment places an artifact;
release exposes users or work. Canary, ring, blue/green, feature flag, and
traffic shadowing are different mechanisms with different state implications.

```
0% exposed --> internal/smoke --> 1% --> 10% --> 50% --> 100%
                  |                |      |       |
                  +---- hold/compare/rollback or roll-forward ----+
```

Automated promotion needs a minimum sample size, observation duration, stable
comparison cohort, and explicit signals. "No alert fired in five minutes" is
not a statistical gate for a low-volume service.

## Database and Message Compatibility

Run schema transition policy as part of delivery:

1. Expand durable interfaces.
2. Deploy compatible readers/writers.
3. Backfill and verify.
4. Increase exposure.
5. Contract after rollback and old-version windows close.

Do not couple destructive migration irreversibly to application startup. One
replica becoming leader by accident is not a migration control plane.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Rust/build | Cargo commands, test tools, artifact metadata, supply-chain tools |
| Runtime | normally tested as a dependency; not the pipeline orchestrator |
| Platform | GitHub Actions, Azure Pipelines, GitLab CI, Jenkins, deployment controllers |

Pipeline syntax is vendor-specific. The evidence graph - revision, toolchain,
tests, artifact digest, attestation, approval, exposure - is universal.

## Old World -> New World Bridge

The universal bridge is from **build automation** to **policy-as-evidence**.
Modern delivery systems do not merely run commands; they preserve an auditable
chain from reviewed source to exposed artifact.

VSTS/Azure Pipelines stages and environments map directly to these concepts.
GitHub Actions, GitLab, and Jenkins implement similar gates; none changes the
build-once or least-privilege principles.

## Decision Cheat Sheet

| Use | When |
|---|---|
| `--locked` | building an application from an approved dependency graph |
| Target/feature matrix | multiple surfaces are supported in production |
| Build once/promote | environment differences are configuration, not compilation |
| Protected release job | signing/publishing requires trusted identity |
| Short-lived federated identity | CI platform can exchange workload identity |
| Canary/ring | live comparison can bound release risk |
| Blue/green | rapid traffic switch and duplicated capacity are acceptable |
| Manual approval | evidence needs human judgment or policy requires separation |

## Common Confusion Points

- **A green CI run is not production readiness.** Operational and recovery
  evidence lives in later gates.
- **Rebuilding per environment breaks artifact identity.**
- **CI code is executable supply-chain code.** Review and pin it.
- **Caching is not reproducibility.**
- **Deployment success is not release success.** Exposure and user impact may
  lag.
- **`--all-features` is not always a valid compatibility test.**

## Primary Sources

- Cargo commands: https://doc.rust-lang.org/cargo/commands/
- Cargo build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html
- OpenID Connect core: https://openid.net/specs/openid-connect-core-1_0.html
- SLSA build track: https://slsa.dev/spec/v1.0/build-requirements
- NIST SSDF: https://csrc.nist.gov/Projects/ssdf
- OpenSSF Scorecard: https://scorecard.dev/

## Related Guides

- Previous: [10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md](10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md)
- Next: [12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md](12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md)
- Artifact contract: [09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md](09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md)
- Final gates: [15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md](15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md)
