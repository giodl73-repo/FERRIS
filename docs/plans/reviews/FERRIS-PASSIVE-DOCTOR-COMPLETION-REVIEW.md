# Ferris Passive Doctor Completion Review

Date: 2026-08-11
Scope: Pulse 04 passive local `doctor`
Disposition: Complete; no existing held-out fixture applicable
Implementation authority: No expansion

Historical note: Pulse 05 later corrected toolchain parity, resource bounds,
owner-output identity, and portable failure identity. This review remains
evidence for the frozen Pulse 04 cutoff only.

## Measured result

`ferris doctor`:

- validates the explicit portable workspace ID;
- accepts only an existing readable file named `Cargo.toml`;
- retains the manifest SHA-256 digest but no path or contents;
- invokes only `cargo --version`;
- runs from the system temporary directory;
- selects the installed stable toolchain;
- disables rustup auto-install and update checks;
- requests Cargo offline mode;
- blocks successful probes that emit stderr;
- strictly parses a three-component stable semantic version;
- hashes framed stdout and stderr;
- emits `ferris.doctor-report/v0`; and
- makes no metadata, build-readiness, support, correctness, security, or
  performance claim.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

The final development suite contains 20 core tests and 12 CLI tests.

An independent review found five issues: rustup passivity was not enforced,
successful evidence omitted stderr, arbitrary files were accepted as
manifests, report identity omitted passive mode, and near-valid non-SemVer
versions were accepted. All five were corrected before final validation.

## Role dispositions

### Rust Safety Steward

Accept. Safe Rust remains sufficient; no owner code or unsafe boundary is
executed or claimed.

### Compiler Performance Engineer

Accept with no performance claim. The bounded file read and version probe are
not build measurements.

### Interop Boundary Auditor

Accept. Only a strict stable semantic version crosses the owner-output
boundary; all other bytes are retained only by digest.

### AI Assurance Skeptic

Accept. No model participates, diagnostics remain failures, and passive
controls are represented in report and invocation identity.

### Ecosystem Strategist

Accept. Cargo and rustup remain owners; Ferris neither installs nor resolves a
toolchain.

### Rust Maintainer

Accept. Human and JSON output expose checks, evidence, unknowns, limitations,
and ordinary Cargo fallback without paths or manifest contents.

### Native Platform Adopter

Accept for the recorded Windows and Unix renewal environments. No platform
SDK, ABI, deployment, or support conclusion is drawn.

### Scope Keeper

Accept. One passive command and one experimental schema were added. Active
doctor, query, affected scope, execution, connectors, and mutation remain out
of scope.

### Validation Checker

Accept. Positive, unavailable-Cargo, diagnostic-stderr, malformed-version,
wrong-file, identity, privacy, configuration, human/JSON, Windows, and Unix
evidence are present.

## Held-out classification

The independent custodian verified cutoff
`ba2a055735a5c6bc8530570e270b77684f996d5b`, tag
`ferris-passive-doctor-pulse-04-cutoff`, and all 12 sealed package digests.
FHIF-001 through FHIF-012 were all outside the passive-doctor scope. No fixture
was executed and no held-out pass was manufactured.

## Remaining gate

- independently seal a Pulse 04-specific passive-doctor fixture before making
  any held-out doctor claim.

## Decision

Pulse 04 is complete for development evidence on the recorded Windows and Unix
environments. It adds no authority beyond the passive local prerequisite
report.
