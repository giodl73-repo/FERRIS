# Held-Out Public-Safe Score Receipt 001

Cutoff: `0cc01df0835f7651a66dd884321325e8a316775c`
Tag: `ferris-read-only-pulse-01-cutoff`
Disposition: All applicable Pulse 01 fixtures passed

This receipt exposes no hidden edit, failure seed, expected output, oracle
predicate, or scoring note.

## Aggregate

- applicable: 2;
- pass: 2;
- fail: 0;
- applicable blocked or invalid: 0; and
- out of scope and not executed: 10.

## Results

| Fixture | Applicability | Disposition | Requirement | Observed class | Observed digest |
|---|---|---|---|---|---|
| FHIF-001 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `2529ecded4e7a9ee9cb4e108ea306339ef186741282e155d955bb904489a4908` |
| FHIF-002 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `360dcaec7f50c32e3c980534c25bdae7ea5182a762b626c954ae51899873e6f0` |
| FHIF-003 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `0484a3dc0cc22368cdd3afb72ec78d1ea7e3b4a5fcc9ef46fa17a73bc035d951` |
| FHIF-004 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `a20e9fa9ec6d8fd406014cffef0b281772bd4b496e3928120591e025bd95fd88` |
| FHIF-005 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `da2d12fbc950c1c1753cd00ee689b9962a8f70109e56d99c73e44278acf18da2` |
| FHIF-006 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `aa070d8670d74c355e16d80ca446c80bd070deb46596c35753d5ab2a3b9b27f9` |
| FHIF-007 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `07246996afdd25230fa8a9424983ba886de7984867d88844c68dfb39032800fb` |
| FHIF-008 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `935a793583dcf765b08fc5b9a6da001460f07444aa020c21fcf4a4d75476f34e` |
| FHIF-009 | applicable | pass | `P01-LOCAL-PLAN` | success | `d18c360877a178e85c2920210d71620e760babd8975279a512cf8dc5b3b5430f` |
| FHIF-010 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `83aa136044977b72a8a54f6dda6b3230f6475208a9f5bae7293c8f4b97251b6b` |
| FHIF-011 | out-of-scope | blocked | `P01-OUT-OF-SCOPE` | not-executed | `c8b33718e6e7c4920d962c1eca1bb91f252df3808f8676810d216d8f13331230` |
| FHIF-012 | applicable | pass | `P01-LOCAL-PARITY` | success | `b3f02c586ae7c182dee635bd830c1c143642bd350326ddc172ad1eecfd711b53` |

Out-of-scope fixtures require a later authorized capability cutoff. They do
not count as passes or failures and did not cause implementation changes.
