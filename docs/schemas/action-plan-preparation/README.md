# Action Plan Preparation Schema

Status: Implemented experimental input contract

`ferris.action-plan-lanes.v1.schema.json` defines the explicit owner policy
accepted by `prepare-action-plan --lanes`. It supplies repository and topology
identity plus an ordered bounded lane list. Each lane names an existing owner
entrypoint and states its gate, requiredness, earlier dependencies, timeout,
and output bounds.

JSON Schema checks structure and scalar bounds. The execution contract also
requires unique lane IDs, dependencies that name only earlier lanes, valid
entrypoint references, exact declaration identities, current source revision,
and unchanged bound files before atomic output.

The record contains no command, approval, environment value, or executable
path. Commands come only from the independently content-bound
`ferris.owner-entrypoints/v1` declaration.

Normative semantics are in the
[Action Plan Execution Contract](../../specs/FERRIS_ACTION_PLAN_EXECUTION_CONTRACT.md).
