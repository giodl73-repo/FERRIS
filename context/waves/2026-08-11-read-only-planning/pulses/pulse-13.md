# Pulse 13: Typed Process Boundary

Status: Validated on Windows and Unix; replacement held-out fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

FHIF-028 completed all 48 frozen processes through cardinality-safe collection
and contract-conformant scoring, then failed the public-safe category
`universal typed non-success coverage`. It is quarantined and MUST NOT be
rerun or rescored.

No hidden input, command matrix, seeded value, expected record, or oracle
predicate informed this correction.

## Corrections

- construct command output in memory before writing either process stream;
- catch unwind-safe internal panics at the CLI process boundary;
- suppress the default panic hook while the guarded single-threaded command
  executes so panic prose cannot precede the typed result;
- convert a caught panic into a privacy-safe `internal` command-result
  envelope on stderr with exit 11;
- convert a failed success-output write into a typed internal result and exit
  11 when stderr remains writable; and
- retain successful help and version output on stdout with exit 0.

## Acceptance

- invalid CLI, parsed command failures, and catchable internal panics use
  `ferris.command-result/v2`;
- caught panics emit no panic payload, checkout path, raw argument value, or
  untyped panic-hook prose;
- non-success envelopes retain empty stdout, typed stderr, null
  command-specific record, and actual/recorded exit agreement;
- output write failure cannot silently retain a success exit;
- Windows and Unix formatting, tests, lint, and changed-file diff checks pass;
  and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.

## Boundaries

- the guard covers unwind panics, not process aborts or operating-system
  termination;
- stderr failure cannot be represented on the failed stderr stream;
- execution remains single-threaded; a later in-process worker model requires
  thread-owned failure capture instead of a process-global panic hook; and
- no command, owner probe, scope, connector, execution, or mutation capability
  is added.
