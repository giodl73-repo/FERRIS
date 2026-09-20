# FERRIS Application Readiness Implementation Review

Date: 2026-09-19
Status: Complete
Specification: APP-READINESS-001
Pulse: Environment Readiness Pulse 07
Disposition: Bounded explicit-file V1 accepted; stop after Pulse 07

## Executive disposition

All eleven roles accept the optional
`doctor --application-readiness <REQUEST_JSON>` implementation. It passively
binds one explicit Application Definition and two to sixteen independent
READINESS-001 workspace declarations, preserves each child result reference,
and reports the exact worst admissible state without invoking Cargo or owner
work.

Application-root requirements, source adapters, owner execution, adoption,
support, production, performance, savings, and successor work remain outside
this implementation and outside Pulse 07 authority.

## Role dispositions

| Role | Disposition | Finding |
|---|---|---|
| Rust Safety Steward | `accept-reviewed-platform-unsafe` | Safe interfaces contain the implementation; the only unsafe block calls Windows `GetFileInformationByHandle` with an open standard-library handle and initialized output only on API success. No broader safety claim follows. |
| Compiler Performance Engineer | `accept-no-performance-claim` | Sequential bounded reads and observations add no benchmark, parallelism, cache, or savings claim. |
| Interop Boundary Auditor | `accept-filesystem-boundary-explicit` | Unix device/inode identity and Windows volume/file identity are platform-specific, link and reparse traversal fails closed, and child READINESS-001 semantics are not flattened. |
| AI Assurance Skeptic | `accept-observed-only` | Typed child evidence and exact precedence remain visible; stale, blocked, incomplete, and unsupported states cannot become success-shaped output. |
| Ecosystem Strategist | `accept-owner-aligned-composition` | Ferris references the owner Application Definition, Cargo manifests, and READINESS-001 declarations without creating a resolver or alternative owner truth. |
| Rust Maintainer | `accept-optional-removable-v1` | One focused core module and one optional existing-`doctor` argument preserve ordinary Cargo, workspace readiness, and legacy doctor behavior; removal changes no owner file. |
| Native Platform Adopter | `accept-windows-evidence-unix-code-boundary` | Windows filesystem identity, hard-link rejection, passive execution, and CLI behavior ran locally; Unix device/inode behavior is cfg-specific and awaits a Unix execution record. |
| Scope Keeper | `accept-bounded-v1` | Only the frozen explicit-file workspace composition was implemented; application-root requirements, adapters, execution, cache, and adopters remain absent. |
| Validation Checker | `accept-targeted-evidence` | Frozen request controls, precedence, exact identities, hard-link aliases, same-byte replacement, unsupported child schemas, privacy, non-execution, and legacy compatibility have targeted tests. |
| Product Value Governor | `continue-within-budget` | The bounded implementation closes the false application-wide readiness gap identified by the enterprise multi-workspace evidence without adding another product layer. |
| Autonomy Supervisor | `stop-after-pulse-07` | The implementation and one corrective review pass are complete; Pulse 07 is exhausted and no successor follows automatically. |

## Implementation basis and files

The implementation is based on
`b048b741c95ef953567f3ea3000b21890a955132`. Product changes are confined to:

- `crates/ferris-core/src/application_readiness.rs`;
- `crates/ferris-core/src/readiness.rs`;
- `crates/ferris-core/src/lib.rs`;
- `crates/ferris-cli/src/entrypoint.rs`; and
- `crates/ferris-cli/tests/cli.rs`.

The shared readiness refactor adds an exact-byte observation entrypoint and a
stale-report helper while preserving standalone READINESS-001 behavior. No
dependency manifest, frozen schema, frozen fixture, owner repository, or
adopter changed.

## Evidence

Compiler and behavioral evidence:

```powershell
cargo test -p ferris-core application_readiness --no-fail-fast
cargo test -p ferris-cli --test cli application_readiness --no-fail-fast
cargo test -p ferris-core readiness --no-fail-fast
```

The Windows runs passed seven focused application-readiness core tests, three
focused CLI integration tests, and twenty-one readiness-filtered core tests plus
the existing asdf readiness evaluation. They demonstrate strict frozen-control
rejection, passive child observation, exact-byte and filesystem-identity stale
handling, hard-link alias rejection, deterministic byte-identical output,
privacy-safe errors, no Cargo launch, typed blocked output, and compatibility
with standalone readiness and legacy doctor.

The frozen request and report fixtures validate against their schemas. Changed
Markdown links and code fences, targeted formatting, and `git diff --check`
also pass.

## Independent review

The first review found four material defects:

1. unsupported child schemas reached standalone report-bearing behavior;
2. a declaration change between application binding and child loading could
   become an internal error;
3. hard-linked manifests could count as separate workspace roots; and
4. same-byte replacement or replacement links could escape stale detection.

The implementation now validates the exact child schema before observation,
observes the exact accepted bytes, revalidates type, identity, and bytes,
deduplicates manifest filesystem identity, and turns later replacement into
stale evidence. Focused regression tests cover each correction. The authorized
final independent review found no remaining material issue.

## Remaining gates and limits

- Execute cfg-specific Unix identity and link behavior before making a
  cross-platform validation claim.
- Application-root requirements require a separately versioned contract.
- Presence and composition do not prove that owner validation succeeds.
- Source adapters, version checks, installation, repair, owner execution,
  network, services, credentials, and capacity probes remain absent.
- No adopter, CI-replacement, support, production, performance, or savings
  claim follows.

## Final authority

APP-READINESS-001 is Implemented V1 for the bounded explicit-file composition.
Pulse 07 is complete and its implementation authority is exhausted. Removal is
the deletion of the optional argument path and focused application-composition
module; Cargo, application definitions, requirements declarations, owner
commands, standalone readiness, and legacy doctor remain unchanged. No
successor work is authorized.
