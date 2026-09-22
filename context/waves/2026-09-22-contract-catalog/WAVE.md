# Wave: Installed Contract Catalog

Status: Complete through Pulse 01
Implementation authority: Pulse 01 only

## Systems-development gap

Ferris exposes versioned records across planning, readiness, execution, replay,
scheduling, and artifact qualification, but an adapter cannot ask an installed
binary which exact schema identifiers it accepts or emits. Source inspection
is not an operational compatibility contract.

## Measurable decision

Add one deterministic, read-only catalog over implemented local schema
handling. The catalog must distinguish accepted, emitted, experimental,
incubating, and legacy-read-only records without implying production support.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Implement the local contract catalog, schema, CLI parity, and tests | Complete |

No negotiation, migration, remote lookup, execution, approval, or support
commitment is authorized.
