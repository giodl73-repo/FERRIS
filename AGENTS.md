# Ferris Agent Instructions

Read `CONTEXT.md` first. It is the canonical operating context for this
repository.

## Working rules

- Use `docs/plans/FERRIS_PROGRAM.md` for product authority and sequence.
- Use `docs/specs/README.md` for normative specification status and
  dependencies.
- Read the applicable `.roles` files before claiming a role review.
- Preserve Cargo and every external owner's authority; do not create parallel
  resolvers, hidden manifests, or synthetic owner truth.
- Keep Typebook/RUNE product-neutral and independently usable.
- Keep observation, planning, approval, and execution as separate records and
  authorities.
- Treat unknown, unsupported, unavailable, stale, failed, and not-observed as
  distinct states.
- Never place credentials or reusable secrets in plans, prompts, roots, refs,
  logs, fixtures, or evidence.
- Preserve ordinary Cargo and owner-native workflows and define removal and
  rollback for every adoption.

## Change authority

The closed bounded pulses in
`context/waves/2026-08-11-read-only-planning/pulses/`
authorize only local read-only `plan`, `explain`, declared-workspace `graph`,
passive local `doctor`, and the Pulse 14 two-file experimental
`profile-diff` product code over explicit local inputs and development
fixtures. Pulse 15 authorizes only the nine-family development fixture matrix
and conformance tests for that existing command; it adds no product behavior.
Pulse 16 authorizes only the public held-out contract, custody protocol, and
review. It does not authorize hidden fixture construction by implementation
authors, scoring, or oracle access. Pulse 17 authorizes only Windows and Unix
development validation evidence for the existing implementation and fixtures.
Pulse 18 authorizes only the public-CLI filesystem immutability test and its
bounded evidence. Pulse 19 authorizes only the representative ordinary-Cargo
consumer fixture, before-and-after conformance test, and evidence. Do not
implement profile generation, affected-only scope, `query`, `go`, mutation,
active probes, connectors, MCP, AI narrowing, approval, deployment, remote
evidence, or held-out oracle access. Any later product capability requires
another separately approved pulse.

The
`context/waves/2026-08-30-owner-validation-domains/` wave records explicit
owner approval for one bounded, non-executable owner-validation-domain pulse.
Pulse 01 authorizes only the optional closed prefix contract, opaque owner
entrypoint selection, conservative Cargo composition, and explicit lexical
classification of missing workspace-root-relative paths. Missing paths must
never narrow Cargo package scope without filesystem evidence. The pulse is
complete after a clean read-only implementation review, targeted core, CLI,
and schema proof, and eleven-role closeout. Its implementation and corrective
budgets are exhausted. External adopter migration, Git discovery, revision
evidence, command interpretation, and owner action execution require separate
pulses.

The
`context/waves/2026-08-30-revision-bound-validation-evidence/` wave records
explicit owner approval for one bounded, read-only revision-evidence pulse.
Pulse 01 authorizes only optional local Git revision resolution for
`validation-plan`, exact merge-base-to-head path classification, binding the
resulting plan to the resolved base, merge base, head, and tested revisions,
and deterministic stale or invalid diagnostics. The caller chooses the
revisions and the repository owner retains entrypoint commands, execution,
artifacts, success semantics, and required-check policy. Do not add remote
fetch, checkout mutation, signing, workflow parsing, execution-result
attestation, cache reuse, or CI narrowing.

The
`context/waves/2026-09-10-enterprise-corpus-qualification/` wave records
explicit owner approval for one bounded qualification of current Ferris
against five existing private enterprise-shape synthetic repositories. Pulse
01 may freeze exact private revisions, execute only their already-declared
scenarios on local Windows and hosted Ubuntu, and retain public-safe aggregate
evidence. It may create one temporary validation branch per private repository
only to run the existing scenario harness in hosted CI; those branches must not
merge and must be removed after evidence custody. It adds no product behavior,
does not alter a corpus scenario, and does not authorize affected-only gating,
support, production representativeness, or savings claims. Private repository
names, revisions, paths, raw output, and identifiable timings must not enter
this public repository.

The
`context/waves/2026-09-10-private-corpus-generator-portability/` wave records
separate explicit owner approval for one bounded correction of the private
`EC-04` deterministic generated-topology integrity failure found by the
enterprise-corpus qualification. Pulse 01 proved platform-native CRLF output
caused the mismatch, changed only the private generator to emit explicit
UTF-8/no-BOM/LF bytes, and passed the unchanged corpus on Windows and hosted
Ubuntu. The private pull request was merged only after separate explicit user
direction. The pulse is complete and
does not authorize Ferris product changes, scenario changes,
generated-topology semantic changes, or rerunning the exhausted qualification.

