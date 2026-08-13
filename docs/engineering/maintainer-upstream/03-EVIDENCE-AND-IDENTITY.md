# Evidence and Identity

Status: Guidance
Implementation authority: None

## Identity rule

A contribution packet is a versioned local evidence artifact. Its identity
must remain distinct from an external issue, pull request, benchmark,
upstream decision, accepted artifact, and later maintenance record. The
canonical required fields and statuses are defined in the
[Rust Performance Contribution Packet](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

## Packet identity

Every packet must record:

- stable packet ID and version;
- originating research question and finding IDs;
- exact upstream repository, issue, goal, standard, or owner;
- packet maintainer and expected upstream owner;
- status and last verification date;
- exact source, fixture, toolchain, environment, and command identity;
- source and fixture licenses;
- public, synthetic, or private provenance;
- related packet, external artifact, supersession, and retirement links; and
- approval state for external posting.

Do not reuse one packet ID for materially different mechanisms, owners, or
requests. Version the packet for evidence-preserving revisions. Create a new
packet when the maintainer question or distinguishing behavior changes.

## Status vocabulary and transition evidence

| Status | Required evidence |
|---|---|
| Draft | Local evidence is being assembled; gaps are explicit |
| Reproduced | The case reruns on the declared environment |
| Minimized | Positive, negative, and correctness controls survive reduction |
| Owner-aligned | Current owner confirms usefulness and destination |
| Submission-ready | Format, license, public safety, commands, validation, burden, and approval are complete |
| Submitted | An approved external artifact exists |
| Accepted | The upstream artifact or decision is accepted |
| External | Owner prefers the case remain outside the project |
| Superseded | A named newer artifact replaces this packet |
| Retired | Reproduction or maintenance has ended for a recorded reason |

Transitions are not automatic. In particular:

- Reproduced does not imply Minimized.
- Minimized does not imply Owner-aligned.
- Submission-ready does not imply permission to post unless approval is
  recorded.
- Submitted does not imply Accepted.
- Accepted does not imply maintained forever.
- External and Retired are valid outcomes, not failures to hide.

## Evidence classes

Keep these classes separate:

| Class | Meaning |
|---|---|
| Observed | Direct result from a named source, command, or owner statement |
| Inferred | A bounded interpretation of observations |
| Predicted | An expected result not yet observed |
| Approved | A human or owner decision with scope and time |
| Executed | An action actually performed |
| Yielded | An outcome produced by execution |
| Failed | The attempt ran and did not satisfy its success condition |
| Unsupported | The owner or declared contract does not support the case |
| Unavailable | A required source or capability could not be accessed |
| Not-observed | The evidence collection did not inspect the fact |
| Stale | Evidence exceeded its renewal window |
| Conflicting | Sources disagree and the conflict remains unresolved |
| Unknown | Evidence is insufficient to classify the fact |

Do not convert failure, unsupported, unavailable, not-observed, stale,
conflicting, or unknown into success-shaped prose.

## Command and environment evidence

Commands must be copyable and name:

- working directory;
- source revision or fixture recipe;
- toolchain and target;
- relevant environment variables;
- manifest, lockfile, features, profile, and target directory;
- baseline, edit or patch, comparison, controls, cleanup, and rerun steps;
- expected exit status;
- output interpretation and resource bounds; and
- platform, hardware, cache, concurrency, power, VM, indexing, or thermal
  uncertainty where relevant.

Performance evidence must distinguish cold, incremental, check, build, test,
link, and runtime workflows. It must include distributions or the
owner-required format and must not promote one wall-time observation. These
requirements align with the
[Compiler Performance Engineer role](../../../.roles/parliament/compiler-performance-engineer.md).

## Public-safe evidence

Before a packet can become Submission-ready:

- source and fixture licenses must permit the intended sharing and use;
- copied code, logs, traces, profiles, screenshots, and generated artifacts
  must be reviewed for disclosure;
- repository names, paths, hostnames, usernames, tenant identifiers, issue
  text, and environment values must be treated according to their provenance;
- credentials, tokens, keys, cookies, signing material, and reusable secrets
  must never enter packets, fixtures, commands, logs, roots, refs, or durable
  evidence;
- private source must be replaced with a licensed public or synthetic
  reproducer before public submission;
- redaction must not destroy the distinguishing mechanism or make commands
  misleading;
- third-party license notices and upstream-required metadata must be retained;
- provenance must identify whether each input is public, synthetic, or
  private.

For rustc-perf benchmarks, include the upstream-required `REUSE.toml`,
`Cargo.lock`, and benchmark licensing steps listed in the
[packet specification](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

## Evidence integrity

- Preserve exact raw outputs or stable digests when retention permits.
- Bind summaries to the source revision, command, environment, and result.
- Record failed and rejected minimization attempts.
- Separate user impact from stable benchmark metrics.
- Record limitations and unmeasured platforms.
- Do not let AI-generated summaries establish owner truth, approval,
  soundness, security, performance, or maintenance state.
- Keep immutable packet versions or auditable changes so later renewal does
  not rewrite historical claims.

These practices implement the evidence and immutable-history boundaries in
[CONTEXT.md](../../../CONTEXT.md) and the
[AI Assurance Skeptic role](../../../.roles/parliament/ai-assurance-skeptic.md).

## Stewardship identity

Do not collapse the following identities:

- crates.io owner and team;
- human or trusted-workflow publisher;
- repository owner;
- code contributor and reviewer;
- CODEOWNER or ruleset authority;
- security contact;
- sponsor or funder;
- release actor;
- consumer support owner;
- successor or alternative maintainer.

Record observation time, source, confidence, and changes. Current publication
policy must remain distinct from historical release provenance. Funding and
organization ownership are context, not continuity proof. See
[Rust maintenance and stewardship](../../research/2026-08-09-rust-maintenance-stewardship.md).

## External link policy

Use repository-relative links for Ferris sources. Include authoritative
external links only when they are already present in the cited source
documents and are necessary for the owner-native workflow. Do not copy broad
web research or unofficial guidance into a packet as authority.

The owner-specific external references already approved in the packet
specification include:

- <https://github.com/rust-lang/rustc-perf/blob/main/collector/compile-benchmarks/README.md>
- <https://github.com/rust-lang/rustc-perf/blob/main/collector/README.md>
- <https://rustc-dev-guide.rust-lang.org/tests/perf.html>
- <https://rustc-dev-guide.rust-lang.org/profiling/with-rustc-perf.html>
- <https://perf.rust-lang.org/help.html>
- <https://github.com/rust-lang/cargo/blob/master/CONTRIBUTING.md>
- <https://doc.crates.io/contrib/>
- <https://github.com/rust-lang/cargo/blob/master/benches/README.md>

## Submission-ready evidence gate

A packet is Submission-ready only when the upstream home and question are
explicit, the reproducer is licensed and public-safe, controls survive
minimization, commands rerun cleanly, owner-required metrics exist,
limitations remain visible, the requested action is bounded, maintenance is
accepted, burden is recorded, and external submission is approved.
