# Wave: Passive Cargo Failure Diagnosis

Status: Complete through Pulse 01
Implementation authority: Pulse 01 only

## Systems-development gap

Ferris classifies failures from its own Cargo metadata calls, but ordinary
owner-run Cargo commands remain outside that boundary. Maintainers cannot use
the same path-private classification without granting Ferris execution.

## Measurable decision

Accept one complete bounded caller-supplied stderr file and emit one
deterministic passive report using only the supported dependency, lockfile,
offline-policy, and unclassified vocabulary.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Implement passive diagnosis, report schema, catalog entry, CLI parity, and negative controls | Complete |

No Cargo execution, arbitrary rustc interpretation, dependency-name
extraction, raw-output retention, synthetic-repository mutation, or root-cause
claim is authorized.