The
`context/waves/2026-09-10-corrected-enterprise-corpus-qualification/` wave
records separate explicit owner approval for one fresh qualification over the
merged corrected private corpus cutoff. Pulse 01 is complete and incomplete:
all five Windows owner preflights passed, 17 small and scale scenarios produced
durable matched evidence, and the federated runner then exceeded the Windows
process command-line limit before launching its maximum-input case. Its earlier
in-memory cases were not durably emitted and remain not observed. No hosted
Ubuntu branch or job ran. The pulse adds no product or scenario behavior and
does not authorize a runner fix, retry, workflow narrowing, production
representativeness, support, savings claims, or any successor run.

The
`context/waves/2026-09-10-private-federated-runner-portability/` wave records
separate explicit owner approval for one bounded correction of the private
`EC-05` federated runner. Pulse 01 is complete: only its Ferris process launch
changed to use the application root as the working directory and relative
in-application arguments, preserving the absolute outside-path negative
control and all 256 maximum inputs. The complete Windows owner and federated
suites passed, and the private pull request's hosted Ubuntu owner workflow
passed. The runner itself was not executed on hosted Ubuntu. The private pull
request was merged only after separate explicit user direction. The pulse does
not authorize a Ferris product change, scenario change, qualification retry,
support, production representativeness, or savings claim.

The
`context/waves/2026-09-10-post-portability-enterprise-corpus-qualification/`
wave records explicit user approval for one fresh evidence-only qualification
after both private owner-tooling corrections merged. Pulse 01 may freeze one
unchanged public Ferris product revision and five exact corrected private
default-branch revisions, then run every existing owner gate and declared
scenario on local Windows and hosted Ubuntu. It may create one temporary
hosted-validation branch per private repository only to invoke those unchanged
scenarios; those branches must not merge and must be removed after custody. It
does not reuse prior attempts as qualification evidence. Pulse 01 is complete:
all five owner gates passed on Windows and hosted Ubuntu; all 26 scenarios on
each platform matched their contracts with stable identities, including two
declared matched failures, seven conservative fallbacks, both 256-input
boundaries, and three typed rejections with no owner execution. No
selected/full divergence occurred. All temporary pull requests closed without
merge and all temporary branches were removed. This qualifies the corpus as a
deterministic regression and demonstration suite only; it authorizes no
product, scenario, owner-command, support, production, affected-only,
performance, or savings change or claim.

The
`context/waves/2026-09-09-real-history-validation-shadow/` wave records
explicit user approval for one bounded local evaluation pulse after PR #26.
Pulse 01 evaluated exactly 40 first-parent revisions from BISECT and
ICELINES, repeated revision-bound planning, and executed only owner commands and
required non-secret environments present at the historical revisions for the
three non-fallback cases. It retains bounded public evidence but adds no product
behavior and does not authorize adopter mutation, CI narrowing,
prevented-iteration, support, or realized-savings claims. The pulse is complete
and its planning and execution authority is exhausted; any new cohort, rerun,
or implementation requires separate approval.

The
`context/waves/2026-09-11-enterprise-onboarding-proof/` wave records explicit
user direction to create additional purpose-built private enterprise consumers.
Pulse 01 authorizes only two private owner-native baselines, identified publicly
as `EO-01` and `EO-02`: one ordinary Cargo workspace and one explicit
multi-workspace application shape. They must establish ordinary owner commands,
hosted Ubuntu CI, deterministic topology and scenario contracts, removal
invariants, and private custody before any Ferris adoption. Pulse 01 is complete
and incomplete: both repositories passed their Windows owner gates, but the
enterprise host kept repository Actions disabled, so zero hosted jobs ran and
no owner-baseline tag was frozen. The pulse stopped without alternate CI,
credentials, a third repository, or Ferris onboarding. It grants no Ferris
product change, consumer Ferris dependency, onboarding adapter, Action Plan,
execution, workflow replacement, production, support, performance, or savings
authority. Pulse 02 remains not authorized.

The active
`context/waves/2026-08-12-platform-profile-conformance/` wave currently
authorizes Pulse 01 documentation and governance plus Pulse 02's frozen
`ferris.platform-profile/v1` schema documents, incomplete exemplar, exact
negative-control mutations, and review. Pulse 03 additionally authorizes one
dependency-free test-only Rust harness that executes those controls. Do not
add production schema types. Pulse 04 authorizes only the pure-data family,
its two zero-dependency consumer revisions, isolated locked/offline owner
commands, source snapshots, and test-only profile materialization. Do not
add external dependencies or generate a product profile. Pulse 05 additionally
authorizes only the CLI/configuration family and reusable integration-test
support for later controlled families. Do not complete another family,
construct hidden held-out material, score an oracle, or change PLATFORM-001
status until the corresponding later pulse and role review grant that exact
authority.

