# CI Cache Topology and Duplicate Rust Work

Date: 2026-08-08
Question: PERF-Q06
Status: Complete
Decision: define a cache diagnosis surface that separates Cargo compatibility,
CI transport keys, immutable-entry lifecycle, trust, and retention; prefer
dependency-only trusted-producer caches and same-job reuse over broad target
archives.

## Executive conclusion

A CI cache has at least three independent identities:

1. Cargo decides whether restored artifact state is compatible and fresh.
2. The CI service decides which archive key, version, branch scope, and writer
   can be restored.
3. Retention policy decides whether that entry still exists.

Collapsing these into "cache hit" hides both duplicate work and correctness
risk.

The controlled METIS-CORE experiment measured a 32.654-second project-cold
median. A local full-target restore plus Cargo validation took 7.046 seconds,
while a dependency-only restore took 12.963 seconds. The dependency-only
archive was 94.1 MB rather than 121.6 MB and correctly rebuilt ten workspace
artifacts while reusing 89 registry artifacts. These local results exclude
network and producer upload.

The public METIS-CORE workflow supplied real transport evidence. Exact
dependency-cache hits restored:

- 101 MB in a seven-second macOS cache step before a nine-second test step;
- 184 MB in a four-second Ubuntu cache step before a twelve-second test step;
- 105 MB in a ten-second Windows cache step before a thirty-two-second test
  step.

The root recompiled on every platform by design. The same cache keys and byte
sizes survived source commits because `Swatinem/rust-cache` excludes workspace
artifacts and source versions from its dependency key.

Two topology failures were more important than hit rate:

- Prusti restored 19 MB and then skipped its only consumer.
- Kani spent more than 76 minutes in installation and never reached the cache
  step placed after installation.

CI cache entries are immutable. `rust-cache` also skips its save path after an
exact hit. Therefore a key that omits a matrix target, feature set, profile, or
cache-schema version can freeze an incomplete payload. Parallel matrix jobs
with the same job ID and key compete to become the one payload that future
runs restore.

FERRIUM should build no cache backend. Its opportunity is a CI cache topology
diagnostic that reports:

- producer and consumer jobs;
- exact versus fallback key matches;
- key dimensions and omitted command axes;
- payload composition and observed Cargo freshness;
- restore, validation, compile, pack, and upload cost;
- first-writer and exact-hit update behavior;
- branch trust and write scope;
- retention, eviction, and quota pressure;
- whether same-job command ordering would preserve more work than transport.

No issue, comment, branch, or pull request was created during this research.

## Decision supported

This research determines:

- which CI key dimensions are correctness boundaries;
- when fallback keys preserve useful dependency work;
- when cache transport costs erase compile savings;
- how job, matrix, command, profile, and target boundaries create duplicate
  work;
- the first evidence-based cache recommendations FERRIUM can make.

It does not authorize a hosted cache, remote artifact trust protocol, workflow
rewrite, or upstream filing.

## Research question

Which CI job, branch, profile, target, runner, and cache-key boundaries discard
compatible Rust work or retain excessive incompatible artifacts?

## Starting and competing hypotheses

The starting hypothesis was that CI keys are often either too broad for
correctness or too narrow for reuse.

The investigation tested:

1. An exact CI cache hit means the Cargo build is warm.
2. Hashing the lockfile is a complete Rust artifact key.
3. Broader fallback keys only improve hit rate.
4. One cache per job is always the safest topology.
5. A cache should be saved whenever a workflow succeeds.
6. Full target archives always beat dependency-only archives.

The evidence rejected all six as general rules.

## Evidence reviewed

### Local evidence

