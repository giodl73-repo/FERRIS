# EXP-01: Reference Model Comparison

Date: 2026-08-10
Question: BLUE-Q01
Result: typed refs over immutable roots fit the required workflows; a generic
label does not.

## Comparison

| System | Exact object | Moving name | Fixed name | History / rollback | Retention | Blueprint consequence |
|---|---|---|---|---|---|---|
| Git | commit ID | branch | annotated tag by convention | reflog; expected-value and transactional updates | external policy | use branch, tag, history, and compare-and-set semantics; keep worktree-local head separate |
| OCI | digest | registry tag | digest | registry-dependent | registry policy | verify roots by digest; channel movement is not integrity |
| Nix / Guix | store path | profile or channel | selected generation or manifest pin | generations and rollback | GC roots | use pins and leases as retention roots; retain generations |
| OSTree | commit checksum | ref | commit checksum | parallel deployments and rollback | repository policy | promote and roll back by ref movement over immutable roots |
| Bazel | action / CAS digest | none equivalent | digest | cache policy | cache policy | keep target labels, action lookup, and content identity separate |
| rustup | exact or dated toolchain | stable, beta, nightly | exact version/date | explicit override or prior toolchain selection | installation policy | reserve channel for moving policy streams |
| npm | package version | dist-tag | immutable published version | move dist-tag to prior version | registry policy | channel-like release aliases must not become version identity |
| GitHub Actions cache | immutable entry after creation | restore-prefix search | exact cache key entry | no general ref history | quota and idle eviction | lookup hints and branch scope are not compatibility or correctness |
| Cargo today | mutable local build state | target-directory convention | final published package version only | rebuild / clean; no build-root refs | local cleanup | add an external root/ref model; do not name or share live target state |

## Hypothesis results

| Hypothesis | Result | Reason |
|---|---|---|
| Rust users need Git-like branches and tags for build state | Partly supported | branch and fixed-tag semantics are useful, but promotion channels and retention pins are distinct |
| One free-form label is sufficient | Rejected | mutation, authority, retention, and trust rules differ by name kind |
| Git branches can be reused directly | Rejected | Git identifies source lineage; Blueprint roots include build, environment, validation, and evidence identity |
| A human ref can serve as a cache key | Rejected | action identity selects candidates and content digests verify bytes |
| Moving `main-green` makes a root trusted | Rejected | trust and validation gates precede channel movement |
| Refs accelerate builds without restoration | Rejected | refs accelerate discovery and decisions; bytes must still be compatibly materialized or rebuilt |
| Retention can use age alone | Rejected | tags, pins, leases, audit policy, quotas, and shared reachability matter |
| Ref updates need no sequencing or compare-and-set | Rejected | stale-writer, replay, and rollback controls require generations and expected values |
| Branch switching should share complete targets | Rejected | local evidence and Cargo issues show poisoning, collision, and concurrency risk |
| Every remote hit is beneficial | Rejected | verification, transfer, extraction, contention, miss risk, and avoided work determine benefit |

## Atomic update control

A disposable Git repository created two immutable commits, then:

1. created `refs/blueprint/channels/stable` at root A;
2. moved it to root B with root A as the expected prior value;
3. attempted a stale move with root A still supplied as the expected value;
4. inspected the reflog; and
5. rolled back to root A with root B as the expected prior value.

Observed result:

- promotion succeeded;
- stale compare-and-set failed with exit code 128;
- the reflog retained both generations; and
- rollback succeeded without modifying either root.

The control supports expected-value updates and durable history. It does not
authorize using Git refs as Blueprint's storage implementation.

## Selected state rules

| Kind | May move? | Trust meaning | Retention meaning | Removal |
|---|---|---|---|---|
| branch | yes, explicit and logged | none | policy default only | delete ref; retain history by policy |
| tag | no | records publication, not correctness by itself | retained according to publication policy | tombstone; do not retarget |
| channel | yes, after policy gates | named promotion policy | while accepted plus policy | expire, move, or tombstone |
| alias | yes, local | none | none | delete |
| pin | no implicit movement | none | prevents collection | explicit release |
| lease | renews until expiry | none | temporary retention | release or expire |
| label | not a ref | none | none | edit metadata under audit policy |
| tombstone | append-only denial | denies future resolution | preserves audit, not necessarily bytes | supersede only through newer policy |

## Safe fallback

Every resolution path ends in one of:

```text
accepted root + eligible material + positive economics -> separately gated materialization
accepted root without eligible material               -> compare or rebuild
missing / stale / incompatible / unknown               -> rebuild
expired / replayed / revoked / corrupt                 -> reject, quarantine, rebuild
uneconomic                                              -> skip restore, rebuild
```

Ordinary Cargo operation remains possible after complete Blueprint removal.
