# Environment Readiness Contract Fixtures

Status: Frozen for READINESS-001 Draft review
Implementation authority: None

- `requirements-valid.json` exercises all four V1 requirement kinds,
  required/advisory policy, platform applicability, and declared-only source
  provenance.
- `report-ready.json` is the complete ready baseline and proves that no value,
  resolved path, or content is retained.
- `report-state-vectors.json` freezes required/advisory aggregation and
  VIEW-001 result classes and process codes.
- `command-result-ready.json`, `command-result-blocked.json`, and
  `command-result-invalid.json` freeze complete success, report-bearing
  non-success, and pre-report envelopes.
- `command-result-vectors.json` freezes the remaining report-bearing and
  pre-report mappings for the existing `ferris.command-result/v2` surface.
- `controls.json` defines exact structural, strict-JSON, size, secret, path,
  reference, ordering, correspondence, and aggregate-consistency mutations.

Schema-valid does not imply semantically valid. The semantic controls require
sorted unique IDs, complete source references, exact requirement/report
correspondence, root-path rules, and aggregate consistency beyond JSON Schema.

These fixtures do not execute probes and do not authorize implementation.
Repository attributes freeze this fixture family to LF so exact-byte digest
vectors remain portable across Windows and Unix checkouts.
