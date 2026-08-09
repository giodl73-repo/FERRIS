# PERF-Q33: Filesystem, Memory, Virtualization, and Hardware Effects

**Status:** Complete

**Area:** System effects

**Depends on:** PERF-Q01

## Question

How much Rust latency variation comes from storage, memory pressure, antivirus,
indexing, virtualization, thermal state, and CPU topology rather than compiler
algorithms?

## Answer

Rust build latency can be dominated by system placement. In one WSL2 control,
moving identical source and target work from ext4 to the mounted Windows
filesystem changed the clean-build median from 4.81 to 16.52 seconds and the
warm no-op median from 57.8 milliseconds to 13.68 seconds.

CPU scheduling also showed a plateau rather than a logical-core rule: eight,
sixteen, and twenty-four Cargo jobs were within 5.3%, while one job was 96.9%
slower than sixteen. Memory reserve, concurrent sessions, VM limits, security,
indexing, power, and thermal state must remain explicit environment dimensions.

## Investigation focus

- Record system conditions and controlled environment comparisons.
- Separate reproducible compiler work from environmental variance.
- Define environment warnings without prescribing unsafe exclusions.

**Model changes if:** variation remains small relative to compiler-component
differences.

## Decision informed

Define benchmark controls and environment diagnostics.

## Decision

Adopt a read-only environment fingerprint and comparison guard. Record source,
target, Cargo home, cache, VM, CPU, memory, protection, indexing, power, and
thermal state with attribution confidence. Prototype only supported
diagnostics. Reject automatic security exclusions, service or power changes,
forced pressure, repository migration, and universal job settings.

## Result

The decision, findings, measurements, and role review are recorded in
[system effects on Rust build latency](../2026-08-09-system-effects-build-latency.md).

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
