# Pulse 17: Cross-Platform Profile Diff Development Validation

Status: Complete; Windows and Unix development gates passed
Implementation authority: Validation evidence only

## Goal and authority

Execute the existing Ferris workspace and Pulse 15 nine-family
`profile-diff` conformance matrix on Windows and Unix from one immutable
implementation cutoff.

This pulse authorizes validation and public-safe evidence only. It adds no
command, schema, runtime behavior, fixture case, profile generation, owner
execution, support claim, hidden held-out material, or PLATFORM-001 status
change.

## Evidence cutoff

Both platform runs used:

```text
f9305bdb5696da4889864b9c885ab4e18a56cdba
```

Both used Rust 1.95.0 and Cargo 1.95.0. The Unix run used Ubuntu 24.04.4 LTS
under WSL2, Cargo offline mode, and an isolated `/tmp` target directory.

The complete environment, commands, results, line-ending observation, and
claim boundary are recorded in the
[cross-platform validation receipt](../../../../docs/plans/validation/PULSE-17-CROSS-PLATFORM-VALIDATION.md).

## Acceptance

- Windows formatting, workspace tests, Clippy, and diff checks pass;
- Unix formatting, workspace tests, and Clippy pass;
- each platform executes 63 passing tests with 2 ignored helper tests;
- the nine-family profile-diff CLI matrix executes on both platforms;
- both runs use the same source cutoff and Rust/Cargo versions;
- Unix build output is isolated from Windows build output;
- the WSL line-ending limitation is preserved rather than hidden;
- all nine roles accept the development-evidence boundary; and
- changed documentation passes repository validation.

## Stop conditions

Stop rather than widening this pulse if work requires:

- changing product code or development fixtures;
- constructing, viewing, executing, or scoring a hidden held-out package;
- treating WSL as universal Linux or native-platform support;
- invoking Cargo or another owner tool from the Ferris product;
- generating or approving a profile;
- converting test success into compatibility, support, security, freshness,
  readiness, or approval; or
- advancing PLATFORM-001.

## Remaining gates

The Pulse 16 held-out program remains entirely outstanding. An independent
validation owner must still construct, qualify, freeze, execute, and score
the 56-case package on both platforms for exactly 112 held-out process
records.
