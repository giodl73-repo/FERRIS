# Evidence and Identity

Status: Guidance
Implementation authority: None

## Purpose

Reference-companion evidence must identify exactly what ran, where it ran,
what it observed, what was expected, and which owner supplied each fact.
Identity, compatibility, trust, validation, and correctness remain separate.

## Identity domains

Do not collapse these domains into one digest:

| Domain | Required examples |
|---|---|
| Fixture | suite ID, case ID, population, revision, lifecycle state |
| Source | repository URL or custody ID, immutable revision, submodules, patch or archive digest |
| Cargo | manifest path and digest, lock digest, workspace membership, features, profile, target |
| Toolchain | rustc and Cargo release, commit, date, host, channel or exact installation identity |
| Owner tools | repository scripts, test runners, SDKs, linkers, native compilers, package tools |
| Environment | OS, architecture, filesystem, locale, shell, clock, network policy, allowlisted variables |
| Invocation | executable, arguments, working directory, stdin, explicit selection, limits, configuration |
| Contract | schema, Typebook/RUNE, ABI, wire, data, provider, and projection versions |
| Result | process result, stage result, diagnostics, bounded output, artifacts, cleanup |
| Validation | requirement set, selected gates, full-reference gates, coverage and capability consequences |
| Evidence | capture method, source owner, timestamps, digests, redaction, retention, expiry |
| Publication | public-safe packet version, disclosure review, omitted sections, viewer and scorer identity |
| Lifecycle | adoption, renewal, rollback, removal, supersession, quarantine, retirement |

Exact equality in one domain does not prove equality in another. A matching
source revision does not prove matching toolchains or results. A passing
result does not prove compatibility beyond its exact contract and environment.

## Minimum fixture manifest

A future manifest should contain:

```text
fixture:
  suite_id
  case_id
  revision
  population
  state
  owner
  expires_at
claim:
  specifications
  conformance_suites
  capability
  limitations
source:
  repository
  revision
  patch_digest
selection:
  manifests
  lock_digest
  features
  profiles
  host
  targets
environment:
  os
  architecture
  filesystem
  locale
  shell
  allowlisted_variables
  network_policy
tools:
  rustc
  cargo
  owner_tools
  native_tools
commands:
  ferris
  full_reference
bounds:
  time
  memory
  storage
  output
  process_count
expectation:
  schema_versions
  result_classes
  mandatory_predicates
  prohibited_predicates
lifecycle:
  setup
  cleanup
  rollback
  removal
  replacement
```

This is guidance vocabulary, not an authorized schema.

## Source identity

Source identity records:

- canonical repository or custody location;
- immutable commit or content digest;
- submodule and generated-source identities;
- manifest and lockfile digests;
- sealed edit, configuration, or failure-seed package digest;
- licensing and redistribution constraints; and
- whether source is public, private, synthetic, minimized, or disclosure
  reviewed.

A moving branch is not a fixture identity. Renewal pins a new revision rather
than moving an existing binding.

The current held-out public source binding illustrates immutable public
revisions plus separately sealed edit packs:
[held-out executable binding](../../simulations/held-out/EXECUTABLE_BINDING.md).

## Toolchain and environment identity

Record exact:

- `rustc` and Cargo version output, including commit and date where available;
- host and target triples;
- installed targets and components;
- SDK, sysroot, linker, archiver, native compiler, package manager, and runner;
- OS edition and build, kernel where relevant, CPU architecture, and
  virtualization boundary;
- filesystem type, case sensitivity, path limits, symlink behavior, and mount
  mode where relevant;
- shell, locale, encoding, time zone, and clock source;
- allowlisted environment-variable names and redacted value identities;
- proxy, registry, offline, network, and certificate policy;
- dependency caches and target-directory topology; and
- CPU, memory, storage, process, time, and output bounds.

Environment redaction must preserve comparison value. For example, record that
an allowlisted variable was present and bind a non-reversible value digest
when policy permits; never publish the secret value.

## Invocation and expected outputs

Every invocation record binds:

