# Failure Modes and Controls

Status: Guidance
Implementation authority: None

## Purpose

These controls stop upstream support from becoming authority capture,
misleading evidence, maintainer burden, or an irreversible downstream fork.
They synthesize the owner and stewardship findings in
[ECOS-Q12](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md)
and [Rust maintenance and stewardship](../../research/2026-08-09-rust-maintenance-stewardship.md).

## Ownership and routing failures

### Treating registry authority as complete ownership

**Failure:** A crates.io owner list is used as the maintainer, reviewer,
security, or support roster.

**Control:** Record publication, source, review, release, security, support,
and consumer authorities separately. Renew owner sets and retain diffs.

### Routing by repository-wide activity

**Failure:** A monorepo's activity is used as evidence for a specific crate.

**Control:** Resolve package-to-path identity and prefer path-scoped commits,
reviews, releases, and response observations.

### Forcing ownership with a large patch

**Failure:** A broad implementation is submitted before the current owner or
intake path is known.

**Control:** Start with a bounded maintainer question and minimal routing
request. Do not post until owner scope and approval are clear.

## Reproducer and evidence failures

### Minimizing away the mechanism

**Failure:** The fixture becomes smaller but loses the original incremental,
feature, macro, native, ABI, runtime, or platform behavior.

**Control:** Rerun distinguishing positive, negative, and correctness controls
after every reduction. Record rejected reductions.

### Confusing compilation with correctness

**Failure:** A successful compile is reported as proof of behavior, soundness,
security, portability, or compatibility.

**Control:** Add dedicated semantic, safety, runtime, negative, target, and
failure evidence. Apply the
[Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md) and
[Validation Checker](../../../.roles/editorial/validation-checker.md) lenses.

### Promoting a microbenchmark

**Failure:** One convenient case or wall-time sample is generalized to user
iteration time.

**Control:** Distinguish cold, incremental, check, build, test, link, and
runtime workflows; record hardware, toolchain, cache, commands, distributions,
causality, variance, and limitations.

### Hiding non-success states

**Failure:** Unsupported, unavailable, stale, failed, conflicting, or unknown
evidence is rewritten as a recommendation or success.

**Control:** Preserve typed states and raw evidence. Do not let models or
presentation code remove failures.

### Leaking private or unlicensed material

**Failure:** Logs, traces, fixtures, profiles, generated files, or commands
expose private source, internal paths, tenant data, credentials, or
third-party code without permission.

**Control:** Perform license and disclosure review, use public or synthetic
fixtures, remove reusable secrets, retain provenance, and verify that
redaction preserves the mechanism.

## Owner-native workflow failures

### Replacing Cargo truth

**Failure:** A downstream plan or tool invents dependency resolution,
workspace membership, feature, target, fingerprint, freshness, build-unit, or
scheduling truth.

**Control:** Use Cargo's output and owner-native workflow. Ferris may organize
evidence but must not create a parallel resolver, hidden manifest, or
synthetic owner truth.

### Using the wrong upstream vocabulary

**Failure:** Local terms are presented to rustc-perf or Cargo without mapping
to their profiles, scenarios, metrics, test forms, or benchmark conventions.

**Control:** Adapt the packet to the owner's vocabulary and contribution
instructions. Keep local terms only with explicit mappings.

### Creating a permanent wrapper

**Failure:** A temporary consumer adapter becomes required infrastructure,
hides semantic loss, or blocks ordinary tool use.

**Control:** Name direction, owner, loss, tests, assumptions, expiry,
substitution, rollback, and removal. Prefer upstream convergence.

## Maintainer burden failures

### Exporting an unfunded obligation

**Failure:** A benchmark, fixture, test, diagnostic, or patch is useful once
but creates ongoing triage, platform, dependency, or release work with no
owner.

**Control:** Estimate review and recurring maintenance cost before submission.
Name the maintainer, response commitment, renewal trigger, noise budget, and
retirement condition. Offer funded review or maintenance where appropriate.

### Measuring success by merge

**Failure:** The program optimizes for PR count, lines changed, or acceptance
without measuring maintenance and owner value.

**Control:** Measure reproduction time, review burden, retained diagnostics,
regression detection, response, renewal, continued upstream ownership, and
clean retirement. A redirect, decline, or external disposition may be correct.

### Over-contacting maintainers

**Failure:** Stale, duplicate, broad, or low-signal requests consume scarce
maintainer time.

**Control:** Renew routing, search current owner records, deduplicate packets,
ask one question, provide copyable commands, summarize limits, and respect
decline or non-response. Do not escalate silence into an unmaintained claim.

## Stewardship and governance failures

### Inferring abandonment

**Failure:** Release age, open issues, downloads, funding, owner count, or
activity concentration becomes an abandonment verdict.

**Control:** Treat those facts as review triggers only. Require explicit
declaration or documented failed contact for an inferred-unmaintained claim.

### Taking over through support

**Failure:** Funding or staffing silently transfers roadmap, release,
publication, or governance authority.

**Control:** Put scope, current-owner decision rights, response, renewal,
succession, data, exit, and public attribution in the support agreement.

### Declaring a fork a successor

**Failure:** Network lineage or name similarity is treated as community
recognition or compatibility.

**Control:** Require maintainer declaration, namespace and source identity,
API and behavior compatibility, migration, publication authority, adoption,
and lifecycle evidence.

### Posting without approval

**Failure:** A complete local packet is mistaken for permission to create a
public issue, comment, branch, benchmark request, or pull request.

**Control:** Require recorded organizational approval and the owner-native
intake gate immediately before posting. No approval means local-only.

## Control checklist

Before advancing, verify:

- one bounded maintainer question;
- current owner and intake path;
- ordinary Cargo and owner workflow;
- minimal reproducer with positive and negative controls;
- exact identity, environment, commands, and typed states;
- public-safe licensed evidence;
- explicit external-post approval;
- review and maintenance burden;
- response, renewal, supersession, and retirement rules;
- adapter or fork decision record where applicable;
- rollback and complete removal; and
- all nine role concerns addressed without claiming role approval.
