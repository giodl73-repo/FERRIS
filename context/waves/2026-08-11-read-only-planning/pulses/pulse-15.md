# Pulse 15: Nine-Family Profile Diff Conformance

Status: Complete; nine-role review and full local validation passed
Implementation authority: Development fixtures and tests only

## Goal and authority

Exercise the existing Pulse 14 `profile-diff` command across all nine
independent profile families required by Draft PLATFORM-001:

1. hosted service;
2. CLI and configuration;
3. pure data processing;
4. embedded and `no_std`;
5. browser WASM;
6. WebAssembly component;
7. bundled or system-native dependency;
8. identity, credential, TLS, and cryptographic provider; and
9. testing, assurance, packaging, and deployment.

This pulse authorizes synthetic `ferris.profile-evidence/v0` development
fixtures and conformance tests only. It does not add or alter a Ferris command,
schema, result type, exit code, identity rule, resource bound, or renderer.

## Fixture contract

Each family has one explicit `before.json` and `after.json` under
`tests/fixtures/profile-evidence/`. Every pair:

- keeps one profile ID and consumer;
- advances revision `r1` to `r2`;
- changes one family-specific evidence area;
- retains all twelve Pulse 14 sections;
- includes distinctive raw section values for negative disclosure checks; and
- remains within the existing 1 MiB input and 10,000-change bounds.

The exact family-to-section mapping is recorded in the
[fixture matrix](../../../../tests/fixtures/profile-evidence/MATRIX.md).

These fixtures are synthetic boundary cases. They are not canonical
PLATFORM-001 records, owner observations, supported stacks, compatibility
results, freshness evidence, approvals, recommendations, or production
examples.

## Conformance behavior

One data-driven CLI integration test executes all nine pairs using:

```console
ferris profile-diff --before <FAMILY_BEFORE> --after <FAMILY_AFTER> --format json
```

For every family, the test requires:

- process exit 1 and typed `difference`;
- no stderr output;
- the expected stable profile identity on both sides;
- the expected changed section;
- at least one exact family-specific JSON Pointer;
- absence of both distinctive raw section values from output; and
- continued use of the existing non-executable Pulse 14 result contract.

## Acceptance

- all nine Draft PLATFORM-001 families have independent fixture directories;
- no family fixture stands in for another;
- every pair is accepted by the strict Pulse 14 parser;
- every pair produces the expected typed difference and pointer;
- raw section values remain absent from machine output;
- the full existing workspace test suite remains green;
- formatting, linting, diff, Markdown link, and code-fence gates pass; and
- all nine roles accept the fixture-only scope.

## Stop conditions

Stop rather than widening this pulse if the work requires:

- profile generation or owner-source collection;
- Cargo, rustc, native tool, provider, network, build, deployment, or platform
  execution;
- new product behavior or schema semantics;
- interpretation of pass, fail, expected rejection, unsupported, unavailable,
  not observed, stale, or unknown;
- support, compatibility, security, freshness, readiness, or approval claims;
- a Proposed or Adopted PLATFORM-001 claim; or
- held-out fixture or oracle access.

## Support, removal, and remaining gates

Removal is deletion of the fixture directories, matrix, integration test, and
Pulse 15 records. It does not change the Ferris binary, either product crate,
Cargo behavior, owner workflows, input files outside the fixture corpus, or
any durable state.

The matrix is one development conformance step only. PLATFORM-001 still
requires exact family revisions, owner evidence, stage matrices, renewal,
substitution, removal, rollback, ordinary Cargo preservation, cross-platform
execution, held-out maintainer workflow, and a fresh advancement review.

The completed review is
[Pulse 15 Nine-Role Conformance Review](../../../../docs/plans/reviews/PULSE-15-ROLE-REVIEW.md).
