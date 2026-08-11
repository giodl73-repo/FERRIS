# Ferris Passive Doctor Entry Review

Date: 2026-08-11
Scope: Pulse 04 passive local `doctor`
Disposition: Approved for bounded implementation
Implementation authority: Pulse 04 only

## Proposed slice

The command reads one explicit Cargo manifest and invokes `cargo --version`.
It reports prerequisite presence and evidence digests without Cargo metadata,
owner work, network access, mutation, sibling discovery, environment dumps, or
support claims.

## Role dispositions

### Rust Safety Steward

Approve. The slice uses safe Rust, executes no compiled owner code, and makes
no soundness claim.

### Compiler Performance Engineer

Approve with no performance claim. The command performs one bounded file read
and one version probe; it does not benchmark or characterize build latency.

### Interop Boundary Auditor

Approve. No ABI or language boundary is inferred. Cargo output is accepted
only through a strict version parser.

### AI Assurance Skeptic

Approve. No model participates, failed evidence cannot become success, and
digests identify retained source bytes without exposing them.

### Ecosystem Strategist

Approve. Cargo remains the owner and `cargo --version` is the conventional
prerequisite probe rather than a parallel toolchain detector.

### Rust Maintainer

Approve. Output uses manifest, Cargo, check, fallback, unknown, and limitation
terms, preserves ordinary Cargo, and is removed with the binary.

### Native Platform Adopter

Approve for Windows and Unix validation only. The pulse makes no SDK, ABI,
deployment, or production-support claim.

### Scope Keeper

Approve. One passive command and one experimental schema are authorized. All
active diagnosis and broader product capabilities remain prohibited.

### Validation Checker

Approve subject to positive, unavailable-Cargo, malformed-version,
identity, privacy, human/JSON, Windows, and Unix evidence.

## Remaining gates

- implement only Pulse 04;
- complete development and negative-control validation;
- perform independent implementation review; and
- freeze and score only if an existing sealed fixture is independently
  classified as applicable.
