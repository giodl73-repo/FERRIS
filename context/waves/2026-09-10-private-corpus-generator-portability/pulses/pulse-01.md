# Pulse 01: Deterministic Topology Serialization

Status: Complete
Implementation authority: Private `EC-04` owner tooling only
Budget: One root-cause investigation, one correction, and one private hosted
validation pull request

## User outcome

Restore byte-for-byte cross-platform reproducibility of the existing private
`EC-04` generated corpus so it can become eligible for a later, separately
authorized Ferris qualification.

## Required investigation

Before editing, compare the checked-in and freshly generated topology bytes and
determine whether the mismatch is content, encoding, newline, or final-terminator
behavior.

## Authorized files

Private corpus only:

- `tools/generate-workspace.ps1`;
- `tools/verify-generated.ps1` only if the existing strict comparison is itself
  incorrect; and
- `.gitattributes` only if explicit generator bytes cannot provide the portable
  contract.

Generated crates, `TOPOLOGY.json`, scenario tags, scenario documentation, Cargo
manifests, owner commands, and Ferris files MUST NOT change semantically.

## Acceptance criteria

- investigation records the exact byte-level cause privately;
- generation produces the existing checked-in bytes on Windows;
- a second independent generation produces the same bytes;
- all existing Windows owner checks pass;
- the private pull request's hosted Ubuntu owner checks pass; and
- the change contains no generated-file or scenario delta.

## Result

Byte-level comparison proved the checked-in `TOPOLOGY.json` and a fresh Windows
generation were semantically equal but byte-different:

- the checked-in file contained eight LF terminators and no CR bytes;
- the Windows-generated file contained eight CRLF terminators;
- the generated file was eight bytes longer; and
- both parsed to the same JSON value.

The private owner generator replaced platform-native text output with explicit
UTF-8 without BOM and LF normalization. No generated file, topology value,
scenario, manifest, owner command, dependency, or Ferris file changed.

Validation completed as follows:

- two independent Windows regeneration checks passed byte-for-byte;
- Windows formatting, Clippy, all 96 package locked/offline tests, and the
  repository generated-corpus verifier passed;
- the one-file private pull request's hosted Ubuntu owner workflow passed; and
- the private pull request was merged only after separate explicit user
  direction.

Exact private revision, pull request, run, hash, path, and raw-output evidence
remain in private custody.

## Role dispositions

The completed dispositions are recorded in
[`Private corpus generator portability role review`](../../../../docs/plans/reviews/FERRIS_PRIVATE_CORPUS_GENERATOR_PORTABILITY_ROLE_REVIEW.md).

## Closeout

The pulse ended after the private pull request and hosted result were recorded.
The later merge used separate explicit user direction. A new Ferris corpus
qualification still requires a separate pulse over the immutable corrected
private cutoff.
