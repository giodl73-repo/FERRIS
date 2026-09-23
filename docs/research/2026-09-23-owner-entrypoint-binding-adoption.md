# Owner Entrypoint Binding Adoption

Date: 2026-09-23

Status: Complete controlled synthetic adopter evaluation

## Question

Can Ferris replace adopter-authored canonical JSON and SHA-256 identity
assembly with one bounded command while preserving exact owner argv, current
revision and file binding, and the existing no-approval/no-execution boundary?

## Control Record

- Product outcome: make valid owner entrypoint declarations a first-class
  Ferris output instead of custom adopter code.
- Ferris base: `51325a29f5c9803e5adad225a8b1c38298ab0d8d`.
- Consumer: `giodl73-repo/ferris-synthetic-chain` at
  `7ae6bfe5bcc8a76c5b141b89dbf813d2458f1817`.
- Toolchain: Cargo and rustc 1.95.0 on Windows.
- Maximum effort: one strict input schema, one CLI command, focused core/CLI
  validation, documentation, and one replay of the existing four-case chain.
- Completion: deterministic declaration output, exact revision/file/argv
  binding, direct `prepare-action-plan` consumption, and fail-closed malformed,
  duplicate, path, credential, environment, and overwrite controls.
- Abandonment: stop if the result requires executable discovery or staging,
  `PATH` lookup, approval, execution, or Action Plan V1 changes.
- Opening Product Value Governor disposition: `continue-within-budget`.

## Implemented Boundary

`ferris bind-owner-entrypoints --intents <JSON> --output <JSON>` accepts strict
`ferris.owner-entrypoint-intents/v1`. Every intent explicitly supplies its ID,
owner, repository-relative executable, exact structured argv, working
directory, sorted inherited-environment names, credential class, and bound
files.

Ferris canonicalizes the current repository, requires bounded repository-local
input and output, validates every explicit path, hashes bound files, binds Git
`HEAD`, computes the existing entrypoint and declaration identities, rechecks
input bytes, revision, and file identities, and atomically creates
`ferris.owner-entrypoints/v1` without overwrite. It does not stage or discover
an executable, search `PATH`, infer policy, create approval, or launch work.

The contract catalog now reports the intents as accepted and the existing
declaration as both accepted and emitted. Action Plan V1 is unchanged.

## Adopter Result

The synthetic chain now commits `.ferris/owner-entrypoint-intents.json`. Its
PowerShell adapter still copies the caller-selected Cargo binary into ignored
repository runtime state, then delegates declaration identity construction to
Ferris. Custom SHA-256, compact canonical JSON, entrypoint projection, and
declaration projection code were removed. The adapter fell from 197 to 124
lines; the explicit 42-line intent record is owner policy rather than identity
machinery.

The committed validation path emitted declaration
`sha256:d266db34154f600debc9ae0e1d147379094268d36ea01455e725651ee6e6f628`
and unsigned validation plan
`sha256:eed8acb07c489544c3d01d3c2bf4d6798e6ae25d70e0c593e4f478c864d8b03e`.
The same declaration identity was repeated across all failure cases.

| Case | Cargo exit | Stderr bytes | Classification | Disposition | Plan |
|---|---:|---:|---|---|---|
| Missing path dependency | 101 | 432 | `dependency` | `route` to `owner.dependencies.review` | none |
| Locked new path dependency | 101 | 244 | `lockfile` | `route` to `owner.lockfile.review` | none |
| Unavailable registry package offline | 101 | 381 | `offline_policy` | `prepare_action` for `owner.cargo-cache.populate` | unsigned V1 |
| Invalid Rust source | 101 | 532 | `unclassified` | `halt` at `owner.failure.manual-review` | none |

The offline case emitted unsigned failure plan
`sha256:5d51dc0de830b7f0ca4ea66aff1ff636132d80877dc8fa103e3fc87600188cc6`.
The owner `cargo test --workspace` oracle passed four unit tests and both
zero-test doc-test targets. No approval, Action Plan execution, or receipt was
created. The tracked adopter remained clean and all ignored runtime and
disposable replay roots were removed.

## Validation

The reproducible implementation checks were:

```console
cargo test -p ferris-cli --test owner_entrypoint_binding --test contracts
cargo check -p ferris-cli
cargo clippy -p ferris-core -p ferris-cli --all-targets -- -D warnings
```

Eight binder integration tests cover deterministic output across repeated and
byte-equivalent cloned roots, exact argv/revision/file identities, direct
Action Plan preparation, strict malformed and unknown input rejection,
the 1 MiB input bound, duplicate IDs and files, an unbound executable,
unsupported credentials, unsorted environment, missing and escaped paths,
repository-external input and output, existing output, and human output. Seven
catalog tests cover the accepted/emitted contract registration.

## Role Review

- Product Value Governor: the user-facing identity assembly gap was removed
  within the one-command budget. Closeout disposition:
  `stop-value-exhausted`; no successor is authorized by this result.
- Scope Keeper: pass. The command binds only explicit owner intent and does not
  absorb Cargo staging, policy selection, approval, or execution.
- Validation Checker: pass for the bounded Windows claim. Public-command
  positive and negative tests plus the committed four-case replay passed.
- Rust Maintainer: pass. The adopter deletes opaque cryptographic projection
  code and retains readable structured command policy. The atomic writer is
  local to the execution module and introduces no dependency.
- Native Platform Adopter: pass with limitation. Migration is one committed
  intent file plus one CLI call, but executable staging remains adopter work
  and this replay does not establish Linux or production support.
- Ecosystem Strategist: pass. Ferris is filling its own declaration boundary,
  not replacing Cargo, a task runner, or a package manager.
- Autonomy Supervisor: stop after documentation, full validation, commit, and
  push. Staging or execution onboarding would be a new product decision.

## Decision

The bounded owner-entrypoint binder is implemented and adopted by the
controlled synthetic chain. It closes the measured custom identity-assembly
gap without expanding Ferris authority. The evidence supports an incubation
usability claim only; it does not establish production adoption,
cross-platform execution, automatic tool discovery, or realized build-time
value.
