# Pulse 01: Relative Federated Process Arguments

Status: Complete
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

The completed dispositions are recorded in
[`Private federated runner portability role review`](../../../../docs/plans/reviews/FERRIS_PRIVATE_FEDERATED_RUNNER_PORTABILITY_ROLE_REVIEW.md).

## Result

The private runner now starts Ferris with the application root as its child
working directory, passes the application file and all in-application changed
paths relatively, and preserves the existing absolute outside-application
negative control. The private diff contains one owner-tooling file with no
scenario, fixture, topology, manifest, command, expected-result, repetition,
identity, evidence-schema, dependency, or Ferris change.

Windows validation completed as follows:

- the complete owner formatting, Clippy, locked/offline test, and generated
  corpus checks passed;
- nine unchanged federated scenarios emitted durable evidence;
- eight scenarios matched pass and one matched the expected failure;
- all nine scenarios retained stable identities across ten plan repetitions;
- the maximum path-only and mixed-input cases completed with all 256 inputs;
  and
- all three typed failures retained stable diagnostic, exit-code, and result
  identities.

The one-file private pull request's existing hosted Ubuntu owner workflow
passed. That workflow does not execute the federated runner, so hosted Ubuntu
runner behavior remains not observed and is not claimed.

## Closeout

The pulse ended after the private pull request and hosted owner result were
recorded. The pull request remains open and unmerged. Merge and any fresh
Ferris corpus qualification require separate explicit direction.
