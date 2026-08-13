# Operating Workflow

Status: Guidance
Implementation authority: None

## Purpose

This workflow describes how a future companion moves from a proposed proof
boundary to renewable Conformance evidence. It is a design for later work, not
an executable procedure authorized today.

## Lifecycle states

```text
candidate
  -> development
  -> review-ready
  -> calibration
  -> held-out-bound
  -> scored
  -> published-public-safe
  -> renewed | quarantined | expired | retired
```

State changes are recorded. A repository name, branch, tag, or passing badge
does not substitute for a lifecycle record.

## Step 1: define the claim

Write one bounded claim with:

- capability and non-goals;
- governing specification versions;
- applicable Conformance suites;
- consumer and owner;
- supported and unsupported platforms;
- exact validation stages;
- expected success, rejection, failure, stale, skew, rollback, and removal
  outcomes; and
- fixed pass, fail, stop, and disable criteria.

If a claim cannot be tied to a reproducible owner-native command or reviewable
procedure, it is not ready for an executable companion.

## Step 2: choose the companion shape

Choose the smallest shape that preserves owner truth:

- a fixture directory within a companion repository;
- a repository with one application or platform boundary;
- a repository family with independently versioned lanes;
- an external public repository pinned at an immutable revision plus sealed
  edit packs; or
- a public contract with private inputs where privacy or anti-leak controls
  require separation.

Do not combine Blueprint, profile, AI, native, platform, and upstream packet
proof merely to reduce repository count.

## Step 3: establish the owner-native baseline

Before adding Ferris integration:

1. freeze source and lock identities;
2. record repository-required setup and commands;
3. run the complete owner-native reference;
4. capture toolchain, environment, platform, resource, and output evidence;
5. classify every stage independently;
6. record failures and unavailable prerequisites without repair by Ferris; and
7. verify cleanup and repeatability.

The baseline remains available for adoption comparison and removal proof.

## Step 4: design the case matrix

Each applicable capability needs:

- one ordinary positive case;
- one negative or expected-rejection case;
- one owner-command failure;
- one unsupported or unavailable case;
- one stale or expired evidence case;
- one version-skew case;
- one resource or output-bound case;
- one privacy or redaction case;
- one rollback case; and
- one complete removal case.

Add cross-platform cases wherever path, process, filesystem, linker, target,
native discovery, shell, locale, or output behavior can differ. Seeded cases
must state what is public and what remains sealed.

## Step 5: bind exact identity

Create a binding record covering the fields in
[Evidence and identity](03-EVIDENCE-AND-IDENTITY.md). At minimum, freeze:

- fixture and revision ID;
- source repository and immutable revision;
- manifest and lock digests;
- patch or input archive digest;
- exact Rust, Cargo, owner tools, native tools, SDKs, and runners;
- host and target triples;
- OS, architecture, filesystem, locale, shell, and allowlisted environment;
- commands and working directories;
- network and dependency acquisition policy;
- resource and output bounds;
- expected schema and result classes; and
- cleanup, rollback, expiry, and owner.

Changes to these values create a new binding revision.

## Step 6: separate populations

Create independent:

- development cases for implementation and diagnostic work;
- calibration cases for thresholds, mapping rules, prompt design, and scorer
  qualification; and
- held-out cases for blind scoring.

Do not derive held-out edits by trivially renaming a development case. Record
population design, source overlap, shared dependencies, and leakage risks.
The separation rules follow the
[held-out README](../../simulations/held-out/README.md) and
[oracle custody protocol](../../simulations/held-out/ORACLE_CUSTODY.md).

## Step 7: qualify harnesses, viewers, and scorers

Before releasing a held-out oracle:

1. execute mixed success and non-success preflight records;
2. prove expected process cardinality;
3. prove complete durable collection;
4. test all accepted schema layouts;
5. test parser, framing, truncation, and stream boundaries;
6. distinguish harness, collection, scorer, fixture, and implementation errors;
7. verify deterministic scoring on sealed copies; and
8. freeze harness and scorer identities.

A scorer must validate semantic fields, not accidental object ordering or
pretty-print layout. Invalid scoring infrastructure yields an invalid result,
not an implementation pass or failure.

## Step 8: execute independently

Run in this order:

1. verify the immutable implementation cutoff;
2. verify input and tool digests;
3. execute Ferris and capture raw outputs;
4. execute the owner-native full reference independently;
5. capture environment and resource evidence;
6. seal all records before oracle release;
7. release the oracle to the scorer;
8. classify deviations without rewriting the implementation or oracle; and
9. record cleanup and residual state.

Retries require a declared policy. A retry must not erase the first failure or
change the environment silently.

## Step 9: publish only public-safe evidence

Public publication may include:

- fixture class and public contract;
- public source repository and immutable revision;
- toolchain and environment classes;
- opaque sealed-package IDs and digests;
- process and record cardinality;
- aggregate disposition;
- public-output digest;
- known limitations and expiry; and
- proof that prohibited material was not accessed.

It must exclude:

- private source or data;
- secrets and reusable credentials;
- hidden edits and failure seeds;
- expected machine records;
- oracle predicates and tolerances;
- private scoring notes;
- model-visible hidden outcomes; and
- identifiers that reveal tenants, users, machines, or undisclosed products.

The [public-safe custody receipt](../../simulations/held-out/PUBLIC_SAFE_RECEIPT.md)
is an example of publishing custody facts without exposing sealed inputs.

## Step 10: renew or retire

Renew when source, dependencies, toolchains, schemas, commands, platforms,
owners, privacy policy, support scope, or expected behavior changes. Renewal:

- creates a new binding revision;
- revalidates owner-native baselines;
- requalifies harnesses and scorers where affected;
- replaces held-out cases if leakage risk changed;
- records compatibility with prior evidence; and
- updates expiry and removal instructions.

Retire when the capability is removed, the owner workflow disappears, support
cost is uneconomic, privacy cannot be preserved, a replacement supersedes it,
or the fixture no longer represents a named consumer.

## Family-specific workflow additions

### Blueprint applications

Record application-definition identity, declared workspaces, Cargo metadata,
owner closures, non-executable plan, explanation, mandatory validation, and
full-reference comparison. Under current authority, stop before affected-only
selection or execution.

### Renewable profiles

Record lock universe and target-active closure separately. Validate each stage
as pass, fail, expected rejection, unsupported, unavailable, not observed,
stale, or unknown. Exercise bounded renewal and exact rollback.

### AI-generated patches

Freeze model, instruction, context, tools, proposed patch, deterministic scope,
human decision, and full-reference validation. Include rejected proposals,
false omissions, abstention, fallback, and patch removal.

### Native boundaries

Use native owner tools in the reference lane. Exercise positive and negative
ABI, allocation, panic, threading, layout, linking, loading, runtime, and
uninstall behavior.

### Platform targets

Execute native target stages where support is claimed. Cross-check or
cross-build results remain separate from native execution.

### Upstream packets

Minimize and redact locally, validate the reproducer in a clean environment,
identify the current owner, and stop at submission-ready state. External
posting requires separate approval.

## Stop conditions

Stop and classify rather than improvising when:

- owner truth is unavailable;
- exact identity cannot be reconstructed;
- a secret or private datum enters the fixture;
- the full reference cannot run;
- expected process cardinality is not met;
- harness or scorer qualification fails;
- a held-out oracle leaks;
- unsupported behavior is being converted into success;
- cleanup or rollback cannot be verified; or
- ordinary Cargo and owner-native operation cannot be restored.
