# Pulse 11: Explicit Selection and Request Identity

Status: Validated on Windows and Unix; new replacement fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

Semantically qualified FHIF-024 completed all 34 frozen processes and failed
the public canonical-evidence contract. It is development evidence and MUST
NOT be rerun or rescored.

Public categories included coherent document, result, and invocation
relationships. Direct contract audit found that doctor success and failure
used different invocation-identity inputs for the same selected manifest, and
the command envelope did not expose selection identity explicitly.

## Corrections

- add explicit `selection_identity` to every command-result envelope;
- advance the command-result envelope to `ferris.command-result/v2` because
  selection identity is a required field;
- bind selection identity into complete `result_identity`;
- derive doctor success and post-read failure invocation identity from the
  same workspace and selection identity;
- keep result diagnostics, evidence, classification, and exit behavior in
  result identity rather than request invocation identity;
- provide portable request selection identity for pre-selection failures; and
- provide privacy-safe selection identity for invalid CLI envelopes.

## Acceptance

- every machine result exposes selection, invocation, and result identity;
- the same doctor request and selected manifest retain selection and
  invocation identity across success and failure outcomes;
- changed outcome evidence changes result identity without changing request
  identity;
- selection identities retain no absolute checkout path;
- Windows and Unix formatting, tests, lint, and changed-file diff checks pass;
  and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.
