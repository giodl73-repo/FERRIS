# Owner Entrypoint Binding Schema

Status: Implemented experimental input contract

`ferris.owner-entrypoint-intents.v1.schema.json` defines the explicit input to
`bind-owner-entrypoints`. Each intent supplies an owner command as structured
argv plus repository-relative executable, working directory, and bound-file
paths. Environment names must be unique and sorted; the implemented execution
slice supports only credential class `none`.

Ferris resolves no command through `PATH` and infers no command, owner, file,
environment, or execution policy. It validates all paths inside the canonical
current repository, hashes the bound files, binds the current Git revision,
and atomically creates `ferris.owner-entrypoints/v1` without overwriting an
existing output. Executable staging, approval, and execution remain separate
owner-controlled operations.

Normative execution semantics are in the
[Action Plan Execution Contract](../../specs/FERRIS_ACTION_PLAN_EXECUTION_CONTRACT.md).
