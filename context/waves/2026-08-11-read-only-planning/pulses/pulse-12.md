# Pulse 12: Universal Typed Non-Success Results

Status: Validated on Windows and Unix; new replacement fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

Fully qualified FHIF-025 completed all 40 frozen processes and exposed one
non-success invocation that did not emit a parseable v2 command-result
envelope. It is development evidence and MUST NOT be rerun or rescored.

## Corrections

- emit exactly one UTF-8 `ferris.command-result/v2` envelope for every
  non-success command invocation;
- apply the same rule to Clap syntax failures even when output format could
  not be parsed;
- emit parsed-command failures as typed envelopes regardless of requested
  success-rendering format;
- keep all non-success output on stderr with stdout empty; and
- derive the actual exit code from the emitted envelope.

Help and version displays remain successful informational output.

## Acceptance

- no non-success invocation falls back to Clap prose or an untyped human-only
  diagnostic;
- every non-success envelope contains selection, invocation, and result
  identity, classification, recorded exit, diagnostics, and null
  command-specific record;
- actual and recorded exits match;
- Windows and Unix formatting, tests, lint, and changed-file diff checks pass;
  and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.
