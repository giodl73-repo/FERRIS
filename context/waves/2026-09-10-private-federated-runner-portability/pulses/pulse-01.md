# Pulse 01: Relative Federated Process Arguments

Status: Active
Implementation authority: Private `EC-05` owner tooling only
Budget: One process-boundary correction and one private hosted validation pull
request

## User outcome

Restore cross-platform execution of the existing private federated scenario
matrix, including its unchanged 256-input boundaries, so the corpus can become
eligible for a later separately authorized Ferris qualification.

## Authorized file

Private corpus only:

- `tools/run-federated.ps1`.

No Ferris file, scenario, fixture, topology, manifest, owner command, expected
outcome, repetition, identity assertion, or evidence schema may change.

## Required correction

The Ferris child process MUST:

- use the declared application root as its working directory;
- receive the application file relative to that root;
- receive every in-root changed path relative to that root;
- preserve absolute input for the existing outside-application typed failure;
  and
- receive all changed packages and paths individually and unchanged.

## Acceptance criteria

- the private diff changes only the authorized runner file;
- both maximum cases still contain exactly 256 explicit inputs;
- all existing scenarios retain their expected matched result;
- all typed failures retain their diagnostic and exit-code expectations;
- repeated plan and result identities remain stable;
- the complete Windows runner emits durable evidence;
- the private pull request's hosted Ubuntu owner workflow passes; and
- no product, scenario, fixture, topology, manifest, command, or evidence-schema
  delta occurs.

## Stop conditions

Stop on any need to change Ferris, alter or omit an input, modify a scenario or
oracle, introduce batching or a response-file protocol, use another corrective
approach, or retry the completed qualification.

## Role dispositions

The eleven required role dispositions will be recorded after Windows and hosted
Ubuntu evidence exist or a stop condition is reached.
