# Pulse 14 Nine-Role Implementation Review

Date: 2026-08-12
Pulse: Local Profile Evidence Diff
Disposition: Accepted as complete after required corrections and local validation
Implementation authority: Pulse 14 only

## Review question

Does the bounded `profile-diff` implementation compare two explicit
`ferris.profile-evidence/v0` fixtures without owner execution, network,
mutation, raw section-value output, support inference, unstable identity, or
unbounded results?

## Initial findings

The first implementation review withheld completion for:

1. duplicate JSON object members being collapsed before canonicalization;
2. pre-read request identities hashing only path suffixes;
3. output-visible identifiers and object keys accepting control characters
   and lacking an explicit privacy boundary;
4. an empty `unknowns` record despite unassessed semantic and authority
   domains; and
5. missing array, empty-container, exact-bound, duplicate-member,
   second-input, identity-collision, and output-visibility tests.

## Corrections completed

- Added a recursive JSON value deserializer that rejects duplicate object
  members before constructing `serde_json::Value`.
- Classified duplicate members and unsafe output-visible metadata as
  `invalid` with process exit 2.
- Bound pre-content request identities to complete lexically normalized paths.
  After the first read, second-input failures bind the first canonical content
  digest and complete normalized second request. Successful identities remain
  content-based and relocation-independent.
- Limited `profile_id`, `revision`, `consumer`, and every JSON object member
  name to 1 through 256 bytes of visible ASCII `!` through `~`.
- Retained `/` and `~` in object keys and verified RFC 6901 token escaping.
- Documented that identifiers, revisions, consumers, and object keys are
  output-visible metadata that must not contain secrets. Raw section values
  remain represented only by digests.
- Added explicit unknowns stating that semantic equivalence, compatibility,
  support, freshness, approval, and decision authority are not assessed.
- Added tests for all reported boundary and failure cases, including exactly
  10,000 changes succeeding and 10,001 changes blocking.

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

No `unsafe`, concurrency, aliasing, ownership, or lifetime boundary was added.
Recursive parsing remains bounded by the 1 MiB input limit and serde JSON
recursion behavior. Compiler acceptance is not presented as behavioral proof.

### Compiler Performance Engineer

**Disposition:** Accept for this bounded command.

The command performs no build work or performance inference. Inputs remain
bounded to 1 MiB each and emitted changes to 10,000. No performance claim is
made.

### Interop Boundary Auditor

**Disposition:** Accept after duplicate-member correction.

JSON object identity is now unambiguous before canonicalization. JSON Pointer
tokens escape `/` and `~`, arrays use documented positional paths, and empty
containers retain explicit added or removed records.

### AI Assurance Skeptic

**Disposition:** Accept.

The record now states the semantic, compatibility, support, freshness,
approval, and authority domains it does not assess. Failures remain typed and
no observation is promoted into support or compatibility.

### Ecosystem Strategist

**Disposition:** Accept.

The implementation compares caller evidence only. It creates no profile
generator, distribution, registry, resolver, owner replacement, or stable
PLATFORM-001 schema claim.

### Rust Maintainer

**Disposition:** Accept after identity and output-boundary correction.

Pre-read failures no longer collide on common filenames or suffixes.
Output-visible metadata is bounded and single-line, diagnostics remain typed,
and removal remains deletion of the command and ordinary transient build
output.

### Native Platform Adopter

**Disposition:** Accept for local validation.

Windows- and Unix-shaped lexical path tests execute without filesystem
assumptions. No ABI, native tool, deployment, installation, or platform
support claim is introduced. Unix execution remains a future independent
validation opportunity, not a current support claim.

### Scope Keeper

**Disposition:** Accept.

The implementation remains one local, read-only, two-file evidence diff. It
does not generate profiles, invoke owner tools, discover repositories, contact
networks, mutate state, approve decisions, or infer support.

### Validation Checker

**Disposition:** Accept after expanded tests and full local validation.

Positive, difference, invalid, unsupported, incomplete, blocked, privacy,
canonicalization, identity, array, empty-container, exact-bound, and
second-input cases are covered.

## Validation evidence

Environment: Windows_NT, repository-local recorded Rust toolchain.

Commands and results:

```console
cargo fmt --all --manifest-path C:\src\FERRIS\Cargo.toml -- --check
```

Passed.

```console
cargo test --locked --workspace --manifest-path C:\src\FERRIS\Cargo.toml
```

Passed: 62 executed tests, 0 failed, 2 ignored helper tests, and 0 doctest
failures.

```console
cargo clippy --locked --workspace --all-targets --manifest-path C:\src\FERRIS\Cargo.toml -- -D warnings
```

Passed with no warnings.

```console
git -C C:\src\FERRIS diff --check
```

Passed.

Markdown local-link and balanced-fence checks passed after this review record
was added.

## Remaining gates

- No held-out profile-diff claim exists.
- No production, compatibility, support, certification, approval, or stable
  PLATFORM-001 schema claim is authorized.
- Cross-platform owner execution is not required by this local fixture pulse;
  future support claims require separate Windows and Unix evidence.
- Profile generation, owner adapters, package selection, mutation, remote
  evidence, and durable records remain deferred.

## Decision and authority

All nine roles accept the corrected implementation as complete for Pulse 14.

Authority remains limited to the local read-only `profile-diff` command over
two explicit experimental fixture files. This review grants no authority
beyond Pulse 14.
