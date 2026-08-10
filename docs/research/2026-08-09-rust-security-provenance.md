# Rust Security and Provenance

Date: 2026-08-09
Status: Complete
Question: ECOS-Q06
Decision: represent dependency assurance as joined, renewable evidence across
archive identity, source revision, publication authority, advisory snapshots,
active closure, executable build surfaces, unsafe and native boundaries,
review attestations, licensing, and explicit unknowns. Do not create a
universal crate safety score.

## Decision supported

ECOS-Q06 defines the security and provenance evidence required by the OSPREY
Ecosystem adapter and Crate Ecosystem Ledger.

It does not:

- certify any selected crate or stack as safe;
- infer source equivalence from `.cargo_vcs_info.json`;
- treat zero advisory matches as vulnerability absence;
- equate zero direct unsafe syntax with soundness;
- treat a Cargo `links` count as a complete native-code inventory;
- decide license compatibility for a consumer; or
- authorize OSPREY implementation.

## Assurance evidence model

| Dimension | Required evidence |
|---|---|
| Registry/archive integrity | Registry, exact package and version, archive hash algorithm and value, expected checksum, match result, fetch time, and mirror or cache identity |
| Source identity | Canonical repository and path, package VCS revision, dirty state, tag or release relation, revision-resolution result, and archive-to-source reproduction state |
| Publication provenance | Human publisher or trusted provider/repository/workflow/run/commit, publish time, trust policy at observation time, and agreement or conflict with package VCS metadata |
| Advisory evidence | Database identity or observation time, query tool and version, submitted package identity, lockfile or active-closure scope, matches, ignores, severity policy, and freshness |
| Dependency scope | Lockfile universe plus active package, feature, target, profile, dependency-kind, build, macro, and native closures |
| Build execution | Build-script package and target, host identity, declared and observed inputs, environment, filesystem, process, network, generated output, Cargo directives, and failure state |
| Macro execution | Procedural-macro package and version, invocation, input and output identity where observable, host resources, generated Rust, diagnostics, and expansion unknowns |
| Unsafe boundary | Direct and expanded unsafe code, FFI, unsafe attributes, mutable statics, lint policy, target and feature scope, reachability, review criteria, and residual unknowns |
| Native boundary | `links`, system discovery, bundled source, compiler and linker invocation, ABI, provider, native version, target, artifact identity, and advisory/license coverage |
| Review evidence | Audit criteria, reviewer or organization, exact version or diff, date, scope, exclusions, import or trust chain, expiration, and supersession |
| Licensing | Declared SPDX expression, packaged texts and notices, detected or clarified files, consumer policy, exceptions, generated/native terms, and distribution disposition |
| Renewal | Immutable observation, source timestamps, typed changes, expiration, replacement query, and explicit stale, unsupported, not-observed, and unknown states |

These dimensions may support a policy decision, but they are not additive
points.

## Measured queue

The nineteen ECOS-Q02 exact releases were inspected. Commands, full revision
table, closure counts, unsafe syntax counts, build-script effects, license
inventory, sources, and limitations are in
[EXP-01](ecos-q06-security-provenance/results/EXP-01-security-provenance-census.md).

Observed:

- all nineteen `.crate` SHA-256 values matched their crates.io checksums;
- all nineteen archives contained `.cargo_vcs_info.json`;
- all nineteen named commits resolved in their canonical repositories;
- `http 1.5.0` recorded a dirty worktree;
- the three trusted-published releases had identical trusted-publication and
  package VCS commits;
- the default Windows closure contained 28 packages and six build scripts;
- enabling Serde derive produced 29 packages and added one procedural macro,
  `serde_derive 1.0.229`;
- neither observed active closure contained a Cargo `links` package;
- the probe lockfile contained 31 dependency packages;
- pinned `cargo audit 0.22.2` reported zero vulnerabilities and zero warnings;
- nineteen exact OSV package-version results were empty at observation time;
- 710 direct source files and six build scripts parsed successfully in the
  syntax census;
- direct source contained 698 unsafe blocks, 582 unsafe functions, 72 unsafe
  impls, two unsafe traits, 7,565 foreign items, and two mutable statics; and
