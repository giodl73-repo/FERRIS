# Wave: First-Class Dependency Failure Diagnostics

Status: Complete through Pulse 01
Implementation authority: Pulse 01 only

## Systems-development gap

Ferris currently collapses Cargo dependency, lockfile, offline-source, and
unknown metadata failures into one `FERRIS-CARGO-METADATA-BLOCKED` diagnostic.
Maintainers and automation cannot distinguish an observed dependency boundary
from another fail-closed metadata failure without inspecting private Cargo
output.

## Measurable decision

Classify only stable Cargo metadata failure shapes into path-free diagnostic
codes inside the existing command-result envelope. Preserve malformed manifests
as invalid and preserve an unknown blocked fallback.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Implement dependency, lockfile, and offline diagnostic classes with focused tests | Complete |

No new command, owner execution, raw-output retention, dependency-name
extraction, rustc parsing, root-cause claim, or command-result schema change is
authorized.
