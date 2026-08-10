# EXP-01: Discovery and Selection Matrix

Date: 2026-08-10
Question: ECOS-Q10
Environment: Windows 11 Enterprise, x86-64, Cargo/rustc 1.95.0
Decision input: determine whether evidence-backed candidate filtering improves
crate selection beyond keyword search, popularity, and unpinned curation.

## Scope

This experiment measures:

1. official search candidate variation across query and sort policy;
2. Cargo CLI agreement with crates.io relevance;
3. relevance reachability and stale/exact-name controls;
4. visibility of established or curated candidates;
5. current curation and composite-ranking methodology;
6. exact identity, closure, compiler, behavior, and assurance evidence for one
   bounded CLI selection profile; and
7. whether mandatory consumer requirements alter eligibility.

It does not produce a universal crate ranking or adoption recommendation.

## Source identity

| Source | Revision or observation |
|---|---|
| crates.io source | `1bb85949b723e3c0f27c730e99c8e31c1b33a5ca`, 2026-08-10 |
| Cargo source | `9184583d2dc29ac1e23c6304f7281fd3941bb1bb`, 2026-08-10 |
| Blessed.rs data | `c750a3d44011465b4d4c7a811e7752c4a63f0415`, 2026-08-03 |
| Lib.rs methodology | live About and Data Processing pages observed 2026-08-10 |
| crates.io API results | observed 2026-08-10 |

The crates.io API describes itself as experimental outside the stable registry
web API surfaces. Search observations therefore retain date and implementation
revision.

## Toolchain

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
host x86_64-pc-windows-msvc
```

Additional compiler controls:

```text
rustc/cargo 1.71.0
rustc/cargo 1.85.0
```

## Official search matrix

### Queries

| ID | Query | Intended capability |
|---|---|---|
| `json` | `json` | JSON serialization and parsing |
| `http-client` | `http client` | HTTP client |
| `async-runtime` | `async runtime` | executor and I/O runtime |
| `cli-parser` | `command line parser` | command-line argument parser |
| `nostd-logging` | `no_std logging` | logging on a `no_std` target |
| `tls` | `tls` | TLS implementation or platform adapter |
| `sqlite` | `sqlite` | SQLite access |
| `datetime` | `date time` | calendar date and time |

### API command shape

```text
GET https://crates.io/api/v1/crates
    ?q=<query>
    &sort=<relevance|downloads|recent-downloads|recent-updates|alpha>
    &per_page=10
    &page=1
