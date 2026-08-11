# Held-Out Fixture Manifest

Manifest revision: 7
State: Frozen
Oracle: Withheld under `ORACLE_CUSTODY.md`

| ID | Frozen distinguishing composition | Platforms | Required surfaces |
|---|---|---|---|
| FHIF-001 | Multi-workspace private edit plus undeclared generated/native input and owner freshness mismatch | Windows, Unix | `affected`, `plan`, `explain` |
| FHIF-002 | Breaking Typebook operation with compiling Rust projection, profile renewal, and optional capability loss | Windows, Unix | `plan`, `check`, `test` |
| FHIF-003 | Typed-ref collision, concurrent channel promotion, ref movement, and clock uncertainty | Windows | `query`, `graph`, `explain` |
| FHIF-004 | Out-of-population AI narrowing proposal with one seeded false omission and periodic full reference | Windows, Unix | `plan`, `test`, MCP planning |
| FHIF-005 | Approved multi-step action with mid-operation revocation, non-interruptible owner work, and concurrent target generation drift | Unix | `run`, action MCP |
| FHIF-006 | MCP indirect prompt injection, post-discovery schema drift, paginated partial result, and connector revocation | Windows | MCP `query`, `plan`, action request |
| FHIF-007 | Unsupported native ABI with high-volume diagnostics, accessible localized output, and no safe automatic workaround | Windows, Unix | `doctor`, `explain`, `check` |
| FHIF-008 | Query Forest store loss with verified old root, incomplete packet reconstruction, stale refs, and partial deletion | Windows | `query`, `graph`, recovery view |
| FHIF-009 | Explicit offline local plan followed by evidence-service outage and uncertain approval expiry | Windows, Unix | `doctor`, `plan`, `run` |
| FHIF-010 | Complete Ferris and connector removal with retained audit, one residual hook, credential revocation, and owner-native verification | Windows, Unix | `doctor`, removal packet |
| FHIF-011 | Million-edge projection with a mandatory unknown beyond the first page and cross-tenant confusable identifiers | Unix | `graph`, `query`, `explain` |
| FHIF-012 | Full nine-command CLI, Cargo, and MCP semantic parity over identical explicit inputs and one default-scope negative control | Windows, Unix | all public commands |
| FHIF-013 | Portable passive-doctor identity, strict Cargo evidence, privacy, bounded manifest/process streams, and passive controls | Windows | `doctor` |
| FHIF-014 | Independent Cargo classification, identity, bounded-failure, privacy, passive-control, and post-read-failure matrix | Windows | `doctor` |
| FHIF-015 | Complete typed doctor identity and binding under bounded machine-output framing | Windows | `doctor` |
| FHIF-016 | Independent bounded machine framing, stream placement, typed identity, privacy, and limit controls | Windows | `doctor` |
| FHIF-017 | Independent typed bounded-failure evidence matrix with separately sealed executable harness | Windows | `doctor` |
| FHIF-018 | Preflighted independent typed bounded-failure matrix and machine-interface controls | Windows | `doctor` |

FHIF-013 failed its first blind score and is reclassified as development
evidence. It MUST NOT be rescored. A replacement fixture requires a new ID and
independently frozen oracle.

FHIF-014 independently replaced FHIF-013, failed its first blind score, and is
also reclassified as development evidence. It MUST NOT be rescored.

FHIF-015 independently replaced FHIF-014, failed its first blind score, and is
also reclassified as development evidence. It MUST NOT be rescored.

FHIF-016 independently replaced FHIF-015, failed its first blind score, and is
also reclassified as development evidence. It MUST NOT be rescored.

FHIF-017 independently replaced FHIF-016 but its frozen harness did not
execute Ferris. It is quarantined as invalid development evidence, provides
no implementation score, and MUST NOT be rerun.

FHIF-018 independently replaced FHIF-017, reached Ferris, but produced only
one invalid/2 public result before failing its frozen oracle. It is
quarantined as development evidence and MUST NOT be rerun or rescored.

## Frozen dimensions for every fixture

Each executable binding MUST fix:

- application and tenant identities;
- repository and revision identities;
- Cargo manifests, lockfiles, workspaces, features, targets, and profiles;
- contracts, native inputs, providers, connectors, and capability snapshots;
- toolchain, platform, filesystem, environment, and clock evidence;
- policy, approval, trust, revocation, retention, and data classification;
- selected and full-reference commands;
- positive, negative, failure, and unsupported controls;
- expected canonical schema versions;
- resource and output budgets; and
- cleanup, rollback, recovery, and removal boundaries.

## Reclassification rule

If an expected outcome, hidden seed, or oracle predicate is disclosed to an
implementation or prompt before scoring, that fixture becomes development
evidence. It MUST receive a replacement ID and independently frozen input.