- `docs/research/2026-08-07-cargo-build-unit-identity.md`
- `docs/research/2026-08-08-cargo-build-unit-multiplication.md`
- `docs/research/2026-08-08-cross-workspace-artifact-reuse.md`
- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/research/perf-q06-ci-cache-topology/results/EXP-01-cache-transport-and-topology.md`

### CI and cache sources

- [GitHub dependency caching reference](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)
- [`actions/cache` README](https://github.com/actions/cache/blob/main/README.md)
- [`actions/cache` caching strategies](https://github.com/actions/cache/blob/main/caching-strategies.md)
- [`actions/cache` tips and workarounds](https://github.com/actions/cache/blob/main/tips-and-workarounds.md)
- [`Swatinem/rust-cache` README](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/README.md)
- [`rust-cache` key construction](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/config.ts#L72-L133)
- [`rust-cache` dependency lock hashing](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/config.ts#L180-L263)
- [`rust-cache` restore and fallback handling](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/restore.ts#L20-L66)
- [`rust-cache` exact-hit save behavior](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/save.ts#L20-L57)
- [`rust-cache` dependency selection](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/workspace.ts#L6-L40)
- [Cargo issue #8603](https://github.com/rust-lang/cargo/issues/8603)

The `rust-cache` source revision was `6323deb102c322ba6fcbdcafc7e3dddab59af2b6`
from 2026-08-06. The reviewed METIS-CORE run resolved `rust-cache@v2` to
`e18b497796c12c097a38f9edb9d0641fb99eee32`.

## CI cache model

FERRIUM distinguishes:

| Layer | Question |
| --- | --- |
| Cargo compatibility | Can this producer artifact satisfy this consumer unit? |
| Transport key | Which archive does the CI service select? |
| Cache version | Can the current paths and compression implementation read it? |
| Branch scope | Is the producer entry visible to this branch, PR, or tag? |
| Writer trust | Which workflow events may publish or replace candidate state? |
| Payload | Which registry, Git, target, profile, and tool artifacts are included? |
| Entry lifecycle | Is the selected immutable payload complete for current consumers? |
| Retention | Has idle eviction, LRU pressure, or manual cleanup removed it? |
| Economics | Does restored compile work exceed lookup, download, extraction, validation, and producer cost? |

The consumer-side value is:

```text
saved latency =
  cold compile
  - restore
  - verification
  - compile after restore