- every archive declared a license expression and packaged license text.

These are observations, not safety verdicts.

## Findings

### FERRIUM-578: release identity is a joined chain, not one hash

**Sources:** crates.io checksums, package archives, Cargo package VCS
documentation, canonical repository revision checks, and ECOS-Q05 publication
records in EXP-01.

**Observed behavior:** Registry checksum, archive bytes, package VCS revision,
canonical repository revision, trusted-publishing commit, tag, and generated
build output are distinct identities. The queue allowed several of them to be
joined, but no single identity covered the whole chain.

**Implication:** The Crate Ecosystem Ledger must model typed identities and
agreement or conflict edges. A checksum match must not be relabeled
provenance, source review, or reproducibility.

**Confidence:** High.

### FERRIUM-579: package VCS metadata is useful but explicitly unverified

**Sources:** Cargo `.cargo_vcs_info.json` documentation and nineteen package
archives.

**Observed behavior:** Every archive named a revision that resolved, but Cargo
documents the file as best effort and does not guarantee that archive source
matches the revision.

**Implication:** Record VCS metadata as a package claim. Archive-to-revision
reproduction or diff evidence must remain a separate observation.

**Confidence:** High.

### FERRIUM-580: dirty publication creates an irreducible source delta

**Sources:** `http 1.5.0` archive metadata.

**Observed behavior:** The package named
`e559023f67e3fad6ecc3ee91307be178e0f13626` and set `dirty: true`. The
registry checksum identifies the published bytes, but the named commit alone
cannot reconstruct the uncommitted delta.

**Implication:** Dirty state is a first-class provenance limitation. A profile
may reject, require archive review, or require a reproducible-source exception,
but FERRIUM must not invent the missing change set.

**Confidence:** High.

### FERRIUM-581: trusted-publication agreement strengthens attribution, not safety

**Sources:** ECOS-Q05 trusted-publishing records and package VCS metadata.

**Observed behavior:** `getrandom 0.4.3`, `rand_core 0.10.1`, and `cc 1.4.2`
had matching trusted workflow and package VCS commits.

**Implication:** Store authenticated publication origin and identity agreement.
Do not infer review quality, reproducible packaging, vulnerability absence, or
safe behavior.

**Confidence:** High.

### FERRIUM-582: advisory output is a dated identity match

**Sources:** `cargo audit 0.22.2`, RustSec database commit
`565436d86a136c840d01ad4a7851fc7391295404`, OSV batch API, and EXP-01.

**Observed behavior:** RustSec found no vulnerabilities or warnings across 31
lockfile dependencies, and OSV returned no records for the nineteen submitted
exact releases.

**Implication:** Advisory evidence must retain database or service time, tool,
submitted identity, match result, ignored records, and query scope. The result
language is `no matching records observed`, never `safe`.

**Confidence:** High.

### FERRIUM-583: lockfile and active closure answer different questions

**Sources:** probe Cargo.lock and default/derive Cargo metadata.

**Observed behavior:** The lockfile contained 31 dependency packages; the
default active Windows closure contained 28 and the derive closure 29.

**Implication:** OSPREY must identify whether an advisory, license, audit, or
unsafe claim covers the lockfile universe or a specific target-feature
closure. Both can be useful; neither may silently stand in for the other.

**Confidence:** High.

### FERRIUM-584: features can select code that executes inside compilation

**Sources:** default and `serde/derive` metadata closures.

**Observed behavior:** Enabling one root feature added
`serde_derive 1.0.229`, a procedural-macro crate. The dependency change altered
the compile-time execution boundary, not only linked library code.

**Implication:** Feature diffs must include added and removed build scripts,
procedural macros, native tools, generated code, and host trust effects.

**Confidence:** High.

### FERRIUM-585: build scripts and procedural macros are host execution boundaries

**Sources:** Cargo build-script documentation, Rust procedural-macro
documentation, and six build-script source reviews.

**Observed behavior:** The selected scripts read environment state, wrote
generated files, invoked processes, and emitted compiler configuration.
Procedural macros run with compiler resource access.

