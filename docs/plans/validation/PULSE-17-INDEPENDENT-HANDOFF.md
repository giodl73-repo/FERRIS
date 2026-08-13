# Pulse 17 Independent Validation Handoff

Date: 2026-08-13
Contract revision: 2
Disposition: Ready for independent repository selection and scorer preflight; execution blocked

## Frozen inputs available to the custodian

- public 56-case matrix and process cardinality;
- custody, preflight, privacy, quarantine, and first-score rules;
- exact public identity derivations and synthetic digest vectors;
- complete Draft 2020-12 output, collection, environment, owner, comparison,
  lifecycle, and immutability schemas;
- exact three-public-repository slot, eligibility, owner-command, sealed
  change, projection, comparison, rollback, removal, cleanup, and threshold
  contract;
- public synthetic preflight fixtures and repository tests;
- immutable repository history containing all nine controlled families and
  lifecycle evidence;
- Windows and Unix Rust/Cargo 1.95.0 development receipts.

## Inputs intentionally unbound

- hidden before/after values and privacy canaries;
- oracle predicates and expected identities;
- executable sealed package;
- independent custodian identity;
- three public repository names, revisions, and hidden changes;
- collection environment identities; and
- score and validity result.

These fields are unbound because the implementation author is not permitted to
choose or observe them. This is a real governance blocker, not missing
implementation work.

An independent custodian may now select the three repositories and run public
scorer qualification. This handoff does not claim that selection, preflight,
sealed construction, collection, scoring, repository execution, or a pass has
occurred.
