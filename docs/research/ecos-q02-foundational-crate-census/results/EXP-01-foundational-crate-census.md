# EXP-01: Foundational Crate Census

Date: 2026-08-09
Question: ECOS-Q02
Method: crates.io registry census, exact-release Cargo metadata, and published
source triage
Result: nineteen releases selected for deeper verification; no adoption
authority granted.

## Commands and environment

Each candidate was pinned alone with default features:

```toml
[package]
name = "ecos_q02_probe"
version = "0.0.0"
edition = "2024"

[dependencies]
candidate = { version = "=<exact-version>" }
```

Resolution command:

```text
cargo metadata --format-version 1 --filter-platform x86_64-pc-windows-msvc
```

Toolchain:

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
```

Registry endpoints:

```text
https://crates.io/api/v1/crates/<crate>
https://crates.io/api/v1/crates/<crate>/owners
https://crates.io/api/v1/crates/<crate>/reverse_dependencies?page=1&per_page=1
https://crates.io/api/v1/crates/<crate>/<version>/dependencies
```

Source triage searched exact published package `src/**/*.rs` files for the
lexical token `unsafe`. The count is only a prioritization indicator: it mixes
code, documentation, tests embedded under `src`, identifiers, and comments.

```powershell
Get-ChildItem <published-crate>\src -Recurse -Filter *.rs |
  Select-String -Pattern '\bunsafe\b' -AllMatches
