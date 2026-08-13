# Pulse 17 Stage B/C Public-Safe Result

Contract revision: 3
Fixture: `P17-R3-D6B553CBC3B1240B673B8190`
Cutoff: `8cbb5356fd7b3acca435bc9fad4e97dabab66bb5`
Disposition: Fail
Score validity: Valid implementation failure; not invalid custody
Failure category: `process-exit-agreement`

The immutable first score completed on first-score attempt 1 and scorer
attempt 1. The one-score program is closed. The fixture and its custody
artifacts are sealed in quarantine and MUST NOT be retried, rescored, or
reused.

## Collection aggregate

Exactly 112 of 112 declared processes were collected:

| Platform | Processes |
|---|---:|
| Windows | 56 |
| Ubuntu 24.04 | 56 |

| Integrity category | Count |
|---|---:|
| Missing | 0 |
| Duplicate | 0 |
| Retried | 0 |
| Extra | 0 |
| Launch failures | 0 |
| Abnormal terminations | 0 |
| Stream failures | 0 |
| Privacy hits | 0 |

Retries: 0
Rescores: 0
Collection aggregate digest:
`sha256:8d1b157ef79bf22741d53e6b4ff68302f88cc04ac317e8b3364b9a83832ef9ba`
Collection seal:
`sha256:71916d7d98e0bcbebbe46ceb25dc619c00fcd69f5224ef5ce01c98ab2534e3b1`

## Aggregate result classes

| Result class | Count |
|---|---:|
| `success` | 8 |
| `difference` | 58 |
| `invalid` | 22 |
| `unsupported` | 2 |
| `incomplete` | 20 |
| `blocked` | 2 |
| **Total** | **112** |

These are aggregate class counts only. They do not identify any hidden case.

## Repository workflow aggregate

The repository workflow aggregate passed. The `hosted`,
`cross_target_no_std`, and `native_bound` slots each passed. All 28 owner
commands completed with zero owner failures, zero comparison failures, and
zero lifecycle failures. Repository cleanup completed.

## Immutable artifact digests

- Source:
  `sha256:5bec6598a5274fd27e8c8c4c275a9cd85ef01bb250df810898a2e13962757910`
- Windows binary:
  `sha256:aef4bd137d49400649186dfd88d2ae37ea100c55bdb31ce1ed136b17f9c9eec1`
- Ubuntu 24.04 binary:
  `sha256:1f3c9acb44002e77fa11f79f4205d9dfb167889f481a507bf0e37aa284de90ad`
- Report seal:
  `sha256:33c1fa87344ee2ef6b186fb59457eff10ca7137e2c2d7019174a11bb96fdf4d0`

The machine-readable companion is
[`PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.json`](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.json).

## Public meaning and disclosure boundary

`process-exit-agreement` retains only the meaning already frozen by the public
contract: the actual process exit must agree with the emitted or corresponding
typed result exit. This result does not identify which hidden cases or which
expected and actual exits differed.

No per-case outcome, hidden input, changed path, privacy canary, expected
output, expected digest, or scorer predicate is disclosed. No hidden material
was disclosed.