```

The cache is portfolio-positive only when future saved latency also amortizes:

```text
producer pack + upload + storage and eviction cost
```

## Key coverage model

An evidence record checks whether the key or designated producer covers:

| Axis | Why it matters |
| --- | --- |
| Cache schema version | Forces refresh when workflow coverage or cleanup semantics change |
| rustc release, host, commit | Compiler artifact compatibility |
| Runner OS and architecture | Native object and executable compatibility |
| Explicit target triple | Cross-target outputs and host/target split |
| Effective profile | Optimization, debug, panic, LTO, and codegen differences |
| Feature set | Dependency unit and semantic differences |
| Relevant environment and flags | Compiler, linker, native tool, and build-script inputs |
| Dependency lock state | Registry and Git package identities |
| Job or producer role | Prevents accidental incompatible first-writer payloads |
| Branch and trust scope | Controls visibility and publication authority |

Not every axis must create a separate archive. A designated producer may
intentionally populate several compatible variants before one save. The report
must show that choice rather than assuming it.

## Findings

### FERRIUM-78: Cargo compatibility, CI key matching, and retention are
different systems

**Sources**

- [GitHub dependency caching reference](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)
- [Cargo artifact identity findings](2026-08-07-cargo-build-unit-identity.md)
- [Cross-workspace provenance findings](2026-08-08-cross-workspace-artifact-reuse.md)

**Observed constraint**

GitHub reports an exact hit when a key and cache version match in an accessible
branch scope. It does not inspect Cargo units. Cargo then independently decides
which restored artifacts are fresh. GitHub may later evict the entry after
seven idle days or under repository quota pressure.

**Implication**

FERRIUM must not use `cache-hit=true` as a build-reuse metric. It needs CI
match state, Cargo artifact freshness, and retained-entry state as separate
fields.

**Confidence:** high.

### FERRIUM-79: the common Rust cache key covers toolchain and dependencies but
not every command axis

**Sources**

- [`rust-cache` key construction](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/config.ts#L72-L133)
- [`rust-cache` lock hashing](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/config.ts#L180-L263)
- [`rust-cache` README](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/README.md)

**Observed constraint**

The default key includes job ID, runner OS and architecture, exact installed
rustc identities, selected Cargo/compiler/native environment variables, Cargo
manifests, and immutable lockfile packages.

It does not automatically include a target triple, profile, feature set, or
command when those values are supplied only as command arguments or uncaptured
matrix variables.

**Implication**

Target, profile, feature, and tool matrices need either:

- an explicit additional key;
- captured environment variables; or
- one designated producer that intentionally creates all required variants.

Blindly adding every axis can also over-partition compatible work, so FERRIUM
should report omitted and redundant key dimensions rather than emit one fixed
key template.

**Confidence:** high.

### FERRIUM-80: immutable exact hits can freeze an incomplete matrix payload

**Sources**

- [GitHub cache immutability](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching#cache-action-usage)
- [`rust-cache` exact-hit save behavior](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/save.ts#L20-L31)
- [`rust-cache` cache-state check](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/config.ts#L355-L364)

**Observed constraint**

GitHub cache entries cannot be changed. `rust-cache` returns from its save path
when the restored key is already exact.

Matrix variants share `GITHUB_JOB`. If target, profile, or features are not in
the remaining key inputs, parallel jobs request the same immutable key. The
first successful writer determines the payload. Later exact-hit runs can build
missing variants locally but do not update that cache.

**Implication**

Every cache design needs a schema/version component and an explicit writer
topology. For additive matrices, prefer one trusted producer followed by
restore-only consumers. Otherwise key the incompatible variants separately.

**Confidence:** high from the documented lifecycle and source logic.

### FERRIUM-81: dependency-only payloads preserve reuse across source commits

**Sources**

- [`rust-cache` dependency selection](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/workspace.ts#L6-L40)
- [`rust-cache` save cleanup](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/save.ts#L33-L57)
- Public METIS-CORE runs
  [`25630776288`](https://github.com/giodl73-repo/METIS-CORE/actions/runs/25630776288)
  and
  [`25951794528`](https://github.com/giodl73-repo/METIS-CORE/actions/runs/25951794528)

**Observed behavior**

METIS-CORE used the same exact per-platform cache keys and byte sizes across
different source commits. Every test job still compiled `metis-core`.

The action selects packages outside the workspace root by default, removes
non-dependency target artifacts before save, and disables incremental
compilation.

**Implication**

Dependency-only is the safe default for changing branches and pull requests.
Workspace artifacts should be recovered by same-job command ordering unless a
separate measurement proves their archive value and source identity.

**Confidence:** high.

### FERRIUM-82: restore cost is large enough to require break-even evidence

**Source**

- Experiment:
  `docs/research/perf-q06-ci-cache-topology/results/EXP-01-cache-transport-and-topology.md`

**Observed behavior**

The local METIS-CORE medians were:

- project-cold: 32.654 seconds;
- full target restore plus Cargo: 7.046 seconds;
- dependency-only restore plus Cargo: 12.963 seconds.

The public exact-hit cache step took four to ten seconds for 101 to 184 MB,
depending on platform. On macOS, the seven-second cache step approached the
nine-second first Cargo workload.

**Implication**

FERRIUM recommendations require payload bytes, restore duration, compile after
restore, and expected hit count. Small or infrequent jobs should skip target
caching when restore cost approaches the work it saves.

**Confidence:** high for the measured fixture and run; transfer results do not
generalize to all networks or runners.

### FERRIUM-83: fallback keys salvage unchanged dependencies and retain stale
variants

**Sources**

- [GitHub restore-key matching](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching#cache-key-matching)
- [`rust-cache` fallback restore](https://github.com/Swatinem/rust-cache/blob/6323deb102c322ba6fcbdcafc7e3dddab59af2b6/src/restore.ts#L35-L62)
- Experiment:
  `docs/research/perf-q06-ci-cache-topology/results/EXP-01-cache-transport-and-topology.md`

**Observed behavior**

After restoring a dependency target built with `itoa 1.0.15` and
`ryu 1.0.20`, the consumer changed only `itoa` to 1.0.14. Cargo reused `ryu`,
rebuilt `itoa` and the root, and retained both `itoa` versions during the run.
Target bytes grew by 40.0%.

`rust-cache` uses a restore key that excludes the final dependency-lock hash
and saves a cleaned payload under the new full key after a successful partial
match.

**Implication**

Fallback is useful for dependency caches but needs post-build cleanup and a
new immutable key. Broad fallback must not include workspace path artifacts.

**Confidence:** high for the controlled fixture and source behavior.

### FERRIUM-84: cache placement can miss the dominant job cost

**Source**

- METIS-CORE run
  [`25951794528`](https://github.com/giodl73-repo/METIS-CORE/actions/runs/25951794528)

**Observed behavior**

The Prusti job restored about 19 MB and skipped the Prusti command. The Kani
job spent over 76 minutes in installation, was cancelled, and never reached
the cache step placed after installation.

**Implication**

A cache topology report must include the job graph and uncached setup phases.
Moving or resizing a Cargo cache cannot improve a dominant tool installation,
runner queue, checkout, or unavailable-service cost.

**Confidence:** high for the observed run.

### FERRIUM-85: job boundaries can discard compatible work that keys cannot
recover

**Sources**

- [Cargo build-unit multiplication findings](2026-08-08-cargo-build-unit-multiplication.md)
- Experiment:
  `docs/research/perf-q06-ci-cache-topology/results/EXP-01-cache-transport-and-topology.md`

**Observed behavior**

PARLOR's build-then-test sequence reused five ordinary library artifacts.
Check-then-test, all-target-check-then-test, check-then-Clippy, and
dev-then-release reused none of the observed consumer artifacts.

`rust-cache` keys by job ID by default, so separate jobs do not share even
compatible dependency payloads unless configured. Dependency-only caches also
intentionally omit the five local workspace libraries.

**Implication**

Co-locate commands with proven workspace-artifact compatibility. Use shared
dependency caches only for units that Cargo can actually reuse. Cache topology
must not be used to justify combining semantically distinct validation.

**Confidence:** high for the measured command sequences.

### FERRIUM-86: branch scope, trust, and retention determine effective hit rate

**Sources**

- [GitHub cache access restrictions](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching#restrictions-for-accessing-a-cache)
- [GitHub cache security guidance](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching#best-practices-for-using-caches-securely)
- [GitHub usage limits and eviction](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching#usage-limits-and-eviction-policy)
- [`actions/cache` PR cleanup guidance](https://github.com/actions/cache/blob/main/tips-and-workarounds.md#force-deletion-of-caches-overriding-default-cache-eviction-policy)

**Observed constraint**

GitHub cache entries are branch-scoped. Pull-request merge-ref caches cannot be
reused by the base branch or other pull requests but still consume quota until
cleanup. Default limits are 10 GB and seven idle days. Cache contents are not
signed or verified, and low-trust workflows receive restricted write access.

The METIS-CORE cache API returned no entries almost twelve weeks after the last
observed access.

**Implication**

Use trusted default-branch producers and restore-only low-trust consumers.
Avoid routine PR writes when merge-ref entries have no future consumer. Record
eviction and miss reason instead of treating absence as an identity failure.

Cryptographic remote producer trust remains PERF-Q30.

**Confidence:** high.

### FERRIUM-87: FERRIUM's CI opportunity is a topology and economics
diagnostic

**Sources**

- Findings FERRIUM-78 through FERRIUM-86
- [Cargo cache corruption issue #8603](https://github.com/rust-lang/cargo/issues/8603)

**Observed constraint**

GitHub and rust-cache already provide transport, key construction, dependency
cleanup, branch scope, and APIs. Cargo owns artifact compatibility. Historical
CI archive corruption also shows that a transport hit is not integrity proof.

The missing surface is an explanation connecting:

- workflow graph and cache placement;
- key axes and matrix variables;
- exact, fallback, and absent entries;
- archive contents and Cargo fresh/dirty artifacts;
- restore and producer costs;
- writer trust, immutability, and retention.

**Implication**

FERRIUM should diagnose existing cache systems rather than create another one.
The output should produce evidence-backed key and job-boundary recommendations
with explicit net-benefit and correctness limits.

**Confidence:** high for the product boundary; broader consumer demand still
requires portfolio census work.

## Recommendations

### Adopt now

- Record cache key, cache version, branch scope, exact or fallback match,
  payload bytes, restore time, Cargo freshness, and save outcome.
- Default to dependency-only target payloads with incremental compilation off
  on ephemeral CI runners.
- Add an explicit cache schema/version key and change it when workflow command
  coverage or cleanup rules change.
- Include target, profile, feature, and native environment dimensions when they
  are not already represented by a designated producer.
- Use trusted default-branch writers and restore-only pull-request consumers
  where branch policy permits.
- Prefer same-job sequencing for commands with observed compatible workspace
  artifacts.
- Skip or narrow caches whose restore cost approaches saved compilation.

### Prototype behind a compatibility boundary

- A read-only workflow and cache-key analyzer for GitHub Actions.
- A cache economics report joining Actions cache API data with Cargo JSON
  freshness.
- Matrix collision detection for omitted target, profile, feature, and schema
  dimensions.
- Optional rust-cache-aware payload modeling pinned to a reviewed action
  revision.

The implementation gate remains closed.

### Reject or defer

- Reject `cache-hit` as the sole success metric.
- Reject broad full-target fallback across unrelated branches or workspaces.
- Reject one immutable key shared accidentally by parallel incompatible matrix
  producers.
- Reject saving unused caches after skipped workloads.
- Defer workflow rewrites until the affected repository owner approves them.
- Defer cache signing, cross-organization trust, revocation, and remote
  provenance to PERF-Q30.
- Defer upstream issue, comment, or pull-request activity until explicit owner
  approval.

## Potential contribution paths

Without creating upstream activity, Q06 identifies:

1. documentation and fixtures for immutable-key matrix first-writer behavior;
2. rust-cache key diagnostics for command-line target, profile, and feature
   axes;
3. a reproducible restore-versus-compile benchmark protocol;
4. workflow guidance for trusted central producers and restore-only consumers;
5. portfolio fixes for cache steps placed after dominant failing setup work.

Any upstream contribution must first confirm that the behavior is not already
documented or intentionally scoped.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: cache hits are not integrity or correctness claims; path artifacts and remote trust remain excluded. |
| Compiler Performance Engineer | Accepted: restore, pack, compile-after-restore, artifact freshness, payload, and variance are separate; local transport is not generalized to hosted networks. |
| Interop Boundary Auditor | Accepted: OS, architecture, target triple, linker, native tools, and build-script environment remain explicit key and exclusion boundaries. |
| AI Assurance Skeptic | Accepted: exact hits with root rebuilds, unused restores, failed setup, eviction, corruption history, and warm-up variance remain visible. |
| Ecosystem Strategist | Accepted: GitHub Actions, rust-cache, and Cargo remain the transport and compatibility owners; FERRIUM supplies diagnosis. |
| Rust Maintainer | Accepted: recommendations preserve ordinary Cargo and existing workflow ownership; no automated workflow changes or upstream activity occurred. |
| Native Platform Adopter | Accepted: Windows, Linux, macOS, branch trust, quota, cleanup, rollback, and tool installation costs are represented. |
| Scope Keeper | Accepted: Q06 covers CI-local transport and retention; signed remote provenance remains Q30. |
| Validation Checker | Accepted: public runs, source revisions, commands, samples, medians, MAD, positive reuse, fallback, unused restore, skipped cache, and limitations are recorded. |

## Decision

PERF-Q06 is complete.

FERRIUM should define a read-only CI cache topology and economics diagnostic.
The next question is PERF-Q07: measure duplicate work and contention between
Cargo, rust-analyzer, editor checks, and concurrent validation commands using
the identity, scheduling, and cache models established in PERF-Q01 through
PERF-Q06.
