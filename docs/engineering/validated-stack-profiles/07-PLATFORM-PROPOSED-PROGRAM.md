# PLATFORM-001 Proposed Program

Status: Active program map
Implementation authority: None

## Purpose

This document maps the measured work required to decide PLATFORM-001 Proposed
status. It refines the
[Validation Roadmap](06-VALIDATION-ROADMAP.md) without replacing the
normative
[Platform Profile Contract](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md).

Controlled repository-local consumers provide development evidence. Three
public repositories and their hidden changes belong only to an independent
held-out custodian.

## Requirement-to-evidence map

| Contract | Required evidence | Program stage | Proposed blocker until |
|---|---|---|---|
| PLATFORM-001 | exact selection, lock universe, active closures, environments, stages, assurance, stewardship, support, lifecycle, renewal, substitution, removal, and rollback | Pulses 02-18 | every acceptance criterion has exact measured evidence |
| CONTRACT-001 | distinct Rust, semantic, ABI, WIT, wire/data, and projection identities where applicable | Pulses 02 and 04-13 | selected contract versions and directional results are frozen |
| EVIDENCE-001 | owner-declared, observed, normalized, inferred, conflicting, stale, unavailable, and unknown claims | Pulses 03 and 13 | source, owner, command, environment, expiry, limitation, and removal are retained |
| VALIDATION-001 | stage-specific positive, rejection, failure, unsupported, unavailable, stale, and unknown results | Pulses 03-17 | selected and full-reference evidence and capability consequences remain distinct |
| TRUST-001 | provenance, privacy, expiry, revocation, consumer-scoped use, and deletion/removal limits | Pulses 02, 11, 12, and 15-17 | secrets are excluded and trust claims cannot replace compatibility or validation |
| PRODUCT-001 | ordinary Cargo preservation and canonical Removal Record | Pulse 16 | owner-native workflows pass before, during, after, and after cleanup |
| CONFORMANCE-001 | schema, evidence, identity, platform, failure, interop, safety, and removal proof | Pulses 03-18 | Windows/Unix and independently held-out controls are valid |

## Canonical fixture boundary

Pulse 02 must define a versioned canonical fixture contract independently of
the experimental `ferris.profile-evidence/v0` input. The canonical contract
must preserve:

- profile, consumer, requirement, selection, source, lock, closure, feature,
  contract, environment, stage, assurance, stewardship, support, and lifecycle
  identity;
- exact source revisions and owner evidence;
- typed pass, fail, expected-rejection, unsupported, unavailable,
  not-observed, stale, conflicting, revoked, blocked, and unknown states;
- freshness, expiry, renewal, substitution, rollback, removal, and
  supersession;
- output-visible metadata and secret-bearing value boundaries; and
- deterministic machine and human projections.

A projection into `ferris.profile-evidence/v0` may support the existing
bounded diff command. It must record loss and must not claim that the
experimental schema is the canonical profile.

## Family completion contract

Each of the nine families must contain:

1. one named consumer operation;
2. exact controlled `r1` and material-change `r2` revisions;
3. manifest, lock, source, compiler, host, target, feature, provider, native,
   runtime, contract, deployment, and environment identities where
   applicable;
4. lock universe and target-active normal, build, and development closures;
5. positive, expected-rejection, failure, unsupported, unavailable, stale,
   and unknown stage evidence;
6. dated assurance, stewardship, support, expiry, and limitation records;
7. adoption, renewal, substitution, emergency, rollback, and removal
   expectations; and
8. a family-specific nine-role checkpoint.

The required families are:

| Family | Distinguishing evidence |
|---|---|
| Pure data | deterministic data contract, error behavior, source/lock closure, ordinary library consumer |
| CLI and configuration | argument/config precedence, filesystem boundary, diagnostics, packaging, installation, rollback |
| Hosted service | runtime, network/service contract, data state, deployment, operational validation, recovery |
| Embedded and `no_std` | target specification, panic, allocator, linker/runner, device availability, host rejection |
| Browser WASM | target, generated bindings, browser/runtime compatibility, web deployment, unsupported host behavior |
| WebAssembly component | WIT package/world, bindings, WASI/runtime, capabilities, version skew, component execution |
| Native dependency | native source mode, ABI, compiler, linker, loader, ownership, allocation, runtime, deployment |
| Identity and provider | provider identity, credentials boundary, trust, revocation, substitution, support, emergency response |
| Assurance and deployment | validation gates, artifacts, provenance, policy, packaging, signing/attestation, deployment, rollback |

## Lifecycle gates

### Renewal

One low-risk real dependency renewal must freeze the baseline, produce an
exact evidence diff, execute affected positive and negative stages, record the
consumer decision, restore the prior state, and prove exact rollback identity.
The researched Clap 4.6.5 to 4.6.6 change is preferred only if licensed source
artifacts, checksums, and offline reproducibility qualify.

### Substitution and emergency

One provider or implementation substitution must preserve or explicitly
revise the consumer contract and exercise migration, coexistence, negative
behavior, rollback, and complete candidate removal. A separate emergency
case must retain stale, revoked, unavailable, failed, and unknown states and
must not let Ferris approve containment.

### Adoption and removal

Every family must run owner-native workflows before, during, and after profile
adoption and after cleanup. At least one case must emit the canonical
PRODUCT-001 Removal Record. Removal must not require a Ferris-owned resolver,
lock, registry, feature, source, environment, credential, service, or writable
shared target directory.

## Independent held-out gates

Implementation authors may review only the public contract and public-safe
results.

1. The existing profile-diff package requires 56 independently constructed
   cases on Windows and Unix, 112 complete process records, one first score,
   and permanent quarantine after failure or invalidation.
2. The maintainer workflow requires three frozen licensed public repositories:
   hosted, cross-target or `no_std`, and native-bound. Hidden changes and
   expected outcomes remain sealed. The custodian compares raw owner-tool
   investigation with profile evidence and measures cost, omissions, false
   conclusions, renewal, exact rollback, and removal.

Neither gate may be recreated, repaired, or rescored by the implementation
team after oracle exposure.

## PLATFORM-001 decision rule

PLATFORM-001 advances only when:

- all nine family revisions and commands are exact;
- every material identity and state remains distinct;
- all lifecycle controls execute;
- ordinary Cargo and non-Ferris consumers remain functional;
- the independent held-out packages are valid;
- dependency amendments are reconciled without implying unrelated
  specification advancement; and
- all nine roles accept the measured package.

Any unmet item keeps PLATFORM-001 at Draft and becomes an explicit blocker.