```

Forty requests produced 400 ordered observations.

### Cargo control

```text
cargo search "<query>" --limit 10 --color never
```

Cargo's top ten matched API relevance for each query:

```text
8 of 8 query sets
Jaccard similarity: 1.000 for every query
```

Cargo search showed crate name, version, and description. The observed API
search row exposed 23 fields, including downloads, timestamps, keywords,
categories, links, repository, exact-match state, and TrustPub-only state.
Neither surface provided the complete consumer-specific evidence required by
ECOS-Q03 through ECOS-Q09.

## Search results

| Query | Reported matches | Relevance first | Downloads first | Recent-downloads first | Recent-updates first |
|---|---:|---|---|---|---|
| JSON | 38,981 | `json` | `serde` | `serde_json` | `cookcli` |
| HTTP client | 11,434 | `rabbitmq_http_client` | `hyper` | `hyper` | `eventsource-client` |
| async runtime | 11,307 | `pyo3-async-runtimes-macros` | `mio` | `mio` | `oxicode-agent` |
| CLI parser | 3,652 | `app` | `clap` | `clap` | `tinysandbox` |
| `no_std` logging | 7,522 | `defmt-or-log` | `hashbrown` | `hashbrown` | `syslog-rs` |
| TLS | 7,191 | `tls` | `rustls` | `rustls` | `syslog-rs` |
| SQLite | 6,926 | `sqlite` | `libsqlite3-sys` | `libsqlite3-sys` | `oxicode-mnemopi` |
| date/time | 5,834 | `chronounit` | `regex` | `regex` | `cookcli` |

The table reports retrieval, not capability fit.

### Top-ten overlap

| Comparison with relevance | Mean Jaccard | Minimum | Maximum |
|---|---:|---:|---:|
| all-time downloads | 0.030 | 0.000 | 0.111 |
| recent downloads | 0.030 | 0.000 | 0.111 |
| recent updates | 0.000 | 0.000 | 0.000 |
| alphabetical/name | 0.020 | 0.000 | 0.053 |

No recent-update top ten shared a crate with the relevance top ten for the same
query.

### Relevance age

Twenty of the 80 relevance results had not been updated since 2024-08-10.

| Query | Older than two years in top ten | Oldest observed result | Last update |
|---|---:|---|---|
| JSON | 5 | `json` | 2020-03-18 |
| HTTP client | 1 | `http-client` | 2022-06-20 |
| async runtime | 0 | `spawns-core` | 2025-09-27 |
| CLI parser | 7 | `app` | 2018-03-02 |
| `no_std` logging | 2 | `defmt-or-log-macros` | 2024-01-16 |
| TLS | 1 | `tls` | 2015-12-11 |
| SQLite | 1 | `sqlite-loadable` | 2023-10-06 |
| date/time | 3 | `date_time_parser` | 2022-08-29 |

Age was not treated as abandonment or rejection evidence.

### Exact-name behavior

Three query strings exactly matched a package name and ranked that package
first:

| Query | Package | Release | Last package update |
|---|---|---:|---|
| `json` | `json` | 0.12.4 | 2020-03-18 |
| `tls` | `tls` | 0.0.3 | 2015-12-11 |
| `sqlite` | `sqlite` | 0.37.0 | 2025-03-28 |

Current crates.io source explicitly orders exact-name matches ahead of
full-text rank.

### Relevance reachability

Control:

```text
GET /api/v1/crates?q=json&sort=relevance&per_page=10&page=101
```

Result:

```text
HTTP 400
Cannot page beyond the first 1000 results when sorting by relevance.
```

The equivalent all-time-download request returned HTTP 200. Source defines
`RELEVANCE_CANDIDATE_LIMIT` as 1,000. The crates.io development update states
that those candidates are selected from matching crates by highest recent
downloads before relevance ranking.

## Known-candidate visibility

The following table checks selected established or curated candidates against
all five top-ten lists.

| Query | Candidate | Observed placement |
|---|---|---|
| JSON | `serde_json` | relevance 4, downloads 3, recent downloads 1 |
| HTTP client | `reqwest` | downloads 4, recent downloads 3 |
| HTTP client | `ureq` | downloads 10, recent downloads 10 |
| HTTP client | `hyper` | downloads 1, recent downloads 1 |
| async runtime | `tokio` | downloads 2, recent downloads 2 |
| async runtime | `async-std` | absent |
| async runtime | `smol` | absent |
| CLI parser | `clap` | relevance 10, downloads 1, recent downloads 1 |
| CLI parser | `lexopt` | absent |
| CLI parser | `pico-args` | absent |
| `no_std` logging | `log` | downloads 2, recent downloads 2 |
| `no_std` logging | `defmt` | absent |
| TLS | `rustls` | downloads 1, recent downloads 1 |
| TLS | `native-tls` | absent |
| TLS | `openssl` | absent |
| SQLite | `rusqlite` | relevance 7, recent downloads 4 |
| date/time | `time` | downloads 3, recent downloads 7 |
| date/time | `chrono` | downloads 4, recent downloads 8 |
| date/time | `jiff` | absent |

Absence means absence from these forty top-ten cells, not absence from the
registry.

## Discovery-source policy

### Cargo and crates.io

Cargo documents textual search and displays a copyable dependency line.
Current crates.io relevance:

1. prefers exact-name matches;
2. bounds candidates to 1,000;
3. applies full-text rank; and
4. uses recent downloads to order rank ties.

Search is therefore an attributed retrieval policy.

### Blessed.rs

The current guide described itself as hand curated. Its CLI recommendations
were:

| Purpose | Candidate | Curator rationale |
|---|---|---|
| fully featured | `clap` | ergonomic, battle tested, kitchen sink, fast at runtime; compile time can be slow |
| minimal | `lexopt` | fast compile/runtime, pedantic correctness, less ergonomic |
| minimal | `pico-args` | fast compile/runtime, more lax correctness, more ergonomic |

The observed JSON data contained recommendation names and notes but no
`version` key. The file had at least 100 commits between 2022-11-11 and the
current 2026-08-03 revision. Pinning a Git commit can preserve the curation
snapshot; it does not verify the current exact release.

### Lib.rs

Lib.rs explicitly states that it is unofficial and that presence is not an
endorsement. Its documented ranking and enrichment include:

- filtered popularity and reverse-dependency signals;
- usage trend, documentation, examples, tests, CI, comments, and metadata;
- release stability and dependency weight;
- inferred maintenance and maintainer reputation;
- special cases and blocklists;
- inferred keywords, categories, repository, MSRV, and `no_std` support; and
- RustSec, cargo-vet, and cargo-crev integration.

These fields can improve candidate recall and triage. Because they combine
source-owned weights, inferences, and manual policy, the resulting rank remains
attributed recommendation evidence.

## Exact CLI candidate control

### Consumer operation

Each fixture had to parse:

```text
--name ferrium --verbose
```

and print:

```text
name=ferrium verbose=true
```

### Candidate source

Blessed.rs divided the candidates into fully featured and minimal groups.
Official search did not return Lexopt or pico-args in any observed top-ten
cell for `command line parser`.

### Exact release identity

| Crate | Release | Published | License | Declared Rust | Owners | VCS revision |
|---|---:|---|---|---:|---:|---|
| `clap` | 4.6.6 | 2026-08-06 | MIT OR Apache-2.0 | 1.85 | 3 | `4a622b4340d5e1fffff60c0ecefdc6882f738159` |
| `lexopt` | 0.3.2 | 2026-02-28 | MIT | not declared | 1 | `f52c6a620b59dcadb01701c039cd4b270e2d5966` |
| `pico-args` | 0.5.0 | 2022-06-04 | MIT | not declared | 1 | `56e8872fa31f168fd87e84fdc5f24bd60875daea` |

Current owner identities:

```text
clap: kbknapp, github:rust-cli:maintainers, github:clap-rs:admins
lexopt: blyxxyz
pico-args: RazrFalcon
```

Current owner count is not a bus-factor or maintenance conclusion.

### Archive identity

| Crate | crates.io checksum | Downloaded archive result |
|---|---|---|
| `clap 4.6.6` | `473c7e07f409a8d772161724aa8db6a765a2532a70f9667eeb7b49d3d02fbdca` | match |
| `lexopt 0.3.2` | `803ec87c9cfb29b9d2633f20cba1f488db3fd53f2158b1024cbefb47ba05d413` | match |
| `pico-args 0.5.0` | `5be167a7af36ee22fe3115051bc51f6e6c7054c9348e28deb4f49bd6f705a315` | match |

### Fixture manifests

```toml
clap = { version = "=4.6.6", features = ["derive"] }
lexopt = "=0.3.2"
pico-args = "=0.5.0"
```

### Build and execution

Command shape:

```text
cargo generate-lockfile --manifest-path <fixture>/Cargo.toml
cargo metadata --format-version 1 --locked \
  --manifest-path <fixture>/Cargo.toml
