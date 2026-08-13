# Adoption, Rollback, and Removal

Status: Guidance
Implementation authority: None

## Purpose

A future reference companion must prove that it can be adopted, renewed,
rolled back, removed, and retired without trapping a repository in hidden
Ferris-owned truth.

## Adoption principles

Adoption is consumer-owned. It must:

- begin from a passing or explicitly classified owner-native baseline;
- name the consumer, support owner, maintenance owner, and validation owner;
- define exact supported repositories, tools, versions, platforms, and stages;
- preserve ordinary Cargo, editor, repository-script, native, and deployment
  workflows;
- isolate fixture artifacts and target directories where required;
- avoid reusable credentials and hidden mutable services;
- define training, diagnostics, audit, privacy, retention, and cost;
- include partial-install recovery;
- set evidence and support expiry; and
- define rollback and complete removal before promotion.

A companion may be useful without being adopted. Adoption does not establish
conformance outside the exact bound claim.

## Adoption record

Record:

1. companion family, suite, fixture, and revision;
2. consumer and repository boundary;
3. governing specification versions;
4. source, lock, toolchain, platform, and environment identity;
5. integration files, hooks, jobs, services, credentials, and storage;
6. owner-native baseline;
7. selected and full-reference commands;
8. expected outputs and thresholds;
9. support, renewal, exception, and expiry policy;
10. privacy, retention, deletion, and publication policy;
11. rollback trigger and procedure; and
12. removal and residual-state checks.

## Phased adoption

### Phase A: observe

Use development fixtures and read-only evidence. Do not alter mandatory gates,
support claims, or owner workflows. Under current Ferris authority, this is
the only product-adjacent phase that can be contemplated, and only within
approved read-only `plan`, `explain`, `graph`, and passive `doctor` behavior.

### Phase B: compare

Run the candidate companion beside owner-native workflows. Compare outputs,
coverage, investigation cost, resource cost, false omissions, over-selection,
and diagnostic quality. No selected path replaces the full reference.

### Phase C: limited support

After a separately approved pulse and all promotion gates, enable one exact
consumer lane with explicit fallback, expiry, and support ownership. Other
repositories, tools, versions, and platforms remain unsupported.

### Phase D: renewable operation

Renew on a fixed cadence and on identity changes. Periodically run full
reference, rollback, removal, privacy, and disaster-recovery controls.

No phase is authorized by this guide.

## Renewal triggers

Renew earlier than scheduled when:

- source, manifest, lock, feature, profile, target, or generated input changes;
- rustc, Cargo, SDK, linker, runner, native tool, owner script, or schema
  changes;
- a platform, provider, connector, upstream owner, or support policy changes;
- an advisory, license, provenance, privacy, or retention condition changes;
- expected outputs or scorer predicates change;
- a required stage fails or becomes unavailable;
- a held-out leakage or infrastructure defect is found; or
- consumer requirements change.

Renewal creates a new immutable revision. It does not rewrite prior evidence.

## Rollback contract

Rollback must define:

- trigger and decision owner;
- last known approved source, lock, toolchain, environment, integration, and
  fixture identities;
- artifact, data, wire, ABI, deployment, and credential consequences;
- commands or owner procedures;
- isolation and cleanup;
- mandatory validation after restoration;
- evidence and audit record;
- timeout, partial rollback, and rollback-failure handling; and
- expiry of the rollback material.

Restoring `Cargo.lock` alone is insufficient when source, generated code,
native libraries, schemas, data, wire formats, deployment, or credentials
changed.

## Family rollback requirements

| Family | Rollback proof |
|---|---|
| Blueprint applications | Restore prior application definition and read-only plan inputs; prove owner workflows were not mutated |
| Renewable profiles | Restore exact manifest and lock, toolchain and target prerequisites, then rerun mandatory stages |
| AI-generated patches | Revert exact patch and related generated, dependency, workflow, or migration changes; rerun full reference |
| Native boundaries | Restore both sides of the ABI, native artifacts, bindings, packaging, and runtime configuration |
| Platform targets | Restore prior SDK, linker, runner, provider, package, and deployment assumptions |
| Upstream packets | Withdraw or supersede local packet state; external withdrawal remains owner-process dependent |

## Removal contract

Companion cleanup and Ferris removal have different record authority. Every
partial or complete removal of Ferris integration requires the canonical,
versioned
[PRODUCT-001 Removal Record](../../specs/FERRIS_PRODUCT_CONTRACT.md#removal-record).
The companion removal evidence below must be attached to or referenced by that
record, including lifecycle phase, owner-native verification, retained
evidence, residual state, and recovery owner.

Complete removal must:

1. stop companion-specific jobs, services, schedulers, and automation;
2. remove Ferris metadata, configuration, hooks, wrappers, caches, target
   directories, credentials, and connector grants according to policy;
3. preserve required historical audit and public-safe evidence;
4. revoke access and delete private retained material where required;
5. remove or replace consumer-owned dependencies and integrations;
6. verify repository status and residual files;
7. execute ordinary Cargo and owner-native workflows;
8. verify native, package, deployment, data, and wire cleanup where applicable;
9. classify any residual hook, credential, artifact, or inaccessible data; and
10. publish a removal disposition with known limitations.

Removal must not require reconstructing a hidden Ferris graph, manifest,
resolver, cache key, or build truth.

## Failed and partial setup

Fixtures must cover:

- setup interrupted before metadata is complete;
- setup interrupted after hooks or jobs are installed;
- credentials issued but later steps fail;
- unsupported platform discovered late;
- dependency or tool installation denied;
- validation failure after integration;
- cleanup failure; and
- restart with stale partial state.

Recovery identifies every created or changed resource and either completes the
approved setup or removes it. A setup command must not silently adopt a
different platform, toolchain, dependency, or provider.

## Expiry and maintenance

Every promoted companion has:

- evidence expiry;
- support expiry;
- named renewal cadence;
- dependency and toolchain update policy;
- platform servicing policy;
- issue triage owner;
- diagnostic and documentation owner;
- storage, CI, runner, and review-cost budget;
- held-out replacement capacity;
- security and privacy review cadence; and
- retirement criteria.

An unmaintained companion becomes expired. It is not left as an apparently
current reference.

## Retirement

Retire when:

- the capability or owner workflow no longer exists;
- the suite is superseded;
- the named consumer leaves;
- support or privacy cost is uneconomic;
- representative execution is no longer possible;
- required platforms cannot be maintained;
- anti-leak separation cannot be sustained; or
- the companion duplicates a better owner-maintained proof.

Retirement records:

- final revision and date;
- replacement, if any;
- retained evidence and retention period;
- deleted evidence and deletion result;
- unresolved failures or unsupported states;
- consumer migration status;
- complete removal result; and
- owners of any remaining obligations.

## Anti-lock-in acceptance

Adoption is not complete until fixtures prove:

- owner-native commands work before, during, and after adoption;
- Ferris metadata can be ignored without changing source correctness;
- no shared writable target directory is required;
- failed setup has a bounded recovery path;
- rollback restores exact prior identities and mandatory validation; and
- complete removal leaves no required hidden Ferris truth.

These requirements follow C-REMOVE in
[CONFORMANCE-001](../../specs/FERRIS_CONFORMANCE_CONTRACT.md#c-remove-adoption-rollback-and-removal)
and the profile lifecycle demonstrated in
[compatibility stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md#lifecycle-policy).
