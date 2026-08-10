# EXP-01: Stewardship Census

Date: 2026-08-09 Pacific / 2026-08-10 UTC
Question: ECOS-Q05
Method: crates.io authority and release history, GitHub repository and bounded
work observations, package-path checks, and RustSec lifecycle advisory census
Result: maintenance required multiple evidence dimensions; release age,
activity, owner count, and repository organization were individually
insufficient.

## Queue

The census reused the nineteen exact releases selected by ECOS-Q02. Registry
checksums, licenses, declared Rust versions, release dates, source
repositories, and the unresolved repository-revision state are in
[ECOS-Q02 EXP-01](../../ecos-q02-foundational-crate-census/results/EXP-01-foundational-crate-census.md).
This pass renewed mutable stewardship fields without substituting a repository
commit for the registry checksum identity.

## Registry commands

For each package:

```text
GET https://crates.io/api/v1/crates/<crate>
GET https://crates.io/api/v1/crates/<crate>/owners
GET https://crates.io/api/v1/crates/<crate>/<exact-version>
```

The crate response supplied current trustpub-only policy and version history.
The owner response supplied current user and team owners. The exact-version
response supplied checksum, human publisher or trusted-publishing provenance,
publish time, and audit actions.

Cargo authority documentation:

- <https://doc.rust-lang.org/cargo/commands/cargo-owner.html>
- <https://doc.rust-lang.org/cargo/reference/publishing.html#yanking>
- <https://crates.io/docs/trusted-publishing>
- <https://forge.rust-lang.org/policies/crate-ownership.html>

## Registry results

`U / T` means current individual-user owners / GitHub-team owners. Recent
authority identities are distinct human publishers or trusted
provider/repository identities across the latest ten non-yanked versions.

| Exact release | U / T | Exact publication authority | Recent authorities | Latest non-yanked | Yanked / versions | Trustpub-only now |
|---|---:|---|---:|---:|---:|---|
| `libc 0.2.189` | 5 / 1 | user `rust-lang-owner` | 1 | 2026-07-21 | 9 / 206 | Yes |
| `cfg-if 1.0.4` | 1 / 1 | user `rust-lang-owner` | 2 | 2025-10-15 | 1 / 16 | No |
| `getrandom 0.4.3` | 1 / 1 | GitHub `rust-random/getrandom` | 3 | 2026-06-17 | 2 / 48 | Yes |
| `proc-macro2 1.0.107` | 1 / 0 | user `dtolnay` | 1 | 2026-07-19 | 2 / 163 | No |
| `quote 1.0.47` | 1 / 0 | user `dtolnay` | 1 | 2026-07-19 | 0 / 95 | No |
| `syn 3.0.3` | 1 / 0 | user `dtolnay` | 1 | 2026-07-22 | 17 / 356 | No |
| `serde_core 1.0.229` | 1 / 1 | user `dtolnay` | 1 | 2026-07-18 | 0 / 10 | No |
| `serde 1.0.229` | 1 / 1 | user `dtolnay` | 1 | 2026-07-18 | 3 / 316 | No |
| `log 0.4.33` | 4 / 2 | user `KodrAus` | 1 | 2026-06-20 | 6 / 63 | No |
| `tracing-core 0.1.36` | 1 / 1 | user `hds` | 2 | 2025-12-18 | 0 / 38 | No |
| `bytes 1.12.1` | 2 / 1 | user `Darksonn` | 1 | 2026-07-08 | 4 / 59 | No |
| `http 1.5.0` | 2 / 0 | user `seanmonstar` | 1 | 2026-07-29 | 0 / 45 | No |
| `tower-service 0.3.3` | 2 / 1 | user `LucioFranco` | 3 | 2024-08-13 | 0 / 9 | No |
| `futures-core 0.3.33` | 3 / 0 | user `taiki-e` | 1 | 2026-07-18 | 2 / 38 | No |
| `rand_core 0.10.1` | 1 / 1 | GitHub `rust-random/rand_core` | 3 | 2026-04-13 | 4 / 38 | Yes |
| `cc 1.4.2` | 1 / 1 | GitHub `rust-lang/cc-rs` | 1 | 2026-08-08 | 2 / 218 | Yes |
| `pkg-config 0.3.33` | 3 / 0 | user `sdroege` | 1 | 2026-04-12 | 0 / 46 | No |
| `hashbrown 0.17.1` | 2 / 0 | user `rust-lang-owner` | 2 | 2026-05-09 | 4 / 56 | No |
| `memchr 2.8.3` | 1 / 0 | user `BurntSushi` | 1 | 2026-07-08 | 1 / 47 | No |