```

## Primary package documentation

- Contract foundations:
  <https://docs.rs/serde_core/1.0.229/serde_core/>,
  <https://docs.rs/serde/1.0.229/serde/>,
  <https://docs.rs/log/0.4.33/log/>,
  <https://docs.rs/tracing-core/0.1.36/tracing_core/>,
  <https://docs.rs/bytes/1.12.1/bytes/>,
  <https://docs.rs/http/1.5.0/http/>,
  <https://docs.rs/tower-service/0.3.3/tower_service/>,
  <https://docs.rs/futures-core/0.3.33/futures_core/>, and
  <https://docs.rs/rand_core/0.10.1/rand_core/>.
- Construction foundations:
  <https://docs.rs/proc-macro2/1.0.107/proc_macro2/>,
  <https://docs.rs/quote/1.0.47/quote/>, and
  <https://docs.rs/syn/3.0.3/syn/>.
- Platform and build foundations:
  <https://docs.rs/libc/0.2.189/libc/>,
  <https://docs.rs/cfg-if/1.0.4/cfg_if/>,
  <https://docs.rs/getrandom/0.4.3/getrandom/>,
  <https://docs.rs/cc/1.4.2/cc/>, and
  <https://docs.rs/pkg-config/0.3.33/pkg_config/>.
- Implementation substrates:
  <https://docs.rs/hashbrown/0.17.1/hashbrown/> and
  <https://docs.rs/memchr/2.8.3/memchr/>.

## Exact selected releases

| Crate | Version | crates.io checksum |
|---|---:|---|
| `libc` | `0.2.189` | `3eaf3ede3fee6db1a4c2ee091bf8a8b4dccdc6d17f656fb07896ee72867612f2` |
| `cfg-if` | `1.0.4` | `9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801` |
| `getrandom` | `0.4.3` | `300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099` |
| `proc-macro2` | `1.0.107` | `985e7ec9bb745e6ce6535b544d84d6cd6f7ad8bd711c398938ae983b91a766d9` |
| `quote` | `1.0.47` | `1fbf4db142a473a8d80c26bbf18454ed458bf8d26c8219c331daecfdbd079001` |
| `syn` | `3.0.3` | `53e9bae58849f64dfa4f5d5ae372c8341f7305f82a3868709269343628b659a3` |
| `serde_core` | `1.0.229` | `67dca2c9c51e58a4791a4b1ed58308b39c64224d349a935ab5039aa360942a48` |
| `serde` | `1.0.229` | `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba` |
| `log` | `0.4.33` | `0ceec5bc11778974d1bcb055b18002eba7f4b3518b6a0081b3af5f21666da9ad` |
| `tracing-core` | `0.1.36` | `db97caf9d906fbde555dd62fa95ddba9eecfd14cb388e4f491a66d74cd5fb79a` |
| `bytes` | `1.12.1` | `fc652a48c352aef3ea3aed32080501cf3ef6ed5da78602a020c991775b0aff04` |
| `http` | `1.5.0` | `918d3568bebf352712bc2ef3d46a8bcf1a75b373be6539de198e9105cbbf9ce0` |
| `tower-service` | `0.3.3` | `8df9b6e13f2d32c91b9bd719c00d1958837bc7dec474d94952798cc8e69eeec3` |
| `futures-core` | `0.3.33` | `2cd50c473c80f6d7c3670a752354b8e569b1a7cbfdc0419ec88e5edad85e0dc7` |
| `rand_core` | `0.10.1` | `63b8176103e19a2643978565ca18b50549f6101881c443590420e4dc998a3c69` |
| `cc` | `1.4.2` | `5d262e149917187838d5b42777c8253bcb64500067342904e7d429499a6f277e` |
| `pkg-config` | `0.3.33` | `19f132c84eca552bf34cab8ec81f1c1dcc229b811638f9d283dceabe58c5569e` |
| `hashbrown` | `0.17.1` | `ed5909b6e89a2db4456e54cd5f673791d7eca6732202bbf2a9cc504fe2f9b84a` |
| `memchr` | `2.8.3` | `cf8baf1c55e62ffcace7a9f06f4bd9cd3f0c4beb022d3b367256b91b87513d98` |

## Selected release evidence

| Crate | Released | License | Declared Rust | Owners or publish teams | Reverse deps | Default packages | `unsafe` token indicator |
|---|---|---|---:|---|---:|---:|---:|
| `libc` 0.2.189 | 2026-07-21 | MIT OR Apache-2.0 | 1.65 | Rust libs and named owners | 13,594 | 1 | 675 |
| `cfg-if` 1.0.4 | 2025-10-15 | MIT OR Apache-2.0 | 1.32 | Rust libs and publisher | 2,696 | 1 | 0 |
| `getrandom` 0.4.3 | 2026-06-17 | MIT OR Apache-2.0 | 1.85 | rust-random maintainers | 3,880 | 2 | 114 |
| `proc-macro2` 1.0.107 | 2026-07-19 | MIT OR Apache-2.0 | 1.71 | named publisher | 14,720 | 2 | 6 |
| `quote` 1.0.47 | 2026-07-19 | MIT OR Apache-2.0 | 1.71 | named publisher | 16,641 | 3 | 0 |
| `syn` 3.0.3 | 2026-07-22 | MIT OR Apache-2.0 | 1.71 | named publisher | 17,037 | 4 | 138 |
| `serde_core` 1.0.229 | 2026-07-18 | MIT OR Apache-2.0 | 1.56 | serde-rs publish team and named publisher | 203 | 1 | 2 |
| `serde` 1.0.229 | 2026-07-18 | MIT OR Apache-2.0 | 1.56 | serde-rs publish team and named publisher | 113,469 | 2 | 2 |
| `log` 0.4.33 | 2026-06-20 | MIT OR Apache-2.0 | 1.71.0 | Rust project teams and named owners | 29,763 | 1 | 8 |
| `tracing-core` 0.1.36 | 2025-12-18 | MIT | 1.65.0 | tokio-rs publish team and named publisher | 315 | 2 | 33 |
| `bytes` 1.12.1 | 2026-07-08 | MIT | 1.57 | tokio-rs core and named owners | 15,228 | 1 | 151 |
| `http` 1.5.0 | 2026-07-29 | MIT OR Apache-2.0 | 1.57.0 | named owners | 7,075 | 3 | 65 |
| `tower-service` 0.3.3 | 2024-08-13 | MIT | Not declared | tower-rs publish team and named owners | 663 | 1 | 0 |
| `futures-core` 0.3.33 | 2026-07-18 | MIT OR Apache-2.0 | 1.36 | Rust publisher and named owners | 1,961 | 1 | 8 |
| `rand_core` 0.10.1 | 2026-04-13 | MIT OR Apache-2.0 | 1.85 | rust-random maintainers | 2,266 | 1 | 0 |
| `cc` 1.4.2 | 2026-08-08 | MIT OR Apache-2.0 | 1.64.0 | Rust libs and publisher | 3,814 | 3 | 19 |
| `pkg-config` 0.3.33 | 2026-04-12 | MIT OR Apache-2.0 | 1.31 | Rust publisher and named owners | 1,150 | 1 | 0 |
| `hashbrown` 0.17.1 | 2026-05-09 | MIT OR Apache-2.0 | 1.85.0 | Rust publisher and named owner | 2,126 | 4 | 495 |
| `memchr` 2.8.3 | 2026-07-08 | Unlicense OR MIT | 1.61 | named publisher | 1,788 | 1 | 340 |

Owner labels summarize the crates.io owner set. Exact owner logins are:

| Crate family | crates.io owner logins |
|---|---|
| libc | `github:rust-lang:libs`, `gnzlbg`, `huonw`, `JohnTitor`, `joshtriplett`, `rust-lang-owner` |
| cfg-if / cc | `github:rust-lang:libs`, `rust-lang-owner` |
| pkg-config | `joshtriplett`, `rust-lang-owner`, `sdroege` |
| getrandom / rand_core | `dhardy`, `github:rust-random:maintainers` |
| proc-macro2 / quote / syn | `dtolnay` |
| serde / serde_core | `dtolnay`, `github:serde-rs:publish` |
| log | `github:rust-lang-nursery:libs`, `github:rust-lang-nursery:log-owners`, `huonw`, `KodrAus`, `rust-lang-owner`, `sfackler` |
| tracing-core | `github:tokio-rs:publish-tracing`, `hawkw` |
| bytes | `carllerche`, `Darksonn`, `github:tokio-rs:core` |
| http | `carllerche`, `seanmonstar` |
| tower-service | `carllerche`, `github:tower-rs:publish`, `seanmonstar` |
| futures-core | `cramertj`, `rust-lang-owner`, `taiki-e` |
| hashbrown | `Amanieu`, `rust-lang-owner` |
| memchr | `BurntSushi` |

## Comparison cohort

| Crate | Version | Disposition | Reverse deps | Default Windows packages | Reason not in selected queue |
|---|---:|---|---:|---:|---|
| `tokio` | 1.53.1 | Domain foundation | 67,620 | 2 | runtime and feature policy; dedicated ECOS-Q04 subject |
| `regex` | 1.13.1 | Domain foundation | 21,878 | 5 | text-search implementation rather than shared cross-domain contract |
| `url` | 2.5.8 | Domain foundation | 14,455 | 35 | web data type and parsing policy; evaluate in profiles |
| `rustls` | 0.23.43 | Domain foundation | 3,745 | 21 | TLS implementation and provider policy |
| `clap` | 4.6.6 | Application choice | 43,872 | 15 | CLI consumer choice |
| `sqlx` | 0.9.0 | Application choice | 3,399 | 132 | database/runtime/feature-specific stack |
| `wgpu` | 30.0.0 | Application choice | 1,411 | 83 | GPU platform and consumer choice |
| `bitflags` | 2.13.1 | Widely reused utility | 4,784 | 1 | generated local flag types, not one shared type identity |
| `smallvec` | 1.15.2 | Widely reused utility | 3,593 | 1 | substitutable container optimization |
| `once_cell` | 1.21.4 | Transitional utility | 8,470 | 1 | overlaps stabilized standard-library primitives |
| `indexmap` | 2.14.0 | Widely reused utility | 5,552 | 3 | useful public container, but replacement remains capability-local |
| `thiserror` | 2.0.20 | Focused helper | 63,500 | 6 | derives standard error implementations, not a shared error identity |
| `anyhow` | 1.0.104 | Focused helper | 46,325 | 1 | application error aggregation policy |

## Observations

1. Reverse dependencies do not reveal whether a crate is exposed in public
   APIs, hidden behind a facade, or merely used during construction.
2. `serde_core` demonstrates why family and re-export identity matter: it has
   low direct reverse-dependency count while carrying the lower-level traits
   re-exported by the much larger `serde` facade.
3. Minimal closure and operational boundary are independent. `pkg-config`
   resolves one Rust package while activating a system package database and
   linker inputs.
4. The selected releases span a wide declared Rust-version range.
5. Lexical unsafe counts identify where ECOS-Q06 should look first, but they
   cannot support safety comparisons.

## Limitations

- Registry observations are point-in-time.
- The exact owner set is publishing authority, not governance evidence.
- Default features are not representative usage.
- The Windows target filter is one resolution profile, not a platform matrix.
- Metadata resolution is not compilation or execution.
- Unsafe token counts are not unsafe-code, soundness, or audit counts.
- Registry checksums do not identify repository commits.
