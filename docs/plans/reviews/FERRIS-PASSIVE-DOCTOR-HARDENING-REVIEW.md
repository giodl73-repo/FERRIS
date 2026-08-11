# Ferris Passive Doctor Hardening Review

Date: 2026-08-11
Scope: Pulse 05 corrections for `doctor` and shared Cargo toolchain context
Disposition: Validated; replacement scoring pending
Implementation authority: No expansion

## Corrected findings

1. Doctor and Cargo metadata now run from the selected manifest directory with
   the same inherited owner toolchain context and offline/no-install/no-update
   guards.
2. Doctor reads at most 1 MiB of manifest data, waits at most five seconds, and
   retains at most 64 KiB per process stream.
3. Doctor report and invocation identities include the framed Cargo owner
   output digest, separating distinct owner builds with the same semantic
   version.
4. Post-read doctor failures carry manifest-digest selection identity instead
   of checkout path identity.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

The suite reports 24 core tests passed, 2 process-helper tests intentionally
ignored except when invoked by their bound tests, and 12 CLI tests passed.
Manifest size, process timeout, output volume, owner-output identity, portable
failure identity, toolchain guards, privacy, and prior command behavior are
covered.

## Role dispositions

### Rust Safety Steward

Accept. Bounded safe-Rust process and file handling introduce no unsafe or
owner-code execution claim.

### Compiler Performance Engineer

Accept. Explicit time and memory bounds prevent the passive diagnostic from
becoming an unbounded workload; no build-performance claim is made.

### Interop Boundary Auditor

Accept. Framed stdout/stderr identity preserves the owner-output boundary and
strict parsing remains unchanged.

### AI Assurance Skeptic

Accept. All review findings have deterministic tests; no missing evidence is
converted into success.

### Ecosystem Strategist

Accept. Cargo and rustup retain toolchain ownership, including explicit
environment selection and selected-directory overrides.

### Rust Maintainer

Accept. Doctor now diagnoses the same Cargo context used by planning commands,
and bounded failures remain actionable.

### Native Platform Adopter

Accept on the recorded Windows and Unix environments. No SDK, ABI, deployment,
or support claim is added.

### Scope Keeper

Accept. The pulse is corrective and adds no command, schema, active probe, or
mutation.

### Validation Checker

Accept pending replacement held-out classification and applicable scoring.

## Remaining gate

- freeze the corrected implementation and independently classify all sealed
  fixtures before execution.
