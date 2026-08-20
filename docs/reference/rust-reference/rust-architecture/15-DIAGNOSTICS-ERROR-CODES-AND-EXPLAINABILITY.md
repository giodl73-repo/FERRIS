---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:diagnostics-error-codes
kind: guide
module: rust-architecture
section: rust-architecture
title: Diagnostics, Error Codes, and Explainability
status: source-custody
source_custody: partial
current_path: rust-architecture/15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md
canonical_path: rust-architecture/15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md
backsource_ids: [proof-backfill:rust-architecture:15-diagnostics-error-codes]
concepts: [diagnostics, error codes, suggestions, lints, json diagnostics, ui tests]
root_concepts: [diagnostics]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Diagnostics, Error Codes, and Explainability

## The Big Picture

Rust diagnostics are not a side channel bolted onto `rustc`; they are a first-class compiler product. Any compiler phase can construct a `Diagnostic`, attach spans into the `SourceMap`, add notes, labels, and structured suggestions, and send it through the `Session` emitter described in [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md). The language authority remains the Rust Reference; the diagnostic machinery is rustc's implementation contract with humans and tools.

```
+===========================================================================+
|                    RUSTC DIAGNOSTIC PIPELINE                              |
|                                                                           |
|  parse/expand/typeck/MIR/borrowck/codegen/lints                           |
|        |                                                                  |
|        v                                                                  |
|  Diagnostic: level + code + primary Span + secondary spans                |
|        |       labels + notes/help + structured Suggestions               |
|        v                                                                  |
|  Session emitter  <---- SourceMap: files, byte offsets, line/column       |
|        |                                                                  |
|        +--> human: color, carets, labels, help                            |
|        +--> JSON: Cargo, rust-analyzer, IDEs, CI systems                  |
|        +--> short: compact terminal output                                |
+===========================================================================+
```

Read [04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md) before treating a caret as magic: diagnostics are only as precise as their spans.

---

## Authority Map

| Authority | Owns | Does not own |
|---|---|---|
| Rust Reference / language teams | Language semantics and compatibility commitments | rustc's diagnostic wording |
| rustc | Compiler diagnostics, spans, error codes, lint emission | Cargo scheduling or Clippy lint policy |
| Cargo | Build orchestration and `cargo fix` workflow | The diagnostic facts emitted by rustc |
| rustup | Installed toolchains, components, and target libraries | Compilation or lint policy |
| Standard library libs/libs-api teams | Public `core`/`alloc`/`std` API surface | Borrow checker diagnostics |
| LLVM/backends | Backend errors and codegen integration points | Front-end type and borrow explanations |
| Ecosystem tools | Clippy, rust-analyzer, IDE rendering, rustfix | Stable ownership of rustc internals |

---

## Diagnostic Anatomy: What Gets Emitted

A diagnostic is a structured object before it is text. That is the key architectural fact.

| Field | Purpose | Stable contract? |
|---|---|---|
| Level | error, warning, note, help, lint level, or ICE path | The broad categories are public behavior |
| Code | `E0502`, `E0308`, etc. when one exists | Stable-ish identifier, not a promise that every case has one |
| Primary span | Main source location | Anchored in public source text, produced by rustc internals |
| Secondary spans | Related source locations | Essential for borrowck and trait errors |
| Labels | Per-span explanation | Wording is not stable |
| Notes/help | Extra context and next actions | Wording is not stable |
| Suggestions | Machine-readable replacements | Structure matters; exact suggestion set can change |

Errors stop compilation. Warnings do not unless promoted. Lints are diagnostics managed by the lint-level lattice. An ICE is different: it is rustc saying the compiler violated its own invariants, not that the user's program is ill-formed.

---

## Structured Suggestions and Applicability

The visible `help: consider ...` line is backed by replacement spans. That is why `cargo fix` can edit code instead of scraping terminal text.

```
+-------------------+       +-----------------------+
| Diagnostic help   | ----> | Suggestion            |
| "try .to_string"  |       | span: byte range      |
+-------------------+       | replacement: text     |
                            | applicability: enum  |
                            +-----------------------+
                                      |
                                      v
                         cargo fix / rustfix applies only
                         safe MachineApplicable edits
```

| Applicability | Meaning | Tool behavior |
|---|---|---|
| `MachineApplicable` | Compiler believes the edit is correct and complete | `cargo fix` / `rustfix` may apply automatically |
| `MaybeIncorrect` | Plausible, needs human judgment | Show to user, do not silently apply |
| `HasPlaceholders` | Edit contains holes like `todo!()` or `_` | Present as guidance |
| `Unspecified` | No confidence recorded | Treat conservatively |

