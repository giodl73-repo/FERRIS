# Pulse 01: Federated Validation Reconciliation

Status: Complete
Implementation authority: Bounded to this document
Budget: One implementation pulse, at most one corrective pass, no successor

## Outcome

Reconcile the preserved `application-definition-prototype` commits
`02a8337`, `eca5599`, and `ba3566f` with canonical current main at `cebce42`.
Add only the prototype's distinct relationship-aware validation composition as
the separate read-only `federated-validation-plan` command.

The prototype branch remains preserved but is superseded for product
integration. The merged request-based `federated-plan` remains canonical for
relationship-free application plan collation.

## Authorized files

- `crates/ferris-core/src/lib.rs`;
- `crates/ferris-cli/src/entrypoint.rs`;
- focused tests under `crates/ferris-cli/tests/`;
- the bounded fixture under `tests/fixtures/sibling-workspaces/`;
- `README.md` and the top-level bounded-authority section of `CONTEXT.md`;
- this wave, pulse, and one all-eleven-role review.

## Required behavior

- strict bounded `ferris.application/v0` input;
- explicit relationships only and acyclic;
- one independent bounded Cargo metadata result per declared workspace;
- direct inputs delegated to current single-workspace validation-plan logic;
- transitive reverse dependents widened to full-workspace owner fallback;
- an application-level unowned path widened to every workspace;
- qualified packages parsed as `WORKSPACE_ID:PACKAGE`;
- deterministic portable output and stable relocated identity;
- human and JSON output through direct and Cargo adapters;
- typed path-free failures; and
- no execution or mutation.

## Stop condition

Stop if the implementation changes an existing V0 contract, modifies
published validation-plan schemas, introduces shared resolution or inferred
relationships, executes owner validation, adds a dependency or unsafe code,
or requires a second architecture layer.

## Measured result

The implementation pulse completed, then independent review consumed the one
bounded corrective pass. The correction removed Application Definition
filename display metadata from semantic success/request/error identities and
added renamed-definition regressions. Package-component whitespace and shared
help parity were completed in the implementation pass.

Implemented behavior:

- three independent fixture workspaces retain separate Cargo metadata and
  lock authority;
- a directly selected package produces the unchanged single-workspace
  validation-plan record;
- two explicit reverse relationship hops widen transitively to workspace
  fallback without fabricated inputs;
- an application-owned path outside all workspace roots widens all three
  workspaces;
- strict shape, unknown workspace/package, invalid qualifier, cycle,
  duplicate ID/Cargo root, traversal, and outside-application controls return
  typed path-free failures;
- direct `ferris`, direct `cargo-ferris`, and Cargo-style invocation match;
- relocated equivalent fixtures with different Application Definition
  filenames retain selection, invocation, federated-validation-plan, and
  embedded validation-plan IDs; loaded typed error identities are stable too;
- current `validation-plan` and request-based `federated-plan` behavior and
  schemas remain unchanged; and
- no dependency, unsafe code, owner execution, or published schema change was
  added.

Validation:

```console
rustfmt --edition 2024 <changed Rust files>
cargo test -p ferris-core --lib --locked --quiet
cargo test -p ferris-cli --test federated_validation_plan --locked --quiet
cargo test -p ferris-cli --bin ferris --bin cargo-ferris --test cli --test federated_plan --locked --quiet
cargo test -p ferris-cli --test validation_plan_schema --locked --quiet
cargo check --workspace --locked --quiet
cargo clippy -p ferris-core --lib -p ferris-cli --bin ferris --bin cargo-ferris --test federated_validation_plan --test cli --test federated_plan --locked -- -D warnings
<parse changed JSON>
git diff --check
```

The core library passed 60 tests with 2 ignored historical controls. The
shared CLI invocation passed 10 `ferris` binary tests, 10 `cargo-ferris`
binary tests, 34 shared integration tests, 10 existing federated-plan tests,
8 focused federated-validation-plan tests, and 4 published validation-plan
schema tests. Workspace check and targeted Clippy passed.

## Boundaries retained

- Cargo metadata runs sequentially once for every declared workspace, even
  when only one workspace is directly affected. At 16 workspaces the
  direct-child timeout ceiling remains 480 seconds plus startup and cleanup.
- Timeout and output termination cover the direct Cargo child, not descendants
  of a custom wrapper.
- Every complete workspace must share the Application Definition parent as a
  common ancestor; different Windows drives cannot be grouped.
- Explicit `depends_on` is a consumer assertion for conservative propagation,
  not evidence of package, artifact, ABI, runtime, deployment, or support
  compatibility.
- The top-level result identity remains evidence-sensitive; the portable
  selection, invocation, federated-validation-plan, and nested
  validation-plan IDs are relocation- and definition-filename-stable. The
  `application_definition` filename is display metadata only.
- Errors for which no supported Application Definition semantic projection can
  be loaded use one non-revealing provisional request identity. That
  placeholder is not a semantic-stability claim.
- The command is unsupported, read-only, non-executable, and not the full
  APPLICATION-001 model.

No successor is authorized.
