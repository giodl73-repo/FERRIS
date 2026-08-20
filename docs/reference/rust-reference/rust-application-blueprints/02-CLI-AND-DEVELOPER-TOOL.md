---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:cli-and-developer-tool
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: CLI and Developer Tool Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/02-CLI-AND-DEVELOPER-TOOL.md
canonical_path: rust-application-blueprints/02-CLI-AND-DEVELOPER-TOOL.md
backsource_ids: [proof-backfill:rust-application-blueprints:02-cli-and-developer-tool]
concepts: [command line interface, developer tool, exit code, stdout, stderr, shell automation, cargo workspace]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# CLI and Developer Tool Blueprint

## The Big Picture

```
+============================================================================+
| CALLER: person | shell | CI job | editor | another process                 |
+-------------------------------+--------------------------------------------+
                                v
+----------------------------------------------------------------------------+
| COMMAND CONTRACT                                                           |
| argv + env + cwd + stdin -> parse -> validate -> dispatch                  |
+-------------------------------+--------------------------------------------+
                                v
+----------------------------------------------------------------------------+
| APPLICATION USE CASE                                                       |
| plan -> effect ports -> progress/events -> outcome                         |
+-------------------+----------------------+---------------------------------+
                    v                      v
        filesystem / network / VCS     renderer
                    |                      |
                    +----------+-----------+
                               v
                  stdout + stderr + exit status
```

A CLI is a process protocol. Human ergonomics matter, but automation contracts
matter more: arguments, streams, exit statuses, current-directory behavior,
configuration precedence, cancellation, and output stability are public
surfaces once scripts depend on them.

## Workspace and Composition Root

```
forge-tool/
|-- Cargo.toml
|-- crates/
|   |-- forge-core/             # plans and domain types
|   |-- forge-application/      # commands/use cases and ports
|   |-- forge-fs/               # filesystem adapter
|   `-- forge-git/              # VCS adapter
|-- apps/
|   `-- forge-cli/
|       |-- Cargo.toml
|       `-- src/{main.rs,args.rs,render.rs}
`-- tests/
    `-- cli-scenarios/
```

```toml
# Cargo.toml (virtual workspace root)
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

```toml
# apps/forge-cli/Cargo.toml
[package]
name = "forge-cli"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "forge"
path = "src/main.rs"
```

`main` should construct settings and adapters, invoke one application command,
render its result, and map it to an exit status. Parsing libraries are useful,
but their types should not leak into the application layer.

## Command and Stream Contracts

| Surface | Stable policy |
|---------|---------------|
| Arguments | names, defaults, conflicts, deprecations, non-interactive form |
| `stdout` | requested data or machine-readable result |
| `stderr` | diagnostics, progress, warnings |
| Exit status | small documented taxonomy; zero means contract success |
| `stdin` | explicit data mode; never surprise an interactive terminal |
| CWD | define whether paths are relative to invocation or discovered root |
| Environment | documented precedence and secret-redaction behavior |

```
human mode:    diagnostics + formatted result
machine mode:  stable schema on stdout
               diagnostics on stderr
               no prompts, color, or progress unless requested
```

Do not promise that prose output is a stable parser interface. Provide a
versioned structured mode when automation needs fields. Detecting terminals can
improve defaults, but an explicit flag must override detection.

## Execution and Operational Boundaries

The CLI owns:

- validation before effects;
- cancellation response, normally from an OS signal or closed stream;
- atomic file replacement where partial output would be corrupt;
- credential acquisition policy without printing secrets;
- a deterministic final status.

The application owns the meaning of the command. Adapters own system calls,
network protocols, and VCS behavior. CI or the invoking shell owns retry unless
the CLI documents a narrower retry for an idempotent remote operation.

| Surface | Owner |
|---------|-------|
| Command vocabulary and compatibility | CLI/application owner |
| Domain mutation and validation | application owner |
| Filesystem/network protocol fidelity | adapter owner |
| Packaging and distribution | release owner |
| Invocation retry and secret injection | caller/CI owner unless delegated explicitly |

