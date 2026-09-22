# Owner Failure Policy

Status: Complete bounded implementation

## Outcome Contract

The Ferris user outcome was to turn a typed Cargo failure diagnosis into a
deterministic owner-controlled disposition without allowing Ferris to invent or
execute a repair. The budget allowed one passive policy contract, one CLI
command, strict schemas, and focused regression coverage. Completion required
matched and fallback decisions, tamper rejection, bounded inputs, and no action
authority. Resolving owner actions or creating an execution layer was the
abandonment condition.

Product Value Governor disposition: `continue-within-budget`.

## Result

`ferris failure-policy` accepts one owner-authored
`ferris.failure-policy/v1` and one successful `diagnose-cargo` JSON result. It
matches unique rules for `dependency`, `lockfile`, `offline_policy`, or
`unclassified`, then emits `ferris.failure-policy-decision/v1` with one of:

- `halt`;
- `route`; or
- `prepare_action`.

Unmatched classifications use the required fallback. The selected
`owner_action_id` remains opaque. Policy and diagnosis inputs are bounded and
strict; duplicate classifications, unknown fields, malformed input, report
tampering, noncanonical command identities, and result-identity tampering fail
closed.

The command supports file input and EOF-delimited stdin, so the intended
pipeline is:

```console
ferris diagnose-cargo --stderr offline.stderr --format json |
  ferris failure-policy --policy policy.json --diagnosis - --format json
```

## Boundaries

The decision does not inspect raw stderr, infer root cause, resolve the owner
action to a command, create an Action Plan, grant approval, retry Cargo, or
execute work. `prepare_action` records owner intent only. This implementation
does not establish policy adoption, repair correctness, failure prevalence, or
cross-version contract compatibility.

Role review found no safety or semantic-transfer veto: the Rust Safety Steward,
Interop Boundary Auditor, AI Assurance Skeptic, Rust Maintainer, Scope Keeper,
and Validation Checker boundaries are represented in the strict identities,
passive output, negative controls, and explicit limitations. The Compiler
Performance Engineer has no performance claim to assess. The Autonomy
Supervisor stop condition is met by ending at the passive decision contract.

## Validation

Validated on Windows with Cargo
`1.95.0 (1.95.0-ms-20260618.5+ed80dadd6a)`:

| Command | Result |
|---|---|
| `cargo fmt -p ferris-core -- --check` | passed |
| `rustfmt --edition 2024 --check crates/ferris-cli/tests/failure_policy.rs` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed |
| `cargo test -p ferris-core --lib` | 131 passed, 2 ignored |
| `cargo test -p ferris-cli --bin ferris` | 10 passed |
| `cargo test -p ferris-cli --test cli` | 62 passed |
| `cargo test -p ferris-cli --test cargo_failure` | 5 passed |
| `cargo test -p ferris-cli --test contracts` | 7 passed |
| `cargo test -p ferris-cli --test failure_policy` | 5 passed |

The failure-policy suite covers all entrypoints, stdin composition, matched
routing, `prepare_action`, fallback, schema catalog entries, missing, empty,
oversized, duplicate, malformed, and tampered inputs. One negative control
recomputes the outer result identity after forging the diagnosis selection
identity; canonical semantic validation still rejects it.

Workspace-wide `cargo fmt --all -- --check` remains excluded because Rust 1.95
reports unrelated historical CLI fixture formatting deltas. Those files were
not rewritten by this pulse.

## Decision

The passive policy boundary is complete. A future capability may translate an
explicit `prepare_action` decision into a separately reviewed Action Plan, but
that would change layers and requires a new Product Value Governor disposition.
