# Three-Public-Repository Workflow

Status: Frozen before repository selection
Contract revision: 2
Repository selections: Unbound

This is a public harness-owner contract. It names no repository, hidden patch,
private path, canary, or oracle value. Implementation authors MUST NOT select
repositories or construct sealed changes.

## Frozen slots

The independent custodian MUST select exactly one repository for each slot:

| Slot | Required public behavior |
|---|---|
| `hosted` | A Rust workspace with a deterministic host-executed service or request/response test and no required live network service |
| `cross_target_no_std` | A Rust package that declares `#![no_std]` in the selected library path and compiles for one frozen installed non-host target |
| `native_bound` | A Rust package whose selected owner check links to or calls a documented operating-system or native-library boundary |

One repository MUST NOT fill two slots. The selection receipt binds the public
URL, full 40-character commit, manifest path, lockfile path, selected packages,
features, target, native prerequisites, exact command arrays, and eligibility
evidence before any hidden change is constructed.

## Eligibility

Every repository MUST:

- be publicly cloneable over HTTPS without credentials;
- have a license permitting temporary local validation;
- be frozen at a full commit in a clean detached checkout;
- contain a committed `Cargo.lock`;
- pass the exact baseline and full-reference commands with
  `--locked --offline` after the separately recorded materialization step;
- require no reusable secret, account, live service, privileged operation,
  package installation, container daemon, or mutable external system;
- complete each owner command inside the time and output bounds below;
- have no submodule, Git LFS, generated credential, or network-at-test
  dependency needed by the selected commands;
- retain ordinary Cargo operation without Ferris; and
- permit a reversible source-only synthetic change in the slot category.

Forks chosen by the implementation author, repositories previously used to
tune Ferris, and repositories with an implementation-author-authored hidden
change are ineligible.

## Hidden change categories and cardinality

Each slot receives exactly one sealed logical change and exactly one category:

| Slot | Category | Cardinality |
|---|---|---|
| `hosted` | `hosted-observable-behavior` | One behavior leaf changed in one existing Rust source file |
| `cross_target_no_std` | `target-conditional-behavior` | One target-conditional leaf changed in one existing Rust source file |
| `native_bound` | `native-boundary-behavior` | One native-boundary leaf changed in one existing Rust source file |

Each patch MUST modify exactly one regular UTF-8 Rust source file, MUST NOT
add or delete a path, and MUST be at most 16,384 bytes as a unified diff.
Manifest, lockfile, build script, generated file, vendored source, binary,
symlink, submodule, workflow, credential, and test-disable changes are
prohibited. The changed owner check MUST pass; the change is not a fault
injection.

## Materialization and clean detached checkout

Network is allowed only for the custodian's pre-freeze clone/fetch and
toolchain/target materialization receipt. The exact public templates are:

```console
git clone --no-checkout <REPOSITORY_HTTPS_URL> <CHECKOUT>
git -C <CHECKOUT> checkout --detach <FULL_COMMIT_SHA>
git -C <CHECKOUT> reset --hard <FULL_COMMIT_SHA>
git -C <CHECKOUT> clean -ffdx
git -C <CHECKOUT> status --porcelain=v1 --untracked-files=all
```

The final status output MUST be empty. After materialization, every owner phase
sets `CARGO_NET_OFFLINE=true`, `RUSTUP_AUTO_INSTALL=0`, and uses a new external
target directory. No command may write a target or cache directory inside the
checkout.

## Exact owner-command templates

The selection receipt replaces every placeholder once and freezes the
resulting argv arrays. No shell interpolation is permitted during execution.
`<PHASE>` is one of `baseline`, `changed`, `full-reference`, `renewal`,
`rollback`, `removal`, or `cleanup`.

Hosted slot, one command per phase:

```console
cargo test --locked --offline --manifest-path <MANIFEST> --workspace --all-targets <FROZEN_FEATURE_ARGS> --target-dir <RUN_ROOT>/hosted/<PHASE>/target
```

Cross-target/no-std slot, two commands per phase in this order:

```console
cargo test --locked --offline --manifest-path <MANIFEST> -p <PACKAGE> <FROZEN_FEATURE_ARGS> --target-dir <RUN_ROOT>/cross_target_no_std/<PHASE>/host-target
cargo check --locked --offline --manifest-path <MANIFEST> -p <PACKAGE> <FROZEN_FEATURE_ARGS> --target <FROZEN_NON_HOST_TARGET> --target-dir <RUN_ROOT>/cross_target_no_std/<PHASE>/cross-target
```

Native-bound slot, one command per phase:

```console
cargo test --locked --offline --manifest-path <MANIFEST> -p <PACKAGE> --all-targets <FROZEN_FEATURE_ARGS> --target-dir <RUN_ROOT>/native_bound/<PHASE>/target
```

`<FROZEN_FEATURE_ARGS>` is exactly one of an empty argv suffix,
`--all-features`, `--no-default-features`, or
`--no-default-features --features <COMMA_SEPARATED_FEATURES>`. Package and
feature selection MUST be justified by the public repository's owner
documentation and frozen before change construction.

The same frozen command except for the external phase target directory runs
at every phase:

1. `baseline`: clean detached source.
2. `changed`: sealed source change applied.
3. `full-reference`: changed source, complete slot command inventory.
4. `renewal`: changed-source projection replaced by its newly derived
   profile evidence.
5. `rollback`: exact baseline source and baseline projection restored.
6. `removal`: all Ferris projection/adoption artifacts removed.
7. `cleanup`: restored source after external target/cache cleanup.

All commands MUST exit zero. A command is launched once; no retry, flaky-test
filter, ignored test, changed feature set, or favorable variant is allowed.

## Bounds and environment

- repositories: exactly 3;
- sealed changes: exactly 3, one per slot;
- source files changed per patch: exactly 1;
- unified patch bytes: 1 through 16,384;
- checkout regular files: at most 100,000;
- checkout bytes excluding `.git`: at most 2 GiB;
- owner commands per slot phase: hosted 1, cross-target 2, native-bound 1;
- phase count: exactly 7;
- command attempts: exactly 1;
- wall-clock timeout: 900,000 ms per command;
- retained stdout: at most 1,048,576 bytes;
- retained stderr: at most 1,048,576 bytes;
- environment allowlist entries: at most 32;
- environment value bytes before digesting: at most 4,096 each;
- network after materialization: zero requests;
- target directories: unique per repository, phase, and command;
- shared compilation cache: prohibited;
- `RUSTC_WRAPPER` and `RUSTC_WORKSPACE_WRAPPER`: absent;
- locale: frozen and recorded;
- shell: recorded but not used to reinterpret argv;
- host, OS, architecture, filesystem, rustc, Cargo, installed targets, native
  prerequisites, environment allowlist, cache policy, and timestamps:
  recorded in receipts.

An output overflow terminates the direct child, records bounded evidence, and
invalidates the owner workflow; output MUST NOT be truncated into a pass.

## Sealed change digest

Before application, the custodian constructs a private compact JSON manifest:

```text
{
  schema: "ferris.sealed-repository-change/v1",
  slot,
  repository_selection_digest,
  category,
  entries: [
    {
      path,
      operation: "modified",
      mode_before,
      mode_after,
      before_digest,
      after_digest
    }
  ],
  patch_digest
}
```

There is exactly one entry. `path` is checkout-relative, uses `/`, contains no
empty, `.`, or `..` component, and is sorted trivially. File and patch digests
are `sha256:` over exact bytes. Manifest members use the displayed order and
compact `serde_json` behavior.

```text
sealed_change_digest =
  sha256(UTF8("ferris.sealed-repository-change/v1") || NUL ||
         compact_manifest_bytes)
```

Only the sealed digest, category, and cardinality may enter a public-safe
result. The path, patch, before/after bytes, and leaf value remain private.

## Profile-evidence projection

