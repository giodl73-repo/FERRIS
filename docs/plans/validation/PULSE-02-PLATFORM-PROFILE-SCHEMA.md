# Pulse 02 Platform Profile Schema Validation

Date: 2026-08-12
Implementation cutoff: `9ab231b9a347885e873de0cd76de8d2e2fa0fa7f`
Disposition: Windows development validation passed
Evidence class: Schema contract validation

## Scope

This receipt validates the frozen `ferris.platform-profile/v1` JSON Schema,
the incomplete pure-data schema exemplar, and the schema-control manifest. It
does not execute the mutation controls; Pulse 03 owns that harness.

## Environment

- operating system: Windows build 26310, x64;
- Python: 3.14.2;
- `jsonschema`: 4.26.0; and
- validator: `Draft202012Validator`.

## Validation

The validator:

1. parsed the schema, base fixture, and control manifest as UTF-8 JSON;
2. selected the Draft 2020-12 validator from the schema declaration;
3. validated the JSON Schema document itself;
4. validated the base fixture with format checking enabled;
5. required the control-manifest schema identity; and
6. required exactly nine frozen control definitions.

Result:

```text
schema=valid
base_fixture=valid
control_definitions=9
```

## Frozen file digests

| File | SHA-256 |
|---|---|
| `docs/schemas/platform-profile/ferris.platform-profile.v1.schema.json` | `c4db7d7c8d96c8447b4b93dfaa3926f4e9c7f2db8a2b7d9110c1cfb6f2d7bea5` |
| `tests/fixtures/platform-profiles/schema/valid/pure-data-r1.json` | `3b350564215cfc52910b23dd49a9ebc319511e7885126314cae454ce0bf58c47` |
| `tests/fixtures/platform-profiles/schema/controls.json` | `ac450ddd300cda73358f9984c919626619845be8d552fd30df593692a51f8abd` |

## Claim boundary

The result establishes schema-document validity and base-fixture conformance
at the recorded cutoff. It does not establish:

- a repository-owned parser or validator;
- duplicate-member or byte-bound enforcement;
- execution of the nine mutation controls;
- Windows/Unix harness parity;
- a completed profile family;
- owner-observed package, closure, target, stage, support, or lifecycle
  evidence;
- RUNE v1 completion; or
- PLATFORM-001 Proposed status.