Pulse 06 additionally authorizes only the in-process hosted-service family.
Do not add sockets, network, databases, TLS, credentials, deployment, or
production service behavior.

Pulse 32 is now permanently closed `invalid` at `cutoff-build-freeze`.
Checkout and package gates passed, but custody could not freeze the Ubuntu
executable; zero preflight, public-input validation, cases, or candidates ran,
and the conclusion is null. Pulse 33 authorizes only the public external
build-freeze release, root-cause record, governance, review, and test-only
validation. It records a WSL non-login `PATH` omission, explicit rustup Cargo
discovery, Cargo JSON artifact output, and deterministic Windows/Ubuntu
build hashes without executing a diagnostic or changing product code.

Pulse 34 authorized one independent diagnostic at an immutable cutoff that
contains the complete Pulse 33 release and predates the authority. Public
checkout, binding, package, build-freeze, adapter-preflight, and input gates
passed, but isolated corpus materialization did not complete. Pulse 34 is
permanently closed `invalid` with zero candidate launches, a null conclusion,
and no retry or reuse authority.

Pulses 69 through 82 are sealed successor infrastructure only. The latest
callable chain terminates at Pulse 82 over the Pulse 78 staging/bootstrap
hardening and Pulse 81 exact Pulse 35 release-tree binding. These releases
grant no diagnostic authority and perform no real FERRIS execution. Any
successor authority requires a separate approved pulse with an immutable
pre-authority cutoff.

Pulse 83 is governance/test-only readiness evidence at merged cutoff
`dfc889b`. It grants no authority and invokes no Pulse 82 callable. A future
authority must use a later self-excluding cutoff containing the complete Pulse
83 review.

Pulse 84 authorizes exactly one later independent invocation attempt of the
exact Pulse 82 callable at self-excluding cutoff `f874ebf`. This pulse records
authority only and performs no custody or execution. Failed pre-call gates do
not consume authority; the sole callable attempt does, and no retry, resume,
alternate callable, score, fix, support, or PLATFORM-001 authority follows.

Pulse 85 permanently closes Pulse 84 after its sole consumed invocation
stopped `not-attempted` at Ubuntu capability build custody with
`P57-WSL-BUNDLE`. Windows custody passed, no seed or candidate ran, no terminal
publication or transfer occurred, cleanup completed, and all conclusions
remain null. Pulse 84 cannot be retried or resumed.

Pulse 86 is sealed prospective capability infrastructure only. It byte-binds
exact Pulse 78, derives the WSL operational username from the native runtime
parent's owner through one explicit-root read-only lookup, and binds every
staging, revalidation, worker, and cleanup spawn with `--user`. It filters no
stderr, grants no authority, and does not retry or reinterpret Pulse 84.

Pulse 87 is the sealed ordered successor over exact Pulse 86. It preserves
Pulse 81's exact Pulse 35 release-tree binding and Pulse 70/Pulse 58 ordering,
carries the parent-owner WSL identity into the ordered layer, and preserves
`P86-INDETERMINATE-STAGE-CLEANUP` before seed creation. It grants no
publication, witness, or authority.

Pulse 88 is the sealed witness-preserving successor over exact Pulse 87. It
retains Pulse 82/Pulse 59 terminal publication and cleanup semantics, proves
`P86-INDETERMINATE-STAGE-CLEANUP` remains publication-not-attempted, and
grants no diagnostic authority or real execution.

## Research and specifications

- Inventory local evidence before using external sources.
- Cite actionable research claims with files, line ranges, URLs, or measured
  commands.
- Give findings stable `FERRIS-*` identifiers; retain historical `FERRIUM-*`
  identifiers unchanged.
- Use MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY according to
  `docs/specs/README.md`.
- Keep specification dependencies acyclic and owner-aligned.
- Do not mark a specification Proposed or Adopted without its stated fixtures,
  measurements, and role approvals.
- A review must record all nine role dispositions, completed revisions,
  remaining gates, and implementation authority.

## Validation and commits

- Validate Markdown links and code fences for changed documentation.
- Run `git diff --check`.
- Review staged paths and the specification dependency graph before committing.
- Keep logical changes in focused commits.
- Include the required Copilot co-author trailer when applicable.
- Do not push unless requested.
- Keep Ferris child-repo commits separate from TRACKER submodule-pointer
  updates.
