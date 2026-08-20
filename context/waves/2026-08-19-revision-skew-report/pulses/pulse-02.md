# Pulse 02: Harden Revision-Skew Trust Boundaries

Status: Complete
Implementation authority: Dirty-checkout trust hardening and adversarial tests
Budget: One additional local Git observation; no schema change

## Outcome

`revision-skew` now requires each explicit producer checkout to be clean
before treating its HEAD as the observed source identity. A dirty checkout is
reported as `unavailable` with an explicit reason because HEAD alone does not
identify modified or untracked source state. Producer HEAD and cleanliness
are verified before lock-evidence classification, so missing or duplicate
lock evidence cannot bypass producer identity checks.

The command remains read-only. The additional observation is bounded:

```console
git -C <CHECKOUT> status --porcelain=v1 --untracked-files=normal
```

It inherits the existing timeout, output bound, disabled prompts, and disabled
optional-lock policy.

## Adversarial matrix

| Scenario | Result |
|---|---|
| Clean locked revision behind producer HEAD | `behind` |
| Claimed observed revision differs from HEAD | `unavailable` |
| Dirty producer checkout at the claimed HEAD | `unavailable` |
| Dirty producer checkout with missing lock evidence | `unavailable` |
| Locked and observed commits on divergent branches | `divergent` |
| Two matching lockfile revisions | `unknown` |
| Missing `Cargo.lock` | `unknown` |
| Producer path containing parent traversal | typed `invalid` |
| Unsupported request schema | typed `unsupported` |

The missing-lock fixture established that locked/no-deps Cargo metadata can
still succeed for this package shape. Ferris therefore retains the direct
bounded lockfile read as the evidence boundary and reports absent unique lock
evidence as `unknown`, rather than inventing a resolution or converting it
into a compatibility conclusion.

## Boundaries

- No request or report schema changed.
- No relationship was discovered.
- No repository was fetched, checked out, updated, or mutated.
- No manifest or lockfile was rewritten.
- No build, test, validation, or owner command was executed by the product.
- Revision topology still establishes no source, API, ABI, behavioral, data,
  deployment, validation, or support compatibility.
