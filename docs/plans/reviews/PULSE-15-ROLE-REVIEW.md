# Pulse 15 Nine-Role Conformance Review

Date: 2026-08-12
Pulse: Nine-Family Profile Diff Conformance
Disposition: Accepted as complete after local validation
Implementation authority: Development fixtures and tests only

## Review question

Does the Pulse 15 fixture matrix exercise the existing bounded
`profile-diff` contract across all nine independent Draft PLATFORM-001
families without creating owner truth, new product behavior, raw section-value
output, or an unsupported advancement claim?

## Evidence reviewed

- `tests/fixtures/profile-evidence/MATRIX.md`;
- eighteen explicit family fixture files;
- the data-driven CLI integration test in
  `crates/ferris-cli/tests/cli.rs`;
- Pulse 14's existing input, privacy, identity, output, and resource bounds;
- Draft `FERRIS_PLATFORM_PROFILE_CONTRACT.md`; and
- the validated-stack-profile validation roadmap.

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

No product Rust, `unsafe`, owner execution, compiler claim, or behavioral
safety claim changed. The fixtures include typed evidence states only as
uninterpreted JSON values.

### Compiler Performance Engineer

**Disposition:** Accept.

The pulse makes no build-performance claim and executes no compiler or build
tool. Nine small bounded fixture pairs add deterministic test cost only.

### Interop Boundary Auditor

**Disposition:** Accept.

Embedded, browser WASM, component, native, and provider families remain
independent. The fixtures do not claim that JSON values preserve ABI, WIT,
wire, native, runtime, or provider semantics.

### AI Assurance Skeptic

**Disposition:** Accept.

The matrix explicitly labels every pair synthetic and non-authoritative.
Distinctive raw values are tested as absent from output, while failures would
remain visible as test failures rather than success-shaped records.

### Ecosystem Strategist

**Disposition:** Accept.

The work validates Ferris's existing comparison boundary rather than creating
a resolver, profile distribution, registry, owner adapter, ranking, or
replacement for Cargo or ecosystem owners.

### Rust Maintainer

**Disposition:** Accept.

The fixture corpus is discoverable through one matrix, uses one uniform file
shape, and is removable without changing the binary or ordinary Cargo
workflows. The data-driven test reports the failing family name.

### Native Platform Adopter

**Disposition:** Accept for local development conformance.

Native and cross-target cases are represented without claiming that their
tools, targets, ABIs, deployments, or recovery paths were executed. Real
Windows and Unix owner evidence remains a later gate.

### Scope Keeper

**Disposition:** Accept.

The pulse adds fixtures, one test, and governance records only. It introduces
no command, schema, runtime behavior, profile generation, mutation, approval,
or PLATFORM-001 status change.

### Validation Checker

**Disposition:** Accept after local validation.

All nine independent families execute through the public CLI path. Assertions
cover typed result class, exit code, profile identity, changed section, exact
pointer, stderr behavior, and raw-value non-disclosure.

## Validation evidence

Environment: Windows_NT, repository-local recorded Rust toolchain.

```console
cargo fmt --all --manifest-path C:\src\FERRIS\Cargo.toml -- --check
cargo test --locked --workspace --manifest-path C:\src\FERRIS\Cargo.toml
cargo clippy --locked --workspace --all-targets --manifest-path C:\src\FERRIS\Cargo.toml -- -D warnings
git -C C:\src\FERRIS diff --check
```

All commands passed. The workspace test run executed 63 tests successfully,
with 2 ignored bounded-command helper tests and no doctest failures. Changed
Markdown local links and code fences also passed.

## Remaining gates

- These are development fixtures, not held-out evidence.
- No fixture establishes compatibility, support, correctness, security,
  freshness, readiness, certification, approval, or owner observation.
- PLATFORM-001 remains Draft and retains every stated advancement criterion.
- Profile generation, owner adapters, remote evidence, mutation, and durable
  profile records remain unauthorized.
- Cross-platform owner execution and the held-out maintainer workflow remain
  future independent gates.

## Decision and authority

All nine roles accept Pulse 15 as complete within its fixture-only authority.
The review grants no product behavior beyond Pulse 14 and no PLATFORM-001
advancement.
