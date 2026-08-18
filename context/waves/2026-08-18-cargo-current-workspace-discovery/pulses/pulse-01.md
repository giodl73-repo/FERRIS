# Pulse 01: Cargo Current-Workspace Discovery

Status: Complete
Implementation authority: Bounded to this document
Budget: One implementation attempt, one final review, no successor

## Frame

The shipped `cargo-ferris` adapter already provides the complete bounded
read-only command surface. Cargo is the working owner system for locating the
current workspace. The missing capability is using that owner result when the
Cargo adapter receives no explicit manifest.

The current workaround repeats `--manifest-path <Cargo.toml>` on every
`cargo ferris` invocation, making the Cargo-native entrypoint no more
convenient than standalone `ferris`.

V1 defaults only manifest selection for direct `cargo-ferris` and Cargo-style
`cargo ferris`. Standalone `ferris` retains explicit application/workspace
selection. Users retain the portable workspace identity. Cargo retains
workspace discovery and metadata semantics. Ferris retains typed result
envelopes and read-only planning semantics.

The deletion target is one repeated CLI argument, not an owner boundary. This
thesis is disproved if Cargo cannot return one workspace manifest safely, if
explicit parity changes, or if portable workspace identity must be inferred.

## Audit

- `CommandArgs` and `ValidationPlanArgs` currently require both
  `--workspace-id` and `--manifest-path`.
- `InvocationKind` already distinguishes standalone `ferris`, direct
  `cargo-ferris`, and Cargo-style `cargo ferris`.
- Every workspace command reaches Cargo metadata through an explicit manifest.
- The existing hardening record requires the portable workspace ID to remain
  explicit.
- `cargo locate-project --workspace --message-format json` returns the same
  root manifest from the repository root and a nested package source
  directory.

## Compare

| Analogue | Classification | Use |
|---|---|---|
| Existing `InvocationKind` adapter split | reuse | apply a default only to the two Cargo adapter forms |
| Existing explicit manifest behavior | reuse | explicit input always wins and preserves output parity |
| Filesystem parent crawling | avoid | would duplicate Cargo's discovery rules |
| Generated workspace identity | avoid | would weaken the explicit portable identity boundary |
| Pulse 88 diagnostic successor chain | stop | no supported product decision justifies another authority cycle |

Cargo documents `cargo locate-project` as the command for locating a manifest,
with `--workspace` selecting the workspace root and JSON output available for
machine consumption:
<https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html>.

SLSA provenance and in-toto distinguish artifact or step evidence from
permission and product value. Exact custody evidence therefore does not itself
justify another diagnostic execution:
<https://slsa.dev/spec/v1.2/provenance> and
<https://in-toto.io/docs/what-is-in-toto/>.

## Evaluate

Pre-implementation findings:

- Runtime owner: Cargo discovery must remain the sole defaulting authority.
- Affected operator: removing the repeated manifest argument is useful only if
  failures remain typed and explicit input remains stable.
- Simplicity/security lens: do not infer workspace identity, retain no
  discovered absolute path in public output, and stop if discovery requires a
  new subsystem.

## Slice

1. one representative nested current-directory fixture;
2. one adapter-only manifest-resolution seam;
3. one successful discovered result and one typed discovery failure;
4. JSON parity against the same explicit manifest; and
5. one deletion mapping: remove the repeated Cargo-adapter manifest argument.

## Authorized files

- `crates/ferris-core/src/lib.rs`;
- `crates/ferris-cli/src/entrypoint.rs`;
- `crates/ferris-cli/tests/cli.rs`;
- this wave, its final review, and directly related README guidance.

## Stop conditions

Stop rather than widening scope if the implementation requires:

- inferring or generating `workspace-id`;
- changing a record schema or command meaning;
- repository configuration or filesystem discovery outside Cargo;
- network access, validation execution, or mutation;
- another implementation attempt or successor pulse; or
- any continuation of the Pulse 83-88 authority lifecycle.

## Measured result

The one implementation attempt completed within budget:

- direct `cargo-ferris` and Cargo-style `cargo ferris` default manifest
  selection for all five existing workspace commands;
- Cargo `locate-project --workspace --message-format json` remains the sole
  discovery authority;
- explicit manifest selection bypasses discovery unchanged;
- standalone `ferris` still requires `--manifest-path`;
- portable workspace identity remains explicit and is validated before Cargo
  discovery;
- successful Cargo output remains authoritative even when Cargo emits bounded
  diagnostics on stderr;
- failed discovery produces a typed, path-free non-success result; and
- no command schema, semantic, execution authority, or platform claim changed.

The final role review is recorded in
`docs/plans/reviews/FERRIS-CARGO-CURRENT-WORKSPACE-DISCOVERY-REVIEW.md`.