cargo run --release --locked \
  --target-dir <isolated-target> \
  --manifest-path <fixture>/Cargo.toml \
  -- --name ferrium --verbose
```

| Candidate | Exit | Output | Active packages including fixture | Build scripts | Procedural macros | Target bytes | Executable bytes |
|---|---:|---|---:|---|---|---:|---:|
| Clap derive | 0 | exact | 22 | `proc-macro2`, `quote` | `clap_derive` | 47,683,181 | 635,904 |
| Lexopt | 0 | exact | 2 | none | none | 3,210,675 | 156,160 |
| pico-args | 0 | exact | 2 | none | none | 3,067,589 | 142,848 |

The target directories include dependency and fixture artifacts from one
release build. The first builds ran concurrently and experienced shared Cargo
cache locking; elapsed times are not compared.

### Invalid-option behavior

Direct executable control:

```text
<fixture> --name ferrium --unknown
```

| Candidate | Exit | Observed behavior |
|---|---:|---|
| Clap derive | 2 | generated unexpected-argument error, usage, and help hint |
| Lexopt | 1 | fixture propagated `invalid option '--unknown'` |
| pico-args | 1 | fixture reported leftover `--unknown` |

The libraries expose different policy boundaries: Clap owns more generated
diagnostic behavior; the minimal libraries leave more behavior to the
application.

### Compiler eligibility

| Candidate | Toolchain | Result |
|---|---:|---|
| `clap 4.6.6` | 1.71.0 | failed; package manifest uses edition 2024 |
| `clap 4.6.6` | 1.85.0 | passed |
| `lexopt 0.3.2` | 1.71.0 | passed |
| `pico-args 0.5.0` | 1.71.0 | passed |

Clap declares Rust 1.85. Lexopt and pico-args do not declare `rust-version`;
their passing checks are observations for this fixture, not inferred MSRV
policies.

### Advisory control

Command:

```text
cargo audit --version
cargo audit --json --file <fixture>/Cargo.lock
```

The observed tool was cargo-audit 0.22.2. All three exact lockfiles returned:

```text
exit 0
vulnerability count 0
```

The result is dated Rust-package matching against the observed RustSec
database. It does not establish crate quality, soundness, or absence of
undisclosed defects.

## Consumer-scoped decisions

### Profile A: Rust 1.71, bounded manual CLI

Mandatory requirements:

- compile under Rust 1.71;
- parse `--name` and `--verbose`;
- reject unknown input;
- no procedural macro or dependency build script in the active closure.

Result:

| Candidate | Eligibility |
|---|---|
| Clap derive | fail: compiler/edition requirement and compile-time closure |
| Lexopt | eligible |
| pico-args | eligible |

The evidence does not decide between Lexopt and pico-args. Their parsing
semantics, error policy, API preferences, stewardship, and broader requirements
remain consumer decisions.

### Profile B: generated derive and usage policy

Mandatory requirements:

- derive-based declaration;
- generated usage and help hint for invalid input;
- Rust 1.85 or later accepted.

Result:

| Candidate | Eligibility |
|---|---|
| Clap derive | eligible in the measured fixture |
| Lexopt | fail unless application-owned generation is added |
| pico-args | fail unless application-owned generation is added |

This is not an assertion that Clap is universally better. The profile selected
the policy boundary that Clap supplies.

## Selection record implied by the controls

```text
consumer profile
  -> retrieval query and source policy
  -> candidate role and exact release
  -> evidence coverage and freshness
  -> mandatory requirement matrix
  -> eligible tradeoff frontier
  -> owner decision with rationale and rollback
  -> renewal diff
