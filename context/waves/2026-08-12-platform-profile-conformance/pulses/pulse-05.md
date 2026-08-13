# Pulse 05: CLI and Configuration Profile Family

Status: Complete; Windows and Unix development gates passed
Implementation authority: Bounded to this document

## Goal and authority

Complete the exact CLI/configuration family and introduce reusable test-only
family support so later fixtures do not copy the pure-data harness.

Revision `r1` resolves a required name with this precedence:

```text
--name > FERRIS_FIXTURE_NAME > built-in default
```

Revision `r2` adds one explicit, bounded UTF-8 configuration file:

```text
--name > --config name=<value> > FERRIS_FIXTURE_NAME > built-in default
```

This pulse authorizes:

- two zero-dependency local CLI consumers with exact lockfiles;
- process-boundary positive, precedence, malformed, missing-file, oversized,
  non-UTF-8, and unknown-argument tests;
- reusable test-only family manifest, profile materialization, digest,
  snapshot, and owner-command support;
- migration of no production code and no public API;
- locked/offline metadata, check, build, Clippy, unit/integration test,
  doctest, and package commands in isolated target directories;
- complete v1 stage, evidence, support, and planned lifecycle records;
- stable source and profile digests;
- Windows and Unix validation; and
- one family-specific nine-role review.

## Acceptance

- `r1` and `r2` preserve their exact precedence rules;
- `r2` reads only the explicit config path and enforces a 1 KiB bound;
- malformed, missing, oversized, non-UTF-8, and unknown input exits are
  deterministic and tested;
- output and diagnostics contain no environment dump or secret value;
- owner commands pass without changing either consumer tree;
- materialized profile digests are exact and distinct;
- configuration-file behavior appears only in `r2` requirements, contracts,
  stages, and limitations;
- unsupported deployment and unobserved signing/rollback stay explicit;
- no dependency or production command is added;
- Windows and Unix use Rust/Cargo 1.95.0; and
- all nine roles accept the measured result.

## Stop conditions

Stop rather than widening if work requires an external parser, discovery
outside the explicit file, network, credentials, installation, shell
completion, mutation, production profile generation, another family, support,
approval, or hidden held-out access.

## Evidence

- [Validation receipt](../../../../docs/plans/validation/PULSE-05-CLI-CONFIG-FAMILY.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-05-CLI-CONFIG-ROLE-REVIEW.md)