Current exact owner logins:

| Package family | Current crates.io owners |
|---|---|
| `libc` | `huonw`, `joshtriplett`, `gnzlbg`, `JohnTitor`, `rust-lang-owner`, `github:rust-lang:libs` |
| `cfg-if` / `cc` | `rust-lang-owner`, `github:rust-lang:libs` |
| `getrandom` / `rand_core` | `dhardy`, `github:rust-random:maintainers` |
| `proc-macro2` / `quote` / `syn` | `dtolnay` |
| `serde` / `serde_core` | `dtolnay`, `github:serde-rs:publish` |
| `log` | `huonw`, `sfackler`, `KodrAus`, `rust-lang-owner`, `github:rust-lang-nursery:log-owners`, `github:rust-lang-nursery:libs` |
| `tracing-core` | `hawkw`, `github:tokio-rs:publish-tracing` |
| `bytes` | `carllerche`, `Darksonn`, `github:tokio-rs:core` |
| `http` | `carllerche`, `seanmonstar` |
| `tower-service` | `carllerche`, `seanmonstar`, `github:tower-rs:publish` |
| `futures-core` | `cramertj`, `taiki-e`, `rust-lang-owner` |
| `pkg-config` | `sdroege`, `joshtriplett`, `rust-lang-owner` |
| `hashbrown` | `Amanieu`, `rust-lang-owner` |
| `memchr` | `BurntSushi` |

## Trusted publication provenance

| Exact release | Provider | Repository | Workflow run | Source commit |
|---|---|---|---|---|
| `getrandom 0.4.3` | GitHub | `rust-random/getrandom` | `27709261998` | `5e7cd5733536844a9856dc7259bd4696bbe5e3ae` |
| `rand_core 0.10.1` | GitHub | `rust-random/rand_core` | `24350942053` | `5cd7df496d0bff1d95534ac8b66a2610f5bf7808` |
| `cc 1.4.2` | GitHub | `rust-lang/cc-rs` | `31233498143` | `a91e05ec40f26d4637d4bff9e9764221d0a59dd8` |

`libc` was trustpub-only at observation time, but its selected release named
`rust-lang-owner` as the publisher and had no trusted-publishing record.

Recent authority histories also exposed transitions:

| Crate | Recent historical authority not in current owner set |
|---|---|
| `cfg-if` | `alexcrichton` |
| `getrandom` | `josephlr`, `newpavlov` |
| `tracing-core` | `hds` |
| `tower-service` | `davidpdrsn`, `hawkw`, `LucioFranco` |

This establishes a mismatch, not the date or reason for an owner change.

## GitHub command

The eighteen distinct canonical repositories were queried through GitHub's
GraphQL API. The bounded query observed:

```graphql
repository(owner: $owner, name: $name) {
  nameWithOwner
  owner { login __typename }
  isArchived
  isDisabled
  isFork
  pushedAt
  securityPolicyUrl
  fundingLinks { platform url }
  defaultBranchRef {
    target {
      ... on Commit {
        history(first: 100, since: "2025-08-09T00:00:00Z") {
          totalCount
          nodes { author { user { login } name } }
        }
      }
    }
  }
  pullRequests(
    states: MERGED
    first: 20
    orderBy: { field: UPDATED_AT, direction: DESC }
  ) {
    nodes {
      mergedAt
      author { login }
      reviews(first: 50) {
        nodes { state author { login } }
      }
    }
  }
}
```

CODEOWNERS was checked at `CODEOWNERS`, `.github/CODEOWNERS`, and
`docs/CODEOWNERS` on the default branch.

GitHub semantics:

- archive:
  <https://docs.github.com/en/repositories/archiving-a-github-repository/archiving-repositories>
- transfer:
  <https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository>
- fork:
  <https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/about-forks>
- CODEOWNERS:
  <https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners>
- security policy:
  <https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/add-security-policy>

## Repository results

All eighteen repositories were enabled, unarchived, and not forks. Fourteen
were organization-owned; proc-macro2, quote, syn, and memchr were held in
user-owned repositories.

`Human authors` counts distinct non-bot author identities in the returned
one-year history sample. `Top share` is the top human author's share of that
sample, not a bus factor. Pull-request columns cover the twenty most recently
updated merged pull requests.