Edition migrations under [01](01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md), compatibility cleanups, and some lint fixes depend on this path. Cargo orchestrates the command; rustfix applies the suggestions; rustc remains the authority that emitted them. See [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md) for the build context and [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md) for ecosystem tools that consume similar diagnostic streams.

---

## Error Codes and `--explain`

Many hard errors carry codes. A code is a useful handle into the Error Codes
Index; neither the code emitted for a particular program nor the short terminal
message is a stable automation contract.

| Command or URL | What it gives you |
|---|---|
| `rustc --explain E0502` | Extended explanation of one error code |
| `rustc --explain E0308` | Type mismatch writeup with examples |
| `https://doc.rust-lang.org/error_codes/` | The published Error Codes Index |
| Terminal diagnostic | Specific source-spanned failure in your build |

Not every diagnostic has a code. Some lints use lint names instead. Codes such
as `E0502` and `E0308` are useful documentation and search handles, but they are
not a compiler API: codes can be retired, and compiler changes can alter which
code a particular program receives. Exact prose, suggestions, spans, and
emitting phases are even more version-sensitive.

---

## Lints, Levels, and Compatibility Pressure

Lints are diagnostics with a policy layer. The compiler has built-in lints; Clippy is a separate ecosystem tool that plugs into the same broad lint model.

```
+----------------+     attributes      +----------------------+
| source crate   | ------------------> | #[allow(dead_code)]  |
+----------------+                     +----------------------+
       |          CLI flags                         |
       +-------> -A/-W/-D/-F lint-name              |
       |                                            v
       |          Cargo manifest             +--------------+
       +-------> [lints] table ------------> | lint levels  |
                                             +--------------+
```

| Level | Meaning | Can later override? |
|---|---|---|
| `allow` | Suppress the lint | Yes |
| `warn` | Print warning | Yes |
| `deny` | Emit as error | Yes, unless capped or forbidden |
| `forbid` | Emit as error and cannot be lowered | No |

Lint groups let teams set policy at scale. `-D warnings` is useful at application boundaries and CI gates, but `#[deny(warnings)]` in libraries is an anti-pattern: it can break downstream users when a new compiler gains a new warning. Cargo's cap-lints behavior keeps dependency warnings from breaking the top-level crate's build. Future-incompat lints are the compatibility pressure valve: warn now, reserve the right to reject later under the language and edition process.

---

## Output Formats and Tool Consumers

The human renderer is optimized for attention. The JSON renderer is optimized for integration.

| Format | Command | Primary consumer |
|---|---|---|
| Human default | `cargo build` | Developer terminal |
| Short | `rustc --error-format=short` | Dense logs |
| JSON | `rustc --error-format=json` | Cargo, IDEs, rust-analyzer, CI parsers |
| Cargo JSON stream | `cargo build --message-format=json` | Build systems and automation |
| Rendered ANSI inside JSON | `--json=diagnostic-rendered-ansi` | Tools that want both structure and rustc's rendering |

```
$ cargo build --message-format=json | jq 'select(.reason == "compiler-message") | .message.code.code'
"E0308"
```

The Cargo envelope (`reason`, package/target context, artifact messages) is the
right integration entry point, but a `compiler-message` embeds rustc's diagnostic
JSON payload, which is explicitly outside rustc's stability guarantees. Robust
consumers should pin/test supported toolchains, ignore unknown fields, tolerate
missing or changed codes, and never parse the rendered English text.

---

## A Concrete Diagnostic Trace

The following is the normal loop: emit a precise diagnostic, inspect the extended explanation, then decide whether a suggestion or lint policy should be applied.

| Step | Artifact |
|---|---|
| Compile | Spanned diagnostic with code and help |
| Explain | Error-index narrative via `rustc --explain` |
| Enforce | Lint levels or warning promotion |
| Repair | Human edit or `cargo fix` for safe suggestions |

```rust
// main.rs
fn main() {
    let s: String = "hello";
    println!("{s}");
}
```

```text
$ rustc main.rs
error[E0308]: mismatched types
 --> main.rs:2:21
  |
2 |     let s: String = "hello";
  |            ------   ^^^^^^^ expected `String`, found `&str`
  |            |
  |            expected due to this
  |
help: try using a conversion method
  |
2 |     let s: String = "hello".to_string();
  |                            ++++++++++++

$ rustc --explain E0308
# prints the extended Error Codes Index entry

$ rustc -D warnings main.rs
# promotes warnings to errors for this invocation

#[allow(dead_code)]
fn intentionally_unused() {}
```

Borrow checking uses the same machinery but with multi-span causality: one span for the original immutable borrow, one for the later mutable borrow, and one for the later use that keeps the first borrow live. That precision is why [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) and later borrow analysis can produce diagnostics that feel semantic rather than merely syntactic.

---

## UI Tests: Regression Control for Diagnostics