**Implication:** Treat these targets as host executables with capabilities,
inputs, outputs, failures, and provenance. A source dependency graph that
omits compile-time execution is incomplete.

**Confidence:** High.

### FERRIUM-586: Cargo metadata does not fully describe native effects

**Sources:** active metadata closures and `cc` / `pkg-config` package roles.

**Observed behavior:** No active package declared `links`, yet downstream
scripts can call `cc` to compile native source or `pkg-config` to discover
system libraries.

**Implication:** Native assurance must join manifest declarations with
observed process, system-package, artifact, linker, ABI, advisory, and license
evidence. `links = 0` means no observed declaration, not no native boundary.

**Confidence:** High.

### FERRIUM-587: unsafe syntax is exposure evidence, not a soundness score

**Sources:** direct syntax census in EXP-01.

**Observed behavior:** Direct unsafe syntax ranged from zero in small contract
crates to hundreds of blocks or functions in implementation and platform
crates. The scanner did not expand macros, apply target cfg, test reachability,
or evaluate invariants.

**Implication:** Preserve direct, expanded, generated, FFI, reachability, lint,
and review evidence separately. Unsafe quantity may focus review but cannot
rank safety or soundness.

**Confidence:** High for direct syntax; Low for semantic risk.

### FERRIUM-588: absence of direct unsafe code does not close the safety boundary

**Sources:** syntax census, build-script review, procedural-macro and Cargo
documentation.

**Observed behavior:** Several crates had no direct unsafe syntax, while their
closures could still include unsafe dependencies, compile-time executables,
generated code, native tools, and platform APIs.

**Implication:** A crate-level `unsafe = 0` label is insufficient. Assurance
must preserve complete closure and generated/native unknowns.

**Confidence:** High.

### FERRIUM-589: license expressions are inputs to policy, not policy decisions

**Sources:** archive manifests, packaged license files, Cargo manifest
documentation, and SPDX license-expression specification.

**Observed behavior:** Every release declared an expression and included
license text, but the expressions represented choices such as
`MIT OR Apache-2.0` and `Unlicense OR MIT`.

**Implication:** Store declared, detected, clarified, selected, and
distribution licenses separately. Compatibility depends on consumer policy,
use, generated output, native components, notices, and exceptions.

**Confidence:** High.

### FERRIUM-590: assurance tools are complementary and criteria-bound

**Sources:** Cargo, RustSec, OSV, cargo-deny, cargo-vet, and measured scanner
boundaries.

**Observed behavior:** Each tool covered a different question: archive
integrity, metadata, advisory matching, configurable dependency policy,
review attestations, or direct syntax.

**Implication:** FERRIUM should ingest evidence without replacing upstream
tools or flattening their criteria into one score. Missing tool evidence must
remain `not observed`, not failed or passed.

**Confidence:** High.

### FERRIUM-591: renewal must preserve changed evidence and stale unknowns

**Sources:** time-dependent registry, publication, advisory, source, policy,
and closure observations from ECOS-Q05 and Q06.

**Observed behavior:** Checksums are immutable per release, while advisories,
audit sets, owner policies, source custody, license policy, features, targets,
and consumer acceptance can change.

**Implication:** Retain immutable assurance snapshots and typed renewal diffs.
Expire mutable claims independently and show stale, unsupported,
not-observed, and unknown states.

**Confidence:** High.

## Decision

### Adopt now

- Adopt the multidimensional assurance record and identity-agreement model.
- Record immutable archive checksum separately from VCS, tag, trusted
  publication, build, and final artifact identities.
- Require advisory database or service time, tool version, submitted identity,
  ignores, and lockfile-versus-active-closure scope.
- Treat build scripts and procedural macros as compile-time host executables.
- Preserve direct, expanded, generated, dependency, FFI, and native unsafe
  boundaries separately.
- Preserve declared, detected, clarified, selected, and distribution license
  states.
- Use `no matching advisories observed`, `no direct unsafe syntax observed`,
  and `no links declaration observed` rather than safety-shaped conclusions.
- Renew mutable evidence and retain explicit unknowns.

Owner: FERRIUM.