| Repository | Commits in year | Human authors in sample | Top human share | PR authors | Approvers | Peer-approved PRs | Security | Funding |
|---|---:|---:|---:|---:|---:|---:|---|---|
| `rust-lang/libc` | 801 | 26 | 39% | 10 | 3 | 13 | Yes | No |
| `rust-lang/cfg-if` | 14 | 3 | 50% | 5 | 3 | 6 | Yes | No |
| `rust-random/getrandom` | 89 | 10 | 52% | 3 | 3 | 19 | Yes | No |
| `dtolnay/proc-macro2` | 88 | 2 | 99% | 3 | 1 | 2 | Yes | Yes |
| `dtolnay/quote` | 95 | 4 | 88% | 5 | 1 | 11 | Yes | Yes |
| `dtolnay/syn` | 422 | 1 | 100% | 1 | 0 | 0 | Yes | Yes |
| `serde-rs/serde` | 169 | 7 | 59% | 6 | 1 | 11 | No | Yes |
| `rust-lang/log` | 71 | 12 | 55% | 10 | 3 | 19 | Yes | No |
| `tokio-rs/tracing` | 65 | 33 | 29% | 18 | 8 | 20 | Yes | No |
| `tokio-rs/bytes` | 23 | 13 | 35% | 13 | 6 | 19 | Yes | No |
| `hyperium/http` | 39 | 17 | 49% | 11 | 1 | 11 | Yes | Yes |
| `tower-rs/tower` | 16 | 6 | 56% | 11 | 7 | 20 | No | No |
| `rust-lang/futures-rs` | 50 | 12 | 66% | 10 | 2 | 10 | Yes | No |
| `rust-random/rand_core` | 99 | 7 | 69% | 5 | 4 | 19 | No | Yes |
| `rust-lang/cc-rs` | 205 | 24 | 26% | 8 | 2 | 15 | Yes | No |
| `rust-lang/pkg-config-rs` | 6 | 5 | 33% | 17 | 1 | 13 | Yes | No |
| `rust-lang/hashbrown` | 179 | 11 | 30% | 5 | 4 | 4 | Yes | No |
| `BurntSushi/memchr` | 29 | 10 | 61% | 13 | 1 | 11 | No | Yes |

The query found a recognized CODEOWNERS file only in `tokio-rs/tracing`.
Visible branch-protection-rule results were not treated as complete because
repository rulesets, permissions, and private policy can affect visibility.

## Package-path checks

Repository-wide history can overcount maintenance for monorepo packages.

```text
GET /repos/<owner>/<repo>/commits?path=<package-path>&per_page=1
```

| Package path | Latest observed path commit | Commit |
|---|---:|---|
| `serde-rs/serde/serde_core` | 2026-07-18 | `7fc3b4c30c94` |
| `tokio-rs/tracing/tracing-core` | 2025-12-28 | `efc690fa6bd1` |
| `tower-rs/tower/tower-service` | 2026-01-12 | `719ec035a6b9` |
| `rust-lang/futures-rs/futures-core` | 2026-07-18 | `c24a06b3e574` |

The `tower-service` path changed after the selected package's 2024 release,
demonstrating that release age and path maintenance can diverge.

## RustSec lifecycle check

The advisory database was observed at:

```text
git clone --depth 1 https://github.com/RustSec/advisory-db.git
git rev-parse HEAD
565436d86a136c840d01ad4a7851fc7391295404
```

No selected package matched:

```toml
informational = "unmaintained"
```

RustSec's policy requires an explicit author declaration or prolonged stale
activity plus failed contact:
<https://github.com/RustSec/advisory-db/blob/main/HOWTO_UNMAINTAINED.md>.

Absence of an advisory is not evidence that a package is maintained.

## Limitations

- Owner data was current rather than a historical owner-event log.
- Team membership and private access controls were not visible.
- Recent authority identities cover only ten non-yanked versions.
- Repository commit concentration used at most one hundred returned nodes.
- Author identity can be distorted by bots, squash merges, co-authorship,
  rebases, and metadata.
- Pull-request approval records do not cover all review channels.
- Repository activity was path-scoped only for four monorepo packages.
- Issue and security-response latency were not measured.
- Funding exposure did not identify amount, duration, allocation, or service.
- No maintainer contact was attempted.
- No lifecycle or successor verdict was inferred from the measurements.
