# EXP-01: Cross-Crate Interface Matrix

Date: 2026-08-08
Question: PERF-Q20
Status: Complete

## Purpose

Determine which upstream source edits currently rebuild downstream crates,
which edits change emitted metadata, and which edit classes are credible
Relink-Don't-Rebuild candidates.

The experiment separates three decisions:

1. whether Cargo rebuilds the edited upstream crate;
2. whether unchanged downstream crates are recompiled;
3. whether the final executable must be relinked because linkable code changed.

## Fixture

The disposable workspace used this dependency chain:

```text
base -> mid -> app
```

`base` exposes ordinary, inline, generic, constant, macro, and opaque-layout
surfaces. `mid` consumes them. `app` consumes `mid` and prints a deterministic
result.

Each scenario used a new workspace and three exploratory repetitions:

1. build `app`;
2. run and hash the initial artifacts;
3. edit only `base/src/lib.rs`;
4. rebuild and run `app`;
5. record Cargo `compiler-artifact` messages and SHA-256 hashes;
6. clean only `base`, rebuild the identical edited source, and verify metadata
   reproducibility.

Command:

```powershell
cargo +nightly build -Z checksum-freshness -p app `
  --message-format=json-render-diagnostics
```

The matrix ran once with development incremental compilation disabled and once
with it enabled. Both profiles used `debug = 0`.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS storage;
- rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- Cargo `1.99.0-nightly (c79e8f894 2026-08-04)`;
- LLVM `23.1.0`;
- host `x86_64-pc-windows-msvc`.

The installed nightly exposed Cargo checksum freshness but no rustc
API-fingerprint or RDR option.

## Results

Every result below was consistent across all three repetitions. Times are
exploratory medians, not promoted optimization claims.

| Edit to `base` | Incremental off | Incremental on | Rebuilt packages | Base and mid `.rmeta` equal to before | Runtime output changed |
|---|---:|---:|---|---|---|
| Identical content rewrite | 122.90 ms | 142.26 ms | None | Yes | No |
| Comment | 721.00 ms | 1,169.48 ms | `base`, `mid`, `app` | No | No |
| Private function body | 668.27 ms | 1,440.95 ms | `base`, `mid`, `app` | No | Yes |
| Public non-inline function body | 729.28 ms | 1,559.59 ms | `base`, `mid`, `app` | No | Yes |
| Public inline function body | 712.24 ms | 1,593.45 ms | `base`, `mid`, `app` | No | Yes |
| Generic function body | 661.41 ms | 1,570.65 ms | `base`, `mid`, `app` | No | Yes |
| Public constant value | 700.92 ms | 1,694.81 ms | `base`, `mid`, `app` | No | Yes |
| Exported macro body | 660.90 ms | 1,680.46 ms | `base`, `mid`, `app` | No | Yes |
| Opaque public type private-field layout | 755.82 ms | 1,670.83 ms | `base`, `mid`, `app` | No | Yes |
| Add private item | 750.54 ms | 1,668.81 ms | `base`, `mid`, `app` | No | No |
| Reorder private items | 643.75 ms | 1,733.12 ms | `base`, `mid`, `app` | No | No |
| Add public item | 693.06 ms | 1,664.44 ms | `base`, `mid`, `app` | No | No |
| Equivalent public type spelling | 724.86 ms | 1,531.46 ms | `base`, `mid`, `app` | No | No |

For every actual content edit:

- Cargo rebuilt all three packages;
- `base` and `mid` `.rmeta` hashes changed;
- `base` and `mid` `.rlib` hashes changed;
- the `app` executable hash changed;
- a forced rebuild of the identical edited source reproduced the same
  `base` `.rmeta` hash.

The forced repeat establishes that the metadata differences were deterministic
effects of the edit, not random output noise.

## Interpretation

Checksum freshness solves one narrower problem: a timestamp-only or
same-content rewrite no longer dirties the package. It does not classify a
real source-content edit by its cross-crate semantic effect.

The current metadata artifact is not an RDR interface digest. It changed for
comments, private item insertion and reordering, private bodies, and an
equivalent public type spelling. Rustc's current crate hash includes all HIR
owners, upstream crate hashes, source identities, visibility state, compiler
options, and, for incremental builds, owner spans. That identity is valuable
for compiler correctness and artifact compatibility, but intentionally broader
than a public-interface cutoff.

Private and public non-inline body edits are the leading RDR eligibility
classes. They change linkable implementation code but need not change the
contract used to compile an unchanged dependent crate. Downstream compilation
can potentially be pruned while relinking remains required.

Inline bodies, generic bodies, constants, exported macros, and layouts can
cross crate boundaries through metadata, monomorphization, evaluation,
expansion, ABI, size, alignment, drop behavior, or optimization. They cannot
be classified from visibility or syntax alone.

Private item insertion and reordering expose a separate identity problem. Even
if the semantic interface is unchanged, retained downstream artifacts can
refer to compiler definition or symbol identities that move. Stable identity
is a prerequisite for safe artifact retention.

## Limitations

- This is a small synthetic graph, not a production workspace distribution.
- Three repetitions support behavior discovery, not a promoted latency claim.
- The experiment observes current Cargo and rustc outputs; it does not
  implement a public-interface hash or force unsafe reuse.
- Byte inequality does not identify which metadata field changed.
- Runtime equality is only one behavioral check and cannot prove reuse safety.
- `debug = 0` avoids debug-information noise but does not cover every profile,
  target, LTO, proc-macro, build-script, native-link, or dynamic-linking case.

## Result

Current nightly behavior has no downstream early cutoff for actual upstream
source edits in this fixture. RDR requires a new, conservative public-interface
decision plus stable retained-artifact identities; it is not a change to Cargo
mtime freshness and it is not equivalent to comparing current `.rmeta` bytes.