| Operation | Preferred boundary |
|-----------|--------------------|
| Inspect/plan | side-effect-free result suitable for `--dry-run` |
| Mutate one file | write temporary sibling, flush as required, rename |
| Mutate many files | explicit plan plus journal or recoverable staging area |
| Remote request | timeout and idempotency semantics |
| Long run | progress events, cancellation points, resumable state if valuable |

Atomic replacement is filesystem- and platform-bounded: create the temporary
file on the same filesystem, preserve required permissions/metadata, define
overwrite behavior, and distinguish process-visible replacement from durable
flush-to-storage. For recursive or repository-root operations, define symlink,
reparse-point, path traversal, and time-of-check/time-of-use policy before
following paths.

Secrets on command lines can appear in process listings and shell history.
Prefer stdin, inherited handles, or an approved secret provider; redact
diagnostics and structured output. If the CLI discovers configuration from the
current directory, state whether untrusted repository files can influence
credential use or executable hooks.

## Testing and Rollback

```
parser tests
    -> application scenario tests
        -> process tests (argv/env/cwd/streams/status)
            -> packaged-artifact smoke test
```

Test the built process rather than calling `main` internals. Scenario fixtures
should use repository-owned directories and must not depend on a developer's
home configuration.

Useful baseline:

```text
cargo test --workspace --all-targets
cargo run -p forge-cli --bin forge -- --help
cargo run -p forge-cli --bin forge -- plan --format json
```

Rollback has two axes:

| Surface | Requirement |
|---------|-------------|
| Binary | retain previous signed/published artifact where distribution permits |
| Output/data | new version must not leave old version unable to read or recover |
| Config | support a deprecation window before removing keys |
| Script contract | add aliases or compatibility mode before breaking automation |

A downgraded executable cannot undo an already-applied external mutation. For
destructive commands, provide plan/apply separation, backups, or a documented
forward-repair path.

## Universal Bridge First

The useful bridge is Unix process algebra: small tools compose through explicit
streams and status, while richer CLIs add a typed command protocol. The same
principle appears in build tools and database clients: human presentation and
machine contracts are separate views of one result.

Supplementally, a .NET global tool or PowerShell cmdlet maps to the same
application core. PowerShell objects offer a richer in-process pipeline; a
portable Rust executable should still expose a stable serialized mode rather
than assuming that host.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Human-only exploratory command | readable default output plus precise status |
| CI/script consumption | non-interactive mode and versioned structured output |
| Destructive mutation | plan/apply, explicit confirmation only in human mode |
| Many subcommands | command modules calling shared application use cases |
| Reuse by another program | extract library [08], do not shell out by default |
| Hours-long durable work | scheduled/batch [05] or worker [04] with a thin CLI trigger |
| OS-specific integration | neutral core plus Windows/native adapter [12] |

## Common Confusion Points

- **`stdout` is not for all text.** Diagnostics on `stdout` corrupt pipelines.
- **A zero exit status must mean semantic success.** "Command ran but operation
  failed" is still failure unless the contract explicitly returns a queued job.
- **Interactive prompts break automation.** Require an explicit interactive
  context and always provide a non-interactive alternative.
- **`--dry-run` must execute planning logic.** A hard-coded message proves
  nothing about the real mutation path.
- **Async does not make a CLI a worker.** Process lifetime and completion
  authority still belong to the caller.
- **Shelling out is an integration boundary.** Capture encoding, quoting,
  status, cancellation, and tool-version assumptions.
- **Atomic rename is not a durability proof.** Filesystem, overwrite, metadata,
  and flush semantics must match the promised recovery contract.

## Primary Sources

- Rust `std::process`: https://doc.rust-lang.org/std/process/
- Rust `std::env`: https://doc.rust-lang.org/std/env/
- Cargo package targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- Cargo testing: https://doc.rust-lang.org/cargo/commands/cargo-test.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

## Related Guides

- Shared contract: [01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md](01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md)
- Reusable library: [08-REUSABLE-LIBRARY-AND-SDK.md](08-REUSABLE-LIBRARY-AND-SDK.md)