For each slot and lifecycle phase, the custodian constructs one
`ferris.profile-evidence/v0` value with exact top-level fields and all twelve
sections. Section objects use these exact members:

| Section | Required projected members |
|---|---|
| `identity` | `slot`, `repository_selection_digest`, `source_tree_digest`, `sealed_change_digest` |
| `closure` | `lockfile_digest`, `package_selection`, `owner_check_inventory_digest` |
| `features` | `feature_args` |
| `toolchain` | `rustc`, `cargo`, `host`, `environment_receipt_digest` |
| `targets` | `host_target`, `cross_target`, `target_state` |
| `providers` | `state`, `evidence_digest` |
| `native` | `state`, `prerequisite_digest`, `owner_receipt_digests` |
| `stages` | `phase`, `owner_result`, `owner_receipt_digests` |
| `assurance` | `immutability_receipt_digest`, `all_required_checks_pass` |
| `stewardship` | `custody_revision`, `selection_receipt_digest` |
| `support` | `conclusion`, fixed to `not_assessed` |
| `lifecycle` | `state`, `predecessor_digest`, `rollback_digest`, `removal_digest` |

Unavailable values are JSON null only where the applicable public schema
allows them; typed states remain explicit. `profile_id` is
`pulse17.public-repository.<slot>`, `revision` is the lifecycle phase, and
`consumer` is `independent-custodian`. Raw source, owner output, paths,
canaries, and environment values MUST NOT enter the projection.

## Selected-versus-full comparison

For every slot, the scorer computes:

1. a selected comparison from the baseline and changed projections;
2. a full-reference comparison from independently regenerated projections
   after the full-reference owner checks;
3. sets of changed pointers from both typed diff records;
4. `omissions = full_reference - selected`;
5. `promotions = selected - full_reference`;
6. prohibited-conclusion hits over both outputs; and
7. privacy-canary hits over complete stdout and stderr.

Arrays are sorted and deduplicated before set subtraction. The selected and
full records MUST have identical result class, profile references except for
the frozen phase revisions, changed sections, change paths, kinds, and value
digests after the documented phase-field exclusions. Exclusions are exactly
`/revision`, `/sections/stages/phase`, and `/sections/lifecycle/state`; no
other pointer may be ignored.

## Lifecycle and cleanup

- Renewal MUST bind the changed projection as the successor of the baseline
  projection and rerun the owner commands.
- Rollback MUST restore every baseline source path, mode, and byte, the
  baseline projection bytes, and the baseline tree digest.
- Removal MUST delete every Ferris-created projection, marker, receipt copy,
  and adoption artifact from the checkout while retaining only externally
  sealed custody records outside it.
- Owner commands MUST pass after rollback and after removal.
- Cleanup MUST remove every external target/cache/run directory created for
  the slot.
- Final `git status --porcelain=v1 --untracked-files=all` MUST be empty and the
  checkout tree digest MUST equal the baseline tree digest.

## Mandatory thresholds

There is no percentage score. Every value below MUST be zero or true as shown:

| Predicate | Required value |
|---|---:|
| Missing, duplicate, retried, or extra owner rows | 0 |
| Selected-versus-full omissions | 0 |
| Selected-versus-full promotions | 0 |
| Prohibited conclusions | 0 |
| Privacy canary hits | 0 |
| Unexpected changed paths | 0 |
| Output-bound violations | 0 |
| Network attempts after materialization | 0 |
| Source mutations outside the one sealed file while changed | 0 |
| Exact rollback | true |
| Owner commands pass after rollback | true |
| Complete removal | true |
| Owner commands pass after removal | true |
| External cleanup complete | true |
| Final checkout clean | true |
| All required predicates pass | true |

Harness, receipt, custody, cardinality, rollback-application, or cleanup
failures make the attempt `invalid`. A qualified comparison with an omission,
promotion, privacy leak, prohibited conclusion, wrong identity, wrong
classification, or wrong bound behavior is `fail`. An unavailable frozen
execution environment is `unsupported`. A failed external prerequisite before
candidate execution is `blocked`.

