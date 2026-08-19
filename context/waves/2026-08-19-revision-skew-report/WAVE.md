# Wave: Read-Only Revision-Skew Report

Status: Complete
Implementation authority: One bounded product pulse
Successor authority: None

## Decision

Add the smallest read-only command that separates explicit cross-repository
impact relationships from actual Cargo revision resolution.

## Authorized slice

- accept one strict `ferris.revision-skew-request/v0` JSON file;
- require explicit producer IDs, repository URLs, local checkout paths,
  observed 40-character revisions, consumer manifests, and package edges;
- require the claimed observed revision to equal the local producer checkout
  HEAD;
- use bounded locked/offline `cargo metadata --no-deps` for workspace-member
  dependency declarations;
- read bounded owner `Cargo.lock` files for resolved git revisions;
- use bounded local read-only Git ancestry checks;
- report `equal`, `behind`, `ahead`, `divergent`, `unavailable`, or `unknown`;
- retain no checkout, manifest, lockfile, or workspace-root paths in output;
  and
- preserve all planner, Cargo, repository-owner, and execution boundaries.

## Stop conditions

Stop rather than expand if the command requires network access, relationship
discovery, manifest or lockfile mutation, dependency updates, checkout
movement, validation execution, semantic compatibility inference, or remote
repository authority.

## Completion

Completion requires:

- typed CLI and Rust records;
- strict request, path, cardinality, and revision validation;
- synthetic coverage of all four ancestry outcomes plus unavailable and typed
  schema rejection;
- replay over the exact public shared-substrate diamond;
- command and boundary documentation; and
- no child-repository mutation.

See [`Pulse 01`](pulses/pulse-01.md) and the
[validation record](../../../docs/plans/validation/FERRIS-REVISION-SKEW-REPORT.md).

## Removal

Remove the command, records, tests, documentation, and this wave. No input or
child repository state requires migration.