```

A missing field remains unknown. A candidate failing one profile may remain
eligible for another.

## Limitations

- Search was measured once with eight English queries.
- Only first-page top-ten results were compared.
- Candidate relevance was not manually labeled across all 400 rows.
- The known-candidate list was illustrative, not exhaustive.
- The CLI fixture did not test completions, subcommands, Unicode, shell
  conventions, localization, fuzzing, or production argument complexity.
- No source-level security or soundness audit of the three CLI crates occurred.
- Target and executable sizes are one Windows release-profile observation.
- No comparable build-time claim is made.
- Rust 1.71 checks do not determine true minimum compiler support.
- Curation and ranking services may change after the observation time.
- No adoption decision was made.

## Source links

- <https://doc.rust-lang.org/cargo/commands/cargo-search.html>
- <https://doc.rust-lang.org/cargo/commands/cargo-info.html>
- <https://github.com/rust-lang/crates.io/blob/main/src/controllers/krate/search.rs>
- <https://blog.rust-lang.org/2026/07/13/crates-io-development-update/>
- <https://github.com/rust-lang/rfcs/blob/master/text/1824-crates.io-default-ranking.md>
- <https://blessed.rs/crates>
- <https://github.com/nicoburns/blessed-rs/blob/main/data/crates.json>
- <https://lib.rs/about>
- <https://lib.rs/data-processing>
- <https://mozilla.github.io/cargo-vet/>
- <https://embarkstudios.github.io/cargo-deny/>
- <https://rustsec.org/>
