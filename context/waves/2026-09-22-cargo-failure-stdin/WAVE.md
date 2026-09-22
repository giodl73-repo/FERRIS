# Wave: Cargo Failure Standard Input

Status: Complete through Pulse 01
Implementation authority: Pulse 01 only

## Systems-development gap

Passive Cargo failure diagnosis requires a temporary file even when CI already
has the complete diagnostic in a pipeline.

## Measurable decision

Accept `--stderr -` as one EOF-delimited standard-input source while preserving
the existing 64 KiB bound, report identity, privacy boundary, and no-execution
behavior.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Add bounded stdin ingestion, entrypoint parity, negative controls, and docs | Complete |

No schema revision, incremental streaming report, batch mode, Cargo execution,
raw-output retention, or shell-pipeline orchestration is authorized.
