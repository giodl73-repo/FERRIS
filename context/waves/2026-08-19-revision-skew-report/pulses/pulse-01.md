# Pulse 01: Implement Revision-Skew Evidence

Status: Complete
Implementation authority: Bounded to this pulse
Budget: One command, one request schema, one result schema

## Outcome

Added:

```console
ferris revision-skew --request <REQUEST_JSON> [--format human|json]
```

The command is non-executable and read-only. It observes only explicit local
evidence:

1. Cargo workspace-member dependency declarations from bounded
   `cargo metadata --format-version 1 --no-deps --offline --locked`;
2. matching package source entries from the owner workspace `Cargo.lock`; and
3. local producer checkout HEAD and commit ancestry through bounded Git
   commands with prompts and optional locks disabled.

## Classification

| Status | Meaning |
|---|---|
| `equal` | locked and observed revisions are identical |
| `behind` | locked revision is an ancestor of observed producer HEAD |
| `ahead` | observed producer HEAD is an ancestor of locked revision |
| `divergent` | both revisions exist locally but neither is an ancestor |
| `unavailable` | checkout HEAD, revision objects, or ancestry cannot be verified |
| `unknown` | no unique matching lockfile revision is available |

These are revision-topology statements only. None establishes source, API,
ABI, behavioral, data, deployment, validation, or support compatibility.

## Real diamond replay

The exact clean revisions from the shared-substrate pilot produced five
`behind` records:

| Consumer | Producer package | Declaration | Locked | Observed |
|---|---|---|---|---|
| BISECT | `fletch-core` | branch | `7c7aacd4...` | `361fb7ed...` |
| BISECT | `metis-core` | branch | `78ae3409...` | `afa59514...` |
| ICELINES | `fletch-core` | branch | `7c7aacd4...` | `361fb7ed...` |
| ROUTE | `fletch-core` | revision | `7c7aacd4...` | `361fb7ed...` |
| ROUTE | `metis-core` | revision | `78ae3409...` | `afa59514...` |

The first implementation attempt used full Cargo metadata. The replay
correctly exposed that this could require unrelated dependency sources in a
large workspace. The final implementation instead combines `--no-deps`
metadata with a bounded direct lockfile read. This is both narrower and more
reproducible offline.

## Boundaries

- Relationships and package names are request-owned assertions.
- Cargo remains authoritative for declaration and lock resolution evidence.
- Git remains authoritative only for local commit identity and ancestry.
- Ferris performs no fetch, checkout, manifest edit, lockfile update, build,
  test, validation, or compatibility decision.
- All five child repositories remained clean and unmodified.
