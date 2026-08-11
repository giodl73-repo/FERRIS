# Pulse 10: Canonical Command Result Records

Status: Validated on Windows and Unix; new replacement fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

Qualified replacement FHIF-020 completed all 18 frozen cases and failed one
oracle check at cutoff `a0c2a5ba991f76ba51ecf59061bf1abf0c256a3c`
under `P09-TYPED-BOUNDED-FAILURE-EVIDENCE`. It is now development evidence and
MUST NOT be rerun or rescored.

The public release identified identity binding, success and bounded-failure
evidence completeness, result classification, and determinism. It required
one stable complete result record per invocation with classification,
identity, evidence, and exit behavior derived consistently from that record.

## Corrections

- advance the command-result envelope to `ferris.command-result/v1`;
- add a `result_identity` over the complete typed command outcome;
- include the numeric `process_exit_code` in the same identity-bound record;
- derive CLI process exit behavior from the envelope rather than separately
  from success or error branches;
- preserve invocation identity as request identity instead of binding outcome
  fields into it;
- require every diagnostic class to match the enclosing result class; and
- canonicalize invalid-CLI identity across executable roots, checkout roots,
  equivalent option spellings, and format case while retaining privacy-safe
  typed value distinction.

## Acceptance

- every JSON invocation emits one complete `v1` command-result record;
- identical requests and outcomes have identical invocation and result
  identities;
- changed outcomes retain invocation identity but change result identity;
- `process_exit_code` equals the documented code for `result_class`, and the
  CLI exits with that recorded code;
- invalid CLI identities do not retain executable or absolute manifest paths;
- distinct hidden argument values do not collapse into one identity;
- Windows and Unix formatting, tests, lint, and changed-file diff checks pass;
  and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.
