# EXP-01: Cross-Command Scope Matrix

Date: 2026-08-10
Question: BLUE-Q03
Result: scope must be modeled as typed coordinates and mappings; package scope
alone cannot explain compilation, execution, or validation across commands.

| Activity | Package/target selection | Additional compile scope | Runtime scope | Valid claim |
|---|---|---|---|---|
| `cargo check` | selected packages and targets | analysis units, build scripts, proc macros, dependencies | none | selected analysis and diagnostics completed |
| `cargo build` | selected buildable targets | codegen, objects, archives, links | none | selected artifacts built |
| `cargo clippy` | selected packages and targets | Clippy-specific analysis and diagnostics | none | selected lint activity completed |
| `cargo test` | selected packages and test-capable targets | test cfg, harnesses, unit/integration targets, dev dependencies | test binaries and selected cases | executed selected tests passed |
| filtered test | usually same containing test target | usually same compiled test binary | matching test cases only | only executed matching cases passed |
| `cargo doc` | selected documentable targets | rustdoc extraction/rendering and metadata-mode dependencies | none | selected documentation generated |
| doctest | selected documentable targets | extracted temporary doctest crates | generated doctest cases | executed doctest cases passed |
| `cargo run` | selected binary target | build and link | one process plus runtime inputs | selected process executed with observed result |
| package | selected package | source/archive/license/generated/native assembly | none | distribution artifact constructed |
| deploy | application-selected artifacts | optional packaging/signing/finalization | service/environment/deployment target | observed deployment gates completed |

## Mapping cardinalities

| Mapping | Typical cardinality |
|---|---|
| file -> module/item | one-to-many |
| module/item -> target | many-to-many |
| package -> target | one-to-many |
| package/target -> Cargo unit | one-to-many by activity/configuration |
| Cargo unit -> rustc invocation | approximately one-to-one per unit execution |
| crate -> owner/body/query | one-to-many |
| generic definitions and consumers -> mono items | many-to-many |
| mono items -> CGUs | many-to-many under partition changes |
| CGUs/objects/native libraries -> link output | many-to-one |
| test binary -> test cases | one-to-many |
| test filter -> executed cases | one-to-many conditional selection |
| contract operation -> packages/services/tests | many-to-many |
| component/service -> artifacts/configuration/deployment | many-to-many |
| observed scopes -> Query Forest root | many-to-one evidence projection |

## Required evidence states

Every mapping is one of:

- declared;
- resolved;
- observed;
- inferred;
- unsupported;
- stale;
- not observed; or
- unknown.

Only evidence-supported mappings may narrow work. Unsupported, stale,
not-observed, or unknown mappings trigger the named owner-boundary fallback.

## AI design controls

| Concern | Rule |
|---|---|
| baseline | start from repository, package, target, activity, platform, validation, artifact, and root anchors |
| fine grain | add module, item, body, query, test-case, or runtime detail only with cited tool evidence |
| narrowing | AI proposes; deterministic policy or human approval authorizes |
| widening | stale, conflicting, unsupported, not-observed, or unknown evidence widens automatically |
| reproducibility | canonical scope IDs, mappings, set operations, mandatory gates, and fallback are deterministic |
| efficiency | cap mapping size, traversal, evidence collection, AI context, and renewal cost |
| overflow | summarize under the nearest safe anchor; never drop affected scope |

## Scope anti-pattern controls

Reject as authoritative:

- whole-workspace scope except as an explicit fallback;
- changed paths without ownership and downstream mappings;
- package or dependency scope presented as complete validation;
- command names without exact selections and configuration;
- runtime test filters presented as compilation scope;
- cache keys, artifact filenames, refs, or labels presented as identity;
- AI-generated semantic clusters without observed or declared mappings;
- compiler-private IDs as durable application scope;
- configuration-free or timeless mappings;
- one flattened green/pass scope;
- ownerless scopes; and
- scopes with no uncertainty fallback or complexity budget.

## Held-out fixtures

SCOPE-001 conformance should include:

1. private body edit;
2. public signature edit;
3. inline or generic body edit;
4. module/visibility change;
5. feature-conditional dependency;
6. host/target platform split;
7. unit-test edit and runtime filter;
8. integration-test target;
9. doctest edit;
10. build-script generated output;
11. proc-macro input;
12. native provider or discovered-library change;
13. contract-operation change;
14. runtime configuration change;
15. repository-policy change;
16. unknown file with full fallback; and
17. complete Blueprint removal followed by ordinary commands.