rustc's own repository pins diagnostic output with UI tests. This is internal project process, not a user-facing stability guarantee, but it explains the polish.

| Piece | Role |
|---|---|
| `tests/ui/*.rs` | Source programs expected to fail or warn in particular ways |
| `.stderr` snapshots | Expected rendered diagnostics |
| compiletest | Harness that compares actual output to snapshots |
| `--bless` | Developer command to accept intentional output changes |
| CI | Prevents accidental diagnostic regressions |

See [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) for the broader bootstrap, CI, perf, and distribution system.

---

## Old World -> New World Bridge

| Old world | Rust analogue | Important difference |
|---|---|---|
| MSVC `Cxxxx` / C# `CSxxxx` | `E0502`, `E0308`, Error Codes Index | Rust codes often encode ownership/type-system concepts, not just parser/type errors |
| Roslyn diagnostics | rustc JSON diagnostics | rustc is not a stable compiler-as-a-service API |
| Roslyn code fixes / lightbulbs | structured suggestions + rustfix / `cargo fix` | Only `MachineApplicable` edits should be automatic |
| SARIF / diagnostic streams | `--error-format=json`, Cargo message stream | JSON is the native rustc/Cargo integration path |
| warning-as-error / EditorConfig severity | lint levels, `-D warnings`, `[lints]` | `forbid` is intentionally stronger than `deny` |

If you remember Visual Studio turning compiler facts into IDE affordances, that is the right model. Rust's twist is that the compiler emits enough structure for tools without exposing a Roslyn-equivalent stable semantic API.

---

## Stability Boundary

| Documented user surface | Version-sensitive diagnostic detail |
|---|---|
| CLI switches such as `--error-format=json` and Cargo `--message-format=json` | rustc diagnostic JSON schema and fields inside Cargo `compiler-message` |
| Public lint names, groups, and levels for the selected toolchain | Mapping from one source program to an `E####` code, spans, suggestions, and prose |
| `cargo fix` workflow for machine-applicable suggestions | Which rustc phase emitted a diagnostic and the internal builder types |
| Cargo's documented message envelope | UI-test harness layout and `.stderr` snapshot mechanics |

This boundary matters operationally. Use documented lint names for policy.
For compiler-diagnostic automation, pin or test the supported compiler range and
parse defensively; do not assert exact English text or assume an error code/JSON
payload is immutable across releases.

---

## Decision Cheat Sheet

| I want to... | Use |
|---|---|
| Understand one compiler error deeply | `rustc --explain E####` |
| Feed diagnostics to a tool | `cargo build --message-format=json`, with tolerant parsing and toolchain-version tests |
| Promote warnings in an application CI build | `cargo clippy -- -D warnings` or `RUSTFLAGS="-D warnings"` with care |
| Silence an intentional local warning | `#[allow(lint_name)]` at the narrowest scope |
| Enforce a lint as policy | `[lints]` in `Cargo.toml` or `-D lint-name` in CI |
| Apply safe compiler fixes | `cargo fix` and review the diff |
| Investigate diagnostic regressions in rustc itself | rustc `tests/ui` snapshots and `--bless` |

---

## Common Confusion Points

| Confusion | Correction |
|---|---|
| "The error text is stable." | No. Use the documented JSON command surface, but treat rustc's payload, codes, spans, and prose as version-sensitive. |
| "Every error has an `E` code." | No. Many do, some do not; lints primarily have lint names. |
| "Clippy is rustc." | No. Clippy is a separate tool, though it uses compiler infrastructure. |
| "`cargo fix` applies every suggestion." | No. It is conservative and primarily applies `MachineApplicable` edits. |
| "`#[deny(warnings)]` is good library hygiene." | Usually no. It can make downstream builds fail when compiler warnings evolve. |
| "UI tests make wording stable for users." | No. They prevent accidental regressions inside rustc development. |

---

## Primary Sources

| Source | Why it matters |
|---|---|
| rustc-dev-guide: Emitting diagnostics | How diagnostics are built and emitted inside rustc |
| rustc-dev-guide: Diagnostic and subdiagnostic structs | The structured diagnostic model |
| rustc-dev-guide: Errors and lints / Adding a lint | Lint architecture and contributor workflow |
| rustc-dev-guide: UI tests | Snapshot-based diagnostic regression process |
| Rust Error Codes Index | Published `E####` explanations |
| rustc book: lint levels, `--error-format`, JSON output | CLI-facing diagnostic controls |
| `rust-lang/rustfix` | Suggestion application engine used by Cargo workflows |

*Cross-links:* start with the landscape in [00](00-OVERVIEW.md), then read [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md) for `Session`, [04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md) for spans, [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md) for tool consumers, and [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) for compiler CI.