# Pulse 19: Ordinary Cargo Preservation Control

Status: Complete; Windows and Unix development gates passed
Implementation authority: Consumer fixture, conformance test, and evidence only

## Goal and authority

Demonstrate on one representative locked Rust consumer that ordinary
owner-native Cargo metadata and unit behavior remain available and unchanged
before and after the existing local `profile-diff` command.

This pulse authorizes one zero-dependency consumer fixture, one integration
test, and validation records. It adds no Ferris command behavior, Cargo
adapter, resolver, profile generation, mutation, owner execution from the
product, hidden held-out material, support claim, or PLATFORM-001 status
change.

## Evidence cutoff

The fixture and test are frozen at:

```text
e1b9e9d427b8bfcca7f21ce7f177fd31d6cf8960
```

Windows and Ubuntu 24.04.4 WSL2 both passed the 65-test workspace suite with
Rust and Cargo 1.95.0.

The exact test sequence, owner commands, environments, results, claim
boundary, and remaining lifecycle gates are in the
[ordinary Cargo preservation receipt](../../../../docs/plans/validation/PULSE-19-ORDINARY-CARGO-PRESERVATION.md).

## Acceptance

- the consumer has an exact lockfile and no external dependency;
- owner Cargo metadata succeeds before and after Ferris;
- parsed metadata and metadata stderr remain exactly equal;
- the owner unit test reports one pass before and after Ferris;
- before and after owner tests use separate external target directories;
- the complete consumer workspace remains unchanged after every step;
- Ferris runs from the consumer workspace without adding local state;
- Windows and Unix suites pass from the same cutoff;
- all nine roles accept the representative-control boundary; and
- code and documentation validation pass.

## Stop conditions

Stop rather than widening this pulse if work requires:

- changing product behavior or invoking Cargo from Ferris;
- external dependency, registry, feature, native, target, provider, or
  deployment claims;
- describing one fixture as universal Cargo preservation;
- claiming complete adoption, rollback, removal, or PRODUCT-001 conformance;
- constructing or accessing hidden held-out material;
- generating or approving profiles; or
- advancing PLATFORM-001.

## Remaining gates

This is one representative Phase 7 development control. Every real profile
family, adoption, renewal, substitution, rollback, and removal case still
requires its own owner-native before, during, after, and cleanup evidence.