Expected validation: ECOS-Q07 target-specific closures, ECOS-Q09 native
execution and provider boundaries, ECOS-Q10 selection policy, ECOS-Q11 profile
renewal, and ECOS-Q12 intervention decisions.

Non-goals: certifying crates, replacing Cargo, crates.io, RustSec, OSV,
cargo-deny, or cargo-vet; issuing legal opinions; proving soundness; or
sandboxing compilation.

### Prototype behind a compatibility boundary

- registry/archive/VCS/trusted-publication identity joins and conflicts;
- lockfile and target-feature closure comparison;
- build-script, procedural-macro, `links`, native-tool, and generated-output
  inventory;
- advisory queries with frozen snapshot or dated service evidence;
- direct unsafe syntax and lint-policy collection with explicit expansion
  gaps;
- cargo-vet audit and cargo-deny policy ingestion without redefining criteria;
- license expression, file, notice, clarification, and consumer disposition;
  and
- immutable assurance snapshots with independently expiring evidence.

### Reject or defer

- one crate safety, trust, provenance, or compliance score;
- source equivalence inferred from `.cargo_vcs_info.json`;
- safe labels from zero advisory matches or zero direct unsafe syntax;
- native-free labels from absent Cargo `links`;
- automatic advisory ignores or policy exceptions;
- automated legal compatibility conclusions;
- FERRIUM-owned advisory or audit authority;
- automatic build-script or procedural-macro sandbox enforcement; and
- OSPREY implementation before the Crates Series gate.

## Role review

### Rust Safety Steward

Accepts the separate unsafe, FFI, generated, native, review, and provenance
boundaries. Requires direct syntax counts to remain non-semantic and rejects
safe or sound labels without dedicated review and target-specific evidence.

### Compiler Performance Engineer

Accepts closure reuse and bounded parsing. Requires future continuous scans to
measure metadata, extraction, advisory, expansion, native tracing, storage,
and renewal cost rather than silently adding them to every build.

### Interop Boundary Auditor

Accepts native provider, ABI, compiler, linker, system package, generated
binding, and license identity as separate evidence. Requires ECOS-Q09 to test
actual native boundaries rather than infer them from `links`.

### AI Assurance Skeptic

Accepts observed, inferred, and unknown separation. Rejects `zero = safe`,
unverified VCS provenance, AI-generated audit conclusions, silent advisory
ignores, and success-shaped treatment of unavailable evidence.

### Ecosystem Strategist

Accepts integration of Cargo, RustSec, OSV, cargo-deny, and cargo-vet evidence.
Requires contribution and interoperability with current owners rather than a
FERRIUM certification authority or duplicate database.

### Rust Maintainer

Accepts exact identities, actionable conflicts, and ordinary Cargo
compatibility. Requires scans to explain why a dependency triggered review,
permit correction and policy ownership, and remain removable.

### Native Platform Adopter

Accepts archive, advisory, license, executable-build, and native-boundary
evidence. Requires later profiles to record supported targets, system
dependencies, tool installation, offline behavior, compliance ownership,
failure diagnosis, and rollback.

### Scope Keeper

Accepts Q06 as an evidence model only. Platform support, exhaustive native
analysis, stack selection, policy automation, sandboxing, certification, and
implementation remain closed.

### Validation Checker

Accepts nineteen exact archives and revisions, two Cargo closures, a pinned
31-package RustSec audit, nineteen OSV results, 710 parsed source files, six
parsed build scripts, commands, sources, negative interpretations, and
limitations. Requires future renewals to retain snapshots, target-feature
scope, parse failures, ignores, and changed evidence.

## Limitations

- Measurements used one Windows host and one toolchain.
- The selected queue is not a universal Rust application stack.
- OSV was a dated service observation rather than a frozen database.
- Source commits resolving did not establish reproducible package construction.
- Build-script effects were source-reviewed rather than dynamically traced.
- Procedural-macro inputs, outputs, and resource use were not traced.
- Syntax analysis did not expand macros, apply cfg, test reachability, or
  evaluate invariants.
- Native and generated-code closures remain incomplete.
- No crate audit was performed by FERRIUM.
- License compatibility and legal obligations were not adjudicated.