- command identity and executable digest;
- argument vector without shell reconstruction ambiguity;
- working directory;
- selected application, workspace, manifest, profile, and target;
- stdin identity or explicit absence;
- environment identity;
- network and mutation classification;
- timeout, cancellation, retry, and cleanup policy; and
- expected output schema and stream contract.

Expected outputs should define:

- result class and process exit class;
- mandatory semantic fields;
- prohibited fields or claims;
- stable values and allowed variance;
- owner, stage, impact, next action, and recovery evidence in diagnostics;
- artifact and evidence identities when applicable;
- selected-only versus full-reference relation;
- cleanup, rollback, and residual-state expectations; and
- cases where exact byte equality is required.

Object ordering, whitespace, temporary paths, durations, and platform-specific
diagnostic prose should be ignored only when the public contract explicitly
declares them non-semantic.

## Evidence capture

Capture three layers:

1. **Raw:** original stdout, stderr, exit status, files, owner logs, resource
   observations, and timestamps.
2. **Canonical:** typed records with explicit source, claim class, stage,
   bounds, redaction, and identity.
3. **Public-safe:** disclosure-reviewed facts, opaque package identifiers,
   digests, aggregate results, limitations, and expiry.

Raw evidence is retained according to classification and access policy. A
canonical projection never replaces raw custody. Public-safe evidence never
implies that omitted private evidence is unnecessary.

## Claim classes

Label each fact as:

- owner-declared;
- directly observed;
- externally reported;
- normalized;
- inferred;
- predicted;
- approved;
- executed;
- yielded; or
- unknown.

Predictions cannot become observations because the expected output matched.
Normalized records cite their raw source. An absent field is not proof of an
empty value.

## Privacy and secrets

Fixtures and evidence must:

- use synthetic or minimized data whenever representative behavior permits;
- exclude reusable credentials from source, prompts, commands, roots, refs,
  logs, outputs, and retained artifacts;
- classify tenant, repository, identity, path, source, diagnostic, and
  artifact data;
- redact before model exposure or public publication;
- retain proof of what was removed and why;
- bound diagnostic and artifact collection;
- define residency, access, retention, deletion, and audit ownership; and
- test redaction failure, over-redaction, and incomplete deletion.

Hashing a low-entropy secret or private identifier is not sufficient
anonymization. Public-safe publication requires disclosure review.

## Anti-leak evidence

For held-out populations, retain:

- fixture author and custodian identities;
- implementation cutoff;
- model, prompt, tool, retrieval, and context exposure;
- development and calibration corpus membership;
- input and oracle access logs;
- sealed package digests;
- collection and scorer cutoffs;
- oracle release time; and
- leakage review disposition.

If hidden outcomes enter an implementation prompt, mapping, threshold, or
debug session, quarantine the fixture and replace it. The old record remains
historical development evidence.

## Portability matrix

Each companion publishes a stage matrix such as:

| Stage | Windows | Unix | Cross target | Native target |
|---|---|---|---|---|
| Resolve | result and identity | result and identity | result and identity | result and identity |
| Check | result and identity | result and identity | result and identity | result and identity |
| Build | result and artifact | result and artifact | result and artifact | result and artifact |
| Link | native or unsupported | native or unsupported | cross-link result | native result |
| Execute | native result | native result | not implied | native result |
| Test | selected and full | selected and full | runner-bound | native result |
| Package/deploy | owner result | owner result | owner result | owner result |

Cells use typed states, not checkmarks. A Windows pass and Linux cross-check
do not establish Linux-native execution. This follows the stage separation
demonstrated in
[compatibility stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md#target-and-stage-matrix).

## Evidence expiry

Evidence expires when any bound identity or support assumption changes,
including:

- source or lock;
- compiler, Cargo, SDK, linker, runner, or owner tool;
- host or target platform;
- schema or contract;
- owner, advisory, privacy, or policy state;
- expected output or scorer;
- resource budget; or
- support commitment.

Expired evidence remains history. It cannot silently support a current claim.
