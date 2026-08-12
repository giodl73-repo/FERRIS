---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:configuration-environments-secrets
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Configuration, Environments, and Secrets
status: source-custody
source_custody: partial
current_path: rust-production-engineering/01-CONFIGURATION-ENVIRONMENTS-AND-SECRETS.md
canonical_path: rust-production-engineering/01-CONFIGURATION-ENVIRONMENTS-AND-SECRETS.md
backsource_ids: [mdloom-backfill:rust-production-engineering:01-configuration-environments-secrets]
concepts: [configuration, environments, secrets, validation, precedence, identity, secret rotation]
root_concepts: [configuration]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Configuration, Environments, and Secrets

## The Big Picture

Configuration is an input contract, not a bag of strings. Production safety
depends on knowing which values are compiled, packaged, deployed, discovered at
runtime, or retrieved as secrets - and on rejecting invalid combinations before
the process accepts work.

```
+============================================================================+
|                       CONFIGURATION PIPELINE                               |
|                                                                            |
|  defaults --> file --> environment --> CLI                                 |
|     low precedence                    high precedence                      |
|          \________ merge + provenance ________/                            |
|                          |                                                 |
|                          v                                                 |
|                 parse + validate typed model                               |
|                          |                                                 |
|        secret references + workload identity                               |
|                          |                                                 |
|                          v                                                 |
|              resolve bounded capabilities/credentials                      |
|                    | pass              | fail                              |
|                    v                   v                                   |
|                start system        exit before serving                     |
+============================================================================+
```

Precedence is a policy and must be documented. Secret material and workload
identity are not generic highest-precedence configuration layers: validated
configuration should name the capability or secret reference, then a dedicated
adapter acquires it. More sources are not automatically better: every source
increases ambiguity, test surface, and the chance that an emergency override
becomes permanent.

## Classify Before Choosing a Mechanism

| Input class | Examples | Preferred ownership |
|---|---|---|
| Build identity | version, commit, feature set, target | embedded in artifact metadata |
| Non-secret deploy config | ports, limits, endpoint names | deployment config |
| Dynamic control | feature exposure, traffic percentage | explicit control plane with audit |
| Secret material | passwords, signing keys, tokens | secret manager or mounted protected file |
| Workload identity | service-to-service authentication | platform-issued identity where available |
| Discovered state | peer addresses, topology | service discovery, not static config |

An "environment" should be a set of deployed values and policy, not a branch of
code. Avoid `if production { ... }` when the real choice is a timeout, endpoint,
or capability.

## Typed Loading and Validation

Parse strings once at the boundary. The rest of the program should receive a
validated immutable value, ideally split so components see only what they need.

```toml
# Cargo.toml
[package]
name = "config-example"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

```rust
use serde::Deserialize;
use std::{env, fs, net::SocketAddr, time::Duration};

#[derive(Debug, Deserialize)]
struct FileConfig {
    listen: SocketAddr,
    request_timeout_ms: u64,
    max_in_flight: usize,
}

#[derive(Debug)]
struct Config {
    listen: SocketAddr,
    request_timeout: Duration,
    max_in_flight: usize,
}

fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let raw: FileConfig = toml::from_str(&text)?;
    let max_in_flight = env::var("APP_MAX_IN_FLIGHT")
        .ok()
        .map(|v| v.parse())
        .transpose()?
        .unwrap_or(raw.max_in_flight);

    if raw.request_timeout_ms == 0 || max_in_flight == 0 {
        return Err("timeouts and capacity must be non-zero".into());
    }

    Ok(Config {
        listen: raw.listen,
        request_timeout: Duration::from_millis(raw.request_timeout_ms),
        max_in_flight,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| "app.toml".into());
    let cfg = load(&path)?;
    println!(
        "listen={} timeout_ms={} max_in_flight={}",
        cfg.listen,
        cfg.request_timeout.as_millis(),
        cfg.max_in_flight
    );
    Ok(())
}
```

Scope: recent stable Rust, a local TOML file, and one documented environment
override. Run with:

```bash
printf 'listen = "127.0.0.1:8080"\nrequest_timeout_ms = 1500\nmax_in_flight = 64\n' > app.toml
cargo generate-lockfile
APP_MAX_IN_FLIGHT=32 cargo run --locked -- app.toml
```

Windows PowerShell uses `cargo generate-lockfile`, then
`$env:APP_MAX_IN_FLIGHT = "32"; cargo run --locked -- app.toml`.

## Validation Is More Than Parsing

```
syntax              semantic             environmental
------              --------             -------------
"1500" is a u64 --> timeout > 0      --> endpoint resolves
address parses  --> capacity bounded --> credential can be acquired
enum is known   --> modes compatible --> required directory is writable
```

Keep `--check-config` side-effect free where possible. It may verify that files
exist and identities can be acquired, but it should not migrate data, publish
messages, or bind public listeners.

## Secret Handling

Secrets require a lifecycle: acquisition, memory residence, use, rotation,
revocation, and audit.

| Technique | Strength | Risk |
|---|---|---|
| Environment variable | widely supported; easy injection | inherited by children; exposed by some diagnostics |
| Mounted file | permissions and atomic replacement possible | reload semantics and stale mounts |
| Secret-manager API | centralized policy, rotation, audit | bootstrap identity and control-plane dependency |
| Workload identity | avoids long-lived shared secret | platform coupling and token refresh behavior |

Do not log full configuration through `Debug`. Wrap secret types so accidental
formatting is redacted, and prefer passing credentials directly to the client
that uses them. Rotation should replace credentials without requiring unsafe
global mutation; a versioned client or atomically swapped credential holder is
usually clearer.

## Library, Runtime, and Platform Choices

| Layer | Decision |
|---|---|
| Library | `serde` plus TOML/YAML/JSON, `config`, `figment`, or a small explicit loader |
| Runtime | whether dynamic reload needs a watcher/task; static startup config needs none |
| Platform | environment injection, mounted files, workload identity, secret store, policy |

No Rust crate can make an insecure platform injection path safe. Conversely, a
secret manager does not prevent an application from putting a token in an error
message.

## Old World -> New World Bridge

The universal bridge is from **global process settings** to **typed dependency
injection**: load once, validate once, then pass narrow configuration values to
constructors. This is the same architectural move whether the predecessor was
an INI file, Java system properties, a .NET configuration provider chain, or a
shell environment.

Azure Key Vault and managed identity are supplemental examples of the
secret-manager and workload-identity patterns. Equivalent capabilities exist
on other clouds and in on-premises systems; application code should depend on a
small credential contract rather than on a cloud vocabulary everywhere.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Compile-time constant | value changes only with code and affects compatibility |
| Checked-in non-secret file | local development or a versioned safe baseline |
| Environment override | a small, documented set of deploy-time scalars |
| CLI option | operator-selected process behavior or diagnostic mode |
| Mounted secret file | platform supports protected mounts and rotation semantics are known |
| Secret-manager client | centralized retrieval, rotation, and audit justify the dependency |
| Workload identity | platform can issue short-lived identity without shared credentials |
| Dynamic config service | changes truly need runtime rollout, validation, audit, and rollback |

## Common Confusion Points

- **Twelve-factor guidance is not a mandate to put every value in the
  environment.** Large structured values and rotating secrets often fit other
  mechanisms better.
- **A default is a production decision.** Silent defaults for security,
  durability, or capacity can be more dangerous than startup failure.
- **Redaction after formatting is too late.** Prevent secrets from entering the
  formatted event.
- **Reload is a distributed state transition.** Define atomicity, partial
  failure, and rollback; a file watcher alone is not a control plane.
- **Environment names are not types.** `PROD=true` hides the specific behaviors
  that actually differ.

## Primary Sources

- Rust `std::env`: https://doc.rust-lang.org/std/env/
- Serde: https://serde.rs/
- Cargo configuration: https://doc.rust-lang.org/cargo/reference/config.html
- OWASP Secrets Management Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html
- NIST digital identity guidance: https://pages.nist.gov/800-63-4/

## Related Guides

- Previous: [00-OVERVIEW.md](00-OVERVIEW.md)
- Next: [02-STRUCTURED-LOGGING-AND-TRACING.md](02-STRUCTURED-LOGGING-AND-TRACING.md)
- Lifecycle validation: [05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md](05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md)
- Release-time configuration: [11-CI-CD-AND-PROMOTION.md](11-CI-CD-AND-PROMOTION.md)
