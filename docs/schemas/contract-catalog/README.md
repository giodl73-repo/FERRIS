# Ferris Contract Catalog Schema

Status: Implemented incubation contract

`ferris.contract-catalog/v1` is the record emitted by `ferris contracts` and
`cargo ferris contracts`. It reports the exact schema identifiers the running
binary accepts and emits.

The catalog is deterministic, sorted by schema identifier, and bound to the
Ferris command version. `accepted` means the binary has an implemented local
consumer for that exact identifier. `emitted` means the binary may produce the
identifier. `legacy_read_only` records remain accepted for compatibility but
are never newly emitted.

The catalog does not negotiate versions, migrate records, establish semantic
compatibility between versions, or make a production-support commitment.

An adopter can evaluate explicit exact-handling requirements against the
catalog with `ferris contracts --requirements <JSON>`. See the
[contract compatibility schemas](../contract-compatibility/README.md).
