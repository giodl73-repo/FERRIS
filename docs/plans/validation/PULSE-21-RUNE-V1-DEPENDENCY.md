# Pulse 21 RUNE v1 Dependency Validation

Status: Complete
Decision: CONTRACT-001 Typebook/RUNE v1 contract-baseline dependency satisfied

## Evidence set

| Kind | Count | Record |
|---|---:|---|
| Closed Draft 2020-12 schema | 1 | [`ferris.rune-v1-dependency-receipt.v1.schema.json`](ferris.rune-v1-dependency-receipt.v1.schema.json) |
| Canonical receipt fixture | 1 | [`PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT.json`](PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT.json) |
| Negative-control mutations | 13 | [`PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT-MUTATIONS.json`](PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT-MUTATIONS.json) |
| Test-only Rust validator | 1 | [`rune_v1_dependency_receipt.rs`](../../../crates/ferris-cli/tests/rune_v1_dependency_receipt.rs) |

The validator checks schema closure, deterministic receipt identity, the exact
RUNE revision and version facts, eight accepted specification rows, the CLI
v1 surface, retained registry and compatibility dispositions, dependency
status, the unchanged fixture binding, and equality with the platform-profile
schema README and controlled semantic fixture.

The 13 mutations reject false SemVer publication, a false Git v1 tag, revision
or workspace-version changes, an accepted-row count change, a profile-version
change, fixture regeneration, profile identity or digest changes, failed
validation, an unsatisfied dependency, unknown fields, and receipt-identity
mismatch.

## Receipt identity

Receipt identity is deterministic:

1. clone the receipt and replace `receipt_identity` with the empty string;
2. serialize compact UTF-8 JSON with object members sorted lexicographically
   and arrays retained in declared order;
3. frame the bytes as
   `ferris.rune-v1-dependency-receipt/v1 NUL <canonical-json>`; and
4. store the lowercase SHA-256 digest with the `sha256:` prefix.

## Decision boundary

Revision `194449444624fb10add4137cb0da8d0327164fa7` satisfies the
Typebook/RUNE v1 dependency as an accepted contract and release-readiness
baseline. It does not represent Cargo SemVer `1.0.0` publication or a Git
`v1.0.0` tag. The workspace remains `0.1.0`; the controlled collection and
neutral profile remain `v0`.

No RUNE source, FERRIS production behavior, semantic fixture bytes, profile
identities, profile digests, Pulse 17 result, Pulse 19 result, or Pulse 20
protocol changed.

## FERRIS validation

```console
cargo fmt --all -- --check
cargo check --locked --workspace
cargo test --locked -p ferris-cli --test rune_v1_dependency_receipt
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
git diff --check
```

All commands passed. Changed Markdown links and code fences also passed local
validation.
