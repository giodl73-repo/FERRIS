# Pulse 06: Blind Doctor Fixture Remediation

Status: Validated on Windows and Unix; replacement held-out fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

An independent custodian designed and sealed FHIF-013 without reading Ferris
implementation source or tests. The fixture failed cutoff
`95a0b905fb31c908a241d57ae17d984e16d8c053` under requirement
`P05-PASSIVE-DOCTOR-BOUNDED-IDENTITY`.

Only public-safe remediation was released: tighten Cargo evidence
classification and post-read report identity. FHIF-013 is now development
evidence and MUST NOT be rescored.

## Required corrections

- accept only canonical single-line Cargo version evidence:
  `cargo <stable-semver>` with an optional canonical commit/date tuple;
- reject leading/trailing whitespace, tabs, embedded line breaks, NUL,
  malformed semantic versions, malformed commits/dates, and additional
  unclassified tokens;
- expose only safe parsed semantic version, commit, and release date fields;
- bind Cargo commit/date and framed owner-output digest into report and
  invocation identities;
- expose manifest, timeout, stdout, stderr, and framing bounds in the typed
  doctor record and human output;
- bind command, selected-manifest-directory semantics, every resource bound,
  output framing, passive controls, toolchain selection, and owner evidence
  into doctor identity; and
- preserve all Pulse 05 bounds, privacy, failure classes, and command limits.

## Acceptance

- canonical current Windows and Unix Cargo output succeeds;
- malformed near-canonical owner output is unsupported;
- report and invocation identity change with every material owner-evidence or
  bound input;
- human and JSON records expose the same safe evidence and bounds;
- Windows and Unix format, tests, lint, and diff checks pass; and
- an independently sealed replacement fixture, not FHIF-013, passes before a
  held-out doctor claim is made.
