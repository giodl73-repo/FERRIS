# Pulse 18: Profile Diff Filesystem Immutability

Status: Complete; Windows and Unix development gates passed
Implementation authority: Conformance test and validation evidence only

## Goal and authority

Prove through the public CLI that `profile-diff` does not modify its explicit
profile inputs, add or remove files beside them, or create durable state in
its working directory.

This pulse authorizes one integration test and its evidence records. It adds
no product behavior, schema, command, profile generation, owner execution,
mutation capability, hidden held-out material, support claim, or
PLATFORM-001 status change.

## Evidence cutoff

The test implementation is frozen at:

```text
ecb10e7ed82009e1a7cf46eb585f97e3769102b8
```

Windows and Ubuntu 24.04.4 WSL2 both passed the 64-test workspace suite with
Rust and Cargo 1.95.0. The detailed test contract, environments, commands,
results, and limitations are in the
[filesystem immutability receipt](../../../../docs/plans/validation/PULSE-18-FILESYSTEM-IMMUTABILITY.md).

## Acceptance

- every Pulse 15 family pair executes through the public CLI;
- before and after files remain byte-identical;
- file lengths and modification times remain unchanged;
- input-directory membership remains unchanged;
- the isolated current directory remains empty;
- difference results still use exit 1 and stdout only;
- Windows and Unix suites pass from the same cutoff;
- the evidence states that it is not a complete sandbox audit;
- all nine roles accept the bounded claim; and
- code and documentation validation pass.

## Stop conditions

Stop rather than widening this pulse if work requires:

- changing product behavior;
- monitoring unrelated filesystem, registry, kernel, service, or network
  state;
- treating a test as universal absence-of-side-effects proof;
- constructing or accessing hidden held-out material;
- claiming complete PRODUCT-001 removal;
- invoking Cargo or owner tools from the Ferris command;
- generating or approving profiles; or
- advancing PLATFORM-001.

## Removal implication

The tested command creates no state in the explicit fixture directories or
its current directory. Removing the command therefore requires no cleanup in
those tested locations. Broader consumer adoption, ordinary Cargo
preservation, evidence retention, and PRODUCT-001 Removal Record obligations
remain separate future gates.
