# Pulse 01: Federated Application Plan

Status: Complete
Implementation authority: Bounded to this document
Budget: One implementation pulse, one corrective review pass, no successor

## Outcome

Add one local read-only `federated-plan` command that replaces repeated manual
invocation and collation of independent one-workspace Blueprint Plans.

Cargo remains authoritative for each workspace. The request author owns the
application grouping. FERRIS owns only bounded request validation,
orchestration, portable identity, and presentation.

## Authorized files

- `crates/ferris-core/src/lib.rs`;
- `crates/ferris-cli/src/entrypoint.rs`;
- `crates/ferris-cli/tests/cli.rs`;
- `crates/ferris-cli/tests/federated_plan.rs`;
- `tests/fixtures/federated-plan/**`;
- `README.md`;
- `CONTEXT.md`;
- this wave, this pulse, and its final review.

## Required behavior

- accept one strict request of at most 1 MiB with schema
  `ferris.federated-plan-request/v0`;
- require 2-16 explicit portable workspace selections;
- require forward-slash descendant manifest paths and a request parent that
  contains every selected manifest and complete Cargo workspace;
- invoke Cargo metadata once per workspace with `--no-deps --offline
  --locked`, a 30-second direct-child timeout, and 4 MiB retained per stream;
- reject duplicate manifest files and duplicate canonical Cargo workspace
  roots;
- return one deterministic `ferris.federated-plan/v0` record containing one
  unchanged `ferris.blueprint-plan/v0` per workspace;
- keep errors typed, path-free, and attributable to portable workspace IDs;
  and
- remain explicitly non-executable.

## Stop conditions

Stop rather than widen scope if the implementation requires:

- shared Cargo resolution or a shared lock identity;
- cross-workspace dependency, affected, validation, native, service, or
  contract inference;
- execution, mutation, hidden discovery, networking, connectors, MCP, AI
  narrowing, approval, deployment, or remote evidence;
- the canonical APPLICATION-001 Application Definition;
- a process-tree control subsystem, new dependency, or unsafe platform FFI;
  or
- another pulse or successor chain.

## Measured result

The pulse completed inside the authorized surface:

- two independent locked fixture workspaces produce a sorted portable record;
- direct `ferris`, direct `cargo-ferris`, and Cargo-style invocation return
  the same result;
- relocated fixtures retain selection, invocation, plan, and federated-plan
  identities;
- strict shape, unsupported schema, cardinality, invalid identifiers,
  duplicate IDs, duplicate manifests, duplicate Cargo workspace roots,
  absolute paths, backslashes, traversal, outer workspace roots, invalid and
  missing manifests, unavailable request identities, and oversized input have
  negative controls;
- nested PlanRecords match direct one-workspace planning;
- the existing one-workspace plan identity remains unchanged; and
- help parity includes the new command.

Focused validation on Windows:

```console
rustfmt --edition 2024 crates\ferris-core\src\lib.rs crates\ferris-cli\src\entrypoint.rs crates\ferris-cli\tests\federated_plan.rs crates\ferris-cli\tests\cli.rs
cargo test -p ferris-core federated_plan --locked --quiet
cargo test -p ferris-cli --test federated_plan --locked --quiet
cargo test -p ferris-cli --bin ferris --bin cargo-ferris --test cli --locked --quiet
cargo check --workspace --locked --quiet
cargo clippy -p ferris-core --lib -p ferris-cli --bin ferris --bin cargo-ferris --test federated_plan --test cli --locked -- -D warnings
git diff --check
```

All focused commands passed. The core filter passed 4 tests, the dedicated
CLI target passed 10 tests, and the shared CLI gate passed 10 binary tests,
10 adapter tests, and 34 integration tests.

Full-workspace tests and full-workspace rustfmt retain inherited failures in
unchanged historical diagnostic authority files. They are baseline evidence,
not a clean-workspace claim and not authority to modify those files.

## Boundaries retained

- The timeout and output termination scope is the direct Cargo child, as
  recorded in bounded-output evidence; descendants of a custom wrapper are
  outside V0.
- Sixteen sequential 30-second bounds permit up to 480 seconds plus process
  startup and cleanup.
- One request-parent ancestor cannot span different Windows drives.
- The retained Blueprint Plan contains no Cargo lock digest.
- No support, compatibility, performance, correctness, conformance, or
  production claim follows.

## Review

The final all-eleven-role disposition is recorded in
`docs/plans/reviews/FERRIS-FEDERATED-APPLICATION-PLAN-REVIEW.md`.

No successor is authorized.
