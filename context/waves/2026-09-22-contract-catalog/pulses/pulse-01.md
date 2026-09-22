# Pulse 01: Installed Contract Catalog

Status: Complete

## User outcome

Allow a large-repository adapter to inspect the exact record schemas supported
by its installed Ferris binary before exchanging durable records.

## Control record

- Product outcome: replace source scraping with one deterministic local query.
- Maximum effort: one core record, one CLI command, one schema, focused docs,
  and targeted tests.
- Completion test: sorted unique output, exact accepted/emitted direction,
  legacy receipt behavior, human/JSON output, and three-entrypoint parity.
- Abandonment condition: stop if the work requires version negotiation,
  migration, remote state, or a production-support promise.
- Product Value Governor: `continue-within-budget`.

## Authority

The user's instruction to continue improving Ferris authorizes this one bounded
read-only implementation pulse. It does not authorize a successor or another
architecture layer.

## Result

`ferris contracts` and `cargo ferris contracts` emit the deterministic
`ferris.contract-catalog/v1` record in human or JSON form. Every entry states
whether the exact schema is accepted, emitted, or both, and classifies V0,
current V1/V2, and legacy-read-only lifecycle posture. Unknown schemas remain
rejected. No repository, environment, remote system, approval, or executable
work is observed or changed.
