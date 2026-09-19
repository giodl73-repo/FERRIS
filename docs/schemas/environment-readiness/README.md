# Environment Readiness Schemas

Status: Frozen Draft contract artifacts
Implementation authority: None

- `ferris.environment-requirements.v1.schema.json` defines strict explicit
  owner requirements.
- `ferris.environment-readiness-report.v1.schema.json` defines deterministic
  passive observations without values or resolved paths.
- `ferris.environment-readiness-command-result.v2.schema.json` specializes the
  existing command envelope for ready, unsupported, incomplete, stale, blocked,
  invalid, and internal results. Validators MUST register the report schema
  under its absolute `$id` when resolving the specialization.

Normative semantics are in
[`READINESS-001`](../../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md).
Frozen examples and mutation controls are under
[`tests/fixtures/environment-readiness`](../../../tests/fixtures/environment-readiness/).

JSON Schema checks structure. READINESS-001 additionally requires unique and
sorted IDs, complete requirement-to-observation correspondence, valid source
references, path normalization, aggregate consistency, and process-result
mapping.
