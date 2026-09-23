# Owner Executable Staging Schema

Status: Implemented experimental output contract

`ferris.owner-executable-staging-receipt.v1.schema.json` defines the successful
output from `stage-owner-executable`. It records only the portable
repository-relative destination, exact staged byte identity and length, and a
deterministic staging identity. The selected source path is not retained.

The receipt is evidence of bounded local materialization, not installation,
approval, or execution authority. Entrypoint binding independently hashes the
destination bytes. Normative behavior is in the
[Action Plan Execution Contract](../../specs/FERRIS_ACTION_PLAN_EXECUTION_CONTRACT.md#owner-executable-staging).
