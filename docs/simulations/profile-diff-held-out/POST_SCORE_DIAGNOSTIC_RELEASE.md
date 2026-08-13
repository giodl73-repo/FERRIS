# Prospective Post-Score Diagnostic Release Protocol

Status: Frozen prospective protocol
Schema: `ferris.post-score-diagnostic-release/v1`

## Applicability

This protocol applies only when a future public held-out contract selects a
disclosure tier before hidden fixture construction. It does not apply
retroactively to Pulse 17.

## Disclosure tiers

| Tier | Post-score release |
|---|---|
| `none` | Immutable score only |
| `category-only` | Score plus predeclared public failure categories |
| `sanitized-reproducer` | Category plus a fresh independently created public reproducer |

The tier MUST NOT be escalated after any candidate output or score is
observed. Experimental tools SHOULD use `sanitized-reproducer`.

## Separate artifacts

The certification result and diagnostic release are different records.

1. The certification result retains its cutoff, attempts, package seals,
   score, and disposition unchanged.
2. The sanitized release contains a new public reproducer and a
   `ferris.post-score-diagnostic-release/v1` receipt.

The diagnostic release MUST NOT be described as a rescore, retry, oracle
release, held-out pass, or certification evidence.

## Required workflow

1. Precommit the disclosure tier in the public contract.
2. Construct and seal hidden material independently.
3. Execute and score once.
4. Seal and retire the original package.
5. In a separate workspace, independently derive a new minimal public
   reproducer for only the released category.
6. Prove category and exit reproduction on every required platform.
7. Run anti-overlap and privacy scans.
8. Publish the reproducer, commands, file digests, bounds, limitations, and
   signed receipt identity.
9. Remove temporary custody state.
10. Permanently mark both packages ineligible for future certification.

## Mandatory anti-overlap gates

All counts below MUST equal zero:

- hidden canary hits;
- original fixture or private identifier hits;
- original private digest hits;
- byte-identical hidden inputs;
- original sealed changed-path hits; and
- oracle predicate, expected identity, or expected-byte disclosures.

The original public result MUST remain byte-for-byte unchanged. The release
MUST use a different release ID and public file digests. Category and actual
exit reproduction MUST agree on every required platform.

## Bounds

A sanitized reproducer contains at most 16 regular files and 1,048,576 total
bytes. It runs at most four commands per platform. Each command has a 60,000
ms bound and retains at most 1,048,576 bytes per stream. Network, credentials,
accounts, privileged operations, and mutable external systems are prohibited.

## Retirement

The original hidden package and public reproducer MUST both record
`future_certification_eligible:false`. Any future certification requires an
independently constructed replacement package.

