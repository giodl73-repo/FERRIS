# Application readiness fixtures

These public fixtures freeze APP-READINESS-001 without authorizing an
implementation.

- `application.json` is the explicit three-workspace application definition.
- `alpha`, `beta`, and `gamma` contain real minimal workspace-root manifests;
  `alpha/nested` supports the nested-root negative control.
- `requirements-*.json` are independently bound READINESS-001 declarations.
- `request-valid.json` binds all three independent Cargo workspaces.
- `report-blocked.json` proves one ready workspace cannot hide one blocked
  workspace.
- `report-stale.json` proves changed composition inputs outrank ready child
  reports.
- `report-workspace-stale.json` proves a stale child report outranks otherwise
  current composition input.
- `controls.json` defines structural and semantic mutations.

Fixture digests bind the exact committed input bytes. Aggregate report IDs bind
canonical JSON with `report_id` omitted. Child report IDs are illustrative
references, stable per workspace and child status across these fixtures,
because child reports remain governed by the existing READINESS-001 fixture
family.

Controls marked `invalid` MUST fail JSON Schema validation. Controls marked
`invalid-semantic` may pass structural validation and MUST fail the normative
cross-record checks in APP-READINESS-001.

Control operations are JSON Pointer mutations over the named base record.
`append-byte` changes exact file bytes without changing parsed JSON;
`append-workspace` adds the supplied structurally valid application workspace;
and `multi` applies every listed mutation as one control. `rebind` names digest
fields that MUST be recomputed after mutation so the intended later semantic
gate is reached. The digest-mismatch control intentionally has no `rebind`.
A future conformance harness MUST implement these operations exactly rather
than infer a different mutation.
